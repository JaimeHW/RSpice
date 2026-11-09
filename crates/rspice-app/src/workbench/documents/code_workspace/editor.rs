//! Language-neutral owned-source editor used by the Verilog-A and Automation pages.
//!
//! It follows the mockup's fixed gutter and non-wrapping code geometry while
//! retaining ordinary `TextEdit` selection, clipboard, IME, and accessibility
//! semantics. Diagnostics are byte-ranged and share the same source identity as
//! the compile/validation receipt that produced them.

use egui::text::{LayoutJob, TextFormat};
use egui::{Color32, FontId, Stroke, Ui};

use crate::state::code_diagnostics::CodeDiagnosticCollection;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::Problem;
use crate::workbench::MessageId;
use crate::workbench::documents::text_editor_commands::{
    EditorSyntax, queue_caret_position, show_go_to_line, take_reveal_line,
    veriloga_syntax::{LineState, SyntaxClass, SyntaxSpan, VerilogAHighlighter, tokenize_line},
};

use crate::workbench::documents::editor_font;
use crate::workbench::framework::requests::StandardCommand;

mod assist;
mod findings;

use findings::{HeldFinding, HeldFindings};

/// The pop-ups beside an editor scale with the interface, not with its text.
const POPUP_FONT_SIZE: f32 = 11.0;
const CODE_LEFT_PADDING: f32 = 12.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeEditorLanguage {
    VerilogA,
    Automation,
    Python,
    Yaml,
    Toml,
}

pub use rspice_vocabulary::diagnostics::{
    DiagnosticSeverity as CodeEditorSeverity, SourceDiagnostic as CodeEditorDiagnostic,
};

#[derive(Debug, Clone)]
pub struct CodeDocumentInteraction {
    pub changed: bool,
    pub cursor_char_index: usize,
    pub selected_char_range: Option<(usize, usize)>,
    pub breakpoint_toggled: Option<usize>,
    /// The source well, for its right-click menu.
    pub response: egui::Response,
    /// What a step to the next or previous finding found, for the page to
    /// say on the status bar.
    pub announcement: Option<String>,
    /// Undo or Redo the editor's own history had no step for: the page steps
    /// the history of the edits it made across the bundle.
    pub unanswered_history: Option<StandardCommand>,
}

/// Where the text an editor shows stands with the last check of it.
#[derive(Debug, Clone, Copy)]
pub enum EditorFindings<'a> {
    /// A check of the very text the editor shows reported these.
    Current(&'a CodeDiagnosticCollection),
    /// The text changed after the last check. What that check found stays
    /// shown, dimmed as outdated, each finding moved through the edits.
    Outdated,
}

pub fn show_code_document_interaction_versioned(
    ui: &mut Ui,
    id: impl std::hash::Hash + std::fmt::Debug,
    source: &mut String,
    source_revision: u64,
    language: CodeEditorLanguage,
    findings: EditorFindings<'_>,
    diagnostic_path: Option<&str>,
    editable: bool,
    messages: crate::workbench::MessageCatalog,
) -> CodeDocumentInteraction {
    show_code_document_impl(
        ui,
        egui::Id::new(id),
        source,
        source_revision,
        language,
        findings,
        diagnostic_path,
        editable,
        messages,
        &[],
        None,
        false,
        &[],
    )
}

/// The Verilog-A source editor, with `include_files` the project files its
/// `` `include `` completion offers.
pub(crate) fn show_veriloga_document_versioned(
    ui: &mut Ui,
    id: impl std::hash::Hash + std::fmt::Debug,
    source: &mut String,
    source_revision: u64,
    language: CodeEditorLanguage,
    findings: EditorFindings<'_>,
    diagnostic_path: Option<&str>,
    editable: bool,
    messages: crate::workbench::MessageCatalog,
    include_files: &[String],
) -> CodeDocumentInteraction {
    show_code_document_impl(
        ui,
        egui::Id::new(id),
        source,
        source_revision,
        language,
        findings,
        diagnostic_path,
        editable,
        messages,
        &[],
        None,
        false,
        include_files,
    )
}

/// Debug-aware source editor with accessible, keyboard-neutral breakpoint
/// markers in the gutter and an exact current-statement projection.
pub fn show_code_document_with_debug_versioned(
    ui: &mut Ui,
    id: impl std::hash::Hash + std::fmt::Debug,
    source: &mut String,
    source_revision: u64,
    language: CodeEditorLanguage,
    findings: EditorFindings<'_>,
    diagnostic_path: Option<&str>,
    editable: bool,
    messages: crate::workbench::MessageCatalog,
    breakpoints: &[usize],
    current_line: Option<usize>,
) -> CodeDocumentInteraction {
    show_code_document_impl(
        ui,
        egui::Id::new(id),
        source,
        source_revision,
        language,
        findings,
        diagnostic_path,
        editable,
        messages,
        breakpoints,
        current_line,
        true,
        &[],
    )
}

