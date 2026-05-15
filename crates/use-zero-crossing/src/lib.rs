#![forbid(unsafe_code)]
//! Primitive zero-crossing helpers.
//!
//! Exact zero values are treated as neutral samples: they do not create a
//! crossing on their own, but they also do not break the sign continuity
//! between surrounding non-zero finite samples.
//!
//! # Examples
//!
//! ```rust
//! use use_zero_crossing::{crosses_zero, zero_crossing_count, zero_crossing_rate};
//!
//! let samples = [-1.0, 0.0, 1.0, -1.0];
//!
//! assert!(crosses_zero(-1.0, 1.0));
//! assert_eq!(zero_crossing_count(&samples), 2);
//! assert_eq!(zero_crossing_rate(&samples), Some(2.0 / 3.0));
//! ```

fn non_zero_sign(sample: f64) -> Option<i8> {
    if !sample.is_finite() || sample == 0.0 {
        None
    } else if sample.is_sign_positive() {
        Some(1)
    } else {
        Some(-1)
    }
}

#[must_use]
pub fn crosses_zero(previous: f64, current: f64) -> bool {
    previous.is_finite()
        && current.is_finite()
        && ((previous < 0.0 && current > 0.0) || (previous > 0.0 && current < 0.0))
}

#[must_use]
pub fn zero_crossing_count(samples: &[f64]) -> usize {
    let mut previous_sign = None;
    let mut count = 0;

    for sample in samples.iter().copied() {
        match non_zero_sign(sample) {
            Some(sign) => {
                if let Some(previous) = previous_sign && previous != sign {
                    count += 1;
                }

                previous_sign = Some(sign);
            }
            None if !sample.is_finite() => previous_sign = None,
            None => {}
        }
    }

    count
}

pub fn zero_crossing_rate(samples: &[f64]) -> Option<f64> {
    if samples.len() < 2 || samples.iter().any(|sample| !sample.is_finite()) {
        return None;
    }

    Some(zero_crossing_count(samples) as f64 / (samples.len() - 1) as f64)
}

#[cfg(test)]
mod tests {
    use super::{crosses_zero, zero_crossing_count, zero_crossing_rate};

    #[test]
    fn detects_direct_crossings() {
        assert!(crosses_zero(-1.0, 1.0));
        assert!(crosses_zero(1.0, -1.0));
        assert!(!crosses_zero(-1.0, 0.0));
        assert!(!crosses_zero(0.0, 1.0));
    }

    #[test]
    fn counts_crossings_while_skipping_exact_zero_samples() {
        assert_eq!(zero_crossing_count(&[-1.0, 0.0, 1.0, 0.0, -1.0]), 2);
        assert_eq!(zero_crossing_count(&[1.0]), 0);
    }

    #[test]
    fn computes_zero_crossing_rate() {
        assert_eq!(zero_crossing_rate(&[-1.0, 0.0, 1.0, -1.0]), Some(2.0 / 3.0));
    }

    #[test]
    fn rejects_invalid_rate_inputs() {
        assert_eq!(zero_crossing_rate(&[]), None);
        assert_eq!(zero_crossing_rate(&[1.0]), None);
        assert_eq!(zero_crossing_rate(&[1.0, f64::NAN]), None);
    }
}
