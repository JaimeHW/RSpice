//! What the definition table claims about a name, and what it refuses to.
//!
//! Which names are contested, which provider actually executes, where the
//! section came from, and which placed instances the closure cannot answer for.

use super::*;

/// A row as the index builds one, ready to be varied per test.
fn row() -> DefinitionRow {
    DefinitionRow {
        definition: "nch".to_owned(),
        scope: ModelConsumerScope::PrimitiveModel,
        providers: vec!["foundry".to_owned(), "vendor".to_owned()],
        resolution: "resolved · foundry".to_owned(),
        resolved_provider: Some("foundry".to_owned()),
        sections: vec!["tt".to_owned()],
    }
}

/// A duplicate the manager settled is not contested.
///
/// `contested()` was `providers.len() > 1`, which marked every multi-provider
/// name — including the ones whose RESOLUTION column, three cells to the right,
/// already said "resolved · <library>". That inflated the card's own count of
/// what has to be repaired and offered a conflict dialog for a name that binds.
#[test]
fn a_definition_with_a_recorded_provider_is_not_contested() {
    let settled = row();
    assert!(
        !settled.contested(),
        "a name with a recorded provider binds, so it is not what has to be repaired"
    );

    let unsettled = DefinitionRow {
        resolved_provider: None,
        resolution: "contested · fails closed".to_owned(),
        ..settled
    };
    assert!(
        unsettled.contested(),
        "two providers and no decision is the case that fails closed"
    );

    let unique = DefinitionRow {
        providers: vec!["foundry".to_owned()],
        resolution: "unique".to_owned(),
        resolved_provider: None,
        ..unsettled
    };
    assert!(!unique.contested(), "one provider is never contested");
}

#[test]
fn a_contested_name_has_no_effective_provider_and_every_provider_is_a_candidate() {
    // The PROVIDER cell states what executes. For a contested name nothing
    // does, and printing the first candidate there — which the mockup's
    // "winning provider" column did — asserts a policy the product refuses.
    let contested = DefinitionRow {
        resolved_provider: None,
        resolution: "contested · fails closed".to_owned(),
        ..row()
    };
    assert_eq!(contested.effective_provider(), None);
    assert_eq!(contested.other_candidates(), "foundry, vendor");

    let settled = row();
    assert_eq!(settled.effective_provider(), Some("foundry"));
    assert_eq!(
        settled.other_candidates(),
        "vendor",
        "the loser of a settled duplicate is kept as a candidate, never dropped"
    );

    let unique = DefinitionRow {
        providers: vec!["foundry".to_owned()],
        resolution: "unique".to_owned(),
        resolved_provider: None,
        ..row()
    };
    assert_eq!(unique.effective_provider(), Some("foundry"));
    assert_eq!(unique.other_candidates(), "—");
}

#[test]
fn a_definition_outside_any_section_says_so_rather_than_guessing() {
    // The parser records the section every definition came from, so an empty
    // list is evidence that the name sits in the file's unsectioned body — a
    // different fact from "we did not look".
    assert_eq!(row().section_label(), "tt");
    let unsectioned = DefinitionRow {
        sections: Vec::new(),
        ..row()
    };
    assert_eq!(unsectioned.section_label(), "—");
}

#[test]
fn the_index_carries_the_section_each_provider_declares_a_name_inside() {
    let mut state = AppState::default();
    state.model_library_manager.clear();
    let mut library = ModelLibrary::new("foundry");
    let mut sectioned = DeviceModel::new("nch", crate::state::model_library::ModelType::Nmos);
    sectioned.section = Some("tt".to_owned());
    library.add_model(sectioned);
    library.add_model(DeviceModel::new(
        "pch",
        crate::state::model_library::ModelType::Pmos,
    ));
    state.model_library_manager.add_library(library);

    let index = definition_index(&state);
    let nch = index
        .iter()
        .find(|row| row.definition == "nch")
        .expect("the model is indexed");
    assert_eq!(nch.sections, ["tt"]);
    let pch = index
        .iter()
        .find(|row| row.definition == "pch")
        .expect("the model is indexed");
    assert!(
        pch.sections.is_empty(),
        "a model outside a section carries none"
    );
}

