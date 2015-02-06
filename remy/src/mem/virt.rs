use mem;

use std::cmp;

struct Layer<'a> {
    base : usize,
    memory : Box<mem::Memory+'a>
}

impl<'a> Layer<'a> {
    fn new(base: usize, memory: Box<mem::Memory+'a>) -> Layer<'a> {
        Layer {
            base: base,
            memory: memory
        }
    }

    fn has_addr(&self, addr: usize) -> bool {
        addr >= self.base && addr < (self.base + self.memory.size())
    }
}

pub enum VirtualMemoryError {
	MemoryOverlap
}

/// Provides an implementation of `mem::Memory` over a list of memories by performing
/// the memory operation on the memory that is mapped at the specified base address
///
/// Warning: Memories may NOT overlap
pub struct VirtualMemory<'a> {
    layers : Vec<Layer<'a>>
}

impl<'a> VirtualMemory<'a> {
    /// Constructs a new Virtual Memory with no member segments
    pub fn new() -> VirtualMemory<'a> {
        VirtualMemory {
            layers: Vec::new()
        }
    }

    /// Attaches another memory to the virtual memory
    ///
    /// # Arguments
    /// * `base` - The address to use as the base for the specified memory
    /// * `mem` - The memory to attach.
    pub fn attach(&mut self, base: usize, mem: Box<mem::Memory+'a>) -> Result<(), VirtualMemoryError> {
    	// Find the appropriate place to attach the memory
    	let new_layer = Layer::new(base, mem);
    	let pos = self.layers.iter()
    		.position(|l| l.base > new_layer.base);

    	let insert_point = match pos {
    		None => self.layers.len(),
    		Some(x) => x
    	};

    	if insert_point > 0 {
    		// Check the memory on the left
    		let left = &self.layers[insert_point - 1];
    		if left.base + left.memory.size() >= base {
    			return Err(VirtualMemoryError::MemoryOverlap)
    		}
    	}

    	self.layers.insert(insert_point, new_layer);
    	Ok(())
    }

    fn find(&self, addr: usize) -> Option<&Layer<'a>> {
    	self.layers.iter().find(|l| l.has_addr(addr))
    }

    fn find_mut(&mut self, addr: usize) -> Option<&mut Layer<'a>> {
    	self.layers.iter_mut().find(|l| l.has_addr(addr))
    }
}

impl<'a> mem::Memory for VirtualMemory<'a> {
    fn size(&self) -> usize {
        unimplemented!()
    }

    fn get(&self, addr: usize, buf: &mut [u8]) -> mem::MemoryResult<()> {
    	let mut ptr = 0;
    	while ptr < buf.len() {
    		// Find the memory at the current address
    		let layer = match self.find(addr + ptr) {
    			Some(l) => l,
    			None => return Err(mem::MemoryError::with_detail(
    				mem::MemoryErrorKind::OutOfBounds,
    				"Unable to locate a suitable memory layer",
    				format!("at address: 0x{:X}", addr + ptr)))
    		};

    		// Calculate effective address
    		let eaddr = (addr - layer.base) + ptr;

    		// Figure out how much to read
    		let to_read = cmp::min(layer.memory.size() - eaddr, buf.len() - ptr);

    		// Read that much
    		if let Err(e) = layer.memory.get(eaddr, &mut buf[ptr .. (ptr+to_read)]) {
    			return Err(e)
    		}

    		// Advance the pointer
    		ptr = ptr + to_read;
    	}

    	Ok(())
    }

    fn set(&mut self, addr: usize, buf: &[u8]) -> mem::MemoryResult<()> {
        let mut ptr = 0;
    	while ptr < buf.len() {
    		// Find the memory at the current address
    		let layer = match self.find_mut(addr + ptr) {
    			Some(l) => l,
    			None => return Err(mem::MemoryError::with_detail(
    				mem::MemoryErrorKind::OutOfBounds,
    				"Unable to locate a suitable memory layer",
    				format!("at address: 0x{:X}", addr + ptr)))
    		};

    		// Calculate effective address
    		let eaddr = (addr - layer.base) + ptr;

    		// Figure out how much to write
    		let to_read = cmp::min(layer.memory.size() - eaddr, buf.len() - ptr);

    		// Write that much
    		if let Err(e) = layer.memory.set(eaddr, &buf[ptr .. (ptr+to_read)]) {
    			return Err(e)
    		}

    		// Advance the pointer
    		ptr = ptr + to_read;
    	}

    	Ok(())
    }
}

#[cfg(test)]
mod test {
    use mem::{Memory,FixedMemory,VirtualMemory};

    // Tests:
    // * attach with no items -> OK
    // * attach at end with no overlap -> OK
    // * attach at end with overlap -> ERR
    // * attach at beginning with no overlap -> OK
    // * attach at beginning with overlap -> ERR
    // * attach in middle with no overlap -> OK
    // * attach in middle with overlap -> ERR

    // * get from one memory
    // * get spanning memories
    // * set to one memory
    // * set spanning memories
}