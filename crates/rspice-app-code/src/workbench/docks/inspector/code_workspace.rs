//! The Netlist workspace's inspector: the deck's diagnostics and provenance.

use egui::{Color32, Pos2, Rect, Sense, Stroke, Ui, Vec2};
use rspice_ui_kit::panels::{Problem, code_inspector_section, panel_inset, section_header};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::MessageCatalog;
use crate::workbench::documents::netlist_document::{
    NetlistDocumentState, active_deck_file_in, active_dependency_in,
};
use rspice_app_state::workbench::app_state::model::Model;

use super::super::super::design_system::{WorkbenchIcon, property_row, property_row_problem};

/// What the Netlist workspace's inspector reads: the model, the netlist
/// editor's state and the message catalogue.
#[derive(Clone, Copy)]
pub(crate) struct CodeInspector<'a> {
    pub(crate) model: &'a Model,
    pub(crate) netlist: &'a NetlistDocumentState,
    pub(crate) messages: MessageCatalog,
}

/// Draw the deck's inspector. Returns the finding whose row was pressed, which
/// the caller sends down the route a Problems row takes.
pub(in crate::workbench) fn code_workspace_inspector(
    ui: &mut Ui,
    reads: &CodeInspector<'_>,
) -> Option<uuid::Uuid> {
    use crate::workbench::documents::netlist_document::{
        ActiveNetlistDocument, DiagnosticSeverity,
    };

    let messages = reads.messages;
    const DIAGNOSTIC_PREVIEW_LIMIT: usize = 20;
    let diagnostics = std::sync::Arc::clone(&reads.netlist.diagnostics);
    let summary = diagnostics.summary();
    let mut pressed = None;

    if summary.current_errors > 0 {
        code_inspector_section(
            ui,
            &messages.text(crate::workbench::MessageId::NetlistErrors),
            Some(&summary.current_errors.to_string()),
            Some(Problem::Failure),
            |ui| {
                for diagnostic in diagnostics
                    .iter()
                    .filter(|diagnostic| {
                        diagnostic.is_current() && diagnostic.severity == DiagnosticSeverity::Error
                    })
                    .take(DIAGNOSTIC_PREVIEW_LIMIT)
                {
                    if diagnostic_row(ui, diagnostic) {
                        pressed = Some(diagnostic.canonical.diagnostic_id);
                    }
                }
                if summary.current_errors > DIAGNOSTIC_PREVIEW_LIMIT {
                    empty_diagnostic_row(
                        ui,
                        &messages.format(
                            crate::workbench::MessageId::NetlistMoreInProblems,
                            &[(
                                "count",
                                &(summary.current_errors - DIAGNOSTIC_PREVIEW_LIMIT).to_string(),
                            )],
                        ),
                    );
                }
            },
        );
    }

    // A document with nothing to advise shows no Advisories section. A
    // section whose whole body says it is empty is a heading spent on the
    // absence of a problem.
    let advisories = summary.current_advisories();
    if advisories > 0 {
        code_inspector_section(
            ui,
            &messages.text(crate::workbench::MessageId::NetlistAdvisories),
            Some(&advisories.to_string()),
            Some(Problem::Warning),
            |ui| {
                for diagnostic in diagnostics
                    .iter()
                    .filter(|diagnostic| {
                        diagnostic.is_current() && diagnostic.severity != DiagnosticSeverity::Error
                    })
                    .take(DIAGNOSTIC_PREVIEW_LIMIT)
                {
                    if diagnostic_row(ui, diagnostic) {
                        pressed = Some(diagnostic.canonical.diagnostic_id);
                    }
                }
                if advisories > DIAGNOSTIC_PREVIEW_LIMIT {
                    empty_diagnostic_row(
                        ui,
                        &messages.format(
                            crate::workbench::MessageId::NetlistMoreInProblems,
                            &[(
                                "count",
                                &(advisories - DIAGNOSTIC_PREVIEW_LIMIT).to_string(),
                            )],
                        ),
                    );
                }
            },
        );
    }

    if active_dependency_in(reads.model, reads.netlist).is_some() {
        dependency_provenance(ui, reads);
    } else {
        match reads.netlist.active_document {
            ActiveNetlistDocument::OwnedSource => deck_file_provenance(ui, reads),
            ActiveNetlistDocument::RunSnapshot => {} // its run's, stated by the strip over the deck
            _ => generated_provenance(ui, reads.model),
        }
    }
    pressed
}

