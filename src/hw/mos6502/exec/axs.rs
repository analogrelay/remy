use mem::Memory;
use hw::mos6502::exec;
use hw::mos6502::{Operand,Mos6502,Flags};

// X := A & X - op ; with sign, zero and carry set as appropriate
pub fn exec<M>(cpu: &mut Mos6502, mem: &M, op: Operand) -> exec::Result where M: Memory {
    let val = (cpu.registers.a & cpu.registers.x) - try!(op.get_u8(cpu, mem));
    cpu.flags.set_sign_and_zero(val);
    cpu.flags.set_if(Flags::CARRY(), (val & 0x80) != 0);
    cpu.registers.x = val;
    Ok(())
}

#[cfg(test)]
mod test {
    use mem;
    use hw::mos6502::exec::axs;
    use hw::mos6502::{Mos6502,Operand,Flags};

    #[test]
    pub fn axs_does_its_crazy_business() {
        let mut cpu = Mos6502::new();

        cpu.registers.a = 0x3C;
        cpu.registers.x = 0x33;
        axs::exec(&mut cpu, &mem::Empty, Operand::Immediate(0x01)).unwrap();
        assert_eq!(0x2F, cpu.registers.x);
    }

    #[test]
    pub fn axs_sets_carry_and_sign_if_result_bit_7_is_set() {
        let mut cpu = Mos6502::new();

        cpu.registers.a = 0xFF;
        cpu.registers.x = 0xFF;
        axs::exec(&mut cpu, &mem::Empty, Operand::Immediate(0x00)).unwrap();

        assert_eq!(0xFF, cpu.registers.x);
        assert_eq!(Flags::CARRY() | Flags::SIGN() | Flags::RESERVED(), cpu.flags);
    }

    #[test]
    pub fn axs_clears_carry_and_sign_if_result_bit_7_is_clear() {
        let mut cpu = Mos6502::new();
        cpu.flags.set(Flags::CARRY() | Flags::SIGN()); 

        cpu.registers.a = 0x01;
        cpu.registers.x = 0x01;
        axs::exec(&mut cpu, &mem::Empty, Operand::Immediate(0x00)).unwrap();

        assert_eq!(0x01, cpu.registers.x);
        assert_eq!(Flags::RESERVED(), cpu.flags);
    }

    #[test]
    pub fn axs_sets_zero_if_result_is_zero() {
        let mut cpu = Mos6502::new();

        cpu.registers.a = 0xFF;
        cpu.registers.x = 0x01;
        axs::exec(&mut cpu, &mem::Empty, Operand::Immediate(0x01)).unwrap();

        assert_eq!(0x00, cpu.registers.x);
        assert_eq!(Flags::ZERO() | Flags::RESERVED(), cpu.flags);
    }

    #[test]
    pub fn axs_clears_zero_if_result_is_non_zero() {
        let mut cpu = Mos6502::new();
        cpu.flags.set(Flags::ZERO()); 

        cpu.registers.a = 0x01;
        cpu.registers.x = 0x01;
        axs::exec(&mut cpu, &mem::Empty, Operand::Immediate(0x00)).unwrap();

        assert_eq!(0x01, cpu.registers.x);
        assert_eq!(Flags::RESERVED(), cpu.flags);
    }
}
