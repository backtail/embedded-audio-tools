#[allow(unused_imports)]
use micromath::F32Ext;

const BIGGEST_SLOPE: f32 = 10.0;
const N_STAGES: usize = 2;

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(C)]
pub enum ARPhase {
    HOLD = -1,
    ATTACK = 0,
    RELEASE = 1,
}

#[repr(C)]
pub struct AttackRelease {
    /// stage time in samples
    stage_time: [f32; 2],

    /// always between -10.0 and 10.0
    stage_slope: [f32; 2],

    /// always between 0.0 and 1.0
    stage_level: [f32; 2],

    /// always between 0.0 and 1.0
    stage_begin_level: f32,

    t: f32,
    envelope_value: f32,
    current_stage: ARPhase,
}

impl AttackRelease {
    pub fn new(init_val: f32) -> AttackRelease {
        AttackRelease {
            stage_time: [0.0001; N_STAGES],
            stage_slope: [0.0; N_STAGES],
            stage_level: [0.0; N_STAGES],
            stage_begin_level: 0.0,

            t: 0.0,
            current_stage: ARPhase::HOLD,
            envelope_value: init_val,
        }
    }

    // ===================
    // PARAMETER INTERFACE
    // ===================

    /// Sets current_stage number `n` time, level at which it will end and its slope parameter.
    pub fn set_all(
        &mut self,
        phase: ARPhase,
        stage_time_in_secs: f32,
        stage_level: f32,
        stage_slope: f32,
        sr: f32,
    ) {
        debug_assert_ne!(phase, ARPhase::HOLD);

        self.stage_time[phase as usize] =
            (1.0 / (stage_time_in_secs * sr)).clamp(f32::EPSILON, f32::MAX);
        self.stage_level[phase as usize] = stage_level;
        self.stage_slope[phase as usize] = stage_slope.clamp(-1.0 * BIGGEST_SLOPE, BIGGEST_SLOPE);
    }

    pub fn set_time(&mut self, phase: ARPhase, stage_time_in_secs: f32, sr: f32) {
        debug_assert_ne!(phase, ARPhase::HOLD);

        self.stage_time[phase as usize] =
            (1.0 / (stage_time_in_secs * sr)).clamp(f32::EPSILON, f32::MAX);
    }

    pub fn set_level(&mut self, phase: ARPhase, stage_level: f32) {
        debug_assert_ne!(phase, ARPhase::HOLD);

        self.stage_level[phase as usize] = stage_level;
    }

    pub fn set_slope(&mut self, phase: ARPhase, stage_slope: f32) {
        debug_assert_ne!(phase, ARPhase::HOLD);

        self.stage_slope[phase as usize] = stage_slope.clamp(-1.0 * BIGGEST_SLOPE, BIGGEST_SLOPE);
    }

    pub fn set_current_env_val(&mut self, val: f32) {
        self.envelope_value = val;
    }

    pub fn set_current_stage(&mut self, phase: ARPhase) {
        self.current_stage = phase;
    }

    pub fn get_stage(&self) -> ARPhase {
        self.current_stage
    }

    pub fn get_current_env_val(&self) -> f32 {
        self.envelope_value
    }

    // =============
    // AR USER API
    // =============

    /// to be called at a fixed rate
    pub fn tick(&mut self) -> f32 {
        match self.current_stage {
            ARPhase::ATTACK | ARPhase::RELEASE => {
                self.advance(self.current_stage);
            }
            _ => {}
        }

        self.envelope_value
    }

    /// hard reset without smoothing
    pub fn reset(&mut self, val: f32) {
        self.t = 0.0;
        self.current_stage = ARPhase::HOLD;
        self.stage_begin_level = val;
    }

    /// also smooth retriggers
    pub fn trigger(&mut self) {
        self.t = 0.0;
        self.stage_begin_level = self.envelope_value;
        self.current_stage = ARPhase::ATTACK;
    }

    /// only ignored when already in last stage
    pub fn release(&mut self) {
        self.t = 0.0;
        self.stage_begin_level = self.envelope_value;
        self.current_stage = ARPhase::RELEASE;
    }

    // =================
    // PRIVATE FUNCTIONS
    // =================

    fn normalized_exp(&mut self, val: f32, slope: f32) -> f32 {
        // makes calculation more numerically stable
        if slope > 0.01 || slope < -0.01 {
            ((slope * val).exp() - 1.0) / (slope.exp() - 1.0)
        } else {
            // divide by zero result
            val
        }
    }

    fn advance(&mut self, phase: ARPhase) {
        debug_assert_ne!(phase, ARPhase::HOLD);

        let begin = self.stage_begin_level;
        let end = self.stage_level[phase as usize];
        let distance = end - begin;

        self.t += self.stage_time[phase as usize];

        self.envelope_value = distance
            * self.normalized_exp(self.t, distance.signum() * self.stage_slope[phase as usize])
            + begin;
    }
}
