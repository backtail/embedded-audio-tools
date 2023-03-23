use core::ops::Neg;

#[allow(unused_imports)]
use micromath::F32Ext;

pub trait MissingF32Ext {
    type Output;
    fn sinh(&self) -> Self::Output;
    fn cosh(&self) -> Self::Output;
    fn fast_tan(&self) -> Self::Output;
    fn modulus(&self, n: Self::Output) -> Self::Output;
}

impl MissingF32Ext for f32 {
    type Output = f32;
    fn sinh(&self) -> Self::Output {
        (self.exp() - self.neg().exp()) * 0.5
    }

    fn cosh(&self) -> Self::Output {
        (self.exp() + self.neg().exp()) * 0.5
    }

    /// Taylor series expansion of tan(x), where x = 0, only accurate between 0.0 and 1.0
    ///
    /// Meant for fast computation of filter coefficients. Error always less than fast_tan(1.0): 0.0009920597
    fn fast_tan(&self) -> Self::Output {
        let mut res = 0.0_f32;
        res += self;
        res += (1.0 / 3.0) * self.powi(3);
        res += (2.0 / 15.0) * self.powi(5);
        res += (17.0 / 315.0) * self.powi(7);
        // res += (62.0 / 2835.0) * self.powi(9);
        // res += (1382.0 / 155925.0) * self.powi(11);
        // res += (21844.0 / 6081075.0) * self.powi(13);
        // res += (929569.0 / 638512875.0) * self.powi(15);
        res
    }

    fn modulus(&self, n: Self::Output) -> Self::Output {
        (self % n + n) % n
    }
}

#[cfg(test)]
mod tests {
    use super::MissingF32Ext;

    #[test]
    fn sinh() {
        assert_eq!(1.0.sinh(), 1.17520119);
    }

    #[test]
    fn cosh() {
        assert_eq!(1.0.cosh(), 1.5430806);
    }
}
