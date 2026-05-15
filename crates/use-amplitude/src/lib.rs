#![forbid(unsafe_code)]
//! Primitive amplitude helpers.
//!
//! The crate provides small slice-based helpers for common amplitude summaries.
//! Empty slices and non-finite values return `None`.
//!
//! # Examples
//!
//! ```rust
//! use use_amplitude::{peak_amplitude, rms_amplitude};
//!
//! let samples = [-1.0, 0.0, 1.0];
//!
//! assert_eq!(peak_amplitude(&samples), Some(1.0));
//! assert_eq!(rms_amplitude(&samples).unwrap(), (2.0_f64 / 3.0).sqrt());
//! ```

fn validated_samples(samples: &[f64]) -> Option<&[f64]> {
    if samples.is_empty() || samples.iter().any(|sample| !sample.is_finite()) {
        None
    } else {
        Some(samples)
    }
}

pub fn peak_amplitude(samples: &[f64]) -> Option<f64> {
    let mut values = validated_samples(samples)?.iter().copied();
    let first = values.next()?.abs();

    Some(values.fold(first, |peak, sample| peak.max(sample.abs())))
}

pub fn min_amplitude(samples: &[f64]) -> Option<f64> {
    let mut values = validated_samples(samples)?.iter().copied();
    let first = values.next()?;

    Some(values.fold(first, f64::min))
}

pub fn max_amplitude(samples: &[f64]) -> Option<f64> {
    let mut values = validated_samples(samples)?.iter().copied();
    let first = values.next()?;

    Some(values.fold(first, f64::max))
}

pub fn peak_to_peak_amplitude(samples: &[f64]) -> Option<f64> {
    Some(max_amplitude(samples)? - min_amplitude(samples)?)
}

pub fn mean_amplitude(samples: &[f64]) -> Option<f64> {
    let samples = validated_samples(samples)?;
    let sum = samples.iter().sum::<f64>();

    Some(sum / samples.len() as f64)
}

pub fn rms_amplitude(samples: &[f64]) -> Option<f64> {
    let samples = validated_samples(samples)?;
    let mean_square =
        samples.iter().map(|sample| sample * sample).sum::<f64>() / samples.len() as f64;

    Some(mean_square.sqrt())
}

#[cfg(test)]
mod tests {
    use super::{
        max_amplitude, mean_amplitude, min_amplitude, peak_amplitude, peak_to_peak_amplitude,
        rms_amplitude,
    };

    #[test]
    fn computes_basic_amplitude_summaries() {
        let samples = [-2.0, -0.5, 1.0, 3.0];

        assert_eq!(peak_amplitude(&samples), Some(3.0));
        assert_eq!(min_amplitude(&samples), Some(-2.0));
        assert_eq!(max_amplitude(&samples), Some(3.0));
        assert_eq!(peak_to_peak_amplitude(&samples), Some(5.0));
        assert_eq!(mean_amplitude(&samples), Some(0.375));
    }

    #[test]
    fn computes_rms_amplitude() {
        let rms = rms_amplitude(&[-1.0, 0.0, 1.0]).unwrap();

        assert!((rms - (2.0_f64 / 3.0).sqrt()).abs() < 1.0e-12);
    }

    #[test]
    fn rejects_empty_inputs() {
        assert_eq!(peak_amplitude(&[]), None);
        assert_eq!(rms_amplitude(&[]), None);
    }

    #[test]
    fn handles_single_values() {
        assert_eq!(peak_amplitude(&[-0.5]), Some(0.5));
        assert_eq!(peak_to_peak_amplitude(&[-0.5]), Some(0.0));
        assert_eq!(mean_amplitude(&[-0.5]), Some(-0.5));
    }

    #[test]
    fn rejects_non_finite_samples() {
        assert_eq!(peak_amplitude(&[0.0, f64::NAN]), None);
        assert_eq!(mean_amplitude(&[1.0, f64::INFINITY]), None);
    }
}
