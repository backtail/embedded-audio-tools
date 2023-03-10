use EnvelopeState::*;

#[allow(unused_imports)]
use micromath::F32Ext;

const SHORTEST_TIME_BASE: f32 = 0.5;
const BIGGEST_SLOPE: f32 = 20.0;

#[derive(Debug, PartialEq)]
enum EnvelopeState {
    Idle,
    Attack,
    Decay,
    Release,
    Sustain,
}

pub struct AudioRateADSR {
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
    slope: f32,

    t: f32,
    state: EnvelopeState,

    envelope_value: f32,
    release_val: f32,

    sr: f32,
}

impl AudioRateADSR {
    pub fn new(
        attack_in_secs: f32,
        decay_in_secs: f32,
        sustain: f32,
        release_in_secs: f32,
        slope: f32,
        sr: f32,
    ) -> AudioRateADSR {
        AudioRateADSR {
            sustain: sustain.clamp(0.0, 1.0),
            decay: set_time_parameter(decay_in_secs, sr),
            attack: set_time_parameter(attack_in_secs, sr),
            release: set_time_parameter(release_in_secs, sr),

            sr,
            slope,
            t: 0.0,
            state: Idle,
            release_val: 0.0,
            envelope_value: 0.0,
        }
    }

    // ===================
    // PARAMETER INTERFACE
    // ===================

    pub fn set_attack(&mut self, attack_in_secs: f32) {
        self.attack = set_time_parameter(attack_in_secs, self.sr);
    }

    pub fn set_decay(&mut self, decay_in_secs: f32) {
        self.decay = set_time_parameter(decay_in_secs, self.sr);
    }

    pub fn set_sustain(&mut self, sustain: f32) {
        self.sustain = sustain.clamp(0.0, 1.0);
    }

    pub fn set_release(&mut self, release_in_secs: f32) {
        self.release = set_time_parameter(release_in_secs, self.sr);
    }

    pub fn set_slope(&mut self, slope: f32) {
        self.slope = slope.clamp(1.0 / BIGGEST_SLOPE, BIGGEST_SLOPE);
    }

    pub fn set_sr(&mut self, sr: f32) {
        self.sr = sr;
    }

    // =============
    // ADSR USER API
    // =============

    pub fn tick(&mut self) -> f32 {
        match self.state {
            Idle => 0.0,
            Sustain => self.sustain,
            Attack => self.next_attack(),
            Decay => self.next_decay(),
            Release => self.next_release(),
        }
    }

    pub fn trigger_on(&mut self) {
        match self.state {
            Idle | Attack => {
                self.state = Attack;
                self.t = 0.0;
            }

            // Retrigger
            Decay | Sustain | Release => self.state = Attack,
        }
    }

    pub fn trigger_off(&mut self) {
        match self.state {
            Attack | Decay | Sustain => {
                self.state = Release;
                self.release_val = self.envelope_value;
                self.t = 0.0;
            }
            _ => {}
        }
    }

    // =================
    // PRIVATE FUNCTIONS
    // =================

    fn next_attack(&mut self) -> f32 {
        self.t += self.attack;
        self.envelope_value = self.t.powf(1.0 / self.slope);

        if self.envelope_value >= 1.0 {
            self.state = Decay;
            self.envelope_value = 1.0;
            self.t = 0.0;
        }

        return self.envelope_value;
    }

    fn next_decay(&mut self) -> f32 {
        self.t += self.decay;
        self.envelope_value =
            ((1.0 - self.t.powf(self.slope)) * (1.0 - self.sustain)) + self.sustain;

        if self.envelope_value <= self.sustain {
            self.state = Sustain;
            self.envelope_value = self.sustain;
        }

        return self.envelope_value;
    }

