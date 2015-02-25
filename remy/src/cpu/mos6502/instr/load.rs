use mem::Memory;
use cpu::mos6502::{ExecError,Mos6502,RegisterName,Operand};

pub fn exec<M>(cpu: &mut Mos6502<M>, reg: RegisterName, op: Operand) -> Result<(), ExecError> where M: Memory {
    let val = try!(op.get_u8(cpu));
    cpu.registers.set(reg, val);
    cpu.flags.set_sign_and_zero(val as usize);
    Ok(())
}

#[cfg(test)]
mod test {
    use mem::VirtualMemory;
    use cpu::mos6502::instr::load;
    use cpu::mos6502::{Mos6502,RegisterName,Flags,Operand};

    #[test]
    pub fn load_sets_register_to_operand_value() {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm); 

        load::exec(&mut cpu, RegisterName::A, Operand::Immediate(42)).unwrap();

        assert_eq!(42, cpu.registers.a);
    }

    #[test]
    fn load_sets_sign_flag_if_new_value_is_negative() {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm); 
        load::exec(&mut cpu, RegisterName::A, Operand::Immediate(-10)).unwrap();
        assert!(cpu.flags.intersects(Flags::SIGN()));
    }

    #[test]
    fn load_clears_sign_flag_if_new_value_is_not_negative() {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm); 
        cpu.flags.set(Flags::SIGN());
        load::exec(&mut cpu, RegisterName::A, Operand::Immediate(0)).unwrap();
        assert!(!cpu.flags.intersects(Flags::SIGN()));
    }

    #[test]
    fn load_sets_zero_flag_if_new_value_is_zero() {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm); 
        load::exec(&mut cpu, RegisterName::A, Operand::Immediate(0)).unwrap();
        assert!(cpu.flags.intersects(Flags::ZERO()));
    }

    #[test]
    fn load_clears_zero_flag_if_new_value_is_nonzero() {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm); 
        cpu.flags.set(Flags::ZERO());
        load::exec(&mut cpu, RegisterName::A, Operand::Immediate(10)).unwrap();
        assert!(!cpu.flags.intersects(Flags::ZERO()));
    }
}
