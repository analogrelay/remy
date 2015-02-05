use mem;

use std::collections::dlist::DList;

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

/// Provides an implementation of `mem::Memory` over a list of Memory Layers by performing
/// the memory operation on the first layer that maps the specified address.
pub struct VirtualMemory<'a> {
    layers : DList<Layer<'a>>
}

impl<'a> VirtualMemory<'a> {
    /// Constructs a new Virtual Memory with no member segments
    pub fn new() -> VirtualMemory<'a> {
        VirtualMemory {
            layers: DList::new()
        }
    }

    /// Attaches another memory as the new bottom layer of this virtual memory.
    ///
    /// # Arguments
    /// * `base` - The address to use as the base for the specified memory
    /// * `mem` - The memory to attach.
    pub fn attach_bottom(&mut self, base: usize, mem: Box<mem::Memory+'a>) {
        self.layers.push_back(Layer::new(base, mem));
    }

    /// Attaches another memory as the new top layer of this virtual memory.
    ///
    /// # Arguments
    /// * `base` - The address to use as the base for the specified memory
    /// * `mem` - The memory to attach.
    pub fn attach_top(&mut self, base: usize, mem: Box<mem::Memory+'a>) {
        self.layers.push_front(Layer::new(base, mem));
    }
}

// impl<'a> mem::Memory for VirtualMemory<'a> {
//     fn size(&self) -> usize {
//         unimplemented!()
//     }

//     fn get_u8(&self, addr: usize) -> Result<u8, mem::MemoryError> {
//         match self.layers.iter().find(|&: layer| layer.has_addr(addr)) {
//             Some(layer) =>  {
//                 layer.memory.get_u8(addr - layer.base)
//             },
//             None => Err(mem::MemoryError::OutOfBounds)
//         }
//     }

//     fn set_u8(&mut self, addr: usize, val: u8) -> Result<(), mem::MemoryError> {
//         match self.layers.iter_mut().find(|&: layer| layer.has_addr(addr)) {
//             Some(layer) =>  {
//                 layer.memory.set_u8(addr - layer.base, val)
//             },
//             None => Err(mem::MemoryError::OutOfBounds)
//         }
//     }
// }

// #[cfg(test)]
// mod test {
//     use mem::{Memory,FixedMemory,VirtualMemory};

//     #[test]
//     pub fn attach_bottom_adds_new_segment_at_end_with_specified_base() {
//         let mut vm = VirtualMemory::new();
//         let mem = Box::new(FixedMemory::with_size(10));
//         assert_eq!(vm.layers.len(), 0);
//         vm.attach_bottom(0x1400, mem);
//         assert_eq!(vm.layers.len(), 1);
//         assert_eq!(vm.layers.front().unwrap().base, 0x1400);
//     }

//     #[test]
//     pub fn attach_top_adds_new_segment_at_front_with_specified_base() {
//         let mut vm = VirtualMemory::new();
//         let mem_bot = Box::new(FixedMemory::with_size(10));
//         let mem_top = Box::new(FixedMemory::with_size(10));
//         assert_eq!(vm.layers.len(), 0);
//         vm.attach_bottom(0x1400, mem_bot);
//         vm.attach_top(0x1000, mem_top);
//         assert_eq!(vm.layers.len(), 2);
//         assert_eq!(vm.layers.front().unwrap().base, 0x1000);
//         assert_eq!(vm.layers.back().unwrap().base, 0x1400);
//     }

//     #[test]
//     pub fn get_u8_reads_from_topmost_layer_containing_specified_address() {
//         let mut vm = VirtualMemory::new();
//         let mut mem1 = Box::new(FixedMemory::with_size(10));
//         let mut mem2 = Box::new(FixedMemory::with_size(10));
//         mem1.set_u8(1, 42).unwrap();
//         mem2.set_u8(1, 24).unwrap();
//         vm.attach_top(0x1400, mem2);
//         vm.attach_top(0x1400, mem1);
//         assert_eq!(vm.get_u8(0x1401).unwrap(), 42);
//     }

//     #[test]
//     pub fn set_u8_writes_to_topmost_layer_containing_specified_address() {
//         let mut vm = VirtualMemory::new();
//         let mut mem1 = Box::new(FixedMemory::with_size(10));
//         let mut mem2 = Box::new(FixedMemory::with_size(10));
//         mem1.set_u8(1, 0).unwrap();
//         mem2.set_u8(1, 0).unwrap();

//         vm.attach_top(0x1400, mem2);
//         vm.attach_top(0x1400, mem1);

//         vm.set_u8(0x1401, 42).unwrap();

//         assert_eq!(vm.layers.back().unwrap().memory.get_u8(1).unwrap(), 0);
//         assert_eq!(vm.layers.front().unwrap().memory.get_u8(1).unwrap(), 42);
//     }
// }