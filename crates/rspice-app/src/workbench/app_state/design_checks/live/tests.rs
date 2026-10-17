//! When live checking runs, what it states, and how it composes with the
//! explicit checks on the one list.

use super::*;
use crate::state::{ComponentType, Point};
use crate::workbench::app_state::diagnostic_store::listing;
use crate::workbench::app_state::waivers::WaiverAction;

fn open_state() -> AppState {
    let mut state = AppState::default();
    state.project_lifecycle.authority.open_session();
    state
}

/// The design-rule rows the page lists, as (identity, rule, object, current,
/// waived).
fn rows(state: &AppState) -> Vec<(uuid::Uuid, String, String, bool, bool)> {
    let listed = listing::listing(state);
    (0..listed.len())
        .filter_map(|index| listed.row(index))
        .filter(|row| row.producer == "rspice.design.rules")
        .map(|row| {
            (
                row.id,
                row.code.to_owned(),
                row.subject.clone(),
                row.current(),
                row.waived(),
            )
        })
        .collect()
}

/// The rows about the component named `name`.
fn rows_about(state: &AppState, name: &str) -> Vec<(uuid::Uuid, String, String, bool, bool)> {
    rows(state)
        .into_iter()
        .filter(|(_, _, object, _, _)| object.contains(&format!(" {name}.")))
        .collect()
}

/// Place a resistor nothing reaches, whose pins the live rules report, and
/// let the edit settle. Returns its reference designator.
fn place_open_resistor(state: &mut AppState, at: Instant, x: i32) -> String {
    let _ = state.refresh_live_checks(at);
    let id = state
        .schematic
        .add_component(ComponentType::Resistor, Point::new(x, 400));
    settle(state, at);
    state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == id)
        .map(|component| component.name.clone())
        .expect("the resistor is placed")
}

fn settle(state: &mut AppState, at: Instant) {
    let wait = state
        .refresh_live_checks(at)
        .expect("an edit waits for the drawing to settle");
    assert_eq!(state.refresh_live_checks(at + wait), None);
}

#[test]
fn an_edit_is_checked_once_it_settles_and_a_frame_without_one_checks_nothing() {
    let mut state = open_state();
    let start = Instant::now();
    assert_eq!(
        state.refresh_live_checks(start),
        None,
        "opening a drawing checks nothing"
    );

    state
        .schematic
        .add_component(ComponentType::Resistor, Point::new(400, 400));
    let wait = state
        .refresh_live_checks(start)
        .expect("the edit waits for the drawing to settle");
    assert!(rows(&state).is_empty(), "nothing is checked while it moves");
    assert_eq!(state.refresh_live_checks(start + wait), None);
    let found = rows(&state);
    assert!(
        !found.is_empty() && found.iter().all(|(_, _, _, current, _)| *current),
        "{found:?}"
    );
    assert!(found.iter().all(|(_, code, _, _, _)| {
        rspice_design::drc::LIVE_RULES
            .iter()
            .any(|rule| rule.rule_code() == code)
    }));

    let published = state.design_checks.published;
    for frame in 2..8 {
        assert_eq!(state.refresh_live_checks(start + wait * frame), None);
    }
    assert_eq!(
        state.design_checks.published, published,
        "a frame with no edit checks nothing"
    );
}

#[test]
fn a_finding_fixed_while_drawing_leaves_at_once_and_check_design_lists_it_nowhere() {
    let mut state = open_state();
    let start = Instant::now();
    let name = place_open_resistor(&mut state, start, 400);
    assert!(!rows_about(&state, &name).is_empty());

    let id = state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.name == name)
        .map(|component| component.id)
        .expect("the resistor is placed");
    state
        .schematic
        .session
        .editor
        .selection
        .select_only_component(id);
    assert!(state.schematic.delete_selection());
    settle(&mut state, start);
    assert!(rows_about(&state, &name).is_empty(), "{:?}", rows(&state));

    state.check_design().expect("the design checks");
    assert!(rows_about(&state, &name).is_empty(), "{:?}", rows(&state));
}

/// A live finding and a check of the cellview's finding of the same rule and
/// object are one finding under one identity, whichever stated it last.
#[test]
fn a_live_finding_and_the_cellview_check_of_it_are_one_row() {
    let mut state = open_state();
    let start = Instant::now();
    let name = place_open_resistor(&mut state, start, 400);
    let live = rows_about(&state, &name);
    let identities = |rows: &[(uuid::Uuid, String, String, bool, bool)]| {
        let mut identities = rows.iter().map(|row| row.0).collect::<Vec<_>>();
        identities.sort();
        identities
    };

    state
        .check_active_cellview()
        .expect("the open cellview checks");
    let checked = rows_about(&state, &name);
    assert_eq!(identities(&checked), identities(&live), "{checked:?}");

    // Another edit: the live pass takes the findings back, still once each.
    let _ = place_open_resistor(&mut state, start, 900);
    let again = rows_about(&state, &name);
    assert_eq!(identities(&again), identities(&live), "{again:?}");
    assert!(again.iter().all(|(_, _, _, current, _)| *current));
}

/// What the canvas marks, as (rule, object, location), or `None` while it
/// marks nothing because the drawing has moved since its findings were found.
type Marker = (String, String, String);

