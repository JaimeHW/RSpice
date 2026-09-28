//! Host source checks for project hierarchy materialization.

pub(crate) use super::hierarchy_resolver::metadata_value;
use super::*;

pub(crate) fn project_veriloga_binding_for_view(
    workspace: &ProjectWorkspace,
    libraries: &LibraryManager,
    reference: &CellViewRef,
) -> Result<ConfigurationVerilogABinding, String> {
    super::hierarchy_resolver::project_veriloga_binding_for_view(
        workspace.content.project.id(),
        &workspace.content.project_sources,
        libraries.catalog(),
        reference,
    )
}

pub(super) fn source_paths_match(left: &Path, right: &Path) -> bool {
    if left == right {
        return true;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        match (std::fs::canonicalize(left), std::fs::canonicalize(right)) {
            (Ok(left), Ok(right)) => left == right,
            _ => false,
        }
    }
    #[cfg(target_arch = "wasm32")]
    false
}

pub(super) fn configured_source_identity(path: &Path) -> String {
    #[cfg(not(target_arch = "wasm32"))]
    let path = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    #[cfg(target_arch = "wasm32")]
    let path = path.to_path_buf();
    path.to_string_lossy()
        .replace('\\', "/")
        .to_ascii_lowercase()
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn validate_source_file(
    source_path: &Path,
    view_type: ViewType,
    binding: &LibraryCellInstance,
) -> Result<(), String> {
    let source = if let Some(section) = binding.model_section.as_deref() {
        let mut processor = rspice_core::netlist::IncludeProcessor::new(source_path);
        processor
            .process_lib(&source_path.to_string_lossy(), Some(section))
            .map_err(|error| {
                format!(
                    "source-backed binding {}/{}/{} cannot resolve model section '{}' from {}: {error}",
                    binding.library,
                    binding.cell,
                    binding.view,
                    section,
                    source_path.display()
                )
            })?
    } else {
        std::fs::read_to_string(source_path).map_err(|error| {
            format!(
                "source-backed binding {}/{}/{} cannot read {}: {error}",
                binding.library,
                binding.cell,
                binding.view,
                source_path.display()
            )
        })?
    };
    let master = binding
        .module_name
        .as_deref()
        .filter(|name| !name.trim().is_empty())
        .unwrap_or(&binding.cell);
    let declaration_found = match view_type {
        ViewType::Verilog | ViewType::VerilogA => source.lines().any(|line| {
            let code = line.split("//").next().unwrap_or_default();
            let mut tokens = code
                .split(|character: char| !character.is_alphanumeric() && character != '_')
                .filter(|token| !token.is_empty());
            tokens.any(|token| token.eq_ignore_ascii_case("module"))
                && tokens.any(|token| token.eq_ignore_ascii_case(master))
        }),
        ViewType::Spice | ViewType::Extracted => source.lines().any(|line| {
            let mut tokens = line.split_ascii_whitespace();
            let directive = tokens.next();
            let declared_name = tokens.next();
            let subcircuit_matches = directive
                .is_some_and(|token| token.eq_ignore_ascii_case(".subckt"))
                && declared_name.is_some_and(|token| token.eq_ignore_ascii_case(master));
            let model_matches = binding.netlist_template.is_some()
                && directive.is_some_and(|token| token.eq_ignore_ascii_case(".model"))
                && declared_name.is_some_and(|token| token.eq_ignore_ascii_case(master));
            subcircuit_matches || model_matches
        }),
        _ => false,
    };
    if declaration_found {
        Ok(())
    } else {
        Err(format!(
            "source-backed binding {}/{}/{} does not declare executable master {master}",
            binding.library, binding.cell, binding.view
        ))
    }
}

#[cfg(target_arch = "wasm32")]
pub(super) fn validate_source_file(
    _source_path: &Path,
    _view_type: ViewType,
    binding: &LibraryCellInstance,
) -> Result<(), String> {
    Err(format!(
        "source-backed binding {}/{}/{} references a desktop path unavailable in this browser session",
        binding.library, binding.cell, binding.view
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        ComponentType, CrossSheetDiscipline, CrossSheetPortAnchor, CrossSheetPortDefinition,
        CrossSheetPortDirection, CrossSheetPortEndpoint, CrossSheetSignalType,
        MoveBoundaryResolution, MoveSelectionRequest, Point, SheetDefinition, SheetPortPolicy,
        SheetTemplate,
    };

    /// Closes the reload defect: a crossing anchored to a component terminal
    /// used to resolve through the rubber-band connection cache, which is
    /// stripped on save and rebuilt only for the buffer the session has open.
    /// Every other governed cell view therefore failed to materialize.
    #[test]
    fn a_component_terminal_crossing_resolves_without_the_connection_cache() {
        let mut workspace = ProjectWorkspace::default();
        let key = CellViewRef::default_top().key();
        let mut schematic = SchematicState::default();
        let stationary = schematic.add_component(ComponentType::Diode, Point::new(0, 0));
        let moved = schematic.add_component(ComponentType::Diode, Point::new(200, 0));
        let stationary_wire = schematic
            .add_wire(vec![Point::new(20, 0), Point::new(60, 0)])
            .expect("a conductor reaching the stationary cathode");
        let moved_wire = schematic
            .add_wire(vec![Point::new(140, 0), Point::new(180, 0)])
            .expect("a conductor reaching the moved anode");
        schematic.document_mut_for_test().connections.clear();

        let main = workspace
            .content
            .design_management
            .bootstrap_for_cell_view(
                &key,
                "Main",
                [stationary, moved, stationary_wire, moved_wire],
            )
            .expect("governed sheet catalog");
        let catalog = workspace
            .content
            .design_management
            .sheet_catalog_mut(&key)
            .expect("the catalog just bootstrapped");
        let auxiliary = catalog
            .create_sheet(
                SheetDefinition {
                    name: "Auxiliary".to_owned(),
                    template: SheetTemplate::AnalogSchematic,
                    port_policy: SheetPortPolicy::TypedOffSheetPorts,
                    explicit_page_number: Some(2),
                },
                Some(main),
            )
            .expect("second sheet");
        catalog
            .move_selection(MoveSelectionRequest {
                expected_catalog_revision: catalog.revision(),
                object_ids: vec![moved, moved_wire],
                destination_sheet_id: auxiliary,
                boundary_resolution: MoveBoundaryResolution::ExplicitPorts {
                    ports: vec![CrossSheetPortDefinition {
                        net_name: "BIAS".to_owned(),
                        first: CrossSheetPortEndpoint {
                            sheet_id: main,
                            anchor: CrossSheetPortAnchor::ComponentTerminal {
                                component_id: stationary,
                                terminal_name: "K".to_owned(),
                            },
                        },
                        second: CrossSheetPortEndpoint {
                            sheet_id: auxiliary,
                            anchor: CrossSheetPortAnchor::ComponentTerminal {
                                component_id: moved,
                                terminal_name: "A".to_owned(),
                            },
                        },
                        direction: CrossSheetPortDirection::Output,
                        signal_type: CrossSheetSignalType::Analog,
                        discipline: CrossSheetDiscipline::Electrical,
                    }],
                },
            })
            .expect("reviewed cross-sheet move");

        let projected = workspace
            .materialize_design_management_schematic(&key, &schematic)
            .expect("a governed design materializes from durable data alone");

        assert_eq!(
            projected
                .document()
                .net_labels
                .iter()
                .filter(|label| label.name == "BIAS")
                .count(),
            2,
            "one off-sheet connector per side of the contract"
        );
    }

    /// A contract whose terminal no longer sits on any conductor is stale, and
    /// must fail before DRC or netlisting rather than bind a bare pin.
    #[test]
    fn a_component_terminal_crossing_off_every_conductor_is_rejected() {
        let mut workspace = ProjectWorkspace::default();
        let key = CellViewRef::default_top().key();
        let mut schematic = SchematicState::default();
        let stationary = schematic.add_component(ComponentType::Diode, Point::new(0, 0));
        let moved = schematic.add_component(ComponentType::Diode, Point::new(200, 0));
        let stationary_wire = schematic
            .add_wire(vec![Point::new(20, 0), Point::new(60, 0)])
            .expect("a conductor reaching the stationary cathode");
        schematic.document_mut_for_test().connections.clear();

        let main = workspace
            .content
            .design_management
            .bootstrap_for_cell_view(&key, "Main", [stationary, moved, stationary_wire])
            .expect("governed sheet catalog");
        let catalog = workspace
            .content
            .design_management
            .sheet_catalog_mut(&key)
            .expect("the catalog just bootstrapped");
        let auxiliary = catalog
            .create_sheet(
                SheetDefinition {
                    name: "Auxiliary".to_owned(),
                    template: SheetTemplate::AnalogSchematic,
                    port_policy: SheetPortPolicy::TypedOffSheetPorts,
                    explicit_page_number: Some(2),
                },
                Some(main),
            )
            .expect("second sheet");
        catalog
            .move_selection(MoveSelectionRequest {
                expected_catalog_revision: catalog.revision(),
                object_ids: vec![moved],
                destination_sheet_id: auxiliary,
                boundary_resolution: MoveBoundaryResolution::ExplicitPorts {
                    ports: vec![CrossSheetPortDefinition {
                        net_name: "BIAS".to_owned(),
                        first: CrossSheetPortEndpoint {
                            sheet_id: main,
                            anchor: CrossSheetPortAnchor::ComponentTerminal {
                                component_id: stationary,
                                terminal_name: "K".to_owned(),
                            },
                        },
                        second: CrossSheetPortEndpoint {
                            sheet_id: auxiliary,
                            anchor: CrossSheetPortAnchor::ComponentTerminal {
                                component_id: moved,
                                terminal_name: "A".to_owned(),
                            },
                        },
                        direction: CrossSheetPortDirection::Output,
                        signal_type: CrossSheetSignalType::Analog,
                        discipline: CrossSheetDiscipline::Electrical,
                    }],
                },
            })
            .expect("reviewed cross-sheet move");

        workspace
            .materialize_design_management_schematic(&key, &schematic)
            .expect_err("a terminal that touches no conductor is not a connection");
    }
}
