// Inspired by Ian Hobsen's ["irh <ian.r.hobson@gmail.com>"] freeverb for Rust
// https://github.com/irh/freeverb-rs/blob/b877287cfaced4c2872f126b0f0e595abb87dbd0/src/freeverb/src/all_pass.rs

use crate::delay_line::DelayLine;
use crate::memory::mut_mem_slice::MutMemSlice;

#[derive(Clone, Copy)]
pub struct AllPass {
    pub delay_line: DelayLine,
}

impl AllPass {
    pub fn new(buffer: MutMemSlice) -> Self {
        Self {
            delay_line: DelayLine::new(buffer),
        }
    }

    pub fn tick(&mut self, input: f32) -> f32 {
        let delayed = self.delay_line.read();
        let output = -input + delayed;

        let feedback = 0.5;

        self.delay_line
            .write_and_advance(input + delayed * feedback);

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mut_mem_slice::from_slice;

    #[test]
    fn basic_ticking() {
        let mut buffer = [0.0_f32; 2];
        let mut allpass = AllPass::new(from_slice(&mut buffer[..]));
        assert_eq!(allpass.tick(1.0), -1.0);
        assert_eq!(allpass.tick(0.0), 0.0);
        assert_eq!(allpass.tick(0.0), 1.0);
        assert_eq!(allpass.tick(0.0), 0.0);
        assert_eq!(allpass.tick(0.0), 0.5);
        assert_eq!(allpass.tick(0.0), 0.0);
        assert_eq!(allpass.tick(0.0), 0.25);
    }
}
