use std::sync::atomic::Ordering;

const STATE_UNLOCKED: usize = 0;
const STATE_LOCKED_W: usize = std::usize::MAX;

pub(crate) struct ReadIOLock<'a> {
    iostate: &'a mut IOLock,
}

impl<'a> Drop for ReadIOLock<'a> {
    fn drop(&mut self) {
        // Decrement this thread (Unlock).
        self.iostate.state.fetch_sub(1, Ordering::SeqCst);
    }
}

pub(crate) struct WriteIOLock<'a> {
    iostate: &'a mut IOLock,
}

impl<'a> Drop for WriteIOLock<'a> {
    fn drop(&mut self) {
        // Unlock writer.
        self.iostate.state.store(STATE_UNLOCKED, Ordering::SeqCst);
    }
}

/// A global IOLock, that allows uninitialized data.
#[repr(C)]
pub(crate) struct IOLock {
    state: std::sync::atomic::AtomicUsize,
}

impl IOLock {
    /// 
    pub const fn new() -> IOLock {
        IOLock {
            state: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    pub fn lock_read<'a>(&'a mut self) -> () {
        // Aquire the lock.
        loop {
            let value = self.state.load(Ordering::SeqCst);
            // Writer has locked data.
            if value == STATE_LOCKED_W { continue }
            // Attempt to lock before another writer lock, now that it's not locked.
            if self.state.compare_and_swap(value, value + 1, Ordering::SeqCst) == value {
                break;
            }
        }
    }

//     pub fn unlock_read<'a>

    pub fn lock_write<'a>(&'a mut self) -> () {
        // Aquire the lock.
        loop {
            let value = self.state.load(Ordering::SeqCst);
            // Writer has locked data.
            if value != STATE_UNLOCKED { continue }
            // Attempt to lock before another writer lock, now that it's not locked.
            if self.state
                .compare_and_swap(value, STATE_LOCKED_W, Ordering::SeqCst) == value
            {
                break;
            }
        }

/*        WriteIOLock {
            iostate: self,
        }*/
    }
}
