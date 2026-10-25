//! Tests that need the whole application.
//!
//! Each one observes a slice's behaviour through `AppState` or `RSpiceApp`,
//! which only the shell can construct, so it lives here rather than beside
//! the code it tests. One module per slice.

mod framework;
mod model;
mod run;
mod services;
