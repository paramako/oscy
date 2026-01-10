#[cfg(test)]
mod tests;

use std::f32::consts::TAU;

use crate::{Oscillator, Waveform};

/// A bandlimited oscillator using polynomial bandlimited step (polyBLEP).
///
/// PolyBLEP reduces aliasing by applying polynomial corrections around
/// waveform discontinuities. This produces cleaner sound than naive
/// oscillators, especially at higher frequencies.
///
/// Note: Triangle waves have no step discontinuities, only derivative
/// discontinuities, so they don't benefit from polyBLEP correction.
///
/// # Example
///
/// ```
/// use oscy::{poly_blep::PolyBlepOsc, Oscillator, Waveform};
///
/// let mut osc = PolyBlepOsc::new(44100.0, 440.0, Waveform::Saw);
/// let sample = osc.next_sample();
/// assert!(sample >= -1.0 && sample <= 1.0);
/// ```
pub struct PolyBlepOsc {
    phase: f32,
    phase_increment: f32,
    sample_rate: f32,
    waveform: Waveform,
}

impl PolyBlepOsc {
    /// Creates a new polyBLEP oscillator.
    pub fn new(sample_rate: f32, frequency: f32, waveform: Waveform) -> Self {
        Self {
            phase: 0.0,
            phase_increment: frequency / sample_rate,
            sample_rate,
            waveform,
        }
    }

    /// Computes the polyBLEP correction for a given phase.
    ///
    /// Returns a correction value to smooth discontinuities:
    /// - Non-zero when phase is within one sample of a discontinuity
    /// - Zero otherwise
    ///
    /// The correction uses a 2-sample polynomial residual that integrates
    /// to zero, preserving the waveform's DC offset.
    pub fn poly_blep(&self, phase: f32) -> f32 {
        // Just after discontinuity: phase in (0, phase_increment)
        if phase < self.phase_increment {
            let t = phase / self.phase_increment;
            return 2.0 * t - t * t - 1.0;
        }

        // Just before discontinuity: phase in (1 - phase_increment, 1)
        if phase > 1.0 - self.phase_increment {
            let t = (phase - 1.0) / self.phase_increment;
            return t * t + 2.0 * t + 1.0;
        }

        0.0
    }
}

impl Oscillator for PolyBlepOsc {
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
            Waveform::Saw => {
                let naive = 2.0 * self.phase - 1.0;
                naive - self.poly_blep(self.phase)
            }
            Waveform::Square => {
                let naive = if self.phase < 0.5 { 1.0 } else { -1.0 };
                naive + self.poly_blep(self.phase) - self.poly_blep((self.phase + 0.5).fract())
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

impl Iterator for PolyBlepOsc {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        Some(self.next_sample())
    }
}
