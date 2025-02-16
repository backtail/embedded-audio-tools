#[allow(unused_imports)]
use micromath::F32Ext;

use crate::envelope::MultiStageEnvelope;

const ATTACK_PHASE: u8 = 0;
const RELEASE_PHASE: u8 = 1;

/// Feed Forward Compressor with adjustable time slope parameters
#[repr(C)]
pub struct FFCompressor {
    /// between 0.0 and 1.0
    threshold: f32,
    /// between 1.0 and +inf
    ratio: f32,
    /// between 1.0 and upper bound
    makeup_gain: f32,

    env: MultiStageEnvelope<3>,

    /// internal
    env_triggered: bool,
}

impl FFCompressor {
    pub fn new(threshold: f32, ratio: f32, makeup_gain: f32, sr: f32) -> Self {
        let mut comp = FFCompressor {
            threshold,
            ratio,
            makeup_gain,
            env: MultiStageEnvelope::new(0.0),
            env_triggered: false,
        };

        comp.env
            .set_all(ATTACK_PHASE as usize, 0.20, 0.0, -10.0, sr);
        comp.env
            .set_all(RELEASE_PHASE as usize, 0.20, 0.0, 10.0, sr);
        comp.env.set_all(2, 0.001, 0.0, 0.0, sr);

        comp
    }

    fn trigger(&mut self) {
        if !self.env_triggered {
            self.env.set_retrigger_stage(ATTACK_PHASE);
            self.env.trigger();
            self.env_triggered = true;
        }
    }

    fn release(&mut self) {
        if self.env_triggered {
            self.env.set_retrigger_stage(RELEASE_PHASE);
            self.env.trigger();
            self.env_triggered = false;
        }
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
        let input_level = input.abs();

        // print!("abs: {:.7}, ", input_level);

        let gain_computer = if input_level > self.threshold {
            self.trigger();
            self.threshold + (input_level - self.threshold) / (self.ratio)
        } else {
            self.release();
            input_level
        };

        // print!("new: {:.7}, ", gain_computer);

        let level_detector_in = input_level / gain_computer;
        self.env
            .set_level(ATTACK_PHASE as usize, level_detector_in - 1.0);

        // print!("before_detector: {:.7}, ", level_detector_in);

        // print!("stage: {:.7}, ", self.env.get_stage());

        let level_detector_out = self.env.tick() + 1.0;

        // print!("env: {:.7}, ", level_detector_out);

        let control_voltage = self.makeup_gain / level_detector_out;

        // print!("cv: {:.7}, ", control_voltage);

        input * control_voltage
    }

    pub fn set_attack(&mut self, val: f32, sr: f32) {
        self.env
            .set_time(ATTACK_PHASE as usize, val.clamp(f32::EPSILON, f32::MAX), sr);
    }

    pub fn set_release(&mut self, val: f32, sr: f32) {
        self.env.set_time(
            RELEASE_PHASE as usize,
            val.clamp(f32::EPSILON, f32::MAX),
            sr,
        );
    }

    pub fn set_attack_slope(&mut self, val: f32) {
        self.env
            .set_slope(ATTACK_PHASE as usize, val.clamp(-10.0, 10.0));
    }

    pub fn set_release_slope(&mut self, val: f32) {
        self.env
            .set_slope(RELEASE_PHASE as usize, val.clamp(-10.0, 10.0));
    }

    pub fn set_threshold(&mut self, val: f32) {
        self.threshold = val.clamp(f32::EPSILON, 1.0);
    }

    pub fn set_ratio(&mut self, val: f32) {
        self.ratio = val.clamp(1.0, 1000.0);
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
