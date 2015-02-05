use std::io;
use std::{isize,usize};

pub use mem::fixed::FixedMemory;
//pub use mem::virt::VirtualMemory;

mod fixed;
mod virt;

pub type MemoryResult<T> = Result<T, MemoryError>;

#[derive(Copy,Show,PartialEq)]
/// Represents an error that occurs when accessing a `Memory`
pub enum MemoryError {
    /// The provided address was outside the bounds of the memory
    OutOfBounds,

    /// The provided address referred to memory that is not readable
    MemoryNotReadable,

    /// The provided address referred to memory that is not writable
    MemoryNotWritable
}

#[derive(Copy)]
/// Describes the [endianness](http://en.wikipedia.org/wiki/Endianness) of a machine
pub enum Endianness {
    /// The machine is big-endian. Higher-order bytes of multi-byte numbers are stored first in memory
    BigEndian,

    /// The machine is little-endian. Lower-order bytes of multi-byte numbers are stored first in memory
    LittleEndian
}

impl Endianness {
    /// Gets an `Endianness` representing the host machine endianness.
    pub fn host() -> Endianness {
        if cfg!(target_endian = "big") {
            Endianness::BigEndian
        } else {
            Endianness::LittleEndian
        }
    }
}

/// Represents any memory accessible to a CPU
///
/// Implementations of this may use various sparse storage techniques to avoid
/// allocating the entire memory buffer, or may use ROM content from files to
/// back the memory. In the current implementation, the memory may not have an
/// address length longer than the native word size on the host platform.
pub trait Memory {
    /// Gets the size of the memory
    fn size(&self) -> usize;

    /// Copies from the memory into the specified buffer, starting at the specified address
    ///
    /// Memory operations are expected to succeed or fail atomically. If a read of any
    /// single byte would fail, the whole operation fails and the buffer is unmodified.
    ///
    /// # Arguments
    /// * `addr` - The address at which to begin reading the data
    /// * `buf` - The data to copy out of the memory
    fn get(&self, addr: usize, buf: &mut [u8]) -> MemoryResult<()>;

    /// Copies the specified buffer in to the memory starting at the specified address
    ///
    /// Memory operations are expected to succeed or fail atomically. If a write of any
    /// single byte would fail, the whole operation fails and the memory is unmodified.
    ///
    /// # Arguments
    /// * `addr` - The address at which to begin writing the data
    /// * `buf` - The data to copy in to the memory
    fn set(&mut self, addr: usize, buf: &[u8]) -> MemoryResult<()>;

    /// Gets a single byte from the specified address
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to read from
    fn get_u8(&self, addr: usize) -> MemoryResult<u8> {
        let mut buf = [0];
        try!(self.get(addr, &mut buf));
        Ok(buf[0])
    }

    /// Writes a single byte to the specified address
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to write to
    fn set_u8(&mut self, addr: usize, val: u8) -> MemoryResult<()> {
        let buf = [val];
        try!(self.set(addr, &buf));
        Ok(())
    }

    /// Gets `n` little-endian unsigned integer bytes from the specified address
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to read from
    /// * `nbytes` - The number of bytes to read (must be between 1 and 8, inclusive)
    fn get_le_uint_n(&self, addr: usize, nbytes: usize) -> MemoryResult<u64> {
        // Borrowed from http://doc.rust-lang.org/src/std/old_io/mod.rs.html#691-701
        assert!(nbytes > 0 && nbytes <= 8);

        let mut val = 0u64;
        let mut pos = 0;
        let mut i = 0;
        while i < nbytes {
            val += (try!(self.get_u8(addr + i)) as u64) << pos;
            pos += 8;
            i += 1;
        }
        Ok(val)
    }

    /// Gets `n` little-endian signed integer bytes from the specified address
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to read from
    /// * `nbytes` - The number of bytes to read (must be between 1 and 8, inclusive)
    fn get_le_int_n(&self, addr: usize, nbytes: usize) -> MemoryResult<i64> {
        self.get_le_uint_n(addr, nbytes).map(|i| extend_sign(i, nbytes))
    }

