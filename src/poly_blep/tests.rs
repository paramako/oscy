use std::f32::consts::TAU;

use super::PolyBlepOsc;
use crate::{Oscillator, Waveform, naive::NaiveOsc};

const EPSILON: f32 = 1e-6;

fn approx_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

#[test]
fn test_poly_blep_zero_outside_discontinuity() {
    let osc = PolyBlepOsc::new(10.0, 1.0, Waveform::Saw); // inc = 0.1

    // Phase values far from discontinuity should return 0
    assert!(approx_eq(osc.poly_blep(0.5), 0.0));
    assert!(approx_eq(osc.poly_blep(0.3), 0.0));
    assert!(approx_eq(osc.poly_blep(0.7), 0.0));
}

#[test]
fn test_poly_blep_nonzero_near_discontinuity() {
    let osc = PolyBlepOsc::new(10.0, 1.0, Waveform::Saw); // inc = 0.1

    // Just after discontinuity (phase < inc)
    let blep_after = osc.poly_blep(0.05);
    assert!(blep_after < 0.0); // Should be negative

    // Just before discontinuity (phase > 1 - inc)
    let blep_before = osc.poly_blep(0.95);
    assert!(blep_before > 0.0); // Should be positive
}

#[test]
fn test_poly_blep_boundary_values() {
    let osc = PolyBlepOsc::new(10.0, 1.0, Waveform::Saw); // inc = 0.1

    // At t=0 (phase=0): blep = 2*0 - 0 - 1 = -1
    assert!(approx_eq(osc.poly_blep(0.0), -1.0));

    // At t=1 (phase=inc): blep = 2*1 - 1 - 1 = 0 (boundary, actually 0 from else)
    // Phase exactly at inc is not < inc, so returns 0
    assert!(approx_eq(osc.poly_blep(0.1), 0.0));

    // At t=-1 (phase=1-inc): blep = 1 - 2 + 1 = 0 (boundary)
    // Phase exactly at 1-inc is not > 1-inc, so returns 0
    assert!(approx_eq(osc.poly_blep(0.9), 0.0));
}

#[test]
fn test_poly_blep_midpoint_values() {
    let osc = PolyBlepOsc::new(10.0, 1.0, Waveform::Saw); // inc = 0.1

    // At t=0.5 (phase=0.05): blep = 2*0.5 - 0.25 - 1 = -0.25
    assert!(approx_eq(osc.poly_blep(0.05), -0.25));

    // At t=-0.5 (phase=0.95): blep = 0.25 - 1 + 1 = 0.25
    assert!(approx_eq(osc.poly_blep(0.95), 0.25));
}

// Sine wave tests (no polyBLEP applied, same as naive)

#[test]
fn test_sine_at_known_phases() {
    let mut osc = PolyBlepOsc::new(4.0, 1.0, Waveform::Sine);

    assert!(approx_eq(osc.next_sample(), 1.0)); // sin(TAU * 0.25)
    assert!(approx_eq(osc.next_sample(), 0.0)); // sin(TAU * 0.5)
    assert!(approx_eq(osc.next_sample(), -1.0)); // sin(TAU * 0.75)
    assert!(approx_eq(osc.next_sample(), 0.0)); // sin(TAU * 1.0)
}

#[test]
fn test_sine_matches_naive() {
    let mut poly = PolyBlepOsc::new(100.0, 5.0, Waveform::Sine);
    let mut naive = NaiveOsc::new(100.0, 5.0, Waveform::Sine);

    for _ in 0..100 {
        assert!(approx_eq(poly.next_sample(), naive.next_sample()));
    }
}

// Saw wave tests

#[test]
fn test_saw_output_range() {
    let mut osc = PolyBlepOsc::new(44100.0, 440.0, Waveform::Saw);

    for _ in 0..1000 {
        let sample = osc.next_sample();
        assert!(sample >= -1.0 && sample <= 1.0);
    }
}

#[test]
fn test_saw_smooths_discontinuity() {
    // Use 16 samples per cycle (inc = 0.0625) to avoid hitting boundaries exactly
    let mut poly = PolyBlepOsc::new(16.0, 1.0, Waveform::Saw);
    let mut naive = NaiveOsc::new(16.0, 1.0, Waveform::Saw);

    let poly_samples: Vec<f32> = (0..16).map(|_| poly.next_sample()).collect();
    let naive_samples: Vec<f32> = (0..16).map(|_| naive.next_sample()).collect();

    // Samples away from discontinuity should be similar
    // (middle of the ramp, e.g., sample 8 at phase 0.5625)
    assert!((poly_samples[8] - naive_samples[8]).abs() < 0.05);

    // Sample 15 wraps to phase 0.0 (just after discontinuity) - should be smoothed
    assert!((poly_samples[15] - naive_samples[15]).abs() > 0.1);

    // Sample 14 is at phase 0.9375 (> 1 - 0.0625 = 0.9375)... still boundary
    // So we only check sample 15 which definitely wraps
}

