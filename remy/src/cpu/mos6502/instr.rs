use std::error;

use mem;

use cpu::mos6502;
use cpu::mos6502::{Mos6502,Operand,OperandError};

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
			Instruction::ADC(op) => {
				let (a, c) = ::util::add_u8_with_carry(cpu.registers.a, try!(op.get_u8(cpu)), cpu.registers.carry());
				cpu.registers.a = a;
				cpu.registers.set_arith_flags(a as isize, c);
				Ok(())
			},
			Instruction::AND(op) => {
				let opv = try!(op.get_u8(cpu));
				let res = cpu.registers.a & opv;
				cpu.registers.a = res;
				if res == 0 {
					cpu.registers.set_flags(mos6502::Flags::ZERO());
				}
				else if res & 0x80 != 0 {
					cpu.registers.set_flags(mos6502::Flags::SIGN());
				}
				Ok(())
			},
			Instruction::ASL(op) => {
				let b = try!(op.get_u8(cpu));
				if b & 0x80 != 0 {
					cpu.registers.set_flags(mos6502::Flags::CARRY());
				}
				let r = (b << 1) & 0xFE;
				try!(op.set_u8(cpu, r));
				if r & 0x80 != 0 {
					cpu.registers.set_flags(mos6502::Flags::SIGN());
				}
				if r == 0 {
					cpu.registers.set_flags(mos6502::Flags::ZERO());
				}
				Ok(())
			},
			Instruction::BCC(offset) => {
				if !cpu.registers.has_flags(mos6502::Flags::CARRY()) {
					cpu.pc.advance(offset as isize)
				}
				Ok(())
			},
			Instruction::BCS(offset) => {
				if cpu.registers.has_flags(mos6502::Flags::CARRY()) {
					cpu.pc.advance(offset as isize)
				}
				Ok(())
			},
			Instruction::BEQ(offset) => {
				if cpu.registers.has_flags(mos6502::Flags::ZERO()) {
					cpu.pc.advance(offset as isize)
				}
				Ok(())
			},
			Instruction::BIT(op) => {
				let m = try!(op.get_u8(cpu));
				let t = cpu.registers.a & m;

				if m & 0x80 != 0 {
					cpu.registers.set_flags(mos6502::Flags::SIGN());
				} else {
					cpu.registers.clear_flags(mos6502::Flags::SIGN());
				}

				if m & 0x40 != 0 {
					cpu.registers.set_flags(mos6502::Flags::OVERFLOW());
				} else {
					cpu.registers.clear_flags(mos6502::Flags::OVERFLOW());
				}

				if t == 0 {
					cpu.registers.set_flags(mos6502::Flags::ZERO());
				} else {
					cpu.registers.clear_flags(mos6502::Flags::ZERO());
				}

				Ok(())
			},
			Instruction::BMI(offset) => {
				if cpu.registers.has_flags(mos6502::Flags::SIGN()) {
					cpu.pc.advance(offset as isize)
				}
				Ok(())
			},
			Instruction::BNE(offset) => {
				if !cpu.registers.has_flags(mos6502::Flags::ZERO()) {
					cpu.pc.advance(offset as isize)
				}
				Ok(())
			},
			Instruction::BPL(offset) => {
				if !cpu.registers.has_flags(mos6502::Flags::SIGN()) {
					cpu.pc.advance(offset as isize)
				}
				Ok(())
			},
			Instruction::BRK => {
				cpu.pc.advance(1);
				let pc = cpu.pc.get();
				try!(cpu.push(((pc & 0xFF00) >> 8) as u8));
				try!(cpu.push((pc & 0x00FF) as u8));

				let new_flags = cpu.registers.get_flags() | mos6502::Flags::BREAK();
				try!(cpu.push(new_flags.bits()));

				cpu.pc.set(try!(cpu.mem.get_le_u16(0xFFFE)) as usize);
				Ok(())
			},
			_ => Err(ExecError::UnknownInstruction)
		}
	}
}

#[cfg(test)]
mod test {
	mod mos6502_instruction {
		use mem;
		use mem::Memory;
		use cpu::mos6502;
		use cpu::mos6502::{Instruction,Operand,Mos6502};
		use cpu::mos6502::cpu::STACK_START;

