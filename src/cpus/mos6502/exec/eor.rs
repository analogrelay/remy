use mem::{Memory,MemoryExt};
use cpus::mos6502::exec;
use cpus::mos6502::{Mos6502,Operand};

pub fn exec<M>(cpu: &mut Mos6502, mem: &M, op: Operand) -> Result<(), exec::Error> where M: Memory {
    let new_value = cpu.registers.a ^ try!(op.get_u8(cpu, mem));
    cpu.flags.set_sign_and_zero(new_value);
    cpu.registers.a = new_value;
    Ok(())
}

#[cfg(test)]
mod test {
    use mem;
    use mem::MemoryExt;
    use cpus::mos6502::exec::eor;
    use cpus::mos6502::{Mos6502,Flags,Operand};

    #[test]
    fn eor_sets_sign_bit_if_result_is_negative() {
        let (mut cpu, mut mem) = init_cpu();
        mem.set_u8(0, 0b11111000).unwrap();
        cpu.registers.a = 0b00001111;
        eor::exec(&mut cpu, &mem, Operand::Absolute(0)).unwrap();
        assert!(cpu.flags.intersects(Flags::SIGN()));
    }

    #[test]
    fn eor_sets_zero_bit_if_result_is_zero() {
        let (mut cpu, mut mem) = init_cpu();
        mem.set_u8(0, 0b11111000).unwrap();
        cpu.registers.a = 0b11111000;
        eor::exec(&mut cpu, &mem, Operand::Absolute(0)).unwrap();
        assert!(cpu.flags.intersects(Flags::ZERO()));
    }

    #[test]
    fn eor_sets_a_to_result_of_xor() {
        let (mut cpu, mut mem) = init_cpu();
        mem.set_u8(0, 0b11111000).unwrap();
        cpu.registers.a = 0b00001111;
        eor::exec(&mut cpu, &mem, Operand::Absolute(0)).unwrap();
        assert_eq!(0b11110111, cpu.registers.a);
    }

    fn init_cpu() -> (Mos6502,mem::Virtual<'static>) {
        let base_memory = mem::Fixed::new(10);
        let mut vm = mem::Virtual::new();

        vm.attach(0, Box::new(base_memory)).unwrap();

        let cpu = Mos6502::new();

        (cpu, vm)
    }
}
