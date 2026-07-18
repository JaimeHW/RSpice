use super::checker::{DrcChecker, DrcConfig};
use super::input::{ComponentInfo, JunctionInfo, NetLabelInfo, PinInfo, WireInfo};
use super::types::{DrcLocation, DrcResult, DrcSeverity, DrcViolation, DrcViolationType};
use crate::simulation::netlist_gen::HierarchySource;
use crate::state::{Component, Point};

/// Extract legacy geometry-only DRC data from a `SchematicState`.
///
/// This compatibility projection intentionally omits explicit junctions. New
/// schematic validation code must use [`extract_drc_data_with_junctions`] with
/// [`DrcChecker::check_connectivity_with_junctions`]; otherwise marked
/// interior/interior crossings are evaluated as disconnected.
///
/// # Performance
/// - O(n) iteration over components
/// - O(n) iteration over wires  
/// - O(n) iteration over net labels
///
/// # Example
/// ```ignore
/// use rspice_ui::services::drc::{DrcChecker, extract_drc_data_with_junctions};
///
/// let (components, wires, net_labels, junctions) =
///     extract_drc_data_with_junctions(&schematic);
/// let result = DrcChecker::new().check_connectivity_with_junctions(
///     &components, &wires, &net_labels, &junctions,
/// );
/// ```
pub fn extract_drc_data(
    schematic: &crate::state::SchematicState,
) -> (Vec<ComponentInfo>, Vec<WireInfo>, Vec<NetLabelInfo>) {
    let (components, wires, net_labels, _) = extract_drc_data_with_junctions(schematic);
    (components, wires, net_labels)
}

/// Extract DRC data including persisted explicit-junction positions.
pub fn extract_drc_data_with_junctions(
    schematic: &crate::state::SchematicState,
) -> (
    Vec<ComponentInfo>,
    Vec<WireInfo>,
    Vec<NetLabelInfo>,
    Vec<JunctionInfo>,
) {
    extract_drc_data_with_terminals_and_junctions(schematic, |comp| {
        comp.terminal_positions()
            .into_iter()
            .map(|(name, pos)| (name.to_owned(), pos))
            .collect()
    })
}

/// Extract legacy junction-blind DRC data with project-cell symbol resolution.
///
/// Prefer [`extract_drc_data_with_hierarchy_and_junctions`] for schematic
/// validation so persisted explicit crossings are retained.
pub fn extract_drc_data_with_hierarchy(
    schematic: &crate::state::SchematicState,
    hierarchy: &HierarchySource<'_>,
) -> (Vec<ComponentInfo>, Vec<WireInfo>, Vec<NetLabelInfo>) {
    let (components, wires, net_labels, _) =
        extract_drc_data_with_hierarchy_and_junctions(schematic, hierarchy);
    (components, wires, net_labels)
}

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
    extract_drc_data_with_terminals_and_junctions(schematic, |comp| {
        let resolved_symbol = comp
            .library_cell
            .as_ref()
            .and_then(|binding| hierarchy.resolved_symbol_for(binding));
        comp.terminal_positions_resolved(resolved_symbol.as_ref())
    })
}

