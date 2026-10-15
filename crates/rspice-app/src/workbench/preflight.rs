//! Revision-bound simulation-preflight workflow.
//!
//! Validation is executed once and captured as an immutable report for the
//! exact project, topology and plan revision it inspected. The report is the
//! authority a run is queued against; nothing here draws it.
//!
//! It used to draw it, in a modal over the design. That meant the one list of
//! reasons a run would not start was readable only where the schematic was
//! not, and acting on any of them meant dismissing the list first. The
//! findings are now records in the canonical diagnostic store, where they sit
//! beside every other condition that stands against the design, and where
//! each one's own destination is what following the row does. What this module
//! still owns is the pass itself: run the checks, prepare the immutable
//! snapshot, and queue only what that snapshot authorizes.

mod topology;

use crate::diagnostics::ConsoleMessage;
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;

use super::state::{
    ConsolePage, ModelsPage, PreflightAdvisory, PreflightIssue, PreflightRemediation,
    PreflightReport, PreparedPreflightContract, Workspace,
};

const TECHNOLOGY_NOT_REQUIRED_ADVISORY: &str = "Technology: not attached — not required by this plan (built-in device models, TT reference process)";

/// Run every local preflight check, retain the exact report for the current
/// project revision, and publish what it found.
///
/// A blocked pass leaves the reader on the Problems page with the blockers
/// listed; a clean one leaves them where they were. Either way the outcome is
/// stated on the status line, because "I pressed the button and nothing
/// visible happened" is the failure mode a command with no dialog has to
/// answer for.
pub(crate) fn run(app: &mut RSpiceApp) {
    app.state.sync_active_schematic_to_workspace();
    crate::workbench::menu_bar::run_design_rule_check(&mut app.state);
    let mut report = collect_report(&app.state);
    if report.blockers.is_empty() {
        match app
            .simulation_controller
            .prepare_run_set_for_preflight(&app.state)
        {
            Ok(metadata) => {
                report
                    .advisories
                    .extend(metadata.advisories.into_iter().map(PreflightAdvisory::from));
                report.prepared = Some(PreparedPreflightContract {
                    snapshot_digest: metadata.snapshot_digest,
                    source_digest: metadata.source_digest,
                    receipt_digest: metadata.receipt_digest,
                    receipt_label: metadata.receipt_label,
                    analysis_ids: metadata.analysis_ids,
                    task_count: metadata.task_count,
                    saved_output_contract_count: metadata.saved_output_contract_count,
                    pvt_point_count: metadata.pvt_point_count,
                    target: metadata.target,
                    save_policy: metadata.save_policy,
                    model_identity_count: metadata.model_identity_count,
                });
            }
            Err(error) => report.blockers.push(PreflightIssue {
                check: preparation_check_label(error.stage()).to_owned(),
                observed: error.message().to_owned(),
                required: preparation_requirement(error.stage()).to_owned(),
                remediation: preparation_remediation(error.stage(), error.line()),
            }),
        }
    }
    if report.prepared.is_none() {
        app.simulation_controller.clear_prepared_run();
    }
    let current_plan = app.state.active_plan_revision();
    let (topology_root, topology_revision, topology_closure) =
        app.state.configured_topology_revision();
    let blocked = !report.is_runnable_for(
        app.state.workspace.content.project.revision().get(),
        &topology_root,
        topology_revision,
        &topology_closure,
        current_plan,
    );
    let message = if blocked {
        format!(
            "Preflight blocked · {} blocking issue{} · design revision {} was not queued",
            report.blockers.len(),
            if report.blockers.len() == 1 { "" } else { "s" },
            report.project_revision
        )
    } else {
        let task_count = report
            .prepared
            .as_ref()
            .map_or(0, |prepared| prepared.task_count);
        format!(
            "Preflight complete · 0 blocking errors · {} advisor{} · design revision {} · {} task{}",
            report.advisories.len(),
            if report.advisories.len() == 1 {
                "y"
            } else {
                "ies"
            },
            report.project_revision,
            task_count,
            if task_count == 1 { "" } else { "s" }
        )
    };

    app.state.publish_dispatch_diagnostics(&report);
    app.state.workbench.preflight.report = Some(report);
    app.state.record_plan_receipt(message);

    if blocked {
        app.state.workbench.console_visible = true;
        app.state.workbench.console_page = ConsolePage::Problems;
    }
}

