//! What one event lane draws across its pixel columns.
//!
//! A lane's change instants map onto the shared time axis. Wherever one pixel
//! column holds two or more changes, the column draws as one activity block
//! instead of as the changes: they could not be told apart on screen, and
//! drawing them one by one would make a frame cost what the trace holds
//! rather than what the lane shows. Every step of the walk below either
//! consumes a whole pixel column or emits one exact edge, and finds its next
//! position by bisection, so a plan costs `O(columns · log changes)` whatever
//! the trace's length.
//!
//! An unknown value is never lost to the zoom. A stretch of `x` narrower than
//! a pixel, alone or inside a dense block, is drawn at least one pixel wide in
//! the unknown style, as SimVision draws it: the reader looking for where a
//! net went unknown has to see every place it did at any zoom.

/// The shared time axis, mapped onto a lane's screen extent.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeMapping {
    pub t0: f64,
    pub t1: f64,
    pub left: f32,
    pub right: f32,
}

impl TimeMapping {
    /// The screen X of time `t`, as the waveform panes map it.
    pub fn x(&self, t: f64) -> f32 {
        let span = self.t1 - self.t0;
        let fraction = if span > 0.0 {
            (t - self.t0) / span
        } else {
            0.0
        };
        self.left + (fraction as f32) * (self.right - self.left)
    }

    /// The time at screen X `x`.
    pub fn t(&self, x: f32) -> f64 {
        let width = f64::from(self.right - self.left);
        let fraction = if width > 0.0 {
            f64::from(x - self.left) / width
        } else {
            0.0
        };
        (self.t1 - self.t0).mul_add(fraction, self.t0)
    }

    fn is_drawable(&self) -> bool {
        self.t0.is_finite() && self.t1.is_finite() && self.t1 > self.t0 && self.right > self.left
    }
}

/// One element of a lane's plan, left to right.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LaneStroke {
    /// The value from change `index` holds across `[x0, x1]`.
    Hold { x0: f32, x1: f32, index: usize },
    /// Change `index` happens exactly at `x`.
    Edge { x: f32, index: usize },
    /// Two or more changes in every column of `[x0, x1)`.
    Activity { x0: f32, x1: f32 },
    /// An unknown value somewhere in every column of `[x0, x1)`: a stretch
    /// too narrow to draw as its own hold, at least one pixel wide.
    Unknown { x0: f32, x1: f32 },
}

