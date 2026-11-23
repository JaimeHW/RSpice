//! The Netlist workspace's inspector: the findings of the text in front, and
//! where that text comes from: the deck's file, an include, the generated
//! netlist's design, the Added lines' design, or the run a run input is of.

use egui::{Color32, Pos2, Rect, Sense, Stroke, Ui, Vec2};
use rspice_ui_kit::panels::{Problem, code_inspector_section, panel_inset, section_header};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::documents::netlist_document::{
    NetlistDocumentState, active_deck_file_in, active_dependency_in,
};
use crate::workbench::{MessageCatalog, MessageId};
use rspice_app_state::workbench::app_state::model::Model;

use super::super::super::design_system::{
    Fact, WorkbenchIcon, fact_rows, paint_problem_mark, property_row, property_row_problem,
};

/// What the Netlist workspace's inspector reads: the model, the netlist
/// editor's state and the message catalogue.
#[derive(Clone, Copy)]
pub(crate) struct SourceInspector<'a> {
    pub(crate) model: &'a Model,
    pub(crate) netlist: &'a NetlistDocumentState,
    /// The deck's facts while the deck's own file is in front.
    pub(crate) deck_facts: Option<&'a crate::workbench::code::netlist_first::DeckFacts>,
    pub(crate) messages: MessageCatalog,
}

/// What the reader pressed on one of the inspector's findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::workbench) enum FindingPress {
    /// The finding's row, which goes to its line as a Problems row does.
    Row(uuid::Uuid),
    /// The fix the finding offers, applied as a Problems row applies it.
    Fix(uuid::Uuid),
}

/// Draw the deck's inspector. Returns what was pressed on a finding: its
/// row, which the caller sends down the route a Problems row takes, or the
/// fix it offers.
pub(in crate::workbench) fn source_editing_inspector(
    ui: &mut Ui,
    reads: &SourceInspector<'_>,
) -> Option<FindingPress> {
    use crate::workbench::documents::netlist_document::NetlistRoot;

    let messages = reads.messages;
    const DIAGNOSTIC_PREVIEW_LIMIT: usize = 20;
    let diagnostics = std::sync::Arc::clone(&reads.netlist.diagnostics);
    let summary = diagnostics.summary();
    let mut pressed = None;

    // Errors, then warnings, then notes, each its own section counting what
    // it lists, as the status bar counts them. A kind with nothing in it has
    // no section: a heading spent on the absence of a problem says nothing.
    let sections = [
        (
            MessageId::NetlistErrors,
            summary.current_errors,
            Some(Problem::Failure),
        ),
        (
            MessageId::NetlistAdvisories,
            summary.current_warnings,
            Some(Problem::Warning),
        ),
        (
            MessageId::NetlistNotes,
            summary.current_information + summary.current_hints,
            None,
        ),
    ];
    for (title, count, problem) in sections {
        if count == 0 {
            continue;
        }
        code_inspector_section(
            ui,
            &messages.text(title),
            Some(&count.to_string()),
            problem,
            |ui| {
                for diagnostic in diagnostics
                    .iter()
                    .filter(|diagnostic| {
                        diagnostic.is_current() && section_of(diagnostic.severity) == title
                    })
                    .take(DIAGNOSTIC_PREVIEW_LIMIT)
                {
                    if let Some(press) = diagnostic_row(ui, diagnostic, messages) {
                        pressed = Some(press);
                    }
                }
                if count > DIAGNOSTIC_PREVIEW_LIMIT {
                    empty_diagnostic_row(
                        ui,
                        &messages.format(
                            MessageId::NetlistMoreInProblems,
                            &[("count", &(count - DIAGNOSTIC_PREVIEW_LIMIT).to_string())],
                        ),
                    );
                }
            },
        );
    }

    if active_dependency_in(reads.model, reads.netlist).is_some() {
        dependency_provenance(ui, reads);
    } else {
        match &reads.netlist.front {
            crate::workbench::documents::netlist_document::NetlistTab::RunInput { run, point } => {
                run_input_facts(ui, reads, *run, *point);
            }
            crate::workbench::documents::netlist_document::NetlistTab::AddedLines => {
                added_lines_facts(ui, reads);
            }
            front => match front.root_kind() {
                NetlistRoot::Deck => deck_file_provenance(ui, reads),
                NetlistRoot::Comparison => comparison_facts(ui, reads),
                _ => generated_provenance(ui, reads.model, messages),
            },
        }
    }
    pressed
}

