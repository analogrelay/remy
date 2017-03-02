use slog;
use mem::Memory;
use hw::mos6502::exec;
use hw::mos6502::{cpu,Mos6502,Operand};

pub fn reg(cpu: &mut Mos6502, reg: cpu::RegisterName, log: &slog::Logger) -> Result<(), exec::Error> {
    let old_val = reg.get(cpu);
    let new_val = (old_val.wrapping_sub(1)) & 0xFF;

    trace!(log, "cpu" => cpu, 
        "reg" => old_val, 
        "r" => new_val, 
        "register" => reg; 
        "evaluated reg[{:?}]-- = r", reg);

    cpu.flags.set_sign_and_zero(new_val); 
    reg.set(cpu, new_val);
    trace!(log, "cpu" => cpu, "register" => reg; "stored result in {:?}", reg);

    Ok(())
}

pub fn mem<M>(cpu: &mut Mos6502, mem: &mut M, op: Operand, log: &slog::Logger) -> Result<(), exec::Error> where M: Memory {
    let _x = cpu.clock.suspend();

    let old_val = try_log!(op.get_u8(cpu, mem), log);
    let new_val = (old_val.wrapping_sub(1)) & 0xFF;

    trace!(log, "cpu" => cpu, 
        "mem" => old_val,
        "r" => new_val,
        "addr" => op.get_addr(cpu, mem).ok();
        "evaluated mem-- = r");

    cpu.flags.set_sign_and_zero(new_val); 
    try_log!(op.set_u8(cpu, mem, new_val), log);
    trace!(log, "cpu" => cpu, "addr" => op.get_addr(cpu, mem).ok(); "stored result");

    Ok(())
}

#[cfg(test)]
mod test {
    use slog;
    use mem;
    use mem::Memory;
    use hw::mos6502::exec::dec;
    use hw::mos6502::{Mos6502,Flags,Operand};

    #[test]
    fn dec_sets_sign_flag_if_new_value_is_negative() {
        let (mut cpu, mut mem) = init_cpu();
        mem.set_u8(0, 0).unwrap();
        dec::mem(&mut cpu, &mut mem, Operand::Absolute(0), &slog::Logger::root(slog::Discard, o!())).unwrap();
        assert!(cpu.flags.intersects(Flags::SIGN()));
    }

    #[test]
    fn dec_clears_sign_flag_if_new_value_is_non_negative() {
        let (mut cpu, mut mem) = init_cpu();
        mem.set_u8(0, 2).unwrap();
        cpu.flags.set(Flags::SIGN());
        dec::mem(&mut cpu, &mut mem, Operand::Absolute(0), &slog::Logger::root(slog::Discard, o!())).unwrap();
        assert!(!cpu.flags.intersects(Flags::SIGN()));
    }

    #[test]
    fn dec_sets_zero_flag_if_new_value_is_zero() {
        let (mut cpu, mut mem) = init_cpu();
        mem.set_u8(0, 1).unwrap();
        dec::mem(&mut cpu, &mut mem, Operand::Absolute(0), &slog::Logger::root(slog::Discard, o!())).unwrap();
        assert!(cpu.flags.intersects(Flags::ZERO()));
    }

    #[test]
    fn dec_clears_zero_flag_if_new_value_is_nonzero() {
        let (mut cpu, mut mem) = init_cpu();
        mem.set_u8(0, 2).unwrap();
        cpu.flags.set(Flags::ZERO());
        dec::mem(&mut cpu, &mut mem, Operand::Absolute(0), &slog::Logger::root(slog::Discard, o!())).unwrap();
        assert!(!cpu.flags.intersects(Flags::ZERO()));
    }

    #[test]
    fn dec_sets_operand_to_original_value_minus_one() {
        let (mut cpu, mut mem) = init_cpu();
        mem.set_u8(0, 42).unwrap();
        dec::mem(&mut cpu, &mut mem, Operand::Absolute(0), &slog::Logger::root(slog::Discard, o!())).unwrap();
        assert_eq!(Ok(41), mem.get_u8(0));
    }

    fn init_cpu() -> (Mos6502,mem::Virtual<'static>) {
        let base_memory = mem::Fixed::new(10);
        let mut vm = mem::Virtual::new();

        vm.attach(0, Box::new(base_memory)).unwrap();

        let cpu = Mos6502::new();

        (cpu, vm)
    }
}
