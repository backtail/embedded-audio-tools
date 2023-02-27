// Inspired by Ian Hobsen's ["irh <ian.r.hobson@gmail.com>"] freeverb for Rust
// https://github.com/irh/freeverb-rs/blob/b877287cfaced4c2872f126b0f0e595abb87dbd0/src/freeverb/src/comb.rs

use crate::delay_line::DelayLine;
use crate::memory::mut_mem_slice::MutMemSlice;

#[derive(Clone, Copy)]
pub struct Comb {
    pub delay_line: DelayLine,
    feedback: f32,
    filter_state: f32,
    dampening: f32,
    dampening_inverse: f32,
}

impl Comb {
    pub fn new(buffer: MutMemSlice) -> Self {
        Self {
            delay_line: DelayLine::new(buffer),
            feedback: 0.5,
            filter_state: 0.0,
            dampening: 0.5,
            dampening_inverse: 0.5,
        }
    }

    pub fn set_dampening(&mut self, value: f32) {
        self.dampening = value;
        self.dampening_inverse = 1.0 - value;
    }

    pub fn set_feedback(&mut self, value: f32) {
        self.feedback = value;
    }

    pub fn tick(&mut self, input: f32) -> f32 {
        let output = self.delay_line.read();

        self.filter_state = output * self.dampening_inverse + self.filter_state * self.dampening;

        self.delay_line
            .write_and_advance(input + self.filter_state * self.feedback);

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::mut_mem_slice::from_slice;

    #[test]
    fn basic_ticking() {
        let mut buffer = [0.0_f32; 2];
        let mut comb = Comb::new(from_slice(&mut buffer[..]));
        assert_eq!(comb.tick(1.0), 0.0);
        assert_eq!(comb.tick(0.0), 0.0);
        assert_eq!(comb.tick(0.0), 1.0);
        assert_eq!(comb.tick(0.0), 0.0);
        assert_eq!(comb.tick(0.0), 0.25);
        assert_eq!(comb.tick(0.0), 0.125);
        assert_eq!(comb.tick(0.0), 0.125);
        assert_eq!(comb.tick(0.0), 0.09375);
    }
}
