//! Cell bindings: persistence, validation, cloning and library renames.

use super::*;

/// A catalog exactly as the release before cell bindings wrote it: one
/// configuration and a lineage clone, compact JSON, digests as stored.
const CATALOG_WITHOUT_CELL_BINDINGS: &str = r#"{"schema_version":2,"configurations":[{"id":"5a990559-0eac-45c2-927a-f0da01afd4ef","revision":1,"lineage":null,"semantic_digest":"62c96728dae3fa8deec885434afeabba7b0d5eec1be25fb5ccfb53b97409f51d","definition":{"name":"Release","root":{"library":"user","cell":"top_tb","view":"testbench"},"dut_path":"/XAFE","executable_view_policy":["schematic","spice"],"stop_views":["spice"],"unresolved_policy":"explicit-fallback-with-review","black_box_policy":"materialized-source-boundaries-only","overrides":[{"instance_path":"/XAFE/XADC","executable_views":["spice"],"stop_view":"spice","model_section":"tt","eligible_platforms":["desktop"]}],"model_profile":"project-run-set-sections","owner":"Analog design"}},{"id":"21fbc4d5-4a69-4407-b8a6-b0c1b3729083","revision":1,"lineage":{"source_id":"5a990559-0eac-45c2-927a-f0da01afd4ef","source_revision":1},"semantic_digest":"6f193767e830ba825206d3c9c8f9e50353453fc663707b2526a1c98c4610d54e","definition":{"name":"Post-layout","root":{"library":"user","cell":"top_tb","view":"testbench"},"dut_path":"/XAFE","executable_view_policy":["schematic","spice"],"stop_views":["spice"],"unresolved_policy":"explicit-fallback-with-review","black_box_policy":"materialized-source-boundaries-only","overrides":[{"instance_path":"/XAFE/XADC","executable_views":["spice"],"stop_view":"spice","model_section":"tt","eligible_platforms":["desktop"]}],"model_profile":"project-run-set-sections","owner":"Analog design"}}],"active_configuration_id":"5a990559-0eac-45c2-927a-f0da01afd4ef"}"#;

fn definition(name: &str) -> ConfigurationSetDefinition {
    ConfigurationSetDefinition {
        name: name.to_owned(),
        root: CellViewRef::new("user", "top_tb", "testbench"),
        dut_path: "/XAFE".to_owned(),
        executable_view_policy: vec!["schematic".to_owned(), "spice".to_owned()],
        stop_views: vec!["spice".to_owned()],
        unresolved_policy: UnresolvedBindingPolicy::BlockNetlist,
        black_box_policy: ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
        overrides: Vec::new(),
        cell_bindings: Vec::new(),
        model_profile: ConfigurationModelProfile::ProjectRunSetSections,
        owner: "Analog design".to_owned(),
    }
}

fn binding(library: &str, cell: &str, views: &[&str]) -> ConfigurationCellBinding {
    ConfigurationCellBinding {
        library: library.to_owned(),
        cell: cell.to_owned(),
        executable_views: views.iter().map(|view| (*view).to_owned()).collect(),
        stop_view: None,
    }
}

fn with_bindings(bindings: Vec<ConfigurationCellBinding>) -> ConfigurationSetDefinition {
    let mut definition = definition("Release");
    definition.cell_bindings = bindings;
    definition
}

