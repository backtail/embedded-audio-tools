#[allow(unused_imports)]
use micromath::F32Ext;

use core::num::FpCategory::{self, *};

pub trait Decibels {
    /// Can yield a `-INF` and `NaN`
    fn to_decibels_unchecked(&self) -> f32;

    /// Cannot yield a `NaN` **but it can yield** `-INF`. Passes a `DecibelsError` if voltage ratio is < 0.
    fn to_decibels(&self) -> Result<f32, FpCategory>;

    /// Cannot yield a `-INF` or `NaN`. Instead passes a `DecibelsError` to handle further processing.
    fn to_decibels_checked(&self) -> Result<f32, FpCategory>;

    /// Outputs a 0.0 if used on a `-INF`.
    fn to_volt_ratio(&self) -> f32;
}

impl Decibels for f32 {
    #[inline(always)]
    fn to_decibels_unchecked(&self) -> f32 {
        20.0 * self.log10()
    }

    #[inline(always)]
    fn to_decibels(&self) -> Result<f32, FpCategory> {
        if *self < 0.0 {
            return Err(Nan);
        }

        Ok(self.to_decibels_unchecked())
    }

    #[inline(always)]
    fn to_decibels_checked(&self) -> Result<f32, FpCategory> {
        if *self < 0.0 {
            return Err(Nan);
        }

        if *self == 0.0 {
            return Err(Infinite);
        }

        Ok(self.to_decibels_unchecked())
    }

    #[inline(always)]
    fn to_volt_ratio(&self) -> f32 {
        10.0.powf(self / 20.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion() {
        assert_eq!(1.0_f32.to_decibels_unchecked(), 0.0);
        assert_eq!(0.0_f32.to_volt_ratio(), 1.0);

        assert_eq!(f32::NEG_INFINITY.to_volt_ratio(), 0.0);
        assert_eq!(0.0_f32.to_decibels_unchecked(), f32::NEG_INFINITY);
    }

    #[test]
    fn check_errors() {
        assert_eq!(0.0_f32.to_decibels(), Ok(f32::NEG_INFINITY));
        assert_eq!((-1.0_f32).to_decibels(), Err(Nan));

        assert_eq!(0.0_f32.to_decibels_checked(), Err(Infinite));
        assert_eq!((-1.0_f32).to_decibels_checked(), Err(Nan));
    }
}
