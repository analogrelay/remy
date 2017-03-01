use slog;
use mem::Memory;
use hw::mos6502::exec;
use hw::mos6502::{cpu,Mos6502,Flags,Operand};

pub fn exec<M>(cpu: &mut Mos6502, mem: &M, reg: cpu::RegisterName, op: Operand, log: &slog::Logger) -> Result<(), exec::Error> where M: Memory {
    let val = try_log!(op.get_u8(cpu, mem), log);
    let r = reg.get(cpu) as i16;
    let t = r - val as i16;
    trace!(log, "cpu" => cpu,
        "reg" => r, 
        "m" => val, 
        "r" => t, 
        "register" => reg, 
        "op" => op; 
        "evaluated reg[{:?}] - m = r", reg);

    if cpu.flags.set_if(Flags::CARRY(), t >= 0) {
        trace!(log, "cpu" => cpu; "setting CARRY");
    } else {
        trace!(log, "cpu" => cpu; "clearing CARRY");
    }

    cpu.flags.set_sign_and_zero(t as u8);
    Ok(())
}

#[cfg(test)]
mod test {
    use mem;
    use hw::mos6502::exec::compare;
    use hw::mos6502::{Mos6502,Flags,Operand,cpu};

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
