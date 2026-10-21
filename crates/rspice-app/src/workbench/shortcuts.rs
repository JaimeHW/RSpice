//! Keyboard shortcut profiles beyond the values the preferences hold.
//!
//! The profile values themselves live in [`crate::state::shortcuts`]. This
//! module owns the portable artifacts that carry a profile between
//! installations, the device-local persistence behind the preset library, and
//! the import and export workflow.

pub(crate) mod artifacts;
pub(crate) mod library_persistence;
#[cfg(test)]
mod profile_tests;
pub(crate) mod profile_workflow;
