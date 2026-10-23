//! Whether a simulation may start, as a rule rather than a method.
//!
//! This is the single static eligibility answer used by governed workflows and
//! availability explanations. User-facing Run affordances remain actionable
//! so they can execute the complete preflight and show ordered remediation;
//! they never dispatch merely because this cheaper predicate passes.
//!
//! Taking those five explicitly makes the rule testable on its own and makes
//! the inputs auditable: a change to the plan, the configured root, the
//! schematic, the process corner, or the retained DRC result is the complete
//! list of what can change the answer.
//!
//! The preflight pass's own transient state lives here too: the report it
//! produced, the request that starts it, and where that report stands.

use crate::state::model_library::ModelLibraryManager;
use crate::state::{PreflightReport, ProjectWorkspace, SchematicState};
use crate::workbench::app_state::SimSetupState;
use rspice_design::drc::DrcResult;
use rspice_simulation::preparation::technology_demand;

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
    let configured_root = workspace.content.simulation_root_reference();
    let Some(configured_schematic) =
        workspace.simulation_root_schematic(&workspace.content.active_view, schematic)
    else {
        return Some(format!(
            "Resolve the configured simulation root '{}' before running",
            configured_root.display_path()
        ));
    };
    if configured_schematic.document().components.is_empty() {
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
    // The technology demand row owns this failure when nothing is attached and
    // the plan demands a technology; reporting both would name two remedies.
    let technology_attached = workspace.content.project.has_audited_technology_binding();
    if (technology_attached
        || technology_demand(sim_setup, workspace.content.physical_layout_documents()).is_empty())
        && let Err(error) =
            model_library.reference_process_model_cards(sim_setup.reference_pvt.process)
    {
        return Some(error);
    }
    // The retained editor DRC result describes only the active schematic.
    // Hierarchy-wide configured-root checks are owned by the execution
    // preflight/controller pipeline, so an unrelated editor tab must not
    // block (or clear) the configured design's run eligibility.
    let configured_root_is_active = configured_root
        .key()
        .eq_ignore_ascii_case(&workspace.content.active_view.key());
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

/// Transient state for the simulation-preflight pass: the report it produced
/// and the request that starts it. Reports never survive a project or
/// application reload.
///
/// There is no visibility flag any more. The pass used to open a dialog over
/// the design to state what it found; its findings are now records in the
/// canonical diagnostic store, so what the pass leaves behind is evidence
/// rather than a window.
#[derive(Debug, Clone, Default)]
pub struct PreflightDialogState {
    pub report: Option<PreflightReport>,
    /// A caller asked for the governed preflight-and-queue pass. The frame
    /// loop consumes it on its next pass, so the request records intent
    /// without its origin naming the workflow.
    pub(in crate::workbench) run_and_queue_requested: bool,
    /// A pass whose run is still being prepared and checked, off the UI
    /// thread.
    pub(in crate::workbench) checking: Option<PreflightCheck>,
    /// The reader stopped that pass. The frame loop cancels it before it
    /// looks for the pass's verdict, so a verdict that lands in between
    /// authorizes nothing.
    pub(in crate::workbench) check_cancel_requested: bool,
}

/// What a pass decided in the frame that took the press, held until the
/// worker has prepared the run and the engine has built its decks.
#[derive(Debug, Clone)]
pub(in crate::workbench) struct PreflightCheck {
    /// The report as far as the press took it: the revisions it inspected,
    /// and its advisories. What preparing the run and building its decks
    /// finds joins it.
    pub(in crate::workbench) report: PreflightReport,
    /// Whether the run is queued once the report authorizes it: a Run press
    /// rather than a check alone.
    pub(in crate::workbench) queue: bool,
    /// The dispatch inputs as they stood at the press, which the findings
    /// are decided against. `None` when they could not be read then.
    pub(in crate::workbench) inputs_at_press: Option<crate::product::ContentDigest>,
}

impl PreflightDialogState {
    /// Drop every artifact backed by a no-longer-current execution contract.
    /// The controller's one-shot permit is invalidated by the owning
    /// `RSpiceApp` at the same mutation boundary.
    ///
    /// A pending run request deliberately survives: the workflow it asks for
    /// re-runs preflight and digest-compares the live contract before it
    /// queues anything, so dropping the request here would lose a deliberate
    /// operator action to a design edit that the workflow already refuses.
    pub fn invalidate(&mut self) {
        self.report = None;
    }

    /// Ask the frame loop to run the governed preflight-and-queue pass.
    pub fn request_run_and_queue(&mut self) {
        self.run_and_queue_requested = true;
    }

    /// Consume a pending request. Returns whether one was outstanding.
    pub fn take_run_and_queue_request(&mut self) -> bool {
        std::mem::take(&mut self.run_and_queue_requested)
    }

    /// Ask the frame loop to stop the pass whose run is being prepared or
    /// checked. Nothing when no pass is.
    pub(in crate::workbench) fn request_check_cancel(&mut self) {
        self.check_cancel_requested = self.checking.is_some();
    }

    /// Consume a pending stop. Returns whether one was outstanding.
    pub(in crate::workbench) fn take_check_cancel_request(&mut self) -> bool {
        std::mem::take(&mut self.check_cancel_requested)
    }

    /// Whether a press asked for a run that the pass in flight will queue
    /// once it authorizes one.
    pub(in crate::workbench) fn run_awaits_check(&self) -> bool {
        self.checking.as_ref().is_some_and(|check| check.queue)
    }

    /// Where the retained report stands against the live design.
    ///
    /// The verdict is derived from [`PreflightReport::is_current_for`] and
    /// nothing else, so every surface that states preflight currency states
    /// the same thing. It lives beside the report rather than in one surface
    /// because the report is not the Analyses page's private fact: a reader
    /// editing outputs, models, or solver options is changing exactly the
    /// inputs that expire it.
    pub fn currency(
        &self,
        project_revision: u64,
        topology_root: &str,
        topology_revision: u64,
        topology_closure: &[(String, u64)],
        simulation_plan: Option<(
            crate::product::SimulationPlanId,
            crate::product::ObjectRevision,
        )>,
    ) -> PreflightCurrency {
        let Some(report) = self.report.as_ref() else {
            return PreflightCurrency::Absent;
        };
        if !report.is_current_for(
            project_revision,
            topology_root,
            topology_revision,
            topology_closure,
            simulation_plan,
        ) {
            return PreflightCurrency::Expired;
        }
        match report.prepared.as_ref() {
            Some(prepared) if report.blockers.is_empty() => PreflightCurrency::Authorized {
                tasks: prepared.task_count,
            },
            _ => PreflightCurrency::Blocked {
                blockers: report.blockers.len(),
            },
        }
    }
}

/// Where a session's retained preflight report stands against the live design.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreflightCurrency {
    /// No preflight has been run in this session.
    Absent,
    /// A report exists, but the design or the active plan moved underneath it.
    Expired,
    /// A current report that still lists blocking findings.
    Blocked { blockers: usize },
    /// A current report that authorizes dispatch.
    Authorized { tasks: usize },
}

