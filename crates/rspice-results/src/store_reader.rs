//! A run's results store segment as the results workspace reads it.
//!
//! One [`StoreReader`] serves every waveform of a segment: its columns and the run's metadata,
//! an exact range of rows, the tile of a view (per pixel, the min and max of the rows in it),
//! the rows either side of a cursor, and the changes of an event signal over a window. Views
//! read the segment's summary pyramid a block at a time, and the blocks they decode stay in one
//! process-wide [`TileCache`] under a byte budget, so a pan or a zoom over rows already drawn
//! reads none of them again.

use std::collections::{BTreeMap, HashMap};
use std::ops::Range;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};

use rspice_results_store::{
    BlockCache, BlockKey, Digest, EventChange, RunMeta, Schema, SegmentIo, SegmentReader,
    SegmentState, StoreError, Tile,
};

/// The bytes of decoded blocks the views of every open segment keep between frames: a working
/// set for drawing, never a bound on what a run holds. A view over rows the cache dropped reads
/// its blocks again.
#[cfg(not(target_arch = "wasm32"))]
const TILE_CACHE_BYTES: usize = 256 << 20;
#[cfg(target_arch = "wasm32")]
const TILE_CACHE_BYTES: usize = 32 << 20;

/// A segment's own bytes, read at offsets.
pub type SegmentBytes = Box<dyn SegmentIo + Send>;

static NEXT_READER: AtomicU64 = AtomicU64::new(1);
static TILE_CACHE: Mutex<TileCache> = Mutex::new(TileCache::new(TILE_CACHE_BYTES));

/// One open segment, shared by the waveforms that read it.
pub struct StoreReader {
    reader: Mutex<Option<SegmentReader<SegmentBytes>>>,
    /// The cache's name for this reader's blocks.
    id: u64,
    rows: u64,
    /// Each chunk's first row and row count, in row order.
    spans: Vec<(u64, usize)>,
    /// The analog signal each value slot begins, by slot.
    slot_signals: Vec<Option<usize>>,
    digest: Option<Digest>,
}

impl StoreReader {
    /// Open the segment `io` holds, recovering what a run that did not end left.
    pub fn open(io: SegmentBytes) -> Result<Self, StoreError> {
        Ok(Self::new(SegmentReader::open(io)?))
    }

    /// Open the segment file at `path`.
    #[cfg(any(unix, windows))]
    pub fn open_file(path: &std::path::Path) -> Result<Self, StoreError> {
        let io = rspice_results_store::FileIo::open(path)?;
        Self::open(Box::new(io))
    }

    /// A reader of the segment `reader` has opened.
    pub fn new(reader: SegmentReader<SegmentBytes>) -> Self {
        let schema = reader.schema();
        let mut slot_signals = vec![None; reader.slots()];
        for signal in 0..schema.signals.len() {
            if let Some(slot) = schema.slot_of(signal)
                && let Some(entry) = slot_signals.get_mut(slot)
            {
                *entry = Some(signal);
            }
        }
        let digest = match reader.state() {
            SegmentState::Complete { digest, .. } => Some(*digest),
            SegmentState::Recovered => None,
        };
        Self {
            id: NEXT_READER.fetch_add(1, Ordering::Relaxed),
            rows: reader.rows(),
            spans: reader.chunk_spans().collect(),
            slot_signals,
            digest,
            reader: Mutex::new(Some(reader)),
        }
    }

    /// Readable rows.
    pub fn rows(&self) -> u64 {
        self.rows
    }

    /// The logical digest its run's `END` gave; none for a segment recovered without one.
    pub fn digest(&self) -> Option<Digest> {
        self.digest
    }

    /// The segment's columns.
    pub fn schema(&self) -> Result<Schema, StoreError> {
        self.with_reader(|reader| Ok(reader.schema().clone()))
    }

    /// What the run said of itself: its analysis card, sweep values, seed and options.
    pub fn run_meta(&self) -> Result<RunMeta, StoreError> {
        self.with_reader(|reader| Ok(reader.run_meta().clone()))
    }

    /// The annotation the run left under `key`, if it left one.
    pub fn annotation(&self, key: &str) -> Result<Option<Vec<u8>>, StoreError> {
        self.with_reader(|reader| Ok(reader.annotation(key).map(<[u8]>::to_vec)))
    }