/// Run the same governed preflight used by the explicit Preflight command and
/// queue only the exact immutable snapshot it authorizes. A retained current
/// report is reusable; the controller still rebuilds and digest-compares the
/// live contract when it consumes the one-shot permit.
pub(crate) fn run_and_queue(app: &mut RSpiceApp) {
    if app.state.simulation.execution.has_active_execution()
        || app.state.simulation.execution.trigger_simulation
    {
        app.state.push_user_message(ConsoleMessage::warning(
            "A simulation is already running or waiting to start".to_owned(),
        ));
        return;
    }

    if queue_retained_run(app) {
        return;
    }

    run(app);
    let _ = queue_retained_run(app);
}

/// Queue a retained authorized report only while every revision it names is
/// still current.
fn queue_retained_run(app: &mut RSpiceApp) -> bool {
    let project_revision = app.state.workspace.content.project.revision().get();
    let (topology_root, topology_revision, topology_closure) =
        app.state.configured_topology_revision();
    let current_plan = app.state.active_plan_revision();
    let runnable = app
        .state
        .workbench
        .preflight
        .report
        .as_ref()
        .is_some_and(|report| {
            report.is_runnable_for(
                project_revision,
                &topology_root,
                topology_revision,
                &topology_closure,
                current_plan,
            )
        });
    if !runnable {
        return false;
    }

    app.state.request_run_set_simulation();
    app.state.workbench.activate(Workspace::Simulate);
    true
}

