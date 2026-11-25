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
    widgets::{FILTER_CLEAR_INSET, docbar_at_height, filter_field_clear, select},
};

/// Document selection and application navigation requested by the bar.
#[derive(Default)]
pub struct DocumentBarActions {
    pub viewer: Option<ResultViewer>,
    pub create_document: bool,
    pub add_pane: bool,
    pub open_properties: bool,
    /// The index of the page the reader picked.
    pub page: Option<usize>,
}

/// What the bar is in front of: a dataset's quick view, or a project result
/// document, which can take more panes and may hold several pages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentBarSource<'a> {
    QuickView,
    Document {
        /// Page titles in document order, and the page being shown.
        pages: &'a [String],
        page: usize,
    },
}

/// Width of the page picker: room for a page title without crowding the
/// viewer tabs beside it.
const PAGE_PICKER_WIDTH: f32 = 150.0;
/// Air on either side of the page picker, the inset a viewer tab keeps.
const PAGE_PICKER_GUTTER: f32 = 8.0;

/// Source-aware sheet controls and immediate application transactions.
pub trait SheetBarHost {
    fn viewer(&self) -> ResultViewer;
    fn export(&mut self, requests: ExportRequests);
    fn operating_point_filter(&mut self) -> &mut String;
    fn instrument(&mut self, ui: &mut Ui);
    fn eye_actions(&mut self, ui: &mut Ui);
    fn table_actions(&mut self, ui: &mut Ui);
    fn domain_controls(&mut self, ui: &mut Ui) -> bool;
    fn purpose(&self) -> String;
    /// The state the sheet's viewer stays readable under, as a mark at the
    /// bar's right end: Out of date, Partial, Running. Nothing when the
    /// result is complete or the state takes the well over.
    fn status_mark(&mut self, _ui: &mut Ui) {}
}

/// A word naming the control after it in a sheet bar: sans, sentence case,
/// in the dim ink, so the controls carry the weight.
pub fn bar_label(ui: &mut Ui, text: &str) {
    ui.label(
        egui::RichText::new(text)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(Tokens::get(ui.ctx()).color.text_dim),
    );
}

pub const fn viewer_has_structured_strip(viewer: ResultViewer) -> bool {
    matches!(viewer, ResultViewer::Op | ResultViewer::Table)
}

/// What the plot pane's controls asked of the pane beside the document.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CompactBarActions {
    pub viewer: Option<ResultViewer>,
    /// Open the run in the Results workspace, where measurements, cursors
    /// and several plots are.
    pub open_in_results: bool,
    /// Move the pane to the document's other side.
    pub turn: bool,
    /// Close the pane.
    pub close: bool,
}

/// How many window places the plot pane's controls take at the right end of
/// the pane's first row: Close, the side and Open in Results, and the sheet
/// when the run has more than one to plot.
pub const fn compact_pane_control_slots(sheets: usize) -> usize {
    if sheets > 1 { 4 } else { 3 }
}

/// Where the plot pane's state mark stands: the window place after its
/// controls, in a pane with no sheet bar to carry it. A caller that draws
/// one keeps that place free in the pane's first row.
pub fn compact_pane_status_rect(pane: egui::Rect, sheets: usize) -> egui::Rect {
    let row = egui::Rect::from_min_size(
        pane.min,
        egui::vec2(pane.width(), crate::waves::header::WAVE_PANE_HEADER_HEIGHT),
    );
    let side = crate::strip::HEADER_ICON_SIDE;
    let cell = crate::strip::window_control_rect(row, compact_pane_control_slots(sheets));
    egui::Rect::from_center_size(cell.center(), egui::vec2(side, side))
}

