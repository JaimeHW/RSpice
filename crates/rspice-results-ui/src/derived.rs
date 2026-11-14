//! Shared derived display columns and measurements for result viewers.
//! The host supplies source keys and decides when source changes require reset.

use rspice_results::waveform::SharedWaveformValues;

mod extrema;

pub use extrema::{ExtremaIndex, Extremes};

/// Source key, interval endpoint bits and branch ordinal.
pub type WindowStatsKey = (u64, u64, u64, usize);
/// Exact interval measurement or its retained diagnostic.
pub type WindowStats = Result<
    rspice_results::measurements::IntervalStatistics,
    rspice_results::measurements::MeasurementError,
>;
type CachedWindowStats = ((u64, u64), WindowStats);

/// Cache of series derived from waveform data (dB conversions), cleared when
/// the simulation data version changes.
#[derive(Debug, Clone, Default)]
pub struct DerivedSeries {
    version: u64,
    map: std::collections::HashMap<u64, SharedWaveformValues>,
    /// Cached finite (min, max) per series key — axis autoranges must not
    /// rescan millions of samples per frame.
    ranges: std::collections::HashMap<u64, Option<(f64, f64)>>,
    /// Block extremes per series key, so a pane fits the X window it shows
    /// without reading every sample in it on every frame.
    extrema: std::collections::HashMap<u64, std::sync::Arc<ExtremaIndex>>,
    /// Gap-aware overview knots, bounded by the caller's drawing budget.
    overview_indices: std::collections::HashMap<u64, std::sync::Arc<Vec<usize>>>,
    /// Cached windowed (min, max, rms) measurements, keyed by
    /// (series key, window-start bits, window-end bits).
    stats: std::collections::HashMap<(u64, usize), CachedWindowStats>,
    /// Cached monotone structure of a series' X column, per series key.
    ///
    /// Classifying costs one pass over the abscissa, and the renderer, the
    /// cursor readout and the marker hit test all need the same answer on
    /// every frame. It is held here rather than on the trace because only the
    /// cache knows when the data behind it changed.
    shapes: std::collections::HashMap<u64, std::sync::Arc<rspice_results::sampling::SweepShape>>,
    /// An analysis' committed event history as lanes, per analysis key.
    /// Reading every point is a pass over the history, so it is done once per
    /// data version rather than each time the strips are rebuilt.
    event_lanes: std::collections::HashMap<u64, std::sync::Arc<crate::waves::logic::EventLanes>>,
    /// A bus word reassembled from its members, per bus identity, so a
    /// reader's grouping is not reassembled on every frame it is drawn.
    bus_lanes: std::collections::HashMap<u64, std::sync::Arc<crate::waves::logic::BusLane>>,
    /// A curve paired with the abscissa its pane is drawn against, per pair
    /// identity: merging two grids is a pass over both, and the pane is drawn
    /// every frame.
    xy_curves: std::collections::HashMap<u64, XyCurveMemo>,
}

type XyCurveMemo = Result<std::sync::Arc<crate::waves::xy::XyCurve>, crate::waves::xy::XyRefusal>;

impl DerivedSeries {
    /// Observe the bounded measurement cache in application integration tests.
    #[cfg(feature = "test-support")]
    pub fn window_stats_count(&self) -> usize {
        self.stats.len()
    }

    /// Discard derived values when the host supplies a different data version.
    pub fn ensure_version(&mut self, version: u64) {
        if self.version != version {
            self.map.clear();
            self.ranges.clear();
            self.extrema.clear();
            self.overview_indices.clear();
            self.stats.clear();
            self.shapes.clear();
            self.event_lanes.clear();
            self.bus_lanes.clear();
            self.xy_curves.clear();
            self.version = version;
        }
    }

    pub fn overview_indices_or(
        &mut self,
        key: u64,
        build: impl FnOnce() -> Vec<usize>,
    ) -> std::sync::Arc<Vec<usize>> {
        std::sync::Arc::clone(
            self.overview_indices
                .entry(key)
                .or_insert_with(|| std::sync::Arc::new(build())),
        )
    }

    /// Fetch or classify the monotone structure of a series' X column.
    pub fn shape_or(
        &mut self,
        key: u64,
        build: impl FnOnce() -> rspice_results::sampling::SweepShape,
    ) -> std::sync::Arc<rspice_results::sampling::SweepShape> {
        if let Some(hit) = self.shapes.get(&key) {
            return std::sync::Arc::clone(hit);
        }
        let shape = std::sync::Arc::new(build());
        self.shapes.insert(key, std::sync::Arc::clone(&shape));
        shape
    }

    /// Fetch or read an analysis' event history as lanes.
    pub fn event_lanes_or(
        &mut self,
        key: u64,
        build: impl FnOnce() -> crate::waves::logic::EventLanes,
    ) -> std::sync::Arc<crate::waves::logic::EventLanes> {
        std::sync::Arc::clone(
            self.event_lanes
                .entry(key)
                .or_insert_with(|| std::sync::Arc::new(build())),
        )
    }

    /// Fetch or reassemble one bus word.
    pub fn bus_lane_or(
        &mut self,
        key: u64,
        build: impl FnOnce() -> crate::waves::logic::BusLane,
    ) -> std::sync::Arc<crate::waves::logic::BusLane> {
        std::sync::Arc::clone(
            self.bus_lanes
                .entry(key)
                .or_insert_with(|| std::sync::Arc::new(build())),
        )
    }

