//! Services Module
//!
//! Backend services for the RSpice UI application.
//! Contains specialized processing and analysis utilities.

pub(crate) mod cloud_account;
#[cfg(windows)]
pub(crate) mod dpapi;
pub(crate) mod drc;
pub(crate) mod license;
pub(crate) mod live_protocol;
pub(crate) mod model_hub;
pub(crate) mod safety;
pub(crate) mod simulation_runner;
pub(crate) mod yield_manager;

// No flattening re-exports: every consumer of a service type names the module
// that defines it, so the owner of a name is readable at the use site.
