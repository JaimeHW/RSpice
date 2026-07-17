//! Language-neutral owned-source editor used by the Verilog-A and Automation pages.
//!
//! It follows the mockup's fixed gutter and non-wrapping code geometry while
//! retaining ordinary `TextEdit` selection, clipboard, IME, and accessibility
//! semantics. Diagnostics are byte-ranged and share the same source identity as
//! the compile/validation receipt that produced them.

use egui::text::{LayoutJob, TextFormat};
use egui::{Color32, FontId, Stroke, Ui};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::Tokens;

const FONT_SIZE: f32 = 11.0;
const LINE_HEIGHT: f32 = 17.05;
const GUTTER_WIDTH: f32 = 47.0;
const NUMBER_RIGHT_PADDING: f32 = 11.0;
const CODE_LEFT_PADDING: f32 = 12.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeEditorLanguage {
    VerilogA,
    Automation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CodeEditorSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeEditorDiagnostic {
    pub severity: CodeEditorSeverity,
    pub message: String,
    pub detail: String,
    pub byte_range: Option<std::ops::Range<usize>>,
    pub line: Option<usize>,
    pub column: Option<usize>,
}

/// Show an exact-entry editor and return whether its UTF-8 bytes changed.
pub fn show_code_editor(
    ui: &mut Ui,
    id: impl std::hash::Hash,
    source: &mut String,
    language: CodeEditorLanguage,
    diagnostics: &[CodeEditorDiagnostic],
) -> bool {
    let tokens = Tokens::get(ui.ctx());
    let colors = tokens.color;
    let font = theme::mono(FONT_SIZE, FontWeight::Regular);
    let editor_id = egui::Id::new(id);
    let mut layouter = |ui: &Ui, text: &dyn egui::TextBuffer, _wrap_width: f32| {
        let job = layout_job(text.as_str(), language, font.clone(), diagnostics, &colors);
        ui.fonts_mut(|fonts| fonts.layout_job(job))
    };
    let height = ui.available_height().max(120.0);
    let mut changed = false;
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
                    .frame(egui::Frame::NONE)
                    .margin(egui::Margin {
                        left: (GUTTER_WIDTH + CODE_LEFT_PADDING) as i8,
                        right: 20,
                        top: 8,
                        bottom: 36,
                    })
                    .layouter(&mut layouter)
                    .show(ui);
                changed = output.response.changed();
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
                    let y = output.galley_pos.y + row.rect().center().y;
                    if !ui.clip_rect().y_range().contains(y) {
                        continue;
                    }
                    let severity = line_severity(source, line_index + 1, diagnostics);
                    let color = severity.map_or(colors.text_faint, |severity| {
                        diagnostic_color(severity, colors.err, colors.warn, colors.info)
                    });
                    painter.text(
                        egui::pos2(
                            output.response.rect.left() + GUTTER_WIDTH - NUMBER_RIGHT_PADDING,
                            y,
                        ),
                        egui::Align2::RIGHT_CENTER,
                        (line_index + 1).to_string(),
                        gutter_font.clone(),
                        color,
                    );
                    if severity.is_some() {
                        painter.circle_filled(
                            egui::pos2(output.response.rect.left() + GUTTER_WIDTH - 5.0, y),
                            2.5,
                            color,
                        );
                    }
                }
            });
    });
    changed
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
                diagnostic_color(severity, colors.err, colors.warn, colors.info),
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

fn line_severity(
    source: &str,
    one_based_line: usize,
    diagnostics: &[CodeEditorDiagnostic],
) -> Option<CodeEditorSeverity> {
    diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.line == Some(one_based_line)
                || diagnostic
                    .byte_range
                    .as_ref()
                    .is_some_and(|range| byte_offset_line(source, range.start) == one_based_line)
        })
        .map(|diagnostic| diagnostic.severity)
        .max()
}

fn byte_offset_line(source: &str, offset: usize) -> usize {
    source
        .as_bytes()
        .iter()
        .take(offset.min(source.len()))
        .filter(|byte| **byte == b'\n')
        .count()
        + 1
}

const fn diagnostic_color(
    severity: CodeEditorSeverity,
    error: Color32,
    warning: Color32,
    info: Color32,
) -> Color32 {
    match severity {
        CodeEditorSeverity::Info => info,
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
    }

    #[test]
    fn unicode_line_lookup_is_byte_safe() {
        let source = "module µ;\nanalog V(o) <+ 1;\nendmodule\n";
        assert_eq!(byte_offset_line(source, source.find("analog").unwrap()), 2);
    }

    #[test]
    fn line_diagnostics_use_one_based_lines() {
        let diagnostic = CodeEditorDiagnostic {
            severity: CodeEditorSeverity::Error,
            message: "broken".to_owned(),
            detail: String::new(),
            byte_range: None,
            line: Some(2),
            column: Some(1),
        };
        assert_eq!(
            line_severity("first\nsecond", 2, &[diagnostic]),
            Some(CodeEditorSeverity::Error)
        );
    }
}
