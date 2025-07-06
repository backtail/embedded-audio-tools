use embedded_audio_tools::envelope::{ARPhase, FixedPointAttackRelease};
use rgb::RGB8;
use textplots::{Chart, ColorPlot, Shape};

const N_STEPS: usize = 48000;
const F_BITS: u8 = 16;

fn main() {
    let mut ar = FixedPointAttackRelease::<F_BITS>::new();

    let mut input_step = vec![];
    let mut out0 = vec![];

    // input values
    for (index, _sample) in (0..N_STEPS).enumerate() {
        let step = if index < (N_STEPS / 2) { 1.0 } else { 0.0 };
        input_step.push((index as f32, step));
    }

    ar.set_all(
        ARPhase::ATTACK,
        1 << (F_BITS - 1),
        2 * 1 << F_BITS,
        -10 * (1 << F_BITS),
        (N_STEPS << F_BITS) as i32,
    );
    ar.set_all(
        ARPhase::RELEASE,
        1 << (F_BITS - 1),
        1 << F_BITS,
        -10 * (1 << F_BITS),
        (N_STEPS << F_BITS) as i32,
    );

    // output values
    for (index, _sample) in &input_step {
        if *index == 0.0 {
            ar.trigger();
        } else if *index == N_STEPS as f32 / 4.0 {
            ar.trigger();
        } else if *index == N_STEPS as f32 / 2.0 {
            ar.release();
        } else if *index == (N_STEPS as f32 * 3.0) / 4.0 {
            ar.trigger();
        }
        out0.push((*index, ar.tick() as f32));
    }

    Chart::new(320, 120, 0.0, N_STEPS as f32)
        // .linecolorplot(&Shape::Lines(input_step.as_slice()), RGB8::new(255, 0, 0))
        .linecolorplot(&Shape::Lines(out0.as_slice()), RGB8::new(0, 255, 0))
        .display();
}