/// Put the caret of the source editor `editor_id` on a one-based line and
/// column the next time it is drawn, scroll it into view and give it the
/// keyboard, so F8 steps on from there. Every way to a place in a source
/// document goes through here: a finding opened from Problems or the
/// inspector, and F8 and Shift+F8.
pub(crate) fn reveal_source_location(
    context: &egui::Context,
    editor_id: egui::Id,
    line: usize,
    column: usize,
) {
    queue_caret_position(context, editor_id, line, column);
    context.memory_mut(|memory| memory.request_focus(editor_id));
}

/// The character index a one-based line starts at in `source`, and its
/// width in characters without its line ending.
fn line_extent(source: &str, line: usize) -> (usize, usize) {
    let mut lines = source.split('\n');
    let start = lines
        .by_ref()
        .take(line.saturating_sub(1))
        .map(|text| text.chars().count() + 1)
        .sum::<usize>();
    let width = lines
        .next()
        .map_or(0, |text| text.trim_end_matches('\r').chars().count());
    (start, width)
}

/// The one-based line and column of a character index in `source`.
fn caret_place(source: &str, char_index: usize) -> (usize, usize) {
    source
        .chars()
        .take(char_index)
        .fold((1, 1), |(line, column), character| {
            if character == '\n' {
                (line + 1, 1)
            } else {
                (line, column + 1)
            }
        })
}

/// A step to the next or the previous finding in the focused source
/// document, from wherever its caret is, both wrapping at the document's
/// ends; or a question about the finding at the caret, which stays put.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProblemStep {
    Next,
    Previous,
    AtCaret,
}

/// The shortcuts and the palette raise a step; the page that draws the
/// focused source editor hands it over just before drawing it.
impl crate::workbench::framework::requests::Request for ProblemStep {
    const DRAIN: crate::workbench::framework::requests::Drain =
        crate::workbench::framework::requests::Drain::SourceEditor;
}

/// Hand `step` to the editor `editor_id`, which takes it the next time it is
/// drawn.
pub(crate) fn queue_problem_step(context: &egui::Context, editor_id: egui::Id, step: ProblemStep) {
    context.data_mut(|data| data.insert_temp(problem_step_id(editor_id), step));
    context.request_repaint();
}

/// The step queued for the editor `editor_id`, taken once.
pub(crate) fn take_problem_step(
    context: &egui::Context,
    editor_id: egui::Id,
) -> Option<ProblemStep> {
    let id = problem_step_id(editor_id);
    context.data_mut(|data| {
        let step = data.get_temp::<ProblemStep>(id);
        data.remove::<ProblemStep>(id);
        step
    })
}

fn problem_step_id(editor_id: egui::Id) -> egui::Id {
    editor_id.with("problem-step")
}

fn reported_caret_id(editor_id: egui::Id) -> egui::Id {
    editor_id.with("reported-caret")
}

/// The finding a step stopped on, and where it left the caret: a second
/// step from there moves on from it, even past findings at the same place.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LastProblemStep {
    index: usize,
    place: (usize, usize),
}

/// Which of `places` (one-based line and optional column, in source order)
/// a step from the caret at `caret_index` lands on in the editor
/// `editor_id`: the first finding after the caret, or the last before it,
/// each wrapping to the other end. The caller moves the caret there. A
/// question about the caret moves nowhere and lands on none.
pub(crate) fn problem_step_index(
    context: &egui::Context,
    editor_id: egui::Id,
    source: &str,
    places: &[(usize, Option<usize>)],
    caret_index: usize,
    step: ProblemStep,
) -> Option<usize> {
    let count = places.len();
    if count == 0 || step == ProblemStep::AtCaret {
        return None;
    }
    // Where the caret lands on a place: on its column, kept within the line,
    // or at the line's start when the finding names no column.
    let landing = |(line, column): (usize, Option<usize>)| {
        let (_, width) = line_extent(source, line);
        (line, column.map_or(1, |column| column.clamp(1, width + 1)))
    };
    let caret = caret_place(source, caret_index);
    let last_id = editor_id.with("last-problem-step");
    let last = context
        .data(|data| data.get_temp::<LastProblemStep>(last_id))
        .filter(|last| last.place == caret && last.index < count)
        .map(|last| last.index);
    let index = match (last, step) {
        (Some(index), ProblemStep::Next) => (index + 1) % count,
        (Some(index), ProblemStep::Previous) => (index + count - 1) % count,
        (None, ProblemStep::Next) => places
            .iter()
            .position(|place| landing(*place) > caret)
            .unwrap_or(0),
        (None, ProblemStep::Previous) => places
            .iter()
            .rposition(|place| landing(*place) < caret)
            .unwrap_or(count - 1),
        (_, ProblemStep::AtCaret) => return None,
    };
    context.data_mut(|data| {
        data.insert_temp(
            last_id,
            LastProblemStep {
                index,
                place: landing(places[index]),
            },
        );
    });
    Some(index)
}

