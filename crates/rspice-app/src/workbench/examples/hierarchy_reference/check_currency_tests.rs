//! When a check's findings stop describing the design, and what it costs to
//! ask.
//!
//! A check's evidence covers exactly what the check read, so an edit anywhere
//! else leaves it current; and asking whether it is current costs nothing
//! while nothing it read has moved, so a frame with no edit digests nothing.

use crate::state::{CellViewRef, Library, Point, ViewType};
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::design_checks::DesignCheckStatus;
use crate::workbench::app_state::diagnostic_store::digest_memo::{
    computed_digests, reset_computed_digests,
};
use crate::workbench::app_state::diagnostic_store::listing;
use crate::workbench::examples::hierarchy_reference::{self, AMP_INSTANCES, HierarchyReference};
use rspice_design::connectivity_contract::GlobalAliasComparisonPolicy;
use rspice_design::projection::{document_digest_counts, reset_document_digest_counts};

/// The reference hierarchy, with a spare schematic no instance names, checked
/// from its configured root.
fn checked_reference() -> (HierarchyReference, CellViewRef) {
    let mut reference = hierarchy_reference::build();
    assert!(
        reference
            .state
            .workspace
            .content
            .active_view
            .key()
            .eq_ignore_ascii_case(&reference.top.key()),
        "precondition: the root is the view in front of the reader"
    );
    let spare = CellViewRef::new(&reference.top.library, "spare", "schematic");
    reference.state.workspace.content.schematic_buffers.insert(
        spare.key(),
        rspice_design::schematic::owned::Schematic::default(),
    );
    reference
        .state
        .run_active_design_checks()
        .expect("the reference hierarchy checks");
    assert!(is_current(&reference.state, &reference.top));
    (reference, spare)
}

fn is_current(state: &AppState, subject: &CellViewRef) -> bool {
    match state.design_check_status(subject) {
        DesignCheckStatus::Current(_) => true,
        DesignCheckStatus::Stale(_) => false,
        DesignCheckStatus::NotRun => panic!("{} was never checked", subject.display_path()),
        DesignCheckStatus::Unavailable { reason, .. } => {
            panic!("{} cannot be judged: {reason}", subject.display_path())
        }
    }
}

/// Edit a stored cellview's content, as any edit of its drawing does.
fn edit_stored(state: &mut AppState, cell_view: &CellViewRef) {
    state
        .workspace
        .content
        .schematic_buffers
        .get_mut(&cell_view.key())
        .expect("the cellview is stored")
        .document_mut_for_test()
        .grid_size += 10;
}

fn reset_counts() {
    reset_computed_digests();
    reset_document_digest_counts();
}

#[test]
fn an_edit_to_a_cellview_the_check_never_read_leaves_its_findings_current() {
    let (mut reference, spare) = checked_reference();
    edit_stored(&mut reference.state, &spare);
    assert!(
        is_current(&reference.state, &reference.top),
        "nothing in the root's hierarchy instantiates the spare cell"
    );
}

#[test]
fn an_edit_to_a_cellview_the_check_instantiates_stales_its_findings() {
    let (mut reference, _) = checked_reference();
    let amp = reference.amp_schematic.clone();
    edit_stored(&mut reference.state, &amp);
    assert!(!is_current(&reference.state, &reference.top));
}

/// Point the configuration's one override, on the second amplifier, at
/// `views`.
fn rebind_second_amplifier(reference: &mut HierarchyReference, views: &[&str]) {
    let state = &mut reference.state;
    let mut catalog = (*state.workspace.content.configuration_sets).clone();
    let configuration = catalog
        .find(reference.configuration)
        .expect("the reference configuration")
        .clone();
    let mut definition = configuration.definition().clone();
    definition.overrides[0].executable_views =
        views.iter().map(|view| (*view).to_owned()).collect();
    catalog
        .update(
            reference.configuration,
            configuration.revision(),
            definition,
        )
        .expect("the override is well formed");
    state
        .workspace
        .content
        .replace_configuration_sets(catalog)
        .expect("the rebinding publishes");
}

#[test]
fn rebinding_a_configuration_entry_the_check_followed_stales_its_findings() {
    let mut reference = hierarchy_reference::build();
    // A second schematic view of the amplifier, for the configuration to
    // choose; provisioned before the check, so the check's evidence covers it.
    let amp = reference.amp_schematic.clone();
    let alternate = CellViewRef::new(&amp.library, &amp.cell, "alternate");
    reference
        .state
        .library_manager
        .edit_library(&amp.library)
        .expect("the amplifier's library")
        .add_view(
            &amp.cell,
            crate::state::View::new("alternate", ViewType::Schematic),
        )
        .expect("the amplifier cell");
    let copy = reference.state.workspace.content.schematic_buffers[&amp.key()].clone();
    reference
        .state
        .workspace
        .content
        .schematic_buffers
        .insert(alternate.key(), copy);
    reference
        .state
        .run_active_design_checks()
        .expect("the reference hierarchy checks");
    assert!(is_current(&reference.state, &reference.top));

    rebind_second_amplifier(&mut reference, &["alternate"]);
    assert!(
        !is_current(&reference.state, &reference.top),
        "{} is bound to another view than the one checked",
        HierarchyReference::instance_path(AMP_INSTANCES[1])
    );
}

