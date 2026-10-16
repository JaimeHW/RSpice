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
use crate::workbench::app_state::diagnostic_store::{
    DiagnosticInputs, DiagnosticRecord, DiagnosticSubject,
};
use crate::workbench::app_state::problems_view::{ProblemColumn, ProblemGrouping};
use crate::workbench::documents::netlist_document::diagnostics::DiagnosticFix;
use crate::workbench::documents::netlist_document::{Diagnostic, NetlistDiagnosticCollection};
use rspice_design::drc::{DrcLocation, DrcResult, DrcViolation, DrcViolationType};

const WIDTH: f32 = 900.0;
const HEIGHT: f32 = 420.0;

/// One finding each from two rules of the live drawing: an error on an object
/// the drawing carries, and a warning on a wire.
fn state_with_design_findings() -> AppState {
    let mut state = AppState::default();
    state.project_lifecycle.authority.open_session();
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
    state
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

/// One node of the access tree, in the three terms these tests read.
///
/// egui puts a label widget's text in the node's *value* and a control's in
/// its name, and reports a selection as a toggle rather than as a selection,
/// so a test that read only `label()` would see nothing for half the rows.
struct Announced {
    role: Role,
    label: Option<String>,
    value: Option<String>,
    toggled: Option<egui::accesskit::Toggled>,
}

impl Announced {
    fn says(&self) -> Option<&str> {
        self.label.as_deref().or(self.value.as_deref())
    }

    fn selected(&self) -> bool {
        matches!(self.toggled, Some(egui::accesskit::Toggled::True))
    }
}

/// Everything one frame put on the screen, in the forms these tests measure.
struct Render {
    texts: Vec<(String, egui::Pos2)>,
    /// Filled rectangles, with the colour they were filled in.
    fills: Vec<(egui::Rect, egui::Color32)>,
    /// Three-point paths: the painted direction and chevron marks.
    triangles: Vec<Vec<egui::Pos2>>,
    nodes: Vec<Announced>,
}

impl Render {
    fn text(&self, needle: &str) -> Option<egui::Pos2> {
        self.texts
            .iter()
            .find(|(text, _)| text.contains(needle))
            .map(|(_, pos)| *pos)
    }

    fn exact(&self, text: &str) -> Option<egui::Pos2> {
        self.texts
            .iter()
            .find(|(painted, _)| painted == text)
            .map(|(_, pos)| *pos)
    }

    fn has(&self, needle: &str) -> bool {
        self.text(needle).is_some()
    }

    /// Every text painted on one row of the grid, left to right.
    ///
    /// Two points of tolerance, not none: the subject column is printed in a
    /// heavier weight than its neighbours and a face's line height is its
    /// own. Rows are a row height apart, so it cannot reach the next one.
    fn row_at(&self, y: f32) -> Vec<(String, egui::Pos2)> {
        let mut row: Vec<(String, egui::Pos2)> = self
            .texts
            .iter()
            .filter(|(_, pos)| (pos.y - y).abs() < 2.0)
            .cloned()
            .collect();
        row.sort_by(|left, right| left.1.x.total_cmp(&right.1.x));
        row
    }
}

/// One egui context, held across frames.
///
/// Focus, scroll position and the row identities the keyboard reads all live
/// in the context, so a test that clicks a row and then keys it has to send
/// both through the same one. A fresh context per frame would have no focus
/// to key against, and the keyboard tests would pass by doing nothing.
struct Harness {
    ctx: egui::Context,
}

impl Harness {
    fn new() -> Self {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        Self { ctx }
    }

    fn frame(&self, state: &mut AppState, input: egui::RawInput) -> Render {
        let output = self.ctx.run_ui(input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| problems(ui, state));
        });
        collect(output)
    }

    /// Draw a frame with nothing happening, which is what settles the sizing
    /// and leaves the rows in memory to be keyed.
    fn settle(&self, state: &mut AppState) {
        let _ = self.frame(state, sized(egui::RawInput::default()));
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
            egui::epaint::Shape::Text(text) => {
                render.texts.push((text.galley.job.text.clone(), text.pos))
            }
            egui::epaint::Shape::Rect(rect) => {
                render.fills.push((rect.rect, rect.fill));
            }
            egui::epaint::Shape::Path(path) if path.points.len() == 3 => {
                render.triangles.push(path.points.clone());
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
            })
            .collect();
    }
    render
}

