use slog;
use mem::Memory;
use hw::mos6502::exec;
use hw::mos6502::{Operand,Mos6502,Flags};

pub fn exec<M>(cpu: &mut Mos6502, mem: &M, op: Operand, with_carry: bool, log: &slog::Logger) -> exec::Result where M: Memory {
    let opv = try!(op.get_u8(cpu, mem));
    let res = cpu.registers.a & opv;

    trace!(log, cpu_state!(cpu),
        "a" => cpu.registers.a,
        "m" => opv,
        "r" => res,
        "op" => op;
        "evaluated a & m = r");

    cpu.registers.a = res;
    cpu.flags.set_sign_and_zero(res);

    trace!(log, cpu_state!(cpu), "stored result in A");

    if with_carry {
        if cpu.flags.set_if(Flags::CARRY(), res & 0x80 != 0) {
            trace!(log, cpu_state!(cpu), "setting CARRY");
        } else {
            trace!(log, cpu_state!(cpu), "clearing CARRY");
        }
    }

    Ok(())
}

pub fn xaa<M>(cpu: &mut Mos6502, mem: &M, op: Operand, log: &slog::Logger) -> exec::Result where M: Memory {
    let m = try!(op.get_u8(cpu, mem));
    let val = cpu.registers.x & m;
    trace!(log, cpu_state!(cpu),
        "x" => { cpu.registers.x },
        "m" => m,
        "r" => val,
        "addr" => try!(op.get_addr(cpu, mem)),
        "op" => op;
        "evaluated x & m = r");
    cpu.registers.a = val;
    cpu.flags.set_sign_and_zero(val);
    Ok(())
}

#[cfg(test)]
mod test {
    use mem;
    use hw::mos6502::exec::and;
    use hw::mos6502::{Mos6502,Operand,Flags};

    #[test]
    pub fn and_ands_value_with_accumulator() {
        let mut cpu = Mos6502::new();
        cpu.registers.a = 42;
        and::exec(&mut cpu, &mem::Empty, Operand::Immediate(24), false).unwrap();
        assert_eq!(cpu.registers.a, 42 & 24);
    }

    #[test]
    pub fn and_sets_zero_flag_if_result_is_zero() {
        let mut cpu = Mos6502::new();
        cpu.registers.a = 42;
        and::exec(&mut cpu, &mem::Empty, Operand::Immediate(0), false).unwrap();
        assert_eq!(cpu.registers.a, 0);
        assert_eq!(cpu.flags, Flags::ZERO() | Flags::RESERVED());
    }

    #[test]
    pub fn and_sets_sign_flag_if_result_has_bit_7_set() {
        let mut cpu = Mos6502::new();
        cpu.registers.a = 0xFF;
        and::exec(&mut cpu, &mem::Empty, Operand::Immediate(0xFF), false).unwrap();
        assert_eq!(cpu.registers.a, 0xFF);
        assert_eq!(cpu.flags, Flags::SIGN() | Flags::RESERVED());
    }

    #[test]
    pub fn and_sets_carry_flag_if_with_carry_true_and_bit_7_set() {
        let mut cpu = Mos6502::new();
        cpu.registers.a = 0xFF;
        and::exec(&mut cpu, &mem::Empty, Operand::Immediate(0xFF), true).unwrap();
        assert_eq!(cpu.registers.a, 0xFF);
        assert_eq!(cpu.flags, Flags::SIGN() | Flags::RESERVED() | Flags::CARRY());
    }

    #[test]
    pub fn and_does_not_set_carry_flag_if_with_carry_true_and_bit_7_not_set() {
        let mut cpu = Mos6502::new();
        cpu.registers.a = 42;
        and::exec(&mut cpu, &mem::Empty, Operand::Immediate(0), true).unwrap();
        assert_eq!(cpu.registers.a, 0);
        assert_eq!(cpu.flags, Flags::ZERO() | Flags::RESERVED());
    }

    #[test]
    pub fn xaa_ands_value_with_x_and_stores_in_a() {
        let mut cpu = Mos6502::new();
        cpu.registers.x = 42;
        and::xaa(&mut cpu, &mem::Empty, Operand::Immediate(24)).unwrap();
        assert_eq!(cpu.registers.a, 42 & 24);
    }

    #[test]
    pub fn xaa_sets_zero_flag_if_result_is_zero() {
        let mut cpu = Mos6502::new();
        cpu.registers.x = 42;
        and::xaa(&mut cpu, &mem::Empty, Operand::Immediate(0)).unwrap();
        assert_eq!(cpu.registers.a, 0);
        assert_eq!(cpu.flags, Flags::ZERO() | Flags::RESERVED());
    }

    #[test]
    pub fn xaa_sets_sign_flag_if_result_has_bit_7_set() {
        let mut cpu = Mos6502::new();
        cpu.registers.x = 0xFF;
        and::xaa(&mut cpu, &mem::Empty, Operand::Immediate(0xFF)).unwrap();
        assert_eq!(cpu.registers.a, 0xFF);
        assert_eq!(cpu.flags, Flags::SIGN() | Flags::RESERVED());
    }
}
