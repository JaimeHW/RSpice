//! Waiving from the page: where the verbs are offered, how the reason is
//! asked for, what the region and a copied row say about a waiver, and what
//! recording or removing one does to a run already checked.

use super::*;
use crate::workbench::app_state::waivers::WaiverAction;

const REASON: &str = "the stub is a probe point";

/// The current finding's record, as the store holds it.
fn current_record(state: &AppState) -> DiagnosticRecord {
    let current = state
        .diagnostics
        .current_finding()
        .expect("a finding is current");
    let subject = state.workspace.content.active_schematic_reference();
    state
        .diagnostics
        .batch(&DiagnosticInputs::DesignCheck { subject })
        .expect("the checks published")
        .records
        .iter()
        .find(|record| record.canonical.diagnostic_id == current)
        .cloned()
        .expect("the current finding is a design-rule finding")
}

/// The design findings of [`state_with_design_findings`], with the dangling
/// wire — a rule that may be waived — current.
fn wire_current() -> AppState {
    let mut state = state_with_design_findings();
    let wire = reported(&state)[1];
    state.diagnostics.set_current_finding(Some(wire));
    state
}

/// Type `text` into whatever holds the keyboard.
fn typed(text: &str) -> egui::RawInput {
    sized(egui::RawInput {
        events: vec![egui::Event::Text(text.to_owned())],
        ..Default::default()
    })
}

/// Waive... is offered on a finding whose rule may be waived and on no
/// other: the missing parameter is repaired, never accepted.
#[test]
fn waive_is_offered_only_where_the_rule_may_be_waived() {
    let mut state = state_with_design_findings();
    let [missing, wire] = reported(&state)[..] else {
        panic!("two findings");
    };

    state.diagnostics.set_current_finding(Some(wire));
    let offered = render(&mut state, sized(egui::RawInput::default()));
    assert!(offered.has("Waive\u{2026}"), "{:?}", offered.texts);

    state.diagnostics.set_current_finding(Some(missing));
    let withheld = render(&mut state, sized(egui::RawInput::default()));
    assert!(withheld.has("R1 has no resistance"), "precondition: drawn");
    assert!(
        !withheld.has("Waive"),
        "a rule that may not be waived offers no waiver: {:?}",
        withheld.texts,
    );
}

/// The reason is asked for where the finding is read, and the keyboard is
/// already in it: typing and Enter record the waiver. The region then states
/// the decision — reason, author, time — and offers to take it away, which
/// Remove waiver does.
#[test]
fn a_waiver_is_recorded_with_its_reason_and_removed_from_the_detail_region() {
    let mut state = wire_current();
    let harness = Harness::new();
    harness.settle(&mut state);
    let render = harness.frame(&mut state, sized(egui::RawInput::default()));
    let _ = harness.frame(&mut state, click_on(&render, "Waive\u{2026}"));
    let asked = harness.frame(&mut state, sized(egui::RawInput::default()));
    assert!(
        asked.has("Reason"),
        "the reason is asked for: {:?}",
        asked.texts
    );
    assert!(
        !asked.has("Copy row"),
        "the reason takes the verbs' place while it is written",
    );
    let _ = harness.frame(&mut state, typed(REASON));
    let _ = harness.frame(&mut state, keyed(egui::Key::Enter));
    harness.settle(&mut state);

    let record = current_record(&state);
    let waiver = record
        .canonical
        .suppression
        .as_ref()
        .expect("Enter records the waiver");
    assert_eq!(waiver.reason, REASON);
    assert_eq!(state.schematic.document().waivers.entries().len(), 1);
    assert!(state.problems_view.waiving.is_none(), "the draft is closed");

    let stated = harness.frame(&mut state, sized(egui::RawInput::default()));
    let line = format!("Waived \u{00b7} {}", detail::waiver_text(waiver));
    assert!(stated.has(&line), "{line:?} in {:?}", stated.texts);
    assert!(stated.has("Remove waiver"), "{:?}", stated.texts);

    let _ = harness.frame(&mut state, click_on(&stated, "Remove waiver"));
    harness.settle(&mut state);
    assert!(state.schematic.document().waivers.is_empty());
    assert!(!current_record(&state).waived());
}

