use mem::Memory;
use cpu::mos6502::{ExecError,Mos6502,Flags};

pub fn exec<M>(cpu: &mut Mos6502<M>) -> Result<(), ExecError> where M: Memory {
    cpu.flags.clear(Flags::BCD());
    Ok(())
}

#[cfg(test)]
mod test {
    use mem::VirtualMemory;
	use cpu::mos6502::instr::cld;
	use cpu::mos6502::{Mos6502,Flags};

    #[test]
    pub fn cld_clears_bcd_flag() {
        let mut cpu = init_cpu();
        cpu.flags.set(Flags::BCD());
        cld::exec(&mut cpu).unwrap();
        assert!(!cpu.flags.intersects(Flags::BCD()));
    }

    fn init_cpu() -> Mos6502<VirtualMemory<'static>> {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm);

        cpu.pc.set(0xABCD);

        cpu
    }
}
