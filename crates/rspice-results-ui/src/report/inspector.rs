//! Output profile controls and release handoff over an application-owned report.
use super::{PaneSeparators, paint_pane_separators, paper_switch};
use egui::{ScrollArea, Ui, Vec2};
use rspice_app_types::product::ResultDocumentId;
use rspice_results::report_document::{
    ReportDocument, ReportDraftMarking, ReportOutputFormats, ReportPageNumbering,
    ReportPublicationPageSize, ReportPublicationProfile, ReportPublicationTemplate,
    ReportTablePrecision,
};
use rspice_ui_kit::{
    panels::{
        Problem, code_inspector_section, panel_body, panel_inset, problem_line,
        property_row_problem, section_block_gap,
    },
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::{Button, select},
};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentPublicationEdit {
    OutputFormats(ReportOutputFormats),
    PublicationProfile(ReportPublicationProfile),
}
pub trait InspectorHost {
    fn writable(&self) -> bool;
    fn blocked_reason(&self) -> &'static str;
    fn bound_result_label(&self, document: &ReportDocument) -> (String, bool);
    fn transaction_error(&self) -> Option<&str>;
    fn commit_publication(&mut self, document: ResultDocumentId, setting: DocumentPublicationEdit);
    fn open_release(&mut self);
}

pub fn show(
    ui: &mut Ui,
    host: &mut impl InspectorHost,
    document: &ReportDocument,
    separators: PaneSeparators,
) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let height = ui.available_height();
    let writable = host.writable();
    let blocked_reason = host.blocked_reason();
    let mut pending_edit = None;
    let mut open_release_owner = false;
    let pane = egui::Frame::new().fill(t.color.bg_panel).show(ui, |ui| {
        ui.set_min_size(Vec2::new(width.max(1.0), height.max(1.0)));
        // One body around every section, so they stack on the section gap
        // rather than on the scroll area's item spacing.
        ScrollArea::vertical()
            .id_salt("report-authoring.inspector")
            .auto_shrink([false, true])
            .show(ui, |ui| {
                panel_body(ui, |ui| {
                    code_inspector_section(ui, "Output formats", None, None, |ui| {
                        ui.scope(|ui| {
                            let mut output_formats = document.output_formats();
                            let enabled_count = [
                                output_formats.pdf_a,
                                output_formats.html_bundle,
                                output_formats.canonical_json,
                                output_formats.selected_csv,
                            ]
                            .into_iter()
                            .filter(|enabled| *enabled)
                            .count();
                            let pdf_a_enabled = output_formats.pdf_a;
                            let html_bundle_enabled = output_formats.html_bundle;
                            let canonical_json_enabled = output_formats.canonical_json;
                            let selected_csv_enabled = output_formats.selected_csv;
                            let mut changed = false;
                            changed |= inspector_switch_row(
                                ui,
                                "PDF/A",
                                &mut output_formats.pdf_a,
                                writable && (!pdf_a_enabled || enabled_count > 1),
                                writable && pdf_a_enabled && enabled_count == 1,
                                blocked_reason,
                            );
                            changed |= inspector_switch_row(
                                ui,
                                "HTML bundle",
                                &mut output_formats.html_bundle,
                                writable && (!html_bundle_enabled || enabled_count > 1),
                                writable && html_bundle_enabled && enabled_count == 1,
                                blocked_reason,
                            );
                            changed |= inspector_switch_row(
                                ui,
                                "JSON data",
                                &mut output_formats.canonical_json,
                                writable && (!canonical_json_enabled || enabled_count > 1),
                                writable && canonical_json_enabled && enabled_count == 1,
                                blocked_reason,
                            );
                            changed |= inspector_switch_row(
                                ui,
                                "CSV tables",
                                &mut output_formats.selected_csv,
                                writable && (!selected_csv_enabled || enabled_count > 1),
                                writable && selected_csv_enabled && enabled_count == 1,
                                blocked_reason,
                            );
                            if changed {
                                pending_edit =
                                    Some(DocumentPublicationEdit::OutputFormats(output_formats));
                            }
                        });
                    });
                    code_inspector_section(ui, "Publication", None, None, |ui| {
                        ui.scope(|ui| {
                            let mut profile = document.publication_profile();
                            let mut changed = false;
                            changed |= inspector_publication_select(
                                ui,
                                "report-publication-template",
                                "Template",
                                &mut profile.template,
                                &[
                                    (
                                        ReportPublicationTemplate::OrganizationVerificationReport,
                                        "Organization verification report",
                                    ),
                                    (
                                        ReportPublicationTemplate::CustomerDatasheet,
                                        "Customer datasheet",
                                    ),
                                    (
                                        ReportPublicationTemplate::InternalReviewMemo,
                                        "Internal review memo",
                                    ),
                                ],
                                writable,
                                blocked_reason,
                            );
                            changed |= inspector_publication_select(
                                ui,
                                "report-publication-page-size",
                                "Page size",
                                &mut profile.page_size,
                                &[
                                    (ReportPublicationPageSize::A4Portrait, "A4 portrait"),
                                    (
                                        ReportPublicationPageSize::UsLetterPortrait,
                                        "US Letter portrait",
                                    ),
                                    (ReportPublicationPageSize::A3Landscape, "A3 landscape"),
                                ],
                                writable,
                                blocked_reason,
                            );
                            changed |= inspector_publication_select(
                                ui,
                                "report-publication-draft-marking",
                                "Draft marking",
                                &mut profile.draft_marking,
                                &[
                                    (
                                        ReportDraftMarking::WatermarkWhileGatesOpen,
                                        "Watermark while gates are open",
                                    ),
                                    (ReportDraftMarking::NeverWatermark, "Never watermark"),
                                ],
                                writable,
                                blocked_reason,
                            );
                            changed |= inspector_publication_select(
                                ui,
                                "report-publication-numbering",
                                "Numbering",
                                &mut profile.numbering,
                                &[
                                    (
                                        ReportPageNumbering::SectionPageOfTotal,
                                        "Section · page of total",
                                    ),
                                    (
                                        ReportPageNumbering::ContinuousPageNumbers,
                                        "Continuous page numbers",
                                    ),
                                ],
                                writable,
                                blocked_reason,
                            );
                            changed |= inspector_publication_select(
                                ui,
                                "report-publication-precision",
                                "Precision in tables",
                                &mut profile.table_precision,
                                &[
                                    (
                                        ReportTablePrecision::SevenSignificantDigits,
                                        "7 significant digits",
                                    ),
                                    (ReportTablePrecision::FullStoredF64, "Full stored f64"),
                                    (
                                        ReportTablePrecision::MatchSourceDisplay,
                                        "Match source display",
                                    ),
                                ],
                                writable,
                                blocked_reason,
                            );
                            if changed {
                                pending_edit =
                                    Some(DocumentPublicationEdit::PublicationProfile(profile));
                            }
                        });
                    });
                    // Which run the report reads, and a mark when that
                    // binding is incomplete. "Candidate", "Package owner" and
                    // "Physical DRC" were constants — the last of them for a
                    // check RSpice does not run — and the report's own UUID
                    // is not the reader's. Both buttons called the same
                    // thing, so there is one.
                    code_inspector_section(ui, "Release", None, None, |ui| {
                        ui.scope(|ui| {
                            let (bound_result, exact_binding) = host.bound_result_label(document);
                            property_row_problem(
                                ui,
                                "Bound result",
                                &bound_result,
                                (!exact_binding).then_some(Problem::Warning),
                            );
                        });
                        section_block_gap(ui);
                        if Button::new("Release…").show(ui).clicked() {
                            open_release_owner = true;
                        }
                    });
                    if let Some(error) = host.transaction_error() {
                        problem_line(ui, Problem::Failure, error);
                    }
                })
            });
    });
    paint_pane_separators(ui, pane.response.rect, separators, t.color.border);
    if let Some(setting) = pending_edit {
        host.commit_publication(document.id(), setting);
    }
    if open_release_owner {
        host.open_release();
    }
}

