use std::num::UnsignedInt;

use mem;

pub struct Stack<'a, A: UnsignedInt, M: mem::Memory<A>> {
	mem: &'a M,
	sp: A
}

impl<'a, A: UnsignedInt, M: mem::Memory<A>> Stack<'a, A, M> {
	pub fn new(memory: &'a M) -> Stack<'a, A, M> {
		Stack {
			mem: memory,
			sp: UnsignedInt::zero()
		}
	}
}