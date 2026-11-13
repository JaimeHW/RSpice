//! The event-marker strip: HDL severity records, the run's end and assertion
//! violations on the strip's time axis.
//!
//! One thin strip above the panes, and only when the run has a marker. A
//! record draws a glyph by severity, painted as a vector (the interface font
//! has no symbol glyphs to borrow). Hovering a marker gives its text and its
//! `file:line`; clicking one moves the cursor to it, and a double click asks
//! the host to open its source line, as a waveform viewer's message pane
//! does. An assertion violation is a shaded span here and on its operand's
//! lane. `$display` and `$info` are Console output, not markers. When a
//! vector file expects outputs after the run's stop time, the stop time is
//! marked with how many of them were not checked.

use super::cursor::EdgeStep;
use super::plan::TimeMapping;
use egui::{Color32, Pos2, Rect, Sense, Shape, Stroke, Ui, pos2, vec2};
use rspice_results::analysis_result::{HdlDiagnostic, RunChecks};
use rspice_results::engine_evidence::{
    AssertLevel, AssertionReport, HdlOutputRecord, HdlPoint, HdlSeverity, HdlStream, ModelFinish,
    ModelFinishPoint, ModelFinishTask,
};
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};

/// Height of the marker strip, in points, its seam included.
pub const MARKER_STRIP_HEIGHT: f32 = 14.0;
/// How far from a glyph's centre the pointer still picks it.
const PICK_RADIUS: f32 = 5.0;
const GLYPH_RADIUS: f32 = 4.0;
/// How strongly a violation span shades.
const SPAN_FILL: f32 = 0.28;
/// Width of each step control in the gutter.
const STEP_WIDTH: f32 = 12.0;
/// Clearance between the gutter's controls and the time axis, so a glyph at
/// the axis' first instant does not touch them.
const GUTTER_CLEARANCE: f32 = GLYPH_RADIUS + 2.0;
const GUTTER_PAD: f32 = 4.0;
/// The strip's name in its gutter.
pub const MARKER_STRIP_NAME: &str = "Checks";

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
    /// The file as the run names it; empty for a line of the run's own deck.
    pub file: String,
    /// One-based.
    pub line: u32,
}

impl std::fmt::Display for SourceLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.file.is_empty() {
            write!(f, "line {}", self.line)
        } else {
            write!(f, "{}:{}", self.file, self.line)
        }
    }
}

/// A file and line an HDL call names, when it names both.
fn hdl_source(file: &str, line: u32) -> Option<SourceLine> {
    (!file.is_empty() && line > 0).then(|| SourceLine {
        file: file.to_owned(),
        line,
    })
}

/// Where on the analysis' axis a record made at `point` stands. A record
/// made before the axis begins, at initialization or at the operating point
/// the analysis starts from, stands at its first sample, `start`.
fn hdl_point_x(point: HdlPoint, start: f64) -> f64 {
    match point {
        HdlPoint::Transient { time } => time,
        HdlPoint::DcSweep { value } => value,
        HdlPoint::Initialization | HdlPoint::OperatingPoint => start,
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
            source: hdl_source(&origin.file, origin.line),
        })
    }

    /// The marker a retained severity record makes: `$warning`, `$error`
    /// and `$fatal` do; `$info` is Console output. `start` is the axis' first
    /// sample, where a record made before the axis begins stands.
    pub fn from_diagnostic(record: &HdlDiagnostic, start: f64) -> Option<Self> {
        let severity = match record.severity {
            HdlSeverity::Info => return None,
            HdlSeverity::Warning => MarkerSeverity::Warning,
            HdlSeverity::Error => MarkerSeverity::Error,
            HdlSeverity::Fatal => MarkerSeverity::Fatal,
        };
        let origin = &record.origin;
        Some(Self {
            time: hdl_point_x(record.point, start),
            severity,
            text: record.text.trim_end().to_owned(),
            scope: origin.scope.clone(),
            source: hdl_source(&origin.file, origin.line),
        })
    }

    /// The marker of the `$finish`, `$stop` or `$fatal` that ended the run,
    /// at the instant it ended and with the instance that asked.
    pub fn from_finish(finish: &ModelFinish, start: f64) -> Self {
        let severity = match finish.task {
            ModelFinishTask::Finish => MarkerSeverity::Finish,
            ModelFinishTask::Stop => MarkerSeverity::Stop,
            ModelFinishTask::Fatal => MarkerSeverity::Fatal,
        };
        let time = match finish.point {
            ModelFinishPoint::Transient { time } => time,
            ModelFinishPoint::DcSweep { value } => value,
            ModelFinishPoint::Frequency { frequency } => frequency,
            ModelFinishPoint::Initialization | ModelFinishPoint::OperatingPoint => start,
        };
        Self {
            time,
            severity,
            text: finish
                .message
                .as_deref()
                .unwrap_or_default()
                .trim_end()
                .to_owned(),
            scope: finish.instance.clone(),
            source: finish
                .source
                .as_ref()
                .and_then(|source| hdl_source(&source.file, source.line)),
        }
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
    /// The lane the violation also shades: the one digital net the check
    /// reads, as `D(net)`. Otherwise the check itself, which names no lane.
    pub operand: String,
    /// The card's name.
    pub label: String,
    /// An error-level violation; otherwise a warning or a notice.
    pub error: bool,
    /// What the card checks, as written.
    pub check: String,
    pub message: Option<String>,
    /// The card's line.
    pub source: Option<SourceLine>,
}

