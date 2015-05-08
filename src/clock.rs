pub struct Clock {
    cycles: u64
}

impl Clock {
    /// Creates a new Clock initialized to zero
    pub fn new() -> Clock {
        Clock { cycles: 0 }
    }

    /// Sets the total number of cycles on the clock
    pub fn set(&mut self, value: u64) {
        self.cycles = value;
    }

    /// Gets the current number of cycles on the clock
    pub fn get(&self) -> u64 {
        self.cycles
    }

    /// Advances the clock forward by `amount` cycles
    pub fn tick(&mut self, amount: u64) {
        self.cycles += amount;
    }
}
