//! What the design-inspection key must and must not notice.
//!
//! A key that misses an input leaves the Bins & geometry page stating bin
//! geometry for cards the deck no longer holds — a wrong answer, not a slow
//! one. A key that notices something no deck reads costs the page a full engine
//! re-inspection for nothing. Both directions are asserted case by case.

use super::*;

/// One named way to vary a catalogue, applied to a clone of the fixture.
///
/// The name is the sentence the failure message finishes, and the argument is
/// the fixture's library name.
type CatalogVariation = (&'static str, Box<dyn Fn(&mut ModelLibraryManager, &str)>);

/// A catalogue holding one imported library with a model, a subcircuit, and a
/// process corner.
///
/// Imported rather than assembled, so the source authority, pinned closure,
/// retained bytes, and parsed namespaces are the ones a real import produces
/// and agree with each other the way the sealing path insists they do. An
/// import mints a fresh source identity every time, so the cases below vary a
/// *clone* of one fixture rather than building a second one.
fn inspection_key_catalog() -> (ModelLibraryManager, String) {
    let mut manager = ModelLibraryManager::new();
    let library_name = manager
        .load_library_bytes(
            "inspection-key.lib",
            b".model nch NMOS (LEVEL=1 VTO=0.4)\n.subckt inv a y\nm1 y a 0 0 nch\n.ends\n".to_vec(),
            None,
        )
        .expect("the inspection-key catalogue imports");
    let library = manager
        .get_library_mut(&library_name)
        .expect("the imported library is retained");
    for corner in ProcessCorner::standard_corners() {
        library.corners.insert(corner.name.clone(), corner);
    }
    (manager, library_name)
}

#[test]
fn design_inspection_catalog_key_depends_on_content_and_not_on_map_iteration_order() {
    // The key is asked for on every frame the Bins & geometry page paints, so
    // a key that moved with `HashMap` iteration order would never hit and the
    // page would redo the whole engine inspection forever. Re-collecting a map
    // rebuilds it behind an independent hasher, which is what happens whenever
    // the catalogue is reconstructed rather than cloned.
    let (mut manager, library_name) = inspection_key_catalog();
    let baseline = manager.design_inspection_catalog_key();
    assert_eq!(
        manager.design_inspection_catalog_key(),
        baseline,
        "asking twice about an untouched catalogue must give one answer"
    );
    assert_eq!(
        manager.clone().design_inspection_catalog_key(),
        baseline,
        "a clone holds the same content and must present the same key"
    );

    let library = manager
        .get_library_mut(&library_name)
        .expect("library retained");
    library.models = library
        .models
        .clone()
        .into_iter()
        .collect::<HashMap<_, _>>();
    library.subcircuits = library
        .subcircuits
        .clone()
        .into_iter()
        .collect::<HashMap<_, _>>();
    library.corners = library
        .corners
        .clone()
        .into_iter()
        .collect::<HashMap<_, _>>();

    assert_eq!(
        manager.design_inspection_catalog_key(),
        baseline,
        "the inspection key must depend on content, never on map iteration order"
    );
}

#[test]
fn design_inspection_catalog_key_moves_with_every_catalogue_input_the_deck_reads() {
    // One case per thing `prepare_design_netlist_for_inspection` reads out of
    // this catalogue. A key that missed any of them would leave the Bins &
    // geometry page stating bin geometry for cards the deck no longer holds,
    // which is worse than the cost the key saves.
    let (baseline_manager, library_name) = inspection_key_catalog();
    let baseline = baseline_manager.design_inspection_catalog_key();
    let name = library_name.clone();

    let mutations: Vec<CatalogVariation> = vec![
        (
            "a second library entered the catalogue",
            Box::new(|manager, _| manager.add_library(ModelLibrary::new("second"))),
        ),
        (
            "the library was removed",
            Box::new(|manager, name| {
                manager.remove_library(name);
            }),
        ),
        (
            "the library was renamed",
            Box::new(|manager, name| {
                let mut library = manager.get_library(name).expect("library retained").clone();
                manager.remove_library(name);
                library.name = "renamed".to_owned();
                manager.add_library(library);
            }),
        ),
        (
            "the root identity moved",
            Box::new(|manager, name| {
                manager.get_library_mut(name).expect("retained").root_path =
                    Some(PathBuf::from("/elsewhere/inspection-key.lib"));
            }),
        ),
        (
            "the source authority changed",
            Box::new(|manager, name| {
                manager
                    .get_library_mut(name)
                    .expect("retained")
                    .source_authority = ModelSourceAuthority::External;
            }),
        ),
        (
            "a pinned source digest changed",
            Box::new(|manager, name| {
                manager
                    .get_library_mut(name)
                    .expect("retained")
                    .source_closure[0]
                    .digest = ContentDigest::from_bytes(Sha256::digest(b"other bytes").into());
            }),
        ),
        (
            "a dependency-resolution edge was recorded",
            Box::new(|manager, name| {
                manager
                    .get_library_mut(name)
                    .expect("retained")
                    .source_edges = vec![ModelSourceEdge {
                    owner: PathBuf::from("/a.lib"),
                    requested_path: "b.lib".to_owned(),
                    target: PathBuf::from("/b.lib"),
                }];
            }),
        ),
        (
            "retained bytes changed length",
            Box::new(|manager, name| {
                manager
                    .get_library_mut(name)
                    .expect("retained")
                    .source_contents[0]
                    .bytes
                    .extend_from_slice(b"\n.model extra NMOS (LEVEL=1)\n");
            }),
        ),
        (
            "a corner section binding changed",
            Box::new(|manager, name| {
                let library = manager.get_library_mut(name).expect("retained");
                let corner = library
                    .corners
                    .values_mut()
                    .next()
                    .expect("standard corners exist");
                corner.section_bindings.clear();
                corner.description = "rebound".to_owned();
            }),
        ),
        (
            "the execution-active corner changed",
            Box::new(|manager, name| {
                manager
                    .get_library_mut(name)
                    .expect("retained")
                    .selected_corner = Some("ff".to_owned());
            }),
        ),
        (
            "a model definition entered the namespace",
            Box::new(|manager, name| {
                manager
                    .get_library_mut(name)
                    .expect("retained")
                    .add_model(DeviceModel::new("pch", ModelType::Pmos));
            }),
        ),
        (
            "a model definition left the namespace",
            Box::new(|manager, name| {
                manager
                    .get_library_mut(name)
                    .expect("retained")
                    .models
                    .remove("nch");
            }),
        ),
        (
            "a subcircuit moved into a section",
            Box::new(|manager, name| {
                manager
                    .get_library_mut(name)
                    .expect("retained")
                    .subcircuits
                    .values_mut()
                    .next()
                    .expect("the imported subcircuit")
                    .section = Some("ff".to_owned());
            }),
        ),
        (
            "an explicit provider decision was recorded",
            Box::new(|manager, name| {
                let record = ModelResolutionRecord {
                    schema_version: MODEL_RESOLUTION_RECORD_SCHEMA_VERSION,
                    consumer_scope: ModelConsumerScope::PrimitiveModel,
                    normalized_name: "nch".to_owned(),
                    provider_library: name.to_owned(),
                    provider_definition: "nch".to_owned(),
                    provider_source_digest: ContentDigest::from_bytes(
                        Sha256::digest(b"provider").into(),
                    ),
                    audit_reason: "directed test".to_owned(),
                    created_at_unix_ms: 1,
                };
                manager.resolution_records.insert(record.key(), record);
            }),
        ),
    ];

    for (what, mutate) in mutations {
        let mut manager = baseline_manager.clone();
        mutate(&mut manager, &name);
        assert_ne!(
            manager.design_inspection_catalog_key(),
            baseline,
            "the design-inspection key did not move when {what}; a cached model-bin \
             inspection would outlive the catalogue it was taken from"
        );
    }
}

#[test]
fn design_inspection_catalog_key_ignores_what_no_deck_can_read() {
    // The exclusions are the point. Every one of these used to expire the
    // cached engine inspection because the old key serialized each library
    // whole, so opening a browser row or editing a description cost a full
    // re-inspection of the design.
    let (baseline_manager, library_name) = inspection_key_catalog();
    let baseline = baseline_manager.design_inspection_catalog_key();
    let name = library_name.clone();

    let irrelevant: Vec<CatalogVariation> = vec![
        (
            "the browser filter text",
            Box::new(|manager, _| manager.filter_text = "nch".to_owned()),
        ),
        (
            "the browser's selected library",
            Box::new(|manager, name| manager.selected_library = Some(name.to_owned())),
        ),
        (
            "the browser's type filter",
            Box::new(|manager, _| manager.filter_type = Some(ModelType::Nmos)),
        ),
        (
            "a library expanded in the browser",
            Box::new(|manager, name| {
                manager.get_library_mut(name).expect("retained").expanded = true;
            }),
        ),
        (
            "the recorded foundry and node",
            Box::new(|manager, name| {
                let library = manager.get_library_mut(name).expect("retained");
                library.pdk_name = "foundry".to_owned();
                library.technology_node = "28nm".to_owned();
                library.version = "2".to_owned();
            }),
        ),
        (
            // The deck materializes cards from the sealed source bytes, so a
            // parsed parameter is a view of the catalogue, never an input to
            // it. Only the definition's name decides anything.
            "a parsed parameter value",
            Box::new(|manager, name| {
                let model = manager
                    .get_library_mut(name)
                    .expect("retained")
                    .models
                    .get_mut("nch")
                    .expect("imported model");
                model.vth0 = Some(0.9);
                model.description = "edited".to_owned();
            }),
        ),
    ];

    for (what, mutate) in irrelevant {
        let mut manager = baseline_manager.clone();
        mutate(&mut manager, &name);
        assert_eq!(
            manager.design_inspection_catalog_key(),
            baseline,
            "the design-inspection key moved for {what}, which no prepared deck reads; \
             the Bins & geometry page will re-inspect the design for nothing"
        );
    }
}
