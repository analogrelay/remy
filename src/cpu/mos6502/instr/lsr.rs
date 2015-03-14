use mem::Memory;
use cpu::mos6502::{ExecError,Mos6502,Operand,Flags};

pub fn exec<M>(cpu: &mut Mos6502<M>, op: Operand) -> Result<(), ExecError> where M : Memory {
    let n = try!(op.get_u8(cpu));
    cpu.flags.clear(Flags::SIGN());
    cpu.flags.set_if(Flags::CARRY(), (n & 0x01) != 0);
    let m = (n >> 1) & 0x7F;
    cpu.flags.set_if(Flags::ZERO(), m == 0);
    try!(op.set_u8(cpu, m));
    Ok(())
}

#[cfg(test)]
mod test {
    use mem::VirtualMemory;
<<<<<<< master:src/cpu/mos6502/instr/lsr.rs
    use cpu::mos6502::instr::lsr;
    use cpu::mos6502::{Mos6502,Operand,Flags,RegisterName};
=======
    use cpus::mos6502::instr::lsr;
    use cpus::mos6502::{Mos6502,Operand,Flags};
>>>>>>> local:src/cpus/mos6502/instr/lsr.rs

    #[test]
    pub fn lsr_clears_sign_flag() {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm);
        cpu.flags.set(Flags::SIGN() | Flags::ZERO() | Flags::CARRY()); 

        lsr::exec(&mut cpu, Operand::Accumulator).unwrap();

        assert!(!cpu.flags.intersects(Flags::SIGN()));
    }

    #[test]
    pub fn lsr_sets_carry_flag_if_first_bit_of_operand_is_one() {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm);
        cpu.flags.set(Flags::SIGN() | Flags::ZERO()); 
        cpu.registers.a = 0b10101011;

        lsr::exec(&mut cpu, Operand::Accumulator).unwrap();

        assert!(cpu.flags.intersects(Flags::CARRY()));
    }

    #[test]
    pub fn lsr_clears_carry_flag_if_first_bit_of_operand_is_zero() {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm);
        cpu.flags.set(Flags::SIGN() | Flags::ZERO() | Flags::CARRY()); 
        cpu.registers.a = 0b10101010;

        lsr::exec(&mut cpu, Operand::Accumulator).unwrap();

        assert!(!cpu.flags.intersects(Flags::CARRY()));
    }

    #[test]
    pub fn lsr_clears_zero_flag_if_result_is_non_zero() {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm);
        cpu.flags.set(Flags::SIGN() | Flags::ZERO() | Flags::CARRY()); 
        cpu.registers.a = 0b10101010;

        lsr::exec(&mut cpu, Operand::Accumulator).unwrap();

        assert!(!cpu.flags.intersects(Flags::ZERO()));
    }

    #[test]
    pub fn lsr_sets_zero_flag_if_result_is_zero() {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm);
        cpu.flags.set(Flags::SIGN() | Flags::CARRY()); 
        cpu.registers.a = 0b00000000;

        lsr::exec(&mut cpu, Operand::Accumulator).unwrap();

        assert!(cpu.flags.intersects(Flags::ZERO()));
    }

    #[test]
    pub fn lsr_performs_logical_right_shift_and_stores_result() {
        let vm = VirtualMemory::new();
        let mut cpu = Mos6502::new(vm);
        cpu.registers.a = 0b10101010;

        lsr::exec(&mut cpu, Operand::Accumulator).unwrap();

        assert_eq!(0b01010101, cpu.registers.a);
    }
}
