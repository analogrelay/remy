#![deny(deprecated,unused_imports)]

#![feature(alloc)]
#![feature(core)]

/// Contains code to emulate supported CPUs
pub mod cpus;

/// Contains abstractions useful for Memory Management
pub mod mem;

/// Contains abstractions useful for implementing instruction sets
pub mod instruction_set;

/// Contains a Program Counter object to track program counter position
pub mod pc;

/// Contains macros useful for defining CPUs
mod macros;
