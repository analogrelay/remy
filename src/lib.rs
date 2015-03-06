#![deny(deprecated,unused_imports)]

#![feature(alloc)]
#![feature(core)]

/// Contains code to emulate supported CPUs
pub mod cpu;

/// Contains abstractions useful for Memory Management
pub mod mem;

/// Contains a Program Counter object to track program counter position
pub mod pc;
