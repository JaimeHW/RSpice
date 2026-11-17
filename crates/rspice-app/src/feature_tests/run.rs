//! The run tier's behaviour, observed through a whole session.

#[cfg(not(target_arch = "wasm32"))]
#[cfg(not(target_arch = "wasm32"))]
mod campaign;
mod completion_paths;
mod controller;
#[cfg(not(target_arch = "wasm32"))]
mod deck_edit_commit;
#[cfg(not(target_arch = "wasm32"))]
mod deck_run_control;
#[cfg(not(target_arch = "wasm32"))]
mod deck_source_menu;
#[cfg(not(target_arch = "wasm32"))]
mod drawn_exit_projects;
#[cfg(not(target_arch = "wasm32"))]
mod engine_recovery_path;
mod measurement;
mod output_contract;
mod preflight;
mod prepared_run;
mod results_post;
mod transient_post;
mod veriloga_project;

use crate::workbench::app_state::AppState;
use rspice_app_runs::simulation::controller::test_host::RunTestHost;

/// A new session holding a run fixture's model and runs' slot, for a case
/// that goes on from the run tier to a feature above it. The session's other
/// slots are a new session's.
fn session_of(host: RunTestHost) -> AppState {
    let mut session = AppState::default();
    session.model = host.model;
    session.runs = host.runs;
    session
}
