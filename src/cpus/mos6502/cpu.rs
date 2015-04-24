use std::fmt;

use mem;
use mem::MemoryExt;

use pc;

/// Indicates the start of the MOS 6502 Stack
pub const STACK_START   : u64 = 0x0100;

/// Indicates the end of the MOS 6502 Stack
pub const STACK_END     : u64 = 0x01FF;

/// Denotes a particular register
#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub enum RegisterName {
    /// Denotes the accumulator ("A" register)
    A,

    /// Denotes the "X" register
    X,

    /// Denotes the "Y" register
    Y,

    /// Denotes the flags register
    P,

    /// Denotes the stack pointer
    S
}

impl RegisterName {
    /// Retrieves the value of the specified register from the provided cpu
    ///
    /// # Arguments
    /// * `cpu` - The cpu to retrieve the register value from
    pub fn get<M>(self, cpu: &Mos6502<M>) -> u8 where M : mem::Memory {
        match self {
            RegisterName::A => cpu.registers.a,
            RegisterName::X => cpu.registers.x,
            RegisterName::Y => cpu.registers.y,
            RegisterName::P => cpu.flags.bits,
            RegisterName::S => cpu.registers.sp
        }
    }

    /// Sets the value of the specified register on the provided cpu
    ///
    /// # Arguments
    /// * `cpu` - The cpu to set the register value on
    /// * `val` - The value to set the register to
    pub fn set<M>(self, cpu: &mut Mos6502<M>, val: u8) where M : mem::Memory {
        match self {
            RegisterName::A => cpu.registers.a = val,
            RegisterName::X => cpu.registers.x = val,
            RegisterName::Y => cpu.registers.y = val,
            RegisterName::P => cpu.flags.replace(Flags::new(val)),
            RegisterName::S => cpu.registers.sp = val
        }
    }
}

impl fmt::Display for RegisterName {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &RegisterName::A => formatter.write_str("A"),
            &RegisterName::X => formatter.write_str("X"),
            &RegisterName::Y => formatter.write_str("Y"),
            &RegisterName::S => formatter.write_str("S"),
            &RegisterName::P => formatter.write_str("P")
        }
    }
}

/// Represents a MOS 6502 Central Processing Unit
///
/// Includes support for Binary Coded Decimal arithmetic, does
/// NOT include an Audio Processing Unit.
pub struct Mos6502<M> where M: mem::Memory {
    /// The registers contained in the cpu
    pub registers: Registers,
    /// The processor status flags
    pub flags: Flags,
    /// The memory attached to the cpu, including external device registers and ROM
    pub mem: M,
    /// The program counter for the cpu
    pub pc: pc::ProgramCounter,
    /// Indicates if BCD arithmetic is enabled on this instance
    pub bcd_enabled: bool
}

impl Mos6502<mem::Fixed> {
    /// Creates a `Mos6502` instance using a fixed memory
    ///
    /// The memory is attached at address `$0000`
    ///
    /// # Arguments
    /// * `size` - The size of the memory to attach.
    pub fn with_fixed_memory(size: usize) -> Self {
        Mos6502::new(mem::Fixed::new(size))
    }
}

impl Mos6502<mem::Empty> {
    /// Creates a `Mos6502` instance with no attached memory
    pub fn without_memory() -> Self {
        Mos6502::new(mem::Empty)
    }
}

impl<M> Mos6502<M> where M: mem::Memory {
    /// Creates a `Mos6502` instance using a provided memory,
    /// with BCD arithmetic enabled
    ///
    /// Use of BCD arithmetic still requires that the
    /// BCD flag be set.
    pub fn new(mem: M) -> Self {
        Mos6502 {
            registers: Registers::new(),
            mem: mem,
            flags: Flags::RESERVED(),
            pc: pc::ProgramCounter::new(),
            bcd_enabled: true
        }
    }

    /// Creates a `Mos6502` instance using a provided memory,
    /// with BCD arithmetic disabled
    ///
    /// BCD arithmetic will not be available, regardless of the
    /// value of the BCD flag.
    pub fn without_bcd(mem: M) -> Self {
        Mos6502 {
            registers: Registers::new(),
            mem: mem,
            flags: Flags::RESERVED(),
            pc: pc::ProgramCounter::new(),
            bcd_enabled: false
        }
    }

