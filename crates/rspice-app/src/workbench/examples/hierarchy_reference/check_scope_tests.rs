//! What Check Cellview and Check Design each state, and how the two compose
//! on the one list: a finding is listed once, by the newest check that looked
//! at the cellview it is drawn in.

use crate::state::{Cell, CellViewRef, ComponentType, Library, Point, View, ViewType};
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::diagnostic_store::listing;
use crate::workbench::examples::hierarchy_reference::{self, USER_LIBRARY};

const UNUSED: &str = "DRC-UNUSED-CELLVIEW";

/// A design-rule row as the page lists it: its rule, the cellview it names,
/// and what it is about.
fn rows(state: &AppState) -> Vec<(String, String, String)> {
    let listed = listing::listing(state);
    (0..listed.len())
        .filter_map(|index| listed.row(index))
        .filter(|row| row.producer == "rspice.design.rules")
        .map(|row| {
            (
                row.code.to_owned(),
                row.location.clone(),
                row.subject.clone(),
            )
        })
        .collect()
}

/// Rows about objects inside an occurrence of a placed cellview.
fn occurrence_rows(state: &AppState) -> usize {
    rows(state)
        .iter()
        .filter(|(_, _, object)| object.contains('\u{00b7}'))
        .count()
}

/// The reference opened on `amp`, with a lone resistor drawn in it — which
/// every occurrence of `amp` then reports — a spare schematic in the
/// project's own library that nothing places, and one in a read-only
/// reference library that nothing places either.
struct Fixture {
    state: AppState,
    top: CellViewRef,
    amp: CellViewRef,
    spare: CellViewRef,
    borrowed: CellViewRef,
}

fn fixture() -> Fixture {
    let reference = hierarchy_reference::build();
    let mut state = reference.state;
    state.project_lifecycle.authority.open_session();

    let spare = CellViewRef::new(USER_LIBRARY, "spare", "schematic");
    let mut cell = Cell::new(&spare.cell);
    cell.add_view(View::new(&spare.view, ViewType::Schematic));
    state
        .library_manager
        .edit_library(USER_LIBRARY)
        .expect("the project library")
        .add_cell(cell);

    let borrowed = CellViewRef::new("reference", "unplaced", "schematic");
    let mut cell = Cell::new(&borrowed.cell);
    cell.add_view(View::new(&borrowed.view, ViewType::Schematic));
    let mut library = Library::new(&borrowed.library);
    library.read_only = true;
    library.add_cell(cell);
    state.library_manager.add_library(library);

    state.open_workspace_view(reference.amp_schematic.clone());
    state
        .schematic
        .add_component(ComponentType::Resistor, Point::new(4000, 4000));
    Fixture {
        state,
        top: reference.top,
        amp: reference.amp_schematic,
        spare,
        borrowed,
    }
}

#[test]
fn check_cellview_states_the_open_cellview_alone() {
    let mut fixture = fixture();

    let result = fixture
        .state
        .check_active_cellview()
        .expect("the open cellview checks");

    assert!(
        !result.violations().is_empty(),
        "the lone resistor is reported"
    );
    let rows = rows(&fixture.state);
    assert!(
        rows.iter()
            .all(|(_, cell_view, _)| *cell_view == fixture.amp.display_path()),
        "only the open cellview's own findings: {rows:?}"
    );
    assert_eq!(occurrence_rows(&fixture.state), 0);
    assert!(rows.iter().all(|(code, _, _)| code != UNUSED));
}

#[test]
fn check_design_states_every_occurrence_and_every_unused_project_schematic() {
    let mut fixture = fixture();

    fixture.state.check_design().expect("the design checks");

    let rows = rows(&fixture.state);
    assert!(
        occurrence_rows(&fixture.state) >= hierarchy_reference::AMP_INSTANCES.len(),
        "the resistor is reported at each occurrence of amp: {rows:?}"
    );
    let unused = rows
        .iter()
        .filter(|(code, _, _)| code == UNUSED)
        .map(|(_, cell_view, _)| cell_view.clone())
        .collect::<Vec<_>>();
    assert!(unused.contains(&fixture.spare.display_path()), "{unused:?}");
    assert!(
        !unused.contains(&fixture.borrowed.display_path()),
        "a reference library is never unused: {unused:?}"
    );
    for reached in [&fixture.top, &fixture.amp] {
        assert!(!unused.contains(&reached.display_path()), "{unused:?}");
    }
}

