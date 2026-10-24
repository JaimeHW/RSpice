//! The Netlist workspace's inspector: the deck's diagnostics and provenance,
//! and the ownership of an automation document.

use egui::{Color32, Pos2, Rect, Sense, Stroke, Ui, Vec2};
use rspice_ui_kit::panels::inspector::muted_inspector_copy;
use rspice_ui_kit::panels::{Problem, code_inspector_section, panel_inset, section_header};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::{AppState, MessageId, RSpiceApp};

use super::super::super::design_system::{WorkbenchIcon, property_row, property_row_problem};

pub(in crate::workbench) fn code_workspace_inspector(ui: &mut Ui, app: &mut RSpiceApp) {
    if app.state.code.code_workspace.page
        == crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist
    {
        netlist(ui, app);
        return;
    }

    let messages = app.state.workbench.messages();
    let Some(context) = crate::workbench::commands::code_context::resolve(app) else {
        muted_inspector_copy(
            ui,
            &messages.text(MessageId::CodeInspectorContextUnavailable),
        );
        return;
    };
    let ownership = messages.text(match context.ownership {
        crate::workbench::commands::code_context::CodeDocumentOwnership::GeneratedReadOnly => {
            MessageId::CodeInspectorGeneratedReadOnly
        }
        crate::workbench::commands::code_context::CodeDocumentOwnership::ProjectOwned => {
            MessageId::CodeInspectorProjectOwned
        }
        crate::workbench::commands::code_context::CodeDocumentOwnership::ExternalReadOnly => {
            MessageId::CodeInspectorExternalReadOnly
        }
        crate::workbench::commands::code_context::CodeDocumentOwnership::ComparisonReadOnly => {
            MessageId::CodeInspectorComparisonReadOnly
        }
        crate::workbench::commands::code_context::CodeDocumentOwnership::GovernedReadOnly => {
            MessageId::CodeInspectorGovernedReadOnly
        }
    });
    // Whether this document can be edited, and nothing else. The page tab
    // names the page, the document tab names the path, and the toolbar's own
    // buttons are the one true statement of which commands are available:
    // five rows reading "available" were that statement copied out by hand.
    property_row(
        ui,
        &messages.text(MessageId::CodeInspectorOwnership),
        &ownership,
    );
}

fn netlist(ui: &mut Ui, app: &mut RSpiceApp) {
    use crate::workbench::documents::netlist_document::{
        ActiveNetlistDocument, DiagnosticSeverity,
    };

    let messages = app.state.workbench.messages();
    const DIAGNOSTIC_PREVIEW_LIMIT: usize = 20;
    let diagnostics = std::sync::Arc::clone(&app.state.code.netlist.diagnostics);
    let summary = diagnostics.summary();

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
                    diagnostic_row(ui, diagnostic);
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
                    diagnostic_row(ui, diagnostic);
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

    // The sandbox itself, rather than a paragraph recommending it.
    let tuning = crate::workbench::commands::vocabulary::Command::VerificationPage(
        crate::workbench::state::VerificationPage::Tuning,
    );
    if let crate::workbench::framework::command_registry::CommandAvailability::Available =
        tuning.availability(app)
    {
        section_header(
            ui,
            &messages.text(crate::workbench::MessageId::NetlistParameterExploration),
            None,
        );
        if crate::ui::widgets::Button::new(
            &messages.text(crate::workbench::MessageId::NetlistTuneParameters),
        )
        .show(ui)
        .clicked()
        {
            tuning.execute(app);
        }
    }

    if crate::workbench::documents::netlist_document::active_dependency(&app.state).is_some() {
        dependency_provenance(ui, &app.state);
    } else {
        match app.state.code.netlist.active_document {
            ActiveNetlistDocument::OwnedSource => owned_source_provenance(ui, &app.state),
            ActiveNetlistDocument::RunSnapshot => {} // its run's, stated by the strip over the deck
            _ => generated_provenance(ui, &app.state),
        }
    }
}