    /// Push a value on to the stack
    ///
    /// Note: A `MemoryError::OutOfBounds` result is returned
    /// if there is no memory available in the stack range
    /// ($0100 - $01FF)
    ///
    /// # Arguments
    /// * `val` - The value to push on to the stack
    pub fn push(&mut self, val: u8) -> mem::Result<()> {
        let addr = (self.registers.sp as u64) + STACK_START;
        try!(self.mem.set_u8(addr, val));
        self.registers.sp -= 1;
        Ok(())
    }

    /// Pulls a value from the stack
    ///
    /// Note: A `MemoryError::OutOfBounds` result is returned
    /// if there is no memory available in the stack range
    /// ($0100 - $01FF)
    pub fn pull(&mut self) -> mem::Result<u8> {
        self.registers.sp += 1;
        let addr = (self.registers.sp as u64) + STACK_START;
        self.mem.get_u8(addr)
    }
}

/// Represents the 8-bit registers available on the MOS 6502 processor
#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub struct Registers {
    /// Contains the value of the accumulator (`A` register)
    pub a: u8,
    /// Contains the value of the `X` register
    pub x: u8,
    /// Contains the value of the `Y` register
    pub y: u8,
    /// Contains the value of the stack pointer (`S` register)
    pub sp: u8,
}

impl Registers {
    /// Allocates an empty set of registers
    pub fn new() -> Registers {
        Registers { a: 0, x: 0, y: 0, sp: 0 }
    }
}

/// Represents the processor status flags supported by the MOS 6502 CPU
#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub struct Flags {
    pub bits: u8
}

impl Flags {
    #[inline] #[allow(non_snake_case)] pub fn SIGN() -> Flags        { Flags::new(0b10000000) }
    #[inline] #[allow(non_snake_case)] pub fn OVERFLOW() -> Flags    { Flags::new(0b01000000) }
    #[inline] #[allow(non_snake_case)] pub fn RESERVED() -> Flags    { Flags::new(0b00100000) }
    #[inline] #[allow(non_snake_case)] pub fn BREAK() -> Flags       { Flags::new(0b00010000) }
    #[inline] #[allow(non_snake_case)] pub fn BCD() -> Flags         { Flags::new(0b00001000) }
    #[inline] #[allow(non_snake_case)] pub fn INTERRUPT() -> Flags   { Flags::new(0b00000100) }
    #[inline] #[allow(non_snake_case)] pub fn ZERO() -> Flags        { Flags::new(0b00000010) }
    #[inline] #[allow(non_snake_case)] pub fn CARRY() -> Flags       { Flags::new(0b00000001) }
    #[inline] #[allow(non_snake_case)] pub fn NONE() -> Flags        { Flags::new(0b00000000) }

    /// Creates a new `Flags` structure from the provided 8-bit value
    pub fn new(bits: u8) -> Flags {
        Flags { bits: bits }
    }

    /// Returns a value indicating if the specified flags are set on this instance
    pub fn intersects(&self, other: Flags) -> bool {
        self.bits & other.bits == other.bits
    }

    /// Returns a value indicating if the CARRY flag is set
    pub fn carry(&self) -> bool {
        self.intersects(Flags::CARRY())
    }

    /// Clears the specified flags
    pub fn clear(&mut self, flags: Flags) {
        let new_val = *self & (!flags);
        self.replace(new_val);
    }

    /// Sets the specified flags (leaving other flags alone)
    pub fn set(&mut self, flags: Flags) {
        let new_val = *self | flags;
        self.replace(new_val);
    }

    /// Sets or clears the specified flags based on the provided condition
    pub fn set_if(&mut self, flag_selector: Flags, condition: bool) {
        self.clear(flag_selector);
        if condition {
            self.set(flag_selector);
        }
    }

    /// Replaces all flags with the provided value
    pub fn replace(&mut self, flags: Flags) {
        self.bits = (flags | Flags::RESERVED()).bits;
    }