/// What a step says it reached: "Error 2 of 5, line 14: …".
pub(crate) fn problem_step_announcement(
    messages: crate::workbench::MessageCatalog,
    severity: CodeEditorSeverity,
    index: usize,
    count: usize,
    line: usize,
    message: &str,
) -> String {
    messages.format(
        MessageId::SourceDiagnosticStep,
        &[
            ("severity", &severity_title(severity, messages)),
            ("index", &(index + 1).to_string()),
            ("count", &count.to_string()),
            ("line", &line.to_string()),
            ("message", message),
        ],
    )
}

/// What "Show problem at caret" says of a finding: its severity, code,
/// place and message, and whether it was found before the latest edits.
pub(crate) fn problem_at_caret_announcement(
    messages: crate::workbench::MessageCatalog,
    severity: CodeEditorSeverity,
    code: &str,
    (line, column): (usize, usize),
    message: &str,
    outdated: bool,
) -> String {
    messages.format(
        if outdated {
            MessageId::SourceDiagnosticAtCaretOutdated
        } else {
            MessageId::SourceDiagnosticAtCaret
        },
        &[
            ("severity", &severity_title(severity, messages)),
            ("code", code),
            ("line", &line.to_string()),
            ("column", &column.to_string()),
            ("message", message),
        ],
    )
}

/// Move the caret of `editor_id` to the finding `step` reaches in this
/// document and say which it is, or say what the finding at the caret is.
fn step_to_problem(
    context: &egui::Context,
    editor_id: egui::Id,
    source: &str,
    findings: &HeldFindings,
    step: ProblemStep,
    messages: crate::workbench::MessageCatalog,
) -> String {
    let caret = context
        .data(|data| data.get_temp::<usize>(reported_caret_id(editor_id)))
        .unwrap_or(0);
    if step == ProblemStep::AtCaret {
        return findings.at_caret(caret).map_or_else(
            || messages.text(MessageId::SourceDiagnosticNoneAtCaret),
            |finding| {
                let (line, column) = findings.line_column(finding.range.start);
                problem_at_caret_announcement(
                    messages,
                    finding.severity,
                    &finding.code,
                    (line + 1, column + 1),
                    &finding.message,
                    findings.is_outdated(),
                )
            },
        );
    }
    let located = findings.iter().collect::<Vec<_>>();
    let places = located
        .iter()
        .map(|finding| {
            let (line, column) = findings.line_column(finding.range.start);
            (line + 1, Some(column + 1))
        })
        .collect::<Vec<_>>();
    let Some(index) = problem_step_index(context, editor_id, source, &places, caret, step) else {
        return messages.text(MessageId::SourceDiagnosticNone);
    };
    let (line, column) = places[index];
    reveal_source_location(context, editor_id, line, column.unwrap_or(1));
    let finding = located[index];
    problem_step_announcement(
        messages,
        finding.severity,
        index,
        located.len(),
        line,
        &finding.message,
    )
}

