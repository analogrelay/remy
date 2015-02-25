use mem::Memory;
use cpu::mos6502::{ExecError,Mos6502,Flags,Operand};

pub fn exec<M>(cpu: &mut Mos6502<M>, op: Operand) -> Result<(), ExecError> where M: Memory {
    let new_value = cpu.registers.a ^ try!(op.get_u8(cpu));
    if (new_value & 0b10000000) != 0 {
        cpu.flags.set(Flags::SIGN());
    }
    if new_value == 0 {
        cpu.flags.set(Flags::ZERO());
    }
    cpu.registers.a = new_value;
    Ok(())
}

#[cfg(test)]
mod test {
    use mem::{Memory,FixedMemory,VirtualMemory};
	use cpu::mos6502::instr::eor;
	use cpu::mos6502::{Mos6502,Flags,Operand};

    #[test]
    fn eor_sets_sign_bit_if_result_is_negative() {
        let mut cpu = init_cpu();
        cpu.mem.set_u8(0, 0b11111000).unwrap();
        cpu.registers.a = 0b00001111;
        eor::exec(&mut cpu, Operand::Absolute(0)).unwrap();
        assert!(cpu.flags.intersects(Flags::SIGN()));
    }

    #[test]
    fn eor_sets_zero_bit_if_result_is_zero() {
        let mut cpu = init_cpu();
        cpu.mem.set_u8(0, 0b11111000).unwrap();
        cpu.registers.a = 0b11111000;
        eor::exec(&mut cpu, Operand::Absolute(0)).unwrap();
        assert!(cpu.flags.intersects(Flags::ZERO()));
    }

    #[test]
    fn eor_sets_a_to_result_of_xor() {
        let mut cpu = init_cpu();
        cpu.mem.set_u8(0, 0b11111000).unwrap();
        cpu.registers.a = 0b00001111;
        eor::exec(&mut cpu, Operand::Absolute(0)).unwrap();
        assert_eq!(0b11110111, cpu.registers.a);
    }

    fn init_cpu() -> Mos6502<VirtualMemory<'static>> {
        let base_memory = FixedMemory::new(10);
        let mut vm = VirtualMemory::new();

        vm.attach(0, Box::new(base_memory)).unwrap();

        let cpu = Mos6502::new(vm);

        cpu
    }
}
