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
