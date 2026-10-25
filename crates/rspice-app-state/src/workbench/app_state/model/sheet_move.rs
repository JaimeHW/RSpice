//! The selection's move to another sheet or into a child cell, resolved
//! through one connectivity authority.
//!
//! Hierarchy extraction and a sheet move both carry the selected instances
//! across a boundary, so both read the same extraction plan: the instances'
//! terminals with their resolved contracts, and the nets the hierarchical
//! netlist joins them by. A boundary net then receives the same typed
//! contract whichever surface asked for the move.

use std::collections::HashMap;

use super::Model;
use crate::state::{
    CellViewRef, Component, ComponentType, CrossSheetDiscipline, CrossSheetPortAnchor,
    CrossSheetPortDefinition, CrossSheetPortDirection, CrossSheetPortEndpoint,
    CrossSheetSignalType, HierarchyExtractionPlan, HierarchyExtractionTerminal,
    HierarchyNetConnectivity, LibraryManager, MoveBoundaryResolution, PortDirection,
    PortDiscipline, SchematicState, SheetId, SheetMoveConnectivityPlan,
    hierarchy_terminal_direction, hierarchy_terminal_discipline,
};
use rspice_design::hierarchy::HierarchySource;
use rspice_design::schematic::owned::Schematic;
use rspice_simulation::netlist_gen::{
    NetlistResult, NetlistSourceData, generate_netlist_hierarchical,
};

impl Model {
    /// Whether the selection is one or more complete instances on a
    /// writable schematic, and nothing else: what hierarchy extraction and
    /// a sheet move both carry.
    pub fn create_hierarchy_available(&self) -> bool {
        let selection = &self.schematic.session.editor.selection;
        !self.schematic_edit_read_only()
            && !selection.components.is_empty()
            && selection.count() == selection.components.len()
            && selection.components.iter().all(|id| {
                self.schematic
                    .document()
                    .components
                    .iter()
                    .any(|component| component.id == *id && component.kind != ComponentType::Port)
            })
    }

    /// The selection's extraction plan: every terminal of the open drawing
    /// with its resolved contract, joined by the nets the hierarchical
    /// netlist finds.
    pub fn hierarchy_source_plan(&self) -> Result<HierarchyExtractionPlan, String> {
        let symbols = self.schematic_symbol_context();
        let component_bounds = self
            .schematic
            .document()
            .components
            .iter()
            .map(|component| (component.id, symbols.component_bounds_tuple(component)))
            .collect::<HashMap<_, _>>();
        let terminals =
            self.schematic
                .document()
                .components
                .iter()
                .flat_map(|component| {
                    symbols.named_terminal_points(component).into_iter().map(
                        move |(name, point)| {
                            let (direction, discipline) =
                                self.resolved_terminal_contract(component, &name);
                            HierarchyExtractionTerminal {
                                component_id: component.id,
                                direction,
                                discipline,
                                terminal_name: name,
                                point,
                            }
                        },
                    )
                })
                .collect::<Vec<_>>();
        let result = Self::hierarchy_netlist(
            &self.schematic,
            &self.workspace.content.schematic_buffers,
            &self.library_manager,
        );
        self.schematic
            .plan_hierarchy_extraction(
                &terminals,
                &HierarchyNetConnectivity {
                    point_to_net: result.point_to_net,
                    net_segments: result.net_segments,
                },
                &component_bounds,
            )
            .map_err(|error| error.to_string())
    }

    /// Resolve the live selected-instance partition through the same
    /// canonical connectivity authority used by transactional hierarchy
    /// extraction. Sheet management consumes this result to preserve
    /// internal conductors and create exact typed contracts for every
    /// source/destination boundary terminal.
    pub fn selected_component_sheet_move_plan(&self) -> Result<SheetMoveConnectivityPlan, String> {
        if !self.create_hierarchy_available() {
            return Err(
                "A governed sheet move requires one or more complete, writable schematic instances and no partial or non-instance selections."
                    .to_owned(),
            );
        }
        self.hierarchy_source_plan()?
            .sheet_move_connectivity(self.schematic.document(), self.schematic.topology_version())
            .map_err(|error| error.to_string())
    }

