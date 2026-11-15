//! Mockup-owned Create model-bound symbol transaction.
//!
//! State, controller, renderer, and verification are intentionally split so
//! this governed workflow can scale without returning to the former GUI shell.

pub mod controller;
mod render;
pub mod state;

pub use controller::{
    open_create_model_bound_symbol_dialog, open_create_subcircuit_bound_symbol_dialog,
    symbol_attribute_definition,
};
pub(in crate::workbench) use render::render_create_model_bound_symbol_dialog;
pub use state::CreateModelBoundSymbolDialogState;

#[cfg(any(test, feature = "test-support"))]
pub mod tests;
