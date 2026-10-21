//! The BSIM model cards the device tests are pinned against, for tests above
//! the device models that run decks built on the same cards.
//!
//! The cards live beside the model that owns them; a test elsewhere names
//! them here rather than by a path into this crate's source tree.

/// The BSIM3v3 oracle cards (`n018`/`p018`, LEVEL=49, CAPMOD=3).
#[doc(hidden)]
pub const BSIM3V3_MODELS018: &str = include_str!("bsim3v3/testdata/models018.lib");

/// The BSIM4 oracle cards (`n45`/`p90`, LEVEL=54, CAPMOD=2).
#[doc(hidden)]
pub const BSIM4V8_MODELS45: &str = include_str!("bsim4v8/testdata/models45.lib");
