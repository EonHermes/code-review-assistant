mod chroma;

use super::*;

/// Key detector using chroma features
pub struct KeyDetector {
    chroma_sum: Vec<f32>,
    frame_count: usize,
}

impl KeyDetector {
    pub fn new() -> Self {
        Self {
            chroma_sum: vec![0.0; 12],
            frame_count: 0,
        }
    }

    pub fn process(&mut self, chunk: &[f32]) -> u8 {
        // Compute chroma features for this chunk
        let chroma = self.compute_chroma(chunk);

        // Accumulate over multiple frames for stability
        for i in 0..12 {
            self.chroma_sum[i] += chroma[i];
        }
        self.frame_count += 1;

        // Reset periodically
        if self.frame_count >= 64 {
            self.frame_count = 0;
            for i in 0..12 {
                self.chroma_sum[i] *= 0.95; // decay
            }
        }

        // Find peak chroma bin
        let mut max_idx = 0;
        let mut max_val = 0.0;
        for (i, &val) in self.chroma_sum.iter().enumerate() {
            if val > max_val {
                max_val = val;
                max_idx = i;
            }
        }

        max_idx as u8
    }

    fn compute_chroma(&self, chunk: &[f32]) -> [f32; 12] {
        let mut chroma = [0.0; 12];

        // Simplified chroma extraction: map frequency bands to 12 pitch classes
        // This is a lightweight approximation suitable for real-time WASM
        for &sample in chunk {
            let magnitude = sample.abs();
            if magnitude < 0.01 {
                continue;
            }

            // Map to frequency band (simplified - no actual FFT for real-time perf)
            // In production, you'd use FFT and map frequency bins to chroma
            let note = ((magnitude * 12.0) % 12.0) as usize;
            chroma[note] += magnitude;
        }

        // Normalize
        let sum: f32 = chroma.iter().sum();
        if sum > 0.0 {
            for c in &mut chroma {
                *c /= sum;
            }
        }

        chroma
    }

    pub fn reset(&mut self) {
        self.chroma_sum.fill(0.0);
        self.frame_count = 0;
    }
}