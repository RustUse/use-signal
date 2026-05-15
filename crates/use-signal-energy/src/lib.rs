#![forbid(unsafe_code)]
//! Primitive signal-energy helpers.
//!
//! The crate provides a few explicit helpers for energy, mean power, RMS power,
//! and decibel conversions.
//!
//! # Examples
//!
//! ```rust
//! use use_signal_energy::{decibels_from_amplitude, mean_power, signal_energy};
//!
//! let samples = [1.0, -1.0, 1.0, -1.0];
//!
//! assert_eq!(signal_energy(&samples), Some(4.0));
//! assert_eq!(mean_power(&samples), Some(1.0));
//! assert_eq!(decibels_from_amplitude(2.0, 1.0).unwrap(), 20.0 * 2.0_f64.log10());
//! ```

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnergyError {
    InvalidPower,
    InvalidReferencePower,
    InvalidAmplitude,
    InvalidReferenceAmplitude,
}

fn validated_samples(samples: &[f64]) -> Option<&[f64]> {
    if samples.is_empty() || samples.iter().any(|sample| !sample.is_finite()) {
        None
    } else {
        Some(samples)
    }
}

pub fn signal_energy(samples: &[f64]) -> Option<f64> {
    Some(
        validated_samples(samples)?
            .iter()
            .map(|sample| sample * sample)
            .sum(),
    )
}

pub fn mean_power(samples: &[f64]) -> Option<f64> {
    let samples = validated_samples(samples)?;
    Some(signal_energy(samples)? / samples.len() as f64)
}

pub fn rms_power(samples: &[f64]) -> Option<f64> {
    Some(mean_power(samples)?.sqrt())
}

pub fn decibels_from_power(power: f64, reference_power: f64) -> Result<f64, EnergyError> {
    if !power.is_finite() || power <= 0.0 {
        return Err(EnergyError::InvalidPower);
    }

    if !reference_power.is_finite() || reference_power <= 0.0 {
        return Err(EnergyError::InvalidReferencePower);
    }

    Ok(10.0 * (power / reference_power).log10())
}

pub fn decibels_from_amplitude(
    amplitude: f64,
    reference_amplitude: f64,
) -> Result<f64, EnergyError> {
    if !amplitude.is_finite() || amplitude <= 0.0 {
        return Err(EnergyError::InvalidAmplitude);
    }

    if !reference_amplitude.is_finite() || reference_amplitude <= 0.0 {
        return Err(EnergyError::InvalidReferenceAmplitude);
    }

    Ok(20.0 * (amplitude / reference_amplitude).log10())
}

#[cfg(test)]
mod tests {
    use super::{
        EnergyError, decibels_from_amplitude, decibels_from_power, mean_power, rms_power,
        signal_energy,
    };

    #[test]
    fn computes_energy_and_power_helpers() {
        let samples = [1.0, -1.0, 1.0, -1.0];

        assert_eq!(signal_energy(&samples), Some(4.0));
        assert_eq!(mean_power(&samples), Some(1.0));
        assert_eq!(rms_power(&samples), Some(1.0));
    }

    #[test]
    fn computes_decibel_conversions() {
        assert_eq!(decibels_from_power(10.0, 1.0).unwrap(), 10.0);
        assert!(
            (decibels_from_amplitude(2.0, 1.0).unwrap() - 6.020_599_913_279_624).abs() < 1.0e-12
        );
    }

    #[test]
    fn rejects_empty_and_invalid_samples() {
        assert_eq!(signal_energy(&[]), None);
        assert_eq!(mean_power(&[1.0, f64::NAN]), None);
    }

    #[test]
    fn rejects_invalid_decibel_inputs() {
        assert_eq!(
            decibels_from_power(0.0, 1.0),
            Err(EnergyError::InvalidPower)
        );
        assert_eq!(
            decibels_from_power(1.0, 0.0),
            Err(EnergyError::InvalidReferencePower)
        );
        assert_eq!(
            decibels_from_amplitude(0.0, 1.0),
            Err(EnergyError::InvalidAmplitude)
        );
        assert_eq!(
            decibels_from_amplitude(1.0, f64::INFINITY),
            Err(EnergyError::InvalidReferenceAmplitude)
        );
    }
}
