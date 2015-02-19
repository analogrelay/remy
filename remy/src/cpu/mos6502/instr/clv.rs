use mem::Memory;
use cpu::mos6502::{ExecError,Mos6502,Flags};

pub fn exec<M>(cpu: &mut Mos6502<M>) -> Result<(), ExecError> where M: Memory {
    cpu.registers.clear_flags(Flags::OVERFLOW());
    Ok(())
}

#[cfg(test)]
mod test {
    use mem::VirtualMemory;
	use cpu::mos6502::instr::clv;
	use cpu::mos6502::{Mos6502,Flags};

    #[test]
    pub fn clv_clears_overflow_flag() {
        let mut cpu = init_cpu();
        cpu.registers.set_flags(Flags::OVERFLOW());
        clv::exec(&mut cpu).unwrap();
        assert!(!cpu.registers.has_flags(Flags::OVERFLOW()));
    }

    fn init_cpu() -> Mos6502<VirtualMemory<'static>> {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm);

        cpu.pc.set(0xABCD);

        cpu
    }
}