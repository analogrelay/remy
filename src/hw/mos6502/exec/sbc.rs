use slog;
use mem::Memory;
use hw::mos6502::exec;
use hw::mos6502::{Operand,Mos6502,Flags};

pub fn exec<M>(cpu: &mut Mos6502, mem: &M, op: Operand, log: &slog::Logger) -> Result<(), exec::Error> where M: Memory {
    let m = try_log!(op.get_u8(cpu, mem), log);
    let a = cpu.registers.a;
    let c = if cpu.flags.carry() { 0 } else { 1 };

    if cpu.bcd_enabled && cpu.flags.intersects(Flags::BCD()) {
        unimplemented!()
    }

    let t = (a as i16) - (m as i16) - (c as i16);
    let r = t as u8;

    trace!(log, "cpu" => cpu,
        "a" => a,
        "m" => m,
        "c" => c,
        "r" => r;
        "evaluated a - m - c = r");

    if cpu.flags.set_if(Flags::CARRY(), t >= 0) {
        trace!(log, "cpu" => cpu; "setting CARRY");
    } else {
        trace!(log, "cpu" => cpu; "clearing CARRY");
    }

    if cpu.flags.set_if(Flags::OVERFLOW(), ((cpu.registers.a ^ r) & 0x80 != 0) && ((cpu.registers.a ^ m) & 0x80 == 0x80)) {
        trace!(log, "cpu" => cpu; "setting OVERFLOW");
    } else {
        trace!(log, "cpu" => cpu; "clearing OVERFLOW");
    }

    cpu.registers.a = r;
    cpu.flags.set_sign_and_zero(cpu.registers.a);
    trace!(log, "cpu" => cpu; "stored result in A");

    Ok(())
}

#[cfg(test)]
mod test {
    use mem;
    use hw::mos6502::exec::sbc;
    use hw::mos6502::{Mos6502,Operand,Flags};

    // The "Borrow" psuedo-flag is defined as !Carry
    // Thus, when Carry is SET, NO Borrow is performed
    // When Carry is CLEAR, A Borrow is performed

    #[test]
    pub fn sbc_subtracts_regularly_when_carry_set() {
        let mut cpu = init_cpu();
        cpu.flags.set(Flags::CARRY()); // Set CARRY()
        sbc::exec(&mut cpu, &mut mem::Empty, Operand::Immediate(1)).unwrap();
        assert_eq!(cpu.registers.a, 41);
    }

    #[test]
    pub fn sbc_borrows_when_carry_flag_is_not_set() {
        let mut cpu = init_cpu();
        sbc::exec(&mut cpu, &mut mem::Empty, Operand::Immediate(1)).unwrap();
        assert_eq!(cpu.registers.a, 40);
    }

    #[test]
    pub fn sbc_sets_flags_when_overflow() {
        let mut cpu = init_cpu();
        cpu.registers.a = 0x80;
        sbc::exec(&mut cpu, &mut mem::Empty, Operand::Immediate(0x00)).unwrap();
        assert_eq!(cpu.registers.a, 0x7F);
        assert_eq!(cpu.flags, Flags::CARRY() | Flags::OVERFLOW() | Flags::RESERVED());
    }

    fn init_cpu() -> Mos6502 {
        let mut cpu = Mos6502::new();
        cpu.registers.a = 42;
        cpu
    }
}
