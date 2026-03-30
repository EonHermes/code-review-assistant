export interface AudioFeatures {
  tempo: number;
  key: number;
  rms: number;
  spectral: SpectralFeatures;
}

export interface SpectralFeatures {
  centroid: number;
  rolloff: number;
  flux: number;
  flatness: number;
}

export interface FeatureHistory {
  time: number[];
  tempo: number[];
  key: number[];
  spectral: {
    centroid: number[];
    rolloff: number[];
    flux: number[];
    flatness: number[];
  };
}

export const KEYS = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'];

export const KEY_COLORS = [
  '#e03131', '#f08c00', '#2f9e44', '#1c7ed6',
  '#7048e8', '#9c36b5', '#e64980', '#e67700',
  '#37b24d', '#1c7ed6', '#5c7cfa', '#cc5de8'
];