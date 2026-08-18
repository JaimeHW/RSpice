//! Guards for the design projection memo: what must rebuild it, what must
//! not, and how much of the design one edit is allowed to re-materialize.

use super::*;

use crate::workbench::app_state::AppState;
use crate::workbench::examples::hierarchy_reference;

fn projection_of(state: &AppState) -> Arc<DesignProjection> {
    state
        .workspace
        .design_projection(
            &state.library_manager,
            &state.workspace.active_view,
            &state.schematic,
        )
        .expect("the reference project resolves into a design projection")
}

fn edit_active_value(state: &mut AppState, value: &str) {
    let committed = state.schematic.with_undo("edit value", |schematic| {
        let component = schematic
            .components
            .iter_mut()
            .find(|component| component.kind == crate::state::ComponentType::VoltageSource)
            .expect("the reference top sheet owns a source");
        component.value = value.to_owned();
    });
    assert!(committed, "a value edit must commit an undo entry");
}

/// Edit a buffer that is not the active sheet. The resistor is deliberately
/// the subject: the amp's ports are components too, and renaming one would
/// change the cell's interface rather than only its content.
fn edit_amp_resistor(state: &mut AppState, amp_key: &str, value: &str) {
    let committed = state
        .workspace
        .schematic_buffers
        .get_mut(amp_key)
        .expect("the reference project persists the amp schematic")
        .with_undo("edit amp value", |schematic| {
            schematic
                .components
                .iter_mut()
                .find(|component| component.kind == crate::state::ComponentType::Resistor)
                .expect("the amp sheet owns a resistor")
                .value = value.to_owned();
        });
    assert!(committed, "a value edit must commit an undo entry");
}

#[test]
fn projection_is_not_rebuilt_without_a_key_change() {
    let mut reference = hierarchy_reference::build();
    let amp_key = reference.amp_schematic.key();
    let state = &mut reference.state;

    let first = projection_of(state);
    assert!(
        Arc::ptr_eq(&first, &projection_of(state)),
        "two calls with nothing changed must share one projection"
    );

    state.workspace.netlist_source_dirty = true;
    assert!(
        Arc::ptr_eq(&first, &projection_of(state)),
        "a workspace field the projection never reads must not rebuild it"
    );

    edit_active_value(state, "2");
    let after_active_edit = projection_of(state);
    assert!(
        !Arc::ptr_eq(&first, &after_active_edit),
        "a value edit on the active sheet must rebuild the projection"
    );

    edit_amp_resistor(state, &amp_key, "9k");
    assert!(
        !Arc::ptr_eq(&after_active_edit, &projection_of(state)),
        "an edit to a buffer that is not the active sheet must rebuild the projection"
    );
}

#[test]
fn projection_names_follow_a_property_rename() {
    let mut reference = hierarchy_reference::build();
    let state = &mut reference.state;

    let before = projection_of(state);
    // The filter instance is the subject because the configuration names the
    // amp instances as its DUT and its one override: renaming those would
    // unresolve the design instead of exercising the memo.
    let original = before
        .root_schematic()
        .expect("the projection carries the root schematic")
        .components
        .iter()
        .find(|component| component.name == hierarchy_reference::FILTER_INSTANCES[0])
        .map(|component| component.id)
        .expect("the reference top sheet places the first filter instance");
    let topology_before = state.schematic.topology_version();

    let committed = state.schematic.with_undo("rename instance", |schematic| {
        schematic
            .components
            .iter_mut()
            .find(|component| component.id == original)
            .expect("the renamed instance is retained")
            .name = "XFILT".to_owned();
    });
    assert!(committed, "a rename must commit an undo entry");
    assert_eq!(
        state.schematic.topology_version(),
        topology_before,
        "a rename is deliberately invisible to the topology version, which is \
         why the projection key cannot be built from it"
    );

    let after = projection_of(state);
    assert!(
        !Arc::ptr_eq(&before, &after),
        "a rename must rebuild the projection"
    );
    let names = after
        .root_schematic()
        .expect("the projection carries the root schematic")
        .components
        .iter()
        .map(|component| component.name.as_str())
        .collect::<Vec<_>>();
    assert!(
        names.contains(&"XFILT"),
        "the projection must carry the new name: {names:?}"
    );
    assert!(
        !names.contains(&hierarchy_reference::FILTER_INSTANCES[0]),
        "the projection must not carry the old name: {names:?}"
    );
}

#[test]
fn one_edit_rematerializes_one_cell_view() {
    let mut reference = hierarchy_reference::build();
    let state = &mut reference.state;
    let cell_views = projection_of(state).schematic_buffers().len();
    assert!(
        cell_views > 1,
        "the reference project must own more than one cell view"
    );

    reset_materialization_count();
    edit_active_value(state, "3");
    let _ = projection_of(state);
    assert_eq!(
        materialization_count(),
        1,
        "one edit must re-materialize exactly one of {cell_views} cell views"
    );

    reset_materialization_count();
    let _ = projection_of(state);
    assert_eq!(
        materialization_count(),
        0,
        "a projection that is still current must not materialize anything"
    );
}

