use mem::Memory;
use hw::mos6502::exec;
use hw::mos6502::{cpu,Mos6502,Operand};

pub fn reg(cpu: &mut Mos6502, reg: cpu::RegisterName) -> Result<(), exec::Error> {
    let new_val = (reg.get(cpu).wrapping_add(1)) & 0xFF;
    cpu.flags.set_sign_and_zero(new_val);
    reg.set(cpu, new_val);
    Ok(())
}

pub fn mem<M>(cpu: &mut Mos6502, mem: &mut M, op: Operand) -> Result<(), exec::Error> where M: Memory {
    let _x = cpu.clock.suspend();

    let new_val = (try!(op.get_u8(cpu, mem)).wrapping_add(1)) & 0xFF;
    cpu.flags.set_sign_and_zero(new_val);
    try!(op.set_u8(cpu, mem, new_val));

    Ok(())
}

#[cfg(test)]
mod test {
    use mem;
    use mem::Memory;
    use hw::mos6502::exec::inc;
    use hw::mos6502::{Mos6502,Flags,Operand};

    #[test]
    fn inc_sets_sign_flag_if_new_value_is_negative() {
        let (mut cpu, mut mem) = init_cpu();
        mem.set_u8(0, 127u8).unwrap();
        inc::mem(&mut cpu, &mut mem, Operand::Absolute(0)).unwrap();
        assert!(cpu.flags.intersects(Flags::SIGN()));
    }

    #[test]
    fn inc_clears_sign_flag_if_new_value_is_not_negative() {
        let (mut cpu, mut mem) = init_cpu();
        cpu.flags.set(Flags::SIGN());
        mem.set_u8(0, -1i8 as u8).unwrap();
        inc::mem(&mut cpu, &mut mem, Operand::Absolute(0)).unwrap();
        assert!(!cpu.flags.intersects(Flags::SIGN()));
    }

    #[test]
    fn inc_sets_zero_flag_if_new_value_is_zero() {
        let (mut cpu, mut mem) = init_cpu();
        mem.set_u8(0, -1i8 as u8).unwrap();
        inc::mem(&mut cpu, &mut mem, Operand::Absolute(0)).unwrap();
        assert!(cpu.flags.intersects(Flags::ZERO()));
    }

    #[test]
    fn inc_clears_zero_flag_if_new_value_is_nonzero() {
        let (mut cpu, mut mem) = init_cpu();
        cpu.flags.set(Flags::ZERO());
        mem.set_u8(0, 0).unwrap();
        inc::mem(&mut cpu, &mut mem, Operand::Absolute(0)).unwrap();
        assert!(!cpu.flags.intersects(Flags::ZERO()));
    }

    #[test]
    fn inc_sets_operand_to_original_value_plus_one() {
        let (mut cpu, mut mem) = init_cpu();
        mem.set_u8(0, 42).unwrap();
        inc::mem(&mut cpu, &mut mem, Operand::Absolute(0)).unwrap();
        assert_eq!(Ok(43), mem.get_u8(0));
    }

    fn init_cpu() -> (Mos6502,mem::Virtual<'static>) {
        let base_memory = mem::Fixed::new(10);
        let mut vm = mem::Virtual::new();

        vm.attach(0, Box::new(base_memory)).unwrap();

        let cpu = Mos6502::new();

        (cpu, vm)
    }
}