fn collect_report(state: &AppState) -> PreflightReport {
    let mut blockers = Vec::new();
    let mut advisories = Vec::new();
    let root_reference = state.workspace.content.simulation_root_reference();
    let inspection = state.workspace.inspect_design_projection(
        &state.library_manager,
        &state.workspace.content.active_view,
        &state.schematic,
    );
    let execution_projection = inspection
        .clone()
        .and_then(|projection| projection.into_execution());

    match &execution_projection {
        Err(error) => blockers.push(PreflightIssue {
            check: "Design topology".to_owned(),
            observed: error.to_string(),
            required: "An existing, fully resolved executable configuration root".to_owned(),
            remediation: PreflightRemediation::DesignChecks,
        }),
        Ok(projection)
            if projection
                .root_schematic()
                .is_some_and(|root| root.document().components.is_empty()) =>
        {
            blockers.push(PreflightIssue {
                check: "Design topology".to_owned(),
                observed: format!(
                    "Configured simulation root '{}' contains no components.",
                    root_reference.display_path()
                ),
                required: "A non-empty circuit topology".to_owned(),
                remediation: PreflightRemediation::DesignChecks,
            });
        }
        Ok(_) => {}
    }

    let hierarchy_bindings = inspection.as_ref().map_or(&[][..], |projection| {
        projection.hierarchy_resolution().bindings.as_slice()
    });
    for binding in hierarchy_bindings
        .iter()
        .filter(|binding| !binding.status.is_resolved())
    {
        let instance_scope = if binding.instance_count == 1 {
            String::new()
        } else {
            format!(" · {} instances", binding.instance_count)
        };
        let diagnostic = binding.diagnostic.clone().unwrap_or_else(|| {
            format!(
                "{}/{} is {}",
                binding.reference.library,
                binding.reference.cell,
                binding.status.label()
            )
        });
        blockers.push(PreflightIssue {
            check: "Hierarchy binding".to_owned(),
            observed: format!("{diagnostic}{instance_scope}"),
            required: "A finite executable master for every hierarchical instance".to_owned(),
            remediation: PreflightRemediation::DesignChecks,
        });
    }
    for binding in hierarchy_bindings
        .iter()
        .filter(|binding| binding.used_review_fallback)
    {
        advisories.push(PreflightAdvisory::from(format!(
            "Configuration fallback reviewed for {} at {}",
            binding.reference.display_path(),
            binding.instance_paths.join(", ")
        )));
    }
    // A warning is a resolved binding the configuration honours differently
    // than it reads. It blocks nothing — the run will proceed — but starting
    // one without saying so leaves the author reading a policy the executed
    // hierarchy does not follow.
    for binding in hierarchy_bindings {
        for warning in &binding.warnings {
            advisories.push(PreflightAdvisory::from(format!(
                "Configuration warning for {}: {warning}",
                binding.reference.display_path()
            )));
        }
    }

    if let Ok(execution_projection) = &execution_projection {
        let root_schematic = execution_projection
            .root_schematic()
            .expect("a successful execution projection has a materialized root");
        let hierarchy_source = rspice_design::hierarchy::HierarchySource::from_execution_projection(
            state.library_manager.catalog(),
            execution_projection,
        );
        let result = rspice_design::drc::run_check_with_hierarchy(
            root_schematic,
            &hierarchy_source,
            rspice_design::drc::DrcConfig {
                check_missing_ground: true,
                ..rspice_design::drc::DrcConfig::default()
            },
        );
        if result.completed {
            for violation in result.errors() {
                blockers.push(PreflightIssue {
                    check: "Schematic validation".to_owned(),
                    observed: format!("{} · {}", violation.message, violation.location.display()),
                    required: violation.violation_type.suggested_fix().to_owned(),
                    remediation: PreflightRemediation::DesignChecks,
                });
            }
            for violation in result.warnings() {
                advisories.push(PreflightAdvisory::from(format!(
                    "{} · {}",
                    violation.message,
                    violation.location.display()
                )));
            }
        } else {
            blockers.push(PreflightIssue {
                check: "Schematic validation".to_owned(),
                observed: "The configured-root schematic validator did not complete.".to_owned(),
                required: "A completed validation result for this topology revision".to_owned(),
                remediation: PreflightRemediation::DesignChecks,
            });
        }

        // What the engine itself refuses to start on. Reaching elaboration to
        // learn that a node has no DC path costs the author a queued run and
        // an engine message that names deck nodes; deciding it here names the
        // drawing's own nodes and instances before anything is dispatched.
        blockers.extend(topology::topology_blockers(
            root_schematic,
            &rspice_design::connectivity::extract_with_hierarchy(
                root_schematic,
                Some(&hierarchy_source),
            ),
        ));
    }

    match state.sim_setup.stable_analysis_plan() {
        Err(error) => blockers.push(PreflightIssue {
            check: "Analysis-instance plan".to_owned(),
            observed: error,
            required: "A migrated, structurally valid simulation plan".to_owned(),
            remediation: PreflightRemediation::SimulationPlan,
        }),
        Ok(plan) => {
            for issue in plan.validation_issues() {
                blockers.push(PreflightIssue {
                    check: "Analysis-instance graph".to_owned(),
                    observed: issue.to_string(),
                    required: "Stable enabled identities with resolved, ordered prerequisites"
                        .to_owned(),
                    remediation: PreflightRemediation::SimulationPlan,
                });
            }
            for instance in plan
                .instances()
                .iter()
                .filter(|instance| instance.enabled())
            {
                if let Some(error) = state
                    .sim_setup
                    .analysis_draft_validation_error(instance.draft())
                {
                    blockers.push(PreflightIssue {
                        check: format!("{} configuration", instance.kind().label()),
                        observed: format!("{}: {error}", instance.id()),
                        required: "A complete, numerically valid analysis configuration".to_owned(),
                        remediation: PreflightRemediation::SimulationPlan,
                    });
                }
            }
        }
    }

    // A project need not have a technology; if it has one it must be valid,
    // and if the plan needs one it must have one. Each demanding entity is
    // reported as its own row so the operator sees what to change.
    let demand = state.technology_demand();
    match state.workspace.content.project.technology_binding() {
        None if demand.is_empty() => {
            advisories.push(PreflightAdvisory::from(
                TECHNOLOGY_NOT_REQUIRED_ADVISORY.to_owned(),
            ));
        }
        None => blockers.extend(demand.reasons().iter().map(|reason| PreflightIssue {
            check: "Project technology contract".to_owned(),
            observed: reason.observed(),
            required: reason.required(),
            remediation: PreflightRemediation::ProjectTechnology,
        })),
        Some(binding) => {
            if state
                .workspace
                .content
                .project
                .technology_change_audit()
                .is_empty()
            {
                blockers.push(PreflightIssue {
                    check: "Project technology contract".to_owned(),
                    observed:
                        "The attached technology binding predates checkpoint-backed authority receipts"
                            .to_owned(),
                    required: "Reattach the technology to record an audited change receipt"
                        .to_owned(),
                    remediation: PreflightRemediation::ProjectTechnology,
                });
            }
            if let Err(error) = state
                .model_library_manager
                .catalog()
                .validate_attached_technology(Some(binding))
            {
                blockers.push(PreflightIssue {
                    check: "Project model technology".to_owned(),
                    observed: error,
                    required:
                        "The exact attached model-source closure must resolve without catalog drift"
                            .to_owned(),
                    remediation: PreflightRemediation::ProjectTechnology,
                });
            }
            if let Err(error) = state
                .pdk_config
                .technology_registry
                .validate_project_binding(binding)
            {
                blockers.push(PreflightIssue {
                    check: "Project signed PDK".to_owned(),
                    observed: error.to_string(),
                    required:
                        "The exact attached package revision must remain signature-trusted and compatible with this runtime"
                            .to_owned(),
                    remediation: PreflightRemediation::ProjectTechnology,
                });
            }
        }
    }

    // An unmet demand already owns the missing reference section and carries the
    // only remediation that resolves it; a second row would contradict it.
    if (state.project_technology_in_effect() || demand.is_empty())
        && let Err(error) = state
            .model_library_manager
            .reference_process_model_cards(state.sim_setup.reference_pvt.process)
    {
        let process = state.sim_setup.reference_pvt.process;
        // The check is asked of the whole closure and its answer is a sentence
        // about the closure, so the destination is resolved from what the
        // failure *names* — the process — rather than parsed back out of that
        // sentence. A process declared by exactly one library has an
        // unambiguous home and the route lands on it with the corner matrix
        // inspected; anything else keeps the page destination and says which
        // library the reader has to pick, because Corners & sections renders
        // from a selection and landing there with none is landing nowhere.
        let providers = state
            .model_library_manager
            .libraries_declaring_process(process);
        let (required, remediation) = match providers.as_slice() {
            [library] => (
                format!("A resolved {} model section", process.short_name()),
                PreflightRemediation::model_corner(*library, None),
            ),
            _ => (
                format!(
                    "A resolved {} model section \u{00b7} select the library that declares {} on \
                     Corners & sections",
                    process.short_name(),
                    process.short_name()
                ),
                PreflightRemediation::models_page(ModelsPage::Corners),
            ),
        };
        blockers.push(PreflightIssue {
            check: "Reference model binding".to_owned(),
            observed: error,
            required,
            remediation,
        });
    }

    advisories.extend(temperature_validity_advisories(state));

    let simulation_plan = state.active_plan_revision();
    let (topology_root, topology_revision, topology_closure) = state.configured_topology_revision();
    PreflightReport {
        project_revision: state.workspace.content.project.revision().get(),
        topology_root,
        topology_revision,
        topology_closure,
        simulation_plan_id: simulation_plan.map(|(id, _)| id),
        simulation_plan_revision: simulation_plan.map(|(_, revision)| revision),
        blockers,
        advisories,
        prepared: None,
    }
}

