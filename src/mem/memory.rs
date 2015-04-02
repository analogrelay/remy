pub type Result<T> = ::std::result::Result<T, Error>;

/// Represents an error that occurs when accessing a `Memory`
#[derive(Clone,Debug,Eq,PartialEq)]
pub struct Error {
    /// The kind of the error
    pub kind: ErrorKind,
    /// A simple static description of the error
    pub desc: &'static str,
    /// An optional detailed description of the error, containing data specific to the input that
    /// caused the error
    pub detail: Option<String>
}

impl Error {
    /// Creates a new `Error` with no detail string
    /// 
    /// # Arguments
    ///
    /// * `desc` - A brief static description of the error
    pub fn new(kind: ErrorKind, desc: &'static str) -> Error {
        Error {
            kind: kind,
            desc: desc,
            detail: None
        }
    }

    /// Creates a new `Error` with the specified detail string
    /// 
    /// # Arguments
    ///
    /// * `desc` - A brief static description of the error
    /// * `detail` - A more detailed description of the error
    pub fn with_detail(kind: ErrorKind, desc: &'static str, detail: String) -> Error {
        Error {
            kind: kind,
            desc: desc,
            detail: Some(detail)
        }
    }
}

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
/// Defines the kind of a `Error`
pub enum ErrorKind {
    /// The provided address was outside the bounds of the memory
    OutOfBounds,

    /// The provided address referred to memory that is not readable
    MemoryNotReadable,

    /// The provided address referred to memory that is not writable
    MemoryNotWritable
}

/// Represents any memory accessible to a CPU
///
/// Implementations of this may use various sparse storage techniques to avoid
/// allocating the entire memory buffer, or may use ROM content from files to
/// back the memory. In the current implementation, the memory may not have an
/// address length longer than the native word size on the host platform.
pub trait Memory {
    /// Gets the size of the memory
    fn size(&self) -> u64;

    /// Copies from the memory into the specified buffer, starting at the specified address
    ///
    /// Memory operations may fail part-way through. The contents of the memory
    /// become undefined at that point.
    ///
    /// # Arguments
    /// * `addr` - The address at which to begin reading the data
    /// * `buf` - The data to copy out of the memory
    fn get(&self, addr: u64, buf: &mut [u8]) -> Result<()>;

    /// Copies the specified buffer in to the memory starting at the specified address
    ///
    /// Memory operations may fail part-way through. The contents of the memory
    /// become undefined at that point.
    ///
    /// # Arguments
    /// * `addr` - The address at which to begin writing the data
    /// * `buf` - The data to copy in to the memory
    fn set(&mut self, addr: u64, buf: &[u8]) -> Result<()>;

    /// Gets a single byte from the specified address
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to read from
    fn get_u8(&self, addr: u64) -> Result<u8> {
        let mut buf = [0];
        try!(self.get(addr, &mut buf));
        Ok(buf[0])
    }

