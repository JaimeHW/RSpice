//! Studio section tables and controls over source summaries and immediate application requests.

use super::{
    chrome::engineering_count,
    dock::VisualizationDock,
    widgets::{
        concept_banner, labeled_combo, numeric_policy, policy_row, section_heading, section_scroll,
        table_header,
    },
};
use egui::{Grid, Ui, vec2};
use rspice_app_types::product::{DatasetId, short_identity as short_dataset};
use rspice_results::{
    result_presentation::ExprTrace,
    studio_presentation::{
        ComplexProjection, DisplayLodPolicy, VisualizationAnnotation, VisualizationAutoscale,
        VisualizationMarker, VisualizationMeasurement, VisualizationPane, VisualizationSection,
    },
};
use rspice_ui_kit::{panels::property_row, widgets::Button};

pub enum AxisSource<'a> {
    MissingAnalysis,
    NoVisibleWaveform,
    Waveform {
        name: &'a str,
        x_range: (f64, f64),
        y_range: (f64, f64),
        frequency: bool,
        complex: bool,
    },
}

pub struct FamilyRow {
    pub dataset_id: DatasetId,
    pub label: String,
    pub analyses: usize,
    pub samples: usize,
    pub active: bool,
    pub overlaid: bool,
}

pub struct MeasurementsView<'a> {
    pub measurements: &'a [VisualizationMeasurement],
    pub expressions: &'a [ExprTrace],
    pub cursor_a: bool,
    pub cursor_b: bool,
    pub linked_cursors: bool,
    pub markers: &'a [VisualizationMarker],
    pub annotations: &'a [VisualizationAnnotation],
}

#[derive(Clone, Copy)]
pub enum SectionAction {
    OpenDock(VisualizationDock),
    ToggleOverlay(DatasetId),
    Fit,
    ApplyLod,
    ExportData,
    ExportFigure,
}

pub trait SectionHost {
    fn panes(&self) -> &[VisualizationPane];
    fn axes(&self) -> AxisSource<'_>;
    fn autoscale(&mut self) -> &mut VisualizationAutoscale;
    fn autoscale_available(&self, value: VisualizationAutoscale) -> bool;
    fn complex_projection(&mut self) -> &mut ComplexProjection;
    fn fit_block_reason(&self) -> Option<&'static str>;
    fn families(&self) -> Vec<FamilyRow>;
    fn measurements(&self) -> MeasurementsView<'_>;
    fn display_lod(&mut self) -> &mut DisplayLodPolicy;
    fn tile_memory_mib(&mut self) -> &mut u32;
    fn exact_export_available(&self) -> bool;
    fn figure_export_available(&self) -> bool;
    fn request(&mut self, action: SectionAction);
}

pub fn document_section(ui: &mut Ui, host: &mut impl SectionHost) {
    section_heading(ui, VisualizationSection::Document);
    section_scroll(ui, "visualization.document", |ui| {
        Grid::new("visualization.document.table")
            .num_columns(6)
            .striped(true)
            .spacing(vec2(18.0, 7.0))
            .show(ui, |ui| {
                for label in [
                    "Pane",
                    "Viewer",
                    "Dataset",
                    "X link",
                    "Cursor group",
                    "Page",
                ] {
                    table_header(ui, label);
                }
                ui.end_row();
                if host.panes().is_empty() {
                    ui.label("—");
                    ui.label("No panes in this result document");
                    for _ in 0..4 {
                        ui.label("—");
                    }
                    ui.end_row();
                }
                for pane in host.panes() {
                    ui.monospace(format!("{:02}", pane.id));
                    ui.label(pane.viewer.label());
                    ui.monospace(short_dataset(pane.dataset_id));
                    ui.monospace(
                        pane.x_link
                            .map_or_else(|| "none".to_owned(), |id| format!("x-{id}")),
                    );
                    ui.monospace(
                        pane.cursor_group
                            .map_or_else(|| "none".to_owned(), |id| format!("cursor-{id}")),
                    );
                    ui.label(&pane.page);
                    ui.end_row();
                }
            });
        ui.add_space(10.0);
        ui.horizontal_wrapped(|ui| {
            dock_action(ui, host, "Add pane…", VisualizationDock::AddPane);
            dock_action(ui, host, "Reorder panes…", VisualizationDock::ReorderPanes);
            dock_action(ui, host, "Link groups…", VisualizationDock::LinkGroups);
            dock_action(ui, host, "Page editor…", VisualizationDock::PageEditor);
        });
    });
}