/// Plan a lane whose value changes at `times` (strictly increasing), and is
/// unknown from the changes `unknown` lists by index (increasing).
///
/// Nothing is drawn before the first change: the net had not been observed.
pub fn plan_lane(times: &[f64], unknown: &[usize], map: &TimeMapping) -> Vec<LaneStroke> {
    let mut strokes = Vec::new();
    if times.is_empty() || !map.is_drawable() {
        return strokes;
    }
    // Changes inside the view: `first..end`.
    let first = times.partition_point(|&t| t <= map.t0);
    let end = times.partition_point(|&t| t <= map.t1);
    let mut held = first.checked_sub(1);
    let mut x_start = map.left;
    // The screen column a time falls in, in f64 so that bisecting on it and
    // assigning a change to its column are the same comparison: a column
    // boundary converted back to a time would round differently from the
    // change instants it is compared with.
    let left = f64::from(map.left);
    let scale = f64::from(map.right - map.left) / (map.t1 - map.t0);
    let column_of = |t: f64| (t - map.t0).mul_add(scale, left).floor();
    // The changes up to the end of `column`.
    let column_end = |column: f64| times[..end].partition_point(|&t| column_of(t) <= column);
    let is_unknown = |index: usize| unknown.binary_search(&index).is_ok();
    let hold = |strokes: &mut Vec<LaneStroke>, held: Option<usize>, x0: f32, x1: f32| {
        let Some(index) = held else { return };
        if x1 <= x0 {
            return;
        }
        if x1 - x0 < 1.0 && is_unknown(index) {
            push_unknown(strokes, x0, x1, map);
        } else {
            strokes.push(LaneStroke::Hold { x0, x1, index });
        }
    };

    let mut index = first;
    while index < end {
        let x = map.x(times[index]);
        let column = column_of(times[index]);
        let next = column_end(column).max(index + 1);
        if next - index >= 2 {
            // A dense column, extended across the dense columns after it.
            let mut block_end = column + 1.0;
            let mut cursor = next;
            while cursor < end {
                let following = column_of(times[cursor]);
                if following > block_end {
                    break;
                }
                let after = column_end(following).max(cursor + 1);
                if after - cursor < 2 {
                    break;
                }
                block_end = following + 1.0;
                cursor = after;
            }
            let x0 = column as f32;
            hold(&mut strokes, held, x_start, x0.max(x_start));
            // The value held into the block's first column counts as one of
            // its changes: an unknown that ends there is inside the block.
            let block = DenseBlock {
                columns: (column, block_end),
                changes: (held.unwrap_or(index), cursor),
            };
            block.plan(&mut strokes, times, unknown, &column_of, map);
            held = Some(cursor - 1);
            x_start = (block_end as f32).min(map.right);
            index = cursor;
        } else {
            hold(&mut strokes, held, x_start, x);
            strokes.push(LaneStroke::Edge { x, index });
            held = Some(index);
            x_start = x;
            index += 1;
        }
    }
    hold(&mut strokes, held, x_start, map.right);
    strokes
}

/// An unknown stretch across `[x0, x1]`, widened to the whole pixels it
/// touches, at least one, and merged into the one it continues.
fn push_unknown(strokes: &mut Vec<LaneStroke>, x0: f32, x1: f32, map: &TimeMapping) {
    let x0 = x0.floor();
    let x1 = x1.ceil().max(x0 + 1.0).min(map.right);
    let x0 = x0.min(x1 - 1.0).max(map.left);
    if let Some(LaneStroke::Unknown { x1: last, .. }) = strokes.last_mut()
        && *last >= x0
    {
        *last = last.max(x1);
        return;
    }
    strokes.push(LaneStroke::Unknown { x0, x1 });
}

/// A run of dense columns and the changes inside it.
struct DenseBlock {
    /// The block's screen columns, `[first, end)`.
    columns: (f64, f64),
    /// The changes whose values the block shows, by index, `[first, end)`.
    changes: (usize, usize),
}

impl DenseBlock {
    /// Draw the block as activity, with every column an unknown value
    /// reaches drawn as unknown instead. Each step consumes at least one
    /// column and finds the next unknown change by bisection.
    fn plan(
        &self,
        strokes: &mut Vec<LaneStroke>,
        times: &[f64],
        unknown: &[usize],
        column_of: &impl Fn(f64) -> f64,
        map: &TimeMapping,
    ) {
        let (block_first, block_end) = self.columns;
        let (first, end) = self.changes;
        let activity = |strokes: &mut Vec<LaneStroke>, from: f64, to: f64| {
            if to > from {
                strokes.push(LaneStroke::Activity {
                    x0: (from as f32).max(map.left),
                    x1: (to as f32).min(map.right),
                });
            }
        };
        let mut at = block_first;
        let mut k = unknown.partition_point(|&change| change < first);
        while k < unknown.len() && unknown[k] < end {
            let change = unknown[k];
            let from = column_of(times[change]).max(block_first);
            let to = times
                .get(change + 1)
                .map_or(block_end - 1.0, |&t| column_of(t))
                .min(block_end - 1.0);
            activity(strokes, at, from);
            push_unknown(strokes, from as f32, (to + 1.0) as f32, map);
            at = to + 1.0;
            k += unknown[k..]
                .partition_point(|&change| change < end && column_of(times[change]) <= to);
        }
        activity(strokes, at, block_end);
    }
}
