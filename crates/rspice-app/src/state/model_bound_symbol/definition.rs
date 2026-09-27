//! Materialization and atomic view/library publication of model-bound definitions.

use super::*;

pub fn materialize_symbol_document(definition: &ModelBoundSymbolDefinition) -> SymbolDocument {
    let half_width = 40;
    let half_height = 40.max(((definition.pins.len() as i32 + 1) / 2) * 10);
    let body = if let Some(imported) = &definition.imported_graphic {
        imported.shapes.clone()
    } else {
        match definition.graphic_template {
            SymbolGraphicTemplate::OperationalAmplifier5Pin => vec![SymbolShape::Polyline {
                points: vec![
                    Point::new(-half_width, -half_height),
                    Point::new(half_width, 0),
                    Point::new(-half_width, half_height),
                ],
                closed: true,
            }],
            SymbolGraphicTemplate::RectangularIc | SymbolGraphicTemplate::RfNPort => {
                vec![SymbolShape::Polyline {
                    points: vec![
                        Point::new(-half_width, -half_height),
                        Point::new(half_width, -half_height),
                        Point::new(half_width, half_height),
                        Point::new(-half_width, half_height),
                    ],
                    closed: true,
                }]
            }
        }
    };
    if let Some(imported) = &definition.imported_graphic
        && !imported.pin_anchors.is_empty()
    {
        let pins =
            if definition.pins.is_empty() {
                imported
                    .pin_anchors
                    .iter()
                    .map(|anchor| {
                        SymbolPin::new(&anchor.name, PortDirection::InOut, Some(anchor.position))
                    })
                    .collect()
            } else {
                let mut ordered = definition.pins.iter().collect::<Vec<_>>();
                ordered.sort_by_key(|pin| pin.order);
                ordered
                    .into_iter()
                    .zip(&imported.pin_anchors)
                    .map(|(pin, anchor)| {
                        let offset = match pin.side {
                            SymbolPinSide::Left | SymbolPinSide::Right => anchor.position.y,
                            SymbolPinSide::Top | SymbolPinSide::Bottom => anchor.position.x,
                        };
                        SymbolPin::new(&pin.name, pin.direction, Some(anchor.position))
                            .with_contract(pin.electrical_type, pin.side, offset)
                    })
                    .collect()
            };
        return SymbolDocument {
            pins,
            body,
            origin: Point::origin(),
            name_anchor: Point::new(-half_width, -half_height - 30),
            value_anchor: Point::new(-half_width, half_height + 30),
        };
    }
    let mut side_counts = HashMap::<SymbolPinSide, usize>::new();
    for pin in &definition.pins {
        *side_counts.entry(pin.side).or_default() += 1;
    }
    let mut side_indexes = HashMap::<SymbolPinSide, usize>::new();
    let mut ordered = definition.pins.iter().collect::<Vec<_>>();
    ordered.sort_by_key(|pin| pin.order);
    let pins = ordered
        .into_iter()
        .map(|pin| {
            let index = side_indexes.entry(pin.side).or_default();
            let count = side_counts[&pin.side];
            let centered = (*index as i32 * 2 + 1 - count as i32) * 10;
            *index += 1;
            let position = match pin.side {
                SymbolPinSide::Left => Point::new(-half_width - 20, centered),
                SymbolPinSide::Right => Point::new(half_width + 20, centered),
                SymbolPinSide::Top => Point::new(centered, -half_height - 20),
                SymbolPinSide::Bottom => Point::new(centered, half_height + 20),
            };
            SymbolPin::new(&pin.name, pin.direction, Some(position)).with_contract(
                pin.electrical_type,
                pin.side,
                centered,
            )
        })
        .collect();
    SymbolDocument {
        pins,
        body,
        origin: Point::origin(),
        name_anchor: Point::new(-half_width, -half_height - 30),
        value_anchor: Point::new(-half_width, half_height + 30),
    }
}