fn extract_drc_data_with_terminals_and_junctions(
    schematic: &crate::state::SchematicState,
    mut terminal_positions_for: impl FnMut(&Component) -> Vec<(String, Point)>,
) -> (
    Vec<ComponentInfo>,
    Vec<WireInfo>,
    Vec<NetLabelInfo>,
    Vec<JunctionInfo>,
) {
    use crate::state::ComponentType;

    let mut components = Vec::with_capacity(schematic.components.len());
    let mut wires = Vec::with_capacity(schematic.wires.len());
    let mut net_labels = Vec::with_capacity(schematic.net_labels.len());
    let mut junctions = Vec::with_capacity(schematic.junctions.len());

    // Build point-to-net mapping from existing net_mapping or create from connectivity
    let net_mapping = &schematic.net_mapping;

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
                    | ComponentType::VoltageSourcePwl
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
                | ComponentType::VoltageSourcePwl
        );

        let is_current_source = matches!(
            comp.kind,
            ComponentType::CurrentSource
                | ComponentType::CurrentSourceAc
                | ComponentType::CurrentSourcePulse
                | ComponentType::CurrentSourceSin
                | ComponentType::CurrentSourcePwl
        );

        components.push(ComponentInfo {
            id: comp.id,
            name: if comp.name.is_empty() {
                comp.spice_instance_name()
            } else {
                comp.name.clone()
            },
            component_type: comp.kind.spice_prefix().to_string(),
            pins,
            is_voltage_source,
            is_current_source,
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
        });
    }
    for binding in
        crate::schematic::bus_connectivity::analyze_bus_connectivity(schematic).scalar_taps
    {
        net_labels.push(NetLabelInfo {
            name: binding.member_name,
            x: binding.point.x as f64,
            y: binding.point.y as f64,
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

/// Run a complete DRC check on a schematic.
///
/// This is a convenience function that extracts data and runs the check
/// in a single call.
///
/// # Example
/// ```ignore
/// use rspice_ui::services::drc::run_drc_check;
///
/// let result = run_drc_check(&schematic);
/// if result.passed() {
///     println!("DRC passed!");
/// }
/// ```
pub fn run_drc_check(schematic: &crate::state::SchematicState) -> DrcResult {
    let start = crate::common::time_compat::Instant::now();
    let (components, wires, net_labels, junctions) = extract_drc_data_with_junctions(schematic);
    let mut checker = DrcChecker::new();
    checker.set_net_naming_policy(schematic.document_policy.net_naming);
    let mut result =
        checker.check_connectivity_with_junctions(&components, &wires, &net_labels, &junctions);
    append_bus_violations(schematic, &mut result, &Default::default());
    result.completed = true;
    result.duration_ms = start.elapsed().as_millis() as u64;
    result
}

/// Run a complete DRC check with project-cell symbol resolution enabled.
pub fn run_drc_check_with_hierarchy(
    schematic: &crate::state::SchematicState,
    hierarchy: &HierarchySource<'_>,
) -> DrcResult {
    let start = crate::common::time_compat::Instant::now();
    let (components, wires, net_labels, junctions) =
        extract_drc_data_with_hierarchy_and_junctions(schematic, hierarchy);
    let mut checker = DrcChecker::new();
    checker.set_net_naming_policy(schematic.document_policy.net_naming);
    let mut result =
        checker.check_connectivity_with_junctions(&components, &wires, &net_labels, &junctions);
    append_bus_violations(schematic, &mut result, &Default::default());
    result.completed = true;
    result.duration_ms = start.elapsed().as_millis() as u64;
    result
}

/// Run a complete DRC check with custom configuration.
pub fn run_drc_check_with_config(
    schematic: &crate::state::SchematicState,
    config: DrcConfig,
) -> DrcResult {
    let start = crate::common::time_compat::Instant::now();
    let (components, wires, net_labels, junctions) = extract_drc_data_with_junctions(schematic);
    let severity_overrides = config.severity_overrides.clone();
    let mut checker = DrcChecker::with_config(config);
    checker.set_net_naming_policy(schematic.document_policy.net_naming);
    let mut result =
        checker.check_connectivity_with_junctions(&components, &wires, &net_labels, &junctions);
    append_bus_violations(schematic, &mut result, &severity_overrides);
    result.completed = true;
    result.duration_ms = start.elapsed().as_millis() as u64;
    result
}

/// Run a configured DRC check with project-cell symbol resolution enabled.
pub fn run_drc_check_with_hierarchy_and_config(
    schematic: &crate::state::SchematicState,
    hierarchy: &HierarchySource<'_>,
    config: DrcConfig,
) -> DrcResult {
    let start = crate::common::time_compat::Instant::now();
    let (components, wires, net_labels, junctions) =
        extract_drc_data_with_hierarchy_and_junctions(schematic, hierarchy);
    let severity_overrides = config.severity_overrides.clone();
    let mut checker = DrcChecker::with_config(config);
    checker.set_net_naming_policy(schematic.document_policy.net_naming);
    let mut result =
        checker.check_connectivity_with_junctions(&components, &wires, &net_labels, &junctions);
    append_bus_violations(schematic, &mut result, &severity_overrides);
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

        let (components, _, _) = extract_drc_data_with_hierarchy(&schematic, &hierarchy);
        let pins: HashMap<_, _> = components[0]
            .pins
            .iter()
            .map(|pin| (pin.name.as_str(), (pin.x, pin.y)))
            .collect();

        assert_eq!(pins.get("IN"), Some(&(Some(60.0), Some(40.0))));
        assert_eq!(pins.get("OUT"), Some(&(Some(170.0), Some(70.0))));
    }

    #[test]
    fn junction_aware_extraction_passes_persisted_junction_positions() {
        let mut schematic = SchematicState::default();
        schematic.add_junction(Point::new(40, -20));
        schematic.add_junction(Point::new(0, 0));

        let (_, _, _, junctions) = extract_drc_data_with_junctions(&schematic);

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

        let result = run_drc_check_with_config(&schematic, config);
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
        let result = run_drc_check_with_config(
            &schematic,
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