impl PreflightCurrency {
    /// Whether the retained report leaves something for the reader to do
    /// before a run can be authorized — an expired report has to be rerun, a
    /// blocked one has findings to clear and then rerun.
    pub const fn wants_rerun(self) -> bool {
        matches!(self, Self::Expired | Self::Blocked { .. })
    }

    /// The chip's leading word.
    pub const fn status(self) -> &'static str {
        match self {
            Self::Absent => "not run",
            Self::Expired => "expired",
            Self::Blocked { .. } => "blocked",
            Self::Authorized { .. } => "current",
        }
    }

    /// What the status is counted in, when it is counted in anything.
    pub fn detail(self) -> Option<String> {
        match self {
            Self::Absent => None,
            Self::Expired => Some("design or plan changed".to_owned()),
            Self::Blocked { blockers } => Some(format!(
                "{blockers} blocking finding{}",
                if blockers == 1 { "" } else { "s" }
            )),
            Self::Authorized { tasks } => Some(format!(
                "{tasks} task{} ready",
                if tasks == 1 { "" } else { "s" }
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalidating_preflight_drops_every_revision_bound_presentation_artifact() {
        let mut preflight = PreflightDialogState {
            report: Some(PreflightReport {
                project_revision: 4,
                topology_root: "user/top/schematic".to_owned(),
                topology_revision: 9,
                topology_closure: vec![("user/top/schematic".to_owned(), 9)],
                simulation_plan_id: Some(crate::product::SimulationPlanId::new()),
                simulation_plan_revision: Some(crate::product::ObjectRevision::INITIAL),
                blockers: Vec::new(),
                advisories: Vec::new(),
                prepared: None,
            }),
            ..Default::default()
        };

        preflight.invalidate();

        assert!(preflight.report.is_none());
    }
}
