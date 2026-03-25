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
                && k.unsigned_abs() as usize > max_k {
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

#[cfg(test)]
mod multi_tone_tests {
    use super::*;

    #[test]
    fn test_frequency_index_dc() {
        let idx = FrequencyIndex::dc(2);
        assert_eq!(idx.indices, vec![0, 0]);
        assert_eq!(idx.order(), 0);
    }

    #[test]
    fn test_frequency_index_single_tone() {
        let idx = FrequencyIndex::single_tone(0, 3, 2);
        assert_eq!(idx.indices, vec![3, 0]);
        assert_eq!(idx.order(), 3);
    }

    #[test]
    fn test_frequency_index_frequency() {
        let idx = FrequencyIndex {
            indices: vec![2, -1],
        };
        let tones = vec![1e9, 800e6];
        // f = 2*1e9 + (-1)*800e6 = 2e9 - 0.8e9 = 1.2e9
        assert!((idx.frequency(&tones) - 1.2e9).abs() < 1.0);
    }

    #[test]
    fn test_frequency_index_validity() {
        let idx = FrequencyIndex {
            indices: vec![2, 3],
        };
        assert!(idx.is_valid(&[5, 5], 10)); // order=5 <= 10
        assert!(!idx.is_valid(&[5, 5], 4)); // order=5 > 4
        assert!(!idx.is_valid(&[1, 5], 10)); // harmonic 0 = 2 > 1
    }

    #[test]
    fn test_frequency_map_single_tone() {
        let map = FrequencyMap::single_tone(1e9, 3);

        // Should have DC, ±1, ±2, ±3 => 7 components
        // But box truncation with order 3 gives: -3,-2,-1,0,1,2,3
        assert_eq!(map.len(), 7);

        // Check DC is present
        assert!(map.find(&FrequencyIndex::dc(1)).is_some());
    }

    #[test]
    fn test_frequency_map_two_tone() {
        let config = MultiToneConfig::new(vec![900e6, 800e6], 2);
        let map = FrequencyMap::new(&config);

        // With max_order=2 and 2 harmonics each:
        // Valid indices where |k1| + |k2| <= 2
        // This gives a diamond pattern
        assert!(!map.is_empty());

        // DC should be present
        assert!(map.find(&FrequencyIndex::dc(2)).is_some());
    }

    #[test]
    fn test_frequency_map_positive_frequencies() {
        let map = FrequencyMap::single_tone(1e9, 2);
        let positive = map.positive_frequencies();

        // Should have DC, f, 2f
        assert!(positive.len() >= 3);

        // All frequencies should be >= 0
        for (_, idx) in &positive {
            assert!(idx.frequency(&[1e9]) >= 0.0);
        }
    }

    #[test]
    fn test_multi_tone_config() {
        let config = MultiToneConfig::new(vec![1e9, 2e9], 5);
        assert_eq!(config.num_tones(), 2);
        assert_eq!(config.max_order, 5);
    }

    #[test]
    fn test_frequency_index_ignores_non_finite_tone_terms() {
        let idx = FrequencyIndex {
            indices: vec![2, 1, -1],
        };
        let freq = idx.frequency(&[1e9, f64::NAN, f64::INFINITY]);
        assert!(freq.is_finite());
        assert!((freq - 2e9).abs() < 1.0);
    }

    #[test]
    fn test_frequency_map_with_non_finite_tones_produces_finite_frequencies() {
        let config = MultiToneConfig::new(vec![1e9, f64::NAN, f64::INFINITY], 2);
        let map = FrequencyMap::new(&config);

        assert!(!map.is_empty());
        assert!(map.all_frequencies().iter().all(|freq| freq.is_finite()));
    }
}
