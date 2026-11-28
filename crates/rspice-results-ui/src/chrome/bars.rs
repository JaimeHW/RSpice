//! Results document bars and sheet controls over explicit host operations.

use super::{
    ExportRequests, ResultBarMetrics, export_menu, instrument_separator, viewer_picker_separator,
    viewer_tab, viewer_tab_scroller,
};
use egui::{Ui, WidgetInfo, WidgetType};
use rspice_results::result_presentation::ResultViewer;
use rspice_ui_kit::{
    panels::WorkbenchIcon,
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::{Button, FILTER_CLEAR_INSET, docbar_at_height, filter_field_clear, select},
};

/// Document selection and application navigation requested by the bar.
#[derive(Default)]
pub struct DocumentBarActions {
    pub viewer: Option<ResultViewer>,
    pub add_pane: bool,
    pub open_properties: bool,
    /// The index of the page the reader picked.
    pub page: Option<usize>,
    /// The reader switched the document between Latest and Pinned.
    pub switch_tracking: bool,
    /// The reader asked for the document to be archived.
    pub archive: bool,
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
        /// Which results it reads and whether it holds them itself.
        standing: DocumentStanding<'a>,
    },
}

/// Which results a saved document reads, and whether it holds them itself:
/// what its bar says of it beside its own commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DocumentStanding<'a> {
    pub tracking: DocumentTracking,
    pub archive: DocumentArchive<'a>,
}

/// Which run a saved document reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentTracking {
    /// It names no plan and analysis to follow, so it says nothing.
    Untracked,
    /// No completed run of the current design and netlist matches what it
    /// shows.
    Historical,
    /// It follows the newest run of its plan and analysis.
    Latest,
    /// It keeps the run it shows, whatever runs after it.
    Pinned,
}

/// Whether a saved document holds the samples of the runs it reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentArchive<'a> {
    /// It can take them in.
    Available,
    /// It holds them, and reads without the run history.
    Archived,
    /// It cannot, for the reason given.
    Unavailable(&'a str),
}

/// A document bar whose strip would hold one view's tab, folded into the
/// sheet bar under it. A strip of one tab offers nothing to pick, so it is
/// not drawn: the sheet bar carries the document's actions at its end, and
/// the row goes to the sheet. The view is named by its pane header alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoldedDocumentBar<'a> {
    /// A project result document's standing, whose bar takes more panes
    /// and names which results it reads; `None` over a quick view.
    pub document: Option<DocumentStanding<'a>>,
}

/// The document bar folded away when its strip would hold only the view
/// being shown and no page picker leads it; `None` when it draws.
#[must_use]
pub fn folded_document_bar<'a>(
    current: ResultViewer,
    available: &[ResultViewer],
    source: DocumentBarSource<'a>,
) -> Option<FoldedDocumentBar<'a>> {
    let (document, pages) = match source {
        DocumentBarSource::QuickView => (None, 1),
        DocumentBarSource::Document {
            pages, standing, ..
        } => (Some(standing), pages.len()),
    };
    (available == [current] && pages <= 1).then_some(FoldedDocumentBar { document })
}

/// The document's own actions, laid out right to left at the end of the
/// strip or of the sheet bar it folded into: Properties…, and over a
/// project `document` Add pane… and, past `separator`, which results it
/// reads. A quick view makes a new result document from the "+" at the end
/// of Results' tabs, so its bar offers only what a document does. Each text
/// control is the kit's bordered button, as every one on the Results bars
/// is, at the row's control height.
fn document_actions(
    ui: &mut Ui,
    document: Option<DocumentStanding<'_>>,
    separator: fn(&mut Ui),
) -> DocumentBarActions {
    let mut actions = DocumentBarActions {
        open_properties: Button::new("Properties…")
            .accessible_label("Plot properties")
            .show(ui)
            .clicked(),
        ..DocumentBarActions::default()
    };
    if let Some(standing) = document {
        actions.add_pane = Button::new("Add pane…")
            .accessible_label("Add a pane to this result document")
            .show(ui)
            .clicked();
        ui.add_space(DOCUMENT_ACTION_GAP);
        separator(ui);
        ui.add_space(DOCUMENT_ACTION_GAP);
        actions.archive = archive_control(ui, standing.archive);
        actions.switch_tracking = tracking_control(ui, standing.tracking);
    }
    actions
}

/// The gap between the document's actions, and from the last of them to the
/// rule before the tabs.
const DOCUMENT_ACTION_GAP: f32 = 4.0;

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