    /// Sets the sign and zero flags based on the provided value
    pub fn set_sign_and_zero(&mut self, val: u8) {
        self.set_if(Flags::ZERO(), val == 0);
        self.set_if(Flags::SIGN(), val & 0x80 != 0);
    }
}

impl ::std::ops::BitOr for Flags {
    type Output = Flags;

    /// Returns a new flags value representing the bitwise OR of the provided flags
    fn bitor(self, rhs: Flags) -> Flags {
        Flags::new(self.bits | rhs.bits)
    }
}

impl ::std::ops::BitAnd for Flags {
    type Output = Flags;

    /// Returns a new flags value representing the bitwise AND of the provided flags 
    fn bitand(self, rhs: Flags) -> Flags {
        Flags::new(self.bits & rhs.bits)
    }
}

impl ::std::ops::Not for Flags {
    type Output = Flags;

    /// Negates the value of the flags
    fn not(self) -> Flags {
        Flags::new(!self.bits)
    }
}

#[cfg(test)]
mod test {
    mod mos6502 {
        use mem;
        use mem::MemoryExt;

        use cpus::mos6502;

        #[test]
        pub fn push_places_value_at_current_sp_location() {
            let mut cpu = setup_cpu();
            cpu.push(42).unwrap();
            assert_eq!(Ok(42), cpu.mem.get_u8(mos6502::cpu::STACK_START + 5));
        }

        #[test]
        pub fn push_decrements_sp() {
            let mut cpu = setup_cpu();
            cpu.push(42).unwrap();
            assert_eq!(4, cpu.registers.sp);
        }

        #[test]
        pub fn pull_gets_value_at_sp_plus_one() {
            let mut cpu = setup_cpu();
            cpu.mem.set_u8(mos6502::cpu::STACK_START + 6, 24).unwrap();
            assert_eq!(Ok(24), cpu.pull());
        }

        #[test]
        pub fn pull_increments_sp() {
            let mut cpu = setup_cpu();
            cpu.mem.set_u8(mos6502::cpu::STACK_START + 6, 24).unwrap();
            cpu.pull().unwrap();
            assert_eq!(6, cpu.registers.sp);
        }

        pub fn setup_cpu<'a>() -> mos6502::Mos6502<mem::Virtual<'a>> {
            let mem = mem::Fixed::new(10);
            let mut vm = mem::Virtual::new();
            vm.attach(mos6502::cpu::STACK_START, Box::new(mem)).unwrap();

            let mut cpu = mos6502::Mos6502::new(vm);
            cpu.registers.sp = 5;
            cpu
        }
    }

    mod flags {
        use cpus::mos6502::Flags;

        #[test]
        pub fn intersects_returns_true_if_all_of_provided_flags_are_set() {
            let f = Flags::CARRY() | Flags::SIGN() | Flags::ZERO();
            assert!(f.intersects(Flags::CARRY() | Flags::SIGN()));
        }

        #[test]
        pub fn intersects_returns_false_if_any_of_provided_flags_are_clear() {
            let f = Flags::CARRY();
            assert!(!f.intersects(Flags::CARRY() | Flags::SIGN()));
        }

        #[test]
        pub fn clear_leaves_flag_clear_if_it_is_already_clear() {
            let mut f = Flags::CARRY();
            f.clear(Flags::SIGN());
            assert!(!f.intersects(Flags::SIGN()));
        }

        #[test]
        pub fn clear_clears_flag_if_it_is_set() {
            let mut f = Flags::CARRY() | Flags::SIGN();
            f.clear(Flags::SIGN());
            assert!(!f.intersects(Flags::SIGN()));
        }

        #[test]
        pub fn set_leaves_flag_set_if_it_is_already_set() {
            let mut f = Flags::CARRY() | Flags::SIGN();
            f.set(Flags::SIGN());
            assert!(f.intersects(Flags::SIGN()));
        }

        #[test]
        pub fn set_sets_flag_if_it_is_clear() {
            let mut f = Flags::CARRY();
            f.set(Flags::SIGN());
            assert!(f.intersects(Flags::SIGN()));
        }

