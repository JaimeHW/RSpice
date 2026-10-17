//! What the detail region offers for the current finding: the rule's own
//! words, the producer's fix, where the finding's batch stands, and the
//! other places the finding names.

use super::*;

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

/// A document place a finding also names is listed, and is offered as a jump
/// only where something can resolve it: a finding whose own route names no
/// source bundle has no editor to open that document in, so it is stated.
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

/// A finding a line of a run's log states offers that line, and following it
/// asks for that run's log at that line. A finding no line states offers no
/// line at all, rather than a jump with nowhere to land.
#[test]
fn a_finding_a_runs_log_states_offers_that_line_and_one_no_line_states_offers_none() {
    use crate::workbench::app_state::diagnostic_store::RunLogPlace;

    let mut state = open_state();
    let place = RunLogPlace {
        run_sequence: 1,
        line: 3,
    };
    let stated = record(
        &state,
        "T-STATED",
        DiagnosticLevel::Warning,
        "stated by the run",
    )
    .with_log_place(Some(place));
    let silent = record(
        &state,
        "T-SILENT",
        DiagnosticLevel::Warning,
        "raised silently",
    );
    publish(&mut state, vec![stated, silent]);
    let order = reported(&state);

    state.diagnostics.set_current_finding(Some(order[1]));
    let unstated = render(&mut state, sized(egui::RawInput::default()));
    assert!(
        unstated.has("raised silently") && !unstated.has(" log \u{00b7} line "),
        "a finding no line states names no line: {:?}",
        unstated.texts,
    );
    assert!(unstated.exact("Go").is_none(), "and offers no jump");

    state.diagnostics.set_current_finding(Some(order[0]));
    let harness = Harness::new();
    harness.settle(&mut state);
    let offered = harness.frame(&mut state, sized(egui::RawInput::default()));
    assert!(
        offered.has("Run 1 log \u{00b7} line 4"),
        "the line, counted as a reader counts: {:?}",
        offered.texts,
    );
    let go = offered.exact("Go").expect("the line is offered as a jump");
    let _ = harness.frame(&mut state, clicked(go + egui::vec2(3.0, 3.0)));
    assert_eq!(
        state.diagnostics.take_pending_route(),
        Some(DiagnosticRoute::RunLogLine {
            run_sequence: 1,
            line: 3,
        }),
    );
}
