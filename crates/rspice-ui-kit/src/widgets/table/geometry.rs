//! Where a table's columns sit, measured from the left edge of the viewport
//! the table is seen through.
//!
//! The leading frozen columns sit where they were laid out, whatever the
//! scroll; every other column sits `offset` points further left and shows
//! only right of the freeze edge. Only the columns that show are laid out
//! and painted, so a cross-tab of hundreds of points costs what its visible
//! part costs.

use std::ops::Range;

/// The least room the scrolled columns keep beside the frozen ones. A
/// viewport narrower than the frozen columns and this scrolls every column,
/// so no column is ever out of reach.
const SCROLLED_MIN_W: f32 = 64.0;

/// One head band's span: a label over the columns `first..=last`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub label: String,
    pub first: usize,
    pub last: usize,
}

impl Span {
    pub fn new(label: impl Into<String>, first: usize, last: usize) -> Self {
        Self {
            label: label.into(),
            first,
            last: last.max(first),
        }
    }

    /// One band from one label per column, starting at `first`: each run of
    /// equal neighbours becomes one span.
    pub fn runs<S: AsRef<str>>(first: usize, labels: &[S]) -> Vec<Self> {
        let mut spans: Vec<Self> = Vec::new();
        for (index, label) in labels.iter().enumerate() {
            let column = first + index;
            match spans.last_mut() {
                Some(span) if span.label == label.as_ref() => span.last = column,
                _ => spans.push(Self::new(label.as_ref(), column, column)),
            }
        }
        spans
    }
}

/// The columns' laid-out edges and how many of them are frozen.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct Columns {
    lefts: Vec<f32>,
    rights: Vec<f32>,
    widths: Vec<f32>,
    frozen: usize,
}

impl Columns {
    /// Columns of `widths`, the first `frozen` of them held in place while a
    /// `viewport` points wide scrolls the rest; none are held when that
    /// would leave the rest no room.
    pub(super) fn new(widths: Vec<f32>, frozen: usize, viewport: f32) -> Self {
        let mut lefts = Vec::with_capacity(widths.len());
        let mut rights = Vec::with_capacity(widths.len());
        let mut x = 0.0;
        for width in &widths {
            lefts.push(x);
            x += width;
            rights.push(x);
        }
        let frozen = frozen.min(widths.len());
        let frozen_width: f32 = widths[..frozen].iter().sum();
        let frozen = if frozen_width + SCROLLED_MIN_W <= viewport {
            frozen
        } else {
            0
        };
        Self {
            lefts,
            rights,
            widths,
            frozen,
        }
    }

    pub(super) fn len(&self) -> usize {
        self.widths.len()
    }

    pub(super) fn width(&self, column: usize) -> f32 {
        self.widths[column]
    }

    pub(super) fn frozen(&self) -> usize {
        self.frozen
    }

    /// Where the frozen columns end and the scrolled ones show from.
    pub(super) fn freeze_edge(&self) -> f32 {
        self.lefts
            .get(self.frozen)
            .copied()
            .unwrap_or_else(|| self.content_width())
    }

    pub(super) fn content_width(&self) -> f32 {
        self.rights.last().copied().unwrap_or(0.0)
    }

    /// Column `column`'s left edge at `offset`.
    pub(super) fn left_at(&self, column: usize, offset: f32) -> f32 {
        if column < self.frozen {
            self.lefts[column]
        } else {
            self.lefts[column] - offset
        }
    }

    /// The scrolled columns any part of which shows in a `viewport` points
    /// wide at `offset`.
    pub(super) fn scrolled_in_view(&self, offset: f32, viewport: f32) -> Range<usize> {
        let from = self.freeze_edge() + offset;
        let to = offset + viewport;
        let first =
            self.frozen + self.rights[self.frozen..].partition_point(|right| *right <= from);
        let end = self.frozen + self.lefts[self.frozen..].partition_point(|left| *left < to);
        first..end.max(first)
    }

    /// Every column that shows at `offset`: the frozen ones, then the
    /// scrolled ones in view.
    pub(super) fn in_view(&self, offset: f32, viewport: f32) -> impl Iterator<Item = usize> {
        (0..self.frozen).chain(self.scrolled_in_view(offset, viewport))
    }

    /// The column under `x`, measured from the viewport's left, at `offset`.
    pub(super) fn column_at(&self, x: f32, offset: f32) -> Option<usize> {
        if x < 0.0 {
            return None;
        }
        let content = if x < self.freeze_edge() {
            x
        } else {
            x + offset
        };
        let column = self
            .lefts
            .partition_point(|left| *left <= content)
            .checked_sub(1)?;
        (content < self.rights[column]).then_some(column)
    }

    /// The offset that brings `column` whole into a `viewport` points wide,
    /// moving no further than it must.
    pub(super) fn reveal(&self, column: usize, offset: f32, viewport: f32) -> f32 {
        if column < self.frozen || column >= self.len() {
            return offset;
        }
        let left = self.lefts[column] - offset;
        let right = self.rights[column] - offset;
        if left < self.freeze_edge() {
            self.lefts[column] - self.freeze_edge()
        } else if right > viewport {
            (self.rights[column] - viewport).min(self.lefts[column] - self.freeze_edge())
        } else {
            offset
        }
    }

    /// Where `span` shows at `offset`, as its left and right edge, and the
    /// part of it in view. A span over frozen columns shows only over the
    /// scrolled ones it reaches: the frozen columns' band cells are blank.
    pub(super) fn span_at(&self, span: &Span, offset: f32, viewport: f32) -> Option<SpanPlace> {
        let first = span.first.max(self.frozen);
        let last = span.last.min(self.len().checked_sub(1)?);
        if first > last {
            return None;
        }
        let left = self.left_at(first, offset);
        let right = self.rights[last] - offset;
        let shown_left = left.max(self.freeze_edge());
        let shown_right = right.min(viewport);
        (shown_right > shown_left).then_some(SpanPlace {
            left,
            right,
            shown: shown_left..shown_right,
        })
    }
}

/// Where one span sits, measured like the columns.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct SpanPlace {
    pub(super) left: f32,
    pub(super) right: f32,
    /// The part of it in view, which its label is centred in.
    pub(super) shown: Range<f32>,
}
