//! Deterministic, portable PDF rendering for shortcut reference artifacts.
//!
//! Rendering consumes only [`ShortcutReferenceModel`]. It never consults live
//! preferences, the filesystem, the clock, or platform APIs, so the same model
//! produces the same bytes on native and WebAssembly targets.

use std::fmt;

use krilla::color::rgb;
use krilla::geom::{PathBuilder, Point, Rect};
use krilla::metadata::{Metadata, PageLayout};
use krilla::page::PageSettings;
use krilla::paint::Fill;
use krilla::text::{Font, TextDirection};
use krilla::{Document, SerializeSettings};
use unicode_segmentation::UnicodeSegmentation;

use super::projection::{ShortcutReferenceModel, ShortcutReferenceRow, ShortcutReferenceStatus};
use super::schema::ShortcutArtifactScope;
use super::{hex_digest, sha256};
use crate::workbench::commands::CommandPlatform;
use crate::workbench::shortcuts::ShortcutBindingSlot;

const PAGE_WIDTH: f32 = 595.28;
const PAGE_HEIGHT: f32 = 841.89;
const MARGIN_X: f32 = 36.0;
const BODY_TOP: f32 = 67.0;
const BODY_BOTTOM: f32 = 800.0;
const CONTENT_WIDTH: f32 = PAGE_WIDTH - 2.0 * MARGIN_X;
const TABLE_HEADER_HEIGHT: f32 = 22.0;
const TABLE_ROW_MIN_HEIGHT: f32 = 24.0;
const TABLE_LINE_HEIGHT: f32 = 9.5;
const TABLE_FONT_SIZE: f32 = 7.5;
const CELL_PADDING_X: f32 = 5.0;
const CELL_PADDING_Y: f32 = 5.5;
const COLUMN_WIDTHS: [f32; 6] = [88.0, 60.0, 142.0, 92.0, 67.0, 74.28];

const IBM_PLEX_SANS_REGULAR: &[u8] =
    include_bytes!("../../../assets/fonts/IBMPlexSans-Regular.ttf");
const IBM_PLEX_SANS_SEMIBOLD: &[u8] =
    include_bytes!("../../../assets/fonts/IBMPlexSans-SemiBold.ttf");
const IBM_PLEX_MONO_REGULAR: &[u8] =
    include_bytes!("../../../assets/fonts/IBMPlexMono-Regular.ttf");

/// PDF serialization failure with a stable, user-presentable description.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutPdfError(String);

impl fmt::Display for ShortcutPdfError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl std::error::Error for ShortcutPdfError {}

/// Render a deterministic PDF shortcut reference from an immutable projection.
///
/// The document embeds the repository IBM Plex faces, repeats its table header
/// on every table page, and includes no timestamps, source paths, recent state,
/// project identity, credentials, or protected-override acknowledgements.
pub fn serialize_shortcut_reference_pdf(
    model: &ShortcutReferenceModel,
) -> Result<Vec<u8>, ShortcutPdfError> {
    let fonts = Fonts::load()?;
    let pages = plan_document(model);
    let page_count = pages.len();

    let settings = SerializeSettings {
        pretty: true,
        compress_content_streams: false,
        xmp_metadata: false,
        enable_tagging: false,
        ..SerializeSettings::default()
    };

    let mut document = Document::new_with(settings);
    document.set_metadata(
        Metadata::new()
            .title("RSpice keyboard shortcuts".to_owned())
            .description(
                "Portable keyboard shortcut reference generated from an immutable RSpice shortcut artifact."
                    .to_owned(),
            )
            .creator("RSpice".to_owned())
            .producer("RSpice deterministic shortcut PDF exporter".to_owned())
            .document_id(document_id(model))
            .page_layout(PageLayout::OneColumn),
    );

    let page_settings = PageSettings::from_wh(PAGE_WIDTH, PAGE_HEIGHT)
        .ok_or_else(|| ShortcutPdfError("invalid PDF page dimensions".to_owned()))?;
    for (index, plan) in pages.iter().enumerate() {
        let mut page = document.start_page_with(page_settings.clone());
        let mut surface = page.surface();
        draw_page_chrome(&mut surface, &fonts, index + 1, page_count);
        draw_intro(&mut surface, &fonts, &plan.intro);
        if let Some(y) = plan.table_title_y {
            draw_text(
                &mut surface,
                &fonts.sans_semibold,
                9.0,
                MARGIN_X,
                y,
                "SHORTCUT REFERENCE",
                Color::Muted,
            );
        }
        if let Some(y) = plan.table_header_y {
            draw_table_header(&mut surface, &fonts, y);
        }
        for row in &plan.rows {
            draw_table_row(&mut surface, &fonts, row);
        }
        surface.finish();
        page.finish();
    }

    document
        .finish()
        .map_err(|error| ShortcutPdfError(format!("could not serialize shortcut PDF: {error}")))
}

