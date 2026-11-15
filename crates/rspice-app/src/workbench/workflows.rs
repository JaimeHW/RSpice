//! Multi-step operations that mutate application state.
//!
//! A workflow is what sits between a command and the state it changes when the
//! change is not a single assignment: saving needs a canonical binding and a
//! transaction, exporting needs a format decision and an IO backend, importing
//! a netlist needs validation before it replaces anything. Each takes an
//! injectable IO backend so the tests can run them without touching a disk or
//! a file picker.

pub(crate) use rspice_app_services::workbench::workflows::export_workflow;
pub(crate) mod file_actions;
pub(crate) use rspice_app_project::workbench::workflows::file_workflow;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod foreign_schematic_workflow;
pub(crate) use rspice_app_project::workbench::workflows::project_workflow;