/// The newest check that looked at a cellview states its findings: a check
/// of `amp` after a check of the design takes `amp`'s findings over, and a
/// check of the design after that takes them back. Neither leaves the other's
/// statement of the same findings on the list.
#[test]
fn the_newest_check_of_a_cellview_states_its_findings_once() {
    let mut fixture = fixture();

    fixture.state.check_design().expect("the design checks");
    let below = occurrence_rows(&fixture.state);
    assert!(below > 0);

    let own = fixture
        .state
        .check_active_cellview()
        .expect("the open cellview checks")
        .violations()
        .len();
    let rows = rows(&fixture.state);
    assert_eq!(
        occurrence_rows(&fixture.state),
        0,
        "the design check's statements inside amp are taken over: {rows:?}"
    );
    assert_eq!(
        rows.iter()
            .filter(
                |(code, cell_view, _)| *cell_view == fixture.amp.display_path() && code != UNUSED
            )
            .count(),
        own,
    );
    assert!(
        rows.iter().any(|(code, _, _)| code == UNUSED),
        "a cellview check does not decide what is used, so it leaves that standing"
    );

    fixture
        .state
        .check_design()
        .expect("the design checks again");
    assert_eq!(occurrence_rows(&fixture.state), below);
    assert!(
        rows_in(&fixture.state, &fixture.amp).is_empty(),
        "the cellview check's own rows are taken back"
    );
}

/// Clearing a cellview's checks withdraws what they stated and gives back
/// what they had taken over: Check Design still describes the design, so its
/// statements inside `amp` are listed again rather than leaving `amp`'s
/// findings off the list while the run gate counts them.
#[test]
fn clearing_a_cellview_check_gives_back_what_it_took_over() {
    let mut fixture = fixture();
    fixture.state.check_design().expect("the design checks");
    let below = occurrence_rows(&fixture.state);
    assert!(below > 0);

    fixture
        .state
        .check_active_cellview()
        .expect("the open cellview checks");
    assert_eq!(occurrence_rows(&fixture.state), 0);

    fixture.state.clear_active_design_check();
    assert_eq!(
        occurrence_rows(&fixture.state),
        below,
        "Check Design's statements inside amp are listed again: {:?}",
        rows(&fixture.state)
    );
}

const SOURCE_LOOP: &str = "DRC-SOURCE-TO-SOURCE";

/// A source drawn in the open `amp` from the global supply to ground. Each
/// occurrence of `amp` places it again, so the copies close a loop through
/// the global that only a walk of the whole design can see: `amp` alone
/// holds one source, and one source closes nothing.
pub(in crate::workbench) fn draw_supply_source(state: &mut AppState) {
    let schematic = &mut state.schematic;
    let id = schematic.add_component(ComponentType::VoltageSource, Point::new(3000, 3000));
    let terminals = schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == id)
        .expect("the source is placed")
        .terminal_positions();
    for (pin, at) in terminals {
        let (end, net) = match pin {
            "+" => (
                Point::new(at.x, at.y - 40),
                hierarchy_reference::GLOBAL_SUPPLY_NET,
            ),
            _ => (Point::new(at.x, at.y + 40), "gnd"),
        };
        schematic
            .add_wire(vec![at, end])
            .expect("a two-point wire commits");
        schematic.add_net_label(end, net.to_owned());
    }
}

/// The source-loop rows the page lists, unwaived and current.
pub(in crate::workbench) fn source_loop_errors(state: &AppState) -> usize {
    let listed = listing::listing(state);
    (0..listed.len())
        .filter_map(|index| listed.row(index))
        .filter(|row| row.code == SOURCE_LOOP && row.current() && !row.waived())
        .count()
}

/// A narrower check takes over only what it computes again. Check Cellview
/// of `amp` walks nothing above it, so it never sees the loop the copies of
/// `amp` close through the global: Check Design's statement of that loop
/// stands, and the page lists the error the run gate and Check and Save
/// block on.
#[test]
fn a_cellview_check_leaves_standing_what_only_the_design_check_computes() {
    let mut fixture = fixture();
    draw_supply_source(&mut fixture.state);

    fixture.state.check_design().expect("the design checks");
    let loops = source_loop_errors(&fixture.state);
    assert!(
        loops > 0,
        "the copies of amp close a loop through the global"
    );

    let own = fixture
        .state
        .check_active_cellview()
        .expect("the open cellview checks");
    assert!(
        own.violations()
            .iter()
            .all(|finding| finding.violation_type.rule_code() != SOURCE_LOOP),
        "amp alone closes no loop"
    );
    assert_eq!(
        source_loop_errors(&fixture.state),
        loops,
        "the loop is still listed: {:?}",
        rows(&fixture.state)
    );

    let gate = fixture
        .state
        .current_design_check(
            &fixture.top,
            crate::workbench::app_state::design_checks::CheckScope::Design,
        )
        .expect("Check Design still describes the design");
    assert!(gate.has_errors(), "the run gate refuses on the loop");
}