/// The inspector section a finding of `severity` is listed in, named by
/// its title: a note or a hint is not a warning.
fn section_of(
    severity: crate::workbench::documents::netlist_document::DiagnosticSeverity,
) -> MessageId {
    use crate::workbench::documents::netlist_document::DiagnosticSeverity;

    match severity {
        DiagnosticSeverity::Error => MessageId::NetlistErrors,
        DiagnosticSeverity::Warning => MessageId::NetlistAdvisories,
        DiagnosticSeverity::Info | DiagnosticSeverity::Hint => MessageId::NetlistNotes,
    }
}

/// One finding: its message, the file and line it is on, and the fix it
/// offers. Pressing the row goes there; what was pressed.
fn diagnostic_row(
    ui: &mut Ui,
    diagnostic: &crate::state::netlist_diagnostics::Diagnostic,
    messages: MessageCatalog,
) -> Option<FindingPress> {
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
    let location = diagnostic_location(diagnostic, messages);
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
                diagnostic_severity_name(diagnostic.severity, messages),
                diagnostic.message
            ),
        )
    });
    if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    rspice_ui_kit::theme::paint_focus_ring(ui, &response, rect);
    let response = response.on_hover_cursor(egui::CursorIcon::PointingHand);

    let mark = Rect::from_min_size(
        Pos2::new(rect.left() + padding_x, rect.top() + PADDING_Y),
        Vec2::splat(ICON_COLUMN_W),
    );
    match diagnostic.severity {
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Hint
        | crate::workbench::documents::netlist_document::DiagnosticSeverity::Info => {
            WorkbenchIcon::Info.paint(ui.painter(), mark, diagnostic_tone(&t, diagnostic.severity));
        }
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Warning => {
            paint_problem_mark(ui.painter(), mark, Problem::Warning);
        }
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Error => {
            paint_problem_mark(ui.painter(), mark, Problem::Failure);
        }
    }

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
    let id = diagnostic.canonical.diagnostic_id;
    let mut pressed = response.clicked().then_some(FindingPress::Row(id));
    // The fix a finding offers stands under it, as on its Problems row: a
    // current finding's only, since a fix reads the text it was found in.
    let mut bottom = rect.bottom();
    if let Some(fix) = diagnostic
        .is_current()
        .then(|| diagnostic.canonical.quick_fixes.first())
        .flatten()
    {
        let control_h = crate::ui::widgets::compact_height(&t.metrics);
        let (band, _) =
            ui.allocate_exact_size(Vec2::new(width, control_h + PADDING_Y), Sense::hover());
        let mut strip = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(Rect::from_min_max(
                    Pos2::new(text_x, band.top()),
                    Pos2::new(band.right() - padding_x, band.top() + control_h),
                ))
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        if crate::ui::widgets::Button::new(&fix.label)
            .compact()
            .show(&mut strip)
            .clicked()
        {
            pressed = Some(FindingPress::Fix(id));
        }
        bottom = band.bottom();
    }
    ui.painter()
        .hline(rect.x_range(), bottom, Stroke::new(1.0, t.color.border));
    pressed
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
fn generated_provenance(ui: &mut Ui, model: &Model, messages: MessageCatalog) {
    let source = model
        .netlist_artifacts
        .generated_document
        .as_ref()
        .and_then(|document| document.generated_artifact())
        .and_then(|artifact| artifact.source_map().first())
        .map(|entry| entry.view_identity().to_owned());
    if let Some(source) = source {
        property_row(ui, &messages.text(MessageId::NetlistGeneratedFrom), &source);
    }
    if let Some((label, problem)) = generated_state(model) {
        property_row_problem(
            ui,
            &messages.text(MessageId::NetlistState),
            &messages.text(label),
            Some(problem),
        );
    }
}

