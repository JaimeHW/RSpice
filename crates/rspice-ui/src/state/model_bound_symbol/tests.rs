//! Model-bound symbol tests.

use super::*;

fn definition(revision: u64) -> ModelBoundSymbolDefinition {
    let model = SymbolModelReference::new("vendor", "OPA189_A").with_source_path(
        std::env::current_dir()
            .unwrap()
            .join("models/opa189.lib")
            .display()
            .to_string(),
    );
    let pins = vec![
        SymbolPinDefinition::new(
            "INP",
            SymbolElectricalType::Analog,
            PortDirection::In,
            SymbolPinSide::Left,
            1,
        ),
        SymbolPinDefinition::new(
            "INN",
            SymbolElectricalType::Analog,
            PortDirection::In,
            SymbolPinSide::Left,
            2,
        ),
        SymbolPinDefinition::new(
            "VSS",
            SymbolElectricalType::Power,
            PortDirection::Supply,
            SymbolPinSide::Bottom,
            3,
        ),
        SymbolPinDefinition::new(
            "OUT",
            SymbolElectricalType::Analog,
            PortDirection::Out,
            SymbolPinSide::Right,
            4,
        ),
        SymbolPinDefinition::new(
            "VDD",
            SymbolElectricalType::Power,
            PortDirection::Supply,
            SymbolPinSide::Top,
            5,
        ),
    ];
    let ports = pins.iter().map(SymbolPinDefinition::port_spec).collect();
    let form = SymbolParameterForm {
        revision,
        sections: vec![SymbolParameterSection {
            key: "electrical".to_owned(),
            label: "Electrical parameters".to_owned(),
            help: "Model-safe electrical overrides.".to_owned(),
            fields: vec![SymbolParameterField {
                key: "gain".to_owned(),
                label: "Gain".to_owned(),
                help: "Closed-loop gain override.".to_owned(),
                property_type: PropertyType::Number,
                default: SymbolParameterDefault::Number {
                    engineering: "100".to_owned(),
                    unit: None,
                },
                unit: None,
                constraints: SymbolParameterConstraints {
                    minimum: Some("1".to_owned()),
                    maximum: Some("1Meg".to_owned()),
                    enum_values: Vec::new(),
                    max_length: None,
                },
                inheritance: ParameterInheritance::InstanceOverride,
                visibility: SymbolParameterVisibility::Visible,
                required: true,
                aliases: vec!["av".to_owned()],
            }],
        }],
    };
    ModelBoundSymbolDefinition::new(
        SymbolIdentity::new(
            "analog_blocks",
            "precision_opamp",
            revision,
            "symbol-opa189",
        ),
        SymbolSourceContract::model(model.clone(), ports),
        pins,
        SymbolGraphicTemplate::OperationalAmplifier5Pin,
        form,
        SymbolNetlistBinding {
            device_prefix: "X".to_owned(),
            model: Some(model),
            template: "X{name} {nodes} {model} {params}".to_owned(),
            parameter_order: vec!["gain".to_owned()],
        },
        GeneratedSymbolViews::default(),
    )
}

#[test]
fn strict_definition_round_trip_preserves_contract_and_metadata_projection() {
    let definition = definition(1);
    let encoded = definition.to_json_pretty().unwrap();
    let restored =
        ModelBoundSymbolDefinition::from_json_bytes(encoded.as_bytes(), "opamp.json").unwrap();
    assert_eq!(restored, definition);

    let mut view = View::new("symbol", ViewType::Symbol);
    restored.store_in_view(&mut view).unwrap();
    assert_eq!(
        ModelBoundSymbolDefinition::load_from_view(&view).unwrap(),
        Some(definition)
    );
    assert!(view.metadata["cdf.parameter_contract"].contains("\"default\":\"100\""));
    assert_eq!(view.metadata["model.family"], "OPA189_A");
}

#[test]
fn duplicate_names_orders_and_gaps_are_rejected() {
    let mut duplicate_name = definition(1);
    duplicate_name.pins[1].name = "inp".to_owned();
    assert!(matches!(
        duplicate_name.validate(),
        Err(SymbolDefinitionError::DuplicatePin(_))
    ));

    let mut duplicate_order = definition(1);
    duplicate_order.pins[1].order = 1;
    assert_eq!(
        duplicate_order.validate(),
        Err(SymbolDefinitionError::DuplicatePinOrder(1))
    );

    let mut gap = definition(1);
    gap.pins[4].order = 6;
    assert!(matches!(
        gap.validate(),
        Err(SymbolDefinitionError::NonContiguousPinOrder { .. })
    ));
}