impl AssertionSpan {
    /// The spans a report's violations shade, a violation still open at the
    /// stop time running to `stop`. A report that could not be evaluated
    /// has none, and neither has a violation at an operating point, which
    /// stands at no instant.
    pub fn from_report(report: &AssertionReport, stop: f64) -> Vec<Self> {
        if report.error.is_some() {
            return Vec::new();
        }
        let operand = lane_operand(&report.target).unwrap_or_else(|| report.target.clone());
        let label = match &report.instance {
            Some(instance) => format!("{} ({instance})", report.name),
            None => report.name.clone(),
        };
        let source = report_source(report);
        report
            .violations
            .iter()
            .filter_map(|violation| {
                let start = violation.start?;
                Some(Self {
                    start,
                    end: violation.end.unwrap_or(stop).max(start),
                    operand: operand.clone(),
                    label: label.clone(),
                    error: report.level == AssertLevel::Error,
                    check: report.target.clone(),
                    message: report.message.clone(),
                    source: source.clone(),
                })
            })
            .collect()
    }

    /// The hover text: which card, over what interval, checking what, and
    /// where it was written.
    pub fn describe(&self, from: &str, to: &str) -> String {
        let mut text = format!("Assertion {} violated from {from} to {to}", self.label);
        text.push('\n');
        text.push_str(&self.check);
        if let Some(message) = &self.message {
            text.push('\n');
            text.push_str(message);
        }
        if let Some(source) = &self.source {
            text.push('\n');
            text.push_str(&source.to_string());
        }
        text
    }
}

/// The line a report's card (or its vector file's row) was written on, the
/// path as diagnostics show it.
fn report_source(report: &AssertionReport) -> Option<SourceLine> {
    let origin = &report.origin;
    let line = u32::try_from(origin.line).ok().filter(|line| *line > 0)?;
    let file = match origin.path {
        Some(_) => {
            let shown = origin.to_string();
            shown
                .strip_suffix(&format!(":{line}"))
                .unwrap_or(&shown)
                .to_owned()
        }
        None => String::new(),
    };
    Some(SourceLine { file, line })
}

/// The one digital net `check` reads, as `D(net)`, when it reads exactly
/// one; names compare as a deck's do, without case.
pub fn lane_operand(check: &str) -> Option<String> {
    let bytes = check.as_bytes();
    let mut found: Option<&str> = None;
    let mut at = 0;
    while at + 1 < bytes.len() {
        let opens = bytes[at].eq_ignore_ascii_case(&b'd')
            && bytes[at + 1] == b'('
            && (at == 0
                || !(bytes[at - 1].is_ascii_alphanumeric() || b"_$.".contains(&bytes[at - 1])));
        if !opens {
            at += 1;
            continue;
        }
        let close = at + 2 + check[at + 2..].find(')')?;
        let net = check[at + 2..close].trim();
        match found {
            Some(seen) if !seen.eq_ignore_ascii_case(net) => return None,
            _ => found = Some(net),
        }
        at = close + 1;
    }
    found
        .filter(|net| !net.is_empty())
        .map(|net| format!("D({net})"))
}

/// The run's stop time, marked when a vector file expected outputs after
/// it: those were not checked.
#[derive(Debug, Clone, PartialEq)]
pub struct StopMark {
    pub time: f64,
    /// How many expected outputs were due after it.
    pub beyond: usize,
    /// The first row that states one.
    pub source: Option<SourceLine>,
}

impl StopMark {
    /// The hover text.
    pub fn describe(&self, time: &str) -> String {
        let mut text = format!(
            "Stop time {time}: {} expected output{} after it {} not checked",
            self.beyond,
            if self.beyond == 1 { "" } else { "s" },
            if self.beyond == 1 { "was" } else { "were" }
        );
        if let Some(source) = &self.source {
            text.push_str("\nThe first is on the row at ");
            text.push_str(&source.to_string());
        }
        text
    }
}

