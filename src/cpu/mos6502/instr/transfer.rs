use mem::Memory;
use cpu::mos6502::{ExecError,Mos6502,RegisterName};

pub fn exec<M>(cpu: &mut Mos6502<M>, src: RegisterName, dst: RegisterName) -> Result<(), ExecError> where M: Memory {
    let val = src.get(cpu);
    dst.set(cpu, val);
    Ok(())
}

#[cfg(test)]
mod test {
    use mem::VirtualMemory;
    use cpu::mos6502::instr::transfer;
    use cpu::mos6502::{Mos6502,RegisterName};

    #[test]
    pub fn transfer_sets_destination_register_to_source_register_value() {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm); 

        cpu.registers.a = 42;
        transfer::exec(&mut cpu, RegisterName::A, RegisterName::X).unwrap();

        assert_eq!(42, cpu.registers.x);
    }
}
