//! Extracting checkable data from a schematic.
//!
//! Flattens the design into the nets, terminals, and parameters the rules
//! examine, resolving junctions so two wires that meet at a dot are one net.

use super::checker::{DrcChecker, DrcConfig};
use super::input::{
    ComponentInfo, JunctionInfo, NetLabelInfo, ParameterRangeIssue, PinInfo, WireInfo,
};
use super::types::{DrcLocation, DrcResult, DrcSeverity, DrcViolation, DrcViolationType};
use crate::simulation::netlist_gen::HierarchySource;
use crate::state::{
    Component, ComponentType, Point, PropertyDefinition, PropertyRegistry, PropertyValue,
};

/// Extract hierarchy-resolved DRC data including explicit junctions.
pub fn extract_drc_data_with_hierarchy_and_junctions(
    schematic: &crate::state::SchematicState,
    hierarchy: &HierarchySource<'_>,
) -> (
    Vec<ComponentInfo>,
    Vec<WireInfo>,
    Vec<NetLabelInfo>,
    Vec<JunctionInfo>,
) {
    extract_drc_data_with_terminals_and_junctions(
        schematic,
        |comp| {
            let resolved_symbol = comp
                .library_cell
                .as_ref()
                .and_then(|binding| hierarchy.resolved_symbol_for(binding));
            comp.terminal_positions_resolved(resolved_symbol.as_ref())
        },
        |comp| {
            if comp.kind != ComponentType::CellInstance {
                return Some(true);
            }
            let Some(binding) = comp.library_cell.as_ref() else {
                return Some(false);
            };
            if binding.source_path.is_some()
                || binding.netlist_template.is_some()
                || binding.is_executable_builtin()
            {
                return Some(true);
            }
            if hierarchy.has_execution_plan() {
                // Top-level DRC extraction has no instance path with which to
                // query an execution-plan rebind. Do not judge the placed
                // binding when the executable authority may select another
                // master.
                return None;
            }
            Some(hierarchy.schematic_master_for_binding(binding).is_some())
        },
    )
}

