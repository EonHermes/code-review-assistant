import React, { useState, useEffect, useRef, useCallback } from 'react';
import * as d3 from 'd3';
import { AudioFeatures, FeatureHistory, KEYS, KEY_COLORS } from './types';
import { initAudioProcessor, processAudioChunk, resetAnalyzer, disposeAudioProcessor } from './audioProcessor';

const MAX_HISTORY = 200;

function App() {
  const [isRecording, setIsRecording] = useState(false);
  const [isProcessing, setIsProcessing] = useState(false);
  const [currentFeatures, setCurrentFeatures] = useState<AudioFeatures | null>(null);
  const [history, setHistory] = useState<FeatureHistory>({
    time: [],
    tempo: [],
    key: [],
    spectral: {
      centroid: [],
      rolloff: [],
      flux: [],
      flatness: []
    }
  });
  const [error, setError] = useState<string | null>(null);

  const audioContextRef = useRef<AudioContext | null>(null);
  const analyzerRef = useRef<any>(null);
  const workletNodeRef = useRef<AudioWorkletNode | null>(null);
  const streamRef = useRef<MediaStream | null>(null);
  const animationFrameRef = useRef<number | null>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const tempoChartRef = useRef<SVGSVGElement>(null);
  const spectralChartRef = useRef<SVGSVGElement>(null);
  const keyChartRef = useRef<SVGSVGElement>(null);

  // Initialize audio processor
  useEffect(() => {
    async function init() {
      try {
        analyzerRef.current = await initAudioProcessor(44100);
      } catch (err) {
        setError('Failed to initialize audio processor: ' + (err as Error).message);
      }
    }
    init();

    return () => {
      disposeAudioProcessor();
    };
  }, []);

  // Update charts when features change
  useEffect(() => {
    if (currentFeatures) {
      drawTempoChart();
      drawSpectralChart();
      drawKeyChart();
    }
  }, [currentFeatures, history]);

  const startMicrophone = async () => {
    try {
      setError(null);
      const stream = await navigator.mediaDevices.getUserMedia({
        audio: {
          sampleRate: 44100,
          channelCount: 1,
          echoCancellation: false,
          noiseSuppression: false,
          autoGainControl: false
        }
      });

      streamRef.current = stream;

      const audioContext = new (window.AudioContext || (window as any).webkitAudioContext)({
        sampleRate: 44100
      });
      audioContextRef.current = audioContext;

      // Create script processor for demo (in production, use AudioWorklet with real WASM)
      const source = audioContext.createMediaStreamSource(stream);
      const processor = audioContext.createScriptProcessor(4096, 1, 1);

      source.connect(processor);
      processor.connect(audioContext.destination);

      processor.onaudioprocess = (e) => {
        if (!isRecording || !analyzerRef.current) return;

        const inputBuffer = e.inputBuffer.getChannelData(0);
        const features = processAudioChunk(analyzerRef.current, inputBuffer);

        setCurrentFeatures(features);
        updateHistory(features);
      };

      workletNodeRef.current = processor;
      setIsRecording(true);
      setIsProcessing(true);

    } catch (err) {
      setError('Microphone access denied or error: ' + (err as Error).message);
    }
  };

  const stopRecording = () => {
    if (animationFrameRef.current) {
      cancelAnimationFrame(animationFrameRef.current);
    }

    if (workletNodeRef.current) {
      workletNodeRef.current.disconnect();
      workletNodeRef.current = null;
    }

    if (streamRef.current) {
      streamRef.current.getTracks().forEach(track => track.stop());
      streamRef.current = null;
    }

    if (audioContextRef.current) {
      audioContextRef.current.close();
      audioContextRef.current = null;
    }

    setIsRecording(false);
    setIsProcessing(false);
  };

  const handleFileUpload = async (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file || !analyzerRef.current) return;

    setError(null);
    setIsProcessing(true);

    try {
      const arrayBuffer = await file.arrayBuffer();
      const audioContext = new (window.AudioContext || (window as any).webkitAudioContext)({
        sampleRate: 44100
      });
      const audioBuffer = await audioContext.decodeAudioData(arrayBuffer);

      const channelData = audioBuffer.getChannelData(0);
      const chunkSize = 4096;

      for (let i = 0; i < channelData.length; i += chunkSize) {
        const chunk = channelData.slice(i, i + chunkSize);
        const features = processAudioChunk(analyzerRef.current, chunk);

        setCurrentFeatures(features);
        updateHistory(features);

        // Small delay for visualization
        await new Promise(resolve => setTimeout(resolve, 10));
      }

      audioContext.close();
    } catch (err) {
      setError('Failed to process audio file: ' + (err as Error).message);
    }

    setIsProcessing(false);

    // Reset file input
    if (fileInputRef.current) {
      fileInputRef.current.value = '';
    }
  };

  const updateHistory = useCallback((features: AudioFeatures) => {
    setHistory(prev => {
      const newTime = [...prev.time, Date.now()];
      const newTempo = [...prev.tempo, features.tempo];
      const newKey = [...prev.key, features.key];
      const newCentroid = [...prev.spectral.centroid, features.spectral.centroid];
      const newRolloff = [...prev.spectral.rolloff, features.spectral.rolloff];
      const newFlux = [...prev.spectral.flux, features.spectral.flux];
      const newFlatness = [...prev.spectral.flatness, features.spectral.flatness];

      if (newTime.length > MAX_HISTORY) {
        newTime.shift();
        newTempo.shift();
        newKey.shift();
        newCentroid.shift();
        newRolloff.shift();
        newFlux.shift();
        newFlatness.shift();
      }

      return {
        time: newTime,
        tempo: newTempo,
        key: newKey,
        spectral: {
          centroid: newCentroid,
          rolloff: newRolloff,
          flux: newFlux,
          flatness: newFlatness
        }
      };
    });
  }, []);

  const resetAll = () => {
    resetAnalyzer(analyzerRef.current);
    setCurrentFeatures(null);
    setHistory({
      time: [],
      tempo: [],
      key: [],
      spectral: {
        centroid: [],
        rolloff: [],
        flux: [],
        flatness: []
      }
    });
  };

  const drawTempoChart = () => {
    if (!tempoChartRef.current || history.tempo.length === 0) return;

    const svg = d3.select(tempoChartRef.current);
    svg.selectAll('*').remove();

    const margin = { top: 20, right: 30, bottom: 40, left: 50 };
    const width = tempoChartRef.current.clientWidth - margin.left - margin.right;
    const height = 250 - margin.top - margin.bottom;

    const g = svg.append('g')
      .attr('transform', `translate(${margin.left},${margin.top})`);

    const x = d3.scaleLinear()
      .domain([0, history.time.length - 1])
      .range([0, width]);

    const y = d3.scaleLinear()
      .domain(d3.extent(history.tempo) as [number, number])
      .nice()
      .range([height, 0]);

    const line = d3.line<number>()
      .x((_, i) => x(i))
      .y(d => y(d))
      .curve(d3.curveMonotoneX);

    const area = d3.area<number>()
      .x((_, i) => x(i))
      .y0(height)
      .y1(d => y(d))
      .curve(d3.curveMonotoneX);

    g.append('path')
      .datum(history.tempo)
      .attr('fill', 'rgba(88, 166, 255, 0.1)')
      .attr('d', area);

    g.append('path')
      .datum(history.tempo)
      .attr('fill', 'none')
      .attr('stroke', '#58a6ff')
      .attr('stroke-width', 2)
      .attr('d', line);

    // Axes
    g.append('g')
      .attr('transform', `translate(0,${height})`)
      .call(d3.axisBottom(x).ticks(5).tickFormat(i => `${i}s`))
      .attr('color', '#9ca3af');

    g.append('g')
      .call(d3.axisLeft(y).ticks(5))
      .attr('color', '#9ca3af');

    g.append('text')
      .attr('transform', 'rotate(-90)')
      .attr('y', 0 - margin.left)
      .attr('x', 0 - (height / 2))
      .attr('dy', '1em')
      .style('text-anchor', 'middle')
      .attr('fill', '#9ca3af')
      .text('BPM');

    // Current value
    if (currentFeatures) {
      g.append('text')
        .attr('x', width)
        .attr('y', y(currentFeatures.tempo) - 5)
        .attr('text-anchor', 'end')
        .attr('fill', '#58a6ff')
        .attr('font-weight', 'bold')
        .text(`${currentFeatures.tempo.toFixed(1)} BPM`);
    }
  };

  const drawSpectralChart = () => {
    if (!spectralChartRef.current) return;

    const svg = d3.select(spectralChartRef.current);
    svg.selectAll('*').remove();

    const margin = { top: 20, right: 30, bottom: 40, left: 50 };
    const width = spectralChartRef.current.clientWidth - margin.left - margin.right;
    const height = 250 - margin.top - margin.bottom;

    const g = svg.append('g')
      .attr('transform', `translate(${margin.left},${margin.top})`);

    const metrics = [
      { key: 'centroid', label: 'Centroid', color: '#3fb950' },
      { key: 'rolloff', label: 'Rolloff', color: '#f0883e' },
      { key: 'flux', label: 'Flux', color: '#bc8cff' },
      { key: 'flatness', label: 'Flatness', color: '#e03131' }
    ];

    const x = d3.scaleBand()
      .domain(metrics.map(m => m.label))
      .range([0, width])
      .padding(0.3);

    const allValues = metrics.flatMap(m => history.spectral[m.key as keyof typeof history.spectral]);
    const y = d3.scaleLinear()
      .domain([0, d3.max(allValues) || 1])
      .range([height, 0]);

    g.selectAll('.bar')
      .data(metrics)
      .enter()
      .append('rect')
      .attr('class', 'bar')
      .attr('x', d => x(d.label) || 0)
      .attr('width', x.bandwidth())
      .attr('y', d => {
        const value = history.spectral[d.key as keyof typeof history.spectral];
        const lastVal = value[value.length - 1] || 0;
        return y(lastVal);
      })
      .attr('height', d => {
        const value = history.spectral[d.key as keyof typeof history.spectral];
        const lastVal = value[value.length - 1] || 0;
        return height - y(lastVal);
      })
      .attr('fill', d => d.color)
      .attr('rx', 4);

    g.append('g')
      .attr('transform', `translate(0,${height})`)
      .call(d3.axisBottom(x))
      .attr('color', '#9ca3af');

    g.append('g')
      .call(d3.axisLeft(y).ticks(5))
      .attr('color', '#9ca3af');
  };

  const drawKeyChart = () => {
    if (!keyChartRef.current || history.key.length === 0) return;

    const svg = d3.select(keyChartRef.current);
    svg.selectAll('*').remove();

    const margin = { top: 20, right: 30, bottom: 40, left: 50 };
    const width = keyChartRef.current.clientWidth - margin.left - margin.right;
    const height = 250 - margin.top - margin.bottom;

    const g = svg.append('g')
      .attr('transform', `translate(${margin.left},${margin.top})`);

    // Count occurrences of each key in history
    const keyCounts = new Array(12).fill(0);
    history.key.forEach(k => {
      if (k >= 0 && k < 12) keyCounts[k]++;
    });

    const x = d3.scaleBand()
      .domain(KEYS)
      .range([0, width])
      .padding(0.2);

    const y = d3.scaleLinear()
      .domain([0, d3.max(keyCounts) || 1])
      .range([height, 0]);

    g.selectAll('.key-bar')
      .data(KEYS)
      .enter()
      .append('rect')
      .attr('class', 'key-bar')
      .attr('x', d => x(d) || 0)
      .attr('width', x.bandwidth())
      .attr('y', d => y(keyCounts[KEYS.indexOf(d)]))
      .attr('height', d => height - y(keyCounts[KEYS.indexOf(d)]))
      .attr('fill', (_, i) => KEY_COLORS[i])
      .attr('rx', 4);

    // Highlight current key
    if (currentFeatures) {
      const currentKeyIdx = currentFeatures.key;
      g.append('rect')
        .attr('x', x(KEYS[currentKeyIdx]) || 0)
        .attr('width', x.bandwidth())
        .attr('y', height)
        .attr('height', 2)
        .attr('fill', '#ffffff')
        .attr('rx', 1);
    }

    g.append('g')
      .attr('transform', `translate(0,${height})`)
      .call(d3.axisBottom(x))
      .attr('color', '#9ca3af');

    g.append('g')
      .call(d3.axisLeft(y).ticks(5))
      .attr('color', '#9ca3af');

    g.append('text')
      .attr('transform', 'rotate(-90)')
      .attr('y', 0 - margin.left)
      .attr('x', 0 - (height / 2))
      .attr('dy', '1em')
      .style('text-anchor', 'middle')
      .attr('fill', '#9ca3af')
      .text('Frequency');
  };

  return (
    <div className="container">
      <header>
        <h1>🎵 Real-Time Audio Feature Extractor</h1>
        <p>Extract BPM, musical key, and spectral features from audio in real-time using Rust + WASM</p>
      </header>

      {error && (
        <div className="status" style={{ borderColor: '#e03131' }}>
          <strong>Error:</strong> {error}
        </div>
      )}

      <div className="controls">
        <button
          className="primary-btn"
          onClick={isRecording ? stopRecording : startMicrophone}
          disabled={isProcessing || !analyzerRef.current}
        >
          {isRecording ? '⏹ Stop Recording' : '🎤 Start Microphone'}
        </button>

        <div className="file-input-wrapper">
          <button className="secondary-btn" onClick={() => fileInputRef.current?.click()}>
            📁 Upload Audio File
          </button>
          <input
            ref={fileInputRef}
            type="file"
            accept="audio/*"
            onChange={handleFileUpload}
            disabled={isProcessing}
          />
        </div>

        <button
          className="secondary-btn"
          onClick={resetAll}
          disabled={isRecording || isProcessing}
        >
          🔄 Reset
        </button>
      </div>

      {isRecording && (
        <div className="status recording">
          🎤 <strong>Recording...</strong> Speak or play audio to analyze features in real-time.
        </div>
      )}

      {isProcessing && !isRecording && (
        <div className="status processing">
          ⚙️ <strong>Processing...</strong> Analyzing uploaded audio file.
        </div>
      )}

      {currentFeatures && (
        <div className="features-grid">
          <div className="feature-card">
            <h3>🥁 Tempo (BPM)</h3>
            <div className="feature-value">
              {currentFeatures.tempo.toFixed(1)}
              <span className="feature-unit">BPM</span>
            </div>
            <p style={{ color: 'var(--text-secondary)', fontSize: '0.9rem' }}>
              Detected using autocorrelation of onset strength envelope
            </p>
          </div>

          <div className="feature-card">
            <h3>🎹 Musical Key</h3>
            <div className="key-indicator" style={{ backgroundColor: KEY_COLORS[currentFeatures.key] }}>
              {KEYS[currentFeatures.key]}
            </div>
            <p style={{ color: 'var(--text-secondary)', fontSize: '0.9rem', marginTop: '0.5rem' }}>
              Chroma-based key detection
            </p>
          </div>

          <div className="feature-card">
            <h3>📊 Volume (RMS)</h3>
            <div className="feature-value">
              {(currentFeatures.rms * 100).toFixed(1)}
              <span className="feature-unit">%</span>
            </div>
            <p style={{ color: 'var(--text-secondary)', fontSize: '0.9rem' }}>
              Root mean square amplitude
            </p>
          </div>
        </div>
      )}

      <div className="visualization-container">
        <h2>Visualizations</h2>
        <div className="charts-grid">
          <div className="chart-container">
            <h3 style={{ marginBottom: '1rem', color: 'var(--accent-blue)' }}>Tempo Over Time</h3>
            <svg ref={tempoChartRef}></svg>
          </div>

          <div className="chart-container">
            <h3 style={{ marginBottom: '1rem', color: 'var(--accent-purple)' }}>Spectral Features</h3>
            <svg ref={spectralChartRef}></svg>
          </div>

          <div className="chart-container">
            <h3 style={{ marginBottom: '1rem', color: 'var(--accent-orange)' }}>Key Distribution</h3>
            <svg ref={keyChartRef}></svg>
          </div>
        </div>
      </div>

      <div className="instructions">
        <h2>How to Use</h2>
        <ol>
          <li>Click <strong>"Start Microphone"</strong> to analyze live audio from your mic, or <strong>"Upload Audio File"</strong> to analyze a file</li>
          <li>Speak, play music, or wait for file processing. Features update in real-time!</li>
          <li>Watch the visualizations update with tempo trends, spectral characteristics, and key detection</li>
          <li>Click <strong>"Reset"</strong> to clear all data and start fresh</li>
        </ol>
        <p style={{ marginTop: '1rem', color: 'var(--text-secondary)' }}>
          <strong>Technical stack:</strong> Rust/WASM for core audio processing (BPM detection, chroma analysis, spectral features),
          React + TypeScript + Vite frontend, D3.js for real-time visualizations.
        </p>
      </div>

      <footer>
        <p>Built with ❤️ by Eon Automation | MIT License</p>
        <p style={{ marginTop: '0.5rem' }}>
          <a href="https://github.com/EonHermes/real-time-audio-feature-extractor" style={{ color: 'var(--accent-blue)' }}>
            View on GitHub
          </a>
        </p>
      </footer>
    </div>
  );
}

export default App;