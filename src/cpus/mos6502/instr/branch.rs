use mem::Memory;
use cpus::mos6502::{ExecError,Mos6502,Flags};

pub fn if_clear<M>(cpu: &mut Mos6502<M>, offset: i8, flags: Flags) -> Result<(), ExecError> where M: Memory {
    if !cpu.flags.intersects(flags) {
        cpu.pc.advance(offset as isize)
    }
    Ok(())
}

pub fn if_set<M>(cpu: &mut Mos6502<M>, offset: i8, flags: Flags) -> Result<(), ExecError> where M: Memory {
    if cpu.flags.intersects(flags) {
        cpu.pc.advance(offset as isize)
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use mem::VirtualMemory;
	use cpus::mos6502::instr::branch;
	use cpus::mos6502::{Mos6502,Flags};

    #[test]
    pub fn if_clear_does_not_modify_pc_if_flag_set() {
        let mut cpu = init_cpu();
        cpu.flags.set(Flags::CARRY() | Flags::SIGN());
        branch::if_clear(&mut cpu, 1, Flags::CARRY()).unwrap();
        assert_eq!(cpu.pc.get(), 0xABCD);
    }

    #[test]
    pub fn if_clear_advances_pc_by_specified_amount_if_flag_clear() {
        let mut cpu = init_cpu();
        branch::if_clear(&mut cpu, 1, Flags::CARRY()).unwrap();
        assert_eq!(cpu.pc.get(), 0xABCE);
    }

    #[test]
    pub fn if_set_does_not_modify_pc_if_flag_clear() {
        let mut cpu = init_cpu();
        branch::if_set(&mut cpu, 1, Flags::CARRY()).unwrap();
        assert_eq!(cpu.pc.get(), 0xABCD);
    }

    #[test]
    pub fn if_set_advances_pc_by_specified_amount_if_flag_set() {
        let mut cpu = init_cpu();
        cpu.flags.set(Flags::CARRY() | Flags::SIGN());
        branch::if_set(&mut cpu, 1, Flags::CARRY()).unwrap();
        assert_eq!(cpu.pc.get(), 0xABCE);
    }

    fn init_cpu() -> Mos6502<VirtualMemory<'static>> {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm);

        cpu.pc.set(0xABCD);

        cpu
    }
}
