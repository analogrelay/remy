use std::sync::{Arc, atomic};

pub struct Clock {
    cycles: u64,
    suspend_count: Arc<atomic::AtomicIsize>
}

impl Clock {
    /// Creates a new Clock initialized to zero
    pub fn new() -> Clock {
        Clock {
            cycles: 0,
            suspend_count: Arc::new(atomic::AtomicIsize::new(0))
        }
    }

    /// Sets the total number of cycles on the clock. Ignores the paused state of the clock.
    pub fn set(&mut self, value: u64) {
        self.cycles = value;
    }

    /// Gets the current number of cycles on the clock.
    pub fn get(&self) -> u64 {
        self.cycles
    }

    /// Advances the clock forward by `amount` cycles
    pub fn tick(&mut self, amount: u64) {
        let suspend_count = self.suspend_count.load(atomic::Ordering::Acquire);
        if suspend_count == 0 {
            self.cycles += amount;
        }
    }

    /// Suspends the clock so that calls to `tick` will have no effect until `resume` is called.
    pub fn suspend(&self) -> ClockSuspendGuard {
        self.suspend_count.fetch_add(1, atomic::Ordering::AcqRel);
        ClockSuspendGuard {
            suspend_count: self.suspend_count.clone()
        }
    }
}

#[must_use]
pub struct ClockSuspendGuard {
    suspend_count: Arc<atomic::AtomicIsize>
}

impl Drop for ClockSuspendGuard {
    fn drop(&mut self) {
        self.suspend_count.fetch_sub(1, atomic::Ordering::AcqRel);
    }
}
