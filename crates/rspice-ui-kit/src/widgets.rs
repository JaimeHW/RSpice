//! The widget vocabulary of the design system.
//!
//! Each submodule implements one widget family, styled exclusively from the
//! active [`crate::tokens::Tokens`]. Widgets take data in and report
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
pub fn mark_response_disabled(response: &mut egui::Response) {
    response.flags.remove(egui::response::Flags::ENABLED);
}

mod button;
mod chip;
mod command_form;
mod dialog;
mod docbar;
mod form;
pub mod notice;
#[cfg(all(any(test, feature = "test-support"), not(target_arch = "wasm32")))]
pub mod painted_runs;
mod pane;
mod schematic_command;
mod section;
mod segmented;
mod select;
mod selection_command;
mod status_mark;
mod switch;
mod table;
mod toast;
mod tree;
mod view_switch;

pub use button::{Button, IconButton};
pub use chip::chip;
pub use command_form::{CommandForm, FormRows};
pub use dialog::{
    Dialog, DialogChoice, DialogHintTone, DialogInitialFocus, DialogResponse, DialogSize,
    DialogTransactionTone,
};
pub use docbar::docbar_at_height;
pub use form::name_control;
pub use form::{
    choice_row, field_label, input_row, kv_row, mono_input, read_only_value, switch_row,
};
pub use pane::{
    PANE_FOOTER_H, PANE_HEADER_H, PANE_RAIL_W, PaneSide, pane_footer, pane_header,
    pane_section_label, two_pane,
};
pub use schematic_command::{SchematicCommandPreview, schematic_command_workflow};
pub use section::section_header;
pub use segmented::{SegmentedWidth, segmented};
pub use select::{SelectOutput, select};
pub use select::{select_mono_with_response, select_with_disabled, select_with_response};
pub use selection_command::{
    NotePreviewStyle, PreviewPoint, SelectionImpact, SelectionPreview, ShapePreviewStroke,
    selection_command_workflow, workflow_preview_status,
};
pub use status_mark::{StatusMark, paint_status_mark};
pub use switch::{SWITCH_WIDTH, paint_switch};
pub use table::measurement_table;
pub use toast::{
    MirroredEntry, NotificationAction, NotificationCategory, NotificationRecord, ToastKind, Toasts,
};
pub use tree::{TreeRow, TreeRowResult};
pub use view_switch::{ViewOption, view_switch};
