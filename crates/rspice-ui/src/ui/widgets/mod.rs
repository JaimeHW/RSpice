//! The widget vocabulary of the design system.
//!
//! Each submodule implements one widget family, styled exclusively from the
//! active [`crate::ui::tokens::Tokens`]. Widgets take data in and report
//! interactions out via [`egui::Response`] (or small result enums) — they
//! never reach into application state.

mod button;
mod chip;
mod dialog;
mod docbar;
mod form;
mod pane;
mod pill;
mod section;
mod select;
mod table;
mod toast;
mod tree;

pub use button::{Button, IconButton};
pub use chip::chip;
pub use dialog::{Dialog, DialogChoice, DialogSize, dialog_tabs};
pub use docbar::{crumb_text, docbar};
pub use form::{check_row, choice_row, input_row, input_row_readonly, kv_row, mono_input};
pub use pane::{
    PANE_FOOTER_H, PANE_HEADER_H, PANE_RAIL_W, PaneSide, pane_footer, pane_header,
    pane_section_label, two_pane,
};
pub use pill::{Pill, PillState};
pub use section::section_header;
pub use select::select;
pub use table::measurement_table;
pub use toast::{NotificationCategory, NotificationRecord, Toast, ToastKind, Toasts};
pub use tree::{TreeRow, TreeRowResult};
