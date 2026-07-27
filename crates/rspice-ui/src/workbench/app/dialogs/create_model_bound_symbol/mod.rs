//! Mockup-owned Create model-bound symbol transaction.
//!
//! State, controller, renderer, and verification are intentionally split so
//! this governed workflow can scale without returning to the former GUI shell.

mod controller;
mod render;
mod state;

pub(crate) use controller::open_create_model_bound_symbol_dialog;
pub(crate) use state::CreateModelBoundSymbolDialogState;

#[cfg(test)]
mod tests;
