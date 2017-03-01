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

serialize_via_debug!(Error);

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
/// Defines the kind of a `Error`
pub enum ErrorKind {
    /// The provided address was outside the bounds of the memory
    OutOfBounds,

    /// The provided address referred to memory that is not readable
    MemoryNotReadable,

    /// The provided address referred to memory that is not writable
    MemoryNotWritable,

    /// The memory represented by this object is not currently available
    MemoryNotPresent,

    /// An unknown error occurred
    Other
}

/// Represents any memory accessible to a CPU
///
/// Implementations of this may use various sparse storage techniques to avoid
/// allocating the entire memory buffer, or may use ROM content from files to
/// back the memory. In the current implementation, the memory may not have an
/// address size larger than 64-bits. Internally, all addresses are expected to
/// be provided as 64-bit integers.
pub trait Memory {
    /// Gets the size of the memory
    fn len(&self) -> u64;

    /// Reads a single byte from the memory at `addr`
    ///
    /// # Arguments
    /// * `addr` - The address at which to begin reading the data
    fn get_u8(&self, addr: u64) -> Result<u8>;

    /// Writes a single byte `val` to the memory at `addr`
    ///
    /// # Arguments
    /// * `addr` - The address at which to begin writing the data
    /// * `val` - The byte to set
    fn set_u8(&mut self, addr: u64, val: u8) -> Result<()>;

    /// Fills the provided buffer with data from the memory starting at `addr`
    fn get(&self, addr: u64, buf: &mut [u8]) -> Result<()> {
        for i in 0..buf.len() {
            buf[i] = try!(self.get_u8(addr + i as u64));
        }
        Ok(())
    }

    /// Writes the provided buffer to the memory starting at `addr`
    fn set(&mut self, addr: u64, buf: &[u8]) -> Result<()> {
        for i in 0..buf.len() {
            try!(self.set_u8(addr + i as u64, buf[i]))
        }
        Ok(())
    }
}

/// Extension trait that provides the ability to read specific values out of memory
pub trait MemoryExt: Memory {
    /// Gets a u16 value, in the specified byte order `B`, from the address specified by `addr`
    fn get_u16<B>(&self, addr: u64) -> Result<u16> where B: ByteOrder {
        let mut raw = [0u8; 2];
        try!(self.get(addr, &mut raw));
        Ok(<B as ByteOrder>::read_u16(&raw))
    }

    /// Gets a i16 value, in the specified byte order `B`, from the address specified by `addr`
    fn get_i16<B>(&self, addr: u64) -> Result<i16> where B: ByteOrder {
        let mut raw = [0u8; 2];
        try!(self.get(addr, &mut raw));
        Ok(<B as ByteOrder>::read_i16(&raw))
    }

    /// Gets a u32 value, in the specified byte order `B`, from the address specified by `addr`
    fn get_u32<B>(&self, addr: u64) -> Result<u32> where B: ByteOrder {
        let mut raw = [0u8; 4];
        try!(self.get(addr, &mut raw));
        Ok(<B as ByteOrder>::read_u32(&raw))
    }

    /// Gets a i32 value, in the specified byte order `B`, from the address specified by `addr`
    fn get_i32<B>(&self, addr: u64) -> Result<i32> where B: ByteOrder {
        let mut raw = [0u8; 4];
        try!(self.get(addr, &mut raw));
        Ok(<B as ByteOrder>::read_i32(&raw))
    }

    /// Gets a u64 value, in the specified byte order `B`, from the address specified by `addr`
    fn get_u64<B>(&self, addr: u64) -> Result<u64> where B: ByteOrder {
        let mut raw = [0u8; 8];
        try!(self.get(addr, &mut raw));
        Ok(<B as ByteOrder>::read_u64(&raw))
    }

    /// Gets a i64 value, in the specified byte order `B`, from the address specified by `addr`
    fn get_i64<B>(&self, addr: u64) -> Result<i64> where B: ByteOrder {
        let mut raw = [0u8; 8];
        try!(self.get(addr, &mut raw));
        Ok(<B as ByteOrder>::read_i64(&raw))
    }

    /// Writes the u16 value specified in `val` to the address specified by `addr`, in the
    /// specified byte order `B`
    fn set_u16<B>(&mut self, addr: u64, val: u16) -> Result<()> where B: ByteOrder {
        let mut buf = [0u8; 2];
        <B as ByteOrder>::write_u16(&mut buf, val);
        try!(self.set(addr, &buf));
        Ok(())
    }

    /// Writes the i16 value specified in `val` to the address specified by `addr`, in the
    /// specified byte order `B`
    fn set_i16<B>(&mut self, addr: u64, val: i16) -> Result<()> where B: ByteOrder {
        let mut buf = [0u8; 2];
        <B as ByteOrder>::write_i16(&mut buf, val);
        try!(self.set(addr, &buf));
        Ok(())
    }