/// One finding: its message, and the file and line it is on. Pressing it goes
/// there; returns whether it was pressed.
fn diagnostic_row(ui: &mut Ui, diagnostic: &crate::state::netlist_diagnostics::Diagnostic) -> bool {
    const ICON_COLUMN_W: f32 = 14.0;
    const COLUMN_GAP: f32 = 7.0;
    const PADDING_Y: f32 = 8.0;
    const DETAIL_GAP: f32 = 3.0;

    let t = Tokens::get(ui.ctx());
    let padding_x = panel_inset(ui);
    let width = ui.available_width().max(1.0);
    let text_width = (width - padding_x * 2.0 - ICON_COLUMN_W - COLUMN_GAP).max(1.0);
    let message_galley = ui.painter().layout(
        diagnostic.message.clone(),
        theme::sans(tokens::FS_0, FontWeight::Medium),
        t.color.text,
        text_width,
    );
    let location = diagnostic_location(diagnostic);
    let location_galley = ui.painter().layout(
        location.clone(),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
        text_width,
    );
    let message_height = message_galley.size().y;
    let height = PADDING_Y * 2.0 + message_height + DETAIL_GAP + location_galley.size().y;
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            format!(
                "{}: {}. {location}",
                diagnostic_severity_name(diagnostic.severity),
                diagnostic.message
            ),
        )
    });
    if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    rspice_ui_kit::theme::paint_focus_ring(ui, &response, rect);
    let response = response.on_hover_cursor(egui::CursorIcon::PointingHand);

    let icon = match diagnostic.severity {
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Hint
        | crate::workbench::documents::netlist_document::DiagnosticSeverity::Info => {
            WorkbenchIcon::Info
        }
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Warning
        | crate::workbench::documents::netlist_document::DiagnosticSeverity::Error => {
            WorkbenchIcon::Warning
        }
    };
    icon.paint(
        ui.painter(),
        Rect::from_min_size(
            Pos2::new(rect.left() + padding_x, rect.top() + PADDING_Y),
            Vec2::splat(ICON_COLUMN_W),
        ),
        diagnostic_tone(&t, diagnostic.severity),
    );

    let text_x = rect.left() + padding_x + ICON_COLUMN_W + COLUMN_GAP;
    ui.painter().galley(
        Pos2::new(text_x, rect.top() + PADDING_Y),
        message_galley,
        t.color.text,
    );
    ui.painter().galley(
        Pos2::new(text_x, rect.top() + PADDING_Y + message_height + DETAIL_GAP),
        location_galley,
        t.color.text_faint,
    );
    response.clicked()
}

fn empty_diagnostic_row(ui: &mut Ui, message: &str) {
    let faint = Tokens::get(ui.ctx()).color.text_faint;
    rspice_ui_kit::panels::inspector::toned_inspector_copy(ui, message, faint);
}

/// Where the generated deck came from, and what is wrong with it.
///
/// The input revision, the input digest and the netlist digest were three
/// counters and hashes a designer cannot act on; the generator's healthy
/// states reported success. What is left is the cellview that produced the
/// deck, and a marked line when the deck is not the one the schematic implies.
fn generated_provenance(ui: &mut Ui, model: &Model) {
    let source = model
        .netlist_artifacts
        .generated_document
        .as_ref()
        .and_then(|document| document.generated_artifact())
        .and_then(|artifact| artifact.source_map().first())
        .map(|entry| entry.view_identity().to_owned());
    if let Some(source) = source {
        property_row(ui, "Generated from", &source);
    }
    if let Some((label, problem)) = generated_state(model) {
        property_row_problem(ui, "State", label, Some(problem));
    }
}

/// The file in front, the deck or one of its includes: where it is, and the
/// encoding and line endings Save writes it back in.
fn deck_file_provenance(ui: &mut Ui, reads: &CodeInspector<'_>) {
    let Some(file) = active_deck_file_in(reads.netlist) else {
        return;
    };
    property_row(ui, "File", &file.path.display().to_string());
    property_row(ui, "Encoding", file.encoding.label());
    property_row(ui, "Line endings", file.line_ending.label());
}