    /// Gets `n` big-endian unsigned integer bytes from the specified address
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to read from
    /// * `nbytes` - The number of bytes to read (must be between 1 and 8, inclusive)
    fn get_be_uint_n(&self, addr: usize, nbytes: usize) -> MemoryResult<u64> {
        // Borrowed from http://doc.rust-lang.org/src/std/old_io/mod.rs.html#691-701

        assert!(nbytes > 0 && nbytes <= 8);

        let mut val = 0u64;
        let mut i = 0;
        while i < nbytes {
            val += (try!(self.get_u8(addr + i)) as u64) << (nbytes - i - 1) * 8;
            i += 1;
        }
        Ok(val)
    }

    /// Gets `n` big-endian signed integer bytes from the specified address
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to read from
    /// * `nbytes` - The number of bytes to read (must be between 1 and 8, inclusive)
    fn get_be_int_n(&self, addr: usize, nbytes: usize) -> MemoryResult<i64> {
        self.get_be_uint_n(addr, nbytes).map(|i| extend_sign(i, nbytes))
    }

    fn get_be_u16(&self, addr: usize) -> MemoryResult<u16> {
        self.get_be_uint_n(addr, 2).map(|i| i as u16)
    }

    fn get_be_u32(&self, addr: usize) -> MemoryResult<u32> {
        self.get_be_uint_n(addr, 4).map(|i| i as u32)
    }

    fn get_be_u64(&self, addr: usize) -> MemoryResult<u64> {
        self.get_be_uint_n(addr, 8)
    }

    fn get_be_usize(&self, addr: usize) -> MemoryResult<usize> {
        self.get_be_uint_n(addr, usize::BYTES).map(|i| i as usize)
    }

    fn get_le_u16(&self, addr: usize) -> MemoryResult<u16> {
        self.get_le_uint_n(addr, 2).map(|i| i as u16)
    }

    fn get_le_u32(&self, addr: usize) -> MemoryResult<u32> {
        self.get_le_uint_n(addr, 4).map(|i| i as u32)
    }

    fn get_le_u64(&self, addr: usize) -> MemoryResult<u64> {
        self.get_le_uint_n(addr, 8)
    }

    fn get_le_usize(&self, addr: usize) -> MemoryResult<usize> {
        self.get_le_uint_n(addr, usize::BYTES).map(|i| i as usize)
    }

    fn get_be_i16(&self, addr: usize) -> MemoryResult<i16> {
        self.get_be_int_n(addr, 2).map(|i| i as i16)
    }

    fn get_be_i32(&self, addr: usize) -> MemoryResult<i32> {
        self.get_be_int_n(addr, 4).map(|i| i as i32)
    }

    fn get_be_i64(&self, addr: usize) -> MemoryResult<i64> {
        self.get_be_int_n(addr, 8)
    }

    fn get_be_isize(&self, addr: usize) -> MemoryResult<isize> {
        self.get_be_int_n(addr, isize::BYTES).map(|i| i as isize)
    }

    fn get_le_i16(&self, addr: usize) -> MemoryResult<i16> {
        self.get_le_int_n(addr, 2).map(|i| i as i16)
    }

    fn get_le_i32(&self, addr: usize) -> MemoryResult<i32> {
        self.get_le_int_n(addr, 4).map(|i| i as i32)
    }

    fn get_le_i64(&self, addr: usize) -> MemoryResult<i64> {
        self.get_le_int_n(addr, 8)
    }

    fn get_le_isize(&self, addr: usize) -> MemoryResult<isize> {
        self.get_le_int_n(addr, isize::BYTES).map(|i| i as isize)
    }

    #[inline]
    fn set_be_u16(&mut self, addr: usize, val: u16) -> MemoryResult<()> {
        io::extensions::u64_to_be_bytes(val as u64, 2, |v| self.set(addr, v))
    }

