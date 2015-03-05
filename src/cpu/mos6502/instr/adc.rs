use mem::Memory;
use cpu::mos6502::{ExecError,Operand,Mos6502,Flags};

pub fn exec<M>(cpu: &mut Mos6502<M>, op: Operand) -> Result<(), ExecError> where M: Memory {
    let n = try!(op.get_u8(cpu)) as isize;
    let a = cpu.registers.a as isize;
    let c = if cpu.flags.carry() { 1 } else { 0 };
    let t = n + a + c; 

    cpu.flags.set_if(Flags::OVERFLOW(), (a & 0x80) != (t & 0x80));
    cpu.flags.set_if(Flags::CARRY(), t > 255);
	cpu.registers.a = (t & 0xFF) as u8;
	cpu.flags.set_sign_and_zero(cpu.registers.a);
	Ok(())
}

#[cfg(test)]
mod test {
    use mem::VirtualMemory;
	use cpu::mos6502::instr::adc;
	use cpu::mos6502::{Mos6502,Operand,Flags};

	#[test]
	pub fn adc_adds_regularly_when_carry_not_set() {
		let mut cpu = init_cpu();
		adc::exec(&mut cpu, Operand::Immediate(1)).unwrap();
		assert_eq!(cpu.registers.a, 43);
	}

	#[test]
	pub fn adc_adds_carry_value_when_carry_flag_is_set() {
		let mut cpu = init_cpu();
		cpu.flags.set(Flags::CARRY()); // Set CARRY()
		adc::exec(&mut cpu, Operand::Immediate(1)).unwrap();
		assert_eq!(cpu.registers.a, 44);
	}

	#[test]
	pub fn adc_sets_flags_when_overflow() {
		let mut cpu = init_cpu();
		adc::exec(&mut cpu, Operand::Immediate(255)).unwrap();
		assert_eq!(cpu.registers.a, 41);
		assert_eq!(cpu.flags, Flags::CARRY() | Flags::RESERVED());
	}

	fn init_cpu() -> Mos6502<VirtualMemory<'static>> {
		let vm = VirtualMemory::new();
		let mut cpu = Mos6502::new(vm);
		cpu.registers.a = 42;
		cpu
	}
}
