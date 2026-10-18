//! A binned family is one catalog row, named as an instance binds it.
//!
//! A foundry kit's `nch.1` … `nch.48` were 48 rows, and binding from one
//! pinned the instance to that bin, which then ran at every size. The row is
//! the family now: its bins are its detail, the instances its bins run for are
//! counted against it, and Bind gives an instance `nch` with its library.

use super::*;

/// A kit holding two bins of `nch` and the card `nch_25`, and one NMOS
/// instance per entry of `models`, `M1` onward, each naming that model.
fn kit_project(models: &[&str]) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.model_library_manager.clear();
    let mut kit = ModelLibrary::new("kit");
    for (name, line, length) in [
        ("nch.1", 10, (0.1e-6, 1e-6)),
        ("nch.2", 20, (1e-6, 10e-6)),
        ("nch_25", 30, (0.1e-6, 10e-6)),
    ] {
        let mut card =
            DeviceModel::new(name, ModelType::Nmos).with_geometry(length.0, length.1, 1e-6, 10e-6);
        card.spice_type = Some("NMOS".to_owned());
        card.file_path = Some(PathBuf::from("kit.lib"));
        card.source_line = Some(line);
        kit.add_model(card);
    }
    app.state.model_library_manager.add_library(kit);
    app.state.workbench.models_page = ModelsPage::Models;

    app.state.workspace.ensure_active_buffer();
    let mut schematic = app
        .state
        .workspace
        .active_schematic()
        .map(|source| source.clone_editor())
        .expect("an active schematic exists");
    for (index, model) in models.iter().enumerate() {
        let id = index as u64 + 1;
        let mut component =
            Component::new(id, ComponentType::Nmos, Point::new(index as i32 * 40, 0));
        component.name = format!("M{id}");
        component.params = format!("model={model}");
        schematic.document_mut_for_test().components.push(component);
    }
    app.state.workspace.save_active_schematic(&schematic);
    app.state.schematic = schematic;
    app
}

fn scan_rows(state: &mut AppState) -> (Vec<(String, usize, usize)>, usize) {
    let mut pending_actions = Vec::new();
    let render = ManagerRenderContext {
        state,
        pending_actions: &mut pending_actions,
    };
    let consumers = ConsumerIndex::build(&render);
    let scan = project_catalog_scan(&render, &consumers);
    assert!(
        scan.consumer_diagnostics.is_empty(),
        "every instance resolves: {:?}",
        scan.consumer_diagnostics
    );
    let rows = scan
        .rows
        .iter()
        .map(|row| (row.model.clone(), row.bins, row.usage_count))
        .collect();
    (rows, scan.total)
}

/// One row for the family, none for its bins, and both the instance naming
/// the family and the one naming a bin are its consumers.
#[test]
fn a_family_is_one_row_that_counts_every_instance_its_bins_run_for() {
    let mut app = kit_project(&["nch", "nch.2", "nch_25"]);
    let (rows, total) = scan_rows(&mut app.state);
    assert_eq!(
        rows,
        [("nch".to_owned(), 2, 2), ("nch_25".to_owned(), 0, 1)],
        "the bins are the family's detail, not rows beside it"
    );
    assert_eq!(total, 2, "the footer counts a family once");

    let mut pending_actions = Vec::new();
    let render = ManagerRenderContext {
        state: &mut app.state,
        pending_actions: &mut pending_actions,
    };
    let consumers = ConsumerIndex::build(&render);
    let kit = render
        .state
        .model_library_manager
        .get_library("kit")
        .expect("kit");
    let family = kit.bin_family("nch").expect("the kit provides nch");
    let labels = consumers.of_family(kit, &family);
    assert_eq!(labels.len(), 2, "{labels:?}");
    assert!(
        labels
            .iter()
            .any(|label| bindings::consumer_designator(label) == "M1"
                && !label.contains("names bin")),
        "{labels:?}"
    );
    assert!(
        labels
            .iter()
            .any(|label| bindings::consumer_designator(label) == "M2"
                && label.ends_with("names bin nch.2")),
        "an instance naming one bin says which: {labels:?}"
    );
}

/// A library holding a card of the family's own name runs that card for the
/// name, so it provides no family and its bins stay cards an instance can
/// bind exactly.
#[test]
fn a_card_of_the_familys_own_name_keeps_its_bins_as_rows() {
    let mut app = kit_project(&["nch.1"]);
    app.state
        .model_library_manager
        .get_library_mut("kit")
        .expect("kit")
        .add_model(DeviceModel::new("nch", ModelType::Nmos));
    let (rows, total) = scan_rows(&mut app.state);
    assert_eq!(
        rows,
        [
            ("nch".to_owned(), 0, 0),
            ("nch.1".to_owned(), 0, 1),
            ("nch.2".to_owned(), 0, 0),
            ("nch_25".to_owned(), 0, 0),
        ]
    );
    assert_eq!(total, 4);
}

/// The catalog opens a family on its first bin, shows another bin from the
/// family's own detail, and in either case binds the selected instance to
/// `nch` with its library, unpinning the bin it named.
#[test]
fn binding_from_the_catalog_binds_the_family_with_its_library() {
    let mut app = kit_project(&["nch.1"]);
    app.state
        .schematic
        .session
        .editor
        .selection
        .select_only_component(1);
    let mut studio = ModelsStudio::open(app);
    assert!(
        studio.announces(|label| label == "nch · 2 bins in kit"),
        "the family is one row; the page announces {:#?}",
        studio.announcements()
    );
    assert!(
        !studio.announces(|label| label.starts_with("nch.1 in") || label.starts_with("nch.2 in")),
        "no bin is a row of its own; the page announces {:#?}",
        studio.announcements()
    );
    assert_eq!(
        studio.app.state.workbench.selected_model.as_deref(),
        Some("nch.1"),
        "a family's selection is a card, so every reader of it still reads one"
    );
    assert!(
        studio
            .painted
            .iter()
            .any(|(text, _)| text.ends_with("names bin nch.1")),
        "where-used names the bin M1 is pinned to"
    );

    studio.click(|label| label.starts_with("nch.2 · L "));
    assert_eq!(
        studio.app.state.workbench.selected_model.as_deref(),
        Some("nch.2")
    );
    assert!(
        studio.announces(|label| label == "nch · 2 bins in kit"),
        "showing another bin keeps the family's row selected"
    );

    studio.click(|label| label == "Bind to M1…");
    let params = &studio
        .app
        .state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == 1)
        .expect("M1")
        .params;
    assert!(
        params.ends_with("model=nch model_library=kit"),
        "Bind gives the instance the family, not the bin on show: {params}"
    );
}
