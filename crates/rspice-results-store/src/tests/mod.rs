//! Crate tests. Byte fixtures are built here from the format's constants; nothing is read from disk.

mod events_tests;
mod manifest_tests;
mod reader_tests;
mod reopen_tests;
#[cfg(not(target_arch = "wasm32"))]
mod run_writer_tests;
#[cfg(not(target_arch = "wasm32"))]
mod sink_tests;
mod support;
mod tile_tests;
#[cfg(any(unix, windows))]
mod watch_tests;
mod writer_tests;
