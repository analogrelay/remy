use std::num;

use mem;

use pc;

use cpu::mos6502;

#[derive(Show,Copy)]
pub enum RegisterName {
    A,
    X,
    Y
}

pub struct Mos6502<M: mem::Memory<u16>> {
    pub registers: Mos6502Registers,
    pub mem: M,
    pub pc: pc::ProgramCounter<u16>
}

impl Mos6502<mem::FixedMemory<u16>> {
    pub fn with_fixed_memory(size: u16) -> Mos6502<mem::FixedMemory<u16>> {
        Mos6502::new(
            mem::FixedMemory::with_size_and_endian(size, mem::Endianness::LittleEndian))
    }
}

impl<M: mem::Memory<u16>> Mos6502<M> {
    pub fn new(mem: M) -> Mos6502<M> {
        Mos6502 {
            registers: Mos6502Registers::new(),
            mem: mem,
            pc: pc::ProgramCounter::new()
        }
    }
}

bitflags! {
    #[derive(Show)]
    flags Mos6502Flags: u8 {
        const FLAGS_SIGN        = 0b10000000,
        const FLAGS_OVERFLOW    = 0b01000000,
        const FLAGS_RESERVED    = 0b00100000,
        const FLAGS_BREAK       = 0b00010000,
        const FLAGS_BCD         = 0b00001000,
        const FLAGS_INTERRUPT   = 0b00000100,
        const FLAGS_ZERO        = 0b00000010,
        const FLAGS_CARRY       = 0b00000001,
        const FLAGS_NONE        = 0b00000000,

        const FLAGS_ARITHMETIC  = FLAGS_SIGN.bits |
                                  FLAGS_OVERFLOW.bits |
                                  FLAGS_ZERO.bits |
                                  FLAGS_CARRY.bits
    }
}

pub struct Mos6502Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u16,
    flags: Mos6502Flags
}

impl Mos6502Registers {
    pub fn new() -> Mos6502Registers {
        Mos6502Registers { a: 0, x: 0, y: 0, sp: 0, flags: FLAGS_RESERVED }
    }

    pub fn with_flags(flags: Mos6502Flags) -> Mos6502Registers {
        Mos6502Registers { a: 0, x: 0, y: 0, sp: 0, flags: flags | FLAGS_RESERVED }   
    }

    pub fn get(&self, r: RegisterName) -> u8 {
        match r {
            RegisterName::A => self.a,
            RegisterName::X => self.x,
            RegisterName::Y => self.y
        }
    }

    pub fn carry(&self) -> bool {
        self.flags.intersects(FLAGS_CARRY)
    }

    pub fn has_flags(&self, flags: Mos6502Flags) -> bool {
        self.flags.intersects(flags)
    }

    pub fn get_flags(&self) -> Mos6502Flags {
        self.flags
    }

    pub fn clear_flags(&mut self, flags: Mos6502Flags) {
        self.flags = self.flags & (!flags)
    }

    pub fn set_flags(&mut self, flags: Mos6502Flags) {
        self.flags = self.flags | flags;
    }

    pub fn replace_flags(&mut self, flags: Mos6502Flags) {
        self.flags = flags | FLAGS_RESERVED;
    }

    pub fn set_arith_flags<I: num::Int + num::FromPrimitive>(&mut self, val: I, carry: bool) {
        // Clear arithmetic flags
        let mut flags = (self.flags & !FLAGS_ARITHMETIC);

        if carry {
            flags = flags | FLAGS_CARRY;
        }

        flags = flags |
            if val == num::FromPrimitive::from_u8(0).unwrap() {
                FLAGS_ZERO
            } else if val > num::FromPrimitive::from_u8(255).unwrap() {
                FLAGS_OVERFLOW
            } else if val < num::FromPrimitive::from_u8(0).unwrap() {
                FLAGS_SIGN
            } else {
                FLAGS_NONE
            };

        self.replace_flags(flags);
    }
}

#[cfg(test)]
mod test {
    mod mos6502_registers {
        mod set_arith_flags {
            use cpu::mos6502;
            use cpu::mos6502::Mos6502Registers;

