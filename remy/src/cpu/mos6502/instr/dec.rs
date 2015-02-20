use mem::Memory;
use cpu::mos6502::{ExecError,Mos6502,Flags,Operand};

pub fn exec<M>(cpu: &mut Mos6502<M>, op: Operand) -> Result<(), ExecError> where M: Memory {
    let new_val = (try!(op.get_u8(cpu)) - 1) & 0xFF;
    if (new_val & 0b1000000) != 0 {
        cpu.registers.set_flags(Flags::SIGN());
    }
    if new_val == 0 {
        cpu.registers.set_flags(Flags::ZERO());
    }
    try!(op.set_u8(cpu, new_val));
    Ok(())
}

#[cfg(test)]
mod test {
    use mem::{Memory,FixedMemory,VirtualMemory};
	use cpu::mos6502::instr::dec;
	use cpu::mos6502::{Mos6502,Flags,Operand};

    #[test]
    fn dec_sets_sign_flag_if_new_value_is_negative() {
        let mut cpu = init_cpu();
        cpu.mem.set_u8(0, 0).unwrap();
        dec::exec(&mut cpu, Operand::Absolute(0)).unwrap();
        assert!(cpu.registers.has_flags(Flags::SIGN()));
    }

    #[test]
    fn dec_sets_zero_flag_if_new_value_is_zero() {
        let mut cpu = init_cpu();
        cpu.mem.set_u8(0, 1).unwrap();
        dec::exec(&mut cpu, Operand::Absolute(0)).unwrap();
        assert!(cpu.registers.has_flags(Flags::ZERO()));
    }

    #[test]
    fn dec_sets_operand_to_original_value_minus_one() {
        let mut cpu = init_cpu();
        cpu.mem.set_u8(0, 42).unwrap();
        dec::exec(&mut cpu, Operand::Absolute(0)).unwrap();
        assert_eq!(Ok(41), cpu.mem.get_u8(0));
    }
    
    fn init_cpu() -> Mos6502<VirtualMemory<'static>> {
        let base_memory = FixedMemory::new(10);
        let mut vm = VirtualMemory::new();

        vm.attach(0, Box::new(base_memory)).unwrap();

        let cpu = Mos6502::new(vm);

        cpu
    }
}