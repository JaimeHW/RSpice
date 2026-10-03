//! Dataset manifest tables, provenance readouts and explicit host actions.

use crate::{presentation::well_hint, virtual_rows::RowOffsets};
use egui::{Ui, WidgetInfo, WidgetType};
use rspice_app_types::product::RunId;
use rspice_results::{
    executed_deck::absent_deck_reason,
    manifest::{ManifestRow, ManifestViewModel},
    saved_output::{SavedOutputMaterializationStatus, SavedOutputReceipt},
};
use rspice_ui_kit::{
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::{measurement_table, section_header},
};
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

pub struct SavedOutputs<'a> {
    pub run_id: RunId,
    pub analysis_id: u64,
    pub analysis_label: &'a str,
    pub receipts: &'a [SavedOutputReceipt],
}
pub struct ManifestPanel<'a> {
    pub manifest: &'a ManifestViewModel,
    pub saved_outputs: Option<SavedOutputs<'a>>,
    pub executed_deck_points: usize,
    pub plan_block: Option<&'a str>,
    pub open_task_deck_label: &'a str,
}
pub struct MaterializeRequest {
    pub run_id: RunId,
    pub analysis_id: u64,
    pub receipt_index: usize,
}
#[derive(Default)]
pub struct ManifestResponse {
    pub materialize: Option<MaterializeRequest>,
    pub open_plan: bool,
    pub open_task_deck: bool,
}
pub fn show(ui: &mut Ui, manifest: Option<&ManifestViewModel>, lifecycle_is_terminal: bool) {
    let Some(manifest) = manifest else {
        well_hint(ui, "No retained dataset is selected");
        return;
    };
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
pub fn right_panel(ui: &mut Ui, source: &ManifestPanel<'_>) -> ManifestResponse {
    let manifest = source.manifest;
    let executed_deck_points = source.executed_deck_points;
    let plan_block = source.plan_block;
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
    let mut materialize = None;
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
        let response = rspice_ui_kit::widgets::Button::new("Open producing plan")
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
    let deck = rspice_ui_kit::widgets::Button::new(source.open_task_deck_label)
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
            absent_deck_reason()
        ));
    }

    if let Some(SavedOutputs {
        run_id,
        analysis_id,
        analysis_label,
        receipts,
    }) = &source.saved_outputs
        && !receipts.is_empty()
    {
        section_header(ui, "Saved outputs", Some(analysis_label));
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
            materialize = Some(MaterializeRequest {
                run_id: *run_id,
                analysis_id: *analysis_id,
                receipt_index,
            });
        }
    }
    ManifestResponse {
        materialize,
        open_plan,
        open_task_deck,
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
