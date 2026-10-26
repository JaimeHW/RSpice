//! The hardcopy feature observed through a whole application: the cases
//! that print what the Results workspace shows, and so read the results
//! feature's state beside the hardcopy's own.

#[cfg(not(target_arch = "wasm32"))]
mod histogram_publication;
mod publish;
mod sources;
