#!/usr/bin/env rust

//! Renders the beep the metronome played through mnomer into
//! `assets/sounds/beep.wav`, with the same tone, filters and fades. The accent
//! is this file played 5 semitones up, so only 1 file is needed.

use std::{f64::consts::PI, fs::write};

const SAMPLE_RATE: f64 = 48000.0;
const FREQUENCY: f64 = 440.0;
const LENGTH: f64 = 0.05;
const AMPLITUDE: f64 = 0.75;
const FADE: f64 = 0.01;

fn main() {
    let count = (LENGTH * SAMPLE_RATE).round() as usize;
    let mut signal: Vec<f64> = (0..count)
        .map(|i| {
            let x = i as f64 * 2.0 * PI / SAMPLE_RATE;
            AMPLITUDE * (x * FREQUENCY).sin() + 0.5 * AMPLITUDE * (x * 2.0 * FREQUENCY).sin()
        })
        .collect();

    filter(&mut signal, 1.001852916, [1.0, -2.0, 1.0], -0.9963044430, 1.9962976018);
    filter(&mut signal, 1.450734152, [1.0, 2.0, 1.0], -0.4775922501, -1.2796324250);
    fade(&mut signal);

    write("assets/sounds/beep.wav", wav(&signal)).expect("write assets/sounds/beep.wav");
    println!("wrote assets/sounds/beep.wav, {count} samples");
}

/// The 2nd order Butterworth filters of mnomer, made by mkfilter.
fn filter(signal: &mut [f64], gain: f64, taps: [f64; 3], y0: f64, y1: f64) {
    let mut xv = [0.0; 3];
    let mut yv = [0.0; 3];
    for sample in signal.iter_mut() {
        xv = [xv[1], xv[2], *sample / gain];
        let y = taps[0] * xv[0] + taps[1] * xv[1] + taps[2] * xv[2] + y0 * yv[1] + y1 * yv[2];
        yv = [yv[1], yv[2], y];
        *sample = y;
    }
}

/// The exponential fade of mnomer, from 1 / i16::MAX up to 1 and back.
fn fade(signal: &mut [f64]) {
    let start = 1.0 / f64::from(i16::MAX);
    let steps = (FADE * SAMPLE_RATE).round() as usize;
    let ratio = (1.0 / start).powf(1.0 / steps as f64);
    let len = signal.len();

    let mut factor = start;
    for sample in &mut signal[..steps] {
        *sample *= factor;
        factor *= ratio;
    }
    let mut factor = 1.0;
    for sample in &mut signal[len - steps..] {
        *sample *= factor;
        factor /= ratio;
    }
}

/// Mono 16 bit PCM. A sample over 1 is cut, the way the output device cut it.
fn wav(signal: &[f64]) -> Vec<u8> {
    let data_len = (signal.len() * 2) as u32;
    let rate = SAMPLE_RATE as u32;
    let mut out = Vec::new();
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data_len).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&rate.to_le_bytes());
    out.extend_from_slice(&(rate * 2).to_le_bytes());
    out.extend_from_slice(&2u16.to_le_bytes());
    out.extend_from_slice(&16u16.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_len.to_le_bytes());
    for sample in signal {
        let value = (sample.clamp(-1.0, 1.0) * f64::from(i16::MAX)).round() as i16;
        out.extend_from_slice(&value.to_le_bytes());
    }
    out
}
