use mem::Memory;
use cpus::mos6502::exec;
use cpus::mos6502::{Operand,Mos6502,Flags};

pub fn exec<M>(cpu: &mut Mos6502<M>, op: Operand, with_carry: bool) -> exec::Result where M: Memory {
    let opv = try!(op.get_u8(cpu));
    let res = cpu.registers.a & opv;
    cpu.registers.a = res;
    cpu.flags.set_sign_and_zero(res);
    cpu.flags.set_if(Flags::CARRY(), with_carry && (res & 0x80 != 0));
    Ok(())
}

#[cfg(test)]
mod test {
    use mem::VirtualMemory;
	use cpus::mos6502::exec::and;
	use cpus::mos6502::{Mos6502,Operand,Flags};

    #[test]
    pub fn and_ands_value_with_accumulator() {
        let mut cpu = init_cpu();
        and::exec(&mut cpu, Operand::Immediate(24), false).unwrap();
        assert_eq!(cpu.registers.a, 42 & 24);
    }

    #[test]
    pub fn and_sets_zero_flag_if_result_is_zero() {
        let mut cpu = init_cpu();
        and::exec(&mut cpu, Operand::Immediate(0), false).unwrap();
        assert_eq!(cpu.registers.a, 0);
        assert_eq!(cpu.flags, Flags::ZERO() | Flags::RESERVED());
    }

    #[test]
    pub fn and_sets_sign_flag_if_result_has_bit_7_set() {
        let mut cpu = init_cpu();
        cpu.registers.a = 0xFF;
        and::exec(&mut cpu, Operand::Immediate(0xFF), false).unwrap();
        assert_eq!(cpu.registers.a, 0xFF);
        assert_eq!(cpu.flags, Flags::SIGN() | Flags::RESERVED());
    }
    
    #[test]
    pub fn and_sets_carry_flag_if_with_carry_true_and_bit_7_set() {
        let mut cpu = init_cpu();
        cpu.registers.a = 0xFF;
        and::exec(&mut cpu, Operand::Immediate(0xFF), true).unwrap();
        assert_eq!(cpu.registers.a, 0xFF);
        assert_eq!(cpu.flags, Flags::SIGN() | Flags::RESERVED() | Flags::CARRY());
    }
    
    #[test]
    pub fn and_does_not_set_carry_flag_if_with_carry_true_and_bit_7_not_set() {
        let mut cpu = init_cpu();
        and::exec(&mut cpu, Operand::Immediate(0), true).unwrap();
        assert_eq!(cpu.registers.a, 0);
        assert_eq!(cpu.flags, Flags::ZERO() | Flags::RESERVED());
    }
    
    fn init_cpu() -> Mos6502<VirtualMemory<'static>> {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm);
        cpu.registers.a = 42;

        cpu
    }
}
