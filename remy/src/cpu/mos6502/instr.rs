use std::error;

use cpu::mos6502;
use cpu::mos6502::{Mos6502,Operand,OperandError};

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