/// Escape abandons the reason without recording anything, and a reason being
/// written goes when the reader stands on another finding: it is never
/// recorded against a finding it was not written for.
#[test]
fn a_reason_is_abandoned_by_escape_and_by_standing_elsewhere() {
    let mut state = wire_current();
    let harness = Harness::new();
    harness.settle(&mut state);
    let render = harness.frame(&mut state, sized(egui::RawInput::default()));
    let _ = harness.frame(&mut state, click_on(&render, "Waive\u{2026}"));
    harness.settle(&mut state);
    let _ = harness.frame(&mut state, typed(REASON));
    let _ = harness.frame(&mut state, keyed(egui::Key::Escape));
    harness.settle(&mut state);
    assert!(state.problems_view.waiving.is_none(), "Escape abandons it");
    assert!(state.schematic.document().waivers.is_empty());

    let render = harness.frame(&mut state, sized(egui::RawInput::default()));
    let _ = harness.frame(&mut state, click_on(&render, "Waive\u{2026}"));
    assert!(state.problems_view.waiving.is_some(), "precondition: asked");
    let missing = reported(&state)[0];
    state.diagnostics.set_current_finding(Some(missing));
    harness.settle(&mut state);
    assert!(
        state.problems_view.waiving.is_none(),
        "the reason goes with the finding it was written for",
    );
    assert!(state.schematic.document().waivers.is_empty());
}