fn project_definition_metadata(
    definition: &ModelBoundSymbolDefinition,
    metadata: &mut HashMap<String, String>,
) -> Result<(), SymbolDefinitionError> {
    metadata.insert(
        MODEL_BOUND_SYMBOL_METADATA_KEY.to_owned(),
        serde_json::to_string(definition)
            .map_err(|error| SymbolDefinitionError::Serialization(error.to_string()))?,
    );
    metadata.insert(
        SYMBOL_PARAMETER_FORM_METADATA_KEY.to_owned(),
        serde_json::to_string(&definition.parameter_form)
            .map_err(|error| SymbolDefinitionError::Serialization(error.to_string()))?,
    );
    let mut pins = definition.pins.iter().collect::<Vec<_>>();
    pins.sort_by_key(|pin| pin.order);
    let names = pins.iter().map(|pin| pin.name.clone()).collect::<Vec<_>>();
    let encoded_ports = serde_json::to_string(&names)
        .map_err(|error| SymbolDefinitionError::Serialization(error.to_string()))?;
    metadata.insert("netlist.ports".to_owned(), encoded_ports.clone());
    metadata.insert("netlist.terminals".to_owned(), encoded_ports);
    metadata.insert(
        "netlist.template".to_owned(),
        definition.netlist.template.clone(),
    );
    metadata.insert(
        "reference.prefix".to_owned(),
        definition.netlist.device_prefix.clone(),
    );
    metadata.insert(
        "netlist.parameter_order".to_owned(),
        serde_json::to_string(&definition.netlist.parameter_order)
            .map_err(|error| SymbolDefinitionError::Serialization(error.to_string()))?,
    );
    metadata.insert(
        "model.family".to_owned(),
        definition
            .netlist
            .model
            .as_ref()
            .map(|model| model.model.clone())
            .unwrap_or_else(|| "unbound".to_owned()),
    );
    if let Some(model) = &definition.netlist.model {
        if let Some(source_path) = &model.source_path {
            metadata.insert("netlist.source_path".to_owned(), source_path.clone());
        }
        if let Some(section) = &model.section {
            metadata.insert("netlist.section".to_owned(), section.clone());
            metadata.insert("model.section".to_owned(), section.clone());
        } else {
            metadata.remove("netlist.section");
            metadata.remove("model.section");
        }
        metadata.insert(
            "netlist.implementation_view".to_owned(),
            model.implementation_view.view_name().to_owned(),
        );
        if let Some(module_name) = &model.module_name {
            metadata.insert("veriloga.module".to_owned(), module_name.clone());
        }
    } else {
        metadata.remove("netlist.source_path");
        metadata.remove("netlist.section");
        metadata.remove("model.section");
        metadata.remove("netlist.implementation_view");
        metadata.remove("veriloga.module");
    }
    #[derive(Serialize)]
    struct PlacementParameter<'a> {
        name: &'a str,
        aliases: &'a [String],
        required: bool,
        default: String,
    }
    let parameters = definition
        .parameter_form
        .fields()
        .filter(|field| field.inheritance.emitted_by_rspice())
        .map(|field| PlacementParameter {
            name: &field.key,
            aliases: &field.aliases,
            required: field.required,
            default: field.default.display_string(),
        })
        .collect::<Vec<_>>();
    metadata.insert(
        "cdf.parameter_contract".to_owned(),
        serde_json::to_string(&parameters)
            .map_err(|error| SymbolDefinitionError::Serialization(error.to_string()))?,
    );
    metadata.insert(
        "cdf.parameter_inheritance".to_owned(),
        serde_json::to_string(
            &definition
                .parameter_form
                .fields()
                .map(|field| (field.key.clone(), field.inheritance))
                .collect::<BTreeMap<_, _>>(),
        )
        .map_err(|error| SymbolDefinitionError::Serialization(error.to_string()))?,
    );
    metadata.insert(
        "netlist.cell_defaults".to_owned(),
        serde_json::to_string(
            &definition
                .parameter_form
                .fields()
                .filter(|field| field.inheritance == ParameterInheritance::CellDefault)
                .map(|field| (field.key.clone(), field.default.display_string()))
                .collect::<BTreeMap<_, _>>(),
        )
        .map_err(|error| SymbolDefinitionError::Serialization(error.to_string()))?,
    );
    if let Some(imported) = &definition.imported_graphic {
        metadata.insert(
            SYMBOL_IMPORT_SOURCE_METADATA_KEY.to_owned(),
            serde_json::to_string(imported)
                .map_err(|error| SymbolDefinitionError::Serialization(error.to_string()))?,
        );
    } else {
        metadata.remove(SYMBOL_IMPORT_SOURCE_METADATA_KEY);
    }
    Ok(())
}