/// The plot pane's controls, at the right end of the pane's first row in
/// the window places that row keeps free for them, so the pane spends no
/// row of its own on them: Close, the side the pane takes, Open in Results,
/// and the sheet when the run has more than one to plot. `beside` says the
/// pane is to the document's right, so the side control offers below.
pub fn compact_pane_controls(
    ui: &mut Ui,
    pane: egui::Rect,
    current: ResultViewer,
    sheets: &[ResultViewer],
    beside: bool,
) -> CompactBarActions {
    let mut actions = CompactBarActions::default();
    let row = egui::Rect::from_min_size(
        pane.min,
        egui::vec2(pane.width(), crate::waves::header::WAVE_PANE_HEADER_HEIGHT),
    );
    let side = crate::strip::HEADER_ICON_SIDE;
    let place = |ui: &mut Ui, slot: usize, salt: &str, draw: &mut dyn FnMut(&mut Ui)| {
        let cell = crate::strip::window_control_rect(row, slot);
        let rect = egui::Rect::from_center_size(cell.center(), egui::vec2(side, side));
        ui.scope_builder(egui::UiBuilder::new().max_rect(rect).id_salt(salt), |ui| {
            draw(ui)
        });
    };
    let size = egui::vec2(side, side);
    place(ui, 0, "plot-pane-close", &mut |ui| {
        actions.close =
            rspice_ui_kit::panels::icon_button(ui, WorkbenchIcon::Close, "Close plot", false, size)
                .on_hover_text("Close plot")
                .clicked();
    });
    let (icon, label) = if beside {
        (WorkbenchIcon::Console, "Plot below the drawing")
    } else {
        (WorkbenchIcon::Inspector, "Plot beside the drawing")
    };
    place(ui, 1, "plot-pane-side", &mut |ui| {
        actions.turn = rspice_ui_kit::panels::icon_button(ui, icon, label, false, size)
            .on_hover_text(label)
            .clicked();
    });
    place(ui, 2, "plot-pane-results", &mut |ui| {
        actions.open_in_results = rspice_ui_kit::panels::icon_button(
            ui,
            WorkbenchIcon::Results,
            "Open in Results",
            false,
            size,
        )
        .on_hover_text("Open in Results: measurements, cursors and every sheet")
        .clicked();
    });
    if sheets.len() > 1 {
        place(ui, 3, "plot-pane-sheet", &mut |ui| {
            let label = format!("Sheet: {}", current.tab_label());
            let button =
                rspice_ui_kit::panels::icon_button(ui, WorkbenchIcon::Layers, &label, false, size)
                    .on_hover_text(&label);
            egui::Popup::menu(&button).show(|ui| {
                for &sheet in sheets {
                    if ui
                        .selectable_label(sheet == current, sheet.tab_label())
                        .clicked()
                    {
                        actions.viewer = Some(sheet);
                        ui.close();
                    }
                }
            });
        });
    }
    actions
}

pub fn document_bar(
    ui: &mut Ui,
    current: ResultViewer,
    available: impl IntoIterator<Item = ResultViewer>,
    source: DocumentBarSource<'_>,
) -> DocumentBarActions {
    let mut actions = DocumentBarActions::default();
    docbar_at_height(ui, ResultBarMetrics::of(ui).viewer_tabs, |ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            actions.open_properties =
                viewer_picker(ui, WorkbenchIcon::Sliders, "Properties…", "Plot properties");
            match source {
                DocumentBarSource::QuickView => {
                    actions.create_document = viewer_picker(
                        ui,
                        WorkbenchIcon::Add,
                        "Create result document…",
                        "Create a dataset-bound result document",
                    );
                }
                DocumentBarSource::Document { .. } => {
                    actions.add_pane = viewer_picker(
                        ui,
                        WorkbenchIcon::Add,
                        "Add pane…",
                        "Add a pane to this result document",
                    );
                }
            }
            viewer_picker_separator(ui);

            let tabs_size = ui.available_size();
            ui.allocate_ui_with_layout(
                tabs_size,
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    // A document of several pages shows one at a time; the
                    // picker leads the row because the tabs below it belong
                    // to the page it names.
                    if let DocumentBarSource::Document { pages, page } = source
                        && pages.len() > 1
                    {
                        ui.add_space(PAGE_PICKER_GUTTER);
                        actions.page = select(
                            ui,
                            "rspice.results.document-page",
                            "Page",
                            pages.get(page).map_or("", String::as_str),
                            pages,
                            PAGE_PICKER_WIDTH,
                        );
                        ui.add_space(PAGE_PICKER_GUTTER);
                        viewer_picker_separator(ui);
                    }
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
    // bury the ones that work; the Create Result Document viewer list is
    // where every viewer and its requirements are published.
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
    // One bar for every sheet: the structured documents' bar is as tall as
    // the plot sheets', and its controls stand at the same height on the
    // same centre line.
    let metrics = ResultBarMetrics::resolve(&t);
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), metrics.sheet_bar),
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
            | ResultViewer::SParameters
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

    let content = rect.shrink2(egui::vec2(8.0, 0.0));
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(content)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    child.spacing_mut().item_spacing.x = 4.0;
    // Every control in the bar stands at the bar's control height, on its
    // centre line: the kit's controls and egui's buttons take the row's
    // interact height, and a button's padding is what centres its words in
    // it rather than the theme's taller control.
    let control = metrics.instrument_control;
    let words = child.text_style_height(&egui::TextStyle::Button);
    // An egui button's frame adds its stroke around the padding.
    let stroke = child.visuals().widgets.inactive.bg_stroke.width;
    let spacing = child.spacing_mut();
    spacing.interact_size.y = control;
    spacing.button_padding.y = ((control - words) * 0.5 - stroke).max(0.0);
    // The sheet's state stands at the bar's right end, after its outputs,
    // so a result read as out of date costs the plot no row of its own.
    child.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        host.status_mark(ui);
        let rest = ui.available_size();
        ui.allocate_ui_with_layout(
            rest,
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| sheet_bar_controls(ui, host, viewer, structured, &t),
        );
    });
}