#[derive(Clone)]
struct Fonts {
    sans_regular: Font,
    sans_semibold: Font,
    mono_regular: Font,
}

impl Fonts {
    fn load() -> Result<Self, ShortcutPdfError> {
        Ok(Self {
            sans_regular: load_font(IBM_PLEX_SANS_REGULAR, "IBM Plex Sans Regular")?,
            sans_semibold: load_font(IBM_PLEX_SANS_SEMIBOLD, "IBM Plex Sans SemiBold")?,
            mono_regular: load_font(IBM_PLEX_MONO_REGULAR, "IBM Plex Mono Regular")?,
        })
    }
}

fn load_font(bytes: &'static [u8], name: &str) -> Result<Font, ShortcutPdfError> {
    Font::new(bytes.into(), 0)
        .ok_or_else(|| ShortcutPdfError(format!("embedded {name} font is invalid")))
}

#[derive(Default)]
struct PlannedPage {
    intro: Vec<PlacedText>,
    table_title_y: Option<f32>,
    table_header_y: Option<f32>,
    rows: Vec<PlacedRow>,
}

struct PlacedText {
    x: f32,
    baseline_y: f32,
    text: String,
    style: TextStyle,
}

struct PlacedRow {
    top: f32,
    height: f32,
    cells: Vec<PreparedCell>,
    shaded: bool,
}

#[derive(Clone, Copy)]
enum TextStyle {
    Section,
    Body,
    Mono,
    Notice,
}

struct PreparedCell {
    lines: Vec<CellLine>,
}

struct CellLine {
    text: String,
    mono: bool,
    muted: bool,
}

fn plan_document(model: &ShortcutReferenceModel) -> Vec<PlannedPage> {
    let mut pages = vec![PlannedPage::default()];
    let mut y = BODY_TOP;
    for line in intro_lines(model) {
        if y + line.height > BODY_BOTTOM {
            pages.push(PlannedPage::default());
            y = BODY_TOP;
        }
        if !line.text.is_empty() {
            pages
                .last_mut()
                .expect("page exists")
                .intro
                .push(PlacedText {
                    x: MARGIN_X + line.indent,
                    baseline_y: y + line.baseline_offset,
                    text: line.text,
                    style: line.style,
                });
        }
        y += line.height;
    }

    const TABLE_TITLE_BLOCK: f32 = 23.0;
    if y + TABLE_TITLE_BLOCK + TABLE_HEADER_HEIGHT + TABLE_ROW_MIN_HEIGHT > BODY_BOTTOM {
        pages.push(PlannedPage::default());
        y = BODY_TOP;
    }
    let page = pages.last_mut().expect("page exists");
    page.table_title_y = Some(y + 9.0);
    y += TABLE_TITLE_BLOCK;
    page.table_header_y = Some(y);
    y += TABLE_HEADER_HEIGHT;

    for (index, row) in model.rows().iter().enumerate() {
        let cells = prepare_cells(row);
        let line_count = cells.iter().map(|cell| cell.lines.len()).max().unwrap_or(1);
        let height =
            TABLE_ROW_MIN_HEIGHT.max(2.0 * CELL_PADDING_Y + line_count as f32 * TABLE_LINE_HEIGHT);
        if y + height > BODY_BOTTOM {
            pages.push(PlannedPage {
                table_header_y: Some(BODY_TOP),
                ..PlannedPage::default()
            });
            y = BODY_TOP + TABLE_HEADER_HEIGHT;
        }
        pages.last_mut().expect("page exists").rows.push(PlacedRow {
            top: y,
            height,
            cells,
            shaded: index % 2 == 1,
        });
        y += height;
    }

    pages
}

