pub use mem::fixed::Fixed;
pub use mem::virt::Virtual;
pub use mem::empty::Empty;
pub use mem::memory::{Result,Error,ErrorKind,Memory};
pub use mem::mirrored::Mirrored;

/// Declares the core `Memory` trait shared by all memory abstractions
pub mod memory;

/// Provides types for working with fixed memory banks
pub mod fixed;

/// Provides types for working with virtual memory banks
pub mod virt;

/// Provides types for working with empty memory banks
pub mod empty;


/// Provides types for working with memory banks that mirror internal contents across a larger
/// space
pub mod mirrored;