#[test]
fn an_instance_naming_something_the_closure_defines_is_not_unresolved() {
    // The callout's whole claim is "no file in the closure declares this".
    // It is answered out of the index the page already built, so this is also
    // the test that the two cannot drift: the same names decide both.
    let mut state = AppState::default();
    state.model_library_manager.clear();
    let mut library = ModelLibrary::new("foundry");
    library.add_model(DeviceModel::new(
        "nch",
        crate::state::model_library::ModelType::Nmos,
    ));
    state.model_library_manager.add_library(library);
    state.workspace.ensure_active_buffer();
    let mut schematic = state
        .workspace
        .active_schematic()
        .cloned()
        .unwrap_or_default();
    for (id, name, model) in [(1u64, "M1", "nch"), (2, "M2", "comparator_fast")] {
        let mut component = crate::state::Component::new(
            id,
            crate::state::ComponentType::Nmos,
            crate::state::Point::new(id as i32, 0),
        );
        component.name = name.to_owned();
        component.value = model.to_owned();
        schematic.components.push(component);
    }
    state.workspace.save_active_schematic(&schematic);

    let definitions = definition_index(&state);
    let unbound = unbound_instances(&state, &definitions);
    assert_eq!(unbound.len(), 1, "only the name nothing declares is listed");
    assert_eq!(unbound[0].instance, "M2");
    assert_eq!(unbound[0].reference, "comparator_fast");
    assert_eq!(unbound[0].component_id, 2);
}

#[test]
fn every_shipped_resolution_rule_names_an_owner() {
    // The card is a claim about who refuses. A row with an empty owner is a
    // rule with nowhere to check it, which is how the mockup's four
    // "run plan" attributions survived: nobody could look them up.
    for (rule, value, owner) in RESOLUTION_RULES {
        assert!(!rule.is_empty() && !value.is_empty() && !owner.is_empty());
    }
    assert!(
        RESOLUTION_RULES
            .iter()
            .any(|(rule, value, _)| rule.contains("search path") && value.starts_with("none")),
        "the absence of a host search path is stated as policy, since it is the reason \
         the mockup's search-path table cannot exist"
    );
}

/// Two libraries, one contested definition, one unresolved instance, rendered.
///
/// Run with `--ignored`; the PNG goes to `RSPICE_RASTER_DIR` (default: the
/// system temp directory).
#[test]
#[ignore = "writes a PNG for a human to look at; run with --ignored"]
fn render_a_populated_include_page() {
    use crate::state::model_library::ModelType;

    let mut state = AppState::default();
    state.model_library_manager.clear();
    for (name, models) in [
        (
            "demo180_corners",
            &[("nch_core", "tt"), ("pch_core", "tt"), ("nch_hv", "ff")][..],
        ),
        (
            "vendor_opa189",
            &[("nch_core", "tt"), ("comparator_slow", "tt")][..],
        ),
    ] {
        let mut library = ModelLibrary::new(name);
        let root = std::path::PathBuf::from(format!("/pdk/{name}.lib"));
        let included = std::path::PathBuf::from(format!("/pdk/{name}_cards.inc"));
        library.root_path = Some(root.clone());
        // A retained closure of two files, so the graph pane draws a real
        // dependency rather than its own empty state.
        library.source_closure = [&root, &included]
            .into_iter()
            .enumerate()
            .map(
                |(index, path)| crate::state::model_library::ModelSourcePin {
                    path: path.clone(),
                    digest: crate::product::ContentDigest::from_bytes(
                        [(index as u8).wrapping_add(name.len() as u8); 32],
                    ),
                },
            )
            .collect();
        library.source_edges = vec![crate::state::model_library::ModelSourceEdge {
            owner: root,
            requested_path: format!("{name}_cards.inc"),
            target: included,
        }];
        for (model, section) in models {
            let mut card = DeviceModel::new(*model, ModelType::Nmos);
            card.section = Some((*section).to_owned());
            library.add_model(card);
        }
        state.model_library_manager.add_library(library);
    }
    state.workspace.ensure_active_buffer();
    let mut schematic = state
        .workspace
        .active_schematic()
        .cloned()
        .unwrap_or_default();
    for (id, name, model) in [
        (1u64, "M1", "nch_core"),
        (2, "M2", "nch_hv"),
        (3, "XCOMP1", "comparator_fast"),
    ] {
        let mut component = crate::state::Component::new(
            id,
            crate::state::ComponentType::Nmos,
            crate::state::Point::new(id as i32 * 2, 0),
        );
        component.name = name.to_owned();
        component.value = model.to_owned();
        schematic.components.push(component);
    }
    state.workspace.save_active_schematic(&schematic);

    let facts = closure_facts(state.model_library_manager.libraries_sorted());
    let mut pending = Vec::new();
    let canvas = crate::ui::raster::render(egui::vec2(1180.0, 980.0), |ui, background| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(background))
            .show(ui, |ui| {
                let mut app = ManagerRenderContext {
                    state: &mut state,
                    pending_actions: &mut pending,
                };
                include_page(ui, &mut app, &facts);
            });
    });

    use std::io::Write as _;
    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let path = directory.join("laneD-04-include-graph.png");
    let height = canvas.content_height().max(200);
    std::fs::write(&path, canvas.png(height)).expect("write png");
    writeln!(
        std::io::stderr(),
        "wrote {} ({}x{height})",
        path.display(),
        canvas.width()
    )
    .ok();
}
