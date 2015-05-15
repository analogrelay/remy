use mem::Memory;
use hw::mos6502::exec;
use hw::mos6502::{Operand,Mos6502,Flags};

pub fn exec<M>(cpu: &mut Mos6502, mem: &M, op: Operand) -> Result<(), exec::Error> where M: Memory {
    let m = try!(op.get_u8(cpu, mem));
    let a = cpu.registers.a;
    let c = if cpu.flags.carry() { 1 } else { 0 };

    if cpu.bcd_enabled && cpu.flags.intersects(Flags::BCD()) {
        unimplemented!()
    }

    let t = (a as u16) + (m as u16) + (c as u16);
    let r = t as u8;

    cpu.flags.set_if(Flags::CARRY(), (t & 0x100) != 0);
    cpu.flags.set_if(Flags::OVERFLOW(), ((cpu.registers.a ^ m) & 0x80 == 0) && ((cpu.registers.a ^ r) & 0x80 == 0x80));

    cpu.registers.a = r;
    cpu.flags.set_sign_and_zero(cpu.registers.a);
    Ok(())
}

#[cfg(test)]
mod test {
    use mem;
    use hw::mos6502::exec::adc;
    use hw::mos6502::{Mos6502,Operand,Flags};

    #[test]
    pub fn adc_adds_regularly_when_carry_not_set() {
        let mut cpu = Mos6502::new();
        cpu.registers.a = 42;
        adc::exec(&mut cpu, &mem::Empty, Operand::Immediate(1)).unwrap();
        assert_eq!(cpu.registers.a, 43);
    }

    #[test]
    pub fn adc_adds_carry_value_when_carry_flag_is_set() {
        let mut cpu = Mos6502::new();
        cpu.registers.a = 42;
        cpu.flags.set(Flags::CARRY());
        adc::exec(&mut cpu, &mem::Empty, Operand::Immediate(1)).unwrap();
        assert_eq!(cpu.registers.a, 44);
    }

    #[test]
    pub fn adc_sets_flags_when_overflow() {
        let mut cpu = Mos6502::new();
        cpu.registers.a = 0x7F;
        adc::exec(&mut cpu, &mem::Empty, Operand::Immediate(0x80)).unwrap();
        assert_eq!(cpu.registers.a, 0xFF);
        assert_eq!(cpu.flags, Flags::SIGN() | Flags::RESERVED());
    }
}
