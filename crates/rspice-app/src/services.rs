//! Services Module
//!
//! Backend services for the RSpice UI application.
//! Contains specialized processing and analysis utilities.

pub(crate) mod cloud_account;
#[cfg(windows)]
pub(crate) mod dpapi;
#[cfg(test)]
mod drc_tests;
pub(crate) mod license;
pub(crate) mod live_protocol;
pub(crate) mod model_hub;
pub(crate) mod model_qualification;

// No flattening re-exports: every consumer of a service type names the module
// that defines it, so the owner of a name is readable at the use site.