fn inspector_switch_row(
    ui: &mut Ui,
    label: &str,
    value: &mut bool,
    enabled: bool,
    protected_last_output: bool,
    blocked_reason: &str,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let before = *value;
    // Read before the row is allocated: on the body's edge the body has
    // already inset the row on both sides.
    let inset = panel_inset(ui);
    ui.horizontal(|ui| {
        ui.set_width(ui.available_width());
        ui.add_space(inset);
        ui.label(
            egui::RichText::new(label)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_space(inset);
            let response = ui
                .add_enabled_ui(enabled, |ui| paper_switch(ui, value))
                .inner;
            if !enabled {
                response.on_disabled_hover_text(if protected_last_output {
                    "At least one report output format must remain enabled."
                } else {
                    blocked_reason
                });
            }
        });
    });
    *value != before
}

fn inspector_publication_select<T>(
    ui: &mut Ui,
    id: &'static str,
    label: &str,
    value: &mut T,
    options: &[(T, &'static str)],
    enabled: bool,
    blocked_reason: &str,
) -> bool
where
    T: Copy + PartialEq,
{
    let t = Tokens::get(ui.ctx());
    let selected = options
        .iter()
        .position(|(candidate, _)| candidate == value)
        .unwrap_or(0);
    let inset = panel_inset(ui);
    ui.horizontal(|ui| {
        ui.add_space(inset);
        ui.label(
            egui::RichText::new(label)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        );
    });
    let labels = options
        .iter()
        .map(|(_, label)| (*label).to_owned())
        .collect::<Vec<_>>();
    let mut selected_index = None;
    ui.horizontal(|ui| {
        ui.add_space(inset);
        let width = (ui.available_width() - inset).max(80.0);
        let output = ui.add_enabled_ui(enabled, |ui| {
            select(ui, id, label, options[selected].1, &labels, width)
        });
        if !enabled {
            output.response.on_disabled_hover_text(blocked_reason);
        }
        selected_index = output.inner;
    });
    ui.add_space(5.0);
    if let Some(index) = selected_index.filter(|index| *index < options.len()) {
        let next = options[index].0;
        if *value != next {
            *value = next;
            return true;
        }
    }
    false
}
