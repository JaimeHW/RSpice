//! Results document bars and sheet controls over explicit host operations.

use super::{
    ExportRequests, ResultBarMetrics, export_menu, viewer_picker, viewer_picker_separator,
    viewer_tab, viewer_tab_scroller,
};
use egui::{Ui, WidgetInfo, WidgetType};
use rspice_results::result_presentation::ResultViewer;
use rspice_ui_kit::{
    panels::WorkbenchIcon,
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::{chip, docbar_at_height},
};

/// Document selection and application navigation requested by the bar.
#[derive(Default)]
pub struct DocumentBarActions {
    pub viewer: Option<ResultViewer>,
    pub create_document: bool,
    pub open_properties: bool,
}

pub enum SpecificationAction {
    Discard,
    Apply,
    Edit,
}

/// Source-aware sheet controls and immediate application transactions.
pub trait SheetBarHost {
    fn viewer(&self) -> ResultViewer;
    fn export(&mut self, requests: ExportRequests);
    fn fft_summary(&self) -> Option<String>;
    fn operating_point_filter(&mut self) -> &mut String;
    fn specification_editing(&self) -> bool;
    fn request(&mut self, action: SpecificationAction);
    fn instrument(&mut self, ui: &mut Ui);
    fn eye_actions(&mut self, ui: &mut Ui);
    fn table_actions(&mut self, ui: &mut Ui);
    fn domain_controls(&mut self, ui: &mut Ui) -> bool;
    fn purpose(&self) -> String;
}
pub const fn viewer_has_sheet_bar(viewer: ResultViewer) -> bool {
    !matches!(
        viewer,
        ResultViewer::TransferFunction | ResultViewer::Manifest
    )
}

pub const fn viewer_has_structured_strip(viewer: ResultViewer) -> bool {
    matches!(
        viewer,
        ResultViewer::Op | ResultViewer::Specs | ResultViewer::Table
    )
}

pub fn compact_document_bar(
    ui: &mut Ui,
    current: ResultViewer,
    available: impl IntoIterator<Item = ResultViewer>,
) -> Option<ResultViewer> {
    let mut selected = None;
    docbar_at_height(ui, ResultBarMetrics::of(ui).viewer_tabs, |ui| {
        viewer_tab_scroller(ui, "rspice.results.split.viewer-tabs", |ui| {
            selected = viewer_tabs(ui, current, available);
        });
    });
    selected
}

pub fn document_bar(
    ui: &mut Ui,
    current: ResultViewer,
    available: impl IntoIterator<Item = ResultViewer>,
) -> DocumentBarActions {
    let mut actions = DocumentBarActions::default();
    docbar_at_height(ui, ResultBarMetrics::of(ui).viewer_tabs, |ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            actions.open_properties =
                viewer_picker(ui, WorkbenchIcon::Sliders, "Properties…", "Plot properties");
            actions.create_document = viewer_picker(
                ui,
                WorkbenchIcon::Add,
                "Create result document…",
                "Create a dataset-bound result document",
            );
            viewer_picker_separator(ui);

            let tabs_size = ui.available_size();
            ui.allocate_ui_with_layout(
                tabs_size,
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    viewer_tab_scroller(ui, "rspice.results.viewer-tabs", |ui| {
                        actions.viewer = viewer_tabs(ui, current, available);
                    });
                },
            );
        });
    });

    actions
}

pub fn viewer_tabs(
    ui: &mut Ui,
    current: ResultViewer,
    available: impl IntoIterator<Item = ResultViewer>,
) -> Option<ResultViewer> {
    ui.spacing_mut().item_spacing.x = 0.0;

    let mut clicked: Option<ResultViewer> = None;

    // The strip lists only the sheets this dataset can feed. With 22 viewers
    // and a typical run feeding a handful, a full row of disabled tabs would
    // bury the ones that work; the Visualization Studio catalog is where every
    // viewer and its requirements are published.
    for viewer in available {
        if viewer_tab(ui, viewer, current == viewer) {
            clicked = Some(viewer);
        }
    }
    clicked
}

