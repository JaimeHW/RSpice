//! What the grid actually paints, measured.
//!
//! Every assertion here reads a position, a width or a role out of one frame's
//! output. "It painted something" is not a test: the defects this page has
//! shipped before were a column clipped at eight characters, a cell printed on
//! top of its neighbour, and a direction mark that rendered as a tofu box, and
//! none of those would fail an assertion about the presence of ink.

use std::sync::Arc;

use egui::accesskit::Role;

use super::super::{CONSOLE_COLUMN_GAP, CONSOLE_MARK_GUTTER};
use super::*;
use crate::state::{ComponentType, Point};
use crate::workbench::app_state::diagnostic_store::listing::listing;
use crate::workbench::app_state::diagnostic_store::{
    DiagnosticInputs, DiagnosticRecord, DiagnosticSubject,
};
use crate::workbench::app_state::problems_view::{ProblemColumn, ProblemGrouping};
use crate::workbench::documents::netlist_document::diagnostics::DiagnosticFix;
use crate::workbench::documents::netlist_document::{Diagnostic, NetlistDiagnosticCollection};
use rspice_design::drc::{DrcLocation, DrcResult, DrcViolation, DrcViolationType};

const WIDTH: f32 = 900.0;
const HEIGHT: f32 = 420.0;

fn open_state() -> AppState {
    let mut state = AppState::default();
    state.project_lifecycle.authority.open_session();
    state
}

/// A waiver of a finding of `rule`, recorded at review.
fn waiver(rule: &str) -> rspice_app_types::diagnostics::DiagnosticSuppression {
    rspice_app_types::diagnostics::DiagnosticSuppression {
        rule: rule.to_owned(),
        reason: "accepted at review".to_owned(),
        author: "ana".to_owned(),
        waived_at_ms: 1_760_000_000_000,
    }
}

/// One finding from the test producer, about one node, saying `message`.
fn record(state: &AppState, code: &str, level: DiagnosticLevel, message: &str) -> DiagnosticRecord {
    DiagnosticRecord::new(
        rspice_app_types::diagnostics::CanonicalDiagnosticMetadata::current(
            "rspice.test",
            code,
            "doc",
            None,
            0,
            "validation",
            message,
        ),
        level,
        DiagnosticSubject::new(state.workspace.content.project.id(), "Node 'n1'"),
        message,
    )
}

fn publish(state: &mut AppState, records: Vec<DiagnosticRecord>) {
    state
        .publish_diagnostics(DiagnosticInputs::Dispatch, records)
        .expect("the dispatch digest is available");
}

/// `count` findings with nowhere to go, reported in the order of their
/// messages: "finding 00", "finding 01", and on.
fn state_with_findings(count: usize) -> AppState {
    let mut state = open_state();
    publish_findings(&mut state, count);
    state
}

fn publish_findings(state: &mut AppState, count: usize) {
    let records = (0..count)
        .map(|index| {
            record(
                state,
                &format!("T-{index:02}"),
                DiagnosticLevel::Warning,
                &format!("finding {index:02}"),
            )
        })
        .collect();
    publish(state, records);
}

/// One finding each from two rules of the live drawing: an error on an object
/// the drawing carries, and a warning on a wire.
fn state_with_design_findings() -> AppState {
    let mut state = open_state();
    publish_design_findings(&mut state);
    state
}

/// The two findings of [`state_with_design_findings`], on `state`'s own
/// drawing. Returns the resistor the error is about.
fn publish_design_findings(state: &mut AppState) -> u64 {
    let resistor = state
        .schematic
        .add_component(ComponentType::Resistor, Point::new(40, 40));
    state.sync_active_schematic_to_workspace();
    let mut result = DrcResult::new();
    result.add_violation(DrcViolation::new(
        1,
        DrcViolationType::MissingParameter,
        "R1 has no resistance",
        DrcLocation::Component {
            id: resistor,
            name: "R1".to_owned(),
        },
    ));
    result.add_violation(DrcViolation::new(
        2,
        DrcViolationType::DanglingWire,
        "Wire ends open",
        DrcLocation::Point { x: 10.0, y: 0.0 },
    ));
    result.completed = true;
    state
        .publish_active_design_check_result(result)
        .expect("the fixture publishes a current receipt");
    resistor
}

/// A netlist finding that carries the fix its producer offered.
fn netlist_finding_with_a_fix(state: &mut AppState) {
    let mut diagnostic = Diagnostic::error("unknown model nchh");
    diagnostic.fix = Some(DiagnosticFix {
        label: "Replace with nch".to_owned(),
        span: 0..4,
        replacement: "nch".to_owned(),
    });
    // The canonical quick fixes are derived when the location is bound, so the
    // fix is attached before the line is.
    let diagnostic = diagnostic.with_line(Some(1));
    state.ui.netlist.diagnostics = Arc::new(
        NetlistDiagnosticCollection::try_new(vec![diagnostic], "nchh\n")
            .expect("the fixture collection is well formed"),
    );
}

/// The listed findings' identities, in the order their producers reported
/// them.
fn reported(state: &AppState) -> Vec<uuid::Uuid> {
    let listed = listing(state);
    (0..listed.len())
        .filter_map(|index| listed.facet(index))
        .map(|facet| facet.id)
        .collect()
}

/// One node of the access tree, in the terms these tests read.
///
/// egui puts a label widget's text in the node's *value* and a control's in
/// its name, so a test that read only `label()` would miss every label.
struct Announced {
    role: Role,
    label: Option<String>,
    value: Option<String>,
    toggled: Option<egui::accesskit::Toggled>,
    selected: Option<bool>,
    expanded: Option<bool>,
    description: Option<String>,
    bounds: Option<egui::Rect>,
}

impl Announced {
    fn says(&self) -> Option<&str> {
        self.label.as_deref().or(self.value.as_deref())
    }

    fn selected(&self) -> bool {
        self.selected == Some(true)
    }
}

/// Everything one frame put on the screen, in the forms these tests measure.
struct Render {
    /// Every painted text, with the rectangle its laid-out galley covers.
    texts: Vec<(String, egui::Rect)>,
    /// Filled rectangles, with the colour they were filled in.
    fills: Vec<(egui::Rect, egui::Color32)>,
    /// Three-point paths: the painted direction and chevron marks.
    triangles: Vec<Vec<egui::Pos2>>,
    /// Straight strokes, with their colour: the strokes of a problem mark
    /// among them.
    segments: Vec<([egui::Pos2; 2], egui::Color32)>,
    /// Outlined rectangles, with their stroke: the focus ring among them.
    outlines: Vec<(egui::Rect, egui::Stroke)>,
    nodes: Vec<Announced>,
}

impl Render {
    fn cell(&self, needle: &str) -> Option<egui::Rect> {
        self.texts
            .iter()
            .find(|(text, _)| text.contains(needle))
            .map(|(_, rect)| *rect)
    }

    fn text(&self, needle: &str) -> Option<egui::Pos2> {
        self.cell(needle).map(|rect| rect.min)
    }

    fn exact(&self, text: &str) -> Option<egui::Pos2> {
        self.texts
            .iter()
            .find(|(painted, _)| painted == text)
            .map(|(_, rect)| rect.min)
    }

    fn has(&self, needle: &str) -> bool {
        self.text(needle).is_some()
    }

    /// Every text painted on one row of the grid, left to right.
    ///
    /// Two points of tolerance, not none: the subject column is printed in a
    /// heavier weight than its neighbours and a face's line height is its
    /// own. Rows are a row height apart, so it cannot reach the next one.
    fn row_at(&self, y: f32) -> Vec<(String, egui::Rect)> {
        let mut row: Vec<(String, egui::Rect)> = self
            .texts
            .iter()
            .filter(|(_, rect)| (rect.min.y - y).abs() < 2.0)
            .cloned()
            .collect();
        row.sort_by(|left, right| left.1.min.x.total_cmp(&right.1.min.x));
        row
    }

    fn node(&self, says: &str) -> Option<&Announced> {
        self.nodes.iter().find(|node| node.says() == Some(says))
    }

    /// The band the header paints, which spans the page's content width: the
    /// widest fill of the header's height, which no control in the strip
    /// above it is.
    fn header_band(&self) -> egui::Rect {
        self.fills
            .iter()
            .filter(|(rect, _)| (rect.height() - grid::HEADER_HEIGHT).abs() < 0.5)
            .map(|(rect, _)| *rect)
            .max_by(|left, right| left.width().total_cmp(&right.width()))
            .expect("the header paints its band")
    }
}

/// The row a printed cell sits in: a row height, centred on the cell.
fn row_of(cell: egui::Rect) -> egui::Rect {
    egui::Rect::from_center_size(
        cell.center(),
        egui::vec2(cell.width(), CONSOLE_ROW_MIN_HEIGHT),
    )
}

/// One egui context, held across frames.
///
/// Focus, scroll position and the arrangement the page keeps between frames
/// all live in the context, so a test that clicks a row and then keys it has
/// to send both through the same one. A fresh context per frame would have no
/// focus to key against, and the keyboard tests would pass by doing nothing.
struct Harness {
    ctx: egui::Context,
    /// The identity of the panel the page was last drawn in, which the body's
    /// scroll position is kept under.
    page: std::cell::Cell<Option<egui::Id>>,
}

impl Harness {
    fn new() -> Self {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        Self {
            ctx,
            page: std::cell::Cell::new(None),
        }
    }