struct IntroLine {
    text: String,
    style: TextStyle,
    indent: f32,
    height: f32,
    baseline_offset: f32,
}

fn intro_lines(model: &ShortcutReferenceModel) -> Vec<IntroLine> {
    let manifest = model.manifest();
    let contexts = manifest.coverage.contexts.join(", ");
    let platforms = manifest
        .coverage
        .platforms
        .iter()
        .map(|platform| platform.label())
        .collect::<Vec<_>>()
        .join(", ");
    let mappings = if manifest.platform_mappings_included {
        "Explicit platform mappings included"
    } else {
        "Current platform mapping only"
    };

    let mut lines = Vec::new();
    push_intro_heading(&mut lines, "REFERENCE DETAILS");
    push_intro_wrapped(
        &mut lines,
        &format!("Schema: rspice.shortcuts/{}", manifest.schema_version),
        TextStyle::Mono,
        0.0,
    );
    push_intro_wrapped(
        &mut lines,
        &format!("Scope: {}", scope_label(manifest.scope)),
        TextStyle::Body,
        0.0,
    );
    push_intro_wrapped(
        &mut lines,
        &format!("Context coverage: {contexts}"),
        TextStyle::Body,
        0.0,
    );
    push_intro_wrapped(
        &mut lines,
        &format!("Platform coverage: {platforms}"),
        TextStyle::Body,
        0.0,
    );
    push_intro_wrapped(&mut lines, mappings, TextStyle::Body, 0.0);
    push_intro_wrapped(
        &mut lines,
        &format!("Reference rows: {}", model.rows().len()),
        TextStyle::Body,
        0.0,
    );
    if manifest.unknown_commands_omitted > 0 {
        push_intro_wrapped(
            &mut lines,
            &format!(
                "Unknown future commands omitted: {}",
                manifest.unknown_commands_omitted
            ),
            TextStyle::Body,
            0.0,
        );
    }

    push_intro_spacer(&mut lines, 6.0);
    push_intro_heading(&mut lines, "PORTABILITY AND PRIVACY");
    push_intro_wrapped(
        &mut lines,
        "This reference contains shortcut bindings and execution policies only. It omits credentials, source paths, project identity, recent activity, automation state, and protected-override acknowledgements.",
        TextStyle::Notice,
        0.0,
    );

    push_intro_spacer(&mut lines, 6.0);
    push_intro_heading(&mut lines, "EXECUTION POLICIES");
    if model.policy_summary().is_empty() {
        push_intro_wrapped(
            &mut lines,
            "Materialized bindings; no execution policy records are embedded.",
            TextStyle::Body,
            0.0,
        );
    } else {
        for (name, value) in model.policy_summary() {
            push_intro_wrapped(
                &mut lines,
                &format!("{}: {}", clean_text(name), clean_text(value)),
                TextStyle::Mono,
                0.0,
            );
        }
    }
    push_intro_spacer(&mut lines, 4.0);
    lines
}

fn push_intro_heading(lines: &mut Vec<IntroLine>, text: &str) {
    lines.push(IntroLine {
        text: text.to_owned(),
        style: TextStyle::Section,
        indent: 0.0,
        height: 17.0,
        baseline_offset: 10.0,
    });
}

fn push_intro_spacer(lines: &mut Vec<IntroLine>, height: f32) {
    lines.push(IntroLine {
        text: String::new(),
        style: TextStyle::Body,
        indent: 0.0,
        height,
        baseline_offset: 0.0,
    });
}

fn push_intro_wrapped(lines: &mut Vec<IntroLine>, text: &str, style: TextStyle, indent: f32) {
    let (size, mono) = match style {
        TextStyle::Mono => (8.0, true),
        TextStyle::Notice => (8.3, false),
        TextStyle::Section | TextStyle::Body => (8.5, false),
    };
    for wrapped in wrap_text(text, CONTENT_WIDTH - indent, size, mono) {
        lines.push(IntroLine {
            text: wrapped,
            style,
            indent,
            height: 12.0,
            baseline_offset: 8.7,
        });
    }
}