fn extract_drc_data_with_terminals_and_junctions(
    schematic: &crate::state::SchematicState,
    mut terminal_positions_for: impl FnMut(&Component) -> Vec<(String, Point)>,
    mut component_known_for: impl FnMut(&Component) -> Option<bool>,
) -> (
    Vec<ComponentInfo>,
    Vec<WireInfo>,
    Vec<NetLabelInfo>,
    Vec<JunctionInfo>,
) {
    let mut components = Vec::with_capacity(schematic.components.len());
    let mut wires = Vec::with_capacity(schematic.wires.len());
    let mut net_labels = Vec::with_capacity(schematic.net_labels.len());
    let mut junctions = Vec::with_capacity(schematic.junctions.len());

    // Build point-to-net mapping from existing net_mapping or create from connectivity
    let net_mapping = &schematic.net_mapping;
    let property_registry = PropertyRegistry::new();

    // Extract components
    for comp in &schematic.components {
        let terminal_positions = terminal_positions_for(comp);
        let mut pins = Vec::with_capacity(terminal_positions.len());
        let declared_output_pins: std::collections::HashSet<String> = comp
            .library_cell
            .as_ref()
            .and_then(|binding| binding.interface())
            .into_iter()
            .flatten()
            .filter(|port| port.direction == crate::state::PortDirection::Out)
            .map(|port| port.name)
            .collect();

        for (pin_name, pin_pos) in terminal_positions {
            // Look up net name from the cached mapping, or create a positional name
            let net_name = net_mapping
                .get(&pin_pos)
                .cloned()
                .unwrap_or_else(|| format!("net_{}_{}", pin_pos.x, pin_pos.y));

            let is_output = matches!(
                comp.kind,
                ComponentType::VoltageSource
                    | ComponentType::VoltageSourceAc
                    | ComponentType::VoltageSourcePulse
                    | ComponentType::VoltageSourceSin
                    | ComponentType::VoltageSourceExp
                    | ComponentType::VoltageSourceSffm
                    | ComponentType::VoltageSourceAm
                    | ComponentType::VoltageSourcePat
                    | ComponentType::VoltageSourceNoise
                    | ComponentType::VoltageSourcePwl
                    | ComponentType::VoltageSourcePwlFile
            ) && pin_name == "+"
                || declared_output_pins.contains(&pin_name);

            pins.push(PinInfo {
                name: pin_name,
                net_name,
                is_output,
                x: Some(pin_pos.x as f64),
                y: Some(pin_pos.y as f64),
            });
        }

        let is_voltage_source = matches!(
            comp.kind,
            ComponentType::VoltageSource
                | ComponentType::VoltageSourceAc
                | ComponentType::VoltageSourcePulse
                | ComponentType::VoltageSourceSin
                | ComponentType::VoltageSourceExp
                | ComponentType::VoltageSourceSffm
                | ComponentType::VoltageSourceAm
                | ComponentType::VoltageSourcePat
                | ComponentType::VoltageSourceNoise
                | ComponentType::VoltageSourcePwl
        );

        let is_current_source = matches!(
            comp.kind,
            ComponentType::CurrentSource
                | ComponentType::CurrentSourceAc
                | ComponentType::CurrentSourcePulse
                | ComponentType::CurrentSourceSin
                | ComponentType::CurrentSourceExp
                | ComponentType::CurrentSourceSffm
                | ComponentType::CurrentSourceAm
                | ComponentType::CurrentSourcePat
                | ComponentType::CurrentSourceNoise
                | ComponentType::CurrentSourcePwl
                | ComponentType::CurrentSourcePwlFile
        );
        let reference_required = !comp.kind.spice_prefix().is_empty();
        let reference_error = (reference_required && !comp.name.trim().is_empty())
            .then(|| comp.validate_reference_designator(comp.name.trim()).err())
            .flatten();

        components.push(ComponentInfo {
            id: comp.id,
            name: comp.name.clone(),
            component_type: comp.kind.spice_prefix().to_string(),
            pins,
            is_voltage_source,
            is_current_source,
            reference_required,
            reference_error,
            component_known: component_known_for(comp),
            missing_parameters: missing_parameters(comp, &property_registry),
            out_of_range_parameters: out_of_range_parameters(comp, &property_registry),
        });
    }

    // Extract wires
    for wire in &schematic.wires {
        if wire.points.len() >= 2 {
            // Create WireInfo for each segment
            for i in 0..wire.points.len() - 1 {
                let start = &wire.points[i];
                let end = &wire.points[i + 1];
                wires.push(WireInfo {
                    id: wire.id,
                    start_x: start.x as f64,
                    start_y: start.y as f64,
                    end_x: end.x as f64,
                    end_y: end.y as f64,
                });
            }
        }
    }

    // Extract net labels (including ground symbols)
    for label in &schematic.net_labels {
        net_labels.push(NetLabelInfo {
            name: label.name.clone(),
            x: label.pos.x as f64,
            y: label.pos.y as f64,
            synthetic: false,
            electrical_anchor: false,
        });
    }
    for binding in
        crate::schematic::bus_connectivity::analyze_bus_connectivity(schematic).scalar_taps
    {
        net_labels.push(NetLabelInfo {
            name: binding.member_name,
            x: binding.point.x as f64,
            y: binding.point.y as f64,
            synthetic: true,
            electrical_anchor: true,
        });
    }

    // Check for ground components (GND symbol)
    for comp in &schematic.components {
        if matches!(comp.kind, ComponentType::Ground) {
            // Ground component acts as a net label for "0"
            net_labels.push(NetLabelInfo {
                name: "0".to_string(),
                x: comp.pos.x as f64,
                y: comp.pos.y as f64,
                synthetic: true,
                electrical_anchor: true,
            });
        }
    }

    junctions.extend(
        schematic
            .junctions
            .iter()
            .map(|junction| JunctionInfo::new(junction.pos.x as f64, junction.pos.y as f64)),
    );

    (components, wires, net_labels, junctions)
}

fn effective_component_properties<'a>(
    component: &Component,
    registry: &'a PropertyRegistry,
) -> Option<(
    &'a crate::state::PropertySheet,
    std::collections::HashMap<String, PropertyValue>,
)> {
    if component.kind == ComponentType::CellInstance {
        // Cell-instance CDF is transaction-scoped and cannot be reconstructed
        // from the built-in registry. Reporting against a previous or generic
        // master's schema would be a false positive.
        return None;
    }
    let sheet = registry.get(component.kind)?;
    Some((
        sheet,
        crate::properties::property_bridge::collect_properties_from_component(component, registry),
    ))
}