/// A run's input: the run, the point of it when it read several, whether
/// it ran a deck or the plan, and when it finished.
fn run_input_facts(ui: &mut Ui, reads: &SourceInspector<'_>, run: u64, point: Option<usize>) {
    use crate::workbench::MessageId;
    let messages = reads.messages;
    let simulation = &reads.model.simulation;
    section_header(
        ui,
        &messages.text(MessageId::NetlistInspectorRunInput),
        None,
    );
    property_row(
        ui,
        &messages.text(MessageId::NetlistInspectorRun),
        &run.to_string(),
    );
    let points = simulation
        .retained
        .executed_decks
        .get(run)
        .map_or(&[][..], |deck| deck.points.as_slice());
    if let Some(index) = point
        && points.len() > 1
        && let Some(of) = points.get(index)
    {
        property_row(
            ui,
            &messages.text(MessageId::NetlistInspectorPoint),
            &messages.format(
                MessageId::NetlistInspectorPointOf,
                &[
                    ("point", &(index + 1).to_string()),
                    ("count", &points.len().to_string()),
                    ("label", &of.label),
                ],
            ),
        );
    }
    let Some(record) = simulation
        .retained
        .runs
        .iter()
        .find(|record| record.id == run)
    else {
        return;
    };
    if let Some(receipt) = record.prepared_receipt() {
        // A run of a plan names the plan, while the project still has it.
        let ran = if receipt.source_domain() == crate::state::AnalysisResultSourceDomain::ManualDeck
        {
            messages.text(MessageId::NetlistInspectorRanDeck)
        } else {
            receipt
                .simulation_plan_id()
                .and_then(|id| plan_name(reads.model, id))
                .unwrap_or_else(|| messages.text(MessageId::NetlistInspectorRanPlan))
        };
        property_row(ui, &messages.text(MessageId::NetlistInspectorRan), &ran);
    }
    if record.elapsed_time > 0.0
        && let Some(finished) = local_time(record.timestamp + record.elapsed_time)
    {
        property_row(
            ui,
            &messages.text(MessageId::NetlistInspectorFinished),
            &finished,
        );
    }
}

/// The name of the plan `id`, the active one or one put away; `None` when
/// the project no longer has it.
fn plan_name(model: &Model, id: crate::product::SimulationPlanId) -> Option<String> {
    let setup = &model.sim_setup;
    if setup
        .stable_analysis_plan()
        .is_ok_and(|plan| plan.id() == id)
    {
        return Some(setup.active_plan_name().as_str().to_owned());
    }
    setup
        .inactive_plans()
        .iter()
        .find(|plan| plan.id() == id)
        .map(|plan| plan.name().as_str().to_owned())
}

/// An instant on the reader's clock, `2026-10-16 14:03:12`; `None` when the
/// clock did not know it.
fn local_time(unix_seconds: f64) -> Option<String> {
    if !(unix_seconds.is_finite() && unix_seconds > 0.0) {
        return None;
    }
    let local = crate::time_compat::local_date_time((unix_seconds * 1_000.0) as u64)?;
    Some(format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        local.year(),
        u8::from(local.month()),
        local.day(),
        local.hour(),
        local.minute(),
        local.second()
    ))
}

/// A comparison: the two texts it compares, by name, and how many lines
/// the second adds to the first and takes from it.
fn comparison_facts(ui: &mut Ui, reads: &SourceInspector<'_>) {
    use crate::workbench::MessageId;
    let Some((left, right)) = reads.netlist.comparison_sides else {
        return;
    };
    let messages = reads.messages;
    let name = |side| {
        crate::workbench::chrome::document_bar::code::compare_side_name(
            reads.model,
            reads.netlist,
            messages,
            side,
        )
    };
    section_header(
        ui,
        &messages.text(MessageId::NetlistNavigatorRevisionComparison),
        None,
    );
    property_row(
        ui,
        &messages.text(MessageId::NetlistInspectorCompareFrom),
        &name(left),
    );
    property_row(
        ui,
        &messages.text(MessageId::NetlistInspectorCompareTo),
        &name(right),
    );
    let (added, removed) = crate::workbench::documents::netlist_document::diff_line_counts(
        &reads.netlist.generated_diff_source,
    );
    let lines = |count: usize| {
        messages.format(
            if count == 1 {
                MessageId::NetlistInspectorLineCountSingular
            } else {
                MessageId::NetlistInspectorLineCount
            },
            &[("count", &count.to_string())],
        )
    };
    property_row(
        ui,
        &messages.text(MessageId::NetlistInspectorLinesAdded),
        &lines(added),
    );
    property_row(
        ui,
        &messages.text(MessageId::NetlistInspectorLinesRemoved),
        &lines(removed),
    );
}

