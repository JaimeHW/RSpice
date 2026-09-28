//! Immutable dataset manifest.
//!
//! This is a dataset-native projection of retained run authority. It does not
//! create a visualization document, execute an analysis, or infer release
//! qualification that the run did not retain.

use egui::{Ui, WidgetInfo, WidgetType};
use rspice_results::manifest::{ManifestRow, ManifestViewModel};

use crate::state::{
    RunHistoryRevision, SavedOutputMaterializationStatus, SavedOutputReceipt, SimulationRun,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{measurement_table, section_header};
use crate::workbench::AppState;

use std::sync::Arc;

use super::frame_work::{self, DatasetWalk};
use super::virtual_rows::RowOffsets;
use super::well_hint;

const MIN_TABLE_WIDTH: f32 = 1_030.0;
const TABLE_HEAD_HEIGHT: f32 = 27.0;
const TABLE_ROW_HEIGHT: f32 = 44.0;
const COLUMN_WEIGHTS: [f32; 7] = [0.12, 0.14, 0.07, 0.17, 0.20, 0.12, 0.18];
const COLUMN_TITLES: [&str; 7] = [
    "ANALYSIS",
    "EXPANSION",
    "TASKS",
    "DOMAIN AXIS",
    "STORED VALUES",
    "PRECISION",
    "ELIGIBILITY",
];

/// Build a manifest, accounting for projection and dataset-digest work.
pub(crate) fn manifest_for_run(run: &SimulationRun) -> ManifestViewModel {
    frame_work::note(DatasetWalk::ManifestViewModel);
    frame_work::note(DatasetWalk::DatasetDigest);
    ManifestViewModel::from_run(run, |analysis| analysis.is_live_partial())
}

/// The manifest projection for one run, and the run generation it describes.
///
/// [`manifest_for_run`] validates the run's provenance, projects a
/// row per retained task and takes the dataset's content digest — a SHA-256
/// over every retained sample in the run. None of that changes while the
/// reader looks at it, and all of it happened on every frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ManifestPlan {
    source: (RunHistoryRevision, u64),
    run: u64,
    pub(crate) model: ManifestViewModel,
}

/// The active run's manifest projection, rebuilt only for a new run or a new
/// dataset generation. The retained history revision also covers restored
/// histories and nested edits that preserve the display counter.
pub(crate) fn active_manifest(state: &mut AppState) -> Option<Arc<ManifestPlan>> {
    let source = (
        state.simulation.runs.revision(),
        state.simulation.data_version,
    );
    let run_id = state.simulation.active_run()?.id;
    if let Some(plan) = state.ui.results.plans.manifest.as_ref()
        && plan.source == source
        && plan.run == run_id
    {
        return Some(Arc::clone(plan));
    }
    let model = manifest_for_run(state.simulation.active_run()?);
    let built = Arc::new(ManifestPlan {
        source,
        run: run_id,
        model,
    });
    state.ui.results.plans.manifest = Some(Arc::clone(&built));
    Some(built)
}

