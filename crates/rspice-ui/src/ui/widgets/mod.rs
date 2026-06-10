//! The widget vocabulary of the design system.
//!
//! Each submodule implements one widget family, styled exclusively from the
//! active [`crate::ui::tokens::Tokens`]. Widgets take data in and report
//! interactions out via [`egui::Response`] (or small result enums) — they
//! never reach into application state.

mod button;
mod chip;
mod docbar;
mod form;
mod pill;
mod section;
mod table;
mod toast;
mod tree;

pub use button::{Button, IconButton};
pub use chip::chip;
pub use docbar::{crumb_text, docbar};
pub use form::{check_row, input_row, input_row_readonly, kv_row, mono_input};
pub use pill::{Pill, PillState};
pub use section::section_header;
pub use table::measurement_table;
pub use toast::{Toast, ToastKind, Toasts};
pub use tree::{TreeRow, TreeRowResult};
