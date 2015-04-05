use mem::Memory;
use cpus::mos6502::exec;
use cpus::mos6502::{Operand,Mos6502,Flags};
use cpus::mos6502::exec::utils::{bcd_to_int,int_to_bcd};

pub fn exec<M>(cpu: &mut Mos6502<M>, op: Operand) -> Result<(), exec::Error> where M: Memory {
    let n = try!(op.get_u8(cpu)) as isize;
    let a = cpu.registers.a as isize;
    let c = if cpu.flags.carry() { 1 } else { 0 };

    let t = 
        if cpu.bcd_enabled && cpu.flags.intersects(Flags::BCD()) {
            let v = bcd_to_int(a) + bcd_to_int(n) + c;
            cpu.flags.set_if(Flags::CARRY(), v > 99);
            int_to_bcd(v)
        } else {
            let v = a + n + c;
            cpu.flags.set_if(Flags::CARRY(), v > 255);
            v
        };

    cpu.flags.set_if(Flags::OVERFLOW(), (a & 0x80) != (t & 0x80));
	cpu.registers.a = (t & 0xFF) as u8;
	cpu.flags.set_sign_and_zero(cpu.registers.a);
	Ok(())
}

#[cfg(test)]
mod test {
    use mem;
	use cpus::mos6502::exec::adc;
	use cpus::mos6502::{Mos6502,Operand,Flags};

	#[test]
	pub fn adc_adds_regularly_when_carry_not_set() {
		let mut cpu = Mos6502::without_memory();
		cpu.registers.a = 42;
		adc::exec(&mut cpu, Operand::Immediate(1)).unwrap();
		assert_eq!(cpu.registers.a, 43);
	}

	#[test]
	pub fn adc_adds_carry_value_when_carry_flag_is_set() {
		let mut cpu = Mos6502::without_memory();
		cpu.registers.a = 42;
		cpu.flags.set(Flags::CARRY()); // Set CARRY()
		adc::exec(&mut cpu, Operand::Immediate(1)).unwrap();
		assert_eq!(cpu.registers.a, 44);
	}

	#[test]
	pub fn adc_sets_flags_when_overflow() {
		let mut cpu = Mos6502::without_memory();
		cpu.registers.a = 42;
		adc::exec(&mut cpu, Operand::Immediate(255)).unwrap();
		assert_eq!(cpu.registers.a, 41);
		assert_eq!(cpu.flags, Flags::CARRY() | Flags::RESERVED());
	}

    #[test]
    pub fn adc_does_regular_addition_when_bcd_disabled_even_when_bcd_flag_set() {
		let mut cpu = Mos6502::without_bcd(mem::Empty);
        cpu.flags.set(Flags::BCD());
        cpu.registers.a = 0xA0;
        adc::exec(&mut cpu, Operand::Immediate(0x0A)).unwrap();
        assert_eq!(0xAA, cpu.registers.a);
    }

    #[test]
    pub fn adc_adds_bcd_when_bcd_flag_set() {
        let mut cpu = Mos6502::without_memory();
        cpu.flags.set(Flags::BCD());
        cpu.registers.a = 0x25;
        adc::exec(&mut cpu, Operand::Immediate(0x24)).unwrap();
        assert_eq!(0x49, cpu.registers.a); // 49 in bcd
    }

    #[test]
    pub fn adc_sets_carry_when_bcd_addition_overflows() {
        let mut cpu = Mos6502::without_memory();
        cpu.flags.set(Flags::BCD());
        cpu.registers.a = 0x90;
        adc::exec(&mut cpu, Operand::Immediate(0x12)).unwrap();
        assert_eq!(0x02, cpu.registers.a);
        assert!(cpu.flags.intersects(Flags::CARRY()));
    }
}
