use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

#[derive(Debug, PartialEq)]
pub enum InterpolationError {
    InputNaN,
    InputInfinite,
    InterpolationRange,
}

#[inline(never)]
pub fn lerp_unchecked(a: f32, b: f32, interpolate: f32) -> f32 {
    (a * (1.0 - interpolate)) + (b * interpolate)
}

#[inline(never)]
pub fn lerp(a: f32, b: f32, interpolate: f32) -> Result<f32, InterpolationError> {
    if a.is_nan() || b.is_nan() {
        return Err(InterpolationError::InputNaN);
    }

    if a.is_infinite() || b.is_infinite() {
        return Err(InterpolationError::InputInfinite);
    }

    if interpolate < 0.0 || interpolate > 1.0 {
        return Err(InterpolationError::InterpolationRange);
    }

    Ok(lerp_unchecked(
        a,
        b,
        (a * (1.0 - interpolate)) + (b * interpolate),
    ))
}

#[inline(never)]
pub fn lagrange(array: &[f32], x_point: f32) -> f32 {
    let mut y_point = 0.0_f32;
    for i in 0..array.len() {
        let mut term = array[i];
        for j in 0..array.len() {
            if i != j {
                term = (term * (x_point - j as f32)) / (i as f32 - j as f32);
            }
        }
        y_point += term;
    }

    return y_point;
}

#[inline(never)]
pub unsafe fn lagrange_only_4_elements(array: &[f32], x_point: f32) -> f32 {
    let mut y_point = 0.0_f32;

    let mut term = unsafe { *array.get_unchecked(0) };
    term = (term * (x_point - 1.0)) / -1.0;
    term = (term * (x_point - 2.0)) / -2.0;
    term = (term * (x_point - 3.0)) / -3.0;

    y_point += term;

    let mut term = unsafe { *array.get_unchecked(1) };
    term = (term * (x_point - 0.0)) / 1.0;
    term = (term * (x_point - 2.0)) / -1.0;
    term = (term * (x_point - 3.0)) / -2.0;

    y_point += term;

    let mut term = unsafe { *array.get_unchecked(2) };
    term = (term * (x_point - 0.0)) / 2.0;
    term = (term * (x_point - 1.0)) / 1.0;
    term = (term * (x_point - 3.0)) / -1.0;

    y_point += term;

    let mut term = unsafe { *array.get_unchecked(3) };
    term = (term * (x_point - 0.0)) / 3.0;
    term = (term * (x_point - 1.0)) / 2.0;
    term = (term * (x_point - 2.0)) / 1.0;

    y_point += term;

    return y_point;
}

const POWER_OF_2: u32 = 4;
const N_ELEMENTS: usize = 2_usize.pow(POWER_OF_2);

fn bench_math(c: &mut Criterion) {
    let mut group = c.benchmark_group("Interpolation");

    group.bench_function(BenchmarkId::new("Linear (no BC)", 0), |b| {
        b.iter(|| lerp_unchecked(0.0, 0.0, 0.0))
    });

    group.bench_function(BenchmarkId::new("Linear (with BC)", 0), |b| {
        b.iter(|| lerp(0.0, 0.0, 0.0))
    });

    for size in (1..=POWER_OF_2).map(|x: u32| 2_u32.pow(x) as usize) {
        let slice = [0.0_f32; N_ELEMENTS];

        if size == 4 {
            group.bench_with_input(
                BenchmarkId::new("Langrange (no BC)", size),
                &size,
                |b, &size| b.iter(|| unsafe { lagrange_only_4_elements(&slice[..size], 0.0) }),
            );
        }

        group.bench_with_input(
            BenchmarkId::new("Langrange (with BC)", size),
            &size,
            |b, &size| b.iter(|| lagrange(&slice[..size], 0.0)),
        );
    }

    group.finish();
}

criterion_group!(benches, bench_math);
criterion_main!(benches);