/// How many expected outputs a vector file's report says were due after the
/// stop time: the one warning the evaluator gives them, which carries no
/// violation and states the count first.
fn samples_after_stop(report: &AssertionReport) -> Option<usize> {
    if !report.violations.is_empty() || report.error.is_some() {
        return None;
    }
    let (count, rest) = report.message.as_deref()?.split_once(' ')?;
    (rest.starts_with("expected-output sample") && rest.contains("after the transient's stop time"))
        .then(|| count.parse().ok())
        .flatten()
}

/// Everything a run's checks put on its strip's time axis.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RunMarkers {
    /// Severity records and the finish, in time order.
    pub hdl: Vec<HdlMarker>,
    pub assertions: Vec<AssertionSpan>,
    pub stop: Option<StopMark>,
}

impl RunMarkers {
    /// The markers `checks` make on an axis whose first and last samples
    /// are `start` and `stop`; `None` when they make none.
    pub fn of(checks: &RunChecks, (start, stop): (f64, f64)) -> Option<Self> {
        let mut hdl: Vec<HdlMarker> = checks
            .hdl
            .iter()
            .filter_map(|record| HdlMarker::from_diagnostic(record, start))
            .chain(
                checks
                    .finish
                    .iter()
                    .map(|finish| HdlMarker::from_finish(finish, start)),
            )
            .collect();
        hdl.sort_by(|a, b| a.time.total_cmp(&b.time));
        let assertions = checks
            .assertions
            .iter()
            .flat_map(|report| AssertionSpan::from_report(report, stop))
            .collect();
        let late: Vec<(&AssertionReport, usize)> = checks
            .assertions
            .iter()
            .filter_map(|report| samples_after_stop(report).map(|count| (report, count)))
            .collect();
        let stop = late.first().map(|(first, _)| StopMark {
            time: stop,
            beyond: late.iter().map(|(_, count)| count).sum(),
            source: report_source(first),
        });
        let markers = Self {
            hdl,
            assertions,
            stop,
        };
        (!markers.is_empty()).then_some(markers)
    }

    pub fn is_empty(&self) -> bool {
        self.hdl.is_empty() && self.assertions.is_empty() && self.stop.is_none()
    }

    /// The first instant after `from` (or the last before it) a marker
    /// stands at: a record, a violation's start, or the marked stop time.
    pub fn step_from(&self, from: f64, step: EdgeStep) -> Option<f64> {
        let instants = self
            .hdl
            .iter()
            .map(|marker| marker.time)
            .chain(self.assertions.iter().map(|span| span.start))
            .chain(self.stop.iter().map(|stop| stop.time));
        match step {
            EdgeStep::Next => instants.filter(|t| *t > from).min_by(f64::total_cmp),
            EdgeStep::Previous => instants.filter(|t| *t < from).max_by(f64::total_cmp),
        }
    }
}

/// Everything one frame of the marker strip draws from.
pub struct MarkerStripInput<'a> {
    pub markers: &'a RunMarkers,
    pub x_range: (f64, f64),
    pub left_margin: f32,
    /// The gutter's name column, where the strip's name sits; zero for a
    /// strip whose gutter has none.
    pub name_width: f32,
    pub right_margin: f32,
    /// The strip's cursors A and B, drawn through it as the panes draw them.
    /// The one placed last is the one a marker moves.
    pub cursor: Option<f64>,
    pub cursor_b: Option<f64>,
}

/// The strip's height: zero when the run has no marker, so it takes no room.
pub fn marker_strip_height(input: &MarkerStripInput<'_>) -> f32 {
    markers_height(input.markers)
}

/// The height a strip with `markers` takes: none for a run without one.
pub fn markers_height(markers: &RunMarkers) -> f32 {
    if markers.is_empty() {
        0.0
    } else {
        MARKER_STRIP_HEIGHT
    }
}

/// What the reader asked of the marker strip this frame.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MarkerStripResponse {
    /// Move the cursor placed last to this exact instant: a marker's, picked
    /// or stepped to.
    pub cursor_to: Option<f64>,
    /// Open the picked marker's source line.
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
        MarkerSeverity::Finish | MarkerSeverity::Stop => tokens.color.canvas_text_dim,
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

