use mem::Memory;
use cpu::mos6502::{ExecError,Mos6502,Flags};

pub fn exec<M>(cpu: &mut Mos6502<M>, offset: i8) -> Result<(), ExecError> where M: Memory {
    if !cpu.registers.has_flags(Flags::OVERFLOW()) {
        cpu.pc.advance(offset as isize)
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use mem::VirtualMemory;
	use cpu::mos6502::instr::bvc;
	use cpu::mos6502::{Mos6502,Flags};

    #[test]
    pub fn bvc_advances_pc_by_specified_amount_if_overflow_flag_clear() {
        let mut cpu = init_cpu();
        bvc::exec(&mut cpu, 1).unwrap();
        assert_eq!(cpu.pc.get(), 0xABCE);
    }

    #[test]
    pub fn bvc_does_not_modify_pc_if_overflow_flag_set() {
        let mut cpu = init_cpu();
        cpu.registers.set_flags(Flags::OVERFLOW());
        bvc::exec(&mut cpu, 1).unwrap();
        assert_eq!(cpu.pc.get(), 0xABCD);
    }

    fn init_cpu() -> Mos6502<VirtualMemory<'static>> {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm);

        cpu.pc.set(0xABCD);

        cpu
    }
}