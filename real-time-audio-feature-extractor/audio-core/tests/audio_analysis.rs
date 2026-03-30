#[cfg(test)]
mod audio_analysis_tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_onset_strength_computation() {
        // Simulate a signal with clear attacks
        let mut detector = TempoDetector::new(44100.0);

        // Pulse signal
        let mut signal = vec![0.0; 2048];
        for i in 0..signal.len() {
            if i % 441 == 0 { // Pulse every ~10ms at 44.1kHz
                signal[i] = 0.8;
            }
        }

        let onset1 = detector.compute_onset_strength(&signal[..1024]);
        let onset2 = detector.compute_onset_strength(&signal[1024..]);

        // Onsets should be detectable (non-zero)
        assert!(onset1 > 0.0);
        assert!(onset2 > 0.0);
    }

    #[test]
    fn test_tempo_estimation_with_steady_pulse() {
        let mut detector = TempoDetector::new(44100.0);

        // Simulate regular onsets for tempo detection
        let mut signal = vec![0.0; 8192];
        for i in (0..signal.len()).step_by(2205) { // ~100 BPM
            signal[i] = 1.0;
        }

        // Process multiple chunks to build history
        for chunk in signal.chunks(1024) {
            detector.process(chunk);
        }

        let tempo = detector.estimate_tempo();
        // Should detect tempo in reasonable range
        assert!(tempo > 60.0 && tempo < 180.0, "Detected tempo: {} BPM", tempo);
    }

    #[test]
    fn test_chroma_normalization() {
        let detector = KeyDetector::new();

        // Create synthetic signal with energy in all bins
        let signal = vec![0.5; 512];
        let chroma = detector.compute_chroma(&signal);

        // Sum should be <= 1.0 (normalized)
        let sum: f32 = chroma.iter().sum();
        assert!(sum <= 1.0 + 1e-6, "Chroma sum {} should be <= 1.0", sum);

        // All values should be >= 0
        for &val in &chroma {
            assert!(val >= 0.0, "Chroma value should be non-negative, got {}", val);
        }
    }

    #[test]
    fn test_spectral_centroid_range() {
        let mut analyzer = SpectralAnalyzer::new(44100.0);

        // Signal with various characteristics
        let signal = vec![0.1; 1024];
        let features = analyzer.analyze(&signal);

        // Centroid should be in valid range (0 to Nyquist)
        assert!(features.centroid >= 0.0 && features.centroid <= 22050.0);
    }

    #[test]
    fn test_spectral_flatness_bounds() {
        let mut analyzer = SpectralAnalyzer::new(44100.0);

        // Test with random-like signal (higher flatness)
        let random: Vec<f32> = (0..1024)
            .map(|_| rand::random::<f32>())
            .collect();

        let features = analyzer.analyze(&random);

        // Flatness should be between 0 and 1
        assert!(features.flatness >= 0.0 && features.flatness <= 1.0);
    }

    #[test]
    fn test_spectral_flux_computation() {
        let mut analyzer = SpectralAnalyzer::new(44100.0);

        // First frame
        let frame1 = vec![0.1; 512];
        let _ = analyzer.analyze(&frame1);

        // Second frame: different value
        let frame2 = vec![0.5; 512];
        let features = analyzer.analyze(&frame2);

        // Flux should be non-negative
        assert!(features.flux >= 0.0);
        // Should detect change
        assert!(features.flux > 0.01);
    }

    #[test]
    fn test_spectral_rolloff_low() {
        let mut analyzer = SpectralAnalyzer::new(44100.0);

        // Signal with energy in lower bands
        let mut signal = vec![0.0; 1024];
        for i in 0..100 {
            signal[i] = 1.0;
        }

        let features = analyzer.analyze(&signal);

        // Rolloff should be relatively low
        assert!(features.rolloff < 5000.0,
            "Rolloff {} should be lower for low-frequency signal",
            features.rolloff);
    }

    #[test]
    fn test_detector_reset() {
        let mut tempo = TempoDetector::new(44100.0);
        let mut key = KeyDetector::new();
        let mut spectral = SpectralAnalyzer::new(44100.0);

        // Process some data
        let signal = vec![0.1; 1024];
        tempo.process(&signal);
        key.process(&signal);
        spectral.analyze(&signal);

        // Reset
        tempo.reset();
        key.reset();

        // State should be cleared
        // Can't directly access buffer, but we can call process again without panic
        let _ = tempo.process(&signal);
        let _ = key.process(&signal);
    }
}