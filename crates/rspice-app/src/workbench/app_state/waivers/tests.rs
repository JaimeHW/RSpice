//! A waiver is a decision about one finding, stored in the document of the
//! cellview it waives: it is saved and reopened with that document, undone
//! like any edit of it, follows its finding's object and nothing else, and is
//! reported wherever it does not do what it says.

use super::*;
use crate::state::{ComponentType, Point};
use crate::workbench::app_state::diagnostic_store::DiagnosticInputs;
use crate::workbench::app_state::diagnostic_store::listing::listing;
use rspice_design::drc::{DrcLocation, DrcViolationType};

/// The rule the fixtures waive: an error by default, and waivable.
const COLLISION: &str = "DRC-CASE-COLLIDING-NET-NAMES";

const REASON: &str = "sense is tied to SENSE on purpose";

/// An open project whose drawing holds two labels the deck folds into one
/// node.
fn colliding_state() -> AppState {
    let mut state = AppState::default();
    state.project_lifecycle.authority.open_session();
    add_collision(&mut state, "sense", 600);
    state
}

/// Two labels spelled `name` and its upper case: one case collision, located
/// at the second label. Drawn as one edit, as the canvas draws, so an undo
/// after it steps back over the edit that follows and no further.
fn add_collision(state: &mut AppState, name: &str, y: i32) {
    state
        .schematic
        .with_undo("Label the collision", |schematic| {
            schematic.add_net_label(Point::new(600, y), name.to_owned());
            schematic.add_net_label(Point::new(600, y + 100), name.to_ascii_uppercase());
        });
    state.sync_active_schematic_to_workspace();
}

/// Run the Check command and return what it published for the view in front
/// of the reader.
fn checked(state: &mut AppState) -> Vec<DiagnosticRecord> {
    state
        .run_active_design_checks()
        .expect("the checks run on the drawing");
    published(state)
}

/// What the design checks have published for the view in front of the
/// reader, as it stands now: re-stated, where something re-stated it.
fn published(state: &AppState) -> Vec<DiagnosticRecord> {
    let subject = state.workspace.content.active_schematic_reference();
    state
        .diagnostics
        .batch(&DiagnosticInputs::DesignCheck { subject })
        .expect("the checks published their findings")
        .records
        .clone()
}

fn of_code<'a>(records: &'a [DiagnosticRecord], code: &str) -> Vec<&'a DiagnosticRecord> {
    records
        .iter()
        .filter(|record| &*record.key.code == code)
        .collect()
}

/// The one record of `code`.
fn one<'a>(records: &'a [DiagnosticRecord], code: &str) -> &'a DiagnosticRecord {
    match of_code(records, code)[..] {
        [record] => record,
        ref found => panic!("one {code} record, found {}", found.len()),
    }
}

/// Where the page would waive `record`.
fn to_waive(record: &DiagnosticRecord) -> WaiverTarget {
    match &record.waiver_action {
        Some(WaiverAction::Waive(target)) => target.clone(),
        other => panic!("{} offers no waiver: {other:?}", record.key.code),
    }
}

/// Where the page would delete the stored waiver `record` reports.
fn to_delete(record: &DiagnosticRecord) -> WaiverTarget {
    match &record.waiver_action {
        Some(WaiverAction::Delete(target)) => target.clone(),
        other => panic!("{} offers no deletion: {other:?}", record.key.code),
    }
}

fn active_waivers(state: &AppState) -> &[DrcWaiver] {
    state.schematic.document().waivers.entries()
}

/// Check the drawing and waive its one case collision, as the page does.
fn waive_the_collision(state: &mut AppState) {
    let target = to_waive(one(&checked(state), COLLISION));
    state
        .waive_finding(&target, REASON)
        .expect("a case collision may be waived");
}

