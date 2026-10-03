//! Source trace, cursor, measurement, annotation, and export dock controls.

use super::super::widgets::{dock_intro, empty_note};
use egui::{RichText, Ui};
use rspice_app_types::product::{DatasetId, short_identity as short_dataset};
use rspice_ui_kit::{
    panels::property_row,
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::Button,
};

#[derive(Clone, Copy)]
pub struct TraceBinding {
    pub dataset_id: Option<DatasetId>,
    pub analysis_id: Option<u64>,
    pub exists: bool,
}
pub struct TraceRows<'a> {
    pub visibility: &'a mut [(String, bool)],
    pub labels: Vec<String>,
}
pub trait TraceHost {
    fn binding(&self) -> TraceBinding;
    fn rows(&mut self) -> TraceRows<'_>;
    fn open_measurement(&mut self);
    fn apply(&mut self, binding: TraceBinding);
}
pub trait CursorHost {
    fn canonical(&self) -> bool;
    fn linked(&self) -> bool;
    fn set_linked(&mut self, canonical: bool, linked: bool);
    fn positions(&self) -> (Option<f64>, Option<f64>);
    fn place_cursor(&mut self);
    fn place_marker(&mut self);
    fn clear_cursors(&mut self, canonical: bool);
    fn clear_markers(&mut self);
}
pub trait MeasurementHost {
    fn expression(&mut self) -> &mut String;
    fn evaluate(&self, definition: &str) -> Result<(DatasetId, u64, f64), String>;
    fn create(&mut self, measurement: (DatasetId, u64, f64), expression: String);
}
pub struct AnnotationAnchor {
    pub dataset_id: DatasetId,
    pub analysis_sequence: u64,
    pub waveform_name: String,
    pub sample_index: usize,
    pub x: f64,
}
pub trait AnnotationHost {
    fn text(&mut self) -> &mut String;
    fn anchor(&self) -> Option<AnnotationAnchor>;
    fn create(&mut self, anchor: AnnotationAnchor, text: String);
}
pub trait ExportHost {
    fn exact_available(&self) -> bool;
    fn figure_available(&self) -> bool;
    fn request_data(&mut self);
    fn request_figure(&mut self);
}

pub fn traces(ui: &mut Ui, host: &mut impl TraceHost) -> bool {
    dock_intro(
        ui,
        "RESULTS · SIGNALS · EXPRESSIONS",
        "Show or hide native source traces and create a derived expression for the selected analysis.",
    );
    let binding = host.binding();
    let rows = host.rows();
    if rows.visibility.is_empty() {
        empty_note(ui, "No active analysis exposes traces.");
    } else {
        for ((_, visible), label) in rows.visibility.iter_mut().zip(rows.labels) {
            ui.checkbox(visible, label);
        }
    }
    ui.add_space(8.0);
    if Button::new("Add expression…").show(ui).clicked() {
        host.open_measurement();
        return false;
    }
    let apply = ui
        .add_enabled(binding.exists, egui::Button::new("Apply trace changes"))
        .on_disabled_hover_text(
            "The immutable analysis bound when this dialog opened is unavailable",
        )
        .clicked();
    if apply {
        host.apply(binding);
    }
    apply
}

pub fn cursors(ui: &mut Ui, host: &mut impl CursorHost) -> bool {
    dock_intro(
        ui,
        "RESULTS · EXACT VALUES",
        "Manage linked cursors, source-sample markers, and exact-value behavior.",
    );
    let canonical = host.canonical();
    let mut linked_cursors = host.linked();
    let link_response = ui.checkbox(
        &mut linked_cursors,
        "Link A/B cursors across compatible panes",
    );
    if link_response.changed() {
        host.set_linked(canonical, linked_cursors);
    }
    let (cursor_a, cursor_b) = host.positions();
    property_row(
        ui,
        "Cursor A",
        &cursor_a.map_or_else(|| "not placed".to_owned(), |x| format!("{x:.17e}")),
    );
    property_row(
        ui,
        "Cursor B",
        &cursor_b.map_or_else(|| "not placed".to_owned(), |x| format!("{x:.17e}")),
    );
    ui.horizontal_wrapped(|ui| {
        if Button::new("Place next at midpoint").show(ui).clicked() {
            host.place_cursor();
        }
        if Button::new("Add exact marker").show(ui).clicked() {
            host.place_marker();
        }
        if Button::new("Clear cursors").show(ui).clicked() {
            host.clear_cursors(canonical);
        }
        if Button::new("Clear markers").show(ui).clicked() {
            host.clear_markers();
        }
    });
    Button::new("Done").show(ui).clicked()
}