fn missing_parameters(component: &Component, registry: &PropertyRegistry) -> Vec<String> {
    let Some((sheet, values)) = effective_component_properties(component, registry) else {
        return Vec::new();
    };
    let mut missing = sheet
        .iter()
        .filter(|definition| definition.name != "name" && definition.required)
        .filter_map(|definition| {
            let value = values
                .get(&definition.name)
                .unwrap_or(&definition.default_value);
            property_value_is_missing(definition, value).then(|| definition.display_name.clone())
        })
        .collect::<Vec<_>>();
    missing.sort();
    missing.dedup();
    missing
}

fn property_value_is_missing(definition: &PropertyDefinition, value: &PropertyValue) -> bool {
    match value {
        PropertyValue::String(value) => value.trim().is_empty(),
        PropertyValue::Expression(value) => {
            definition.required
                && matches!(
                    definition.prop_type,
                    crate::state::PropertyType::Number | crate::state::PropertyType::Expression
                )
                && value.trim().is_empty()
        }
        PropertyValue::Number { value, .. } => value.is_nan(),
        PropertyValue::Enum { .. } | PropertyValue::Boolean(_) => false,
    }
}

fn out_of_range_parameters(
    component: &Component,
    registry: &PropertyRegistry,
) -> Vec<ParameterRangeIssue> {
    let Some((sheet, values)) = effective_component_properties(component, registry) else {
        return Vec::new();
    };
    let mut issues = sheet
        .iter()
        .filter(|definition| definition.min_value.is_some() || definition.max_value.is_some())
        .filter_map(|definition| {
            let value = values
                .get(&definition.name)
                .unwrap_or(&definition.default_value);
            let numeric = exact_numeric_constant(definition, value)?;
            let below = definition
                .min_value
                .is_some_and(|minimum| numeric < minimum);
            let above = definition
                .max_value
                .is_some_and(|maximum| numeric > maximum);
            (below || above).then(|| ParameterRangeIssue {
                name: definition.name.clone(),
                display_name: definition.display_name.clone(),
                value: numeric,
                min: definition.min_value,
                max: definition.max_value,
            })
        })
        .collect::<Vec<_>>();
    issues.sort_by(|left, right| left.name.cmp(&right.name));
    issues
}

fn exact_numeric_constant(definition: &PropertyDefinition, value: &PropertyValue) -> Option<f64> {
    if let PropertyValue::Number { value, .. } = value {
        return value.is_finite().then_some(*value);
    }
    let PropertyValue::Expression(source) = value else {
        return None;
    };

    // Reuse the production property editor parser (quantity policy,
    // expression grammar, and unit normalization) while temporarily removing
    // the range itself. This reveals a definite numeric constant without
    // relying on validator error strings; symbolic expressions remain unknown.
    let mut unconstrained = definition.clone();
    unconstrained.min_value = None;
    unconstrained.max_value = None;
    crate::properties::tabbed_dialog::parse_expression_source(
        &unconstrained,
        source,
        crate::quantity::QuantityPresentationPolicy::default(),
        crate::quantity::UiNumberLocale::default(),
    )
    .ok()
    .and_then(|value| value.as_number())
    .filter(|value| value.is_finite())
}

/// Run a configured DRC check with project-cell symbol resolution enabled.
pub fn run_drc_check_with_hierarchy_and_config(
    schematic: &crate::state::SchematicState,
    hierarchy: &HierarchySource<'_>,
    config: DrcConfig,
) -> DrcResult {
    let start = crate::time_compat::Instant::now();
    let (components, wires, net_labels, junctions) =
        extract_drc_data_with_hierarchy_and_junctions(schematic, hierarchy);
    let severity_overrides = config.severity_overrides.clone();
    let mut checker = DrcChecker::with_config(config);
    checker.set_net_naming_policy(schematic.document_policy.net_naming);
    let mut result =
        checker.check_connectivity_with_junctions(&components, &wires, &net_labels, &junctions);
    append_bus_violations(schematic, &mut result, &severity_overrides);
    super::checker::append_off_sheet_connector_violations(
        schematic,
        &mut result,
        &severity_overrides,
    );
    result.completed = true;
    result.duration_ms = start.elapsed().as_millis() as u64;
    result
}

