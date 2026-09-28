//! Host source checks and projected cross-sheet geometry.

pub(super) use super::hierarchy_resolver::hierarchy_stop_view;
pub(crate) use super::hierarchy_resolver::metadata_value;
use super::*;

pub(super) fn materialize_authoritative_source_binding(
    placed: &LibraryCellInstance,
    library: &Library,
    cell: &Cell,
    view: &View,
    workspace: &ProjectWorkspace,
    libraries: &LibraryManager,
) -> Result<LibraryCellInstance, String> {
    super::hierarchy_resolver::materialize_authoritative_source_binding(
        placed,
        library,
        cell,
        view,
        workspace.project.id(),
        &workspace.project_sources,
        libraries.catalog(),
    )
}

pub(crate) fn project_veriloga_binding_for_view(
    workspace: &ProjectWorkspace,
    libraries: &LibraryManager,
    reference: &CellViewRef,
) -> Result<ConfigurationVerilogABinding, String> {
    super::hierarchy_resolver::project_veriloga_binding_for_view(
        workspace.project.id(),
        &workspace.project_sources,
        libraries.catalog(),
        reference,
    )
}

pub(super) fn translated_point(
    point: crate::state::Point,
    delta: crate::state::Point,
) -> Result<crate::state::Point, crate::state::DesignManagementError> {
    Ok(crate::state::Point::new(
        point
            .x
            .checked_add(delta.x)
            .ok_or(crate::state::DesignManagementError::NumericRange(
                "materialized sheet x coordinate",
            ))?,
        point
            .y
            .checked_add(delta.y)
            .ok_or(crate::state::DesignManagementError::NumericRange(
                "materialized sheet y coordinate",
            ))?,
    ))
}

/// The world point one named terminal of a placed instance sits at, read from
/// the instance's own durable placement and bound interface.
///
/// A bound cell instance names its terminals twice: by interface port where
/// the binding carries one, and by ordinal position otherwise. A contract may
/// have been authored against either spelling, so both are matched — against
/// the same transformed point in both cases, because placement, rotation and
/// mirror are what put the terminal there.
fn component_terminal_point(
    component: &crate::state::Component,
    terminal_name: &str,
) -> Option<crate::state::Point> {
    let interface = component.instance_pin_layout();
    component
        .terminal_positions()
        .into_iter()
        .enumerate()
        .find(|(index, (label, _))| {
            label.eq_ignore_ascii_case(terminal_name)
                || interface.get(*index).is_some_and(|(port, _)| {
                    port.as_deref()
                        .is_some_and(|port| port.eq_ignore_ascii_case(terminal_name))
                })
        })
        .map(|(_, (_, point))| point)
}

/// Resolve one typed cross-sheet endpoint against the authored topology and
/// then project it into the endpoint sheet's execution namespace. A wire
/// point must still lie on its retained conductor; a component terminal must
/// still sit on one. Stale contracts fail before DRC or netlisting rather
/// than silently connecting a label to a component origin.
///
/// Both endpoints resolve from durable design data alone. The runtime
/// connection cache is stripped on save and rebuilt only for the buffer the
/// session has open, so a contract that resolved through it failed for every
/// cell view that was not the active one.
pub(super) fn projected_cross_sheet_anchor(
    source: &SchematicState,
    projected: &SchematicState,
    endpoint: &crate::state::CrossSheetPortEndpoint,
    delta: crate::state::Point,
) -> Result<crate::state::Point, crate::state::DesignManagementError> {
    let authored_point = match &endpoint.anchor {
        crate::state::CrossSheetPortAnchor::WirePoint { wire_id, point } => {
            let wire = source
                .document()
                .wires
                .iter()
                .find(|wire| wire.id == *wire_id)
                .ok_or_else(|| crate::state::DesignManagementError::MissingReference {
                    domain: "cross-sheet wire anchor",
                    identity: wire_id.to_string(),
                })?;
            if !wire.contains_point(*point) {
                return Err(crate::state::DesignManagementError::MissingReference {
                    domain: "cross-sheet wire anchor point",
                    identity: format!("{}@{},{}", wire_id, point.x, point.y),
                });
            }
            *point
        }
        crate::state::CrossSheetPortAnchor::ComponentTerminal {
            component_id,
            terminal_name,
        } => {
            let component = source
                .document()
                .components
                .iter()
                .find(|component| component.id == *component_id)
                .ok_or_else(|| crate::state::DesignManagementError::MissingReference {
                    domain: "cross-sheet component anchor",
                    identity: component_id.to_string(),
                })?;
            let point = component_terminal_point(component, terminal_name).ok_or_else(|| {
                crate::state::DesignManagementError::MissingReference {
                    domain: "cross-sheet component terminal",
                    identity: format!("{}:{}", component_id, terminal_name),
                }
            })?;
            if !source
                .document()
                .wires
                .iter()
                .any(|wire| wire.contains_point(point))
            {
                return Err(crate::state::DesignManagementError::MissingReference {
                    domain: "cross-sheet component terminal connection",
                    identity: format!("{}:{}", component_id, terminal_name),
                });
            }
            point
        }
    };
    let anchor = translated_point(authored_point, delta)?;
    match &endpoint.anchor {
        crate::state::CrossSheetPortAnchor::WirePoint { wire_id, .. } => {
            if !projected
                .document()
                .wires
                .iter()
                .any(|wire| wire.id == *wire_id && wire.contains_point(anchor))
            {
                return Err(crate::state::DesignManagementError::MissingReference {
                    domain: "projected cross-sheet wire anchor",
                    identity: wire_id.to_string(),
                });
            }
        }
        crate::state::CrossSheetPortAnchor::ComponentTerminal { component_id, .. } => {
            if !projected
                .document()
                .components
                .iter()
                .any(|component| component.id == *component_id)
            {
                return Err(crate::state::DesignManagementError::MissingReference {
                    domain: "projected cross-sheet component anchor",
                    identity: component_id.to_string(),
                });
            }
        }
    }
    Ok(anchor)
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
        schematic.document.connections.clear();

        let main = workspace
            .design_management
            .bootstrap_for_cell_view(
                &key,
                "Main",
                [stationary, moved, stationary_wire, moved_wire],
            )
            .expect("governed sheet catalog");
        let catalog = workspace
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
                .document
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
        schematic.document.connections.clear();

        let main = workspace
            .design_management
            .bootstrap_for_cell_view(&key, "Main", [stationary, moved, stationary_wire])
            .expect("governed sheet catalog");
        let catalog = workspace
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
