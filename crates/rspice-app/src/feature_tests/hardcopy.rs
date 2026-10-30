//! The hardcopy feature observed through the session or a whole
//! application: the cases that build the application state, read the
//! product's own commands, or print what the Results workspace shows beside
//! the hardcopy's own state.

#[cfg(not(target_arch = "wasm32"))]
mod content_options;
mod exports;
#[cfg(not(target_arch = "wasm32"))]
mod histogram_publication;
mod publication_snapshot;
mod publish;
mod render;
mod report_authoring;
mod sources;
mod worker;