fn show_code_document_impl(
    ui: &mut Ui,
    editor_id: egui::Id,
    source: &mut String,
    source_revision: u64,
    language: CodeEditorLanguage,
    findings: EditorFindings<'_>,
    diagnostic_path: Option<&str>,
    editable: bool,
    messages: crate::workbench::MessageCatalog,
    breakpoints: &[usize],
    current_line: Option<usize>,
    breakpoints_enabled: bool,
    include_files: &[String],
) -> CodeDocumentInteraction {
    // The findings shown are the last check's, kept through the edits made
    // since it ran.
    let held_id = editor_id.with("held-findings");
    let mut held = ui
        .ctx()
        .data_mut(|data| data.remove_temp::<HeldFindings>(held_id))
        .unwrap_or_default();
    match findings {
        EditorFindings::Current(diagnostics) => held.take_current(
            source,
            source_revision,
            std::ptr::from_ref(diagnostics).addr(),
            &diagnostics.view(diagnostic_path),
        ),
        EditorFindings::Outdated => held.keep_through_edits(source, source_revision),
    }
    // Before the reveal is taken, so the step lands in this frame.
    let announcement = take_problem_step(ui.ctx(), editor_id)
        .map(|step| step_to_problem(ui.ctx(), editor_id, source, &held, step, messages));
    let revealed = take_reveal_line(ui, editor_id, source);
    let requested_line = revealed.or_else(|| show_go_to_line(ui, editor_id, source));
    let tokens = Tokens::get(ui.ctx());
    let colors = tokens.color;
    let text_size = editor_font::text_size(ui.ctx());
    let font = theme::mono(text_size, FontWeight::Regular);
    let syntax = match language {
        CodeEditorLanguage::VerilogA => EditorSyntax::VerilogA,
        CodeEditorLanguage::Automation | CodeEditorLanguage::Python => EditorSyntax::Python,
        CodeEditorLanguage::Yaml => EditorSyntax::Yaml,
        CodeEditorLanguage::Toml => EditorSyntax::Toml,
    };
    let edited_lines = std::collections::HashSet::new();
    let accessible_label = messages.text(match (language, editable) {
        (CodeEditorLanguage::VerilogA, true) => MessageId::EditorVerilogASourceEditor,
        (CodeEditorLanguage::VerilogA, false) => MessageId::EditorVerilogASourceViewer,
        (CodeEditorLanguage::Automation, true) => MessageId::EditorAutomationSourceEditor,
        (CodeEditorLanguage::Automation, false) => MessageId::EditorAutomationSourceViewer,
        (CodeEditorLanguage::Python, true) => MessageId::EditorPythonSourceEditor,
        (CodeEditorLanguage::Python, false) => MessageId::EditorPythonSourceViewer,
        (CodeEditorLanguage::Yaml, true) => MessageId::EditorRunPlanEditor,
        (CodeEditorLanguage::Yaml, false) => MessageId::EditorRunPlanViewer,
        (CodeEditorLanguage::Toml, true) => MessageId::EditorTomlManifestEditor,
        (CodeEditorLanguage::Toml, false) => MessageId::EditorTomlManifestViewer,
    });
    let style = crate::workbench::documents::virtual_text_editor::VirtualEditorStyle {
        accessible_label: &accessible_label,
        messages,
        font: font.clone(),
        line_height: editor_font::line_height(text_size),
        gutter_width: editor_font::gutter_width(text_size),
        code_left_padding: CODE_LEFT_PADDING,
        top_padding: 8.0,
        background: colors.bg_inset,
        hover_background: colors.bg_hover,
        active_background: colors.syntax.current_line,
        selection_background: colors.syntax.selection,
        text: colors.text,
        text_dim: colors.text_dim,
        text_faint: colors.text_faint,
        border: colors.border,
        accent: colors.accent,
        error: colors.err,
        warning: colors.warn,
        edited_lines: &edited_lines,
        breakpoints,
        breakpoints_enabled,
        current_line,
        cross_probe_line: None,
        marks: held.marks(),
        marks_outdated: held.is_outdated(),
    };
    // The reading of each line is kept between frames, so a frame reads
    // only the lines it shows.
    let highlighter_id = editor_id.with("veriloga-syntax");
    let mut highlighter = (language == CodeEditorLanguage::VerilogA).then(|| {
        ui.ctx()
            .data_mut(|data| data.remove_temp::<VerilogAHighlighter>(highlighter_id))
            .unwrap_or_default()
    });
    let assist_keys = (language == CodeEditorLanguage::VerilogA)
        .then(|| assist::consume_keys(ui, editor_id, editable));
    let output = crate::workbench::documents::virtual_text_editor::show_virtual_text_editor(
        ui,
        editor_id,
        source,
        source_revision,
        editable,
        syntax,
        requested_line,
        &style,
        |ui, row, font| {
            let job = match highlighter.as_mut() {
                Some(highlighter) => {
                    let (spans, line) = highlighter.line(row.document, row.logical_line);
                    veriloga_row_job(&spans, &line, row.text, row.first_column, font, &colors)
                }
                None => layout_job(row.text, language, font, &[], &colors),
            };
            ui.fonts_mut(|fonts| fonts.layout_job(job))
        },
        |logical_line| {
            held.worst_on_line(logical_line)
                .and_then(|finding| diagnostic_problem(finding.severity))
        },
        |logical_line| {
            held.worst_on_line(logical_line)
                .map(|finding| line_diagnostic_description(finding, messages))
        },
    );
    let assist_answer = assist_keys.map(|keys| {
        assist::show(
            ui,
            editor_id,
            &assist::AssistFrame {
                source,
                source_revision,
                editable,
                include_files,
                messages,
            },
            &output,
            keys,
        )
    });
    // The findings under the pointer, while the text they were placed in
    // is the text on screen, and what the name under it is.
    if let Some(char_index) = output.hover_char_index
        && !output.changed
    {
        let covering = held.covering(char_index);
        let information = assist_answer
            .as_ref()
            .and_then(|answer| answer.hover.as_ref());
        if !covering.is_empty() || information.is_some() {
            output.response.clone().on_hover_ui_at_pointer(|ui| {
                if !covering.is_empty() {
                    findings_tooltip(ui, &held, &covering, messages, &colors);
                }
                if let Some(information) = information {
                    ui.set_max_width(420.0);
                    assist::information_body(ui, information, &colors);
                }
            });
        }
    }
    let announcement =
        announcement.or_else(|| assist_answer.and_then(|answer| answer.announcement));
    ui.ctx().data_mut(|data| {
        data.insert_temp(reported_caret_id(editor_id), output.cursor_char_index);
        data.insert_temp(held_id, held);
        if let Some(highlighter) = highlighter {
            data.insert_temp(highlighter_id, highlighter);
        }
    });
    CodeDocumentInteraction {
        changed: output.changed,
        cursor_char_index: output.cursor_char_index,
        selected_char_range: output.selected_char_range,
        breakpoint_toggled: output.breakpoint_toggled,
        response: output.response,
        announcement,
        unanswered_history: output.unanswered_history,
    }
}

