//! The event-marker strip: HDL severity records and assertion violations on
//! the strip's time axis.
//!
//! One thin strip above the panes, and only when the run has a marker. A
//! record draws a glyph by severity, painted as a vector (the interface font
//! has no symbol glyphs to borrow). Hovering a marker gives its text and its
//! `file:line`; clicking one moves the cursor to it and asks the host to
//! open its source line. An assertion violation is a shaded span here and on
//! its operand's lane. `$display` and `$info` are Console output, not markers.

use super::plan::TimeMapping;
use egui::{Color32, Pos2, Rect, Sense, Shape, Stroke, Ui, pos2, vec2};
use rspice_core::config::{HdlOutputRecord, HdlSeverity, HdlStream};
use rspice_ui_kit::tokens::Tokens;

/// Height of the marker strip, in points.
pub const MARKER_STRIP_HEIGHT: f32 = 14.0;
/// How far from a glyph's centre the pointer still picks it.
const PICK_RADIUS: f32 = 5.0;
const GLYPH_RADIUS: f32 = 4.0;
/// How strongly a violation span shades.
const SPAN_FILL: f32 = 0.28;

/// What an HDL record reports, in the order a marker outranks another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MarkerSeverity {
    Finish,
    Stop,
    Warning,
    Error,
    Fatal,
}

impl MarkerSeverity {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Finish => "$finish",
            Self::Stop => "$stop",
            Self::Warning => "Warning",
            Self::Error => "Error",
            Self::Fatal => "Fatal",
        }
    }
}

/// A source line a marker was written at.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLine {
    pub file: String,
    /// One-based.
    pub line: u32,
}

impl std::fmt::Display for SourceLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.file, self.line)
    }
}

/// One HDL severity or end-of-run record on the time axis.
#[derive(Debug, Clone, PartialEq)]
pub struct HdlMarker {
    pub time: f64,
    pub severity: MarkerSeverity,
    pub text: String,
    /// The hierarchical scope that ran the call.
    pub scope: String,
    pub source: Option<SourceLine>,
}

impl HdlMarker {
    /// The marker an HDL output record makes: `$fatal`, `$error` and
    /// `$warning` records, and `$finish` and `$stop`. Every other record is
    /// Console output and makes none.
    pub fn from_record(record: &HdlOutputRecord) -> Option<Self> {
        let severity = match record.stream {
            HdlStream::Diagnostic {
                severity: HdlSeverity::Fatal,
            } => MarkerSeverity::Fatal,
            HdlStream::Diagnostic {
                severity: HdlSeverity::Error,
            } => MarkerSeverity::Error,
            HdlStream::Diagnostic {
                severity: HdlSeverity::Warning,
            } => MarkerSeverity::Warning,
            HdlStream::Finish { stop: false, .. } => MarkerSeverity::Finish,
            HdlStream::Finish { stop: true, .. } => MarkerSeverity::Stop,
            _ => return None,
        };
        let origin = &record.origin;
        Some(Self {
            time: record.seconds,
            severity,
            text: record.text.trim_end().to_owned(),
            scope: origin.scope.clone(),
            source: (!origin.file.is_empty() && origin.line > 0).then(|| SourceLine {
                file: origin.file.clone(),
                line: origin.line,
            }),
        })
    }

    /// The hover text: what, when, and where it was written.
    pub fn describe(&self, time: &str) -> String {
        let mut text = format!("{} at {time}", self.severity.label());
        if !self.text.is_empty() {
            text.push_str(": ");
            text.push_str(&self.text);
        }
        if !self.scope.is_empty() {
            text.push_str("\nin ");
            text.push_str(&self.scope);
        }
        if let Some(source) = &self.source {
            text.push('\n');
            text.push_str(&source.to_string());
        }
        text
    }
}

/// One interval over which an assertion failed.
#[derive(Debug, Clone, PartialEq)]
pub struct AssertionSpan {
    pub start: f64,
    pub end: f64,
    /// The lane name of the operand the assertion reads.
    pub operand: String,
    pub label: String,
    /// An error-level violation; otherwise a warning or a notice.
    pub error: bool,
}

/// Everything one frame of the marker strip draws from.
pub struct MarkerStripInput<'a> {
    /// Records in time order.
    pub hdl: &'a [HdlMarker],
    pub assertions: &'a [AssertionSpan],
    pub x_range: (f64, f64),
    pub left_margin: f32,
    pub right_margin: f32,
}

/// The strip's height: zero when the run has no marker, so it takes no room.
pub fn marker_strip_height(input: &MarkerStripInput<'_>) -> f32 {
    if input.hdl.is_empty() && input.assertions.is_empty() {
        0.0
    } else {
        MARKER_STRIP_HEIGHT
    }
}

/// What the reader asked of the marker strip this frame.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MarkerStripResponse {
    /// Move the time cursor to the picked marker's instant.
    pub cursor_to: Option<f64>,
    /// Open the picked record's source line.
    pub open_source: Option<SourceLine>,
    /// Select the picked assertion's Problems row (index into the input).
    pub select_assertion: Option<usize>,
}

/// The colour a severity's glyph and span are drawn in: the kit's own status
/// tokens, and the dim text tone for an end of run.
fn severity_color(severity: MarkerSeverity, tokens: &Tokens) -> Color32 {
    match severity {
        MarkerSeverity::Fatal | MarkerSeverity::Error => tokens.color.err,
        MarkerSeverity::Warning => tokens.color.warn,
        MarkerSeverity::Finish | MarkerSeverity::Stop => tokens.color.text_dim,
    }
}

