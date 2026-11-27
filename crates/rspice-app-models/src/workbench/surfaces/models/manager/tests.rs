//! A bound consumer is attributed to the *effective* provider rather than to
//! whichever library happens to declare the name.

use super::*;
use crate::state::model_library::ModelType;
use crate::state::{Component, ComponentType, Point};

mod binned_family;

#[test]
fn bound_consumers_are_attributed_only_to_the_effective_provider() {
    let mut state = harness();
    state.model.model_library_manager.clear();
    for name in ["alpha", "beta"] {
        let mut library = ModelLibrary::new(name);
        library.add_model(DeviceModel::new("nch", ModelType::Nmos));
        state.model.model_library_manager.add_library(library);
    }
    state
        .model
        .model_library_manager
        .resolve_definition_provider(
            ModelConsumerScope::PrimitiveModel,
            "nch",
            "alpha",
            "provider-aware consumer test",
        )
        .expect("the contested provider can be resolved");

    state.model.workspace.ensure_active_buffer();
    let mut schematic = state
        .model
        .workspace
        .active_schematic()
        .map(|source| source.clone_editor())
        .expect("an active schematic exists");
    let mut component = Component::new(1, ComponentType::Nmos, Point::origin());
    component.name = "M1".to_owned();
    component.params = "model=nch model_library=alpha".to_owned();
    schematic.document_mut_for_test().components.push(component);
    state.model.workspace.save_active_schematic(&schematic);

    let mut pending_actions = Vec::new();
    let render = ManagerRenderContext::over(&mut state, &mut pending_actions);
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

    // The column carries the designator, not the whole consumer label: the
    // kind and the coordinates are what the where-used list is for.
    assert_eq!(alpha.usage.as_deref(), Some("M1"));
    assert_eq!(alpha.usage_count, 1);
    assert_eq!(beta.usage, None);
    assert_eq!(beta.usage_count, 0);
    assert!(scan.consumer_diagnostics.is_empty());
}

#[test]
fn pdk_technology_symbol_variant_is_authored_in_one_project_revision() {
    let mut state = harness();
    state
        .model
        .provision_test_project_symbol_technology_contract();
    state
        .model
        .library_manager
        .add_library(crate::state::Library::new("signed_variant_test"));
    let initial_revision = state.model.workspace.content.project.revision();
    let package_digest = state
        .model
        .project_pdk_package()
        .expect("exact package resolves")
        .expect("project has package")
        .archive_digest();
    let mut pending_actions = Vec::new();
    let mut render = ManagerRenderContext::over(&mut state, &mut pending_actions);

    let receipt = author_technology_symbol_variant(
        &mut render,
        "demo180",
        "nmos_demo",
        "signed_variant_test",
        "nmos_custom",
    )
    .expect("variant commits");

    assert!(receipt.contains("PDK technology symbol 'demo180/nmos_demo'"));
    assert_eq!(
        render.model.workspace.content.project.revision().get(),
        initial_revision.get() + 1
    );
    let view = render
        .model
        .library_manager
        .get_library("signed_variant_test")
        .and_then(|library| library.get_cell("nmos_custom"))
        .and_then(|cell| cell.get_view("symbol"))
        .expect("variant symbol view exists");
    let definition = crate::state::load_model_bound_symbol(view)
        .expect("metadata loads")
        .expect("typed definition exists");
    assert_eq!(definition.identity.library, "signed_variant_test");
    assert_eq!(definition.identity.cell, "nmos_custom");
    assert_eq!(definition.identity.revision, 1);
    let model = definition
        .netlist
        .model
        .as_ref()
        .expect("PDK model binding");
    assert_eq!(model.library, "pdk:demo-models-tt");
    assert!(
        model
            .source_path
            .as_deref()
            .is_some_and(|path| path.contains(&package_digest.to_string()))
    );
    definition
        .validate()
        .expect("project variant stays executable");

    let committed_revision = render.model.workspace.content.project.revision();
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
        render.model.workspace.content.project.revision(),
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
    fn row(model: &str, library: &str, family: &'static str) -> ProjectModelRow {
        ProjectModelRow {
            library: library.to_owned(),
            model: model.to_owned(),
            bins: 0,
            family,
            source: format!("{library}.lib"),
            pinned: false,
            review: false,
            drifted: false,
            usage: None,
            usage_count: 0,
        }
    }

    let original = || {
        vec![
            row("nch", "beta", "NMOS"),
            row("pch", "alpha", "PMOS"),
            row("Nch", "alpha", "NMOS"),
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
        bins: 0,
        family: "NMOS",
        source: "alpha.lib".to_owned(),
        pinned: false,
        review: false,
        drifted: false,
        usage: None,
        usage_count: 0,
    };
    assert_eq!(catalog_status(&row).label(), "");
    row.pinned = true;
    assert_eq!(catalog_status(&row).label(), "pinned");
    row.review = true;
    assert_eq!(
        catalog_status(&row).label(),
        "review",
        "a finding outranks a pin: it is the thing an engineer has to act on"
    );
}

/// The status word and the mark in front of it are one value.
///
/// A cell that says "review" without the mark is worse than one that says
/// nothing, and the two used to be chosen in different places. A retained pin
/// is a settled state and takes no mark: marking it is how a warning comes to
/// mean nothing.
#[test]
fn only_a_finding_in_the_status_column_carries_a_mark() {
    assert_eq!(CatalogStatus::Review.problem(), Some(Problem::Warning));
    assert_eq!(CatalogStatus::Pinned.problem(), None);
    assert_eq!(CatalogStatus::Unstated.problem(), None);
}

/// The USED BY cell names one instance and counts the rest.
///
/// A model bound eleven times and a model bound once showed the same single
/// designator, so the column could not be read for what it is for.
#[test]
fn the_used_by_cell_names_one_consumer_and_counts_the_others() {
    let mut row = ProjectModelRow {
        library: "alpha".to_owned(),
        model: "nch".to_owned(),
        bins: 0,
        family: "NMOS",
        source: "alpha.lib".to_owned(),
        pinned: false,
        review: false,
        drifted: false,
        usage: None,
        usage_count: 0,
    };
    assert_eq!(used_by_cell(&row), "");
    row.usage = Some("M1".to_owned());
    row.usage_count = 1;
    assert_eq!(used_by_cell(&row), "M1");
    row.usage_count = 4;
    assert_eq!(used_by_cell(&row), "M1 · +3");
}

/// The catalog's USED BY cell reads the designator out of a consumer label,
/// and the where-used list reads the rest of it.
///
/// Both halves are spelled beside the code that writes the label, so a change
/// to its shape cannot leave one of them reading the wrong field.
#[test]
fn a_consumer_label_splits_into_a_designator_and_a_location() {
    let label = "M1 · NMOS · (120, 40)";
    assert_eq!(bindings::consumer_designator(label), "M1");
    assert_eq!(bindings::consumer_location(label), "NMOS · (120, 40)");

    // A label with no separator is all designator: the column still names
    // something rather than emptying itself.
    assert_eq!(bindings::consumer_designator("U1"), "U1");
    assert_eq!(bindings::consumer_location("U1"), "");
}
