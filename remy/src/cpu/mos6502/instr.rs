use std::error;

use mem;

use pc;

use cpu::mos6502;
use cpu::mos6502::{Mos6502,Operand,OperandError};

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
	// BRK,
	// BVC(Operand),
	// BVS(Operand),
	// CLC,
	// CLD,
	// CLI,
	// CLV,
	// CMP(Operand),
	// CPX(Operand),
	// CPY(Operand),
	// DEC(Operand),
	// DEX,
	// DEY,
	// EOR(Operand),
	// INC(Operand),
	// INX,
	// INY,
	// JMP(Operand),
	// JSR(Operand),
	// LDA(Operand),
	// LDX(Operand),
	// LDY(Operand),
	// LSR(Operand),
	// NOP,
	// ORA(Operand),
	// PHA,
	// PHP,
	// PLA,
	// PLP,
	// ROL(Operand),
	// ROR(Operand),
	// RTI,
	// RTS,
	// SBC(Operand),
	// SEC,
	// SED,
	// SEI,
	// STA(Operand),
	// STX(Operand),
	// STY(Operand),
	// TAX,
	// TAY,
	// TSX,
	// TXA,
	// TXS,
	// TYA,
}

#[derive(Show)]
pub enum ExecError {
	FailedToRetrieveOperand(OperandError),
	ErrorAdjustingProgramCounter(pc::ProgramCounterError)
}

impl error::FromError<OperandError> for ExecError {
	fn from_error(err: OperandError) -> ExecError {
		ExecError::FailedToRetrieveOperand(err)
	}
}

impl error::FromError<pc::ProgramCounterError> for ExecError {
	fn from_error(err: pc::ProgramCounterError) -> ExecError {
		ExecError::ErrorAdjustingProgramCounter(err)
	}
}

impl Instruction {
	pub fn exec<M>(self, cpu: &mut Mos6502<M>) -> Result<(), ExecError> where M: mem::Memory {
		match self {
			Instruction::ADC(op) => {
				let (a, c) = ::util::add_u8_with_carry(cpu.registers.a, try!(op.get_u8(cpu)), cpu.registers.carry());
				cpu.registers.a = a;
				cpu.registers.set_arith_flags(a, c);
				Ok(())
			},
			Instruction::AND(op) => {
				let opv = try!(op.get_u8(cpu));
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
				let b = try!(op.get_u8(cpu));
				if b & 0x80 != 0 {
					cpu.registers.set_flags(mos6502::FLAGS_CARRY);
				}
				let r = (b << 1) & 0xFE;
				try!(op.set_u8(cpu, r));
				if r & 0x80 != 0 {
					cpu.registers.set_flags(mos6502::FLAGS_SIGN);
				}
				if r == 0 {
					cpu.registers.set_flags(mos6502::FLAGS_ZERO);
				}
				Ok(())
			},
			Instruction::BCC(offset) => {
				if !cpu.registers.has_flags(mos6502::FLAGS_CARRY) {
					try!(cpu.pc.advance(offset))
				}
				Ok(())
			},
			Instruction::BCS(offset) => {
				if cpu.registers.has_flags(mos6502::FLAGS_CARRY) {
					try!(cpu.pc.advance(offset))
				}
				Ok(())
			},
			Instruction::BEQ(offset) => {
				if cpu.registers.has_flags(mos6502::FLAGS_ZERO) {
					try!(cpu.pc.advance(offset))
				}
				Ok(())
			},
			Instruction::BIT(op) => {
				let m = try!(op.get_u8(cpu));
				let t = cpu.registers.a & m;

				if m & 0x80 != 0 {
					cpu.registers.set_flags(mos6502::FLAGS_SIGN);
				} else {
					cpu.registers.clear_flags(mos6502::FLAGS_SIGN);
				}

				if m & 0x40 != 0 {
					cpu.registers.set_flags(mos6502::FLAGS_OVERFLOW);
				} else {
					cpu.registers.clear_flags(mos6502::FLAGS_OVERFLOW);
				}

				if t == 0 {
					cpu.registers.set_flags(mos6502::FLAGS_ZERO);
				} else {
					cpu.registers.clear_flags(mos6502::FLAGS_ZERO);
				}

				Ok(())
			},
			Instruction::BMI(offset) => {
				if cpu.registers.has_flags(mos6502::FLAGS_SIGN) {
					try!(cpu.pc.advance(offset))
				}
				Ok(())
			},
			Instruction::BNE(offset) => {
				if !cpu.registers.has_flags(mos6502::FLAGS_ZERO) {
					try!(cpu.pc.advance(offset))
				}
				Ok(())
			},
			Instruction::BPL(offset) => {
				if !cpu.registers.has_flags(mos6502::FLAGS_SIGN) {
					try!(cpu.pc.advance(offset))
				}
				Ok(())
			}
		}
	}
}

