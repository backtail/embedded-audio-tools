//! # Embedded Audio Tools
//!
//! Toolbox for creating audio effects with focus on the embedded aspect of things.
//!
//! ### Memory
//!
//! With `MemSlice` and `MutMemSlice` statically allocated buffers can easily and safely be manipulated.
//! Creating `SubSlice`s of existing buffers is easy an can be either mutable or non-mutable. They also
//! implement `Send` as long as the underlying buffer is considered static. When the size of a buffer is
//! known at compile time, then can this crate handle the task.

#![no_std]

pub(crate) mod all_pass;
pub(crate) mod comb;
pub(crate) mod delay_line;
pub(crate) mod memory;

pub use all_pass::AllPass;
pub use comb::Comb;
pub use delay_line::DelayLine;
pub use memory::mem_slice::MemSlice;
pub use memory::mut_mem_slice::MutMemSlice;

pub mod mut_mem_slice {
    pub use crate::memory::mut_mem_slice::from_slice;
}

pub mod mem_slice {
    pub use crate::memory::mem_slice::from_slice;
}

pub mod errors {
    pub use crate::memory::MemSliceError;
}
