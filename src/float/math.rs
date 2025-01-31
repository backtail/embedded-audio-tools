use crate::{
    fixed_point::math::sin_i16_unchecked,
    float::integral::simpsons_rule,
    float::lerp_unchecked,
    memory_access::from_slice,
    oscillator::lookup_tables::{bl_rect::BANDLIMITED_RECT, sine_table},
};

use core::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};
use core::ops::Neg;

#[allow(unused_imports)]
use micromath::F32Ext;

/// Extends the feature set of functions for `f32` with `#![no_std]` compatibility:
///
/// * sinh(x)
/// * cosh(x)
/// * sinc(x)
/// * SI(x) (sine integral)
///
/// Also adds some approximation functions which perform faster at the cost of accuracy:
///
/// * sin(x) (LUT)
/// * sin(x) (fixed point Taylor series approximation)
/// * cos(x) (fixed point Taylor series approximation)
/// * tan(x) (Taylor series expansion)
/// * rect(x) (bandlimiting LUT)
pub trait AdditionalF32Ext {
    type Output;
    fn si(&self) -> Self::Output;
    fn sinc(&self) -> Self::Output;
    fn sinh(&self) -> Self::Output;
    fn cosh(&self) -> Self::Output;
    fn fast_tan(&self) -> Self::Output;
    fn tanh(&self) -> Self::Output;
    fn lookup_sin(&self) -> Self::Output;
    fn lookup_bl_rect(&self) -> Self::Output;
    fn fixed_point_sin(&self) -> Self::Output;
    fn fixed_point_cos(&self) -> Self::Output;
}

impl AdditionalF32Ext for f32 {
    type Output = f32;

    /// Computes the sinus hyperbolicus
    ///
    /// ## Example
    /// ```rust
    /// use embedded_audio_tools::float::AdditionalF32Ext;
    ///
    /// assert_eq!(1.0.sinh(), 1.17520119);
    /// ```
    fn sinh(&self) -> Self::Output {
        (self.exp() - self.neg().exp()) * 0.5
    }

    /// Computes the cosinus hyperbolicus
    ///
    /// ## Example
    /// ```rust
    /// use embedded_audio_tools::float::AdditionalF32Ext;
    ///
    /// assert_eq!(1.0.cosh(), 1.5430806);
    /// ```
    fn cosh(&self) -> Self::Output {
        (self.exp() + self.neg().exp()) * 0.5
    }

    /// Taylor series expansion of tan(x), where x = 0
    ///
    /// ## Accuracy
    ///
    /// Since this function is primarily for filter coefficient calculations,
    /// only the range between 0 and 1 is accurately represented. See below
    /// for the exact accuracy.
    ///
    /// ## Example
    /// ```rust
    /// use embedded_audio_tools::float::AdditionalF32Ext;
    ///
    /// assert!(0.0.fast_tan() == 0.0);
    /// assert!((0.5_f32.tan() - 0.5.fast_tan()).abs() < f32::EPSILON);
    /// assert!((1.0_f32.tan() - 1.0.fast_tan()).abs() == 0.0009920597);
    /// ```
    ///
    fn fast_tan(&self) -> Self::Output {
        let mut res = 0.0_f32;
        res += self;
        res += (1.0 / 3.0) * self.powi(3);
        res += (2.0 / 15.0) * self.powi(5);
        res += (17.0 / 315.0) * self.powi(7);
        res += (62.0 / 2835.0) * self.powi(9);
        res += (1382.0 / 155925.0) * self.powi(11);
        res += (21844.0 / 6081075.0) * self.powi(13);
        res += (929569.0 / 638512875.0) * self.powi(15);
        res
    }

    /// Taylor series expansion of tanh(x), where x = 0
    ///
    /// tanh(x) = sinh(x) / cosh(x)
    /// where sinh(x) = (e^x - e^(-x)) / 2
    /// and cosh(x) = (e^x + e^(-x)) / 2
    fn tanh(&self) -> Self::Output {
        fn exp_approx(x: f32) -> f32 {
            1.0 + x + (x * x) / 2.0 // Simple approximation of e^x
        }

        // Calculate sinh(x) and cosh(x)
        let exp_x = exp_approx(*self);
        let exp_neg_x = exp_approx(self.neg());

        let sinh_x = (exp_x - exp_neg_x) / 2.0;
        let cosh_x = (exp_x + exp_neg_x) / 2.0;

        sinh_x / cosh_x
    }

