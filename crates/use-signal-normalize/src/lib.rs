#![forbid(unsafe_code)]
//! Primitive signal-normalization helpers.
//!
//! The crate provides small, explicit helpers for peak normalization, mapping a
//! signal into a target range, and removing DC offset.
//!
//! # Examples
//!
//! ```rust
//! use use_signal_normalize::{center_signal, normalize_peak, normalize_range};
//!
//! let normalized = normalize_peak(&[-2.0, 0.0, 1.0]).unwrap();
//! let centered = center_signal(&[1.0, 2.0, 3.0]).unwrap();
//! let ranged = normalize_range(&[0.0, 1.0], -1.0, 1.0).unwrap();
//!
//! assert_eq!(normalized, vec![-1.0, 0.0, 0.5]);
//! assert_eq!(centered, vec![-1.0, 0.0, 1.0]);
//! assert_eq!(ranged, vec![-1.0, 1.0]);
//! ```

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalizeError {
    EmptySamples,
    InvalidSample,
    InvalidRange,
}

fn validated_samples(samples: &[f64]) -> Result<&[f64], NormalizeError> {
    if samples.is_empty() {
        return Err(NormalizeError::EmptySamples);
    }

    if samples.iter().any(|sample| !sample.is_finite()) {
        return Err(NormalizeError::InvalidSample);
    }

    Ok(samples)
}

fn sample_min_max(samples: &[f64]) -> (f64, f64) {
    let mut values = samples.iter().copied();
    let first = values.next().unwrap();

    values.fold((first, first), |(min, max), sample| {
        (min.min(sample), max.max(sample))
    })
}

pub fn normalize_peak(samples: &[f64]) -> Option<Vec<f64>> {
    let samples = validated_samples(samples).ok()?;
    let peak = samples
        .iter()
        .map(|sample| sample.abs())
        .fold(0.0, f64::max);

    if peak == 0.0 {
        return Some(samples.to_vec());
    }

    Some(samples.iter().map(|sample| sample / peak).collect())
}

pub fn normalize_range(samples: &[f64], min: f64, max: f64) -> Result<Vec<f64>, NormalizeError> {
    let samples = validated_samples(samples)?;

    if !min.is_finite() || !max.is_finite() || min >= max {
        return Err(NormalizeError::InvalidRange);
    }

    let (source_min, source_max) = sample_min_max(samples);
    let source_span = source_max - source_min;

    if source_span == 0.0 {
        let midpoint = (min + max) / 2.0;
        return Ok(vec![midpoint; samples.len()]);
    }

    let target_span = max - min;

    Ok(samples
        .iter()
        .map(|sample| min + ((sample - source_min) / source_span) * target_span)
        .collect())
}

pub fn center_signal(samples: &[f64]) -> Option<Vec<f64>> {
    let samples = validated_samples(samples).ok()?;
    let mean = samples.iter().sum::<f64>() / samples.len() as f64;

    Some(samples.iter().map(|sample| sample - mean).collect())
}

pub fn remove_dc_offset(samples: &[f64]) -> Option<Vec<f64>> {
    center_signal(samples)
}

#[cfg(test)]
mod tests {
    use super::{NormalizeError, center_signal, normalize_peak, normalize_range, remove_dc_offset};

    #[test]
    fn normalizes_peak_values() {
        assert_eq!(
            normalize_peak(&[-2.0, 0.0, 1.0]),
            Some(vec![-1.0, 0.0, 0.5])
        );
        assert_eq!(normalize_peak(&[0.0, 0.0]), Some(vec![0.0, 0.0]));
    }

    #[test]
    fn normalizes_into_target_ranges() {
        assert_eq!(
            normalize_range(&[0.0, 1.0], -1.0, 1.0).unwrap(),
            vec![-1.0, 1.0]
        );
        assert_eq!(
            normalize_range(&[5.0, 5.0], -1.0, 1.0).unwrap(),
            vec![0.0, 0.0]
        );
    }

    #[test]
    fn centers_signals_and_removes_dc_offset() {
        assert_eq!(center_signal(&[1.0, 2.0, 3.0]), Some(vec![-1.0, 0.0, 1.0]));
        assert_eq!(
            remove_dc_offset(&[1.0, 2.0, 3.0]),
            Some(vec![-1.0, 0.0, 1.0])
        );
    }

    #[test]
    fn rejects_empty_and_invalid_samples() {
        assert_eq!(normalize_peak(&[]), None);
        assert_eq!(center_signal(&[1.0, f64::NAN]), None);
        assert_eq!(
            normalize_range(&[], -1.0, 1.0),
            Err(NormalizeError::EmptySamples)
        );
        assert_eq!(
            normalize_range(&[1.0, f64::INFINITY], -1.0, 1.0),
            Err(NormalizeError::InvalidSample)
        );
    }

    #[test]
    fn rejects_invalid_target_ranges() {
        assert_eq!(
            normalize_range(&[0.0, 1.0], 1.0, 1.0),
            Err(NormalizeError::InvalidRange)
        );
        assert_eq!(
            normalize_range(&[0.0, 1.0], f64::NAN, 1.0),
            Err(NormalizeError::InvalidRange)
        );
    }
}
