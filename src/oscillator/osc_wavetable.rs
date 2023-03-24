use crate::{
    memory::{memory_slice::MemorySlice, NonMutable},
    oscillator::phase_accumulator::PhaseAccumulator,
};

pub struct WavetableOscillator<PA>
where
    PA: PhaseAccumulator,
{
    lookup_table: MemorySlice<NonMutable>,
    acc: PA,
}

impl<PA: PhaseAccumulator> WavetableOscillator<PA> {
    pub fn new(lookup_table: MemorySlice<NonMutable>, acc: PA) -> Self {
        WavetableOscillator { lookup_table, acc }
    }

    pub fn next(&mut self) -> f32 {
        // calculate phase
        let phase = self.acc.next_value_normalized();

        // get interpolated sample
        unsafe {
            self.lookup_table
                .lerp_unchecked(self.lookup_table.len() as f32 * phase)
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
    pub fn set_sr_unchecked(&mut self, sr: f32) {
        self.acc.set_sr_unchecked(sr);
    }
}
