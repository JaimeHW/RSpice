//! The frozen hierarchical reference project, as one released schema wrote it.
//!
//! `hierarchy_reference_legacy.rspiceproj` is the exact output of
//! `serialize_project_file` for the project
//! `workbench::examples::hierarchy_reference::build` produces, captured once
//! and never regenerated. The live builder answers for what the application
//! can author today; these bytes answer for what it must still be able to
//! open, which is a different question the moment a schema moves.
//!
//! Every later schema change extends this file's suite rather than replacing
//! the fixture: a migration lane adds the cases that pin how its own change
//! reads these bytes, and restates — never relaxes — the assertions below
//! wherever its own rewrite changes what a load produces.

use super::*;

/// The frozen bytes. Never rewrite them — a regenerated fixture proves only
/// that today's writer agrees with today's reader.
const FROZEN_PROJECT: &str = include_str!("hierarchy_reference_legacy.rspiceproj");

const USER_LIBRARY: &str = "user";
const VENDOR_LIBRARY: &str = "vendor";
const AMP_VIEW_KEY: &str = "user/amp/schematic";
const FILTER_CELL: &str = "filter";
const CROSS_SHEET_NET: &str = "mid";
/// The two configured scopes as the frozen bytes spell them, from the schema
/// that still named the design root.
const AUTHORED_DUT_PATH: &str = "/top/X1";
const AUTHORED_OVERRIDE_PATH: &str = "/top/X2";
/// The same two scopes as every reader sees them. The catalog rewrites the
/// authored spelling while it deserializes, so deserialization is the last
/// place the old grammar exists.
const DUT_PATH: &str = "/X1";
const OVERRIDE_PATH: &str = "/X2";
const SAVED_OUTPUT_EXPRESSION: &str = "V(out)";

#[test]
fn the_frozen_hierarchical_reference_still_loads_and_validates() {
    let project = load_project_text(FROZEN_PROJECT, None).expect("the frozen reference loads");
    project.validate().expect("the frozen reference validates");
    assert!(
        project.simulation_results_warning.is_none(),
        "{:?}",
        project.simulation_results_warning
    );

    for view in [
        "user/top/schematic",
        AMP_VIEW_KEY,
        "user/filter/schematic",
        "vendor/filter/schematic",
    ] {
        assert!(
            project.workspace.schematic_buffers.contains_key(view),
            "{view} lost its schematic buffer"
        );
    }
    for library in [USER_LIBRARY, VENDOR_LIBRARY] {
        assert!(
            project
                .libraries
                .get_library(library)
                .and_then(|library| library.get_cell(FILTER_CELL))
                .is_some(),
            "{library} lost its '{FILTER_CELL}' cell"
        );
    }
    assert_eq!(
        project
            .libraries
            .get_library(USER_LIBRARY)
            .and_then(|library| library.get_cell("amp"))
            .and_then(|cell| cell.get_view("symbol"))
            .map(|view| view.view_type),
        Some(ViewType::Symbol)
    );
}

#[test]
fn the_frozen_hierarchical_reference_keeps_its_sheets_configuration_and_outputs() {
    let project = load_project_text(FROZEN_PROJECT, None).expect("the frozen reference loads");

    let catalog = project
        .workspace
        .design_management
        .sheet_catalog(AMP_VIEW_KEY)
        .expect("the amp sheet catalog is retained");
    assert_eq!(catalog.sheets().len(), 2);
    assert_eq!(
        catalog
            .cross_sheet_ports()
            .iter()
            .map(|port| port.definition().net_name.as_str())
            .collect::<Vec<_>>(),
        vec![CROSS_SHEET_NET]
    );

    let configuration = project
        .workspace
        .configuration_sets
        .active()
        .expect("the reference configuration is still active");
    assert_eq!(configuration.dut_path(), DUT_PATH);
    assert_eq!(configuration.root().key(), "user/top/schematic");

    let plan = project
        .execution_context
        .as_ref()
        .and_then(|context| context.simulation_plan.analysis_plan.as_ref())
        .expect("the frozen reference carries a stable analysis plan")
        .id();
    assert_eq!(
        project.workspace.plan_data(plan).map(|payload| payload
            .saved_outputs
            .iter()
            .map(|output| output.source_expression.as_str())
            .collect::<Vec<_>>()),
        Some(vec![SAVED_OUTPUT_EXPRESSION])
    );
}

#[test]
fn the_frozen_hierarchical_reference_rewrites_to_a_project_that_loads_again() {
    let project = load_project_text(FROZEN_PROJECT, None).expect("the frozen reference loads");
    let rewritten = serialize_project_file(&project).expect("the loaded reference re-serializes");
    load_project_text(&rewritten, None)
        .expect("the re-serialized reference loads")
        .validate()
        .expect("the re-serialized reference validates");
}

