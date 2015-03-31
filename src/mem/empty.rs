use mem;

/// Represents a memory with absolutely no addresses. All operations return
/// an `OutOfBounds` error.
pub struct Empty;

impl mem::Memory for Empty {
    fn size(&self) -> usize { 0 }

    #[allow(unused_variables)]
    fn get(&self, addr: usize, buf: &mut [u8]) -> mem::Result<()> {
        Err(mem::Error::new(mem::ErrorKind::OutOfBounds, "EmptyMemory cannot be read from"))
    }

    #[allow(unused_variables)]
    fn set(&mut self, addr: usize, buf: &[u8]) -> mem::Result<()> {
        Err(mem::Error::new(mem::ErrorKind::OutOfBounds, "EmptyMemory cannot be written to"))
    }
}
