use cpu::mos6502;
use cpu::mos6502::Mos6502;

use mem;

use std::error;

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

pub enum Instruction {
	ADC(Operand),
	AND(Operand),
	ASL(Operand),
	BCC(Operand),
	BCS(Operand),
	BEQ(Operand),
	BIT(Operand),
	BMI(Operand),
	BNE(Operand),
	BPL(Operand),
	BRK,
	BVC(Operand),
	BVS(Operand),
	CLC,
	CLD,
	CLI,
	CLV,
	CMP(Operand),
	CPX(Operand),
	CPY(Operand),
	DEC(Operand),
	DEX,
	DEY,
	EOR(Operand),
	INC(Operand),
	INX,
	INY,
	JMP(Operand),
	JSR(Operand),
	LDA(Operand),
	LDX(Operand),
	LDY(Operand),
	LSR(Operand),
	NOP,
	ORA(Operand),
	PHA,
	PHP,
	PLA,
	PLP,
	ROL(Operand),
	ROR(Operand),
	RTI,
	RTS,
	SBC(Operand),
	SEC,
	SED,
	SEI,
	STA(Operand),
	STX(Operand),
	STY(Operand),
	TAX,
	TAY,
	TSX,
	TXA,
	TXS,
	TYA,
}

#[derive(Show)]
pub enum ExecError {
	FailedToRetrieveOperand(OperandError),
	InstructionNotImplemented
}

impl error::FromError<OperandError> for ExecError {
	fn from_error(err: OperandError) -> ExecError {
		ExecError::FailedToRetrieveOperand(err)
	}
}

impl Instruction {
	pub fn exec(self, cpu: &mut Mos6502) -> Result<(), ExecError> {
		match self {
			Instruction::ADC(op) => {
				let (a, c) = ::util::add_u8_with_carry(cpu.registers.a, try!(op.get(cpu)), cpu.registers.carry());
				cpu.registers.a = a;
				cpu.registers.set_arith_flags(a, c);
				Ok(())
			}
			_ => Err(ExecError::InstructionNotImplemented)
		}
	}
}