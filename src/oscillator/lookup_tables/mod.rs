pub mod bl_rect;

use crate::fixed_point::math::sin_i16_unchecked;

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
