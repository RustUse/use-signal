#![forbid(unsafe_code)]
//! Thin facade for the `use-signal` workspace.
//!
//! The crate reexports the focused signal-processing crates directly so
//! consumers can opt into one dependency while still using the smaller APIs.
//!
//! # Examples
//!
//! ```rust
//! use use_signal::*;
//!
//! let sample = Sample::new(-0.25).unwrap();
//! let windowed = apply_window(&[1.0, 2.0, 3.0], WindowKind::Hann).unwrap();
//! let filtered = moving_average_filter(&windowed, 2).unwrap();
//!
//! assert_eq!(sample.abs(), 0.25);
//! assert_eq!(peak_amplitude(&filtered).unwrap(), 1.0);
//! ```

pub use use_amplitude;
pub use use_amplitude::*;
pub use use_frequency;
pub use use_frequency::*;
pub use use_sample;
pub use use_sample::*;
pub use use_signal_energy;
pub use use_signal_energy::*;
pub use use_signal_normalize;
pub use use_signal_normalize::*;
pub use use_signal_window;
pub use use_signal_window::*;
pub use use_simple_filter;
pub use use_simple_filter::*;
pub use use_zero_crossing;
pub use use_zero_crossing::*;

#[cfg(test)]
mod tests {
    use super::{
        Frequency, Sample, WindowKind, apply_window, moving_average_filter, normalize_peak,
        peak_amplitude, signal_energy, zero_crossing_count,
    };

    #[test]
    fn facade_reexports_workspace_apis() {
        let sample = Sample::new(-0.25).unwrap();
        assert_eq!(sample.abs(), 0.25);

        let frequency = Frequency::new(440.0).unwrap();
        assert!(frequency.period_seconds() > 0.0);

        let windowed = apply_window(&[1.0, 2.0, 3.0], WindowKind::Hann).unwrap();
        assert_eq!(windowed.len(), 3);

        let normalized = normalize_peak(&[-2.0, 0.0, 1.0]).unwrap();
        assert_eq!(peak_amplitude(&normalized).unwrap(), 1.0);

        assert_eq!(signal_energy(&[1.0, -1.0]).unwrap(), 2.0);
        assert_eq!(zero_crossing_count(&[-1.0, 1.0, -1.0]), 2);

        let filtered = moving_average_filter(&[1.0, 3.0, 5.0], 2).unwrap();
        assert_eq!(filtered, vec![1.0, 2.0, 4.0]);
    }
}
