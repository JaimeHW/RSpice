//! Crate tests. Byte fixtures are built here from the format's constants; nothing is read from disk.

mod events_tests;
mod manifest_tests;
mod reader_tests;
#[cfg(not(target_arch = "wasm32"))]
mod run_writer_tests;
mod support;
mod tile_tests;
mod writer_tests;
