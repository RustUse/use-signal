#![forbid(unsafe_code)]
//! Primitive windowing helpers.
//!
//! The crate intentionally limits itself to a few deterministic windows without
//! introducing FFT or DSP dependencies.
//!
//! # Examples
//!
//! ```rust
//! use use_signal_window::{WindowKind, apply_window, window_coefficients};
//!
//! let coefficients = window_coefficients(WindowKind::Hann, 4).unwrap();
//! let windowed = apply_window(&[1.0, 1.0, 1.0, 1.0], WindowKind::Hann).unwrap();
//!
//! assert_eq!(coefficients, windowed);
//! ```

use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowKind {
    Rectangular,
    Hann,
    Hamming,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowError {
    InvalidLength,
    InvalidSample,
}

pub fn window_coefficients(kind: WindowKind, len: usize) -> Result<Vec<f64>, WindowError> {
    if len == 0 {
        return Err(WindowError::InvalidLength);
    }

    if len == 1 {
        return Ok(vec![1.0]);
    }

    let denominator = (len - 1) as f64;
    let mut coefficients = Vec::with_capacity(len);

    for index in 0..len {
        let angle = 2.0 * PI * index as f64 / denominator;
        let coefficient = match kind {
            WindowKind::Rectangular => 1.0,
            WindowKind::Hann => 0.5 - 0.5 * angle.cos(),
            WindowKind::Hamming => 0.54 - 0.46 * angle.cos(),
        };
        coefficients.push(coefficient);
    }

    Ok(coefficients)
}

pub fn apply_window(samples: &[f64], kind: WindowKind) -> Result<Vec<f64>, WindowError> {
    if samples.iter().any(|sample| !sample.is_finite()) {
        return Err(WindowError::InvalidSample);
    }

    let coefficients = window_coefficients(kind, samples.len())?;

    Ok(samples
        .iter()
        .zip(coefficients)
        .map(|(sample, coefficient)| sample * coefficient)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::{WindowError, WindowKind, apply_window, window_coefficients};

    #[test]
    fn computes_window_coefficients_with_expected_lengths() {
        let coefficients = window_coefficients(WindowKind::Hann, 4).unwrap();

        assert_eq!(coefficients.len(), 4);
        assert!((coefficients[0] - 0.0).abs() < 1.0e-12);
        assert!((coefficients[3] - 0.0).abs() < 1.0e-12);
    }

    #[test]
    fn handles_single_value_windows_deliberately() {
        assert_eq!(
            window_coefficients(WindowKind::Rectangular, 1).unwrap(),
            vec![1.0]
        );
        assert_eq!(window_coefficients(WindowKind::Hann, 1).unwrap(), vec![1.0]);
        assert_eq!(
            apply_window(&[2.0], WindowKind::Hamming).unwrap(),
            vec![2.0]
        );
    }

    #[test]
    fn applies_windows_to_samples() {
        let windowed = apply_window(&[1.0, 1.0, 1.0, 1.0], WindowKind::Rectangular).unwrap();

        assert_eq!(windowed, vec![1.0, 1.0, 1.0, 1.0]);
    }

    #[test]
    fn rejects_empty_windows() {
        assert_eq!(
            window_coefficients(WindowKind::Rectangular, 0),
            Err(WindowError::InvalidLength)
        );
        assert_eq!(
            apply_window(&[], WindowKind::Hann),
            Err(WindowError::InvalidLength)
        );
    }

    #[test]
    fn rejects_non_finite_samples() {
        assert_eq!(
            apply_window(&[1.0, f64::NAN], WindowKind::Hann),
            Err(WindowError::InvalidSample)
        );
    }
}
