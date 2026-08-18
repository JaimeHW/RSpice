//! Guards for the design projection memo: what must rebuild it, what must
//! not, and how much of the design one edit is allowed to re-materialize.
//!
//! The fixtures are built from state-layer primitives alone. A workbench
//! example project would be a richer design, but the projection sits well
//! below the workbench and a test that reaches up for a fixture is still a
//! reference from `state` to `workbench`.

use super::*;

use crate::state::{
    Cell, ComponentType, ConfigurationBlackBoxPolicy, ConfigurationModelProfile,
    ConfigurationSetCatalog, ConfigurationSetDefinition, GlobalNetPromotionPolicy, Library,
    LibraryCellInstance, Point, UnresolvedBindingPolicy, View, ViewType,
};

/// Reference designator of the instance the configuration names as its DUT.
const INSTANCE_NAME: &str = "X1";
/// Reference designator of a top-level component nothing else names, so a
/// rename of it exercises the memo without unresolving the configuration.
const LOAD_NAME: &str = "RLOAD";
const RENAMED_LOAD: &str = "RSENSE";

fn child_key() -> String {
    CellViewRef::new("work", "amp", "schematic").key()
}

/// `work/amp`: a resistor between two ports. The port terminals coincide with
/// the resistor terminals, so connectivity needs no wires.
fn amp_master() -> SchematicState {
    let mut master = SchematicState::default();
    place_port(&mut master, "a", Point::new(20, 0));
    master.add_component(ComponentType::Resistor, Point::new(30, 0));
    place_port(&mut master, "b", Point::new(60, 0));
    master
}

fn place_port(schematic: &mut SchematicState, name: &str, position: Point) {
    let id = schematic.add_component(ComponentType::Port, position);
    schematic
        .components
        .iter_mut()
        .find(|component| component.id == id)
        .expect("the placed port is retained")
        .value = name.to_owned();
}

fn set_name(schematic: &mut SchematicState, id: u64, name: &str) {
    schematic
        .components
        .iter_mut()
        .find(|component| component.id == id)
        .expect("the placed component is retained")
        .name = name.to_owned();
}

/// Edit content without advancing any commit counter, which is how the
/// editing surfaces mutate while a session is open.
fn set_first_resistor_value(schematic: &mut SchematicState, value: &str) {
    schematic
        .components
        .iter_mut()
        .find(|component| component.kind == ComponentType::Resistor)
        .expect("the fixture owns a resistor")
        .value = value.to_owned();
}

/// Two cell views under one active configuration: `work/amp` is a persisted
/// buffer, and `user/top` is the live editor buffer that places one instance
/// of it beside a load resistor.
fn workspace_with_two_cell_views() -> (
    ProjectWorkspace,
    LibraryManager,
    CellViewRef,
    SchematicState,
) {
    let mut libraries = LibraryManager::new();
    let mut user = Library::new("user");
    let mut top_cell = Cell::new("top");
    top_cell.add_view(View::new("schematic", ViewType::Schematic));
    user.add_cell(top_cell);
    libraries.add_library(user);

    let mut work = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    work.add_cell(amp);
    libraries.add_library(work);

    let mut workspace = ProjectWorkspace::default();
    workspace
        .schematic_buffers
        .insert(child_key(), amp_master());

    let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
    binding.terminal_order = vec!["a".to_owned(), "b".to_owned()];
    let mut top = SchematicState::default();
    let instance = top.add_library_cell_component(Point::new(100, 0), binding);
    set_name(&mut top, instance, INSTANCE_NAME);
    let load = top.add_component(ComponentType::Resistor, Point::new(240, 0));
    set_name(&mut top, load, LOAD_NAME);

    let active_reference = workspace.active_view.clone();
    workspace
        .schematic_buffers
        .insert(active_reference.key(), top.clone());

    let mut catalog = ConfigurationSetCatalog::default();
    catalog
        .create(ConfigurationSetDefinition {
            name: "Projection fixture".to_owned(),
            root: active_reference.clone(),
            dut_path: format!("/{INSTANCE_NAME}"),
            executable_view_policy: vec!["schematic".to_owned()],
            stop_views: Vec::new(),
            unresolved_policy: UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy: ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: Vec::new(),
            model_profile: ConfigurationModelProfile::ProjectRunSetSections,
            owner: "projection fixture".to_owned(),
        })
        .expect("the fixture configuration is well formed");
    workspace.configuration_sets = catalog;

    (workspace, libraries, active_reference, top)
}

fn projection_of(
    workspace: &ProjectWorkspace,
    libraries: &LibraryManager,
    active_reference: &CellViewRef,
    active_schematic: &SchematicState,
) -> Arc<DesignProjection> {
    workspace
        .design_projection(libraries, active_reference, active_schematic)
        .expect("the fixture resolves into a design projection")
}

#[test]
fn projection_is_not_rebuilt_without_a_key_change() {
    let (mut workspace, libraries, reference, mut active) = workspace_with_two_cell_views();

    let first = projection_of(&workspace, &libraries, &reference, &active);
    assert!(
        Arc::ptr_eq(
            &first,
            &projection_of(&workspace, &libraries, &reference, &active)
        ),
        "two calls with nothing changed must share one projection"
    );

    workspace.netlist_source_dirty = true;
    assert!(
        Arc::ptr_eq(
            &first,
            &projection_of(&workspace, &libraries, &reference, &active)
        ),
        "a workspace field the projection never reads must not rebuild it"
    );

    set_first_resistor_value(&mut active, "2k");
    let after_active_edit = projection_of(&workspace, &libraries, &reference, &active);
    assert!(
        !Arc::ptr_eq(&first, &after_active_edit),
        "a value edit on the active sheet must rebuild the projection"
    );

    set_first_resistor_value(
        workspace
            .schematic_buffers
            .get_mut(&child_key())
            .expect("the fixture persists the child schematic"),
        "9k",
    );
    assert!(
        !Arc::ptr_eq(
            &after_active_edit,
            &projection_of(&workspace, &libraries, &reference, &active)
        ),
        "an edit to a buffer that is not the active sheet must rebuild the projection"
    );
}

