//! Turning live documents into printable pages.
//!
//! Named for what it does rather than what it prints, because `crate::hardcopy`
//! already owns the persisted side — page setup, print mappings, and the
//! digest-authenticated source-set records. This is the half that cannot live
//! down there: resolving a source set needs the schematic symbol library and
//! the analysis viewers, so it stays in the shell.
//!
//! The three run in order. `sources` freezes exact document revisions and
//! authenticates them, `render` turns that frozen snapshot into pages, and
//! `print` hands the pages to the platform.

pub(crate) mod print;
pub(crate) mod render;
pub(crate) mod sources;
