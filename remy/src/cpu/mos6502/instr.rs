use std::error;

use mem;

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
	pub fn exec<M: mem::Memory<u16>>(self, cpu: &mut Mos6502<M>) -> Result<(), ExecError> {
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

#[cfg(test)]
mod test {
	mod instruction {
		use mem;
		use cpu::mos6502;
		use cpu::mos6502::{Instruction,Operand,Mos6502};

		#[test]
		pub fn adc_adds_regularly_when_carry_not_set() {
			let mut cpu = init_cpu();
			assert!(Instruction::ADC(Operand::Immediate(1)).exec(&mut cpu).is_ok());
			assert_eq!(cpu.registers.a, 43);
		}

		#[test]
		pub fn adc_adds_carry_value_when_carry_flag_is_set() {
			let mut cpu = init_cpu();
			cpu.registers.set_flags(mos6502::FLAGS_CARRY); // Set carry
			assert!(Instruction::ADC(Operand::Immediate(1)).exec(&mut cpu).is_ok());
			assert_eq!(cpu.registers.a, 44);	
		}

		#[test]
		pub fn adc_sets_flags_when_overflow() {
			let mut cpu = init_cpu();
			assert!(Instruction::ADC(Operand::Immediate(255)).exec(&mut cpu).is_ok());
			assert_eq!(cpu.registers.a, 41);
			assert_eq!(cpu.registers.get_flags(), mos6502::FLAGS_CARRY | mos6502::FLAGS_RESERVED);
		}

		fn init_cpu() -> Mos6502<mem::FixedMemory<u16>> {
			let mut cpu = Mos6502::with_fixed_memory(32);
			cpu.registers.a = 42;

			cpu
		}
	}
}