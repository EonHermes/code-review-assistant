mod spectral;

use super::*;

/// Spectral feature extractor
pub struct SpectralAnalyzer {
    sample_rate: f32,
    previous_spectrum: Vec<f32>,
}

impl SpectralAnalyzer {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            previous_spectrum: Vec::new(),
        }
    }

    pub fn analyze(&mut self, chunk: &[f32]) -> SpectralFeatures {
        // Compute approximate spectrum using zero crossings and energy
        let spectrum = self.approximate_spectrum(chunk);

        // Compute features
        let centroid = self.spectral_centroid(&spectrum);
        let rolloff = self.spectral_rolloff(&spectrum);
        let flux = self.spectral_flux(&spectrum);
        let flatness = self.spectral_flatness(&spectrum);

        // Store for next frame
        self.previous_spectrum = spectrum;

        SpectralFeatures {
            centroid,
            rolloff,
            flux,
            flatness,
        }
    }

    fn approximate_spectrum(&self, chunk: &[f32]) -> Vec<f32> {
        // Simplified spectral approximation using energy in frequency bands
        // For real production, you'd use FFT, but this lightweight version
        // provides reasonable estimates for visualization purposes
        let mut spectrum = vec![0.0; 64]; // 64 frequency bands

        let mut sum_abs = 0.0;
        for &sample in chunk {
            sum_abs += sample.abs();
        }
        let avg_abs = sum_abs / chunk.len() as f32;

        // Distribute energy across bands based on zero crossing rate
        let zcr = self.zero_crossing_rate(chunk);
        let dominant_band = (zcr * 63.0) as usize;

        for i in 0..spectrum.len() {
            let dist = (i as f32 - dominant_band as f32).abs();
            let weight = (-dist * dist / 50.0).exp();
            spectrum[i] = avg_abs * weight;
        }

        spectrum
    }

    fn zero_crossing_rate(&self, chunk: &[f32]) -> f32 {
        let mut crossings = 0;
        for i in 1..chunk.len() {
            if (chunk[i] > 0.0) != (chunk[i - 1] > 0.0) {
                crossings += 1;
            }
        }
        crossings as f32 / (chunk.len() as f32 - 1.0).max(1.0)
    }

    fn spectral_centroid(&self, spectrum: &[f32]) -> f32 {
        let mut weighted_sum = 0.0;
        let mut sum_weights = 0.0;

        for (i, &magnitude) in spectrum.iter().enumerate() {
            weighted_sum += i as f32 * magnitude;
            sum_weights += magnitude;
        }

        if sum_weights > 0.0 {
            (weighted_sum / sum_weights) * (self.sample_rate / 2.0) / spectrum.len() as f32
        } else {
            0.0
        }
    }

    fn spectral_rolloff(&self, spectrum: &[f32]) -> f32 {
        let sum: f32 = spectrum.iter().sum();
        let threshold = sum * 0.85; // 85% of total energy

        let mut cumsum = 0.0;
        for (i, &magnitude) in spectrum.iter().enumerate() {
            cumsum += magnitude;
            if cumsum >= threshold {
                return i as f32 * (self.sample_rate / 2.0) / spectrum.len() as f32;
            }
        }

        self.sample_rate / 2.0
    }

    fn spectral_flux(&self, spectrum: &[f32]) -> f32 {
        if self.previous_spectrum.len() != spectrum.len() {
            return 0.0;
        }

        let mut flux = 0.0;
        for (curr, prev) in spectrum.iter().zip(&self.previous_spectrum) {
            let diff = (curr - prev).max(0.0);
            flux += diff * diff;
        }

        (flux / spectrum.len() as f32).sqrt()
    }

    fn spectral_flatness(&self, spectrum: &[f32]) -> f32 {
        if spectrum.is_empty() {
            return 0.0;
        }

        let n = spectrum.len() as f32;
        let geometric_mean = spectrum.iter()
            .filter(|&&x| x > 0.0)
            .fold(1.0, |prod, &x| prod * x)
            .powf(1.0 / n);

        let arithmetic_mean = spectrum.iter().sum::<f32>() / n;

        if arithmetic_mean > 0.0 {
            geometric_mean / arithmetic_mean
        } else {
            0.0
        }
    }
}