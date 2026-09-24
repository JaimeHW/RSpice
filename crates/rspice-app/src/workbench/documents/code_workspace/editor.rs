//! Language-neutral owned-source editor used by the Verilog-A and Automation pages.
//!
//! It follows the mockup's fixed gutter and non-wrapping code geometry while
//! retaining ordinary `TextEdit` selection, clipboard, IME, and accessibility
//! semantics. Diagnostics are byte-ranged and share the same source identity as
//! the compile/validation receipt that produced them.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use egui::text::{LayoutJob, TextFormat};
use egui::{Color32, FontId, Stroke, Ui};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::Tokens;
use crate::workbench::MessageId;
use crate::workbench::documents::text_editor_commands::{
    EditorSyntax, consume_standard_command, consume_structural_shortcut, show_go_to_line,
    take_reveal_line,
};

const FONT_SIZE: f32 = 11.0;
const LINE_HEIGHT: f32 = 17.05;
const GUTTER_WIDTH: f32 = 47.0;
const NUMBER_RIGHT_PADDING: f32 = 11.0;
const CODE_LEFT_PADDING: f32 = 12.0;
// One editor substrate must own cursor, folding, multi-selection, IME, and
// undo semantics at every document size. Keeping a second small-file TextEdit
// path made commands such as fold-all change behavior at an arbitrary byte
// boundary. Empty buffers alone use TextEdit until their first insertion.
const VIRTUALIZATION_THRESHOLD_BYTES: usize = 1;
pub const MAX_CODE_DIAGNOSTICS: usize = 1_000_000;

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

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum CodeEditorSeverity {
    #[serde(rename = "hint")]
    Hint,
    #[serde(rename = "information")]
    Info,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CodeEditorDiagnostic {
    #[serde(flatten)]
    pub(crate) canonical:
        crate::workbench::documents::canonical_diagnostics::CanonicalDiagnosticMetadata,
    pub severity: CodeEditorSeverity,
    pub message: Arc<str>,
    #[serde(rename = "details")]
    pub detail: Arc<str>,
    /// Stable logical source identity for multi-file diagnostics.
    #[serde(rename = "logicalPath")]
    pub source_path: Option<Arc<str>>,
    /// Exact retained UTF-8 source paired with `source_path`.
    #[serde(rename = "sourceText")]
    pub source: Option<Arc<str>>,
    #[serde(rename = "byteRange")]
    pub byte_range: Option<std::ops::Range<usize>>,
    #[serde(rename = "sourceStartLine")]
    pub line: Option<usize>,
    #[serde(rename = "sourceStartColumn")]
    pub column: Option<usize>,
}

impl CodeEditorDiagnostic {
    pub(crate) fn current(
        producer: impl Into<String>,
        code: impl Into<String>,
        severity: CodeEditorSeverity,
        message: impl Into<Arc<str>>,
        detail: impl Into<Arc<str>>,
        source_path: Option<String>,
        source: Option<String>,
        byte_range: Option<std::ops::Range<usize>>,
        line: Option<usize>,
        column: Option<usize>,
    ) -> Self {
        let message = message.into();
        let source_path = source_path.map(Arc::<str>::from);
        let source = source.map(Arc::<str>::from);
        let document_id = source_path
            .clone()
            .unwrap_or_else(|| Arc::<str>::from("workspace"));
        let range = crate::workbench::documents::canonical_diagnostics::range_from_legacy(
            byte_range.as_ref(),
            line,
            column,
        );
        Self {
            canonical: crate::workbench::documents::canonical_diagnostics::CanonicalDiagnosticMetadata::current(
                Arc::<str>::from(producer.into()),
                Arc::<str>::from(code.into()),
                document_id,
                range,
                0,
                "unbound-validation",
                message.as_ref(),
            ),
            severity,
            message,
            detail: detail.into(),
            source_path,
            source,
            byte_range,
            line,
            column,
        }
    }

    /// The producer's stable diagnostic code, as shown beside the message.
    ///
    /// A surface never composes this from a phase or a producer name: the code
    /// is identity owned by whatever raised the diagnostic.
    pub fn code(&self) -> &str {
        self.canonical.code.as_ref()
    }

    pub(crate) fn bind_validation(
        mut self,
        document_id: impl Into<Arc<str>>,
        revision: u64,
        validation_id: impl Into<String>,
    ) -> Self {
        let old = self.canonical;
        self.canonical = crate::workbench::documents::canonical_diagnostics::CanonicalDiagnosticMetadata::current(
            old.source.clone(),
            old.code.clone(),
            document_id,
            old.range,
            revision,
            Arc::<str>::from(validation_id.into()),
            self.message.as_ref(),
        );
        self.canonical.related_locations = old.related_locations;
        self.canonical.quick_fixes = old.quick_fixes;
        self.canonical.suppression = old.suppression;
        self.canonical.currentness = old.currentness;
        self.canonical.affected_consumers = old.affected_consumers;
        self
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CodeDiagnosticSummary {
    pub hints: usize,
    pub information: usize,
    pub warnings: usize,
    pub errors: usize,
}

impl CodeDiagnosticSummary {
    pub const fn total(self) -> usize {
        self.hints + self.information + self.warnings + self.errors
    }

    fn record(&mut self, severity: CodeEditorSeverity) {
        match severity {
            CodeEditorSeverity::Hint => self.hints += 1,
            CodeEditorSeverity::Info => self.information += 1,
            CodeEditorSeverity::Warning => self.warnings += 1,
            CodeEditorSeverity::Error => self.errors += 1,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct DocumentDiagnosticIndex {
    summary: CodeDiagnosticSummary,
    record_indices: Vec<usize>,
    severity_by_line: HashMap<usize, CodeEditorSeverity>,
}

/// Bounded canonical collection with publication-time summaries and line
/// indexes. Product surfaces clone an `Arc` to this collection, so a million
/// diagnostics are never copied or rescanned during a frame.
#[derive(Debug, Clone, Default)]
pub struct CodeDiagnosticCollection {
    records: Vec<CodeEditorDiagnostic>,
    summary: CodeDiagnosticSummary,
    global: DocumentDiagnosticIndex,
    documents: HashMap<String, DocumentDiagnosticIndex>,
    interned_strings: HashMap<Arc<str>, Arc<str>>,
    diagnostic_ids: HashSet<uuid::Uuid>,
}

impl CodeDiagnosticCollection {
    pub fn try_new(records: Vec<CodeEditorDiagnostic>) -> Result<Self, String> {
        if records.len() > MAX_CODE_DIAGNOSTICS {
            return Err(format!(
                "Diagnostic collection contains {} records; the supported maximum is {MAX_CODE_DIAGNOSTICS}.",
                records.len()
            ));
        }
        let mut collection = Self::default();
        collection.records.reserve(records.len());
        for record in records {
            collection.push_indexed(record)?;
        }
        Ok(collection)
    }

    pub fn try_push(&mut self, record: CodeEditorDiagnostic) -> Result<(), String> {
        if self.records.len() >= MAX_CODE_DIAGNOSTICS {
            return Err(format!(
                "Diagnostic collection reached the supported maximum of {MAX_CODE_DIAGNOSTICS} records."
            ));
        }
        self.push_indexed(record)
    }

    pub fn clear(&mut self) {
        self.records.clear();
        self.summary = CodeDiagnosticSummary::default();
        self.global = DocumentDiagnosticIndex::default();
        self.documents.clear();
        self.interned_strings.clear();
        self.diagnostic_ids.clear();
    }

    pub fn summary(&self) -> CodeDiagnosticSummary {
        self.summary
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, CodeEditorDiagnostic> {
        self.records.iter()
    }

    pub fn as_slice(&self) -> &[CodeEditorDiagnostic] {
        &self.records
    }

    pub fn view<'a>(&'a self, logical_path: Option<&str>) -> CodeDiagnosticView<'a> {
        CodeDiagnosticView {
            collection: self,
            document_key: logical_path.map(normalize_diagnostic_path),
        }
    }

    fn push_indexed(&mut self, mut record: CodeEditorDiagnostic) -> Result<(), String> {
        if !self.diagnostic_ids.insert(record.canonical.diagnostic_id) {
            return Err(format!(
                "Diagnostic collection contains duplicate canonical ID {}.",
                record.canonical.diagnostic_id
            ));
        }
        let index = self.records.len();
        self.summary.record(record.severity);
        let line = diagnostic_line(&record);
        if record.line.is_none() {
            record.line = line;
        }
        // Source text belongs to the versioned project bundle, not to every
        // retained diagnostic. Line information is materialized above before
        // releasing this transient compiler payload.
        record.source = None;
        intern_diagnostic_strings(&mut self.interned_strings, &mut record);
        let target = match record.source_path.as_deref() {
            Some(path) => self
                .documents
                .entry(normalize_diagnostic_path(path))
                .or_default(),
            None => &mut self.global,
        };
        target.summary.record(record.severity);
        target.record_indices.push(index);
        if let Some(line) = line {
            target
                .severity_by_line
                .entry(line)
                .and_modify(|severity| *severity = (*severity).max(record.severity))
                .or_insert(record.severity);
        }
        self.records.push(record);
        Ok(())
    }
}

fn intern_diagnostic_strings(
    pool: &mut HashMap<Arc<str>, Arc<str>>,
    diagnostic: &mut CodeEditorDiagnostic,
) {
    intern_string(pool, &mut diagnostic.canonical.source);
    intern_string(pool, &mut diagnostic.canonical.code);
    intern_string(pool, &mut diagnostic.canonical.document_id);
    intern_string(pool, &mut diagnostic.canonical.validation_id);
    if let Some(path) = &mut diagnostic.source_path {
        intern_string(pool, path);
    }
}

fn intern_string(pool: &mut HashMap<Arc<str>, Arc<str>>, value: &mut Arc<str>) {
    if let Some(existing) = pool.get(value.as_ref()) {
        *value = Arc::clone(existing);
    } else {
        pool.insert(Arc::clone(value), Arc::clone(value));
    }
}

impl AsRef<[CodeEditorDiagnostic]> for CodeDiagnosticCollection {
    fn as_ref(&self) -> &[CodeEditorDiagnostic] {
        self.as_slice()
    }
}

#[derive(Debug, Clone)]
pub struct CodeDiagnosticView<'a> {
    collection: &'a CodeDiagnosticCollection,
    document_key: Option<String>,
}

impl CodeDiagnosticView<'_> {
    pub fn severity_for_line(&self, one_based_line: usize) -> Option<CodeEditorSeverity> {
        let global = self
            .collection
            .global
            .severity_by_line
            .get(&one_based_line)
            .copied();
        let document = self
            .document_index()
            .and_then(|index| index.severity_by_line.get(&one_based_line).copied());
        global.max(document)
    }

    fn document_index(&self) -> Option<&DocumentDiagnosticIndex> {
        self.document_key
            .as_deref()
            .and_then(|key| self.collection.documents.get(key))
    }
}

fn normalize_diagnostic_path(path: &str) -> String {
    path.replace('\\', "/").to_ascii_lowercase()
}

fn diagnostic_line(diagnostic: &CodeEditorDiagnostic) -> Option<usize> {
    diagnostic.line.or_else(|| {
        diagnostic
            .source
            .as_deref()
            .zip(diagnostic.byte_range.as_ref())
            .map(|(source, range)| byte_offset_line(source, range.start))
    })
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CodeDocumentInteraction {
    pub changed: bool,
    pub cursor_char_index: usize,
    pub selected_char_range: Option<(usize, usize)>,
    pub breakpoint_toggled: Option<usize>,
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
            information: colors.info,
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
                    .map(|severity| {
                        diagnostic_color(severity, colors.err, colors.warn, colors.info)
                    })
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
        let _ = output.response;
        return CodeDocumentInteraction {
            changed: output.changed,
            cursor_char_index: output.cursor_char_index,
            selected_char_range: output.selected_char_range,
            breakpoint_toggled: output.breakpoint_toggled,
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
                    let severity = diagnostic_view.severity_for_line(line_number);
                    let color = severity.map_or(colors.text_faint, |severity| {
                        diagnostic_color(severity, colors.err, colors.warn, colors.info)
                    });
                    painter.text(
                        egui::pos2(
                            output.response.rect.left() + GUTTER_WIDTH - NUMBER_RIGHT_PADDING,
                            y,
                        ),
                        egui::Align2::RIGHT_CENTER,
                        line_number.to_string(),
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
        CodeEditorSeverity::Hint => info,
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
    fn unicode_line_lookup_is_byte_safe() {
        let source = "module µ;\nanalog V(o) <+ 1;\nendmodule\n";
        assert_eq!(byte_offset_line(source, source.find("analog").unwrap()), 2);
    }

    #[test]
    fn line_diagnostics_use_one_based_lines() {
        let diagnostic = CodeEditorDiagnostic::current(
            "rspice.test",
            "TEST-001",
            CodeEditorSeverity::Error,
            "broken",
            "",
            Some("model.va".to_owned()),
            None,
            None,
            Some(2),
            Some(1),
        );
        let diagnostics = CodeDiagnosticCollection::try_new(vec![diagnostic]).unwrap();
        assert_eq!(
            diagnostics.view(Some("MODEL.VA")).severity_for_line(2),
            Some(CodeEditorSeverity::Error)
        );
        assert_eq!(
            diagnostics.view(Some("other.va")).severity_for_line(2),
            None
        );
    }

    #[test]
    fn diagnostic_collection_summarizes_without_frame_time_rescans() {
        let diagnostics = CodeDiagnosticCollection::try_new(vec![
            CodeEditorDiagnostic::current(
                "rspice.test",
                "TEST-ERROR",
                CodeEditorSeverity::Error,
                "error",
                "",
                None,
                None,
                None,
                None,
                None,
            ),
            CodeEditorDiagnostic::current(
                "rspice.test",
                "TEST-WARNING",
                CodeEditorSeverity::Warning,
                "warning",
                "",
                None,
                None,
                None,
                None,
                None,
            ),
        ])
        .unwrap();
        assert_eq!(diagnostics.summary().errors, 1);
        assert_eq!(diagnostics.summary().warnings, 1);
        assert_eq!(diagnostics.summary().total(), 2);
    }

    #[test]
    fn diagnostic_collection_compacts_identity_and_releases_embedded_source() {
        let records = (0..2)
            .map(|index| {
                CodeEditorDiagnostic::current(
                    "rspice.test",
                    "TEST-COMPACT",
                    CodeEditorSeverity::Warning,
                    format!("message {index}"),
                    "detail",
                    Some("models/compact.va".to_owned()),
                    Some("first\nsecond\n".to_owned()),
                    Some(6..12),
                    None,
                    Some(1),
                )
                .bind_validation("models/compact.va", 7, "validation-7")
            })
            .collect::<Vec<_>>();
        let diagnostics = CodeDiagnosticCollection::try_new(records).unwrap();
        let first = &diagnostics.as_slice()[0];
        let second = &diagnostics.as_slice()[1];

        assert_eq!(first.line, Some(2));
        assert!(first.source.is_none());
        assert!(second.source.is_none());
        assert!(Arc::ptr_eq(
            &first.canonical.source,
            &second.canonical.source
        ));
        assert!(Arc::ptr_eq(&first.canonical.code, &second.canonical.code));
        assert!(Arc::ptr_eq(
            &first.canonical.document_id,
            &second.canonical.document_id
        ));
        assert!(Arc::ptr_eq(
            &first.canonical.validation_id,
            &second.canonical.validation_id
        ));
        assert!(Arc::ptr_eq(
            first.source_path.as_ref().unwrap(),
            second.source_path.as_ref().unwrap()
        ));
    }

    #[test]
    fn diagnostic_collection_rejects_duplicate_canonical_identity() {
        let diagnostic = CodeEditorDiagnostic::current(
            "rspice.test",
            "TEST-DUPLICATE",
            CodeEditorSeverity::Error,
            "duplicate",
            "detail",
            None,
            None,
            None,
            None,
            None,
        );
        let error = CodeDiagnosticCollection::try_new(vec![diagnostic.clone(), diagnostic])
            .expect_err("duplicate canonical IDs must fail closed");
        assert!(error.contains("duplicate canonical ID"));
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
