//! Trace decimation — per-pixel-column min/max envelopes, cached.
//!
//! Stroking a million-point transient directly would tessellate a million
//! segments per frame. Instead each trace is reduced to at most two points
//! per pixel column (the column's min and max, in encounter order, which
//! preserves the drawn envelope exactly). The reduction is a single O(n)
//! pass and is cached keyed by (data identity, column count, x-range), so a
//! steady view costs only the cheap data→screen transform per frame.

use std::collections::HashMap;
use std::sync::Arc;

mod blocks;

use rspice_results::waveform::{ColumnTile, Fetched, ViewTile};

use super::sample::{LogicalView, SweepClass, SweepShape, XOrientation};
use super::scale::XScale;
#[cfg(test)]
use rspice_results::sampling::sample_at_with;
pub use rspice_results::sampling::{SampleInterpolation, sample_at};

/// The columns a plot `width` points wide is decimated into at `pixels_per_point`: one per
/// device pixel, so a dense trace at 250 % draws as a solid stroke and a stored trace is read at
/// the screen's resolution, rounded up to a multiple of 64 so a live window resize reuses the
/// envelopes it cached instead of reducing every trace every frame.
pub fn display_columns(width: f32, pixels_per_point: f32) -> usize {
    ((width * pixels_per_point).ceil().max(0.0) as usize).next_multiple_of(64)
}

/// Cache of decimated envelopes. Owned by the results workspace state;
/// cleared wholesale when the data version changes.
#[derive(Debug, Clone)]
pub struct DecimationCache {
    /// Everything the cache holds, behind one allocation: the cache sits
    /// inline in the results workspace's state, whose size is budgeted.
    held: Box<Held>,
    /// Retained source identity when trace keys are scoped to a source owner.
    source: Option<rspice_vocabulary::source_revision::SourceRevision>,
    /// Data version the cache contents belong to.
    version: u64,
    /// Frame tick, advanced once per frame by `ensure_version` — eviction
    /// keeps entries the current frame touched instead of nuking the hot
    /// working set.
    tick: u64,
    /// Hard resident-memory budget for reconstructable display envelopes.
    byte_capacity: usize,
    resident_bytes: usize,
}

#[derive(Debug, Clone, Default)]
struct Held {
    map: HashMap<CacheKey, Entry>,
    /// Block extrema of the long ascending traces the last frames reduced,
    /// so a pan or zoom re-reduces them a block at a time. Each is about a
    /// twentieth of its trace's values and lives only while frames keep
    /// drawing from it.
    blocks: HashMap<u64, (Arc<blocks::BlockIndex>, u64)>,
    #[cfg(any(test, feature = "test-support"))]
    work: DecimationWork,
}

/// What the cache has done since it was made, for the frame-cost gates.
///
/// A steady or hovered frame is entitled to reuse every envelope it drew on
/// the frame before. Counting the rebuilds is how a gate holds it to that
/// without timing anything.
#[cfg(any(test, feature = "test-support"))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DecimationWork {
    /// Lookups answered from a retained envelope.
    pub hits: u64,
    /// Envelopes rebuilt from the retained samples, one O(n) pass each.
    pub rebuilds: u64,
    /// Retained envelopes dropped to admit another.
    pub evictions: u64,
}

#[cfg(any(test, feature = "test-support"))]
impl DecimationWork {
    /// The work done since `earlier` was read.
    #[must_use]
    pub const fn since(self, earlier: Self) -> Self {
        Self {
            hits: self.hits - earlier.hits,
            rebuilds: self.rebuilds - earlier.rebuilds,
            evictions: self.evictions - earlier.evictions,
        }
    }
}

#[derive(Debug, Clone)]
struct Entry {
    envelope: Arc<[[f64; 2]]>,
    last_used: u64,
}

/// Which reduction produced an entry. Two of them answer different questions
/// about the same trace, so it belongs in the key.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Reduction {
    /// X-ordered, read in source order.
    Ordered,
    /// X-ordered, read from the far end: a reverse sweep.
    OrderedReversed,
    /// Order-agnostic screen-cell walk: a locus or a multi-branch sweep.
    PixelCells,
    /// Per-column extremes of a whole set of curves; see
    /// [`DecimationCache::band`].
    Band,
    /// Per-pixel extremes from a results store's tiles; see
    /// [`DecimationCache::stored_series`].
    Tiles,
}

