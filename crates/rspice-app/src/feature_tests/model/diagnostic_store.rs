//! `workbench/app_state/diagnostic_store` observed through a whole session.

use crate::state::CampaignMemberRefusal;
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::diagnostic_store::*;
use rspice_vocabulary::diagnostics::CanonicalDiagnosticMetadata;

/// A symbol view whose pins the checks can read: `work/amp/symbol`, with
/// one pin, beside a schematic that declares the matching port.
fn open_symbol_view(state: &mut AppState) -> crate::state::CellViewRef {
    use crate::state::{
        Cell, CellViewRef, ComponentType, Library, Point, PortDirection, SchematicState,
        SymbolDocument, SymbolPin, View, ViewType,
    };

    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    amp.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(amp);
    state.model.library_manager.add_library(library);
    let mut schematic = SchematicState::default();
    let port = schematic.add_component(ComponentType::Port, Point::new(0, 0));
    schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == port)
        .expect("the fixture just placed this port")
        .value = "IN".to_owned();
    state.model.workspace.insert_schematic_editor(
        CellViewRef::new("work", "amp", "schematic").key(),
        schematic,
    );
    let symbol = CellViewRef::new("work", "amp", "symbol");
    state.open_workspace_view(symbol.clone());
    state
        .store_active_symbol_document(&SymbolDocument {
            pins: vec![SymbolPin::new("IN", PortDirection::In, None)],
            ..SymbolDocument::default()
        })
        .expect("the fixture stores its symbol");
    symbol
}