fn layout_job(
    source: &str,
    language: CodeEditorLanguage,
    font: FontId,
    diagnostics: &[CodeEditorDiagnostic],
    colors: &crate::ui::palette::Palette,
) -> LayoutJob {
    let mut job = LayoutJob {
        break_on_newline: true,
        ..Default::default()
    };
    job.wrap.max_width = f32::INFINITY;
    let mut byte_offset = 0;
    let mut state = LineState::default();
    for segment in source.split_inclusive('\n') {
        let (line, newline) = segment
            .strip_suffix('\n')
            .map_or((segment, ""), |line| (line, "\n"));
        if language == CodeEditorLanguage::VerilogA {
            let (spans, exit) = tokenize_line(line, state);
            state = exit;
            append_classified(&mut job, &spans, line, 0..line.len(), &font, colors);
        } else {
            highlight_line(
                &mut job,
                line,
                language,
                byte_offset,
                &font,
                diagnostics,
                colors,
            );
        }
        if !newline.is_empty() {
            append(&mut job, newline, font.clone(), colors.text, Stroke::NONE);
        }
        byte_offset += segment.len();
    }
    if source.is_empty() {
        append(&mut job, "", font, colors.text, Stroke::NONE);
    }
    job
}

/// Lay out the part of a Verilog-A line a row shows: `line` is the whole
/// line, classified as `spans`, and `shown` the part of it from character
/// `first_column`. The classes come from the whole line, so a row scrolled
/// sideways colours a string or a comment it starts inside of.
fn veriloga_row_job(
    spans: &[SyntaxSpan],
    line: &str,
    shown: &str,
    first_column: usize,
    font: FontId,
    colors: &crate::ui::palette::Palette,
) -> LayoutJob {
    let mut job = LayoutJob::default();
    job.wrap.max_width = f32::INFINITY;
    let start = line
        .char_indices()
        .nth(first_column)
        .map_or(line.len(), |(index, _)| index);
    let end = start + shown.len();
    if line.get(start..end) == Some(shown) {
        append_classified(&mut job, spans, line, start..end, &font, colors);
    } else {
        // The row is not the line this frame read: lay it out plainly
        // rather than colour the wrong text.
        append(&mut job, shown, font, colors.text, Stroke::NONE);
    }
    job
}

/// Append `line[shown]` to `job`, each stretch in the ink of its class.
fn append_classified(
    job: &mut LayoutJob,
    spans: &[SyntaxSpan],
    line: &str,
    shown: std::ops::Range<usize>,
    font: &FontId,
    colors: &crate::ui::palette::Palette,
) {
    let mut cursor = shown.start;
    for span in spans {
        let from = span.range.start.max(cursor);
        let to = span.range.end.min(shown.end);
        if from >= to {
            continue;
        }
        if cursor < from {
            append(
                job,
                &line[cursor..from],
                font.clone(),
                colors.text,
                Stroke::NONE,
            );
        }
        append(
            job,
            &line[from..to],
            font.clone(),
            syntax_color(span.class, colors),
            Stroke::NONE,
        );
        cursor = to;
    }
    if cursor < shown.end {
        append(
            job,
            &line[cursor..shown.end],
            font.clone(),
            colors.text,
            Stroke::NONE,
        );
    }
    if shown.is_empty() {
        append(job, "", font.clone(), colors.text, Stroke::NONE);
    }
}

