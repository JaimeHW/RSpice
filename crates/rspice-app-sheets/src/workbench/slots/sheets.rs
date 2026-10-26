//! The sheets feature's dialogs: drawing-sheet defaults, size presets,
//! reconciliation, the overflow review and sheet deletion, and the request to
//! reveal the inspector's Sheet section.

/// What the sheets feature keeps open between frames.
#[derive(Debug, Clone, Default)]
pub struct SheetsState {
    /// Project and personal defaults for future authored drawing sheets.
    pub drawing_sheet_defaults: crate::workbench::app::DrawingSheetDefaultsDialogState,

    /// Custom drawing-sheet size library, editor, and portable JSON transfer.
    pub drawing_sheet_presets: crate::workbench::app::DrawingSheetPresetDialogsState,

    /// Document-wide authored-sheet reconciliation and title-field editing.
    pub drawing_sheet_support: crate::workbench::app::DrawingSheetSupportState,

    /// Live advisory review of printable objects outside the authored drawing
    /// area or overlapping the title block.
    pub drawing_sheet_overflow_open: bool,

    /// A sheet delete waiting for confirmation because it moves objects.
    pub delete_sheet: Option<crate::workbench::app::dialogs::delete_sheet::PendingSheetDelete>,

    /// A one-frame request that the inspector's sheet panel scroll its Sheet
    /// section into view, the next time it shows.
    pub reveal_sheet_section: bool,
}
