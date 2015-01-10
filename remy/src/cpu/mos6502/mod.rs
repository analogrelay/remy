use mem;

use std::num;

mod instr;

enum RegisterName {
    A,
    X,
    Y
}

struct Mos6502<'a> {
    registers: Mos6502Registers,
    mem: Box<mem::Memory<u16>+'a>
}

struct Mos6502Registers {
    a: u8,
    x: u8,
    y: u8,
    sp: u8,
    pc: u16,
    flags: u8
}

impl Mos6502Registers {
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
    mod Mos6502Registers {
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