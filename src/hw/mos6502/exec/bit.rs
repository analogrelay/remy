use slog;
use mem::Memory;
use hw::mos6502::exec;
use hw::mos6502::{Mos6502,Flags,Operand};

pub fn exec<M>(cpu: &mut Mos6502, mem: &M, op: Operand, log: &slog::Logger) -> Result<(), exec::Error> where M: Memory {
    let m = try_log!(op.get_u8(cpu, mem), log);
    let t = cpu.registers.a & m;

    trace!(log, "cpu" => cpu,
        "a" => cpu.registers.a,
        "m" => m,
        "r" => t,
        "op" => op;
        "evaluated a & m = r");

    if m & 0x80 != 0 {
        cpu.flags.set(Flags::SIGN());
        trace!(log, "cpu" => cpu; "setting SIGN");
    } else {
        cpu.flags.clear(Flags::SIGN());
        trace!(log, "cpu" => cpu; "clearing SIGN");
    }

    if m & 0x40 != 0 {
        cpu.flags.set(Flags::OVERFLOW());
        trace!(log, "cpu" => cpu; "setting OVERFLOW");
    } else {
        cpu.flags.clear(Flags::OVERFLOW());
        trace!(log, "cpu" => cpu; "clearing OVERFLOW");
    }

    if t == 0 {
        cpu.flags.set(Flags::ZERO());
        trace!(log, "cpu" => cpu; "setting ZERO");
    } else {
        cpu.flags.clear(Flags::ZERO());
        trace!(log, "cpu" => cpu; "clearing ZERO");
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use mem;
    use hw::mos6502::exec::bit;
    use hw::mos6502::{Mos6502,Flags,Operand};

    #[test]
    pub fn bit_sets_sign_bit_if_bit_7_of_operand_is_set() {
        let mut cpu = Mos6502::new();
        cpu.pc.set(0xABCD);
        cpu.registers.a = 0xFF;
        bit::exec(&mut cpu, &mem::Empty, Operand::Immediate(0x80)).unwrap();
        assert_eq!(cpu.flags, Flags::SIGN() | Flags::RESERVED());
    }

    #[test]
    pub fn bit_clears_sign_bit_if_bit_7_of_operand_is_not_set() {
        let mut cpu = Mos6502::new();
        cpu.pc.set(0xABCD);
        cpu.registers.a = 0xFF;
        cpu.flags.set(Flags::SIGN() | Flags::RESERVED());
        bit::exec(&mut cpu, &mem::Empty, Operand::Immediate(0x01)).unwrap();
        assert_eq!(cpu.flags, Flags::RESERVED());
    }

    #[test]
    pub fn bit_sets_overflow_bit_if_bit_6_of_operand_is_set() {
        let mut cpu = Mos6502::new();
        cpu.pc.set(0xABCD);
        cpu.registers.a = 0xFF;
        bit::exec(&mut cpu, &mem::Empty, Operand::Immediate(0x40)).unwrap();
        assert_eq!(cpu.flags, Flags::OVERFLOW() | Flags::RESERVED());
    }

    #[test]
    pub fn bit_clears_overflow_bit_if_bit_6_of_operand_is_not_set() {
        let mut cpu = Mos6502::new();
        cpu.pc.set(0xABCD);
        cpu.registers.a = 0xFF;
        cpu.flags.set(Flags::OVERFLOW() | Flags::RESERVED());
        bit::exec(&mut cpu, &mem::Empty, Operand::Immediate(0x01)).unwrap();
        assert_eq!(cpu.flags, Flags::RESERVED());
    }

    #[test]
    pub fn bit_sets_zero_flag_if_result_of_masking_operand_with_a_is_zero() {
        let mut cpu = Mos6502::new();
        cpu.pc.set(0xABCD);
        cpu.registers.a = 0x02;
        bit::exec(&mut cpu, &mem::Empty, Operand::Immediate(0x01)).unwrap();
        assert_eq!(cpu.flags, Flags::ZERO() | Flags::RESERVED());
    }

    #[test]
    pub fn bit_clears_zero_flag_if_result_of_masking_operand_with_a_is_nonzero() {
        let mut cpu = Mos6502::new();
        cpu.pc.set(0xABCD);
        cpu.registers.a = 0x02;
        cpu.flags.set(Flags::ZERO() | Flags::RESERVED());
        bit::exec(&mut cpu, &mem::Empty, Operand::Immediate(0x03)).unwrap();
        assert_eq!(cpu.flags, Flags::RESERVED());
    }
}
