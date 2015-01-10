use std::num;

use mem;

#[derive(Show)]
pub enum RegisterName {
    A,
    X,
    Y
}

pub struct Mos6502<M: mem::Memory<u16>> {
    pub registers: Mos6502Registers,
    pub mem: M
}

impl Mos6502<mem::FixedMemory<u16>> {
    pub fn with_fixed_memory(size: u16) -> Mos6502<mem::FixedMemory<u16>> {
        Mos6502::new(mem::FixedMemory::with_size_and_endian(size, mem::Endianness::LittleEndian))
    }
}

impl<M: mem::Memory<u16>> Mos6502<M> {
    pub fn new(mem: M) -> Mos6502<M> {
        Mos6502 {
            registers: Mos6502Registers::new(),
            mem: mem
        }
    }
}

pub struct Mos6502Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u16,
    pub pc: u16,
    pub flags: u8
}

impl Mos6502Registers {
    pub fn new() -> Mos6502Registers {
        Mos6502Registers { a: 0, x: 0, y: 0, sp: 0, pc: 0, flags: 0 }
    }

    pub fn get(&self, r: RegisterName) -> u8 {
        match r {
            RegisterName::A => self.a,
            RegisterName::X => self.x,
            RegisterName::Y => self.y
        }
    }

    pub fn carry(&self) -> bool {
        (self.flags & 0x1) != 0
    }

    pub fn set_arith_flags<I: num::Int + num::FromPrimitive>(&mut self, val: I, carry: bool) {
        // Clear arithmetic flags (0x1C selects the non-arithmetic flags)
        self.flags &= 0x1C;

        self.flags |= if carry                                           { 0x01 } else { 0x0 };
        self.flags |= if val == num::FromPrimitive::from_u8(0).unwrap()  { 0x02 } else { 0x0 };
        self.flags |= if val > num::FromPrimitive::from_u8(255).unwrap() { 0x40 } else { 0x0 };
        self.flags |= if val < num::FromPrimitive::from_u8(0).unwrap()   { 0x80 } else { 0x0 };
    }
}

#[cfg(test)]
mod test {
    mod mos6502_registers {
        mod set_arith_flags {
            use cpu::mos6502::Mos6502Registers;

            #[test]
            pub fn sets_carry_flag_if_carry_true() {
                let mut r = Mos6502Registers { a: 0, x: 0, y: 0, sp: 0, pc: 0, flags: 0 };
                r.set_arith_flags(10, true);
                assert_eq!(r.flags, 0x01);
            }

            #[test]
            pub fn unsets_carry_flag_if_carry_false() {
                let mut r = Mos6502Registers { a: 0, x: 0, y: 0, sp: 0, pc: 0, flags: 0x01 };
                r.set_arith_flags(10, false);
                assert_eq!(r.flags, 0x00);
            }

            #[test]
            pub fn sets_zero_flag_if_value_is_zero() {
                let mut r = Mos6502Registers { a: 0, x: 0, y: 0, sp: 0, pc: 0, flags: 0 };
                r.set_arith_flags(0, false);
                assert_eq!(r.flags, 0x02);
            }

            #[test]
            pub fn unsets_zero_flag_if_value_is_nonzero() {
                let mut r = Mos6502Registers { a: 0, x: 0, y: 0, sp: 0, pc: 0, flags: 0x02 };
                r.set_arith_flags(42, false);
                assert_eq!(r.flags, 0x00);
            }

            #[test]
            pub fn sets_overflow_flag_if_value_is_higher_than_255() {
                let mut r = Mos6502Registers { a: 0, x: 0, y: 0, sp: 0, pc: 0, flags: 0 };
                r.set_arith_flags(1024, false);
                assert_eq!(r.flags, 0x40);
            }

            #[test]
            pub fn unsets_overflow_flag_if_value_is_less_than_or_equal_to_255() {
                let mut r = Mos6502Registers { a: 0, x: 0, y: 0, sp: 0, pc: 0, flags: 0x40 };
                r.set_arith_flags(128, false);
                assert_eq!(r.flags, 0x00);
            }

            #[test]
            pub fn sets_sign_flag_if_value_is_negative() {
                let mut r = Mos6502Registers { a: 0, x: 0, y: 0, sp: 0, pc: 0, flags: 0 };
                r.set_arith_flags(-10, false);
                assert_eq!(r.flags, 0x80);
            }

            #[test]
            pub fn unsets_sign_flag_if_value_is_non_negative() {
                let mut r = Mos6502Registers { a: 0, x: 0, y: 0, sp: 0, pc: 0, flags: 0x80 };
                r.set_arith_flags(128, false);
                assert_eq!(r.flags, 0x00);
            }

            #[test]
            pub fn does_not_change_non_arith_flags() {
                let mut r = Mos6502Registers { a: 0, x: 0, y: 0, sp: 0, pc: 0, flags: 0x1C };
                r.set_arith_flags(-10, false);
                assert_eq!(r.flags, 0x9C);
            }

            #[test]
            pub fn sets_all_relevant_flags() {
                let mut r = Mos6502Registers { a: 0, x: 0, y: 0, sp: 0, pc: 0, flags: 0 };
                r.set_arith_flags(0, true);
                assert_eq!(r.flags, 0x03); // carry + zero
            }
        }

        mod get {
            use cpu::mos6502::{Mos6502Registers, RegisterName};

            #[test]
            pub fn gets_a() {
                let r = Mos6502Registers { a: 42, x: 0, y: 0, sp: 0, pc: 0, flags: 0 };
                assert_eq!(r.get(RegisterName::A), 42);
            }

            #[test]
            pub fn gets_x() {
                let r = Mos6502Registers { a: 0, x: 42, y: 0, sp: 0, pc: 0, flags: 0 };
                assert_eq!(r.get(RegisterName::X), 42);
            }

            #[test]
            pub fn gets_y() {
                let r = Mos6502Registers { a: 0, x: 0, y: 42, sp: 0, pc: 0, flags: 0 };
                assert_eq!(r.get(RegisterName::Y), 42);
            }
        }

        mod carry {
            use cpu::mos6502::Mos6502Registers;

            #[test]
            pub fn returns_true_if_carry_bit_set() {
                let r = Mos6502Registers { a: 0, x: 0, y: 0, sp: 0, pc: 0, flags: 0x01 };
                assert!(r.carry());
            }

            #[test]
            pub fn returns_false_if_carry_bit_not_set() {
                let r = Mos6502Registers { a: 0, x: 0, y: 0, sp: 0, pc: 0, flags: 0x02 };
                assert!(!r.carry());
            }
        }
    }
}