#[test]
fn legacy_paths_load_and_reserialize_in_the_new_grammar() {
    assert!(
        FROZEN_PROJECT.contains(AUTHORED_DUT_PATH)
            && FROZEN_PROJECT.contains(AUTHORED_OVERRIDE_PATH),
        "the fixture is only a migration case while it still names the design root"
    );

    let loaded = load_project_text(FROZEN_PROJECT, None).expect("the frozen reference loads");
    let configuration = loaded
        .workspace
        .configuration_sets
        .active()
        .expect("the reference configuration is still active");
    assert_eq!(configuration.dut_path(), DUT_PATH);
    assert_eq!(configuration.overrides()[0].instance_path, OVERRIDE_PATH);
    assert_eq!(
        configuration.revision(),
        1,
        "migration restates an authored choice rather than making one"
    );
    assert_eq!(
        loaded.workspace.configuration_sets.schema_version(),
        crate::state::ConfigurationSetCatalog::default().schema_version(),
        "the catalog is relabelled as the schema it now holds"
    );
    loaded
        .workspace
        .configuration_sets
        .validate()
        .expect("the migrated catalog's digests cover what it now stores");

    let rewritten = serialize_project_file(&loaded).expect("the migrated project serializes");
    assert!(
        rewritten.contains(&format!("\"dut_path\": \"{DUT_PATH}\"")),
        "{rewritten}"
    );
    assert!(
        rewritten.contains(&format!("\"instance_path\": \"{OVERRIDE_PATH}\"")),
        "{rewritten}"
    );
    for authored in [AUTHORED_DUT_PATH, AUTHORED_OVERRIDE_PATH] {
        assert!(
            !rewritten.contains(authored),
            "the old spelling of {authored} is not rewritten back"
        );
    }

    let reloaded = load_project_text(&rewritten, None).expect("the migrated project reloads");
    assert_eq!(
        reloaded.workspace.configuration_sets, loaded.workspace.configuration_sets,
        "migration is a one-time rewrite"
    );
}

/// Editor schema 1 kept the amp's `AMP` lettering in the symbol sidecar,
/// where only the editor drew it. These bytes still spell it that way, so a
/// load has to move it into the body — that is the whole of the migration,
/// and it is what puts the lettering on every placed instance and export.
#[test]
fn the_frozen_symbol_sidecar_text_loads_as_body_geometry() {
    let project = load_project_text(FROZEN_PROJECT, None).expect("the frozen reference loads");
    let symbol = project
        .libraries
        .get_library(USER_LIBRARY)
        .and_then(|library| library.get_cell("amp"))
        .and_then(|cell| cell.get_view("symbol"))
        .expect("the frozen amp symbol view is retained");

    let document =
        crate::state::SymbolDocument::load_from_view(symbol).expect("the frozen symbol loads");

    assert_eq!(
        document.text_runs().collect::<Vec<_>>(),
        vec![("AMP", crate::state::Point::origin())]
    );
}

#[test]
fn a_configured_path_the_engine_cannot_name_is_refused_at_load() {
    let mut libraries = LibraryManager::with_primitives();
    let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
    let root = workspace.active_view.clone();
    workspace
        .configuration_sets
        .create(configuration_named("Release", &root, "/XDUT"))
        .expect("an addressable configuration");
    let project = ProjectFile::new(workspace, libraries);
    let mut value: serde_json::Value =
        serde_json::from_str(&serialize_project_file(&project).expect("the project serializes"))
            .expect("the project is JSON");

    // The catalog is replaced wholesale rather than edited in place, so the
    // digest it carries is genuine and the load fails on the path itself.
    let mut unnameable = crate::state::ConfigurationSetCatalog::default();
    unnameable
        .create(configuration_named("Release", &root, "/Xβ"))
        .expect("a design may name an instance the engine cannot");
    value["workspace"]["configuration_sets"] =
        serde_json::to_value(&unnameable).expect("the catalog serializes");

    let error = load_project_text(&value.to_string(), None)
        .expect_err("a project that cannot be netlisted does not open");
    let message = error.to_string();
    assert!(message.contains("/Xβ"), "{message} must name the path");
    assert!(message.contains("cannot be netlisted"), "{message}");
}

fn configuration_named(
    name: &str,
    root: &CellViewRef,
    dut_path: &str,
) -> crate::state::ConfigurationSetDefinition {
    crate::state::ConfigurationSetDefinition {
        name: name.to_owned(),
        root: root.clone(),
        dut_path: dut_path.to_owned(),
        executable_view_policy: vec!["schematic".to_owned(), "spice".to_owned()],
        stop_views: vec!["spice".to_owned()],
        unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
        black_box_policy:
            crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
        overrides: Vec::new(),
        model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
        owner: "Verification".to_owned(),
    }
}
