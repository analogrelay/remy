use std::num::UnsignedInt;

use mem;

pub enum StackError {
	ErrorAccessingMemory(StackError)
	ReachedPushBoundary,
	ReachedPopBoundary
}

pub struct Stack<'a, A: UnsignedInt, M: mem::Memory<A>> {
	mem: &'a M,
	sp: A,

	// The memory location at which pushes may no longer occur (end of the stack)
	push_boundary: Option<A>,

	// The memory location at which pops may no longer occur
	pop_boundary: Option<A>
}

impl<'a, A: UnsignedInt, M: mem::Memory<A>> Stack<'a, A, M> {
	pub fn new(memory: &'a M) -> Stack<'a, A, M> {
		Stack {
			mem: memory,
			sp: UnsignedInt::zero()
		}
	}

	pub fn push<I: UnsignedInt>(&mut self, val: I) -> Result<(), StackError> {
		// Check the boundaries
		if push_boundary.map_or(false, |b| sp <= b) {
			Err(StackError::ReachedPushBoundary)
		}
		else {
			try!(mem.set(sp, val));
			sp = sp - 1;
			Ok(());
		}
	}

	pub fn pop<I: UnsignedInt>(&mut self) -> Result<I, StackError> {
		if pop_boundary.map_or(false, |b| sp >= b) {
			Err(StackError::ReachedPopBoundary)
		}
		else {
			sp = sp + 1;
			Ok(try!(mem.get(sp, val)))
		}
	}
}