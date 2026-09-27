//! Headless pin-access fixture documents for model-bound symbols.

use super::*;

/// Build the editable pin-access harness represented by
/// `test_fixture_contract`. It never invents a stimulus or analysis.
pub fn build_symbol_test_fixture_document(
    definition: &ModelBoundSymbolDefinition,
) -> Result<SchematicDocument, SymbolDefinitionError> {
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

    let mut schematic = SchematicDocument::default();
    let dut = Component::new(1, ComponentType::CellInstance, Point::new(200, 200))
        .with_library_cell(binding)
        .with_name_value(&contract.dut_instance_name, &contract.cell);
    schematic.components.push(dut);

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
        schematic.components.push(port);
        let route = if dut_terminal.x == port_terminal.x || dut_terminal.y == port_terminal.y {
            vec![dut_terminal, port_terminal]
        } else {
            vec![
                dut_terminal,
                Point::new(port_terminal.x, dut_terminal.y),
                port_terminal,
            ]
        };
        schematic.wires.push(Wire::new(wire_id, route));
        schematic
            .connections
            .push(WireConnection::new(wire_id, 0, 1, &access.port_name));
        schematic
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
            schematic.components.push(ground);
            schematic
                .wires
                .push(Wire::new(wire_id, vec![dut_terminal, ground_terminal]));
            schematic
                .connections
                .push(WireConnection::new(wire_id, 0, 1, &access.port_name));
            schematic
                .connections
                .push(WireConnection::new(wire_id, 1, ground_id, "GND"));
            component_id += 1;
            wire_id += 1;
        }
    }
    Ok(schematic)
}
