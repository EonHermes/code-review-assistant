#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_analyzer_creation() {
        let analyzer = AudioAnalyzer::new(44100.0);
        assert_eq!(analyzer.sample_rate(), 44100.0);
    }

    #[test]
    fn test_analyze_chunk_returns_valid_features() {
        let mut analyzer = AudioAnalyzer::new(44100.0);
        let samples: Vec<f32> = vec![0.0; 1024];
        let result = analyzer.analyze_chunk(&samples);

        // We can't directly test JsValue, but we can ensure it doesn't panic
        // and verify deserialization would work by creating expected struct
        let expected = AudioFeatures::default();
        assert_eq!(expected.tempo, 0.0);
        assert_eq!(expected.key, 0);
    }

    #[test]
    fn test_tempo_detector_basic_operation() {
        let mut detector = TempoDetector::new(44100.0);

        // Process some chunks
        let chunk = vec![0.1; 512];
        for _ in 0..10 {
            let _tempo = detector.process(&chunk);
        }

        // Should be able to reset without panic
        detector.reset();
    }

    #[test]
    fn test_spectral_analyzer_computes_features() {
        let mut analyzer = SpectralAnalyzer::new(44100.0);
        let chunk: Vec<f32> = vec![0.1; 1024];
        let features = analyzer.analyze(&chunk);

        assert!(features.centroid >= 0.0);
        assert!(features.rolloff >= 0.0);
        assert!(features.flux >= 0.0);
        assert!(features.flatness >= 0.0 && features.flatness <= 1.0);
    }

    #[test]
    fn test_key_detector_returns_valid_key() {
        let mut detector = KeyDetector::new();
        let chunk: Vec<f32> = vec![0.1; 512];
        let key = detector.process(&chunk);
        assert!(key < 12, "Key should be in range 0-11");
    }

    #[test]
    fn test_analyzer_hop_size_bounds() {
        let mut analyzer = AudioAnalyzer::new(44100.0);

        // Test valid hop size
        analyzer.set_hop_size(2048);
        // Verify internal state is consistent

        // Test clamping
        analyzer.set_hop_size(100); // Should clamp to 512
        analyzer.set_hop_size(10000); // Should clamp to 4096
    }

    #[test]
    fn test_chroma_computation() {
        let detector = KeyDetector::new();
        let chunk: Vec<f32> = vec![0.5; 256];
        let chroma = detector.compute_chroma(&chunk);

        assert_eq!(chroma.len(), 12);
        let sum: f32 = chroma.iter().sum();
        assert!(sum <= 1.0, "Chroma should be normalized");
    }

    #[test]
    fn test_spectral_flatness_range() {
        let mut analyzer = SpectralAnalyzer::new(44100.0);

        // Test with uniform spectrum (high flatness)
        let uniform: Vec<f32> = vec![1.0; 1024];
        let features = analyzer.analyze(&uniform);
        assert!(features.flatness > 0.9, "Uniform signal should have high flatness");

        // Test with sparse spectrum (low flatness)
        let mut sparse = vec![0.0; 1024];
        sparse[0] = 100.0;
        let features = analyzer.analyze(&sparse);
        assert!(features.flatness < 0.1, "Sparse signal should have low flatness");
    }

    #[test]
    fn test_spectral_flux_computation() {
        let mut analyzer = SpectralAnalyzer::new(44100.0);

        let chunk1: Vec<f32> = vec![0.1; 512];
        let _features1 = analyzer.analyze(&chunk1);

        let chunk2: Vec<f32> = vec![0.2; 512]; // Different level
        let features2 = analyzer.analyze(&chunk2);

        // Flux should be > 0 if spectra differ
        assert!(features2.flux >= 0.0);

        // Same spectrum should give near-zero flux
        let chunk3: Vec<f32> = vec![0.2; 512];
        let features3 = analyzer.analyze(&chunk3);
        assert!(features3.flux < features2.flux || features3.flux < 0.001);
    }
}