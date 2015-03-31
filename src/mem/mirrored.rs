use mem;

/// Provides a `mem::Memory` that mirrors a provided `Memory` through a certain size
///
/// This is useful in systems like the NES where the 2KB of on-board RAM occupy addresses
/// 0x0000 through 0x2000 and simply repeat every 0x800 bytes. So a read or write to
/// 0x0042 is exactly the same as a read or write to 0x0842 or 0x1042 or 0x1842
pub struct Mirrored<M> where M: mem::Memory {
    end: u64,
    mem: M
}

impl<M> mem::Memory for Mirrored<M> where M: mem::Memory {
    fn size(&self) -> u64 { self.end }

    fn get(&self, addr: u64, buf: &mut [u8]) -> mem::Result<()> {
        unimplemented!()
    }

    fn set(&mut self, addr: u64, buf: &[u8]) -> mem::Result<()> {
        unimplemented!()
    }
}