/// Corners the run set asks to run outside the range the PDK qualified them for.
///
/// An advisory rather than a blocker, deliberately. A run at 150 °C against a
/// corner qualified to 125 °C is a run the foundry does not vouch for — it is
/// not a run the tool can prove wrong, and refusing it would be the tool
/// substituting its judgement for the engineer's. What the tool owes is the
/// sentence, once, before the run rather than after it.
///
/// Only the corner each library has *active* is checked here: that is the one
/// a run expands. Every corner the project is authoring is checked on the
/// Corners page, where authoring one is what a reader is doing.
fn temperature_validity_advisories(state: &AppState) -> Vec<PreflightAdvisory> {
    let requested = state.sim_setup.requested_temperatures_celsius();
    state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .filter_map(|library| {
            let active = library.selected_corner.as_deref()?;
            let corner = library
                .corners
                .values()
                .find(|corner| corner.name.eq_ignore_ascii_case(active))?;
            let outside = corner.temperatures_outside_qualified_range(&requested);
            (!outside.is_empty()).then(|| PreflightAdvisory {
                message: format!(
                    "'{}' is active in '{}' and qualified {}; this run set requests {}",
                    corner.name,
                    library.name,
                    corner.qualified_range_label(),
                    crate::state::model_library::stated_temperatures(&outside)
                ),
                // The two identities the sentence above is built from travel
                // with the destination, so Corners & sections opens on this
                // library's matrix with this corner inspected.
                remediation: Some(PreflightRemediation::model_corner(
                    library.name.clone(),
                    Some(corner.name.clone()),
                )),
            })
        })
        .collect()
}

