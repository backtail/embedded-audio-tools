use embedded_audio_tools::envelope::ar::{ARPhase, AttackRelease};
use rgb::RGB8;
use textplots::{Chart, ColorPlot, Shape};

const N_STEPS: usize = 100;

fn main() {
    let mut ar = AttackRelease::new(0.0);

    let mut input_step = vec![];
    let mut out0 = vec![];

    // input values
    for (index, _sample) in (0..N_STEPS).enumerate() {
        let step = if index < (N_STEPS / 2) { 1.0 } else { 0.0 };
        input_step.push((index as f32, step));
    }

    ar.set_all(ARPhase::ATTACK, 0.5, 1.0, -2.0, N_STEPS as f32);
    ar.set_all(ARPhase::RELEASE, 0.5, 0.0, 0.0, N_STEPS as f32);

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
        out0.push((*index, ar.tick()));
    }

    Chart::new(320, 120, 0.0, N_STEPS as f32)
        .linecolorplot(&Shape::Lines(input_step.as_slice()), RGB8::new(255, 0, 0))
        .linecolorplot(&Shape::Lines(out0.as_slice()), RGB8::new(0, 255, 0))
        .display();
}
