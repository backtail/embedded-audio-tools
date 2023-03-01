/// Raw slice pointer that implements the `Send` trait since it's only acting on static memory
#[derive(Debug, PartialEq)]
pub enum InterpolationError {
    InputNaN,
    InputInfinite,
    InterpolationRange,
}

#[inline(always)]
pub fn lerp_unchecked(a: f32, b: f32, interpolate: f32) -> f32 {
    (a * (1.0 - interpolate)) + (b * interpolate)
}

pub fn lerp(a: f32, b: f32, interpolate: f32) -> Result<f32, InterpolationError> {
    if a.is_nan() || b.is_nan() {
        return Err(InterpolationError::InputNaN);
    }

    if a.is_infinite() || b.is_infinite() {
        return Err(InterpolationError::InputInfinite);
    }

    if interpolate < 0.0 || interpolate > 1.0 {
        return Err(InterpolationError::InterpolationRange);
    }

    Ok(lerp_unchecked(a, b, interpolate))
}

#[cfg(test)]
mod tests {
    use super::*;
    use InterpolationError::*;

    #[test]
    fn interpolate_unchecked() {
        assert_eq!(lerp_unchecked(0.0, 1.0, 0.5), 0.5);
    }

    #[test]
    fn interpolate_checked() {
        assert_eq!(lerp(f32::NAN, 0.0, 0.0), Err(InputNaN));
        assert_eq!(lerp(0.0, f32::NAN, 0.0), Err(InputNaN));
        assert_eq!(lerp(f32::INFINITY, 0.0, 0.0), Err(InputInfinite));
        assert_eq!(lerp(0.0, f32::INFINITY, 0.0), Err(InputInfinite));
        assert_eq!(lerp(0.0, 0.0, -1.0), Err(InterpolationRange));
        assert_eq!(lerp(0.0, 0.0, 2.0), Err(InterpolationRange));
        assert_eq!(lerp(0.0, 1.0, 0.5).unwrap(), 0.5);
    }
}
