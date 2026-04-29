use super::*;

impl FftState {
    /// Toggle grid
    pub fn toggle_grid(&mut self) {
        self.show_grid = !self.show_grid;
    }

    /// Toggle peaks
    pub fn toggle_peaks(&mut self) {
        self.show_peaks = !self.show_peaks;
    }

    /// Toggle harmonics
    pub fn toggle_harmonics(&mut self) {
        self.show_harmonics = !self.show_harmonics;
    }

    /// Add a user marker frequency. Maintains sorted order and bounded count.
    pub fn add_marker(&mut self, frequency_hz: f64) {
        if !frequency_hz.is_finite() || frequency_hz < 0.0 {
            return;
        }
        if self
            .marker_frequencies
            .iter()
            .any(|f| (*f - frequency_hz).abs() <= MARKER_MERGE_EPS_HZ)
        {
            return;
        }
        self.marker_frequencies.push(frequency_hz);
        self.marker_frequencies.sort_by(|a, b| a.total_cmp(b));
        if self.marker_frequencies.len() > MAX_USER_MARKERS {
            self.marker_frequencies.remove(0);
        }
    }

    /// Remove the nearest marker within a tolerance window.
    pub fn remove_nearest_marker(&mut self, frequency_hz: f64, tolerance_hz: f64) -> bool {
        if !frequency_hz.is_finite() || !tolerance_hz.is_finite() || tolerance_hz < 0.0 {
            return false;
        }
        let Some((idx, dist)) = self
            .marker_frequencies
            .iter()
            .enumerate()
            .map(|(idx, marker)| (idx, (*marker - frequency_hz).abs()))
            .min_by(|a, b| a.1.total_cmp(&b.1))
        else {
            return false;
        };
        if dist <= tolerance_hz {
            self.marker_frequencies.remove(idx);
            true
        } else {
            false
        }
    }

    /// Number of user marker slots with assigned frequencies.
    pub fn marker_count(&self) -> usize {
        self.marker_frequencies.len()
    }

    /// Clear all user markers.
    pub fn clear_markers(&mut self) {
        self.marker_frequencies.clear();
    }

    /// Remove marker at explicit index.
    pub fn remove_marker_at(&mut self, index: usize) -> bool {
        if index >= self.marker_frequencies.len() {
            return false;
        }
        self.marker_frequencies.remove(index);
        true
    }
}
