//! Report paper, summary and element controls. Source authentication stays with the host.
use super::*;
use egui::{Color32, ScrollArea, Sense, Stroke, Ui, Vec2};
use egui_extras::{Column, TableBuilder};
use rspice_app_types::product::ResultDocumentId;
use rspice_results::report_document::{
    ReportBlock, ReportBlockId, ReportBlockKind, ReportDocument, ReportPage, ReportPageId,
    ReportReferenceMode, ReportSourceId,
};
use rspice_ui_kit::{
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::Button,
};
use std::borrow::Cow;

#[derive(Debug, Clone, Copy)]
pub struct SummaryMetrics {
    pub checks_passing: usize,
    pub checks_total: usize,
    pub joint_yield_percent: Option<f64>,
    pub pvt_completed: Option<usize>,
    pub pvt_total: Option<usize>,
}

pub trait PreviewHost {
    fn summary_metrics(&self) -> SummaryMetrics;
    fn selected_block(&self) -> Option<ReportBlockId>;
    fn select_block(&mut self, block: ReportBlockId);
    fn reference_resolves(&self, reference: &ReportReferenceMode) -> bool;
    fn writable(&self) -> bool;
    fn blocked_reason(&self) -> &'static str;
    fn set_block_enabled(
        &mut self,
        document: ResultDocumentId,
        block: ReportBlockId,
        enabled: bool,
    );
    fn retained_figure_available(&self) -> bool;
    fn add_element(&mut self);
    fn remove_element(&mut self);
    fn insert_result(&mut self);
}

pub fn show(
    ui: &mut Ui,
    host: &mut impl PreviewHost,
    document: &ReportDocument,
    selected_page: Option<ReportPageId>,
    separators: PaneSeparators,
) {
    let t = Tokens::get(ui.ctx());
    let summary_metrics = host.summary_metrics();
    let width = ui.available_width();
    let height = ui.available_height();
    let pane = egui::Frame::new()
        .fill(PAPER_PANEL)
        .show(ui, |ui| {
            ui.set_min_size(Vec2::new(width.max(1.0), height.max(1.0)));
            ScrollArea::vertical()
                .id_salt("report-authoring.preview")
                .show(ui, |ui| {
                    let compact = ui.available_width() < 560.0;
                    let horizontal_margin = if compact { 18.0 } else { 42.0 };
                    let top_margin = if compact { 24.0 } else { 36.0 };
                    egui::Frame::new()
                        .inner_margin(egui::Margin {
                            left: horizontal_margin as i8,
                            right: horizontal_margin as i8,
                            top: top_margin as i8,
                            bottom: 36,
                        })
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            paper_label(
                                ui,
                                "PROJECT REPORT DOCUMENT",
                                theme::mono(tokens::FS_0, FontWeight::Medium),
                                PAPER_MUTED,
                            );
                            ui.add_space(8.0);
                            paper_label(
                                ui,
                                document.title(),
                                theme::sans(26.0, FontWeight::SemiBold),
                                PAPER_TEXT,
                            );
                            ui.add_space(4.0);
                            paper_label(
                                ui,
                                &format!(
                                    "Document revision {} · {}",
                                    document.revision().get(),
                                    report_template_label(document.template()),
                                ),
                                theme::sans(tokens::FS_1, FontWeight::Regular),
                                PAPER_MUTED,
                            );

                            let page = selected_page.and_then(|id| document.page(id));
                            let page_index = page.and_then(|page| {
                                document
                                    .pages()
                                    .iter()
                                    .position(|candidate| candidate.id() == page.id())
                            });
                            let marker = page_index
                                .map(|index| page_marker(index, page.map_or("", |p| p.title())))
                                .unwrap_or("—");
                            let title = page.map_or("No report page selected", |page| page.title());
                            let description = page.map_or_else(
                                || "Select a page from the report outline.".to_owned(),
                                |page| {
                                    format!(
                                        "Page revision {} · {}",
                                        page.revision().get(),
                                        page_update_policy_label(page.update_policy())
                                    )
                                },
                            );
                            ui.add_space(24.0);
                            section_heading(ui, marker, title, &description);
                            ui.add_space(28.0);
                            summary_grid(ui, summary_metrics, compact);
                            if let Some(page) = page {
                                ui.add_space(28.0);
                                page_elements(ui, host, document, page, compact);
                            }
                            ui.add_space(24.0);
                            paper_label(
                                ui,
                                "Document source",
                                theme::sans(tokens::FS_3, FontWeight::SemiBold),
                                PAPER_TEXT,
                            );
                            ui.add_space(7.0);
                            paper_label(
                                ui,
                                "This source is the canonical project-owned ReportDocument. Page and document changes are applied as validated, revision-checked transactions and persisted with the project.",
                                theme::sans(tokens::FS_1, FontWeight::Regular),
                                PAPER_MUTED,
                            );
                        });
                });
        });
    paint_pane_separators(ui, pane.response.rect, separators, t.color.border);
}

