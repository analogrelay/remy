use mem::Memory;
use cpu::mos6502::{ExecError,Mos6502,Flags};

pub fn exec<M>(cpu: &mut Mos6502<M>) -> Result<(), ExecError> where M: Memory {
    let new_val = (cpu.registers.y - 1) & 0xFF;
    if (new_val & 0b1000000) != 0 {
        cpu.registers.set_flags(Flags::SIGN());
    }
    if new_val == 0 {
        cpu.registers.set_flags(Flags::ZERO());
    }
    cpu.registers.y = new_val;
    Ok(())
}

#[cfg(test)]
mod test {
    use mem::{FixedMemory,VirtualMemory};
	use cpu::mos6502::instr::dey;
	use cpu::mos6502::{Mos6502,Flags};

    #[test]
    fn dey_sets_sign_flag_if_new_value_is_negative() {
        let mut cpu = init_cpu();
        cpu.registers.y = 0;
        dey::exec(&mut cpu).unwrap();
        assert!(cpu.registers.has_flags(Flags::SIGN()));
    }

    #[test]
    fn dey_sets_zero_flag_if_new_value_is_zero() {
        let mut cpu = init_cpu();
        cpu.registers.y = 1;
        dey::exec(&mut cpu).unwrap();
        assert!(cpu.registers.has_flags(Flags::ZERO()));
    }

    #[test]
    fn dey_sets_operand_to_original_value_minus_one() {
        let mut cpu = init_cpu();
        cpu.registers.y = 42; 
        dey::exec(&mut cpu).unwrap();
        assert_eq!(41, cpu.registers.y);
    }
    
    fn init_cpu() -> Mos6502<VirtualMemory<'static>> {
        let base_memory = FixedMemory::new(10);
        let mut vm = VirtualMemory::new();

        vm.attach(0, Box::new(base_memory)).unwrap();

        let cpu = Mos6502::new(vm);

        cpu
    }
}