    /// The explicit boundary-port contract for one sheet move of `plan`
    /// from `source` to `destination`.
    pub fn sheet_move_boundary_resolution(
        plan: &SheetMoveConnectivityPlan,
        source: SheetId,
        destination: SheetId,
    ) -> MoveBoundaryResolution {
        let ports = plan
            .boundaries
            .iter()
            .map(|boundary| CrossSheetPortDefinition {
                net_name: boundary.net_name.clone(),
                first: CrossSheetPortEndpoint {
                    sheet_id: source,
                    anchor: CrossSheetPortAnchor::WirePoint {
                        wire_id: boundary.stationary_wire_id,
                        point: boundary.stationary_point,
                    },
                },
                second: CrossSheetPortEndpoint {
                    sheet_id: destination,
                    anchor: CrossSheetPortAnchor::ComponentTerminal {
                        component_id: boundary.moved_component_id,
                        terminal_name: boundary.moved_terminal_name.clone(),
                    },
                },
                direction: match boundary.direction {
                    PortDirection::In => CrossSheetPortDirection::Input,
                    PortDirection::Out => CrossSheetPortDirection::Output,
                    PortDirection::InOut => CrossSheetPortDirection::InOut,
                    PortDirection::Supply => CrossSheetPortDirection::Supply,
                },
                signal_type: match (boundary.direction, boundary.discipline) {
                    (PortDirection::Supply, _) => CrossSheetSignalType::Power,
                    (_, PortDiscipline::Logic) => CrossSheetSignalType::Logic,
                    _ => CrossSheetSignalType::Analog,
                },
                discipline: match boundary.discipline {
                    PortDiscipline::Electrical => CrossSheetDiscipline::Electrical,
                    PortDiscipline::Logic => CrossSheetDiscipline::Logic,
                    PortDiscipline::Wreal => CrossSheetDiscipline::Wreal,
                    PortDiscipline::Thermal => CrossSheetDiscipline::Thermal,
                },
            })
            .collect::<Vec<_>>();
        if ports.is_empty() {
            MoveBoundaryResolution::VerifiedNoBoundaryNets
        } else {
            MoveBoundaryResolution::ExplicitPorts { ports }
        }
    }

    /// The hierarchical netlist of `schematic` among `buffers` and
    /// `libraries`, which joins an extraction's terminals into nets.
    pub fn hierarchy_netlist(
        schematic: &SchematicState,
        buffers: &HashMap<String, Schematic>,
        libraries: &LibraryManager,
    ) -> NetlistResult {
        let hierarchy = HierarchySource::from_workspace(libraries.catalog(), buffers);
        generate_netlist_hierarchical(
            schematic,
            &[],
            &hierarchy,
            &NetlistSourceData::new(&crate::simulation::table_route::SourceFiles),
        )
    }

    /// The direction and discipline `component`'s `terminal_name` carries:
    /// its master's port contract where the master's schematic is open,
    /// else what the instance itself says.
    fn resolved_terminal_contract(
        &self,
        component: &Component,
        terminal_name: &str,
    ) -> (Option<PortDirection>, PortDiscipline) {
        let fallback = (
            hierarchy_terminal_direction(component, terminal_name),
            hierarchy_terminal_discipline(component, terminal_name),
        );
        let Some(binding) = component.library_cell.as_ref() else {
            return fallback;
        };
        let reference = CellViewRef::new(&binding.library, &binding.cell, "schematic");
        let Some(master) = self
            .workspace
            .content
            .schematic_buffers
            .get(&reference.key())
        else {
            return fallback;
        };
        master
            .document()
            .components
            .iter()
            .filter_map(|port| port.port_contract().map(|contract| (port, contract)))
            .find(|(port, _)| port.value.trim().eq_ignore_ascii_case(terminal_name))
            .map_or(fallback, |(_, contract)| {
                (Some(contract.direction), contract.discipline)
            })
    }
}
