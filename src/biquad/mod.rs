use core::marker::PhantomData;

pub mod butterworth;

pub struct BiquadCoeffs<T> {
    pub b0: f32,
    pub b1: f32,
    pub b2: f32,
    pub a1: f32,
    pub a2: f32,

    state: PhantomData<T>,
}

/**
Little suite of filters in a `Biquad` topology.

### Example

```rust
use embedded_audio_tools::filter::{Biquad, BiquadCoeffs, Butterworth};

let mut biquad: Biquad<Butterworth> = Biquad::new(BiquadCoeffs::new());

// update coeffs for a lowpass
biquad.coeffs.lowpass(1000.0, 1.0, 48_000.0); // Cutoff = 1kHz, Q = 1.0, fs = 48kHz

// during audio callback
biquad.process(1.0); // process a sample
```
*/
pub struct Biquad<T> {
    z1: f32,
    z2: f32,

    pub coeffs: BiquadCoeffs<T>,
}

impl<T> Biquad<T> {
    pub fn new(filter_type: BiquadCoeffs<T>) -> Biquad<T> {
        Biquad {
            z1: 0.0,
            z2: 0.0,

            coeffs: filter_type,
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let out = self.coeffs.b0 * input + self.z1;

        self.z1 = self.coeffs.b1 * input + self.z2 - self.coeffs.a1 * out;
        self.z2 = self.coeffs.b2 * input - self.coeffs.a2 * out;

        out
    }
}
