//! The dataset manifest sheet: one row per retained analysis.

use crate::{presentation::well_hint, virtual_rows::RowOffsets};
use egui::{Ui, WidgetInfo, WidgetType};
use rspice_results::manifest::{ManifestRow, ManifestViewModel};
use rspice_ui_kit::{
    theme::{self, FontWeight},
    tokens::{self, Tokens},
};
const MIN_TABLE_WIDTH: f32 = 1_030.0;
const TABLE_HEAD_HEIGHT: f32 = 27.0;
const TABLE_ROW_HEIGHT: f32 = 44.0;
const COLUMN_WEIGHTS: [f32; 7] = [0.12, 0.14, 0.07, 0.17, 0.20, 0.12, 0.18];
const COLUMN_TITLES: [&str; 7] = [
    "Analysis",
    "Expansion",
    "Tasks",
    "Domain axis",
    "Stored values",
    "Precision",
    "Eligibility",
];

pub fn show(ui: &mut Ui, manifest: Option<&ManifestViewModel>) {
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
                    .color(t.color.text_dim),
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
