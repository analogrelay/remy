use hw::mos6502::{exec,cpu};
use hw::mos6502::Mos6502;

pub fn exec(cpu: &mut Mos6502, src: cpu::RegisterName, dst: cpu::RegisterName) -> Result<(), exec::Error> {
    let val = src.get(cpu);
    dst.set(cpu, val);
    if dst != cpu::RegisterName::S {
        cpu.flags.set_sign_and_zero(val);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use hw::mos6502::exec::transfer;
    use hw::mos6502::{cpu,Mos6502,Flags};

    #[test]
    pub fn transfer_sets_destination_register_to_source_register_value() {
        let mut cpu = Mos6502::new();

        cpu.registers.a = 42;
        transfer::exec(&mut cpu, cpu::RegisterName::A, cpu::RegisterName::X).unwrap();

        assert_eq!(42, cpu.registers.x);
    }

    #[test]
    pub fn transfer_sets_sign_flag_when_value_transferred_is_negative() {
        let mut cpu = Mos6502::new();

        cpu.registers.a = 0xFF;
        transfer::exec(&mut cpu, cpu::RegisterName::A, cpu::RegisterName::X).unwrap();

        assert!(cpu.flags.intersects(Flags::SIGN()));
        assert_eq!(0xFF, cpu.registers.x);
    }

    #[test]
    pub fn transfer_does_not_set_sign_flag_when_transferring_to_sp() {
        let mut cpu = Mos6502::new();

        cpu.registers.a = 0xFF;
        transfer::exec(&mut cpu, cpu::RegisterName::A, cpu::RegisterName::S).unwrap();

        assert!(!cpu.flags.intersects(Flags::SIGN()));
        assert_eq!(0xFF, cpu.registers.sp);
    }

    #[test]
    pub fn transfer_clears_sign_flag_when_value_transferred_is_positive() {
        let mut cpu = Mos6502::new();

        cpu.flags.set(Flags::SIGN());
        cpu.registers.a = 0x0F;
        transfer::exec(&mut cpu, cpu::RegisterName::A, cpu::RegisterName::X).unwrap();

        assert!(!cpu.flags.intersects(Flags::SIGN()));
        assert_eq!(0x0F, cpu.registers.x);
    }

    #[test]
    pub fn transfer_sets_zero_flag_when_value_transferred_is_zero() {
        let mut cpu = Mos6502::new();

        cpu.registers.a = 0x00;
        transfer::exec(&mut cpu, cpu::RegisterName::A, cpu::RegisterName::X).unwrap();

        assert!(cpu.flags.intersects(Flags::ZERO()));
        assert_eq!(0x00, cpu.registers.x);
    }

    #[test]
    pub fn transfer_clears_zero_flag_when_value_transferred_is_non_zero() {
        let mut cpu = Mos6502::new();

        cpu.flags.set(Flags::ZERO());
        cpu.registers.a = 0x0F;
        transfer::exec(&mut cpu, cpu::RegisterName::A, cpu::RegisterName::X).unwrap();

        assert!(!cpu.flags.intersects(Flags::ZERO()));
        assert_eq!(0x0F, cpu.registers.x);
    }

    #[test]
    pub fn transfer_does_not_set_zero_flag_when_transferring_to_sp() {
        let mut cpu = Mos6502::new();

        cpu.registers.a = 0x00;
        transfer::exec(&mut cpu, cpu::RegisterName::A, cpu::RegisterName::S).unwrap();

        assert!(!cpu.flags.intersects(Flags::ZERO()));
        assert_eq!(0x00, cpu.registers.sp);
    }
}
