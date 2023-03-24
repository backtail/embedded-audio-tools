pub fn simpsons_rule<const N: usize>(f: fn(f32) -> f32, a: f32, b: f32) -> f32 {
    let h = (b - a) / (N as f32);
    let mut x = [0.0; N];
    for i in 0..N {
        x[i] = a + (i as f32) * h;
    }
    let mut y = [0.0; N];
    for i in 0..N {
        y[i] = f(x[i]);
    }
    let integral = h / 3.0
        * (f(a)
            + 4.0 * y[1..N].iter().step_by(2).sum::<f32>()
            + 2.0 * y[2..N - 1].iter().step_by(2).sum::<f32>()
            + 4.0 * y[1..].iter().step_by(2).sum::<f32>()
            + if N % 2 == 0 { f(b) } else { 0.0 });
    integral
}
