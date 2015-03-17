use mem::Memory;
use cpus::mos6502::{ExecError,Operand,Mos6502,Flags};

pub fn exec<M>(cpu: &mut Mos6502<M>, op: Operand) -> Result<(), ExecError> where M: Memory {
    let opv = try!(op.get_u8(cpu));
    let res = cpu.registers.a & opv;
    cpu.registers.a = res;
    if res == 0 {
        cpu.flags.set(Flags::ZERO());
    }
    else if res & 0x80 != 0 {
        cpu.flags.set(Flags::SIGN());
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use mem::VirtualMemory;
	use cpus::mos6502::instr::and;
	use cpus::mos6502::{Mos6502,Operand,Flags};

    #[test]
    pub fn and_ands_value_with_accumulator() {
        let mut cpu = init_cpu();
        and::exec(&mut cpu, Operand::Immediate(24)).unwrap();
        assert_eq!(cpu.registers.a, 42 & 24);
    }

    #[test]
    pub fn and_sets_zero_flag_if_result_is_zero() {
        let mut cpu = init_cpu();
        and::exec(&mut cpu, Operand::Immediate(0)).unwrap();
        assert_eq!(cpu.registers.a, 0);
        assert_eq!(cpu.flags, Flags::ZERO() | Flags::RESERVED());
    }

    #[test]
    pub fn and_sets_sign_flag_if_result_has_bit_7_set() {
        let mut cpu = init_cpu();
        cpu.registers.a = 0xFF;
        and::exec(&mut cpu, Operand::Immediate(0xFF)).unwrap();
        assert_eq!(cpu.registers.a, 0xFF);
        assert_eq!(cpu.flags, Flags::SIGN() | Flags::RESERVED());
    }
    
    fn init_cpu() -> Mos6502<VirtualMemory<'static>> {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm);
        cpu.registers.a = 42;

        cpu
    }
}