pub fn axes_section(ui: &mut Ui, host: &mut impl SectionHost) {
    section_heading(ui, VisualizationSection::Axes);
    section_scroll(ui, "visualization.axes", |ui| {
        Grid::new("visualization.axes.table")
            .num_columns(6)
            .striped(true)
            .spacing(vec2(18.0, 7.0))
            .show(ui, |ui| {
                for label in ["Axis", "Quantity", "Transform", "Range", "Ticks", "Unit"] {
                    table_header(ui, label);
                }
                ui.end_row();
                let source = host.axes();
                if matches!(source, AxisSource::MissingAnalysis) {
                    ui.label("—");
                    ui.label("No active result dataset");
                    for _ in 0..4 {
                        ui.label("—");
                    }
                    ui.end_row();
                    return;
                };
                if let AxisSource::Waveform {
                    name,
                    x_range: (x0, x1),
                    y_range: (y0, y1),
                    frequency,
                    complex,
                } = source
                {
                    axis_row(
                        ui,
                        "X1",
                        if frequency {
                            "frequency"
                        } else {
                            "time / sweep"
                        },
                        if frequency { "log10" } else { "linear" },
                        (x0, x1),
                        if frequency { "decade" } else { "engineering" },
                        if frequency { "Hz" } else { "source" },
                    );
                    axis_row(ui, "Y1L", name, "linear", (y0, y1), "engineering", "source");
                    if complex {
                        axis_row(
                            ui,
                            "Y1R",
                            "complex projection",
                            "phase",
                            (-180.0, 180.0),
                            "45°",
                            "deg",
                        );
                    }
                } else {
                    ui.label("—");
                    ui.label("Active analysis has no visible waveform");
                    for _ in 0..4 {
                        ui.label("—");
                    }
                    ui.end_row();
                }
            });
        ui.add_space(12.0);
        ui.horizontal_wrapped(|ui| {
                labeled_combo(
                    ui,
                    "Autoscale",
                    host.autoscale().label(),
                    |ui| {
                        for value in VisualizationAutoscale::ALL {
                            let configured = host.autoscale_available(value);
                            ui.add_enabled_ui(configured, |ui| {
                                ui.selectable_value(
                                    host.autoscale(),
                                    value,
                                    value.label(),
                                );
                            })
                            .response
                            .on_disabled_hover_text(
                                "This fit policy is unavailable for the active renderer or requires a quantity-mapped axis limit.",
                            );
                        }
                    },
                );
                labeled_combo(
                    ui,
                    "Complex projection",
                    host.complex_projection().label(),
                    |ui| {
                        for value in ComplexProjection::ALL {
                            ui.selectable_value(
                                host.complex_projection(),
                                value,
                                value.label(),
                            );
                        }
                    },
                );
                let fit_blocker = host.fit_block_reason();
                let fit = Button::new("Fit active view")
                    .enabled(fit_blocker.is_none())
                    .show(ui);
                let fit = if let Some(reason) = fit_blocker {
                    fit.on_disabled_hover_text(reason)
                } else {
                    fit
                };
                if fit.clicked() {
                    host.request(SectionAction::Fit);
                }
            });
    });
}

fn axis_row(
    ui: &mut Ui,
    axis: &str,
    quantity: &str,
    transform: &str,
    range: (f64, f64),
    ticks: &str,
    unit: &str,
) {
    ui.monospace(axis);
    ui.label(quantity);
    ui.monospace(transform);
    ui.monospace(format!("{:.6e}…{:.6e}", range.0, range.1));
    ui.label(ticks);
    ui.monospace(unit);
    ui.end_row();
}

