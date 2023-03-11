use FrequencyError::*;

pub trait PhaseAccumulator {
    type Object;
    fn new(freq: f32, sr: f32) -> Self::Object;
    fn tick(&mut self);
    fn set_sr_unchecked(&mut self, sr: f32);
    fn set_freq_unchecked(&mut self, freq: f32);
    fn get_counter(&self) -> u32;
}

impl PhaseAccumulator for SoftPhaseAccumulator {
    type Object = SoftPhaseAccumulator;

    fn new(freq: f32, sr: f32) -> SoftPhaseAccumulator {
        SoftPhaseAccumulator {
            counter: 0,
            freq,
            min_step: u32::MAX as f32 / sr,
        }
    }

    fn tick(&mut self) {
        self.counter = self
            .counter
            .wrapping_add((self.freq * self.min_step) as u32);
    }

    fn set_sr_unchecked(&mut self, sr: f32) {
        self.min_step = u32::MAX as f32 / sr;
    }

    fn set_freq_unchecked(&mut self, freq: f32) {
        self.freq = freq;
    }

    fn get_counter(&self) -> u32 {
        self.counter
    }
}

pub struct SoftPhaseAccumulator {
    counter: u32,
    freq: f32,
    min_step: f32,
}

pub enum FrequencyError {
    Zero,
    Negative,
    BiggerThanNyquist,
}

impl SoftPhaseAccumulator {
    pub fn set_freq(mut self, freq: f32) -> Result<(), FrequencyError> {
        if freq == 0.0 {
            return Err(Zero);
        }

        if freq < 0.0 {
            return Err(Negative);
        }

        return Ok(self.set_freq_unchecked(freq));
    }
}
