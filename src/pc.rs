#[derive(Copy,Debug,Eq,PartialEq)]
pub struct ProgramCounter {
	pc: usize
}

impl ProgramCounter {
	pub fn new() -> ProgramCounter {
		ProgramCounter { pc: 0 }
	}

	pub fn get(&self) -> usize {
		self.pc
	}

	pub fn set(&mut self, val: usize) {
		self.pc = val;
	}

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