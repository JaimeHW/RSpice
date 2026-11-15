//! The min/max pyramid above the chunks' level-1 summaries.
//!
//! Level 1 is 64-row buckets, stored in each `CHUNK`. Each level above folds 16 buckets of the level
//! below into one (level 2 is 1,024 rows, level 3 16,384, level 4 262,144, ...). Buckets are aligned
//! to absolute row multiples, so a bucket's rows never depend on chunking. A `SUMMARY` frame holds
//! 64 buckets of one level and is written when its 64th bucket completes; at the end every level
//! writes what it has, up to the first level whose whole run fits in one frame.

use crate::format::{
    FrameBuf, FrameKind, SUMMARY_FANOUT, SUMMARY_FRAME_BUCKETS, canonical, fold_max, fold_min,
};

const PAIRS: usize = 2 * SUMMARY_FRAME_BUCKETS;

/// One level's open frame.
struct Level {
    /// `(x_min, x_max)` per bucket.
    x: Vec<f64>,
    /// Signal-major `(min, max)` per bucket, `PAIRS` values per signal.
    values: Vec<f64>,
    /// Completed buckets in the open frame.
    buckets: usize,
    /// Children folded into the open bucket.
    children: u64,
    /// Frames of this level written so far.
    frames: u64,
}

impl Level {
    fn new(signals: usize) -> Self {
        Self {
            x: vec![f64::NAN; PAIRS],
            values: vec![f64::NAN; signals * PAIRS],
            buckets: 0,
            children: 0,
            frames: 0,
        }
    }
}

/// The writer's summary levels 2 and up. Memory is one 64-bucket frame per level.
pub(crate) struct Pyramid {
    signals: usize,
    /// `levels[0]` is level 2.
    levels: Vec<Level>,
    /// The bucket being folded upward: `(min, max)` per signal.
    child: Vec<f64>,
}

impl Pyramid {
    pub(crate) fn new(signals: usize) -> Self {
        Self {
            signals,
            levels: Vec::new(),
            child: vec![f64::NAN; 2 * signals],
        }
    }

    /// The child buffer to fill with one level-1 bucket's `(min, max)` per signal before
    /// [`Self::push_level1`].
    pub(crate) fn child_mut(&mut self) -> &mut [f64] {
        &mut self.child
    }

    /// Fold the level-1 bucket in the child buffer, spanning `x_lo ..= x_hi`, into level 2 and up.
    pub(crate) fn push_level1(&mut self, x_lo: f64, x_hi: f64) {
        self.push(0, x_lo, x_hi);
    }

    fn push(&mut self, mut index: usize, mut x_lo: f64, mut x_hi: f64) {
        let Self {
            signals,
            levels,
            child,
        } = self;
        let signals = *signals;
        loop {
            if index == levels.len() {
                levels.push(Level::new(signals));
            }
            let level = &mut levels[index];
            debug_assert!(level.buckets < SUMMARY_FRAME_BUCKETS);
            let at = 2 * level.buckets;
            if level.children == 0 {
                level.x[at] = x_lo;
                level.x[at + 1] = x_hi;
                for signal in 0..signals {
                    level.values[signal * PAIRS + at] = child[2 * signal];
                    level.values[signal * PAIRS + at + 1] = child[2 * signal + 1];
                }
            } else {
                level.x[at] = fold_min(level.x[at], x_lo);
                level.x[at + 1] = fold_max(level.x[at + 1], x_hi);
                for signal in 0..signals {
                    let slot = signal * PAIRS + at;
                    level.values[slot] = fold_min(level.values[slot], child[2 * signal]);
                    level.values[slot + 1] =
                        fold_max(level.values[slot + 1], child[2 * signal + 1]);
                }
            }
            level.children += 1;
            if level.children < SUMMARY_FANOUT {
                return;
            }
            level.children = 0;
            level.buckets += 1;
            x_lo = level.x[at];
            x_hi = level.x[at + 1];
            for signal in 0..signals {
                child[2 * signal] = level.values[signal * PAIRS + at];
                child[2 * signal + 1] = level.values[signal * PAIRS + at + 1];
            }
            index += 1;
        }
    }

    /// Whether level `index`'s open frame holds all its buckets.
    pub(crate) fn is_full(&self, index: usize) -> bool {
        self.levels[index].buckets == SUMMARY_FRAME_BUCKETS
    }

    pub(crate) fn levels(&self) -> usize {
        self.levels.len()
    }

    /// Close level `index`'s partial bucket, folding it upward. Returns whether the level has
    /// buckets to write.
    pub(crate) fn close(&mut self, index: usize) -> bool {
        let level = &mut self.levels[index];
        if level.children > 0 {
            level.children = 0;
            level.buckets += 1;
            let at = 2 * (level.buckets - 1);
            let (x_lo, x_hi) = (level.x[at], level.x[at + 1]);
            for signal in 0..self.signals {
                self.child[2 * signal] = level.values[signal * PAIRS + at];
                self.child[2 * signal + 1] = level.values[signal * PAIRS + at + 1];
            }
            self.push(index + 1, x_lo, x_hi);
        }
        self.levels[index].buckets > 0
    }

    /// Frames level `index` has written.
    pub(crate) fn frames(&self, index: usize) -> u64 {
        self.levels[index].frames
    }

    /// Encode level `index`'s open frame (`rows` rows from `first_row`) and start its next frame.
    /// Returns the frame's x range.
    pub(crate) fn encode(
        &mut self,
        index: usize,
        first_row: u64,
        rows: u64,
        frame: &mut FrameBuf,
    ) -> (f64, f64) {
        let level = &mut self.levels[index];
        let buckets = level.buckets;
        let (mut x_min, mut x_max) = (f64::NAN, f64::NAN);
        frame.begin(FrameKind::Summary, 0);
        frame.put_u8(index as u8 + 2);
        frame.put(&[0, 0, 0]);
        frame.put_u32(buckets as u32);
        frame.put_u64(level.frames);
        frame.put_u64(first_row);
        frame.put_u64(rows);
        for &value in &level.x[..2 * buckets] {
            frame.put_f64(value);
        }
        for pair in level.x[..2 * buckets].chunks_exact(2) {
            x_min = fold_min(x_min, pair[0]);
            x_max = fold_max(x_max, pair[1]);
        }
        for signal in 0..self.signals {
            for &value in &level.values[signal * PAIRS..signal * PAIRS + 2 * buckets] {
                frame.put_f64(value);
            }
        }
        level.frames += 1;
        level.buckets = 0;
        (canonical(x_min), canonical(x_max))
    }
}
