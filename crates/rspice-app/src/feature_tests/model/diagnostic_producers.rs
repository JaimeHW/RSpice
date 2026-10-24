//! `workbench/app_state/diagnostic_store/producers` observed through a whole session.

use crate::diagnostics::LogAnchor;
use crate::state::ConvergenceAttribution;
use crate::state::PreflightAdvisory;
use crate::state::PreflightIssue;
use crate::state::PreflightRemediation;
use crate::state::PreflightReport;
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::diagnostic_store::DiagnosticRoute;
use crate::workbench::app_state::diagnostic_store::listing;
use crate::workbench::app_state::diagnostic_store::producers::*;

fn open_state() -> AppState {
    let mut state = AppState::default();
    state.open_project_session();
    state
}

#[test]
fn a_code_is_one_word_per_label_word() {
    assert_eq!(
        screaming_code("MODEL", "Model bindings"),
        "MODEL-MODEL-BINDINGS"
    );
    assert_eq!(
        screaming_code("RUN", "Corners & sections"),
        "RUN-CORNERS-SECTIONS"
    );
}

/// The blocker's own destination is what the row follows. Before this the
/// only way to reach it was the dialog's Action button.
#[test]
fn a_blocker_becomes_a_record_routed_to_its_own_remediation() {
    let mut state = open_state();
    let report = PreflightReport {
        project_revision: 4,
        topology_root: String::new(),
        topology_revision: 0,
        topology_closure: Vec::new(),
        simulation_plan_id: None,
        simulation_plan_revision: None,
        blockers: vec![PreflightIssue {
            check: "Model bindings".to_owned(),
            observed: "Corner tt declares no bulk section".to_owned(),
            required: "Every enabled corner binds each required domain".to_owned(),
            remediation: PreflightRemediation::model_corner("analog", Some("tt".to_owned())),
        }],
        advisories: vec![PreflightAdvisory::from(
            "Technology: not attached".to_owned(),
        )],
        prepared: None,
    };

    state.publish_dispatch_diagnostics(&report);

    let listed = listing::listing(&state);
    assert_eq!(listed.len(), 2);
    let counts = listed.counts();
    assert_eq!(counts.errors, 1);
    assert_eq!(counts.advisories, 1);
    let blocker = listed.row(0).expect("the blocker is listed");
    assert_eq!(blocker.code, "MODEL-MODEL-BINDINGS");
    assert_eq!(blocker.subject, "analog · tt");
    assert_eq!(
        blocker.route,
        Some(DiagnosticRoute::Repair(PreflightRemediation::model_corner(
            "analog",
            Some("tt".to_owned())
        ))),
        "a blocker's route is the destination it named for itself",
    );
    let advisory = listed.row(1).expect("the advisory is listed");
    assert!(
        advisory.route.is_none(),
        "this advisory names no destination"
    );
}

/// Advisories that name no destination keep their identities, and the
/// reader's selection, when the next pass reports them in another order.
#[test]
fn an_advisory_with_no_destination_is_named_by_what_it_is_about() {
    let mut state = open_state();
    let report = |advisories: Vec<PreflightAdvisory>| PreflightReport {
        project_revision: 4,
        topology_root: String::new(),
        topology_revision: 0,
        topology_closure: Vec::new(),
        simulation_plan_id: None,
        simulation_plan_revision: None,
        blockers: Vec::new(),
        advisories,
        prepared: None,
    };
    let fallback = PreflightAdvisory::about(
        "configuration-fallback",
        "lib/amp/schematic",
        "Configuration fallback reviewed for lib/amp/schematic at X1".to_owned(),
    );
    let warning = PreflightAdvisory::about(
        "configuration-warning",
        "lib/amp/schematic:stop view",
        "Configuration warning for lib/amp/schematic: stop view".to_owned(),
    );
    let identities = |state: &AppState| {
        let listed = listing::listing(state);
        (0..listed.len())
            .filter_map(|index| listed.row(index))
            .map(|row| (row.message.to_owned(), row.id))
            .collect::<std::collections::BTreeMap<_, _>>()
    };

    state.publish_dispatch_diagnostics(&report(vec![fallback.clone(), warning.clone()]));
    let before = identities(&state);
    assert_eq!(before.len(), 2);
    let selected = before[&warning.message];
    state.model.diagnostics.set_current_finding(Some(selected));

    state.publish_dispatch_diagnostics(&report(vec![warning.clone(), fallback.clone()]));
    assert_eq!(identities(&state), before);
    assert_eq!(state.model.diagnostics.current_finding(), Some(selected));

    // One of them gone: the other keeps its identity rather than taking
    // over the place the first one held.
    state.publish_dispatch_diagnostics(&report(vec![warning.clone()]));
    assert_eq!(
        identities(&state).get(&warning.message),
        Some(&selected),
        "the remaining advisory keeps its identity"
    );
    assert_eq!(state.model.diagnostics.current_finding(), Some(selected));
}

/// An attribution recorded for a solve that was later rescued must not be
/// shown against an unrelated failure.
#[test]
fn a_run_failure_takes_the_attribution_that_describes_it() {
    use rspice_results::convergence_attribution::{
        ConvergenceFailureClass, ConvergenceSite, ConvergenceSiteKind,
    };

    let mut state = open_state();
    let attribution = ConvergenceAttribution {
        class: ConvergenceFailureClass::NoDcPathToGround,
        sites: vec![ConvergenceSite {
            name: "vbias".to_owned(),
            kind: ConvergenceSiteKind::Node,
            residual: None,
        }],
        elided_sites: 0,
        failure_message: "no DC path at vbias".to_owned(),
    };

    state.publish_run_failure(
        "Transient",
        "Analysis failed: no DC path at vbias",
        Some(&attribution),
        7,
    );
    let listed = listing::listing(&state);
    let row = listed.row(0).expect("the failure is listed");
    assert_eq!(row.code, "RUN-NO-DC-PATH-TO-GROUND");
    assert_eq!(row.subject, "Transient · run 7");
    assert!(matches!(
        row.route,
        Some(DiagnosticRoute::Anchor(LogAnchor::Simulation { .. }))
    ));

    state.publish_run_failure(
        "Transient",
        "Analysis failed: unrelated refusal",
        Some(&attribution),
        7,
    );
    let listed = listing::listing(&state);
    let row = listed.row(1).expect("the second failure is listed");
    assert_eq!(
        row.code, "RUN-FAILED",
        "an attribution for another failure must not be adopted",
    );
    assert!(row.route.is_none());
}

/// Two analyses that fail in one run are two findings. Each failure used
/// to be published as the whole batch, so the second replaced the first.
#[test]
fn every_failure_in_one_run_stands_under_its_own_identity() {
    let mut state = open_state();
    state.publish_run_failure("Operating point", "Analysis failed: OP", None, 3);
    state.publish_run_failure("Transient", "Analysis failed: tran", None, 3);

    let listed = listing::listing(&state);
    assert_eq!(listed.len(), 2, "both failures stand");
    let first = listed.row(0).expect("the first failure is listed");
    let second = listed.row(1).expect("the second failure is listed");
    assert_eq!(first.subject, "Operating point · run 3");
    assert_eq!(second.subject, "Transient · run 3");
    assert_ne!(first.id, second.id);
    assert_eq!(listed.counts().errors, 2);
}