/// The mark of a simulation's end: a rule at the stop time with, before it,
/// how many expected outputs lie beyond it.
fn paint_stop_mark(painter: &egui::Painter, plot: Rect, x: f32, beyond: usize, tokens: &Tokens) {
    let color = tokens.color.warn;
    painter.vline(x, plot.y_range(), Stroke::new(1.0, color));
    let galley = painter.layout_no_wrap(
        format!("+{beyond}"),
        theme::mono(tokens::FS_MICRO, FontWeight::Regular),
        color,
    );
    let origin = pos2(
        x - GLYPH_RADIUS - 2.0 - galley.size().x,
        plot.center().y - galley.size().y * 0.5,
    );
    painter.galley(origin, galley, color);
}

/// A small triangle pointing left or right, painted as a vector.
fn paint_step_arrow(painter: &egui::Painter, centre: Pos2, next: bool, color: Color32) {
    let (r, x, y) = (3.0, centre.x, centre.y);
    let (tip, back) = if next { (x + r, x - r) } else { (x - r, x + r) };
    painter.add(Shape::convex_polygon(
        vec![pos2(back, y - r), pos2(tip, y), pos2(back, y + r)],
        color,
        Stroke::NONE,
    ));
}

/// What the pointer is on.
#[derive(Clone, Copy)]
enum Pick {
    Record(usize),
    Stop,
    Span(usize),
}

/// Draw the marker strip into the next [`MARKER_STRIP_HEIGHT`] points.
///
/// Records sharing a pixel column draw one glyph, the most severe of them,
/// so a burst of records at one site costs one glyph. The gutter holds the
/// strip's name and controls that step the cursor to the previous and next
/// marker, which Left and Right also do while the strip has focus.
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
    let markers = input.markers;
    let tokens = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::click());
    let map = TimeMapping {
        t0: input.x_range.0,
        t1: input.x_range.1,
        left: rect.left() + input.left_margin,
        right: rect.right() - input.right_margin,
    };
    // The bottom point is the seam to the lanes or panes under the strip.
    let seam = Rect::from_x_y_ranges(rect.x_range(), rect.bottom() - 1.0..=rect.bottom());
    ui.painter()
        .rect_filled(seam, 0.0, tokens.color.canvas_grid);
    let plot = Rect::from_x_y_ranges(map.left..=map.right, rect.top()..=seam.top());
    let centre_y = plot.center().y;
    let span_painter = ui.painter().with_clip_rect(plot);
    // A glyph at either end of the axis is drawn whole.
    let glyph_painter = ui
        .painter()
        .with_clip_rect(plot.expand2(vec2(GLYPH_RADIUS + 1.0, 0.0)));
    for span in &markers.assertions {
        paint_violation_span(&span_painter, plot, span, &map, &tokens);
    }
    for (cursor, color) in [
        (input.cursor, tokens.color.canvas_cursor_a),
        (input.cursor_b, tokens.color.canvas_cursor_b),
    ] {
        if let Some(t) = cursor
            && (map.t0..=map.t1).contains(&t)
        {
            span_painter.vline(map.x(t), plot.y_range(), Stroke::new(1.0, color));
        }
    }
    let stop_x = markers
        .stop
        .as_ref()
        .filter(|stop| (map.t0..=map.t1).contains(&stop.time))
        .map(|stop| {
            let x = map.x(stop.time);
            paint_stop_mark(&glyph_painter, plot, x, stop.beyond, &tokens);
            x
        });
    let glyphs = marker_glyphs(&markers.hdl, &map);
    for (x, index) in &glyphs {
        let severity = markers.hdl[*index].severity;
        paint_marker_glyph(
            &glyph_painter,
            pos2(*x, centre_y),
            severity,
            severity_color(severity, &tokens),
        );
    }

    let from = |step| {
        input.cursor_b.or(input.cursor).unwrap_or(match step {
            EdgeStep::Previous => f64::INFINITY,
            EdgeStep::Next => f64::NEG_INFINITY,
        })
    };
    if let Some(t) = show_gutter(ui, (rect, response.id), input, &tokens, |step| {
        markers.step_from(from(step), step)
    }) {
        output.cursor_to = Some(t);
    }

    let pointer = response
        .hover_pos()
        .or_else(|| response.interact_pointer_pos())
        .filter(|pointer| pointer.x >= map.left - PICK_RADIUS);
    let pick = pointer.and_then(|pointer| {
        let record = glyphs
            .iter()
            .filter(|(x, _)| (x - pointer.x).abs() <= PICK_RADIUS)
            .min_by(|a, b| (a.0 - pointer.x).abs().total_cmp(&(b.0 - pointer.x).abs()))
            .map(|(_, index)| Pick::Record(*index));
        let stop = stop_x
            .filter(|x| (x - pointer.x).abs() <= PICK_RADIUS)
            .map(|_| Pick::Stop);
        let span = markers
            .assertions
            .iter()
            .position(|span| (map.x(span.start)..=map.x(span.end)).contains(&pointer.x))
            .map(Pick::Span);
        record.or(stop).or(span)
    });
    if let Some(pick) = pick {
        let (time, source, text) = match pick {
            Pick::Record(index) => {
                let marker = &markers.hdl[index];
                let text = marker.describe(&format_time(marker.time));
                (marker.time, marker.source.as_ref(), text)
            }
            Pick::Stop => {
                let stop = markers.stop.as_ref().expect("a picked stop mark exists");
                (
                    stop.time,
                    stop.source.as_ref(),
                    stop.describe(&format_time(stop.time)),
                )
            }
            Pick::Span(index) => {
                let span = &markers.assertions[index];
                let text = span.describe(&format_time(span.start), &format_time(span.end));
                (span.start, span.source.as_ref(), text)
            }
        };
        if response.clicked() {
            output.cursor_to = Some(time);
            if let Pick::Span(index) = pick {
                output.select_assertion = Some(index);
            }
            // Focus is what routes Left and Right here; a pointer press
            // takes it without showing a ring.
            response.request_focus();
        }
        if response.double_clicked() {
            output.open_source = source.cloned();
        }
        let hint = if source.is_some() {
            "\nDouble-click to open the source"
        } else {
            ""
        };
        // At the pointer: the strip is the plot's width, so a tooltip under
        // its left edge would sit far from the marker it states.
        response
            .clone()
            .on_hover_text_at_pointer(format!("{text}{hint}"));
    } else if response.clicked() {
        response.request_focus();
    }
    if response.has_focus() {
        ui.memory_mut(|memory| {
            memory.set_focus_lock_filter(
                response.id,
                egui::EventFilter {
                    horizontal_arrows: true,
                    ..Default::default()
                },
            );
        });
        let step = ui.input(|i| {
            if i.key_pressed(egui::Key::ArrowLeft) {
                Some(EdgeStep::Previous)
            } else if i.key_pressed(egui::Key::ArrowRight) {
                Some(EdgeStep::Next)
            } else {
                None
            }
        });
        if let Some(step) = step
            && let Some(t) = markers.step_from(from(step), step)
        {
            output.cursor_to = Some(t);
        }
    }
    output
}

