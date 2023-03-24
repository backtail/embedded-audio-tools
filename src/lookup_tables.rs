use crate::{lookup::BANDLIMITED_RECT, memory::memory_slice::from_slice};
use core::f32::consts::{FRAC_1_PI, FRAC_PI_4, PI};

#[allow(unused_imports)]
use micromath::F32Ext;

use crate::{float::math::MissingF32Ext, interpolation::lerp_unchecked};

pub const fn sine_table<const N: usize>() -> [i16; N] {
    let min_step = (u16::MAX / N as u16) as usize;
    let mut buffer = [0; N];

    let mut index = 0;

    while index < buffer.len() {
        let phase = (index as i32 * min_step as i32 - i16::MAX as i32) as i16;
        buffer[index] = unsafe { sin_i16_unchecked(phase, 4) };
        index += 1;
    }

    return buffer;
}

/// Fixed point approximation of the sine function
pub fn fixed_point_sin(val: f32) -> f32 {
    let normalized_rads = ((val / 2.0 - FRAC_PI_4).modulus(PI) / FRAC_PI_4) - 2.0;
    let quadrant_rads = normalized_rads.abs() - 1.0;

    unsafe {
        sin_i16_unchecked((i16::MAX as f32 * quadrant_rads) as i16, 4) as f32 / i16::MAX as f32
    }
}

/// Accepts values between 0 and 1, otherwise clamps at boundery
///
/// Bandlimited rectable function
pub fn lookup_rect(val: f32) -> f32 {
    let buffer = from_slice(&BANDLIMITED_RECT[..]);
    let len = buffer.len() - 1;

    if val <= 0.0 {
        return 0.0;
    }

    if val <= 0.25 {
        let f_index = val * 4.0;
        return unsafe { buffer.lerp_unchecked(f_index * len as f32) };
    }

    if val <= 0.5 {
        let f_index = (0.25 - (val - 0.25)) * 4.0;
        return unsafe { buffer.lerp_unchecked(f_index * len as f32) };
    }

    if val <= 0.75 {
        let f_index = (val - 0.5) * 4.0;
        return unsafe { -1.0 * buffer.lerp_unchecked(f_index * len as f32) };
    }

    if val < 1.0 {
        let f_index = (0.25 - (val - 0.75)) * 4.0;
        return unsafe { -1.0 * buffer.lerp_unchecked(f_index * len as f32) };
    }

    return 0.0;
}

/// Interpolated fixed point approximation lookup of the sine function
///
/// Not accurate at all in moment!
pub fn lookup_sin(val: f32) -> f32 {
    const SINE_LOOKUP: [i16; 4096] = sine_table::<4096>();

    let normalized_rads = ((val / 2.0 - FRAC_PI_4).modulus(PI) / FRAC_PI_4) - 2.0;
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

/// ### Fixed point taylor series expansion of the sine function
///
/// `phase` is the full `i16` range
/// which corrensponds to sin(x) where x is from `-π/2` to `π/2`
///
/// `degree` of the polynomial approximation. **Highest: 4**
///
/// Returns the full `i16` range
pub const fn sin_i16(phase: i16, degree: u8) -> i16 {
    if degree > 4 {
        unsafe { sin_i16_unchecked(phase, 4) }
    } else {
        unsafe { sin_i16_unchecked(phase, degree) }
    }
}

/// ### Fixed point taylor series expansion of the sine function
///
/// `phase` is the full `i16` range
/// which corrensponds to sin(x) where x is from `-π/2` to `π/2`
///
/// `degree` of the polynomial approximation. **Highest: 4**
///
/// Returns the full `i16` range
///
/// # Safety
///
/// Panics if a degree higher than 4 is being passed into function!
pub const unsafe fn sin_i16_unchecked(phase: i16, degree: u8) -> i16 {
    // Polynomial constants
    const B: [i32; 5] = [102_944, -42_334, 5223, -307, 10];
    const N: u32 = 15;

    let z = phase as i32;
    let mut d = degree as usize;

    let mut res: i32 = 0;

    while d != 0 {
        res += B[d as usize];
        res *= z;
        res >>= N;
        res *= z;
        res >>= N;

        d -= 1;
    }

    res += B[0];
    res = res.saturating_mul(z);
    res >>= N + 1;

    res as i16
}

pub fn simpsons_rule<const N: usize>(f: fn(f32) -> f32, a: f32, b: f32) -> f32 {
    let h = (b - a) / (N as f32);
    let mut x = [0.0; N];
    for i in 0..N {
        x[i] = a + (i as f32) * h;
    }
    let mut y = [0.0; N];
    for i in 0..N {
        y[i] = f(x[i]);
    }
    let integral = h / 3.0
        * (f(a)
            + 4.0 * y[1..N].iter().step_by(2).sum::<f32>()
            + 2.0 * y[2..N - 1].iter().step_by(2).sum::<f32>()
            + 4.0 * y[1..].iter().step_by(2).sum::<f32>()
            + if N % 2 == 0 { f(b) } else { 0.0 });
    integral
}

pub fn si(x: f32) -> f32 {
    let f = |t: f32| -> f32 {
        if t == 0.0 {
            1.0
        } else {
            ((t * PI).sin() / t) * FRAC_1_PI
        }
    };
    simpsons_rule::<1000>(f, 0.0, x)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn const_sine_bounds() {
        assert_eq!(sin_i16(0, 4), 0);
        assert_eq!(sin_i16(i16::MIN, 4), i16::MIN);
        assert_eq!(sin_i16(i16::MAX, 4), i16::MAX - 1);
    }

    #[test]
    fn fixed_point_sin_bounds() {
        assert_eq!(fixed_point_sin(0.0), 0.0);
        assert_eq!(fixed_point_sin(core::f32::consts::PI), 0.0);
        assert_eq!(fixed_point_sin(core::f32::consts::TAU), 0.0);
        assert_eq!(fixed_point_sin(-core::f32::consts::PI), 0.0);
        assert_eq!(fixed_point_sin(-core::f32::consts::TAU), 0.0);
    }

    #[test]
    fn lookup_rect_bounds() {
        assert_eq!(lookup_rect(0.0), 0.0);
        assert_eq!(lookup_rect(1.0), 0.0);
    }
}
