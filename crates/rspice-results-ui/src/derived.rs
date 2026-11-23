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
    /// A strip's overview miniature per trace and scale, read once per data
    /// version rather than on every frame the lane is drawn.
    overviews: std::collections::HashMap<u64, std::sync::Arc<OverviewMiniature>>,
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
    /// A bus word reassembled from its members, per bus row and reader, with
    /// the window its lanes were read over, so a reader's grouping is not
    /// reassembled on every frame it is drawn. A stored run's lanes are read a
    /// window at a time; the word over a window no longer shown is dropped
    /// when its row is read over another.
    bus_lanes:
        std::collections::HashMap<u64, (BusWindow, std::sync::Arc<crate::waves::logic::BusLane>)>,
    /// A curve paired with the abscissa its pane is drawn against, per pair
    /// identity: merging two grids is a pass over both, and the pane is drawn
    /// every frame.
    xy_curves: std::collections::HashMap<u64, XyCurveMemo>,
    /// The extent of a stored series over its whole record and over the last
    /// window asked of it, per series key: a steady view reads no tile for its
    /// fit.
    stored_extents: std::collections::HashMap<u64, StoredExtentMemo>,
}

/// A stored series' extent over its whole record, and over the last window
/// asked of it by its bounds' bits. A pane asks both on every frame, whether
/// it draws anything over the whole record and its fit over the window it
/// shows, so each is kept beside the other rather than in its place.
#[derive(Debug, Clone, Copy, Default)]
struct StoredExtentMemo {
    whole: Option<Option<rspice_results::waveform::ValueExtent>>,
    window: Option<((u64, u64), Option<rspice_results::waveform::ValueExtent>)>,
}

/// What a strip's overview lane draws of its trace: the vertical fit and the
/// gap-separated segments of the miniature.
#[derive(Debug, Clone, PartialEq)]
pub struct OverviewMiniature {
    pub range: Option<(f64, f64)>,
    pub segments: Vec<Vec<(f64, f64)>>,
}