#[test]
fn a_memo_hit_carries_what_a_full_rebuild_would_have_produced() {
    let mut reference = hierarchy_reference::build();
    let amp_key = reference.amp_schematic.key();
    let state = &mut reference.state;

    for step in 0..4 {
        match step {
            0 => edit_active_value(state, "5"),
            1 => edit_amp_resistor(state, &amp_key, "7k"),
            2 => {
                state.workspace.connectivity.policy.global_promotion =
                    crate::state::GlobalNetPromotionPolicy::TechnologyDefinedOnly;
            }
            _ => {
                state.workspace.schematic_buffers.insert(
                    "user/spare/schematic".to_owned(),
                    crate::state::SchematicState::default(),
                );
            }
        }

        let memoized = projection_of(state);
        let rebuilt = state
            .workspace
            .build_design_projection(
                &state.library_manager,
                &state.workspace.active_view,
                &state.schematic,
                None,
                None,
            )
            .expect("the reference project rebuilds");
        assert_eq!(
            memoized.root(),
            rebuilt.root(),
            "step {step}: memoized root disagrees with a full rebuild"
        );
        assert_eq!(
            memoized.plan(),
            rebuilt.plan(),
            "step {step}: memoized plan disagrees with a full rebuild"
        );
        assert_eq!(
            memoized.connectivity(),
            rebuilt.connectivity(),
            "step {step}: memoized connectivity disagrees with a full rebuild"
        );
        let mut memoized_views = memoized.schematic_buffers().keys().collect::<Vec<_>>();
        let mut rebuilt_views = rebuilt.schematic_buffers().keys().collect::<Vec<_>>();
        memoized_views.sort_unstable();
        rebuilt_views.sort_unstable();
        assert_eq!(
            memoized_views, rebuilt_views,
            "step {step}: memoized cell views disagree with a full rebuild"
        );
        for (key, schematic) in memoized.schematic_buffers() {
            let fresh = rebuilt
                .schematic_buffers()
                .get(key)
                .expect("the cell-view sets already matched");
            assert_eq!(
                schematic.components, fresh.components,
                "step {step}: memoized `{key}` disagrees with a full rebuild"
            );
            assert_eq!(
                schematic.net_labels, fresh.net_labels,
                "step {step}: memoized `{key}` disagrees with a full rebuild"
            );
        }
    }
}

#[test]
fn nets_come_from_the_projection_and_are_retained() {
    let reference = hierarchy_reference::build();
    let state = &reference.state;
    let projection = projection_of(state);
    let root_key = projection.root().key();

    let nets = projection.nets_for(&state.library_manager, &root_key);
    assert!(!nets.is_empty(), "the reference root resolves nets");
    assert!(
        Arc::ptr_eq(
            &nets,
            &projection.nets_for(&state.library_manager, &root_key)
        ),
        "a second request for the same cell view must reuse the extraction"
    );
    assert!(
        projection
            .nets_for(&state.library_manager, "user/absent/schematic")
            .is_empty(),
        "a cell view the projection does not own has no nets"
    );
}

/// Timing guard for the memo being load-bearing. Ignored by default because it
/// reports a wall clock rather than asserting one; run it with `--ignored
/// --nocapture` when changing what the projection key covers.
#[test]
#[ignore = "reports a wall clock rather than asserting one"]
fn two_hundred_projections_over_thirty_cell_views() {
    const CELL_VIEWS: usize = 30;
    const CALLS: usize = 200;

    let mut reference = hierarchy_reference::build();
    let amp_key = reference.amp_schematic.key();
    let state = &mut reference.state;
    let filler = state
        .workspace
        .schematic_buffers
        .get(&amp_key)
        .expect("the reference project persists the amp schematic")
        .clone();
    while state.workspace.schematic_buffers.len() < CELL_VIEWS {
        let ordinal = state.workspace.schematic_buffers.len();
        state
            .workspace
            .schematic_buffers
            .insert(format!("user/pad{ordinal:02}/schematic"), filler.clone());
    }

    let _ = projection_of(state);
    let started = std::time::Instant::now();
    for _ in 0..CALLS {
        let _ = projection_of(state);
    }
    let memoized = started.elapsed();

    let started = std::time::Instant::now();
    for _ in 0..CALLS {
        *state.workspace.design_projection_cache.borrow_mut() = None;
        state.workspace.materialized_buffers.borrow_mut().clear();
        let _ = projection_of(state);
    }
    let cold = started.elapsed();

    println!(
        "{CALLS} projections over {CELL_VIEWS} cell views: memoized {memoized:?}, cold {cold:?}"
    );
}
