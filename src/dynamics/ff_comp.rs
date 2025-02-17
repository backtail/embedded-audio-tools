#[allow(unused_imports)]
use micromath::F32Ext;

use crate::envelope::ar::{ARPhase, AttackRelease};

/// Feed Forward Compressor with adjustable time slope parameters
#[repr(C)]
pub struct FFCompressor {
    /// between 0.0 and 1.0
    threshold: f32,
    /// between 1.0 and +inf
    ratio: f32,
    /// between 1.0 and upper bound
    makeup_gain: f32,

    env: AttackRelease,

    /// internal
    env_triggered: bool,
    cv: f32,
}

impl FFCompressor {
    pub fn new(threshold: f32, ratio: f32, makeup_gain: f32) -> Self {
        let mut comp = FFCompressor {
            threshold,
            ratio,
            makeup_gain,
            env: AttackRelease::new(0.0),
            env_triggered: false,
            cv: 1.0,
        };

        // envelope
        comp.env.set_level(ARPhase::ATTACK, 1.0);
        comp.env.set_level(ARPhase::RELEASE, 1.0);
        comp.env.reset(1.0);

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
        self.env.set_level(ARPhase::ATTACK, input);
        self.env.tick().clamp(1.0, f32::MAX)
    }

    pub fn set_attack(&mut self, val: f32, sr: f32) {
        self.env
            .set_time(ARPhase::ATTACK, val.clamp(f32::EPSILON, f32::MAX), sr);
    }

    pub fn set_release(&mut self, val: f32, sr: f32) {
        self.env
            .set_time(ARPhase::RELEASE, val.clamp(f32::EPSILON, f32::MAX), sr);
    }

    pub fn set_attack_slope(&mut self, val: f32) {
        self.env.set_slope(ARPhase::ATTACK, val.clamp(-10.0, 10.0));
    }

    pub fn set_release_slope(&mut self, val: f32) {
        self.env.set_slope(ARPhase::RELEASE, val.clamp(-10.0, 10.0));
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
        self.env.get_current_env_val()
    }

    pub fn get_current_threshold(&self) -> f32 {
        self.threshold
    }

    pub fn get_current_env_stage(&self) -> i8 {
        self.env.get_stage() as i8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn okay() {
        let mut comp = FFCompressor::new(0.2, 2.0, 1.0, 100.0);

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
