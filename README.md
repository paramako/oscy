# oscy

[![Crates.io](https://img.shields.io/crates/v/oscy)](https://crates.io/crates/oscy)
[![Docs.rs](https://docs.rs/oscy/badge.svg)](https://docs.rs/oscy)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Changelog](https://img.shields.io/badge/changelog-md-blue)](CHANGELOG.md)

A Rust library for audio oscillators and waveform generation.

## Oscillators

| Oscillator | Status | Description |
|------------|--------|-------------|
| `NaiveOsc` | Implemented | Simple oscillator without anti-aliasing. Fast but produces aliasing at higher frequencies. |
| `PolyBLEPOsc` | Coming soon | Band-limited oscillator using PolyBLEP for reduced aliasing. |
| More | Coming soon | Additional oscillator implementations planned. |

## Usage

```rust
use oscy::{naive::NaiveOsc, Oscillator, Waveform};

let mut osc = NaiveOsc::new(44100.0, 440.0, Waveform::Sine);
let sample = osc.next_sample();
```

## Supported waveforms

- Sine
- Saw
- Square
- Triangle

## License

MIT License - see [LICENSE](LICENSE) for details.