#[test]
fn malformed_or_extended_import_is_rejected_without_coercion() {
    let encoded = definition(1).to_json_pretty().unwrap();
    let with_unknown = encoded.replacen(
        "\"schema_version\": 1,",
        "\"schema_version\": 1, \"future_behavior\": true,",
        1,
    );
    assert!(
        ModelBoundSymbolDefinition::from_json_bytes(with_unknown.as_bytes(), "bad.json").is_err()
    );
    assert!(SymbolDefinitionImport::from_bytes(b"not a symbol", "bad.svg", None).is_err());
}

#[test]
fn construction_plan_is_atomic_stale_guarded_and_reversible() {
    let mut library = Library::new("analog_blocks");
    let before = serde_json::to_string(&library).unwrap();
    let plan = definition(1).build_plan(&library).unwrap();
    assert_eq!(
        serde_json::to_string(&library).unwrap(),
        before,
        "planning is read-only"
    );

    library.add_cell(Cell::new("precision_opamp"));
    assert!(matches!(
        plan.commit(&mut library),
        Err(SymbolDefinitionError::StaleTarget(_))
    ));
    assert_eq!(library.get_cell("precision_opamp").unwrap().view_count(), 0);

    library.remove_cell("precision_opamp");
    let receipt = definition(1)
        .build_plan(&library)
        .unwrap()
        .commit(&mut library)
        .unwrap();
    assert!(
        library
            .get_cell("precision_opamp")
            .unwrap()
            .get_view("symbol")
            .is_some()
    );
    receipt.undo(&mut library).unwrap();
    assert!(library.get_cell("precision_opamp").is_none());
}

#[test]
fn parameter_form_reports_field_addressed_type_and_constraint_failures() {
    let mut definition = definition(1);
    let field = &mut definition.parameter_form.sections[0].fields[0];
    field.default = SymbolParameterDefault::String {
        value: "wrong".to_owned(),
    };
    field.help.clear();

    let diagnostics = definition.parameter_form.validate_diagnostics();
    assert!(
        diagnostics
            .iter()
            .all(|finding| finding.field_key.as_deref() == Some("gain"))
    );
    assert!(
        diagnostics
            .iter()
            .any(|finding| finding.message.contains("typed default"))
    );
}

#[test]
fn parameter_form_projects_ordered_typed_inheritance_and_constraints() {
    let mut form = definition(1).parameter_form;
    form.sections[0].fields.push(SymbolParameterField {
        key: "label".to_owned(),
        label: "Instance label".to_owned(),
        help: "Immutable cell-authored label.".to_owned(),
        property_type: PropertyType::String,
        default: SymbolParameterDefault::String {
            value: "precision".to_owned(),
        },
        unit: None,
        constraints: SymbolParameterConstraints {
            max_length: Some(16),
            ..Default::default()
        },
        inheritance: ParameterInheritance::CellDefault,
        visibility: SymbolParameterVisibility::Advanced,
        required: true,
        aliases: Vec::new(),
    });
    form.sections[0].fields.push(SymbolParameterField {
        key: "corner".to_owned(),
        label: "Model corner".to_owned(),
        help: "Authoritative default supplied by the referenced model.".to_owned(),
        property_type: PropertyType::Enum,
        default: SymbolParameterDefault::Enum {
            selected: "tt".to_owned(),
        },
        unit: None,
        constraints: SymbolParameterConstraints {
            enum_values: vec!["tt".to_owned(), "ss".to_owned(), "ff".to_owned()],
            ..Default::default()
        },
        inheritance: ParameterInheritance::ModelDefault,
        visibility: SymbolParameterVisibility::Visible,
        required: false,
        aliases: Vec::new(),
    });

    let sheet = form.to_property_sheet().unwrap();
    assert_eq!(sheet.names(), ["gain", "label", "corner"]);
    assert!(!sheet.get("gain").unwrap().read_only);
    let label = sheet.get("label").unwrap();
    assert!(label.read_only);
    assert_eq!(label.max_length, Some(16));
    assert_eq!(label.display_mode, DisplayMode::Advanced);
    assert_eq!(label.category, "Electrical parameters");
    assert!(sheet.get("corner").unwrap().read_only);
    assert_eq!(form.netlist_parameter_order(), ["gain", "label"]);

    let mut invalid = form.clone();
    invalid.sections[0].fields[1].constraints.max_length = Some(4);
    assert!(invalid.validate().is_err());
}

