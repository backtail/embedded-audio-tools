use embedded_audio_tools::FFCompressor;
use hound::WavReader;
use rgb::RGB8;
use textplots::{Chart, ColorPlot, Shape};

const SR: f32 = 44100.0;
const RATIO: f32 = 100.5;
fn main() {
    let mut comp = FFCompressor::new(0.3, 2.0, 1.0);

    comp.set_threshold(0.3);
    comp.set_attack(0.001, SR);
    comp.set_release(0.3, SR);
    comp.set_attack_slope(-10.0);
    comp.set_release_slope(10.0);
    comp.set_ratio(RATIO);
    comp.set_makeup_gain(1.3);

    let mut reader = WavReader::open("examples/break.wav").unwrap();

    let mut input_samples = vec![];
    let mut abs_in_samples = vec![];
    let mut output_samples = vec![];
    let mut envelope = vec![];
    let mut threshold = vec![];
    let mut threshold_inv = vec![];
    let mut env_stage = vec![];
    let mut cv = vec![];

    // input values
    for (index, sample) in reader.samples::<f32>().enumerate() {
        let val = sample.unwrap();
        input_samples.push((index as f32 / SR, val));
        abs_in_samples.push((index as f32 / SR, val.abs()));
    }

    let max_samples = input_samples
        .iter()
        .fold(f32::NEG_INFINITY, |prev, index| prev.max(index.0));

    // output values
    for sample in &input_samples {
        envelope.push((sample.0, comp.get_current_env_val()));
        cv.push((sample.0, 1.0 / (comp.get_current_cv())));
        threshold.push((sample.0, comp.get_current_threshold()));
        threshold_inv.push((sample.0, -comp.get_current_threshold()));
        env_stage.push((sample.0, comp.get_current_env_stage() as f32 / 2.0));
        output_samples.push((sample.0, comp.tick(sample.1)));
    }

    let range = max_samples;

    Chart::new(320, 60, 0.0, range)
        .linecolorplot(
            &Shape::Lines(input_samples.as_slice()),
            RGB8::new(255, 0, 0),
        )
        .linecolorplot(
            &Shape::Lines(output_samples.as_slice()),
            RGB8::new(0, 255, 0),
        )
        .linecolorplot(
            &Shape::Lines(threshold_inv.as_slice()),
            RGB8::new(0, 0, 255),
        )
        .linecolorplot(&Shape::Lines(threshold.as_slice()), RGB8::new(0, 0, 255))
        .display();

    Chart::new(320, 50, 0.0, range)
        .linecolorplot(&Shape::Lines(env_stage.as_slice()), RGB8::new(255, 255, 0))
        .linecolorplot(&Shape::Lines(cv.as_slice()), RGB8::new(0, 255, 255))
        // .linecolorplot(&Shape::Lines(envelope.as_slice()), RGB8::new(255, 0, 255))
        .display();
}
