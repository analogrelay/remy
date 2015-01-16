use std::error;

use mem;

use pc;

use cpu::mos6502;
use cpu::mos6502::Mos6502;

#[derive(Show)]
pub enum Operand {
    Accumulator,
    Immediate(u8),
    Absolute(u16),
    Indexed(u16, mos6502::RegisterName),
    Indirect(u16),
    PreIndexedIndirect(u8),
    PostIndexedIndirect(u8)
}

#[derive(Show,PartialEq)]
pub enum OperandError {
    ErrorAccessingMemory(mem::MemoryError),
    OperandSizeMismatch,
    ReadOnlyOperand
}

impl error::FromError<mem::MemoryError> for OperandError {
    fn from_error(err: mem::MemoryError) -> OperandError {
        OperandError::ErrorAccessingMemory(err)
    }
}

impl Operand {
    pub fn get<M: mem::Memory>(&self, cpu: &Mos6502<M>) -> Result<u8, OperandError> {
        Ok(match *self {
            Operand::Accumulator                => cpu.registers.a,
            Operand::Immediate(n)               => n,
            Operand::Absolute(addr)             => try!(cpu.mem.get(addr as usize)),
            Operand::Indexed(addr, r)           => try!(cpu.mem.get(addr as usize + cpu.registers.get(r) as usize)),
            Operand::PreIndexedIndirect(addr)   => try!(cpu.mem.get(try!(cpu.mem.get::<u16>(addr as usize + cpu.registers.x as usize)) as usize)),
            Operand::PostIndexedIndirect(addr)  => try!(cpu.mem.get(try!(cpu.mem.get::<u16>(addr as usize)) as usize + cpu.registers.y as usize)),
            _                                   => return Err(OperandError::OperandSizeMismatch),
        })
    }

    pub fn get_u16<M: mem::Memory>(&self, cpu: &Mos6502<M>) -> Result<u16, OperandError> {
        match *self {
            Operand::Indirect(addr)     => Ok(try!(cpu.mem.get(try!(cpu.mem.get::<u16>(addr as usize)) as usize))),
            _                           => Err(OperandError::OperandSizeMismatch)
        }
    }

    pub fn set<M: mem::Memory>(&self, cpu: &mut Mos6502<M>, val: u8) -> Result<(), OperandError> {
        match *self {
            Operand::Accumulator        => Ok(cpu.registers.a = val),
            Operand::Absolute(addr)     => Ok(try!(cpu.mem.set(addr as usize, val))),
            Operand::Indexed(addr, r)   => Ok(try!(cpu.mem.set(addr as usize + cpu.registers.get(r) as usize, val))),
            _                           => Err(OperandError::ReadOnlyOperand)
        }
    }
}

#[cfg(test)]
mod test {
    mod operand {
        use mem::Memory;
        use cpu::mos6502::{Mos6502,Operand,OperandError,RegisterName};

        #[test]
        pub fn set_accumulator_puts_value_in_accumulator() {
            let mut cpu = Mos6502::with_fixed_memory(10);
            cpu.registers.a = 42;
            assert!(Operand::Accumulator.set(&mut cpu, 24).is_ok());
            assert_eq!(cpu.registers.a, 24);
        }

        #[test]
        pub fn set_absolute_puts_value_in_memory_location() {
            let mut cpu = Mos6502::with_fixed_memory(10);
            assert!(Operand::Absolute(2).set(&mut cpu, 24).is_ok());
            let val : u8 = cpu.mem.get(2).unwrap();
            assert_eq!(val, 24);
        }

        #[test]
        pub fn set_indexed_x_puts_value_in_memory_location() {
            let mut cpu = Mos6502::with_fixed_memory(10);
            cpu.registers.x = 1;
            assert!(Operand::Indexed(2, RegisterName::X).set(&mut cpu, 24).is_ok());
            let val : u8 = cpu.mem.get(3).unwrap();
            assert_eq!(val, 24);
        }

        #[test]
        pub fn set_indexed_y_puts_value_in_memory_location() {
            let mut cpu = Mos6502::with_fixed_memory(10);
            cpu.registers.y = 1;
            assert!(Operand::Indexed(2, RegisterName::Y).set(&mut cpu, 24).is_ok());
            let val : u8 = cpu.mem.get(3).unwrap();
            assert_eq!(val, 24);
        }

