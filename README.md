# oscy

[![Crates.io](https://img.shields.io/crates/v/oscy)](https://crates.io/crates/oscy)
[![Docs.rs](https://docs.rs/oscy/badge.svg)](https://docs.rs/oscy)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Changelog](https://img.shields.io/badge/changelog-md-blue)](CHANGELOG.md)

A Rust library for audio oscillators, waveform generation, and noise.

## Sound sources

| Source | Description |
|--------|-------------|
| `NaiveOsc` | Simple oscillator without anti-aliasing. Fast but produces aliasing at higher frequencies. |
| `PolyBlepOsc` | Band-limited oscillator using polyBLEP for reduced aliasing. |
| `NoiseGen` | Noise generator with white, pink, and brown noise. Requires `noise` feature. |

## Usage

### Filling an audio buffer

```rust
use oscy::{poly_blep::PolyBlepOsc, Oscillator, Waveform};

let mut osc = PolyBlepOsc::new(44100.0, 440.0, Waveform::Saw);
let mut buffer = [0.0f32; 512];
osc.fill(&mut buffer);
```

### Using as an iterator

```rust
use oscy::{poly_blep::PolyBlepOsc, Waveform};

let osc = PolyBlepOsc::new(44100.0, 440.0, Waveform::Square);
let samples: Vec<f32> = osc.take(1024).collect();
```

### Naive vs PolyBLEP

Use `NaiveOsc` when performance is critical and aliasing is acceptable (e.g., low frequencies, or when followed by filtering). Use `PolyBlepOsc` for cleaner sound at higher frequencies.

```rust
use oscy::{naive::NaiveOsc, poly_blep::PolyBlepOsc, Oscillator, Waveform};

// Naive: simple and fast, but aliases
let mut naive = NaiveOsc::new(44100.0, 2000.0, Waveform::Saw);

// PolyBLEP: smooths discontinuities to reduce aliasing
let mut blep = PolyBlepOsc::new(44100.0, 2000.0, Waveform::Saw);
```

### Noise generator

Enable the `noise` feature in your `Cargo.toml`:

```toml
oscy = { version = "0.1", features = ["noise"] }
```

```rust
use oscy::noise::NoiseGen;

let mut noise = NoiseGen::pink();
let samples: Vec<f32> = noise.take(1024).collect();
```

Available types: `white()`, `pink()`, `brown()`.

## Supported waveforms

- Sine
- Saw
- Square
- Triangle

## License

MIT License - see [LICENSE](LICENSE) for details.