    /// Writes a single byte to the specified address
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to write to
    fn set_u8(&mut self, addr: u64, val: u8) -> Result<()> {
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
    fn get_le_uint_n(&self, addr: u64, nbytes: u32) -> Result<u64> {
        // Borrowed from http://doc.rust-lang.org/src/std/old_io/mod.rs.html#691-701
        assert!(nbytes > 0 && nbytes <= 8);

        let mut val = 0u64;
        let mut pos = 0;
        let mut i = 0;
        while i < (nbytes as u64) {
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
    fn get_le_int_n(&self, addr: u64, nbytes: u32) -> Result<i64> {
        self.get_le_uint_n(addr, nbytes).map(|i| extend_sign(i, nbytes))
    }

    /// Gets `n` big-endian unsigned integer bytes from the specified address
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to read from
    /// * `nbytes` - The number of bytes to read (must be between 1 and 8, inclusive)
    fn get_be_uint_n(&self, addr: u64, nbytes: u32) -> Result<u64> {
        // Borrowed from http://doc.rust-lang.org/src/std/old_io/mod.rs.html#691-701

        assert!(nbytes > 0 && nbytes <= 8);

        let mut val = 0u64;
        let mut i = 0;
        while i < (nbytes as u64) {
            val += (try!(self.get_u8(addr + i)) as u64) << (nbytes as u64 - i - 1) * 8;
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
    fn get_be_int_n(&self, addr: u64, nbytes: u32) -> Result<i64> {
        self.get_be_uint_n(addr, nbytes).map(|i| extend_sign(i, nbytes))
    }

    /// Gets a big-endian 16-bit unsigned integer from the specified address
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to read from
    fn get_be_u16(&self, addr: u64) -> Result<u16> {
        self.get_be_uint_n(addr, 2).map(|i| i as u16)
    }

    /// Gets a big-endian 32-bit unsigned integer from the specified address
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to read from
    fn get_be_u32(&self, addr: u64) -> Result<u32> {
        self.get_be_uint_n(addr, 4).map(|i| i as u32)
    }

    /// Gets a big-endian 64-bit unsigned integer from the specified address
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to read from
    fn get_be_u64(&self, addr: u64) -> Result<u64> {
        self.get_be_uint_n(addr, 8)
    }

    /// Gets a little-endian 16-bit unsigned integer from the specified address
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to read from
    fn get_le_u16(&self, addr: u64) -> Result<u16> {
        self.get_le_uint_n(addr, 2).map(|i| i as u16)
    }

    /// Gets a little-endian 32-bit unsigned integer from the specified address
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to read from
    fn get_le_u32(&self, addr: u64) -> Result<u32> {
        self.get_le_uint_n(addr, 4).map(|i| i as u32)
    }

    /// Gets a little-endian 64-bit unsigned integer from the specified address
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to read from
    fn get_le_u64(&self, addr: u64) -> Result<u64> {
        self.get_le_uint_n(addr, 8)
    }

    /// Gets a big-endian 16-bit signed integer from the specified address
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to read from
    fn get_be_i16(&self, addr: u64) -> Result<i16> {
        self.get_be_int_n(addr, 2).map(|i| i as i16)
    }

    /// Gets a big-endian 32-bit signed integer from the specified address
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to read from
    fn get_be_i32(&self, addr: u64) -> Result<i32> {
        self.get_be_int_n(addr, 4).map(|i| i as i32)
    }

    /// Gets a big-endian 64-bit signed integer from the specified address
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to read from
    fn get_be_i64(&self, addr: u64) -> Result<i64> {
        self.get_be_int_n(addr, 8)
    }

    /// Gets a little-endian 16-bit signed integer from the specified address
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to read from
    fn get_le_i16(&self, addr: u64) -> Result<i16> {
        self.get_le_int_n(addr, 2).map(|i| i as i16)
    }

    /// Gets a little-endian 32-bit signed integer from the specified address
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to read from
    fn get_le_i32(&self, addr: u64) -> Result<i32> {
        self.get_le_int_n(addr, 4).map(|i| i as i32)
    }

    /// Gets a little-endian 64-bit signed integer from the specified address
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to read from
    fn get_le_i64(&self, addr: u64) -> Result<i64> {
        self.get_le_int_n(addr, 8)
    }

    /// Sets a big-endian 16-bit unsigned integer to the specified address
    ///
    /// # Arguments
    /// * `addr` - The address to write to
    /// * `val` - The value to write
    #[inline]
    fn set_be_u16(&mut self, addr: u64, val: u16) -> Result<()> {
        u64_to_be_bytes(val as u64, 2, |v| self.set(addr, v))
    }

    /// Sets a big-endian 32-bit unsigned integer to the specified address
    ///
    /// # Arguments
    /// * `addr` - The address to write to
    /// * `val` - The value to write
    #[inline]
    fn set_be_u32(&mut self, addr: u64, val: u32) -> Result<()> {
        u64_to_be_bytes(val as u64, 4, |v| self.set(addr, v))
    }

    /// Sets a big-endian 64-bit unsigned integer to the specified address
    ///
    /// # Arguments
    /// * `addr` - The address to write to
    /// * `val` - The value to write
    #[inline]
    fn set_be_u64(&mut self, addr: u64, val: u64) -> Result<()> {
        u64_to_be_bytes(val, 8, |v| self.set(addr, v))
    }

    /// Sets a big-endian 16-bit signed integer to the specified address
    ///
    /// # Arguments
    /// * `addr` - The address to write to
    /// * `val` - The value to write
    #[inline]
    fn set_be_i16(&mut self, addr: u64, val: i16) -> Result<()> {
        u64_to_be_bytes(val as u64, 2, |v| self.set(addr, v))
    }

    /// Sets a big-endian 32-bit signed integer to the specified address
    ///
    /// # Arguments
    /// * `addr` - The address to write to
    /// * `val` - The value to write
    #[inline]
    fn set_be_i32(&mut self, addr: u64, val: i32) -> Result<()> {
        u64_to_be_bytes(val as u64, 4, |v| self.set(addr, v))
    }

    /// Sets a big-endian 64-bit signed integer to the specified address
    ///
    /// # Arguments
    /// * `addr` - The address to write to
    /// * `val` - The value to write
    #[inline]
    fn set_be_i64(&mut self, addr: u64, val: i64) -> Result<()> {
        u64_to_be_bytes(val as u64, 8, |v| self.set(addr, v))
    }
 
    /// Sets a little-endian 16-bit unsigned integer to the specified address
    ///
    /// # Arguments
    /// * `addr` - The address to write to
    /// * `val` - The value to write
    #[inline]
    fn set_le_u16(&mut self, addr: u64, val: u16) -> Result<()> {
        u64_to_le_bytes(val as u64, 2, |v| self.set(addr, v))
    }

    /// Sets a little-endian 32-bit unsigned integer to the specified address
    ///
    /// # Arguments
    /// * `addr` - The address to write to
    /// * `val` - The value to write
    #[inline]
    fn set_le_u32(&mut self, addr: u64, val: u32) -> Result<()> {
        u64_to_le_bytes(val as u64, 4, |v| self.set(addr, v))
    }

    /// Sets a little-endian 64-bit unsigned integer to the specified address
    ///
    /// # Arguments
    /// * `addr` - The address to write to
    /// * `val` - The value to write
    #[inline]
    fn set_le_u64(&mut self, addr: u64, val: u64) -> Result<()> {
        u64_to_le_bytes(val, 8, |v| self.set(addr, v))
    }

    /// Sets a little-endian 16-bit signed integer to the specified address
    ///
    /// # Arguments
    /// * `addr` - The address to write to
    /// * `val` - The value to write
    #[inline]
    fn set_le_i16(&mut self, addr: u64, val: i16) -> Result<()> {
        u64_to_le_bytes(val as u64, 2, |v| self.set(addr, v))
    }

    /// Sets a little-endian 32-bit signed integer to the specified address
    ///
    /// # Arguments
    /// * `addr` - The address to write to
    /// * `val` - The value to write
    #[inline]
    fn set_le_i32(&mut self, addr: u64, val: i32) -> Result<()> {
        u64_to_le_bytes(val as u64, 4, |v| self.set(addr, v))
    }

    /// Sets a little-endian 64-bit signed integer to the specified address
    ///
    /// # Arguments
    /// * `addr` - The address to write to
    /// * `val` - The value to write
    #[inline]
    fn set_le_i64(&mut self, addr: u64, val: i64) -> Result<()> {
        u64_to_le_bytes(val as u64, 8, |v| self.set(addr, v))
    }
}

// From http://doc.rust-lang.org/src/std/old_io/mod.rs.html#976-979
fn extend_sign(val: u64, nbytes: u32) -> i64 {
    let shift = (8 - nbytes) * 8;
    (val << shift) as i64 >> shift
}

// Borrowed straight from the rust old_io code :)
fn u64_to_be_bytes<T, F>(n: u64, size: u32, f: F) -> T where
    F: FnOnce(&[u8]) -> T,
{
    use std::mem::transmute;

    // LLVM fails to properly optimize this when using shifts instead of the to_be* intrinsics
    assert!(size <= 8);
    match size {
      1 => f(&[n as u8]),
      2 => f(unsafe { & transmute::<_, [u8; 2]>((n as u16).to_be()) }),
      4 => f(unsafe { & transmute::<_, [u8; 4]>((n as u32).to_be()) }),
      8 => f(unsafe { & transmute::<_, [u8; 8]>(n.to_be()) }),
      _ => {
        let mut bytes = vec!();
        let mut i = size;
        while i > 0 {
            let shift = (i - 1) * 8;
            bytes.push((n >> shift) as u8);
            i -= 1;
        }
        f(&bytes)
      }
    }
}

fn u64_to_le_bytes<T, F>(n: u64, size: u32, f: F) -> T where
    F: FnOnce(&[u8]) -> T,
{
    use std::mem::transmute;

    // LLVM fails to properly optimize this when using shifts instead of the to_le* intrinsics
    assert!(size <= 8);
    match size {
      1 => f(&[n as u8]),
      2 => f(unsafe { & transmute::<_, [u8; 2]>((n as u16).to_le()) }),
      4 => f(unsafe { & transmute::<_, [u8; 4]>((n as u32).to_le()) }),
      8 => f(unsafe { & transmute::<_, [u8; 8]>(n.to_le()) }),
      _ => {

        let mut bytes = vec!();
        let mut i = size;
        let mut n = n;
        while i > 0 {
            bytes.push((n & 255_u64) as u8);
            n >>= 8;
            i -= 1;
        }
        f(&bytes)
      }
    }
}

#[cfg(test)]
mod test {
    use mem;
    use mem::Memory;

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
        let mut mem = mem::Fixed::new(1);
        mem.set(0, &[42]).unwrap();
        assert_eq!(mem.get_u8(0).unwrap(), 42);
    }

    #[test]
    pub fn set_u8_writes_single_byte_at_location() {
        let mut mem = mem::Fixed::new(1);
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
        let mut mem = mem::Fixed::new(10);
        mem.set_be_u16(1, 0x2345).unwrap();
        let mut buf = [0, 0];
        mem.get(1, &mut buf).unwrap();
        assert_eq!([0x23, 0x45], buf);
    }

    #[test]
    pub fn set_be_u32_works() {
        let mut mem = mem::Fixed::new(10);
        mem.set_be_u32(1, 0x23456789).unwrap();
        let mut buf = [0, 0, 0, 0];
        mem.get(1, &mut buf).unwrap();
        assert_eq!([0x23, 0x45, 0x67, 0x89], buf);
    }

    #[test]
    pub fn set_be_u64_works() {
        let mut mem = mem::Fixed::new(10);
        mem.set_be_u64(1, 0x23456789ABCDEF00).unwrap();
        let mut buf = [0, 0, 0, 0, 0, 0, 0, 0];
        mem.get(1, &mut buf).unwrap();
        assert_eq!([0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x00], buf);
    }

    #[test]
    pub fn set_le_u16_works() {
        let mut mem = mem::Fixed::new(10);
        mem.set_le_u16(1, 0x2345).unwrap();
        let mut buf = [0, 0];
        mem.get(1, &mut buf).unwrap();
        assert_eq!([0x45, 0x23], buf);
    }

    #[test]
    pub fn set_le_u32_works() {
        let mut mem = mem::Fixed::new(10);
        mem.set_le_u32(1, 0x23456789).unwrap();
        let mut buf = [0, 0, 0, 0];
        mem.get(1, &mut buf).unwrap();
        assert_eq!([0x89, 0x67, 0x45, 0x23], buf);
    }

    #[test]
    pub fn set_le_u64_works() {
        let mut mem = mem::Fixed::new(10);
        mem.set_le_u64(1, 0x23456789ABCDEF00).unwrap();
        let mut buf = [0, 0, 0, 0, 0, 0, 0, 0];
        mem.get(1, &mut buf).unwrap();
        assert_eq!([0x00, 0xEF, 0xCD, 0xAB, 0x89, 0x67, 0x45, 0x23], buf);
    }

    fn init_mem_get() -> mem::Fixed {
        let mut mem = mem::Fixed::new(10);
        mem.set(0, &[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x00, 0x00]).unwrap();
        mem
    }
}