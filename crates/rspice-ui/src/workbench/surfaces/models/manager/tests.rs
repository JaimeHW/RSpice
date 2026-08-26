//! Attribution and detachment both resolve to one project revision.
//!
//! A bound consumer is attributed to the *effective* provider rather than to
//! whichever library happens to declare the name, and detaching a pack takes
//! every library it attached with it in a single revision — a partial detach
//! leaves bindings pointing at a provider the project no longer has.

use super::*;
use crate::state::model_library::ModelType;
use crate::state::{Component, ComponentType, Point};

#[test]
fn bound_consumers_are_attributed_only_to_the_effective_provider() {
    let mut state = AppState::default();
    state.model_library_manager.clear();
    for name in ["alpha", "beta"] {
        let mut library = ModelLibrary::new(name);
        library.add_model(DeviceModel::new("nch", ModelType::Nmos));
        state.model_library_manager.add_library(library);
    }
    state
        .model_library_manager
        .resolve_definition_provider(
            ModelConsumerScope::PrimitiveModel,
            "nch",
            "alpha",
            "provider-aware consumer test",
        )
        .expect("the contested provider can be resolved");

    state.workspace.ensure_active_buffer();
    let mut schematic = state
        .workspace
        .active_schematic()
        .cloned()
        .expect("an active schematic exists");
    let mut component = Component::new(1, ComponentType::Nmos, Point::origin());
    component.name = "M1".to_owned();
    component.params = "model=nch model_library=alpha".to_owned();
    schematic.components.push(component);
    state.workspace.save_active_schematic(&schematic);

    let mut pending_actions = Vec::new();
    let render = ManagerRenderContext {
        state: &mut state,
        pending_actions: &mut pending_actions,
    };
    let consumers = ConsumerIndex::build(&render);
    let scan = project_catalog_scan(&render, &consumers);
    let alpha = scan
        .rows
        .iter()
        .find(|row| row.library == "alpha" && row.model == "nch")
        .expect("alpha catalog row");
    let beta = scan
        .rows
        .iter()
        .find(|row| row.library == "beta" && row.model == "nch")
        .expect("beta catalog row");

    assert!(
        alpha
            .usage
            .as_deref()
            .is_some_and(|usage| usage.starts_with("M1"))
    );
    assert_eq!(beta.usage, None);
    assert!(scan.consumer_diagnostics.is_empty());
}

#[test]
fn detaching_a_pack_removes_every_attached_library_in_one_revision() {
    let mut state = AppState::default();
    for (library_name, model_name) in [("pack-a", "na"), ("pack-b", "nb")] {
        let mut library = ModelLibrary::new(library_name);
        library.pack_id = Some("shared-pack".to_owned());
        library.add_model(DeviceModel::new(model_name, ModelType::Nmos));
        state.model_library_manager.add_library(library);
    }
    let initial_revision = state.workspace.project.revision();
    let initial_epoch = state.design_execution_epoch;
    let mut pending_actions = Vec::new();
    let mut render = ManagerRenderContext {
        state: &mut state,
        pending_actions: &mut pending_actions,
    };

    detach_pack(&mut render, "shared-pack");

    assert!(
        render
            .state
            .model_library_manager
            .get_library("pack-a")
            .is_none()
    );
    assert!(
        render
            .state
            .model_library_manager
            .get_library("pack-b")
            .is_none()
    );
    assert_eq!(
        render.state.workspace.project.revision().get(),
        initial_revision.get() + 1
    );
    assert_eq!(
        render.state.design_execution_epoch,
        initial_epoch.wrapping_add(1)
    );
}

#[test]
fn signed_technology_symbol_variant_is_authored_in_one_project_revision() {
    let mut state = AppState::default();
    state.provision_test_project_symbol_technology_contract();
    state
        .library_manager
        .add_library(crate::state::Library::new("signed_variant_test"));
    let initial_revision = state.workspace.project.revision();
    let package_digest = state
        .project_signed_technology_package()
        .expect("exact package resolves")
        .expect("project has package")
        .archive_digest();
    let mut pending_actions = Vec::new();
    let mut render = ManagerRenderContext {
        state: &mut state,
        pending_actions: &mut pending_actions,
    };

    let receipt = author_technology_symbol_variant(
        &mut render,
        "demo180",
        "nmos_demo",
        "signed_variant_test",
        "nmos_custom",
    )
    .expect("variant commits");

    assert!(receipt.contains("signed technology symbol 'demo180/nmos_demo'"));
    assert_eq!(
        render.state.workspace.project.revision().get(),
        initial_revision.get() + 1
    );
    let view = render
        .state
        .library_manager
        .get_library("signed_variant_test")
        .and_then(|library| library.get_cell("nmos_custom"))
        .and_then(|cell| cell.get_view("symbol"))
        .expect("variant symbol view exists");
    let definition = ModelBoundSymbolDefinition::load_from_view(view)
        .expect("metadata loads")
        .expect("typed definition exists");
    assert_eq!(definition.identity.library, "signed_variant_test");
    assert_eq!(definition.identity.cell, "nmos_custom");
    assert_eq!(definition.identity.revision, 1);
    let model = definition
        .netlist
        .model
        .as_ref()
        .expect("signed model binding");
    assert_eq!(model.library, "signed-pdk:demo-models-tt");
    assert!(
        model
            .source_path
            .as_deref()
            .is_some_and(|path| path.contains(&package_digest.to_string()))
    );
    definition
        .validate()
        .expect("project variant stays executable");

    let committed_revision = render.state.workspace.project.revision();
    assert!(
        author_technology_symbol_variant(
            &mut render,
            "demo180",
            "nmos_demo",
            "signed_variant_test",
            "nmos_custom",
        )
        .is_err()
    );
    assert_eq!(
        render.state.workspace.project.revision(),
        committed_revision,
        "a rejected overwrite must not publish a partial transaction"
    );
}