    #[inline]
    fn set_be_u32(&mut self, addr: usize, val: u32) -> MemoryResult<()> {
        io::extensions::u64_to_be_bytes(val as u64, 4, |v| self.set(addr, v))
    }

    #[inline]
    fn set_be_u64(&mut self, addr: usize, val: u64) -> MemoryResult<()> {
        io::extensions::u64_to_be_bytes(val, 8, |v| self.set(addr, v))
    }

    #[inline]
    fn set_be_usize(&mut self, addr: usize, val: usize) -> MemoryResult<()> {
        io::extensions::u64_to_be_bytes(val as u64, usize::BYTES, |v| self.set(addr, v))
    }

    #[inline]
    fn set_be_i16(&mut self, addr: usize, val: i16) -> MemoryResult<()> {
        io::extensions::u64_to_be_bytes(val as u64, 2, |v| self.set(addr, v))
    }

    #[inline]
    fn set_be_i32(&mut self, addr: usize, val: i32) -> MemoryResult<()> {
        io::extensions::u64_to_be_bytes(val as u64, 4, |v| self.set(addr, v))
    }

    #[inline]
    fn set_be_i64(&mut self, addr: usize, val: i64) -> MemoryResult<()> {
        io::extensions::u64_to_be_bytes(val as u64, 8, |v| self.set(addr, v))
    }

    #[inline]
    fn set_be_isize(&mut self, addr: usize, val: isize) -> MemoryResult<()> {
        io::extensions::u64_to_be_bytes(val as u64, isize::BYTES, |v| self.set(addr, v))
    }

    #[inline]
    fn set_le_u16(&mut self, addr: usize, val: u16) -> MemoryResult<()> {
        io::extensions::u64_to_le_bytes(val as u64, 2, |v| self.set(addr, v))
    }

    #[inline]
    fn set_le_u32(&mut self, addr: usize, val: u32) -> MemoryResult<()> {
        io::extensions::u64_to_le_bytes(val as u64, 4, |v| self.set(addr, v))
    }

    #[inline]
    fn set_le_u64(&mut self, addr: usize, val: u64) -> MemoryResult<()> {
        io::extensions::u64_to_le_bytes(val, 8, |v| self.set(addr, v))
    }

    #[inline]
    fn set_le_usize(&mut self, addr: usize, val: usize) -> MemoryResult<()> {
        io::extensions::u64_to_le_bytes(val as u64, usize::BYTES, |v| self.set(addr, v))
    }

    #[inline]
    fn set_le_i16(&mut self, addr: usize, val: i16) -> MemoryResult<()> {
        io::extensions::u64_to_le_bytes(val as u64, 2, |v| self.set(addr, v))
    }

    #[inline]
    fn set_le_i32(&mut self, addr: usize, val: i32) -> MemoryResult<()> {
        io::extensions::u64_to_le_bytes(val as u64, 4, |v| self.set(addr, v))
    }

    #[inline]
    fn set_le_i64(&mut self, addr: usize, val: i64) -> MemoryResult<()> {
        io::extensions::u64_to_le_bytes(val as u64, 8, |v| self.set(addr, v))
    }

    #[inline]
    fn set_le_isize(&mut self, addr: usize, val: isize) -> MemoryResult<()> {
        io::extensions::u64_to_le_bytes(val as u64, isize::BYTES, |v| self.set(addr, v))
    }
}

// From http://doc.rust-lang.org/src/std/old_io/mod.rs.html#976-979
fn extend_sign(val: u64, nbytes: usize) -> i64 {
    let shift = (8 - nbytes) * 8;
    (val << shift) as i64 >> shift
}

#[cfg(test)]
mod test {
    use mem::{Memory,FixedMemory};

    macro_rules! assert_eq_hex(
        ( $ left : expr , $ right : expr ) => (
        {
            match ( & ( $ left ) , & ( $ right ) ) {
                ( left_val , right_val ) => {
                    if ! ( ( * left_val == * right_val ) && ( * right_val == * left_val ) ) {
                        panic ! (
                            "assertion failed: `(left == right) && (right == left)` (left: `0x{:X}`, right: `0x{:X}`)"
                        , * left_val , * right_val ) } } } } )
    );

