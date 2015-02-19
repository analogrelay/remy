use std::error;

use mem;

use cpu::mos6502::{Mos6502,Operand,OperandError};

mod adc;
mod and;
mod asl;
mod bcc;
mod bcs;
mod beq;
mod bit;
mod bmi;
mod bne;
mod bpl;
mod brk;
mod bvc;
mod bvs;
mod clc;
mod cld;
mod cli;
mod clv;
mod cmp;

#[derive(Copy,Debug,Eq,PartialEq)]
pub enum Instruction {
	ADC(Operand),
	AND(Operand),
	ASL(Operand),
	BCC(i8),
	BCS(i8),
	BEQ(i8),
	BIT(Operand),
	BMI(i8),
	BNE(i8),
	BPL(i8),
	BRK,
	BVC(i8),
	BVS(i8),
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

#[derive(Clone,Debug,Eq,PartialEq)]
pub enum ExecError {
	ErrorRetrievingOperand(OperandError),
	ErrorReadingMemory(mem::MemoryError),
	UnknownInstruction
}

impl error::FromError<OperandError> for ExecError {
	fn from_error(err: OperandError) -> ExecError {
		ExecError::ErrorRetrievingOperand(err)
	}
}

impl error::FromError<mem::MemoryError> for ExecError {
	fn from_error(err: mem::MemoryError) -> ExecError {
		ExecError::ErrorReadingMemory(err)
	}
}

impl Instruction {
	pub fn exec<M>(self, cpu: &mut Mos6502<M>) -> Result<(), ExecError> where M: mem::Memory {
		match self {
			Instruction::ADC(op) => adc::exec(cpu, op),
			Instruction::AND(op) => and::exec(cpu, op),
			Instruction::ASL(op) => asl::exec(cpu, op), 
			Instruction::BCC(offset) => bcc::exec(cpu, offset),
            Instruction::BCS(offset) => bcs::exec(cpu, offset), 
			Instruction::BEQ(offset) => beq::exec(cpu, offset), 
			Instruction::BIT(op) => bit::exec(cpu, op), 
			Instruction::BMI(offset) => bmi::exec(cpu, offset), 
			Instruction::BNE(offset) => bne::exec(cpu, offset), 
			Instruction::BPL(offset) => bpl::exec(cpu, offset),
			Instruction::BRK => brk::exec(cpu), 
			Instruction::BVC(offset) => bvc::exec(cpu, offset), 
			Instruction::BVS(offset) => bvs::exec(cpu, offset), 
			Instruction::CLC => clc::exec(cpu), 
			Instruction::CLD => cld::exec(cpu), 
			Instruction::CLI => cli::exec(cpu), 
			Instruction::CLV => clv::exec(cpu), 
			Instruction::CMP(op) => cmp::exec(cpu, op), 
			_ => Err(ExecError::UnknownInstruction)
		}
	}
}