#[test]
fn replacing_form_synchronizes_the_exact_emitted_parameter_order() {
    let original = definition(1);
    let mut replacement = original.parameter_form.clone();
    replacement.revision = 2;
    replacement.sections[0].fields.push(SymbolParameterField {
        key: "cell_gain".to_owned(),
        label: "Cell gain".to_owned(),
        help: "Immutable cell-authored gain.".to_owned(),
        property_type: PropertyType::Number,
        default: SymbolParameterDefault::Number {
            engineering: "2".to_owned(),
            unit: None,
        },
        unit: None,
        constraints: SymbolParameterConstraints::default(),
        inheritance: ParameterInheritance::CellDefault,
        visibility: SymbolParameterVisibility::Visible,
        required: true,
        aliases: Vec::new(),
    });
    replacement.sections[0].fields.push(SymbolParameterField {
        key: "model_gain".to_owned(),
        label: "Model gain".to_owned(),
        help: "Gain retained from the model.".to_owned(),
        property_type: PropertyType::Number,
        default: SymbolParameterDefault::Number {
            engineering: "3".to_owned(),
            unit: None,
        },
        unit: None,
        constraints: SymbolParameterConstraints::default(),
        inheritance: ParameterInheritance::ModelDefault,
        visibility: SymbolParameterVisibility::Visible,
        required: false,
        aliases: Vec::new(),
    });

    let replaced = original.replace_parameter_form(replacement).unwrap();
    assert_eq!(replaced.identity.revision, 2);
    assert_eq!(replaced.netlist.parameter_order, ["gain", "cell_gain"]);

    let mut view = View::new("symbol", ViewType::Symbol);
    replaced.store_in_view(&mut view).unwrap();
    assert!(view.metadata["cdf.parameter_contract"].contains("cell_gain"));
    assert!(!view.metadata["cdf.parameter_contract"].contains("model_gain"));
    assert!(view.metadata["netlist.cell_defaults"].contains("cell_gain"));
    assert!(view.metadata["cdf.parameter_inheritance"].contains("model_default"));
}

#[test]
fn svg_import_retains_geometry_but_never_infers_electrical_semantics() {
    let svg = br#"<svg xmlns="http://www.w3.org/2000/svg"><rect x="0" y="0" width="10" height="10"/></svg>"#;
    assert!(SymbolDefinitionImport::from_bytes(svg, "symbol.svg", None).is_err());
    let imported =
        SymbolDefinitionImport::from_bytes(svg, "symbol.svg", Some(definition(1))).unwrap();
    assert_eq!(imported.report.primitive_count, 1);
    assert_eq!(imported.report.explicit_pin_anchor_count, 0);
    assert_eq!(
        imported
            .definition
            .imported_graphic
            .as_ref()
            .unwrap()
            .source
            .as_bytes(),
        svg
    );
}

#[test]
fn netlist_render_strips_only_the_declared_reference_prefix() {
    let mut definition = definition(1);
    definition.netlist.device_prefix = "M".to_owned();
    definition.netlist.template = "M{name} {nodes} {model} {params}".to_owned();
    let nodes = ["d", "g", "s", "b", "bulk"].map(str::to_owned).to_vec();
    let parameters = HashMap::from([("gain".to_owned(), "10".to_owned())]);

    let rendered = definition
        .netlist
        .render(
            "MFOO",
            &definition.pins,
            &nodes,
            &parameters,
            &definition.parameter_form,
        )
        .unwrap();
    assert_eq!(rendered, "MFOO d g s b bulk OPA189_A gain=10");
    assert!(
        definition
            .netlist
            .render(
                "FOO",
                &definition.pins,
                &nodes,
                &parameters,
                &definition.parameter_form,
            )
            .is_err()
    );
    assert!(
        definition
            .netlist
            .render(
                "M",
                &definition.pins,
                &nodes,
                &parameters,
                &definition.parameter_form,
            )
            .is_err()
    );
    assert_eq!(
        definition
            .test_fixture_contract()
            .unwrap()
            .dut_instance_name,
        "MDUT"
    );
}

#[test]
fn canonical_extension_and_ltspice_review_anchors_are_preserved() {
    let encoded = definition(1).to_json_pretty().unwrap();
    let canonical =
        SymbolDefinitionImport::from_bytes(encoded.as_bytes(), "opamp.rspicesym", None).unwrap();
    assert_eq!(canonical.report.format, SymbolImportFormat::RSpiceJson);
    assert!(canonical.report.binding_valid);

    let asy = b"Version 4\nSymbolType CELL\nLINE Normal 0 0 32 0\nRECTANGLE Normal 0 0 32 32\nPIN 0 16 LEFT 8\nPINATTR PinName IN\nPINATTR SpiceOrder 1\nSYMATTR Prefix X\nSYMATTR Value REVIEW_ONLY\n";
    let imported = SymbolDefinitionImport::from_bytes(
        asy,
        "review.asy",
        Some(ModelBoundSymbolDefinition::review_only("scratch", "review")),
    )
    .unwrap();
    assert_eq!(imported.report.format, SymbolImportFormat::LtspiceAsy);
    assert_eq!(imported.report.primitive_count, 2);
    assert_eq!(imported.report.explicit_pin_anchor_count, 1);
    assert!(!imported.report.binding_valid);
    assert!(!imported.definition.netlist.is_executable());

    let document = imported.definition.symbol_document();
    assert_eq!(document.pins.len(), 1);
    assert_eq!(document.pins[0].name, "IN");
    assert_eq!(document.pins[0].position, Some(Point::new(0, 16)));
}
