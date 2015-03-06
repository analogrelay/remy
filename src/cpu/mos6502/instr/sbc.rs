use mem::Memory;
use cpu::mos6502::{ExecError,Operand,Mos6502,Flags};
use cpu::mos6502::instr::utils::{bcd_to_int,int_to_bcd};

pub fn exec<M>(cpu: &mut Mos6502<M>, op: Operand) -> Result<(), ExecError> where M: Memory {
    let n = try!(op.get_u8(cpu)) as isize;
    let a = cpu.registers.a as isize;
    let b = if cpu.flags.carry() { 0 } else { 1 };

    let t = 
        if cpu.bcd_enabled && cpu.flags.intersects(Flags::BCD()) {
            let v = bcd_to_int(a) - bcd_to_int(n) - b;
            cpu.flags.set_if(Flags::OVERFLOW(), v > 99 || v < 0);
            int_to_bcd(v)
        } else {
            let v = a - n - b;
            cpu.flags.set_if(Flags::OVERFLOW(), v > 127 || v < -128);
            v
        };

    cpu.flags.set_if(Flags::CARRY(), t >= 0); 
	cpu.registers.a = (t & 0xFF) as u8;
	cpu.flags.set_sign_and_zero(cpu.registers.a);
	Ok(())
}

#[cfg(test)]
mod test {
    use mem::VirtualMemory;
	use cpu::mos6502::instr::sbc;
	use cpu::mos6502::{Mos6502,Operand,Flags};

    // The "Borrow" psuedo-flag is defined as !Carry
    // Thus, when Carry is SET, NO Borrow is performed
    // When Carry is CLEAR, A Borrow is performed

	#[test]
	pub fn sbc_subtracts_regularly_when_carry_set() {
		let mut cpu = init_cpu();
		cpu.flags.set(Flags::CARRY()); // Set CARRY()
		sbc::exec(&mut cpu, Operand::Immediate(1)).unwrap();
		assert_eq!(cpu.registers.a, 41);
	}

	#[test]
	pub fn sbc_borrows_when_carry_flag_is_not_set() {
		let mut cpu = init_cpu();
		sbc::exec(&mut cpu, Operand::Immediate(1)).unwrap();
		assert_eq!(cpu.registers.a, 40);
	}

	#[test]
	pub fn sbc_sets_flags_when_overflow() {
		let mut cpu = init_cpu();
		sbc::exec(&mut cpu, Operand::Immediate(172)).unwrap();
		assert_eq!(cpu.registers.a, -131);
		assert_eq!(cpu.flags, Flags::OVERFLOW() | Flags::RESERVED());
	}

    #[test]
    pub fn sbc_does_regular_subtraction_when_bcd_disabled_even_when_bcd_flag_set() {
		let vm = VirtualMemory::new();
		let mut cpu = Mos6502::without_bcd(vm);
        cpu.flags.set(Flags::BCD() | Flags::CARRY());
        cpu.registers.a = 42;
        sbc::exec(&mut cpu, Operand::Immediate(1)).unwrap();
        assert_eq!(41, cpu.registers.a);
    }

    #[test]
    pub fn sbc_subtracts_bcd_when_bcd_flag_set() {
        let mut cpu = init_cpu();
        cpu.flags.set(Flags::BCD() | Flags::CARRY());
        cpu.registers.a = 0x25;
        sbc::exec(&mut cpu, Operand::Immediate(0x24)).unwrap();
        assert_eq!(0x01, cpu.registers.a);
    }

    #[test]
    pub fn sbc_sets_overflow_when_bcd_subtraction_underflows() {
        let mut cpu = init_cpu();
        cpu.flags.set(Flags::BCD() | Flags::CARRY());
        cpu.registers.a = 0x00;
        sbc::exec(&mut cpu, Operand::Immediate(0x01)).unwrap();
        assert_eq!(0x99, cpu.registers.a);
        assert!(cpu.flags.intersects(Flags::CARRY()));
    }

	fn init_cpu() -> Mos6502<VirtualMemory<'static>> {
		let vm = VirtualMemory::new();
		let mut cpu = Mos6502::new(vm);
		cpu.registers.a = 42;
		cpu
	}
}
