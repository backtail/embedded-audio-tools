pub const fn sine_table<const N: usize>() -> [i32; N] {
    let min_step = (u16::MAX / N as u16) as usize;
    let mut buffer = [0; N];

    let mut index = 0;

    while index < buffer.len() {
        let phase = (index as i32 * min_step as i32 - i16::MAX as i32) as i16;
        buffer[index] = unsafe { sin_i16_unchecked(phase, 4) as i32 };
        index += 1;
    }

    return buffer;
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
    res *= z;
    res >>= N + 1;

    res as i16
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn const_sine() {
        assert_eq!(sin_i16(0, 4), 0);
        assert_eq!(sin_i16(i16::MIN, 4), i16::MIN);
        assert_eq!(sin_i16(i16::MAX, 4), i16::MAX - 1);
    }

    #[test]
    fn sine_table_bounds_check() {
        const LENGTH: usize = i16::MAX as usize;
        let buffer = sine_table::<LENGTH>();
    }
}
