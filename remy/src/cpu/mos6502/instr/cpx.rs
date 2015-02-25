use mem::Memory;
use cpu::mos6502::{ExecError,Mos6502,Flags,Operand};

pub fn exec<M>(cpu: &mut Mos6502<M>, op: Operand) -> Result<(), ExecError> where M: Memory {
    let val = try!(op.get_u8(cpu));
    let t = (cpu.registers.x as isize) - (val as isize);

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
    use mem::VirtualMemory;
	use cpu::mos6502::instr::cpx;
	use cpu::mos6502::{Mos6502,Flags,Operand};

    #[test]
    pub fn cpx_sets_sign_bit_if_operand_greater_than_a() {
        let mut cpu = init_cpu();
        cpx::exec(&mut cpu, Operand::Immediate(43)).unwrap();
        assert!(cpu.flags.intersects(Flags::SIGN()));
    }

    #[test]
    pub fn cpx_clears_sign_bit_if_operand_less_than_a() {
        let mut cpu = init_cpu();
        cpu.flags.set(Flags::SIGN());
        cpx::exec(&mut cpu, Operand::Immediate(41)).unwrap();
        assert!(!cpu.flags.intersects(Flags::SIGN()));
    }

    #[test]
    pub fn cpx_sets_carry_bit_if_x_greater_than_operand() {
        let mut cpu = init_cpu();
        cpx::exec(&mut cpu, Operand::Immediate(41)).unwrap();
        assert!(cpu.flags.intersects(Flags::CARRY()));
    }

    #[test]
    pub fn cpx_sets_carry_bit_if_x_equal_to_operand() {
        let mut cpu = init_cpu();
        cpx::exec(&mut cpu, Operand::Immediate(42)).unwrap();
        assert!(cpu.flags.intersects(Flags::CARRY()));
    }

    #[test]
    pub fn cpx_clears_carry_bit_if_x_less_than_operand() {
        let mut cpu = init_cpu();
        cpu.flags.set(Flags::CARRY());
        cpx::exec(&mut cpu, Operand::Immediate(43)).unwrap();
        assert!(!cpu.flags.intersects(Flags::CARRY()));
    }

    #[test]
    pub fn cpx_sets_zero_bit_if_x_equal_to_operand() {
        let mut cpu = init_cpu();
        cpx::exec(&mut cpu, Operand::Immediate(42)).unwrap();
        assert!(cpu.flags.intersects(Flags::ZERO()));
    }

    #[test]
    pub fn cpx_clears_zero_bit_if_x_less_than_operand() {
        let mut cpu = init_cpu();
        cpu.flags.set(Flags::ZERO());
        cpx::exec(&mut cpu, Operand::Immediate(43)).unwrap();
        assert!(!cpu.flags.intersects(Flags::ZERO()));
    }

    #[test]
    pub fn cpx_clears_zero_bit_if_x_greater_than_operand() {
        let mut cpu = init_cpu();
        cpu.flags.set(Flags::ZERO());
        cpx::exec(&mut cpu, Operand::Immediate(41)).unwrap();
        assert!(!cpu.flags.intersects(Flags::ZERO()));
    }

    fn init_cpu() -> Mos6502<VirtualMemory<'static>> {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm);

        cpu.pc.set(0xABCD);
        cpu.registers.x = 42;

        cpu
    }
}
