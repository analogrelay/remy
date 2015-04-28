use mem::Memory;
use cpus::mos6502::exec;
use cpus::mos6502::{Operand,Mos6502,Flags};

pub fn exec<M>(cpu: &mut Mos6502, mem: &mut M, op: Operand) -> Result<(), exec::Error> where M: Memory {
    let b = try!(op.get_u8(cpu, mem));
    if b & 0x80 != 0 {
        cpu.flags.set(Flags::CARRY());
    }
    let r = (b << 1) & 0xFE;
    try!(op.set_u8(cpu, mem, r));
    if r & 0x80 != 0 {
        cpu.flags.set(Flags::SIGN());
    }
    if r == 0 {
        cpu.flags.set(Flags::ZERO());
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use mem;
    use cpus::mos6502::exec::asl;
    use cpus::mos6502::{Mos6502,Operand,Flags};

    #[test]
    pub fn asl_shifts_value_left() {
        let mut cpu = Mos6502::new();
        cpu.registers.a = 0x0F;
        asl::exec(&mut cpu, &mut mem::Empty, Operand::Accumulator).unwrap();
        assert_eq!(cpu.registers.a, 0x1E);
    }

    #[test]
    pub fn asl_sets_carry_if_bit_7_is_set_before_shifting() {
        let mut cpu = Mos6502::new();
        cpu.registers.a = 0x81;
        asl::exec(&mut cpu, &mut mem::Empty, Operand::Accumulator).unwrap();
        assert_eq!(cpu.registers.a, 0x02);
        assert_eq!(cpu.flags, Flags::CARRY() | Flags::RESERVED());
    }

    #[test]
    pub fn asl_sets_sign_if_bit_7_is_set_after_shifting() {
        let mut cpu = Mos6502::new();
        cpu.registers.a = 0x40;
        asl::exec(&mut cpu, &mut mem::Empty, Operand::Accumulator).unwrap();
        assert_eq!(cpu.registers.a, 0x80);
        assert_eq!(cpu.flags, Flags::SIGN() | Flags::RESERVED());
    }

    #[test]
    pub fn asl_sets_zero_if_value_is_zero_after_shifting() {
        let mut cpu = Mos6502::new();
        cpu.registers.a = 0x00;
        asl::exec(&mut cpu, &mut mem::Empty, Operand::Accumulator).unwrap();
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.flags, Flags::ZERO() | Flags::RESERVED());
    }
}