fn prepare_cells(row: &ShortcutReferenceRow) -> Vec<PreparedCell> {
    let binding_slot = row.slot.map_or("Unbound", |slot| match slot {
        ShortcutBindingSlot::Primary => "Primary",
        ShortcutBindingSlot::Alternate => "Alternate",
    });
    let binding = row
        .sequence
        .as_ref()
        .map_or("-", |_| row.display_sequence.as_str());
    let platform = row.platform.map_or("All covered", CommandPlatform::label);

    vec![
        text_cell(&row.context, 0, false, false),
        text_cell(&row.group, 1, false, false),
        command_cell(&row.command_label, &row.command_id),
        multi_style_cell(&[(binding_slot, false, true), (binding, true, false)], 3),
        text_cell(platform, 4, false, false),
        text_cell(status_label(row.status), 5, false, false),
    ]
}

fn command_cell(label: &str, stable_id: &str) -> PreparedCell {
    multi_style_cell(&[(label, false, false), (stable_id, true, true)], 2)
}

fn text_cell(text: &str, column: usize, mono: bool, muted: bool) -> PreparedCell {
    multi_style_cell(&[(text, mono, muted)], column)
}

fn multi_style_cell(parts: &[(&str, bool, bool)], column: usize) -> PreparedCell {
    let width = COLUMN_WIDTHS[column] - 2.0 * CELL_PADDING_X;
    let mut lines = Vec::new();
    for (text, mono, muted) in parts {
        lines.extend(
            wrap_text(text, width, TABLE_FONT_SIZE, *mono)
                .into_iter()
                .map(|text| CellLine {
                    text,
                    mono: *mono,
                    muted: *muted,
                }),
        );
    }
    if lines.is_empty() {
        lines.push(CellLine {
            text: "-".to_owned(),
            mono: false,
            muted: false,
        });
    }
    PreparedCell { lines }
}

fn draw_page_chrome(
    surface: &mut krilla::surface::Surface<'_>,
    fonts: &Fonts,
    page_number: usize,
    page_count: usize,
) {
    fill_rect(surface, 0.0, 0.0, PAGE_WIDTH, PAGE_HEIGHT, Color::Paper);
    fill_rect(surface, MARGIN_X, 29.0, 4.0, 18.0, Color::Accent);
    draw_text(
        surface,
        &fonts.sans_semibold,
        13.0,
        MARGIN_X + 12.0,
        43.5,
        "RSpice keyboard shortcuts",
        Color::Ink,
    );
    fill_rect(surface, MARGIN_X, 55.0, CONTENT_WIDTH, 0.7, Color::Rule);
    fill_rect(surface, MARGIN_X, 812.0, CONTENT_WIDTH, 0.7, Color::Rule);
    draw_text(
        surface,
        &fonts.sans_regular,
        7.2,
        MARGIN_X,
        827.0,
        "Portable shortcut reference",
        Color::Muted,
    );
    draw_text(
        surface,
        &fonts.mono_regular,
        7.2,
        PAGE_WIDTH - MARGIN_X - 55.0,
        827.0,
        &format!("Page {page_number} of {page_count}"),
        Color::Muted,
    );
}

fn draw_intro(surface: &mut krilla::surface::Surface<'_>, fonts: &Fonts, lines: &[PlacedText]) {
    for line in lines {
        let (font, size, color) = match line.style {
            TextStyle::Section => (&fonts.sans_semibold, 8.5, Color::Muted),
            TextStyle::Body => (&fonts.sans_regular, 8.5, Color::Ink),
            TextStyle::Mono => (&fonts.mono_regular, 8.0, Color::Ink),
            TextStyle::Notice => (&fonts.sans_regular, 8.3, Color::Muted),
        };
        draw_text(
            surface,
            font,
            size,
            line.x,
            line.baseline_y,
            &line.text,
            color,
        );
    }
}

fn draw_table_header(surface: &mut krilla::surface::Surface<'_>, fonts: &Fonts, top: f32) {
    fill_rect(
        surface,
        MARGIN_X,
        top,
        CONTENT_WIDTH,
        TABLE_HEADER_HEIGHT,
        Color::Header,
    );
    let labels = [
        "CONTEXT", "GROUP", "COMMAND", "BINDING", "PLATFORM", "STATUS",
    ];
    let mut x = MARGIN_X;
    for (label, width) in labels.into_iter().zip(COLUMN_WIDTHS) {
        draw_text(
            surface,
            &fonts.sans_semibold,
            7.0,
            x + CELL_PADDING_X,
            top + 14.0,
            label,
            Color::Paper,
        );
        x += width;
    }
}