fn append_bus_violations(
    schematic: &crate::state::SchematicState,
    result: &mut DrcResult,
    severity_overrides: &std::collections::HashMap<DrcViolationType, DrcSeverity>,
) {
    use crate::schematic::bus_connectivity::{BusDiagnosticKind, analyze_bus_connectivity};

    let mut next_id = result.total_count();
    for diagnostic in analyze_bus_connectivity(schematic).diagnostics {
        let violation_type = match diagnostic.kind {
            BusDiagnosticKind::MalformedBus => DrcViolationType::MalformedBus,
            BusDiagnosticKind::UnnamedBus => DrcViolationType::UnnamedBus,
            BusDiagnosticKind::RangeConflict => DrcViolationType::BusRangeConflict,
            BusDiagnosticKind::DanglingTap => DrcViolationType::DanglingBusTap,
            BusDiagnosticKind::MixedTap => DrcViolationType::MixedBusTap,
        };
        let location = if let Some(tap_id) = diagnostic.tap_id {
            DrcLocation::BusTap { id: tap_id }
        } else if let Some(bus_id) = diagnostic.bus_id {
            DrcLocation::Bus { id: bus_id }
        } else {
            DrcLocation::Point {
                x: f64::from(diagnostic.point.x),
                y: f64::from(diagnostic.point.y),
            }
        };
        let mut violation =
            DrcViolation::new(next_id, violation_type, diagnostic.message, location);
        if let Some(severity) = severity_overrides.get(&violation_type) {
            violation.severity = *severity;
        }
        result.add_violation(violation);
        next_id += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        Bus, BusDeclaration, BusSlice, BusTapOrientation, Cell, CellViewRef, Component,
        ComponentType, Library, LibraryCellInstance, LibraryManager, NetLabel, Point,
        PortDirection, PortSpec, SchematicState, SymbolDocument, SymbolPin, View, ViewType, Wire,
    };
    use std::collections::HashMap;

    fn port(name: &str, direction: PortDirection) -> PortSpec {
        PortSpec {
            name: name.to_owned(),
            direction,
        }
    }

    fn library_with_authored_amp_symbol() -> (LibraryManager, HashMap<String, SchematicState>) {
        let mut libraries = LibraryManager::new();
        let mut library = Library::new("work");
        let mut cell = Cell::new("amp");
        cell.add_view(View::new("schematic", ViewType::Schematic));

        let document = SymbolDocument {
            pins: vec![
                SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(70, 20))),
                SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, -10))),
            ],
            ..SymbolDocument::default()
        };
        let mut symbol_view = View::new("symbol", ViewType::Symbol);
        document
            .store_in_view(&mut symbol_view)
            .expect("symbol stores");
        cell.add_view(symbol_view);
        library.add_cell(cell);
        libraries.add_library(library);

        let mut master = SchematicState::default();
        for (idx, name) in ["IN", "OUT"].iter().enumerate() {
            let id = master.add_component(ComponentType::Port, Point::new(idx as i32 * 40, 0));
            master
                .components
                .iter_mut()
                .find(|component| component.id == id)
                .expect("port component")
                .value = (*name).to_owned();
        }

        let mut buffers = HashMap::new();
        buffers.insert(CellViewRef::new("work", "amp", "schematic").key(), master);
        (libraries, buffers)
    }

    fn authored_amp_instance() -> Component {
        let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&[
            port("IN", PortDirection::In),
            port("OUT", PortDirection::Out),
        ]);
        Component::new(1, ComponentType::CellInstance, Point::new(100, 50))
            .with_library_cell(binding)
    }

    #[test]
    fn hierarchy_extraction_uses_authored_symbol_pin_coordinates() {
        let (libraries, buffers) = library_with_authored_amp_symbol();
        let hierarchy = HierarchySource::from_workspace(&libraries, &buffers);
        let mut schematic = SchematicState::default();
        schematic.components.push(authored_amp_instance());

        let (components, _, _, _) =
            extract_drc_data_with_hierarchy_and_junctions(&schematic, &hierarchy);
        let pins: HashMap<_, _> = components[0]
            .pins
            .iter()
            .map(|pin| (pin.name.as_str(), (pin.x, pin.y)))
            .collect();

        assert_eq!(pins.get("IN"), Some(&(Some(60.0), Some(40.0))));
        assert_eq!(pins.get("OUT"), Some(&(Some(170.0), Some(70.0))));
    }

    #[test]
    fn hierarchy_resolved_unconnected_pin_check_uses_authored_terminal_geometry() {
        let (libraries, buffers) = library_with_authored_amp_symbol();
        let hierarchy = HierarchySource::from_workspace(&libraries, &buffers);
        let mut instance = authored_amp_instance();
        instance.name = "X1".to_owned();
        let mut schematic = SchematicState::default();
        schematic.components.push(instance);
        schematic
            .wires
            .push(Wire::segment(90, Point::new(40, 40), Point::new(60, 40)));

        let result = run_drc_check_with_hierarchy_and_config(
            &schematic,
            &hierarchy,
            DrcConfig {
                check_missing_ground: false,
                check_floating_nodes: false,
                ..DrcConfig::default()
            },
        );
        let unconnected = result
            .violations()
            .iter()
            .filter(|finding| finding.violation_type == DrcViolationType::UnconnectedPin)
            .collect::<Vec<_>>();

        assert_eq!(unconnected.len(), 1);
        assert!(unconnected[0].message.contains("X1.OUT"));
        assert_eq!(
            unconnected[0].location,
            DrcLocation::Component {
                id: 1,
                name: "X1".to_owned()
            }
        );
    }

    #[test]
    fn canonical_property_schema_extracts_only_definite_parameter_failures() {
        let mut schematic = SchematicState::default();
        let mut resistor = Component::new(20, ComponentType::Resistor, Point::origin());
        resistor.name = "R20".to_owned();
        resistor.value.clear(); // The canonical 1k default is an effective value.
        resistor.params = "m=0".to_owned();
        schematic.components.push(resistor);

        let (components, _, _, _) =
            extract_drc_data_with_hierarchy_and_junctions(&schematic, &HierarchySource::empty());
        assert!(components[0].missing_parameters.is_empty());
        assert_eq!(
            components[0].out_of_range_parameters,
            vec![ParameterRangeIssue {
                name: "m".to_owned(),
                display_name: "Multiplier".to_owned(),
                value: 0.0,
                min: Some(1.0),
                max: Some(10_000.0),
            }]
        );
    }

    #[test]
    /// A cell the hierarchy cannot resolve is reported as definitively
    /// unknown rather than left undetermined, so the rule that follows can
    /// raise it. `component_known` is only ever `Some` because every caller
    /// reaches extraction through a hierarchy-resolved entry point.
    fn unresolvable_project_cell_is_reported_as_definitively_unknown() {
        let binding = LibraryCellInstance::new("work", "missing", "schematic");
        let mut instance = Component::new(30, ComponentType::CellInstance, Point::origin())
            .with_library_cell(binding);
        instance.name = "X30".to_owned();
        let mut schematic = SchematicState::default();
        schematic.components.push(instance);

        let hierarchy = HierarchySource::empty();
        let (resolved, _, _, _) =
            extract_drc_data_with_hierarchy_and_junctions(&schematic, &hierarchy);
        assert_eq!(resolved[0].component_known, Some(false));
    }

    #[test]
    fn junction_aware_extraction_passes_persisted_junction_positions() {
        let mut schematic = SchematicState::default();
        schematic.add_junction(Point::new(40, -20));
        schematic.add_junction(Point::new(0, 0));

        let (_, _, _, junctions) =
            extract_drc_data_with_hierarchy_and_junctions(&schematic, &HierarchySource::empty());

        assert_eq!(
            junctions,
            vec![JunctionInfo::new(40.0, -20.0), JunctionInfo::new(0.0, 0.0)]
        );
    }

    #[test]
    fn typed_bus_member_conflict_is_reported_by_drc_and_honors_severity_policy() {
        let mut schematic = SchematicState::default();
        let bus_id = schematic
            .add_bus(
                vec![Point::new(0, 0), Point::new(20, 0)],
                Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
            )
            .unwrap();
        schematic
            .wires
            .push(Wire::segment(100, Point::new(5, 10), Point::new(20, 10)));
        schematic
            .place_bus_tap(
                bus_id,
                Point::new(5, 0),
                Point::new(5, 10),
                BusSlice::parse("DATA[3]").unwrap(),
                BusTapOrientation::Down,
            )
            .unwrap();
        schematic
            .net_labels
            .push(NetLabel::new(101, Point::new(15, 10), "FOO"));

        let mut config = DrcConfig {
            check_missing_ground: false,
            check_floating_nodes: false,
            ..DrcConfig::default()
        };
        config
            .severity_overrides
            .insert(DrcViolationType::BusRangeConflict, DrcSeverity::Critical);

        let result =
            run_drc_check_with_hierarchy_and_config(&schematic, &HierarchySource::empty(), config);
        let conflict = result
            .violations()
            .iter()
            .find(|violation| violation.violation_type == DrcViolationType::BusRangeConflict)
            .expect("typed member conflict");
        assert_eq!(conflict.severity, DrcSeverity::Critical);
        assert!(conflict.message.contains("DATA[3]"));
        assert!(conflict.message.contains("FOO"));
        assert_eq!(
            conflict.location,
            DrcLocation::Node {
                net_name: "DATA[3]".to_owned()
            }
        );
    }

    #[test]
    fn duplicate_authored_cell_outputs_on_bus_member_are_reported_with_policy_severity() {
        let (libraries, buffers) = library_with_authored_amp_symbol();
        let hierarchy = HierarchySource::from_workspace(&libraries, &buffers);
        let mut first = authored_amp_instance();
        first.id = 10;
        first.name = "X1".to_owned();
        let mut second = authored_amp_instance();
        second.id = 11;
        second.name = "X2".to_owned();
        second.pos = Point::new(300, 50);

        let mut schematic = SchematicState::default();
        schematic.components.extend([first, second]);
        schematic
            .wires
            .push(Wire::segment(20, Point::new(170, 70), Point::new(370, 70)));
        let bus_id = schematic
            .add_bus(
                vec![Point::new(270, 0), Point::new(370, 0)],
                Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
            )
            .unwrap();
        schematic
            .place_bus_tap(
                bus_id,
                Point::new(270, 0),
                Point::new(270, 70),
                BusSlice::parse("DATA[3]").unwrap(),
                BusTapOrientation::Down,
            )
            .unwrap();

        let mut config = DrcConfig {
            check_missing_ground: false,
            check_floating_nodes: false,
            ..DrcConfig::default()
        };
        config.severity_overrides.insert(
            DrcViolationType::DuplicateBusMemberDriver,
            DrcSeverity::Critical,
        );
        let result = run_drc_check_with_hierarchy_and_config(&schematic, &hierarchy, config);
        let violation = result
            .violations()
            .iter()
            .find(|violation| {
                violation.violation_type == DrcViolationType::DuplicateBusMemberDriver
            })
            .expect("duplicate typed output driver");

        assert_eq!(violation.severity, DrcSeverity::Critical);
        assert!(violation.message.contains("DATA[3]"));
        assert!(violation.message.contains("X1"));
        assert!(violation.message.contains("X2"));
    }

    #[test]
    fn bus_drc_locations_preserve_full_u64_identity() {
        let mut schematic = SchematicState::default();
        schematic
            .buses
            .push(Bus::segment(u64::MAX, Point::new(0, 0), Point::new(20, 0), None).unwrap());
        let result = run_drc_check_with_hierarchy_and_config(
            &schematic,
            &HierarchySource::empty(),
            DrcConfig {
                check_missing_ground: false,
                check_floating_nodes: false,
                ..DrcConfig::default()
            },
        );
        assert!(result.completed);
        assert!(result.violations().iter().any(|violation| {
            violation.violation_type == DrcViolationType::UnnamedBus
                && violation.location == DrcLocation::Bus { id: u64::MAX }
        }));
    }
}