    fn next_release(&mut self) -> f32 {
        self.t += self.release;
        self.envelope_value = (1.0 - self.t.powf(self.slope)) * self.release_val;

        // reach idle at ca. -140dB
        if self.envelope_value <= f32::EPSILON {
            self.state = Idle;
            self.envelope_value = 0.0;
            self.t = 0.0;
        }

        return self.envelope_value;
    }
}

fn set_time_parameter(parameter_in_secs: f32, sr: f32) -> f32 {
    (1.0 / (parameter_in_secs * sr)).clamp(SHORTEST_TIME_BASE / sr, f32::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_adsr() {
        let time = 0.003; // 3ms
        let sr: f32 = 48_000.0;
        let parameter_len = (time * sr) as usize;
        let slope = 0.5;
        let mut adsr = AudioRateADSR::new(time, time, 0.8, time, slope, sr);

        // ==========
        // IDLE STATE
        // ==========

        // Without a gate trigger_on nothing happens
        assert_eq!(adsr.tick(), 0.0);
        assert_eq!(adsr.state, Idle);

        // ====================
        // TRANSITION TO ATTACK
        // ====================

        adsr.trigger_on();
        adsr.tick();
        assert_eq!(adsr.state, Attack);

        // Check bounds
        for i in 0..parameter_len - 1 {
            let envelope = adsr.tick();
            assert!(
                envelope <= 1.0 && envelope >= 0.0,
                "failed at index: {}, val: {}",
                i,
                envelope
            );
        }

        // ===================
        // TRANSITION TO DECAY
        // ===================

        assert_eq!(adsr.state, Attack);
        assert!(adsr.tick() == 1.0);
        assert_eq!(adsr.state, Decay);

        // Check bounds
        for i in 0..parameter_len {
            let envelope = adsr.tick();
            assert!(
                envelope <= 1.0 && envelope >= adsr.sustain,
                "failed at index: {}, val: {}",
                i,
                envelope
            );
        }

        // ==================
        // TRANSITION TO HOLD
        // ==================

        assert_eq!(adsr.state, Decay);
        assert!(adsr.tick() == adsr.sustain);
        assert_eq!(adsr.state, Sustain);

        assert!(adsr.tick() == adsr.sustain);
        assert_eq!(adsr.state, Sustain);

        assert!(adsr.tick() == adsr.sustain);
        assert_eq!(adsr.state, Sustain);

        // =====================
        // TRANSITION TO RELEASE
        // =====================

        adsr.trigger_off();
        assert!(adsr.tick() != adsr.sustain);
        assert_eq!(adsr.state, Release);

        // Check bounds
        for i in 0..parameter_len - 1 {
            let envelope = adsr.tick();
            assert!(
                envelope <= adsr.sustain && envelope >= 0.0,
                "failed at index: {}, val: {}",
                i,
                envelope
            );
        }

        // ==================
        // TRANSITION TO IDLE
        // ==================

        assert_eq!(adsr.state, Release);
        assert!(adsr.tick() == 0.0);
        assert_eq!(adsr.state, Idle);
    }

    #[test]
    fn check_early_release() {
        let time = 0.003; // 3ms
        let sr: f32 = 48_000.0;
        let parameter_len = (time * sr) as usize;
        let slope = 0.5;
        let mut adsr = AudioRateADSR::new(time, time, 0.8, time, slope, sr);

        adsr.trigger_on();

        for _ in 0..parameter_len / 2 {
            adsr.tick();
        }

        // trigger_on early release
        assert_eq!(adsr.state, Attack);
        adsr.trigger_off();
        assert_eq!(adsr.state, Release);
        assert_eq!(adsr.envelope_value, adsr.release_val);

        // Check bounds
        for i in 0..parameter_len {
            let envelope = adsr.tick();
            assert!(
                envelope <= adsr.release_val && envelope >= 0.0,
                "failed at index: {}, val: {}",
                i,
                envelope
            );
        }

        assert_eq!(adsr.state, Release);
        assert!(adsr.tick() == 0.0);
        assert_eq!(adsr.state, Idle);
    }
}