/// The ink a Verilog-A syntax class is painted in.
const fn syntax_color(class: SyntaxClass, colors: &crate::ui::palette::Palette) -> Color32 {
    let syntax = &colors.syntax;
    match class {
        SyntaxClass::Text => colors.text,
        SyntaxClass::Keyword => syntax.keyword,
        SyntaxClass::Discipline => syntax.discipline,
        SyntaxClass::SystemTask => syntax.system,
        SyntaxClass::AnalogOperator => syntax.analog_operator,
        SyntaxClass::Contribution => syntax.contribution,
        SyntaxClass::Number => syntax.number,
        SyntaxClass::String => syntax.string,
        SyntaxClass::Comment => syntax.comment,
        SyntaxClass::Directive => syntax.directive,
        SyntaxClass::Attribute => syntax.attribute,
        SyntaxClass::ModuleName => syntax.module_name,
        SyntaxClass::PortName => syntax.port_name,
        SyntaxClass::ParameterName => syntax.parameter_name,
    }
}

fn highlight_line(
    job: &mut LayoutJob,
    line: &str,
    language: CodeEditorLanguage,
    line_start: usize,
    font: &FontId,
    diagnostics: &[CodeEditorDiagnostic],
    colors: &crate::ui::palette::Palette,
) {
    let mut cursor = 0;
    while cursor < line.len() {
        let rest = &line[cursor..];
        let (length, color) = next_token(rest, language, colors);
        let length = length.max(rest.chars().next().map_or(0, char::len_utf8));
        let end = (cursor + length).min(line.len());
        let range = (line_start + cursor)..(line_start + end);
        let severity = diagnostics
            .iter()
            .filter_map(|diagnostic| {
                let diagnostic_range = diagnostic.byte_range.as_ref()?;
                (diagnostic_range.start < range.end && range.start < diagnostic_range.end)
                    .then_some(diagnostic.severity)
            })
            .max();
        let underline = severity.map_or(Stroke::NONE, |severity| {
            Stroke::new(
                1.0,
                diagnostic_color(severity, colors.err, colors.warn, colors.text_faint),
            )
        });
        append(job, &line[cursor..end], font.clone(), color, underline);
        cursor = end;
    }
}

fn next_token(
    text: &str,
    language: CodeEditorLanguage,
    colors: &crate::ui::palette::Palette,
) -> (usize, Color32) {
    if text.starts_with("//") || text.starts_with('#') {
        return (text.len(), colors.text_faint);
    }
    if let Some(quote) = text
        .chars()
        .next()
        .filter(|character| *character == '"' || *character == '\'')
    {
        let mut escaped = false;
        for (index, character) in text.char_indices().skip(1) {
            if !escaped && character == quote {
                return (index + character.len_utf8(), colors.warn);
            }
            escaped = !escaped && character == '\\';
            if character != '\\' {
                escaped = false;
            }
        }
        return (text.len(), colors.warn);
    }
    let first = text.chars().next().unwrap_or_default();
    if first.is_ascii_digit() {
        let length = text
            .char_indices()
            .take_while(|(_, ch)| {
                ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | '+' | '-' | ':')
            })
            .last()
            .map_or(first.len_utf8(), |(index, ch)| index + ch.len_utf8());
        return (length, colors.traces[3]);
    }
    if first.is_alphabetic() || matches!(first, '_' | '$' | '`') {
        let length = text
            .char_indices()
            .take_while(|(_, ch)| ch.is_alphanumeric() || matches!(ch, '_' | '$' | '`' | '-'))
            .last()
            .map_or(first.len_utf8(), |(index, ch)| index + ch.len_utf8());
        let word = &text[..length];
        let color = match language {
            CodeEditorLanguage::Automation if word == "plan" => colors.traces[1],
            CodeEditorLanguage::Automation if matches!(word, "True" | "False") => colors.net_label,
            CodeEditorLanguage::Python
                if matches!(
                    word,
                    "from"
                        | "import"
                        | "as"
                        | "def"
                        | "class"
                        | "return"
                        | "if"
                        | "else"
                        | "elif"
                        | "for"
                        | "while"
                        | "try"
                        | "except"
                        | "finally"
                        | "with"
                        | "raise"
                ) =>
            {
                colors.traces[1]
            }
            CodeEditorLanguage::Python if matches!(word, "True" | "False" | "None") => {
                colors.net_label
            }
            CodeEditorLanguage::Yaml | CodeEditorLanguage::Toml
                if text[length..].trim_start().starts_with(':')
                    || text[length..].trim_start().starts_with('=') =>
            {
                colors.net_label
            }
            _ => colors.text,
        };
        return (length, color);
    }
    (first.len_utf8(), colors.text)
}