    /// Writes the u32 value specified in `val` to the address specified by `addr`, in the
    /// specified byte order `B`
    fn set_u32<B>(&mut self, addr: u64, val: u32) -> Result<()> where B: ByteOrder {
        let mut buf = [0u8; 4];
        <B as ByteOrder>::write_u32(&mut buf, val);
        try!(self.set(addr, &buf));
        Ok(())
    }

    /// Writes the i32 value specified in `val` to the address specified by `addr`, in the
    /// specified byte order `B`
    fn set_i32<B>(&mut self, addr: u64, val: i32) -> Result<()> where B: ByteOrder {
        let mut buf = [0u8; 4];
        <B as ByteOrder>::write_i32(&mut buf, val);
        try!(self.set(addr, &buf));
        Ok(())
    }

    /// Writes the u64 value specified in `val` to the address specified by `addr`, in the
    /// specified byte order `B`
    fn set_u64<B>(&mut self, addr: u64, val: u64) -> Result<()> where B: ByteOrder {
        let mut buf = [0u8; 8];
        <B as ByteOrder>::write_u64(&mut buf, val);
        try!(self.set(addr, &buf));
        Ok(())
    }

    /// Writes the i64 value specified in `val` to the address specified by `addr`, in the
    /// specified byte order `B`
    fn set_i64<B>(&mut self, addr: u64, val: i64) -> Result<()> where B: ByteOrder {
        let mut buf = [0u8; 8];
        <B as ByteOrder>::write_i64(&mut buf, val);
        try!(self.set(addr, &buf));
        Ok(())
    }
}

impl<M: Memory + ?Sized> MemoryExt for M {}

#[cfg(test)]
mod test {
    use mem;
    use mem::{Memory,MemoryExt};
    use byteorder::{BigEndian,LittleEndian};

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
    pub fn get_be_u16_works() { assert_eq_hex!(0x2345, init_mem_get().get_u16::<BigEndian>(1).unwrap()); }

    #[test]
    pub fn get_le_u16_works() { assert_eq_hex!(0x4523, init_mem_get().get_u16::<LittleEndian>(1).unwrap()); }

    #[test]
    pub fn get_be_u32_works() { assert_eq_hex!(0x23456789, init_mem_get().get_u32::<BigEndian>(1).unwrap()); }

    #[test]
    pub fn get_le_u32_works() { assert_eq_hex!(0x89674523, init_mem_get().get_u32::<LittleEndian>(1).unwrap()); }

    #[test]
    pub fn get_be_u64_works() { assert_eq_hex!(0x23456789ABCDEF00, init_mem_get().get_u64::<BigEndian>(1).unwrap()); }

    #[test]
    pub fn get_le_u64_works() { assert_eq_hex!(0x00EFCDAB89674523, init_mem_get().get_u64::<LittleEndian>(1).unwrap()); }

    #[test]
    pub fn set_be_u16_works() {
        let mut mem = mem::Fixed::new(10);
        mem.set_u16::<BigEndian>(1, 0x2345).unwrap();
        let mut buf = [0, 0];
        mem.get(1, &mut buf).unwrap();
        assert_eq!([0x23, 0x45], buf);
    }

    #[test]
    pub fn set_be_u32_works() {
        let mut mem = mem::Fixed::new(10);
        mem.set_u32::<BigEndian>(1, 0x23456789).unwrap();
        let mut buf = [0, 0, 0, 0];
        mem.get(1, &mut buf).unwrap();
        assert_eq!([0x23, 0x45, 0x67, 0x89], buf);
    }

    #[test]
    pub fn set_be_u64_works() {
        let mut mem = mem::Fixed::new(10);
        mem.set_u64::<BigEndian>(1, 0x23456789ABCDEF00).unwrap();
        let mut buf = [0, 0, 0, 0, 0, 0, 0, 0];
        mem.get(1, &mut buf).unwrap();
        assert_eq!([0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x00], buf);
    }

    #[test]
    pub fn set_le_u16_works() {
        let mut mem = mem::Fixed::new(10);
        mem.set_u16::<LittleEndian>(1, 0x2345).unwrap();
        let mut buf = [0, 0];
        mem.get(1, &mut buf).unwrap();
        assert_eq!([0x45, 0x23], buf);
    }

    #[test]
    pub fn set_le_u32_works() {
        let mut mem = mem::Fixed::new(10);
        mem.set_u32::<LittleEndian>(1, 0x23456789).unwrap();
        let mut buf = [0, 0, 0, 0];
        mem.get(1, &mut buf).unwrap();
        assert_eq!([0x89, 0x67, 0x45, 0x23], buf);
    }

    #[test]
    pub fn set_le_u64_works() {
        let mut mem = mem::Fixed::new(10);
        mem.set_u64::<LittleEndian>(1, 0x23456789ABCDEF00).unwrap();
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
