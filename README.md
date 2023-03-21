# Embedded Audio Tools
Toolbox for creating audio effects with focus on the embedded aspect of things.

## Memory
`MemorySlice` implements the `Send` trait since it only works **safely** on **statically allocated memory**.

### Example
```rust
use embedded_audio_tools::memory_access::*;

// Thread-safe mutable slice
let mut buffer = [0.0_f32; 24];
let mut mut_slice = from_slice_mut(&mut buffer[..]);

// Null pointer and length of 0
let mut ptr_buffer = null_mut();

// Change associated buffer in runtime
unsafe { 
    ptr_buffer.change_mut_slice_unchecked(buffer.as_mut_ptr(), buffer.len());
}

assert_eq!(ptr_buffer.as_slice(), mut_slice.as_slice());
```

## Biquad
Little suite of filters in a `Biquad` topology.

| `FilterType` | `Butterworth` | `Chebyshev` | `Bessel` |
| ------------ | :-----------: | :---------: | :------: |
| `Lowpass`    |     - [x]     |    - [ ]    |  - [ ]   |
| `Highpass`   |     - [x]     |    - [ ]    |  - [ ]   |
| `Allpass`    |     - [x]     |    - [ ]    |  - [ ]   |
| `Notch`      |     - [x]     |    - [ ]    |  - [ ]   |
| `Bell`       |     - [x]     |    - [ ]    |  - [ ]   |
| `Lowshelf`   |     - [x]     |    - [ ]    |  - [ ]   |
| `Highshelf`  |     - [ ]     |    - [ ]    |  - [ ]   |

### Example
```rust
use embedded_audio_tools::filter::{Biquad, BiquadCoeffs, Butterworth};

let mut biquad: Biquad<Butterworth> = Biquad::new(BiquadCoeffs::new());

// update coeffs for a lowpass
biquad.coeffs.lowpass(1000.0, 1.0, 48_000.0); // Cutoff = 1kHz, Q = 1.0, fs = 48kHz

// during audio callback
biquad.process(1.0); // process a sample
```

## Delay Line
Uses the `MemorySlice` as an underlying building block for buffer handling. Can optionally interpolate in between samples either with `lerp` or `lagrange`.

**Derivates**
* `Comb`
* `Allpass`

## Floats
Some common float related stuff:
* Bitreduction/manipulation
* Raw IEEE 754 conversion
* Various Interpolation Algorithms
* Additional embedded targeted math
* Decibel to voltage (and back) conversion

## Envelope Generator
Currently only implements an `ADSR` with varying steepness.

## Oscillator
A very bad audio oscillator (with no anti-aliasing whatsoever), but maybe not a bad LFO. Comes with the common waveforms:
* `Sine`
* `Square`
* `Triangle`
* `Sawtooth`

It is based on a software phase accumulator which is implemented as a trait bound. In theory, one could implement a hardware accumulator (i.e. timer).

## Stereo
Panning, balacing and crossfading