pub(crate) fn show(ui: &mut Ui, state: &mut AppState) {
    let Some(plan) = active_manifest(state) else {
        well_hint(ui, "No retained dataset is selected");
        return;
    };
    let manifest = &plan.model;
    let lifecycle_is_terminal = state
        .simulation
        .active_run()
        .is_some_and(|run| run.lifecycle.is_terminal());
    let t = Tokens::get(ui.ctx());

    let header = ui
        .horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(
                    egui::RichText::new(format!(
                        "{} · {}",
                        manifest.inventory_title, manifest.run_label
                    ))
                    .font(theme::sans(tokens::FS_3, FontWeight::SemiBold))
                    .color(t.color.text),
                );
                ui.label(
                    egui::RichText::new(format!(
                        "{} retained results across {} manifest tasks",
                        manifest.retained_result_count, manifest.task_count
                    ))
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(t.color.text_dim),
                );
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new(format!(
                        "{} · {}",
                        manifest.lifecycle, manifest.inventory_status
                    ))
                    .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
                    .color(if lifecycle_is_terminal {
                        t.color.ok
                    } else {
                        t.color.warn
                    }),
                );
            });
        })
        .response;
    ui.ctx().accesskit_node_builder(header.id, |node| {
        node.set_role(egui::accesskit::Role::Heading);
        node.set_label(manifest.inventory_title.clone());
    });
    ui.add_space(tokens::SP_4);

    let table_height = (ui.available_height() - 42.0).max(80.0);
    ui.allocate_ui(egui::vec2(ui.available_width(), table_height), |ui| {
        egui::ScrollArea::both()
            .id_salt("rspice.results.dataset-manifest")
            .auto_shrink([false, false])
            .show_viewport(ui, |ui, viewport| {
                let width = MIN_TABLE_WIDTH.max(ui.available_width());
                ui.set_min_width(width);
                paint_header_row(ui);
                // The inventory is one row per retained analysis task, and a
                // save-all run retains thousands.
                let offsets = RowOffsets::from_heights(std::iter::repeat_n(
                    TABLE_ROW_HEIGHT,
                    manifest.rows.len(),
                ));
                let plan = offsets.plan(egui::Rangef::new(
                    viewport.min.y - TABLE_HEAD_HEIGHT,
                    viewport.max.y - TABLE_HEAD_HEIGHT,
                ));
                ui.allocate_space(egui::vec2(width, plan.leading));
                for row in &manifest.rows[plan.range()] {
                    paint_manifest_row(ui, row);
                }
                ui.allocate_space(egui::vec2(width, plan.trailing));
                if manifest.rows.is_empty() {
                    let (rect, response) = ui.allocate_exact_size(
                        egui::vec2(ui.available_width(), TABLE_ROW_HEIGHT),
                        egui::Sense::hover(),
                    );
                    response.widget_info(|| {
                        WidgetInfo::labeled(
                            WidgetType::Label,
                            ui.is_enabled(),
                            "No analysis tasks are retained",
                        )
                    });
                    ui.painter().text(
                        rect.left_center() + egui::vec2(tokens::SP_5, 0.0),
                        egui::Align2::LEFT_CENTER,
                        "No analysis tasks are retained",
                        theme::sans(tokens::FS_1, FontWeight::Regular),
                        t.color.text_dim,
                    );
                }
            });
    });

    ui.add_space(tokens::SP_3);
    ui.label(
        egui::RichText::new(format!(
            "Bound to dataset content digest {} · this view does not execute or recompute analyses.",
            manifest.dataset_digest
        ))
        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
        .color(t.color.text_faint),
    );
}