pub fn show(ui: &mut Ui, host: &mut impl SheetBarHost) {
    let t = Tokens::get(ui.ctx());
    let viewer = host.viewer();
    let structured = viewer_has_structured_strip(viewer);
    let metrics = ResultBarMetrics::resolve(&t);
    let height = if structured {
        metrics.structured_strip
    } else {
        metrics.sheet_bar
    };
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, t.color.border),
    );
    let accessible_label = if structured {
        "Structured result controls"
    } else if matches!(
        viewer,
        ResultViewer::Waves
            | ResultViewer::DcSweep
            | ResultViewer::Bode
            | ResultViewer::NoiseContrib
    ) {
        "Plot instrument controls"
    } else {
        "Result sheet controls"
    };
    response
        .widget_info(|| WidgetInfo::labeled(WidgetType::Other, ui.is_enabled(), accessible_label));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Toolbar);
        node.set_label(accessible_label);
    });

    let content = rect.shrink2(egui::vec2(if structured { 12.0 } else { 8.0 }, 0.0));
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(content)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    child.spacing_mut().item_spacing.x = 4.0;
    if structured {
        show_structured_result_strip(&mut child, host);
    } else if matches!(
        viewer,
        ResultViewer::Waves
            | ResultViewer::DcSweep
            | ResultViewer::Bode
            | ResultViewer::NoiseContrib
    ) {
        host.instrument(&mut child);
    } else {
        child.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // The evidence sheets exist to be recorded, so they carry the same
            // export affordance the other tabular sheets do. The plotted
            // sheets keep theirs on the instrument strip instead.
            inline_result_actions(ui, host);
            let remaining = ui.available_size();
            ui.allocate_ui_with_layout(
                remaining,
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    if host.domain_controls(ui) {
                        return;
                    }
                    ui.label(
                        egui::RichText::new(host.purpose())
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_dim),
                    );
                },
            );
        });
    }
}

fn show_structured_result_strip(ui: &mut Ui, host: &mut impl SheetBarHost) {
    let t = Tokens::get(ui.ctx());
    let title = match host.viewer() {
        ResultViewer::Op => "Operating point · DC solution",
        ResultViewer::Specs => "Specifications",
        ResultViewer::Table => "Exact retained samples",
        _ => return,
    };

    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        host.export(export_menu(ui));
        let remaining = ui.available_size();
        ui.allocate_ui_with_layout(
            remaining,
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                egui::ScrollArea::horizontal()
                    .id_salt(("rspice.results.structured-strip", host.viewer()))
                    .auto_shrink([false, true])
                    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 10.0;
                            ui.label(
                                egui::RichText::new(title)
                                    .font(theme::sans(tokens::FS_1, FontWeight::Medium))
                                    .color(t.color.text),
                            );
                            result_viewer_actions(ui, host);
                        });
                    });
            },
        );
    });
}

fn inline_result_actions(ui: &mut Ui, host: &mut impl SheetBarHost) {
    host.export(export_menu(ui));
    result_viewer_actions(ui, host);
}

fn result_viewer_actions(ui: &mut Ui, host: &mut impl SheetBarHost) {
    match host.viewer() {
        ResultViewer::Fft => {
            let label = host.fft_summary();
            if let Some(label) = label {
                ui.label(
                    egui::RichText::new(label)
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(Tokens::get(ui.ctx()).color.text_faint),
                );
            }
        }
        ResultViewer::Eye => host.eye_actions(ui),
        ResultViewer::Op => {
            let filter = host.operating_point_filter();
            if !filter.is_empty() && chip(ui, "clear", true).clicked() {
                filter.clear();
            }
            ui.add(
                egui::TextEdit::singleline(filter)
                    .desired_width(150.0)
                    .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                    .hint_text("filter devices…"),
            );
        }
        ResultViewer::Specs => {
            if host.specification_editing() {
                if ui.button("Discard").clicked() {
                    host.request(SpecificationAction::Discard);
                }
                if ui.button("Apply").clicked() {
                    host.request(SpecificationAction::Apply);
                }
            } else if ui.button("Edit specs…").clicked() {
                host.request(SpecificationAction::Edit);
            }
        }
        ResultViewer::Table => host.table_actions(ui),
        _ => {}
    }
}
