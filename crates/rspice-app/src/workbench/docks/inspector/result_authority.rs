//! The retained dataset's authority record, and the routes out of it.
//!
//! Two of them exist, and both are the same shape: the record is rendered
//! from a borrow of the run it describes while every route mutates the
//! session. So this reports what a reader asked for and the caller acts once
//! that borrow is done, rather than taking the whole application to save a
//! line at the call site.

use egui::Ui;

use super::super::super::design_system::{
    property_row, property_row_height, property_row_toned, property_row_wrapped,
};
use super::{
    executed_deck, inspector_disclosure, inspector_inset, muted_inspector_copy, section_header,
};
use crate::ui::tokens::Tokens;

/// A virtualized list of property rows, for a record that runs to more rows
/// than a section should show at once.
fn scrolled_rows(ui: &mut Ui, count: usize, mut row: impl FnMut(&mut Ui, usize)) {
    let row_height = property_row_height(ui);
    egui::ScrollArea::vertical()
        .max_height(160.0)
        .auto_shrink([false, true])
        .show_rows(ui, row_height, count, |ui, range| {
            for index in range {
                row(ui, index);
            }
        });
}

pub(super) fn result_dc_sweep(ui: &mut Ui, analysis: &crate::state::AnalysisResult) {
    use crate::state::{AnalysisResultPayload, AnalysisType, DcCurveSelection, DcSweepFamily};
    if analysis.analysis_type != AnalysisType::DcSweep || !analysis.success {
        return;
    }
    section_header(ui, "DC sweep", None);
    let Some(AnalysisResultPayload::DcSweep { evidence }) = &analysis.result_payload else {
        property_row(ui, "Curve identity", "not recorded");
        property_row(ui, "Traversal", "not recorded");
        return;
    };
    property_row(ui, "Primary source", &evidence.source);
    property_row(ui, "Primary traversal", evidence.direction.label());
    property_row(
        ui,
        "Solved quantities",
        &evidence.quantities.len().to_string(),
    );
    if let DcCurveSelection::Saved(curves) = &evidence.selection {
        property_row(ui, "Retained source curves", &curves.len().to_string());
    }
    match &evidence.family {
        DcSweepFamily::Single => {
            property_row(ui, "Sweep family", "Single");
        }
        DcSweepFamily::Retraced => {
            property_row(ui, "Sweep family", "Forward and reverse continuation");
        }
        DcSweepFamily::Nested { source, values } => {
            property_row(ui, "Secondary source", source);
            property_row(ui, "Secondary points", &values.len().to_string());
            if let Some(last) = values.last() {
                property_row(ui, "Final secondary value", &format!("{last:.16e}"));
            }
            if inspector_disclosure(
                ui,
                "dc-secondary-points",
                "Secondary coordinates",
                "executed order",
            ) {
                scrolled_rows(ui, values.len(), |ui, index| {
                    property_row(
                        ui,
                        &format!("Point {}", index + 1),
                        &format!("{:.16e}", values[index]),
                    );
                });
            }
        }
    }
    muted_inspector_copy(
        ui,
        "Curves use ascending display axes; traversal records the order actually solved.",
    );
}

/// Solver evidence belongs to the selected result, including after project reload.
pub(super) fn result_convergence(ui: &mut Ui, analysis: &crate::state::AnalysisResult) {
    use crate::state::{AnalysisType, PeriodicInitializationMethod};
    let Some(quality) = analysis.convergence.as_deref() else {
        if matches!(
            analysis.analysis_type,
            AnalysisType::Transient
                | AnalysisType::TransientNoise
                | AnalysisType::Envelope
                | AnalysisType::Fourier
                | AnalysisType::Soa
        ) {
            section_header(ui, "Transient convergence", None);
            property_row(ui, "Solver evidence", "not retained");
        }
        return;
    };
    section_header(ui, "Transient convergence", None);
    let forced = forced_lte_acceptances(quality);
    if forced == 0 {
        property_row(ui, "LTE acceptances", "none forced");
    } else {
        property_row_toned(
            ui,
            "LTE acceptances",
            &format!("{forced} forced"),
            Tokens::get(ui.ctx()).color.warn,
        );
    }
    if !inspector_disclosure(ui, "result-convergence", "Convergence details", "recorded") {
        return;
    }
    muted_inspector_copy(
        ui,
        "LTE is local truncation error. A forced acceptance passed Newton convergence but \
         exceeded the LTE criterion. These statistics do not establish overall numerical \
         accuracy.",
    );
    convergence_report(ui, &quality.transient, "Source transient");
    if let Some(initialization) = &quality.initialization {
        let phase = match initialization.method {
            PeriodicInitializationMethod::Shooting => "Shooting initialization",
            PeriodicInitializationMethod::HarmonicBalance => "Harmonic-balance initialization",
        };
        section_header(ui, phase, None);
        let iterations_label = match initialization.method {
            PeriodicInitializationMethod::Shooting => "Shooting corrections",
            PeriodicInitializationMethod::HarmonicBalance => "HB Newton iterations",
        };
        property_row(
            ui,
            iterations_label,
            &initialization.solver_iterations.to_string(),
        );
        property_row(
            ui,
            "Final periodic residual",
            &format!("{:.6e}", initialization.final_residual),
        );
        muted_inspector_copy(
            ui,
            "The counters below cover recorded helper solves during initialization; they are \
             separate from the periodic solver's iteration count.",
        );
        convergence_report(ui, &initialization.report, "Initialization helper solves");
    }
}

