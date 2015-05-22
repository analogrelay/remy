use mem;

/// Represents a memory with absolutely no addresses. All operations return
/// an `OutOfBounds` error.
pub struct Empty;

impl mem::Memory for Empty {
    fn len(&self) -> u64 { 0 }

    #[allow(unused_variables)]
    fn get_u8(&self, addr: u64) -> mem::Result<u8> {
        Err(mem::Error::new(mem::ErrorKind::MemoryNotReadable, "EmptyMemory cannot be read from"))
    }

    #[allow(unused_variables)]
    fn set_u8(&mut self, addr: u64, val: u8) -> mem::Result<()> {
        Err(mem::Error::new(mem::ErrorKind::MemoryNotWritable, "EmptyMemory cannot be written to"))
    }
}