fn dependency_provenance(ui: &mut Ui, reads: &CodeInspector<'_>) {
    let Some(dependency) = active_dependency_in(reads.model, reads.netlist) else {
        return;
    };
    let messages = reads.messages;
    section_header(
        ui,
        &messages.text(crate::workbench::MessageId::NetlistIncludeProvenance),
        None,
    );
    property_row(
        ui,
        &messages.text(crate::workbench::MessageId::NetlistDisplayName),
        dependency.locator().display_name(),
    );
    // Only worth a row when the deck asked for something other than what it
    // got: that difference is the thing a designer chases. When they agree the
    // row restates the display name directly above it.
    if dependency.requested_locator() != dependency.locator().display_name() {
        property_row(
            ui,
            &messages.text(crate::workbench::MessageId::NetlistRequestedAs),
            dependency.requested_locator(),
        );
    }
    // Where the bytes come from, and the one failure worth a mark here: an
    // include the deck names that nothing resolved to.
    property_row_problem(
        ui,
        &messages.text(crate::workbench::MessageId::NetlistOrigin),
        &if dependency.resolution().content_digest().is_none() {
            messages.text(crate::workbench::MessageId::NetlistUnresolved)
        } else {
            dependency
                .locator()
                .native_origin()
                .map(str::to_owned)
                .unwrap_or_else(|| {
                    messages.text(crate::workbench::MessageId::NetlistRetainedAuthenticatedSource)
                })
        },
        (dependency.resolution().content_digest().is_none()).then_some(Problem::Failure),
    );
    let relationship = messages.text(if dependency.direct_include_index().is_some() {
        crate::workbench::MessageId::NetlistDirectInclude
    } else {
        crate::workbench::MessageId::NetlistTransitiveInclude
    });
    property_row(
        ui,
        &messages.text(crate::workbench::MessageId::NetlistRelationship),
        &relationship,
    );
    if let Some(parent) = dependency.parent() {
        property_row(
            ui,
            &messages.text(crate::workbench::MessageId::NetlistParent),
            parent.display_name(),
        );
    }
}

/// What is wrong with the generated deck, or nothing when it matches the
/// schematic. A deck that is current — validated or not — says nothing.
fn generated_state(model: &Model) -> Option<(&'static str, Problem)> {
    let netlist = &model.netlist_artifacts;
    if netlist.generated_source.is_empty() {
        Some(("Not generated yet", Problem::Warning))
    } else if netlist.generation_error.is_some() {
        Some((
            "Could not generate; showing the last deck",
            Problem::Failure,
        ))
    } else if netlist.generated_input_digest != netlist.current_generation_input_digest {
        Some(("Older than the schematic", Problem::Warning))
    } else {
        None
    }
}

fn diagnostic_location(diagnostic: &crate::state::netlist_diagnostics::Diagnostic) -> String {
    let location = match (
        diagnostic.source_line.or(diagnostic.line),
        diagnostic.column,
    ) {
        (Some(line), Some(column)) => format!("line {} · column {}", line + 1, column + 1),
        (Some(line), None) => format!("line {}", line + 1),
        (None, _) => "document scope".to_owned(),
    };
    // The file is named the way its tab names it.
    let file = |path: &std::path::Path| {
        path.file_name().map_or_else(
            || path.display().to_string(),
            |name| name.to_string_lossy().into_owned(),
        )
    };
    match diagnostic.source_path.as_deref() {
        Some(path) if diagnostic.source_line.is_some() => format!("{} · {location}", file(path)),
        Some(path) => file(path),
        None => location,
    }
}

fn diagnostic_tone(
    tokens: &Tokens,
    severity: crate::workbench::documents::netlist_document::DiagnosticSeverity,
) -> Color32 {
    match severity {
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Hint => {
            tokens.color.text_dim
        }
        // An informational diagnostic is not a problem, so it is not news:
        // it reads as the hint beside it does. Warning and error keep their
        // tones because those are the two the reader must act on.
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Info => {
            tokens.color.text_dim
        }
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Warning => {
            tokens.color.warn
        }
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Error => {
            tokens.color.err
        }
    }
}

fn diagnostic_severity_name(
    severity: crate::workbench::documents::netlist_document::DiagnosticSeverity,
) -> &'static str {
    match severity {
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Hint => "Hint",
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Info => "Information",
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Warning => "Warning",
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Error => "Error",
    }
}

#[cfg(test)]
mod tests;