pub fn document_bar(
    ui: &mut Ui,
    current: ResultViewer,
    available: impl IntoIterator<Item = ResultViewer>,
    source: DocumentBarSource<'_>,
) -> DocumentBarActions {
    let mut actions = DocumentBarActions::default();
    let strip = docbar_at_height(ui, ResultBarMetrics::of(ui).viewer_tabs, |ui| {
        // The strip's last point is its rule: its tabs and buttons stand
        // centred in what is above it, each with a whole, equal gap above
        // and below.
        let above_rule = ui.max_rect().with_max_y(ui.max_rect().bottom() - 1.0);
        let ui = &mut ui.new_child(
            egui::UiBuilder::new()
                .max_rect(above_rule)
                .layout(egui::Layout::right_to_left(egui::Align::Center)),
        );
        ui.spacing_mut().item_spacing.x = DOCUMENT_ACTION_GAP;
        let document = match source {
            DocumentBarSource::QuickView => None,
            DocumentBarSource::Document { standing, .. } => Some(standing),
        };
        actions = document_actions(ui, document, viewer_picker_separator);
        ui.add_space(DOCUMENT_ACTION_GAP);
        viewer_picker_separator(ui);

        let tabs_size = ui.available_size();
        ui.allocate_ui_with_layout(
            tabs_size,
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                // A document of several pages shows one at a time; the
                // picker leads the row because the tabs below it belong to
                // the page it names.
                if let DocumentBarSource::Document { pages, page, .. } = source
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
    // Named as the sheet bar is, so the strip is one toolbar to a reader.
    strip
        .response
        .widget_info(|| WidgetInfo::labeled(WidgetType::Other, ui.is_enabled(), "Result views"));
    ui.ctx().accesskit_node_builder(strip.response.id, |node| {
        node.set_role(egui::accesskit::Role::Toolbar);
        node.set_label("Result views");
    });

    actions
}

/// Archive: embed the samples of the runs the document reads, so it reads
/// without the run history; an archived document says so instead. True when
/// the reader asked for it.
fn archive_control(ui: &mut Ui, archive: DocumentArchive<'_>) -> bool {
    match archive {
        DocumentArchive::Available => bar_command(
            ui,
            "Archive",
            Ok(
                "Archive into document: embed the samples of the runs this document reads, \
                so it reads without the run history.",
            ),
        ),
        DocumentArchive::Archived => {
            state_badge(
                ui,
                WorkbenchIcon::Save,
                "Archived",
                "This document holds its runs' samples and reads without the run history.",
                Tokens::get(ui.ctx()).color.text_dim,
            );
            false
        }
        DocumentArchive::Unavailable(reason) => bar_command(ui, "Archive", Err(reason)),
    }
}

/// Which run the document reads: Latest or Pinned, which a press switches,
/// or Historical, which only says so. True when the reader switched it.
fn tracking_control(ui: &mut Ui, tracking: DocumentTracking) -> bool {
    match tracking {
        DocumentTracking::Untracked => false,
        DocumentTracking::Historical => {
            state_badge(
                ui,
                WorkbenchIcon::History,
                "Historical",
                "This document still shows the results it was made from, but no completed \
                 run matches the current design and its netlist.",
                Tokens::get(ui.ctx()).color.warn,
            );
            false
        }
        DocumentTracking::Latest => bar_command(
            ui,
            "Latest",
            Ok(
                "Tracking latest: this document follows the newest run of its plan and \
                analysis. Press to pin it to the run it shows.",
            ),
        ),
        DocumentTracking::Pinned => bar_command(
            ui,
            "Pinned",
            Ok(
                "Pinned to this run's results; a rerun does not replace them. Press to \
                follow the newest run again.",
            ),
        ),
    }
}

/// A state of the document the bar names, not a command: its icon and
/// words in `ink`, the state's own, on the bar's ground with no hover fill
/// and nothing to press, and `meaning` as its tooltip.
fn state_badge(ui: &mut Ui, icon: WorkbenchIcon, label: &str, meaning: &str, ink: egui::Color32) {
    let galley = ui.painter().layout_no_wrap(
        label.to_owned(),
        theme::sans(tokens::FS_1, FontWeight::Regular),
        ink,
    );
    let width = galley.size().x + 36.0;
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(width, ResultBarMetrics::of(ui).viewer_tab),
        egui::Sense::hover(),
    );
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, true, label));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_description(meaning);
    });
    if ui.is_rect_visible(rect) {
        icon.paint(
            ui.painter(),
            egui::Rect::from_center_size(
                egui::pos2(rect.left() + 14.0, rect.center().y),
                egui::vec2(13.0, 13.0),
            ),
            ink,
        );
        ui.painter().galley(
            egui::pos2(rect.left() + 26.0, rect.center().y - galley.size().y * 0.5),
            galley,
            ink,
        );
    }
    response.on_hover_text(meaning);
}