pub(crate) fn right_panel(ui: &mut Ui, state: &mut AppState) {
    // The panel reads the same projection the sheet does rather than taking a
    // second dataset digest of the same immutable run.
    let Some(plan) = active_manifest(state) else {
        return;
    };
    let manifest = &plan.model;
    let Some((saved_outputs, run_sequence)) = state.simulation.active_run().map(|run| {
        let saved_outputs = state.simulation.active_analysis().map(|analysis| {
            (
                run.run_id,
                analysis.id,
                analysis.label.clone(),
                analysis.saved_output_receipts.clone(),
            )
        });
        (saved_outputs, run.id)
    }) else {
        return;
    };
    // How many per-task decks this session still holds for the run, which is
    // what decides whether the route below is offered at all.
    let executed_deck_points = state
        .simulation
        .executed_decks
        .get(run_sequence)
        .map_or(0, |deck| deck.points.len());
    let t = Tokens::get(ui.ctx());

    section_header(ui, "Dataset identity", None);
    let identity = [
        ("Dataset", manifest.dataset_id.as_str()),
        ("Content digest", manifest.dataset_digest.as_str()),
        ("Run", manifest.run_id.as_str()),
        ("Run sequence", manifest.run_sequence.as_str()),
        ("Lifecycle", manifest.lifecycle.as_str()),
        ("Execution target", manifest.execution_target.as_str()),
        ("Duration", manifest.elapsed_time.as_str()),
    ];
    measurement_table(ui, &identity);

    section_header(ui, "Integrity and eligibility", None);
    let task_count = manifest.task_count.to_string();
    let retained_result_count = manifest.retained_result_count.to_string();
    let integrity = [
        ("Receipt", manifest.integrity.as_str()),
        ("Qualification", manifest.qualification.as_str()),
        ("Manifest tasks", task_count.as_str()),
        ("Retained results", retained_result_count.as_str()),
    ];
    measurement_table(ui, &integrity);

    // Resolved before the table so the control that leaves this document
    // states the same refusal the dispatcher would, before the click rather
    // than after it. Read-only documents do not dispatch; the frame drains
    // the request, as it already does for the exports they raise.
    let plan_block = crate::workbench::state::plan_provenance::producing_plan_block(
        &state.simulation,
        state.sim_setup.stable_analysis_plan().ok(),
    );
    let mut open_plan = false;
    let mut open_task_deck = false;
    if let Some(authority) = &manifest.authority {
        section_header(ui, "Prepared source authority", None);
        let plan = authority
            .simulation_plan_id
            .as_deref()
            .unwrap_or("manual deck · no simulation plan");
        let source = [
            ("Source domain", authority.source_domain.as_str()),
            ("Simulation plan", plan),
            ("Project revision", authority.project_revision.as_str()),
            (
                "Prepared snapshot",
                authority.prepared_snapshot_digest.as_str(),
            ),
            ("Source content", authority.source_content_digest.as_str()),
            ("Source check", authority.source_check.as_str()),
            ("Check digest", authority.source_check_digest.as_str()),
        ];
        measurement_table(ui, &source);

        // The plan row above is an identity; this is the way back to the
        // authoring surface that owns it, with the producing instance
        // selected on arrival.
        let response = crate::ui::widgets::Button::new("Open producing plan")
            .enabled(plan_block.is_none())
            .show(ui);
        match plan_block {
            None => {
                open_plan = response
                    .on_hover_text(
                        "Open the Analyses page of the plan that produced this dataset, with the \
                         producing instance selected",
                    )
                    .clicked();
            }
            Some(reason) => {
                response.on_hover_text(reason);
            }
        }

        if !authority.model_sources.is_empty() {
            section_header(ui, "Model source digests", None);
            let rows: Vec<(&str, &str)> = authority
                .model_sources
                .iter()
                .map(|(name, digest)| (name.as_str(), digest.as_str()))
                .collect();
            measurement_table(ui, &rows);
        }
    }

    // The digests above identify the source this run was authorized over.
    // This is the way to the source itself, through the same owner the Netlist
    // run strip's own control uses, so the document reached from here is
    // byte-identical to the one reached from there. Offered for every run that
    // still holds its decks, not only for the plan-backed ones: a manual deck
    // run carries no prepared authority and its executed source is exactly as
    // worth reading.
    section_header(ui, "Executed source", None);
    let deck =
        crate::ui::widgets::Button::new(crate::workbench::commands::vocabulary::OPEN_TASK_DECK)
            .enabled(executed_deck_points > 0)
            .show(ui);
    if executed_deck_points > 0 {
        open_task_deck = deck
            .on_hover_text(format!(
                "Opens the exact source this run handed its first task, as a read-only document \
                 sealed with the run. The archive holds {executed_deck_points} of them."
            ))
            .clicked();
    } else {
        deck.on_hover_text(format!(
            "This run has no executed source to open: {}.",
            crate::state::absent_deck_reason()
        ));
    }

    if let Some((run_id, analysis_id, analysis_label, receipts)) = saved_outputs
        && !receipts.is_empty()
    {
        section_header(ui, "Saved outputs", Some(&analysis_label));
        let mut requested = None;
        for (receipt_index, receipt) in receipts.iter().enumerate() {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(&receipt.name);
                    ui.label(
                        egui::RichText::new(saved_output_status_label(receipt))
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_dim),
                    );
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if receipt.status == SavedOutputMaterializationStatus::Deferred
                        && ui.button("Materialize").clicked()
                    {
                        requested = Some(receipt_index);
                    }
                });
            });
            ui.add_space(tokens::SP_2);
        }
        if let Some(receipt_index) = requested {
            match state.simulation.materialize_deferred_saved_output(
                run_id,
                analysis_id,
                receipt_index,
            ) {
                Ok(()) => {
                    state.synchronize_specialized_viewer_cache_authority();
                    state.push_sim_message(crate::diagnostics::ConsoleMessage::info(
                        "Deferred saved output materialized from retained source data".to_owned(),
                    ));
                }
                Err(error) => state.push_sim_message(crate::diagnostics::ConsoleMessage::error(
                    format!("Saved output could not be materialized: {error}"),
                )),
            }
        }
    }
    if open_plan {
        state.ui.open_producing_plan_requested = true;
    }
    // Acted on after the panel is drawn, like the plan route above it, so the
    // workspace switch happens between frames rather than under the widget
    // that asked for it. The route answers whether the bytes are still held,
    // and a released deck is refused by name rather than opening nothing.
    if open_task_deck
        && !crate::workbench::documents::netlist_document::reveal_executed_deck(
            state,
            run_sequence,
            0,
        )
    {
        state.push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
            "The source Run {run_sequence} executed cannot be opened: {}.",
            crate::state::absent_deck_reason()
        )));
    }
}

