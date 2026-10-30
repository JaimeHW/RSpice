//! `workbench/app_state/design_checks` observed through a whole session.

use crate::state::CellViewRef;
use crate::state::ViewType;
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::design_checks::*;
use crate::workbench::app_state::diagnostic_store::DiagnosticInputs;
use crate::workbench::app_state::diagnostic_store::DiagnosticRecord;
use crate::workbench::app_state::diagnostic_store::DiagnosticSubject;
use crate::workbench::app_state::diagnostic_store::FindingKey;
use crate::workbench::app_state::diagnostic_store::mint_canonical;
use rspice_design::drc::DrcLocation;
use rspice_design::drc::DrcResult;
use rspice_design::drc::DrcViolationType;

fn completed_result() -> DrcResult {
    let mut result = DrcResult::new();
    result.completed = true;
    result
}

fn open_pin(component_id: u64, component: &str, pin: &str) -> DrcLocation {
    DrcLocation::Terminal {
        component_id,
        component_name: component.to_owned(),
        pin_name: pin.to_owned(),
        point: crate::state::Point::new(0, 0),
    }
}

/// Publish one completed pass of the active view's checks, in the order
/// given — which is the order the checks would have reported them in.
fn publish_pass(state: &mut AppState, findings: &[(DrcLocation, &str)]) {
    let mut result = DrcResult::new();
    for (ordinal, (location, message)) in findings.iter().enumerate() {
        result.add_violation(rspice_design::drc::DrcViolation::new(
            ordinal + 1,
            rspice_design::drc::DrcViolationType::UnconnectedPin,
            *message,
            location.clone(),
        ));
    }
    result.completed = true;
    state
        .publish_active_design_check_result(result)
        .expect("the pass publishes");
}

/// The identity the published batch gives the finding about `object`.
fn identity_of(state: &AppState, object: &str) -> uuid::Uuid {
    let subject = state.model.workspace.content.active_schematic_reference();
    let published = DiagnosticInputs::DesignCheck { subject };
    state
        .model
        .diagnostics
        .batches()
        .find(|batch| batch.inputs == published)
        .and_then(|batch| {
            batch
                .records
                .iter()
                .find(|record| record.subject.object == object)
        })
        .map(|record| record.canonical.diagnostic_id)
        .unwrap_or_else(|| panic!("a finding about {object} is published"))
}

/// An edit elsewhere moves everything an identity used to be minted
/// from — the project revision, the finding's ordinal, its wording — and
/// none of it is what the finding is about.
#[test]
fn an_unrelated_edit_and_a_recheck_keep_the_current_finding() {
    let mut state = AppState::default();
    publish_pass(
        &mut state,
        &[
            (open_pin(4, "R1", "A"), "Pin R1.A is not connected"),
            (open_pin(7, "R2", "B"), "Pin R2.B is not connected"),
        ],
    );
    let standing = identity_of(&state, "Pin R2.B");
    state.model.diagnostics.set_current_finding(Some(standing));

    state
        .model
        .workspace
        .content
        .project
        .advance_revision()
        .expect("the fixture revision advances");
    publish_pass(
        &mut state,
        &[
            (open_pin(2, "C1", "A"), "Pin C1.A is not connected"),
            (open_pin(4, "R1", "A"), "Pin R1.A is not connected"),
            (open_pin(7, "R2", "B"), "Pin R2.B has nothing attached"),
        ],
    );

    assert_eq!(identity_of(&state, "Pin R2.B"), standing);
    assert_eq!(
        state.model.diagnostics.current_finding(),
        Some(standing),
        "the shared current finding survives the re-check",
    );
}