fn preparation_check_label(stage: crate::simulation::execution::PreparationStage) -> &'static str {
    use crate::simulation::execution::PreparationStage;
    match stage {
        PreparationStage::DesignChecks => "Schematic validation",
        PreparationStage::SourceChecks => "Source closure",
        PreparationStage::AnalysisPlan => "Run set",
        PreparationStage::ModelBindings => "Model bindings",
        PreparationStage::Netlist => "Executable netlist",
        PreparationStage::Authorization => "Dispatch authorization",
    }
}

fn preparation_requirement(stage: crate::simulation::execution::PreparationStage) -> &'static str {
    use crate::simulation::execution::PreparationStage;
    match stage {
        PreparationStage::DesignChecks => "A current completed source-check receipt",
        PreparationStage::SourceChecks => "A closed, parseable source dependency set",
        PreparationStage::AnalysisPlan => "A complete executable analysis configuration",
        PreparationStage::ModelBindings => "Authenticated materialized model sources",
        PreparationStage::Netlist => "A deterministic self-contained executable netlist",
        PreparationStage::Authorization => "A one-use generation-safe dispatch authorization",
    }
}

/// Where a preparation failure is repaired, and — for the netlist stage —
/// exactly where in the deck.
///
/// `line` comes from the failure itself rather than from its wording. A
/// stage that reports no line yields a destination with none, so the row
/// that offers it can tell a located finding from an unlocated one instead
/// of promising a jump it cannot make.
fn preparation_remediation(
    stage: crate::simulation::execution::PreparationStage,
    line: Option<usize>,
) -> PreflightRemediation {
    use crate::simulation::execution::PreparationStage;
    match stage {
        PreparationStage::AnalysisPlan | PreparationStage::Authorization => {
            PreflightRemediation::SimulationPlan
        }
        // A model-binding failure is repaired where the bindings live, not by
        // re-running the source checks that reported it.
        PreparationStage::ModelBindings => PreflightRemediation::models_page(ModelsPage::Corners),
        // A netlist-stage failure is a defect in the deck the run would
        // execute, so it opens the deck. Running the design checks again is
        // what this used to do, and those checks had already passed — that is
        // how the stage got as far as generating a netlist.
        PreparationStage::Netlist => PreflightRemediation::NetlistSource { line },
        PreparationStage::DesignChecks | PreparationStage::SourceChecks => {
            PreflightRemediation::DesignChecks
        }
    }
}

#[cfg(test)]
mod tests;