/// A command of the document's bar: the kit's bordered button every text
/// control on the Results bars is, pressed when `explained` is `Ok` with
/// what it does, and greyed out, saying why on hover, when it is `Err`.
/// True when pressed.
fn bar_command(ui: &mut Ui, label: &str, explained: Result<&str, &str>) -> bool {
    let (Ok(words) | Err(words)) = explained;
    Button::new(label)
        .enabled(explained.is_ok())
        .show(ui)
        .on_hover_text(words)
        .on_disabled_hover_text(words)
        .clicked()
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
    // bury the ones that work.
    for viewer in available {
        if viewer_tab(ui, viewer, current == viewer) {
            clicked = Some(viewer);
        }
    }
    clicked
}

/// The sheet's one bar. With `folded`, the document bar over it drew
/// nothing, and this bar carries the document's actions, which it returns,
/// at its end.
pub fn show(
    ui: &mut Ui,
    host: &mut impl SheetBarHost,
    folded: Option<FoldedDocumentBar<'_>>,
) -> DocumentBarActions {
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

    // The bar's last point is its rule: its controls stand centred in what
    // is above the rule, with an equal gap above and below each.
    let mut content = rect
        .with_max_y(rect.bottom() - 1.0)
        .shrink2(egui::vec2(8.0, 0.0));
    // Every control in the bar stands at the bar's control height, on its
    // centre line: the kit's controls and egui's buttons take the row's
    // interact height, and a button's padding is what centres its words in
    // it rather than the theme's taller control.
    let control = metrics.instrument_control;
    let bar_ui = |ui: &mut Ui, rect: egui::Rect, layout: egui::Layout| {
        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(rect).layout(layout));
        child.spacing_mut().item_spacing.x = 4.0;
        let words = child.text_style_height(&egui::TextStyle::Button);
        // An egui button's frame adds its stroke around the padding.
        let stroke = child.visuals().widgets.inactive.bg_stroke.width;
        let spacing = child.spacing_mut();
        spacing.interact_size.y = control;
        spacing.button_padding.y = ((control - words) * 0.5 - stroke).max(0.0);
        child
    };
    let mut actions = DocumentBarActions::default();
    if let Some(folded) = folded {
        // The document's actions close the bar, as they close the strip
        // they came from. The bar opens with the view's controls, as it
        // does under a strip, so a second view's strip coming in leaves them
        // where they stand.
        let mut end = bar_ui(
            ui,
            content,
            egui::Layout::right_to_left(egui::Align::Center),
        );
        actions = document_actions(&mut end, folded.document, instrument_separator);
        instrument_separator(&mut end);
        content.max.x = end.min_rect().left() - 4.0;
    }
    let mut child = bar_ui(
        ui,
        content,
        egui::Layout::left_to_right(egui::Align::Center),
    );
    if structured {
        show_structured_result_strip(&mut child, host);
    } else if matches!(
        viewer,
        ResultViewer::Waves
            | ResultViewer::DcSweep
            | ResultViewer::Bode
            | ResultViewer::SParameters
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
    actions
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

fn operating_point_filter(ui: &mut Ui, filter: &mut String) {
    sheet_filter_field(ui, operating_point_filter_id(), filter);
}

/// A sheet's filter on its bar, the field `id` names. Nothing beside it
/// comes or goes as the reader types, and the inset is kept whether or not
/// the mark is showing, so neither the field nor its text moves on the
/// first keystroke.
pub(crate) fn sheet_filter_field(ui: &mut Ui, id: egui::Id, filter: &mut String) {
    // egui fades the body tone for a hint, which fell to 3.84:1 on the field
    // in the light interface; the palette's tertiary text holds 4.5:1.
    let faint = Tokens::get(ui.ctx()).color.text_faint;
    let restored = ui.visuals_mut().weak_text_color.replace(faint);
    // Exactly as tall as every other control in the bar, its words on its
    // centre line. Margins rounded to whole points made a field a point
    // taller than the bar's control whenever the line left an odd remainder,
    // so it stood a point nearer the rule than the top.
    let height = ui.spacing().interact_size.y;
    let field = ui.add_sized(
        [150.0, height],
        egui::TextEdit::singleline(filter)
            .id(id)
            .font(theme::field_mono())
            .vertical_align(egui::Align::Center)
            .margin(egui::Margin {
                left: 4,
                right: FILTER_CLEAR_INSET,
                top: 0,
                bottom: 0,
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
