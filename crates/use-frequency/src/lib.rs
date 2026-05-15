#![forbid(unsafe_code)]
//! Primitive frequency helpers.
//!
//! The crate keeps frequency calculations narrow: finite positive Hertz values,
//! periods, angular frequency, and Nyquist limits.
//!
//! # Examples
//!
//! ```rust
//! use use_frequency::{Frequency, nyquist_frequency};
//!
//! let frequency = Frequency::new(440.0).unwrap();
//!
//! assert!(frequency.period_seconds() > 0.0);
//! assert_eq!(nyquist_frequency(48_000.0).unwrap(), 24_000.0);
//! ```

use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frequency {
    pub hz: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrequencyError {
    InvalidFrequency,
    InvalidSampleRate,
}

fn validate_positive(value: f64, error: FrequencyError) -> Result<f64, FrequencyError> {
    if !value.is_finite() || value <= 0.0 {
        Err(error)
    } else {
        Ok(value)
    }
}

impl Frequency {
    pub fn new(hz: f64) -> Result<Self, FrequencyError> {
        Ok(Self {
            hz: validate_positive(hz, FrequencyError::InvalidFrequency)?,
        })
    }

    #[must_use]
    pub fn hz(&self) -> f64 {
        self.hz
    }

    #[must_use]
    pub fn period_seconds(&self) -> f64 {
        1.0 / self.hz
    }

    #[must_use]
    pub fn angular_frequency(&self) -> f64 {
        2.0 * PI * self.hz
    }
}

pub fn period_seconds(hz: f64) -> Result<f64, FrequencyError> {
    Ok(1.0 / validate_positive(hz, FrequencyError::InvalidFrequency)?)
}

pub fn angular_frequency(hz: f64) -> Result<f64, FrequencyError> {
    Ok(2.0 * PI * validate_positive(hz, FrequencyError::InvalidFrequency)?)
}

pub fn nyquist_frequency(sample_rate_hz: f64) -> Result<f64, FrequencyError> {
    Ok(validate_positive(sample_rate_hz, FrequencyError::InvalidSampleRate)? / 2.0)
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::{Frequency, FrequencyError, angular_frequency, nyquist_frequency, period_seconds};

    #[test]
    fn constructs_frequency_and_reports_values() {
        let frequency = Frequency::new(4.0).unwrap();

        assert_eq!(frequency.hz(), 4.0);
        assert_eq!(frequency.period_seconds(), 0.25);
        assert_eq!(frequency.angular_frequency(), 8.0 * PI);
    }

    #[test]
    fn computes_free_frequency_helpers() {
        assert_eq!(period_seconds(2.0).unwrap(), 0.5);
        assert_eq!(angular_frequency(2.0).unwrap(), 4.0 * PI);
        assert_eq!(nyquist_frequency(48_000.0).unwrap(), 24_000.0);
    }

    #[test]
    fn rejects_invalid_frequencies() {
        assert_eq!(Frequency::new(0.0), Err(FrequencyError::InvalidFrequency));
        assert_eq!(
            period_seconds(f64::NAN),
            Err(FrequencyError::InvalidFrequency)
        );
    }

    #[test]
    fn rejects_invalid_sample_rates() {
        assert_eq!(
            nyquist_frequency(0.0),
            Err(FrequencyError::InvalidSampleRate)
        );
        assert_eq!(
            nyquist_frequency(f64::INFINITY),
            Err(FrequencyError::InvalidSampleRate)
        );
    }
}