/// The bar's controls left of its state: the structured strip, the plot
/// instrument, or a sheet's domain controls and outputs.
fn sheet_bar_controls(
    child: &mut Ui,
    host: &mut impl SheetBarHost,
    viewer: ResultViewer,
    structured: bool,
    t: &Tokens,
) {
    if structured {
        show_structured_result_strip(child, host);
    } else if matches!(
        viewer,
        ResultViewer::Waves
            | ResultViewer::DcSweep
            | ResultViewer::Bode
            | ResultViewer::SParameters
            | ResultViewer::NoiseContrib
    ) {
        host.instrument(child);
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
                            host.domain_controls(ui);
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

fn operating_point_filter_id() -> egui::Id {
    egui::Id::new("rspice.results.operating-point-filter")
}

/// The sheet's filter. Nothing beside it comes or goes as the reader types,
/// and the inset is kept whether or not the mark is showing, so neither the
/// field nor its text moves on the first keystroke.
fn operating_point_filter(ui: &mut Ui, filter: &mut String) {
    // egui fades the body tone for a hint, which fell to 3.84:1 on the field
    // in the light interface; the palette's tertiary text holds 4.5:1.
    let faint = Tokens::get(ui.ctx()).color.text_faint;
    let restored = ui.visuals_mut().weak_text_color.replace(faint);
    // As tall as every other control in the bar, its words on its centre
    // line: a field is its line of text and its margins, so the margins make
    // up the bar's control height.
    let font = theme::field_mono();
    let line = ui.fonts_mut(|fonts| fonts.row_height(&font));
    let pad = ((ui.spacing().interact_size.y - line) * 0.5)
        .round()
        .max(0.0) as i8;
    let field = ui.add(
        egui::TextEdit::singleline(filter)
            .id(operating_point_filter_id())
            .desired_width(150.0)
            .font(font)
            .margin(egui::Margin {
                left: 4,
                right: FILTER_CLEAR_INSET,
                top: pad,
                bottom: pad,
            })
            .hint_text("Filter…"),
    );
    ui.visuals_mut().weak_text_color = restored;
    filter_field_clear(ui, &field, filter);
}

fn result_viewer_actions(ui: &mut Ui, host: &mut impl SheetBarHost) {
    match host.viewer() {
        ResultViewer::Eye => host.eye_actions(ui),
        ResultViewer::Op => operating_point_filter(ui, host.operating_point_filter()),
        ResultViewer::Table => host.table_actions(ui),
        _ => {}
    }
}

#[cfg(test)]
mod tests;
