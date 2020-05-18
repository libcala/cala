use std::sync::{Arc, Mutex, atomic::{Ordering, AtomicU8}};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

use pasts::prelude::*;
use pasts::CvarExec;

const RUNNING_FALSE: u8 = 0;
const RUNNING_TRUE: u8 = 1;
const RUNNING_QUITTING: u8 = 2;

static RUNNING: AtomicU8 = AtomicU8::new(RUNNING_FALSE);

struct ExitFuture(Arc<Mutex<Option<Waker>>>);

impl Future for ExitFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if RUNNING.load(Ordering::Relaxed) == RUNNING_TRUE {
            *self.get_mut().0.lock().unwrap() = Some(cx.waker().clone());
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

// A processor core.
struct Core {
    executor: Option<Box<CvarExec>>,
    task_list: Vec<Box<dyn Send + FnOnce() -> Pin<Box<dyn Future<Output=()> + 'static>>>>,
    waker: Arc<Mutex<Option<Waker>>>,
}

/// A page (or screen) of the program.
pub struct Page {
    count: usize,
    index: usize,
    cores: Vec<Core>,
}

impl Page {
    /// Create a new page.
    pub fn new() -> Self {
        #[cfg(not(target_arch = "wasm"))]
        let max_threads = num_cpus::get();
        #[cfg(target_arch = "wasm")]
        let max_threads = 1;
        let mut cores = Vec::with_capacity(max_threads);

        for _ in 0..max_threads {
            cores.push(Core { executor: Some(Box::new(CvarExec::new())), task_list: Vec::new(), waker: Arc::new(Mutex::new(None)) });
        }

        if RUNNING.compare_and_swap(RUNNING_FALSE, RUNNING_TRUE, Ordering::Relaxed) != RUNNING_FALSE {
            panic!("Can't be on more than one page at the same time!");
        }

        Page {
            count: 0,
            index: 0,
            cores,
        }
    }

    /// Spawn a repeating task that may or may not be run on a separate thread
    /// (depending on the target - WASM won't run threads).  It should always be
    /// assumed that the task will run on a thread for maximum portability.  It
    /// also must "not block".  If you need to run multiple tasks on the same
    /// thread, use `[task1, task2].select().await`.  Specify tasks in order of
    /// priority (spawn the highest priority task first) for best performance.
    #[allow(unsafe_code)]
    pub fn spawn<A, F>(mut self, future: A) -> Self
        where A: Send + 'static + FnOnce() -> F, F: Future<Output=()> + 'static
    {
        self.count += 1;
        // Add task to core
        self.cores[self.index].task_list.push(Box::new(|| Box::pin(future())));
        // Cycle through cores
        self.index += 1;
        if self.index == self.cores.len() {
            self.index = 0;
        }

        

        /*let new_waker = Arc::new(Mutex::new(None));

            let Core { executor, task_list } = self.cores.pop().unwrap();

            let waker = Arc::clone(&self.waker);
            let task_waker = Arc::clone(&new_waker);
            let handle = std::thread::spawn(move || {
                let executor = Box::leak(executor);
                let _box = unsafe { Box::from_raw(executor) };

                let mut exit_future = ExitFuture(task_waker);
                executor.block_on([future().fut(), exit_future.fut()].select());
                RUNNING.store(RUNNING_QUITTING, Ordering::Relaxed);
                if let Some(waker) = waker.lock().unwrap().take() {
                    waker.wake();
                }
            });

            self.threads.push(handle);
            self.task_wakers.push(new_waker);*/

        self
    }

    /// Wait for the user to exit the page.
    pub fn join(mut self) {
        let mut threads = Vec::with_capacity(self.cores.len() - 1);
        let core_count = self.cores.len();
        for i in 0..core_count {
            // Don't spawn a thread, use the current thread.
            if i == self.index { continue }
            
            let executor = Box::leak(self.cores[i].executor.take().unwrap());
            self.cores[i].executor = Some(unsafe { Box::from_raw(executor) });
            
            let mut task_list = Vec::new();
            std::mem::swap(&mut task_list, &mut self.cores[i].task_list);
            
            // Spawn a thread
            let task_waker = Arc::clone(&self.cores[i].waker);
            let main_waker = Arc::clone(&self.cores[self.index].waker);
            threads.push(std::thread::spawn(move || {
                let mut futures = Vec::new();
                let mut future_list = Vec::new();
                for task in task_list.drain(..) {
                    future_list.push(task());
                }
                for task in &mut future_list {
                    futures.push(DynBoxFut::box_fut(task));
                }
                let mut exit_future = ExitFuture(task_waker);
                futures.push(exit_future.fut());
                executor.block_on(futures.as_mut_slice().select());
                RUNNING.store(RUNNING_QUITTING, Ordering::SeqCst);
                if let Some(waker) = main_waker.lock().unwrap().take() {
                    waker.wake();
                }
            }));
        }

        // Start this thread's executor.
        let executor = Box::leak(self.cores[self.index].executor.take().unwrap());
        self.cores[self.index].executor = Some(unsafe { Box::from_raw(executor) });

        let mut task_list = Vec::new();
        std::mem::swap(&mut task_list, &mut self.cores[self.index].task_list);
        
        let task_waker = Arc::clone(&self.cores[self.index].waker);

        let mut futures = Vec::new();
        let mut future_list = Vec::new();
        for task in task_list.drain(..) {
            future_list.push(task());
        }
        for task in &mut future_list {
            futures.push(DynBoxFut::box_fut(task));
        }
        let mut exit_future = ExitFuture(task_waker);
        futures.push(exit_future.fut());
        executor.block_on(futures.as_mut_slice().select());
        RUNNING.store(RUNNING_QUITTING, Ordering::SeqCst);
        
        // Wake all that have not been waken.
        for i in 0..self.cores.len() {
            // Skip this thread, we're already awake.
            if i == self.index { continue }

            if let Some(waker) = self.cores[i].waker.lock().unwrap().take() {
                waker.wake();
            }
        }
        // Join all of the threads now that they have been told to exit.
        while let Some(thread) = threads.pop() {
            thread.join().unwrap();
        }
    }

    /// Returns the number of still unused cores.  If more threads are used than
    /// the number of cores, returns 0.
    pub fn unused_core_count(&self) -> usize {
        if self.count >= self.cores.len() {
            0
        } else {
            self.cores.len() - self.count
        }
    }
}
