use PanningError::*;

#[allow(unused_imports)]
use micromath::F32Ext;

#[derive(Debug, PartialEq)]
pub enum PanningError {
    TooLeft,
    TooRight,
}

#[inline(always)]
fn check_pan_error(amount: f32) -> Result<f32, PanningError> {
    if amount < -1.0 {
        return Err(TooLeft);
    }

    if amount > 1.0 {
        return Err(TooRight);
    }

    Ok(amount)
}

// =======
// CHECKED
// =======

pub fn stereo_pan(amount: f32, samples: (f32, f32)) -> Result<(f32, f32), PanningError> {
    Ok(stereo_pan_unchecked(check_pan_error(amount)?, samples))
}

pub fn mono_pan(amount: f32, sample: f32) -> Result<(f32, f32), PanningError> {
    Ok(mono_pan_unchecked(check_pan_error(amount)?, sample))
}

// =========
// UNCHECKED
// =========

#[inline(always)]
pub fn stereo_pan_unchecked(amount: f32, samples: (f32, f32)) -> (f32, f32) {
    let pan = equal_power_pan_unchecked(amount);
    (samples.0 * pan.0, samples.1 * pan.1)
}

#[inline(always)]
pub fn mono_pan_unchecked(amount: f32, sample: f32) -> (f32, f32) {
    let pan = equal_amplitude_pan_unchecked(amount);
    (sample * pan.0, sample * pan.1)
}

#[inline(always)]
fn equal_amplitude_pan_unchecked(amount: f32) -> (f32, f32) {
    ((1.0 - amount) * 0.5, (1.0 + amount) * 0.5)
}

#[inline(always)]
fn equal_power_pan_unchecked(amount: f32) -> (f32, f32) {
    let linear = equal_amplitude_pan_unchecked(amount);
    (linear.0.sqrt(), linear.1.sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_panning() {
        assert_eq!(equal_amplitude_pan_unchecked(-1.0), (1.0, 0.0));
        assert_eq!(equal_amplitude_pan_unchecked(0.0), (0.5, 0.5)); // -6dB
        assert_eq!(equal_amplitude_pan_unchecked(1.0), (0.0, 1.0));
    }

    #[test]
    fn squared_panning() {
        assert_eq!(equal_power_pan_unchecked(-1.0), (1.0, 0.0));
        assert_eq!(equal_power_pan_unchecked(0.0), (0.70710677, 0.70710677)); // roughly -3dB
        assert_eq!(equal_power_pan_unchecked(1.0), (0.0, 1.0));
    }

    #[test]
    fn pan_error() {
        assert_eq!(mono_pan(-5.0, 1.0), Err(TooLeft));
        assert_eq!(mono_pan(5.0, 1.0), Err(TooRight));

        assert_eq!(stereo_pan(-5.0, (1.0, 1.0)), Err(TooLeft));
        assert_eq!(stereo_pan(5.0, (1.0, 1.0)), Err(TooRight));
    }
}