pub fn families_section(ui: &mut Ui, host: &mut impl SectionHost) {
    section_heading(ui, VisualizationSection::Families);
    let rows = host.families();
    section_scroll(ui, "visualization.families", |ui| {
        Grid::new("visualization.families.table")
            .num_columns(6)
            .striped(true)
            .spacing(vec2(18.0, 7.0))
            .show(ui, |ui| {
                for label in ["Dataset", "Run", "Analyses", "Samples", "Role", "Display"] {
                    table_header(ui, label);
                }
                ui.end_row();
                if rows.is_empty() {
                    ui.label("—");
                    ui.label("No retained datasets");
                    for _ in 0..4 {
                        ui.label("—");
                    }
                    ui.end_row();
                }
                let mut overlay_change = None;
                for FamilyRow {
                    dataset_id,
                    label,
                    analyses,
                    samples,
                    active,
                    overlaid,
                } in &rows
                {
                    ui.monospace(short_dataset(*dataset_id));
                    ui.label(label);
                    ui.monospace(analyses.to_string());
                    ui.monospace(engineering_count(*samples));
                    ui.label(if *active {
                        "active family"
                    } else {
                        "retained family"
                    });
                    if *active {
                        ui.label("always visible");
                    } else if ui.checkbox(&mut overlaid.clone(), "Overlay").changed() {
                        overlay_change = Some(*dataset_id);
                    }
                    ui.end_row();
                }
                if let Some(dataset_id) = overlay_change {
                    host.request(SectionAction::ToggleOverlay(dataset_id));
                }
            });
        ui.add_space(10.0);
        ui.horizontal_wrapped(|ui| {
            dock_action(ui, host, "Slice and pivot…", VisualizationDock::FamilySlice);
            dock_action(
                ui,
                host,
                "Visual encoding…",
                VisualizationDock::FamilyEncoding,
            );
            dock_action(
                ui,
                host,
                "Advanced filter…",
                VisualizationDock::FamilyFilter,
            );
        });
        concept_banner(
            ui,
            "Dataset overlays use stable dataset identities. Missing analyses remain absent; the viewer never invents family points or generated trace indices.",
        );
    });
}

pub fn measurements_section(ui: &mut Ui, host: &mut impl SectionHost) {
    section_heading(ui, VisualizationSection::Measurements);
    section_scroll(ui, "visualization.measurements", |ui| {
        Grid::new("visualization.measurements.table")
            .num_columns(6)
            .striped(true)
            .spacing(vec2(18.0, 7.0))
            .show(ui, |ui| {
                for label in ["Item", "Type", "Definition", "Unit", "Consumers", "Status"] {
                    table_header(ui, label);
                }
                ui.end_row();
                let view = host.measurements();
                let expressions = view.expressions;
                for measurement in view.measurements {
                    measurement_row(
                        ui,
                        &format!("M{}", measurement.id),
                        "scalar measurement",
                        &measurement.expression,
                        "source-derived",
                        &short_dataset(measurement.dataset_id),
                        &format!("{:.9e}", measurement.value),
                    );
                }
                for (index, expression) in expressions.iter().enumerate() {
                    measurement_row(
                        ui,
                        &format!("expr-{}", index + 1),
                        "expression",
                        &expression.text,
                        "source-derived",
                        "active pane",
                        if expression.visible {
                            "visible"
                        } else {
                            "hidden"
                        },
                    );
                }
                if view.cursor_a || view.cursor_b {
                    measurement_row(
                        ui,
                        "A / B",
                        "linked cursors",
                        "exact source coordinates",
                        "source",
                        "compatible panes",
                        if view.linked_cursors {
                            "linked"
                        } else {
                            "pane local"
                        },
                    );
                }
                for marker in view.markers {
                    measurement_row(
                        ui,
                        &marker.label,
                        "sample marker",
                        &format!(
                            "{}[{}] @ {:.9e}",
                            marker.waveform_name, marker.sample_index, marker.x
                        ),
                        "source",
                        "active pane",
                        "exact",
                    );
                }
                for annotation in view.annotations {
                    measurement_row(
                        ui,
                        &format!("NOTE-{}", annotation.id),
                        "review annotation",
                        &annotation.text,
                        "—",
                        "result document",
                        "open",
                    );
                }
                if view.measurements.is_empty()
                    && expressions.is_empty()
                    && !view.cursor_a
                    && view.markers.is_empty()
                    && view.annotations.is_empty()
                {
                    ui.label("—");
                    ui.label("No derived or review entities");
                    for _ in 0..4 {
                        ui.label("—");
                    }
                    ui.end_row();
                }
            });
        ui.add_space(10.0);
        ui.horizontal_wrapped(|ui| {
            dock_action(ui, host, "New measurement…", VisualizationDock::Measurement);
            dock_action(
                ui,
                host,
                "Cursor manager…",
                VisualizationDock::CursorManager,
            );
            dock_action(ui, host, "New annotation…", VisualizationDock::Annotation);
        });
    });
}

