//! Whether a simulation may start, as a rule rather than a method.
//!
//! This is the single answer behind every Run affordance: the toolbar, the run
//! bar, the menu entry, and F5 all gate on it, so it must not drift between
//! them. It used to live as a method on `AppState`, which meant reading it
//! required an entire application and hid which five things it actually
//! consults.
//!
//! Taking those five explicitly makes the rule testable on its own and makes
//! the inputs auditable: a change to the plan, the configured root, the
//! schematic, the process corner, or the retained DRC result is the complete
//! list of what can change the answer.

use crate::services::drc::DrcResult;
use crate::state::model_library::ModelLibraryManager;
use crate::state::{ProjectWorkspace, SchematicState};
use crate::workbench::app_state::SimSetupState;

/// The user-facing reason a new run cannot start, excluding the transient
/// "already running" state so a queued re-run can share the same rule.
///
/// `blocking_drc` is the retained editor DRC result *only when it still
/// describes the active schematic*; a stale result is not a blocker because it
/// no longer describes the current topology.
pub(crate) fn run_preflight_block_reason(
    sim_setup: &SimSetupState,
    workspace: &ProjectWorkspace,
    schematic: &SchematicState,
    model_library: &ModelLibraryManager,
    blocking_drc: Option<&DrcResult>,
) -> Option<String> {
    let plan = match sim_setup.stable_analysis_plan() {
        Ok(plan) => plan,
        Err(error) => return Some(error),
    };
    let enabled = plan
        .instances()
        .iter()
        .filter(|instance| instance.enabled())
        .collect::<Vec<_>>();
    if enabled.is_empty() {
        return Some("Enable at least one analysis instance in the simulation plan".to_owned());
    }
    let configured_root = workspace.simulation_root_reference();
    let Some(configured_schematic) =
        workspace.simulation_root_schematic(&workspace.active_view, schematic)
    else {
        return Some(format!(
            "Resolve the configured simulation root '{}' before running",
            configured_root.display_path()
        ));
    };
    if configured_schematic.components.is_empty() {
        return Some(format!(
            "Add a component to the configured simulation root '{}' before running",
            configured_root.display_path()
        ));
    }
    if let Some(issue) = plan.validation_issues().first() {
        return Some(format!("Correct simulation plan: {issue}"));
    }
    if let Some((instance, error)) = enabled.iter().find_map(|instance| {
        sim_setup
            .analysis_draft_validation_error(instance.draft())
            .map(|error| (*instance, error))
    }) {
        return Some(format!(
            "Correct {} instance {}: {error}",
            instance.kind().label(),
            instance.id()
        ));
    }
    if let Err(error) = model_library.reference_process_model_cards(sim_setup.reference_pvt.process)
    {
        return Some(error);
    }
    // The retained editor DRC result describes only the active schematic.
    // Hierarchy-wide configured-root checks are owned by the execution
    // preflight/controller pipeline, so an unrelated editor tab must not
    // block (or clear) the configured design's run eligibility.
    let configured_root_is_active = configured_root
        .key()
        .eq_ignore_ascii_case(&workspace.active_view.key());
    if configured_root_is_active && let Some(result) = blocking_drc {
        let summary = result.summary();
        return Some(format!(
            "Fix current DRC errors before simulation ({} critical, {} error{})",
            summary.critical,
            summary.errors,
            if summary.errors == 1 { "" } else { "s" }
        ));
    }
    None
}