/// A finding re-reported in other words — the rule's message reworded,
/// or a value it quotes changed — is the same finding. Its identity is
/// the rule, the cell view and the object, and nothing the message says
/// is part of it, so the current selection stays on it and so does a
/// waiver recorded against it. An identity hashed from the message, as
/// the source editors' own identities are, would have re-keyed it.
#[test]
fn rewording_a_finding_keeps_its_identity_its_selection_and_its_waiver() {
    let mut state = AppState::default();
    publish_pass(
        &mut state,
        &[
            (open_pin(4, "R1", "A"), "Pin R1.A is not connected"),
            (open_pin(7, "R2", "B"), "Pin R2.B is not connected"),
        ],
    );
    let identity = identity_of(&state, "Pin R2.B");
    state.model.diagnostics.set_current_finding(Some(identity));
    // A waiver is recorded against a finding by what the finding is.
    let subject = state.model.workspace.content.active_schematic_reference();
    let waived = |state: &AppState| {
        state
            .model
            .diagnostics
            .batch(&DiagnosticInputs::DesignCheck {
                subject: subject.clone(),
            })
            .and_then(|batch| {
                batch
                    .records
                    .iter()
                    .find(|record| record.canonical.diagnostic_id == identity)
            })
            .map(|record| (record.key.clone(), record.message.clone()))
    };
    let (waiver_key, _) = waived(&state).expect("the finding is published");

    publish_pass(
        &mut state,
        &[
            (open_pin(4, "R1", "A"), "Pin R1.A is not connected"),
            (
                open_pin(7, "R2", "B"),
                "Terminal B of R2 has no connection (2 nets within 10 grid units)",
            ),
        ],
    );

    assert_eq!(identity_of(&state, "Pin R2.B"), identity);
    assert_eq!(
        state.model.diagnostics.current_finding(),
        Some(identity),
        "the current selection stays on the reworded finding",
    );
    let (key, message) = waived(&state).expect("the waived identity still resolves");
    assert_eq!(key, waiver_key, "what a waiver names is unchanged");
    assert_eq!(
        message, "Terminal B of R2 has no connection (2 nets within 10 grid units)",
        "the finding states its new words under its old identity",
    );
}

/// Fixing one finding takes its identity away with it. With an ordinal in
/// the identity, the next finding of the same rule inherited it — and a
/// selection, or a waiver, on the fixed finding moved to that one.
#[test]
fn fixing_one_finding_does_not_hand_its_identity_to_another() {
    let mut state = AppState::default();
    publish_pass(
        &mut state,
        &[
            (open_pin(4, "R1", "A"), "Pin R1.A is not connected"),
            (open_pin(7, "R2", "B"), "Pin R2.B is not connected"),
        ],
    );
    let fixed = identity_of(&state, "Pin R1.A");
    let remaining = identity_of(&state, "Pin R2.B");
    state.model.diagnostics.set_current_finding(Some(fixed));

    publish_pass(
        &mut state,
        &[(open_pin(7, "R2", "B"), "Pin R2.B is not connected")],
    );

    let subject = state.model.workspace.content.active_schematic_reference();
    assert_eq!(
        state.model.design_rule_finding_identities(&subject),
        vec![remaining],
        "the remaining finding keeps its identity and the fixed one's is gone",
    );
    assert_ne!(
        state.model.diagnostics.current_finding(),
        Some(remaining),
        "a selection on the fixed finding does not move to another finding",
    );
}

/// Two findings about one object are told apart by the order they were
/// reported in. When the first goes, the second takes its occurrence —
/// and a selection on the second follows it there rather than being lost.
#[test]
fn a_selection_follows_its_finding_when_its_occurrence_moves() {
    let mut state = AppState::default();
    publish_pass(
        &mut state,
        &[
            (open_pin(4, "R1", "A"), "first report"),
            (open_pin(4, "R1", "A"), "second report"),
        ],
    );
    let subject = state.model.workspace.content.active_schematic_reference();
    let identities = state.model.design_rule_finding_identities(&subject);
    assert_eq!(identities.len(), 2);
    assert_ne!(identities[0], identities[1]);
    state
        .model
        .diagnostics
        .set_current_finding(Some(identities[1]));

    publish_pass(&mut state, &[(open_pin(4, "R1", "A"), "second report")]);

    let remaining = state.model.design_rule_finding_identities(&subject);
    assert_eq!(remaining, vec![identities[0]]);
    assert_eq!(
        state.model.diagnostics.current_finding(),
        Some(remaining[0])
    );
}

