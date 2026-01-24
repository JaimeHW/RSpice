//! Simulation Dialog Module
//!
//! A tabbed dialog for configuring simulation analyses.
//! Replaces manual SPICE command editing with intuitive form inputs.
//!
//! This module is split into focused submodules for maintainability:
//! - `types` - AnalysisTab enum and dialog props
//! - `form_components` - Reusable form input components
//! - `tabs/` - Individual analysis configuration tabs
//! - `dialog` - Main SimulationDialog component

mod dialog;
mod form_components;
pub mod tabs;
mod types;

// Re-export public types
pub use dialog::SimulationDialog;
pub use form_components::{FormInput, FormRow};
pub use types::{AnalysisTab, SimulationDialogProps};