#[test]
fn projection_names_follow_a_property_rename() {
    let (workspace, libraries, reference, mut active) = workspace_with_two_cell_views();

    let before = projection_of(&workspace, &libraries, &reference, &active);
    let topology_before = active.topology_version();
    active
        .components
        .iter_mut()
        .find(|component| component.name == LOAD_NAME)
        .expect("the fixture top sheet owns the load")
        .name = RENAMED_LOAD.to_owned();
    assert_eq!(
        active.topology_version(),
        topology_before,
        "a rename is deliberately invisible to the topology version, which is \
         why the projection key cannot be built from it"
    );

    let after = projection_of(&workspace, &libraries, &reference, &active);
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
        names.contains(&RENAMED_LOAD),
        "the projection must carry the new name: {names:?}"
    );
    assert!(
        !names.contains(&LOAD_NAME),
        "the projection must not carry the old name: {names:?}"
    );
}

#[test]
fn one_edit_rematerializes_one_cell_view() {
    let (workspace, libraries, reference, mut active) = workspace_with_two_cell_views();
    let cell_views = projection_of(&workspace, &libraries, &reference, &active)
        .schematic_buffers()
        .len();
    assert!(
        cell_views > 1,
        "the fixture must own more than one cell view"
    );

    reset_materialization_count();
    set_first_resistor_value(&mut active, "3k");
    let _ = projection_of(&workspace, &libraries, &reference, &active);
    assert_eq!(
        materialization_count(),
        1,
        "one edit must re-materialize exactly one of {cell_views} cell views"
    );

    reset_materialization_count();
    let _ = projection_of(&workspace, &libraries, &reference, &active);
    assert_eq!(
        materialization_count(),
        0,
        "a projection that is still current must not materialize anything"
    );
}

#[test]
fn a_memo_hit_carries_what_a_full_rebuild_would_have_produced() {
    let (mut workspace, libraries, reference, mut active) = workspace_with_two_cell_views();

    for step in 0..4 {
        match step {
            0 => set_first_resistor_value(&mut active, "5k"),
            1 => set_first_resistor_value(
                workspace
                    .schematic_buffers
                    .get_mut(&child_key())
                    .expect("the fixture persists the child schematic"),
                "7k",
            ),
            2 => {
                workspace.connectivity.policy.global_promotion =
                    GlobalNetPromotionPolicy::TechnologyDefinedOnly;
            }
            _ => {
                workspace
                    .schematic_buffers
                    .insert("user/spare/schematic".to_owned(), SchematicState::default());
            }
        }

        let memoized = projection_of(&workspace, &libraries, &reference, &active);
        let rebuilt = workspace
            .build_design_projection(&libraries, &reference, &active, None, None)
            .expect("the fixture rebuilds");
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
fn a_memo_slot_is_built_once_per_cell_view_and_folds_its_key() {
    let (workspace, libraries, reference, active) = workspace_with_two_cell_views();
    let projection = projection_of(&workspace, &libraries, &reference, &active);
    let root_key = projection.root().key();
    let builds = std::cell::Cell::new(0_u32);
    let build = || {
        builds.set(builds.get() + 1);
        Arc::new(builds.get()) as Arc<dyn std::any::Any + Send + Sync>
    };

    let first = projection.memo_nets(&root_key, build);
    let second = projection.memo_nets(&root_key.to_ascii_uppercase(), build);
    assert_eq!(builds.get(), 1, "a retained slot must not be rebuilt");
    assert!(
        Arc::ptr_eq(&first, &second),
        "the slot key folds case, exactly as cell-view lookup does"
    );

    let other = projection.memo_nets("user/absent/schematic", build);
    assert_eq!(builds.get(), 2, "a different cell view owns its own slot");
    assert!(!Arc::ptr_eq(&first, &other));
}

/// Timing guard for the memo being load-bearing. Ignored by default because it
/// reports a wall clock rather than asserting one; run it with `--ignored
/// --nocapture` when changing what the projection key covers.
#[test]
#[ignore = "reports a wall clock rather than asserting one"]
fn two_hundred_projections_over_thirty_cell_views() {
    const CELL_VIEWS: usize = 30;
    const CALLS: usize = 200;

    let (mut workspace, libraries, reference, active) = workspace_with_two_cell_views();
    let filler = amp_master();
    while workspace.schematic_buffers.len() < CELL_VIEWS {
        let ordinal = workspace.schematic_buffers.len();
        workspace
            .schematic_buffers
            .insert(format!("user/pad{ordinal:02}/schematic"), filler.clone());
    }

    let _ = projection_of(&workspace, &libraries, &reference, &active);
    let started = std::time::Instant::now();
    for _ in 0..CALLS {
        let _ = projection_of(&workspace, &libraries, &reference, &active);
    }
    let memoized = started.elapsed();

    let started = std::time::Instant::now();
    for _ in 0..CALLS {
        *workspace.design_projection_cache.borrow_mut() = None;
        workspace.materialized_buffers.borrow_mut().clear();
        let _ = projection_of(&workspace, &libraries, &reference, &active);
    }
    let cold = started.elapsed();

    println!(
        "{CALLS} projections over {CELL_VIEWS} cell views: memoized {memoized:?}, cold {cold:?}"
    );
}