    /// Run `read` with the segment's reader.
    pub fn with_reader<T>(
        &self,
        read: impl FnOnce(&mut SegmentReader<SegmentBytes>) -> Result<T, StoreError>,
    ) -> Result<T, StoreError> {
        let mut reader = self.reader.lock().unwrap_or_else(|p| p.into_inner());
        let reader = reader.as_mut().ok_or_else(|| StoreError::Io {
            kind: std::io::ErrorKind::Other,
            message: "the results segment was closed".to_owned(),
        })?;
        read(reader)
    }

    /// Close the segment: every read after this fails. A host closes a segment before it deletes
    /// or moves its file, which some hosts refuse while it is open.
    pub fn close(&self) {
        drop(self.reader.lock().unwrap_or_else(|p| p.into_inner()).take());
        lock_cache().forget(self.id);
    }

    /// Rows `first .. first + out.len()` of the x axis (`slot` `None`) or a value slot, decoded
    /// chunk by chunk into `out`.
    pub fn read(&self, slot: Option<usize>, first: u64, out: &mut [f64]) -> Result<(), StoreError> {
        let end = first + out.len() as u64;
        if end > self.rows {
            return Err(StoreError::OutOfRange(format!(
                "rows {first} .. {end} of a segment with {} rows",
                self.rows
            )));
        }
        self.with_reader(|reader| {
            let mut chunk = self
                .spans
                .partition_point(|&(start, rows)| start + rows as u64 <= first);
            let mut buffer = Vec::new();
            let mut row = first;
            while row < end {
                let (start, rows) = self.spans[chunk];
                match slot {
                    None => reader.read_chunk_x(chunk, &mut buffer)?,
                    Some(slot) => reader.read_chunk_slot(chunk, slot, &mut buffer)?,
                }
                let from = (row - start) as usize;
                let to = rows.min((end - start) as usize);
                let at = (row - first) as usize;
                out[at..at + to - from].copy_from_slice(&buffer[from..to]);
                row = start + to as u64;
                chunk += 1;
            }
            Ok(())
        })
    }

    /// The tile of the analog signal whose values begin at `slot`, over the rows in `rows` with
    /// x in `x0 ..= x1`, at `pixels` pixels.
    pub fn tile(
        &self,
        slot: usize,
        rows: Range<u64>,
        x0: f64,
        x1: f64,
        pixels: usize,
    ) -> Result<Tile, StoreError> {
        let signal = self.signal_of(slot)?;
        self.with_reader(|reader| {
            let mut cache = lock_cache();
            let mut blocks = Blocks {
                cache: &mut cache,
                reader: self.id,
            };
            reader.tile_in(signal, rows, x0, x1, pixels, &mut blocks)
        })
    }

    /// The rows of `rows` around `x` of the analog signal whose values begin at `slot`, each as
    /// (x, value), with the index of the first: up to two at or before `x` and up to two after.
    pub fn rows_around(
        &self,
        slot: usize,
        rows: Range<u64>,
        x: f64,
    ) -> Result<(u64, Vec<(f64, f64)>), StoreError> {
        let signal = self.signal_of(slot)?;
        self.with_reader(|reader| {
            let mut cache = lock_cache();
            let mut blocks = Blocks {
                cache: &mut cache,
                reader: self.id,
            };
            reader.rows_around(signal, rows, x, &mut blocks)
        })
    }

    /// The changes of event signal `signal` with time in `t0 ..= t1`, in commit order.
    pub fn events(&self, signal: usize, t0: f64, t1: f64) -> Result<Vec<EventChange>, StoreError> {
        self.with_reader(|reader| reader.events(signal, t0, t1))
    }

    /// How many changes event signal `signal` holds.
    pub fn event_count(&self, signal: usize) -> Result<u64, StoreError> {
        self.with_reader(|reader| reader.event_count(signal))
    }

    /// How many changes of event signal `signal` come before time `t`.
    pub fn events_before(&self, signal: usize, t: f64) -> Result<u64, StoreError> {
        self.with_reader(|reader| reader.events_before(signal, t))
    }

    /// The changes of event signal `signal` with time in `t0 ..= t1`, with the last change
    /// before `t0` and the first after `t1`.
    pub fn events_around(
        &self,
        signal: usize,
        t0: f64,
        t1: f64,
    ) -> Result<rspice_results_store::EventSpan, StoreError> {
        self.with_reader(|reader| reader.events_around(signal, t0, t1))
    }

