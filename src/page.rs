use std::thread::JoinHandle;
use std::sync::{Arc, Mutex, atomic::{Ordering, AtomicBool}};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

use pasts::prelude::*;

static RUNNING: AtomicBool = AtomicBool::new(false);

struct ExitFuture(Arc<Mutex<Option<Waker>>>);

impl Future for ExitFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if RUNNING.load(Ordering::Relaxed) {
            *self.get_mut().0.lock().unwrap() = Some(cx.waker().clone());
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

/// A page (or screen) of the app.
pub struct Page {
    max_threads: usize,
    threads: Vec<JoinHandle<()>>,
    use_threads: bool,
    tasks: Vec<Box<dyn Future<Output=()> + 'static>>,
    waker: Arc<Mutex<Option<Waker>>>,
    task_wakers: Vec<Arc<Mutex<Option<Waker>>>>,
}

impl Page {
    /// Create a new page.
    pub fn new() -> Self {
        let max_threads = num_cpus::get();

        RUNNING.store(true, Ordering::Relaxed);
        Page {
            waker: Arc::new(Mutex::new(None)),
            task_wakers: Vec::new(),
            max_threads,
            #[cfg(not(target_arch = "wasm"))]
            threads: Vec::with_capacity(max_threads),
            #[cfg(target_arch = "wasm")]
            threads: Vec::new(),
            tasks: Vec::new(),
            #[cfg(not(target_arch = "wasm"))]
            use_threads: true,
            #[cfg(target_arch = "wasm")]
            use_threads: false,
        }
    }

    /// Force using or not using threads for all platforms.  WARNING: This will
    /// break your code some platforms if you set it to true.
    pub fn use_threads(mut self, use_threads: bool) -> Self {
        self.use_threads = use_threads;
        self
    }

    /// Spawn a repeating task that may or may not be run on a separate thread
    /// (depending on the target - WASM won't run threads).  It should always be
    /// assumed that the task will run on a thread for maximum portability.  It
    /// also must "not block".  If you need to run multiple tasks on the same
    /// thread, use `[task1, task2].select().await`.
    pub fn spawn<F: Future<Output=()> + 'static, A: Send + 'static + FnOnce() -> F>(mut self, future: A) -> Self {
        let new_waker = Arc::new(Mutex::new(None));

        if self.use_threads {
            let waker = Arc::clone(&self.waker);
            let task_waker = Arc::clone(&new_waker);
            let handle = std::thread::spawn(move || {
                // FIXME: Different executor for each thread.
                static EXECUTOR: pasts::CvarExec = pasts::CvarExec::new();
            
                let mut exit_future = ExitFuture(task_waker);
                println!("Starting Thread");
                EXECUTOR.block_on([future().fut(), exit_future.fut()].select());
                println!("Exit Thread");
                RUNNING.store(false, Ordering::Relaxed);
                if let Some(waker) = waker.lock().unwrap().take() {
                    waker.wake();
                }
            });

            self.threads.push(handle);
            self.task_wakers.push(new_waker);
        } else {
            
        }
        self
    }

    /// Wait for the user to exit the page.
    pub fn join(mut self) {
        static EXECUTOR: pasts::CvarExec = pasts::CvarExec::new();

        let exit_future = ExitFuture(Arc::clone(&self.waker));
        EXECUTOR.block_on(
            async {
                exit_future.await;
                println!("EXIT");
            }
        );
        println!("Finished main thread");
        // Wake all that have not been waken.
        for ref mut waker in self.task_wakers {
            if let Some(waker) = waker.lock().unwrap().take() {
                println!("IS Waker {:p}", &waker);
                waker.wake();
            }
        }
        while let Some(thread) = self.threads.pop() {
            thread.join().unwrap();
        }
    }

    /// Returns the number of still unused cores.  If more threads are used than
    /// the number of cores, returns 0.
    pub fn unused_core_count(&self) -> usize {
        if self.threads.len() >= self.max_threads {
            0
        } else {
            self.max_threads - (self.threads.len())
        }
    }
}
