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
use crate::ui::widgets::{Problem, paint_problem_mark};
use crate::workbench::MessageId;
use crate::workbench::documents::text_editor_commands::{
    EditorSyntax, consume_standard_command, consume_structural_shortcut, show_go_to_line,
    take_reveal_line,
};

const FONT_SIZE: f32 = 11.0;
const LINE_HEIGHT: f32 = 17.05;
const GUTTER_WIDTH: f32 = 47.0;
const NUMBER_RIGHT_PADDING: f32 = 11.0;
/// Side of a diagnostic's gutter mark, set in the line number's right padding.
const GUTTER_MARK_SIDE: f32 = 8.0;
const CODE_LEFT_PADDING: f32 = 12.0;
// One editor substrate must own cursor, folding, multi-selection, IME, and
// undo semantics at every document size. Keeping a second small-file TextEdit
// path made commands such as fold-all change behavior at an arbitrary byte
// boundary. Empty buffers alone use TextEdit until their first insertion.
const VIRTUALIZATION_THRESHOLD_BYTES: usize = 1;

fn code_editor_frame() -> egui::Frame {
    egui::Frame::new().inner_margin(egui::Margin {
        left: (GUTTER_WIDTH + CODE_LEFT_PADDING) as i8,
        right: 20,
        top: 8,
        bottom: 36,
    })
}

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

#[derive(Debug, Clone, Default)]
pub struct CodeDocumentInteraction {
    pub changed: bool,
    pub cursor_char_index: usize,
    pub selected_char_range: Option<(usize, usize)>,
    pub breakpoint_toggled: Option<usize>,
    /// The source well, for its right-click menu. An empty document is a
    /// plain text field with nothing to act on, and has none.
    pub response: Option<egui::Response>,
}

pub fn show_code_document_interaction_versioned(
    ui: &mut Ui,
    id: impl std::hash::Hash + std::fmt::Debug,
    source: &mut String,
    source_revision: u64,
    language: CodeEditorLanguage,
    diagnostics: &CodeDiagnosticCollection,
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
        diagnostics,
        diagnostic_path,
        editable,
        messages,
        &[],
        None,
        false,
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
    diagnostics: &CodeDiagnosticCollection,
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
        diagnostics,
        diagnostic_path,
        editable,
        messages,
        breakpoints,
        current_line,
        true,
    )
}

