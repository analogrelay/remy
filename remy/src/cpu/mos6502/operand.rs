use std::error;

use mem;
use mem::Memory;

use cpu::mos6502;
use cpu::mos6502::Mos6502;

#[derive(Copy,Debug,Eq,PartialEq)]
pub enum Operand {
    Accumulator,
    Immediate(u8),
    Absolute(u16),
    Indexed(u16, mos6502::RegisterName),
    Indirect(u16),
    PreIndexedIndirect(u8),
    PostIndexedIndirect(u8),
    Register(mos6502::RegisterName)
}

#[derive(Clone,Debug,Eq,PartialEq)]
pub enum OperandError {
    ErrorAccessingMemory(mem::MemoryError),
    OperandSizeMismatch,
    ReadOnlyOperand,
    NonAddressOperand
}

impl error::FromError<mem::MemoryError> for OperandError {
    fn from_error(err: mem::MemoryError) -> OperandError {
        OperandError::ErrorAccessingMemory(err)
    }
}

impl Operand {
    pub fn get_u8<M>(&self, cpu: &Mos6502<M>) -> Result<u8, OperandError> where M: mem::Memory {
        Ok(match *self {
            Operand::Accumulator                => cpu.registers.a,
            Operand::Immediate(n)               => n,
            _                                   => try!(cpu.mem.get_u8(try!(self.get_addr(cpu)) as usize))
        })
    }

    pub fn set_u8<M>(&self, cpu: &mut Mos6502<M>, val: u8) -> Result<(), OperandError> where M: mem::Memory {
        match *self {
            Operand::Accumulator        => Ok(cpu.registers.a = val),
            Operand::Absolute(addr)     => Ok(try!(cpu.mem.set_u8(addr as usize, val))),
            Operand::Indexed(addr, r)   => Ok(try!(cpu.mem.set_u8(addr as usize + cpu.registers.get(r) as usize, val))),
            Operand::Register(r)        => Ok(cpu.registers.set(r, val)),
            _                           => Err(OperandError::ReadOnlyOperand)
        }
    }

    pub fn get_addr<M>(&self, cpu: &Mos6502<M>) -> Result<u16, OperandError> where M: mem::Memory {
        Ok(match *self {
            Operand::Absolute(addr)             => addr,
            Operand::Indirect(addr)             => try!(cpu.mem.get_le_u16(addr as usize)),
            Operand::Indexed(addr, r)           => addr + cpu.registers.get(r) as u16,
            Operand::PreIndexedIndirect(addr)   => try!(cpu.mem.get_le_u16(addr as usize + cpu.registers.x as usize)),
            Operand::PostIndexedIndirect(addr)  => try!(cpu.mem.get_le_u16(addr as usize)) + cpu.registers.y as u16,
            _                                   => return Err(OperandError::NonAddressOperand)
        })
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
            assert!(Operand::Accumulator.set_u8(&mut cpu, 24).is_ok());
            assert_eq!(cpu.registers.a, 24);
        }

        #[test]
        pub fn set_absolute_puts_value_in_memory_location() {
            let mut cpu = Mos6502::with_fixed_memory(10);
            assert!(Operand::Absolute(2).set_u8(&mut cpu, 24).is_ok());
            let val : u8 = cpu.mem.get_u8(2).unwrap();
            assert_eq!(val, 24);
        }

        #[test]
        pub fn set_indexed_x_puts_value_in_memory_location() {
            let mut cpu = Mos6502::with_fixed_memory(10);
            cpu.registers.x = 1;
            assert!(Operand::Indexed(2, RegisterName::X).set_u8(&mut cpu, 24).is_ok());
            let val : u8 = cpu.mem.get_u8(3).unwrap();
            assert_eq!(val, 24);
        }

        #[test]
        pub fn set_indexed_y_puts_value_in_memory_location() {
            let mut cpu = Mos6502::with_fixed_memory(10);
            cpu.registers.y = 1;
            assert!(Operand::Indexed(2, RegisterName::Y).set_u8(&mut cpu, 24).is_ok());
            let val : u8 = cpu.mem.get_u8(3).unwrap();
            assert_eq!(val, 24);
        }

        #[test]
        pub fn get_accumulator_returns_value_of_accumulator() {
            let mut cpu = Mos6502::with_fixed_memory(10);
            cpu.registers.a = 42;
            let val = Operand::Accumulator.get_u8(&cpu).unwrap();
            assert_eq!(val, 42);
        }

        #[test]
        pub fn get_immediate_returns_immediate_value() {
            let cpu = Mos6502::with_fixed_memory(10);
            let val = Operand::Immediate(42).get_u8(&cpu).unwrap();
            assert_eq!(val, 42);
        }

        #[test]
        pub fn get_absolute_returns_value_from_memory_address() {
            let mut cpu = Mos6502::with_fixed_memory(10);
            assert!(cpu.mem.set_u8(4, 42).is_ok());
            let val = Operand::Absolute(4).get_u8(&cpu).unwrap();
            assert_eq!(val, 42);
        }

        #[test]
        pub fn get_indexed_x_adds_x_to_address() {
            let mut cpu = Mos6502::with_fixed_memory(10);
            assert!(cpu.mem.set_u8(4, 42).is_ok());
            cpu.registers.x = 2;
            let val = Operand::Indexed(2, RegisterName::X).get_u8(&cpu).unwrap();
            assert_eq!(val, 42);
        }

        #[test]
        pub fn get_indexed_y_adds_y_to_address() {
            let mut cpu = Mos6502::with_fixed_memory(10);
            assert!(cpu.mem.set_u8(4, 42).is_ok());
            cpu.registers.y = 2;
            let val = Operand::Indexed(2, RegisterName::Y).get_u8(&cpu).unwrap();
            assert_eq!(val, 42);
        }

        #[test]
        pub fn get_preindexed_indirect_works() {
            let mut cpu = Mos6502::with_fixed_memory(10);
            assert!(cpu.mem.set_u8(8, 42).is_ok()); // Value
            assert!(cpu.mem.set_le_u16(6, 8).is_ok()); // Indirect Memory Address
            cpu.registers.x = 2;
            let val = Operand::PreIndexedIndirect(4).get_u8(&cpu).unwrap();
            assert_eq!(val, 42);
        }

        #[test]
        pub fn get_postindexed_indirect_works() {
            let mut cpu = Mos6502::with_fixed_memory(10);
            assert!(cpu.mem.set_u8(8, 42).is_ok()); // Value
            assert!(cpu.mem.set_le_u16(2, 6).is_ok()); // Indirect Memory Address
            cpu.registers.y = 2;
            let val = Operand::PostIndexedIndirect(2).get_u8(&cpu).unwrap();
            assert_eq!(val, 42);
        }
    }
}