#[cfg(test)]
mod test {
	mod mos6502_instruction {
		use pc;
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

		#[test]
		pub fn bcc_does_not_modify_pc_if_carry_flag_set() {
			let mut cpu = init_cpu();
			cpu.registers.set_flags(mos6502::FLAGS_CARRY);
			assert!(Instruction::BCC(1).exec(&mut cpu).is_ok());
			assert_eq!(cpu.pc.get(), 42);
		}

		#[test]
		pub fn bcc_advances_pc_by_specified_amount_if_carry_flag_clear() {
			let mut cpu = init_cpu();
			assert!(Instruction::BCC(1).exec(&mut cpu).is_ok());
			assert_eq!(cpu.pc.get(), 43);
		}

		#[test]
		pub fn bcs_does_not_modify_pc_if_carry_flag_unset() {
			let mut cpu = init_cpu();
			assert!(Instruction::BCS(1).exec(&mut cpu).is_ok());
			assert_eq!(cpu.pc.get(), 42);
		}

		#[test]
		pub fn bcs_advances_pc_by_specified_amount_if_carry_flag_set() {
			let mut cpu = init_cpu();
			cpu.registers.set_flags(mos6502::FLAGS_CARRY);
			assert!(Instruction::BCS(1).exec(&mut cpu).is_ok());
			assert_eq!(cpu.pc.get(), 43);
		}

		#[test]
		pub fn beq_advances_pc_by_specified_amount_if_zero_flag_set() {
			let mut cpu = init_cpu();
			cpu.registers.set_flags(mos6502::FLAGS_ZERO);
			assert!(Instruction::BEQ(1).exec(&mut cpu).is_ok());
			assert_eq!(cpu.pc.get(), 43);
		}

		#[test]
		pub fn beq_does_not_modify_pc_if_zero_flag_unset() {
			let mut cpu = init_cpu();
			assert!(Instruction::BEQ(1).exec(&mut cpu).is_ok());
			assert_eq!(cpu.pc.get(), 42);
		}

		#[test]
		pub fn bit_sets_sign_bit_if_bit_7_of_operand_is_set() {
			let mut cpu = init_cpu();
			cpu.registers.a = 0xFF;
			assert!(Instruction::BIT(Operand::Immediate(0x80)).exec(&mut cpu).is_ok());
			assert_eq!(cpu.registers.get_flags(), mos6502::FLAGS_SIGN | mos6502::FLAGS_RESERVED);
		}

		#[test]
		pub fn bit_clears_sign_bit_if_bit_7_of_operand_is_not_set() {
			let mut cpu = init_cpu();
			cpu.registers.a = 0xFF;
			cpu.registers.set_flags(mos6502::FLAGS_SIGN | mos6502::FLAGS_RESERVED);
			assert!(Instruction::BIT(Operand::Immediate(0x01)).exec(&mut cpu).is_ok());
			assert_eq!(cpu.registers.get_flags(), mos6502::FLAGS_RESERVED);
		}

