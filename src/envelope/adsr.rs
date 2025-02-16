use EnvelopeState::*;

#[allow(unused_imports)]
use micromath::F32Ext;

const BIGGEST_SLOPE: f32 = 10.0;

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(C)]
pub enum EnvelopeState {
    Idle = -1, // Default
    Stage0 = 0,
    Stage1 = 1,
    Stage2 = 2,
}

#[repr(C)]
pub struct AudioRateADSR {
    /// stage time in samples
    stage_time: [f32; 3],

    /// always between 0.0 and 1.0
    stage_level: [f32; 3],

    /// always between -10.0 and 10.0
    stage_slope: [f32; 3],

    /// always between 0.0 and 1.0
    init_level: f32,

    /// internal timing
    t: f32,

    // current output value
    envelope_value: f32,

    /// sample rate
    sr: f32,

    /// state machine
    state: EnvelopeState,
}

impl AudioRateADSR {
    pub fn new(sr: f32) -> AudioRateADSR {
        AudioRateADSR {
            stage_time: [0.001; 3],
            stage_level: [0.0; 3],
            stage_slope: [0.0; 3],
            init_level: 0.0,
            sr,
            t: 0.0,
            state: Idle,
            envelope_value: 0.0,
        }
    }

    // ===================
    // PARAMETER INTERFACE
    // ===================

    /// Clamps between 0.0 and 1.0
    pub fn set_init_level(&mut self, val: f32) {
        self.init_level = val.clamp(0.0, 1.0);
    }

    /// Sets stage number `n` time, level at which it will end and its slope parameter.
    pub fn set_stage(&mut self, n: usize, stage_in_secs: f32, stage_level: f32, stage_slope: f32) {
        self.stage_time[n] = (1.0 / (stage_in_secs * self.sr)).clamp(f32::EPSILON, f32::MAX);
        self.stage_level[n] = stage_level.clamp(0.0, 1.0);
        self.stage_slope[n] = stage_slope.clamp(-1.0 * BIGGEST_SLOPE, BIGGEST_SLOPE);
    }

    /// Rate at which `tick()` is called
    pub fn set_sr(&mut self, sr: f32) {
        for n in 0..3 {
            self.set_stage(
                n,
                self.stage_time[n] / sr,
                self.stage_level[n],
                self.stage_slope[n],
            );
        }

        self.sr = sr;
    }

    pub fn get_state(&mut self) -> EnvelopeState {
        self.state
    }

    // =============
    // ADSR USER API
    // =============

    pub fn tick(&mut self) -> f32 {
        match self.state {
            Idle => 0.0,
            Stage0 => self.tick_stage0(),
            Stage1 => self.tick_stage1(),
            Stage2 => self.tick_stage2(),
        }
    }

    pub fn reset(&mut self) {
        self.envelope_value = 0.0;
        self.state = Idle;
    }

    pub fn trigger_on(&mut self) {
        match self.state {
            Idle | Stage0 => {
                self.state = Stage0;
                self.t = 0.0;
            }

            // Retrigger
            Stage1 | Stage2 => self.state = Stage0,
        }
    }

    pub fn trigger_off(&mut self) {
        match self.state {
            Stage0 | Stage1 => {
                self.state = Stage2;
                self.t = 0.0;
            }
            _ => {}
        }
    }

    // =================
    // PRIVATE FUNCTIONS
    // =================

    fn normalized_exp(&mut self, val: f32, slope: f32) -> f32 {
        // makes calculation more numerically stable
        if slope > 0.1 || slope < -0.1 {
            ((slope * val).exp() - 1.0) / (slope.exp() - 1.0)
        } else {
            // divide by zero result
            val
        }
    }

    fn tick_stage0(&mut self) -> f32 {
        self.t += self.stage_time[0];
        self.envelope_value = (self.stage_level[0] - self.init_level)
            * self.normalized_exp(self.t, self.stage_slope[0])
            + self.init_level;

        if (self.envelope_value - self.stage_level[0]).abs() <= 100.0 * f32::EPSILON
            || self.envelope_value > 1.0
            || self.envelope_value < 0.0
        {
            self.state = Stage1;
            self.envelope_value = self.stage_level[0];
            self.t = 0.0;
        }

        return self.envelope_value;
    }

    fn tick_stage1(&mut self) -> f32 {
        self.t += self.stage_time[1];
        self.envelope_value = (self.stage_level[0] - self.stage_level[1])
            * (1.0 - self.normalized_exp(self.t, self.stage_slope[1]))
            + self.stage_level[1];

        if (self.envelope_value - self.stage_level[1]).abs() <= 100.0 * f32::EPSILON
            || self.envelope_value > 1.0
            || self.envelope_value < 0.0
        {
            self.state = Stage2;
            self.envelope_value = self.stage_level[1];
            self.t = 0.0;
        }

        return self.envelope_value;
    }

    fn tick_stage2(&mut self) -> f32 {
        self.t += self.stage_time[2];
        self.envelope_value = (self.stage_level[1] - self.stage_level[2])
            * (1.0 - self.normalized_exp(self.t, self.stage_slope[2]))
            + self.stage_level[2];

        if (self.envelope_value - self.stage_level[2]).abs() <= 100.0 * f32::EPSILON
            || self.envelope_value > 1.0
            || self.envelope_value < 0.0
        {
            self.state = Idle;
            self.envelope_value = self.stage_level[2];
            self.t = 0.0;
        }

        return self.envelope_value;
    }
}
