//! Output profile controls and release handoff over an application-owned report.
use super::{PaneSeparators, paint_pane_separators, paper_switch};
use egui::{ScrollArea, Stroke, Ui, Vec2};
use rspice_app_types::product::ResultDocumentId;
use rspice_results::report_document::{
    ReportDocument, ReportDraftMarking, ReportOutputFormats, ReportPageNumbering,
    ReportPublicationPageSize, ReportPublicationProfile, ReportPublicationTemplate,
    ReportTablePrecision,
};
use rspice_ui_kit::{
    panels::{code_inspector_property_list, code_inspector_section, property_row},
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
    let mut open_package_assembly = false;
    let pane = egui::Frame::new().fill(t.color.bg_panel).show(ui, |ui| {
        ui.set_min_size(Vec2::new(width.max(1.0), height.max(1.0)));
        ScrollArea::vertical()
            .id_salt("report-authoring.inspector")
            .show(ui, |ui| {
                code_inspector_section(ui, "Output formats", None, |ui| {
                    code_inspector_property_list(ui, |ui| {
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
                            "Canonical JSON",
                            &mut output_formats.canonical_json,
                            writable && (!canonical_json_enabled || enabled_count > 1),
                            writable && canonical_json_enabled && enabled_count == 1,
                            blocked_reason,
                        );
                        changed |= inspector_switch_row(
                            ui,
                            "Selected CSV",
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
                code_inspector_section(ui, "Publication", None, |ui| {
                    code_inspector_property_list(ui, |ui| {
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
                code_inspector_section(ui, "Release handoff", None, |ui| {
                    code_inspector_property_list(ui, |ui| {
                        let (bound_result, exact_binding) = host.bound_result_label(document);
                        property_row(ui, "Artifact identity", &document.id().to_string());
                        property_row(ui, "Bound result", &bound_result);
                        property_row(ui, "Candidate", "Not attached");
                        property_row(
                            ui,
                            "Compatibility",
                            if exact_binding {
                                "awaiting candidate review"
                            } else {
                                "result binding incomplete"
                            },
                        );
                        property_row(ui, "Package owner", "Release closure");
                        property_row(ui, "Physical DRC", "No retained DRC evidence");
                    });
                    release_handoff_card(
                        ui,
                        document,
                        host.bound_result_label(document).1,
                        &mut open_release_owner,
                    );
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.add_space(10.0);
                        if Button::new("Open package assembly").show(ui).clicked() {
                            open_package_assembly = true;
                        }
                    });
                    ui.add_space(8.0);
                });
                if let Some(error) = host.transaction_error() {
                    ui.add_space(8.0);
                    ui.colored_label(t.color.err, error);
                    ui.add_space(8.0);
                }
            });
    });
    paint_pane_separators(ui, pane.response.rect, separators, t.color.border);
    if let Some(setting) = pending_edit {
        host.commit_publication(document.id(), setting);
    }
    if open_release_owner || open_package_assembly {
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
    ui.horizontal(|ui| {
        ui.set_width(ui.available_width());
        ui.add_space(10.0);
        ui.label(
            egui::RichText::new(label)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_space(10.0);
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
    ui.horizontal(|ui| {
        ui.add_space(10.0);
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
        ui.add_space(10.0);
        let width = (ui.available_width() - 20.0).max(80.0);
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

fn release_handoff_card(
    ui: &mut Ui,
    document: &ReportDocument,
    exact_result_binding: bool,
    open_owner: &mut bool,
) {
    let t = Tokens::get(ui.ctx());
    let shown = egui::Frame::new()
        .fill(t.color.bg_panel_2)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(egui::Margin::symmetric(10, 8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("OWNED BY RELEASE CLOSURE")
                        .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                        .color(t.color.text_dim),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new(if exact_result_binding {
                            "ready for candidate review"
                        } else {
                            "binding incomplete"
                        })
                        .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                        .color(if exact_result_binding {
                            t.color.ok
                        } else {
                            t.color.err
                        }),
                    );
                });
            });
            ui.add_space(5.0);
            let id = document.id().to_string();
            ui.label(
                egui::RichText::new(format!(
                    "Attach report {} to a release candidate",
                    id.get(..8).unwrap_or(&id)
                ))
                .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                .color(t.color.text),
            );
            ui.label(
                egui::RichText::new(
                    "Compatibility review only; packaging and promotion remain external.",
                )
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
            );
            ui.add_space(6.0);
            let button_width = ui.available_width();
            if Button::new("Open owner →")
                .min_width(button_width)
                .show(ui)
                .clicked()
            {
                *open_owner = true;
            }
        });
    ui.painter().hline(
        shown.response.rect.x_range(),
        shown.response.rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
}
