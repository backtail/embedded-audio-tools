use core::{f32::consts::TAU, ops::Neg};

use crate::lookup_tables::fixed_point_sin;
use crate::{interpolation::lerp_unchecked, phase_accumulator::PhaseAccumulator};

use Waveform::*;

#[allow(unused_imports)]
use micromath::F32Ext;

pub enum Waveform {
    Sine = 0,
    Rectangle = 1,
    Sawtooth = 2,
    Triangle = 3,
}

pub struct UnipolarOscillator<PA>
where
    PA: PhaseAccumulator,
{
    acc: PA,
    wave: Waveform,
}

impl<PA: PhaseAccumulator> UnipolarOscillator<PA> {
    pub fn new(phase_accumulator: PA) -> UnipolarOscillator<PA> {
        UnipolarOscillator {
            acc: phase_accumulator,
            wave: Sine,
        }
    }

    pub fn next(&mut self) -> f32 {
        match self.wave {
            Sine => self.next_sine(),
            Rectangle => self.next_rect(),
            Sawtooth => self.next_saw(),
            Triangle => self.next_tri(),
        }
    }

    #[inline(always)]
    fn next_saw(&mut self) -> f32 {
        normalize_full_u32(self.acc.next_value()) * 2.0 - 1.0
    }

    #[inline(always)]
    fn next_rect(&mut self) -> f32 {
        ((self.next_saw() + 1.0).floor()) * 2.0 - 1.0
    }

    #[inline(always)]
    fn next_sine(&mut self) -> f32 {
        let phase = scale_full_u32_to_f32(self.acc.next_value(), 0.0, TAU);
        fixed_point_sin(phase)
    }

    #[inline(always)]
    fn next_tri(&mut self) -> f32 {
        let x = self.next_saw();
        if x.is_sign_positive() {
            return (1.0 - x) * 2.0 - 1.0;
        } else {
            return (1.0 - x.neg()) * 2.0 - 1.0;
        }
    }

    #[inline(always)]
    pub fn set_freq_unchecked(&mut self, freq: f32) {
        self.acc.set_freq_unchecked(freq);
    }

    #[inline(always)]
    pub fn set_phase_shift_unchecked(&mut self, shift: f32) {
        self.acc.set_phase_shift((shift * u32::MAX as f32) as u32)
    }

    #[inline(always)]
    pub fn set_wave(&mut self, wave_select: Waveform) {
        self.wave = wave_select;
    }

    #[inline(always)]
    pub fn set_sr_unchecked(&mut self, sr: f32) {
        self.acc.set_sr_unchecked(sr);
    }
}

#[inline(always)]
fn normalize_full_u32(phase: u32) -> f32 {
    phase as f32 / (u32::MAX as f32 + 1.0)
}

#[inline(always)]
fn scale_full_u32_to_f32(phase: u32, min: f32, max: f32) -> f32 {
    lerp_unchecked(min, max, normalize_full_u32(phase))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::phase_accumulator::SoftPhaseAccumulator;

    const SR: f32 = 48_000.0;
    const FREQ: f32 = 1000.0;
    const CYLCLE_SAMPLES: u32 = (SR / FREQ) as u32;

    #[test]
    fn check_bounds_saw() {
        let mut osc = UnipolarOscillator::new(SoftPhaseAccumulator::new(FREQ, SR));

        osc.set_wave(Sawtooth);

        for i in 0..(2 * CYLCLE_SAMPLES) {
            let val = osc.next();
            assert!(val >= 0.0 && val <= 1.0, "Failed at index: {}", i);
        }
    }

    #[test]
    fn check_bounds_rect() {
        let mut osc = UnipolarOscillator::new(SoftPhaseAccumulator::new(FREQ, SR));

        osc.set_wave(Rectangle);

        for i in 0..(2 * CYLCLE_SAMPLES) {
            let val = osc.next();
            assert!(val == 0.0 || val == 1.0, "Failed at index: {}", i);
        }
    }

    #[test]
    fn check_bounds_sine() {
        let mut osc = UnipolarOscillator::new(SoftPhaseAccumulator::new(FREQ, SR));

        osc.set_wave(Sine);

        for i in 0..(2 * CYLCLE_SAMPLES) {
            let val = osc.next();
            assert!(val >= 0.0 && val <= 1.0, "Failed at index: {}", i);
        }
    }

    #[test]
    fn check_bounds_tri() {
        let mut osc = UnipolarOscillator::new(SoftPhaseAccumulator::new(FREQ, SR));

        osc.set_wave(Triangle);

        for i in 0..(2 * CYLCLE_SAMPLES) {
            let val = osc.next();
            assert!(val >= 0.0 && val <= 1.0, "Failed at index: {}", i);
        }
    }
}