#[test]
fn a_catalog_written_before_cell_bindings_keeps_its_bytes_and_digests() {
    let catalog: ConfigurationSetCatalog =
        serde_json::from_str(CATALOG_WITHOUT_CELL_BINDINGS).expect("the stored catalog loads");
    catalog.validate().expect("the stored catalog validates");
    assert_eq!(
        serde_json::to_string(&catalog).expect("the catalog serializes"),
        CATALOG_WITHOUT_CELL_BINDINGS,
        "a definition without cell bindings is written exactly as before"
    );
    let stored: serde_json::Value =
        serde_json::from_str(CATALOG_WITHOUT_CELL_BINDINGS).expect("stored JSON");
    for (index, configuration) in catalog.configurations().iter().enumerate() {
        assert!(configuration.cell_bindings().is_empty());
        assert_eq!(
            serde_json::to_value(configuration.semantic_digest()).expect("digest"),
            stored["configurations"][index]["semantic_digest"],
            "the stored digest is the digest of the loaded definition"
        );
    }

    // Only a binding the author adds reaches the bytes and the digest.
    let mut edited = catalog.clone();
    let configuration = &edited.configurations()[0];
    let (id, revision) = (configuration.id(), configuration.revision());
    let previous_digest = configuration.semantic_digest();
    let mut definition = configuration.definition().clone();
    definition.cell_bindings = vec![binding("user", "adc", &["spice"])];
    edited
        .update(id, revision, definition)
        .expect("add a binding");
    let written = serde_json::to_string(&edited).expect("the edited catalog serializes");
    assert!(written.contains(r#""cell_bindings":[{"library":"user","cell":"adc""#));
    assert_ne!(edited.find(id).unwrap().semantic_digest(), previous_digest);
    let reloaded: ConfigurationSetCatalog =
        serde_json::from_str(&written).expect("the edited catalog reloads");
    assert_eq!(reloaded, edited);
}

#[test]
fn cell_bindings_are_normalized_into_one_stored_order() {
    let mut first = binding(" user ", "Ladder", &[" Spice ", "schematic"]);
    first.stop_view = Some(" SPICE ".to_owned());
    let second = binding("analog", "amp", &["schematic"]);
    let third = binding("USER", "adc", &["spice"]);
    let mut catalog = ConfigurationSetCatalog::default();
    let id = catalog
        .create(with_bindings(vec![first, second, third]))
        .expect("create");

    let stored = catalog.find(id).unwrap().cell_bindings();
    let cells: Vec<(&str, &str)> = stored
        .iter()
        .map(|binding| (binding.library.as_str(), binding.cell.as_str()))
        .collect();
    assert_eq!(
        cells,
        [("analog", "amp"), ("USER", "adc"), ("user", "Ladder")]
    );
    assert_eq!(stored[2].executable_views, ["spice", "schematic"]);
    assert_eq!(stored[2].stop_view.as_deref(), Some("spice"));

    // The author's order is not part of the digest: the same bindings typed
    // in another order are the same configuration.
    let mut reordered = ConfigurationSetCatalog::default();
    let reordered_id = reordered
        .create(with_bindings(stored.iter().rev().cloned().collect()))
        .expect("create reordered");
    assert_eq!(
        reordered.find(reordered_id).unwrap().semantic_digest(),
        catalog.find(id).unwrap().semantic_digest()
    );
}

#[test]
fn a_binding_for_a_cell_no_library_holds_is_kept() {
    let mut catalog = ConfigurationSetCatalog::default();
    let id = catalog
        .create(with_bindings(vec![binding("retired", "ghost", &["spice"])]))
        .expect("a binding names a cell, not a library entry");
    assert_eq!(catalog.find(id).unwrap().cell_bindings().len(), 1);
}

#[test]
fn every_invalid_cell_binding_is_refused_without_changing_the_catalog() {
    let mut catalog = ConfigurationSetCatalog::default();
    catalog.create(definition("Existing")).expect("existing");
    let before = catalog.clone();

    let mut outside = binding("user", "adc", &["schematic"]);
    outside.stop_view = Some("spice".to_owned());
    let mut misnamed_stop = binding("user", "adc", &["schematic"]);
    misnamed_stop.stop_view = Some("spice tt".to_owned());
    let too_many_views: Vec<String> = (0..=MAX_CONFIGURATION_VIEW_POLICY_ENTRIES)
        .map(|index| format!("view_{index}"))
        .collect();
    let mut long_list = binding("user", "adc", &[]);
    long_list.executable_views = too_many_views;

    let cases: Vec<(&str, Vec<ConfigurationCellBinding>, ConfigurationSetError)> = vec![
        (
            "library name",
            vec![binding("user-lib", "adc", &["spice"])],
            ConfigurationSetError::InvalidCellBindingName {
                field: "configuration.cell-binding.library",
                name: "user-lib".to_owned(),
                reason: "contains unsupported character '-'; only letters, numbers, and underscores are allowed".to_owned(),
            },
        ),
        (
            "cell name",
            vec![binding("user", "  ", &["spice"])],
            ConfigurationSetError::InvalidCellBindingName {
                field: "configuration.cell-binding.cell",
                name: String::new(),
                reason: "must not be empty".to_owned(),
            },
        ),
        (
            "duplicate cell",
            vec![
                binding("user", "adc", &["spice"]),
                binding("USER", "Adc", &["schematic"]),
            ],
            ConfigurationSetError::DuplicateCellBinding {
                library: "user".to_owned(),
                cell: "adc".to_owned(),
            },
        ),
        (
            "empty view list",
            vec![binding("user", "adc", &[])],
            ConfigurationSetError::InvalidViewPolicyLength {
                field: "configuration.cell-binding.executable-views",
                actual: 0,
                maximum: MAX_CONFIGURATION_VIEW_POLICY_ENTRIES,
                require_nonempty: true,
            },
        ),
        (
            "view list too long",
            vec![long_list],
            ConfigurationSetError::InvalidViewPolicyLength {
                field: "configuration.cell-binding.executable-views",
                actual: MAX_CONFIGURATION_VIEW_POLICY_ENTRIES + 1,
                maximum: MAX_CONFIGURATION_VIEW_POLICY_ENTRIES,
                require_nonempty: true,
            },
        ),
        (
            "view name",
            vec![binding("user", "adc", &["spice tt"])],
            ConfigurationSetError::InvalidViewName {
                field: "configuration.cell-binding.executable-views",
                view: "spice tt".to_owned(),
                reason: "contains unsupported character ' '; only letters, numbers, and underscores are allowed".to_owned(),
            },
        ),
        (
            "duplicate view",
            vec![binding("user", "adc", &["spice", "SPICE"])],
            ConfigurationSetError::DuplicateView {
                field: "configuration.cell-binding.executable-views",
                view: "spice".to_owned(),
            },
        ),
        (
            "stop view name",
            vec![misnamed_stop],
            ConfigurationSetError::InvalidViewName {
                field: "configuration.cell-binding.stop-view",
                view: "spice tt".to_owned(),
                reason: "contains unsupported character ' '; only letters, numbers, and underscores are allowed".to_owned(),
            },
        ),
        (
            "stop view outside its list",
            vec![outside],
            ConfigurationSetError::StopViewOutsidePolicy {
                field: "configuration.cell-binding.stop-view",
                view: "spice".to_owned(),
            },
        ),
        (
            "binding count",
            (0..=MAX_CONFIGURATION_CELL_BINDINGS)
                .map(|index| binding("user", &format!("cell_{index:05}"), &["spice"]))
                .collect(),
            ConfigurationSetError::CellBindingLimitExceeded {
                actual: MAX_CONFIGURATION_CELL_BINDINGS + 1,
                maximum: MAX_CONFIGURATION_CELL_BINDINGS,
            },
        ),
    ];
    for (case, bindings, expected) in cases {
        let mut definition = definition("Candidate");
        definition.cell_bindings = bindings;
        assert_eq!(catalog.create(definition), Err(expected), "{case}");
        assert_eq!(catalog, before, "{case}: a refused create changes nothing");
    }

    // An update is refused the same way and leaves the revision alone.
    let id = catalog.configurations()[0].id();
    assert_eq!(
        catalog.update(id, 1, with_bindings(vec![binding("user", "adc", &[])])),
        Err(ConfigurationSetError::InvalidViewPolicyLength {
            field: "configuration.cell-binding.executable-views",
            actual: 0,
            maximum: MAX_CONFIGURATION_VIEW_POLICY_ENTRIES,
            require_nonempty: true,
        })
    );
    assert_eq!(catalog, before);
}

#[test]
fn a_stored_catalog_with_malformed_cell_bindings_fails_closed() {
    let mut catalog = ConfigurationSetCatalog::default();
    catalog
        .create(with_bindings(vec![
            binding("user", "adc", &["spice"]),
            binding("user", "dac", &["spice"]),
        ]))
        .expect("create");
    let stored = serde_json::to_value(&catalog).expect("serialize");
    let path = "/configurations/0/definition/cell_bindings";

    let mut unsorted = stored.clone();
    let bindings = unsorted
        .pointer_mut(path)
        .and_then(serde_json::Value::as_array_mut)
        .expect("stored bindings");
    bindings.reverse();
    let error = serde_json::from_value::<ConfigurationSetCatalog>(unsorted)
        .expect_err("an unsorted stored list is refused");
    assert!(
        error
            .to_string()
            .contains(&ConfigurationSetError::UnsortedCellBindings.to_string()),
        "{error}"
    );

    let mut unknown = stored;
    unknown.pointer_mut(path).expect("stored bindings")[0]["instance_path"] =
        serde_json::json!("/XADC");
    assert!(serde_json::from_value::<ConfigurationSetCatalog>(unknown).is_err());
}

#[test]
fn a_bindings_only_clone_carries_the_cell_bindings() {
    let mut catalog = ConfigurationSetCatalog::default();
    let source = catalog
        .create(with_bindings(vec![binding("user", "adc", &["spice"])]))
        .expect("source");
    let mut defaults = definition("Defaults");
    defaults.cell_bindings = vec![binding("user", "dac", &["schematic"])];

    let bindings_clone = catalog
        .clone_configuration_scoped(
            source,
            1,
            "Bindings",
            ConfigurationCloneScope::BindingsOnly,
            defaults.clone(),
        )
        .expect("bindings clone");
    let environment_clone = catalog
        .clone_configuration_scoped(
            source,
            1,
            "Environment",
            ConfigurationCloneScope::EnvironmentOnly,
            defaults,
        )
        .expect("environment clone");

    assert_eq!(
        catalog.find(bindings_clone).unwrap().cell_bindings(),
        catalog.find(source).unwrap().cell_bindings()
    );
    assert_eq!(
        catalog.find(environment_clone).unwrap().cell_bindings()[0].cell,
        "dac",
        "an environment clone keeps the destination's own bindings"
    );
}

#[test]
fn library_and_cell_renames_carry_the_cell_bindings() {
    let mut catalog = ConfigurationSetCatalog::default();
    let mut stopped = binding("user", "adc", &["spice", "schematic"]);
    stopped.stop_view = Some("spice".to_owned());
    let id = catalog
        .create(with_bindings(vec![
            stopped,
            binding("user", "dac", &["schematic"]),
            binding("analog", "adc", &["spice"]),
        ]))
        .expect("create");

    // The root is not in the renamed cell; the binding alone revises it.
    assert_eq!(catalog.rename_cell_roots("USER", "ADC", "sar_adc"), Ok(1));
    let configuration = catalog.find(id).unwrap();
    assert_eq!(configuration.revision(), 2);
    assert_eq!(configuration.root().cell, "top_tb");
    let cells: Vec<(&str, &str)> = configuration
        .cell_bindings()
        .iter()
        .map(|binding| (binding.library.as_str(), binding.cell.as_str()))
        .collect();
    assert_eq!(
        cells,
        [("analog", "adc"), ("user", "dac"), ("user", "sar_adc")],
        "a renamed binding takes its place in the stored order"
    );
    assert_eq!(
        configuration.cell_bindings()[2].stop_view.as_deref(),
        Some("spice"),
        "a rename moves the binding, not its views"
    );

    assert_eq!(catalog.rename_library_roots("user", "mixed"), Ok(1));
    let configuration = catalog.find(id).unwrap();
    assert_eq!(configuration.root().library, "mixed");
    assert!(
        configuration
            .cell_bindings()
            .iter()
            .all(|binding| binding.library != "user")
    );
    assert_eq!(configuration.cell_bindings()[0].library, "analog");

    // A view rename leaves the views a binding lists as authored.
    assert_eq!(
        catalog.rename_view_roots("mixed", "sar_adc", "spice", "spice_tt"),
        Ok(0)
    );
    catalog.validate().expect("renamed catalog validates");
}

#[test]
fn a_rename_onto_a_cell_another_binding_names_is_refused() {
    let mut catalog = ConfigurationSetCatalog::default();
    catalog
        .create(with_bindings(vec![
            binding("user", "adc", &["spice"]),
            binding("user", "sar_adc", &["schematic"]),
        ]))
        .expect("create");
    let before = catalog.clone();

    assert_eq!(
        catalog.rename_cell_roots("user", "adc", "SAR_ADC"),
        Err(ConfigurationSetError::DuplicateCellBinding {
            library: "user".to_owned(),
            cell: "sar_adc".to_owned(),
        })
    );
    assert_eq!(catalog, before, "a refused rename changes nothing");
}
