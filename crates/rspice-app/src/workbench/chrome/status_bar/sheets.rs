//! The status bar's page segment: where the reader is in a multi-sheet
//! drawing.

use crate::workbench::app_state::Model;

/// Where the reader is in a multi-sheet drawing: the active sheet's page
/// number over the number of sheets, and the sentence that expands it.
///
/// A single-sheet cell view has no position to report, so the segment is
/// absent rather than reading `p1/1`. The page number is the catalog's, so it
/// is the number the sheet strip's chip and the printed title block carry.
pub(crate) fn sheet_position_summary(model: &Model) -> Option<(String, String)> {
    let catalog = model.multi_sheet_catalog()?;
    let active = catalog.active_sheet_id()?;
    let (page, count) = catalog.page_number_and_count(active)?;
    let name = catalog.find(active)?.name();
    Some((
        format!("p{page}/{count}"),
        format!("Sheet {page} of {count} \u{00b7} {name}"),
    ))
}
