#[allow(unused_imports)]
use micromath::F32Ext;

const BIGGEST_SLOPE: f32 = 10.0;

const FIRST_STAGE: i8 = 0;
const ENV_IDLE: i8 = -1;
const ENV_HOLD: i8 = -2;

#[repr(C)]
pub struct MultiStageEnvelope<const STAGES: usize> {
    /// stage time in samples
    stage_time: [f32; STAGES],

    /// always between -10.0 and 10.0
    stage_slope: [f32; STAGES],

    /// always between 0.0 and 1.0
    stage_level: [f32; STAGES],

    /// always between 0.0 and 1.0
    stage_begin_level: f32,

    pub t: f32,
    envelope_value: f32,
    pub current_stage: i8,
    retrigger_stage: u8,
}

impl<const STAGES: usize> MultiStageEnvelope<STAGES> {
    pub fn new(init_val: f32) -> MultiStageEnvelope<STAGES> {
        assert!(STAGES > 1, "Must have at least 2 stages!");

        MultiStageEnvelope {
            stage_time: [0.0001; STAGES],
            stage_slope: [0.0; STAGES],
            stage_level: [0.0; STAGES],
            stage_begin_level: 0.0,

            t: 0.0,
            current_stage: ENV_IDLE,
            envelope_value: init_val.clamp(0.0, 1.0),
            retrigger_stage: FIRST_STAGE as u8,
        }
    }

    // ===================
    // PARAMETER INTERFACE
    // ===================

    /// Sets current_stage number `n` time, level at which it will end and its slope parameter.
    pub fn set_all(
        &mut self,
        n: usize,
        stage_time_in_secs: f32,
        stage_level: f32,
        stage_slope: f32,
        sr: f32,
    ) {
        debug_assert!(n < STAGES, "Stage {} doesn't exist!", n);

        self.stage_time[n] = (1.0 / (stage_time_in_secs * sr)).clamp(f32::EPSILON, f32::MAX);
        self.stage_level[n] = stage_level.clamp(0.0, 1.0);
        self.stage_slope[n] = stage_slope.clamp(-1.0 * BIGGEST_SLOPE, BIGGEST_SLOPE);
    }

    pub fn set_time(&mut self, n: usize, stage_time_in_secs: f32, sr: f32) {
        debug_assert!(n < STAGES, "Stage {} doesn't exist!", n);

        self.stage_time[n] = (1.0 / (stage_time_in_secs * sr)).clamp(f32::EPSILON, f32::MAX);
    }

    pub fn set_level(&mut self, n: usize, stage_level: f32) {
        debug_assert!(n < STAGES, "Stage {} doesn't exist!", n);

        self.stage_level[n] = stage_level.clamp(0.0, 1.0);
    }

    pub fn set_slope(&mut self, n: usize, stage_slope: f32) {
        debug_assert!(n < STAGES, "Stage {} doesn't exist!", n);

        self.stage_slope[n] = stage_slope.clamp(-1.0 * BIGGEST_SLOPE, BIGGEST_SLOPE);
    }

    /// index of stage starting at 0
    /// otherwise
    /// -1 is idle state
    /// -2 is hold state
    pub fn get_stage(&mut self) -> i8 {
        self.current_stage
    }

    pub fn set_retrigger_stage(&mut self, n: u8) {
        self.retrigger_stage = n;
    }

    // =============
    // ADSR USER API
    // =============

    /// to be called at a fixed rate
    pub fn tick(&mut self) -> f32 {
        if self.current_stage >= FIRST_STAGE {
            // tick away
            self.advance(self.current_stage as usize);

            // check if next stage is reached
            if self.t >= 1.0 {
                match self.current_stage {
                    val if val < STAGES as i8 - 2 => {
                        self.current_stage += 1; // next stage
                        self.stage_begin_level = self.envelope_value; // smooth operator
                        self.t = 0.0;
                    }
                    val if val == STAGES as i8 - 2 => self.current_stage = ENV_HOLD, // cannot be retriggered
                    val if val == STAGES as i8 - 1 => self.current_stage = ENV_IDLE, // can be (re)triggered
                    _ => panic!("Should not be reachable!"),
                }
            }
        }

        self.envelope_value
    }

    /// hard reset without smoothing
    pub fn reset(&mut self, val: f32) {
        self.current_stage = ENV_IDLE;
        self.envelope_value = val;
    }

    /// also smooth retriggers
    pub fn trigger(&mut self) {
        self.t = 0.0;
        self.stage_begin_level = self.envelope_value;
        self.current_stage = self.retrigger_stage as i8;
    }

    /// only ignored when already in last stage
    pub fn release(&mut self) {
        if self.current_stage < STAGES as i8 - 1 {
            self.t = 0.0;
            self.stage_begin_level = self.envelope_value;
            self.current_stage = STAGES as i8 - 1;
        }
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

    fn advance(&mut self, n: usize) {
        debug_assert!(
            n != ENV_IDLE as usize,
            "Tried to calculate envelope values when idle...",
        );

        debug_assert!(
            n != ENV_HOLD as usize,
            "Tried to calculate envelope values when holding...",
        );

        debug_assert!(
            n < STAGES,
            "Too many stages! Tried to calculate stage: {}",
            self.current_stage
        );

        let begin = self.stage_begin_level;
        let end = self.stage_level[n];
        let distance = end - begin;

        self.t += self.stage_time[n];

        self.envelope_value =
            distance * self.normalized_exp(self.t, distance.signum() * self.stage_slope[n]) + begin;
    }
}
