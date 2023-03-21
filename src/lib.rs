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
//!
//! ### DSP building blocks
//!
//! This crate inlcudes basic compents to customize an audio effect:
//!     - Delay line
//!     - Comb filter
//!     - Allpass filter
//!
//! ### Floats
//!
//! Interpolate or bitmanipulate audio streams or buffers.

#![no_std]

pub(crate) mod all_pass;
pub(crate) mod biquad;
pub(crate) mod comb;
pub(crate) mod delay_line;
pub(crate) mod float;
pub(crate) mod memory;

pub mod decibels;
pub mod envelope;
pub mod oscillator;
pub mod phase_accumulator;
pub mod stereo;

pub use all_pass::AllPass;
pub use comb::Comb;
pub use delay_line::DelayLine;
pub use float::conversion::F32Components;
pub use memory::mem_slice::MemSlice;
pub use memory::mut_mem_slice::MutMemSlice;

pub mod filter {
    pub use crate::biquad::{butterworth::Butterworth, Biquad, BiquadCoeffs};

    pub mod butterworth {
        pub use crate::biquad::butterworth::ButterworthType;
    }
}

pub mod memory_access {
    pub use crate::memory::memory_slice::{from_slice, from_slice_mut, MemorySlice};
    pub use crate::memory::{Mutable, NonMutable};
}

pub mod mut_mem_slice {
    pub use crate::memory::mut_mem_slice::from_slice;
}

pub mod mem_slice {
    pub use crate::memory::mem_slice::from_slice;
}

pub mod interpolation {
    pub use crate::float::interpolation::{
        lagrange, lagrange_only_4_elements, lerp, lerp_unchecked,
    };
}

pub mod bit_manipulation {
    pub use crate::float::bit_manipulation::{
        bit_reduce, bit_reduce_exp, bit_reduce_exp_unchecked, bit_reduce_unchecked,
    };
}

pub mod errors {
    pub use crate::float::bit_manipulation::BitReductionError;
    pub use crate::float::interpolation::InterpolationError;
    pub use crate::memory::MemSliceError;
}

pub mod traits {
    pub use crate::float::conversion::{FromF32Components, FromRaw, ToF32Components, ToRaw};
}
