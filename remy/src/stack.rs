use std::error::FromError;
use std::mem::size_of;
use std::num::{Int,UnsignedInt,NumCast,FromPrimitive};

use mem;

#[derive(Show,PartialEq)]
pub enum StackError {
	ErrorAccessingMemory(mem::MemoryError),
	StackPointerTooLarge, // Stack Pointer is too large to represent in a native int.
	ReachedPushBoundary,
	ReachedPopBoundary
}

impl FromError<mem::MemoryError> for StackError {
	fn from_error(err: mem::MemoryError) -> StackError {
		StackError::ErrorAccessingMemory(err)
	}
}

pub struct Stack<'a, A: UnsignedInt, M: mem::Memory<A>+'a> {
	memory: &'a mut M,
	sp: A,

	// The memory location at which pushes may no longer occur (end of the stack)
	push_boundary: Option<A>,

	// The memory location at which pops may no longer occur
	pop_boundary: Option<A>
}

impl<'a, A: UnsignedInt+FromPrimitive, M: mem::Memory<A>> Stack<'a, A, M> {
	pub fn new(memory: &'a mut M, sp: A) -> Stack<'a, A, M> {
		Stack::with_boundaries(memory, sp, Option::None, Option::None)
	}

	pub fn with_boundaries(memory: &'a mut M, sp: A, push_boundary: Option<A>, pop_boundary: Option<A>) -> Stack<'a, A, M> {
		Stack {
			memory: memory,
			sp: sp,
			push_boundary: push_boundary,
			pop_boundary: pop_boundary
		}
	}

	pub fn push<I: Int>(&mut self, val: I) -> Result<(), StackError> {
		// Check the boundaries
		if self.push_boundary.map_or(false, |b| self.sp <= b) {
			Err(StackError::ReachedPushBoundary)
		}
		else {
			try!(self.memory.set(self.sp, val));

			// Adjust stack pointer
			let native_sp : usize = try!(NumCast::from(self.sp).ok_or(StackError::StackPointerTooLarge));
			let new_val = native_sp - size_of::<I>();
			self.sp = try!(FromPrimitive::from_uint(new_val).ok_or(StackError::StackPointerTooLarge));

			Ok(())
		}
	}

	pub fn pop<I: Int>(&mut self) -> Result<I, StackError> {
		if self.pop_boundary.map_or(false, |b| self.sp >= b) {
			Err(StackError::ReachedPopBoundary)
		}
		else {
			// Adjust stack pointer
			let native_sp : usize = try!(NumCast::from(self.sp).ok_or(StackError::StackPointerTooLarge));
			let new_val = native_sp + size_of::<I>();
			self.sp = try!(FromPrimitive::from_uint(new_val).ok_or(StackError::StackPointerTooLarge));

			Ok(try!(self.memory.get(self.sp)))
		}
	}
}

#[cfg(test)]
mod test {
	use mem;
	use mem::Memory;
	use stack::{Stack,StackError};

	#[test]
	pub fn push_returns_error_if_at_push_boundary() {
		let mut memory = mem::FixedMemory::with_size(16u8);
		let mut stack = Stack::with_boundaries(&mut memory, 2, Option::Some(0x2), Option::None);
		assert_eq!(stack.push(10).unwrap_err(), StackError::ReachedPushBoundary);
	}

	#[test]
	pub fn push_u8_writes_u8_value_to_current_sp_location() {
		let mut memory = mem::FixedMemory::with_size(16u8); // Make a 16-byte memory
		let mut stack = Stack::new(&mut memory, 15); // Start the stack at the end of memory

		assert!(stack.push(42u8).is_ok());
		assert_eq!(stack.memory.get::<u8>(15).unwrap(), 42);
	}

	#[test]
	pub fn push_u8_moves_stack_pointer_down_1_byte() {
		let mut memory = mem::FixedMemory::with_size(16u8); // Make a 16-byte memory
		let mut stack = Stack::new(&mut memory, 15); // Start the stack at the end of memory

		assert!(stack.push(42u8).is_ok());
		assert_eq!(stack.sp, 14);
	}

	#[test]
	pub fn push_u16_writes_u16_value_to_current_sp_location() {
		let mut memory = mem::FixedMemory::with_size(16u8); // Make a 16-byte memory
		let mut stack = Stack::new(&mut memory, 15); // Start the stack at the end of memory

		assert!(stack.push(1024u16).is_ok());
		assert_eq!(stack.memory.get::<u16>(15).unwrap(), 1024u16);
	}

	#[test]
	pub fn push_u16_moves_stack_pointer_down_2_bytes() {
		let mut memory = mem::FixedMemory::with_size(16u8); // Make a 16-byte memory
		let mut stack = Stack::new(&mut memory, 15); // Start the stack at the end of memory

		assert!(stack.push(1024u16).is_ok());
		assert_eq!(stack.sp, 13);
	}

	#[test]
	pub fn push_u32_writes_u32_value_to_current_sp_location() {
		let mut memory = mem::FixedMemory::with_size(16u8); // Make a 16-byte memory
		let mut stack = Stack::new(&mut memory, 15); // Start the stack at the end of memory

		assert!(stack.push(75536u32).is_ok());
		assert_eq!(stack.memory.get::<u32>(15).unwrap(), 75536u32);
	}

	#[test]
	pub fn push_u32_moves_stack_pointer_down_4_bytes() {
		let mut memory = mem::FixedMemory::with_size(16u8); // Make a 16-byte memory
		let mut stack = Stack::new(&mut memory, 15); // Start the stack at the end of memory

		assert!(stack.push(75536u32).is_ok());
		assert_eq!(stack.sp, 11);
	}

	#[test]
	pub fn push_i8_writes_i8_value_to_current_sp_location() {
		let mut memory = mem::FixedMemory::with_size(16u8); // Make a 16-byte memory
		let mut stack = Stack::new(&mut memory, 15); // Start the stack at the end of memory

		assert!(stack.push(42i8).is_ok());
		assert_eq!(stack.memory.get::<i8>(15).unwrap(), 42);
	}

	#[test]
	pub fn push_i8_moves_stack_pointer_down_1_byte() {
		let mut memory = mem::FixedMemory::with_size(16u8); // Make a 16-byte memory
		let mut stack = Stack::new(&mut memory, 15); // Start the stack at the end of memory

		assert!(stack.push(42i8).is_ok());
		assert_eq!(stack.sp, 14);
	}

	#[test]
	pub fn push_i16_writes_i16_value_to_current_sp_location() {
		let mut memory = mem::FixedMemory::with_size(16u8); // Make a 16-byte memory
		let mut stack = Stack::new(&mut memory, 15); // Start the stack at the end of memory

		assert!(stack.push(1024i16).is_ok());
		assert_eq!(stack.memory.get::<i16>(15).unwrap(), 1024i16);
	}

	#[test]
	pub fn push_i16_moves_stack_pointer_down_2_bytes() {
		let mut memory = mem::FixedMemory::with_size(16u8); // Make a 16-byte memory
		let mut stack = Stack::new(&mut memory, 15); // Start the stack at the end of memory

		assert!(stack.push(1024i16).is_ok());
		assert_eq!(stack.sp, 13);
	}

	#[test]
	pub fn push_i32_writes_i32_value_to_current_sp_location() {
		let mut memory = mem::FixedMemory::with_size(16u8); // Make a 16-byte memory
		let mut stack = Stack::new(&mut memory, 15); // Start the stack at the end of memory

		assert!(stack.push(75536i32).is_ok());
		assert_eq!(stack.memory.get::<i32>(15).unwrap(), 75536i32);
	}

	#[test]
	pub fn push_i32_moves_stack_pointer_down_4_bytes() {
		let mut memory = mem::FixedMemory::with_size(16u8); // Make a 16-byte memory
		let mut stack = Stack::new(&mut memory, 15); // Start the stack at the end of memory

		assert!(stack.push(75536i32).is_ok());
		assert_eq!(stack.sp, 11);
	}

	#[test]
	pub fn pop_returns_error_if_at_push_boundary() {
		let mut memory = mem::FixedMemory::with_size(16u8);
		let mut stack = Stack::with_boundaries(&mut memory, 2, Option::None, Option::Some(0x2));
		assert_eq!(stack.pop::<u8>().unwrap_err(), StackError::ReachedPopBoundary);
	}

	#[test]
	pub fn pop_u8_reads_u8_value_from_next_sp_location() {
		let mut memory = mem::FixedMemory::with_size(16u8); // Make a 16-byte memory
		memory.set(1, 42u8);
		let mut stack = Stack::new(&mut memory, 0); // Start the stack at the end of memory

		assert_eq!(stack.pop::<u8>().unwrap(), 42);
	}

	#[test]
	pub fn pop_u8_moves_stack_pointer_up_1_byte() {
		let mut memory = mem::FixedMemory::with_size(16u8); // Make a 16-byte memory
		let mut stack = Stack::new(&mut memory, 0); // Start the stack at the end of memory

		assert!(stack.pop::<u8>().is_ok());
		assert_eq!(stack.sp, 1);
	}

	#[test]
	pub fn pop_i8_reads_i8_value_from_next_sp_location() {
		let mut memory = mem::FixedMemory::with_size(16u8); // Make a 16-byte memory
		memory.set(1, -42i8);
		let mut stack = Stack::new(&mut memory, 0); // Start the stack at the end of memory

		assert_eq!(stack.pop::<i8>().unwrap(), -42);
	}

	#[test]
	pub fn pop_i8_moves_stack_pointer_up_1_byte() {
		let mut memory = mem::FixedMemory::with_size(16u8); // Make a 16-byte memory
		let mut stack = Stack::new(&mut memory, 0); // Start the stack at the end of memory

		assert!(stack.pop::<i8>().is_ok());
		assert_eq!(stack.sp, 1);
	}

	#[test]
	pub fn pop_u16_reads_u16_value_from_next_sp_location() {
		let mut memory = mem::FixedMemory::with_size(16u8); // Make a 16-byte memory
		memory.set(2, 1024u16);
		let mut stack = Stack::new(&mut memory, 0); // Start the stack at the end of memory

		assert_eq!(stack.pop::<u16>().unwrap(), 1024);
	}

	#[test]
	pub fn pop_u16_moves_stack_pointer_up_2_bytes() {
		let mut memory = mem::FixedMemory::with_size(16u8); // Make a 16-byte memory
		let mut stack = Stack::new(&mut memory, 0); // Start the stack at the end of memory

		assert!(stack.pop::<u16>().is_ok());
		assert_eq!(stack.sp, 2);
	}

	#[test]
	pub fn pop_i16_reads_i16_value_from_next_sp_location() {
		let mut memory = mem::FixedMemory::with_size(16u8); // Make a 16-byte memory
		memory.set(2, -1024i16);
		let mut stack = Stack::new(&mut memory, 0); // Start the stack at the end of memory

		assert_eq!(stack.pop::<i16>().unwrap(), -1024);
	}

	#[test]
	pub fn pop_i16_moves_stack_pointer_up_2_bytes() {
		let mut memory = mem::FixedMemory::with_size(16u8); // Make a 16-byte memory
		let mut stack = Stack::new(&mut memory, 0); // Start the stack at the end of memory

		assert!(stack.pop::<i16>().is_ok());
		assert_eq!(stack.sp, 2);
	}

	#[test]
	pub fn pop_u32_reads_u32_value_from_next_sp_location() {
		let mut memory = mem::FixedMemory::with_size(16u8); // Make a 16-byte memory
		memory.set(4, 75536u32);
		let mut stack = Stack::new(&mut memory, 0); // Start the stack at the end of memory

		assert_eq!(stack.pop::<u32>().unwrap(), 75536);
	}

	#[test]
	pub fn pop_u32_moves_stack_pointer_up_4_bytes() {
		let mut memory = mem::FixedMemory::with_size(16u8); // Make a 16-byte memory
		let mut stack = Stack::new(&mut memory, 0); // Start the stack at the end of memory

		assert!(stack.pop::<u32>().is_ok());
		assert_eq!(stack.sp, 4);
	}

	#[test]
	pub fn pop_i32_reads_i32_value_from_next_sp_location() {
		let mut memory = mem::FixedMemory::with_size(16u8); // Make a 16-byte memory
		memory.set(4, -75536i32);
		let mut stack = Stack::new(&mut memory, 0); // Start the stack at the end of memory

		assert_eq!(stack.pop::<i32>().unwrap(), -75536);
	}

	#[test]
	pub fn pop_i32_moves_stack_pointer_up_4_bytes() {
		let mut memory = mem::FixedMemory::with_size(16u8); // Make a 16-byte memory
		let mut stack = Stack::new(&mut memory, 0); // Start the stack at the end of memory

		assert!(stack.pop::<i32>().is_ok());
		assert_eq!(stack.sp, 4);
	}
}