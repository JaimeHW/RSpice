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
mod canvas_entry;
mod chip;
mod combo;
mod command_form;
mod context_menu;
mod dialog;
mod dialog_tabs;
mod docbar;
mod filter_clear;
mod form;
pub mod notice;
mod ordered_chips;
#[cfg(all(any(test, feature = "test-support"), not(target_arch = "wasm32")))]
pub mod painted_runs;
mod pane;
mod searchable_select;
mod segmented;
mod select;
mod status_mark;
mod switch;
mod table;
mod toast;
mod tree;
mod view_switch;

pub use button::{Button, IconButton};
pub use canvas_entry::{CanvasEntry, CanvasEntryChoice, CanvasEntryRows};
pub use chip::chip;
pub use combo::Combo;
pub(crate) use command_form::middle_elided;
pub use command_form::{CommandForm, FieldLine, FormRows, ListEdit, ListWords, TableColumn};
pub use context_menu::{context_menu_popup, raise_context_menu};
pub use dialog::{
    DIALOG_BODY_INSET, Dialog, DialogChoice, DialogHintTone, DialogInitialFocus, DialogResponse,
    DialogSize, DialogTransactionTone, band_header, focus_is_within_modal,
};
pub use dialog_tabs::dialog_tabs;
pub use docbar::docbar_at_height;
pub use filter_clear::{FILTER_CLEAR_INSET, filter_field_clear};
pub use form::name_control;
pub use form::{
    choice_row, field_label, input_row, kv_row, mono_input, mono_input_with_unit, switch_row,
    text_field, value_row,
};
pub use ordered_chips::{OrderedChipsEdit, ordered_chips};
pub use pane::{
    PANE_FOOTER_H, PANE_HEADER_H, PANE_INSET, PANE_RAIL_W, PaneSide, pane_footer,
    pane_footer_inset, pane_header, pane_header_height, pane_header_inset, two_pane,
};
pub use searchable_select::{SelectGroup, searchable_select};
pub use segmented::{SegmentedWidth, segmented};
pub use select::{SelectOutput, select};
pub use select::{
    SelectSwitch, select_mono_with_response, select_with_disabled, select_with_response,
    select_with_switch,
};
pub use status_mark::{
    PROBLEM_MARK_ADVANCE, PROBLEM_MARK_GAP, PROBLEM_MARK_SIDE, Problem, StatusMark, busy_mark,
    paint_problem_mark, paint_status_mark, problem_line,
};
pub use switch::{SWITCH_WIDTH, paint_switch};
pub use table::measurement_table;
pub use toast::{
    MirroredEntry, NotificationAction, NotificationCategory, NotificationRecord, ToastKind, Toasts,
};
pub use tree::{TreeRow, TreeRowResult};
pub use view_switch::{ViewOption, view_switch};