		#[test]
		pub fn adc_adds_regularly_when_carry_not_set() {
			let mut cpu = init_cpu();
			Instruction::ADC(Operand::Immediate(1)).exec(&mut cpu).unwrap();
			assert_eq!(cpu.registers.a, 43);
		}

		#[test]
		pub fn adc_adds_carry_value_when_carry_flag_is_set() {
			let mut cpu = init_cpu();
			cpu.registers.set_flags(mos6502::Flags::CARRY()); // Set CARRY()
			Instruction::ADC(Operand::Immediate(1)).exec(&mut cpu).unwrap();
			assert_eq!(cpu.registers.a, 44);
		}

		#[test]
		pub fn adc_sets_flags_when_overflow() {
			let mut cpu = init_cpu();
			Instruction::ADC(Operand::Immediate(255)).exec(&mut cpu).unwrap();
			assert_eq!(cpu.registers.a, 41);
			assert_eq!(cpu.registers.get_flags(), mos6502::Flags::CARRY() | mos6502::Flags::RESERVED());
		}

		#[test]
		pub fn and_ands_value_with_accumulator() {
			let mut cpu = init_cpu();
			Instruction::AND(Operand::Immediate(24)).exec(&mut cpu).unwrap();
			assert_eq!(cpu.registers.a, 42 & 24);
		}

		#[test]
		pub fn and_sets_zero_flag_if_result_is_zero() {
			let mut cpu = init_cpu();
			Instruction::AND(Operand::Immediate(0)).exec(&mut cpu).unwrap();
			assert_eq!(cpu.registers.a, 0);
			assert_eq!(cpu.registers.get_flags(), mos6502::Flags::ZERO() | mos6502::Flags::RESERVED());
		}

		#[test]
		pub fn and_sets_sign_flag_if_result_has_bit_7_set() {
			let mut cpu = init_cpu();
			cpu.registers.a = 0xFF;
			Instruction::AND(Operand::Immediate(0xFF)).exec(&mut cpu).unwrap();
			assert_eq!(cpu.registers.a, 0xFF);
			assert_eq!(cpu.registers.get_flags(), mos6502::Flags::SIGN() | mos6502::Flags::RESERVED());
		}

		#[test]
		pub fn asl_shifts_value_left() {
			let mut cpu = init_cpu();
			cpu.registers.a = 0x0F;
			Instruction::ASL(Operand::Accumulator).exec(&mut cpu).unwrap();
			assert_eq!(cpu.registers.a, 0x1E);
		}

		#[test]
		pub fn asl_sets_carry_if_bit_7_is_set_before_shifting() {
			let mut cpu = init_cpu();
			cpu.registers.a = 0x81;
			Instruction::ASL(Operand::Accumulator).exec(&mut cpu).unwrap();
			assert_eq!(cpu.registers.a, 0x02);
			assert_eq!(cpu.registers.get_flags(), mos6502::Flags::CARRY() | mos6502::Flags::RESERVED());
		}

		#[test]
		pub fn asl_sets_sign_if_bit_7_is_set_after_shifting() {
			let mut cpu = init_cpu();
			cpu.registers.a = 0x40;
			Instruction::ASL(Operand::Accumulator).exec(&mut cpu).unwrap();
			assert_eq!(cpu.registers.a, 0x80);
			assert_eq!(cpu.registers.get_flags(), mos6502::Flags::SIGN() | mos6502::Flags::RESERVED());
		}

		#[test]
		pub fn asl_sets_zero_if_value_is_zero_after_shifting() {
			let mut cpu = init_cpu();
			cpu.registers.a = 0x00;
			Instruction::ASL(Operand::Accumulator).exec(&mut cpu).unwrap();
			assert_eq!(cpu.registers.a, 0x00);
			assert_eq!(cpu.registers.get_flags(), mos6502::Flags::ZERO() | mos6502::Flags::RESERVED());
		}

		#[test]
		pub fn bcc_does_not_modify_pc_if_carry_flag_set() {
			let mut cpu = init_cpu();
			cpu.registers.set_flags(mos6502::Flags::CARRY());
			Instruction::BCC(1).exec(&mut cpu).unwrap();
			assert_eq!(cpu.pc.get(), 0xABCD);
		}

