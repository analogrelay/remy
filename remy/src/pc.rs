use std::num;

#[derive(Copy,Show,PartialEq)]
pub enum ProgramCounterError {
	AdvancedOutOfBounds
}

pub struct ProgramCounter<S: num::UnsignedInt + num::FromPrimitive> {
	pc: S
}

impl<S: num::UnsignedInt + num::FromPrimitive> ProgramCounter<S> {
	pub fn new() -> ProgramCounter<S> {
		ProgramCounter { pc: num::Int::zero() }
	}

	pub fn get(&self) -> S {
		self.pc
	}

	pub fn set(&mut self, val: S) {
		self.pc = val;
	}

	pub fn advance<A: num::Int>(&mut self, amount: A) -> Result<(), ProgramCounterError> {
		let cur = self.pc.to_u64().unwrap(); // This should be safe unless we're emulating a 128-bit arch??
		let amo = amount.to_i64().unwrap();  // Again, should be safe

		let res = (cur as i64 + amo) as u64;
		let new = num::NumCast::from(res);
		match new {
			Some(v) => { self.pc = v; Ok(()) },
			None    => Err(ProgramCounterError::AdvancedOutOfBounds)
 		}
	}
}

#[cfg(test)]
mod test {
	use pc::{ProgramCounter,ProgramCounterError};

	#[test]
	pub fn advance_by_positive_value_increases_pc() {
		let mut pc : ProgramCounter<u8> = ProgramCounter::new();
		assert!(pc.advance(42).is_ok());
		assert_eq!(pc.get(), 42);
	}

	#[test]
	pub fn advance_by_negative_value_decreases_pc() {
		let mut pc : ProgramCounter<u8> = ProgramCounter::new();
		assert!(pc.advance(42).is_ok());
		assert!(pc.advance(-24).is_ok());
		assert_eq!(pc.get(), 18);
	}

	#[test]
	pub fn advance_below_zero_causes_error() {
		let mut pc : ProgramCounter<u8> = ProgramCounter::new();
		assert!(pc.advance(42).is_ok());
		assert_eq!(pc.advance(-43).unwrap_err(), ProgramCounterError::AdvancedOutOfBounds);
		assert_eq!(pc.get(), 42); // PC should be unchanged
	}

	#[test]
	pub fn advance_above_pc_size_zero_causes_error() {
		let mut pc : ProgramCounter<u8> = ProgramCounter::new();
		assert!(pc.advance(250).is_ok());
		assert_eq!(pc.advance(255).unwrap_err(), ProgramCounterError::AdvancedOutOfBounds);
		assert_eq!(pc.get(), 250); // PC should be unchanged
	}
}