/// The store can refuse a batch, so a receipt is stored only once the
/// store has taken the findings it describes. Stored first, it put
/// findings on the canvas and the check pill that the page did not list.
#[test]
fn a_refused_publication_stores_no_receipt() {
    let mut state = AppState::default();
    let subject = state.model.workspace.content.active_schematic_reference();
    let location = open_pin(4, "R1", "A");
    // A record that already holds the identity this pass will mint.
    let key = FindingKey::new(
        DESIGN_RULE_PRODUCER,
        rspice_design::drc::DrcViolationType::UnconnectedPin.rule_code(),
        subject.key(),
        location_key(&location),
    );
    let squatter = DiagnosticRecord::new(
        mint_canonical(&[key], 0, "elsewhere").remove(0),
        crate::workbench::app_state::diagnostic_store::DiagnosticLevel::Error,
        DiagnosticSubject::new(state.model.workspace.content.project.id(), "elsewhere"),
        "elsewhere",
    );
    state
        .publish_diagnostics(DiagnosticInputs::Dispatch, vec![squatter])
        .expect("the squatting record publishes");

    let mut result = DrcResult::new();
    result.add_violation(rspice_design::drc::DrcViolation::new(
        1,
        rspice_design::drc::DrcViolationType::UnconnectedPin,
        "Pin R1.A is not connected",
        location,
    ));
    result.completed = true;
    let error = state
        .publish_active_design_check_result(result)
        .expect_err("the store refuses a second holder of one identity");

    assert!(error.contains("duplicate diagnostic ID"), "{error}");
    assert!(
        matches!(
            state.active_design_check_status(),
            DesignCheckStatus::NotRun
        ),
        "a refused publication leaves no receipt behind"
    );
    assert!(state.model.design_checks.drc_results.is_none());
}

#[test]
fn receipts_never_alias_another_active_cell_view() {
    let mut state = AppState::default();
    let root = state.model.workspace.content.active_schematic_reference();
    let other = CellViewRef::new(&root.library, "aux", &root.view);
    state
        .model
        .workspace
        .insert_schematic_editor(other.key(), crate::state::SchematicState::default());
    state
        .publish_active_design_check_result(completed_result())
        .expect("publish root receipt");

    state
        .model
        .workspace
        .insert_schematic_editor(root.key(), state.model.schematic.clone());
    state.model.workspace.content.active_view = other;
    state.model.schematic = crate::state::SchematicState::default();
    state.refresh_active_design_check_projection();

    assert!(matches!(
        state.active_design_check_status(),
        DesignCheckStatus::NotRun
    ));
    assert!(state.model.design_checks.drc_results.is_none());
    assert_eq!(state.model.design_checks.drc_checked_version, 0);
    assert!(matches!(
        state.design_check_status(&root),
        DesignCheckStatus::Current(_)
    ));
}

#[test]
fn publishing_an_active_receipt_refreshes_the_canvas_projection() {
    let mut state = AppState::default();
    let result = completed_result();

    state
        .publish_active_design_check_result(result)
        .expect("publish active receipt");

    assert!(
        state
            .model
            .design_checks
            .drc_results
            .as_ref()
            .is_some_and(|projected| projected.completed)
    );
    assert_eq!(
        state.model.design_checks.drc_checked_version,
        state.model.schematic.topology_version()
    );
}

#[test]
fn connectivity_changes_stale_previously_current_receipts() {
    let mut state = AppState::default();
    state
        .publish_active_design_check_result(completed_result())
        .expect("publish current receipt");
    assert!(matches!(
        state.active_design_check_status(),
        DesignCheckStatus::Current(_)
    ));

    state
        .model
        .workspace
        .content
        .connectivity
        .policy
        .width_mismatch = crate::state::BundleWidthMismatchPolicy::ExplicitSliceOrExtend;

    assert!(matches!(
        state.active_design_check_status(),
        DesignCheckStatus::Stale(_)
    ));
}

