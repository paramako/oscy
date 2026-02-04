enum NoiseType {
    White,
    Pink {
        b0: f32,
        b1: f32,
        b2: f32,
        b3: f32,
        b4: f32,
        b5: f32,
    },
    Brown {
        prev: f32,
    },
}

/// A noise generator supporting white, pink, and brown noise.
///
/// Unlike oscillators, noise generators produce aperiodic signals with no
/// defined frequency or phase. Each noise type has different spectral
/// characteristics useful for various audio applications.
///
/// # Example
///
/// ```
/// use oscy::noise::NoiseGen;
///
/// let mut noise = NoiseGen::white();
/// let sample = noise.next_sample();
/// assert!(sample >= -1.0 && sample <= 1.0);
/// ```
pub struct NoiseGen {
    noise_type: NoiseType,
}

impl NoiseGen {
    /// Creates a white noise generator.
    ///
    /// White noise has equal energy across all frequencies, producing
    /// a bright, hissing sound.
    pub fn white() -> Self {
        Self {
            noise_type: NoiseType::White,
        }
    }

    /// Creates a pink noise generator using Paul Kellet's economy method.
    ///
    /// Pink noise has equal energy per octave (power decreases at 3dB/octave),
    /// producing a more natural, balanced sound often used for audio testing.
    pub fn pink() -> Self {
        Self {
            noise_type: NoiseType::Pink {
                b0: 0.0,
                b1: 0.0,
                b2: 0.0,
                b3: 0.0,
                b4: 0.0,
                b5: 0.0,
            },
        }
    }

    /// Creates a brown (Brownian) noise generator.
    ///
    /// Brown noise has power decreasing at 6dB/octave, producing a deep,
    /// rumbling sound similar to a waterfall or strong wind.
    pub fn brown() -> Self {
        Self {
            noise_type: NoiseType::Brown { prev: 0.0 },
        }
    }

    /// Generates and returns the next sample.
    pub fn next_sample(&mut self) -> f32 {
        match &mut self.noise_type {
            NoiseType::White => fastrand::f32() * 2.0 - 1.0,

            NoiseType::Pink {
                b0,
                b1,
                b2,
                b3,
                b4,
                b5,
            } => {
                let white = fastrand::f32() * 2.0 - 1.0;
                *b0 = 0.99886 * *b0 + white * 0.0555179;
                *b1 = 0.99332 * *b1 + white * 0.0750759;
                *b2 = 0.96900 * *b2 + white * 0.1538520;
                *b3 = 0.86650 * *b3 + white * 0.3104856;
                *b4 = 0.55000 * *b4 + white * 0.5329522;
                *b5 = -0.7616 * *b5 - white * 0.0168980;
                (*b0 + *b1 + *b2 + *b3 + *b4 + *b5 + white * 0.5362) * 0.11
            }

            NoiseType::Brown { prev } => {
                let white = fastrand::f32() * 2.0 - 1.0;
                *prev = (*prev + 0.02 * white) / 1.02;
                *prev * 3.5
            }
        }
    }
}

impl Iterator for NoiseGen {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        Some(self.next_sample())
    }
}
