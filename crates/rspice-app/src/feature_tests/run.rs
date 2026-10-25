//! The run tier's behaviour, observed through a whole session.

mod campaign;
mod completion_paths;
mod controller;
mod measurement;
mod output_contract;
mod preflight;
mod prepared_run;
mod results_post;
mod transient_post;
mod veriloga_project;

use crate::simulation::controller::test_host::RunTestHost;
use crate::workbench::app_state::AppState;

/// A new session holding a run fixture's model and runs' slot, for a case
/// that goes on from the run tier to a feature above it. The session's other
/// slots are a new session's.
fn session_of(host: RunTestHost) -> AppState {
    let mut session = AppState::default();
    session.model = host.model;
    session.runs = host.runs;
    session
}
