mod bit_manipulation;
mod conversion;
mod dsp_util;
mod interpolation;
mod math;

pub(crate) mod integral;

pub use bit_manipulation::*;
pub use conversion::*;
pub use dsp_util::DSPUtility;
pub use interpolation::*;
pub use math::AdditionalF32Ext;
pub use micromath::F32Ext;

pub trait F32Tools {
    type Output;
    fn map(&self, from: Self::Output, to: Self::Output) -> Self::Output;
}

impl F32Tools for f32 {
    type Output = f32;

    /// Maps (linearly scales) `self` in case of \[`0.0`; `1.0`] -> \[`from`; `to`]
    ///
    /// Does not perform any error or bound checking!
    ///
    /// ## Example
    /// ```rust
    /// use embedded_audio_tools::float::F32Tools;
    ///
    /// assert_eq!(0.5.map(1.0, 2.0), 1.5);
    /// ```
    fn map(&self, from: Self::Output, to: Self::Output) -> Self::Output {
        (from * (1.0 - self)) + (to * self)
    }
}
