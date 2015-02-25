use mem::Memory;
use cpu::mos6502::{ExecError,Mos6502,Flags};

pub fn exec<M>(cpu: &mut Mos6502<M>, flag_selector: Flags) -> Result<(), ExecError> where M: Memory {
    cpu.flags.clear(flag_selector);
    Ok(())
}

#[cfg(test)]
mod test {
    use mem::VirtualMemory;
	use cpu::mos6502::instr::clear_flag;
	use cpu::mos6502::{Mos6502,Flags};

    #[test]
    pub fn clear_flag_clears_flag() {
        let mut cpu = init_cpu();
        cpu.flags.set(Flags::CARRY() | Flags::SIGN());
        clear_flag::exec(&mut cpu, Flags::CARRY()).unwrap();
        assert!(!cpu.flags.intersects(Flags::CARRY()));
        assert!(cpu.flags.intersects(Flags::SIGN()));
    }

    fn init_cpu() -> Mos6502<VirtualMemory<'static>> {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm);

        cpu.pc.set(0xABCD);

        cpu
    }
}