fn diagnostic_row(ui: &mut Ui, diagnostic: &crate::state::netlist_diagnostics::Diagnostic) {
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
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            format!(
                "{}: {}. {location}",
                diagnostic_severity_name(diagnostic.severity),
                diagnostic.message
            ),
        )
    });
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );

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
fn generated_provenance(ui: &mut Ui, state: &AppState) {
    let source = state
        .model
        .netlist_artifacts
        .generated_document
        .as_ref()
        .and_then(|document| document.generated_artifact())
        .and_then(|artifact| artifact.source_map().first())
        .map(|entry| entry.view_identity().to_owned());
    if let Some(source) = source {
        property_row(ui, "Generated from", &source);
    }
    if let Some((label, problem)) = generated_state(state) {
        property_row_problem(ui, "State", label, Some(problem));
    }
}

/// Which file the owned deck lives in, and what is wrong with it.
///
/// The project revision, the document revision, the generated base and the
/// two digests were five counters and hashes with nothing a designer can do
/// about them; the document state reported success in catalog words. What is
/// left is the file and a marked line when the deck needs attention.
fn owned_source_provenance(ui: &mut Ui, state: &AppState) {
    if let Some(path) = state.model.workspace.content.netlist_source_path.as_ref() {
        property_row(ui, "File", &path.display().to_string());
    }
    let digest = crate::state::content_digest(&state.model.simulation.source.netlist_content);
    if let Some(label) = owned_source_state(state, digest) {
        // The same split the deck toolbar paints, and for the same reason:
        // not yet checked is where the deck is, not a verdict on it. A file
        // that changed underneath the editor is the one to act on.
        let problem = state
            .code
            .netlist
            .external_change
            .is_some()
            .then_some(Problem::Warning);
        property_row_problem(ui, "State", label, problem);
    }
}

fn dependency_provenance(ui: &mut Ui, state: &AppState) {
    let Some(dependency) = crate::workbench::documents::netlist_document::active_dependency(state)
    else {
        return;
    };
    let messages = state.workbench.messages();
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
    let owned = state
        .model
        .workspace
        .content
        .netlist_descriptor
        .as_ref()
        .and_then(|descriptor| descriptor.owned_include(dependency.locator().logical_identity()));
    property_row(
        ui,
        &messages.text(crate::workbench::MessageId::NetlistOwnership),
        &owned.map_or_else(
            || {
                messages.text(match dependency.authority() {
                    crate::state::DependencySourceAuthority::External => {
                        crate::workbench::MessageId::NetlistExternalReferenceReadOnly
                    }
                    crate::state::DependencySourceAuthority::Vendor => {
                        crate::workbench::MessageId::NetlistVendorSourceReadOnly
                    }
                    crate::state::DependencySourceAuthority::TechnologyPackage => {
                        crate::workbench::MessageId::NetlistTechnologyPackageReadOnly
                    }
                    crate::state::DependencySourceAuthority::StandardLibrary => {
                        crate::workbench::MessageId::NetlistStandardLibraryReadOnly
                    }
                })
            },
            |_| messages.text(crate::workbench::MessageId::NetlistProjectOwnedEditable),
        ),
    );
}

/// What is wrong with the generated deck, or nothing when it matches the
/// schematic. A deck that is current — validated or not — says nothing.
fn generated_state(state: &AppState) -> Option<(&'static str, Problem)> {
    let netlist = &state.model.netlist_artifacts;
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

fn owned_source_state(
    state: &AppState,
    digest: crate::product::ContentDigest,
) -> Option<&'static str> {
    crate::workbench::documents::netlist_document::owned_netlist_publication_state(state, digest)
        .label()
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
    match diagnostic.source_path.as_deref() {
        Some(path) if diagnostic.source_line.is_some() => {
            format!("{} · {location}", path.display())
        }
        Some(path) => path.display().to_string(),
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