#[test]
fn test_saw_ramp_direction() {
    let mut osc = PolyBlepOsc::new(100.0, 1.0, Waveform::Saw);

    // Skip first few samples near discontinuity
    for _ in 0..10 {
        osc.next_sample();
    }

    // Away from discontinuity, saw should ramp upward
    let s1 = osc.next_sample();
    let s2 = osc.next_sample();
    let s3 = osc.next_sample();

    assert!(s2 > s1);
    assert!(s3 > s2);
}

// Square wave tests

#[test]
fn test_square_output_range() {
    let mut osc = PolyBlepOsc::new(44100.0, 440.0, Waveform::Square);

    for _ in 0..1000 {
        let sample = osc.next_sample();
        assert!(sample >= -1.0 && sample <= 1.0);
    }
}

#[test]
fn test_square_smooths_both_edges() {
    // With 20 samples per cycle (inc = 0.05), we can observe the smoothing
    let mut poly = PolyBlepOsc::new(20.0, 1.0, Waveform::Square);
    let mut naive = NaiveOsc::new(20.0, 1.0, Waveform::Square);

    let poly_samples: Vec<f32> = (0..20).map(|_| poly.next_sample()).collect();
    let naive_samples: Vec<f32> = (0..20).map(|_| naive.next_sample()).collect();

    // Rising edge at phase 0: sample 19 wraps to phase 0.0, should be smoothed
    assert!((poly_samples[19] - naive_samples[19]).abs() > 0.1);

    // Falling edge at phase 0.5: sample 9 is at phase 0.5, sample 10 is at 0.55
    // Sample 9 (phase 0.5) and sample 10 (phase 0.55) should show smoothing
    assert!((poly_samples[9] - naive_samples[9]).abs() > 0.1);
}

#[test]
fn test_square_high_low_regions() {
    // With many samples, verify square spends time in high and low regions
    let mut osc = PolyBlepOsc::new(100.0, 1.0, Waveform::Square);

    let samples: Vec<f32> = (0..100).map(|_| osc.next_sample()).collect();

    let high_count = samples.iter().filter(|&&s| s > 0.5).count();
    let low_count = samples.iter().filter(|&&s| s < -0.5).count();

    // Should spend roughly equal time high and low (50% duty cycle)
    assert!(high_count > 30 && high_count < 70);
    assert!(low_count > 30 && low_count < 70);
}

// Triangle wave tests (no polyBLEP, same as naive)

#[test]
fn test_triangle_peaks_at_midpoint() {
    let mut osc = PolyBlepOsc::new(4.0, 1.0, Waveform::Triangle);

    assert!(approx_eq(osc.next_sample(), 0.0)); // 4 * 0.25 - 1
    assert!(approx_eq(osc.next_sample(), 1.0)); // -4 * 0.5 + 3
    assert!(approx_eq(osc.next_sample(), 0.0)); // -4 * 0.75 + 3
    assert!(approx_eq(osc.next_sample(), -1.0)); // 4 * 0.0 - 1
}

#[test]
fn test_triangle_matches_naive() {
    let mut poly = PolyBlepOsc::new(100.0, 5.0, Waveform::Triangle);
    let mut naive = NaiveOsc::new(100.0, 5.0, Waveform::Triangle);

    for _ in 0..100 {
        assert!(approx_eq(poly.next_sample(), naive.next_sample()));
    }
}

// trait impl tests

#[test]
fn test_set_frequency_changes_increment() {
    let mut osc = PolyBlepOsc::new(100.0, 10.0, Waveform::Sine);
    osc.next_sample(); // phase = 0.1

    osc.set_frequency(20.0); // now increment is 0.2
    osc.next_sample(); // phase = 0.3
    osc.next_sample(); // phase = 0.5

    assert!(approx_eq(osc.next_sample(), (0.7 * TAU).sin()));
}

#[test]
fn test_set_phase() {
    let mut osc = PolyBlepOsc::new(4.0, 1.0, Waveform::Sine);
    osc.set_phase(0.25);

    // After set_phase(0.25), next sample adds 0.25 -> phase 0.5
    assert!(approx_eq(osc.next_sample(), 0.0)); // sin(TAU * 0.5)
}

#[test]
fn test_reset_zeros_phase() {
    let mut osc = PolyBlepOsc::new(4.0, 1.0, Waveform::Sine);
    osc.next_sample();
    osc.next_sample();

    osc.reset();

    assert!(approx_eq(osc.next_sample(), 1.0)); // sin(TAU * 0.25)
}

#[test]
fn test_fill_buffer() {
    let mut osc = PolyBlepOsc::new(4.0, 1.0, Waveform::Sine);
    let mut buffer = [0.0f32; 4];

    osc.fill(&mut buffer);

    assert!(approx_eq(buffer[0], 1.0));
    assert!(approx_eq(buffer[1], 0.0));
    assert!(approx_eq(buffer[2], -1.0));
    assert!(approx_eq(buffer[3], 0.0));
}

#[test]
fn test_iterator() {
    let osc = PolyBlepOsc::new(4.0, 1.0, Waveform::Sine);
    let samples: Vec<f32> = osc.take(4).collect();

    assert!(approx_eq(samples[0], 1.0));
    assert!(approx_eq(samples[1], 0.0));
    assert!(approx_eq(samples[2], -1.0));
    assert!(approx_eq(samples[3], 0.0));
}
