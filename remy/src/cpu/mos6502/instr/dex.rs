use mem::Memory;
use cpu::mos6502::{ExecError,Mos6502,Flags};

pub fn exec<M>(cpu: &mut Mos6502<M>) -> Result<(), ExecError> where M: Memory {
    let new_val = (cpu.registers.x - 1) & 0xFF;
    if (new_val & 0b1000000) != 0 {
        cpu.registers.set_flags(Flags::SIGN());
    }
    if new_val == 0 {
        cpu.registers.set_flags(Flags::ZERO());
    }
    cpu.registers.x = new_val;
    Ok(())
}

#[cfg(test)]
mod test {
    use mem::{FixedMemory,VirtualMemory};
	use cpu::mos6502::instr::dex;
	use cpu::mos6502::{Mos6502,Flags};

    #[test]
    fn dex_sets_sign_flag_if_new_value_is_negative() {
        let mut cpu = init_cpu();
        cpu.registers.x = 0;
        dex::exec(&mut cpu).unwrap();
        assert!(cpu.registers.has_flags(Flags::SIGN()));
    }

    #[test]
    fn dex_sets_zero_flag_if_new_value_is_zero() {
        let mut cpu = init_cpu();
        cpu.registers.x = 1;
        dex::exec(&mut cpu).unwrap();
        assert!(cpu.registers.has_flags(Flags::ZERO()));
    }

    #[test]
    fn dex_sets_operand_to_original_value_minus_one() {
        let mut cpu = init_cpu();
        cpu.registers.x = 42; 
        dex::exec(&mut cpu).unwrap();
        assert_eq!(41, cpu.registers.x);
    }
    
    fn init_cpu() -> Mos6502<VirtualMemory<'static>> {
        let base_memory = FixedMemory::new(10);
        let mut vm = VirtualMemory::new();

        vm.attach(0, Box::new(base_memory)).unwrap();

        let cpu = Mos6502::new(vm);

        cpu
    }
}
