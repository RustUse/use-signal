# use-signal

Composable primitive signal-processing utilities for Rust.

`use-signal` is part of RustUse, alongside sibling repositories such as
`use-math`, `use-stats`, `use-optimization`, `use-simulation`,
`use-control`, `use-wave`, `use-acoustics`, `use-physics`, `use-color`,
`use-text`, `use-time`, and `use-units`. It groups small, focused crates for
signal samples, amplitudes, frequencies, windows, normalization, energy,
zero-crossing analysis, and simple filtering.

The RustUse approach in this workspace stays intentionally narrow:

- crates stay small and independently useful
- APIs stay explicit, documented, tested, and composable
- implementations favor practical `f64`, `usize`, and small enums or structs
- dependencies stay minimal so each crate is easy to audit and adopt

## Workspace crates

- `use-signal`: thin facade crate that reexports the full signal workspace
- `use-sample`: primitive sample validation and duration helpers
- `use-amplitude`: peak, min, max, mean, and RMS amplitude helpers
- `use-frequency`: finite frequency validation, period, and Nyquist helpers
- `use-signal-window`: rectangular, Hann, and Hamming window helpers
- `use-signal-normalize`: peak, range, and DC-offset normalization helpers
- `use-signal-energy`: energy, power, RMS power, and decibel helpers
- `use-zero-crossing`: zero-crossing counting and rate helpers
- `use-simple-filter`: moving-average and first-order filter helpers

## Facade crate

If you want one dependency for the whole workspace, use `use-signal`. It
reexports each focused crate and exposes the focused APIs directly so this
works:

```rust
use use_signal::*;

let sample = Sample::new(-0.25).unwrap();
let frequency = Frequency::new(440.0).unwrap();
let normalized = normalize_peak(&[-1.0, 0.5, 1.0]).unwrap();

assert_eq!(sample.abs(), 0.25);
assert!(frequency.angular_frequency() > 0.0);
assert_eq!(peak_amplitude(&normalized).unwrap(), 1.0);
```

## Status

This workspace is experimental while it remains below `0.3.0`. Expect the
public API to stay small and practical, but still evolve as the RustUse signal
surface becomes clearer.

## Development

Run the standard workspace checks from the repository root:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo doc --workspace --no-deps
```