pub fn measurement(ui: &mut Ui, host: &mut impl MeasurementHost) -> bool {
    dock_intro(
        ui,
        "RESULTS · SCALAR EXPRESSION",
        "Evaluate and retain a finite scalar measurement against the exact selected analysis.",
    );
    ui.label("Expression");
    ui.text_edit_singleline(host.expression());
    property_row(
        ui,
        "Scope",
        "Current pane · active analysis · immutable source inputs",
    );
    let definition = host.expression().trim().to_owned();
    let evaluation = host.evaluate(&definition);
    match &evaluation {
        Ok((_, _, value)) => {
            property_row(ui, "Validated value", &format!("{value:.17e}"));
        }
        Err(error) if !definition.is_empty() => {
            ui.label(
                RichText::new(error)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(Tokens::get(ui.ctx()).color.err),
            );
        }
        Err(_) => {
            property_row(
                ui,
                "Validation",
                "Enter a scalar expression such as rms(V(out))",
            );
        }
    }
    let valid = evaluation.is_ok();
    let add = ui
        .add_enabled(valid, egui::Button::new("Create measurement"))
        .clicked();
    if add {
        host.create(
            evaluation.expect("enabled measurement has a validated scalar result"),
            definition,
        );
    }
    add
}

pub fn annotation(ui: &mut Ui, host: &mut impl AnnotationHost) -> bool {
    dock_intro(
        ui,
        "RESULTS · REVIEW ANCHOR",
        "Anchor a review note to an immutable dataset and exact source coordinate.",
    );
    ui.label("Annotation text");
    ui.text_edit_multiline(host.text());
    let anchor = host.anchor();
    property_row(
        ui,
        "Anchor",
        &anchor.as_ref().map_or_else(
            || "No exact source row".to_owned(),
            |AnnotationAnchor {
                 dataset_id: dataset,
                 analysis_sequence: analysis,
                 waveform_name: waveform,
                 sample_index: index,
                 x,
             }| {
                format!(
                    "{} · analysis {} · {}[{}] · {:.17e}",
                    short_dataset(*dataset),
                    analysis,
                    waveform,
                    index,
                    x
                )
            },
        ),
    );
    let valid = anchor.is_some() && !host.text().trim().is_empty();
    let add = ui
        .add_enabled(valid, egui::Button::new("Create annotation"))
        .clicked();
    if add {
        let anchor = anchor.expect("enabled annotation has an exact anchor");
        let text = host.text().trim().to_owned();
        host.create(anchor, text);
    }
    add
}

pub fn export(ui: &mut Ui, host: &mut impl ExportHost) -> bool {
    dock_intro(
        ui,
        "RESULTS · EXACT DATA OR RENDERED VIEW",
        "Choose a writer backed by the active immutable result dataset or viewer crop.",
    );
    let exact_enabled = host.exact_available();
    let figure_enabled = host.figure_available();
    let mut close = false;
    if ui
        .add_enabled(
            exact_enabled,
            egui::Button::new("Export exact engineering data…"),
        )
        .clicked()
    {
        host.request_data();
        close = true;
    }
    if ui
        .add_enabled(
            figure_enabled,
            egui::Button::new("Export active viewer figure…"),
        )
        .clicked()
    {
        host.request_figure();
        close = true;
    }
    if !exact_enabled {
        empty_note(
            ui,
            "A completed immutable result is required before export.",
        );
    } else if !figure_enabled {
        empty_note(
            ui,
            "Exact data export is available. This viewer has no semantic figure writer yet.",
        );
    }
    close
}