#[test]
fn incomplete_results_can_never_become_current_evidence() {
    let mut state = AppState::default();

    let error = state
        .publish_active_design_check_result(DrcResult::new())
        .expect_err("incomplete result must be rejected");

    assert!(error.contains("incomplete"));
    assert!(matches!(
        state.active_design_check_status(),
        DesignCheckStatus::NotRun
    ));
    assert!(state.model.design_checks.drc_results.is_none());
}

#[test]
fn focusing_a_non_schematic_view_does_not_stale_root_evidence() {
    let mut state = AppState::default();
    let root = state.model.workspace.content.active_view.clone();
    state
        .publish_active_design_check_result(completed_result())
        .expect("publish root receipt");
    state
        .model
        .workspace
        .insert_schematic_editor(root.key(), state.model.schematic.clone());
    let layout = CellViewRef::new(&root.library, &root.cell, "layout");
    state
        .model
        .workspace
        .content
        .open_views
        .push(crate::state::OpenCellView::new(
            layout.clone(),
            ViewType::Layout,
        ));
    state.model.workspace.content.active_view = layout;
    state.model.schematic = crate::state::SchematicState::default();

    assert!(matches!(
        state.design_check_status(&root),
        DesignCheckStatus::Current(_)
    ));
}

#[test]
fn symbol_focus_never_aliases_or_clears_its_sibling_schematic_receipt() {
    let mut state = AppState::default();
    let root = state.model.workspace.content.active_view.clone();
    state
        .publish_active_design_check_result(completed_result())
        .expect("publish root receipt");
    let symbol = CellViewRef::new(&root.library, &root.cell, "symbol");
    state
        .model
        .workspace
        .content
        .open_views
        .push(crate::state::OpenCellView::new(
            symbol.clone(),
            ViewType::Symbol,
        ));
    state.model.workspace.content.active_view = symbol;

    state.refresh_active_design_check_projection();

    assert!(matches!(
        state.active_design_check_status(),
        DesignCheckStatus::NotRun
    ));
    assert!(state.model.design_checks.drc_results.is_none());
    assert!(
        state
            .publish_active_design_check_result(completed_result())
            .is_err()
    );
    state.clear_active_design_check();
    assert!(matches!(
        state.design_check_status(&root),
        DesignCheckStatus::Current(_)
    ));
}

/// Two electrically separate conductors drawn at exactly the same
/// coordinates on two governed sheets.
///
/// The editor buffer holds one coordinate space, so anything read off it
/// merges the two pages into one net. The projection namespaces the pages
/// apart before anything electrical is read, which is the whole reason
/// design checks must not read the buffer.
fn state_with_two_coincident_sheets() -> AppState {
    const FIRST_WIRE: u64 = 101;
    const SECOND_WIRE: u64 = 102;

    let mut state = AppState::default();
    state
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(crate::state::Wire::segment(
            FIRST_WIRE,
            crate::state::Point::new(0, 0),
            crate::state::Point::new(40, 0),
        ));
    state
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(crate::state::Wire::segment(
            SECOND_WIRE,
            crate::state::Point::new(0, 0),
            crate::state::Point::new(40, 0),
        ));
    state.sync_active_schematic_to_workspace();

    let key = state
        .model
        .workspace
        .content
        .active_schematic_reference()
        .key();
    let first = state
        .model
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&key, "Page 1", [FIRST_WIRE])
        .expect("a fresh cell view accepts its first governed sheet");
    let catalog = state
        .model
        .workspace
        .content
        .design_management
        .sheet_catalog_mut(&key)
        .expect("the sheet catalog was just created");
    let second = catalog
        .create_sheet(
            crate::state::SheetDefinition {
                name: "Page 2".to_owned(),
                template: crate::state::SheetTemplate::AnalogSchematic,
                port_policy: crate::state::SheetPortPolicy::TypedOffSheetPorts,
                explicit_page_number: Some(2),
            },
            Some(first),
        )
        .expect("a second governed sheet inserts after the first");
    catalog
        .assign_objects(catalog.revision(), second, [SECOND_WIRE])
        .expect("the second conductor belongs to the second page");
    state
}

