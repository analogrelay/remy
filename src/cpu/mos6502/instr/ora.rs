use mem::Memory;
use cpu::mos6502::{ExecError,Mos6502,Operand};

pub fn exec<M>(cpu: &mut Mos6502<M>, op: Operand) -> Result<(), ExecError> where M : Memory {
    let v = cpu.registers.a | try!(op.get_u8(cpu));
    cpu.flags.set_sign_and_zero(v);
    cpu.registers.a = v;
    Ok(())
}

#[cfg(test)]
mod test {
    use mem::{Memory,FixedMemory,VirtualMemory};
	use cpu::mos6502::instr::ora;
	use cpu::mos6502::{Mos6502,Flags,Operand};

    #[test]
    fn ora_sets_sign_bit_if_result_is_negative() {
        let mut cpu = init_cpu();
        cpu.mem.set_u8(0, 0b11111000).unwrap();
        cpu.registers.a = 0b00001111;
        ora::exec(&mut cpu, Operand::Absolute(0)).unwrap();
        assert!(cpu.flags.intersects(Flags::SIGN()));
    }

    #[test]
    fn ora_sets_zero_bit_if_result_is_zero() {
        let mut cpu = init_cpu();
        cpu.mem.set_u8(0, 0b00000000).unwrap();
        cpu.registers.a = 0b00000000;
        ora::exec(&mut cpu, Operand::Absolute(0)).unwrap();
        assert!(cpu.flags.intersects(Flags::ZERO()));
    }

    #[test]
    fn ora_sets_a_to_result_of_or() {
        let mut cpu = init_cpu();
        cpu.mem.set_u8(0, 0b11111000).unwrap();
        cpu.registers.a = 0b00001111;
        ora::exec(&mut cpu, Operand::Absolute(0)).unwrap();
        assert_eq!(0b11111111, cpu.registers.a);
    }

    fn init_cpu() -> Mos6502<VirtualMemory<'static>> {
        let base_memory = FixedMemory::new(10);
        let mut vm = VirtualMemory::new();

        vm.attach(0, Box::new(base_memory)).unwrap();

        let cpu = Mos6502::new(vm);

        cpu
    }
}