    /// Fixed point approximation of the sine function
    ///
    /// ## Example
    /// ```rust
    /// # use core::f32::consts::{PI, FRAC_PI_2};
    /// use embedded_audio_tools::float::AdditionalF32Ext;
    ///
    /// assert_eq!(0.0.fixed_point_sin(), 0.0);
    /// assert_eq!((FRAC_PI_2.sin() - FRAC_PI_2.fixed_point_sin()).abs(), 0.000030517578);
    /// assert_eq!((PI.sin() - PI.fixed_point_sin()).abs(), 0.00000008742278);
    /// ```
    fn fixed_point_sin(&self) -> Self::Output {
        let normalized_rads = ((self / 2.0 - FRAC_PI_4).rem_euclid(PI) / FRAC_PI_4) - 2.0;
        let quadrant_rads = normalized_rads.abs() - 1.0;

        unsafe {
            sin_i16_unchecked((i16::MAX as f32 * quadrant_rads) as i16, 4) as f32 / i16::MAX as f32
        }
    }

    /// Fixed point approximation of the sine function
    ///
    /// ## Example
    /// ```rust
    /// # use core::f32::consts::{PI, FRAC_PI_2};
    /// use embedded_audio_tools::float::AdditionalF32Ext;
    ///
    /// assert_eq!(0.0.fixed_point_cos(), 0.9999695);
    /// assert_eq!((FRAC_PI_2.cos() - FRAC_PI_2.fixed_point_cos()).abs(), 4.371139e-8);
    /// assert_eq!((PI.cos() - PI.fixed_point_cos()).abs(), 3.0517578e-5);
    /// ```
    fn fixed_point_cos(&self) -> Self::Output {
        (FRAC_PI_2 - self).fixed_point_sin()
    }

    /// Accepts values between 0 and 1, otherwise clamps at boundery
    ///
    /// Bandlimited rectangle function
    fn lookup_bl_rect(&self) -> Self::Output {
        let buffer = from_slice(&BANDLIMITED_RECT[..]);
        let len = buffer.len() - 1;

        if *self <= 0.0 {
            return 0.0;
        }

        if *self <= 0.25 {
            let f_index = self * 4.0;
            return unsafe { buffer.lerp_unchecked(f_index * len as f32) };
        }

        if *self <= 0.5 {
            let f_index = (0.25 - (self - 0.25)) * 4.0;
            return unsafe { buffer.lerp_unchecked(f_index * len as f32) };
        }

        if *self <= 0.75 {
            let f_index = (self - 0.5) * 4.0;
            return unsafe { -1.0 * buffer.lerp_unchecked(f_index * len as f32) };
        }

        if *self < 1.0 {
            let f_index = (0.25 - (self - 0.75)) * 4.0;
            return unsafe { -1.0 * buffer.lerp_unchecked(f_index * len as f32) };
        }

        return 0.0;
    }

    /// Interpolated fixed point approximation lookup of the sine function
    ///
    /// Not accurate at all in moment!
    fn lookup_sin(&self) -> Self::Output {
        const SINE_LOOKUP: [i16; 4096] = sine_table::<4096>();

        let normalized_rads = ((self / 2.0 - FRAC_PI_4).rem_euclid(PI) / FRAC_PI_4) - 2.0;
        let quadrant_rads = normalized_rads.abs() / 2.0;

        let f_index = (SINE_LOOKUP.len() - 1) as f32 * quadrant_rads;
        let i_index = f_index as usize;

        if i_index != SINE_LOOKUP.len() - 1 {
            lerp_unchecked(
                SINE_LOOKUP[i_index] as f32,
                SINE_LOOKUP[i_index + 1] as f32,
                f_index - i_index as f32,
            ) / i16::MAX as f32
        } else {
            SINE_LOOKUP[i_index] as f32 / i16::MAX as f32
        }
    }

    /// Computes sin(x)/x
    fn sinc(&self) -> Self::Output {
        __sinc_f32(*self)
    }

    /// Computes the sine integral from 0 to `self`. The smaller the number, the more accurate the result.
    fn si(&self) -> Self::Output {
        simpsons_rule::<1000>(__sinc_f32, 0.0, *self)
    }
}

/// Computes sin(x)/x for f32 with a fixed point approximation of sin(x)
#[inline(always)]
fn __sinc_f32(val: f32) -> f32 {
    if val == 0.0 {
        1.0
    } else {
        val.fixed_point_sin() / val
    }
}