/// Every point any phase of the run accepted past its LTE criterion: the
/// source transient's and, for a periodic run, its initialization's.
fn forced_lte_acceptances(quality: &crate::state::TransientConvergenceEvidence) -> u64 {
    quality.transient.force_accepted_points
        + quality.initialization.as_ref().map_or(0, |initialization| {
            initialization.report.force_accepted_points
        })
}

fn convergence_report(ui: &mut Ui, report: &crate::state::ConvergenceReport, phase: &str) {
    section_header(ui, phase, None);
    for (name, count) in [
        ("Newton iterations", report.total_iterations),
        ("Gmin stepping", report.gmin_stepping_count),
        ("Source stepping", report.source_stepping_count),
        ("Forced LTE points", report.force_accepted_points),
        ("Step reductions", report.timestep_reductions),
        ("LTE rejections", report.lte_rejections),
        ("Device bypasses", report.bypassed_device_evaluations),
    ] {
        property_row(ui, name, &count.to_string());
    }
    property_row(
        ui,
        "Maximum residual",
        &format!("{:.6e}", report.max_residual),
    );
    property_row(
        ui,
        "Iterations / solve",
        &format!("{:.6}", report.avg_iterations_per_solve),
    );
    if let Some(basis) = &report.time_basis {
        property_row(ui, "Source samples", &basis.sample_count.to_string());
        property_row(ui, "Source start", &format!("{:.16e} s", basis.start_s));
        property_row(ui, "Source stop", &format!("{:.16e} s", basis.stop_s));
        muted_inspector_copy(
            ui,
            "Affected times refer to the original solver trajectory, before cropping, envelope \
             extraction or Fourier analysis.",
        );
    } else {
        property_row(
            ui,
            "Source times",
            "Not available for the internal initialization trajectory",
        );
    }
    let forced = &report.force_accepted_indices;
    if !forced.is_empty()
        && inspector_disclosure(
            ui,
            &format!("forced-lte-locations-{phase}"),
            "Forced LTE locations",
            &forced.len().to_string(),
        )
    {
        // Each location is a row: the source sample it was forced at, and
        // when that sample was, where the trajectory kept a time basis.
        ui.push_id(phase, |ui| {
            scrolled_rows(ui, forced.len(), |ui, position| {
                let index = forced[position];
                let time = report.time_basis.as_ref().map_or_else(
                    || "zero based · time not retained".to_owned(),
                    |basis| format!("{:.16e} s", basis.force_accepted_times_s[position]),
                );
                property_row(ui, &format!("Source index {index}"), &time);
            });
        });
    }
    if let Some(diagnostic) = &report.failure_diagnostic {
        muted_inspector_copy(
            ui,
            "An attempted solve left this diagnostic; a later convergence aid may have \
             recovered it.",
        );
        property_row_wrapped(ui, "Diagnostic", &diagnostic.summary());
        if !diagnostic.failure_message.is_empty() {
            property_row_wrapped(ui, "Solver message", &diagnostic.failure_message);
        }
    }
}

/// What a reader asked the authority record to do next.
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct AuthorityRoutes {
    /// The run whose executed deck should be revealed.
    pub(super) reveal_executed_deck: Option<u64>,
    /// Whether the reader asked to go back to the plan that produced this
    /// dataset.
    pub(super) open_producing_plan: bool,
}

pub(super) const RESULT_QUALIFICATION_GAPS: [(&str, &str); 5] = [
    ("Qualification receipt", "not retained"),
    ("Requirements mapping", "not retained"),
    ("Release gates", "not assessed"),
    ("Sign-off eligibility", "not assessed"),
    ("Approval authority", "not retained"),
];