    fn frame(&self, state: &mut AppState, input: egui::RawInput) -> Render {
        if self.ctx.cumulative_pass_nr() == 0 {
            // This context counts passes from zero; a memo another harness
            // took of the same session must not answer for them.
            state.diagnostics.forget_pass();
        }
        let output = self.ctx.run_ui(input, |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                self.page.set(Some(ui.id()));
                problems(ui, state);
            });
        });
        collect(output)
    }

    /// Scroll the body to `offset`, as the reader's own scrolling would.
    fn scroll_body_to(&self, offset: f32) {
        let id = self.body_scroll_id();
        let mut scroll =
            egui::scroll_area::State::load(&self.ctx, id).expect("the body has been drawn");
        scroll.offset.y = offset;
        scroll.store(&self.ctx, id);
    }

    /// Where the body is scrolled to.
    fn body_offset(&self) -> f32 {
        egui::scroll_area::State::load(&self.ctx, self.body_scroll_id())
            .expect("the body has been drawn")
            .offset
            .y
    }

    fn body_scroll_id(&self) -> egui::Id {
        self.page
            .get()
            .expect("the page has been drawn")
            .with(egui::IdSalt::new("workbench.problems"))
    }

    /// Draw a frame with nothing happening, which is what settles the sizing
    /// and the scroll position a previous frame asked for.
    fn settle(&self, state: &mut AppState) {
        let _ = self.frame(state, sized(egui::RawInput::default()));
    }

    /// Give the table keyboard focus, as Tab does.
    fn focus_table(&self) {
        self.ctx
            .memory_mut(|memory| memory.request_focus(grid::body_id()));
    }

    fn focused(&self) -> Option<egui::Id> {
        self.ctx.memory(|memory| memory.focused())
    }

    /// Where the table's rows are shown, between the header and whatever is
    /// under the body.
    fn table(&self) -> egui::Rect {
        self.ctx
            .read_response(grid::body_id())
            .expect("the table was drawn")
            .rect
    }

    /// Whether the row printing `message` is shown whole in the table.
    fn shows_whole(&self, render: &Render, message: &str) -> bool {
        let table = self.table();
        render.cell(message).is_some_and(|cell| {
            let row = row_of(cell);
            row.top() >= table.top() - 0.5 && row.bottom() <= table.bottom() + 0.5
        })
    }
}

fn render(state: &mut AppState, input: egui::RawInput) -> Render {
    let harness = Harness::new();
    harness.settle(state);
    harness.frame(state, input)
}

fn collect(output: egui::FullOutput) -> Render {
    fn walk(shape: &egui::epaint::Shape, render: &mut Render) {
        match shape {
            egui::epaint::Shape::Text(text) => render.texts.push((
                text.galley.job.text.clone(),
                egui::Rect::from_min_size(text.pos, text.galley.size()),
            )),
            egui::epaint::Shape::Rect(rect) => {
                render.fills.push((rect.rect, rect.fill));
                if rect.stroke.width > 0.0 {
                    render.outlines.push((rect.rect, rect.stroke));
                }
            }
            egui::epaint::Shape::Path(path) if path.points.len() == 3 => {
                render.triangles.push(path.points.clone());
            }
            egui::epaint::Shape::LineSegment { points, stroke } => {
                render.segments.push((*points, stroke.color));
            }
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    walk(shape, render);
                }
            }
            _ => {}
        }
    }

    let mut render = Render {
        texts: Vec::new(),
        fills: Vec::new(),
        triangles: Vec::new(),
        segments: Vec::new(),
        outlines: Vec::new(),
        nodes: Vec::new(),
    };
    for clipped in &output.shapes {
        walk(&clipped.shape, &mut render);
    }
    if let Some(update) = output.platform_output.accesskit_update {
        render.nodes = update
            .nodes
            .iter()
            .map(|(_, node)| Announced {
                role: node.role(),
                label: node.label().map(str::to_owned),
                value: node.value().map(str::to_owned),
                toggled: node.toggled(),
                selected: node.is_selected(),
                expanded: node.is_expanded(),
                description: node.description().map(str::to_owned),
                bounds: node.bounds().map(|bounds| {
                    egui::Rect::from_min_max(
                        egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                        egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                    )
                }),
            })
            .collect();
    }
    render
}

fn sized(input: egui::RawInput) -> egui::RawInput {
    sized_to(input, HEIGHT)
}

fn sized_to(input: egui::RawInput, height: f32) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(WIDTH, height),
        )),
        ..input
    }
}

fn keyed_with(key: egui::Key, modifiers: egui::Modifiers) -> egui::RawInput {
    sized(egui::RawInput {
        events: vec![egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed: true,
            repeat: false,
            modifiers,
        }],
        modifiers,
        ..Default::default()
    })
}

fn keyed(key: egui::Key) -> egui::RawInput {
    keyed_with(key, egui::Modifiers::NONE)
}

fn clicked(pos: egui::Pos2) -> egui::RawInput {
    sized(egui::RawInput {
        events: vec![
            egui::Event::PointerMoved(pos),
            egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
        ],
        ..Default::default()
    })
}

/// A click on the middle of the printed cell that says `message`.
fn click_on(render: &Render, message: &str) -> egui::RawInput {
    let cell = render
        .cell(message)
        .unwrap_or_else(|| panic!("{message:?} is printed"));
    clicked(cell.center())
}

/// Every cell sits under its own heading, fits the column it is printed in,
/// and is printed whole wherever the column has room for it: a cell is cut
/// only when its whole text would not fit.
///
/// This is the assertion the first version of this page could not have passed:
/// it fixed two columns at 58 px and 62 px and painted the rest over them.
#[test]
fn every_cell_is_painted_under_its_own_heading_and_fits_its_column() {
    let mut state = state_with_design_findings();
    let whole: Vec<String> = {
        let listed = listing(&state);
        let row = (0..listed.len())
            .filter_map(|index| listed.row(index))
            .find(|row| row.message == "R1 has no resistance")
            .expect("the error finding is listed");
        vec![
            severity_cell(&row),
            row.code.to_owned(),
            row.subject.clone(),
            row.producer.to_owned(),
            row.location.clone(),
            row.message.to_owned(),
        ]
    };
    let render = render(&mut state, sized(egui::RawInput::default()));

    let headings: Vec<egui::Pos2> = ProblemColumn::ALL
        .iter()
        .map(|column| {
            render
                .exact(column.label())
                .unwrap_or_else(|| panic!("the {} heading is painted", column.label()))
        })
        .collect();
    let header_y = headings[0].y;
    assert!(
        headings.iter().all(|pos| (pos.y - header_y).abs() < 1.0),
        "every heading sits on the header row: {headings:?}",
    );

    // The row for the error finding, found by the cell only it carries.
    let message = render
        .text("R1 has no resistance")
        .expect("the finding's message is painted");
    let row = render.row_at(message.y);
    assert_eq!(
        row.len(),
        ProblemColumn::ALL.len(),
        "every column of the row is painted: {row:?}",
    );
    let scroll = harness_scrollbar();
    let row_end = render.header_band().right() - scroll;
    for (index, (cell, heading)) in row.iter().zip(&headings).enumerate() {
        assert!(
            (cell.1.min.x - heading.x).abs() < 0.5,
            "the cell {:?} at {} is not under its heading at {}",
            cell.0,
            cell.1.min.x,
            heading.x,
        );
        // A column ends one gap short of the next heading; the last one ends
        // where the scrollbar's allowance begins.
        let column_end = headings
            .get(index + 1)
            .map_or(row_end, |next| next.x - CONSOLE_COLUMN_GAP);
        assert!(
            cell.1.right() <= column_end + 0.5,
            "the cell {:?} ends at {} past its column's end at {column_end}",
            cell.0,
            cell.1.right(),
        );
        // The face is monospace, so the painted cell gives the advance, and
        // the whole text's width follows from its length.
        if cell.0 != whole[index] {
            let advance = cell.1.width() / cell.0.chars().count() as f32;
            let needed = whole[index].chars().count() as f32 * advance;
            let room = column_end - heading.x;
            assert!(
                cell.0.contains('…') && needed > room + 0.5,
                "{:?} is cut to {:?} though its {needed} fits the column's {room}",
                whole[index],
                cell.0,
            );
        }
    }
    for (cell, whole) in row.iter().zip(&whole).take(4) {
        assert_eq!(
            &cell.0, whole,
            "the severity, rule, subject and producer fit whole at {WIDTH} px",
        );
    }
    assert_eq!(row[0].0, "Error", "the severity cell leads the row");
    assert!(
        row[1].0.starts_with("DRC-MISSING"),
        "the rule column names the rule: {:?}",
        row[1].0,
    );
    assert_eq!(row[2].0, "Component R1", "the subject is the object named");
    assert!(
        row[3].0.ends_with("design.rules"),
        "the producer column names who reported it: {:?}",
        row[3].0,
    );
    assert!(
        row[0].1.min.x >= render.header_band().left() + CONSOLE_MARK_GUTTER - 0.5,
        "the severity cell clears the mark gutter",
    );
}

/// The scrollbar allowance the column measurement makes, read from the style
/// the page is drawn in.
fn harness_scrollbar() -> f32 {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let style = ctx.global_style();
    style.spacing.scroll.bar_width + style.spacing.scroll.bar_inner_margin
}

/// With more findings than the body can show, its scrollbar is drawn — and the
/// rows still line up under the header drawn outside the scrolled area, and a
/// message too long for its column is cut short of the bar rather than
/// printed under it.
#[test]
fn with_a_scrollbar_the_rows_still_line_up_and_stop_short_of_it() {
    let mut state = open_state();
    let mut records: Vec<DiagnosticRecord> = (0..40)
        .map(|index| {
            record(
                &state,
                &format!("T-{index:02}"),
                DiagnosticLevel::Warning,
                &format!("finding {index:02}"),
            )
        })
        .collect();
    let long = "a message long enough that no column on a nine hundred point page \
                could print it whole, so the grid has to cut it where its column ends";
    records.insert(0, record(&state, "T-LONG", DiagnosticLevel::Error, long));
    publish(&mut state, records);

    let harness = Harness::new();
    harness.settle(&mut state);
    let render = harness.frame(&mut state, sized(egui::RawInput::default()));
    assert!(
        !harness.shows_whole(&render, "finding 39"),
        "precondition: the list is longer than the body, so the body scrolls",
    );

    let message_heading = render
        .exact(ProblemColumn::Message.label())
        .expect("the message heading is painted");
    let severity_heading = render
        .exact(ProblemColumn::Severity.label())
        .expect("the severity heading is painted");
    let cut = render
        .texts
        .iter()
        .find(|(text, _)| text.starts_with("a message long enough"))
        .expect("the long message is printed");
    assert!(cut.0.ends_with('…'), "it is cut, and says so: {:?}", cut.0);
    assert!(
        (cut.1.min.x - message_heading.x).abs() < 0.5,
        "with the bar drawn, the message still starts under its heading",
    );
    let bar_left = render.header_band().right() - harness_scrollbar();
    assert!(
        cut.1.right() <= bar_left + 0.5,
        "the cut message ends at {} under the scrollbar starting at {bar_left}",
        cut.1.right(),
    );
    let first = render.row_at(cut.1.min.y);
    assert!(
        (first[0].1.min.x - severity_heading.x).abs() < 0.5,
        "and every row's severity is under its heading: {first:?}",
    );
}