/// Currency is the digest and nothing else, for every producer: each
/// owner's batch is current as published, and stale once an input its
/// digest covers moves. Walks every owner, so a new one cannot be added
/// without saying here what stales it.
#[test]
fn an_input_change_stales_a_published_batch_for_every_owner() {
    for owner in DiagnosticOwner::ALL {
        let mut state = AppState::default();
        let inputs = match owner {
            DiagnosticOwner::DesignRules => DiagnosticInputs::DesignCheck {
                subject: state.model.workspace.content.active_schematic_reference(),
            },
            DiagnosticOwner::SymbolPins => DiagnosticInputs::SymbolPins {
                subject: open_symbol_view(&mut state),
            },
            DiagnosticOwner::Dispatch => DiagnosticInputs::Dispatch,
            DiagnosticOwner::Run => {
                state.model.simulation.retained.runs =
                    vec![crate::state::SimulationRun::new(3)].into();
                DiagnosticInputs::Run { run_sequence: 3 }
            }
            DiagnosticOwner::RuleDeck => DiagnosticInputs::RuleDeck,
        };
        state
            .publish_diagnostics(inputs.clone(), Vec::new())
            .expect("an empty batch still records that the checks ran");
        let batch = state
            .model
            .diagnostics
            .batches()
            .find(|batch| batch.inputs == inputs)
            .expect("the batch was just published")
            .clone();
        assert!(
            state.diagnostic_batch_standing(&batch).is_current(),
            "{owner:?} is current as published"
        );

        match owner {
            DiagnosticOwner::DesignRules => {
                state
                    .model
                    .workspace
                    .content
                    .connectivity
                    .policy
                    .width_mismatch =
                    crate::state::BundleWidthMismatchPolicy::ExplicitSliceOrExtend;
            }
            DiagnosticOwner::SymbolPins => {
                // The artwork the pins are drawn on.
                let mut symbol = state
                    .load_active_symbol_document()
                    .expect("the fixture symbol loads");
                symbol.pins.push(crate::state::SymbolPin::new(
                    "OUT",
                    crate::state::PortDirection::Out,
                    None,
                ));
                state
                    .store_active_symbol_document(&symbol)
                    .expect("the edited symbol stores");
            }
            DiagnosticOwner::Dispatch => {
                let plan = state
                    .model
                    .sim_setup
                    .stable_analysis_plan_mut()
                    .expect("the fixture owns a stable plan");
                let instance = plan.instances()[0].id();
                plan.edit(instance, |_| ())
                    .expect("an analysis edit advances the plan revision");
            }
            DiagnosticOwner::Run => {
                // The configured design the failure's names were read off.
                state.model.schematic.add_component(
                    crate::state::ComponentType::Resistor,
                    crate::state::Point::new(0, 0),
                );
            }
            DiagnosticOwner::RuleDeck => {
                // A deck holding something this build does not apply.
                state.model.workspace.content.design_rules =
                    serde_json::from_str(r#"{"schema_version":2}"#)
                        .expect("a stored deck always reads");
            }
        }

        assert_eq!(
            state.diagnostic_batch_standing(&batch),
            BatchStanding::Stale,
            "{owner:?} stales when an input it was derived from moves"
        );
    }
}

fn test_record(state: &AppState, message: &str) -> DiagnosticRecord {
    DiagnosticRecord::new(
        CanonicalDiagnosticMetadata::current(
            "rspice.test",
            "TEST-1",
            "doc",
            None,
            0,
            "validation",
            message,
        ),
        DiagnosticLevel::Error,
        DiagnosticSubject::new(state.model.workspace.content.project.id(), "Node 'n1'"),
        message,
    )
}

/// A republication takes back the identities the batch it replaces held,
/// or the second publication of the same finding would collide with the
/// first and be refused.
#[test]
fn republishing_a_batch_releases_the_identities_it_replaces() {
    let mut state = AppState::default();
    let record = test_record(&state, "first");
    state
        .publish_diagnostics(DiagnosticInputs::Dispatch, vec![record.clone()])
        .expect("the dispatch digest is available");
    state
        .publish_diagnostics(DiagnosticInputs::Dispatch, vec![record])
        .expect("republishing the same finding is not a collision");
    assert_eq!(state.model.diagnostics.batches().count(), 1);
}

fn run_batches(state: &AppState) -> Vec<u64> {
    state
        .model
        .diagnostics
        .batches()
        .filter_map(|batch| match batch.inputs {
            DiagnosticInputs::Run { run_sequence } => Some(run_sequence),
            DiagnosticInputs::DesignCheck { .. }
            | DiagnosticInputs::LiveCheck { .. }
            | DiagnosticInputs::SymbolPins { .. }
            | DiagnosticInputs::Dispatch
            | DiagnosticInputs::CampaignMember { .. }
            | DiagnosticInputs::RuleDeck => None,
        })
        .collect()
}

/// The next run supersedes the last one's outcome. Nothing withdrew a
/// failed run's batch before, so after a re-run that succeeded the status
/// bar went on saying "Problems · 1 error", current, forever.
#[test]
fn a_dispatched_run_supersedes_the_failures_earlier_runs_reported() {
    let mut state = AppState::default();
    state.model.simulation.retained.runs = vec![
        crate::state::SimulationRun::new(1),
        crate::state::SimulationRun::new(2),
    ]
    .into();
    state.publish_run_failure("Transient", "Analysis failed: tran", None, 1);
    assert_eq!(run_batches(&state), [1]);
    let failed = state
        .model
        .diagnostics
        .batches()
        .flat_map(|batch| batch.records.iter())
        .map(|record| record.canonical.diagnostic_id)
        .next()
        .expect("the failure is published");
    state.model.diagnostics.set_current_finding(Some(failed));

    state.model.supersede_run_diagnostics(2);
    assert!(
        run_batches(&state).is_empty(),
        "the earlier run's failure stops standing when the next run is dispatched"
    );

    // The next run fails the same way: the same condition, standing
    // again, under the same identity — and still the current finding.
    state.publish_run_failure("Transient", "Analysis failed: tran again", None, 2);
    assert_eq!(run_batches(&state), [2]);
    assert_eq!(state.model.diagnostics.current_finding(), Some(failed));
    let republished = state
        .model
        .diagnostics
        .batches()
        .flat_map(|batch| batch.records.iter())
        .map(|record| record.canonical.diagnostic_id)
        .next();
    assert_eq!(republished, Some(failed));
}

/// A run with nothing retained has no outcome left to state.
#[test]
fn a_retired_run_takes_its_failure_with_it() {
    let mut state = AppState::default();
    state.model.simulation.retained.runs = vec![crate::state::SimulationRun::new(4)].into();
    state.publish_run_failure("Transient", "Analysis failed: tran", None, 4);
    state.model.withdraw_retired_run_diagnostics();
    assert_eq!(run_batches(&state), [4], "a retained run's failure stands");

    state.model.simulation.clear_runs();
    state.model.withdraw_retired_run_diagnostics();
    assert!(run_batches(&state).is_empty());
}

/// A campaign member refused at authorization is a finding about that
/// member, filed where its failures would have been had it run. It
/// supersedes what an earlier run reported, stands beside the members of
/// its own campaign, goes stale with the design, and gives way to the
/// next run outside the campaign. It used to reach only the Console.
#[test]
fn a_member_refused_at_authorization_stands_until_a_later_run_supersedes_it() {
    let mut state = AppState::default();
    let campaign = crate::product::SimulationCampaignId::new();
    let mut sibling = crate::state::SimulationRun::new(5);
    sibling
        .set_campaign_membership(
            crate::state::SimulationCampaignMembership::new(campaign, "Nightly", 2, 2)
                .expect("the fixture membership is valid"),
        )
        .expect("a fresh run joins its campaign");
    state.model.simulation.retained.runs = vec![
        crate::state::SimulationRun::new(4),
        sibling,
        crate::state::SimulationRun::new(6),
    ]
    .into();
    state.publish_run_failure("Transient", "Analysis failed: tran", None, 4);

    state.publish_campaign_member_refusal(
        campaign,
        1,
        "Slow corner",
        CampaignMemberRefusal::Authorization,
        "could not authorize member: permit generation exhausted",
    );
    assert_eq!(
        state.model.diagnostics.published_member_refusals(),
        [(
            "simulation-run/member-1".to_owned(),
            "Slow corner · campaign member 1".to_owned(),
            "The member could not be authorized: could not authorize member: permit \
             generation exhausted"
                .to_owned(),
        )],
    );
    assert!(
        run_batches(&state).is_empty(),
        "the campaign supersedes the outcome an earlier run reported"
    );
    let refusal = state
        .model
        .diagnostics
        .batches()
        .find(|batch| matches!(batch.inputs, DiagnosticInputs::CampaignMember { .. }))
        .expect("the refusal is filed")
        .clone();
    assert_eq!(refusal.records[0].level, DiagnosticLevel::Error);
    assert_eq!(&*refusal.records[0].key.code, "RUN-MEMBER-UNAUTHORIZED");
    assert!(state.diagnostic_batch_standing(&refusal).is_current());

    // The campaign's next member is dispatched beside it.
    state.model.supersede_run_diagnostics(5);
    assert_eq!(
        state.model.diagnostics.published_member_refusals().len(),
        1,
        "members of one campaign do not supersede each other"
    );

    // An edit to the configured design stales it, as it stales a run.
    state.model.schematic.add_component(
        crate::state::ComponentType::Resistor,
        crate::state::Point::new(0, 0),
    );
    assert_eq!(
        state.diagnostic_batch_standing(&refusal),
        BatchStanding::Stale
    );

    // The next run outside the campaign is a new measurement.
    state.model.supersede_run_diagnostics(6);
    assert!(
        state
            .model
            .diagnostics
            .published_member_refusals()
            .is_empty()
    );
}

/// A campaign run again whose member is refused again replaces the last
/// refusal of that member, under the same identity, rather than being
/// refused as a second finding with it.
#[test]
fn a_rerun_campaign_replaces_the_refusal_of_the_same_member() {
    let mut state = AppState::default();
    let refuse = |state: &mut AppState, campaign| {
        state.publish_campaign_member_refusal(
            campaign,
            2,
            "Slow corner",
            CampaignMemberRefusal::Start,
            "run sequence is exhausted",
        );
    };
    refuse(&mut state, crate::product::SimulationCampaignId::new());
    let first = state
        .model
        .diagnostics
        .batches()
        .flat_map(|batch| batch.records.iter())
        .map(|record| record.canonical.diagnostic_id)
        .next()
        .expect("the refusal is filed");
    state.model.diagnostics.set_current_finding(Some(first));
    let entries_before = state.model.log_buffer.entries().count();

    refuse(&mut state, crate::product::SimulationCampaignId::new());

    assert_eq!(state.model.diagnostics.batches().count(), 1);
    assert_eq!(state.model.diagnostics.current_finding(), Some(first));
    assert_eq!(
        state.model.log_buffer.entries().count(),
        entries_before,
        "the store refused nothing"
    );
}

/// A run failure names conductors in the configured design. Opening
/// another cell view moves nothing it was read off.
#[test]
fn a_run_failure_stays_current_while_another_view_is_open() {
    let mut state = AppState::default();
    state.model.simulation.retained.runs = vec![crate::state::SimulationRun::new(5)].into();
    state.publish_run_failure("Transient", "Analysis failed: tran", None, 5);
    let batch = state
        .model
        .diagnostics
        .batches()
        .next()
        .expect("the failure is published")
        .clone();
    assert!(state.diagnostic_batch_standing(&batch).is_current());

    let root = state.model.workspace.content.active_view.clone();
    state
        .model
        .workspace
        .insert_schematic_editor(root.key(), state.model.schematic.clone());
    state.model.workspace.content.active_view =
        crate::state::CellViewRef::new(&root.library, "elsewhere", &root.view);
    state.model.schematic = crate::state::SchematicState::default();
    state.model.schematic.add_component(
        crate::state::ComponentType::Resistor,
        crate::state::Point::new(0, 0),
    );

    assert!(
        state.diagnostic_batch_standing(&batch).is_current(),
        "the open view is not the design the run executed"
    );
}

fn run_batch(state: &AppState, run_sequence: u64) -> DiagnosticBatch {
    state
        .model
        .diagnostics
        .batch(&DiagnosticInputs::Run { run_sequence })
        .expect("the run's outcome is published")
        .clone()
}

/// A value edit is the commonest repair for a run that would not converge,
/// and the topology counter ignores it.
#[test]
fn a_value_edit_stales_a_run_failure() {
    let mut state = AppState::default();
    state.model.simulation.retained.runs = vec![crate::state::SimulationRun::new(6)].into();
    state.model.schematic.add_component(
        crate::state::ComponentType::Resistor,
        crate::state::Point::new(0, 0),
    );
    state.publish_run_failure("Transient", "Analysis failed: tran", None, 6);
    assert!(
        state
            .diagnostic_batch_standing(&run_batch(&state, 6))
            .is_current()
    );

    state.model.schematic.document_mut_for_test().components[0].value = "10k".to_owned();

    assert_eq!(
        state.diagnostic_batch_standing(&run_batch(&state, 6)),
        BatchStanding::Stale,
    );
}

/// A failure is about the design the run was dispatched with. An edit
/// made while it solved is not what it solved, and must not become the
/// baseline the failure is decided against.
#[test]
fn a_failure_is_decided_against_the_design_that_was_dispatched() {
    let mut state = AppState::default();
    state.model.simulation.retained.runs = vec![crate::state::SimulationRun::new(7)].into();
    state.model.schematic.add_component(
        crate::state::ComponentType::Resistor,
        crate::state::Point::new(0, 0),
    );
    state.open_run_outcome(7);
    assert!(
        state
            .model
            .diagnostics
            .batch(&DiagnosticInputs::Run { run_sequence: 7 })
            .is_none(),
        "a run that has failed nothing has nothing to list"
    );

    state.model.schematic.document_mut_for_test().components[0].value = "10k".to_owned();
    state.publish_run_failure("Transient", "Analysis failed: tran", None, 7);
    state.publish_run_failure("Operating point", "Analysis failed: op", None, 7);

    let batch = run_batch(&state, 7);
    assert_eq!(batch.records.len(), 2);
    assert_eq!(
        state.diagnostic_batch_standing(&batch),
        BatchStanding::Stale,
        "the design was edited after the run was dispatched"
    );
}

/// The store lists runs in the order they were made.
///
/// Two runs' outcomes stand together only where neither supersedes the
/// other, and a failure of one analysis in two plain runs is one condition
/// under one identity — so the two runs here fail different analyses.
#[test]
fn run_ten_is_filed_after_run_nine() {
    let mut state = AppState::default();
    state.model.simulation.retained.runs = vec![
        crate::state::SimulationRun::new(9),
        crate::state::SimulationRun::new(10),
    ]
    .into();
    state.publish_run_failure("Transient", "Analysis failed: tran", None, 10);
    state.publish_run_failure("Operating point", "Analysis failed: op", None, 9);
    assert_eq!(run_batches(&state), [9, 10]);
}

/// A failure of one analysis in two plain runs is one condition, and the
/// later run's outcome supersedes the earlier one's at dispatch. Were the
/// earlier batch still standing, the second would be refused rather than
/// listed twice.
#[test]
fn one_analysis_failing_in_two_plain_runs_is_one_condition() {
    let mut state = AppState::default();
    state.model.simulation.retained.runs = vec![
        crate::state::SimulationRun::new(9),
        crate::state::SimulationRun::new(10),
    ]
    .into();
    state.publish_run_failure("Transient", "Analysis failed: tran", None, 9);
    state.model.supersede_run_diagnostics(10);
    state.open_run_outcome(10);
    state.publish_run_failure("Transient", "Analysis failed: tran", None, 10);
    assert_eq!(run_batches(&state), [10]);
}

/// Receipts are filed under a cell view's canonical key; the batches they
/// describe must be too, or one view checked under two spellings lists
/// every finding twice.
#[test]
fn two_spellings_of_one_cell_view_are_one_batch() {
    let mut state = AppState::default();
    let subject = state.model.workspace.content.active_schematic_reference();
    let respelled = crate::state::CellViewRef::new(
        subject.library.to_uppercase(),
        subject.cell.to_uppercase(),
        &subject.view,
    );
    for subject in [subject, respelled.clone()] {
        state
            .publish_diagnostics(DiagnosticInputs::DesignCheck { subject }, Vec::new())
            .expect("the design-check digest is available");
    }
    assert_eq!(state.model.diagnostics.batches().count(), 1);
    assert!(
        state
            .model
            .diagnostics
            .batch(&DiagnosticInputs::DesignCheck { subject: respelled })
            .is_some()
    );
}

/// Two conditions that mint one identity would shadow each other, so the
/// publication is refused instead and the store is left as it was.
#[test]
fn a_producer_that_mints_one_identity_twice_is_refused() {
    let mut state = AppState::default();
    let kept = test_record(&state, "kept");
    state
        .publish_diagnostics(DiagnosticInputs::Dispatch, vec![kept])
        .expect("the dispatch digest is available");

    let colliding = test_record(&state, "collides");
    let error = state
        .publish_diagnostics(
            DiagnosticInputs::Dispatch,
            vec![colliding.clone(), colliding],
        )
        .expect_err("two records with one identity must be refused");

    assert!(error.contains("duplicate diagnostic ID"), "{error}");
    let batch = state
        .model
        .diagnostics
        .batches()
        .next()
        .expect("the refused publication leaves the previous batch in place");
    assert_eq!(batch.records.len(), 1);
    assert_eq!(batch.records[0].message, "kept");
}