fn definition_from_cell(
    cell: &Cell,
) -> Result<Option<ModelBoundSymbolDefinition>, SymbolDefinitionError> {
    let Some(encoded) = cell.metadata.get(MODEL_BOUND_SYMBOL_METADATA_KEY) else {
        return Ok(None);
    };
    ModelBoundSymbolDefinition::from_json_bytes(encoded.as_bytes(), &cell.name).map(Some)
}

pub(super) fn serialize_cell(cell: &Cell) -> Result<String, SymbolDefinitionError> {
    serde_json::to_string(cell)
        .map_err(|error| SymbolDefinitionError::Serialization(error.to_string()))
}

pub fn load_model_bound_symbol(
    view: &View,
) -> Result<Option<ModelBoundSymbolDefinition>, SymbolDefinitionError> {
    let Some(encoded) = view.metadata.get(MODEL_BOUND_SYMBOL_METADATA_KEY) else {
        return Ok(None);
    };

    // Counted below the guard on purpose: a legacy symbol carrying no typed
    // contract costs one map lookup, which is not what the counter watches.
    #[cfg(test)]
    crate::state::SYMBOL_VIEW_PARSES.with(|count| count.set(count.get() + 1));

    ModelBoundSymbolDefinition::from_json_bytes(encoded.as_bytes(), &view.name).map(Some)
}

pub fn store_model_bound_symbol(
    definition: &ModelBoundSymbolDefinition,
    view: &mut View,
) -> Result<(), SymbolDefinitionError> {
    definition.validate()?;
    let mut candidate = view.clone();
    project_definition_metadata(definition, &mut candidate.metadata)?;
    if definition.generated_views.symbol || view.view_type == ViewType::Symbol {
        materialize_symbol_document(definition)
            .store_in_view(&mut candidate)
            .map_err(SymbolDefinitionError::Serialization)?;
    }
    candidate.modified = true;
    *view = candidate;
    Ok(())
}

