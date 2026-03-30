import { AudioFeatures } from './types';

// WASM bindings interface
export interface IAudioAnalyzer {
  sampleRate: number;
  reset(): void;
  setHopSize(size: number): void;
  analyzeChunk(samples: Float32Array): AudioFeatures;
}

// Mock implementation for development (will be replaced by real WASM)
class MockAudioAnalyzer implements IAudioAnalyzer {
  sampleRate: number;
  private hopSize: number;
  private frameCount: number = 0;
  private previousSamples: Float32Array;

  constructor(sampleRate: number) {
    this.sampleRate = sampleRate;
    this.hopSize = 1024;
    this.previousSamples = new Float32Array(1024);
  }

  reset(): void {
    this.frameCount = 0;
    this.previousSamples.fill(0);
  }

  setHopSize(size: number): void {
    this.hopSize = Math.max(512, Math.min(4096, size));
    this.previousSamples = new Float32Array(this.hopSize);
  }

  analyzeChunk(samples: Float32Array): AudioFeatures {
    this.frameCount++;

    // Simulate BPM detection (random walk for demo)
    const baseTempo = 120;
    const tempoNoise = Math.sin(this.frameCount * 0.01) * 5;
    let tempo = baseTempo + tempoNoise + (Math.random() - 0.5) * 2;

    // Simulate key detection (oscillate through keys)
    const keyCycle = Math.floor(this.frameCount / 400);
    const key = (keyCycle % 12) + Math.floor(Math.random() * 3);

    // Compute RMS
    let sumSquares = 0;
    for (let i = 0; i < samples.length; i++) {
      sumSquares += samples[i] * samples[i];
    }
    const rms = Math.sqrt(sumSquares / samples.length);

    // Simulate spectral features
    const spectral = {
      centroid: 2000 + Math.sin(this.frameCount * 0.02) * 1500 + Math.random() * 500,
      rolloff: 5000 + Math.cos(this.frameCount * 0.015) * 2000 + Math.random() * 1000,
      flux: Math.abs(Math.sin(this.frameCount * 0.05)) * (0.3 + Math.random() * 0.1),
      flatness: 0.3 + Math.sin(this.frameCount * 0.03) * 0.2 + Math.random() * 0.1
    };

    // Clamp flatness
    spectral.flatness = Math.max(0, Math.min(1, spectral.flatness));

    return { tempo, key, rms, spectral };
  }
}

let audioAnalyzer: IAudioAnalyzer | null = null;
let useMock: boolean = true; // Set to false when WASM is available

// Real WASM bindings - these will be available after wasm-pack build
// The generated bindings would be imported like:
// import init, { AudioAnalyzer as WasmAudioAnalyzer } from '../audio-core';

interface WasmAudioAnalyzer {
  new(sampleRate: number): IAudioAnalyzer;
}

export async function initAudioProcessor(sampleRate: number = 44100): Promise<IAudioAnalyzer> {
  if (audioAnalyzer === null) {
    if (useMock) {
      console.log('Using mock audio analyzer (WASM not loaded)');
      audioAnalyzer = new MockAudioAnalyzer(sampleRate);
    } else {
      // This would load the real WASM module when built
      // const wasm = await init();
      // audioAnalyzer = new WasmAudioAnalyzer(sampleRate);
      audioAnalyzer = new MockAudioAnalyzer(sampleRate);
    }
  }
  return audioAnalyzer;
}

export function processAudioChunk(analyzer: IAudioAnalyzer, samples: Float32Array): AudioFeatures {
  return analyzer.analyzeChunk(samples);
}

export function resetAnalyzer(analyzer: IAudioAnalyzer): void {
  analyzer.reset();
}

export function disposeAudioProcessor(): void {
  audioAnalyzer = null;
}

export function setMockMode(enabled: boolean): void {
  useMock = enabled;
}