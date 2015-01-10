use std::error;

use mem;

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
    PostIndexedIndirect(u8),
    Relative(i8)
}

#[derive(Show,PartialEq)]
pub enum OperandError {
    ErrorAccessingMemory(mem::MemoryError),
    OperandSizeMismatch
}

impl error::FromError<mem::MemoryError> for OperandError {
    fn from_error(err: mem::MemoryError) -> OperandError {
        OperandError::ErrorAccessingMemory(err)
    }
}

impl Operand {
    pub fn get<M: mem::Memory<u16>>(self, cpu: &Mos6502<M>) -> Result<u8, OperandError> {
        Ok(match self {
            Operand::Accumulator                =>  cpu.registers.a,
            Operand::Immediate(n)               => n,
            Operand::Absolute(addr)             => try!(cpu.mem.get_u8(addr)),
            Operand::Indexed(addr, r)           => try!(cpu.mem.get_u8(addr + cpu.registers.get(r) as u16)),
            Operand::PreIndexedIndirect(addr)   => try!(cpu.mem.get_u8(try!(cpu.mem.get_u16(addr as u16 + cpu.registers.x as u16)))),
            Operand::PostIndexedIndirect(addr)  => try!(cpu.mem.get_u8(try!(cpu.mem.get_u16(addr as u16)) + cpu.registers.y as u16)),
            _                                   => return Err(OperandError::OperandSizeMismatch),
        })
    }

    pub fn get_u16<M: mem::Memory<u16>>(self, cpu: &Mos6502<M>) -> Result<u16, OperandError> {
        match self {
            Operand::Indirect(addr)     => Ok(try!(cpu.mem.get_u16(try!(cpu.mem.get_u16(addr))))),
            Operand::Relative(offset)   => Ok(((cpu.registers.pc as isize) + (offset as isize)) as u16),
            _                           => Err(OperandError::OperandSizeMismatch)
        }
    }
}

#[cfg(test)]
mod test {
    mod operand {
        use mem::Memory;
        use cpu::mos6502::{Mos6502,Operand,OperandError,RegisterName};

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
            assert!(cpu.mem.set_u8(4, 42).is_ok());
            let val = Operand::Absolute(4).get(&cpu).unwrap();
            assert_eq!(val, 42);
        }

        #[test]
        pub fn get_indexed_x_adds_x_to_address() {
            let mut cpu = Mos6502::with_fixed_memory(10);
            assert!(cpu.mem.set_u8(4, 42).is_ok());
            cpu.registers.x = 2;
            let val = Operand::Indexed(2, RegisterName::X).get(&cpu).unwrap();
            assert_eq!(val, 42);
        }

        #[test]
        pub fn get_indexed_y_adds_y_to_address() {
            let mut cpu = Mos6502::with_fixed_memory(10);
            assert!(cpu.mem.set_u8(4, 42).is_ok());
            cpu.registers.y = 2;
            let val = Operand::Indexed(2, RegisterName::Y).get(&cpu).unwrap();
            assert_eq!(val, 42);
        }

        #[test]
        pub fn get_preindexed_indirect_works() {
            let mut cpu = Mos6502::with_fixed_memory(10);
            assert!(cpu.mem.set_u8(9, 42).is_ok()); // Value
            assert!(cpu.mem.set_u16(7, 9).is_ok()); // Indirect Memory Address
            cpu.registers.x = 2;
            let val = Operand::PreIndexedIndirect(5).get(&cpu).unwrap();
            assert_eq!(val, 42);
        }

        #[test]
        pub fn get_postindexed_indirect_works() {
            let mut cpu = Mos6502::with_fixed_memory(10);
            assert!(cpu.mem.set_u8(9, 42).is_ok()); // Value
            assert!(cpu.mem.set_u16(7, 7).is_ok()); // Indirect Memory Address
            cpu.registers.y = 2;
            let val = Operand::PostIndexedIndirect(7).get(&cpu).unwrap();
            assert_eq!(val, 42);
        }

        #[test]
        pub fn get_u16_indirect_reads_indirect_u16_value() {
            let mut cpu = Mos6502::with_fixed_memory(10);
            assert!(cpu.mem.set_u16(8, 1024).is_ok()); // Value
            assert!(cpu.mem.set_u16(2, 8).is_ok());    // Indirect Memory Address
            let val = Operand::Indirect(2).get_u16(&cpu).unwrap();
            assert_eq!(val, 1024);
        }

        #[test]
        pub fn get_u16_relative_adds_positive_value_to_pc() {
            let mut cpu = Mos6502::with_fixed_memory(10);
            cpu.registers.pc = 1024;
            let val = Operand::Relative(1).get_u16(&cpu).unwrap();
            assert_eq!(val, 1025);
        }

        #[test]
        pub fn get_u16_relative_adds_negative_value_to_pc() {
            let mut cpu = Mos6502::with_fixed_memory(10);
            cpu.registers.pc = 1024;
            let val = Operand::Relative(-1).get_u16(&cpu).unwrap();
            assert_eq!(val, 1023);
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
        pub fn get_returns_error_if_given_relative_operand() {
            let cpu = Mos6502::with_fixed_memory(10);
            let val = Operand::Relative(-1).get(&cpu).unwrap_err();
            assert_eq!(val, OperandError::OperandSizeMismatch);
        }

        #[test]
        pub fn get_returns_error_if_given_indirect_operand() {
            let cpu = Mos6502::with_fixed_memory(10);
            let val = Operand::Indirect(1).get(&cpu).unwrap_err();
            assert_eq!(val, OperandError::OperandSizeMismatch);
        }
    }
}