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

#[test]
fn zz_measure_live_check() {
    let reference = hierarchy_reference::build();
    let mut state = reference.state;
    state.project_lifecycle.authority.open_session();
    for subject in [
        reference.top.clone(),
        reference.amp_schematic.clone(),
        reference.user_filter.clone(),
    ] {
        let _ = state.run_live_check(subject.clone());
        let mut best = std::time::Duration::MAX;
        for _ in 0..20 {
            let start = std::time::Instant::now();
            let _ = state.run_live_check(subject.clone()).expect("live");
            best = best.min(start.elapsed());
        }
        let start = std::time::Instant::now();
        let _ = state.check_design().expect("design");
        let design = start.elapsed();
        let components = state.workspace.content.schematic_buffers[&subject.key()]
            .document()
            .components
            .len();
        eprintln!(
            "LIVE-MEASURE {} components={components} live_best={best:?} check_design={design:?}",
            subject.display_path()
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
