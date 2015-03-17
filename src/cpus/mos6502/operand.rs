use std::{error,fmt};

use mem;
use mem::Memory;

use cpus::mos6502;
use cpus::mos6502::Mos6502;

/// Represents an operand that can be provided to an instruction
#[derive(Copy,Debug,Eq,PartialEq)]
pub enum Operand {
    /// Indicates an operand provided inline with the instruction
    Immediate(u8),
    /// Indicates an operand stored in the Accumulator (A register) 
    Accumulator,
    /// Indicates an operand stored at the provided memory address
    ///
    /// If the provided address is `m`, this operand is defined as `*m`
    Absolute(u16),
    /// Indicates an operand stored at the provided index from the current value of the provided
    /// register
    ///
    /// If the provided address is `m`, this operand is defined as `*(m+x)` or `*(m+y)` depending
    /// on the register specified
    Indexed(u16, mos6502::RegisterName),
    /// Indicates an operand stored at an address stored in the provided address
    ///
    /// If the provided address is `m`, this operand is defined as `**m`
    Indirect(u16),
    /// Indicates an operand stored at an address stored in the provided address (indexed by the
    /// `X` register)
    ///
    /// If the provided address is `m`, this operand is defined as `**(m+x)`
    PreIndexedIndirect(u8),
    /// Indicates an operand stored at an address (indexed by the `Y` register) stored in the provided address
    ///
    /// If the provided address is `x`, this operand is defined as `*(*m+y)`
    PostIndexedIndirect(u8)
}

/// Represents an error that occurred which accessing an `Operand`
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum OperandError {
    /// Indicates an error occurred reading or writing memory
    ErrorAccessingMemory(mem::MemoryError),
    /// Indicates that a request was made to write to a read-only operand such as
    /// `Operand::Immediate`
    ReadOnlyOperand,
    /// Indicates that a request was made to take the address of a non-addressable operand
    /// such as `Operand::Immediate`
    NonAddressOperand
}

impl error::FromError<mem::MemoryError> for OperandError {
    fn from_error(err: mem::MemoryError) -> OperandError {
        OperandError::ErrorAccessingMemory(err)
    }
}

impl Operand {
    /// Retrieves the operand value
    ///
    /// # Arguments
    ///
    /// * `cpu` - The cpu from which to get the operand value
    pub fn get_u8<M>(&self, cpu: &Mos6502<M>) -> Result<u8, OperandError> where M: mem::Memory {
        Ok(match self {
            &Operand::Immediate(n)      => n,
            &Operand::Accumulator       => cpu.registers.a,
            _                           => try!(cpu.mem.get_u8(try!(self.get_addr(cpu)) as usize))
        })
    }

    /// Sets the value of the operand on the specified cpu
    ///
    /// # Arguments
    ///
    /// * `cpu` - The cpu on which to set the operand value
    /// * `val` - The value to set the operand to
    pub fn set_u8<M>(&self, cpu: &mut Mos6502<M>, val: u8) -> Result<(), OperandError> where M: mem::Memory {
        match self {
            &Operand::Absolute(addr)     => Ok(try!(cpu.mem.set_u8(addr as usize, val))),
            &Operand::Indexed(addr, r)   => {
                let rv = r.get(cpu) as usize;
                Ok(try!(cpu.mem.set_u8(addr as usize + rv, val)))
            }
            &Operand::Accumulator        => { cpu.registers.a = val; Ok(()) },
            _                            => Err(OperandError::ReadOnlyOperand)
        }
    }

    /// Retrieves the address of the operand on the specified cpu
    ///
    /// # Arguments
    ///
    /// * `cpu` - The cpu on which to get the operand value
    pub fn get_addr<M>(&self, cpu: &Mos6502<M>) -> Result<u16, OperandError> where M: mem::Memory {
        Ok(match self {
            &Operand::Absolute(addr)             => addr,
            &Operand::Indirect(addr)             => try!(cpu.mem.get_le_u16(addr as usize)),
            &Operand::Indexed(addr, r)           => addr + r.get(cpu) as u16,
            &Operand::PreIndexedIndirect(addr)   => try!(cpu.mem.get_le_u16(addr as usize + cpu.registers.x as usize)),
            &Operand::PostIndexedIndirect(addr)  => try!(cpu.mem.get_le_u16(addr as usize)) + cpu.registers.y as u16,
            _                                   => return Err(OperandError::NonAddressOperand)
        })
    }
}

impl fmt::Display for Operand {
    /// Returns a string representing the instruction
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &Operand::Immediate(val)             => formatter.write_fmt(format_args!("#${:X}", val)),
            &Operand::Accumulator                => formatter.write_str("A"),
            &Operand::Absolute(val)              => formatter.write_fmt(format_args!("${:X}", val)),
            &Operand::Indexed(val, reg)          => formatter.write_fmt(format_args!("${:X},{}", val, reg)),
            &Operand::Indirect(val)              => formatter.write_fmt(format_args!("${:X}", val)),
            &Operand::PreIndexedIndirect(val)    => formatter.write_fmt(format_args!("(${:X},X)", val)),
            &Operand::PostIndexedIndirect(val)   => formatter.write_fmt(format_args!("(${:X}),Y", val))
        }
    }
}

#[cfg(test)]
mod test {
    mod operand {
        use mem::Memory;
        use cpus::mos6502::{Mos6502,Operand,RegisterName};

        #[test]
        pub fn to_string_test() {
            assert_eq!("#$AB", Operand::Immediate(0xAB).to_string());
            assert_eq!("A", Operand::Accumulator.to_string());
            assert_eq!("$ABCD", Operand::Absolute(0xABCD).to_string());
            assert_eq!("$AB", Operand::Absolute(0x00AB).to_string());
            assert_eq!("$ABCD,X", Operand::Indexed(0xABCD, RegisterName::X).to_string());
            assert_eq!("$ABCD,Y", Operand::Indexed(0xABCD, RegisterName::Y).to_string());
            assert_eq!("$AB,X", Operand::Indexed(0x00AB, RegisterName::X).to_string());
            assert_eq!("$AB,Y", Operand::Indexed(0x00AB, RegisterName::Y).to_string());
            assert_eq!("$ABCD", Operand::Indirect(0xABCD).to_string());
            assert_eq!("($AB,X)", Operand::PreIndexedIndirect(0xAB).to_string());
            assert_eq!("($AB),Y", Operand::PostIndexedIndirect(0xAB).to_string());
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
        pub fn set_accumulator_puts_value_in_accumulator() {
            let mut cpu = Mos6502::with_fixed_memory(10);
            cpu.registers.a = 24;
            Operand::Accumulator.set_u8(&mut cpu, 42).unwrap();
            assert_eq!(cpu.registers.a, 42);
        }

        #[test]
        pub fn get_accumulator_returns_value_from_accumulator() {
            let mut cpu = Mos6502::with_fixed_memory(10);
            cpu.registers.a = 42;
            assert_eq!(Ok(42), Operand::Accumulator.get_u8(&mut cpu));
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
