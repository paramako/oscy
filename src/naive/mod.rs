#[cfg(test)]
mod tests;

use std::f32::consts::TAU;

use crate::{Oscillator, Waveform};

/// A naive oscillator with no anti-aliasing.
///
/// Generates basic waveforms using direct computation. Simple and efficient,
/// but produces aliasing artifacts for non-sine waveforms at higher frequencies.
///
/// # Example
///
/// ```
/// use oscy::{naive::NaiveOsc, Oscillator, Waveform};
///
/// // At quarter sample rate, first sample hits peak of sine wave
/// let mut osc = NaiveOsc::new(100.0, 25.0, Waveform::Sine); // 100Hz SR, 25Hz freq
/// let sample = osc.next_sample();
/// assert!((sample - 1.0).abs() < 1e-6); // sin(TAU/4) = 1.0
/// ```
pub struct NaiveOsc {
    phase: f32,
    phase_increment: f32,
    sample_rate: f32,
    waveform: Waveform,
}

impl NaiveOsc {
    /// Creates a new naive oscillator.
    pub fn new(sample_rate: f32, frequency: f32, waveform: Waveform) -> Self {
        Self {
            phase: 0.0,
            phase_increment: frequency / sample_rate,
            sample_rate,
            waveform,
        }
    }
}

impl Oscillator for NaiveOsc {
    fn set_frequency(&mut self, hz: f32) {
        self.phase_increment = hz / self.sample_rate
    }

    fn set_phase(&mut self, phase: f32) {
        self.phase = phase.fract();
    }

    fn reset(&mut self) {
        self.phase = 0.0;
    }

    fn next_sample(&mut self) -> f32 {
        self.phase += self.phase_increment;

        // subtraction only when needed.
        // cheaper than fract()/modulo every sample
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        match self.waveform {
            Waveform::Sine => (self.phase * TAU).sin(),
            Waveform::Saw => 2.0 * self.phase - 1.0,
            Waveform::Square => {
                if self.phase < 0.5 {
                    1.0
                } else {
                    -1.0
                }
            }
            Waveform::Triangle => {
                if self.phase < 0.5 {
                    4.0 * self.phase - 1.0
                } else {
                    -4.0 * self.phase + 3.0
                }
            }
        }
    }
}

impl Iterator for NaiveOsc {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        Some(self.next_sample())
    }
}
