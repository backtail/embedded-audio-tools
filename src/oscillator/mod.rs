pub(crate) mod lookup_tables;
pub mod osc_functional;
pub mod osc_wavetable;
pub mod phase_accumulator;

pub use osc_functional::FunctionalOscillator;
pub use osc_wavetable::WavetableOscillator;
pub use phase_accumulator::{PhaseAccumulator, SoftPhaseAccumulator};

#[repr(C)]
pub enum Waveform {
    Sine = 0,
    Rectangle = 1,
    Sawtooth = 2,
    Triangle = 3,
}
