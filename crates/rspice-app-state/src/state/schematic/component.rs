//! Component document exports and app integration coverage.

pub use rspice_design::schematic::component::{
    Component, ComponentDisplayMode, InstanceMultiplicity, LibraryCellInstance,
    explicit_component_model,
};
pub use rspice_model_library::symbol::validate_library_netlist_template;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Point, Rotation};

    use crate::state::{
        Cell, ComponentType, Library, LibraryManager, PortDirection, PortSpec, SchematicState,
        SymbolDocument, SymbolPin, SymbolResolver, View, ViewType,
    };
    use std::collections::HashMap;

    fn port(name: &str, direction: PortDirection) -> PortSpec {
        PortSpec {
            name: name.to_owned(),
            direction,
        }
    }

    fn resolved_amp_symbol() -> crate::state::ResolvedCellSymbol {
        let document = SymbolDocument {
            pins: vec![
                SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(70, 20))),
                SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, -10))),
            ],
            ..SymbolDocument::default()
        };

        let mut libraries = LibraryManager::new();
        let mut library = Library::new("work");
        let mut cell = Cell::new("amp");
        let mut symbol_view = View::new("symbol", ViewType::Symbol);
        document
            .store_in_view(&mut symbol_view)
            .expect("symbol stores");
        cell.add_view(symbol_view);
        library.add_cell(cell);
        libraries.add_library(library);

        let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&[
            port("IN", PortDirection::In),
            port("OUT", PortDirection::Out),
        ]);

        let buffers = HashMap::<String, SchematicState>::new();
        SymbolResolver::new(&libraries, &buffers)
            .resolve_binding(&binding)
            .expect("symbol resolves")
    }

    #[test]
    fn resolved_instance_terminals_use_authored_symbol_offsets() {
        let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&[
            port("IN", PortDirection::In),
            port("OUT", PortDirection::Out),
        ]);
        let component = Component::new(7, ComponentType::CellInstance, Point::new(100, 50))
            .with_library_cell(binding)
            .with_rotation(Rotation::R90)
            .with_mirror_h(true);
        let resolved = resolved_amp_symbol();

        let terminals = component.terminal_positions_resolved(Some(&resolved));

        assert_eq!(
            terminals,
            vec![
                ("IN".to_owned(), Point::new(110, 90)),
                ("OUT".to_owned(), Point::new(80, -20)),
            ]
        );
    }

    #[test]
    fn an_adopted_source_round_trips_its_provenance() {
        let mut source = Component::new(3, ComponentType::VoltageSourceSin, Point::origin())
            .with_name_value("V3", "0");
        source.params = "va=3m freq=1k".to_owned();
        let definition = crate::state::stimulus_library::definition::StimulusDefinition::new(
            "sensor_diff_1k",
            ComponentType::VoltageSourceSin,
            crate::state::stimulus_library::now_unix_ms,
        )
        .expect("definition");
        definition.adopt_onto(&mut source).expect("adopt");

        let encoded = ron::ser::to_string(&source).expect("serialize");
        let decoded: Component = ron::from_str(&encoded).expect("deserialize");
        assert_eq!(decoded, source);
        assert_eq!(
            decoded
                .stimulus_provenance
                .as_ref()
                .map(|provenance| provenance.definition.as_str()),
            Some("sensor_diff_1k")
        );
    }
}