/// Identity is structural, so an edit elsewhere in the drawing leaves the
/// waiver on the finding it was recorded against.
#[test]
fn a_waiver_survives_a_recheck_after_an_unrelated_edit() {
    let mut state = colliding_state();
    waive_the_collision(&mut state);
    state
        .schematic
        .with_undo("Unrelated drawing edit", |schematic| {
            schematic.add_component(ComponentType::Resistor, Point::new(1_200, 1_200));
        });
    state.sync_active_schematic_to_workspace();

    let records = checked(&mut state);
    let finding = one(&records, COLLISION);
    assert!(finding.waived(), "the waiver still covers its finding");
    assert!(
        of_code(&records, "WAIVER-UNMATCHED").is_empty(),
        "and nothing says it covers nothing"
    );
}

/// A waiver is on one object. Another finding of the same rule, on another
/// object, is not covered by it and is offered a waiver of its own.
#[test]
fn a_waiver_follows_nothing_to_another_finding_of_its_rule() {
    let mut state = colliding_state();
    add_collision(&mut state, "bias", 900);
    let records = checked(&mut state);
    let collisions = of_code(&records, COLLISION);
    assert_eq!(collisions.len(), 2, "precondition: two collisions");
    let first = to_waive(collisions[0]);
    let second = to_waive(collisions[1]);
    assert_ne!(first.object, second.object);
    state
        .waive_finding(&first, REASON)
        .expect("a case collision may be waived");

    let records = checked(&mut state);
    let waived = of_code(&records, COLLISION)
        .into_iter()
        .map(DiagnosticRecord::waived)
        .collect::<Vec<_>>();
    assert_eq!(waived, [true, false]);
    assert_eq!(
        to_waive(of_code(&records, COLLISION)[1]),
        second,
        "the other finding is offered its own waiver"
    );
}

/// A waiver is an edit of the document that stores it: undo takes it away,
/// and the history step is what re-states the published finding, without
/// waiting for somebody to re-run the checks. That the Undo command takes
/// the step is the Problems page's test; this is the state it leaves.
#[test]
fn a_history_step_that_takes_a_waiver_away_re_states_the_finding() {
    let mut state = colliding_state();
    let records = checked(&mut state);
    let errors = listing(&state).counts().errors;
    assert!(errors > 0, "precondition: the collision counts as an error");
    let target = to_waive(one(&records, COLLISION));
    state
        .waive_finding(&target, REASON)
        .expect("a case collision may be waived");
    assert!(
        state.schematic.session.is_dirty,
        "a waiver dirties its cellview"
    );
    assert!(one(&published(&state), COLLISION).waived());
    assert_eq!(listing(&state).counts().errors, errors - 1);

    let before = state.stored_waivers();
    assert!(state.schematic.undo(), "the waiver is one undo step");
    state.sync_active_schematic_to_workspace();
    assert!(active_waivers(&state).is_empty(), "undo takes it away");
    assert!(
        state.restate_waived_findings(&before),
        "a schematic's waivers changed"
    );
    let finding = one(&published(&state), COLLISION).clone();
    assert!(
        !finding.waived(),
        "the published finding is no longer waived"
    );
    assert!(matches!(
        finding.waiver_action,
        Some(WaiverAction::Waive(_))
    ));
    assert_eq!(listing(&state).counts().errors, errors);

    let unchanged = state.stored_waivers();
    assert!(
        !state.restate_waived_findings(&unchanged),
        "a step that changes no waiver re-states nothing"
    );
}

/// A finding of a rule the table says may not be waived carries no waiver
/// action, and a waiver asked for anyway is refused.
#[test]
fn a_rule_that_may_not_be_waived_offers_no_waiver() {
    let mut state = colliding_state();
    let records = checked(&mut state);
    assert!(
        records
            .iter()
            .filter(|record| record.key.code.starts_with("DRC-"))
            .all(|record| {
                let waivable = DrcViolationType::from_rule_code(&record.key.code)
                    .is_some_and(|rule| rule.rule().waivable);
                record.waiver_action.is_some() == waivable
            }),
        "a waiver is offered exactly where the rule may be waived",
    );

    publish_duplicate_name(&mut state);
    let records = published(&state);
    let duplicate = one(&records, "DRC-DUPLICATE-NAME");
    assert!(duplicate.waiver_action.is_none());
    let refused = state.waive_finding(
        &WaiverTarget {
            cell_view: state.workspace.content.active_schematic_reference(),
            rule: Arc::from("DRC-DUPLICATE-NAME"),
            object: "component:7".to_owned(),
        },
        REASON,
    );
    assert!(
        refused
            .as_ref()
            .is_err_and(|error| error.contains("may not be waived")),
        "{refused:?}"
    );
    assert!(active_waivers(&state).is_empty());
}

