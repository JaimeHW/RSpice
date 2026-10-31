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
    app.state.model.model_library_manager.clear();
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
    app.state.model.model_library_manager.add_library(kit);
    app.state.models.models_page = ModelsPage::Models;

    app.state.model.workspace.ensure_active_buffer();
    let mut schematic = app
        .state
        .model
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
    app.state.model.workspace.save_active_schematic(&schematic);
    app.state.model.schematic = schematic;
    app
}

/// The catalog opens a family on its first bin, shows another bin from the
/// family's own detail, and in either case binds the selected instance to
/// `nch` with its library, unpinning the bin it named.
#[test]
fn binding_from_the_catalog_binds_the_family_with_its_library() {
    let mut app = kit_project(&["nch.1"]);
    app.state
        .model
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
        studio.app.state.models.selected_model.as_deref(),
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
        studio.app.state.models.selected_model.as_deref(),
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
        .model
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