const GROUND_NOT_ZERO: &str = "DRC-GROUND-NOT-NODE-ZERO";

/// A resistor drawn in the open `amp` whose leg is labelled `vx` and then
/// `gnd` at the same point: the name that claimed the net first keeps it, so
/// the deck writes `vx` where the drawing says ground.
fn draw_lost_ground_label(state: &mut AppState) {
    let schematic = &mut state.schematic;
    let id = schematic.add_component(ComponentType::Resistor, Point::new(3000, 3600));
    let leg = schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == id)
        .expect("the resistor is placed")
        .terminal_positions()[0]
        .1;
    let end = Point::new(leg.x - 40, leg.y);
    schematic
        .add_wire(vec![leg, end])
        .expect("a two-point wire commits");
    schematic.add_net_label(end, "vx".to_owned());
    schematic.add_net_label(end, "gnd".to_owned());
}

/// A ground label the deck does not write as node 0 is a fact of the
/// cellview that draws it, whoever checks it. Waived where Check Design
/// found it inside `amp`, it is still found, and still waived, by a check of
/// `amp` alone — which therefore never reports the waiver as covering
/// nothing, or offers to delete it.
#[test]
fn a_cell_states_its_own_lost_ground_label_and_keeps_its_waiver() {
    let mut fixture = fixture();
    draw_lost_ground_label(&mut fixture.state);

    fixture.state.check_design().expect("the design checks");
    let target = {
        let listed = listing::listing(&fixture.state);
        (0..listed.len())
            .filter_map(|index| listed.row(index))
            .find(|row| row.code == GROUND_NOT_ZERO)
            .and_then(|row| match row.waiver_action {
                Some(crate::workbench::app_state::waivers::WaiverAction::Waive(target)) => {
                    Some(target.clone())
                }
                _ => None,
            })
            .expect("Check Design finds the lost label inside amp, and it may be waived")
    };
    assert_eq!(target.cell_view, fixture.amp, "the waiver is amp's own");
    fixture
        .state
        .waive_finding(&target, "vx is the local return on purpose")
        .expect("the finding is waived");

    fixture
        .state
        .check_active_cellview()
        .expect("the open cellview checks");
    let listed = listing::listing(&fixture.state);
    let unmatched = (0..listed.len())
        .filter_map(|index| listed.row(index))
        .filter(|row| row.code == "WAIVER-UNMATCHED")
        .map(|row| row.message.to_owned())
        .collect::<Vec<_>>();
    assert!(
        unmatched.is_empty(),
        "the waiver still covers its finding: {unmatched:?}"
    );
    let lost = (0..listed.len())
        .filter_map(|index| listed.row(index))
        .filter(|row| row.code == GROUND_NOT_ZERO)
        .map(|row| row.waived())
        .collect::<Vec<_>>();
    assert!(
        !lost.is_empty() && lost.iter().all(|waived| *waived),
        "amp's own check states the lost label, waived: {lost:?}"
    );
}