/// The ordered column says which way it is read, in words a reader can hear
/// and in a mark that is drawn rather than typed — the bundled face has no
/// arrow glyphs and would print a tofu box.
#[test]
fn the_ordered_column_says_which_way_and_draws_its_mark() {
    let mut state = state_with_design_findings();
    let unsorted = render(&mut state, sized(egui::RawInput::default()));
    assert!(
        unsorted
            .nodes
            .iter()
            .any(|node| node.role == Role::Button && node.says() == Some("Sort by Severity")),
        "an unordered heading offers to order by itself",
    );
    let header_marks = unsorted.triangles.len();

    state.problems_view.sort = Some(ProblemColumn::Severity);
    let ascending = render(&mut state, sized(egui::RawInput::default()));
    assert!(
        ascending
            .nodes
            .iter()
            .any(|node| node.role == Role::Button
                && node.says() == Some("Severity, sorted worst first")),
        "the ordered heading states the order it is read in",
    );
    assert_eq!(
        ascending.triangles.len(),
        header_marks + 1,
        "ordering a column adds exactly one painted mark",
    );

    state.problems_view.descending = true;
    let descending = render(&mut state, sized(egui::RawInput::default()));
    assert!(descending.nodes.iter().any(|node| node.role == Role::Button
        && node.says() == Some("Severity, sorted least severe first")),);
    // The mark itself, found in the header band rather than among the rows'
    // own problem marks, and read for which way it points: ascending draws its
    // apex up, descending draws it down.
    let apex_down = |render: &Render| {
        let heading = render.exact("Severity").expect("the heading is painted");
        let mark = render
            .triangles
            .iter()
            .find(|points| {
                points
                    .iter()
                    .all(|point| (point.y - heading.y).abs() < grid::HEADER_HEIGHT)
            })
            .unwrap_or_else(|| {
                panic!(
                    "the ordered column carries a painted mark on the header row at {}: {:?}",
                    heading.y, render.triangles
                )
            });
        mark[2].y > mark[0].y
    };
    assert!(!apex_down(&ascending), "ascending points its apex up");
    assert!(apex_down(&descending), "descending points its apex down");
}

/// Ordering by severity puts the worst finding first, and reversing it puts it
/// last. The fixture reports the warning first, so an unsorted page would
/// fail the first assertion.
#[test]
fn ordering_by_severity_lists_the_worst_finding_first() {
    let mut state = open_state();
    let warning = record(&state, "T-WARN", DiagnosticLevel::Warning, "the warning");
    let error = record(&state, "T-ERROR", DiagnosticLevel::Error, "the error");
    publish(&mut state, vec![warning, error]);
    let positions = |render: &Render| {
        (
            render.text("the error").expect("the error is listed").y,
            render.text("the warning").expect("the warning is listed").y,
        )
    };

    let as_reported = render(&mut state, sized(egui::RawInput::default()));
    let (error, warning) = positions(&as_reported);
    assert!(
        warning < error,
        "precondition: the producer reported the warning first",
    );

    state.problems_view.sort = Some(ProblemColumn::Severity);
    let (error, warning) = positions(&render(&mut state, sized(egui::RawInput::default())));
    assert!(
        error < warning,
        "the error at {error} must be listed above the warning at {warning}",
    );

    state.problems_view.descending = true;
    let (error, warning) = positions(&render(&mut state, sized(egui::RawInput::default())));
    assert!(warning < error, "reversing the order reverses the rows");
}

/// Findings with equal keys keep the order their producer reported them in,
/// whichever way the column is read.
#[test]
fn findings_with_equal_keys_keep_their_reported_order_either_way() {
    let mut state = open_state();
    let records = vec![
        record(&state, "T-B", DiagnosticLevel::Warning, "b reported first"),
        record(&state, "T-A", DiagnosticLevel::Warning, "a reported first"),
        record(&state, "T-B", DiagnosticLevel::Warning, "b reported second"),
        record(&state, "T-A", DiagnosticLevel::Warning, "a reported second"),
    ];
    publish(&mut state, records);

    let messages = |descending: bool| {
        let mut view = state.problems_view.clone();
        view.sort = Some(ProblemColumn::Code);
        view.descending = descending;
        let listed = listing(&state);
        let arrangement = Arrangement::build(&listed, &view);
        arrangement
            .listed_rows()
            .filter_map(|index| listed.row(index).map(|row| row.message.to_owned()))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        messages(false),
        [
            "a reported first",
            "a reported second",
            "b reported first",
            "b reported second"
        ],
    );
    assert_eq!(
        messages(true),
        [
            "b reported first",
            "b reported second",
            "a reported first",
            "a reported second"
        ],
    );
}

/// The location column orders by line and then column, as numbers: line 9
/// comes before line 10.
#[test]
fn locations_order_by_line_number_not_by_spelling() {
    let mut state = open_state();
    let tenth = Diagnostic::error("on the tenth line").with_line(Some(9));
    let ninth = Diagnostic::error("on the ninth line").with_line(Some(8));
    state.ui.netlist.diagnostics = Arc::new(
        NetlistDiagnosticCollection::try_new(vec![tenth, ninth], "")
            .expect("the fixture collection is well formed"),
    );
    state.problems_view.sort = Some(ProblemColumn::Location);

    let render = render(&mut state, sized(egui::RawInput::default()));
    let ninth = render.text("on the ninth line").expect("listed").y;
    let tenth = render.text("on the tenth line").expect("listed").y;
    assert!(
        ninth < tenth,
        "line 9 at {ninth} is listed above line 10 at {tenth}",
    );
}

/// A count beside a toggle describes what that toggle would list under the
/// other filters, not what the whole list holds.
#[test]
fn a_severity_count_describes_what_its_toggle_would_list() {
    let mut state = state_with_design_findings();
    netlist_finding_with_a_fix(&mut state);

    let view = state.problems_view.clone();
    let listed = listing(&state);
    let arrangement = Arrangement::build(&listed, &view);
    assert_eq!(
        arrangement.tally.severity(DiagnosticLevel::Error),
        2,
        "one rule error and one netlist error",
    );
    assert_eq!(arrangement.tally.severity(DiagnosticLevel::Warning), 1);
    assert_eq!(arrangement.listed, 3);
    drop(listed);

    // Narrowed to one producer, the error count describes what the severity
    // toggle would list *there* — not the two errors the whole list holds.
    state.problems_view.producer = Some("rspice.design.rules".to_owned());
    let view = state.problems_view.clone();
    let listed = listing(&state);
    let arrangement = Arrangement::build(&listed, &view);
    assert_eq!(arrangement.tally.severity(DiagnosticLevel::Error), 1);
    assert_eq!(arrangement.listed, 2);
    assert_eq!(
        arrangement.tally.producers.len(),
        2,
        "the producer control still offers every producer present",
    );
}

/// The waiver toggle says how many waived findings it would list under the
/// other filters, and keeps saying so while it is hiding them.
#[test]
fn the_waiver_toggle_counts_what_it_would_list() {
    let mut state = open_state();
    let mut waived = record(
        &state,
        "T-WAIVED",
        DiagnosticLevel::Error,
        "a waived finding",
    );
    waived.canonical.suppression = Some(waiver("T-WAIVED"));
    let open = record(
        &state,
        "T-OPEN",
        DiagnosticLevel::Warning,
        "an open finding",
    );
    publish(&mut state, vec![waived, open]);

    let listed = render(&mut state, sized(egui::RawInput::default()));
    assert!(listed.exact("Waived 1").is_some(), "{:?}", listed.texts);
    assert!(listed.has("a waived finding"));

    state.problems_view.waived = false;
    let hidden = render(&mut state, sized(egui::RawInput::default()));
    assert!(
        hidden.exact("Waived 1").is_some(),
        "hiding waived findings leaves the toggle saying what it would list",
    );
    assert!(!hidden.has("a waived finding"));

    state
        .problems_view
        .severities
        .toggle(DiagnosticLevel::Error);
    let narrowed = render(&mut state, sized(egui::RawInput::default()));
    assert!(
        narrowed.exact("Waived 0").is_some(),
        "with errors hidden too, the toggle would list nothing: {:?}",
        narrowed.texts,
    );
}

/// A waived current error keeps the shape of its mark without its colour:
/// it is not counted against the design, and a red cross would say it is.
#[test]
fn a_waived_error_is_marked_without_the_failure_colour() {
    let mut state = open_state();
    let mut waived = record(&state, "T-WAIVED", DiagnosticLevel::Error, "a waived error");
    waived.canonical.suppression = Some(waiver("T-WAIVED"));
    publish(&mut state, vec![waived]);
    let palette = crate::ui::tokens::Tokens::default().color;

    let render = render(&mut state, sized(egui::RawInput::default()));
    let row = row_of(render.cell("a waived error").expect("listed"));
    let gutter_end = render.header_band().left() + CONSOLE_MARK_GUTTER;
    let marked = |color: egui::Color32| {
        render
            .segments
            .iter()
            .filter(|(points, stroke)| {
                *stroke == color
                    && points.iter().all(|point| {
                        point.y >= row.top() && point.y <= row.bottom() && point.x <= gutter_end
                    })
            })
            .count()
    };
    assert_eq!(
        marked(palette.err),
        0,
        "no failure cross in the error colour"
    );
    assert_eq!(
        marked(palette.text_faint),
        2,
        "the cross's two strokes, in the faint ink",
    );
}

/// The text filter matches the rule, the subject and the message, and lists
/// nothing else.
#[test]
fn the_text_filter_lists_only_what_it_matches() {
    let mut state = state_with_design_findings();
    state.problems_view.text = "resistance".to_owned();
    let render = render(&mut state, sized(egui::RawInput::default()));
    assert!(render.has("R1 has no resistance"));
    assert!(
        !render.has("Wire ends open"),
        "a finding the filter excludes is not listed",
    );
    assert!(
        render.has("of 2 findings listed"),
        "the page says how much of the list it is showing: {:?}",
        render.texts,
    );
}

/// Grouping prints one header per group, announced with the group's own
/// counts and whether it is expanded, and the header collapses it.
#[test]
fn grouping_prints_a_header_that_carries_its_counts_and_collapses() {
    let mut state = state_with_design_findings();
    netlist_finding_with_a_fix(&mut state);
    state.problems_view.grouping = ProblemGrouping::Producer;
    let header = "rspice.design.rules · 1 error · 1 advisory";

    let grouped = render(&mut state, sized(egui::RawInput::default()));
    let node = grouped
        .node(header)
        .unwrap_or_else(|| panic!("the group header is announced as {header:?}"));
    assert_eq!(node.role, Role::Button);
    assert_eq!(
        node.expanded,
        Some(true),
        "an open group says it is expanded"
    );
    assert_eq!(node.toggled, None, "and not that it is a toggle pressed");
    assert!(grouped.has("R1 has no resistance"));

    state.problems_view.collapse("rspice.design.rules", true);
    let collapsed = render(&mut state, sized(egui::RawInput::default()));
    let node = collapsed
        .node(header)
        .expect("a collapsed group still states what it holds");
    assert_eq!(node.expanded, Some(false));
    assert!(
        !collapsed.has("R1 has no resistance"),
        "a collapsed group lists none of its rows",
    );
}

