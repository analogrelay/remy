use slog;
use hw::mos6502::exec;
use hw::mos6502::{Mos6502,Flags};

pub fn exec(cpu: &mut Mos6502, flag_selector: Flags, log: &slog::Logger) -> Result<(), exec::Error> {
    trace!(log, "cpu" => cpu, "flag" => flag_selector; "clearing {:?}", flag_selector);
    cpu.flags.clear(flag_selector);
    Ok(())
}

#[cfg(test)]
mod test {
    use slog;
    use hw::mos6502::exec::clear_flag;
    use hw::mos6502::{Mos6502,Flags};

    #[test]
    pub fn clear_flag_clears_flag() {
        let mut cpu = Mos6502::new();
        cpu.pc.set(0xABCD);
        cpu.flags.set(Flags::CARRY() | Flags::SIGN());
        clear_flag::exec(&mut cpu, Flags::CARRY(), &slog::Logger::root(slog::Discard, o!())).unwrap();
        assert!(!cpu.flags.intersects(Flags::CARRY()));
        assert!(cpu.flags.intersects(Flags::SIGN()));
    }
}