fn saved_output_status_label(receipt: &SavedOutputReceipt) -> String {
    match &receipt.status {
        SavedOutputMaterializationStatus::Materialized { sample_count, .. } => {
            format!("materialized · {sample_count} samples")
        }
        SavedOutputMaterializationStatus::MaterializedDcFamily { members } => {
            let samples = members
                .iter()
                .map(|member| u128::from(member.sample_count))
                .sum::<u128>();
            format!(
                "materialized · {} DC members · {samples} samples",
                members.len()
            )
        }
        SavedOutputMaterializationStatus::Deferred => {
            "deferred · retained source available".to_owned()
        }
        SavedOutputMaterializationStatus::SuppressedOnSuccess => {
            "suppressed on successful analysis".to_owned()
        }
        SavedOutputMaterializationStatus::Unavailable { reason } => {
            format!("unavailable · {reason}")
        }
    }
}

fn paint_header_row(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), TABLE_HEAD_HEIGHT),
        egui::Sense::hover(),
    );
    response.widget_info(|| {
        WidgetInfo::labeled(
            WidgetType::Label,
            ui.is_enabled(),
            "Analysis manifest columns",
        )
    });
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    paint_cells(
        ui,
        rect,
        &COLUMN_TITLES,
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text_faint,
    );
}

fn paint_manifest_row(ui: &mut Ui, row: &ManifestRow) {
    let t = Tokens::get(ui.ctx());
    let cells = [
        row.analysis.as_str(),
        row.expansion.as_str(),
        row.tasks.as_str(),
        row.domain_axis.as_str(),
        row.stored_values.as_str(),
        row.precision.as_str(),
        row.eligibility.as_str(),
    ];
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), TABLE_ROW_HEIGHT),
        egui::Sense::hover(),
    );
    response.widget_info(|| {
        WidgetInfo::labeled(
            WidgetType::Label,
            ui.is_enabled(),
            format!(
                "{}; {}; {}; {}",
                row.analysis, row.domain_axis, row.stored_values, row.eligibility
            ),
        )
    });
    if row.task_identity.is_some() || row.config_digest.is_some() {
        let mut details = Vec::new();
        if let Some(identity) = &row.task_identity {
            details.push(format!("Task {identity}"));
        }
        if let Some(digest) = &row.config_digest {
            details.push(format!("Configuration digest {digest}"));
        }
        response.clone().on_hover_text(details.join("\n"));
    }
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Row);
    });
    if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, t.color.border),
    );
    paint_cells(
        ui,
        rect,
        &cells,
        theme::sans(tokens::FS_1, FontWeight::Regular),
        t.color.text,
    );
}

fn paint_cells(
    ui: &Ui,
    rect: egui::Rect,
    cells: &[&str; 7],
    font: egui::FontId,
    color: egui::Color32,
) {
    let mut left = rect.left();
    for (index, (value, weight)) in cells.iter().zip(COLUMN_WEIGHTS).enumerate() {
        let width = if index + 1 == cells.len() {
            rect.right() - left
        } else {
            rect.width() * weight
        };
        let cell = egui::Rect::from_min_size(
            egui::pos2(left, rect.top()),
            egui::vec2(width.max(1.0), rect.height()),
        );
        let text_rect = cell.shrink2(egui::vec2(tokens::SP_4, tokens::SP_2));
        let galley =
            ui.painter()
                .layout((*value).to_owned(), font.clone(), color, text_rect.width());
        ui.painter().with_clip_rect(text_rect).galley(
            egui::pos2(
                text_rect.left(),
                text_rect.center().y - galley.size().y * 0.5,
            ),
            galley,
            color,
        );
        if index + 1 < cells.len() {
            ui.painter().vline(
                cell.right() - 0.5,
                cell.y_range(),
                egui::Stroke::new(1.0, Tokens::get(ui.ctx()).color.border),
            );
        }
        left = cell.right();
    }
}