/// Copy copies what the count on it says: every listed finding, a collapsed
/// group's included.
#[test]
fn copying_what_is_listed_includes_collapsed_groups() {
    let mut state = state_with_design_findings();
    netlist_finding_with_a_fix(&mut state);
    state.problems_view.grouping = ProblemGrouping::Producer;
    state.problems_view.collapse("rspice.design.rules", true);

    let listed = listing(&state);
    let arrangement = Arrangement::build(&listed, &state.problems_view);
    assert_eq!(arrangement.listed, 3);
    let printed = (0..arrangement.lines())
        .filter(|line| matches!(arrangement.line(*line), Some(Line::Row(_))))
        .count();
    assert!(
        printed < arrangement.listed,
        "precondition: the collapsed group's findings are not printed",
    );
    let copied = copy_of_listed(&listed, &arrangement);
    assert_eq!(
        copied.lines().count(),
        arrangement.listed + 1,
        "a heading and one line per listed finding: {copied}",
    );
    assert!(copied.contains("R1 has no resistance"));
    assert!(copied.contains("Wire ends open"));
}

/// A message holding a tab, a line break and quotes stays one cell when it is
/// pasted into a sheet: the field is quoted and its quotes doubled.
#[test]
fn a_copied_row_keeps_tabs_and_line_breaks_inside_their_cell() {
    let mut state = open_state();
    let message = record(
        &state,
        "T-1",
        DiagnosticLevel::Error,
        "two\tcolumns\nand \"two\" lines",
    );
    publish(&mut state, vec![message]);
    let listed = listing(&state);
    let row = listed.row(0).expect("listed");
    assert_eq!(
        copy_of(&row),
        "Error\tT-1\tNode 'n1'\trspice.test\t\t\"two\tcolumns\nand \"\"two\"\" lines\"",
    );
}

/// The canvas tour and the grid stand on one finding. Before this the page had
/// no selection and the tour kept an index nothing else could read.
#[test]
fn the_canvas_tour_and_the_grid_stand_on_one_finding() {
    let mut state = state_with_design_findings();
    crate::schematic::view::violations::cycle_violation(&mut state, 1);
    let standing = state
        .diagnostics
        .current_finding()
        .expect("the tour stands on a finding");

    let message = state
        .diagnostics
        .batches()
        .flat_map(|batch| batch.records.iter())
        .find(|record| record.canonical.diagnostic_id == standing)
        .map(|record| record.message.clone())
        .expect("the finding the tour stands on is a published record");

    let render = render(&mut state, sized(egui::RawInput::default()));
    // The rows, told apart from the detail region's own lines by the role only
    // a row carries.
    let rows: Vec<&Announced> = render
        .nodes
        .iter()
        .filter(|node| {
            node.role == Role::ListBoxOption
                && node.says().is_some_and(|says| {
                    says.contains("R1 has no resistance") || says.contains("Wire ends open")
                })
        })
        .collect();
    assert_eq!(
        rows.len(),
        2,
        "both findings are reachable: {:?}",
        rows.iter().map(|node| node.says()).collect::<Vec<_>>(),
    );
    let selected: Vec<&&Announced> = rows.iter().filter(|node| node.selected()).collect();
    assert_eq!(
        selected.len(),
        1,
        "exactly one row is the current row: {:?}",
        rows.iter()
            .map(|node| (node.says(), node.selected()))
            .collect::<Vec<_>>(),
    );
    assert!(
        selected[0]
            .says()
            .is_some_and(|says| says.contains(&message)),
        "the selected row is the finding the tour stands on: {message:?}",
    );

    // And the selection is painted, not merely announced.
    let accent = crate::ui::tokens::Tokens::default()
        .color
        .accent
        .linear_multiply(0.14);
    assert!(
        render
            .fills
            .iter()
            .any(|(rect, fill)| *fill == accent
                && (rect.height() - CONSOLE_ROW_MIN_HEIGHT).abs() < 0.5),
        "the current row carries the selection fill",
    );
}

/// Clicking a row that leads somewhere follows it and hands the keyboard on:
/// the table does not keep focus, so the shortcuts of the surface the route
/// lands on are not held off by it. Next finding is the one checked; it is
/// suppressed whenever anything outside the canvas holds focus.
#[test]
fn following_a_row_with_a_click_leaves_the_shortcuts_working() {
    use crate::workbench::app_state::session::shortcuts::{
        ShortcutEnvironment, ShortcutInputSnapshot, runtime_command_platform,
    };
    use crate::workbench::commands::vocabulary::Command;

    let mut state = state_with_design_findings();
    let harness = Harness::new();
    harness.settle(&mut state);
    let render = harness.frame(&mut state, sized(egui::RawInput::default()));
    let _ = harness.frame(&mut state, click_on(&render, "R1 has no resistance"));
    assert!(
        state.diagnostics.take_pending_route().is_some(),
        "precondition: the click followed the finding",
    );
    harness.settle(&mut state);
    assert_eq!(harness.focused(), None, "nothing on the page kept focus");

    // The frame loop resolves shortcuts before it draws, with "something
    // other than the canvas has focus" read exactly as here.
    let next_finding = |harness: &Harness, state: &mut AppState| {
        let mut command = None;
        let _ = harness.ctx.run_ui(keyed(egui::Key::CloseBracket), |ui| {
            let ctx = ui.ctx().clone();
            let non_canvas_focus = ctx.memory(|memory| memory.focused().is_some());
            let snapshot =
                ctx.input(|input| ShortcutInputSnapshot::from_input_state(input, non_canvas_focus));
            let profile = state.ui.preferences.shortcuts().clone();
            command = state
                .shortcut_resolver
                .resolve(
                    &snapshot,
                    &profile,
                    runtime_command_platform(&ctx),
                    ShortcutEnvironment {
                        workspace: crate::workbench::state::Workspace::Design,
                        active_view: crate::state::ViewType::Schematic,
                        canvas_focus: false,
                        source_editor_focus: false,
                    },
                    std::time::Duration::from_secs(1),
                    |_| true,
                )
                .command;
            egui::CentralPanel::default().show(ui, |ui| problems(ui, state));
        });
        command
    };
    assert_eq!(
        next_finding(&harness, &mut state),
        Some(Command::NextViolation),
        "the next-finding shortcut works straight after the click",
    );

    // The same key while the table holds focus is held off, which is why the
    // table must not keep it.
    harness.focus_table();
    harness.settle(&mut state);
    assert_eq!(next_finding(&harness, &mut state), None);
}

/// The application root with a project open on the Design workspace and an
/// empty drawing, as the frame loop finds it.
fn design_app() -> crate::workbench::RSpiceApp {
    let mut app = crate::workbench::RSpiceApp::test_instance();
    app.state.project_lifecycle.authority.open_session();
    app.state
        .workbench
        .activate(crate::workbench::state::Workspace::Design);
    app.state.schematic = crate::state::SchematicState::default();
    app.state.schematic.session.editor.needs_fit = false;
    app.state.schematic.session.editor.needs_drawing_sheet_fit = false;
    app
}

/// One frame in the order the frame loop runs it: shortcuts first, then the
/// dock, then the central canvas, and last the route the dock asked for.
fn app_frame(
    ctx: &egui::Context,
    app: &mut crate::workbench::RSpiceApp,
    input: egui::RawInput,
) -> Render {
    collect(ctx.run_ui(input, |ui| {
        app.handle_shortcuts(ui.ctx());
        let whole = ui.max_rect();
        let (canvas, dock) = whole.split_top_bottom_at_y(whole.top() + 160.0);
        ui.scope_builder(egui::UiBuilder::new().max_rect(dock), |ui| {
            problems(ui, &mut app.state);
        });
        ui.scope_builder(egui::UiBuilder::new().max_rect(canvas), |ui| {
            crate::schematic::view::render_schematic_view(ui, &mut app.state, None);
        });
        crate::workbench::diagnostic_routes::apply_pending(ui.ctx(), app);
    }))
}

/// Following a finding to the drawing gives the canvas the keyboard, so the
/// canvas's own keys act on what the finding named straight away rather than
/// after a click on the canvas the reader was just taken to.
#[test]
fn following_a_finding_to_the_drawing_gives_the_canvas_the_keyboard() {
    use crate::workbench::app_state::session::shortcuts::engineering_canvas_has_focus;

    let mut app = design_app();
    let resistor = publish_design_findings(&mut app.state);
    let harness = Harness::new();
    let _ = app_frame(&harness.ctx, &mut app, sized(egui::RawInput::default()));
    let render = app_frame(&harness.ctx, &mut app, sized(egui::RawInput::default()));
    let _ = app_frame(
        &harness.ctx,
        &mut app,
        click_on(&render, "R1 has no resistance"),
    );
    assert!(
        app.state
            .schematic
            .session
            .editor
            .selection
            .has_component(resistor),
        "precondition: the route was followed to the resistor",
    );

    let _ = app_frame(&harness.ctx, &mut app, sized(egui::RawInput::default()));
    assert!(
        engineering_canvas_has_focus(
            &harness.ctx,
            crate::workbench::state::Workspace::Design,
            crate::state::ViewType::Schematic,
        ),
        "the canvas the finding led to holds the keyboard",
    );
    let _ = app_frame(&harness.ctx, &mut app, keyed(egui::Key::Delete));
    assert!(
        app.state
            .schematic
            .document()
            .components
            .iter()
            .all(|component| component.id != resistor),
        "Delete acts on the object the finding named",
    );
}

