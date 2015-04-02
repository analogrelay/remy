#![deny(deprecated,unused_imports,unused_must_use,unused_mut,unused_features)]

// Features used in all builds
#![feature(core)]
#![feature(debug_builders)]

// Features used by tests
#![cfg_attr(test, feature(convert))]

/// Contains code to emulate supported CPUs
pub mod cpus;

/// Contains abstractions useful for Memory Management
pub mod mem;

/// Contains abstractions useful for implementing instruction sets
pub mod instr;

/// Contains a Program Counter object to track program counter position
pub mod pc;

/// Contains code to run various complete systems 
pub mod systems;