    /// Fetch or pair one curve with its pane's abscissa.
    pub fn xy_curve_or(
        &mut self,
        key: u64,
        build: impl FnOnce() -> Result<crate::waves::xy::XyCurve, crate::waves::xy::XyRefusal>,
    ) -> XyCurveMemo {
        self.xy_curves
            .entry(key)
            .or_insert_with(|| build().map(std::sync::Arc::new))
            .clone()
    }

    /// Fetch or compute the cached finite (min, max) of a series.
    pub fn range_or(
        &mut self,
        key: u64,
        build: impl FnOnce() -> Option<(f64, f64)>,
    ) -> Option<(f64, f64)> {
        *self.ranges.entry(key).or_insert_with(build)
    }

    /// Fetch or build the block extremes of `values` under `key`. A key
    /// whose summary was built from other values is rebuilt, so a reused key
    /// can never answer for a different series. `note_samples` hears how
    /// many samples a build read.
    pub fn extrema_or(
        &mut self,
        key: u64,
        values: &[f64],
        note_samples: impl FnOnce(usize),
    ) -> std::sync::Arc<ExtremaIndex> {
        if let Some(hit) = self.extrema.get(&key)
            && hit.describes(values)
        {
            return std::sync::Arc::clone(hit);
        }
        note_samples(values.len());
        let index = std::sync::Arc::new(ExtremaIndex::build(values));
        self.extrema.insert(key, std::sync::Arc::clone(&index));
        // The pass that built the summary read the whole-record range too,
        // and a reader of `range_or` under the same key (the shared X axis'
        // overview) need not make another.
        self.ranges
            .entry(key)
            .or_insert_with(|| index.whole().range());
        index
    }

    /// Keep one interval per source/branch. Cursor dragging must not retain a
    /// new measurement for every historical pointer position.
    pub fn stats_or(
        &mut self,
        key: WindowStatsKey,
        build: impl FnOnce() -> WindowStats,
    ) -> WindowStats {
        let (source, a, b, branch) = key;
        let window = (a, b);
        if let Some(&(cached_window, value)) = self.stats.get(&(source, branch))
            && cached_window == window
        {
            return value;
        }
        let value = build();
        self.stats.insert((source, branch), (window, value));
        value
    }

    /// Fetch or build a derived series under `key`.
    pub fn get_or(
        &mut self,
        key: u64,
        build: impl FnOnce() -> SharedWaveformValues,
    ) -> SharedWaveformValues {
        if let Some(hit) = self.map.get(&key) {
            return std::sync::Arc::clone(hit);
        }
        let series = build();
        self.map.insert(key, std::sync::Arc::clone(&series));
        series
    }

    /// 20·log₁₀ of a linear-magnitude series, cached under `key`.
    pub fn db(&mut self, key: u64, magnitude: &[f64]) -> SharedWaveformValues {
        self.get_or(key, || {
            std::sync::Arc::new(
                magnitude
                    .iter()
                    .map(|&m| 20.0 * m.log10())
                    .collect::<Vec<_>>(),
            )
        })
    }

    /// Key-space bit separating unwrapped-phase entries from the dB entries,
    /// which share the `(analysis << 32 | waveform)` key convention.
    const UNWRAP_KEY_BIT: u64 = 1 << 62;

    /// Key-space bit for nV/√Hz noise-density projections of retained
    /// V²/Hz PSD series, sharing the same key convention.
    const NOISE_DENSITY_KEY_BIT: u64 = 1 << 61;

    /// nV/√Hz spectral density of a retained V²/Hz PSD series, cached
    /// under `key` like `db`. Negative retained samples clamp to zero
    /// rather than inventing NaNs the plot would silently drop.
    pub fn noise_density_nv(&mut self, key: u64, psd_v2_per_hz: &[f64]) -> SharedWaveformValues {
        self.get_or(Self::NOISE_DENSITY_KEY_BIT | key, || {
            std::sync::Arc::new(
                psd_v2_per_hz
                    .iter()
                    .map(|&value| {
                        if value.is_finite() {
                            1.0e9 * value.max(0.0).sqrt()
                        } else {
                            f64::NAN
                        }
                    })
                    .collect::<Vec<_>>(),
            )
        })
    }

    /// Continuous (unwrapped) copy of a ±180°-wrapped phase-degree series,
    /// cached under `key` like `db`.
    pub fn unwrapped(&mut self, key: u64, phase_deg: &[f64]) -> SharedWaveformValues {
        self.get_or(Self::UNWRAP_KEY_BIT | key, || {
            std::sync::Arc::new(rspice_results::calculator::functions::unwrap_phase_deg(
                phase_deg,
            ))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_transforms_preserve_gaps_and_restart_phase_after_them() {
        let mut derived = DerivedSeries::default();
        let values = [1.0, f64::NAN, 4.0];
        assert!(derived.db(1, &values)[1].is_nan());
        let noise = derived.noise_density_nv(1, &values);
        assert!(noise[1].is_nan());
        assert_eq!(noise[2], 2.0e9);
        let phase = derived.unwrapped(1, &[170.0, -170.0, f64::NAN, -170.0, 170.0]);
        assert_eq!(&phase[..2], &[170.0, 190.0]);
        assert!(phase[2].is_nan());
        assert_eq!(&phase[3..], &[-170.0, -190.0]);
    }
}