/// Escape on the first frame the table holds focus clears the row in the
/// application too. The frame loop resolves shortcuts before any dock is
/// drawn, and on that frame egui has already let go of the table, so without
/// the table's claim the Escape would be taken as Cancel and never reach it.
#[test]
fn escape_on_the_first_focused_frame_is_the_tables_and_not_cancel() {
    let mut app = design_app();
    publish_findings(&mut app.state, 3);
    let order = reported(&app.state);
    let harness = Harness::new();

    let mut taken = false;
    let _ = harness.ctx.run_ui(keyed(egui::Key::Escape), |ui| {
        app.handle_shortcuts(ui.ctx());
        taken = !ui.input(|input| input.key_pressed(egui::Key::Escape));
    });
    assert!(
        taken,
        "precondition: with nothing focused, Escape is a shortcut's"
    );

    let _ = app_frame(&harness.ctx, &mut app, sized(egui::RawInput::default()));
    let render = app_frame(&harness.ctx, &mut app, sized(egui::RawInput::default()));
    // A row with nowhere to go gives the table focus on the frame it is
    // clicked, which leaves the next frame its first focused one.
    let _ = app_frame(&harness.ctx, &mut app, click_on(&render, "finding 01"));
    assert_eq!(app.state.diagnostics.current_finding(), Some(order[1]));
    assert_eq!(harness.focused(), Some(grid::body_id()));

    let _ = app_frame(&harness.ctx, &mut app, keyed(egui::Key::Escape));
    assert_eq!(
        app.state.diagnostics.current_finding(),
        None,
        "the Escape cleared the row",
    );
}

/// A row with nowhere to go keeps the reader in the table: the click makes it
/// current and gives the table the keyboard, so the arrows go on from it.
#[test]
fn clicking_a_row_with_nowhere_to_go_keeps_the_keyboard_in_the_table() {
    let mut state = state_with_findings(4);
    let order = reported(&state);
    let harness = Harness::new();
    harness.settle(&mut state);
    let render = harness.frame(&mut state, sized(egui::RawInput::default()));
    let _ = harness.frame(&mut state, click_on(&render, "finding 01"));
    assert_eq!(state.diagnostics.current_finding(), Some(order[1]));
    assert_eq!(harness.focused(), Some(grid::body_id()));
    assert!(state.diagnostics.take_pending_route().is_none());

    harness.settle(&mut state);
    let _ = harness.frame(&mut state, keyed(egui::Key::ArrowDown));
    assert_eq!(state.diagnostics.current_finding(), Some(order[2]));
}

/// The arrows move the one shared selection, Enter follows the current row —
/// the row the keys moved to, not the one first clicked — and Escape clears
/// it.
#[test]
fn the_keyboard_moves_follows_and_clears_the_current_finding() {
    let mut state = state_with_design_findings();
    let order = reported(&state);
    assert_eq!(order.len(), 2, "the fixture publishes two findings");
    let route_of = |state: &AppState, finding: uuid::Uuid| {
        let listed = listing(state);
        (0..listed.len())
            .filter_map(|index| listed.row(index))
            .find(|row| row.id == finding)
            .and_then(|row| row.route)
    };
    let second_route = route_of(&state, order[1]).expect("the wire finding leads somewhere");

    let harness = Harness::new();
    harness.settle(&mut state);
    harness.focus_table();
    let _ = harness.frame(&mut state, keyed(egui::Key::ArrowDown));
    assert_eq!(
        state.diagnostics.current_finding(),
        Some(order[0]),
        "Down from nothing current stands on the first listed finding",
    );
    let _ = harness.frame(&mut state, keyed(egui::Key::ArrowDown));
    assert_eq!(state.diagnostics.current_finding(), Some(order[1]));
    let _ = harness.frame(&mut state, keyed(egui::Key::ArrowUp));
    assert_eq!(state.diagnostics.current_finding(), Some(order[0]));
    let _ = harness.frame(&mut state, keyed(egui::Key::ArrowDown));

    let _ = harness.frame(&mut state, keyed(egui::Key::Enter));
    assert_eq!(
        state.diagnostics.take_pending_route(),
        Some(second_route),
        "Enter follows the finding the keys stand on",
    );
    assert_eq!(
        harness.focused(),
        None,
        "and hands the keyboard to where it leads",
    );

    harness.focus_table();
    harness.settle(&mut state);
    let _ = harness.frame(&mut state, keyed(egui::Key::Escape));
    assert_eq!(
        state.diagnostics.current_finding(),
        None,
        "Escape clears the current finding",
    );
    assert_eq!(harness.focused(), None, "and lets the table go");
}

/// Escape on the very first frame the table holds focus still clears the
/// row, though egui lets go of focus on that frame before the page runs.
#[test]
fn escape_straight_after_focusing_the_table_clears_the_row() {
    let mut state = state_with_findings(3);
    let order = reported(&state);
    state.diagnostics.set_current_finding(Some(order[1]));
    let harness = Harness::new();
    harness.settle(&mut state);
    harness.focus_table();
    let _ = harness.frame(&mut state, keyed(egui::Key::Escape));
    assert_eq!(state.diagnostics.current_finding(), None);
}

/// Arrows on consecutive frames, the first of them on the frame the table
/// gained focus, each move exactly one row, and the table keeps focus
/// throughout: egui's own focus never moves past a row.
#[test]
fn arrows_on_consecutive_frames_move_one_row_each() {
    let mut state = state_with_findings(6);
    let order = reported(&state);
    let harness = Harness::new();
    harness.settle(&mut state);
    harness.focus_table();
    for expected in &order[..4] {
        let _ = harness.frame(&mut state, keyed(egui::Key::ArrowDown));
        assert_eq!(state.diagnostics.current_finding(), Some(*expected));
        assert_eq!(harness.focused(), Some(grid::body_id()));
    }
}

/// Home and End go to the ends of the list; the page keys move by what the
/// body shows and stop at the ends rather than wrapping.
#[test]
fn home_end_and_the_page_keys_move_through_the_list() {
    let mut state = state_with_findings(60);
    let order = reported(&state);
    let harness = Harness::new();
    harness.settle(&mut state);
    harness.focus_table();

    let _ = harness.frame(&mut state, keyed(egui::Key::End));
    assert_eq!(state.diagnostics.current_finding(), order.last().copied());
    let _ = harness.frame(&mut state, keyed(egui::Key::PageDown));
    assert_eq!(
        state.diagnostics.current_finding(),
        order.last().copied(),
        "a page key stops at the end of the list",
    );
    let _ = harness.frame(&mut state, keyed(egui::Key::Home));
    assert_eq!(state.diagnostics.current_finding(), Some(order[0]));
    let _ = harness.frame(&mut state, keyed(egui::Key::PageUp));
    assert_eq!(state.diagnostics.current_finding(), Some(order[0]));

    let _ = harness.frame(&mut state, keyed(egui::Key::PageDown));
    let paged = order
        .iter()
        .position(|finding| state.diagnostics.current_finding() == Some(*finding))
        .expect("a listed finding is current");
    assert!(
        paged > 1 && paged < order.len() - 1,
        "a page moves by more than a row and less than the list: {paged}",
    );
    let render = harness.frame(&mut state, sized(egui::RawInput::default()));
    assert!(
        harness.shows_whole(&render, &format!("finding {paged:02}")),
        "and the row it lands on is shown",
    );
}

/// The groups `state` lists under its own reading position, by label, in the
/// order they are printed.
fn group_labels(state: &AppState) -> Vec<String> {
    let listed = listing(state);
    let arrangement = Arrangement::build(&listed, &state.problems_view);
    (0..arrangement.lines())
        .filter_map(|line| match arrangement.line(line) {
            Some(Line::Group(group)) => arrangement.group(group).map(|group| group.label.clone()),
            Some(Line::Row(_)) | None => None,
        })
        .collect()
}

/// A group closed with the mouse opens again from the keyboard alone, as a
/// tree does: the header is a line the arrows stand on, Right opens it and
/// goes into it, Left comes back out to it and closes it, and Enter and Space
/// open and close it.
#[test]
fn a_group_closed_with_the_mouse_opens_again_from_the_keyboard() {
    let mut state = state_with_design_findings();
    netlist_finding_with_a_fix(&mut state);
    state.problems_view.grouping = ProblemGrouping::Producer;
    let label = "rspice.design.rules";
    let first_of_group = {
        let listed = listing(&state);
        (0..listed.len())
            .filter_map(|index| listed.row(index))
            .find(|row| row.message == "R1 has no resistance")
            .map(|row| row.id)
            .expect("the resistor's finding is listed")
    };
    let harness = Harness::new();
    harness.settle(&mut state);
    let render = harness.frame(&mut state, sized(egui::RawInput::default()));
    let header = render
        .nodes
        .iter()
        .find(|node| {
            node.expanded.is_some() && node.says().is_some_and(|says| says.starts_with(label))
        })
        .and_then(|node| node.bounds)
        .expect("the group's header is drawn");

    let _ = harness.frame(&mut state, clicked(header.center()));
    assert!(
        state.problems_view.is_collapsed(label),
        "the click closed it"
    );
    assert_eq!(
        harness.focused(),
        Some(grid::body_id()),
        "and left the keyboard in the table",
    );
    assert_eq!(state.problems_view.current_group.as_deref(), Some(label));
    assert_eq!(state.diagnostics.current_finding(), None);

    harness.settle(&mut state);
    let _ = harness.frame(&mut state, keyed(egui::Key::ArrowRight));
    assert!(!state.problems_view.is_collapsed(label), "Right opens it");
    harness.settle(&mut state);
    let _ = harness.frame(&mut state, keyed(egui::Key::ArrowRight));
    assert_eq!(
        state.diagnostics.current_finding(),
        Some(first_of_group),
        "Right on an open header goes to its first finding",
    );
    assert_eq!(state.problems_view.current_group, None);

    let _ = harness.frame(&mut state, keyed(egui::Key::ArrowLeft));
    assert_eq!(
        state.problems_view.current_group.as_deref(),
        Some(label),
        "Left on a finding goes to its header",
    );
    assert_eq!(state.diagnostics.current_finding(), None);
    assert!(!state.problems_view.is_collapsed(label));
    let _ = harness.frame(&mut state, keyed(egui::Key::ArrowLeft));
    assert!(
        state.problems_view.is_collapsed(label),
        "Left on an open header closes it",
    );

    harness.settle(&mut state);
    let _ = harness.frame(&mut state, keyed(egui::Key::Enter));
    assert!(!state.problems_view.is_collapsed(label), "Enter opens it");
    harness.settle(&mut state);
    let _ = harness.frame(&mut state, keyed(egui::Key::Space));
    assert!(state.problems_view.is_collapsed(label), "Space closes it");
    assert_eq!(harness.focused(), Some(grid::body_id()));
}

