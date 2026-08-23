//! Immutable dataset manifest.
//!
//! This is a dataset-native projection of retained run authority. It does not
//! create a visualization document, execute an analysis, or infer release
//! qualification that the run did not retain.

use egui::{Ui, WidgetInfo, WidgetType};

use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisResultPayload,
    AnalysisResultSourceDomain, AnalysisType, SavedOutputMaterializationStatus, SavedOutputReceipt,
    SimulationRun, SimulationRunLifecycle,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{measurement_table, section_header};
use crate::workbench::AppState;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ManifestRow {
    pub analysis: String,
    pub expansion: String,
    pub tasks: String,
    pub domain_axis: String,
    pub stored_values: String,
    pub precision: String,
    pub eligibility: String,
    pub task_identity: Option<String>,
    pub config_digest: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ManifestAuthority {
    pub source_domain: String,
    pub simulation_plan_id: Option<String>,
    pub project_revision: String,
    pub prepared_snapshot_digest: String,
    pub source_content_digest: String,
    pub source_check: String,
    pub source_check_digest: String,
    pub model_sources: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ManifestViewModel {
    pub dataset_id: String,
    pub dataset_digest: String,
    pub run_id: String,
    pub run_sequence: String,
    pub run_label: String,
    pub lifecycle: String,
    pub execution_target: String,
    pub elapsed_time: String,
    pub inventory_title: String,
    pub inventory_status: String,
    pub integrity: String,
    pub qualification: String,
    pub task_count: usize,
    pub retained_result_count: usize,
    pub rows: Vec<ManifestRow>,
    pub authority: Option<ManifestAuthority>,
}

impl ManifestViewModel {
    #[must_use]
    pub(crate) fn from_run(run: &SimulationRun) -> Self {
        let provenance_validation = run.validate_provenance();
        let provenance_is_valid = provenance_validation.is_ok();
        let prepared = run.prepared_receipt();
        let rows = match prepared {
            Some(receipt) if provenance_validation.is_ok() => receipt
                .tasks()
                .iter()
                .enumerate()
                .map(|(index, task)| {
                    let result = run.analyses.get(index);
                    ManifestRow {
                        analysis: task.result_analysis_type().display_name().to_owned(),
                        expansion: result
                            .map_or_else(|| "not retained".to_owned(), expansion_label),
                        tasks: "1".to_owned(),
                        domain_axis: domain_meta(task.result_analysis_type()).axis.to_owned(),
                        stored_values: result
                            .map_or_else(|| "not retained".to_owned(), stored_values_label),
                        precision: result.map_or_else(
                            || {
                                domain_meta(task.result_analysis_type())
                                    .precision
                                    .to_owned()
                            },
                            precision_label,
                        ),
                        eligibility: result.map_or_else(
                            || format!("{} · non-sign-off", missing_result_status(run.lifecycle)),
                            |analysis| {
                                if analysis.is_live_partial() {
                                    "running · accepted samples · non-sign-off".to_owned()
                                } else if !analysis.success {
                                    "failed · non-sign-off".to_owned()
                                } else if task.canonical_kind().availability().blocks_sign_off() {
                                    // The tier is a property of this task's
                                    // engine, so it is stated on this row
                                    // rather than only in the run-wide
                                    // qualification line above.
                                    "retained · preview engine · non-sign-off".to_owned()
                                } else {
                                    "retained · receipt matched · sign-off unavailable".to_owned()
                                }
                            },
                        ),
                        task_identity: Some(task.instance_id().to_string()),
                        config_digest: Some(task.config_digest().to_string()),
                    }
                })
                .collect(),
            Some(receipt) => receipt
                .tasks()
                .iter()
                .map(|task| ManifestRow {
                    analysis: task.result_analysis_type().display_name().to_owned(),
                    expansion: "association withheld".to_owned(),
                    tasks: "1".to_owned(),
                    domain_axis: domain_meta(task.result_analysis_type()).axis.to_owned(),
                    stored_values: "integrity mismatch".to_owned(),
                    precision: domain_meta(task.result_analysis_type())
                        .precision
                        .to_owned(),
                    eligibility: "blocked by receipt mismatch · non-sign-off".to_owned(),
                    task_identity: Some(task.instance_id().to_string()),
                    config_digest: Some(task.config_digest().to_string()),
                })
                .collect(),
            None => run
                .analyses
                .iter()
                .map(|analysis| ManifestRow {
                    analysis: analysis.analysis_type.display_name().to_owned(),
                    expansion: expansion_label(analysis),
                    tasks: "1".to_owned(),
                    domain_axis: domain_meta(analysis.analysis_type).axis.to_owned(),
                    stored_values: stored_values_label(analysis),
                    precision: precision_label(analysis),
                    eligibility: if analysis.is_live_partial() {
                        "running · accepted samples · non-sign-off".to_owned()
                    } else if analysis.success {
                        "legacy · no prepared receipt · sign-off unavailable".to_owned()
                    } else {
                        "failed · no prepared receipt · non-sign-off".to_owned()
                    },
                    task_identity: analysis
                        .provenance()
                        .map(|provenance| provenance.source_instance_id().to_string()),
                    config_digest: None,
                })
                .collect(),
        };

        let integrity = match (&prepared, provenance_validation) {
            (Some(_), Ok(())) => "prepared receipt valid".to_owned(),
            (Some(_), Err(error)) => format!("blocked · {error}"),
            (None, Ok(())) => "legacy provenance valid".to_owned(),
            (None, Err(_)) if run.provenance().is_none() => {
                "unsealed · no authoritative provenance".to_owned()
            }
            (None, Err(error)) => format!("blocked · {error}"),
        };
        let authority = prepared.map(|receipt| {
            let source_check = if receipt.source_check_receipt().is_schematic_drc() {
                "schematic DRC"
            } else {
                "manual source check"
            };
            ManifestAuthority {
                source_domain: source_domain_label(receipt.source_domain()).to_owned(),
                simulation_plan_id: receipt.simulation_plan_id().map(|id| id.to_string()),
                project_revision: receipt.project_revision().get().to_string(),
                prepared_snapshot_digest: receipt.prepared_snapshot_digest().to_string(),
                source_content_digest: receipt.source_content_digest().to_string(),
                source_check: source_check.to_owned(),
                source_check_digest: receipt.source_check_receipt().digest().to_string(),
                model_sources: receipt
                    .project_model_sources()
                    .iter()
                    .map(|model| {
                        (
                            format!(
                                "{} · {} · revision {}",
                                model.model_name(),
                                model.source_id(),
                                model.revision().get()
                            ),
                            model.content_digest().to_string(),
                        )
                    })
                    .collect(),
            }
        });
        let task_count = prepared.map_or(run.analyses.len(), |receipt| receipt.tasks().len());

        Self {
            dataset_id: run.dataset_id.to_string(),
            dataset_digest: run.dataset_content_digest().to_string(),
            run_id: run.run_id.to_string(),
            run_sequence: run.id.to_string(),
            run_label: run.label.clone(),
            lifecycle: lifecycle_label(run.lifecycle).to_owned(),
            execution_target: run.execution_target.map_or_else(
                || "not retained".to_owned(),
                |target| target.label().to_owned(),
            ),
            elapsed_time: format!("{:.3} s", run.elapsed_time),
            inventory_title: inventory_title(run.lifecycle).to_owned(),
            inventory_status: inventory_status(run.lifecycle).to_owned(),
            integrity,
            qualification: qualification_label(run, provenance_is_valid),
            task_count,
            retained_result_count: run.analyses.len(),
            rows,
            authority,
        }
    }
}

pub(crate) fn show(ui: &mut Ui, state: &AppState) {
    let Some(run) = state.simulation.active_run() else {
        well_hint(ui, "No retained dataset is selected");
        return;
    };
    let manifest = ManifestViewModel::from_run(run);
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
                    .color(if run.lifecycle.is_terminal() {
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
    let Some((manifest, saved_outputs, run_sequence)) = state.simulation.active_run().map(|run| {
        let saved_outputs = state.simulation.active_analysis().map(|analysis| {
            (
                run.run_id,
                analysis.id,
                analysis.label.clone(),
                analysis.saved_output_receipts.clone(),
            )
        });
        (ManifestViewModel::from_run(run), saved_outputs, run.id)
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
    let deck = crate::ui::widgets::Button::new("Open task deck")
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DomainMeta {
    axis: &'static str,
    precision: &'static str,
}

const fn domain_meta(analysis: AnalysisType) -> DomainMeta {
    use AnalysisType as A;
    match analysis {
        A::DcOp => DomainMeta {
            axis: "scalar operating point",
            precision: "f64",
        },
        A::DcSweep | A::Parametric | A::DcMismatch => DomainMeta {
            axis: "swept source or parameter",
            precision: "f64",
        },
        A::Ac | A::Stb => DomainMeta {
            axis: "log frequency",
            precision: "complex128",
        },
        A::Disto => DomainMeta {
            axis: "tone product",
            precision: "complex128",
        },
        A::Transient | A::TransientNoise => DomainMeta {
            axis: "adaptive time",
            precision: "f64",
        },
        A::Noise => DomainMeta {
            axis: "log frequency",
            precision: "f64",
        },
        A::PoleZero => DomainMeta {
            axis: "complex plane",
            precision: "complex128",
        },
        A::Tf => DomainMeta {
            axis: "frequency or operating point",
            precision: "complex128",
        },
        A::Sensitivity => DomainMeta {
            axis: "parameter vector",
            precision: "f64",
        },
        A::Pac | A::Pxf | A::Pstb | A::Qpac | A::Qpxf => DomainMeta {
            axis: "translated frequency",
            precision: "complex128",
        },
        A::Pnoise | A::Qpnoise | A::Hbnoise => DomainMeta {
            axis: "offset frequency",
            precision: "f64",
        },
        A::MonteCarlo => DomainMeta {
            axis: "sample family",
            precision: "f64",
        },
        A::Corner => DomainMeta {
            axis: "PVT family",
            precision: "f64 / complex128",
        },
        A::Reliability => DomainMeta {
            axis: "mission age",
            precision: "f64",
        },
        A::Optimization => DomainMeta {
            axis: "iteration / candidate",
            precision: "f64",
        },
        A::Soa => DomainMeta {
            axis: "device / rule",
            precision: "f64",
        },
        A::SParameter | A::Hbsp | A::Psp => DomainMeta {
            axis: "frequency",
            precision: "complex128",
        },
        A::Envelope => DomainMeta {
            axis: "slow time",
            precision: "complex128",
        },
        A::Fourier => DomainMeta {
            axis: "harmonic index",
            precision: "complex128",
        },
        A::HarmonicBalance => DomainMeta {
            axis: "tone family",
            precision: "complex128",
        },
        A::Pss | A::Qpss => DomainMeta {
            axis: "periodic phase",
            precision: "f64",
        },
    }
}

fn precision_label(analysis: &AnalysisResult) -> String {
    if analysis
        .waveforms
        .iter()
        .any(|waveform| waveform.complex.is_some())
        || matches!(
            analysis.result_payload.as_ref(),
            Some(AnalysisResultPayload::PoleZero { .. })
        )
    {
        "complex128".to_owned()
    } else {
        domain_meta(analysis.analysis_type).precision.to_owned()
    }
}

fn stored_values_label(analysis: &AnalysisResult) -> String {
    let mut parts = Vec::new();
    if !analysis.waveforms.is_empty() {
        let samples: usize = analysis
            .waveforms
            .iter()
            .map(|waveform| waveform.x.len().min(waveform.y.len()))
            .sum();
        parts.push(format!(
            "{} waveform{} / {samples} samples",
            analysis.waveforms.len(),
            plural(analysis.waveforms.len())
        ));
    }
    if let Some(op) = &analysis.dc_op {
        parts.push(format!(
            "{} nodes / {} branches / {} power values",
            op.node_voltages.len(),
            op.branch_currents.len(),
            op.power_dissipation.len()
        ));
    }
    if analysis.device_op.is_some() {
        parts.push("device OP report".to_owned());
    }
    if let Some(noise) = &analysis.noise_summary {
        parts.push(format!(
            "{} noise contributor{}",
            noise.rows.len(),
            plural(noise.rows.len())
        ));
    }
    if let Some(family) = &analysis.family_metadata {
        parts.push(family_values_label(family));
    }
    if let Some(payload) = &analysis.result_payload {
        parts.push(payload_values_label(payload));
    }
    if !analysis.measurements.is_empty() {
        parts.push(format!(
            "{} measurement{}",
            analysis.measurements.len(),
            plural(analysis.measurements.len())
        ));
    }
    if !analysis.saved_output_receipts.is_empty() {
        parts.push(format!(
            "{} saved-output receipt{}",
            analysis.saved_output_receipts.len(),
            plural(analysis.saved_output_receipts.len())
        ));
    }
    if parts.is_empty() {
        if analysis.success {
            "no retained values".to_owned()
        } else {
            "failed · no retained values".to_owned()
        }
    } else {
        parts.join(" · ")
    }
}

fn family_values_label(family: &AnalysisResultFamilyMetadata) -> String {
    match family {
        AnalysisResultFamilyMetadata::Parametric { sweep_values, .. } => {
            format!("{} sweep points", sweep_values.len())
        }
        AnalysisResultFamilyMetadata::Corner { x_values, .. } => {
            format!("{} corner points", x_values.len())
        }
        AnalysisResultFamilyMetadata::MonteCarlo {
            runs_completed,
            variables,
            ..
        } => format!("{runs_completed} samples / {} variables", variables.len()),
        AnalysisResultFamilyMetadata::Reliability { years } => {
            format!("{} mission ages", years.len())
        }
        AnalysisResultFamilyMetadata::Optimization { iterations, .. } => {
            format!("{} iterations", iterations.len())
        }
        AnalysisResultFamilyMetadata::Soa { time } => {
            format!("{} SOA time points", time.len())
        }
        AnalysisResultFamilyMetadata::PeriodicNoise {
            output_quantity,
            carrier_frequency_hz,
        } => {
            let quantity = match output_quantity {
                crate::state::PeriodicNoiseOutputQuantity::OutputNoisePowerSpectralDensity => {
                    "output-noise PSD"
                }
                crate::state::PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz => {
                    "phase noise in dBc/Hz"
                }
            };
            carrier_frequency_hz.map_or_else(
                || quantity.to_owned(),
                |carrier| format!("{quantity} / {} carrier", format_frequency(carrier)),
            )
        }
        AnalysisResultFamilyMetadata::SParameter {
            reference_impedances_ohm,
        } => format!(
            "{}-port S-parameter references ({})",
            reference_impedances_ohm.len(),
            reference_impedances_ohm
                .iter()
                .map(|value| format!("{value} ohm"))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

/// Serialize the dataset-native manifest rather than falling through to a
/// waveform export that has no meaningful samples for this sheet.
pub(crate) fn export_csv(run: &SimulationRun) -> super::ResultSheetCsv {
    let manifest = ManifestViewModel::from_run(run);
    let mut contents = String::from(
        "section,field,value,analysis,expansion,tasks,domain_axis,stored_values,precision,eligibility,task_identity,config_digest\n",
    );
    {
        let mut metadata = |section: &str, field: &str, value: &str| {
            contents.push_str(&format!(
                "{},{},{},,,,,,,,,\n",
                super::csv_field(section),
                super::csv_field(field),
                super::csv_field(value)
            ));
        };
        for (field, value) in [
            ("dataset_id", manifest.dataset_id.as_str()),
            ("dataset_digest", manifest.dataset_digest.as_str()),
            ("run_id", manifest.run_id.as_str()),
            ("run_sequence", manifest.run_sequence.as_str()),
            ("run_label", manifest.run_label.as_str()),
            ("lifecycle", manifest.lifecycle.as_str()),
            ("execution_target", manifest.execution_target.as_str()),
            ("elapsed_time", manifest.elapsed_time.as_str()),
            ("inventory_title", manifest.inventory_title.as_str()),
            ("inventory_status", manifest.inventory_status.as_str()),
            ("integrity", manifest.integrity.as_str()),
            ("qualification", manifest.qualification.as_str()),
        ] {
            metadata("dataset", field, value);
        }
        metadata("dataset", "task_count", &manifest.task_count.to_string());
        metadata(
            "dataset",
            "retained_result_count",
            &manifest.retained_result_count.to_string(),
        );
    }
    for row in &manifest.rows {
        contents.push_str(&format!(
            "inventory,,,{},{},{},{},{},{},{},{},{}\n",
            super::csv_field(&row.analysis),
            super::csv_field(&row.expansion),
            super::csv_field(&row.tasks),
            super::csv_field(&row.domain_axis),
            super::csv_field(&row.stored_values),
            super::csv_field(&row.precision),
            super::csv_field(&row.eligibility),
            super::csv_field(row.task_identity.as_deref().unwrap_or_default()),
            super::csv_field(row.config_digest.as_deref().unwrap_or_default()),
        ));
    }

    if let Some(authority) = &manifest.authority {
        let mut authority_field = |field: &str, value: &str| {
            contents.push_str(&format!(
                "authority,{},{},,,,,,,,,\n",
                super::csv_field(field),
                super::csv_field(value)
            ));
        };
        for (field, value) in [
            ("source_domain", authority.source_domain.as_str()),
            (
                "simulation_plan_id",
                authority.simulation_plan_id.as_deref().unwrap_or_default(),
            ),
            ("project_revision", authority.project_revision.as_str()),
            (
                "prepared_snapshot_digest",
                authority.prepared_snapshot_digest.as_str(),
            ),
            (
                "source_content_digest",
                authority.source_content_digest.as_str(),
            ),
            ("source_check", authority.source_check.as_str()),
            (
                "source_check_digest",
                authority.source_check_digest.as_str(),
            ),
        ] {
            authority_field(field, value);
        }
        for (name, digest) in &authority.model_sources {
            authority_field(&format!("model_source:{name}"), digest);
        }
    }

    super::ResultSheetCsv {
        default_name: "rspice-result-manifest.csv",
        detail: format!("{} retained analyses", manifest.rows.len()),
        contents,
    }
}

fn format_frequency(value: f64) -> String {
    if value >= 1.0e9 {
        format!("{:.6} GHz", value / 1.0e9)
    } else if value >= 1.0e6 {
        format!("{:.6} MHz", value / 1.0e6)
    } else if value >= 1.0e3 {
        format!("{:.6} kHz", value / 1.0e3)
    } else {
        format!("{value:.6} Hz")
    }
}

fn payload_values_label(payload: &AnalysisResultPayload) -> String {
    match payload {
        AnalysisResultPayload::OperatingPoint {
            mna_node_names,
            mna_branch_names,
            ..
        } => format!(
            "{} MNA nodes / {} MNA branches",
            mna_node_names.len(),
            mna_branch_names.len()
        ),
        AnalysisResultPayload::PoleZero { poles, zeros, .. } => {
            format!("{} poles / {} zeros", poles.len(), zeros.len())
        }
        AnalysisResultPayload::Sensitivity { rows, .. } => {
            format!("{} sensitivities", rows.len())
        }
        AnalysisResultPayload::ScalarMeasurements { values } => {
            format!("{} scalar values", values.len())
        }
        AnalysisResultPayload::TransferFunction { .. } => "transfer / impedance scalars".to_owned(),
        AnalysisResultPayload::Reliability { devices } => {
            format!("{} reliability devices", devices.len())
        }
        AnalysisResultPayload::Soa {
            evaluations,
            violations,
        } => format!(
            "{} SOA evaluations / {} violations",
            evaluations.len(),
            violations.len()
        ),
        AnalysisResultPayload::TransientEvents {
            digital_traces,
            real_traces,
        } => {
            let events: usize = digital_traces
                .iter()
                .map(|trace| trace.points.len())
                .chain(real_traces.iter().map(|trace| trace.points.len()))
                .sum();
            format!(
                "{} event nodes / {events} committed events",
                digital_traces.len() + real_traces.len()
            )
        }
    }
}

fn expansion_label(analysis: &AnalysisResult) -> String {
    analysis.provenance().map_or_else(
        || "legacy result".to_owned(),
        |provenance| {
            if provenance.authored_source_instance_id() == provenance.source_instance_id() {
                "single task".to_owned()
            } else {
                "materialized PVT point".to_owned()
            }
        },
    )
}

const fn missing_result_status(lifecycle: SimulationRunLifecycle) -> &'static str {
    match lifecycle {
        SimulationRunLifecycle::Preparing
        | SimulationRunLifecycle::Running
        | SimulationRunLifecycle::Cancelling => "pending · not yet retained",
        SimulationRunLifecycle::Failed
        | SimulationRunLifecycle::Aborted
        | SimulationRunLifecycle::Interrupted => "not produced",
        SimulationRunLifecycle::Completed | SimulationRunLifecycle::LegacyUnknown => "not retained",
    }
}

/// The run-wide qualification line.
///
/// A valid receipt never *grants* sign-off — nothing here retains a sign-off
/// record — so the terminal case still reports it unavailable. What it also
/// does is name the blocker the receipt carries, read from the one owner
/// [`crate::state::PreparedRunReceipt::sign_off_blocker`].
///
/// Its doc said that before this. What the code did was rebuild the verdict out
/// of `unqualified_model_sources()` and `preview_engine_kinds()` — the two
/// halves that owner is a fold of — and restate them in a vocabulary of its
/// own, so a third disqualifying condition added to the receipt would leave
/// this line calling the run merely unqualified while Verify's tile refused it,
/// and the blocker named the objects while this named only their category.
fn qualification_label(run: &SimulationRun, provenance_is_valid: bool) -> String {
    match run.lifecycle {
        SimulationRunLifecycle::LegacyUnknown => {
            "unavailable · legacy lifecycle unknown · non-sign-off".to_owned()
        }
        SimulationRunLifecycle::Preparing
        | SimulationRunLifecycle::Running
        | SimulationRunLifecycle::Cancelling => {
            "unavailable · run is not terminal · non-sign-off".to_owned()
        }
        SimulationRunLifecycle::Completed
        | SimulationRunLifecycle::Failed
        | SimulationRunLifecycle::Aborted
        | SimulationRunLifecycle::Interrupted => {
            let Some(receipt) = run.prepared_receipt() else {
                return "unavailable · no retained qualification authority · non-sign-off"
                    .to_owned();
            };
            if !provenance_is_valid {
                return "blocked · receipt integrity mismatch · non-sign-off".to_owned();
            }
            receipt.sign_off_blocker().map_or_else(
                || "unavailable · no retained sign-off qualification".to_owned(),
                |blocker| format!("blocked · {blocker} · non-sign-off"),
            )
        }
    }
}

const fn inventory_title(lifecycle: SimulationRunLifecycle) -> &'static str {
    match lifecycle {
        SimulationRunLifecycle::LegacyUnknown => "Legacy analysis inventory",
        SimulationRunLifecycle::Preparing
        | SimulationRunLifecycle::Running
        | SimulationRunLifecycle::Cancelling => "Live analysis inventory",
        SimulationRunLifecycle::Completed
        | SimulationRunLifecycle::Failed
        | SimulationRunLifecycle::Aborted
        | SimulationRunLifecycle::Interrupted => "Retained analysis inventory",
    }
}

const fn inventory_status(lifecycle: SimulationRunLifecycle) -> &'static str {
    match lifecycle {
        SimulationRunLifecycle::LegacyUnknown => {
            "legacy manifest · mutability authority unavailable"
        }
        SimulationRunLifecycle::Preparing
        | SimulationRunLifecycle::Running
        | SimulationRunLifecycle::Cancelling => "live manifest · digest changes until terminal",
        SimulationRunLifecycle::Completed
        | SimulationRunLifecycle::Failed
        | SimulationRunLifecycle::Aborted
        | SimulationRunLifecycle::Interrupted => "locked manifest",
    }
}

const fn lifecycle_label(lifecycle: SimulationRunLifecycle) -> &'static str {
    match lifecycle {
        SimulationRunLifecycle::LegacyUnknown => "legacy status unknown",
        SimulationRunLifecycle::Preparing => "preparing",
        SimulationRunLifecycle::Running => "running",
        SimulationRunLifecycle::Cancelling => "cancelling",
        SimulationRunLifecycle::Completed => "completed",
        SimulationRunLifecycle::Failed => "failed",
        SimulationRunLifecycle::Aborted => "aborted",
        SimulationRunLifecycle::Interrupted => "interrupted",
    }
}

const fn source_domain_label(domain: AnalysisResultSourceDomain) -> &'static str {
    match domain {
        AnalysisResultSourceDomain::SimulationPlan => "simulation plan",
        AnalysisResultSourceDomain::ManualDeck => "manual deck",
        AnalysisResultSourceDomain::LegacyUnclassified => "legacy unclassified",
    }
}

const fn plural(count: usize) -> &'static str {
    if count == 1 { "" } else { "s" }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision, SimulationPlanId};
    use crate::state::{
        AnalysisResult, AnalysisResultProvenance, AnalysisResultSourceDomain, PreparedRunReceipt,
        PreparedRunTaskReceipt, PreparedSourceCheckReceipt, SimulationRun, WaveformData,
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

        let manifest = ManifestViewModel::from_run(&run);

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

        let manifest = ManifestViewModel::from_run(&run);

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

        let manifest = ManifestViewModel::from_run(&run);

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
        let receipt = PreparedRunReceipt::new(
            AnalysisResultSourceDomain::SimulationPlan,
            Some(SimulationPlanId::new()),
            revision,
            snapshot,
            digest(0x52),
            PreparedSourceCheckReceipt::SchematicDrc(digest(0x53)),
            vec![
                PreparedRunTaskReceipt::new(
                    instance_id,
                    revision,
                    Vec::new(),
                    envelope_tag,
                    digest(0x54),
                )
                .expect("valid task"),
            ],
        )
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

        let manifest = ManifestViewModel::from_run(&run);

        assert_eq!(manifest.qualification, format!("blocked · {blocker} · non-sign-off"));
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

    #[test]
    fn valid_prepared_receipt_does_not_invent_sign_off_qualification() {
        let instance_id = AnalysisInstanceId::new();
        let revision = ObjectRevision::INITIAL;
        let snapshot = digest(0x41);
        let receipt = PreparedRunReceipt::new(
            AnalysisResultSourceDomain::SimulationPlan,
            Some(SimulationPlanId::new()),
            revision,
            snapshot,
            digest(0x42),
            PreparedSourceCheckReceipt::SchematicDrc(digest(0x43)),
            vec![
                PreparedRunTaskReceipt::new(instance_id, revision, Vec::new(), 5, digest(0x44))
                    .expect("valid task"),
            ],
        )
        .expect("valid receipt");
        let provenance = AnalysisResultProvenance::new(instance_id, revision, snapshot, Vec::new())
            .expect("valid provenance");
        let mut run = SimulationRun::new_prepared(10, receipt);
        run.lifecycle = SimulationRunLifecycle::Completed;
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "Transient")
                .with_provenance(provenance),
        );

        let manifest = ManifestViewModel::from_run(&run);

        assert_eq!(
            manifest.qualification,
            "unavailable · no retained sign-off qualification"
        );
        assert_eq!(
            manifest.rows[0].eligibility,
            "retained · receipt matched · sign-off unavailable"
        );
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
            AnalysisType::Reliability,
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
            let meta = domain_meta(kind);
            assert!(!meta.axis.is_empty(), "{kind:?}");
            assert!(!meta.precision.is_empty(), "{kind:?}");
        }
    }
}