#[test]
fn the_pin_action_names_whichever_reason_actually_blocks_it() {
    // Two conditions disable the pin and only one hover text existed, so a
    // user who had started an import was told the external library in front of
    // them was built in.
    const BUILT_IN: &str = "Built-in sources do not have an external file to pin.";
    const BUSY: &str = "Another model-source operation is still running.";

    assert_eq!(pin_source_block_reason(true, false), None);
    assert_eq!(pin_source_block_reason(true, true), Some(BUSY));
    assert_eq!(pin_source_block_reason(false, false), Some(BUILT_IN));
    assert_eq!(
        pin_source_block_reason(false, true),
        Some(BUILT_IN),
        "a built-in source never becomes pinnable, so waiting is not the advice"
    );
}

/// Ordering the catalog reorders the rows the page already derived, and never
/// leaves two of them in an order that depends on the frame.
///
/// The model identity is the final tie-break under every key, which is what
/// makes the order total: a column full of equal cells still lands the same way
/// twice, and reversing the direction reverses the whole order rather than
/// shuffling within its ties.
#[test]
fn ordering_the_catalog_is_total_under_every_column_and_reverses_whole() {
    fn row(model: &str, library: &str, family: &'static str, vectors: usize) -> ProjectModelRow {
        ProjectModelRow {
            library: library.to_owned(),
            model: model.to_owned(),
            family,
            source: format!("{library}.lib"),
            pinned: false,
            review: false,
            drifted: false,
            usage: None,
            vectors,
        }
    }

    let original = || {
        vec![
            row("nch", "beta", "NMOS", 4),
            row("pch", "alpha", "PMOS", 1),
            row("Nch", "alpha", "NMOS", 9),
        ]
    };
    let names = |rows: &[ProjectModelRow]| {
        rows.iter()
            .map(|row| format!("{}/{}", row.library, row.model))
            .collect::<Vec<_>>()
    };

    // The default is the order the table has always opened in: model name,
    // case-folded, library breaking ties.
    let mut rows = original();
    sort_catalog_rows(&mut rows, ModelsTableSort::default());
    assert_eq!(names(&rows), ["alpha/Nch", "beta/nch", "alpha/pch"]);

    let mut reversed = original();
    sort_catalog_rows(
        &mut reversed,
        ModelsTableSort {
            key: ModelsCatalogSortKey::Model,
            descending: true,
        },
    );
    let mut expected = names(&rows);
    expected.reverse();
    assert_eq!(names(&reversed), expected);

    // A numeric column orders by the number, not by its printed form.
    let mut by_vectors = original();
    sort_catalog_rows(
        &mut by_vectors,
        ModelsTableSort {
            key: ModelsCatalogSortKey::Vectors,
            descending: false,
        },
    );
    assert_eq!(
        by_vectors.iter().map(|row| row.vectors).collect::<Vec<_>>(),
        [1, 4, 9]
    );

    // A column whose cells are all equal still produces one order, and it is
    // the identity order.
    let mut tied = original();
    sort_catalog_rows(
        &mut tied,
        ModelsTableSort {
            key: ModelsCatalogSortKey::Status,
            descending: false,
        },
    );
    assert_eq!(names(&tied), names(&rows));
}

/// The STATUS column and the STATUS cell are one fact.
#[test]
fn the_status_column_orders_by_exactly_what_the_cell_says() {
    let mut row = ProjectModelRow {
        library: "alpha".to_owned(),
        model: "nch".to_owned(),
        family: "NMOS",
        source: "alpha.lib".to_owned(),
        pinned: false,
        review: false,
        drifted: false,
        usage: None,
        vectors: 0,
    };
    assert_eq!(catalog_status(&row), "");
    row.pinned = true;
    assert_eq!(catalog_status(&row), "pinned");
    row.review = true;
    assert_eq!(
        catalog_status(&row),
        "review",
        "a finding outranks a pin: it is the thing an engineer has to act on"
    );
}
