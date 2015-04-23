use std::{error,fmt};
use byteorder::ByteOrder;

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

impl error::Error for Error {
    fn description(&self) -> &str {
        self.desc
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use std::error::Error;
        self.description().fmt(fmt)
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
    fn len(&self) -> u64;

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
}

pub trait MemoryExt: Memory {
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

    /// Reads a u16 value in the specified byte order
    fn get_u16<B>(&self, addr: u64) -> Result<u16> where B: ByteOrder {
        let mut raw = [0u8; 2];
        try!(self.get(addr, &mut raw));
        Ok(ByteOrder::read_u16(&raw))
    }

    /// Reads an i16 value in the specified byte order
    fn get_i16<B>(&self, addr: u64) -> Result<i16> where B: ByteOrder {
        let mut raw = [0u8; 2];
        try!(self.get(addr, &mut raw));
        Ok(ByteOrder::read_i16(&raw))
    }

    /// Reads a u32 value in the specified byte order
    fn get_u32<B>(&self, addr: u64) -> Result<u32> where B: ByteOrder {
        let mut raw = [0u8; 4];
        try!(self.get(addr, &mut raw));
        Ok(ByteOrder::read_u32(&raw))
    }

    /// Reads an i32 value in the specified byte order
    fn get_i32<B>(&self, addr: u64) -> Result<i32> where B: ByteOrder {
        let mut raw = [0u8; 4];
        try!(self.get(addr, &mut raw));
        Ok(ByteOrder::read_i32(&raw))
    }

    /// Reads a u64 value in the specified byte order
    fn get_u64<B>(&self, addr: u64) -> Result<u64> where B: ByteOrder {
        let mut raw = [0u8; 8];
        try!(self.get(addr, &mut raw));
        Ok(ByteOrder::read_u64(&raw))
    }

    /// Reads an i64 value in the specified byte order
    fn get_i64<B>(&self, addr: u64) -> Result<i64> where B: ByteOrder {
        let mut raw = [0u8; 8];
        try!(self.get(addr, &mut raw));
        Ok(ByteOrder::read_i64(&raw))
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

    /// Writes a u16 value in the specified byte order
    fn set_u16<B>(&self, addr: u64, val: u16) -> Result<()> {
        let mut buf = [0u8; 2];
        ByteOrder::write_u16(&mut buf, val);
        try!(self.set(addr, &buf));
        Ok(())
    }
}

impl<M: Memory + ?Sized> MemoryExt for M {}

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