fn append(job: &mut LayoutJob, text: &str, font: FontId, color: Color32, underline: Stroke) {
    let line_height = editor_font::line_height(font.size);
    job.append(
        text,
        0.0,
        TextFormat {
            font_id: font,
            color,
            line_height: Some(line_height),
            underline,
            ..Default::default()
        },
    );
}

/// A finding's severity as the word that opens a sentence about it.
pub(crate) fn severity_title(
    severity: CodeEditorSeverity,
    messages: crate::workbench::MessageCatalog,
) -> String {
    messages.text(match severity {
        CodeEditorSeverity::Error => MessageId::SourceDiagnosticSeverityError,
        CodeEditorSeverity::Warning => MessageId::SourceDiagnosticSeverityWarning,
        CodeEditorSeverity::Info => MessageId::SourceDiagnosticSeverityInformation,
        CodeEditorSeverity::Hint => MessageId::SourceDiagnosticSeverityHint,
    })
}

/// What a screen reader hears for a source line that has a finding: its
/// severity and its message, so the line says what is wrong with it.
fn line_diagnostic_description(
    finding: &HeldFinding,
    messages: crate::workbench::MessageCatalog,
) -> String {
    messages.format(
        MessageId::EditorAccessibleLineDiagnostic,
        &[
            ("severity", &severity_title(finding.severity, messages)),
            ("message", &finding.message),
        ],
    )
}

/// The findings under the pointer: for each, its severity, code and place
/// over its message, and a word when they were found before the latest edits.
fn findings_tooltip(
    ui: &mut Ui,
    held: &HeldFindings,
    covering: &[&HeldFinding],
    messages: crate::workbench::MessageCatalog,
    colors: &crate::ui::palette::Palette,
) {
    ui.set_max_width(420.0);
    for finding in covering {
        let (line, column) = held.line_column(finding.range.start);
        let place = messages.format(
            MessageId::SourceDiagnosticHoverPlace,
            &[
                ("severity", &severity_title(finding.severity, messages)),
                ("code", &finding.code),
                ("line", &(line + 1).to_string()),
                ("column", &(column + 1).to_string()),
            ],
        );
        ui.label(
            egui::RichText::new(place)
                .font(theme::mono(POPUP_FONT_SIZE, FontWeight::Medium))
                .color(colors.text_dim),
        );
        ui.label(egui::RichText::new(finding.message.as_ref()).color(colors.text));
    }
    if held.is_outdated() {
        ui.label(
            egui::RichText::new(messages.text(MessageId::SourceDiagnosticOutdated))
                .color(colors.text_dim),
        );
    }
}

/// The gutter mark a diagnostic earns: only a warning or an error is marked.
/// A note or a hint keeps its squiggle and leaves the gutter alone.
const fn diagnostic_problem(severity: CodeEditorSeverity) -> Option<Problem> {
    match severity {
        CodeEditorSeverity::Error => Some(Problem::Failure),
        CodeEditorSeverity::Warning => Some(Problem::Warning),
        CodeEditorSeverity::Info | CodeEditorSeverity::Hint => None,
    }
}