/// An unused cellview is a note about the project's libraries, never a
/// reason to refuse a run. A stored deck that promotes it to an error is
/// checked at a warning: the page lists the spare cellview as a warning, and
/// Check Design's receipt, which preflight blocks on, holds no more errors
/// than under the shipped deck. The run gate never computes the rule, so
/// the three agree.
#[test]
fn a_deck_cannot_make_an_unused_cellview_block_a_run() {
    use crate::workbench::app_state::design_checks::CheckScope;
    use crate::workbench::app_state::diagnostic_store::DiagnosticLevel;

    let errors = |fixture: &mut Fixture| {
        fixture.state.check_design().expect("the design checks");
        let summary = fixture
            .state
            .current_design_check(&fixture.top, CheckScope::Design)
            .expect("Check Design describes the design")
            .summary();
        summary.errors + summary.critical
    };
    let shipped = errors(&mut fixture());

    let mut promoted = fixture();
    promoted.state.workspace.content.design_rules = serde_json::from_str(
        r#"{"schema_version":1,"severities":{"DRC-UNUSED-CELLVIEW":"Error"}}"#,
    )
    .expect("a stored deck always reads");
    assert_eq!(
        errors(&mut promoted),
        shipped,
        "preflight blocks on no more"
    );

    let listed = listing::listing(&promoted.state);
    let unused = (0..listed.len())
        .filter_map(|index| listed.row(index))
        .filter(|row| row.code == UNUSED)
        .map(|row| (row.subject.clone(), row.level))
        .collect::<Vec<_>>();
    assert_eq!(
        unused,
        [(promoted.spare.display_path(), DiagnosticLevel::Warning)],
        "{:?}",
        rows(&promoted.state)
    );
}

/// A waiver is an edit of the document that stores it, so it is changed only
/// where the workbench edits that document: the open view, or a schematic
/// the project holds open. Elsewhere the page and the edit give one reason —
/// open the cellview first, or its library is borrowed read-only.
#[test]
fn a_waiver_is_changed_only_where_its_document_is_edited() {
    use crate::workbench::app_state::waivers::WaiverTarget;

    let mut fixture = fixture();
    let symbol = CellViewRef::new(USER_LIBRARY, "amp", "symbol");
    let state = &fixture.state;
    assert_eq!(
        state.waiver_edit_refusal(&fixture.amp),
        None,
        "the open view"
    );
    assert_eq!(
        state.waiver_edit_refusal(&fixture.top),
        None,
        "a schematic the project holds"
    );
    assert_eq!(
        state.waiver_edit_refusal(&symbol),
        Some(format!(
            "Open {} to change its waivers",
            symbol.display_path()
        ))
    );
    assert_eq!(
        state.waiver_edit_refusal(&fixture.borrowed),
        Some(format!("{} is read-only", fixture.borrowed.library))
    );

    for cell_view in [symbol, fixture.borrowed.clone()] {
        let reason = fixture
            .state
            .waiver_edit_refusal(&cell_view)
            .expect("refused above");
        let target = WaiverTarget {
            cell_view,
            rule: std::sync::Arc::from("DRC-FLOATING-NODE"),
            object: "component:1".to_owned(),
        };
        assert_eq!(
            fixture.state.waive_finding(&target, "a probe point"),
            Err(format!("{reason}.")),
            "the edit refuses for the reason the page gives"
        );
    }
}

/// The objects of the rows naming `cell_view`.
fn rows_in(state: &AppState, cell_view: &CellViewRef) -> Vec<String> {
    rows(state)
        .into_iter()
        .filter(|(_, named, _)| *named == cell_view.display_path())
        .map(|(_, _, object)| object)
        .collect()
}

/// Leaving a view while its edit settles checks the edit on the way out,
/// from the drawing left behind in the workspace, which is still part of the
/// configured design: the check runs, and nothing says it failed.
#[test]
fn an_edit_settling_as_its_view_is_left_is_checked_without_a_warning() {
    use crate::workbench::app_state::diagnostic_store::DiagnosticInputs;

    let reference = hierarchy_reference::build();
    let mut state = reference.state;
    state.project_lifecycle.authority.open_session();
    let amp = reference.amp_schematic;
    state.open_workspace_view(amp.clone());
    let start = crate::time_compat::Instant::now();
    let _ = state.refresh_live_checks(start);
    let live = DiagnosticInputs::LiveCheck {
        subject: amp.clone(),
    };
    let stated = |state: &AppState| {
        state
            .diagnostics
            .batch(&live)
            .map_or(0, |batch| batch.records.len())
    };
    let before = stated(&state);
    state
        .schematic
        .add_component(ComponentType::Resistor, Point::new(4000, 4000));
    assert!(
        state.refresh_live_checks(start).is_some(),
        "the edit settles"
    );

    state.open_workspace_view(reference.top);
    assert_eq!(state.refresh_live_checks(start), None);
    assert!(
        stated(&state) > before,
        "the resistor's open legs are stated for amp on the way out"
    );
    assert!(
        !state
            .log_buffer
            .entries()
            .any(|entry| entry.message.starts_with("Live checks could not run")),
        "nothing says the check failed"
    );
}