/// Serialize the dataset-native manifest rather than falling through to a
/// waveform export that has no meaningful samples for this sheet.
pub(crate) fn export_csv(run: &SimulationRun) -> super::ResultSheetCsv {
    let manifest = manifest_for_run(run);
    super::ResultSheetCsv {
        default_name: "rspice-result-manifest.csv",
        detail: format!("{} retained analyses", manifest.rows.len()),
        contents: rspice_formats::result_csv::encode_manifest_csv(&manifest),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision, SimulationPlanId};
    use crate::state::{
        AnalysisResult, AnalysisResultPayload, AnalysisResultProvenance,
        AnalysisResultSourceDomain, AnalysisType, PreparedRunReceipt, PreparedRunTaskReceipt,
        PreparedSourceCheckReceipt, SimulationRun, SimulationRunLifecycle, WaveformData,
    };

    fn digest(byte: u8) -> ContentDigest {
        ContentDigest::from_bytes([byte; 32])
    }

    #[test]
    fn legacy_manifest_is_digest_bound_and_fails_closed() {
        let mut run = SimulationRun::new(7);
        run.lifecycle = SimulationRunLifecycle::Completed;
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
                WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 2.0], "#ffbd2e"),
            ]),
        );

        let manifest = manifest_for_run(&run);

        assert_eq!(manifest.dataset_id, run.dataset_id.to_string());
        assert_eq!(
            manifest.dataset_digest,
            run.dataset_content_digest().to_string()
        );
        assert_eq!(manifest.rows.len(), 1);
        assert_eq!(manifest.rows[0].domain_axis, "adaptive time");
        assert!(manifest.rows[0].stored_values.contains("2 samples"));
        assert_eq!(
            manifest.rows[0].eligibility,
            "legacy · no prepared receipt · sign-off unavailable"
        );
        assert_eq!(
            manifest.qualification,
            "unavailable · no retained qualification authority · non-sign-off"
        );
        assert_eq!(manifest.inventory_title, "Retained analysis inventory");
    }

    #[test]
    fn active_manifest_never_claims_to_be_frozen_or_qualified() {
        let run = SimulationRun::new(8);

        let manifest = manifest_for_run(&run);

        assert_eq!(manifest.inventory_title, "Live analysis inventory");
        assert!(manifest.inventory_status.starts_with("live manifest"));
        assert_eq!(
            manifest.qualification,
            "unavailable · run is not terminal · non-sign-off"
        );
        assert!(!manifest.inventory_title.contains("Frozen"));
    }

    #[test]
    fn legacy_unknown_manifest_does_not_claim_live_or_locked_authority() {
        let mut run = SimulationRun::new(9);
        run.lifecycle = SimulationRunLifecycle::LegacyUnknown;

        let manifest = manifest_for_run(&run);

        assert_eq!(manifest.inventory_title, "Legacy analysis inventory");
        assert!(manifest.inventory_status.starts_with("legacy manifest"));
        assert_eq!(
            manifest.qualification,
            "unavailable · legacy lifecycle unknown · non-sign-off"
        );
    }

    /// A preview-engine run must not read as merely "no retained qualification"
    /// here while Verify's tile calls the same receipt eligible — and the two
    /// must say it in the same words.
    ///
    /// This line's doc said it read the receipt; what it did was rebuild the
    /// verdict from the two halves `sign_off_blocker` folds and restate them in
    /// a vocabulary of its own. So it named the *category* — "preview engine" —
    /// where the owner names the object, and a third disqualifying condition
    /// added to the receipt would have left it calling the run merely
    /// unqualified while Verify refused it.
    #[test]
    fn a_preview_engine_run_is_blocked_in_the_words_the_receipt_uses() {
        let instance_id = AnalysisInstanceId::new();
        let revision = ObjectRevision::INITIAL;
        let snapshot = digest(0x51);
        let envelope_tag = crate::state::CanonicalAnalysisKind::Envelope.tag();
        let receipt = PreparedRunReceipt::new(crate::state::PreparedRunReceiptInput {
            source_domain: AnalysisResultSourceDomain::SimulationPlan,
            simulation_plan_id: Some(SimulationPlanId::new()),
            project_revision: revision,
            prepared_snapshot_digest: snapshot,
            source_content_digest: digest(0x52),
            source_check_receipt: PreparedSourceCheckReceipt::SchematicDrc(digest(0x53)),
            project_model_sources: Vec::new(),
            specifications: Vec::new(),
            specification_policy:
                rspice_results::specification::PreparedSpecificationPolicy::default(),
            tasks: vec![
                PreparedRunTaskReceipt::new(
                    instance_id,
                    revision,
                    Vec::new(),
                    envelope_tag,
                    digest(0x54),
                )
                .expect("valid task"),
            ],
        })
        .expect("valid receipt");
        let blocker = receipt
            .sign_off_blocker()
            .expect("a preview kind blocks sign-off");
        let provenance = AnalysisResultProvenance::new(instance_id, revision, snapshot, Vec::new())
            .expect("valid provenance");
        let mut run = SimulationRun::new_prepared(11, receipt);
        run.lifecycle = SimulationRunLifecycle::Completed;
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Envelope, "Envelope").with_provenance(provenance),
        );

        let manifest = manifest_for_run(&run);

        assert_eq!(
            manifest.qualification,
            format!("blocked · {blocker} · non-sign-off")
        );
        assert!(
            manifest.qualification.contains("Envelope"),
            "the owner names the object, so this line does too: {}",
            manifest.qualification
        );
        assert_eq!(
            manifest.rows[0].eligibility,
            "retained · preview engine · non-sign-off"
        );
    }

    /// A receipt nothing disqualifies is called eligible here too.
    ///
    /// This line printed "unavailable · no retained sign-off qualification" for
    /// exactly the receipt Verify's tile stamps `Eligible`, so a reader with
    /// both surfaces open was told the same dataset may and may not be cited.
    /// Both sentences come from [`crate::state::SignOffStanding`] now, and this
    /// asserts the pair together.
    #[test]
    fn an_eligible_receipt_is_called_eligible_in_the_words_both_surfaces_use() {
        let instance_id = AnalysisInstanceId::new();
        let revision = ObjectRevision::INITIAL;
        let snapshot = digest(0x41);
        let receipt = PreparedRunReceipt::new(crate::state::PreparedRunReceiptInput {
            source_domain: AnalysisResultSourceDomain::SimulationPlan,
            simulation_plan_id: Some(SimulationPlanId::new()),
            project_revision: revision,
            prepared_snapshot_digest: snapshot,
            source_content_digest: digest(0x42),
            source_check_receipt: PreparedSourceCheckReceipt::SchematicDrc(digest(0x43)),
            project_model_sources: Vec::new(),
            specifications: Vec::new(),
            specification_policy:
                rspice_results::specification::PreparedSpecificationPolicy::default(),
            tasks: vec![
                PreparedRunTaskReceipt::new(instance_id, revision, Vec::new(), 5, digest(0x44))
                    .expect("valid task"),
            ],
        })
        .expect("valid receipt");
        let provenance = AnalysisResultProvenance::new(instance_id, revision, snapshot, Vec::new())
            .expect("valid provenance");
        let mut run = SimulationRun::new_prepared(10, receipt);
        run.lifecycle = SimulationRunLifecycle::Completed;
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "Transient")
                .with_provenance(provenance),
        );

        let standing = run
            .prepared_receipt()
            .expect("the run carries its receipt")
            .sign_off_standing();
        let manifest = manifest_for_run(&run);

        // What Verify's tile stamps, and what this cell prints, for the one
        // receipt.
        assert_eq!(standing.verdict(), "Eligible");
        assert_eq!(
            manifest.qualification,
            "eligible · every model released · every analysis production"
        );
        assert_eq!(manifest.qualification, standing.qualification());
        assert!(
            !manifest.qualification.contains("unavailable")
                && !manifest.qualification.contains("blocked"),
            "nothing disqualifies this run: {}",
            manifest.qualification
        );
        assert_eq!(
            manifest.rows[0].eligibility,
            "retained · receipt matched · sign-off unavailable"
        );
    }

    fn row_for_analysis(analysis: AnalysisResult) -> ManifestRow {
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        manifest_for_run(&run).rows.remove(0)
    }

    #[test]
    fn every_analysis_kind_has_a_truthful_domain_contract() {
        let kinds = [
            AnalysisType::DcOp,
            AnalysisType::DcSweep,
            AnalysisType::Ac,
            AnalysisType::Disto,
            AnalysisType::Transient,
            AnalysisType::Noise,
            AnalysisType::PoleZero,
            AnalysisType::Tf,
            AnalysisType::Sensitivity,
            AnalysisType::Pac,
            AnalysisType::Pnoise,
            AnalysisType::Pxf,
            AnalysisType::Pstb,
            AnalysisType::Stb,
            AnalysisType::MonteCarlo,
            AnalysisType::Parametric,
            AnalysisType::Corner,
            AnalysisType::Optimization,
            AnalysisType::Soa,
            AnalysisType::SParameter,
            AnalysisType::Envelope,
            AnalysisType::Fourier,
            AnalysisType::HarmonicBalance,
            AnalysisType::Pss,
            AnalysisType::Qpss,
            AnalysisType::Hbsp,
            AnalysisType::Hbnoise,
            AnalysisType::Psp,
            AnalysisType::Qpac,
            AnalysisType::Qpnoise,
            AnalysisType::Qpxf,
            AnalysisType::TransientNoise,
            AnalysisType::DcMismatch,
        ];
        for kind in kinds {
            let meta = row_for_analysis(AnalysisResult::new(1, kind, "domain"));
            assert!(!meta.domain_axis.is_empty(), "{kind:?}");
            assert!(!meta.precision.is_empty(), "{kind:?}");
        }
    }

    /// The Studio's axis caption and the manifest's domain name one quantity.
    ///
    /// `.PXF` used to say "Offset Frequency" on one surface and "translated
    /// frequency" on the other, which teaches a reader to trust neither. The
    /// manifest spells it in lower case; that is the only difference allowed.
    #[test]
    fn the_periodic_small_signal_family_names_one_quantity_on_both_surfaces() {
        for periodic in [
            AnalysisType::Pac,
            AnalysisType::Pxf,
            AnalysisType::Qpac,
            AnalysisType::Qpxf,
            AnalysisType::Pnoise,
            AnalysisType::Qpnoise,
            AnalysisType::Hbnoise,
        ] {
            let meta = row_for_analysis(AnalysisResult::new(1, periodic, "periodic"));
            assert_eq!(
                meta.domain_axis,
                if matches!(periodic, AnalysisType::Qpxf | AnalysisType::Qpnoise) {
                    "output frequency"
                } else {
                    "offset frequency"
                },
                "{periodic:?}"
            );
            assert_eq!(
                periodic.axis_info().0.to_ascii_lowercase(),
                meta.domain_axis,
                "the Studio caption and the manifest domain disagree for {periodic:?}"
            );
        }
        // The quantity these were named after is a different number, and
        // nothing shipped here publishes it as an abscissa. Split so this
        // test's own prose cannot trip the scan.
        let retired = ["translated ", "frequency"].concat();
        assert!(
            [
                include_str!("manifest.rs"),
                include_str!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/../rspice-results/src/manifest.rs"
                )),
            ]
            .into_iter()
            .all(|source| !crate::source_guard::production_source(source).contains(&retired)),
            "the translated frequency is `offset + n*f0`, not the swept axis"
        );
    }

    #[test]
    fn periodic_payload_manifest_uses_complete_floquet_semantics() {
        let pss = AnalysisResult::new(1, AnalysisType::Pss, "PSS").with_result_payload(
            AnalysisResultPayload::legacy_periodic_marker(AnalysisType::Pss).unwrap(),
        );
        let pstb = AnalysisResult::new(2, AnalysisType::Pstb, "PSTB").with_result_payload(
            AnalysisResultPayload::legacy_periodic_marker(AnalysisType::Pstb).unwrap(),
        );

        let pss = row_for_analysis(pss);
        let pstb = row_for_analysis(pstb);
        assert_eq!(pstb.domain_axis, "Floquet mode index");
        assert_eq!(pss.precision, "complex128");
        assert_eq!(pstb.precision, "complex128");
        assert!(
            pss.stored_values.contains("retained PSS multipliers"),
            "{}",
            pss.stored_values
        );
        assert!(
            pstb.stored_values.contains("retained PSTB modes"),
            "{}",
            pstb.stored_values
        );
    }

    fn state_with_run(label: &str) -> AppState {
        let mut run = SimulationRun::new(7);
        run.lifecycle = SimulationRunLifecycle::Completed;
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, label).with_waveforms(vec![
                WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 2.0], "#ffbd2e"),
            ]),
        );
        let mut state = AppState::default();
        state.simulation.runs = vec![run].into();
        assert!(state.simulation.select_run(0));
        state
    }

    #[test]
    fn retained_view_source_manifest_tracks_restoration_and_nested_edits() {
        let mut state = AppState::default();
        let run = state.simulation.start_run();
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
                WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 2.0], "#ffbd2e"),
            ]),
        );
        run.restore_provenance(crate::state::SimulationRunProvenance::LegacyUnattributed)
            .unwrap();
        run.mark_running().unwrap();
        run.finish_lifecycle(SimulationRunLifecycle::Completed)
            .unwrap();
        state.simulation.complete_run();
        let original = active_manifest(&mut state).unwrap();
        let version = state.simulation.data_version;
        let mut replacement = state.simulation.clone();
        replacement.runs[0].analyses[0].waveforms[0].y = Arc::new(vec![0.0, 9.0]);
        state.simulation = crate::io::simulation_state_from_results(
            crate::io::capture_simulation_results(&replacement),
        )
        .unwrap();
        assert_eq!(state.simulation.data_version, version);
        let restored = active_manifest(&mut state).unwrap();
        assert_eq!(restored.model.dataset_id, original.model.dataset_id);
        assert_ne!(restored.model.dataset_digest, original.model.dataset_digest);
        assert_eq!(
            restored.model,
            manifest_for_run(state.simulation.active_run().unwrap())
        );

        state.simulation.runs[0].analyses[0]
            .waveforms
            .push(WaveformData::new(
                "I(V1)",
                vec![0.0, 1.0],
                vec![0.0, -0.01],
                "#55aaff",
            ));
        let edited = active_manifest(&mut state).unwrap();
        assert_eq!(
            edited.model,
            manifest_for_run(state.simulation.active_run().unwrap())
        );
        assert_ne!(edited.model, restored.model);
        assert_eq!(state.simulation.data_version, version);
    }

    #[test]
    fn retained_view_source_manifest_reuses_large_unchanged_history_clones() {
        let mut state = state_with_run("Transient");
        state.simulation.runs[0].analyses[0].waveforms = vec![WaveformData::new(
            "V(out)",
            (0..100_000).map(f64::from).collect::<Vec<_>>(),
            vec![2.0; 100_000],
            "#ffbd2e",
        )];
        let original = active_manifest(&mut state).unwrap();
        let mut other = AppState::default();
        other.simulation = state.simulation.clone();
        other.ui.results = state.ui.results.clone();
        let work = frame_work::WorkCounts::reset();
        for _ in 0..12 {
            assert!(Arc::ptr_eq(
                &original,
                &active_manifest(&mut state).unwrap()
            ));
            assert!(Arc::ptr_eq(
                &original,
                &active_manifest(&mut other).unwrap()
            ));
        }
        assert_eq!(work.since().total(), 0);
        other.simulation.runs[0].analyses[0].waveforms[0].y = Arc::new(vec![3.0; 100_000]);
        assert_ne!(
            active_manifest(&mut other).unwrap().model.dataset_digest,
            original.model.dataset_digest
        );
        assert!(Arc::ptr_eq(
            &original,
            &active_manifest(&mut state).unwrap()
        ));
    }

    /// The memo has to be the same projection, digest included — that digest
    /// is what binds every statement on the sheet to the retained samples.
    #[test]
    fn the_memoized_manifest_is_the_projection_it_replaced() {
        let mut state = state_with_run("Transient");
        let direct = manifest_for_run(
            state
                .simulation
                .active_run()
                .expect("the fixture selects a run"),
        );

        let plan = active_manifest(&mut state).expect("a manifest for the active run");
        assert_eq!(plan.model, direct);
        assert_eq!(
            plan.model.dataset_digest,
            state
                .simulation
                .active_run()
                .expect("the fixture selects a run")
                .dataset_content_digest()
                .to_string()
        );

        // Asking again is the same answer from the same allocation.
        let again = active_manifest(&mut state).expect("a manifest for the active run");
        assert!(Arc::ptr_eq(&plan, &again));
    }

    /// The digest addresses the samples, so a changed dataset must produce a
    /// changed manifest rather than the one the memo happens to hold.
    #[test]
    fn a_new_dataset_generation_reprojects_the_manifest() {
        let mut state = state_with_run("Transient");
        let before = active_manifest(&mut state).expect("a manifest for the active run");
        let before_digest = before.model.dataset_digest.clone();

        state.simulation.runs[0].analyses[0].waveforms[0].y = std::sync::Arc::new(vec![0.0, 9.0]);
        state.simulation.data_version = state.simulation.data_version.wrapping_add(1);

        let after = active_manifest(&mut state).expect("a manifest for the active run");
        assert_ne!(
            after.model.dataset_digest, before_digest,
            "the sheet kept the previous dataset generation's content digest"
        );
        assert_eq!(
            after.model.dataset_digest,
            state.simulation.runs[0]
                .dataset_content_digest()
                .to_string()
        );
    }

    /// Selecting a different run is a different manifest, even at the same
    /// dataset generation.
    #[test]
    fn selecting_another_run_reprojects_the_manifest() {
        let mut state = state_with_run("First");
        let first = active_manifest(&mut state)
            .expect("a manifest for the active run")
            .model
            .clone();

        let mut second = SimulationRun::new(9);
        second.lifecycle = SimulationRunLifecycle::Completed;
        second.add_analysis(
            AnalysisResult::new(2, AnalysisType::Transient, "Second").with_waveforms(vec![
                WaveformData::new("V(out)", vec![0.0, 1.0], vec![5.0, 6.0], "#ffbd2e"),
            ]),
        );
        state.simulation.runs.push(second);
        assert!(state.simulation.select_run(1));

        let after = active_manifest(&mut state).expect("a manifest for the active run");
        assert_ne!(after.model.run_sequence, first.run_sequence);
        assert_ne!(after.model.dataset_digest, first.dataset_digest);
    }
}
