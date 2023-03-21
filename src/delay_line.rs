// Inspired by Ian Hobsen's ["irh <ian.r.hobson@gmail.com>"] freeverb for Rust
// https://github.com/irh/freeverb-rs/blob/b877287cfaced4c2872f126b0f0e595abb87dbd0/src/freeverb/src/delay_line.rs

use crate::memory::{memory_slice::MemorySlice, Mutable};

#[derive(Clone, Copy)]
pub struct DelayLine {
    buffer: MemorySlice<Mutable>,
    index: usize,
}

impl DelayLine {
    pub fn new(buffer: MemorySlice<Mutable>) -> Self {
        Self { buffer, index: 0 }
    }

    #[inline(always)]
    pub fn change_buffer(&mut self, new_slice: MemorySlice<Mutable>) {
        self.buffer = new_slice;
    }

    pub fn read(&self) -> f32 {
        unsafe { self.buffer.get_unchecked(self.index) }
    }

    pub fn read_wrapped_at(&self, offset: isize) -> f32 {
        self.buffer.get_wrapped(self.index as isize + offset)
    }

    pub fn read_lerp_wrapped_at(&self, offset: f32) -> f32 {
        self.buffer.lerp_wrapped(self.index as f32 + offset)
    }

    pub fn read_lagrange_wrapped_at(&self, offset: f32) -> f32 {
        self.buffer.lagrange_wrapped(self.index as f32 + offset, 5)
    }

    pub fn write_and_advance(&mut self, value: f32) {
        unsafe {
            self.buffer.assign_unchecked(self.index, value);
        }

        if self.index == self.buffer.length - 1 {
            self.index = 0;
        } else {
            self.index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::memory_slice::from_slice_mut;

    #[test]
    fn write_and_advance() {
        let mut buffer = [0_f32; 24];

        let mut delay_line = DelayLine::new(from_slice_mut(&mut buffer[..]));

        for (i, val) in buffer.iter().enumerate() {
            delay_line.write_and_advance(i as f32);
            assert_eq!(*val, i as f32);
        }
    }

    #[test]
    fn read() {
        let mut buffer = [0_f32; 24];
        for (i, val) in buffer.iter_mut().enumerate() {
            *val = i as f32;
        }

        let mut delay_line = DelayLine::new(from_slice_mut(&mut buffer[..]));

        for val in buffer {
            assert_eq!(val, delay_line.read());
            delay_line.index += 1;
        }
    }
}