fn marked(state: &AppState) -> Option<Vec<Marker>> {
    let result = state.dialogs.drc_results.as_ref()?;
    (state.dialogs.drc_checked_version == state.schematic.topology_version()).then(|| {
        result
            .violations()
            .iter()
            .map(|violation| {
                (
                    violation.violation_type.rule_code().to_owned(),
                    violation.location.display(),
                    format!("{:?}", violation.location),
                )
            })
            .collect()
    })
}

/// The markers about the component named `name`.
fn marked_about(state: &AppState, name: &str) -> Option<Vec<Marker>> {
    marked(state).map(|markers| {
        markers
            .into_iter()
            .filter(|(_, object, _)| object.contains(&format!(" {name}.")))
            .collect()
    })
}

fn select_component_named(state: &mut AppState, name: &str) {
    let id = state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.name == name)
        .map(|component| component.id)
        .expect("the component is placed");
    state
        .schematic
        .session
        .editor
        .selection
        .select_only_component(id);
}

/// The canvas marks what the page lists: a finding fixed while drawing loses
/// its marker with its row, though the check of the cellview that found it
/// had marked it.
#[test]
fn a_finding_fixed_while_drawing_loses_its_marker_at_once() {
    let mut state = open_state();
    let start = Instant::now();
    let name = place_open_resistor(&mut state, start, 400);
    let sorted = |mut markers: Vec<Marker>| {
        markers.sort();
        markers
    };
    let live = sorted(marked_about(&state, &name).expect("the live pass is marked"));
    assert!(!live.is_empty());
    state
        .check_active_cellview()
        .expect("the open cellview checks");
    assert_eq!(marked_about(&state, &name).map(sorted), Some(live));

    select_component_named(&mut state, &name);
    assert!(state.schematic.delete_selection());
    assert_eq!(marked(&state), None, "a drawing that moved marks nothing");
    settle(&mut state, start);
    assert_eq!(marked_about(&state, &name), Some(Vec::new()));
    assert!(rows_about(&state, &name).is_empty());
}

/// A finding both passes state is marked once, by the newer; and once the
/// drawing moves, the older pass's marker is not left where the finding was.
#[test]
fn a_finding_a_live_pass_restated_is_marked_once_where_it_now_is() {
    let mut state = open_state();
    let start = Instant::now();
    let name = place_open_resistor(&mut state, start, 400);
    state
        .check_active_cellview()
        .expect("the open cellview checks");
    let subject = state.workspace.content.active_schematic_reference();
    // Both passes describe the drawing; the live one is the newer.
    state
        .run_live_check(subject.clone())
        .expect("the drawing checks live");
    let both = marked_about(&state, &name).expect("both passes are current");
    let distinct = both
        .iter()
        .map(|(code, object, _)| (code.clone(), object.clone()))
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(distinct.len(), both.len(), "marked twice: {both:?}");
    assert!(!both.is_empty());

    // Turned, its pins are open where they now are.
    select_component_named(&mut state, &name);
    state.schematic.rotate_selection();
    settle(&mut state, start);
    let turned = marked_about(&state, &name).expect("the live pass is current");
    let live_now = state
        .current_live_receipt(&subject)
        .expect("the live pass describes the drawing")
        .result
        .violations()
        .iter()
        .filter(|violation| violation.location.display().contains(&format!(" {name}.")))
        .map(|violation| format!("{:?}", violation.location))
        .collect::<Vec<_>>();
    assert_eq!(
        turned
            .iter()
            .map(|(_, _, location)| location.clone())
            .collect::<Vec<_>>(),
        live_now,
        "every marker is the live pass's"
    );
    assert!(
        both.iter().any(|old| !turned.contains(old)),
        "turning it moved a finding: {both:?} {turned:?}"
    );
}

#[test]
fn a_waiver_applies_to_a_live_finding_as_to_any_other() {
    let mut state = open_state();
    let start = Instant::now();
    let name = place_open_resistor(&mut state, start, 400);
    let subject = state.workspace.content.active_schematic_reference();
    let (id, target) = state
        .diagnostics
        .batch(&DiagnosticInputs::LiveCheck { subject })
        .expect("the live pass is published")
        .records
        .iter()
        .find_map(|record| match &record.waiver_action {
            Some(WaiverAction::Waive(target)) => {
                Some((record.canonical.diagnostic_id, target.clone()))
            }
            _ => None,
        })
        .expect("an open pin may be waived");

    state
        .waive_finding(&target, "left open by design")
        .expect("the waiver is recorded");

    let waived = rows_about(&state, &name)
        .into_iter()
        .find(|row| row.0 == id)
        .expect("the waived finding is still listed");
    assert!(waived.4, "{waived:?}");
}

#[test]
fn turning_live_checking_off_withdraws_its_findings() {
    let mut state = open_state();
    let start = Instant::now();
    let name = place_open_resistor(&mut state, start, 400);
    assert!(!rows_about(&state, &name).is_empty());

    state
        .ui
        .preferences
        .set_toggle(TogglePreference::IncrementalConnectivityChecks, false);
    assert_eq!(state.refresh_live_checks(start), None);
    assert!(rows(&state).is_empty());
}
