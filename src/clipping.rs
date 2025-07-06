#[allow(unused_imports)]
use micromath::F32Ext;

pub fn digital_clip(sample: f32) -> f32 {
    sample.clamp(-1.0, 1.0)
}

pub fn tanh_clip(sample: f32) -> f32 {
    if sample > 1.0 {
        1.0
    } else if sample < -1.0 {
        -1.0
    } else {
        todo!()
    }
}

pub fn poly_clip(sample: f32, _n: f32) -> f32 {
    if sample > 1.0 {
        1.0
    } else if sample < -1.0 {
        -1.0
    } else {
        todo!()
    }
}

pub fn sigmoid_clip(sample: f32, _k: f32) -> f32 {
    if sample > 1.0 {
        1.0
    } else if sample < -1.0 {
        -1.0
    } else {
        todo!()
    }
}
