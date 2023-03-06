use tock_registers::register_bitfields;

register_bitfields! {
        u32,
        F32 [
                SIGN  OFFSET(31) NUMBITS(1)  [],
                EXP   OFFSET(23) NUMBITS(8)  [],
                MANT  OFFSET(0)  NUMBITS(23) [],
            ],
}

#[derive(Debug, PartialEq)]
pub struct F32Components {
    pub sign: bool,
    pub exponent: u8,
    pub mantissa: u32,
}

// =================
// CONVERSION TRAITS
// =================

pub trait ToRaw {
    fn to_raw_word(&self) -> u32;
}

pub trait FromRaw {
    type Output;
    fn from_raw_word(val: u32) -> Self::Output;
}

pub trait ToF32Components {
    fn to_f32_components(&self) -> F32Components;
}

pub trait FromF32Components {
    type Output;
    fn from_f32_components(components: F32Components) -> Self::Output;
}

// ==========================
// CONVERSION IMPLEMENTATIONS
// ==========================

impl ToRaw for f32 {
    #[inline(always)]
    fn to_raw_word(&self) -> u32 {
        let mut full_integer = 0_u32;

        for (i, byte) in self.to_le_bytes().iter().enumerate() {
            full_integer += (*byte as u32) << (8 * i);
        }

        full_integer
    }
}

impl FromRaw for f32 {
    type Output = f32;
    fn from_raw_word(val: u32) -> Self::Output {
        f32::from_le_bytes(val.to_le_bytes())
    }
}

impl ToF32Components for u32 {
    #[inline(always)]
    fn to_f32_components(&self) -> F32Components {
        F32Components {
            sign: ((self >> F32::SIGN.shift) & F32::SIGN.mask) != 0,
            exponent: ((self >> F32::EXP.shift) & F32::EXP.mask) as u8,
            mantissa: (self >> F32::MANT.shift) & F32::MANT.mask,
        }
    }
}

impl ToF32Components for f32 {
    #[inline(always)]
    fn to_f32_components(&self) -> F32Components {
        self.to_raw_word().to_f32_components()
    }
}

impl FromF32Components for u32 {
    type Output = u32;
    #[inline(always)]
    fn from_f32_components(components: F32Components) -> u32 {
        ((components.sign as u32) << F32::SIGN.shift)
            + ((components.exponent as u32) << F32::EXP.shift)
            + ((components.mantissa as u32) << F32::MANT.shift)
    }
}

impl FromF32Components for f32 {
    type Output = f32;
    #[inline(always)]
    fn from_f32_components(components: F32Components) -> f32 {
        f32::from_raw_word(u32::from_f32_components(components))
    }
}

// ================
// CONVERSION TESTS
// ================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion() {
        use core::f32::consts::PI;
        const RAW_PI: u32 = 0x40490fdb;
        const COMPONENTS_OF_PI: F32Components = F32Components {
            sign: false,
            exponent: 128,
            mantissa: 4788187,
        };

        assert_eq!(PI.to_raw_word(), RAW_PI);
        assert_eq!(f32::from_raw_word(RAW_PI), PI);
        assert_eq!(PI.to_f32_components(), COMPONENTS_OF_PI);
        assert_eq!(f32::from_f32_components(COMPONENTS_OF_PI), PI);
    }
}
