mod tempo;

use super::*;

/// BPM detector using autocorrelation of onset strength envelope
pub struct TempoDetector {
    sample_rate: f32,
    onset_history: Vec<f32>,
    history_len: usize,
    min_bpm: f32,
    max_bpm: f32,
}

impl TempoDetector {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            onset_history: Vec::with_capacity(4096),
            history_len: 4096,
            min_bpm: 60.0,
            max_bpm: 240.0,
        }
    }

    pub fn process(&mut self, chunk: &[f32]) -> f32 {
        // Compute onset strength (energy envelope)
        let onset = self.compute_onset_strength(chunk);
        self.onset_history.push(onset);
        if self.onset_history.len() > self.history_len {
            self.onset_history.remove(0);
        }

        // Only estimate tempo if we have sufficient history
        if self.onset_history.len() < 1024 {
            return 0.0;
        }

        self.estimate_tempo()
    }

    fn compute_onset_strength(&self, chunk: &[f32]) -> f32 {
        // Simplified onset detection: difference envelope
        let mut sum_abs_diff = 0.0;
        for i in 1..chunk.len() {
            sum_abs_diff += (chunk[i] - chunk[i - 1]).abs();
        }
        sum_abs_diff / chunk.len() as f32
    }

    fn estimate_tempo(&self) -> f32 {
        let signal = &self.onset_history;
        let len = signal.len();

        // Autocorrelation
        let mut best_lag = 0;
        let mut best_corr = 0.0;

        let min_lag = (self.sample_rate * 60.0 / self.max_bpm).round() as usize;
        let max_lag = (self.sample_rate * 60.0 / self.min_bpm).round() as usize;

        for lag in min_lag..max_lag.min(len - 1) {
            let mut corr = 0.0;
            for i in 0..len - lag {
                corr += signal[i] * signal[i + lag];
            }
            if corr > best_corr {
                best_corr = corr;
                best_lag = lag;
            }
        }

        if best_lag > 0 {
            self.sample_rate * 60.0 / best_lag as f32
        } else {
            0.0
        }
    }

    pub fn reset(&mut self) {
        self.onset_history.clear();
    }
}