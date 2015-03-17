use mem::Memory;
use cpus::mos6502::exec;
use cpus::mos6502::{Mos6502,Flags,Operand};

pub fn exec<M>(cpu: &mut Mos6502<M>, op: Operand) -> Result<(), exec::Error> where M: Memory {
    let m = try!(op.get_u8(cpu));
    let t = cpu.registers.a & m;

    if m & 0x80 != 0 {
        cpu.flags.set(Flags::SIGN());
    } else {
        cpu.flags.clear(Flags::SIGN());
    }

    if m & 0x40 != 0 {
        cpu.flags.set(Flags::OVERFLOW());
    } else {
        cpu.flags.clear(Flags::OVERFLOW());
    }

    if t == 0 {
        cpu.flags.set(Flags::ZERO());
    } else {
        cpu.flags.clear(Flags::ZERO());
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use mem::VirtualMemory;
	use cpus::mos6502::exec::bit;
	use cpus::mos6502::{Mos6502,Flags,Operand};

    #[test]
    pub fn bit_sets_sign_bit_if_bit_7_of_operand_is_set() {
        let mut cpu = init_cpu();
        cpu.registers.a = 0xFF;
        bit::exec(&mut cpu, Operand::Immediate(0x80)).unwrap();
        assert_eq!(cpu.flags, Flags::SIGN() | Flags::RESERVED());
    }

    #[test]
    pub fn bit_clears_sign_bit_if_bit_7_of_operand_is_not_set() {
        let mut cpu = init_cpu();
        cpu.registers.a = 0xFF;
        cpu.flags.set(Flags::SIGN() | Flags::RESERVED());
        bit::exec(&mut cpu, Operand::Immediate(0x01)).unwrap();
        assert_eq!(cpu.flags, Flags::RESERVED());
    }

    #[test]
    pub fn bit_sets_overflow_bit_if_bit_6_of_operand_is_set() {
        let mut cpu = init_cpu();
        cpu.registers.a = 0xFF;
        bit::exec(&mut cpu, Operand::Immediate(0x40)).unwrap();
        assert_eq!(cpu.flags, Flags::OVERFLOW() | Flags::RESERVED());
    }

    #[test]
    pub fn bit_clears_overflow_bit_if_bit_6_of_operand_is_not_set() {
        let mut cpu = init_cpu();
        cpu.registers.a = 0xFF;
        cpu.flags.set(Flags::OVERFLOW() | Flags::RESERVED());
        bit::exec(&mut cpu, Operand::Immediate(0x01)).unwrap();
        assert_eq!(cpu.flags, Flags::RESERVED());
    }

    #[test]
    pub fn bit_sets_zero_flag_if_result_of_masking_operand_with_a_is_zero() {
        let mut cpu = init_cpu();
        cpu.registers.a = 0x02;
        bit::exec(&mut cpu, Operand::Immediate(0x01)).unwrap();
        assert_eq!(cpu.flags, Flags::ZERO() | Flags::RESERVED());
    }

    #[test]
    pub fn bit_clears_zero_flag_if_result_of_masking_operand_with_a_is_nonzero() {
        let mut cpu = init_cpu();
        cpu.registers.a = 0x02;
        cpu.flags.set(Flags::ZERO() | Flags::RESERVED());
        bit::exec(&mut cpu, Operand::Immediate(0x03)).unwrap();
        assert_eq!(cpu.flags, Flags::RESERVED());
    }

    fn init_cpu() -> Mos6502<VirtualMemory<'static>> {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm);

        cpu.pc.set(0xABCD);

        cpu
    }
}