/// The receipt half of the results inspector, and the routes out of it.
///
/// It reports what was asked for rather than doing it: the record is rendered
/// from a borrow of the run itself, and both routes mutate the session. The
/// caller acts once that borrow is done.
///
/// `plan_block` is the refusal the producing-plan route would give, resolved
/// by the caller under the same borrow so the control and the rows above it
/// speak about one dataset.
pub(super) fn result_dataset_authority(
    ui: &mut Ui,
    run: &crate::state::SimulationRun,
    manifest: &rspice_results::manifest::ManifestViewModel,
    executed: Option<&[String]>,
    plan_block: Option<&'static str>,
) -> AuthorityRoutes {
    let successful_results = run
        .analyses
        .iter()
        .filter(|analysis| analysis.success)
        .count();

    // Identity leads with what a reader uses to tell one dataset from
    // another; the identifiers that only prove it are provenance, and fold
    // away with the rest of the authority record below.
    section_header(ui, "Dataset identity", Some("current"));
    property_row(ui, "Dataset", &manifest.run_label);
    property_row(ui, "Run sequence", &manifest.run_sequence);
    property_row(ui, "Lifecycle", &manifest.lifecycle);
    property_row(ui, "Duration", &manifest.elapsed_time);
    property_row(ui, "Execution target", &manifest.execution_target);

    if !inspector_disclosure(ui, "result-provenance", "Run provenance", "immutable") {
        return AuthorityRoutes::default();
    }
    let mut open_producing_plan = false;

    property_row(ui, "Dataset ID", &manifest.dataset_id);
    property_row(ui, "Dataset digest", &manifest.dataset_digest);
    property_row(ui, "Run ID", &manifest.run_id);
    property_row(
        ui,
        "Job ID",
        &run.job_id
            .map_or_else(|| "not retained".to_owned(), |id| id.to_string()),
    );

    section_header(ui, "Retained inventory", None);
    property_row(
        ui,
        "Task receipts",
        &manifest.authority.as_ref().map_or_else(
            || "not retained".to_owned(),
            |_| manifest.task_count.to_string(),
        ),
    );
    property_row(
        ui,
        "Retained results",
        &manifest.retained_result_count.to_string(),
    );
    property_row(
        ui,
        "Successful results",
        &format!("{successful_results} / {}", manifest.retained_result_count),
    );
    property_row(ui, "Receipt integrity", &manifest.integrity);

    section_header(ui, "Prepared source authority", None);
    if let Some(authority) = &manifest.authority {
        property_row(ui, "Source domain", &authority.source_domain);
        property_row(
            ui,
            "Simulation plan",
            authority
                .simulation_plan_id
                .as_deref()
                .unwrap_or("manual deck · no simulation plan"),
        );
        property_row(ui, "Project revision", &authority.project_revision);
        property_row(
            ui,
            "Prepared input digest",
            &authority.prepared_snapshot_digest,
        );
        property_row(
            ui,
            "Source content digest",
            &authority.source_content_digest,
        );
        property_row(ui, "Source check", &authority.source_check);
        property_row(ui, "Source-check digest", &authority.source_check_digest);
        // The plan row above is an identity a receipt is checked against;
        // this is the route back to the surface that owns it. Refusals are
        // stated on the disabled control, in the same words the dispatcher
        // would have used, so the reason is visible before the click.
        let route = inspector_inset(ui, |ui| {
            crate::ui::widgets::Button::new("Open producing plan")
                .min_width(ui.available_width())
                .enabled(plan_block.is_none())
                .show(ui)
        });
        match plan_block {
            None => {
                open_producing_plan = route
                    .on_hover_text(
                        "Open the Analyses page of the plan that produced this dataset, with the \
                         producing instance selected",
                    )
                    .clicked();
            }
            Some(reason) => {
                route.on_hover_text(reason);
            }
        }

        section_header(ui, "Model source digests", None);
        if authority.model_sources.is_empty() {
            property_row(ui, "Model identities", "not retained");
        } else {
            for (identity, digest) in &authority.model_sources {
                property_row(ui, identity, digest);
            }
        }
    } else {
        for label in [
            "Source domain",
            "Simulation plan",
            "Project revision",
            "Prepared input digest",
            "Source content digest",
            "Source check",
            "Source-check digest",
        ] {
            property_row(ui, label, "not retained");
        }
        section_header(ui, "Model source digests", None);
        property_row(ui, "Model identities", "not retained");
    }

    // The receipt names project-owned model definitions by digest. What the
    // deck itself was sealed under — a pack release, a built-in, a retained
    // import — is a different and coarser fact, and the only one that covers
    // sources the receipt does not admit at all.
    let reveal = executed_deck::record(ui, run.id, executed);
    result_qualification_gaps(ui);
    AuthorityRoutes {
        reveal_executed_deck: reveal,
        open_producing_plan,
    }
}

pub(super) fn result_qualification_gaps(ui: &mut Ui) {
    section_header(ui, "Qualification and release", None);
    for (label, value) in RESULT_QUALIFICATION_GAPS {
        property_row(ui, label, value);
    }
}