fn draw_table_row(surface: &mut krilla::surface::Surface<'_>, fonts: &Fonts, row: &PlacedRow) {
    if row.shaded {
        fill_rect(
            surface,
            MARGIN_X,
            row.top,
            CONTENT_WIDTH,
            row.height,
            Color::Stripe,
        );
    }
    fill_rect(
        surface,
        MARGIN_X,
        row.top + row.height - 0.5,
        CONTENT_WIDTH,
        0.5,
        Color::Rule,
    );

    let mut x = MARGIN_X;
    for (cell, width) in row.cells.iter().zip(COLUMN_WIDTHS) {
        for (line_index, line) in cell.lines.iter().enumerate() {
            let font = if line.mono {
                &fonts.mono_regular
            } else {
                &fonts.sans_regular
            };
            draw_text(
                surface,
                font,
                TABLE_FONT_SIZE,
                x + CELL_PADDING_X,
                row.top + CELL_PADDING_Y + 6.2 + line_index as f32 * TABLE_LINE_HEIGHT,
                &line.text,
                if line.muted { Color::Muted } else { Color::Ink },
            );
        }
        x += width;
    }
}

fn fill_rect(
    surface: &mut krilla::surface::Surface<'_>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Color,
) {
    let Some(rect) = Rect::from_xywh(x, y, width, height) else {
        return;
    };
    let mut builder = PathBuilder::new();
    builder.push_rect(rect);
    let Some(path) = builder.finish() else {
        return;
    };
    surface.set_stroke(None);
    surface.set_fill(Some(Fill {
        paint: color.rgb().into(),
        ..Fill::default()
    }));
    surface.draw_path(&path);
}

fn draw_text(
    surface: &mut krilla::surface::Surface<'_>,
    font: &Font,
    size: f32,
    x: f32,
    y: f32,
    text: &str,
    color: Color,
) {
    surface.set_stroke(None);
    surface.set_fill(Some(Fill {
        paint: color.rgb().into(),
        ..Fill::default()
    }));
    surface.draw_text(
        Point::from_xy(x, y),
        font.clone(),
        size,
        text,
        false,
        TextDirection::LeftToRight,
    );
}

#[derive(Clone, Copy)]
enum Color {
    Ink,
    Muted,
    Accent,
    Header,
    Stripe,
    Rule,
    Paper,
}

impl Color {
    fn rgb(self) -> rgb::Color {
        match self {
            Self::Ink => rgb::Color::new(25, 32, 38),
            Self::Muted => rgb::Color::new(92, 103, 112),
            Self::Accent => rgb::Color::new(245, 183, 24),
            Self::Header => rgb::Color::new(34, 44, 52),
            Self::Stripe => rgb::Color::new(246, 248, 249),
            Self::Rule => rgb::Color::new(210, 216, 220),
            Self::Paper => rgb::Color::new(255, 255, 255),
        }
    }
}

fn wrap_text(text: &str, max_width: f32, font_size: f32, mono: bool) -> Vec<String> {
    let cleaned = clean_text(text);
    if cleaned.is_empty() {
        return vec!["-".to_owned()];
    }

    let mut lines = Vec::new();
    let mut current = String::new();
    for word in cleaned.split_whitespace() {
        let candidate = if current.is_empty() {
            word.to_owned()
        } else {
            format!("{current} {word}")
        };
        if text_width(&candidate, font_size, mono) <= max_width {
            current = candidate;
            continue;
        }
        if !current.is_empty() {
            lines.push(std::mem::take(&mut current));
        }
        if text_width(word, font_size, mono) <= max_width {
            current.push_str(word);
            continue;
        }

        let mut segment = String::new();
        for grapheme in word.graphemes(true) {
            let candidate = format!("{segment}{grapheme}");
            if !segment.is_empty() && text_width(&candidate, font_size, mono) > max_width {
                lines.push(std::mem::take(&mut segment));
            }
            segment.push_str(grapheme);
        }
        current = segment;
    }
    if !current.is_empty() {
        lines.push(current);
    }
    if lines.is_empty() {
        lines.push("-".to_owned());
    }
    lines
}

