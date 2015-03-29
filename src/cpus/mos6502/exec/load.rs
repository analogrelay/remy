use mem::Memory;
use cpus::mos6502::{exec, cpu};
use cpus::mos6502::{Mos6502,Operand};

pub fn exec<M>(cpu: &mut Mos6502<M>, reg: cpu::RegisterName, op: Operand) -> exec::Result where M: Memory {
    let val = try!(op.get_u8(cpu));
    reg.set(cpu, val);
    cpu.flags.set_sign_and_zero(val);
    Ok(())
}

pub fn las<M>(cpu: &mut Mos6502<M>, op: Operand) -> exec::Result where M: Memory {
    let val = try!(op.get_u8(cpu)) & cpu.registers.sp;
    cpu.registers.a = val;
    cpu.registers.x = val;
    cpu.registers.sp = val;
    cpu.flags.set_sign_and_zero(val);
    Ok(())
}

#[cfg(test)]
mod test {
    use mem::VirtualMemory;
    use cpus::mos6502::exec::load;
    use cpus::mos6502::{cpu,Mos6502,Flags,Operand};

    #[test]
    pub fn load_sets_register_to_operand_value() {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm); 

        load::exec(&mut cpu, cpu::RegisterName::A, Operand::Immediate(42)).unwrap();

        assert_eq!(42, cpu.registers.a);
    }

    #[test]
    fn load_sets_sign_flag_if_new_value_is_negative() {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm); 
        load::exec(&mut cpu, cpu::RegisterName::A, Operand::Immediate(-10)).unwrap();
        assert!(cpu.flags.intersects(Flags::SIGN()));
    }

    #[test]
    fn load_clears_sign_flag_if_new_value_is_not_negative() {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm); 
        cpu.flags.set(Flags::SIGN());
        load::exec(&mut cpu, cpu::RegisterName::A, Operand::Immediate(0)).unwrap();
        assert!(!cpu.flags.intersects(Flags::SIGN()));
    }

    #[test]
    fn load_sets_zero_flag_if_new_value_is_zero() {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm); 
        load::exec(&mut cpu, cpu::RegisterName::A, Operand::Immediate(0)).unwrap();
        assert!(cpu.flags.intersects(Flags::ZERO()));
    }

    #[test]
    fn load_clears_zero_flag_if_new_value_is_nonzero() {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm); 
        cpu.flags.set(Flags::ZERO());
        load::exec(&mut cpu, cpu::RegisterName::A, Operand::Immediate(10)).unwrap();
        assert!(!cpu.flags.intersects(Flags::ZERO()));
    }

    #[test]
    fn las_loads_a_x_and_sp_with_operand_and_current_sp() {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm); 
        cpu.registers.sp = 0x3C;
        load::las(&mut cpu, Operand::Immediate(0xF0)).unwrap();

        assert_eq!(0x30, cpu.registers.a);
        assert_eq!(0x30, cpu.registers.x);
        assert_eq!(0x30, cpu.registers.sp);
    }

    #[test]
    fn las_sets_sign_flag_if_new_value_is_negative() {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm); 
        cpu.registers.sp = 0xF0;

        load::las(&mut cpu, Operand::Immediate(0xF0)).unwrap();

        assert_eq!(Flags::SIGN() | Flags::RESERVED(), cpu.flags);
    }

    #[test]
    fn las_clears_sign_flag_if_new_value_is_non_negative() {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm); 
        cpu.flags.set(Flags::SIGN());
        cpu.registers.sp = 0x70;

        load::las(&mut cpu, Operand::Immediate(0xF0)).unwrap();

        assert_eq!(Flags::RESERVED(), cpu.flags);
    }

    #[test]
    fn las_sets_zero_flag_if_new_value_is_zero() {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm); 
        cpu.registers.sp = 0xF0;

        load::las(&mut cpu, Operand::Immediate(0x0F)).unwrap();

        assert_eq!(Flags::ZERO() | Flags::RESERVED(), cpu.flags);
    }

    #[test]
    fn las_clears_zero_flag_if_new_value_is_non_zero() {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm); 
        cpu.flags.set(Flags::ZERO());
        cpu.registers.sp = 0x70;

        load::las(&mut cpu, Operand::Immediate(0xF0)).unwrap();

        assert_eq!(Flags::RESERVED(), cpu.flags);
    }
}