		#[test]
		pub fn bcc_advances_pc_by_specified_amount_if_carry_flag_clear() {
			let mut cpu = init_cpu();
			Instruction::BCC(1).exec(&mut cpu).unwrap();
			assert_eq!(cpu.pc.get(), 0xABCE);
		}

		#[test]
		pub fn bcs_does_not_modify_pc_if_carry_flag_unset() {
			let mut cpu = init_cpu();
			Instruction::BCS(1).exec(&mut cpu).unwrap();
			assert_eq!(cpu.pc.get(), 0xABCD);
		}

		#[test]
		pub fn bcs_advances_pc_by_specified_amount_if_carry_flag_set() {
			let mut cpu = init_cpu();
			cpu.registers.set_flags(mos6502::Flags::CARRY());
			Instruction::BCS(1).exec(&mut cpu).unwrap();
			assert_eq!(cpu.pc.get(), 0xABCE);
		}

		#[test]
		pub fn beq_advances_pc_by_specified_amount_if_zero_flag_set() {
			let mut cpu = init_cpu();
			cpu.registers.set_flags(mos6502::Flags::ZERO());
			Instruction::BEQ(1).exec(&mut cpu).unwrap();
			assert_eq!(cpu.pc.get(), 0xABCE);
		}

		#[test]
		pub fn beq_does_not_modify_pc_if_zero_flag_unset() {
			let mut cpu = init_cpu();
			Instruction::BEQ(1).exec(&mut cpu).unwrap();
			assert_eq!(cpu.pc.get(), 0xABCD);
		}

		#[test]
		pub fn bit_sets_sign_bit_if_bit_7_of_operand_is_set() {
			let mut cpu = init_cpu();
			cpu.registers.a = 0xFF;
			Instruction::BIT(Operand::Immediate(0x80)).exec(&mut cpu).unwrap();
			assert_eq!(cpu.registers.get_flags(), mos6502::Flags::SIGN() | mos6502::Flags::RESERVED());
		}

		#[test]
		pub fn bit_clears_sign_bit_if_bit_7_of_operand_is_not_set() {
			let mut cpu = init_cpu();
			cpu.registers.a = 0xFF;
			cpu.registers.set_flags(mos6502::Flags::SIGN() | mos6502::Flags::RESERVED());
			Instruction::BIT(Operand::Immediate(0x01)).exec(&mut cpu).unwrap();
			assert_eq!(cpu.registers.get_flags(), mos6502::Flags::RESERVED());
		}

		#[test]
		pub fn bit_sets_overflow_bit_if_bit_6_of_operand_is_set() {
			let mut cpu = init_cpu();
			cpu.registers.a = 0xFF;
			Instruction::BIT(Operand::Immediate(0x40)).exec(&mut cpu).unwrap();
			assert_eq!(cpu.registers.get_flags(), mos6502::Flags::OVERFLOW() | mos6502::Flags::RESERVED());
		}

		#[test]
		pub fn bit_clears_overflow_bit_if_bit_6_of_operand_is_not_set() {
			let mut cpu = init_cpu();
			cpu.registers.a = 0xFF;
			cpu.registers.set_flags(mos6502::Flags::OVERFLOW() | mos6502::Flags::RESERVED());
			Instruction::BIT(Operand::Immediate(0x01)).exec(&mut cpu).unwrap();
			assert_eq!(cpu.registers.get_flags(), mos6502::Flags::RESERVED());
		}

		#[test]
		pub fn bit_sets_zero_flag_if_result_of_masking_operand_with_a_is_zero() {
			let mut cpu = init_cpu();
			cpu.registers.a = 0x02;
			Instruction::BIT(Operand::Immediate(0x01)).exec(&mut cpu).unwrap();
			assert_eq!(cpu.registers.get_flags(), mos6502::Flags::ZERO() | mos6502::Flags::RESERVED());
		}

		#[test]
		pub fn bit_clears_zero_flag_if_result_of_masking_operand_with_a_is_nonzero() {
			let mut cpu = init_cpu();
			cpu.registers.a = 0x02;
			cpu.registers.set_flags(mos6502::Flags::ZERO() | mos6502::Flags::RESERVED());
			Instruction::BIT(Operand::Immediate(0x03)).exec(&mut cpu).unwrap();
			assert_eq!(cpu.registers.get_flags(), mos6502::Flags::RESERVED());
		}