		#[test]
		pub fn bit_sets_overflow_bit_if_bit_6_of_operand_is_set() {
			let mut cpu = init_cpu();
			cpu.registers.a = 0xFF;
			assert!(Instruction::BIT(Operand::Immediate(0x40)).exec(&mut cpu).is_ok());
			assert_eq!(cpu.registers.get_flags(), mos6502::FLAGS_OVERFLOW | mos6502::FLAGS_RESERVED);
		}

		#[test]
		pub fn bit_clears_overflow_bit_if_bit_6_of_operand_is_not_set() {
			let mut cpu = init_cpu();
			cpu.registers.a = 0xFF;
			cpu.registers.set_flags(mos6502::FLAGS_OVERFLOW | mos6502::FLAGS_RESERVED);
			assert!(Instruction::BIT(Operand::Immediate(0x01)).exec(&mut cpu).is_ok());
			assert_eq!(cpu.registers.get_flags(), mos6502::FLAGS_RESERVED);
		}

		#[test]
		pub fn bit_sets_zero_flag_if_result_of_masking_operand_with_a_is_zero() {
			let mut cpu = init_cpu();
			cpu.registers.a = 0x02;
			assert!(Instruction::BIT(Operand::Immediate(0x01)).exec(&mut cpu).is_ok());
			assert_eq!(cpu.registers.get_flags(), mos6502::FLAGS_ZERO | mos6502::FLAGS_RESERVED);
		}

		#[test]
		pub fn bit_clears_zero_flag_if_result_of_masking_operand_with_a_is_nonzero() {
			let mut cpu = init_cpu();
			cpu.registers.a = 0x02;
			cpu.registers.set_flags(mos6502::FLAGS_ZERO | mos6502::FLAGS_RESERVED);
			assert!(Instruction::BIT(Operand::Immediate(0x03)).exec(&mut cpu).is_ok());
			assert_eq!(cpu.registers.get_flags(), mos6502::FLAGS_RESERVED);
		}

		#[test]
		pub fn bmi_advances_pc_by_specified_amount_if_sign_flag_set() {
			let mut cpu = init_cpu();
			cpu.registers.set_flags(mos6502::FLAGS_SIGN);
			assert!(Instruction::BMI(1).exec(&mut cpu).is_ok());
			assert_eq!(cpu.pc.get(), 43);
		}

		#[test]
		pub fn bmi_does_not_modify_pc_if_sign_flag_unset() {
			let mut cpu = init_cpu();
			assert!(Instruction::BMI(1).exec(&mut cpu).is_ok());
			assert_eq!(cpu.pc.get(), 42);
		}

		#[test]
		pub fn bne_advances_pc_by_specified_amount_if_zero_flag_unset() {
			let mut cpu = init_cpu();
			assert!(Instruction::BNE(1).exec(&mut cpu).is_ok());
			assert_eq!(cpu.pc.get(), 43);
		}

		#[test]
		pub fn bne_does_not_modify_pc_if_zero_flag_set() {
			let mut cpu = init_cpu();
			cpu.registers.set_flags(mos6502::FLAGS_ZERO);
			assert!(Instruction::BNE(1).exec(&mut cpu).is_ok());
			assert_eq!(cpu.pc.get(), 42);
		}

		#[test]
		pub fn bpl_advances_pc_by_specified_amount_if_sign_flag_unset() {
			let mut cpu = init_cpu();
			assert!(Instruction::BPL(1).exec(&mut cpu).is_ok());
			assert_eq!(cpu.pc.get(), 43);
		}

		#[test]
		pub fn bpl_does_not_modify_pc_if_sign_flag_set() {
			let mut cpu = init_cpu();
			cpu.registers.set_flags(mos6502::FLAGS_SIGN);
			assert!(Instruction::BPL(1).exec(&mut cpu).is_ok());
			assert_eq!(cpu.pc.get(), 42);
		}

		fn init_cpu() -> Mos6502<mem::FixedMemory> {
			let mut cpu = Mos6502::with_fixed_memory(32);
			cpu.registers.a = 42;
			cpu.pc.set(42);

			cpu
		}
	}
}