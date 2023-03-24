use core::ops::Neg;

use super::conversion::{FromF32Components, ToF32Components};
use BitReductionError::*;

const MAX_RANGE: f32 = 0x7FFFFFFF_u32 as f32;

pub enum BitReductionError {
    OverBitReduction,
    InputExceedsRange,
}

#[inline(always)]
pub fn bit_reduce_unchecked(input: f32, bit_depth: u8) -> f32 {
    if bit_depth != 0 {
        if input.is_sign_positive() {
            // Scale input to biggest signed integer
            let scaled = (input * MAX_RANGE) as u32;

            // Shift it back and forth
            let shifted = ((scaled >> bit_depth) << bit_depth) as f32;

            // Scale back again
            shifted / MAX_RANGE
        } else if input.is_sign_negative() {
            // Scale input to biggest signed integer
            let scaled = (input.neg() * MAX_RANGE) as u32;

            // Shift it back and forth
            let shifted = ((scaled >> bit_depth) << bit_depth) as f32;

            // Scale back again
            shifted.neg() / MAX_RANGE
        } else {
            input
        }
    } else {
        input
    }
}

#[inline(always)]
pub fn bit_reduce(input: f32, bit_depth: u8) -> Result<f32, BitReductionError> {
    if bit_depth > 30 {
        return Err(OverBitReduction);
    }

    if input < -1.0 || input > 1.0 {
        return Err(InputExceedsRange);
    }

    Ok(bit_reduce_unchecked(input, bit_depth))
}

#[inline(always)]
pub fn bit_reduce_exp_unchecked(input: f32, bit_depth: u8) -> f32 {
    if bit_depth != 0 {
        // Extract IEEE-754 floating point components
        let mut components = input.to_f32_components();

        // Shift mantissa back and forth
        components.mantissa = (components.mantissa >> bit_depth) << bit_depth;

        // Return exponentially bit reduced value
        f32::from_f32_components(components)
    } else {
        input
    }
}

pub fn bit_reduce_exp(input: f32, bit_depth: u8) -> Result<f32, BitReductionError> {
    if bit_depth > 23 {
        return Err(OverBitReduction);
    }

    Ok(bit_reduce_exp_unchecked(input, bit_depth))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::convert::FromRawBytes;
    use core::f32::consts::PI;

    #[test]
    fn exponential_reduction() {
        assert_eq!(bit_reduce_exp_unchecked(PI, 0), PI);
        assert_eq!(
            bit_reduce_exp_unchecked(PI, 4),
            f32::from_raw_word(0x40490fd0)
        );
    }
}