#[test]
fn a_frame_with_no_edit_digests_nothing() {
    let (mut reference, _) = checked_reference();
    let state = &mut reference.state;
    let top = reference.top.clone();
    reset_counts();
    for pass in 1..=3 {
        let listed = listing::listing_in_pass(state, pass);
        assert_eq!(listed.counts().stale, 0);
        assert!(is_current(state, &top));
        state.refresh_active_design_check_projection();
    }
    assert_eq!(computed_digests(), 0, "no batch digest was computed");
    assert_eq!(
        document_digest_counts(),
        (0, 0),
        "no stored or projected document was digested"
    );
}

#[test]
fn an_edit_digests_the_one_cellview_it_touched() {
    let (mut reference, _) = checked_reference();
    let amp = reference.amp_schematic.clone();
    edit_stored(&mut reference.state, &amp);
    reset_counts();
    let listed = listing::listing_in_pass(&reference.state, 1);
    assert_eq!(
        listed.counts().stale,
        listed.len(),
        "the root's findings moved"
    );
    assert_eq!(computed_digests(), 1, "the root's one batch");
    assert_eq!(
        document_digest_counts(),
        (1, 1),
        "the edited cellview, stored and projected, and nothing else"
    );
}

/// Every input a check's digest is remembered by moves when it is changed
/// through any of the ways the application changes it, and the next reader
/// digests again rather than reading the remembered answer.
#[test]
fn every_input_the_memo_is_keyed_by_moves_on_every_way_it_is_changed() {
    type Mutation = fn(&mut HierarchyReference);
    let mutations: [(&str, Mutation); 9] = [
        (
            "an edit to the drawing in front of the reader",
            |reference| {
                reference.state.schematic.add_component(
                    crate::state::ComponentType::Resistor,
                    Point::new(4000, 4000),
                );
            },
        ),
        ("an undo", |reference| {
            reference.state.schematic.add_component(
                crate::state::ComponentType::Resistor,
                Point::new(4000, 4000),
            );
            let _ = listing::listing_in_pass(&reference.state, 90);
            assert!(reference.state.schematic.undo(), "the edit is undone");
        }),
        ("an edit to a stored cellview", |reference| {
            let amp = reference.amp_schematic.clone();
            edit_stored(&mut reference.state, &amp);
        }),
        ("a library added", |reference| {
            reference
                .state
                .library_manager
                .add_library(Library::new("scratch"));
        }),
        ("the connectivity contract", |reference| {
            reference
                .state
                .workspace
                .content
                .connectivity
                .policy
                .alias_comparison = GlobalAliasComparisonPolicy::DialectCompatibility;
        }),
        ("design management, edited in place", |reference| {
            let management = &mut reference.state.workspace.content.design_management;
            let mut settings = management.drawing_sheet_settings().clone();
            settings.title_block_field_values.insert(
                crate::state::DrawingSheetTitleFieldId::Organization,
                "Edited".to_owned(),
            );
            let revision = management.revision();
            management
                .update_drawing_sheet_settings(revision, settings)
                .expect("the settings are valid");
        }),
        ("design management, replaced", |reference| {
            let mut candidate = (*reference.state.workspace.content.design_management).clone();
            let mut settings = candidate.drawing_sheet_settings().clone();
            settings.title_block_field_values.insert(
                crate::state::DrawingSheetTitleFieldId::Organization,
                "Replaced".to_owned(),
            );
            let revision = candidate.revision();
            candidate
                .update_drawing_sheet_settings(revision, settings)
                .expect("the settings are valid");
            reference
                .state
                .workspace
                .content
                .replace_design_management(candidate)
                .expect("the candidate publishes");
        }),
        ("the configuration sets", |reference| {
            rebind_second_amplifier(reference, &["schematic", "symbol"]);
        }),
        ("the rule deck, even unchanged", |reference| {
            let _ = &mut *reference.state.workspace.content.design_rules;
        }),
    ];
    for (name, mutate) in mutations {
        let (mut reference, _) = checked_reference();
        let _ = listing::listing_in_pass(&reference.state, 1);
        mutate(&mut reference);
        reset_counts();
        let _ = listing::listing_in_pass(&reference.state, 2);
        assert_eq!(
            computed_digests(),
            1,
            "{name}: the root's batch is digested again"
        );
    }
}
