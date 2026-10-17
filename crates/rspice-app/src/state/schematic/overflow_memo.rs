//! The sheet's overflow advisories, kept across frames while the drawing and
//! the sheet stand still.
//!
//! Finding them measures every object's printable extent; the answer changes
//! only on an edit or a sheet change, so a steady frame reuses it.

use std::sync::Arc;

use rspice_schematic_editor::view::drawing_sheet::{
    DrawingSheetGeometry, DrawingSheetOverflowSummary,
};

use super::transform_preview_memo::DrawingInputs;

/// Everything the advisories are found from: the drawing, its symbols and
/// active sheet, and the sheet's geometry.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct OverflowKey {
    pub(crate) drawing: DrawingInputs,
    pub(crate) geometry: DrawingSheetGeometry,
}

#[derive(Clone, Default)]
pub struct OverflowSummaryMemo {
    entry: Option<(OverflowKey, Arc<DrawingSheetOverflowSummary>)>,
}

impl std::fmt::Debug for OverflowSummaryMemo {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("OverflowSummaryMemo")
            .field(
                "findings",
                &self
                    .entry
                    .as_ref()
                    .map(|(_, summary)| summary.finding_count()),
            )
            .finish()
    }
}

impl OverflowSummaryMemo {
    /// The advisories found from exactly `key`, if they are held.
    pub(crate) fn current(&self, key: &OverflowKey) -> Option<Arc<DrawingSheetOverflowSummary>> {
        self.entry
            .as_ref()
            .filter(|(held, _)| held == key)
            .map(|(_, summary)| Arc::clone(summary))
    }

    pub(crate) fn store(&mut self, key: OverflowKey, summary: Arc<DrawingSheetOverflowSummary>) {
        self.entry = Some((key, summary));
    }
}
