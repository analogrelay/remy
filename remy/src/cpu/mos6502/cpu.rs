use mem;

use pc;

pub const STACK_START   : usize = 0x0100;
pub const STACK_END     : usize = 0x01FF;

#[derive(Copy,Debug,Eq,PartialEq)]
pub enum RegisterName {
    A,
    X,
    Y
}

pub struct Mos6502<M> where M: mem::Memory {
    pub registers: Registers,
    pub mem: M,
    pub pc: pc::ProgramCounter
}

impl Mos6502<mem::FixedMemory> {
    pub fn with_fixed_memory(size: usize) -> Self {
        Mos6502::new(mem::FixedMemory::new(size))
    }
}

impl<M> Mos6502<M> where M: mem::Memory {
    pub fn new(mem: M) -> Self {
        Mos6502 {
            registers: Registers::new(),
            mem: mem,
            pc: pc::ProgramCounter::new()
        }
    }
}

pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    flags: Flags
}

impl Registers {
    pub fn new() -> Registers {
        Registers { a: 0, x: 0, y: 0, sp: 0, flags: Flags::RESERVED() }
    }

    pub fn with_flags(flags: Flags) -> Registers {
        Registers { a: 0, x: 0, y: 0, sp: 0, flags: flags | Flags::RESERVED() }
    }

    pub fn get(&self, r: RegisterName) -> u8 {
        match r {
            RegisterName::A => self.a,
            RegisterName::X => self.x,
            RegisterName::Y => self.y
        }
    }

    pub fn carry(&self) -> bool {
        self.flags.intersects(Flags::CARRY())
    }

    pub fn has_flags(&self, flags: Flags) -> bool {
        self.flags.intersects(flags)
    }

    pub fn get_flags(&self) -> Flags {
        self.flags
    }

    pub fn clear_flags(&mut self, flags: Flags) {
        self.flags = self.flags & (!flags)
    }

    pub fn set_flags(&mut self, flags: Flags) {
        self.flags = self.flags | flags;
    }

    pub fn replace_flags(&mut self, flags: Flags) {
        self.flags = flags | Flags::RESERVED();
    }

    pub fn set_arith_flags(&mut self, val: isize, carry: bool) {
        // Clear arithmetic flags
        let mut flags = self.flags & !Flags::ARITHMETIC();

        if carry {
            flags = flags | Flags::CARRY();
        }

        flags = flags |
            if val == 0 {
                Flags::ZERO()
            } else if val > 255 {
                Flags::OVERFLOW()
            } else if val < 0 {
                Flags::SIGN()
            } else {
                Flags::NONE()
            };

        self.replace_flags(flags);
    }
}

#[derive(Copy,Debug,Eq,PartialEq)]
pub struct Flags(u8);

impl Flags {
    #[inline] #[allow(non_snake_case)] pub fn SIGN() -> Flags        { Flags(0b10000000) }
    #[inline] #[allow(non_snake_case)] pub fn OVERFLOW() -> Flags    { Flags(0b01000000) }
    #[inline] #[allow(non_snake_case)] pub fn RESERVED() -> Flags    { Flags(0b00100000) }
    #[inline] #[allow(non_snake_case)] pub fn BREAK() -> Flags       { Flags(0b00010000) }
    #[inline] #[allow(non_snake_case)] pub fn BCD() -> Flags         { Flags(0b00001000) }
    #[inline] #[allow(non_snake_case)] pub fn INTERRUPT() -> Flags   { Flags(0b00000100) }
    #[inline] #[allow(non_snake_case)] pub fn ZERO() -> Flags        { Flags(0b00000010) }
    #[inline] #[allow(non_snake_case)] pub fn CARRY() -> Flags       { Flags(0b00000001) }
    #[inline] #[allow(non_snake_case)] pub fn NONE() -> Flags        { Flags(0b00000000) }
    #[inline] #[allow(non_snake_case)] pub fn ARITHMETIC() -> Flags  { Flags(0b11000011) }

    pub fn intersects(&self, other: Flags) -> bool {
        (*self & other) != Flags::NONE()
    }
}

impl ::std::ops::BitOr for Flags {
    type Output = Flags;

    fn bitor(self, rhs: Flags) -> Flags {
        let Flags(l) = self;
        let Flags(r) = rhs;
        Flags(l | r)
    }
}

impl ::std::ops::BitAnd for Flags {
    type Output = Flags;

