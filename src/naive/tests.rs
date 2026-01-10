use std::f32::consts::TAU;

use super::NaiveOsc;
use crate::{Oscillator, Waveform};

const EPSILON: f32 = 1e-6;

fn approx_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

#[test]
fn test_sine_at_known_phases() {
    // 4 samples per cycle: phases 0.25, 0.5, 0.75, 0.0
    let mut osc = NaiveOsc::new(4.0, 1.0, Waveform::Sine);

    assert!(approx_eq(osc.next_sample(), 1.0)); // sin(TAU * 0.25) = 1
    assert!(approx_eq(osc.next_sample(), 0.0)); // sin(TAU * 0.5) = 0
    assert!(approx_eq(osc.next_sample(), -1.0)); // sin(TAU * 0.75) = -1
    assert!(approx_eq(osc.next_sample(), 0.0)); // sin(TAU * 1.0) = 0
}

#[test]
fn test_saw_ramps_up() {
    let mut osc = NaiveOsc::new(4.0, 1.0, Waveform::Saw);

    // Saw: 2 * phase - 1, phases: 0.25, 0.5, 0.75, 0.0
    assert!(approx_eq(osc.next_sample(), -0.5)); // 2 * 0.25 - 1
    assert!(approx_eq(osc.next_sample(), 0.0)); // 2 * 0.5 - 1
    assert!(approx_eq(osc.next_sample(), 0.5)); // 2 * 0.75 - 1
    assert!(approx_eq(osc.next_sample(), -1.0)); // 2 * 0.0 - 1 (wrapped)
}

#[test]
fn test_square_alternates() {
    let mut osc = NaiveOsc::new(4.0, 1.0, Waveform::Square);

    // Square: 1 if phase < 0.5, else -1
    assert!(approx_eq(osc.next_sample(), 1.0)); // phase 0.25
    assert!(approx_eq(osc.next_sample(), -1.0)); // phase 0.5
    assert!(approx_eq(osc.next_sample(), -1.0)); // phase 0.75
    assert!(approx_eq(osc.next_sample(), 1.0)); // phase 0.0 (wrapped)
}

#[test]
fn test_triangle_peaks_at_midpoint() {
    let mut osc = NaiveOsc::new(4.0, 1.0, Waveform::Triangle);

    // Triangle: ramps up to 1 at phase 0.5, then down to -1
    assert!(approx_eq(osc.next_sample(), 0.0)); // 4 * 0.25 - 1 = 0
    assert!(approx_eq(osc.next_sample(), 1.0)); // -4 * 0.5 + 3 = 1
    assert!(approx_eq(osc.next_sample(), 0.0)); // -4 * 0.75 + 3 = 0
    assert!(approx_eq(osc.next_sample(), -1.0)); // 4 * 0.0 - 1 = -1
}

#[test]
fn test_set_frequency_changes_increment() {
    let mut osc = NaiveOsc::new(100.0, 10.0, Waveform::Sine);
    osc.next_sample(); // phase = 0.1

    osc.set_frequency(20.0); // now increment is 0.2
    osc.next_sample(); // phase = 0.3
    osc.next_sample(); // phase = 0.5

    assert!(approx_eq(osc.next_sample(), (0.7 * TAU).sin()));
}

#[test]
fn test_set_phase() {
    let mut osc = NaiveOsc::new(4.0, 1.0, Waveform::Sine);
    osc.set_phase(0.25);

    // After set_phase(0.25), next sample adds 0.25 -> phase 0.5
    assert!(approx_eq(osc.next_sample(), 0.0)); // sin(TAU * 0.5)
}

#[test]
fn test_reset_zeros_phase() {
    let mut osc = NaiveOsc::new(4.0, 1.0, Waveform::Sine);
    osc.next_sample();
    osc.next_sample();

    osc.reset();

    // After reset phase = 0, next_sample increments to 0.25, sin(0.25 * TAU) = 1.0
    assert!(approx_eq(osc.next_sample(), 1.0));
}

#[test]
fn test_fill_buffer() {
    let mut osc = NaiveOsc::new(4.0, 1.0, Waveform::Sine);
    let mut buffer = [0.0f32; 4];

    osc.fill(&mut buffer);

    assert!(approx_eq(buffer[0], 1.0));
    assert!(approx_eq(buffer[1], 0.0));
    assert!(approx_eq(buffer[2], -1.0));
    assert!(approx_eq(buffer[3], 0.0));
}

#[test]
fn test_iterator() {
    let osc = NaiveOsc::new(4.0, 1.0, Waveform::Sine);
    let samples: Vec<f32> = osc.take(4).collect();

    assert!(approx_eq(samples[0], 1.0));
    assert!(approx_eq(samples[1], 0.0));
    assert!(approx_eq(samples[2], -1.0));
    assert!(approx_eq(samples[3], 0.0));
}
