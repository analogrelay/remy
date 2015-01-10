use std::error;

use mem;

use cpu::mos6502;
use cpu::mos6502::Mos6502;

#[derive(Show)]
pub enum Operand {
	Accumulator,
	Immediate(u8),
	Absolute(u16),
	Indexed(u16, mos6502::RegisterName),
	Indirect(u16),
	PreIndexedIndirect(u8),
	PostIndexedIndirect(u8),
	Relative(i8)
}

#[derive(Show)]
pub enum OperandError {
	ErrorAccessingMemory(mem::MemoryError)
}

impl error::FromError<mem::MemoryError> for OperandError {
	fn from_error(err: mem::MemoryError) -> OperandError {
		OperandError::ErrorAccessingMemory(err)
	}
}

impl Operand {
	pub fn get(self, cpu: &Mos6502) -> Result<u8, OperandError> {
		Ok(try!(self.get_u16(cpu)) as u8)
	}

	pub fn get_u16(self, cpu: &Mos6502) -> Result<u16, OperandError> {
		Ok(match self {
			Operand::Accumulator =>					cpu.registers.a as u16,
			Operand::Immediate(n) => 				n as u16,
			Operand::Absolute(addr) => 				try!(cpu.mem.get_u8(addr)) as u16,
			Operand::Indexed(addr, r) =>			try!(cpu.mem.get_u8(addr + cpu.registers.get(r) as u16)) as u16,
			Operand::Indirect(addr) =>				try!(cpu.mem.get_u16(addr)) as u16,
			Operand::PreIndexedIndirect(addr) =>	try!(cpu.mem.get_u8(try!(cpu.mem.get_u16(addr as u16 + cpu.registers.x as u16)))) as u16,
			Operand::PostIndexedIndirect(addr) =>	try!(cpu.mem.get_u8(try!(cpu.mem.get_u16(addr as u16)) + cpu.registers.y as u16)) as u16,
			Operand::Relative(offset) => 			((cpu.registers.pc as isize) + (offset as isize)) as u16
		})
	}
}