            #[test]
            pub fn sets_carry_flag_if_carry_true() {
                let mut r = Mos6502Registers::new();
                r.set_arith_flags(10, true);
                assert_eq!(r.get_flags(), mos6502::FLAGS_CARRY | mos6502::FLAGS_RESERVED);
            }

            #[test]
            pub fn unsets_carry_flag_if_carry_false() {
                let mut r = Mos6502Registers::with_flags(mos6502::FLAGS_CARRY);
                r.set_arith_flags(10, false);
                assert_eq!(r.get_flags(), mos6502::FLAGS_RESERVED);
            }

            #[test]
            pub fn sets_zero_flag_if_value_is_zero() {
                let mut r = Mos6502Registers::new();
                r.set_arith_flags(0, false);
                assert_eq!(r.get_flags(), mos6502::FLAGS_ZERO | mos6502::FLAGS_RESERVED);
            }

            #[test]
            pub fn unsets_zero_flag_if_value_is_nonzero() {
                let mut r = Mos6502Registers::with_flags(mos6502::FLAGS_ZERO);
                r.set_arith_flags(42, false);
                assert_eq!(r.get_flags(), mos6502::FLAGS_RESERVED);
            }

            #[test]
            pub fn sets_overflow_flag_if_value_is_higher_than_255() {
                let mut r = Mos6502Registers::new();
                r.set_arith_flags(1024, false);
                assert_eq!(r.get_flags(), mos6502::FLAGS_OVERFLOW | mos6502::FLAGS_RESERVED);
            }

            #[test]
            pub fn unsets_overflow_flag_if_value_is_less_than_or_equal_to_255() {
                let mut r = Mos6502Registers::with_flags(mos6502::FLAGS_OVERFLOW);
                r.set_arith_flags(128, false);
                assert_eq!(r.get_flags(), mos6502::FLAGS_RESERVED);
            }

            #[test]
            pub fn sets_sign_flag_if_value_is_negative() {
                let mut r = Mos6502Registers::new();
                r.set_arith_flags(-10, false);
                assert_eq!(r.get_flags(), mos6502::FLAGS_SIGN | mos6502::FLAGS_RESERVED);
            }

            #[test]
            pub fn unsets_sign_flag_if_value_is_non_negative() {
                let mut r = Mos6502Registers::with_flags(mos6502::FLAGS_SIGN);
                r.set_arith_flags(128, false);
                assert_eq!(r.get_flags(), mos6502::FLAGS_RESERVED);
            }

            #[test]
            pub fn does_not_change_non_arith_flags() {
                let mut r = Mos6502Registers::with_flags(!mos6502::FLAGS_ARITHMETIC);
                r.set_arith_flags(-10, false);
                assert_eq!(r.get_flags(), (!mos6502::FLAGS_ARITHMETIC) | mos6502::FLAGS_SIGN | mos6502::FLAGS_RESERVED);
            }

            #[test]
            pub fn sets_all_relevant_flags() {
                let mut r = Mos6502Registers::new();
                r.set_arith_flags(0, true);
                assert_eq!(r.get_flags(), mos6502::FLAGS_CARRY | mos6502::FLAGS_ZERO | mos6502::FLAGS_RESERVED);
            }
        }

        mod get {
            use cpu::mos6502::{Mos6502Registers, RegisterName};

            #[test]
            pub fn gets_a() {
                let mut r = Mos6502Registers::new();
                r.a = 42;
                assert_eq!(r.get(RegisterName::A), 42);
            }

            #[test]
            pub fn gets_x() {
                let mut r = Mos6502Registers::new();
                r.x = 42;
                assert_eq!(r.get(RegisterName::X), 42);
            }

            #[test]
            pub fn gets_y() {
                let mut r = Mos6502Registers::new();
                r.y = 42;
                assert_eq!(r.get(RegisterName::Y), 42);
            }
        }

        mod carry {
            use cpu::mos6502;
            use cpu::mos6502::Mos6502Registers;

            #[test]
            pub fn returns_true_if_carry_bit_set() {
                let r = Mos6502Registers::with_flags(mos6502::FLAGS_CARRY);
                assert!(r.carry());
            }

            #[test]
            pub fn returns_false_if_carry_bit_not_set() {
                let r = Mos6502Registers::with_flags(mos6502::FLAGS_SIGN);
                assert!(!r.carry());
            }
        }
    }
}