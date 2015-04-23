use mem::Memory;
use cpus::mos6502::{exec,Mos6502,Flags};

pub fn if_clear<M>(cpu: &mut Mos6502<M>, offset: i8, flags: Flags) -> Result<(), exec::Error> where M: Memory {
    if !cpu.flags.intersects(flags) {
        cpu.pc.advance(offset as i64);
    }
    Ok(())
}

pub fn if_set<M>(cpu: &mut Mos6502<M>, offset: i8, flags: Flags) -> Result<(), exec::Error> where M: Memory {
    if cpu.flags.intersects(flags) {
        cpu.pc.advance(offset as i64);
    }
    Ok(())
}

#[cfg(test)]
mod test {
	use cpus::mos6502::exec::branch;
	use cpus::mos6502::{Mos6502,Flags};

    #[test]
    pub fn if_clear_does_not_modify_pc_if_flag_set() {
        let mut cpu = Mos6502::without_memory();
        cpu.pc.set(0xABCD);
        cpu.flags.set(Flags::CARRY() | Flags::SIGN());
        branch::if_clear(&mut cpu, 1, Flags::CARRY()).unwrap();
        assert_eq!(cpu.pc.get(), 0xABCD);
    }

    #[test]
    pub fn if_clear_advances_pc_by_specified_amount_if_flag_clear() {
        let mut cpu = Mos6502::without_memory();
        cpu.pc.set(0xABCD);
        branch::if_clear(&mut cpu, 1, Flags::CARRY()).unwrap();
        assert_eq!(cpu.pc.get(), 0xABCE);
    }

    #[test]
    pub fn if_set_does_not_modify_pc_if_flag_clear() {
        let mut cpu = Mos6502::without_memory();
        cpu.pc.set(0xABCD);
        branch::if_set(&mut cpu, 1, Flags::CARRY()).unwrap();
        assert_eq!(cpu.pc.get(), 0xABCD);
    }

    #[test]
    pub fn if_set_advances_pc_by_specified_amount_if_flag_set() {
        let mut cpu = Mos6502::without_memory();
        cpu.pc.set(0xABCD);
        cpu.flags.set(Flags::CARRY() | Flags::SIGN());
        branch::if_set(&mut cpu, 1, Flags::CARRY()).unwrap();
        assert_eq!(cpu.pc.get(), 0xABCE);
    }
}