/// The strip's gutter: its name in the name column, and in the value column
/// the controls that step the cursor to the previous and next marker. Returns
/// the instant a pressed control steps to.
fn show_gutter(
    ui: &Ui,
    (rect, id): (Rect, egui::Id),
    input: &MarkerStripInput<'_>,
    tokens: &Tokens,
    step_to: impl Fn(EdgeStep) -> Option<f64>,
) -> Option<f64> {
    let centre_y = rect.top() + (rect.height() - 1.0) * 0.5;
    let painter = ui.painter();
    if input.name_width > 0.0 {
        let galley = painter.layout_no_wrap(
            MARKER_STRIP_NAME.to_owned(),
            theme::mono(tokens::FS_MICRO, FontWeight::Regular),
            tokens.color.canvas_text_dim,
        );
        if galley.size().x <= input.name_width.min(input.left_margin) - 2.0 * GUTTER_PAD {
            let origin = pos2(rect.left() + GUTTER_PAD, centre_y - galley.size().y * 0.5);
            painter.galley(origin, galley, tokens.color.canvas_text_dim);
        }
    }
    let controls_right = rect.left() + input.left_margin - GUTTER_CLEARANCE;
    if controls_right - 2.0 * STEP_WIDTH < rect.left() + input.name_width.min(input.left_margin) {
        return None;
    }
    let mut stepped = None;
    for (step, slot, label) in [
        (EdgeStep::Previous, 2.0_f32, "Previous check"),
        (EdgeStep::Next, 1.0, "Next check"),
    ] {
        let left = slot.mul_add(-STEP_WIDTH, controls_right);
        let hit = Rect::from_x_y_ranges(left..=left + STEP_WIDTH, rect.top()..=rect.bottom() - 1.0);
        let target = step_to(step);
        let sense = if target.is_some() {
            Sense::click()
        } else {
            Sense::hover()
        };
        let response = ui.interact(hit, id.with(label), sense).on_hover_text(label);
        let color = if target.is_none() {
            tokens.color.canvas_text_faint
        } else if response.hovered() {
            tokens.color.canvas_text
        } else {
            tokens.color.canvas_text_dim
        };
        paint_step_arrow(
            painter,
            pos2(hit.center().x, centre_y),
            step == EdgeStep::Next,
            color,
        );
        if response.clicked() {
            stepped = target;
        }
    }
    stepped
}