/// With every group closed the lines are the headers alone, and the arrows,
/// Home and End still move over them.
#[test]
fn with_every_group_closed_the_arrows_still_move() {
    let mut state = state_with_design_findings();
    netlist_finding_with_a_fix(&mut state);
    state.problems_view.grouping = ProblemGrouping::Producer;
    let labels = group_labels(&state);
    assert_eq!(labels.len(), 2, "precondition: two groups: {labels:?}");
    for label in &labels {
        state.problems_view.collapse(label, true);
    }
    let standing = |state: &AppState| state.problems_view.current_group.clone();

    let harness = Harness::new();
    harness.settle(&mut state);
    harness.focus_table();
    let _ = harness.frame(&mut state, keyed(egui::Key::ArrowDown));
    assert_eq!(standing(&state).as_ref(), Some(&labels[0]));
    let _ = harness.frame(&mut state, keyed(egui::Key::ArrowDown));
    assert_eq!(standing(&state).as_ref(), Some(&labels[1]));
    let _ = harness.frame(&mut state, keyed(egui::Key::ArrowDown));
    assert_eq!(
        standing(&state).as_ref(),
        Some(&labels[0]),
        "the arrows wrap as they do over findings",
    );
    let _ = harness.frame(&mut state, keyed(egui::Key::End));
    assert_eq!(standing(&state).as_ref(), Some(&labels[1]));
    let _ = harness.frame(&mut state, keyed(egui::Key::Home));
    assert_eq!(standing(&state).as_ref(), Some(&labels[0]));
    assert_eq!(state.diagnostics.current_finding(), None);
    assert_eq!(harness.focused(), Some(grid::body_id()));
}

/// ArrowUp on the frame the table gains focus moves within the table. egui
/// reads that arrow as a move to the widget above before any page runs, and
/// above the table are the column headings.
#[test]
fn arrow_up_on_the_first_focused_frame_stays_in_the_table() {
    let mut state = state_with_findings(4);
    let order = reported(&state);
    let harness = Harness::new();
    harness.settle(&mut state);
    harness.focus_table();
    let _ = harness.frame(&mut state, keyed(egui::Key::ArrowUp));
    assert_eq!(
        harness.focused(),
        Some(grid::body_id()),
        "the arrow did not carry the keyboard to a heading",
    );
    assert_eq!(
        state.diagnostics.current_finding(),
        order.last().copied(),
        "Up from nothing stands on the last line",
    );
}

/// Tab into the table frames the line it stands on with the focus ring; a
/// pointer press that gives the table focus frames nothing.
#[test]
fn the_focus_ring_shows_after_tab_and_not_after_a_press() {
    let mut state = state_with_findings(5);
    let order = reported(&state);
    state.diagnostics.set_current_finding(Some(order[2]));
    let accent = crate::ui::tokens::Tokens::default().color.accent;
    let ring = |render: &Render| {
        render
            .outlines
            .iter()
            .find(|(_, stroke)| *stroke == egui::Stroke::new(2.0, accent))
            .map(|(rect, _)| *rect)
    };

    let harness = Harness::new();
    harness.settle(&mut state);
    let last_heading = egui::Id::new(("workbench.problems.header", ProblemColumn::Message.label()));
    harness
        .ctx
        .memory_mut(|memory| memory.request_focus(last_heading));
    harness.settle(&mut state);
    let _ = harness.frame(&mut state, keyed(egui::Key::Tab));
    let tabbed = harness.frame(&mut state, sized(egui::RawInput::default()));
    assert_eq!(harness.focused(), Some(grid::body_id()));
    let framed = ring(&tabbed).expect("Tab shows the focus ring");
    let row = row_of(tabbed.cell("finding 02").expect("the current row is drawn"));
    assert!(
        framed.contains(row.center()) && framed.height() < 2.0 * CONSOLE_ROW_MIN_HEIGHT,
        "the ring frames the current row: {framed:?} around {row:?}",
    );

    harness
        .ctx
        .memory_mut(|memory| memory.surrender_focus(grid::body_id()));
    harness.settle(&mut state);
    let _ = harness.frame(&mut state, click_on(&tabbed, "finding 01"));
    let pressed = harness.frame(&mut state, sized(egui::RawInput::default()));
    assert_eq!(
        harness.focused(),
        Some(grid::body_id()),
        "precondition: the press gave the table the keyboard",
    );
    assert_eq!(ring(&pressed), None, "a press shows no ring");
}

/// With the table unfocused the page takes no keys: a reader typing in the
/// canvas does not move the findings list.
#[test]
fn the_table_takes_no_keys_while_it_is_unfocused() {
    let mut state = state_with_design_findings();
    let order = reported(&state);
    state.diagnostics.set_current_finding(Some(order[0]));
    let harness = Harness::new();
    harness.settle(&mut state);
    for key in [
        egui::Key::ArrowDown,
        egui::Key::End,
        egui::Key::Enter,
        egui::Key::Escape,
    ] {
        let _ = harness.frame(&mut state, keyed(key));
        assert_eq!(
            state.diagnostics.current_finding(),
            Some(order[0]),
            "{key:?}"
        );
    }
    assert!(state.diagnostics.take_pending_route().is_none());
}

/// The table is one Tab stop. Tab from the last heading lands on it, the next
/// Tab leaves it, and neither changes the current finding — Tab moves the
/// keyboard, not the selection the canvas shares.
#[test]
fn the_table_is_one_tab_stop() {
    let mut state = state_with_findings(5);
    let order = reported(&state);
    state.diagnostics.set_current_finding(Some(order[2]));
    let harness = Harness::new();
    harness.settle(&mut state);
    let last_heading = egui::Id::new(("workbench.problems.header", ProblemColumn::Message.label()));
    harness
        .ctx
        .memory_mut(|memory| memory.request_focus(last_heading));
    harness.settle(&mut state);

    let _ = harness.frame(&mut state, keyed(egui::Key::Tab));
    assert_eq!(harness.focused(), Some(grid::body_id()));
    assert_eq!(state.diagnostics.current_finding(), Some(order[2]));

    harness.settle(&mut state);
    let _ = harness.frame(&mut state, keyed(egui::Key::Tab));
    let after = harness.focused();
    assert_ne!(
        after,
        Some(grid::body_id()),
        "the next Tab leaves the table"
    );
    assert!(
        order
            .iter()
            .all(|finding| after != Some(grid::row_id(*finding))),
        "and does not stop on a row",
    );
    assert_eq!(state.diagnostics.current_finding(), Some(order[2]));

    harness.settle(&mut state);
    let _ = harness.frame(
        &mut state,
        keyed_with(egui::Key::Tab, egui::Modifiers::SHIFT),
    );
    // egui hands focus back a frame later, so `gained_focus` can fire.
    harness.settle(&mut state);
    assert_eq!(
        harness.focused(),
        Some(grid::body_id()),
        "Shift+Tab comes back to it",
    );
}

/// A finding the canvas tour makes current is brought into view, as one the
/// keys move to is.
#[test]
fn a_finding_made_current_elsewhere_is_scrolled_into_view() {
    let mut state = state_with_findings(60);
    let order = reported(&state);
    let harness = Harness::new();
    harness.settle(&mut state);
    let render = harness.frame(&mut state, sized(egui::RawInput::default()));
    assert!(
        !harness.shows_whole(&render, "finding 45"),
        "precondition: the finding starts out of view",
    );

    state.diagnostics.set_current_finding(Some(order[45]));
    harness.settle(&mut state);
    let render = harness.frame(&mut state, sized(egui::RawInput::default()));
    assert!(harness.shows_whole(&render, "finding 45"));
}

/// A row the body's edge cuts in half is out of view: made current, it is
/// scrolled until it shows whole.
#[test]
fn a_half_shown_row_made_current_is_scrolled_whole() {
    let mut state = state_with_findings(60);
    let order = reported(&state);
    // Standing on a finding far down leaves the body scrolled to wherever that
    // finding's bottom edge put it. Whether its top edge then cuts a row
    // depends on the body's height, so the page is tried at heights a point
    // apart until one does.
    for height in (400_u16..440).map(f32::from) {
        let input = || sized_to(egui::RawInput::default(), height);
        let harness = Harness::new();
        state.diagnostics.set_current_finding(Some(order[45]));
        let _ = harness.frame(&mut state, input());
        let _ = harness.frame(&mut state, input());
        let render = harness.frame(&mut state, input());
        let table = harness.table();
        let Some(cut) = (0..order.len())
            .map(|index| format!("finding {index:02}"))
            .find(|message| {
                render.cell(message).is_some_and(|cell| {
                    let row = row_of(cell);
                    row.top() < table.top() - 0.5 && row.bottom() > table.top() + 0.5
                })
            })
        else {
            continue;
        };

        let index: usize = cut["finding ".len()..].parse().expect("a numbered finding");
        state.diagnostics.set_current_finding(Some(order[index]));
        let _ = harness.frame(&mut state, input());
        let render = harness.frame(&mut state, input());
        assert!(harness.shows_whole(&render, &cut), "{cut} shows whole");
        return;
    }
    panic!("precondition: some page height between 400 and 440 points cuts a row");
}

/// Clicking a row low in a full body opens the detail region under it, which
/// takes height from the body. The clicked row stays in view, and the table
/// keeps the keyboard it gave it.
#[test]
fn a_clicked_row_stays_in_view_when_the_detail_region_opens() {
    let mut state = state_with_findings(60);
    let harness = Harness::new();
    harness.settle(&mut state);
    let render = harness.frame(&mut state, sized(egui::RawInput::default()));
    let table = harness.table();
    let lowest = render
        .texts
        .iter()
        .filter(|(text, cell)| {
            text.starts_with("finding ") && row_of(*cell).bottom() <= table.bottom() + 0.5
        })
        .max_by(|left, right| left.1.min.y.total_cmp(&right.1.min.y))
        .map(|(text, _)| text.clone())
        .expect("rows are shown");

    let _ = harness.frame(&mut state, click_on(&render, &lowest));
    harness.settle(&mut state);
    let render = harness.frame(&mut state, sized(egui::RawInput::default()));
    assert!(
        harness.table().height() < table.height(),
        "precondition: the detail region took height from the body",
    );
    assert!(
        harness.shows_whole(&render, &lowest),
        "{lowest} stays in view"
    );
    assert_eq!(harness.focused(), Some(grid::body_id()));
}

/// The lowest message the body shows whole.
fn lowest_whole_row(harness: &Harness, render: &Render) -> String {
    let table = harness.table();
    render
        .texts
        .iter()
        .filter(|(text, cell)| {
            text.starts_with("finding ")
                && row_of(*cell).top() >= table.top() - 0.5
                && row_of(*cell).bottom() <= table.bottom() + 0.5
        })
        .max_by(|left, right| left.1.min.y.total_cmp(&right.1.min.y))
        .map(|(text, _)| text.clone())
        .expect("rows are shown")
}

