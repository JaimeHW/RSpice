//! The widget vocabulary of the design system.
//!
//! Each submodule implements one widget family, styled exclusively from the
//! active [`crate::ui::tokens::Tokens`]. Widgets take data in and report
//! interactions out via [`egui::Response`] (or small result enums) — they
//! never reach into application state.

/// Report a widget's own disabled state on the response it returns.
///
/// A widget that draws its own disabled look — rather than deferring to
/// [`egui::Ui::add_enabled_ui`] — has to say so here too. `Ui::allocate_*` and
/// `Ui::interact` copy `enabled` from the **`Ui`**, which is still enabled, and
/// every disabled tooltip in egui keys off the response's own flag. Without
/// this the reason a call site attaches to a blocked control is dropped with
/// no diagnostic: the code reads as if it explains itself and nothing renders.
/// Hover for a disabled response is resolved from the rect rather than the
/// flag, so clearing it costs no hit testing.
pub(crate) fn mark_response_disabled(response: &mut egui::Response) {
    response.flags.remove(egui::response::Flags::ENABLED);
}

mod button;
mod chip;
mod dialog;
mod docbar;
mod form;
mod pane;
mod schematic_command;
mod section;
mod segmented;
mod select;
mod selection_command;
mod table;
mod toast;
mod tree;

pub use button::{Button, IconButton};
pub use chip::chip;
pub use dialog::{Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone};
pub use docbar::docbar_at_height;
pub use form::{check_row, choice_row, input_row, kv_row, mono_input};
pub use pane::{
    PANE_FOOTER_H, PANE_HEADER_H, PANE_RAIL_W, PaneSide, pane_footer, pane_header,
    pane_section_label, two_pane,
};
pub(crate) use schematic_command::{SchematicCommandPreview, schematic_command_workflow};
pub use section::section_header;
pub(crate) use segmented::{SegmentedWidth, segmented};
pub use select::select;
pub(crate) use select::{select_mono_with_response, select_with_disabled, select_with_response};
pub(crate) use selection_command::{
    NotePreviewStyle, PreviewPoint, SelectionImpact, SelectionPreview, ShapePreviewStroke,
    selection_command_workflow, workflow_preview_status,
};
pub use table::measurement_table;
pub use toast::{
    MirroredEntry, NotificationAction, NotificationCategory, NotificationRecord, ToastKind, Toasts,
};
pub use tree::{TreeRow, TreeRowResult};
