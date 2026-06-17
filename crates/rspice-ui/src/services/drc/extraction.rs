use super::checker::{DrcChecker, DrcConfig};
use super::input::{ComponentInfo, NetLabelInfo, PinInfo, WireInfo};
use super::types::DrcResult;
use crate::simulation::netlist_gen::HierarchySource;
use crate::state::{Component, Point};

/// Extract DRC-compatible data from a SchematicState.
///
/// This is the bridge between the schematic representation and the DRC checker.
/// It extracts components, wires, and net labels in the format required by
/// DrcChecker::check_connectivity().
///
/// # Performance
/// - O(n) iteration over components
/// - O(n) iteration over wires  
/// - O(n) iteration over net labels
///
/// # Example
/// ```ignore
/// use rspice_ui::services::drc::{DrcChecker, extract_drc_data};
///
/// let (components, wires, net_labels) = extract_drc_data(&schematic);
/// let result = DrcChecker::new().check_connectivity(&components, &wires, &net_labels);
/// ```
pub fn extract_drc_data(
    schematic: &crate::state::SchematicState,
) -> (Vec<ComponentInfo>, Vec<WireInfo>, Vec<NetLabelInfo>) {
    extract_drc_data_with_terminals(schematic, |comp| {
        comp.terminal_positions()
            .into_iter()
            .map(|(name, pos)| (name.to_owned(), pos))
            .collect()
    })
}

/// Extract DRC data with project-cell symbol resolution enabled.
pub fn extract_drc_data_with_hierarchy(
    schematic: &crate::state::SchematicState,
    hierarchy: &HierarchySource<'_>,
) -> (Vec<ComponentInfo>, Vec<WireInfo>, Vec<NetLabelInfo>) {
    extract_drc_data_with_terminals(schematic, |comp| {
        let resolved_symbol = comp
            .library_cell
            .as_ref()
            .and_then(|binding| hierarchy.resolved_symbol_for(binding));
        comp.terminal_positions_resolved(resolved_symbol.as_ref())
    })
}

fn extract_drc_data_with_terminals(
    schematic: &crate::state::SchematicState,
    mut terminal_positions_for: impl FnMut(&Component) -> Vec<(String, Point)>,
) -> (Vec<ComponentInfo>, Vec<WireInfo>, Vec<NetLabelInfo>) {
    use crate::state::ComponentType;

    let mut components = Vec::with_capacity(schematic.components.len());
    let mut wires = Vec::with_capacity(schematic.wires.len());
    let mut net_labels = Vec::with_capacity(schematic.net_labels.len());

    // Build point-to-net mapping from existing net_mapping or create from connectivity
    let net_mapping = &schematic.net_mapping;

    // Extract components
    for comp in &schematic.components {
        let terminal_positions = terminal_positions_for(comp);
        let mut pins = Vec::with_capacity(terminal_positions.len());

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
            ) && pin_name == "+";

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
            id: comp.id as usize,
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
                    id: wire.id as usize,
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

    (components, wires, net_labels)
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
    let (components, wires, net_labels) = extract_drc_data(schematic);
    let mut checker = DrcChecker::new();
    checker.check_connectivity(&components, &wires, &net_labels)
}

/// Run a complete DRC check with project-cell symbol resolution enabled.
pub fn run_drc_check_with_hierarchy(
    schematic: &crate::state::SchematicState,
    hierarchy: &HierarchySource<'_>,
) -> DrcResult {
    let (components, wires, net_labels) = extract_drc_data_with_hierarchy(schematic, hierarchy);
    let mut checker = DrcChecker::new();
    checker.check_connectivity(&components, &wires, &net_labels)
}

/// Run a complete DRC check with custom configuration.
pub fn run_drc_check_with_config(
    schematic: &crate::state::SchematicState,
    config: DrcConfig,
) -> DrcResult {
    let (components, wires, net_labels) = extract_drc_data(schematic);
    let mut checker = DrcChecker::with_config(config);
    checker.check_connectivity(&components, &wires, &net_labels)
}

/// Run a configured DRC check with project-cell symbol resolution enabled.
pub fn run_drc_check_with_hierarchy_and_config(
    schematic: &crate::state::SchematicState,
    hierarchy: &HierarchySource<'_>,
    config: DrcConfig,
) -> DrcResult {
    let (components, wires, net_labels) = extract_drc_data_with_hierarchy(schematic, hierarchy);
    let mut checker = DrcChecker::with_config(config);
    checker.check_connectivity(&components, &wires, &net_labels)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        Cell, CellViewRef, Component, ComponentType, Library, LibraryCellInstance, LibraryManager,
        PortDirection, PortSpec, SchematicState, SymbolDocument, SymbolPin, View, ViewType,
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
}
