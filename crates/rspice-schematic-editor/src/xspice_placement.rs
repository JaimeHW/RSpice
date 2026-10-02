//! Built-in XSPICE vector-width form over canonical port metadata and a local draft.

use egui::{DragValue, Grid, RichText, Ui};
use rspice_design::schematic::device_catalog::CatalogXspiceVectorPort;
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};
use std::collections::BTreeMap;

/// Current validation takes priority; an edit dismisses a retained submission error.
pub fn show(
    ui: &mut Ui,
    ports: &[CatalogXspiceVectorPort],
    widths: &mut BTreeMap<String, usize>,
    validation_error: Option<&str>,
    retained_error: Option<&str>,
) -> (Option<egui::Id>, bool) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        RichText::new("Vector interface")
            .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
            .color(t.color.text),
    );
    ui.add_space(6.0);

    let mut first = None;
    let mut edited = false;
    Grid::new("builtin-xspice-vector-widths")
        .num_columns(3)
        .spacing(egui::vec2(12.0, 8.0))
        .show(ui, |ui| {
            ui.strong("Port");
            ui.strong("Width");
            ui.strong("Executable range");
            ui.end_row();
            for port in ports {
                ui.label(&port.name);
                let width = widths
                    .entry(port.name.clone())
                    .or_insert(port.default_width);
                let maximum = port.maximum.unwrap_or(usize::MAX);
                let fixed = port.minimum == maximum;
                let response = ui.add_enabled(
                    !fixed,
                    DragValue::new(width)
                        .range(port.minimum..=maximum)
                        .speed(1.0),
                );
                first.get_or_insert(response.id);
                edited |= response.changed();
                let range = if fixed {
                    format!("fixed at {}", port.minimum)
                } else if port.null_allowed && port.minimum == 0 {
                    format!("0–{maximum} · 0 omits/nulls the port")
                } else {
                    format!("{}–{maximum}", port.minimum)
                };
                ui.label(RichText::new(range).color(t.color.text_dim));
                ui.end_row();
            }
        });

    ui.add_space(10.0);
    ui.label(
        RichText::new(
            "Every materialized vector element (and each side of a differential element) remains a distinct connectable schematic terminal.",
        )
        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
        .color(t.color.text_dim),
    );
    if let Some(error) = validation_error.or(if edited { None } else { retained_error }) {
        ui.add_space(6.0);
        ui.label(
            RichText::new(error)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.err),
        );
    }
    (first, edited)
}