fn clean_text(value: &str) -> String {
    value
        .chars()
        .map(|character| match character {
            '\r' | '\n' | '\t' => ' ',
            character if character.is_control() => ' ',
            '\u{2010}' | '\u{2011}' | '\u{2012}' | '\u{2013}' | '\u{2014}' | '\u{2212}' => '-',
            character => character,
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn text_width(value: &str, font_size: f32, mono: bool) -> f32 {
    value
        .graphemes(true)
        .map(|grapheme| {
            if mono {
                0.6
            } else {
                match grapheme {
                    " " => 0.28,
                    "i" | "l" | "I" | "." | "," | ":" | ";" | "'" | "!" | "|" => 0.29,
                    "m" | "w" | "M" | "W" | "@" | "%" | "&" => 0.82,
                    _ if grapheme.is_ascii() => 0.54,
                    _ => 0.64,
                }
            }
        })
        .sum::<f32>()
        * font_size
}

fn document_id(model: &ShortcutReferenceModel) -> String {
    let manifest = model.manifest();
    let mut input = format!(
        "rspice.shortcuts/{}|{}|{}|{}|",
        manifest.schema_version,
        scope_label(manifest.scope),
        manifest.platform_mappings_included,
        manifest.unknown_commands_omitted,
    );
    for context in &manifest.coverage.contexts {
        input.push_str(context);
        input.push('|');
    }
    for platform in &manifest.coverage.platforms {
        input.push_str(platform.label());
        input.push('|');
    }
    for row in model.rows() {
        input.push_str(&row.context);
        input.push('|');
        input.push_str(&row.group);
        input.push('|');
        input.push_str(&row.command_id);
        input.push('|');
        input.push_str(&row.command_label);
        input.push('|');
        input.push_str(&row.display_sequence);
        input.push('|');
        input.push_str(status_label(row.status));
        input.push('|');
    }
    for (name, value) in model.policy_summary() {
        input.push_str(name);
        input.push('=');
        input.push_str(value);
        input.push('|');
    }
    format!("rspice-shortcuts-{}", hex_digest(sha256(input.as_bytes())))
}

const fn scope_label(scope: ShortcutArtifactScope) -> &'static str {
    match scope {
        ShortcutArtifactScope::UserOverrides => "User overrides and platform exceptions",
        ShortcutArtifactScope::CompleteResolved => "Complete resolved shortcut map",
        ShortcutArtifactScope::CurrentWorkspace => "Current workspace context",
    }
}

const fn status_label(status: ShortcutReferenceStatus) -> &'static str {
    match status {
        ShortcutReferenceStatus::Default => "Default",
        ShortcutReferenceStatus::UserOverride => "User override",
        ShortcutReferenceStatus::Unbound => "Unbound",
    }
}

#[cfg(test)]
mod tests {
    use egui::os::OperatingSystem;
    use lopdf::{Document as ParsedPdf, Object};

    use super::*;
    use crate::common::shortcut_artifacts::projection::{
        ShortcutExportRequest, ShortcutExportScope, build_shortcut_reference_model,
    };
    use crate::workbench::ShortcutPreferences;
    use crate::workbench::commands::Command;

    fn complete_model() -> ShortcutReferenceModel {
        build_shortcut_reference_model(
            &ShortcutPreferences::default(),
            &ShortcutExportRequest {
                scope: ShortcutExportScope::CompleteResolved,
                include_platform_mappings: false,
                runtime_platform: CommandPlatform::Desktop,
                operating_system: OperatingSystem::Windows,
                current_contexts: Vec::new(),
            },
        )
        .unwrap()
    }

    #[test]
    fn pdf_is_byte_deterministic_and_has_no_time_metadata() {
        let model = complete_model();
        let first = serialize_shortcut_reference_pdf(&model).unwrap();
        let second = serialize_shortcut_reference_pdf(&model).unwrap();
        assert_eq!(first, second);

        let parsed = ParsedPdf::load_mem(&first).unwrap();
        let info = parsed
            .trailer
            .get(b"Info")
            .ok()
            .and_then(|object| object.as_reference().ok())
            .and_then(|id| parsed.get_object(id).ok())
            .and_then(|object| object.as_dict().ok());
        if let Some(info) = info {
            assert!(!info.has(b"CreationDate"));
            assert!(!info.has(b"ModDate"));
        }
    }

    #[test]
    fn pdf_parser_recovers_structure_text_and_embedded_fonts() {
        let pdf = serialize_shortcut_reference_pdf(&complete_model()).unwrap();
        let parsed = ParsedPdf::load_mem(&pdf).unwrap();
        let pages = parsed.get_pages();
        assert!(
            pages.len() > 1,
            "default resolved map must exercise pagination"
        );

        let text = parsed
            .extract_text(&pages.keys().copied().collect::<Vec<_>>())
            .unwrap();
        assert!(text.contains("RSpice keyboard shortcuts"));
        assert!(text.contains("save-project"));
        assert!(text.contains("Ctrl+S"));
        assert!(text.contains("Page 1 of"));

        let raw = String::from_utf8_lossy(&pdf);
        assert!(raw.contains("/FontFile2"));
        assert!(raw.contains("IBMPlexSans"));
        assert!(raw.contains("IBMPlexMono"));
    }

    #[test]
    fn every_multipage_table_page_repeats_column_headers() {
        let pdf = serialize_shortcut_reference_pdf(&complete_model()).unwrap();
        let parsed = ParsedPdf::load_mem(&pdf).unwrap();
        let pages = parsed.get_pages();
        assert!(pages.len() > 2);
        for page_number in pages.keys().copied() {
            let text = parsed.extract_text(&[page_number]).unwrap();
            if text.contains("save-project")
                || text.contains("SHORTCUT REFERENCE")
                || text.contains("CONTEXT")
            {
                for header in [
                    "CONTEXT", "GROUP", "COMMAND", "BINDING", "PLATFORM", "STATUS",
                ] {
                    assert!(
                        text.contains(header),
                        "page {page_number} is missing repeated table header {header}"
                    );
                }
            }
        }
    }

    #[test]
    fn pdf_excludes_private_profile_state_and_filesystem_paths() {
        let mut profile = ShortcutPreferences::default();
        profile.acknowledge_protected_override(Command::Save);
        let model = build_shortcut_reference_model(
            &profile,
            &ShortcutExportRequest {
                scope: ShortcutExportScope::UserOverrides,
                include_platform_mappings: true,
                runtime_platform: CommandPlatform::Desktop,
                operating_system: OperatingSystem::Windows,
                current_contexts: Vec::new(),
            },
        )
        .unwrap();
        let pdf = serialize_shortcut_reference_pdf(&model).unwrap();
        let parsed = ParsedPdf::load_mem(&pdf).unwrap();
        let page_numbers = parsed.get_pages().keys().copied().collect::<Vec<_>>();
        let text = parsed.extract_text(&page_numbers).unwrap();
        let raw = String::from_utf8_lossy(&pdf);
        for private in [
            "protected-override-acknowledgements",
            "source-path",
            "project-identity",
            "recent-commands",
            "C:\\\\",
            "/Users/",
            "/home/",
        ] {
            assert!(!text.contains(private));
            assert!(!raw.contains(private));
        }
    }

    #[test]
    fn page_tree_contains_only_page_objects_and_numbered_footers() {
        let pdf = serialize_shortcut_reference_pdf(&complete_model()).unwrap();
        let parsed = ParsedPdf::load_mem(&pdf).unwrap();
        let pages = parsed.get_pages();
        let page_count = pages.len();
        for (page_number, object_id) in pages {
            let page = parsed.get_object(object_id).unwrap().as_dict().unwrap();
            assert_eq!(page.get(b"Type").unwrap(), &Object::Name(b"Page".to_vec()));
            let text = parsed.extract_text(&[page_number]).unwrap();
            assert!(text.contains(&format!("Page {page_number} of {page_count}")));
        }
    }

    #[test]
    fn wrapping_normalizes_controls_and_prevents_overwide_tokens() {
        let lines = wrap_text("alpha\nvery-long-token-with-many-segments", 42.0, 8.0, true);
        assert!(lines.len() > 2);
        assert!(lines.iter().all(|line| !line.contains('\n')));
        assert!(lines.iter().all(|line| text_width(line, 8.0, true) <= 42.0));
    }
}
