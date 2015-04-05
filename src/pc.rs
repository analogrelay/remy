/// Represents a program counter value
#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub struct ProgramCounter {
	pc: usize
}

impl ProgramCounter {
    /// Allocates a new program counter value (initialized to 0)
	pub fn new() -> ProgramCounter {
		ProgramCounter { pc: 0 }
	}

    /// Retrieves the current value of the program counter
	pub fn get(&self) -> usize {
		self.pc
	}

    /// Sets the program counter to the provided value
    ///
    /// # Arguments
    ///
    /// * `val` - The value to set the program counter to
	pub fn set(&mut self, val: usize) {
		self.pc = val;
	}

    /// Advances or retreats the program counter by the provided (signed) amount
    ///
    /// # Argument
    ///
    /// * `amount` - The amount to advance (or retreat) the program counter by
	pub fn advance(&mut self, amount: isize) {
		self.pc = (self.pc as isize + amount) as usize;
	}
}

#[cfg(test)]
mod test {
	use pc::ProgramCounter;

	#[test]
	pub fn advance_by_positive_value_increases_pc() {
		let mut pc = ProgramCounter::new();
		pc.advance(42);
		assert_eq!(pc.get(), 42);
	}

	#[test]
	pub fn advance_by_negative_value_decreases_pc() {
		let mut pc = ProgramCounter::new();
		pc.advance(42);
		pc.advance(-24);
		assert_eq!(pc.get(), 18);
	}
}
