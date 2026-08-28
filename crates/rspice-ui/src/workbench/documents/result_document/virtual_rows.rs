//! Viewport-bounded row plans for the evidence tables.
//!
//! The waveform sheets never draw more than their pixels, but the evidence
//! tables were laying out, sensing and painting every retained row every
//! frame. That is fine for a rule list and ruinous for an operating point:
//! `save_device_op` on a real block emits one row per device, and the OP
//! sheet is the largest table the product produces.
//!
//! `egui`'s own `show_rows` wants one uniform row height, and these tables
//! interleave group headers with data rows at deliberately different heights.
//! So the plan is computed here from a prefix sum instead: the caller pads
//! above and below the rows it actually draws, which keeps the scrollbar
//! honest about the full extent.

/// Which rows a viewport covers, and the space standing in for the rest.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct RowPlan {
    /// First row to draw.
    pub(super) first: usize,
    /// One past the last row to draw.
    pub(super) end: usize,
    /// Height of the rows above `first`.
    pub(super) leading: f32,
    /// Height of the rows from `end` onward.
    pub(super) trailing: f32,
}

impl RowPlan {
    /// Rows `first..end`, as a range.
    pub(super) const fn range(self) -> std::ops::Range<usize> {
        self.first..self.end
    }
}

/// Running row offsets: `offsets[i]` is the top of row `i`, and the last
/// element is the total height. Built once per (data, filter, sort) rather
/// than per frame.
#[derive(Debug, Clone, Default, PartialEq)]
pub(super) struct RowOffsets {
    offsets: Vec<f32>,
}

impl RowOffsets {
    pub(super) fn from_heights(heights: impl IntoIterator<Item = f32>) -> Self {
        let mut offsets = vec![0.0];
        let mut total = 0.0;
        for height in heights {
            total += height.max(0.0);
            offsets.push(total);
        }
        Self { offsets }
    }

    pub(super) fn rows(&self) -> usize {
        self.offsets.len().saturating_sub(1)
    }

    pub(super) fn total_height(&self) -> f32 {
        self.offsets.last().copied().unwrap_or(0.0)
    }

    /// The rows intersecting `viewport`, with one row of margin on each side
    /// so a partially scrolled row is drawn rather than popping in.
    pub(super) fn plan(&self, viewport: egui::Rangef) -> RowPlan {
        let rows = self.rows();
        if rows == 0 {
            return RowPlan {
                first: 0,
                end: 0,
                leading: 0.0,
                trailing: 0.0,
            };
        }
        // `partition_point` is sound here by construction: the offsets are a
        // running sum of non-negative heights, so they are non-decreasing.
        let first = self
            .offsets
            .partition_point(|offset| *offset <= viewport.min)
            .saturating_sub(1)
            .min(rows - 1);
        let end = self
            .offsets
            .partition_point(|offset| *offset < viewport.max)
            .clamp(first + 1, rows);
        RowPlan {
            first,
            end,
            leading: self.offsets[first],
            trailing: self.total_height() - self.offsets[end],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn uniform(rows: usize, height: f32) -> RowOffsets {
        RowOffsets::from_heights(std::iter::repeat_n(height, rows))
    }

    #[test]
    fn a_viewport_covers_only_the_rows_it_can_show() {
        let offsets = uniform(100_000, 25.0);
        let plan = offsets.plan(egui::Rangef::new(0.0, 700.0));

        assert_eq!(plan.first, 0);
        assert!(plan.end <= 30, "drew {} rows for 700 px", plan.end);
        assert_eq!(plan.leading, 0.0);
        // The space below has to account for everything not drawn, or the
        // scrollbar claims the table is 28 rows long.
        assert!(
            (plan.leading + plan.trailing + (plan.end - plan.first) as f32 * 25.0
                - offsets.total_height())
            .abs()
                < 0.5
        );
    }

    #[test]
    fn a_scrolled_viewport_keeps_the_partly_visible_row_on_each_edge() {
        let offsets = uniform(1_000, 20.0);
        let plan = offsets.plan(egui::Rangef::new(510.0, 610.0));

        // 510 falls inside row 25 and 610 inside row 30; both must be drawn.
        assert!(plan.first <= 25, "first = {}", plan.first);
        assert!(plan.end >= 31, "end = {}", plan.end);
        assert!(plan.leading <= 510.0);
        assert!(
            (plan.leading + plan.trailing + (plan.end - plan.first) as f32 * 20.0
                - offsets.total_height())
            .abs()
                < 0.5
        );
    }

    #[test]
    fn heterogeneous_rows_keep_their_own_heights() {
        // The shape these tables actually have: a group header, then its
        // rows, then the next group.
        let offsets = RowOffsets::from_heights([24.0, 25.0, 25.0, 24.0, 25.0, 25.0, 25.0]);
        assert_eq!(offsets.rows(), 7);
        assert!((offsets.total_height() - 173.0).abs() < f32::EPSILON * 173.0);

        let plan = offsets.plan(egui::Rangef::new(0.0, 173.0));
        assert_eq!(plan.range(), 0..7);
        assert_eq!(plan.leading, 0.0);
        assert_eq!(plan.trailing, 0.0);
    }

    #[test]
    fn a_viewport_past_the_end_still_names_a_drawable_row() {
        let offsets = uniform(10, 25.0);
        let plan = offsets.plan(egui::Rangef::new(10_000.0, 10_700.0));

        assert!(plan.first < offsets.rows());
        assert!(plan.end <= offsets.rows());
        assert!(plan.end > plan.first, "an empty plan draws nothing at all");
    }

    #[test]
    fn no_rows_plans_nothing() {
        let offsets = RowOffsets::from_heights([]);
        assert_eq!(offsets.rows(), 0);
        assert_eq!(offsets.total_height(), 0.0);
        assert_eq!(offsets.plan(egui::Rangef::new(0.0, 700.0)).range(), 0..0);
    }
}