pub fn prepare_symbol_construction(
    definition: &ModelBoundSymbolDefinition,
    library: &Library,
) -> Result<SymbolConstructionPlan, SymbolDefinitionError> {
    definition.validate()?;

    if library.read_only {
        return Err(SymbolDefinitionError::ReadOnlyLibrary(library.name.clone()));
    }
    if library.name != definition.identity.library {
        return Err(SymbolDefinitionError::LibraryIdentityMismatch {
            expected: definition.identity.library.clone(),
            actual: library.name.clone(),
        });
    }
    let before = library.get_cell(&definition.identity.cell).cloned();
    if let SymbolSourceContract::ExistingSchematicPins { schematic_view, .. } = &definition.source {
        let view = before
            .as_ref()
            .and_then(|cell| cell.get_view(schematic_view))
            .ok_or_else(|| {
                SymbolDefinitionError::SourcePinMismatch(format!(
                    "existing schematic view `{schematic_view}` does not exist in {}/{}",
                    definition.identity.library, definition.identity.cell
                ))
            })?;
        if !matches!(view.view_type, ViewType::Schematic | ViewType::Testbench) {
            return Err(SymbolDefinitionError::SourcePinMismatch(format!(
                "existing source view `{schematic_view}` is not a schematic/testbench"
            )));
        }
    }
    if let Some(existing) = &before
        && let Some(current) = definition_from_cell(existing)?
        && current.identity.revision >= definition.identity.revision
    {
        return Err(SymbolDefinitionError::NonMonotonicRevision {
            current: current.identity.revision,
            proposed: definition.identity.revision,
        });
    }
    let mut after = before
        .clone()
        .unwrap_or_else(|| Cell::new(&definition.identity.cell));
    after.name = definition.identity.cell.clone();
    project_definition_metadata(definition, &mut after.metadata)?;

    if definition.generated_views.symbol {
        let mut view = after
            .get_view(SYMBOL_VIEW_NAME)
            .cloned()
            .unwrap_or_else(|| View::new(SYMBOL_VIEW_NAME, ViewType::Symbol));
        view.view_type = ViewType::Symbol;
        store_model_bound_symbol(definition, &mut view)?;
        after.add_view(view);
    }
    if definition.generated_views.parameter_form {
        let mut view = after
            .get_view(PARAMETER_FORM_VIEW_NAME)
            .cloned()
            .unwrap_or_else(|| View::new(PARAMETER_FORM_VIEW_NAME, ViewType::Custom));
        project_definition_metadata(definition, &mut view.metadata)?;
        view.metadata.insert(
            SYMBOL_PARAMETER_FORM_METADATA_KEY.to_owned(),
            serde_json::to_string(&definition.parameter_form)
                .map_err(|error| SymbolDefinitionError::Serialization(error.to_string()))?,
        );
        view.modified = true;
        after.add_view(view);
    }
    if definition.generated_views.simulation_test_fixture {
        let mut view = after
            .get_view(TEST_FIXTURE_VIEW_NAME)
            .cloned()
            .unwrap_or_else(|| View::new(TEST_FIXTURE_VIEW_NAME, ViewType::Testbench));
        project_definition_metadata(definition, &mut view.metadata)?;
        view.metadata.insert(
            "test_fixture.contract".to_owned(),
            "pin_access_harness.v1".to_owned(),
        );
        view.metadata.insert(
            "test_fixture.buffer".to_owned(),
            serde_json::to_string(&definition.test_fixture_contract()?)
                .map_err(|error| SymbolDefinitionError::Serialization(error.to_string()))?,
        );
        view.modified = true;
        after.add_view(view);
    }

    if let SymbolSourceContract::Model { model, .. } = &definition.source {
        let view_name = model.implementation_view.view_name();
        let mut view = after.get_view(view_name).cloned().unwrap_or_else(|| {
            View::new(
                view_name,
                symbol_implementation_view_type(model.implementation_view),
            )
        });
        view.view_type = symbol_implementation_view_type(model.implementation_view);
        project_definition_metadata(definition, &mut view.metadata)?;
        view.file_path = model.source_path.as_ref().map(std::path::PathBuf::from);
        if let Some(module_name) = &model.module_name {
            view.metadata
                .insert("veriloga.module".to_owned(), module_name.clone());
        }
        view.modified = true;
        after.add_view(view);
    }

    let expected_cell_json = before.map(|cell| serialize_cell(&cell)).transpose()?;
    Ok(SymbolConstructionPlan {
        library: library.name.clone(),
        cell: definition.identity.cell.clone(),
        expected_cell_json,
        after,
    })
}