		#[test]
		pub fn bmi_advances_pc_by_specified_amount_if_sign_flag_set() {
			let mut cpu = init_cpu();
			cpu.registers.set_flags(mos6502::Flags::SIGN());
			Instruction::BMI(1).exec(&mut cpu).unwrap();
			assert_eq!(cpu.pc.get(), 0xABCE);
		}

		#[test]
		pub fn bmi_does_not_modify_pc_if_sign_flag_unset() {
			let mut cpu = init_cpu();
			Instruction::BMI(1).exec(&mut cpu).unwrap();
			assert_eq!(cpu.pc.get(), 0xABCD);
		}

		#[test]
		pub fn bne_advances_pc_by_specified_amount_if_zero_flag_unset() {
			let mut cpu = init_cpu();
			Instruction::BNE(1).exec(&mut cpu).unwrap();
			assert_eq!(cpu.pc.get(), 0xABCE);
		}

		#[test]
		pub fn bne_does_not_modify_pc_if_zero_flag_set() {
			let mut cpu = init_cpu();
			cpu.registers.set_flags(mos6502::Flags::ZERO());
			Instruction::BNE(1).exec(&mut cpu).unwrap();
			assert_eq!(cpu.pc.get(), 0xABCD);
		}

		#[test]
		pub fn bpl_advances_pc_by_specified_amount_if_sign_flag_unset() {
			let mut cpu = init_cpu();
			Instruction::BPL(1).exec(&mut cpu).unwrap();
			assert_eq!(cpu.pc.get(), 0xABCE);
		}

		#[test]
		pub fn bpl_does_not_modify_pc_if_sign_flag_set() {
			let mut cpu = init_cpu();
			cpu.registers.set_flags(mos6502::Flags::SIGN());
			Instruction::BPL(1).exec(&mut cpu).unwrap();
			assert_eq!(cpu.pc.get(), 0xABCD);
		}

		#[test]
		pub fn brk_increments_and_pushes_pc_on_to_stack() {
			let mut cpu = init_cpu();
			Instruction::BRK.exec(&mut cpu).unwrap();

			assert_eq!(Ok(0xAB), cpu.mem.get_u8(STACK_START + 16));
			assert_eq!(Ok(0xCE), cpu.mem.get_u8(STACK_START + 15));
		}

		#[test]
		pub fn brk_sets_break_flag_and_pushes_flags_on_to_stack() {
			let mut cpu = init_cpu();
			let flags = mos6502::Flags::SIGN() | mos6502::Flags::OVERFLOW() | mos6502::Flags::RESERVED();
			cpu.registers.set_flags(flags);
			Instruction::BRK.exec(&mut cpu).unwrap();

			assert_eq!(Ok((flags | mos6502::Flags::BREAK()).bits()), cpu.mem.get_u8(STACK_START + 14));
		}

		#[test]
		pub fn brk_does_not_set_break_flag_on_current_flags() {
			let mut cpu = init_cpu();
			let flags = mos6502::Flags::SIGN() | mos6502::Flags::OVERFLOW() | mos6502::Flags::RESERVED();
			cpu.registers.set_flags(flags);
			Instruction::BRK.exec(&mut cpu).unwrap();

			assert_eq!(flags, cpu.registers.get_flags());
		}

		#[test]
		pub fn brk_sets_pc_to_address_at_vector() {
			let mut cpu = init_cpu();
			Instruction::BRK.exec(&mut cpu).unwrap();

			assert_eq!(0xBEEF, cpu.pc.get());
		}

		fn init_cpu() -> Mos6502<mem::VirtualMemory<'static>> {
			let base_memory = mem::FixedMemory::new(32);
			let stack_memory = mem::FixedMemory::new(32);
			let vector_memory = mem::FixedMemory::new(6);
			let mut vm = mem::VirtualMemory::new();
			vm.attach(0, Box::new(base_memory)).unwrap();
			vm.attach(STACK_START, Box::new(stack_memory)).unwrap();
			vm.attach(0xFFFA, Box::new(vector_memory)).unwrap();

			let mut cpu = Mos6502::new(vm);
			cpu.registers.a = 42;
			cpu.registers.sp = 16;
			cpu.pc.set(0xABCD);
			cpu.mem.set_le_u16(0xFFFE, 0xBEEF).unwrap();

			cpu
		}
	}
}