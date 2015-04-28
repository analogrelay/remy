use mem::Memory;
use cpus::mos6502::exec;
use cpus::mos6502::{cpu,Mos6502,Flags,Operand};

pub fn exec<M>(cpu: &mut Mos6502, mem: &M, reg: cpu::RegisterName, op: Operand) -> Result<(), exec::Error> where M: Memory {
    let val = try!(op.get_u8(cpu, mem));
    let t = (reg.get(cpu) as isize) - (val as isize);

    cpu.flags.clear(
        Flags::SIGN() |
        Flags::CARRY() |
        Flags::ZERO());

    if t < 0 {
        cpu.flags.set(Flags::SIGN());
    } else if t >= 0 {
        cpu.flags.set(Flags::CARRY());
        if t == 0 {
            cpu.flags.set(Flags::ZERO());
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use mem;
    use cpus::mos6502::exec::compare;
    use cpus::mos6502::{Mos6502,Flags,Operand,cpu};

    #[test]
    pub fn compare_sets_sign_bit_if_operand_greater_than_a() {
        let mut cpu = init_cpu();
        compare::exec(&mut cpu, &mem::Empty, cpu::RegisterName::A, Operand::Immediate(43)).unwrap();
        assert!(cpu.flags.intersects(Flags::SIGN()));
    }

    #[test]
    pub fn compare_clears_sign_bit_if_operand_less_than_a() {
        let mut cpu = init_cpu();
        cpu.flags.set(Flags::SIGN());
        compare::exec(&mut cpu, &mem::Empty, cpu::RegisterName::A, Operand::Immediate(41)).unwrap();
        assert!(!cpu.flags.intersects(Flags::SIGN()));
    }

    #[test]
    pub fn compare_sets_carry_bit_if_a_greater_than_operand() {
        let mut cpu = init_cpu();
        compare::exec(&mut cpu, &mem::Empty, cpu::RegisterName::A, Operand::Immediate(41)).unwrap();
        assert!(cpu.flags.intersects(Flags::CARRY()));
    }

    #[test]
    pub fn compare_sets_carry_bit_if_a_equal_to_operand() {
        let mut cpu = init_cpu();
        compare::exec(&mut cpu, &mem::Empty, cpu::RegisterName::A, Operand::Immediate(42)).unwrap();
        assert!(cpu.flags.intersects(Flags::CARRY()));
    }

    #[test]
    pub fn compare_clears_carry_bit_if_a_less_than_operand() {
        let mut cpu = init_cpu();
        cpu.flags.set(Flags::CARRY());
        compare::exec(&mut cpu, &mem::Empty, cpu::RegisterName::A, Operand::Immediate(43)).unwrap();
        assert!(!cpu.flags.intersects(Flags::CARRY()));
    }

    #[test]
    pub fn compare_sets_zero_bit_if_a_equal_to_operand() {
        let mut cpu = init_cpu();
        compare::exec(&mut cpu, &mem::Empty, cpu::RegisterName::A, Operand::Immediate(42)).unwrap();
        assert!(cpu.flags.intersects(Flags::ZERO()));
    }

    #[test]
    pub fn compare_clears_zero_bit_if_a_less_than_operand() {
        let mut cpu = init_cpu();
        cpu.flags.set(Flags::ZERO());
        compare::exec(&mut cpu, &mem::Empty, cpu::RegisterName::A, Operand::Immediate(43)).unwrap();
        assert!(!cpu.flags.intersects(Flags::ZERO()));
    }

    #[test]
    pub fn compare_clears_zero_bit_if_a_greater_than_operand() {
        let mut cpu = init_cpu();
        cpu.flags.set(Flags::ZERO());
        compare::exec(&mut cpu, &mem::Empty, cpu::RegisterName::A, Operand::Immediate(41)).unwrap();
        assert!(!cpu.flags.intersects(Flags::ZERO()));
    }

    fn init_cpu() -> Mos6502 {
        let mut cpu = Mos6502::new();

        cpu.pc.set(0xABCD);
        cpu.registers.a = 42;

        cpu
    }
}