fn measurement_row(
    ui: &mut Ui,
    item: &str,
    kind: &str,
    definition: &str,
    unit: &str,
    consumers: &str,
    status: &str,
) {
    ui.monospace(item);
    ui.label(kind);
    ui.monospace(definition);
    ui.monospace(unit);
    ui.label(consumers);
    ui.label(status);
    ui.end_row();
}

pub fn large_data_section(ui: &mut Ui, host: &mut impl SectionHost) {
    section_heading(ui, VisualizationSection::LargeData);
    section_scroll(ui, "visualization.large-data", |ui| {
        let previous = *host.display_lod();
        ui.horizontal_wrapped(|ui| {
            labeled_combo(ui, "Display LOD", host.display_lod().label(), |ui| {
                for value in DisplayLodPolicy::ALL {
                    ui.selectable_value(host.display_lod(), value, value.label());
                }
            });
            numeric_policy(
                ui,
                "Tile memory",
                host.tile_memory_mib(),
                64..=16_384,
                "MiB",
            );
        });
        // A property row is a full-width label/value row with fixed columns,
        // so the cache policy is stated under the two controls rather than
        // claiming whatever is left of their wrapped row.
        property_row(ui, "Disk cache", "Not configured · no filesystem writes");
        ui.add_space(10.0);
        Grid::new("visualization.large-data.policies")
            .num_columns(2)
            .striped(true)
            .spacing(vec2(18.0, 7.0))
            .show(ui, |ui| {
                policy_row(
                    ui,
                    "Exact cursor query",
                    "Read original f64/complex source samples on demand",
                );
                policy_row(
                    ui,
                    "Remote streaming",
                    "Local immutable dataset registry; remote sources fail closed",
                );
                policy_row(
                    ui,
                    "Backpressure",
                    "Preserve solver output · delay presentation cache",
                );
                policy_row(
                    ui,
                    "Source precision",
                    "Measurements and exports bypass display LOD",
                );
            });
        if previous != *host.display_lod() {
            host.request(SectionAction::ApplyLod);
        }
        concept_banner(
            ui,
            "Decimation and level-of-detail affect rendering only. Measurements, exports, and cursor exact-value requests operate on the immutable source dataset.",
        );
    });
}

pub fn export_section(ui: &mut Ui, host: &mut impl SectionHost) {
    section_heading(ui, VisualizationSection::ExportReport);
    section_scroll(ui, "visualization.export", |ui| {
        Grid::new("visualization.export.table")
            .num_columns(5)
            .striped(true)
            .spacing(vec2(18.0, 7.0))
            .show(ui, |ui| {
                for label in ["Output", "Format", "Precision", "Layout", "Provenance"] {
                    table_header(ui, label);
                }
                ui.end_row();
                export_row(
                    ui,
                    "Active engineering viewer",
                    "PNG",
                    "rendered pixels",
                    "active viewport",
                    "dataset + revision in document",
                );
                export_row(
                    ui,
                    "Engineering dataset",
                    "CSV",
                    "full stored f64",
                    "shared-axis table",
                    "source analysis identity",
                );
            });
        ui.add_space(10.0);
        let exact_export_available = host.exact_export_available();
        let figure_export_available = host.figure_export_available();
        ui.horizontal_wrapped(|ui| {
            dock_action(
                ui,
                host,
                "Edit report pages…",
                VisualizationDock::PageEditor,
            );
            if Button::new("Export exact data…")
                .accent()
                .enabled(exact_export_available)
                .show(ui)
                .clicked()
            {
                host.request(SectionAction::ExportData);
            }
            if Button::new("Export viewer figure…")
                .enabled(figure_export_available)
                .show(ui)
                .clicked()
            {
                host.request(SectionAction::ExportFigure);
            }
        });
        concept_banner(
            ui,
            "Every enabled export action is backed by a real writer. Formats without an installed writer are not offered and no placeholder artifact is created.",
        );
    });
}

fn export_row(
    ui: &mut Ui,
    output: &str,
    format: &str,
    precision: &str,
    layout: &str,
    provenance: &str,
) {
    ui.label(output);
    ui.monospace(format);
    ui.label(precision);
    ui.label(layout);
    ui.label(provenance);
    ui.end_row();
}

fn dock_action(
    ui: &mut Ui,
    host: &mut impl SectionHost,
    label: &'static str,
    dock: VisualizationDock,
) {
    if Button::new(label).show(ui).clicked() {
        host.request(SectionAction::OpenDock(dock));
    }
}