fn sized(input: egui::RawInput) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(WIDTH, HEIGHT),
        )),
        ..input
    }
}

fn keyed(key: egui::Key) -> egui::RawInput {
    sized(egui::RawInput {
        events: vec![egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
        ..Default::default()
    })
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

/// Every cell sits under its own heading, and no cell starts before the column
/// in front of it has had its gap.
///
/// This is the assertion the first version of this page could not have passed:
/// it fixed two columns at 58 px and 62 px and painted the rest over them.
#[test]
fn every_cell_is_painted_under_its_own_heading() {
    let mut state = state_with_design_findings();
    let render = render(&mut state, sized(egui::RawInput::default()));

    let headings: Vec<egui::Pos2> = ProblemColumn::ALL
        .iter()
        .map(|column| {
            render
                .exact(column.label())
                .unwrap_or_else(|| panic!("the {} heading is painted", column.label()))
        })
        .collect();
    for pair in headings.windows(2) {
        assert!(
            pair[1].x - pair[0].x >= CONSOLE_COLUMN_GAP,
            "headings at {} and {} are closer than one gap",
            pair[0].x,
            pair[1].x,
        );
    }
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
    for (cell, heading) in row.iter().zip(&headings) {
        assert!(
            (cell.1.x - heading.x).abs() < 0.5,
            "the cell {:?} at {} is not under its heading at {}",
            cell.0,
            cell.1.x,
            heading.x,
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
        row[0].1.x >= CONSOLE_MARK_GUTTER,
        "the severity cell clears the mark gutter",
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
/// last. The fixture's error and warning are reported in the other order.
#[test]
fn ordering_by_severity_lists_the_worst_finding_first() {
    let mut state = state_with_design_findings();
    state.problems_view.sort = Some(ProblemColumn::Severity);

    let worst_first = render(&mut state, sized(egui::RawInput::default()));
    let error = worst_first
        .text("R1 has no resistance")
        .expect("the error is listed");
    let warning = worst_first
        .text("Wire ends open")
        .expect("the warning is listed");
    assert!(
        error.y < warning.y,
        "the error at {} must be listed above the warning at {}",
        error.y,
        warning.y,
    );

    state.problems_view.descending = true;
    let least_first = render(&mut state, sized(egui::RawInput::default()));
    let error = least_first.text("R1 has no resistance").expect("listed");
    let warning = least_first.text("Wire ends open").expect("listed");
    assert!(warning.y < error.y, "reversing the order reverses the rows",);
}

/// A count beside a toggle describes what that toggle would list under the
/// other filters, not what the whole list holds.
#[test]
fn a_severity_count_describes_what_its_toggle_would_list() {
    use super::arrange::Arrangement;
    use crate::workbench::app_state::diagnostic_store::DiagnosticLevel;
    use crate::workbench::app_state::diagnostic_store::listing::listing;

    let mut state = state_with_design_findings();
    netlist_finding_with_a_fix(&mut state);

    let view = state.problems_view.clone();
    let listed = listing(&state);
    let arrangement = Arrangement::build(&listed, &view, None);
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
    let arrangement = Arrangement::build(&listed, &view, None);
    assert_eq!(arrangement.tally.severity(DiagnosticLevel::Error), 1);
    assert_eq!(arrangement.listed, 2);
    assert_eq!(
        arrangement.tally.producers.len(),
        2,
        "the producer control still offers every producer present",
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

/// Grouping prints one header per group, with the group's own counts, and the
/// header collapses it.
#[test]
fn grouping_prints_a_header_that_carries_its_counts_and_collapses() {
    let mut state = state_with_design_findings();
    netlist_finding_with_a_fix(&mut state);
    state.problems_view.grouping = ProblemGrouping::Producer;

    let grouped = render(&mut state, sized(egui::RawInput::default()));
    assert!(grouped.has("rspice.design.rules"));
    assert!(
        grouped.has("1 error · 1 advisory"),
        "the group header states what it holds: {:?}",
        grouped.texts,
    );
    assert!(grouped.has("R1 has no resistance"));

    state.problems_view.collapse("rspice.design.rules", true);
    let collapsed = render(&mut state, sized(egui::RawInput::default()));
    assert!(
        collapsed.has("1 error · 1 advisory"),
        "a collapsed group still states what it holds",
    );
    assert!(
        !collapsed.has("R1 has no resistance"),
        "a collapsed group lists none of its rows",
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
    // The rows, told apart from the detail region's own lines by the selection
    // state only a row and a group header carry, and from the strip's toggles
    // by the message only a row prints.
    let rows: Vec<&Announced> = render
        .nodes
        .iter()
        .filter(|node| {
            node.toggled.is_some()
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

/// Clicking a row makes it current, Down moves it, Enter follows it and
/// Escape clears it — all of them moving the one shared selection.
#[test]
fn the_keyboard_moves_follows_and_clears_the_current_finding() {
    let mut state = state_with_design_findings();
    let listed = crate::workbench::app_state::diagnostic_store::listing::listing(&state);
    let order: Vec<uuid::Uuid> = (0..listed.len())
        .filter_map(|index| listed.facet(index))
        .map(|facet| facet.id)
        .collect();
    drop(listed);
    assert_eq!(order.len(), 2, "the fixture publishes two findings");

    let harness = Harness::new();
    harness.settle(&mut state);
    let first = harness.frame(&mut state, sized(egui::RawInput::default()));
    let row = first
        .text("R1 has no resistance")
        .expect("the first finding is listed");

    let _ = harness.frame(&mut state, clicked(egui::pos2(row.x + 4.0, row.y + 4.0)));
    assert_eq!(
        state.diagnostics.current_finding(),
        Some(order[0]),
        "clicking the first row makes that finding current",
    );
    // Clicking a row that can be followed also follows it, which is the
    // behaviour this page has always had.
    assert!(state.diagnostics.take_pending_route().is_some());

    harness.settle(&mut state);
    let _ = harness.frame(&mut state, keyed(egui::Key::ArrowDown));
    assert_eq!(
        state.diagnostics.current_finding(),
        Some(order[1]),
        "Down moves to the next listed finding",
    );

    harness.settle(&mut state);
    let _ = harness.frame(&mut state, keyed(egui::Key::ArrowUp));
    assert_eq!(
        state.diagnostics.current_finding(),
        Some(order[0]),
        "Up moves it back",
    );

    harness.settle(&mut state);
    let _ = harness.frame(&mut state, keyed(egui::Key::Enter));
    assert!(
        state.diagnostics.take_pending_route().is_some(),
        "Enter follows the current finding's route",
    );

    harness.settle(&mut state);
    let _ = harness.frame(&mut state, keyed(egui::Key::Escape));
    assert_eq!(
        state.diagnostics.current_finding(),
        None,
        "Escape clears the current finding",
    );
}

/// A row with nowhere to go is still a row: it can be read and selected, and
/// it does not announce itself as something that can be pressed.
#[test]
fn a_finding_with_nowhere_to_go_announces_as_text() {
    let mut state = AppState::default();
    state.project_lifecycle.authority.open_session();
    let record = DiagnosticRecord::new(
        rspice_app_types::diagnostics::CanonicalDiagnosticMetadata::current(
            "rspice.test",
            "TEST-NOWHERE",
            "doc",
            None,
            0,
            "validation",
            "a finding with nowhere to go",
        ),
        crate::workbench::app_state::diagnostic_store::DiagnosticLevel::Error,
        DiagnosticSubject::new(state.workspace.content.project.id(), "Node 'n1'"),
        "a finding with nowhere to go",
    );
    state
        .publish_diagnostics(DiagnosticInputs::Dispatch, vec![record])
        .expect("the dispatch digest is available");

    let render = render(&mut state, sized(egui::RawInput::default()));
    let row = render
        .nodes
        .iter()
        .find(|node| {
            node.says()
                .is_some_and(|says| says.contains("a finding with nowhere to go"))
        })
        .expect("the finding is reachable");
    assert_eq!(
        row.role,
        Role::Label,
        "a finding with no route must not announce as a control",
    );
    assert!(
        !row.selected(),
        "it still says whether it is the current row",
    );
    assert!(
        row.toggled.is_some(),
        "and it says so rather than leaving the state unstated",
    );
}

/// Explain states the rule's own words, and is drawn only where the rule has
/// words of its own.
#[test]
fn explain_is_offered_only_where_the_rule_has_words() {
    let mut state = state_with_design_findings();
    let listing = crate::workbench::app_state::diagnostic_store::listing::listing(&state);
    let finding = listing.facet(0).expect("the first finding is listed").id;
    drop(listing);
    state.diagnostics.set_current_finding(Some(finding));

    let unopened = render(&mut state, sized(egui::RawInput::default()));
    assert!(
        unopened.has("Explain"),
        "a rule finding offers its rule: {:?}",
        unopened.texts,
    );
    state.problems_view.explain = true;
    let explained = render(&mut state, sized(egui::RawInput::default()));
    assert!(
        explained.has("Required component parameter is missing"),
        "Explain states the rule in the rule's own words: {:?}",
        explained.texts,
    );

    // A failed run is not an instance of a rule, so there is nothing to state
    // and no control that says there is.
    let mut run = AppState::default();
    run.project_lifecycle.authority.open_session();
    run.publish_run_failure("Transient", "Analysis failed: no convergence", None, 1);
    let listing = crate::workbench::app_state::diagnostic_store::listing::listing(&run);
    let finding = listing.facet(0).expect("the failure is listed").id;
    drop(listing);
    run.diagnostics.set_current_finding(Some(finding));
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
    let mut state = AppState::default();
    state.project_lifecycle.authority.open_session();
    netlist_finding_with_a_fix(&mut state);
    let listing = crate::workbench::app_state::diagnostic_store::listing::listing(&state);
    let finding = listing.facet(0).expect("the netlist finding is listed").id;
    drop(listing);
    state.diagnostics.set_current_finding(Some(finding));
    let offered = render(&mut state, sized(egui::RawInput::default()));
    assert!(
        offered.has("Replace with nch"),
        "the fix is offered in its own words: {:?}",
        offered.texts,
    );

    let mut rules = state_with_design_findings();
    let listing = crate::workbench::app_state::diagnostic_store::listing::listing(&rules);
    let finding = listing.facet(0).expect("listed").id;
    drop(listing);
    rules.diagnostics.set_current_finding(Some(finding));
    let unoffered = render(&mut rules, sized(egui::RawInput::default()));
    assert!(
        !unoffered.has("Replace with"),
        "a producer that offered no edits gets no fix control",
    );
}

/// The detail region says where the finding's batch stands, and what moved
/// when it no longer describes the design.
#[test]
fn the_detail_region_states_where_the_batch_stands() {
    let mut state = state_with_design_findings();
    let listing = crate::workbench::app_state::diagnostic_store::listing::listing(&state);
    let finding = listing.facet(0).expect("listed").id;
    drop(listing);
    state.diagnostics.set_current_finding(Some(finding));

    let current = render(&mut state, sized(egui::RawInput::default()));
    assert!(
        current.has("Current"),
        "a current finding says so: {:?}",
        current.texts,
    );

    // Move the inputs under the batch. The finding keeps its row and now says
    // what moved rather than only that it is stale.
    state.workspace.content.connectivity.policy.width_mismatch =
        crate::state::BundleWidthMismatchPolicy::ExplicitSliceOrExtend;
    let stale = render(&mut state, sized(egui::RawInput::default()));
    assert!(
        stale.has("Stale · the schematic working set behind"),
        "a stale finding says what moved: {:?}",
        stale.texts,
    );
    assert!(
        stale.has("Stale Error"),
        "and its row says so in the severity column",
    );
}

/// A place a finding also names is listed, and is offered as a jump only
/// where something can resolve it. A rule finding's related places are in the
/// drawing, which no document route addresses, so they are stated.
#[test]
fn a_related_place_is_stated_even_where_it_cannot_be_followed() {
    let mut state = AppState::default();
    state.project_lifecycle.authority.open_session();
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
        crate::workbench::app_state::diagnostic_store::DiagnosticLevel::Error,
        DiagnosticSubject::new(state.workspace.content.project.id(), "Net 'out'"),
        "two outputs drive this net",
    );
    state
        .publish_diagnostics(DiagnosticInputs::Dispatch, vec![record])
        .expect("the dispatch digest is available");
    let listing = crate::workbench::app_state::diagnostic_store::listing::listing(&state);
    let finding = listing.facet(0).expect("the finding is listed").id;
    drop(listing);
    state.diagnostics.set_current_finding(Some(finding));

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

/// The page is not a severity filter over the session log.
///
/// Before the canonical store it read the log buffer, so every refusal,
/// engine warning and system notice that had ever been printed stood here
/// as a permanent problem — about 715 refusal sites' worth.
#[test]
fn a_logged_failure_is_not_a_problem() {
    let mut state = AppState::default();
    state.project_lifecycle.authority.open_session();
    state.log_buffer.log(
        crate::diagnostics::LogSeverity::Error,
        crate::diagnostics::LogSource::Engine,
        "the engine printed a failure",
        None,
    );

    assert_eq!(active_problem_count(&state), ProblemCount::default());
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

/// Every severity cell the page can print fits the cell the column is
/// measured against.
#[test]
fn every_severity_cell_fits_the_widest_measured_cell() {
    use crate::workbench::app_state::diagnostic_store::DiagnosticLevel;
    use crate::workbench::app_state::diagnostic_store::listing::{DiagnosticRow, RowStanding};

    let mut widest = String::new();
    for level in DiagnosticLevel::ALL {
        for standing in [
            RowStanding::Current,
            RowStanding::Stale("inputs".to_owned()),
        ] {
            for waiver in [
                None,
                Some(rspice_app_types::diagnostics::DiagnosticSuppression {
                    rule: "T-1".to_owned(),
                    scope: "cell".to_owned(),
                    justification_required: false,
                }),
            ] {
                let cell = severity_cell(&DiagnosticRow {
                    id: uuid::Uuid::nil(),
                    code: "T-1",
                    producer: "rspice.test",
                    level,
                    standing: standing.clone(),
                    subject: String::new(),
                    location: String::new(),
                    message: "",
                    details: "",
                    rule: "",
                    related: &[],
                    quick_fixes: &[],
                    suppression: waiver.as_ref(),
                    route: None,
                });
                assert!(
                    cell.chars().count() <= grid::WIDEST_SEVERITY_CELL.chars().count(),
                    "{cell} is wider than the cell the column is measured against",
                );
                if cell.chars().count() > widest.chars().count() {
                    widest = cell;
                }
            }
        }
    }
    assert_eq!(
        widest.chars().count(),
        grid::WIDEST_SEVERITY_CELL.chars().count(),
        "the measured cell must be the longest one the page can print",
    );
}