    fn bitand(self, rhs: Flags) -> Flags {
        let Flags(l) = self;
        let Flags(r) = rhs;
        Flags(l & r)
    }
}

impl ::std::ops::Not for Flags {
    type Output = Flags;

    fn not(self) -> Flags {
        let Flags(l) = self;
        Flags(!l)
    }
}

#[cfg(test)]
mod test {
    mod mos6502_registers {
        mod set_arith_flags {
            use cpu::mos6502;
            use cpu::mos6502::Registers;

            #[test]
            pub fn sets_carry_flag_if_carry_true() {
                let mut r = Registers::new();
                r.set_arith_flags(10, true);
                assert_eq!(r.get_flags(), mos6502::Flags::CARRY() | mos6502::Flags::RESERVED());
            }

            #[test]
            pub fn unsets_carry_flag_if_carry_false() {
                let mut r = Registers::with_flags(mos6502::Flags::CARRY());
                r.set_arith_flags(10, false);
                assert_eq!(r.get_flags(), mos6502::Flags::RESERVED());
            }

            #[test]
            pub fn sets_zero_flag_if_value_is_zero() {
                let mut r = Registers::new();
                r.set_arith_flags(0, false);
                assert_eq!(r.get_flags(), mos6502::Flags::ZERO() | mos6502::Flags::RESERVED());
            }

            #[test]
            pub fn unsets_zero_flag_if_value_is_nonzero() {
                let mut r = Registers::with_flags(mos6502::Flags::ZERO());
                r.set_arith_flags(42, false);
                assert_eq!(r.get_flags(), mos6502::Flags::RESERVED());
            }

            #[test]
            pub fn sets_overflow_flag_if_value_is_higher_than_255() {
                let mut r = Registers::new();
                r.set_arith_flags(1024, false);
                assert_eq!(r.get_flags(), mos6502::Flags::OVERFLOW() | mos6502::Flags::RESERVED());
            }

            #[test]
            pub fn unsets_overflow_flag_if_value_is_less_than_or_equal_to_255() {
                let mut r = Registers::with_flags(mos6502::Flags::OVERFLOW());
                r.set_arith_flags(128, false);
                assert_eq!(r.get_flags(), mos6502::Flags::RESERVED());
            }

            #[test]
            pub fn sets_sign_flag_if_value_is_negative() {
                let mut r = Registers::new();
                r.set_arith_flags(-10, false);
                assert_eq!(r.get_flags(), mos6502::Flags::SIGN() | mos6502::Flags::RESERVED());
            }

            #[test]
            pub fn unsets_sign_flag_if_value_is_non_negative() {
                let mut r = Registers::with_flags(mos6502::Flags::SIGN());
                r.set_arith_flags(128, false);
                assert_eq!(r.get_flags(), mos6502::Flags::RESERVED());
            }

            #[test]
            pub fn does_not_change_non_arith_flags() {
                let mut r = Registers::with_flags(!mos6502::Flags::ARITHMETIC());
                r.set_arith_flags(-10, false);
                assert_eq!(r.get_flags(), (!mos6502::Flags::ARITHMETIC()) | mos6502::Flags::SIGN() | mos6502::Flags::RESERVED());
            }

            #[test]
            pub fn sets_all_relevant_flags() {
                let mut r = Registers::new();
                r.set_arith_flags(0, true);
                assert_eq!(r.get_flags(), mos6502::Flags::CARRY() | mos6502::Flags::ZERO() | mos6502::Flags::RESERVED());
            }
        }

        mod get {
            use cpu::mos6502::{Registers, RegisterName};

            #[test]
            pub fn gets_a() {
                let mut r = Registers::new();
                r.a = 42;
                assert_eq!(r.get(RegisterName::A), 42);
            }

            #[test]
            pub fn gets_x() {
                let mut r = Registers::new();
                r.x = 42;
                assert_eq!(r.get(RegisterName::X), 42);
            }

            #[test]
            pub fn gets_y() {
                let mut r = Registers::new();
                r.y = 42;
                assert_eq!(r.get(RegisterName::Y), 42);
            }
        }

        mod carry {
            use cpu::mos6502;
            use cpu::mos6502::Registers;

            #[test]
            pub fn returns_true_if_carry_bit_set() {
                let r = Registers::with_flags(mos6502::Flags::CARRY());
                assert!(r.carry());
            }

            #[test]
            pub fn returns_false_if_carry_bit_not_set() {
                let r = Registers::with_flags(mos6502::Flags::SIGN());
                assert!(!r.carry());
            }
        }
    }
}