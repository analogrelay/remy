use mem::Memory;
use cpu::mos6502::{ExecError,Mos6502,Flags,Operand};

pub fn exec<M>(cpu: &mut Mos6502<M>, op: Operand) -> Result<(), ExecError> where M: Memory {
    let val = try!(op.get_u8(cpu));
    let t = (cpu.registers.a as isize) - (val as isize);

    cpu.registers.clear_flags(
        Flags::SIGN() |
        Flags::CARRY() |
        Flags::ZERO());

    if t < 0 {
        cpu.registers.set_flags(Flags::SIGN());
    } else if t >= 0 {
        cpu.registers.set_flags(Flags::CARRY());
        if t == 0 {
            cpu.registers.set_flags(Flags::ZERO());
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use mem::VirtualMemory;
	use cpu::mos6502::instr::cmp;
	use cpu::mos6502::{Mos6502,Flags,Operand};

    #[test]
    pub fn cmp_sets_sign_bit_if_operand_greater_than_a() {
        let mut cpu = init_cpu();
        cmp::exec(&mut cpu, Operand::Immediate(43)).unwrap();
        assert!(cpu.registers.has_flags(Flags::SIGN()));
    }

    #[test]
    pub fn cmp_clears_sign_bit_if_operand_less_than_a() {
        let mut cpu = init_cpu();
        cpu.registers.set_flags(Flags::SIGN());
        cmp::exec(&mut cpu, Operand::Immediate(41)).unwrap();
        assert!(!cpu.registers.has_flags(Flags::SIGN()));
    }

    #[test]
    pub fn cmp_sets_carry_bit_if_a_greater_than_operand() {
        let mut cpu = init_cpu();
        cmp::exec(&mut cpu, Operand::Immediate(41)).unwrap();
        assert!(cpu.registers.has_flags(Flags::CARRY()));
    }

    #[test]
    pub fn cmp_sets_carry_bit_if_a_equal_to_operand() {
        let mut cpu = init_cpu();
        cmp::exec(&mut cpu, Operand::Immediate(42)).unwrap();
        assert!(cpu.registers.has_flags(Flags::CARRY()));
    }

    #[test]
    pub fn cmp_clears_carry_bit_if_a_less_than_operand() {
        let mut cpu = init_cpu();
        cpu.registers.set_flags(Flags::CARRY());
        cmp::exec(&mut cpu, Operand::Immediate(43)).unwrap();
        assert!(!cpu.registers.has_flags(Flags::CARRY()));
    }

    #[test]
    pub fn cmp_sets_zero_bit_if_a_equal_to_operand() {
        let mut cpu = init_cpu();
        cmp::exec(&mut cpu, Operand::Immediate(42)).unwrap();
        assert!(cpu.registers.has_flags(Flags::ZERO()));
    }

    #[test]
    pub fn cmp_clears_zero_bit_if_a_less_than_operand() {
        let mut cpu = init_cpu();
        cpu.registers.set_flags(Flags::ZERO());
        cmp::exec(&mut cpu, Operand::Immediate(43)).unwrap();
        assert!(!cpu.registers.has_flags(Flags::ZERO()));
    }

    #[test]
    pub fn cmp_clears_zero_bit_if_a_greater_than_operand() {
        let mut cpu = init_cpu();
        cpu.registers.set_flags(Flags::ZERO());
        cmp::exec(&mut cpu, Operand::Immediate(41)).unwrap();
        assert!(!cpu.registers.has_flags(Flags::ZERO()));
    }

    fn init_cpu() -> Mos6502<VirtualMemory<'static>> {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm);

        cpu.pc.set(0xABCD);
        cpu.registers.a = 42;

        cpu
    }
}