#[test]
fn design_checks_read_the_projection_so_coincident_pages_stay_two_nets() {
    let mut state = state_with_two_coincident_sheets();
    assert_eq!(
        rspice_design::connectivity::summary::design_nets(&state.model.schematic).len(),
        1,
        "the editor buffer holds one coordinate space, so its pages overlap"
    );

    let projection = state
        .model
        .workspace
        .design_projection(
            &state.model.library_manager,
            &state.model.workspace.content.active_view,
            &state.model.schematic,
        )
        .expect("the fixture configuration resolves");
    assert_eq!(
        rspice_design::connectivity::summary::projection_nets(
            state.model.library_manager.catalog(),
            &projection,
            &state.model.workspace.content.active_view.key(),
        )
        .len(),
        2,
        "the projection namespaces the two pages apart"
    );

    let result = state
        .run_active_design_checks()
        .expect("the checks run over the projected design");
    assert!(result.completed);
}

#[test]
fn design_checks_state_an_unresolved_configuration_rather_than_checking_the_buffer() {
    let mut state = AppState::default();
    let root = state.model.workspace.content.active_view.clone();
    state
        .model
        .workspace
        .content
        .configuration_sets
        .create(crate::state::ConfigurationSetDefinition {
            name: "Unresolvable DUT".to_owned(),
            root,
            dut_path: "/XABSENT".to_owned(),
            executable_view_policy: vec!["schematic".to_owned()],
            stop_views: Vec::new(),
            unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy:
                crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: Vec::new(),
            cell_bindings: Vec::new(),
            model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
            owner: "projection consumer test".to_owned(),
        })
        .expect("the fixture configuration is well formed");

    let reason = state
        .run_active_design_checks()
        .expect_err("an unresolvable configuration cannot be checked");
    assert!(reason.contains("XABSENT"), "{reason}");
    assert!(
        matches!(
            state.active_design_check_status(),
            DesignCheckStatus::NotRun
        ),
        "a refused check publishes no evidence"
    );
}

/// What else a finding involves is how it is shown, not what it is: a
/// loop that gains, loses or reorders a source is the same finding, so a
/// selection, a waiver's match and the page's current row all stay on it.
#[test]
fn a_findings_identity_is_unmoved_by_the_objects_it_involves() {
    use rspice_design::drc::{DrcRelated, DrcRelation, DrcViolation};

    let subject = CellViewRef::new("work", "top", "schematic");
    let source = |id: u64| {
        DrcRelated::new(
            DrcLocation::Component {
                id,
                name: format!("V{id}"),
            },
            DrcRelation::LoopSource,
        )
    };
    let pass = |related: Vec<DrcRelated>| {
        let mut result = completed_result();
        result.add_violation(
            DrcViolation::new(
                1,
                DrcViolationType::SourceToSource,
                "a loop of voltage sources",
                DrcLocation::Node {
                    net_name: "a".to_owned(),
                    terminals: Vec::new(),
                },
            )
            .with_related(related),
        );
        result
    };

    let alone = pass(Vec::new());
    let identity = finding_identities(&subject, &alone);
    let key = finding_keys(&subject, &alone);
    for related in [
        vec![source(1)],
        vec![source(1), source(2), source(3)],
        vec![source(3), source(1), source(2)],
    ] {
        let involved = pass(related);
        assert_eq!(finding_identities(&subject, &involved), identity);
        assert_eq!(finding_keys(&subject, &involved), key);
    }
}