        #[test]
        pub fn replace_sets_flags_to_provided_value_plus_reserved_flag() {
            let mut f = Flags::CARRY() | Flags::SIGN();
            f.replace(Flags::INTERRUPT() | Flags::ZERO());
            assert_eq!(Flags::INTERRUPT() | Flags::ZERO() | Flags::RESERVED(), f);
        }

        #[test]
        pub fn carry_returns_true_if_carry_bit_set() {
            let f = Flags::CARRY();
            assert!(f.carry());
        }

        #[test]
        pub fn carry_returns_false_if_carry_bit_not_set() {
            let f = Flags::SIGN();
            assert!(!f.carry());
        }

        #[test]
        pub fn set_sign_and_zero_sets_zero_flag_if_value_is_zero() {
            let mut r = Flags::NONE();
            r.set_sign_and_zero(0);
            assert_eq!(r, Flags::ZERO() | Flags::RESERVED());
        }

        #[test]
        pub fn set_sign_and_zero_unsets_zero_flag_if_value_is_nonzero() {
            let mut r = Flags::ZERO();
            r.set_sign_and_zero(42);
            assert_eq!(r, Flags::RESERVED());
        }

        #[test]
        pub fn set_sign_and_zero_sets_sign_flag_if_value_is_negative() {
            let mut r = Flags::NONE();
            r.set_sign_and_zero(0xFF);
            assert_eq!(r, Flags::SIGN() | Flags::RESERVED());
        }

        #[test]
        pub fn set_sign_and_zero_unsets_sign_flag_if_value_is_non_negative() {
            let mut r = Flags::SIGN();
            r.set_sign_and_zero(0x7F);
            assert_eq!(r, Flags::RESERVED());
        }

        #[test]
        pub fn set_if_sets_flag_if_condition_true() {
            let mut r = Flags::NONE();
            r.clear(Flags::SIGN());
            r.set_if(Flags::SIGN(), true);
            assert!(r.intersects(Flags::SIGN()));
        }

        #[test]
        pub fn set_if_clears_flag_if_condition_false() {
            let mut r = Flags::NONE();
            r.set(Flags::SIGN());
            r.set_if(Flags::SIGN(), false);
            assert!(!r.intersects(Flags::SIGN()));
        }
    }

    mod register_name {
        use cpus::mos6502::{cpu,Mos6502,Flags};

        #[test]
        pub fn gets_a() {
            let mut cpu = Mos6502::without_memory();
            cpu.registers.a = 42;
            assert_eq!(cpu::RegisterName::A.get(&cpu), 42);
        }

        #[test]
        pub fn gets_x() {
            let mut cpu = Mos6502::without_memory();
            cpu.registers.x = 42;
            assert_eq!(cpu::RegisterName::X.get(&cpu), 42);
        }

        #[test]
        pub fn gets_y() {
            let mut cpu = Mos6502::without_memory();
            cpu.registers.y = 42;
            assert_eq!(cpu::RegisterName::Y.get(&cpu), 42);
        }

        #[test]
        pub fn gets_p() {
            let mut cpu = Mos6502::without_memory();
            cpu.flags.set(Flags::SIGN() | Flags::CARRY()); 
            assert_eq!(cpu::RegisterName::P.get(&cpu), cpu.flags.bits);
        }

        #[test]
        pub fn sets_a() {
            let mut cpu = Mos6502::without_memory();
            cpu::RegisterName::A.set(&mut cpu, 42);
            assert_eq!(cpu.registers.a, 42);
        }

        #[test]
        pub fn sets_x() {
            let mut cpu = Mos6502::without_memory();
            cpu::RegisterName::X.set(&mut cpu, 42);
            assert_eq!(cpu.registers.x, 42);
        }

        #[test]
        pub fn sets_y() {
            let mut cpu = Mos6502::without_memory();
            cpu::RegisterName::Y.set(&mut cpu, 42);
            assert_eq!(cpu.registers.y, 42);
        }

        #[test]
        pub fn sets_p() {
            let mut cpu = Mos6502::without_memory();
            cpu::RegisterName::P.set(&mut cpu, (Flags::SIGN() | Flags::CARRY()).bits);
            assert_eq!(Flags::SIGN() | Flags::CARRY() | Flags::RESERVED(), cpu.flags);
        }
    }
}
