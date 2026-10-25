//! The status bar's engine segment: what the engine is doing, and how far
//! preflight has got while it prepares a run.

use crate::simulation::status::EngineAvailability;

pub fn simulation_progress_percent(progress: f64) -> u8 {
    (progress.clamp(0.0, 1.0) * 100.0).round() as u8
}

/// The engine segment, or nothing while the engine sits ready and idle.
pub fn simulation_engine_status(
    simulation: &crate::state::SimulationState,
    availability: &EngineAvailability,
) -> Option<String> {
    if availability.failure_reason().is_some() {
        return Some("Engine unavailable".to_owned());
    }
    if *availability == EngineAvailability::Starting && !simulation.cancellation_is_pending() {
        return Some("Engine starting".to_owned());
    }
    if !simulation.execution.has_active_execution() {
        return (*availability == EngineAvailability::Restartable)
            .then(|| "Engine stopped".to_owned());
    }
    let phase = match simulation.active_execution_lifecycle() {
        Some(crate::state::SimulationRunLifecycle::Preparing) => "preparing",
        Some(crate::state::SimulationRunLifecycle::Cancelling) => "stopping",
        Some(crate::state::SimulationRunLifecycle::Running)
        | Some(
            crate::state::SimulationRunLifecycle::LegacyUnknown
            | crate::state::SimulationRunLifecycle::Completed
            | crate::state::SimulationRunLifecycle::Failed
            | crate::state::SimulationRunLifecycle::Aborted
            | crate::state::SimulationRunLifecycle::Interrupted,
        )
        | None => "running",
    };
    let phase = if simulation.cancellation_is_pending() {
        "stopping"
    } else {
        phase
    };
    Some(format!(
        "Engine {phase} · {}%",
        simulation_progress_percent(simulation.execution.progress)
    ))
}

/// The engine segment while preflight prepares the run and then has the
/// engine build its decks: the stage, and how far it has got once it has
/// counted what it works through. A run of one point is not counted.
pub fn preflight_status(stage: crate::simulation::controller::PreflightStage) -> String {
    use crate::simulation::controller::PreflightStage;
    match stage {
        PreflightStage::Preparing(progress) if progress.total > 1 => format!(
            "Preparing · {} of {} points",
            progress.prepared, progress.total
        ),
        PreflightStage::Preparing(_) => "Preparing".to_owned(),
        PreflightStage::EngineChecking(progress) if progress.total > 0 => format!(
            "Engine checking · {} of {} deck{}",
            progress.built,
            progress.total,
            if progress.total == 1 { "" } else { "s" }
        ),
        PreflightStage::EngineChecking(_) => "Engine checking".to_owned(),
    }
}
