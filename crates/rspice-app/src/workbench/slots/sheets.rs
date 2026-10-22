//! The sheets feature's dialogs: drawing-sheet defaults, size presets,
//! reconciliation, the overflow review and sheet deletion.

/// What the sheets feature keeps open between frames.
#[derive(Debug, Clone, Default)]
pub(crate) struct SheetsState {
    /// Project and personal defaults for future authored drawing sheets.
    pub(crate) drawing_sheet_defaults: crate::workbench::app::DrawingSheetDefaultsDialogState,

    /// Custom drawing-sheet size library, editor, and portable JSON transfer.
    pub(crate) drawing_sheet_presets: crate::workbench::app::DrawingSheetPresetDialogsState,

    /// Document-wide authored-sheet reconciliation and title-field editing.
    pub(crate) drawing_sheet_support: crate::workbench::app::DrawingSheetSupportState,

    /// Live advisory review of printable objects outside the authored drawing
    /// area or overlapping the title block.
    pub(crate) drawing_sheet_overflow_open: bool,

    /// A sheet delete waiting for confirmation because it moves objects.
    pub(crate) delete_sheet:
        Option<crate::workbench::app::dialogs::delete_sheet::PendingSheetDelete>,
}