fn section_heading(ui: &mut Ui, marker: &str, title: &str, description: &str) {
    let (line, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 1.0), Sense::hover());
    ui.painter().hline(
        line.x_range(),
        line.center().y,
        Stroke::new(1.0, PAPER_BORDER),
    );
    ui.add_space(13.0);
    ui.horizontal_top(|ui| {
        ui.set_width(ui.available_width());
        ui.allocate_ui_with_layout(
            Vec2::new(34.0, 0.0),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                paper_label(
                    ui,
                    marker,
                    theme::mono(tokens::FS_2, FontWeight::SemiBold),
                    PAPER_ACCENT,
                );
            },
        );
        ui.add_space(10.0);
        ui.vertical(|ui| {
            paper_label(
                ui,
                title,
                theme::sans(15.0, FontWeight::SemiBold),
                PAPER_TEXT,
            );
            ui.add_space(4.0);
            paper_label(
                ui,
                description,
                theme::sans(tokens::FS_1, FontWeight::Regular),
                PAPER_MUTED,
            );
        });
    });
}

fn summary_grid(ui: &mut Ui, metrics: SummaryMetrics, compact: bool) {
    let yield_value = metrics
        .joint_yield_percent
        .map_or_else(|| "not run".to_owned(), |joint| format!("{:.1}%", joint));
    let pvt_value = metrics.pvt_completed.zip(metrics.pvt_total).map_or_else(
        || "not run".to_owned(),
        |(completed, total)| format!("{completed} / {total}"),
    );
    let cells = [
        (
            format!("{} / {}", metrics.checks_passing, metrics.checks_total),
            "configured checks passing",
        ),
        (yield_value, "Monte Carlo yield estimate"),
        (pvt_value, "PVT points completed"),
    ];
    if compact {
        for (value, label) in cells {
            summary_cell(ui, &value, label);
            ui.add_space(8.0);
        }
    } else {
        let width = ui.available_width();
        let cell_width = ((width - 16.0) / 3.0).max(1.0);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            for (value, label) in cells {
                ui.allocate_ui_with_layout(
                    Vec2::new(cell_width, 76.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| summary_cell(ui, &value, label),
                );
            }
        });
    }
}

fn summary_cell(ui: &mut Ui, value: &str, label: &str) {
    egui::Frame::new()
        .fill(PAPER)
        .stroke(Stroke::new(1.0, PAPER_BORDER))
        .inner_margin(egui::Margin::same(14))
        .show(ui, |ui| {
            ui.set_min_width((ui.available_width() - 28.0).max(1.0));
            paper_label(
                ui,
                value,
                theme::sans(20.0, FontWeight::SemiBold),
                PAPER_TEXT,
            );
            ui.add_space(4.0);
            paper_label(
                ui,
                label,
                theme::sans(tokens::FS_0, FontWeight::Regular),
                PAPER_FAINT,
            );
        });
}

