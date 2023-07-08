pub trait DSPUtility {
    type Output;
    fn samples_to_seconds(&self, sr: f32) -> Self::Output;
    fn samples_to_millis(&self, sr: f32) -> Self::Output;

    fn seconds_to_samples(&self, sr: f32) -> Self::Output;
    fn millis_to_samples(&self, sr: f32) -> Self::Output;
}

impl DSPUtility for f32 {
    type Output = f32;

    /// Converts samples to seconds
    ///
    /// ## Example
    /// ```rust
    /// use embedded_audio_tools::float::DSPUtility;
    ///
    /// assert_eq!(48.0.samples_to_seconds(48_000.0), 0.001);
    /// ```
    #[inline(always)]
    fn samples_to_seconds(&self, sr: f32) -> Self::Output {
        self / sr
    }

    /// Converts samples to milliseconds
    ///
    /// ## Example
    /// ```rust
    /// use embedded_audio_tools::float::DSPUtility;
    ///
    /// assert_eq!(48.0.samples_to_millis(48_000.0), 1.0);
    /// ```
    #[inline(always)]
    fn samples_to_millis(&self, sr: f32) -> Self::Output {
        (self / sr) * 1000.0
    }

    /// Converts seconds to samples
    ///
    /// ## Example
    /// ```rust
    /// use embedded_audio_tools::float::DSPUtility;
    ///
    /// assert_eq!(1.0.seconds_to_samples(48_000.0), 48_000.0);
    /// ```
    #[inline(always)]
    fn seconds_to_samples(&self, sr: f32) -> Self::Output {
        self * sr
    }

    /// Converts seconds to samples
    ///
    /// ## Example
    /// ```rust
    /// use embedded_audio_tools::float::DSPUtility;
    ///
    /// assert_eq!(1.0.millis_to_samples(48_000.0), 48.0);
    /// ```
    #[inline(always)]
    fn millis_to_samples(&self, sr: f32) -> Self::Output {
        (self * sr) / 1000.0
    }
}
