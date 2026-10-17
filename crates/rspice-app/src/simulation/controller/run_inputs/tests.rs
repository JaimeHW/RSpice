//! What a run set's identity holds of the retained runs a checkpoint resumes
//! from.

use super::*;
use crate::simulation::controller::test_execution::monte_carlo::{
    checkpoint_state, completed_checkpoint_fixture,
};
use crate::state::{MonteCarloCheckpointEvidence, SimulationRun};
use rspice_simulation_contract::mc_draft::McDialogState;

/// The input a run is refused for when what a checkpoint resumes from moved.
const RESUMED: &str = "the retained runs a checkpoint resumes from";

/// Set the checkpoint fixture's Monte Carlo to resume from `image`.
fn resume_from(state: &mut AppState, image: crate::product::ContentDigest) {
    let plan = state
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("the fixture owns a plan");
    let monte_carlo = plan
        .instances()
        .iter()
        .find(|instance| matches!(instance.draft(), AnalysisDraft::MonteCarlo(_)))
        .expect("the fixture runs a Monte Carlo")
        .id();
    plan.edit(monte_carlo, |draft| {
        let config = match draft {
            AnalysisDraft::MonteCarlo(dialog) => dialog.to_config(),
            _ => unreachable!("the instance found is the Monte Carlo"),
        };
        let mut config = config.expect("the fixture's Monte Carlo lowers");
        config
            .checkpoint
            .as_mut()
            .expect("the fixture's Monte Carlo captures checkpoints")
            .resume = vec![image];
        *draft = AnalysisDraft::MonteCarlo(McDialogState::from_config(&config));
    })
    .expect("the Monte Carlo takes the edit");
}

/// Run `sequence`, holding one Monte Carlo result: `image`, the checkpoint a
/// run in flight last published, from a task prepared into the snapshot
/// `snapshot`.
fn run_holding(sequence: u64, image: &MonteCarloCheckpointEvidence, snapshot: u8) -> SimulationRun {
    let mut run = SimulationRun::new(sequence);
    run.analyses.push(
        crate::state::AnalysisResult::live_monte_carlo_partial("Monte Carlo", image.clone())
            .with_provenance(
                crate::state::AnalysisResultProvenance::new(
                    crate::product::AnalysisInstanceId::new(),
                    crate::product::ObjectRevision::INITIAL,
                    crate::product::ContentDigest::from_bytes([snapshot; 32]),
                    Vec::new(),
                )
                .expect("a prepared-task provenance"),
            ),
    );
    run
}

/// A Monte Carlo that resumes from a checkpoint reads, of the history, the
/// result its selection takes the checkpoint from: the newest that holds it.
/// A run that holds no checkpoint leaves what it resumes from unchanged, and
/// so does a newer run holding the same checkpoint as admissibly, as the run
/// of a resume whose every trial was cached does when it publishes the
/// journal it resumed from. A newer holder the checkpoint cannot be resumed
/// against changes it, and so does no holder at all.
#[test]
fn a_monte_carlo_resuming_from_a_checkpoint_is_held_to_what_it_resumes_from() {
    let (_, bytes, _) = completed_checkpoint_fixture();
    let image = MonteCarloCheckpointEvidence::from_bytes(bytes).expect("a completed journal");
    let mut state = checkpoint_state();
    state
        .simulation
        .retained
        .runs
        .insert(0, run_holding(1, &image, 1));
    resume_from(&mut state, image.digest());
    let identity = RunInputIdentity::of_run_set(&state);
    assert_eq!(identity.run_set_moved_in(&state), None);

    state
        .simulation
        .retained
        .runs
        .insert(0, SimulationRun::new(2));
    assert_eq!(
        identity.run_set_moved_in(&state),
        None,
        "a run holding no checkpoint"
    );

    state
        .simulation
        .retained
        .runs
        .insert(0, run_holding(3, &image, 3));
    assert_eq!(
        identity.run_set_moved_in(&state),
        None,
        "a newer run holding the checkpoint as admissibly"
    );

    state.simulation.retained.runs[0].analyses[0].success = true;
    assert_eq!(
        identity.run_set_moved_in(&state),
        Some(RESUMED),
        "a newer holder that completed without its trial roster"
    );

    state.simulation.retained.runs.remove(0);
    assert_eq!(
        identity.run_set_moved_in(&state),
        None,
        "the holder it was taken from, once more the newest"
    );

    state
        .simulation
        .retained
        .runs
        .retain(|run| run.analyses.is_empty());
    assert_eq!(
        identity.run_set_moved_in(&state),
        Some(RESUMED),
        "no run holds the checkpoint"
    );
}