/// Paint one severity glyph centred at `centre`.
pub fn paint_marker_glyph(
    painter: &egui::Painter,
    centre: Pos2,
    severity: MarkerSeverity,
    color: Color32,
) {
    let r = GLYPH_RADIUS;
    let (x, y) = (centre.x, centre.y);
    match severity {
        // A filled diamond: the run ended on it.
        MarkerSeverity::Fatal => {
            painter.add(Shape::convex_polygon(
                vec![
                    pos2(x, y - r),
                    pos2(x + r, y),
                    pos2(x, y + r),
                    pos2(x - r, y),
                ],
                color,
                Stroke::NONE,
            ));
        }
        // A filled triangle pointing down at the instant.
        MarkerSeverity::Error => {
            painter.add(Shape::convex_polygon(
                vec![
                    pos2(x - r, y - r * 0.8),
                    pos2(x + r, y - r * 0.8),
                    pos2(x, y + r),
                ],
                color,
                Stroke::NONE,
            ));
        }
        // The same triangle, outlined.
        MarkerSeverity::Warning => {
            painter.add(Shape::closed_line(
                vec![
                    pos2(x - r, y - r * 0.8),
                    pos2(x + r, y - r * 0.8),
                    pos2(x, y + r),
                ],
                Stroke::new(1.0, color),
            ));
        }
        // A flag on a staff: the run finished here.
        MarkerSeverity::Finish => {
            painter.vline(x, (y - r)..=(y + r), Stroke::new(1.0, color));
            painter.add(Shape::convex_polygon(
                vec![pos2(x, y - r), pos2(x + r, y - r * 0.5), pos2(x, y)],
                color,
                Stroke::NONE,
            ));
        }
        // A square: the run paused here.
        MarkerSeverity::Stop => {
            painter.rect_filled(
                Rect::from_center_size(centre, vec2(r * 1.5, r * 1.5)),
                0.0,
                color,
            );
        }
    }
}

/// Shade an assertion violation across `[start, end)` within `rect`.
pub fn paint_violation_span(
    painter: &egui::Painter,
    rect: Rect,
    span: &AssertionSpan,
    map: &TimeMapping,
    tokens: &Tokens,
) {
    let (x0, x1) = (map.x(span.start), map.x(span.end));
    let color = if span.error {
        tokens.color.err
    } else {
        tokens.color.warn
    };
    painter.rect_filled(
        Rect::from_x_y_ranges(x0..=x1.max(x0 + 1.0), rect.y_range()),
        0.0,
        color.gamma_multiply(SPAN_FILL),
    );
}

/// The glyphs the visible records draw, as `(x, record index)`: one per pixel
/// column, the most severe record in it.
pub fn marker_glyphs(markers: &[HdlMarker], map: &TimeMapping) -> Vec<(f32, usize)> {
    let mut column: Option<(f32, usize)> = None;
    let mut glyphs: Vec<(f32, usize)> = Vec::new();
    for (index, marker) in markers.iter().enumerate() {
        if !(map.t0..=map.t1).contains(&marker.time) {
            continue;
        }
        let x = map.x(marker.time);
        match column {
            Some((held_x, held)) if x.floor() == held_x.floor() => {
                if marker.severity > markers[held].severity {
                    column = Some((x, index));
                }
            }
            _ => {
                glyphs.extend(column);
                column = Some((x, index));
            }
        }
    }
    glyphs.extend(column);
    glyphs
}

/// Draw the marker strip into the next [`MARKER_STRIP_HEIGHT`] points.
///
/// Records sharing a pixel column draw one glyph, the most severe of them,
/// so a burst of records at one site costs one glyph.
pub fn show_marker_strip(
    ui: &mut Ui,
    input: &MarkerStripInput<'_>,
    format_time: &dyn Fn(f64) -> String,
) -> MarkerStripResponse {
    let mut output = MarkerStripResponse::default();
    let height = marker_strip_height(input);
    if height == 0.0 {
        return output;
    }
    let tokens = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::click());
    let map = TimeMapping {
        t0: input.x_range.0,
        t1: input.x_range.1,
        left: rect.left() + input.left_margin,
        right: rect.right() - input.right_margin,
    };
    let painter = ui
        .painter()
        .with_clip_rect(Rect::from_x_y_ranges(map.left..=map.right, rect.y_range()));
    for span in input.assertions {
        paint_violation_span(&painter, rect, span, &map, &tokens);
    }
    let glyphs = marker_glyphs(input.hdl, &map);
    for (x, index) in &glyphs {
        let severity = input.hdl[*index].severity;
        paint_marker_glyph(
            &painter,
            pos2(*x, rect.center().y),
            severity,
            severity_color(severity, &tokens),
        );
    }

    let pointer = response
        .hover_pos()
        .or_else(|| response.interact_pointer_pos());
    let picked = pointer.and_then(|pointer| {
        glyphs
            .iter()
            .filter(|(x, _)| (x - pointer.x).abs() <= PICK_RADIUS)
            .min_by(|a, b| (a.0 - pointer.x).abs().total_cmp(&(b.0 - pointer.x).abs()))
            .map(|(_, index)| *index)
    });
    let picked_span = pointer.and_then(|pointer| {
        input
            .assertions
            .iter()
            .position(|span| (map.x(span.start)..=map.x(span.end)).contains(&pointer.x))
    });
    if let Some(index) = picked {
        let marker = &input.hdl[index];
        let text = marker.describe(&format_time(marker.time));
        if response.clicked() {
            output.cursor_to = Some(marker.time);
            output.open_source.clone_from(&marker.source);
        }
        response.on_hover_text(text);
    } else if let Some(index) = picked_span {
        let span = &input.assertions[index];
        if response.clicked() {
            output.cursor_to = Some(span.start);
            output.select_assertion = Some(index);
        }
        response.on_hover_text(format!(
            "{} on {} from {} to {}",
            span.label,
            span.operand,
            format_time(span.start),
            format_time(span.end)
        ));
    }
    output
}