fn show_code_document_impl(
    ui: &mut Ui,
    editor_id: egui::Id,
    source: &mut String,
    source_revision: u64,
    language: CodeEditorLanguage,
    diagnostics: &CodeDiagnosticCollection,
    diagnostic_path: Option<&str>,
    editable: bool,
    messages: crate::workbench::MessageCatalog,
    breakpoints: &[usize],
    current_line: Option<usize>,
    breakpoints_enabled: bool,
) -> CodeDocumentInteraction {
    let requested_line =
        take_reveal_line(ui, editor_id, source).or_else(|| show_go_to_line(ui, editor_id, source));
    let tokens = Tokens::get(ui.ctx());
    let colors = tokens.color;
    let font = theme::mono(FONT_SIZE, FontWeight::Regular);
    let syntax = match language {
        CodeEditorLanguage::VerilogA => EditorSyntax::VerilogA,
        CodeEditorLanguage::Automation | CodeEditorLanguage::Python => EditorSyntax::Python,
        CodeEditorLanguage::Yaml => EditorSyntax::Yaml,
        CodeEditorLanguage::Toml => EditorSyntax::Toml,
    };
    let diagnostic_view = diagnostics.view(diagnostic_path);
    if source.len() >= VIRTUALIZATION_THRESHOLD_BYTES {
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
            line_height: LINE_HEIGHT,
            gutter_width: GUTTER_WIDTH,
            code_left_padding: CODE_LEFT_PADDING,
            top_padding: 8.0,
            background: colors.bg_inset,
            hover_background: colors.bg_hover,
            active_background: colors.accent.linear_multiply(0.075),
            selection_background: colors.accent.linear_multiply(0.28),
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
        };
        let output = crate::workbench::documents::virtual_text_editor::show_virtual_text_editor(
            ui,
            editor_id,
            source,
            source_revision,
            editable,
            syntax,
            requested_line,
            &style,
            |ui, text, _line, font| {
                let job = layout_job(text, language, font, &[], &colors);
                ui.fonts_mut(|fonts| fonts.layout_job(job))
            },
            |logical_line| {
                diagnostic_view
                    .severity_for_line(logical_line.saturating_add(1))
                    .and_then(diagnostic_problem)
            },
            |logical_line| {
                diagnostic_view
                    .severity_for_line(logical_line.saturating_add(1))
                    .map(|severity| {
                        format!(
                            "{} diagnostic on this line.",
                            messages.text(match severity {
                                CodeEditorSeverity::Hint => MessageId::EditorAccessibleSeverityHint,
                                CodeEditorSeverity::Info => {
                                    MessageId::EditorAccessibleSeverityInformation
                                }
                                CodeEditorSeverity::Warning => {
                                    MessageId::EditorAccessibleSeverityWarning
                                }
                                CodeEditorSeverity::Error => {
                                    MessageId::EditorAccessibleSeverityError
                                }
                            })
                        )
                    })
            },
        );
        return CodeDocumentInteraction {
            changed: output.changed,
            cursor_char_index: output.cursor_char_index,
            selected_char_range: output.selected_char_range,
            breakpoint_toggled: output.breakpoint_toggled,
            response: Some(output.response),
        };
    }
    let standard_changed = consume_standard_command(ui, editor_id, source, editable);
    let shortcut_changed = consume_structural_shortcut(ui, editor_id, source, syntax, editable);
    let mut layouter = |ui: &Ui, text: &dyn egui::TextBuffer, _wrap_width: f32| {
        let job = layout_job(text.as_str(), language, font.clone(), &[], &colors);
        ui.fonts_mut(|fonts| fonts.layout_job(job))
    };
    let height = ui.available_height().max(120.0);
    let mut changed = standard_changed || shortcut_changed;
    let mut breakpoint_toggled = None;
    ui.allocate_ui(egui::vec2(ui.available_width(), height), |ui| {
        egui::ScrollArea::both()
            .id_salt(editor_id.with("scroll"))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // Reserve the hover wash before TextEdit emits glyphs and
                // selection shapes. The row geometry is known only after the
                // galley is laid out, so the placeholder is filled below.
                let hover_background = ui.painter().add(egui::Shape::Noop);
                let output = egui::TextEdit::multiline(source)
                    .id(editor_id)
                    .code_editor()
                    .font(font.clone())
                    .desired_width(f32::INFINITY)
                    .desired_rows(24)
                    .interactive(editable)
                    // egui intentionally ignores `TextEdit::margin` whenever
                    // a custom frame is supplied. Put the canonical code-well
                    // insets on that frame so source glyphs cannot render
                    // underneath the gutter or against the toolbar divider.
                    .frame(code_editor_frame())
                    .layouter(&mut layouter)
                    .show(ui);
                changed |= output.response.changed();
                let painter = ui.painter();
                let row_rect = |row: &egui::epaint::text::PlacedRow| {
                    egui::Rect::from_min_max(
                        egui::pos2(
                            output.response.rect.left(),
                            output.galley_pos.y + row.rect().top(),
                        ),
                        egui::pos2(
                            output.response.rect.right(),
                            output.galley_pos.y + row.rect().bottom(),
                        ),
                    )
                };
                if let Some(line) = requested_line
                    && let Some(row) = output.galley.rows.get(line.saturating_sub(1))
                {
                    ui.scroll_to_rect(row_rect(row), Some(egui::Align::Center));
                }
                if let Some(pointer) = ui.input(|input| input.pointer.hover_pos())
                    && output.response.rect.contains(pointer)
                    && let Some(row) = output
                        .galley
                        .rows
                        .iter()
                        .find(|row| row_rect(row).contains(pointer))
                {
                    painter.set(
                        hover_background,
                        egui::Shape::rect_filled(row_rect(row), 0.0, colors.bg_hover),
                    );
                }
                painter.vline(
                    output.response.rect.left() + GUTTER_WIDTH,
                    output.response.rect.y_range(),
                    Stroke::new(1.0, colors.border),
                );
                let gutter_font = theme::mono(FONT_SIZE, FontWeight::Regular);
                for (line_index, row) in output.galley.rows.iter().enumerate() {
                    let line_number = line_index + 1;
                    let y = output.galley_pos.y + row.rect().center().y;
                    if !ui.clip_rect().y_range().contains(y) {
                        continue;
                    }
                    let line_rect = row_rect(row);
                    let gutter_rect = egui::Rect::from_min_max(
                        line_rect.min,
                        egui::pos2(line_rect.left() + GUTTER_WIDTH, line_rect.bottom()),
                    );
                    if breakpoints_enabled {
                        let gutter_response = ui.interact(
                            gutter_rect,
                            editor_id.with(("breakpoint", line_number)),
                            egui::Sense::click(),
                        );
                        gutter_response.widget_info(|| {
                            egui::WidgetInfo::labeled(
                                egui::WidgetType::Button,
                                ui.is_enabled(),
                                format!("Toggle breakpoint on line {line_number}"),
                            )
                        });
                        theme::paint_focus_ring(ui, &gutter_response, gutter_rect);
                        if gutter_response.clicked() {
                            breakpoint_toggled = Some(line_number);
                        }
                    }
                    if current_line == Some(line_number) {
                        painter.rect_filled(line_rect, 0.0, colors.accent.linear_multiply(0.12));
                        painter.circle_filled(
                            egui::pos2(line_rect.left() + 18.0, y),
                            4.0,
                            colors.warn,
                        );
                    }
                    if breakpoints.contains(&line_number) {
                        painter.circle_filled(
                            egui::pos2(line_rect.left() + 7.0, y),
                            4.0,
                            colors.err,
                        );
                    }
                    // The number is neutral; a warning or an error on the
                    // line is a mark in the number's right padding.
                    painter.text(
                        egui::pos2(
                            output.response.rect.left() + GUTTER_WIDTH - NUMBER_RIGHT_PADDING,
                            y,
                        ),
                        egui::Align2::RIGHT_CENTER,
                        line_number.to_string(),
                        gutter_font.clone(),
                        colors.text_faint,
                    );
                    if let Some(problem) = diagnostic_view
                        .severity_for_line(line_number)
                        .and_then(diagnostic_problem)
                    {
                        paint_problem_mark(
                            painter,
                            egui::Rect::from_center_size(
                                egui::pos2(output.response.rect.left() + GUTTER_WIDTH - 5.0, y),
                                egui::Vec2::splat(GUTTER_MARK_SIDE),
                            ),
                            problem,
                        );
                    }
                }
            });
    });
    let char_range = egui::text_edit::TextEditState::load(ui.ctx(), editor_id)
        .and_then(|state| state.cursor.char_range());
    let cursor_char_index = char_range.map_or(0, |range| range.primary.index.0);
    let selected_char_range = char_range.and_then(|range| {
        let range = range.as_sorted_char_range();
        (range.start != range.end).then_some((range.start.0, range.end.0))
    });
    CodeDocumentInteraction {
        changed,
        cursor_char_index,
        selected_char_range,
        breakpoint_toggled,
        response: None,
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
    for segment in source.split_inclusive('\n') {
        let (line, newline) = segment
            .strip_suffix('\n')
            .map_or((segment, ""), |line| (line, "\n"));
        highlight_line(
            &mut job,
            line,
            language,
            byte_offset,
            &font,
            diagnostics,
            colors,
        );
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
        let (length, color) = next_token(rest, &line[..cursor], language, colors);
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
    prefix: &str,
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
    if language == CodeEditorLanguage::VerilogA && text.starts_with("parameter ") {
        let length = text
            .find('=')
            .map_or(text.len(), |index| text[..index].trim_end().len());
        return (length.max("parameter".len()), colors.net_label);
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
            CodeEditorLanguage::VerilogA
                if matches!(word, "module" | "endmodule" | "`include" | "`define") =>
            {
                colors.traces[1]
            }
            CodeEditorLanguage::VerilogA if prefix.trim_end().ends_with("module") => {
                colors.traces[0]
            }
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
    job.append(
        text,
        0.0,
        TextFormat {
            font_id: font,
            color,
            line_height: Some(LINE_HEIGHT),
            underline,
            ..Default::default()
        },
    );
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

    #[test]
    fn editor_geometry_matches_canonical_code_workspace() {
        assert_eq!(FONT_SIZE, 11.0);
        assert_eq!(LINE_HEIGHT, 17.05);
        assert_eq!(GUTTER_WIDTH, 47.0);
        assert_eq!(NUMBER_RIGHT_PADDING, 11.0);
        assert_eq!(CODE_LEFT_PADDING, 12.0);
        let frame = code_editor_frame();
        assert_eq!(frame.inner_margin.left, 59);
        assert_eq!(frame.inner_margin.right, 20);
        assert_eq!(frame.inner_margin.top, 8);
        assert_eq!(frame.inner_margin.bottom, 36);
    }

    #[test]
    fn canonical_source_tokens_use_mockup_palette_roles() {
        let colors = crate::ui::palette::INSTRUMENT_DARK;
        assert_eq!(
            next_token("module", "", CodeEditorLanguage::VerilogA, &colors).1,
            colors.traces[1]
        );
        assert_eq!(
            next_token(
                "sensor_bridge",
                "module ",
                CodeEditorLanguage::VerilogA,
                &colors,
            )
            .1,
            colors.traces[0]
        );
        assert_eq!(
            next_token("100.0", "", CodeEditorLanguage::VerilogA, &colors).1,
            colors.traces[3]
        );
        assert_eq!(
            next_token("\"all\"", "", CodeEditorLanguage::Automation, &colors).1,
            colors.warn
        );
        assert_eq!(
            next_token("plan", "", CodeEditorLanguage::Automation, &colors).1,
            colors.traces[1]
        );
        assert_eq!(
            next_token("True", "", CodeEditorLanguage::Automation, &colors).1,
            colors.net_label
        );
        assert_eq!(
            next_token("from", "", CodeEditorLanguage::Python, &colors).1,
            colors.traces[1]
        );
        assert_eq!(
            next_token("schema: value", "", CodeEditorLanguage::Yaml, &colors).1,
            colors.net_label
        );
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