        #[test]
        pub fn get_accumulator_returns_value_of_accumulator() {
            let mut cpu = Mos6502::with_fixed_memory(10);
            cpu.registers.a = 42;
            let val = Operand::Accumulator.get(&cpu).unwrap();
            assert_eq!(val, 42);
        }

        #[test]
        pub fn get_immediate_returns_immediate_value() {
            let cpu = Mos6502::with_fixed_memory(10);
            let val = Operand::Immediate(42).get(&cpu).unwrap();
            assert_eq!(val, 42);
        }

        #[test]
        pub fn get_absolute_returns_value_from_memory_address() {
            let mut cpu = Mos6502::with_fixed_memory(10);
            assert!(cpu.mem.set(4, 42u8).is_ok());
            let val = Operand::Absolute(4).get(&cpu).unwrap();
            assert_eq!(val, 42);
        }

        #[test]
        pub fn get_indexed_x_adds_x_to_address() {
            let mut cpu = Mos6502::with_fixed_memory(10);
            assert!(cpu.mem.set(4, 42u8).is_ok());
            cpu.registers.x = 2;
            let val = Operand::Indexed(2, RegisterName::X).get(&cpu).unwrap();
            assert_eq!(val, 42);
        }

        #[test]
        pub fn get_indexed_y_adds_y_to_address() {
            let mut cpu = Mos6502::with_fixed_memory(10);
            assert!(cpu.mem.set(4, 42u8).is_ok());
            cpu.registers.y = 2;
            let val = Operand::Indexed(2, RegisterName::Y).get(&cpu).unwrap();
            assert_eq!(val, 42);
        }

        #[test]
        pub fn get_preindexed_indirect_works() {
            let mut cpu = Mos6502::with_fixed_memory(10);
            assert!(cpu.mem.set(8, 42u8).is_ok()); // Value
            assert!(cpu.mem.set(6, 8u16).is_ok()); // Indirect Memory Address
            cpu.registers.x = 2;
            let val = Operand::PreIndexedIndirect(4).get(&cpu).unwrap();
            assert_eq!(val, 42);
        }

        #[test]
        pub fn get_postindexed_indirect_works() {
            let mut cpu = Mos6502::with_fixed_memory(10);
            assert!(cpu.mem.set(8, 42u8).is_ok()); // Value
            assert!(cpu.mem.set(2, 6u16).is_ok()); // Indirect Memory Address
            cpu.registers.y = 2;
            let val = Operand::PostIndexedIndirect(2).get(&cpu).unwrap();
            assert_eq!(val, 42);
        }

        #[test]
        pub fn get_u16_indirect_reads_indirect_u16_value() {
            let mut cpu = Mos6502::with_fixed_memory(10);
            assert!(cpu.mem.set(7, 1024u16).is_ok()); // Value
            assert!(cpu.mem.set(2, 7u16).is_ok());    // Indirect Memory Address
            let val = Operand::Indirect(2).get_u16(&cpu).unwrap();
            assert_eq!(val, 1024);
        }

        #[test]
        pub fn get_u16_returns_error_if_given_u8_operand() {
            let cpu = Mos6502::with_fixed_memory(10);

            assert_eq!(Operand::Accumulator.get_u16(&cpu).unwrap_err(), OperandError::OperandSizeMismatch);
            assert_eq!(Operand::Immediate(42).get_u16(&cpu).unwrap_err(), OperandError::OperandSizeMismatch);
            assert_eq!(Operand::Absolute(42).get_u16(&cpu).unwrap_err(), OperandError::OperandSizeMismatch);
            assert_eq!(Operand::Indexed(42, RegisterName::X).get_u16(&cpu).unwrap_err(), OperandError::OperandSizeMismatch);
            assert_eq!(Operand::Indexed(42, RegisterName::Y).get_u16(&cpu).unwrap_err(), OperandError::OperandSizeMismatch);
            assert_eq!(Operand::PreIndexedIndirect(42).get_u16(&cpu).unwrap_err(), OperandError::OperandSizeMismatch);
            assert_eq!(Operand::PostIndexedIndirect(42).get_u16(&cpu).unwrap_err(), OperandError::OperandSizeMismatch);
        }

        #[test]
        pub fn get_returns_error_if_given_u16_operand() {
            let cpu = Mos6502::with_fixed_memory(10);
            let val = Operand::Indirect(1).get(&cpu).unwrap_err();
            assert_eq!(val, OperandError::OperandSizeMismatch);
        }
    }
}