use crate::float::AdditionalF32Ext;

use super::BiquadCoeffs;
use core::{f32::consts::PI, marker::PhantomData, ops::Neg};

#[allow(unused_imports)]
use micromath::F32Ext;

pub enum ButterworthType {
    Lowpass = 0,
    Highpass = 1,
    Allpass = 2,
    Notch = 3,
    Bell = 4,
    LowShelf = 5,
}

/// Coeffiecients based on this article: https://www.musicdsp.org/en/latest/Filters/37-zoelzer-biquad-filters.html
///
/// Uses tan instead of cos and sin to calculate coefficients
pub struct Butterworth;

impl BiquadCoeffs<Butterworth> {
    pub fn new() -> BiquadCoeffs<Butterworth> {
        BiquadCoeffs {
            b0: 0.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
            state: PhantomData,
        }
    }

    #[inline(always)]
    fn setup_coeffs(&self, fc: f32, q: f32, sr: f32) -> (f32, f32) {
        let k = ((PI * fc) / sr).fast_tan();
        (k * k, k / q)
    }

    pub fn lowpass(&mut self, fc: f32, q: f32, sr: f32) {
        let (k2, k_q) = self.setup_coeffs(fc, q, sr);

        let a0 = 1.0 + k_q + k2;
        let norm = 1.0 / a0;

        self.b0 = norm * k2;
        self.b1 = norm * 2.0 * k2;
        self.b2 = self.b0;
        self.a1 = norm * 2.0 * (k2 - 1.0);
        self.a2 = norm * (1.0 - k_q + k2);
    }

    pub fn highpass(&mut self, fc: f32, q: f32, sr: f32) {
        let (k2, k_q) = self.setup_coeffs(fc, q, sr);

        let a0 = 1.0 + k_q + k2;
        let norm = 1.0 / a0;

        self.b0 = norm;
        self.b1 = norm * -2.0;
        self.b2 = norm;
        self.a1 = norm * (2.0 * (k2 - 1.0));
        self.a2 = norm * (1.0 - k_q + k2)
    }

    pub fn allpass(&mut self, fc: f32, q: f32, sr: f32) {
        let (k2, k_q) = self.setup_coeffs(fc, q, sr);

        let a0 = 1.0 + k_q + k2;
        let norm = 1.0 / a0;

        self.b0 = norm * (1.0 - k_q + k2);
        self.b1 = norm * (2.0 * (k2 - 1.0));
        self.b2 = 1.0;
        self.a1 = self.b1;
        self.a2 = self.b0;
    }

    pub fn notch(&mut self, fc: f32, q: f32, sr: f32) {
        let (k2, k_q) = self.setup_coeffs(fc, q, sr);

        let a0 = 1.0 + k_q + k2;
        let norm = 1.0 / a0;

        self.b0 = norm * (1.0 + k2);
        self.b1 = norm * (2.0 * (k2 - 1.0));
        self.b2 = self.b0;
        self.a1 = self.b1;
        self.a2 = norm * (1.0 - k_q + k2);
    }

    #[inline(always)]
    pub fn bell_boost_only(&mut self, fc: f32, q: f32, gain: f32, sr: f32) {
        let (k2, k_q) = self.setup_coeffs(fc, q, sr);

        let a0 = 1.0 + k_q + k2;
        let norm = 1.0 / a0;

        self.b0 = norm * (1.0 + gain * k_q + k2);
        self.b1 = norm * (2.0 * (k2 - 1.0));
        self.b2 = norm * (1.0 - gain * k_q + k2);
        self.a1 = self.b1;
        self.a2 = norm * (1.0 - k_q + k2);
    }

    #[inline(always)]
    pub fn bell_cut_only(&mut self, fc: f32, q: f32, gain: f32, sr: f32) {
        let (k2, k_q) = self.setup_coeffs(fc, q, sr);

        let gain = gain.neg();
        let a0 = 1.0 + gain * k_q + k2;
        let norm = 1.0 / a0;

        self.b0 = norm * (1.0 + k_q + k2);
        self.b1 = norm * (2.0 * (k2 - 1.0));
        self.b2 = norm * (1.0 - k_q + k2);
        self.a1 = self.b1;
        self.a2 = norm * (1.0 - gain * k_q + k2);
    }

    pub fn bell(&mut self, fc: f32, q: f32, gain: f32, sr: f32) {
        if gain.is_sign_positive() {
            self.bell_boost_only(fc, q, gain, sr);
        } else {
            self.bell_cut_only(fc, q, gain, sr);
        }
    }

    #[inline(always)]
    pub fn low_shelf_boost_only(&mut self, fc: f32, q: f32, gain: f32, sr: f32) {
        let (k2, k_q) = self.setup_coeffs(fc, q, sr);

        let a0 = 1.0 + k_q + k2;
        let norm = 1.0 / a0;

        self.b0 = norm * (1.0 + (gain * 2.0 * k2).sqrt() + gain * k2);
        self.b1 = norm * (2.0 * (gain * k2 - 1.0));
        self.b2 = norm * (1.0 - (gain * 2.0 * k2).sqrt() + gain * k2);
        self.a1 = norm * (2.0 * (k2 - 1.0));
        self.a2 = norm * (1.0 - k_q + k2);
    }

    #[inline(always)]
    pub fn low_shelf_cut_only(&mut self, fc: f32, q: f32, gain: f32, sr: f32) {
        let (k2, k_q) = self.setup_coeffs(fc, q, sr);

        let gain = gain.neg();
        let a0 = 1.0 + (gain * 2.0 * k2).sqrt() + gain * k2;
        let norm = 1.0 / a0;

        self.b0 = norm * (1.0 + k_q + k2);
        self.b1 = norm * (2.0 * (k2 - 1.0));
        self.b2 = norm * (1.0 - k_q + k2);
        self.a1 = norm * (2.0 * (gain * k2 - 1.0));
        self.a2 = norm * (1.0 - (gain * 2.0 * k2).sqrt() + gain * k2);
    }

    pub fn low_shelf(&mut self, fc: f32, q: f32, gain: f32, sr: f32) {
        if gain.is_sign_positive() {
            self.low_shelf_boost_only(fc, q, gain, sr);
        } else {
            self.low_shelf_cut_only(fc, q, gain, sr);
        }
    }
}