/// One duplicate-name error, on component 7, published as a pass of the
/// view in front of the reader.
fn publish_duplicate_name(state: &mut AppState) {
    let mut result = DrcResult::new();
    result.add_violation(DrcViolation::new(
        1,
        DrcViolationType::DuplicateName,
        "R7 is named twice",
        DrcLocation::Component {
            id: 7,
            name: "R7".to_owned(),
        },
    ));
    result.completed = true;
    state
        .publish_active_design_check_result(result)
        .expect("the pass publishes");
}

/// A waiver stored on a rule that may not be waived — written by hand, or by
/// a build whose table differed — is kept, applied to nothing, and reported
/// with a Delete action that removes it.
#[test]
fn a_stored_waiver_on_a_rule_that_may_not_be_waived_is_reported_and_ignored() {
    let mut state = colliding_state();
    state.schematic.document_mut_for_test().waivers = serde_json::from_value(serde_json::json!({
        "schema_version": 1,
        "waivers": [{
            "rule": "DRC-DUPLICATE-NAME",
            "object": "component:7",
            "reason": "written by hand",
            "author": "ana",
            "waived_at_ms": 1_760_000_000_000_u64,
        }],
    }))
    .expect("a stored list always reads");
    state.sync_active_schematic_to_workspace();
    publish_duplicate_name(&mut state);

    let records = published(&state);
    let duplicate = one(&records, "DRC-DUPLICATE-NAME");
    assert!(!duplicate.waived(), "the waiver is applied to nothing");
    assert_eq!(listing(&state).counts().errors, 1, "the error still counts");
    let report = one(&records, "WAIVER-NOT-WAIVABLE");
    assert_eq!(report.level, DiagnosticLevel::Warning);
    assert!(
        report.message.contains("DRC-DUPLICATE-NAME"),
        "{}",
        report.message
    );

    state
        .remove_finding_waiver(&to_delete(report))
        .expect("any stored waiver may be deleted");
    assert!(active_waivers(&state).is_empty());
    assert!(of_code(&published(&state), "WAIVER-NOT-WAIVABLE").is_empty());
}

/// A waiver whose finding is gone is neither kept silently nor deleted
/// silently: it is reported, once, and its Delete action removes it.
#[test]
fn an_unmatched_waiver_is_reported_and_its_delete_removes_it() {
    let mut state = colliding_state();
    waive_the_collision(&mut state);
    let label = state
        .schematic
        .document()
        .net_labels
        .iter()
        .find(|label| label.name == "SENSE")
        .cloned()
        .expect("the second spelling is drawn");
    assert!(
        state
            .schematic
            .rename_net_label(label, "sense".to_owned())
            .expect("the label renames"),
        "the two labels are now spelled one way"
    );
    state.sync_active_schematic_to_workspace();

    let records = checked(&mut state);
    assert!(
        of_code(&records, COLLISION).is_empty(),
        "precondition: repaired"
    );
    let report = one(&records, "WAIVER-UNMATCHED");
    assert_eq!(report.level, DiagnosticLevel::Info);
    assert!(report.message.contains(COLLISION), "{}", report.message);

    state
        .remove_finding_waiver(&to_delete(report))
        .expect("the waiver is deleted");
    assert!(active_waivers(&state).is_empty());
    assert!(
        of_code(&published(&state), "WAIVER-UNMATCHED").is_empty(),
        "the report goes with the waiver"
    );
}
