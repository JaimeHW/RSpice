//! The retained dataset's authority record, and the routes out of it.
//!
//! Two of them exist, and both are the same shape: the record is rendered
//! from a borrow of the run it describes while every route mutates the
//! session. So this reports what a reader asked for and the caller acts once
//! that borrow is done, rather than taking the whole application to save a
//! line at the call site.

use egui::Ui;

use super::super::super::design_system::property_row;
use super::{executed_deck, inspector_disclosure, section_header};

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
    manifest: &crate::workbench::documents::result_document::manifest::ManifestViewModel,
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
        let route = crate::ui::widgets::Button::new("Open producing plan")
            .enabled(plan_block.is_none())
            .show(ui);
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
