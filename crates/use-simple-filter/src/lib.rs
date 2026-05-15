#![forbid(unsafe_code)]
//! Primitive filtering helpers.
//!
//! The crate intentionally keeps filtering narrow: a causal moving average plus
//! first-order low-pass and high-pass filters.
//!
//! # Examples
//!
//! ```rust
//! use use_simple_filter::{first_order_low_pass, moving_average_filter};
//!
//! assert_eq!(moving_average_filter(&[1.0, 3.0, 5.0], 2).unwrap(), vec![1.0, 2.0, 4.0]);
//! assert_eq!(first_order_low_pass(&[0.0, 1.0, 1.0], 0.5).unwrap(), vec![0.0, 0.5, 0.75]);
//! ```

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterError {
    EmptySamples,
    InvalidSample,
    InvalidWindowSize,
    InvalidAlpha,
}

fn validate_samples(samples: &[f64]) -> Result<(), FilterError> {
    if samples.is_empty() {
        return Err(FilterError::EmptySamples);
    }

    if samples.iter().any(|sample| !sample.is_finite()) {
        return Err(FilterError::InvalidSample);
    }

    Ok(())
}

fn validate_alpha(alpha: f64) -> Result<(), FilterError> {
    if !alpha.is_finite() || !(0.0..=1.0).contains(&alpha) {
        Err(FilterError::InvalidAlpha)
    } else {
        Ok(())
    }
}

pub fn moving_average_filter(samples: &[f64], window_size: usize) -> Result<Vec<f64>, FilterError> {
    validate_samples(samples)?;

    if window_size == 0 {
        return Err(FilterError::InvalidWindowSize);
    }

    let mut running_sum = 0.0;
    let mut output = Vec::with_capacity(samples.len());

    for (index, sample) in samples.iter().copied().enumerate() {
        running_sum += sample;

        if index >= window_size {
            running_sum -= samples[index - window_size];
        }

        let divisor = usize::min(index + 1, window_size) as f64;
        output.push(running_sum / divisor);
    }

    Ok(output)
}

pub fn first_order_low_pass(samples: &[f64], alpha: f64) -> Result<Vec<f64>, FilterError> {
    validate_samples(samples)?;
    validate_alpha(alpha)?;

    let mut output = Vec::with_capacity(samples.len());
    let mut previous = samples[0];
    output.push(previous);

    for sample in &samples[1..] {
        previous = alpha * sample + (1.0 - alpha) * previous;
        output.push(previous);
    }

    Ok(output)
}

pub fn first_order_high_pass(samples: &[f64], alpha: f64) -> Result<Vec<f64>, FilterError> {
    validate_samples(samples)?;
    validate_alpha(alpha)?;

    let mut output = Vec::with_capacity(samples.len());
    let mut previous_input = samples[0];
    let mut previous_output = samples[0];
    output.push(previous_output);

    for sample in &samples[1..] {
        let current = alpha * (previous_output + sample - previous_input);
        output.push(current);
        previous_input = *sample;
        previous_output = current;
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::{FilterError, first_order_high_pass, first_order_low_pass, moving_average_filter};

    #[test]
    fn applies_moving_average_filter() {
        assert_eq!(
            moving_average_filter(&[1.0, 3.0, 5.0, 7.0], 2).unwrap(),
            vec![1.0, 2.0, 4.0, 6.0]
        );
    }

    #[test]
    fn applies_first_order_low_pass_filter() {
        assert_eq!(
            first_order_low_pass(&[0.0, 1.0, 1.0], 0.5).unwrap(),
            vec![0.0, 0.5, 0.75]
        );
    }

    #[test]
    fn applies_first_order_high_pass_filter() {
        let filtered = first_order_high_pass(&[1.0, 1.0, 1.0], 0.5).unwrap();

        assert_eq!(filtered, vec![1.0, 0.5, 0.25]);
    }

    #[test]
    fn rejects_invalid_inputs() {
        assert_eq!(
            moving_average_filter(&[], 2),
            Err(FilterError::EmptySamples)
        );
        assert_eq!(
            moving_average_filter(&[1.0], 0),
            Err(FilterError::InvalidWindowSize)
        );
        assert_eq!(
            first_order_low_pass(&[1.0, f64::NAN], 0.5),
            Err(FilterError::InvalidSample)
        );
        assert_eq!(
            first_order_high_pass(&[1.0], 1.5),
            Err(FilterError::InvalidAlpha)
        );
    }

    #[test]
    fn handles_single_value_inputs() {
        assert_eq!(moving_average_filter(&[2.0], 4).unwrap(), vec![2.0]);
        assert_eq!(first_order_low_pass(&[2.0], 0.2).unwrap(), vec![2.0]);
        assert_eq!(first_order_high_pass(&[2.0], 0.2).unwrap(), vec![2.0]);
    }
}
