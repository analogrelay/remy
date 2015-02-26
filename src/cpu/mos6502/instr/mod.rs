use std::error;

use mem;

use cpu::mos6502::{Mos6502,Operand,OperandError,RegisterName,Flags};

mod adc;
mod and;
mod asl;
mod bit;
mod branch;
mod brk;
mod clear_flag;
mod compare;
mod dec;
mod eor;
mod inc;
mod jmp;
mod jsr;
mod load;
mod lsr;

#[derive(Copy,Debug,Eq,PartialEq)]
pub enum Instruction {
	ADC(Operand),
	AND(Operand),
	ASL(Operand),
    BranchIfClear(i8, Flags),
    BranchIfSet(i8, Flags),
	BIT(Operand),
	BRK,
    ClearFlag(Flags),
    Compare(RegisterName, Operand),
	DEC(Operand),
	EOR(Operand),
	INC(Operand),
	JMP(Operand),
	JSR(u16),
	Load(RegisterName, Operand),
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
    SetFlag(Flags),
    Store(RegisterName, Operand),
    Transfer(RegisterName, RegisterName)
}

#[derive(Clone,Debug,Eq,PartialEq)]
pub enum ExecError {
	ErrorRetrievingOperand(OperandError),
	ErrorReadingMemory(mem::MemoryError),
    IllegalOperand,
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
            Instruction::BranchIfClear(offset, flag_selector) => branch::if_clear(cpu, offset, flag_selector),
            Instruction::BranchIfSet(offset, flag_selector) => branch::if_set(cpu, offset, flag_selector),
			Instruction::BIT(op) => bit::exec(cpu, op), 
			Instruction::BRK => brk::exec(cpu), 
            Instruction::ClearFlag(flag_selector) => clear_flag::exec(cpu, flag_selector),
            Instruction::Compare(reg, op) => compare::exec(cpu, reg, op),
            Instruction::DEC(op) => dec::exec(cpu, op),
            Instruction::EOR(op) => eor::exec(cpu, op),
            Instruction::INC(op) => inc::exec(cpu, op),
            Instruction::JMP(op) => jmp::exec(cpu, op),
            Instruction::JSR(addr) => jsr::exec(cpu, addr),
            Instruction::Load(reg, op) => load::exec(cpu, reg, op),
            Instruction::LSR(op) => lsr::exec(cpu, op),
			_ => Err(ExecError::UnknownInstruction)
		}
	}
}