/// The row's own menu offers the same verb, and opens the same reason in the
/// detail region, on the row it was opened from.
#[test]
fn the_row_menu_waives_the_row_it_was_opened_on() {
    let mut state = state_with_design_findings();
    let wire = reported(&state)[1];
    let harness = Harness::new();
    harness.settle(&mut state);
    let render = harness.frame(&mut state, sized(egui::RawInput::default()));
    let row = render.cell("Wire ends open").expect("the row is drawn");
    let pressed = |pressed| egui::Event::PointerButton {
        pos: row.center(),
        button: egui::PointerButton::Secondary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    let _ = harness.frame(
        &mut state,
        sized(egui::RawInput {
            events: vec![
                egui::Event::PointerMoved(row.center()),
                pressed(true),
                pressed(false),
            ],
            ..Default::default()
        }),
    );
    // A menu is laid out unseen on the frame it opens, and drawn after.
    harness.settle(&mut state);
    let menu = harness.frame(&mut state, sized(egui::RawInput::default()));
    let _ = harness.frame(&mut state, click_on(&menu, "Waive\u{2026}"));
    harness.settle(&mut state);
    assert_eq!(state.diagnostics.current_finding(), Some(wire));
    assert_eq!(
        state
            .problems_view
            .waiving
            .as_ref()
            .map(|draft| draft.finding),
        Some(wire),
        "the menu opens the reason for the row it was opened on",
    );
    let asked = harness.frame(&mut state, sized(egui::RawInput::default()));
    assert!(asked.has("Reason"), "{:?}", asked.texts);
}

/// A copied row carries the waiver as its last column, and the copied heading
/// names it: a waived finding pasted into a review says who accepted it, why
/// and when.
#[test]
fn a_copied_row_carries_its_waiver() {
    let mut state = open_state();
    let mut waived = record(&state, "T-1", DiagnosticLevel::Error, "a waived error");
    waived.canonical.suppression = Some(waiver("T-1"));
    let open = record(&state, "T-2", DiagnosticLevel::Error, "an open error");
    publish(&mut state, vec![waived, open]);
    let listed = listing(&state);

    let stamp = crate::time_compat::utc_stamp(1_760_000_000_000);
    assert!(copy_of(&listed.row(0).expect("listed")).ends_with(&format!(
        "\taccepted at review \u{00b7} ana \u{00b7} {stamp}"
    )),);
    assert!(
        copy_of(&listed.row(1).expect("listed")).ends_with("\tan open error\t"),
        "an unwaived row leaves the column empty",
    );
    let arrangement = Arrangement::build(&listed, &state.problems_view);
    let copied = copy_of_listed(&listed, &arrangement);
    assert_eq!(
        copied.lines().next(),
        Some("Severity\tRule\tSubject\tProducer\tLocation\tMessage\tWaiver"),
    );
}

/// The voltage divider, runnable as it stands, with two labels the deck
/// folds into one node: an error the run is refused on, and one a waiver may
/// accept. The Console is open on the Problems page.
fn runnable_colliding_app() -> crate::workbench::RSpiceApp {
    let mut app = crate::workbench::RSpiceApp::test_instance();
    for dimension in &mut app.state.sim_setup.run_set.dimensions {
        if dimension.kind
            == rspice_simulation_contract::run_set::RunSetDimensionKind::ProcessSection
        {
            dimension.enabled = false;
        }
    }
    crate::workbench::examples::load_example("Voltage Divider", &mut app.state.schematic);
    app.state.provision_test_project_technology_contract();
    app.state
        .schematic
        .add_net_label(Point::new(600, 600), "sense".to_owned());
    app.state
        .schematic
        .add_net_label(Point::new(600, 700), "SENSE".to_owned());
    app.state.sync_active_schematic_to_workspace();
    app.state
        .run_active_design_checks()
        .expect("the checks run");
    app.state.workbench.console_visible = true;
    app.state.workbench.console_page = crate::workbench::state::ConsolePage::Problems;
    app
}

/// Make the case collision current.
fn collision_current(state: &mut AppState) {
    let subject = state.workspace.content.active_schematic_reference();
    let collision = state
        .diagnostics
        .batch(&DiagnosticInputs::DesignCheck { subject })
        .expect("the checks published")
        .records
        .iter()
        .find(|record| &*record.key.code == "DRC-CASE-COLLIDING-NET-NAMES")
        .map(|record| record.canonical.diagnostic_id)
        .expect("the collision is found");
    state.diagnostics.set_current_finding(Some(collision));
}

/// One frame of the Console dock, as the workbench draws it.
fn console_frame(
    harness: &Harness,
    app: &mut crate::workbench::RSpiceApp,
    input: egui::RawInput,
) -> Render {
    collect(harness.ctx.run_ui(input, |ui| {
        let layout =
            crate::workbench::layout::LayoutSpec::resolve(1_280.0, 760.0, &app.state.workbench);
        super::super::super::show(ui, app, layout);
    }))
}

/// A waiver taken away on the page changes what refuses a run, as applying
/// the rule deck does: the preflight report that let the run through and the
/// run it prepared are both dropped, and the run is refused again.
#[test]
fn a_waiver_removed_on_the_page_drops_preflight_and_the_prepared_run() {
    let mut app = runnable_colliding_app();
    collision_current(&mut app.state);
    let target = match current_record(&app.state).waiver_action {
        Some(WaiverAction::Waive(target)) => target,
        other => panic!("the collision may be waived: {other:?}"),
    };
    app.state
        .waive_finding(&target, REASON)
        .expect("a case collision may be waived");
    assert!(
        app.simulation_controller
            .prepare_run_set_for_preflight(&app.state)
            .is_ok(),
        "precondition: the waived error passes the run gate",
    );
    assert!(app.simulation_controller.retained_run_deck().is_some());
    retain_report(&mut app.state, Vec::new());
    collision_current(&mut app.state);

    let harness = Harness::new();
    let _ = console_frame(&harness, &mut app, sized(egui::RawInput::default()));
    let render = console_frame(&harness, &mut app, sized(egui::RawInput::default()));
    let _ = console_frame(&harness, &mut app, click_on(&render, "Remove waiver"));

    assert!(app.state.schematic.document().waivers.is_empty());
    assert!(
        app.state.workbench.preflight.report.is_none(),
        "the report that let the run through no longer stands",
    );
    assert!(
        app.simulation_controller.retained_run_deck().is_none(),
        "nor does the run it prepared",
    );
    assert!(
        app.simulation_controller
            .prepare_run_set_for_preflight(&app.state)
            .is_err(),
        "the error refuses the run again",
    );
}

/// A waiver recorded on the page drops a report that refused the run on the
/// finding it waives, so the next pass judges the design as it now stands.
#[test]
fn a_waiver_recorded_on_the_page_drops_the_report_that_refused_the_run() {
    let mut app = runnable_colliding_app();
    assert!(
        app.simulation_controller
            .prepare_run_set_for_preflight(&app.state)
            .is_err(),
        "precondition: the error refuses the run",
    );
    retain_report(
        &mut app.state,
        vec![crate::workbench::state::PreflightIssue {
            check: "Schematic validation".to_owned(),
            observed: "1 design-rule error".to_owned(),
            required: "No error-level design-rule findings".to_owned(),
            remediation: crate::workbench::state::PreflightRemediation::DesignChecks,
        }],
    );
    collision_current(&mut app.state);

    let harness = Harness::new();
    let _ = console_frame(&harness, &mut app, sized(egui::RawInput::default()));
    let render = console_frame(&harness, &mut app, sized(egui::RawInput::default()));
    let _ = console_frame(&harness, &mut app, click_on(&render, "Waive\u{2026}"));
    let _ = console_frame(&harness, &mut app, sized(egui::RawInput::default()));
    let _ = console_frame(&harness, &mut app, typed(REASON));
    let asked = console_frame(&harness, &mut app, sized(egui::RawInput::default()));
    let record = asked
        .exact("Waive")
        .expect("the verb that records the reason is drawn");
    let _ = console_frame(&harness, &mut app, clicked(record + egui::vec2(4.0, 4.0)));

    assert_eq!(app.state.schematic.document().waivers.entries().len(), 1);
    assert!(
        app.state.workbench.preflight.report.is_none(),
        "the report that refused the run no longer stands",
    );
    assert!(
        app.simulation_controller
            .prepare_run_set_for_preflight(&app.state)
            .is_ok(),
        "the next pass lets the waived error through",
    );
}

/// Retain a preflight report over the design as it stands, as a pass does,
/// with `blockers` against it.
fn retain_report(state: &mut AppState, blockers: Vec<crate::workbench::state::PreflightIssue>) {
    let (topology_root, topology_revision, topology_closure) = state.configured_topology_revision();
    let plan = state.active_plan_revision();
    state.workbench.preflight.report = Some(crate::workbench::state::PreflightReport {
        project_revision: state.workspace.content.project.revision().get(),
        topology_root,
        topology_revision,
        topology_closure,
        simulation_plan_id: plan.map(|(id, _)| id),
        simulation_plan_revision: plan.map(|(_, revision)| revision),
        blockers,
        advisories: Vec::new(),
        prepared: None,
    });
}

/// The Undo command takes a waiver away and Redo puts it back, and each
/// re-states the finding and drops the preflight checked under the waivers
/// as they stood, as the page does when it records or removes one.
#[test]
fn undo_and_redo_of_a_waiver_re_state_the_finding_and_drop_preflight() {
    // Only the two labels, drawn as one edit: undo then steps back over the
    // waiver and nothing else.
    let mut app = design_app();
    app.state
        .schematic
        .with_undo("Label the collision", |schematic| {
            schematic.add_net_label(Point::new(600, 600), "sense".to_owned());
            schematic.add_net_label(Point::new(600, 700), "SENSE".to_owned());
        });
    app.state.sync_active_schematic_to_workspace();
    app.state
        .run_active_design_checks()
        .expect("the checks run");
    collision_current(&mut app.state);
    let target = match current_record(&app.state).waiver_action {
        Some(WaiverAction::Waive(target)) => target,
        other => panic!("the collision may be waived: {other:?}"),
    };
    app.state
        .waive_finding(&target, REASON)
        .expect("a case collision may be waived");
    collision_current(&mut app.state);
    assert!(current_record(&app.state).waived());

    retain_report(&mut app.state, Vec::new());
    app.action_edit_undo();
    assert!(app.state.schematic.document().waivers.is_empty());
    collision_current(&mut app.state);
    assert!(
        !current_record(&app.state).waived(),
        "the finding is stated unwaived without a re-check",
    );
    assert!(
        app.state.workbench.preflight.report.is_none(),
        "the report checked under the waiver is dropped",
    );

    retain_report(&mut app.state, Vec::new());
    app.action_edit_redo();
    assert_eq!(app.state.schematic.document().waivers.entries().len(), 1);
    collision_current(&mut app.state);
    assert!(current_record(&app.state).waived(), "redo puts it back");
    assert!(app.state.workbench.preflight.report.is_none());
}