fn page_elements(
    ui: &mut Ui,
    host: &mut impl PreviewHost,
    document: &ReportDocument,
    page: &ReportPage,
    compact: bool,
) {
    paper_label(
        ui,
        "Page elements",
        theme::sans(tokens::FS_3, FontWeight::SemiBold),
        PAPER_TEXT,
    );
    ui.add_space(8.0);

    let blocks = page
        .sections()
        .iter()
        .flat_map(|section| section.blocks())
        .collect::<Vec<_>>();
    if blocks.is_empty() {
        egui::Frame::new()
            .fill(PAPER)
            .stroke(Stroke::new(1.0, PAPER_BORDER))
            .inner_margin(egui::Margin::same(14))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                paper_label(
                    ui,
                    "No page elements. Add authored content or insert an immutable result document.",
                    theme::sans(tokens::FS_1, FontWeight::Regular),
                    PAPER_MUTED,
                );
            });
    } else {
        ScrollArea::horizontal()
            .id_salt(("report-page-elements", page.id()))
            .show(ui, |ui| {
                ui.set_min_width(if compact {
                    680.0
                } else {
                    ui.available_width().max(680.0)
                });
                let previous_faint_background = ui.visuals().faint_bg_color;
                ui.visuals_mut().faint_bg_color = Color32::from_rgb(249, 249, 247);
                TableBuilder::new(ui)
                    .id_salt(("report-page-elements-table", page.id()))
                    .striped(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::remainder().at_least(150.0))
                    .column(Column::initial(110.0).at_least(96.0))
                    .column(Column::remainder().at_least(180.0))
                    .column(Column::initial(104.0).at_least(90.0))
                    .column(Column::initial(46.0).at_least(42.0))
                    .header(27.0, |mut header| {
                        for label in ["Element", "Type", "Bound source", "State", "On"] {
                            header.col(|ui| {
                                ui.painter().rect_filled(
                                    ui.max_rect(),
                                    0.0,
                                    Color32::from_rgb(244, 243, 239),
                                );
                                ui.label(
                                    egui::RichText::new(label)
                                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                                        .color(PAPER_MUTED),
                                );
                            });
                        }
                    })
                    .body(|mut body| {
                        for block in blocks {
                            body.row(24.0, |mut row| {
                                row.col(|ui| {
                                    let selected = host.selected_block() == Some(block.id());
                                    if ui
                                        .selectable_label(
                                            selected,
                                            egui::RichText::new(report_block_element_title(
                                                block.kind(),
                                            ))
                                            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                                            .color(PAPER_TEXT),
                                        )
                                        .clicked()
                                    {
                                        host.select_block(block.id());
                                    }
                                });
                                row.col(|ui| {
                                    ui.label(
                                        egui::RichText::new(report_block_kind_label(block.kind()))
                                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                            .color(PAPER_MUTED),
                                    );
                                });
                                row.col(|ui| {
                                    ui.label(
                                        egui::RichText::new(report_block_bound_source(
                                            block.kind(),
                                        ))
                                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                        .color(PAPER_MUTED),
                                    );
                                });
                                row.col(|ui| {
                                    let (state, color) = report_block_state(host, block);
                                    ui.label(
                                        egui::RichText::new(state)
                                            .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                                            .color(color),
                                    );
                                });
                                row.col(|ui| {
                                    let mut enabled = block.enabled();
                                    let response = ui
                                        .add_enabled_ui(host.writable(), |ui| {
                                            paper_switch(ui, &mut enabled)
                                        })
                                        .inner
                                        .on_disabled_hover_text(host.blocked_reason());
                                    if response.changed() {
                                        host.set_block_enabled(document.id(), block.id(), enabled);
                                    }
                                });
                            });
                        }
                    });
                ui.visuals_mut().faint_bg_color = previous_faint_background;
            });
    }

    ui.add_space(10.0);
    let writable = host.writable();
    let selected_block_exists = host
        .selected_block()
        .is_some_and(|block_id| document.block(block_id).is_some());
    let retained_figure_available = host.retained_figure_available();
    ui.horizontal_wrapped(|ui| {
        let add = Button::new("Add element…")
            .enabled(writable)
            .show(ui)
            .on_disabled_hover_text(host.blocked_reason());
        if add.clicked() {
            host.add_element();
        }
        let remove = Button::new("Remove")
            .enabled(writable && selected_block_exists)
            .show(ui)
            .on_disabled_hover_text(if writable {
                "Select a page element to remove."
            } else {
                host.blocked_reason()
            });
        if remove.clicked() {
            host.remove_element();
        }
        let insert = Button::new("Insert result document…")
            .enabled(writable && retained_figure_available)
            .show(ui)
            .on_disabled_hover_text(if writable {
                "Create or retain a result document before inserting it into this report page."
            } else {
                host.blocked_reason()
            });
        if insert.clicked() {
            host.insert_result();
        }
    });
    let banner = egui::Frame::new()
        .fill(PAPER)
        .outer_margin(egui::Margin::same(8))
        .inner_margin(egui::Margin::same(8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            paper_label(
                ui,
                "Release closure validates this artifact against exact immutable result revisions and dataset bindings before building a signed package. Report elements may state blocked gates but cannot change source-owned gate state.",
                theme::sans(tokens::FS_1, FontWeight::Regular),
                PAPER_TEXT,
            );
        });
    paint_dashed_rect(ui, banner.response.rect, PAPER_BORDER);
}

