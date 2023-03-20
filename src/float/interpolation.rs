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

/// Comuptes the lagrange interpolation on the whole set of data points provided.
pub fn lagrange(array: &[f32], x_point: f32) -> f32 {
    assert!(x_point <= (array.len() - 1) as f32);

    let mut y_point = 0.0_f32;
    for i in 0..array.len() {
        let mut term = array[i];
        for j in 0..array.len() {
            if i != j {
                term = (term * (x_point - j as f32)) / (i as f32 - j as f32);
            }
        }
        y_point += term;
    }

    return y_point;
}

#[inline(always)]
pub unsafe fn lagrange_only_4_elements(array: &[f32], x_point: f32) -> f32 {
    let mut y_point = 0.0_f32;

    let mut term = unsafe { *array.get_unchecked(0) };
    term = (term * (x_point - 1.0)) / -1.0;
    term = (term * (x_point - 2.0)) / -2.0;
    term = (term * (x_point - 3.0)) / -3.0;

    y_point += term;

    let mut term = unsafe { *array.get_unchecked(1) };
    term = (term * (x_point - 0.0)) / 1.0;
    term = (term * (x_point - 2.0)) / -1.0;
    term = (term * (x_point - 3.0)) / -2.0;

    y_point += term;

    let mut term = unsafe { *array.get_unchecked(2) };
    term = (term * (x_point - 0.0)) / 2.0;
    term = (term * (x_point - 1.0)) / 1.0;
    term = (term * (x_point - 3.0)) / -1.0;

    y_point += term;

    let mut term = unsafe { *array.get_unchecked(3) };
    term = (term * (x_point - 0.0)) / 3.0;
    term = (term * (x_point - 1.0)) / 2.0;
    term = (term * (x_point - 2.0)) / 1.0;

    y_point += term;

    return y_point;
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
