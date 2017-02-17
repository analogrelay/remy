// Deny ALL THE THINGS!
#![deny(
    unused_mut,
    deprecated,
    unknown_lints,
    unused_parens,
    unused_imports,
    unused_must_use,
    unused_features,
    unused_variables,
    unused_comparisons,
    non_shorthand_field_patterns)]

// Import crates

/// Copy of the 'slog' logging crate (https://github.com/slog-rs/slog). Will be overridden (and shared) by the application's copy if one is added
#[macro_use]
pub extern crate slog;
extern crate slog_stdlog;

extern crate byteorder;

// Internal macros
#[macro_use]
mod macros;

/// Contains code to emulate supported Hardware
pub mod hw;

/// Contains abstractions useful for Memory Management
pub mod mem;

/// Contains abstractions useful for implementing instruction sets
pub mod instr;

/// Contains a Program Counter object to track program counter position
pub mod pc;

/// Contains code to run various complete systems
pub mod systems;

/// Contains code to manage clock cycles
pub mod clock;