    fn signal_of(&self, slot: usize) -> Result<usize, StoreError> {
        self.slot_signals
            .get(slot)
            .copied()
            .flatten()
            .ok_or_else(|| StoreError::OutOfRange(format!("slot {slot} begins no analog signal")))
    }
}

impl From<Tile> for crate::waveform::ColumnTile {
    fn from(tile: Tile) -> Self {
        match tile {
            Tile::Samples {
                x,
                y,
                before,
                after,
            } => Self::Samples {
                x,
                y,
                before,
                after,
            },
            Tile::Envelope {
                min, max, touched, ..
            } => Self::Envelope { min, max, touched },
        }
    }
}

impl Drop for StoreReader {
    fn drop(&mut self) {
        lock_cache().forget(self.id);
    }
}

fn lock_cache() -> MutexGuard<'static, TileCache> {
    TILE_CACHE.lock().unwrap_or_else(|p| p.into_inner())
}

/// One reader's view of the cache.
struct Blocks<'a> {
    cache: &'a mut TileCache,
    reader: u64,
}

impl BlockCache for Blocks<'_> {
    fn get(&mut self, key: &BlockKey) -> Option<Arc<[f64]>> {
        self.cache.get(&(self.reader, *key))
    }

    fn put(&mut self, key: BlockKey, block: Arc<[f64]>) {
        self.cache.put((self.reader, key), block);
    }
}

/// A block as the cache names it: the reader of its segment, and the block.
type CacheKey = (u64, BlockKey);

/// A block the cache keeps, and the tick it was last used at.
type Kept = (Arc<[f64]>, u64);

/// Decoded blocks of open segments, least recently used first out, under a byte budget: keyed
/// by the reader of the segment, the column, the summary level and the rows the block covers.
pub struct TileCache {
    budget: usize,
    bytes: usize,
    tick: u64,
    /// Each block, and when it was last used.
    blocks: Option<HashMap<CacheKey, Kept>>,
    /// The key of each block, by when it was last used.
    order: BTreeMap<u64, CacheKey>,
}

impl TileCache {
    /// An empty cache that keeps at most `budget` bytes of blocks.
    pub const fn new(budget: usize) -> Self {
        Self {
            budget,
            bytes: 0,
            tick: 0,
            blocks: None,
            order: BTreeMap::new(),
        }
    }

    /// The bytes of blocks it keeps.
    pub fn bytes(&self) -> usize {
        self.bytes
    }

    fn get(&mut self, key: &CacheKey) -> Option<Arc<[f64]>> {
        self.tick += 1;
        let tick = self.tick;
        let (block, used) = self.blocks.as_mut()?.get_mut(key)?;
        self.order.remove(used);
        *used = tick;
        self.order.insert(tick, *key);
        Some(Arc::clone(block))
    }

    fn put(&mut self, key: CacheKey, block: Arc<[f64]>) {
        let size = block_bytes(&block);
        if size > self.budget {
            return;
        }
        self.tick += 1;
        let tick = self.tick;
        if let Some((old, used)) = self
            .blocks
            .get_or_insert_with(HashMap::new)
            .insert(key, (block, tick))
        {
            self.order.remove(&used);
            self.bytes -= block_bytes(&old);
        }
        self.order.insert(tick, key);
        self.bytes += size;
        while self.bytes > self.budget {
            let Some((_, oldest)) = self.order.pop_first() else {
                break;
            };
            if let Some((block, _)) = self.blocks.as_mut().and_then(|map| map.remove(&oldest)) {
                self.bytes -= block_bytes(&block);
            }
        }
    }

    /// Drop every block of reader `reader`.
    fn forget(&mut self, reader: u64) {
        let Some(blocks) = self.blocks.as_mut() else {
            return;
        };
        let order = &mut self.order;
        let bytes = &mut self.bytes;
        blocks.retain(|(owner, _), (block, used)| {
            let keep = *owner != reader;
            if !keep {
                order.remove(used);
                *bytes -= block_bytes(block);
            }
            keep
        });
    }
}

fn block_bytes(block: &[f64]) -> usize {
    std::mem::size_of_val(block)
}

#[cfg(test)]
mod tests;
