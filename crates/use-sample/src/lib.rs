#![forbid(unsafe_code)]
//! Primitive signal sample helpers.
//!
//! The crate intentionally keeps sample handling explicit: finite sample values,
//! simple counting, and duration calculations from a sample rate.
//!
//! # Examples
//!
//! ```rust
//! use use_sample::{Sample, duration_seconds};
//!
//! let sample = Sample::new(-0.25).unwrap();
//!
//! assert_eq!(sample.abs(), 0.25);
//! assert_eq!(duration_seconds(480, 48_000.0).unwrap(), 0.01);
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sample {
    pub value: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleError {
    InvalidValue,
    InvalidSampleRate,
}

impl Sample {
    pub fn new(value: f64) -> Result<Self, SampleError> {
        if !value.is_finite() {
            return Err(SampleError::InvalidValue);
        }

        Ok(Self { value })
    }

    #[must_use]
    pub fn value(&self) -> f64 {
        self.value
    }

    #[must_use]
    pub fn abs(&self) -> f64 {
        self.value.abs()
    }

    #[must_use]
    pub fn is_silent(&self) -> bool {
        self.value == 0.0
    }
}

pub fn validate_samples(samples: &[f64]) -> Result<(), SampleError> {
    if samples.iter().all(|sample| sample.is_finite()) {
        Ok(())
    } else {
        Err(SampleError::InvalidValue)
    }
}

#[must_use]
pub fn sample_count(samples: &[f64]) -> usize {
    samples.len()
}

pub fn duration_seconds(sample_count: usize, sample_rate_hz: f64) -> Result<f64, SampleError> {
    if !sample_rate_hz.is_finite() || sample_rate_hz <= 0.0 {
        return Err(SampleError::InvalidSampleRate);
    }

    Ok(sample_count as f64 / sample_rate_hz)
}

#[cfg(test)]
mod tests {
    use super::{duration_seconds, sample_count, validate_samples, Sample, SampleError};

    #[test]
    fn constructs_sample_and_reports_properties() {
        let sample = Sample::new(-0.25).unwrap();

        assert_eq!(sample.value(), -0.25);
        assert_eq!(sample.abs(), 0.25);
        assert!(!sample.is_silent());
    }

    #[test]
    fn validates_finite_samples_and_counts_them() {
        let values = [-1.0, 0.0, 1.0];

        assert_eq!(validate_samples(&values), Ok(()));
        assert_eq!(sample_count(&values), 3);
    }

    #[test]
    fn accepts_empty_sample_slices() {
        assert_eq!(validate_samples(&[]), Ok(()));
        assert_eq!(sample_count(&[]), 0);
        assert_eq!(duration_seconds(0, 48_000.0).unwrap(), 0.0);
    }

    #[test]
    fn rejects_invalid_sample_values() {
        assert_eq!(Sample::new(f64::NAN), Err(SampleError::InvalidValue));
        assert_eq!(
            validate_samples(&[0.0, f64::INFINITY]),
            Err(SampleError::InvalidValue)
        );
    }

    #[test]
    fn rejects_invalid_sample_rates() {
        assert_eq!(
            duration_seconds(100, 0.0),
            Err(SampleError::InvalidSampleRate)
        );
        assert_eq!(
            duration_seconds(100, f64::NAN),
            Err(SampleError::InvalidSampleRate)
        );
    }
}