/// Build the editable pin-access harness represented by
/// `test_fixture_contract`. It never invents a stimulus or analysis.
pub fn build_symbol_test_fixture(
    definition: &ModelBoundSymbolDefinition,
) -> Result<SchematicState, SymbolDefinitionError> {
    let contract = definition.test_fixture_contract()?;
    let model = definition.netlist.model.as_ref();
    let ports = contract
        .accesses
        .iter()
        .map(|access| PortSpec {
            name: access.port_name.clone(),
            direction: access.direction,
        })
        .collect::<Vec<_>>();
    let mut binding = LibraryCellInstance::new(
        &contract.library,
        &contract.cell,
        &contract.implementation_view,
    );
    binding.bind_interface(&ports);
    binding.source_path = model
        .and_then(|model| model.source_path.as_ref())
        .map(std::path::PathBuf::from);
    binding.module_name = model
        .and_then(|model| model.module_name.clone())
        .or_else(|| model.map(|model| model.model.clone()));
    binding.netlist_template = Some(definition.netlist.template.clone());
    binding.model_section = model.and_then(|model| model.section.clone());
    binding.reference_prefix = Some(definition.netlist.device_prefix.clone());
    binding.parameter_order = definition.netlist.parameter_order.clone();

    let mut schematic = SchematicState::default();
    let dut = Component::new(1, ComponentType::CellInstance, Point::new(200, 200))
        .with_library_cell(binding)
        .with_name_value(&contract.dut_instance_name, &contract.cell);
    schematic.document.components.push(dut);

    let document = materialize_symbol_document(definition);
    let mut ordered_pins = definition.pins.iter().collect::<Vec<_>>();
    ordered_pins.sort_by_key(|pin| pin.order);

    let mut component_id = 2u64;
    let mut wire_id = 1u64;
    for (access, definition_pin) in contract.accesses.iter().zip(ordered_pins) {
        let offset = document
            .pin(&access.port_name)
            .and_then(|pin| pin.position)
            .ok_or_else(|| {
                SymbolDefinitionError::InvalidNetlist(
                    "authored symbol has an unplaced test-fixture terminal".to_owned(),
                )
            })?;
        let dut_terminal = Point::new(200 + offset.x, 200 + offset.y);
        let (port_position, rotation) = match definition_pin.side {
            SymbolPinSide::Left => (Point::new(60, dut_terminal.y), Rotation::R180),
            SymbolPinSide::Right => (Point::new(340, dut_terminal.y), Rotation::R0),
            SymbolPinSide::Top => (Point::new(dut_terminal.x, 60), Rotation::R270),
            SymbolPinSide::Bottom => (Point::new(dut_terminal.x, 340), Rotation::R90),
        };
        let mut port = Component::new(component_id, ComponentType::Port, port_position)
            .with_rotation(rotation)
            .with_name_value("", &access.port_name);
        port.params = format!(
            "dir={} signal_type={} discipline={} interface_order={} documentation={}_pin_access",
            access.direction.keyword(),
            match access.electrical_type {
                SymbolElectricalType::Logic => "logic",
                SymbolElectricalType::Power | SymbolElectricalType::Ground => "power",
                _ => "analog",
            },
            if access.electrical_type == SymbolElectricalType::Logic {
                "logic"
            } else {
                "electrical"
            },
            access.order,
            access.port_name
        );
        let port_id = port.id;
        let (_, port_terminal) = port.terminal_positions()[0];
        schematic.document.components.push(port);
        let route = if dut_terminal.x == port_terminal.x || dut_terminal.y == port_terminal.y {
            vec![dut_terminal, port_terminal]
        } else {
            vec![
                dut_terminal,
                Point::new(port_terminal.x, dut_terminal.y),
                port_terminal,
            ]
        };
        schematic.document.wires.push(Wire::new(wire_id, route));
        schematic
            .document
            .connections
            .push(WireConnection::new(wire_id, 0, 1, &access.port_name));
        schematic
            .document
            .connections
            .push(WireConnection::new(wire_id, 1, port_id, "P"));
        component_id += 1;
        wire_id += 1;

        if access.ground {
            let ground = Component::new(
                component_id,
                ComponentType::Ground,
                Point::new(dut_terminal.x, dut_terminal.y.saturating_add(60)),
            );
            let ground_id = ground.id;
            let (_, ground_terminal) = ground.terminal_positions()[0];
            schematic.document.components.push(ground);
            schematic
                .document
                .wires
                .push(Wire::new(wire_id, vec![dut_terminal, ground_terminal]));
            schematic.document.connections.push(WireConnection::new(
                wire_id,
                0,
                1,
                &access.port_name,
            ));
            schematic
                .document
                .connections
                .push(WireConnection::new(wire_id, 1, ground_id, "GND"));
            component_id += 1;
            wire_id += 1;
        }
    }
    schematic.is_dirty = true;
    schematic.needs_fit = true;
    Ok(schematic)
}

const fn symbol_implementation_view_type(view: SymbolImplementationView) -> ViewType {
    match view {
        SymbolImplementationView::Spice => ViewType::Spice,
        SymbolImplementationView::VerilogA => ViewType::VerilogA,
    }
}