/// A dock that loses height brings back a current row the lost height hid.
#[test]
fn a_shrinking_body_brings_back_the_current_row_it_hid() {
    let mut state = state_with_findings(60);
    let order = reported(&state);
    // The detail region is open throughout, so the only height the body
    // loses is the dock's own.
    state.diagnostics.set_current_finding(Some(order[0]));
    let harness = Harness::new();
    harness.settle(&mut state);
    let render = harness.frame(&mut state, sized(egui::RawInput::default()));
    let lowest = lowest_whole_row(&harness, &render);
    let index: usize = lowest["finding ".len()..]
        .parse()
        .expect("a numbered finding");
    state.diagnostics.set_current_finding(Some(order[index]));
    harness.settle(&mut state);

    let smaller = || {
        sized_to(
            egui::RawInput::default(),
            HEIGHT - 3.0 * CONSOLE_ROW_MIN_HEIGHT,
        )
    };
    let _ = harness.frame(&mut state, smaller());
    let render = harness.frame(&mut state, smaller());
    assert!(
        harness.shows_whole(&render, &lowest),
        "{lowest} is brought back into view",
    );
}

/// A dock that loses height leaves the reader where they scrolled to when the
/// current row was not in view: the reader had left it, and the lost height
/// hid nothing they were looking at.
#[test]
fn a_shrinking_body_keeps_the_readers_place_away_from_the_current_row() {
    let mut state = state_with_findings(60);
    let order = reported(&state);
    state.diagnostics.set_current_finding(Some(order[2]));
    let harness = Harness::new();
    harness.settle(&mut state);
    harness.settle(&mut state);
    let away = 30.0 * CONSOLE_ROW_MIN_HEIGHT;
    harness.scroll_body_to(away);
    let render = harness.frame(&mut state, sized(egui::RawInput::default()));
    assert!(
        !harness.shows_whole(&render, "finding 02"),
        "precondition: the reader scrolled away from the current row",
    );
    let offset = harness.body_offset();

    let smaller = || {
        sized_to(
            egui::RawInput::default(),
            HEIGHT - 3.0 * CONSOLE_ROW_MIN_HEIGHT,
        )
    };
    let _ = harness.frame(&mut state, smaller());
    let render = harness.frame(&mut state, smaller());
    assert!(
        !harness.shows_whole(&render, "finding 02"),
        "the current row is not pulled back",
    );
    assert!(
        (harness.body_offset() - offset).abs() < 0.5,
        "the body stays where the reader scrolled it: {} then {}",
        offset,
        harness.body_offset(),
    );
}

/// A note under the rows that wraps is given the height it wraps to: the body
/// gives up exactly that much for it. Given one line, the note would push the
/// detail region under it off the bottom of the dock.
#[test]
fn a_wrapped_note_takes_its_wrapped_height_from_the_body() {
    // More rule findings than the body shows, so the body is as tall as it
    // was given rather than as tall as its rows.
    let mut state = open_state();
    let mut result = DrcResult::new();
    for index in 0..40 {
        result.add_violation(DrcViolation::new(
            index,
            DrcViolationType::DanglingWire,
            format!("wire {index:02} ends open"),
            DrcLocation::Point {
                x: 10.0 * index as f64,
                y: 0.0,
            },
        ));
    }
    result.completed = true;
    state
        .publish_active_design_check_result(result)
        .expect("the fixture publishes a current receipt");
    state
        .diagnostics
        .set_current_finding(reported(&state).first().copied());
    let narrow = || egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(360.0, HEIGHT),
        )),
        ..Default::default()
    };
    let harness = Harness::new();
    let _ = harness.frame(&mut state, narrow());
    let _ = harness.frame(&mut state, narrow());
    let without_note = harness.table().height();

    state.workspace.content.connectivity.policy.width_mismatch =
        crate::state::BundleWidthMismatchPolicy::ExplicitSliceOrExtend;
    let _ = harness.frame(&mut state, narrow());
    let render = harness.frame(&mut state, narrow());
    let note = render
        .cell("Faint findings were reported")
        .expect("the stale note is drawn");
    assert!(
        note.height() > 1.5 * CONSOLE_ROW_MIN_HEIGHT,
        "precondition: the note wraps at this width: {note:?}",
    );
    let given_up = without_note - harness.table().height();
    assert!(
        (given_up - note.height()).abs() < 0.5,
        "the body gave up {given_up} for a note {} tall",
        note.height(),
    );
}

/// Every row is an option of the list the table is, whether or not it leads
/// anywhere, and says whether it is the current one as a list does: selected,
/// rather than a toggle pressed or not. One role for both kinds of row, so the
/// row that leads nowhere says so in its description.
#[test]
fn every_row_announces_as_an_option_of_the_list() {
    let mut state = state_with_design_findings();
    let nowhere = record(
        &state,
        "TEST-NOWHERE",
        DiagnosticLevel::Error,
        "a finding with nowhere to go",
    );
    publish(&mut state, vec![nowhere]);
    let listed = listing(&state);
    let followed = (0..listed.len())
        .filter_map(|index| listed.row(index))
        .find(|row| row.message == "R1 has no resistance")
        .map(|row| row.id);
    state.diagnostics.set_current_finding(followed);

    let render = render(&mut state, sized(egui::RawInput::default()));
    for (message, selected, description) in [
        ("R1 has no resistance", true, None),
        (
            "a finding with nowhere to go",
            false,
            Some("Has no location to go to"),
        ),
    ] {
        let row = render
            .nodes
            .iter()
            .find(|node| {
                node.role == Role::ListBoxOption
                    && node.says().is_some_and(|says| says.contains(message))
            })
            .unwrap_or_else(|| panic!("{message:?} is an option of the list"));
        assert_eq!(
            row.selected,
            Some(selected),
            "{message:?} says whether it is the current row",
        );
        assert_eq!(row.toggled, None, "{message:?} is not a toggle");
        assert_eq!(
            row.description.as_deref(),
            description,
            "{message:?} says whether it leads anywhere",
        );
    }
}

/// Explain states the rule's own words, and is drawn only where the rule has
/// words of its own.
#[test]
fn explain_is_offered_only_where_the_rule_has_words() {
    let mut state = state_with_design_findings();
    state
        .diagnostics
        .set_current_finding(reported(&state).first().copied());

    let unopened = render(&mut state, sized(egui::RawInput::default()));
    assert!(
        unopened.has("Explain"),
        "a rule finding offers its rule: {:?}",
        unopened.texts,
    );
    state.problems_view.explain = true;
    let explained = render(&mut state, sized(egui::RawInput::default()));
    assert!(
        explained.has(
            rspice_design::drc::DrcViolationType::MissingParameter
                .rule()
                .explanation
        ),
        "Explain states the rule in the rule table's own words: {:?}",
        explained.texts,
    );

    // A failed run is not an instance of a rule, so there is nothing to state
    // and no control that says there is.
    let mut run = open_state();
    run.publish_run_failure("Transient", "Analysis failed: no convergence", None, 1);
    run.diagnostics
        .set_current_finding(reported(&run).first().copied());
    let rendered = render(&mut run, sized(egui::RawInput::default()));
    assert!(
        rendered.has("Analysis failed: no convergence"),
        "the failure is listed",
    );
    assert!(
        !rendered.has("Explain"),
        "nothing offers to state a rule that has no words",
    );
}

/// A fix is offered only where the producer populated one, and it is labelled
/// with the fix's own words.
#[test]
fn a_fix_is_offered_only_where_the_producer_offered_edits() {
    let mut state = open_state();
    netlist_finding_with_a_fix(&mut state);
    state
        .diagnostics
        .set_current_finding(reported(&state).first().copied());
    let offered = render(&mut state, sized(egui::RawInput::default()));
    assert!(
        offered.has("Replace with nch"),
        "the fix is offered in its own words: {:?}",
        offered.texts,
    );

    let mut rules = state_with_design_findings();
    rules
        .diagnostics
        .set_current_finding(reported(&rules).first().copied());
    let unoffered = render(&mut rules, sized(egui::RawInput::default()));
    assert!(
        !unoffered.has("Replace with"),
        "a producer that offered no edits gets no fix control",
    );
}

/// The detail region says where the finding's batch stands only where that
/// is news: a current finding states nothing, and a stale one says what moved.
#[test]
fn the_detail_region_states_where_the_batch_stands_only_when_it_is_not_current() {
    let mut state = state_with_design_findings();
    state
        .diagnostics
        .set_current_finding(reported(&state).first().copied());

    let current = render(&mut state, sized(egui::RawInput::default()));
    assert!(
        current.has("R1 has no resistance")
            && current.has("Missing parameter \u{00b7} DRC-MISSING-PARAMETER"),
        "precondition: the detail region is drawn: {:?}",
        current.texts,
    );
    assert!(
        current.exact("Current").is_none() && !current.has("Stale"),
        "a current finding has no standing line: {:?}",
        current.texts,
    );

    // Move the inputs under the batch. The finding keeps its row and now says
    // what moved rather than only that it is stale.
    state.workspace.content.connectivity.policy.width_mismatch =
        crate::state::BundleWidthMismatchPolicy::ExplicitSliceOrExtend;
    let reason = {
        use crate::workbench::app_state::diagnostic_store::listing::RowStanding;
        let listed = listing(&state);
        match listed.row(0).expect("the finding keeps its row").standing {
            RowStanding::Stale(reason) => reason,
            standing => panic!("the moved inputs stale the batch: {standing:?}"),
        }
    };
    let stale = render(&mut state, sized(egui::RawInput::default()));
    let line = format!("Stale · {reason}");
    assert!(
        stale.exact(&line).is_some(),
        "a stale finding states its producer's reason as it stands, {line:?}: {:?}",
        stale.texts,
    );
    assert!(
        stale.has("Stale Error"),
        "and its row says so in the severity column",
    );
    assert!(
        stale.has("Select one to see what moved") && !stale.has("Run the checks"),
        "the note under the rows names no remedy that only some producers take: {:?}",
        stale.texts,
    );
}

