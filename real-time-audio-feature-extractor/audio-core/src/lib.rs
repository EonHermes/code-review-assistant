//! Real-time Audio Feature Extractor
//!
//! This library provides real-time audio analysis capabilities compiled to WebAssembly.
//! Features include:
//! - BPM (beats per minute) detection using autocorrelation
//! - Musical key detection using chroma features
//! - Spectral feature extraction (centroid, rolloff, flux, flatness)
//! - Real-time streaming analysis via Web Audio API
//!
//! # Example
//!
//! ```rust
//! use audio_core::{AudioAnalyzer, AudioFeatures};
//!
//! let mut analyzer = AudioAnalyzer::new(44100);
//! let features = analyzer.analyze_chunk(&samples);
//! ```

#![allow(clippy::missing_safety_doc)]
#![allow(clippy::unnecessary_cast)]

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

mod chroma;
mod spectral;
mod tempo;

pub use chroma::*;
pub use spectral::*;
pub use tempo::*;

/// Primary audio analyzer struct
#[wasm_bindgen]
pub struct AudioAnalyzer {
    sample_rate: f32,
    hop_size: usize,
    buffer: Vec<f32>,
    tempo_detector: TempoDetector,
    key_detector: KeyDetector,
    spectral_analyzer: SpectralAnalyzer,
}

#[wasm_bindgen]
impl AudioAnalyzer {
    /// Create a new AudioAnalyzer
    #[wasm_bindgen(constructor)]
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            hop_size: 1024,
            buffer: Vec::with_capacity(2048),
            tempo_detector: TempoDetector::new(sample_rate),
            key_detector: KeyDetector::new(),
            spectral_analyzer: SpectralAnalyzer::new(sample_rate),
        }
    }

    /// Process a chunk of audio samples (mono, normalized to [-1, 1])
    #[wasm_bindgen]
    pub fn analyze_chunk(&mut self, samples: &[f32]) -> JsValue {
        // Accumulate samples into buffer
        self.buffer.extend_from_slice(samples);

        let mut features = AudioFeatures::default();

        // Only analyze if we have enough samples
        if self.buffer.len() >= self.hop_size {
            let chunk = &self.buffer[..self.hop_size];
            self.buffer.drain(..self.hop_size);

            // Extract features
            features.tempo = self.tempo_detector.process(chunk);
            features.key = self.key_detector.process(chunk);
            features.spectral = self.spectral_analyzer.analyze(chunk);

            // Compute RMS for gain/volume
            features.rms = chunk.iter()
                .map(|s| s * s)
                .sum::<f32>()
                .sqrt() / chunk.len() as f32;
        }

        serde_wasm_bindgen::to_value(&features).unwrap()
    }

    /// Reset all internal state buffers
    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.buffer.clear();
        self.tempo_detector.reset();
        self.key_detector.reset();
    }

    /// Set the hop size (analysis window stride)
    #[wasm_bindgen]
    pub fn set_hop_size(&mut self, size: usize) {
        self.hop_size = size.max(512).min(4096);
    }

    /// Get the current sample rate
    #[wasm_bindgen(getter)]
    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }
}

/// Extracted audio features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioFeatures {
    /// Detected BPM (beats per minute)
    pub tempo: f32,
    /// Detected musical key (0-11, where 0=C, 1=C#, etc.)
    pub key: u8,
    /// Spectral features
    pub spectral: SpectralFeatures,
    /// Root mean square (volume)
    pub rms: f32,
}

impl Default for AudioFeatures {
    fn default() -> Self {
        Self {
            tempo: 0.0,
            key: 0,
            spectral: SpectralFeatures::default(),
            rms: 0.0,
        }
    }
}

/// Spectral features extracted from audio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpectralFeatures {
    /// Spectral centroid (brightness)
    pub centroid: f32,
    /// Spectral rolloff (frequency below which 85% of energy resides)
    pub rolloff: f32,
    /// Spectral flux (change between frames)
    pub flux: f32,
    /// Spectral flatness (how noise-like vs tonal)
    pub flatness: f32,
}

impl Default for SpectralFeatures {
    fn default() -> Self {
        Self {
            centroid: 0.0,
            rolloff: 0.0,
            flux: 0.0,
            flatness: 0.0,
        }
    }
}