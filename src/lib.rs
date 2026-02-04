//! A library for audio oscillators, waveform generation, and noise.
//!
//! Provides the [`Oscillator`] trait for building audio oscillators, along with
//! ready-to-use implementations in [`naive`] and [`poly_blep`]. For noise
//! generation, see the [`noise`] module (requires the `noise` feature).

/// Naive oscillator implementations without anti-aliasing.
///
/// These are simple and efficient but will produce aliasing artifacts
/// for non-sine waveforms at higher frequencies.
pub mod naive;

/// Bandlimited oscillators using polynomial bandlimited step (polyBLEP).
///
/// PolyBLEP reduces aliasing by applying polynomial corrections to samples
/// near waveform discontinuities. When a waveform has a sharp transition
/// (like the reset in a sawtooth or edges in a square wave), the correction
/// smooths samples within one sample period of the discontinuity, reducing
/// high-frequency artifacts.
pub mod poly_blep;

/// Noise generators for white, pink, and brown noise.
///
/// Noise generators produce aperiodic signals with different spectral
/// characteristics. Unlike oscillators, they have no frequency or phase.
#[cfg(feature = "noise")]
pub mod noise;

/// A trait for audio oscillators that generate periodic waveforms.
pub trait Oscillator {
    /// Sets the oscillator frequency in hertz.
    fn set_frequency(&mut self, hz: f32);

    /// Sets the current phase of the oscillator.
    ///
    /// Phase is typically in the range [0.0, 1.0], where 1.0 represents
    /// a full cycle.
    fn set_phase(&mut self, phase: f32);

    /// Resets the oscillator to its initial state.
    fn reset(&mut self);

    /// Generates and returns the next sample.
    fn next_sample(&mut self) -> f32;

    /// Fills a buffer with consecutive samples.
    fn fill(&mut self, buffer: &mut [f32]) {
        for sample in buffer.iter_mut() {
            *sample = self.next_sample();
        }
    }
}

/// Standard waveform shapes for oscillators.
pub enum Waveform {
    /// A pure sinusoidal wave. Produces no harmonics.
    Sine,
    /// A sawtooth wave with a linear ramp up and instant reset.
    Saw,
    /// A square wave alternating between high and low values.
    Square,
    /// A triangle wave with linear slopes in both directions.
    Triangle,
}
