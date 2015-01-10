use std::error;

use mem;

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
	pub fn exec<M: mem::Memory<u16>>(self, cpu: &mut Mos6502<M>) -> Result<(), ExecError> {
		match self {
			Instruction::ADC(op) => {
				let (a, c) = ::util::add_u8_with_carry(cpu.registers.a, try!(op.get(cpu)), cpu.registers.carry());
				cpu.registers.a = a;
				cpu.registers.set_arith_flags(a, c);
				Ok(())
			},
			Instruction::AND(op) => {
				let opv = try!(op.get(cpu));
				let res = cpu.registers.a & opv;
				cpu.registers.a = res;
				if res == 0 {
					cpu.registers.set_flags(mos6502::FLAGS_ZERO);
				}
				else if res & 0x80 != 0 {
					cpu.registers.set_flags(mos6502::FLAGS_SIGN);
				}
				Ok(())
			},
			Instruction::ASL(op) => {
				let b = try!(op.get(cpu));
				if b & 0x80 != 0 {
					cpu.registers.set_flags(mos6502::FLAGS_CARRY);
				}
				let r = (b << 1) & 0xFE;
				op.set(cpu, r);
				if r & 0x80 != 0 {
					cpu.registers.set_flags(mos6502::FLAGS_SIGN);
				}
				if r == 0 {
					cpu.registers.set_flags(mos6502::FLAGS_ZERO);
				}
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

		#[test]
		pub fn and_ands_value_with_accumulator() {
			let mut cpu = init_cpu();
			assert!(Instruction::AND(Operand::Immediate(24)).exec(&mut cpu).is_ok());
			assert_eq!(cpu.registers.a, 42 & 24);
		}

		#[test]
		pub fn and_sets_zero_flag_if_result_is_zero() {
			let mut cpu = init_cpu();
			assert!(Instruction::AND(Operand::Immediate(0)).exec(&mut cpu).is_ok());
			assert_eq!(cpu.registers.a, 0);
			assert_eq!(cpu.registers.get_flags(), mos6502::FLAGS_ZERO | mos6502::FLAGS_RESERVED);
		}

		#[test]
		pub fn and_sets_sign_flag_if_result_has_bit_7_set() {
			let mut cpu = init_cpu();
			cpu.registers.a = 0xFF;
			assert!(Instruction::AND(Operand::Immediate(0xFF)).exec(&mut cpu).is_ok());
			assert_eq!(cpu.registers.a, 0xFF);
			assert_eq!(cpu.registers.get_flags(), mos6502::FLAGS_SIGN | mos6502::FLAGS_RESERVED);
		}

		#[test]
		pub fn asl_shifts_value_left() {
			let mut cpu = init_cpu();
			cpu.registers.a = 0x0F;
			assert!(Instruction::ASL(Operand::Accumulator).exec(&mut cpu).is_ok());
			assert_eq!(cpu.registers.a, 0x1E);
		}

		#[test]
		pub fn asl_sets_carry_if_bit_7_is_set_before_shifting() {
			let mut cpu = init_cpu();
			cpu.registers.a = 0x81;
			assert!(Instruction::ASL(Operand::Accumulator).exec(&mut cpu).is_ok());
			assert_eq!(cpu.registers.a, 0x02);
			assert_eq!(cpu.registers.get_flags(), mos6502::FLAGS_CARRY | mos6502::FLAGS_RESERVED);
		}

		#[test]
		pub fn asl_sets_sign_if_bit_7_is_set_after_shifting() {
			let mut cpu = init_cpu();
			cpu.registers.a = 0x40;
			assert!(Instruction::ASL(Operand::Accumulator).exec(&mut cpu).is_ok());
			assert_eq!(cpu.registers.a, 0x80);
			assert_eq!(cpu.registers.get_flags(), mos6502::FLAGS_SIGN | mos6502::FLAGS_RESERVED);
		}

		#[test]
		pub fn asl_sets_zero_if_value_is_zero_after_shifting() {
			let mut cpu = init_cpu();
			cpu.registers.a = 0x00;
			assert!(Instruction::ASL(Operand::Accumulator).exec(&mut cpu).is_ok());
			assert_eq!(cpu.registers.a, 0x00);
			assert_eq!(cpu.registers.get_flags(), mos6502::FLAGS_ZERO | mos6502::FLAGS_RESERVED);
		}

		fn init_cpu() -> Mos6502<mem::FixedMemory<u16>> {
			let mut cpu = Mos6502::with_fixed_memory(32);
			cpu.registers.a = 42;

			cpu
		}
	}
}