pub fn report_block_element_title(kind: &ReportBlockKind) -> Cow<'_, str> {
    match kind {
        ReportBlockKind::PlotFigure(block) => Cow::Borrowed(&block.caption),
        ReportBlockKind::DataTable(block) => Cow::Borrowed(&block.title),
        ReportBlockKind::Datasheet(block) => Cow::Borrowed(&block.title),
        ReportBlockKind::Requirements(block) => Cow::Borrowed(&block.title),
        ReportBlockKind::Specifications(block) => Cow::Borrowed(&block.title),
        ReportBlockKind::Prose(block) => {
            let (text, _) = bounded_text_preview(&block.markdown, 56);
            text
        }
        ReportBlockKind::ReviewNote(block) => Cow::Owned(format!("Review note · {}", block.author)),
        ReportBlockKind::Evidence(block) => Cow::Borrowed(&block.title),
    }
}

fn report_block_bound_source(kind: &ReportBlockKind) -> String {
    kind.reference().map_or_else(
        || "Authored report source".to_owned(),
        |reference| {
            format!(
                "{} · {}",
                report_source_label(&reference.snapshot().source),
                if reference.is_frozen() {
                    "frozen"
                } else {
                    "linked"
                }
            )
        },
    )
}

fn report_block_state(host: &impl PreviewHost, block: &ReportBlock) -> (&'static str, Color32) {
    if !block.enabled() {
        ("excluded", PAPER_FAINT)
    } else if block
        .kind()
        .reference()
        .is_some_and(ReportReferenceMode::is_frozen)
    {
        ("frozen", Color32::from_rgb(72, 122, 78))
    } else if block
        .kind()
        .reference()
        .is_some_and(|reference| !host.reference_resolves(reference))
    {
        ("source missing", Color32::from_rgb(177, 64, 52))
    } else if block.kind().reference().is_some() {
        ("bound", Color32::from_rgb(72, 122, 78))
    } else {
        ("authored", PAPER_MUTED)
    }
}

fn bounded_text_preview(value: &str, maximum_characters: usize) -> (Cow<'_, str>, bool) {
    value.char_indices().nth(maximum_characters).map_or_else(
        || (Cow::Borrowed(value), false),
        |(byte_index, _)| {
            let mut preview = String::with_capacity(byte_index.saturating_add(1));
            preview.push_str(&value[..byte_index]);
            preview.push('…');
            (Cow::Owned(preview), true)
        },
    )
}

fn report_source_label(source: &ReportSourceId) -> String {
    match source {
        ReportSourceId::VisualizationDocument { document_id } => {
            format!("visualization {document_id}")
        }
        ReportSourceId::Dataset { dataset_id } => format!("dataset {dataset_id}"),
        ReportSourceId::VerificationEvidence { evidence_id } => {
            format!("verification evidence {evidence_id}")
        }
        ReportSourceId::ExternalRecord { namespace, key } => {
            format!("external {namespace}:{key}")
        }
    }
}

fn report_block_kind_label(kind: &ReportBlockKind) -> &'static str {
    match kind {
        ReportBlockKind::PlotFigure(_) => "PLOT FIGURE",
        ReportBlockKind::DataTable(_) => "DATA TABLE",
        ReportBlockKind::Datasheet(_) => "DATASHEET",
        ReportBlockKind::Requirements(_) => "REQUIREMENTS",
        ReportBlockKind::Specifications(_) => "SPECIFICATIONS",
        ReportBlockKind::Prose(_) => "PROSE",
        ReportBlockKind::ReviewNote(_) => "REVIEW NOTE",
        ReportBlockKind::Evidence(_) => "EVIDENCE",
    }
}

fn paper_label(ui: &mut Ui, text: &str, font: egui::FontId, color: Color32) {
    ui.add(
        egui::Label::new(egui::RichText::new(text).font(font).color(color))
            .wrap()
            .selectable(true),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn prose_preview_is_unicode_safe_and_bounded() {
        const MAXIMUM_CHARACTERS: usize = 4_096;
        let exact = "a".repeat(MAXIMUM_CHARACTERS);
        let (preview, truncated) = bounded_text_preview(&exact, MAXIMUM_CHARACTERS);
        assert!(!truncated);
        assert_eq!(preview, exact);

        let oversized = format!("{}é-tail", exact);
        let (preview, truncated) = bounded_text_preview(&oversized, MAXIMUM_CHARACTERS);
        assert!(truncated);
        assert_eq!(preview.chars().count(), MAXIMUM_CHARACTERS + 1);
        assert!(preview.ends_with('…'));
    }
}