impl Reduction {
    /// Whether this reduction quantizes the Y axis too.
    fn reads_y(self) -> bool {
        matches!(self, Reduction::PixelCells)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct CacheKey {
    trace: u64,
    columns: u32,
    x0_bits: u64,
    x1_bits: u64,
    /// Screen quantization of the Y axis. Only a cell reduction reads the Y
    /// projection, so an X-ordered trace deliberately keys these to zero:
    /// including a live autoscale bound would invalidate every envelope each
    /// time a pane refits, which is exactly the cost this cache exists to
    /// avoid.
    rows: u32,
    y0_bits: u64,
    y1_bits: u64,
    reduction: Reduction,
    /// Both scales place the column boundaries the reduction quantizes
    /// against, so a scale change is a different reduction of the same data.
    x_scale: XScale,
    y_scale: XScale,
}

/// The data→screen projection a decimated series is quantized against.
///
/// This mirrors `render`'s `mx`/`my` exactly. A reduction is only valid for
/// the projection that produced it, so every field belongs in the cache key.
#[derive(Debug, Clone, Copy)]
pub struct TraceView {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub x_scale: XScale,
    pub y_scale: XScale,
    pub columns: usize,
    pub rows: usize,
}

/// Entry bound for envelopes no frame is drawing any more.
///
/// It bounds the views the reader has left, never the one on screen. A frame
/// that draws more traces than this keeps every envelope it drew: with a hard
/// bound, a 500-run family read in a cycle missed on every lookup and rebuilt
/// every envelope on every repaint, a hover included. What the current frame
/// draws is bounded by the byte budget and [`ENTRY_CEILING`] alone.
const STALE_ENTRY_CAP: usize = 256;

/// Absolute entry bound, the current frame's own envelopes included.
///
/// Eviction scans every entry for the least recently used one, so the scan
/// stays cheap even for a caller that never advances the frame tick.
const ENTRY_CEILING: usize = 16_384;

/// Samples per column, across a trace's whole extent, from which its block
/// index pays for itself.
///
/// Building one reads the trace once more. A 100k-sample trace on a
/// 1 728-column strip holds about sixty samples a column, under one block, so
/// its index never skips anything and its first reduction cost 15% more than
/// reading the samples; a 5M-sample trace holds about 2 900, and its first
/// reduction cost 26% less and every pan frame after it a ninth.
const BLOCK_INDEX_SAMPLES_PER_COLUMN: usize = 256;

/// Default resident budget for reconstructable display envelopes.
///
/// Every entry here is derived data that costs one O(n) pass to rebuild, so
/// the budget buys latency, not correctness. A workstation can spare half a
/// gigabyte for it; a phone browser cannot — the whole WebAssembly heap is
/// often smaller than that, and the retained waveforms have to live in it too.
#[cfg(not(target_arch = "wasm32"))]
const DEFAULT_DISPLAY_CACHE_MIB: u32 = 512;
#[cfg(target_arch = "wasm32")]
const DEFAULT_DISPLAY_CACHE_MIB: u32 = 96;

const DEFAULT_BYTE_CAPACITY: usize = DEFAULT_DISPLAY_CACHE_MIB as usize * 1024 * 1024;

impl Default for DecimationCache {
    fn default() -> Self {
        Self {
            held: Box::default(),
            source: None,
            version: 0,
            tick: 0,
            byte_capacity: DEFAULT_BYTE_CAPACITY,
            resident_bytes: 0,
        }
    }
}

impl DecimationCache {
    /// Everything the cache has done since it was made.
    #[cfg(any(test, feature = "test-support"))]
    pub fn work(&self) -> DecimationWork {
        self.held.work
    }

    /// Bind logical trace keys to their current source before reading them.
    /// A restored source or an edited clone can reuse numeric IDs and versions;
    /// retaining this small revision prevents either from reusing old samples.
    pub fn ensure_source(&mut self, source: &rspice_vocabulary::source_revision::SourceRevision) {
        if self.source.as_ref() != Some(source) {
            self.invalidate();
            self.source = Some(source.clone());
        }
    }

    /// Discard derived samples while preserving the configured memory budget.
    pub fn invalidate(&mut self) {
        self.held.map.clear();
        self.held.blocks.clear();
        self.resident_bytes = 0;
    }

    /// Drop everything if `version` differs from the cached one (new run /
    /// new analysis selection), and advance the frame tick.
    pub fn ensure_version(&mut self, version: u64) {
        if self.version != version {
            self.invalidate();
            self.version = version;
        }
        self.tick = self.tick.wrapping_add(1);
    }

    fn evict_to_fit(&mut self, incoming_bytes: usize) {
        while !self.held.map.is_empty() {
            let over_budget =
                self.resident_bytes.saturating_add(incoming_bytes) > self.byte_capacity;
            let over_ceiling = self.held.map.len() >= ENTRY_CEILING;
            if !over_budget && !over_ceiling && self.held.map.len() < STALE_ENTRY_CAP {
                break;
            }
            let Some((oldest, last_used)) = self
                .held
                .map
                .iter()
                .min_by_key(|(_, entry)| entry.last_used)
                .map(|(key, entry)| (*key, entry.last_used))
            else {
                break;
            };
            // Past the stale bound alone, only an envelope this frame has not
            // drawn gives way; see `STALE_ENTRY_CAP`.
            if !over_budget && !over_ceiling && last_used == self.tick {
                break;
            }
            if let Some(removed) = self.held.map.remove(&oldest) {
                self.resident_bytes = self
                    .resident_bytes
                    .saturating_sub(removed.envelope.len() * size_of::<[f64; 2]>());
                #[cfg(any(test, feature = "test-support"))]
                {
                    self.held.work.evictions += 1;
                }
            }
        }
    }

    /// Fetch or compute a bounded display series for one trace at the given
    /// view. Full-resolution rendering deliberately bypasses this cache.
    ///
    /// `parametric` and `shape` select the reduction, and both are properties
    /// of the data, never of the user's display preference: a locus revisits X
    /// values and a hysteresis loop answers twice, so the X-ordered reductions
    /// would be answering a question those curves have no single answer to.
    /// See [`decimate_pixel_cells`]. A trace that declares no shape is read as
    /// ascending, which is what every caller meant before shapes existed.
    pub fn series(
        &mut self,
        trace_key: u64,
        x: &[f64],
        y: &[f64],
        view: TraceView,
        parametric: bool,
        shape: Option<&SweepShape>,
    ) -> Arc<[[f64; 2]]> {
        let reduction = reduction_for(parametric, shape);
        let reads_y = reduction.reads_y();
        let key = CacheKey {
            trace: trace_key,
            columns: view.columns as u32,
            x0_bits: view.x0.to_bits(),
            x1_bits: view.x1.to_bits(),
            rows: if reads_y { view.rows as u32 } else { 0 },
            y0_bits: if reads_y { view.y0.to_bits() } else { 0 },
            y1_bits: if reads_y { view.y1.to_bits() } else { 0 },
            reduction,
            x_scale: view.x_scale,
            y_scale: view.y_scale,
        };
        if let Some(hit) = self.lookup(&key) {
            return hit;
        }
        let ascending = shape.is_some_and(|shape| shape.class() == SweepClass::Ascending);
        let envelope: Arc<[[f64; 2]]> = match reduction {
            Reduction::PixelCells => decimate_pixel_cells(x, y, view),
            // A long ascending trace is reduced a block at a time; the result
            // is the reference reduction's, point for point.
            Reduction::Ordered
                if ascending
                    && x.len().min(y.len()) >= view.columns * BLOCK_INDEX_SAMPLES_PER_COLUMN =>
            {
                self.block_index(trace_key, x, y)
                    .and_then(|index| {
                        index.decimate_ascending(x, y, view.x0, view.x1, view.x_scale, view.columns)
                    })
                    .unwrap_or_else(|| {
                        decimate_minmax(
                            x,
                            y,
                            view.x0,
                            view.x1,
                            view.x_scale,
                            view.columns,
                            XOrientation::Ascending,
                        )
                    })
            }
            // `reduction_for` never selects a band or tiles; only `band` and
            // `stored_series` key them.
            Reduction::Ordered
            | Reduction::OrderedReversed
            | Reduction::Band
            | Reduction::Tiles => {
                let orientation = if reduction == Reduction::OrderedReversed {
                    XOrientation::Descending
                } else {
                    XOrientation::Ascending
                };
                decimate_minmax(
                    x,
                    y,
                    view.x0,
                    view.x1,
                    view.x_scale,
                    view.columns,
                    orientation,
                )
            }
        }
        .into();
        self.retain(key, &envelope);
        envelope
    }

    /// [`Self::series`] of samples a results store keeps: the store's tile of
    /// the view, read without reading either column. The reader service
    /// reads it off the frame; until it has, a coarser tile it holds is drawn,
    /// and kept for no later frame, so the view sharpens as its own tile
    /// arrives. Without a `trace_key` the tile is asked for every frame.
    pub fn stored_series(
        &mut self,
        trace_key: Option<u64>,
        stored: super::spec::StoredSamples<'_>,
        view: TraceView,
    ) -> Arc<[[f64; 2]]> {
        let Some(trace_key) = trace_key else {
            return stored_envelope(stored, view).0;
        };
        let key = CacheKey {
            trace: trace_key,
            columns: view.columns as u32,
            x0_bits: view.x0.to_bits(),
            x1_bits: view.x1.to_bits(),
            rows: 0,
            y0_bits: 0,
            y1_bits: 0,
            reduction: Reduction::Tiles,
            x_scale: view.x_scale,
            y_scale: view.y_scale,
        };
        if let Some(hit) = self.lookup(&key) {
            return hit;
        }
        let (envelope, complete) = stored_envelope(stored, view);
        if complete {
            self.retain(key, &envelope);
        }
        envelope
    }

    /// The per-column extremes of a whole set of curves, for a pane that draws
    /// the members of a large family as the band they occupy.
    ///
    /// One row per screen column of `view`: the least and the greatest value
    /// any member takes in that column, or NaN in both where none has a
    /// sample to show. On a decade Y scale a value at or below zero has no
    /// place on screen and is left out, as the curve's own stroke leaves it.
    ///
    /// `band_key` names the member set, so a frame that finds the band cached
    /// reads none of the members' samples at all. A band whose stored members'
    /// tiles have not all arrived folds what has, and is kept once they have.
    pub fn band(
        &mut self,
        band_key: u64,
        members: &[BandMember<'_>],
        view: TraceView,
    ) -> Arc<[[f64; 2]]> {
        let key = band_cache_key(band_key, view);
        if let Some(hit) = self.lookup(&key) {
            return hit;
        }
        let (band, complete) = band_columns(members, view);
        let band: Arc<[[f64; 2]]> = band.into();
        if complete {
            self.retain(key, &band);
        }
        band
    }

    /// The band [`Self::band`] keeps under `band_key` for `view`, if it keeps
    /// one: a caller whose members' samples are costly to hand over asks
    /// first.
    pub fn held_band(&mut self, band_key: u64, view: TraceView) -> Option<Arc<[[f64; 2]]>> {
        self.lookup(&band_cache_key(band_key, view))
    }

    /// The block extrema of one trace, read now if no index of these very
    /// samples is held. Building one drops the indices no frame has drawn
    /// from since the frame before.
    fn block_index(
        &mut self,
        trace_key: u64,
        x: &[f64],
        y: &[f64],
    ) -> Option<Arc<blocks::BlockIndex>> {
        let tick = self.tick;
        if !self
            .held
            .blocks
            .get(&trace_key)
            .is_some_and(|(index, _)| index.describes(x, y))
        {
            let index = Arc::new(blocks::BlockIndex::build(x, y)?);
            self.held
                .blocks
                .retain(|_, (_, last_used)| last_used.saturating_add(1) >= tick);
            self.held.blocks.insert(trace_key, (index, tick));
        }
        let (index, last_used) = self.held.blocks.get_mut(&trace_key)?;
        *last_used = tick;
        Some(Arc::clone(index))
    }

    fn lookup(&mut self, key: &CacheKey) -> Option<Arc<[[f64; 2]]>> {
        let hit = self.held.map.get_mut(key)?;
        hit.last_used = self.tick;
        #[cfg(any(test, feature = "test-support"))]
        {
            self.held.work.hits += 1;
        }
        Some(Arc::clone(&hit.envelope))
    }

    fn retain(&mut self, key: CacheKey, envelope: &Arc<[[f64; 2]]>) {
        #[cfg(any(test, feature = "test-support"))]
        {
            self.held.work.rebuilds += 1;
        }
        let bytes = envelope.len() * size_of::<[f64; 2]>();
        if bytes <= self.byte_capacity {
            self.evict_to_fit(bytes);
            self.held.map.insert(
                key,
                Entry {
                    envelope: Arc::clone(envelope),
                    last_used: self.tick,
                },
            );
            self.resident_bytes = self.resident_bytes.saturating_add(bytes);
        }
    }
}

/// One member of a banded family, as [`DecimationCache::band`] reads it.
///
/// `parametric` and `shape` are the curve's data facts, exactly as
/// [`DecimationCache::series`] takes them: they decide whether the visible
/// window can be found by bisection or the whole curve has to be read.
#[derive(Debug, Clone, Copy)]
pub struct BandMember<'a> {
    pub x: &'a [f64],
    pub y: &'a [f64],
    pub parametric: bool,
    pub shape: Option<&'a SweepShape>,
    /// Samples a results store keeps, folded from its tile of the view rather than read; `x`
    /// and `y` are then empty.
    pub stored: Option<super::spec::StoredSamples<'a>>,
}

/// Fold every member's stroke into the per-column extremes of `view`.
///
/// A column reads every sample that falls in it, and also every segment that
/// crosses its centre, at the crossing: a member zoomed in past one sample
/// per column is still a continuous stroke, and its band must not fall apart
/// into the columns that happen to hold a sample. A hole breaks the stroke,
/// so it breaks the segment across it too.
///
/// The columns are the ones [`decimate_minmax`] quantizes against, so the
/// band's edge is where the members' own envelopes would have stroked it.
/// Where a band over `view` is kept.
fn band_cache_key(band_key: u64, view: TraceView) -> CacheKey {
    CacheKey {
        trace: band_key,
        columns: view.columns as u32,
        x0_bits: view.x0.to_bits(),
        x1_bits: view.x1.to_bits(),
        rows: 0,
        y0_bits: 0,
        y1_bits: 0,
        reduction: Reduction::Band,
        x_scale: view.x_scale,
        y_scale: view.y_scale,
    }
}

fn band_columns(members: &[BandMember<'_>], view: TraceView) -> (Vec<[f64; 2]>, bool) {
    let columns = view.columns;
    let mut out = vec![[f64::NAN; 2]; columns];
    if columns == 0
        || !matches!(
            view.x1.partial_cmp(&view.x0),
            Some(std::cmp::Ordering::Greater)
        )
    {
        return (out, true);
    }
    let mut complete = true;
    let width = columns as f64;
    let decade_y = view.y_scale == XScale::Log10;
    let mut fold = |column: usize, value: f64| {
        let cell = &mut out[column];
        // `f64::min` and `max` answer the other operand when one is NaN.
        cell[0] = cell[0].min(value);
        cell[1] = cell[1].max(value);
    };
    for member in members {
        // A stored member's tile holds each column's extremes; a view few
        // enough of its rows fall in is those rows, stroked as any member's.
        let tile_rows;
        let mut member = *member;
        if let Some(stored) = member.stored {
            let log = view.x_scale == XScale::Log10;
            let tile = match stored
                .y
                .fetch_tile(stored.x, log, view.x0, view.x1, columns)
            {
                Fetched::Ready(tile) => tile,
                Fetched::Pending => {
                    complete = false;
                    continue;
                }
                // Samples the store cannot tile are not folded: reading them
                // would read the column whole.
                Fetched::Refused(_) => continue,
            };
            complete &= tile.exact;
            match tile.tile.as_ref() {
                ColumnTile::Envelope { min, max, touched } => {
                    let pixels = min.len();
                    for pixel in (0..pixels).filter(|&pixel| touched[pixel]) {
                        for column in tile_columns(&tile, pixel, pixels, view)
                            .into_iter()
                            .flatten()
                        {
                            for value in [min[pixel], max[pixel]] {
                                if value.is_finite() && !(decade_y && value <= 0.0) {
                                    fold(column, value);
                                }
                            }
                        }
                    }
                    continue;
                }
                ColumnTile::Samples {
                    x,
                    y,
                    before,
                    after,
                } => {
                    let rows: Vec<(f64, f64)> = before
                        .iter()
                        .copied()
                        .chain(x.iter().copied().zip(y.iter().copied()))
                        .chain(after.iter().copied())
                        .collect();
                    tile_rows = rows.into_iter().unzip::<f64, f64, Vec<f64>, Vec<f64>>();
                    member.x = &tile_rows.0;
                    member.y = &tile_rows.1;
                }
            }
        }
        // An X-ordered member is read over its visible window, with one
        // sample of margin each side so the strokes into the plot edges are
        // read too; anything else is read whole, in source order.
        let (logical, window) = match reduction_for(member.parametric, member.shape) {
            Reduction::PixelCells | Reduction::Band | Reduction::Tiles => {
                let logical = LogicalView::new(member.x, member.y, false);
                (logical, 0..logical.paired_len())
            }
            reduction @ (Reduction::Ordered | Reduction::OrderedReversed) => {
                let logical =
                    LogicalView::new(member.x, member.y, reduction == Reduction::OrderedReversed);
                let start = logical.below(view.x0).saturating_sub(1);
                let end = (logical.at_or_below(view.x1) + 1).min(logical.paired_len());
                (logical, start..end)
            }
        };
        // The previous sample's screen position and its value in the space
        // the stroke is straight in: a decade pane strokes straight in log Y.
        let mut previous: Option<(f64, f64)> = None;
        for index in window {
            let (x, y) = (logical.x(index), logical.y(index));
            let t = view.x_scale.normalize(x, view.x0, view.x1);
            if !(x.is_finite() && y.is_finite() && t.is_finite()) || (decade_y && y <= 0.0) {
                previous = None;
                continue;
            }
            let value = if decade_y { y.log10() } else { y };
            if (0.0..=1.0).contains(&t) {
                fold(((t * width) as usize).min(columns - 1), y);
            }
            if let Some((t0, value0)) = previous {
                let (low, high) = (t0.min(t), t0.max(t));
                // Centres strictly inside the segment: (column + 0.5) / width.
                let first = (low.mul_add(width, -0.5).floor() + 1.0).max(0.0);
                let last = high.mul_add(width, -0.5).ceil() - 1.0;
                let last = last.min(width - 1.0);
                if first <= last {
                    for column in first as usize..=last as usize {
                        let centre = (column as f64 + 0.5) / width;
                        let crossing = (value - value0).mul_add((centre - t0) / (t - t0), value0);
                        fold(
                            column,
                            if decade_y {
                                10f64.powf(crossing)
                            } else {
                                crossing
                            },
                        );
                    }
                }
            }
            previous = Some((t, value));
        }
    }
    (out, complete)
}

/// The columns of `view` that pixel `pixel` of a tile of `pixels` pixels folds into: every
/// column whose centre the pixel spans, or the column of its middle when it spans none, as a
/// pixel narrower than a column does. `None` for a pixel off the view.
fn tile_columns(
    tile: &ViewTile,
    pixel: usize,
    pixels: usize,
    view: TraceView,
) -> Option<std::ops::RangeInclusive<usize>> {
    let width = view.columns as f64;
    let at = |x: f64| view.x_scale.normalize(x, view.x0, view.x1) * width;
    let (start, end) = (
        at(tile.edge(pixel, pixels)),
        at(tile.edge(pixel + 1, pixels)),
    );
    let first = (start - 0.5).ceil().max(0.0);
    let last = ((end - 0.5).ceil() - 1.0).min(width - 1.0);
    if first <= last {
        return Some(first as usize..=last as usize);
    }
    let middle = at(tile.middle(pixel, pixels));
    (0.0..width).contains(&middle).then(|| {
        let column = middle as usize;
        column..=column
    })
}

/// Which reduction a trace's declared data facts select.
fn reduction_for(parametric: bool, shape: Option<&SweepShape>) -> Reduction {
    if parametric {
        return Reduction::PixelCells;
    }
    match shape.map(SweepShape::class) {
        None | Some(SweepClass::Empty | SweepClass::Ascending) => Reduction::Ordered,
        Some(SweepClass::Descending) => Reduction::OrderedReversed,
        Some(SweepClass::MultiBranch | SweepClass::NonSweep) => Reduction::PixelCells,
    }
}

/// Reduce `(x, y)` to a per-column min/max envelope over `[x0, x1]` split
/// into `columns` equal screen columns. Returns the raw points when the
/// range already holds fewer than `2 × columns` samples.
///
/// # Precondition
///
/// `x` must be monotone in the direction `orientation` names. Both the
/// visible-window binary search and the per-column reduction below depend on
/// it, and neither fails loudly without it — a curve that turns around loses
/// whichever branch falls outside the contiguous index window. A trace that
/// turns around declares a [`SweepShape`] and is routed to
/// [`decimate_pixel_cells`] instead.
pub fn decimate_minmax(
    x: &[f64],
    y: &[f64],
    x0: f64,
    x1: f64,
    x_scale: XScale,
    columns: usize,
    orientation: XOrientation,
) -> Vec<[f64; 2]> {
    let view = LogicalView::new(x, y, orientation == XOrientation::Descending);
    let n = view.paired_len();
    if n == 0 || columns == 0 || !matches!(x1.partial_cmp(&x0), Some(std::cmp::Ordering::Greater)) {
        return Vec::new();
    }
    // Visible index range, in ascending-X order whichever way the sweep ran.
    // Keep one sample of margin so strokes continue to the plot edge.
    let start = view.below(x0).saturating_sub(1);
    let end = (view.at_or_below(x1) + 1).min(n);
    let visible = end.saturating_sub(start);

    if visible <= columns * 2 {
        return (start..end)
            .map(|index| [view.x(index), view.y(index)])
            .collect();
    }

    // Past this point the ordering decides the output, so a violation stops
    // being a small inaccuracy and starts deleting whole branches of the
    // curve. Non-finite abscissae are ordinary — a diverged run carries them
    // and `finite_runs` breaks the stroke around them downstream — so the
    // precondition is only about the samples that do carry a position.
    debug_assert!(
        (start..end)
            .map(|index| view.x(index))
            .filter(|value| value.is_finite())
            .try_fold(f64::NEG_INFINITY, |previous, value| (previous <= value)
                .then_some(value))
            .is_some(),
        "decimate_minmax requires monotone x; a curve that turns around belongs in \
         decimate_pixel_cells"
    );

    let mut out: Vec<[f64; 2]> = Vec::with_capacity(columns * 2 + 4);
    let mut column = 0usize;
    let mut min_v = f64::INFINITY;
    let mut max_v = f64::NEG_INFINITY;
    let mut min_i = 0usize;
    let mut max_i = 0usize;
    let mut col_x = 0.0f64;
    let mut has = false;

    let flush = |out: &mut Vec<[f64; 2]>,
                 col_x: f64,
                 min_v: f64,
                 max_v: f64,
                 min_i: usize,
                 max_i: usize| {
        // Emit min/max in encounter order so the stroke zig-zags the way the
        // raw data would, not always downward.
        if min_i <= max_i {
            out.push([col_x, min_v]);
            if max_i != min_i {
                out.push([col_x, max_v]);
            }
        } else {
            out.push([col_x, max_v]);
            out.push([col_x, min_v]);
        }
    };

    for i in start..end {
        let (xv, yv) = (view.x(i), view.y(i));
        // A hole is a fact about the run, not a sample to fold into an
        // extremum: it neither raises the maximum nor lowers the minimum, so
        // an envelope that swallowed it would stroke straight through a gap
        // the full-resolution path breaks at. Emit it, and close the column
        // either side, so `finite_runs` splits the envelope in the same place.
        if !(xv.is_finite() && yv.is_finite()) {
            if has {
                flush(&mut out, col_x, min_v, max_v, min_i, max_i);
                min_v = f64::INFINITY;
                max_v = f64::NEG_INFINITY;
                has = false;
            }
            out.push([xv, yv]);
            continue;
        }
        let t = x_scale.normalize(xv, x0, x1).clamp(0.0, 1.0);
        let c = ((t * columns as f64) as usize).min(columns - 1);
        if c != column && has {
            flush(&mut out, col_x, min_v, max_v, min_i, max_i);
            min_v = f64::INFINITY;
            max_v = f64::NEG_INFINITY;
        }
        column = c;
        if yv < min_v {
            min_v = yv;
            min_i = i;
        }
        if yv > max_v {
            max_v = yv;
            max_i = i;
        }
        col_x = xv;
        has = true;
    }
    if has {
        flush(&mut out, col_x, min_v, max_v, min_i, max_i);
    }
    out
}

/// The envelope of a stored trace over `view`, and whether it is the view's own: the store's
/// tile, read through `reader`, each pixel's least and greatest value at the pixel's middle in
/// the order that strokes it, evenly in log10 x on a decade axis. Only the tile's pixels that
/// reach the view are drawn, with one either side so the stroke enters and leaves the plot where
/// the data does. A tile not yet read draws the coarser one standing in for it, or nothing.
/// Samples the store cannot tile (an axis that does not ascend, or a decade view reaching zero)
/// are reduced as [`decimate_minmax`] reduces them when the loaded results may hold them whole,
/// read off the frame, and are not drawn when they may not.
fn stored_envelope(
    stored: super::spec::StoredSamples<'_>,
    view: TraceView,
) -> (Arc<[[f64; 2]]>, bool) {
    if view.columns == 0
        || !matches!(
            view.x1.partial_cmp(&view.x0),
            Some(std::cmp::Ordering::Greater)
        )
    {
        return (Arc::from([]), true);
    }
    let log = view.x_scale == XScale::Log10;
    let tile = match stored
        .y
        .fetch_tile(stored.x, log, view.x0, view.x1, view.columns)
    {
        Fetched::Ready(tile) => tile,
        Fetched::Pending => return (Arc::from([]), false),
        Fetched::Refused(_) => {
            return match (stored.x.fetch_whole(), stored.y.fetch_whole()) {
                (Fetched::Ready(x), Fetched::Ready(y)) => (
                    decimate_minmax(
                        &x,
                        &y,
                        view.x0,
                        view.x1,
                        view.x_scale,
                        view.columns,
                        XOrientation::Ascending,
                    )
                    .into(),
                    true,
                ),
                (Fetched::Pending, _) | (_, Fetched::Pending) => (Arc::from([]), false),
                _ => (Arc::from([]), true),
            };
        }
    };
    let envelope = match tile.tile.as_ref() {
        ColumnTile::Samples {
            x,
            y,
            before,
            after,
        } => before
            .iter()
            .copied()
            .chain(x.iter().copied().zip(y.iter().copied()))
            .chain(after.iter().copied())
            .map(|(x, y)| [x, y])
            .collect(),
        ColumnTile::Envelope { min, max, touched } => {
            let pixels = min.len();
            // The pixels that reach the view, and one either side.
            let (u0, u1) = (tile.edge(0, pixels), tile.edge(pixels, pixels));
            let fraction = |x: f64| view.x_scale.normalize(x, u0, u1) * pixels as f64;
            let first = (fraction(view.x0).floor().max(0.0) as usize).saturating_sub(1);
            let last = ((fraction(view.x1).ceil().max(0.0) as usize).saturating_add(1)).min(pixels);
            let mut out = Vec::with_capacity(2 * last.saturating_sub(first));
            let mut previous = f64::NAN;
            for pixel in first..last {
                if !touched[pixel] {
                    continue;
                }
                let x = tile.middle(pixel, pixels);
                let (lo, hi) = (min[pixel], max[pixel]);
                if lo.is_nan() || hi.is_nan() {
                    out.push([x, f64::NAN]);
                    previous = f64::NAN;
                    continue;
                }
                let (first, second) = if (hi - previous).abs() < (lo - previous).abs() {
                    (hi, lo)
                } else {
                    (lo, hi)
                };
                out.push([x, first]);
                if second != first {
                    out.push([x, second]);
                }
                previous = second;
            }
            out.into()
        }
    };
    (envelope, tile.exact)
}

/// Screen cell index of one sample under `view`'s projection.
///
/// Deliberately unclamped, so a sample outside the viewport keeps a distinct
/// cell and the path still enters and leaves the plot where the data does.
/// The normalized coordinate is bounded before the cast because an outlier
/// far beyond the window would otherwise overflow the index; anything that
/// far off screen is indistinguishable from anything else that far off.
fn screen_cell(x: f64, y: f64, view: TraceView) -> (i64, i64) {
    const LIMIT: f64 = 1.0e9;
    let column = (view
        .x_scale
        .normalize(x, view.x0, view.x1)
        .clamp(-LIMIT, LIMIT)
        * view.columns as f64) as i64;
    let row = (view
        .y_scale
        .normalize(y, view.y0, view.y1)
        .clamp(-LIMIT, LIMIT)
        * view.rows as f64) as i64;
    (column, row)
}

/// Reduce a parametric series — one whose X is not monotone — to at most a
/// few points per screen cell, in source order.
///
/// A locus (a Smith or Nyquist curve) revisits X values as the sweep runs, so
/// neither a binary search for the visible index window nor a per-X-column
/// min/max envelope describes it: the first assumes the visible samples are
/// contiguous in index order, and the second assumes one column holds one
/// span of the curve. A circle violates both, and the failure is silent —
/// whichever branch falls outside the contiguous window simply is not drawn.
///
/// This walks the samples in the order the sweep produced them, projects each
/// to an integer screen cell, and keeps a sample whenever the cell changes,
/// plus the last sample of the run it is leaving. The drawn path therefore
/// follows the sweep, every visible excursion survives, and the output is
/// bounded by the number of distinct cells the curve actually visits.
///
/// Non-finite samples pass through unchanged so `render`'s `finite_runs`
/// still splits the stroke exactly where the data stops.
pub fn decimate_pixel_cells(x: &[f64], y: &[f64], view: TraceView) -> Vec<[f64; 2]> {
    let n = x.len().min(y.len());
    if n == 0
        || view.columns == 0
        || view.rows == 0
        || !matches!(
            view.x1.partial_cmp(&view.x0),
            Some(std::cmp::Ordering::Greater)
        )
        || !matches!(
            view.y1.partial_cmp(&view.y0),
            Some(std::cmp::Ordering::Greater)
        )
    {
        return Vec::new();
    }

    let mut out: Vec<[f64; 2]> = Vec::new();
    let mut current: Option<(i64, i64)> = None;
    // The most recent sample of the cell run in progress. Emitting it as the
    // run ends keeps the segment leaving a cell anchored where the curve
    // actually was, not where it first arrived.
    let mut trailing: Option<[f64; 2]> = None;

    for index in 0..n {
        let (xv, yv) = (x[index], y[index]);
        if !xv.is_finite() || !yv.is_finite() {
            if let Some(point) = trailing.take() {
                out.push(point);
            }
            out.push([xv, yv]);
            current = None;
            continue;
        }
        let cell = screen_cell(xv, yv, view);
        if current == Some(cell) {
            trailing = Some([xv, yv]);
            continue;
        }
        if let Some(point) = trailing.take() {
            out.push(point);
        }
        out.push([xv, yv]);
        current = Some(cell);
    }
    if let Some(point) = trailing.take() {
        out.push(point);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_data_passes_through() {
        let x: Vec<f64> = (0..10).map(|i| i as f64).collect();
        let y: Vec<f64> = (0..10).map(|i| (i * i) as f64).collect();
        let env = decimate_minmax(
            &x,
            &y,
            0.0,
            9.0,
            XScale::Linear,
            100,
            XOrientation::Ascending,
        );
        assert_eq!(env.len(), 10);
    }

    #[test]
    fn dense_data_bounded_by_columns() {
        let n = 100_000;
        let x: Vec<f64> = (0..n).map(|i| i as f64 / n as f64).collect();
        let y: Vec<f64> = (0..n).map(|i| (i as f64 * 0.1).sin()).collect();
        let env = decimate_minmax(
            &x,
            &y,
            0.0,
            1.0,
            XScale::Linear,
            800,
            XOrientation::Ascending,
        );
        assert!(env.len() <= 800 * 2 + 4, "len = {}", env.len());
        // Envelope must preserve the global extremes.
        let max = env.iter().map(|p| p[1]).fold(f64::MIN, f64::max);
        assert!(max > 0.999);
    }

    #[test]
    fn interpolation() {
        let x = [0.0, 1.0, 2.0];
        let y = [0.0, 10.0, 20.0];
        assert!((sample_at(&x, &y, 0.5) - 5.0).abs() < 1e-12);
        assert!((sample_at(&x, &y, 5.0) - 20.0).abs() < 1e-12);
    }

    #[test]
    fn cursor_interpolation_modes_are_distinct_and_monotone_is_shape_preserving() {
        let x = [0.0, 1.0, 2.0, 3.0];
        let y = [0.0, 1.0, 1.5, 1.75];
        let nearest = sample_at_with(&x, &y, 1.25, SampleInterpolation::Nearest);
        let linear = sample_at_with(&x, &y, 1.25, SampleInterpolation::Linear);
        let cubic = sample_at_with(&x, &y, 1.25, SampleInterpolation::MonotoneCubic);
        assert_eq!(nearest, 1.0);
        assert_eq!(linear, 1.125);
        assert!((1.0..=1.5).contains(&cubic));
        assert_ne!(cubic, linear);
    }

    /// A Smith/Nyquist locus at a sweep density that decimates: 4001 samples
    /// around a circle, the shape `smith.rs` and `nyquist.rs` actually hand
    /// the renderer.
    fn locus(samples: usize) -> (Vec<f64>, Vec<f64>) {
        let angle = |i: usize| i as f64 / (samples - 1) as f64 * std::f64::consts::TAU;
        (
            (0..samples).map(|i| angle(i).cos() * 0.9).collect(),
            (0..samples).map(|i| angle(i).sin() * 0.9).collect(),
        )
    }

    fn locus_view(x0: f64, x1: f64) -> TraceView {
        TraceView {
            x0,
            x1,
            y0: -1.12,
            y1: 1.12,
            x_scale: XScale::Linear,
            y_scale: XScale::Linear,
            columns: 1216,
            rows: 704,
        }
    }

    #[test]
    fn a_zoomed_locus_keeps_every_branch_that_crosses_the_window() {
        // The defect this pins: the X-ordered reduction finds its visible
        // window with a binary search, so on a locus it returned one
        // contiguous index run and the other branch — half the curve — was
        // silently not drawn. Each of these windows is crossed twice.
        let (re, im) = locus(4001);
        for (x0, x1) in [(0.2, 0.5), (-0.6, -0.2), (0.0, 0.3)] {
            let drawn = decimate_pixel_cells(&re, &im, locus_view(x0, x1));
            let upper = drawn
                .iter()
                .filter(|point| (x0..=x1).contains(&point[0]) && point[1] > 0.0)
                .count();
            let lower = drawn
                .iter()
                .filter(|point| (x0..=x1).contains(&point[0]) && point[1] < 0.0)
                .count();
            assert!(
                upper > 0 && lower > 0,
                "window [{x0}, {x1}] drew upper={upper} lower={lower}"
            );
        }
    }

    #[test]
    fn a_locus_reduction_stays_bounded_and_ordered() {
        let (re, im) = locus(40_001);
        let view = locus_view(-1.12, 1.12);
        let drawn = decimate_pixel_cells(&re, &im, view);
        assert!(drawn.len() < re.len() / 4, "no reduction: {}", drawn.len());
        // Source order, not X order: consecutive output points must stay
        // within a cell step of each other, which an X-sorted reduction of a
        // circle could not satisfy.
        for pair in drawn.windows(2) {
            let (a, b) = (
                screen_cell(pair[0][0], pair[0][1], view),
                screen_cell(pair[1][0], pair[1][1], view),
            );
            assert!(
                (a.0 - b.0).abs() <= 1 && (a.1 - b.1).abs() <= 1,
                "path jumped from {a:?} to {b:?}"
            );
        }
    }

    #[test]
    fn a_locus_preserves_the_gaps_in_its_own_data() {
        let mut re = vec![0.0, 0.1, 0.2, 0.3, 0.4];
        let im = vec![0.0, 0.1, 0.2, 0.3, 0.4];
        re[2] = f64::NAN;
        let drawn = decimate_pixel_cells(&re, &im, locus_view(-1.12, 1.12));
        assert_eq!(
            drawn.iter().filter(|point| !point[0].is_finite()).count(),
            1,
            "the hole must survive so finite_runs can split the stroke"
        );
    }

    #[test]
    fn a_degenerate_locus_window_draws_nothing_rather_than_guessing() {
        let (re, im) = locus(64);
        let mut view = locus_view(0.0, 0.0);
        assert!(decimate_pixel_cells(&re, &im, view).is_empty());
        view = locus_view(-1.0, 1.0);
        view.rows = 0;
        assert!(decimate_pixel_cells(&re, &im, view).is_empty());
        assert!(decimate_pixel_cells(&[], &[], locus_view(-1.0, 1.0)).is_empty());
    }

    /// Oracle 1 (closed form): a descending sweep is the same curve as its
    /// ascending mirror, so it has to sample the same.
    #[test]
    fn a_descending_sweep_samples_the_mirror_of_its_ascending_twin() {
        let x = [2.0, 1.0, 0.0];
        let y = [20.0, 10.0, 0.0];
        assert_eq!(
            sample_at_with(&x, &y, 0.5, SampleInterpolation::Linear),
            5.0
        );
        assert_eq!(
            sample_at_with(&x, &y, 2.5, SampleInterpolation::Linear),
            20.0
        );
        assert_eq!(
            sample_at_with(&x, &y, -1.0, SampleInterpolation::Linear),
            0.0
        );
        assert_eq!(
            sample_at_with(&x, &y, 0.4, SampleInterpolation::Nearest),
            0.0
        );
    }

    /// An axis whose head is `-inf` is reachable: `log_frequency_axis` maps
    /// `f <= 0` there, and an imported Touchstone or CSV whose AC vector opens
    /// at `f = 0` produces exactly that. The first interval then spans an
    /// infinite width, and the interpolation fraction is `inf / inf` — NaN,
    /// which propagates into a margin, a crossing read, or a cursor readout
    /// with nothing to say it came from an unrepresentable abscissa.
    #[test]
    fn an_infinite_leading_interval_reads_the_finite_end_rather_than_nan() {
        let x = [f64::NEG_INFINITY, 1.0, 2.0];
        let y = [10.0, 20.0, 30.0];

        for mode in [
            SampleInterpolation::Linear,
            SampleInterpolation::Nearest,
            SampleInterpolation::MonotoneCubic,
        ] {
            let sampled = sample_at_with(&x, &y, 0.5, mode);
            assert!(
                sampled.is_finite(),
                "{mode:?} sampled an infinite leading interval as {sampled}"
            );
            assert_eq!(
                sampled, 20.0,
                "{mode:?} did not read the only bracketing sample that exists"
            );
        }

        // The mirror: a descending axis whose trailing sample is `-inf`.
        let x = [2.0, 1.0, f64::NEG_INFINITY];
        let y = [30.0, 20.0, 10.0];
        let sampled = sample_at_with(&x, &y, 0.5, SampleInterpolation::Linear);
        assert!(
            sampled.is_finite(),
            "a reversed infinite interval sampled as {sampled}"
        );
        assert_eq!(sampled, 20.0);
    }

    /// Oracle 1 (property): a reverse sweep is the mirror of its ascending
    /// twin, for every interpolation mode. Cubic is the one that matters —
    /// it pins the logical index map through three slope estimates and two
    /// endpoint rules, not just the bracket search.
    #[test]
    fn reversing_a_monotone_sweep_never_changes_what_it_samples() {
        let mut state = 0x2545_F491_4F6C_DD1D_u64;
        let mut random = move || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            (state >> 11) as f64 / (1_u64 << 53) as f64
        };
        for case in 0..40usize {
            let count = 3 + case % 17;
            let (mut x, mut y) = (Vec::new(), Vec::new());
            let mut cursor = -3.0;
            for _ in 0..count {
                cursor += 0.05 + random() * 2.0;
                x.push(cursor);
                y.push(random() * 10.0 - 5.0);
            }
            let reversed_x: Vec<f64> = x.iter().rev().copied().collect();
            let reversed_y: Vec<f64> = y.iter().rev().copied().collect();
            let (low, high) = (x[0] - 1.0, x[count - 1] + 1.0);
            for mode in [
                SampleInterpolation::Nearest,
                SampleInterpolation::Linear,
                SampleInterpolation::MonotoneCubic,
            ] {
                for step in 0..=60 {
                    let xq = low + (high - low) * f64::from(step) / 60.0;
                    assert_eq!(
                        sample_at_with(&x, &y, xq, mode),
                        sample_at_with(&reversed_x, &reversed_y, xq, mode),
                        "case {case} mode {mode:?} at {xq}"
                    );
                }
            }
        }
    }

    /// A zoomed window on a descending sweep still has a curve in it. The
    /// ascending-only binary search collapsed the window to one sample, and
    /// the trace vanished at exactly the zoom where it mattered.
    #[test]
    fn a_zoomed_descending_sweep_keeps_its_curve() {
        let n = 100_000;
        let x: Vec<f64> = (0..n).map(|i| 1.0 - i as f64 / n as f64).collect();
        let y: Vec<f64> = x.iter().map(|v| (v * 40.0).sin()).collect();
        let drawn = decimate_minmax(
            &x,
            &y,
            0.25,
            0.35,
            XScale::Linear,
            800,
            XOrientation::Descending,
        );
        assert!(drawn.len() >= 800, "drew {} points", drawn.len());
    }

    /// A hole in the data breaks the envelope exactly as it breaks a
    /// full-resolution stroke: the reduction must not bridge it.
    #[test]
    fn a_hole_breaks_the_decimated_envelope() {
        let n = 5_000;
        let x: Vec<f64> = (0..n).map(|i| i as f64 / n as f64).collect();
        let mut y: Vec<f64> = x.iter().map(|v| (v * 30.0).sin()).collect();
        y[2_500] = f64::NAN;
        let drawn = decimate_minmax(
            &x,
            &y,
            0.0,
            1.0,
            XScale::Linear,
            100,
            XOrientation::Ascending,
        );
        assert_eq!(
            drawn
                .iter()
                .filter(|point| !(point[0].is_finite() && point[1].is_finite()))
                .count(),
            1,
            "the hole must survive so finite_runs can split the stroke"
        );
    }

    /// A two-branch hysteresis sweep: up then back down, the shape a
    /// B-H loop or a latch transfer curve actually has.
    fn hysteresis(samples: usize) -> (Vec<f64>, Vec<f64>) {
        let half = samples / 2;
        let mut x = Vec::with_capacity(samples);
        let mut y = Vec::with_capacity(samples);
        for i in 0..half {
            let t = i as f64 / (half - 1) as f64;
            x.push(t);
            y.push(t * t);
        }
        for i in 0..half {
            let t = 1.0 - i as f64 / (half - 1) as f64;
            x.push(t);
            y.push(t * t + 0.5);
        }
        (x, y)
    }

    fn sweep_view(x0: f64, x1: f64) -> TraceView {
        TraceView {
            x0,
            x1,
            y0: -0.2,
            y1: 1.8,
            x_scale: XScale::Linear,
            y_scale: XScale::Linear,
            columns: 512,
            rows: 320,
        }
    }

    #[test]
    fn invalidating_retained_data_rebuilds_envelopes_without_resetting_the_budget() {
        let mut cache = DecimationCache::default();
        let budget = cache.byte_capacity;
        let x = [0.0, 1.0, 2.0];
        let first = cache.series(1, &x, &[0.0, 0.0, 0.0], sweep_view(0.0, 2.0), false, None);
        assert!(cache.resident_bytes > 0);
        cache.invalidate();
        assert_eq!(cache.resident_bytes, 0);
        assert_eq!(cache.byte_capacity, budget);
        let next = cache.series(1, &x, &[0.0, 1.0, 0.0], sweep_view(0.0, 2.0), false, None);
        assert!(first.iter().all(|point| point[1] == 0.0));
        assert!(next.iter().any(|point| point[1] == 1.0));
        assert_eq!(cache.resident_bytes, next.len() * size_of::<[f64; 2]>());
    }

    /// Oracle 6: a hysteresis trace through the cache reaches an
    /// order-agnostic reduction and both branches survive it. Before the
    /// routing existed this call reached the X-ordered envelope, which drew
    /// one branch and — in a debug build — tripped its own precondition.
    #[test]
    fn a_hysteresis_trace_through_the_cache_draws_both_branches() {
        let (x, y) = hysteresis(20_000);
        let shape = SweepShape::of(&x);
        assert_eq!(shape.class(), SweepClass::MultiBranch);
        let mut cache = DecimationCache::default();
        let drawn = cache.series(11, &x, &y, sweep_view(0.3, 0.6), false, Some(&shape));
        let lower = drawn
            .iter()
            .filter(|point| (0.3..=0.6).contains(&point[0]) && point[1] < 0.45)
            .count();
        let upper = drawn
            .iter()
            .filter(|point| (0.3..=0.6).contains(&point[0]) && point[1] > 0.55)
            .count();
        assert!(
            lower > 0 && upper > 0,
            "window drew lower={lower} upper={upper} of {} points",
            drawn.len()
        );
    }

    /// Oracle 6: a shaped descending trace, zoomed, still reduces to a
    /// pixel-dense curve rather than the single sample the ascending window
    /// search collapsed it to.
    #[test]
    fn a_shaped_descending_trace_survives_the_zoom_that_erased_it() {
        let n = 100_000;
        let x: Vec<f64> = (0..n).map(|i| 1.0 - f64::from(i) / f64::from(n)).collect();
        let y: Vec<f64> = x.iter().map(|value| (value * 40.0).sin()).collect();
        let shape = SweepShape::of(&x);
        assert_eq!(shape.class(), SweepClass::Descending);
        let view = sweep_view(0.25, 0.35);
        let mut cache = DecimationCache::default();
        let drawn = cache.series(5, &x, &y, view, false, Some(&shape));

        assert!(
            drawn.len() >= view.columns,
            "{} points for {} columns",
            drawn.len(),
            view.columns
        );
        assert!(drawn.len() <= view.columns * 2 + 4);
        assert!(
            drawn.iter().all(|point| (0.24..=0.36).contains(&point[0])),
            "reduction left the window"
        );
        // The same data read as ascending is a different reduction, so it
        // must not be served from this entry.
        let mistaken = cache.series(5, &x, &y, view, false, None);
        assert_ne!(drawn.as_ref(), mistaken.as_ref());
    }

    /// The cache's own contract: every input that changes the reduction has
    /// to be in the key. The axis scale changes every column boundary.
    #[test]
    fn a_scale_change_invalidates_the_cached_reduction() {
        let x: Vec<f64> = (1..=4_000).map(f64::from).collect();
        let y: Vec<f64> = x.iter().map(|value| value.ln()).collect();
        let mut cache = DecimationCache::default();
        let linear = sweep_view(1.0, 4_000.0);
        let logarithmic = TraceView {
            x_scale: XScale::Log10,
            ..linear
        };
        let first = cache.series(3, &x, &y, linear, false, None);
        let second = cache.series(3, &x, &y, logarithmic, false, None);

        assert_ne!(
            first.as_ref(),
            second.as_ref(),
            "a log reduction was served from the linear cache entry"
        );
    }

    /// One frame of `traces` curves at `view`, as the renderer asks for them.
    fn draw_frame(cache: &mut DecimationCache, traces: u64, x: &[f64], y: &[f64], view: TraceView) {
        cache.ensure_version(1);
        for trace in 0..traces {
            let _ = cache.series(trace, x, y, view, false, None);
        }
    }

    /// A frame that draws more curves than the stale bound keeps every
    /// envelope it drew. With a hard 256-entry bound, a 500-run family read in
    /// a cycle missed on every lookup: every repaint rebuilt every envelope.
    #[test]
    fn a_frame_drawing_more_traces_than_the_stale_bound_rebuilds_nothing_on_the_next() {
        let x: Vec<f64> = (0..2_000).map(f64::from).collect();
        let y: Vec<f64> = x.iter().map(|value| (value * 0.01).sin()).collect();
        let view = sweep_view(0.0, 1_999.0);
        let mut cache = DecimationCache::default();
        draw_frame(&mut cache, 500, &x, &y, view);
        assert_eq!(cache.work().rebuilds, 500);
        let first = cache.work();
        for _ in 0..3 {
            draw_frame(&mut cache, 500, &x, &y, view);
        }
        let steady = cache.work().since(first);
        assert_eq!(
            (steady.rebuilds, steady.evictions, steady.hits),
            (0, 0, 1_500),
            "a steady frame must reuse every envelope it drew on the frame before"
        );
    }

    /// The stale bound still bounds what the reader has left: a view nobody
    /// draws any more gives way to the one on screen.
    #[test]
    fn envelopes_of_a_view_the_reader_left_are_bounded_by_the_stale_cap() {
        let x: Vec<f64> = (0..2_000).map(f64::from).collect();
        let y: Vec<f64> = x.iter().map(|value| (value * 0.01).sin()).collect();
        let mut cache = DecimationCache::default();
        draw_frame(&mut cache, 500, &x, &y, sweep_view(0.0, 1_999.0));
        assert_eq!(cache.held.map.len(), 500);
        draw_frame(&mut cache, 10, &x, &y, sweep_view(0.0, 999.0));
        assert!(
            cache.held.map.len() <= STALE_ENTRY_CAP,
            "{} entries survived a view change",
            cache.held.map.len()
        );
        // The ten curves the reader is looking at are all still there.
        let before = cache.work();
        draw_frame(&mut cache, 10, &x, &y, sweep_view(0.0, 999.0));
        assert_eq!(cache.work().since(before).rebuilds, 0);
    }

    fn member<'a>(x: &'a [f64], y: &'a [f64]) -> BandMember<'a> {
        BandMember {
            x,
            y,
            parametric: false,
            shape: None,
            stored: None,
        }
    }

    /// Every member's own envelope lies inside the band, column by column,
    /// and the band is no wider than the members reach in that column and
    /// its neighbours.
    #[test]
    fn a_band_holds_every_member_and_no_more() {
        let x: Vec<f64> = (0..20_000).map(|index| f64::from(index) * 1.0e-3).collect();
        let curves: Vec<Vec<f64>> = (0..5)
            .map(|phase| {
                x.iter()
                    .map(|value| (value + f64::from(phase) * 0.3).sin())
                    .collect()
            })
            .collect();
        let view = sweep_view(0.0, 19.999);
        let members: Vec<BandMember<'_>> = curves.iter().map(|y| member(&x, y)).collect();
        let (band, _) = band_columns(&members, view);
        assert_eq!(band.len(), view.columns);
        let column = |value: f64| {
            ((XScale::Linear.normalize(value, view.x0, view.x1) * view.columns as f64) as usize)
                .min(view.columns - 1)
        };
        let mut reach = vec![[f64::INFINITY, f64::NEG_INFINITY]; view.columns];
        for y in &curves {
            for point in decimate_minmax(
                &x,
                y,
                view.x0,
                view.x1,
                XScale::Linear,
                view.columns,
                XOrientation::Ascending,
            ) {
                let [low, high] = band[column(point[0])];
                assert!(
                    low <= point[1] && point[1] <= high,
                    "{point:?} outside [{low}, {high}]"
                );
            }
            for (&value, &sample) in x.iter().zip(y) {
                let cell = &mut reach[column(value)];
                cell[0] = cell[0].min(sample);
                cell[1] = cell[1].max(sample);
            }
        }
        for (index, [low, high]) in band.iter().enumerate() {
            let neighbours = &reach[index.saturating_sub(1)..(index + 2).min(view.columns)];
            let floor = neighbours
                .iter()
                .map(|cell| cell[0])
                .fold(f64::INFINITY, f64::min);
            let ceiling = neighbours
                .iter()
                .map(|cell| cell[1])
                .fold(f64::NEG_INFINITY, f64::max);
            assert!(
                floor <= *low && *high <= ceiling,
                "column {index} widened past its members"
            );
        }
    }

    /// Zoomed past one sample per column, a band still covers every column
    /// its members' strokes cross. Folding samples alone left it a row of
    /// isolated columns that never painted.
    #[test]
    fn a_band_zoomed_past_its_samples_stays_continuous() {
        let x: Vec<f64> = (0..11).map(f64::from).collect();
        let rising: Vec<f64> = x.clone();
        let falling: Vec<f64> = x.iter().map(|value| 10.0 - value).collect();
        let members = [member(&x, &rising), member(&x, &falling)];
        let (band, _) = band_columns(&members, sweep_view(0.0, 10.0));
        assert!(
            band.iter()
                .all(|[low, high]| low.is_finite() && high.is_finite())
        );
        // At the midpoint the two strokes cross at 5.
        let [low, high] = band[band.len() / 2];
        assert!(
            (low - 5.0).abs() < 0.05 && (high - 5.0).abs() < 0.05,
            "{low} {high}"
        );
        // A quarter of the way along they are 2.5 and 7.5.
        let [low, high] = band[band.len() / 4];
        assert!(
            (low - 2.5).abs() < 0.05 && (high - 7.5).abs() < 0.05,
            "{low} {high}"
        );
    }

    /// Where no member reaches, the band has nothing to say; a hole in a
    /// member breaks its stroke there and is not bridged.
    #[test]
    fn a_band_is_empty_where_no_member_strokes() {
        let x: Vec<f64> = (0..=50).map(f64::from).collect();
        let mut y: Vec<f64> = x.iter().map(|value| value.sin()).collect();
        y[20] = f64::NAN;
        let (band, _) = band_columns(&[member(&x, &y)], sweep_view(0.0, 100.0));
        let empty = |[low, high]: [f64; 2]| low.is_nan() && high.is_nan();
        assert!(band[band.len() * 3 / 4..].iter().copied().all(empty));
        // Columns strictly between samples 19 and 21 are left to nobody.
        let hole = (19.5 / 100.0 * band.len() as f64) as usize
            ..(20.5 / 100.0 * band.len() as f64) as usize;
        assert!(band[hole].iter().copied().all(empty));
        assert!(!empty(band[(10.0 / 100.0 * band.len() as f64) as usize]));
    }

    /// On a decade pane a value at or below zero has no place, and a stroke
    /// between two values is straight in log Y.
    #[test]
    fn a_decade_band_skips_what_a_decade_pane_cannot_show() {
        let x = [0.0, 1.0, 2.0];
        let y = [-1.0, 1.0, 100.0];
        let view = TraceView {
            y0: 0.1,
            y1: 1_000.0,
            y_scale: XScale::Log10,
            ..sweep_view(0.0, 2.0)
        };
        let (band, _) = band_columns(&[member(&x, &y)], view);
        assert!(
            band[..band.len() / 2 - 1]
                .iter()
                .all(|[low, _]| low.is_nan())
        );
        // Three quarters along, halfway between 1 and 100 in log Y is 10.
        let [low, high] = band[band.len() * 3 / 4];
        assert!(
            (low.log10() - 1.0).abs() < 0.01 && (high.log10() - 1.0).abs() < 0.01,
            "{low}"
        );
    }

    /// A band is cached under its member set: a second ask reads no samples.
    #[test]
    fn a_cached_band_is_served_without_reading_its_members() {
        let x: Vec<f64> = (0..2_000).map(f64::from).collect();
        let y: Vec<f64> = x.iter().map(|value| (value * 0.01).sin()).collect();
        let view = sweep_view(0.0, 1_999.0);
        let mut cache = DecimationCache::default();
        cache.ensure_version(1);
        assert!(cache.held_band(7, view).is_none(), "nothing is kept yet");
        let first = cache.band(7, &[member(&x, &y), member(&x, &y)], view);
        let before = cache.work();
        let again = cache.band(7, &[], view);
        assert!(Arc::ptr_eq(&first, &again));
        let held = cache.held_band(7, view).expect("the band is kept");
        assert!(Arc::ptr_eq(&first, &held));
        assert_eq!(cache.work().since(before).rebuilds, 0);
        // A trace with the same key is a different reduction.
        let trace = cache.series(7, &x, &y, view, false, None);
        assert!(!Arc::ptr_eq(&first, &trace));
    }

    /// At 2.5 device pixels per point a plot is reduced to 2.5 times the
    /// columns it is reduced to at 1: one per device pixel.
    #[test]
    fn two_and_a_half_pixels_per_point_ask_two_and_a_half_times_the_columns() {
        assert_eq!(display_columns(640.0, 1.0), 640);
        assert_eq!(display_columns(640.0, 2.5), 1600);
    }

    /// A run's axis and one of its series, kept by a store that tiles them
    /// from memory, as a segment's summaries would.
    struct Tiled {
        x: Arc<Vec<f64>>,
        y: Vec<f64>,
        axis: bool,
    }

    impl rspice_results::waveform::ColumnSource for Tiled {
        fn name(&self) -> &str {
            if self.axis { "time" } else { "V(out)" }
        }

        fn store(&self) -> String {
            "run.rsr".to_owned()
        }

        fn rows(&self) -> u64 {
            self.y.len() as u64
        }

        fn read(&self, first: u64, out: &mut [f64]) -> Result<(), String> {
            let first = first as usize;
            out.copy_from_slice(&self.y[first..first + out.len()]);
            Ok(())
        }

        fn ascending(&self) -> bool {
            self.axis
        }

        fn tile(
            &self,
            rows: std::ops::Range<u64>,
            x0: f64,
            x1: f64,
            pixels: usize,
        ) -> Option<Result<ColumnTile, String>> {
            let mut min = vec![f64::NAN; pixels];
            let mut max = vec![f64::NAN; pixels];
            let mut touched = vec![false; pixels];
            for row in rows.start as usize..rows.end as usize {
                let at = self.x[row];
                if !(x0..=x1).contains(&at) {
                    continue;
                }
                let pixel = (((at - x0) / (x1 - x0) * pixels as f64) as usize).min(pixels - 1);
                touched[pixel] = true;
                min[pixel] = min[pixel].min(self.y[row]);
                max[pixel] = max[pixel].max(self.y[row]);
            }
            Some(Ok(ColumnTile::Envelope { min, max, touched }))
        }
    }

    fn tiled_sine(
        rows: usize,
    ) -> (
        rspice_results::waveform::WaveformValues,
        rspice_results::waveform::WaveformValues,
    ) {
        use rspice_results::waveform::{StoredWindow, WaveformValues, WindowLead};
        let x: Arc<Vec<f64>> = Arc::new((0..rows).map(|row| row as f64).collect());
        let y: Vec<f64> = x.iter().map(|at| (at * 0.01).sin()).collect();
        let window = StoredWindow {
            first: 0,
            rows: rows as u64,
            lead: WindowLead::None,
        };
        let stored = |y: Vec<f64>, axis: bool| {
            let source = Arc::new(Tiled {
                x: Arc::clone(&x),
                y,
                axis,
            });
            WaveformValues::stored(source, window).expect("the window fits the column")
        };
        (stored(x.to_vec(), true), stored(y, false))
    }

    /// A reader service whose reads arrive only when the test lets them.
    #[derive(Default)]
    struct Patient {
        held: std::sync::Mutex<
            Vec<(
                rspice_results::waveform::Read,
                rspice_results::waveform::ReadValue,
            )>,
        >,
        asked: std::sync::Mutex<
            Vec<(
                rspice_results::waveform::WaveformValues,
                rspice_results::waveform::WaveformValues,
                rspice_results::waveform::Read,
            )>,
        >,
    }

    impl Patient {
        /// Every read asked for so far arrives.
        fn arrive(&self) {
            let asked = std::mem::take(&mut *self.asked.lock().unwrap());
            for (values, x, read) in asked {
                let value = rspice_results::waveform::perform(&values, &x, read);
                self.held.lock().unwrap().push((read, value));
            }
        }
    }

    impl rspice_results::waveform::ReadService for Patient {
        fn request(
            &self,
            run: u64,
            values: &rspice_results::waveform::WaveformValues,
            x: &rspice_results::waveform::WaveformValues,
            read: rspice_results::waveform::Read,
        ) -> Option<rspice_results::waveform::ReadValue> {
            if let Some(value) = self.held(run, values, read) {
                return Some(value);
            }
            let mut asked = self.asked.lock().unwrap();
            if !asked.iter().any(|(_, _, queued)| *queued == read) {
                asked.push((values.clone(), x.clone(), read));
            }
            None
        }

        fn held(
            &self,
            _run: u64,
            _values: &rspice_results::waveform::WaveformValues,
            read: rspice_results::waveform::Read,
        ) -> Option<rspice_results::waveform::ReadValue> {
            let held = self.held.lock().unwrap();
            held.iter()
                .find(|(kept, _)| *kept == read)
                .map(|(_, value)| value.clone())
        }
    }

    /// A stored trace is drawn from what the reader holds: nothing before its
    /// first tile arrives, then that tile, kept; zoomed in, the coarser tile
    /// it already holds stands in, kept for no later frame, until the view's
    /// own arrives and sharpens it.
    #[test]
    fn a_tile_not_yet_read_draws_its_coarser_parent_and_refines() {
        let reader: &'static Patient = Box::leak(Box::default());
        let (x, y) = tiled_sine(100_000);
        let stored = super::super::spec::StoredSamples { x: &x, y: &y };
        let mut cache = DecimationCache::default();
        cache.ensure_version(1);
        rspice_results::waveform::read_through(reader, || {
            let whole = sweep_view(0.0, 99_999.0);
            assert!(
                cache.stored_series(Some(1), stored, whole).is_empty(),
                "nothing is drawn before a tile arrives"
            );
            reader.arrive();
            let first = cache.stored_series(Some(1), stored, whole);
            assert!(!first.is_empty(), "the arrived tile is drawn");
            let again = cache.stored_series(Some(1), stored, whole);
            assert!(Arc::ptr_eq(&first, &again), "the view's own tile is kept");

            let zoomed = sweep_view(10_000.0, 22_500.0);
            let in_view = |envelope: &[[f64; 2]]| {
                envelope
                    .iter()
                    .filter(|point| (zoomed.x0..=zoomed.x1).contains(&point[0]))
                    .count()
            };
            let standing_in = cache.stored_series(Some(1), stored, zoomed);
            assert!(
                in_view(&standing_in) > 0,
                "the coarser tile draws the zoomed view"
            );
            let still = cache.stored_series(Some(1), stored, zoomed);
            assert!(!Arc::ptr_eq(&standing_in, &still), "a stand-in is not kept");

            reader.arrive();
            let refined = cache.stored_series(Some(1), stored, zoomed);
            assert!(
                in_view(&refined) >= zoomed.columns,
                "the view's own tile has a pixel in every column: {} points",
                in_view(&refined)
            );
            assert!(in_view(&refined) > 4 * in_view(&standing_in));
            let kept = cache.stored_series(Some(1), stored, zoomed);
            assert!(Arc::ptr_eq(&refined, &kept), "the refined envelope is kept");
        });
    }
}
