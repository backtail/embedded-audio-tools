#[allow(unused_imports)]
use micromath::F32Ext;

use crate::envelope::{ARPhase, FixedPointAttackRelease};

/// Feed Forward Compressor with adjustable time slope parameters
#[repr(C)]
pub struct FixedPointFFCompressor<const F_BITS: u8> {
    /// between 0.0 and 1.0
    threshold: f32,
    /// between 1.0 and +inf
    ratio: f32,
    /// between 1.0 and upper bound
    makeup_gain: f32,

    env: FixedPointAttackRelease<F_BITS>,

    /// internal
    cv: f32,
}

impl<const F_BITS: u8> FixedPointFFCompressor<F_BITS> {
    pub fn new() -> FixedPointFFCompressor<F_BITS> {
        let mut comp = FixedPointFFCompressor {
            threshold: 1.0,
            ratio: 1.0,
            makeup_gain: 1.0,
            env: FixedPointAttackRelease::<F_BITS>::new(),
            cv: 1.0,
        };

        // envelope
        comp.env.set_level(ARPhase::ATTACK, one(F_BITS));
        comp.env.set_level(ARPhase::RELEASE, one(F_BITS));
        comp.env.reset(one(F_BITS));

        comp
    }

    //
    // ---o------------------------------------------------------------------------> x --------->
    //    |                                                                          ^
    //    |                                                                          |
    //    |  |-----|  |----|     |---------------| -     |----------------| -     |-----|
    //    |->| ABS |->| dB |--o--| GAIN COMPUTER |-> + --| LEVEL DETECTOR |-> + --| LIN |
    //       |-----|  |----|  |  |---------------|   ^   |----------------|   ^   |-----|
    //                        |                      |                        |
    //                        |----------------------|                        |
    //                                                                   MAKEUP GAIN
    pub fn tick(&mut self, input: f32) -> f32 {
        let rectified = input.abs();
        let kneed = rectified / self.compute_gain(rectified);
        self.cv = self.makeup_gain / self.level_detect(kneed);

        input * self.cv
    }

    fn compute_gain(&mut self, input: f32) -> f32 {
        if input > self.threshold {
            // retrigger attack stage
            self.env.trigger();
            self.threshold + (input - self.threshold) / self.ratio
        } else {
            // retrigger release
            self.env.release();
            input
        }
    }

    fn level_detect(&mut self, input: f32) -> f32 {
        self.env
            .set_level(ARPhase::ATTACK, (input * one(F_BITS) as f32) as i32);
        self.env.tick().clamp(one(F_BITS), i32::MAX) as f32 / one(F_BITS) as f32
    }

    pub fn set_attack(&mut self, val: f32, sr: f32) {
        self.env.set_time(
            ARPhase::ATTACK,
            (val.clamp(f32::EPSILON, f32::MAX) * one(F_BITS) as f32) as i32,
            sr as i32,
        );
    }

    pub fn set_release(&mut self, val: f32, sr: f32) {
        self.env.set_time(
            ARPhase::RELEASE,
            (val.clamp(f32::EPSILON, f32::MAX) * one(F_BITS) as f32) as i32,
            sr as i32,
        );
    }

    pub fn set_attack_slope(&mut self, val: f32) {
        self.env.set_slope(
            ARPhase::ATTACK,
            (val.clamp(-10.0, 10.0) * one(F_BITS) as f32) as i32,
        );
    }

    pub fn set_release_slope(&mut self, val: f32) {
        self.env.set_slope(
            ARPhase::RELEASE,
            (val.clamp(-10.0, 10.0) * one(F_BITS) as f32) as i32,
        );
    }

    pub fn set_threshold(&mut self, val: f32) {
        self.threshold = val.clamp(f32::EPSILON, 1.0);
    }

    pub fn set_ratio(&mut self, val: f32) {
        self.ratio = val.clamp(1.0, f32::MAX);
    }

    pub fn set_makeup_gain(&mut self, val: f32) {
        self.makeup_gain = val.clamp(1.0, f32::MAX);
    }

    pub fn get_current_cv(&self) -> f32 {
        self.cv
    }

    pub fn get_current_env_val(&self) -> f32 {
        self.env.get_current_env_val() as f32 / one(F_BITS) as f32
    }

    pub fn get_current_threshold(&self) -> f32 {
        self.threshold
    }

    pub fn get_current_env_stage(&self) -> i8 {
        self.env.get_stage() as i8
    }
}

#[inline(always)]
const fn one(f_bits: u8) -> i32 {
    1 << f_bits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn okay() {
        let mut comp = FixedPointFFCompressor::new(0.2, 2.0, 1.0, 100.0);

        for i in 0..25 {
            let sample = if i < 5 || i > 15 { 0.0 } else { 1.0 };

            #[cfg(not(feature = "no_std"))]
            println!("out: {:.7}", comp.tick(sample));
        }

        for i in 0..25 {
            let sample = if i < 5 || i > 15 { 0.0 } else { 1.0 };

            #[cfg(not(feature = "no_std"))]
            println!("out: {:.7}", comp.tick(sample));
        }
    }
}
