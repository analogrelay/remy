use cpus::mos6502::{exec,Mos6502,Flags,Operand};

pub fn if_clear(cpu: &mut Mos6502, op: Operand, flags: Flags) -> Result<(), exec::Error> {
    if let Operand::Offset(offset) = op {
        if !cpu.flags.intersects(flags) {
            cpu.pc.advance(offset as i64);
        }
        Ok(())
    } else {
        Err(exec::Error::IllegalOperand)
    }
}

pub fn if_set(cpu: &mut Mos6502, op: Operand, flags: Flags) -> Result<(), exec::Error> {
    if let Operand::Offset(offset) = op {
        if cpu.flags.intersects(flags) {
            cpu.pc.advance(offset as i64);
        }
        Ok(())
    } else {
        Err(exec::Error::IllegalOperand)
    }
}

#[cfg(test)]
mod test {
    use cpus::mos6502::exec::branch;
    use cpus::mos6502::{Mos6502,Flags,Operand};

    #[test]
    pub fn if_clear_does_not_modify_pc_if_flag_set() {
        let mut cpu = Mos6502::new();
        cpu.pc.set(0xABCD);
        cpu.flags.set(Flags::CARRY() | Flags::SIGN());
        branch::if_clear(&mut cpu, Operand::Offset(1), Flags::CARRY()).unwrap();
        assert_eq!(cpu.pc.get(), 0xABCD);
    }

    #[test]
    pub fn if_clear_advances_pc_by_specified_amount_if_flag_clear() {
        let mut cpu = Mos6502::new();
        cpu.pc.set(0xABCD);
        branch::if_clear(&mut cpu, Operand::Offset(1), Flags::CARRY()).unwrap();
        assert_eq!(cpu.pc.get(), 0xABCE);
    }

    #[test]
    pub fn if_set_does_not_modify_pc_if_flag_clear() {
        let mut cpu = Mos6502::new();
        cpu.pc.set(0xABCD);
        branch::if_set(&mut cpu, Operand::Offset(1), Flags::CARRY()).unwrap();
        assert_eq!(cpu.pc.get(), 0xABCD);
    }

    #[test]
    pub fn if_set_advances_pc_by_specified_amount_if_flag_set() {
        let mut cpu = Mos6502::new();
        cpu.pc.set(0xABCD);
        cpu.flags.set(Flags::CARRY() | Flags::SIGN());
        branch::if_set(&mut cpu, Operand::Offset(1), Flags::CARRY()).unwrap();
        assert_eq!(cpu.pc.get(), 0xABCE);
    }
}