    #[test]
    pub fn get_u8_returns_single_byte_at_location() {
        let mut mem = FixedMemory::with_size(1);
        mem.set(0, &[42]).unwrap();
        assert_eq!(mem.get_u8(0).unwrap(), 42);
    }

    #[test]
    pub fn set_u8_writes_single_byte_at_location() {
        let mut mem = FixedMemory::with_size(1);
        mem.set_u8(0, 42).unwrap();
        let mut buf = [0];
        mem.get(0, &mut buf).unwrap();
        assert_eq!([42], buf);
    }

    #[test]
    pub fn get_be_u16_works() { assert_eq_hex!(0x2345, init_mem_get().get_be_u16(1).unwrap()); }

    #[test]
    pub fn get_le_u16_works() { assert_eq_hex!(0x4523, init_mem_get().get_le_u16(1).unwrap()); }

    #[test]
    pub fn get_be_u32_works() { assert_eq_hex!(0x23456789, init_mem_get().get_be_u32(1).unwrap()); }

    #[test]
    pub fn get_le_u32_works() { assert_eq_hex!(0x89674523, init_mem_get().get_le_u32(1).unwrap()); }

    #[test]
    pub fn get_be_u64_works() { assert_eq_hex!(0x23456789ABCDEF00, init_mem_get().get_be_u64(1).unwrap()); }

    #[test]
    pub fn get_le_u64_works() { assert_eq_hex!(0x00EFCDAB89674523, init_mem_get().get_le_u64(1).unwrap()); }

    #[test]
    pub fn set_be_u16_works() {
        let mut mem = FixedMemory::with_size(10);
        mem.set_be_u16(1, 0x2345).unwrap();
        let mut buf = [0, 0];
        mem.get(1, &mut buf).unwrap();
        assert_eq!([0x23, 0x45], buf);
    }

    #[test]
    pub fn set_be_u32_works() {
        let mut mem = FixedMemory::with_size(10);
        mem.set_be_u32(1, 0x23456789).unwrap();
        let mut buf = [0, 0, 0, 0];
        mem.get(1, &mut buf).unwrap();
        assert_eq!([0x23, 0x45, 0x67, 0x89], buf);
    }

    #[test]
    pub fn set_be_u64_works() {
        let mut mem = FixedMemory::with_size(10);
        mem.set_be_u64(1, 0x23456789ABCDEF00).unwrap();
        let mut buf = [0, 0, 0, 0, 0, 0, 0, 0];
        mem.get(1, &mut buf).unwrap();
        assert_eq!([0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x00], buf);
    }

    #[test]
    pub fn set_le_u16_works() {
        let mut mem = FixedMemory::with_size(10);
        mem.set_le_u16(1, 0x2345).unwrap();
        let mut buf = [0, 0];
        mem.get(1, &mut buf).unwrap();
        assert_eq!([0x45, 0x23], buf);
    }

    #[test]
    pub fn set_le_u32_works() {
        let mut mem = FixedMemory::with_size(10);
        mem.set_le_u32(1, 0x23456789).unwrap();
        let mut buf = [0, 0, 0, 0];
        mem.get(1, &mut buf).unwrap();
        assert_eq!([0x89, 0x67, 0x45, 0x23], buf);
    }

    #[test]
    pub fn set_le_u64_works() {
        let mut mem = FixedMemory::with_size(10);
        mem.set_le_u64(1, 0x23456789ABCDEF00).unwrap();
        let mut buf = [0, 0, 0, 0, 0, 0, 0, 0];
        mem.get(1, &mut buf).unwrap();
        assert_eq!([0x00, 0xEF, 0xCD, 0xAB, 0x89, 0x67, 0x45, 0x23], buf);
    }

    fn init_mem_get() -> FixedMemory {
        let mut mem = FixedMemory::with_size(10);
        mem.set(0, &[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x00, 0x00]).unwrap();
        mem
    }
}