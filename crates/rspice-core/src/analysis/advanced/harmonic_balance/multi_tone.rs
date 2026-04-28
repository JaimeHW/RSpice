//! Multi-Tone Frequency Handling for Harmonic Balance
//!
//! Handles circuits with multiple incommensurate frequencies (e.g., mixers).
//! Uses box truncation to limit the number of intermodulation products.

use crate::Value;

/// Configuration for multi-tone analysis
#[derive(Debug, Clone)]
pub struct MultiToneConfig {
    /// Tone frequencies
    pub frequencies: Vec<Value>,
    /// Number of harmonics per tone
    pub harmonics_per_tone: Vec<usize>,
    /// Maximum intermodulation order
    pub max_order: usize,
}

impl MultiToneConfig {
    /// Create a new multi-tone configuration
    pub fn new(frequencies: Vec<Value>, max_order: usize) -> Self {
        let n_tones = frequencies.len();
        Self {
            frequencies,
            harmonics_per_tone: vec![max_order; n_tones],
            max_order,
        }
    }

    /// Get number of tones
    pub fn num_tones(&self) -> usize {
        self.frequencies.len()
    }
}

/// Frequency index for multi-tone HB
///
/// Represents a frequency as a sum of harmonics: f = Σ kᵢ * fᵢ
/// where kᵢ is the harmonic index for tone i
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FrequencyIndex {
    /// Harmonic indices for each tone
    pub indices: Vec<i32>,
}

impl FrequencyIndex {
    /// Create DC index (all zeros)
    pub fn dc(num_tones: usize) -> Self {
        Self {
            indices: vec![0; num_tones],
        }
    }

    /// Create single-tone harmonic index
    pub fn single_tone(tone: usize, harmonic: i32, num_tones: usize) -> Self {
        let mut indices = vec![0; num_tones];
        if tone < num_tones {
            indices[tone] = harmonic;
        }
        Self { indices }
    }

    /// Get the total order (sum of absolute harmonic indices)
    pub fn order(&self) -> usize {
        self.indices.iter().map(|k| k.unsigned_abs() as usize).sum()
    }

    /// Compute the frequency from tone frequencies
    pub fn frequency(&self, tone_frequencies: &[Value]) -> Value {
        self.indices
            .iter()
            .zip(tone_frequencies.iter())
            .filter_map(|(&k, &f)| {
                let term = k as f64 * f;
                term.is_finite().then_some(term)
            })
            .sum()
    }

    /// Check if this is a valid index (within box truncation limits)
    pub fn is_valid(&self, max_harmonics: &[usize], max_order: usize) -> bool {
        if self.order() > max_order {
            return false;
        }
        for (i, &k) in self.indices.iter().enumerate() {
            if let Some(&max_k) = max_harmonics.get(i)
                && k.unsigned_abs() as usize > max_k
            {
                return false;
            }
        }
        true
    }
}

/// Frequency map for multi-tone analysis
///
/// Maps between linear spectral indices and multi-tone frequency indices.
#[derive(Debug, Clone)]
pub struct FrequencyMap {
    /// Tone frequencies
    tone_frequencies: Vec<Value>,

    /// Maximum harmonics per tone
    max_harmonics: Vec<usize>,

    /// Maximum mixing order
    max_order: usize,

    /// Ordered list of frequency indices
    indices: Vec<FrequencyIndex>,

    /// Corresponding frequencies
    frequencies: Vec<Value>,
}

impl FrequencyMap {
    /// Create a new frequency map for multi-tone analysis
    pub fn new(config: &MultiToneConfig) -> Self {
        let mut map = Self {
            tone_frequencies: config.frequencies.clone(),
            max_harmonics: config.harmonics_per_tone.clone(),
            max_order: config.max_order,
            indices: Vec::new(),
            frequencies: Vec::new(),
        };
        map.build_indices();
        map
    }

    /// Create a single-tone frequency map
    pub fn single_tone(fundamental: Value, num_harmonics: usize) -> Self {
        let config = MultiToneConfig::new(vec![fundamental], num_harmonics);
        Self::new(&config)
    }

    /// Build the list of valid frequency indices using box truncation
    fn build_indices(&mut self) {
        self.indices.clear();
        self.frequencies.clear();

        let num_tones = self.tone_frequencies.len();
        if num_tones == 0 {
            return;
        }

        // Generate all combinations within box truncation limits
        self.generate_indices_recursive(vec![0; num_tones], 0);

        // Sort by frequency
        let mut indexed: Vec<_> = self
            .indices
            .iter()
            .enumerate()
            .map(|(i, idx)| (i, idx.frequency(&self.tone_frequencies)))
            .collect();
        indexed.sort_by(|a, b| a.1.total_cmp(&b.1));

        let sorted_indices: Vec<_> = indexed
            .iter()
            .map(|(i, _)| self.indices[*i].clone())
            .collect();

        self.indices = sorted_indices;
        self.frequencies = self
            .indices
            .iter()
            .map(|idx| idx.frequency(&self.tone_frequencies))
            .collect();
    }

    /// Recursive helper to generate all valid frequency indices
    fn generate_indices_recursive(&mut self, mut current: Vec<i32>, tone: usize) {
        if tone >= self.tone_frequencies.len() {
            let idx = FrequencyIndex { indices: current };
            if idx.is_valid(&self.max_harmonics, self.max_order) {
                self.indices.push(idx);
            }
            return;
        }

        let max_k = self.max_harmonics.get(tone).copied().unwrap_or(0) as i32;
        for k in -max_k..=max_k {
            current[tone] = k;
            // Prune early if order already exceeds maximum
            let partial_order: usize = current
                .iter()
                .take(tone + 1)
                .map(|x| x.unsigned_abs() as usize)
                .sum();
            if partial_order <= self.max_order {
                self.generate_indices_recursive(current.clone(), tone + 1);
            }
        }
    }

    /// Get number of frequency components
    pub fn len(&self) -> usize {
        self.indices.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.indices.is_empty()
    }

    /// Get frequency at linear index
    pub fn frequency(&self, index: usize) -> Value {
        self.frequencies.get(index).copied().unwrap_or(0.0)
    }

    /// Get frequency index at linear index
    pub fn index(&self, linear_index: usize) -> Option<&FrequencyIndex> {
        self.indices.get(linear_index)
    }

    /// Find linear index for a frequency index
    pub fn find(&self, freq_index: &FrequencyIndex) -> Option<usize> {
        self.indices.iter().position(|i| i == freq_index)
    }

    /// Get all frequencies
    pub fn all_frequencies(&self) -> &[Value] {
        &self.frequencies
    }

    /// Get positive (or zero) frequency indices only
    /// For real signals, we only need one-sided spectrum
    pub fn positive_frequencies(&self) -> Vec<(usize, &FrequencyIndex)> {
        self.indices
            .iter()
            .enumerate()
            .filter(|(_, idx)| idx.frequency(&self.tone_frequencies) >= 0.0)
            .collect()
    }
}
