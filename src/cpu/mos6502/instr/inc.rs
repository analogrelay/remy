use mem::Memory;
use cpu::mos6502::{ExecError,Mos6502,Operand};

pub fn exec<M>(cpu: &mut Mos6502<M>, op: Operand) -> Result<(), ExecError> where M: Memory {
    let new_val = (try!(op.get_u8(cpu)) + 1) & 0xFF;
    cpu.flags.set_sign_and_zero(new_val as usize);
    try!(op.set_u8(cpu, new_val));
    Ok(())
}

#[cfg(test)]
mod test {
    use mem::{Memory,FixedMemory,VirtualMemory};
	use cpu::mos6502::instr::inc;
	use cpu::mos6502::{Mos6502,Flags,Operand};

    #[test]
    fn inc_sets_sign_flag_if_new_value_is_negative() {
        let mut cpu = init_cpu();
        cpu.mem.set_u8(0, 127u8).unwrap();
        inc::exec(&mut cpu, Operand::Absolute(0)).unwrap();
        assert!(cpu.flags.intersects(Flags::SIGN()));
    }
    
    #[test]
    fn inc_clears_sign_flag_if_new_value_is_not_negative() {
        let mut cpu = init_cpu();
        cpu.flags.set(Flags::SIGN());
        cpu.mem.set_u8(0, -1).unwrap();
        inc::exec(&mut cpu, Operand::Absolute(0)).unwrap();
        assert!(!cpu.flags.intersects(Flags::SIGN()));
    }

    #[test]
    fn inc_sets_zero_flag_if_new_value_is_zero() {
        let mut cpu = init_cpu();
        cpu.mem.set_u8(0, -1).unwrap();
        inc::exec(&mut cpu, Operand::Absolute(0)).unwrap();
        assert!(cpu.flags.intersects(Flags::ZERO()));
    }

    #[test]
    fn inc_clears_zero_flag_if_new_value_is_nonzero() {
        let mut cpu = init_cpu();
        cpu.flags.set(Flags::ZERO());
        cpu.mem.set_u8(0, 0).unwrap();
        inc::exec(&mut cpu, Operand::Absolute(0)).unwrap();
        assert!(!cpu.flags.intersects(Flags::ZERO()));
    }

    #[test]
    fn inc_sets_operand_to_original_value_plus_one() {
        let mut cpu = init_cpu();
        cpu.mem.set_u8(0, 42).unwrap();
        inc::exec(&mut cpu, Operand::Absolute(0)).unwrap();
        assert_eq!(Ok(43), cpu.mem.get_u8(0));
    }
    
    fn init_cpu() -> Mos6502<VirtualMemory<'static>> {
        let base_memory = FixedMemory::new(10);
        let mut vm = VirtualMemory::new();

        vm.attach(0, Box::new(base_memory)).unwrap();

        let cpu = Mos6502::new(vm);

        cpu
    }
}