/// The Added lines: the design they belong to, and the plan they run with,
/// by its name.
fn added_lines_facts(ui: &mut Ui, reads: &SourceInspector<'_>) {
    use crate::workbench::MessageId;
    let messages = reads.messages;
    section_header(
        ui,
        &messages.text(MessageId::NetlistAddedLinesDocument),
        None,
    );
    if let Some(design) =
        crate::workbench::documents::netlist_document::added_lines_design(reads.model)
    {
        property_row(
            ui,
            &messages.text(MessageId::NetlistInspectorDesign),
            &design.display_path(),
        );
    }
    property_row(
        ui,
        &messages.text(MessageId::NetlistInspectorRunsWith),
        reads.model.sim_setup.active_plan_name().as_str(),
    );
}

/// The file in front, the deck or one of its includes: where it is, and the
/// encoding and line endings Save writes it back in.
fn deck_file_provenance(ui: &mut Ui, reads: &SourceInspector<'_>) {
    let messages = reads.messages;
    let Some(file) = active_deck_file_in(reads.netlist) else {
        // A deck whose file is missing: the path it was last at, and that
        // it is missing.
        if let Some(missing) = crate::workbench::documents::deck_files::missing_deck_in_front(
            reads.model,
            reads.netlist,
        ) {
            property_row(
                ui,
                &messages.text(MessageId::CodeSourceFileFile),
                &missing.path.display().to_string(),
            );
            property_row_problem(
                ui,
                &messages.text(MessageId::NetlistState),
                &messages.text(MessageId::NetlistDeckMissingState),
                Some(Problem::Failure),
            );
        }
        return;
    };
    let encoding = messages.text(MessageId::NetlistEncoding);
    let line_endings = messages.text(MessageId::CodeSourceLineEndings);
    let line_ending = file.line_ending.label(messages);
    let save = [
        Fact::new(&encoding, file.encoding.label()),
        Fact::new(&line_endings, &line_ending),
    ];
    if let Some(facts) = reads.deck_facts {
        // The rows every deck page's inspector lists, then how Save writes
        // the file back, in the same list.
        crate::workbench::code::netlist_first::deck_fact_rows(ui, facts, &save, messages);
        return;
    }
    let file_label = messages.text(MessageId::CodeSourceFileFile);
    let path = file.path.display().to_string();
    let mut rows = vec![Fact::path(&file_label, &path)];
    rows.extend_from_slice(&save);
    fact_rows(ui, &rows);
}

fn dependency_provenance(ui: &mut Ui, reads: &SourceInspector<'_>) {
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
fn generated_state(model: &Model) -> Option<(MessageId, Problem)> {
    let netlist = &model.netlist_artifacts;
    if netlist.generated_source.is_empty() {
        Some((MessageId::NetlistNotGeneratedYet, Problem::Warning))
    } else if netlist.generation_error.is_some() {
        Some((MessageId::NetlistCouldNotGenerate, Problem::Failure))
    } else if netlist.generated_input_digest != netlist.current_generation_input_digest {
        Some((MessageId::NetlistOlderThanSchematic, Problem::Warning))
    } else {
        None
    }
}

fn diagnostic_location(
    diagnostic: &crate::state::netlist_diagnostics::Diagnostic,
    messages: MessageCatalog,
) -> String {
    let location = match (
        diagnostic.source_line.or(diagnostic.line),
        diagnostic.column,
    ) {
        (Some(line), Some(column)) => messages.format(
            MessageId::NetlistLineColumn,
            &[
                ("line", &(line + 1).to_string()),
                ("column", &(column + 1).to_string()),
            ],
        ),
        (Some(line), None) => messages.format(
            MessageId::NetlistNavigatorLine,
            &[("line", &(line + 1).to_string())],
        ),
        (None, _) => messages.text(MessageId::NetlistDocumentScope),
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
    messages: MessageCatalog,
) -> String {
    use crate::workbench::documents::netlist_document::DiagnosticSeverity;
    messages.text(match severity {
        DiagnosticSeverity::Hint => MessageId::SourceDiagnosticSeverityHint,
        DiagnosticSeverity::Info => MessageId::SourceDiagnosticSeverityInformation,
        DiagnosticSeverity::Warning => MessageId::SourceDiagnosticSeverityWarning,
        DiagnosticSeverity::Error => MessageId::SourceDiagnosticSeverityError,
    })
}

#[cfg(test)]
mod tests;