/// The squiggle under the offending text: an error or a warning in its own
/// colour, a note or a hint in the faint neutral, since neither is a problem.
const fn diagnostic_color(
    severity: CodeEditorSeverity,
    error: Color32,
    warning: Color32,
    neutral: Color32,
) -> Color32 {
    match severity {
        CodeEditorSeverity::Hint | CodeEditorSeverity::Info => neutral,
        CodeEditorSeverity::Warning => warning,
        CodeEditorSeverity::Error => error,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The character index of a one-based line and column in `source`, the
    /// column kept within the line.
    fn char_index_at(source: &str, line: usize, column: usize) -> usize {
        let (start, width) = line_extent(source, line);
        start + column.saturating_sub(1).min(width)
    }

    /// F8 from a caret goes to the first finding after it and Shift+F8 to
    /// the last before it, both wrapping; two findings at one place are
    /// stepped through one after the other rather than skipped.
    #[test]
    fn a_problem_step_moves_on_from_the_caret_and_through_shared_places() {
        let context = egui::Context::default();
        let editor = egui::Id::new("problem-step-test");
        let source = "a\nbb = cc;\nd\n";
        // Line 2 column 6 twice, then line 3 with no column.
        let places = [(2, Some(6)), (2, Some(6)), (3, None)];
        let caret_at = |line, column| char_index_at(source, line, column);
        let step = |caret, step| {
            problem_step_index(&context, editor, source, &places, caret, step)
                .expect("a document with findings")
        };

        assert_eq!(step(caret_at(1, 1), ProblemStep::Next), 0);
        // The caret now stands on the shared place: the second there is next.
        assert_eq!(step(caret_at(2, 6), ProblemStep::Next), 1);
        assert_eq!(step(caret_at(2, 6), ProblemStep::Next), 2);
        assert_eq!(step(caret_at(3, 1), ProblemStep::Next), 0, "wraps");
        // A caret the reader moved is stepped from where it now is.
        assert_eq!(step(caret_at(3, 2), ProblemStep::Previous), 2);
        assert_eq!(step(caret_at(1, 1), ProblemStep::Previous), 2, "wraps");
        assert_eq!(
            problem_step_index(&context, editor, source, &[], 0, ProblemStep::Next),
            None
        );
        // A column past the line's end lands at its end.
        assert_eq!(char_index_at(source, 2, 40), caret_at(2, 9));
        assert_eq!(caret_place(source, caret_at(2, 6)), (2, 6));
    }

    #[test]
    fn canonical_source_tokens_use_mockup_palette_roles() {
        let colors = crate::ui::palette::INSTRUMENT_DARK;
        assert_eq!(
            next_token("\"all\"", CodeEditorLanguage::Automation, &colors).1,
            colors.warn
        );
        assert_eq!(
            next_token("plan", CodeEditorLanguage::Automation, &colors).1,
            colors.traces[1]
        );
        assert_eq!(
            next_token("True", CodeEditorLanguage::Automation, &colors).1,
            colors.net_label
        );
        assert_eq!(
            next_token("from", CodeEditorLanguage::Python, &colors).1,
            colors.traces[1]
        );
        assert_eq!(
            next_token("schema: value", CodeEditorLanguage::Yaml, &colors).1,
            colors.net_label
        );
    }

    /// A Verilog-A row wears the syntax inks of the theme it is drawn in, and
    /// a row scrolled sideways keeps the class of the text it starts inside.
    #[test]
    fn veriloga_rows_wear_the_syntax_inks_of_their_theme() {
        let line = "  parameter real gain = 10k; // open loop";
        let (spans, _) = tokenize_line(line, LineState::default());
        let font = FontId::monospace(POPUP_FONT_SIZE);
        for colors in [
            crate::ui::palette::INSTRUMENT_DARK,
            crate::ui::palette::INSTRUMENT_LIGHT,
        ] {
            let job = veriloga_row_job(&spans, line, line, 0, font.clone(), &colors);
            let ink = |text: &str| {
                let start = line.find(text).expect("the text is on the line");
                job.format_at_byte(egui::epaint::text::ByteIndex(start))
                    .color
            };
            let syntax = colors.syntax;
            assert_eq!(ink("parameter"), syntax.keyword);
            assert_eq!(ink("gain"), syntax.parameter_name);
            assert_eq!(ink("10k"), syntax.number);
            assert_eq!(ink("// open"), syntax.comment);
            assert_eq!(ink("="), colors.text);
            // Scrolled to start inside the comment, the row is still comment.
            let shown = &line[line.find("open").expect("the comment")..];
            let first_column = line[..line.len() - shown.len()].chars().count();
            let job = veriloga_row_job(&spans, line, shown, first_column, font.clone(), &colors);
            assert_eq!(job.text, shown);
            assert!(
                job.sections
                    .iter()
                    .all(|section| section.format.color == syntax.comment)
            );
        }
    }

    #[test]
    fn serialized_editor_diagnostic_contains_the_complete_canonical_contract() {
        let diagnostic = CodeEditorDiagnostic::current(
            "rspice.automation.runtime",
            "PY-100",
            CodeEditorSeverity::Hint,
            "consider a bounded sweep",
            "The current range is large.",
            Some("workflow.py".to_owned()),
            Some("plan.run()\n".to_owned()),
            Some(0..4),
            Some(1),
            Some(1),
        )
        .bind_validation("document-1", 7, "validation-7");
        let value = serde_json::to_value(diagnostic).unwrap();
        for field in [
            "diagnosticId",
            "code",
            "severity",
            "source",
            "documentId",
            "range",
            "message",
            "details",
            "relatedLocations",
            "quickFixes",
            "suppression",
            "revision",
            "validationId",
            "currentness",
            "affectedConsumers",
        ] {
            assert!(value.get(field).is_some(), "missing {field}");
        }
        assert_eq!(value["revision"], 7);
        assert_eq!(value["currentness"], "current");
    }
}
