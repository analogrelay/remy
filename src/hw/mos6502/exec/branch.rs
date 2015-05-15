use hw::mos6502::{exec,Mos6502,Flags,Operand};

pub fn if_clear(cpu: &mut Mos6502, op: Operand, flags: Flags) -> Result<(), exec::Error> {
    if let Operand::Offset(offset) = op {
        if !cpu.flags.intersects(flags) {
            let target = calc_target_and_tick_clock(cpu, offset);
            cpu.pc.set(target);
        }
        Ok(())
    } else {
        Err(exec::Error::IllegalOperand)
    }
}

pub fn if_set(cpu: &mut Mos6502, op: Operand, flags: Flags) -> Result<(), exec::Error> {
    if let Operand::Offset(offset) = op {
        if cpu.flags.intersects(flags) {
            let target = calc_target_and_tick_clock(cpu, offset);
            cpu.pc.set(target);
        }
        Ok(())
    } else {
        Err(exec::Error::IllegalOperand)
    }
}

fn calc_target_and_tick_clock(cpu: &mut Mos6502, offset: i8) -> u64 {
    // Check if we're jumping pages
    let current = cpu.pc.get();
    let target = ((current as i64) + (offset as i64)) as u64;
    if (current & 0xFF00) == (target & 0xFF00) {
        cpu.clock.tick(1);
    } else {
        cpu.clock.tick(2);
    }
    target
}

#[cfg(test)]
mod test {
    use hw::mos6502::exec::branch;
    use hw::mos6502::{Mos6502,Flags,Operand};

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