/// A place a finding also names is listed, and is offered as a jump only
/// where something can resolve it. A rule finding's related places are in the
/// drawing, which no document route addresses, so they are stated.
#[test]
fn a_related_place_is_stated_even_where_it_cannot_be_followed() {
    let mut state = open_state();
    let mut canonical = rspice_app_types::diagnostics::CanonicalDiagnosticMetadata::current(
        "rspice.test",
        "TEST-RELATED",
        "doc",
        None,
        0,
        "validation",
        "two outputs drive this net",
    );
    canonical.related_locations = vec![rspice_app_types::diagnostics::DiagnosticRelatedLocation {
        document_id: "user/top/schematic".to_owned(),
        range: None,
        message: "M2 also drives it".to_owned(),
    }];
    let record = DiagnosticRecord::new(
        canonical,
        DiagnosticLevel::Error,
        DiagnosticSubject::new(state.workspace.content.project.id(), "Net 'out'"),
        "two outputs drive this net",
    );
    publish(&mut state, vec![record]);
    state
        .diagnostics
        .set_current_finding(reported(&state).first().copied());

    let render = render(&mut state, sized(egui::RawInput::default()));
    assert!(
        render.has("M2 also drives it"),
        "the related place is stated: {:?}",
        render.texts,
    );
    assert!(
        render.exact("Go").is_none(),
        "and not offered as a jump nothing can resolve",
    );
}

/// Closing the project, or replacing it with another, forgets how the last
/// one's findings were being read: the next project's findings came from
/// somewhere else, and a filter carried over would hide conditions nobody
/// there chose to hide.
#[test]
fn closing_or_replacing_the_project_forgets_the_reading_position() {
    use crate::workbench::app_state::problems_view::ProblemsView;
    use crate::workbench::workflows::project_workflow::{
        close_project_discard, create_new_project,
    };

    fn read_narrowly(state: &mut AppState) {
        let view = &mut state.problems_view;
        view.text = "ground".to_owned();
        view.producer = Some("rspice.design.rules".to_owned());
        view.waived = false;
        view.severities.toggle(DiagnosticLevel::Hint);
        view.order_by(ProblemColumn::Code);
        view.grouping = ProblemGrouping::Code;
        view.collapse("DRC-FLOATING-NODE", true);
        view.explain = true;
    }

    let mut state = AppState::default();
    read_narrowly(&mut state);
    assert!(close_project_discard(&mut state));
    assert_eq!(state.problems_view, ProblemsView::default());

    read_narrowly(&mut state);
    create_new_project(&mut state);
    assert_eq!(state.problems_view, ProblemsView::default());
}

/// The page is not a severity filter over the session log.
///
/// Before the canonical store it read the log buffer, so every refusal,
/// engine warning and system notice that had ever been printed stood here
/// as a permanent problem — about 715 refusal sites' worth.
#[test]
fn a_logged_failure_is_not_a_problem() {
    let mut state = open_state();
    state.log_buffer.log(
        crate::diagnostics::LogSeverity::Error,
        crate::diagnostics::LogSource::Engine,
        "the engine printed a failure",
        None,
    );

    assert_eq!(active_problem_count(&state, 0), ProblemCount::default());
    let render = render(&mut state, sized(egui::RawInput::default()));
    assert!(
        !render.has("the engine printed a failure"),
        "the page lists records, not log entries: {:?}",
        render.texts,
    );
}

#[test]
fn counts_name_their_severities_separately() {
    let count = ProblemCount {
        errors: 1,
        advisories: 12,
    };
    assert_eq!(count.total(), 13);
    assert_eq!(count.announcement(), "1 error, 12 advisories");
}

/// A row that says nothing but its severity, standing and waiver.
fn bare_row(
    level: DiagnosticLevel,
    standing: RowStanding,
    suppression: Option<&rspice_app_types::diagnostics::DiagnosticSuppression>,
) -> DiagnosticRow<'_> {
    DiagnosticRow {
        id: uuid::Uuid::nil(),
        code: "T-1",
        producer: "rspice.test",
        level,
        standing,
        subject: String::new(),
        location: String::new(),
        message: "",
        details: "",
        rule: "",
        related: &[],
        quick_fixes: &[],
        suppression,
        route: None,
        waiver_action: None,
    }
}

/// A finding whose inputs moved says it is stale. One whose inputs cannot be
/// read at all is not known to be stale, and says it cannot be evaluated.
#[test]
fn the_severity_cell_tells_an_unreadable_finding_from_a_stale_one() {
    let cell = |standing| severity_cell(&bare_row(DiagnosticLevel::Error, standing, None));
    assert_eq!(cell(RowStanding::Current), "Error");
    assert_eq!(
        cell(RowStanding::Stale("the drawing changed".to_owned())),
        "Stale Error",
    );
    assert_eq!(
        cell(RowStanding::Unavailable(
            "the receipt cannot be read".to_owned()
        )),
        "Unavailable Error",
    );
}

/// Every severity cell the page can print fits the cell the column is
/// measured against: the evaluable cells the narrower one, and a finding that
/// cannot be evaluated the wider one the column takes while such a finding is
/// listed.
#[test]
fn every_severity_cell_fits_the_widest_measured_cell() {
    let mut widest = String::new();
    let mut widest_unavailable = String::new();
    for level in DiagnosticLevel::ALL {
        for standing in [
            RowStanding::Current,
            RowStanding::Stale("inputs".to_owned()),
            RowStanding::Unavailable("unreadable".to_owned()),
        ] {
            for waiver in [None, Some(waiver("T-1"))] {
                let cell = severity_cell(&bare_row(level, standing.clone(), waiver.as_ref()));
                let (bound, longest) = if matches!(standing, RowStanding::Unavailable(_)) {
                    (
                        grid::WIDEST_UNAVAILABLE_SEVERITY_CELL,
                        &mut widest_unavailable,
                    )
                } else {
                    (grid::WIDEST_SEVERITY_CELL, &mut widest)
                };
                assert!(
                    cell.chars().count() <= bound.chars().count(),
                    "{cell} is wider than the cell the column is measured against",
                );
                if cell.chars().count() > longest.chars().count() {
                    *longest = cell;
                }
            }
        }
    }
    assert_eq!(
        widest.chars().count(),
        grid::WIDEST_SEVERITY_CELL.chars().count(),
        "the measured cell must be the longest one the page can print",
    );
    assert_eq!(
        widest_unavailable.chars().count(),
        grid::WIDEST_UNAVAILABLE_SEVERITY_CELL.chars().count(),
        "and the wider one the longest a finding that cannot be evaluated prints",
    );
}

/// A frame whose listing and reading position match the last one reuses the
/// arrangement built for it, and a change to either rebuilds it.
#[test]
fn the_arrangement_is_rebuilt_only_when_what_it_was_built_from_changes() {
    let mut state = state_with_findings(5);
    state.problems_view.sort = Some(ProblemColumn::Message);
    let ctx = egui::Context::default();
    let arranged = |state: &AppState| {
        let listed = listing(state);
        arrange::arrange(&ctx, &listed, &state.problems_view, None).0
    };

    let first = arranged(&state);
    assert!(
        Arc::ptr_eq(&first, &arranged(&state)),
        "an unchanged frame reuses the arrangement",
    );

    state.problems_view.explain = true;
    assert!(
        Arc::ptr_eq(&first, &arranged(&state)),
        "the detail region's own state arranges nothing",
    );

    state.problems_view.descending = true;
    let reversed = arranged(&state);
    assert!(!Arc::ptr_eq(&first, &reversed), "a new order rebuilds it");

    let extra = record(&state, "T-99", DiagnosticLevel::Error, "finding 99");
    let mut records: Vec<DiagnosticRecord> = state
        .diagnostics
        .batches()
        .flat_map(|batch| batch.records.iter().cloned())
        .collect();
    records.push(extra);
    publish(&mut state, records);
    let republished = arranged(&state);
    assert!(
        !Arc::ptr_eq(&reversed, &republished),
        "a republished list rebuilds it",
    );
    assert_eq!(republished.listed, 6);
}

/// A row that changes in place — same identity, same number of rows — still
/// rebuilds the arrangement when what changed is something the arrangement
/// reads: the waiver, the severity and the standing always, and the message
/// and subject where the reading position reads them. A count of rows would
/// see none of these.
#[test]
fn the_arrangement_is_rebuilt_when_a_row_changes_in_place() {
    let ctx = egui::Context::default();
    let arranged = |state: &AppState| {
        let listed = listing(state);
        arrange::arrange(&ctx, &listed, &state.problems_view, None).0
    };
    let republish = |state: &mut AppState, change: &dyn Fn(&mut DiagnosticRecord)| {
        let mut records: Vec<DiagnosticRecord> = state
            .diagnostics
            .batches()
            .flat_map(|batch| batch.records.iter().cloned())
            .collect();
        change(&mut records[0]);
        let identities = reported(state);
        publish(state, records);
        assert_eq!(
            reported(state),
            identities,
            "precondition: the same rows, under the same identities",
        );
    };

    let mut state = state_with_findings(5);
    let built = arranged(&state);
    republish(&mut state, &|_| {});
    assert!(
        Arc::ptr_eq(&built, &arranged(&state)),
        "the same rows republished reuse the arrangement",
    );

    republish(&mut state, &|record| {
        record.canonical.suppression = Some(waiver("T-00"));
    });
    let waived = arranged(&state);
    assert!(!Arc::ptr_eq(&built, &waived), "a new waiver rebuilds it");

    republish(&mut state, &|record| record.level = DiagnosticLevel::Error);
    let raised = arranged(&state);
    assert!(!Arc::ptr_eq(&waived, &raised), "a new severity rebuilds it");

    state.problems_view.text = "finding".to_owned();
    let filtered = arranged(&state);
    republish(&mut state, &|record| {
        record.message = "finding 00, restated".to_owned();
    });
    assert!(
        !Arc::ptr_eq(&filtered, &arranged(&state)),
        "under a text filter, a new message rebuilds it",
    );

    state.problems_view.text.clear();
    state.problems_view.sort = Some(ProblemColumn::Subject);
    let by_subject = arranged(&state);
    republish(&mut state, &|record| {
        record.subject.object = "Node 'n9'".to_owned();
    });
    assert!(
        !Arc::ptr_eq(&by_subject, &arranged(&state)),
        "ordered by subject, a new subject rebuilds it",
    );

    let mut design = state_with_design_findings();
    let current = arranged(&design);
    let rows = listing(&design).len();
    design.workspace.content.connectivity.policy.width_mismatch =
        crate::state::BundleWidthMismatchPolicy::ExplicitSliceOrExtend;
    assert_eq!(listing(&design).len(), rows, "precondition: no row left");
    assert!(
        !Arc::ptr_eq(&current, &arranged(&design)),
        "a batch gone stale rebuilds it",
    );
}