/// The bits of the window a bus word's lanes were read over; `None` for lanes
/// that hold every point.
pub type BusWindow = Option<(u64, u64)>;

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
            self.overviews.clear();
            self.stats.clear();
            self.shapes.clear();
            self.event_lanes.clear();
            self.bus_lanes.clear();
            self.xy_curves.clear();
            self.stored_extents.clear();
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

    /// The overview miniature held under `key`, if one was kept.
    pub fn overview(&self, key: u64) -> Option<std::sync::Arc<OverviewMiniature>> {
        self.overviews.get(&key).cloned()
    }

    /// Keep `miniature` under `key` until the data version changes.
    pub fn keep_overview(
        &mut self,
        key: u64,
        miniature: OverviewMiniature,
    ) -> std::sync::Arc<OverviewMiniature> {
        let miniature = std::sync::Arc::new(miniature);
        self.overviews
            .insert(key, std::sync::Arc::clone(&miniature));
        miniature
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

    /// Fetch the bus word kept under `key` over `window`, or reassemble it,
    /// replacing the word kept for another window.
    pub fn bus_lane_or(
        &mut self,
        key: u64,
        window: BusWindow,
        build: impl FnOnce() -> crate::waves::logic::BusLane,
    ) -> std::sync::Arc<crate::waves::logic::BusLane> {
        if let Some((held, lane)) = self.bus_lanes.get(&key)
            && *held == window
        {
            return std::sync::Arc::clone(lane);
        }
        let lane = std::sync::Arc::new(build());
        self.bus_lanes
            .insert(key, (window, std::sync::Arc::clone(&lane)));
        lane
    }

    /// How many bus words are kept.
    #[cfg(any(test, feature = "test-support"))]
    pub fn bus_lane_count(&self) -> usize {
        self.bus_lanes.len()
    }

    /// Fetch or pair one curve with its pane's abscissa.
    /// The pairing memoised under `key`, if one is.
    pub fn xy_curve(&self, key: u64) -> Option<XyCurveMemo> {
        self.xy_curves.get(&key).cloned()
    }

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
    /// The extent of the stored series under `key` over `window`, or over its
    /// whole record when there is none: the whole record's built once per
    /// data version, a window's when it differs from the last window asked of
    /// the key.
    pub fn stored_extent_or(
        &mut self,
        key: u64,
        window: Option<(f64, f64)>,
        build: impl FnOnce() -> Option<rspice_results::waveform::ValueExtent>,
    ) -> Option<rspice_results::waveform::ValueExtent> {
        let memo = self.stored_extents.entry(key).or_default();
        let Some((x0, x1)) = window else {
            return *memo.whole.get_or_insert_with(build);
        };
        let bits = (x0.to_bits(), x1.to_bits());
        if let Some((held, extent)) = memo.window
            && held == bits
        {
            return extent;
        }
        let extent = build();
        memo.window = Some((bits, extent));
        extent
    }

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
            return hit.clone();
        }
        let series = build();
        self.map.insert(key, series.clone());
        series
    }

    /// The series `build` makes of every sample of `source`, cached under `key`. A stored source
    /// is read off the frame: until it has arrived the series is empty and is not kept.
    fn derive(
        &mut self,
        key: u64,
        source: &SharedWaveformValues,
        build: impl FnOnce(&[f64]) -> Vec<f64>,
    ) -> SharedWaveformValues {
        if let Some(hit) = self.map.get(&key) {
            return hit.clone();
        }
        let Some(samples) = source.fetch_whole().ready() else {
            return SharedWaveformValues::default();
        };
        let series = SharedWaveformValues::from(build(&samples));
        self.map.insert(key, series.clone());
        series
    }

    /// 20·log₁₀ of a linear-magnitude series, cached under `key`.
    pub fn db(&mut self, key: u64, magnitude: &SharedWaveformValues) -> SharedWaveformValues {
        self.derive(key, magnitude, |magnitude| {
            magnitude.iter().map(|&m| 20.0 * m.log10()).collect()
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
    pub fn noise_density_nv(
        &mut self,
        key: u64,
        psd_v2_per_hz: &SharedWaveformValues,
    ) -> SharedWaveformValues {
        self.derive(Self::NOISE_DENSITY_KEY_BIT | key, psd_v2_per_hz, |psd| {
            psd.iter()
                .map(|&value| {
                    if value.is_finite() {
                        1.0e9 * value.max(0.0).sqrt()
                    } else {
                        f64::NAN
                    }
                })
                .collect()
        })
    }

    /// Continuous (unwrapped) copy of a ±180°-wrapped phase-degree series,
    /// cached under `key` like `db`.
    pub fn unwrapped(
        &mut self,
        key: u64,
        phase_deg: &SharedWaveformValues,
    ) -> SharedWaveformValues {
        self.derive(
            Self::UNWRAP_KEY_BIT | key,
            phase_deg,
            rspice_results::calculator::functions::unwrap_phase_deg,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_transforms_preserve_gaps_and_restart_phase_after_them() {
        let mut derived = DerivedSeries::default();
        let values = SharedWaveformValues::from(vec![1.0, f64::NAN, 4.0]);
        assert!(derived.db(1, &values).in_memory().unwrap()[1].is_nan());
        let noise = derived.noise_density_nv(1, &values);
        assert!(noise.in_memory().unwrap()[1].is_nan());
        assert_eq!(noise.in_memory().unwrap()[2], 2.0e9);
        let phase = derived.unwrapped(
            1,
            &SharedWaveformValues::from(vec![170.0, -170.0, f64::NAN, -170.0, 170.0]),
        );
        assert_eq!(&phase.in_memory().unwrap()[..2], &[170.0, 190.0]);
        assert!(phase.in_memory().unwrap()[2].is_nan());
        assert_eq!(&phase.in_memory().unwrap()[3..], &[-170.0, -190.0]);
    }
}

#[cfg(test)]
mod stored_extent_tests {
    use super::DerivedSeries;
    use rspice_results::waveform::ValueExtent;

    /// A pane asks a stored curve's extent over its whole record and over the
    /// window it shows on every frame: neither answer is read again while the
    /// view stands, and a new window is read once.
    #[test]
    fn a_curves_whole_and_windowed_extents_are_each_read_once_while_the_view_stands() {
        let mut derived = DerivedSeries::default();
        let reads = std::cell::Cell::new(0);
        let extent = |min: f64| ValueExtent {
            min,
            max: min + 1.0,
            min_positive: min + 1.0,
        };
        let read = |min: f64| {
            reads.set(reads.get() + 1);
            Some(extent(min))
        };
        for _frame in 0..5 {
            assert_eq!(
                derived.stored_extent_or(7, None, || read(0.0)),
                Some(extent(0.0))
            );
            assert_eq!(
                derived.stored_extent_or(7, Some((1.0, 2.0)), || read(5.0)),
                Some(extent(5.0))
            );
        }
        assert_eq!(
            reads.get(),
            2,
            "one read each for the whole record and the window"
        );
        assert_eq!(
            derived.stored_extent_or(7, Some((1.0, 3.0)), || read(6.0)),
            Some(extent(6.0))
        );
        assert_eq!(
            derived.stored_extent_or(7, None, || read(9.0)),
            Some(extent(0.0)),
            "the whole record's extent outlives a change of window"
        );
        assert_eq!(reads.get(), 3);
    }
}

#[cfg(test)]
mod bus_word_tests {
    use super::DerivedSeries;
    use crate::waves::logic::BusLane;

    /// A user bus's word read over window after window, as a stored run's
    /// lanes are while the reader pans, keeps only the word over the window
    /// the reader is shown.
    #[test]
    fn a_bus_word_over_a_window_no_longer_shown_is_dropped() {
        let mut derived = DerivedSeries::default();
        let builds = std::cell::Cell::new(0);
        let build = || {
            builds.set(builds.get() + 1);
            BusLane::from_logic_lanes("bus1".to_owned(), &[])
        };
        for window in 0..20_u64 {
            let _ = derived.bus_lane_or(7, Some((window, window + 1)), build);
        }
        assert_eq!(derived.bus_lane_count(), 1);
        let _ = derived.bus_lane_or(7, Some((19, 20)), build);
        assert_eq!(builds.get(), 20, "the word over the window shown is kept");
    }
}
