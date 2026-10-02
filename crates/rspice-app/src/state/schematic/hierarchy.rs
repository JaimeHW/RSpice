//! Editor candidates for hierarchy extraction.

use super::hierarchy_edit;
#[cfg(test)]
use super::{ComponentType, PortDirection, PortDiscipline, Rotation};
use super::{LibraryCellInstance, Point, SchematicState};
pub use hierarchy_edit::{
    HierarchyExtractionError, HierarchyExtractionPlan, HierarchyExtractionTerminal,
    HierarchyNetConnectivity, SheetMoveConnectivityPlan, hierarchy_terminal_direction,
    hierarchy_terminal_discipline,
};
#[cfg(test)]
use std::collections::BTreeSet;
use std::collections::HashMap;

/// Complete non-mutating result ready for project-level validation/publication.
#[derive(Debug, Clone)]
pub struct HierarchyExtractionCandidate {
    pub parent: SchematicState,
    pub child: SchematicState,
    pub instance_id: u64,
    pub instance_name: String,
    pub binding: LibraryCellInstance,
}

impl HierarchyExtractionCandidate {
    pub fn validate_connectivity(
        &self,
        plan: &HierarchyExtractionPlan,
        parent_point_to_net: &HashMap<Point, String>,
        child_point_to_net: &HashMap<Point, String>,
    ) -> Result<(), HierarchyExtractionError> {
        hierarchy_edit::validate_candidate_connectivity(
            &self.parent.design.document(),
            &self.child.design.document(),
            self.instance_id,
            plan,
            parent_point_to_net,
            child_point_to_net,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::netlist_gen::generate_netlist;
    use crate::state::WireConnection;

    fn terminals(schematic: &SchematicState) -> Vec<HierarchyExtractionTerminal> {
        schematic
            .design
            .document()
            .components
            .iter()
            .flat_map(|component| {
                component
                    .terminal_positions_resolved(None)
                    .into_iter()
                    .map(move |(name, point)| HierarchyExtractionTerminal {
                        component_id: component.id,
                        direction: hierarchy_terminal_direction(component, &name),
                        discipline: hierarchy_terminal_discipline(component, &name),
                        terminal_name: name,
                        point,
                    })
            })
            .collect()
    }

    fn connectivity(schematic: &SchematicState) -> HierarchyNetConnectivity {
        let result = generate_netlist(schematic);
        HierarchyNetConnectivity {
            point_to_net: result.point_to_net,
            net_segments: result.net_segments,
        }
    }

    fn bounds(schematic: &SchematicState) -> HashMap<u64, (i32, i32, i32, i32)> {
        schematic
            .design
            .document()
            .components
            .iter()
            .map(|component| (component.id, component.bounding_box()))
            .collect()
    }

    fn normalized_segments(
        segments: impl IntoIterator<Item = (Point, Point)>,
    ) -> Vec<(i32, i32, i32, i32)> {
        let mut result = segments
            .into_iter()
            .map(|(start, end)| {
                if (start.x, start.y) <= (end.x, end.y) {
                    (start.x, start.y, end.x, end.y)
                } else {
                    (end.x, end.y, start.x, start.y)
                }
            })
            .collect::<Vec<_>>();
        result.sort_unstable();
        result.dedup();
        result
    }

    #[test]
    fn extraction_moves_instances_infers_boundary_ports_and_preserves_internal_net() {
        let mut schematic = SchematicState::default();
        let r1 = schematic.add_component(ComponentType::Resistor, Point::new(0, 0));
        let r2 = schematic.add_component(ComponentType::Resistor, Point::new(80, 0));
        let load = schematic.add_component(ComponentType::Resistor, Point::new(160, 0));
        schematic.add_wire(vec![Point::new(20, 0), Point::new(60, 0)]);
        schematic.add_wire(vec![Point::new(100, 0), Point::new(140, 0)]);
        schematic.session.editor.selection.select_component(r1);
        schematic.session.editor.selection.select_component(r2);

        let plan = schematic
            .plan_hierarchy_extraction(
                &terminals(&schematic),
                &connectivity(&schematic),
                &bounds(&schematic),
            )
            .expect("plan");
        assert_eq!(plan.source_net_count, 3);
        assert_eq!(plan.ports.len(), 1);
        let candidate = schematic
            .materialize_hierarchy_extraction(&plan, "work", "sensor_frontend", "schematic")
            .expect("candidate");
        assert_eq!(candidate.parent.design.document().components.len(), 2);
        assert!(
            candidate
                .parent
                .design
                .document()
                .components
                .iter()
                .any(|component| component.id == load)
        );
        assert_eq!(
            candidate
                .child
                .design
                .document()
                .components
                .iter()
                .filter(|component| component.kind != ComponentType::Port)
                .count(),
            2
        );
        assert_eq!(candidate.child.interface_ports().len(), 1);
    }

    #[test]
    fn mixed_selection_and_parent_ports_fail_closed_without_mutation() {
        let mut schematic = SchematicState::default();
        let r1 = schematic.add_component(ComponentType::Resistor, Point::origin());
        let p1 = schematic.add_component(ComponentType::Port, Point::new(40, 0));
        let before =
            super::super::undo_history::SchematicSnapshot::capture(&schematic.design.document());
        schematic.session.editor.selection.select_component(r1);
        schematic.session.editor.selection.select_component(p1);
        let error = schematic
            .plan_hierarchy_extraction(
                &terminals(&schematic),
                &connectivity(&schematic),
                &bounds(&schematic),
            )
            .expect_err("parent port is rejected");
        assert!(matches!(
            error,
            HierarchyExtractionError::InterfacePortSelected(_)
        ));
        assert!(before.is_equal_document(&schematic.design.document()));
    }

    #[test]
    fn branched_boundary_prunes_only_the_unretained_selected_stub() {
        let mut schematic = SchematicState::default();
        let outside = schematic.add_component(ComponentType::Resistor, Point::new(-80, 0));
        let selected_main = schematic.add_component(ComponentType::Resistor, Point::origin());
        let selected_branch = schematic.add_component(ComponentType::Resistor, Point::new(-40, 60));
        schematic
            .design
            .document_mut_for_test()
            .components
            .iter_mut()
            .find(|component| component.id == selected_branch)
            .expect("branch")
            .rotation = Rotation::R90;
        schematic.add_wire(vec![Point::new(-60, 0), Point::new(-20, 0)]);
        schematic.add_wire(vec![Point::new(-40, 0), Point::new(-40, 40)]);
        schematic.add_junction(Point::new(-40, 0));
        schematic.add_net_label(Point::new(-40, 0), "sense".to_owned());
        schematic
            .session
            .editor
            .selection
            .select_component(selected_main);
        schematic
            .session
            .editor
            .selection
            .select_component(selected_branch);

        let plan = schematic
            .plan_hierarchy_extraction(
                &terminals(&schematic),
                &connectivity(&schematic),
                &bounds(&schematic),
            )
            .expect("plan");
        let candidate = schematic
            .materialize_hierarchy_extraction(&plan, "work", "child", "schematic")
            .expect("candidate");
        assert!(
            candidate
                .parent
                .design
                .document()
                .components
                .iter()
                .any(|component| component.id == outside)
        );
        assert!(
            candidate
                .parent
                .design
                .document()
                .net_labels
                .iter()
                .any(|label| { label.name == "sense" && label.pos == Point::new(-40, 0) })
        );
        assert!(
            candidate
                .parent
                .design
                .document()
                .junctions
                .iter()
                .any(|junction| { junction.pos == Point::new(-40, 0) })
        );
        assert!(
            !candidate.parent.design.document().wires.iter().any(|wire| {
                wire.segments().any(|segment| {
                    segment.is_orthogonal()
                        && segment.contains_point(Point::new(-20, 0))
                        && segment.start != Point::new(-20, 0)
                        && segment.end != Point::new(-20, 0)
                })
            })
        );

        let parent = generate_netlist(&candidate.parent);
        let child = generate_netlist(&candidate.child);
        candidate
            .validate_connectivity(&plan, &parent.point_to_net, &child.point_to_net)
            .expect("electrical partition remains exact");
    }

    #[test]
    fn closed_selection_creates_an_explicit_zero_port_hierarchy_instance() {
        let mut schematic = SchematicState::default();
        let r1 = schematic.add_component(ComponentType::Resistor, Point::origin());
        let r2 = schematic.add_component(ComponentType::Resistor, Point::new(80, 0));
        schematic.add_wire(vec![Point::new(20, 0), Point::new(60, 0)]);
        schematic.session.editor.selection.select_component(r1);
        schematic.session.editor.selection.select_component(r2);

        let plan = schematic
            .plan_hierarchy_extraction(
                &terminals(&schematic),
                &connectivity(&schematic),
                &bounds(&schematic),
            )
            .expect("plan");
        assert!(plan.ports.is_empty());
        let candidate = schematic
            .materialize_hierarchy_extraction(&plan, "work", "closed", "schematic")
            .expect("candidate");
        assert_eq!(candidate.binding.interface(), Some(Vec::new()));
        assert!(
            candidate
                .parent
                .design
                .document()
                .components
                .iter()
                .find(|component| component.id == candidate.instance_id)
                .expect("instance")
                .terminal_positions()
                .is_empty()
        );
        assert!(candidate.child.interface_ports().is_empty());

        let parent = generate_netlist(&candidate.parent);
        let child = generate_netlist(&candidate.child);
        candidate
            .validate_connectivity(&plan, &parent.point_to_net, &child.point_to_net)
            .expect("closed topology remains exact");
    }

    #[test]
    fn global_ground_is_preserved_without_becoming_a_hierarchy_port() {
        let mut schematic = SchematicState::default();
        let selected = schematic.add_component(ComponentType::Resistor, Point::origin());
        schematic.add_component(ComponentType::Ground, Point::new(-20, 10));
        schematic
            .session
            .editor
            .selection
            .select_component(selected);

        let plan = schematic
            .plan_hierarchy_extraction(
                &terminals(&schematic),
                &connectivity(&schematic),
                &bounds(&schematic),
            )
            .expect("plan");
        assert!(plan.ports.iter().all(|port| port.source_net != "0"));
        let candidate = schematic
            .materialize_hierarchy_extraction(&plan, "work", "grounded", "schematic")
            .expect("candidate");
        assert!(
            candidate
                .child
                .design
                .document()
                .components
                .iter()
                .any(|component| component.kind == ComponentType::Ground)
        );
        let parent = generate_netlist(&candidate.parent);
        let child = generate_netlist(&candidate.child);
        candidate
            .validate_connectivity(&plan, &parent.point_to_net, &child.point_to_net)
            .expect("global node zero remains exact");
    }

    #[test]
    fn mixed_signal_bridge_infers_terminal_specific_disciplines() {
        let mut schematic = SchematicState::default();
        let adc = schematic.add_component(ComponentType::XspiceAdcBridge, Point::origin());
        schematic.add_component(ComponentType::Resistor, Point::new(80, 0));
        schematic.add_wire(vec![Point::new(20, 0), Point::new(60, 0)]);
        schematic.session.editor.selection.select_component(adc);

        let plan = schematic
            .plan_hierarchy_extraction(
                &terminals(&schematic),
                &connectivity(&schematic),
                &bounds(&schematic),
            )
            .expect("plan");
        assert_eq!(plan.ports.len(), 1);
        assert_eq!(plan.ports[0].discipline, PortDiscipline::Logic);
        assert_eq!(plan.ports[0].direction, PortDirection::Out);
    }

    #[test]
    fn extraction_preserves_complete_authored_internal_geometry_and_objects() {
        let mut schematic = SchematicState::default();
        let r1 = schematic.add_component(ComponentType::Resistor, Point::origin());
        let r2 = schematic.add_component(ComponentType::Resistor, Point::new(100, 0));
        schematic.add_wire(vec![
            Point::new(20, 0),
            Point::new(20, -20),
            Point::new(80, -20),
            Point::new(80, 0),
        ]);
        schematic.add_wire(vec![
            Point::new(20, 0),
            Point::new(20, 20),
            Point::new(80, 20),
            Point::new(80, 0),
        ]);
        schematic.add_wire(vec![Point::new(50, -20), Point::new(50, -40)]);
        schematic.add_junction(Point::new(50, -20));
        schematic.add_net_label(Point::new(50, -40), "sense".to_owned());
        schematic.session.editor.selection.select_component(r1);
        schematic.session.editor.selection.select_component(r2);

        let source_connectivity = connectivity(&schematic);
        let plan = schematic
            .plan_hierarchy_extraction(
                &terminals(&schematic),
                &source_connectivity,
                &bounds(&schematic),
            )
            .expect("plan");
        let candidate = schematic
            .materialize_hierarchy_extraction(&plan, "work", "loop", "schematic")
            .expect("candidate");
        let child_connectivity = connectivity(&candidate.child);

        let expected = normalized_segments(source_connectivity.net_segments["sense"].iter().map(
            |(start, end)| {
                (
                    Point::new(start.x - plan.origin.x, start.y - plan.origin.y),
                    Point::new(end.x - plan.origin.x, end.y - plan.origin.y),
                )
            },
        ));
        assert_eq!(
            normalized_segments(child_connectivity.net_segments["sense"].iter().copied()),
            expected,
            "cycles and dangling stubs remain exact"
        );
        assert!(
            candidate
                .child
                .design
                .document()
                .junctions
                .iter()
                .any(|junction| {
                    junction.pos == Point::new(50 - plan.origin.x, -20 - plan.origin.y)
                })
        );
        assert!(
            candidate
                .child
                .design
                .document()
                .net_labels
                .iter()
                .any(|label| {
                    label.name == "sense"
                        && label.pos == Point::new(50 - plan.origin.x, -40 - plan.origin.y)
                })
        );
        assert!(candidate.parent.design.document().wires.is_empty());
        assert!(candidate.parent.design.document().junctions.is_empty());
        assert!(candidate.parent.design.document().net_labels.is_empty());
    }

    #[test]
    fn label_only_connectivity_is_preserved_without_fabricating_a_wire() {
        let mut schematic = SchematicState::default();
        let r1 = schematic.add_component(ComponentType::Resistor, Point::origin());
        let r2 = schematic.add_component(ComponentType::Resistor, Point::new(100, 0));
        schematic.add_net_label(Point::new(20, 0), "sense".to_owned());
        schematic.add_net_label(Point::new(80, 0), "sense".to_owned());
        schematic.session.editor.selection.select_component(r1);
        schematic.session.editor.selection.select_component(r2);

        let plan = schematic
            .plan_hierarchy_extraction(
                &terminals(&schematic),
                &connectivity(&schematic),
                &bounds(&schematic),
            )
            .expect("plan");
        let candidate = schematic
            .materialize_hierarchy_extraction(&plan, "work", "labels", "schematic")
            .expect("candidate");

        assert!(candidate.child.design.document().wires.is_empty());
        assert_eq!(
            candidate
                .child
                .design
                .document()
                .net_labels
                .iter()
                .filter(|label| label.name == "sense")
                .count(),
            2
        );
        let parent = generate_netlist(&candidate.parent);
        let child = generate_netlist(&candidate.child);
        candidate
            .validate_connectivity(&plan, &parent.point_to_net, &child.point_to_net)
            .expect("repeated labels retain the exact logical connection");
    }

    #[test]
    fn direct_pin_boundary_allows_route_contact_at_the_external_anchor() {
        let mut schematic = SchematicState::default();
        let selected = schematic.add_component(ComponentType::Resistor, Point::origin());
        schematic.add_component(ComponentType::Resistor, Point::new(40, 0));
        schematic
            .session
            .editor
            .selection
            .select_component(selected);

        let plan = schematic
            .plan_hierarchy_extraction(
                &terminals(&schematic),
                &connectivity(&schematic),
                &bounds(&schematic),
            )
            .expect("plan");
        assert_eq!(plan.ports.len(), 1);
        let candidate = schematic
            .materialize_hierarchy_extraction(&plan, "work", "direct_pin", "schematic")
            .expect("the exact pin contact is a valid route endpoint");
        let parent = generate_netlist(&candidate.parent);
        let child = generate_netlist(&candidate.child);
        candidate
            .validate_connectivity(&plan, &parent.point_to_net, &child.point_to_net)
            .expect("direct pin boundary remains connected");
    }

    #[test]
    fn boundary_wire_stays_in_parent_instead_of_being_duplicated_into_child() {
        let mut schematic = SchematicState::default();
        let selected = schematic.add_component(ComponentType::Resistor, Point::origin());
        schematic.add_component(ComponentType::Resistor, Point::new(100, 0));
        schematic.add_wire(vec![Point::new(20, 0), Point::new(80, 0)]);
        schematic
            .session
            .editor
            .selection
            .select_component(selected);

        let plan = schematic
            .plan_hierarchy_extraction(
                &terminals(&schematic),
                &connectivity(&schematic),
                &bounds(&schematic),
            )
            .expect("plan");
        let candidate = schematic
            .materialize_hierarchy_extraction(&plan, "work", "boundary", "schematic")
            .expect("candidate");

        assert_eq!(
            candidate.child.design.document().wires.len(),
            1,
            "the child owns only its generated port route"
        );
        assert!(
            candidate
                .parent
                .design
                .document()
                .wires
                .iter()
                .any(|wire| { wire.points == [Point::new(20, 0), Point::new(80, 0)] })
        );
        let parent = generate_netlist(&candidate.parent);
        let child = generate_netlist(&candidate.child);
        candidate
            .validate_connectivity(&plan, &parent.point_to_net, &child.point_to_net)
            .expect("boundary extraction retains the electrical contract");
    }

    #[test]
    fn sheet_move_plan_retains_boundary_wire_and_names_exact_moved_terminal() {
        let mut schematic = SchematicState::default();
        let selected = schematic.add_component(ComponentType::Resistor, Point::origin());
        let stationary = schematic.add_component(ComponentType::Resistor, Point::new(80, 0));
        let selected_terminal = schematic
            .design
            .document()
            .components
            .iter()
            .find(|component| component.id == selected)
            .expect("selected component")
            .terminal_positions_resolved(None)
            .into_iter()
            .max_by_key(|(_, point)| point.x)
            .expect("selected terminal");
        let stationary_terminal = schematic
            .design
            .document()
            .components
            .iter()
            .find(|component| component.id == stationary)
            .expect("stationary component")
            .terminal_positions_resolved(None)
            .into_iter()
            .min_by_key(|(_, point)| point.x)
            .expect("stationary terminal");
        let wire = schematic
            .add_wire(vec![selected_terminal.1, stationary_terminal.1])
            .expect("boundary wire");
        schematic
            .design
            .document_mut_for_test()
            .connections
            .push(WireConnection::new(
                wire,
                0,
                selected,
                selected_terminal.0.clone(),
            ));
        schematic
            .design
            .document_mut_for_test()
            .connections
            .push(WireConnection::new(
                wire,
                1,
                stationary,
                stationary_terminal.0,
            ));
        schematic
            .session
            .editor
            .selection
            .select_component(selected);

        let plan = schematic
            .plan_hierarchy_extraction(
                &terminals(&schematic),
                &connectivity(&schematic),
                &bounds(&schematic),
            )
            .expect("hierarchy connectivity plan");
        let sheet_move = plan
            .sheet_move_connectivity(&schematic.design.document(), schematic.topology_version())
            .expect("typed sheet move");

        assert_eq!(sheet_move.source_component_ids, [selected]);
        assert_eq!(sheet_move.moved_object_ids, [selected]);
        assert_eq!(sheet_move.boundaries.len(), 1);
        let boundary = &sheet_move.boundaries[0];
        assert_eq!(boundary.stationary_wire_id, wire);
        assert_eq!(boundary.stationary_point, selected_terminal.1);
        assert_eq!(boundary.moved_component_id, selected);
        assert_eq!(boundary.moved_terminal_name, selected_terminal.0);
    }

    #[test]
    fn sheet_move_plan_moves_complete_internal_scalar_topology() {
        let mut schematic = SchematicState::default();
        let left = schematic.add_component(ComponentType::Resistor, Point::origin());
        let right = schematic.add_component(ComponentType::Resistor, Point::new(80, 0));
        let left_terminal = schematic
            .design
            .document()
            .components
            .iter()
            .find(|component| component.id == left)
            .expect("left component")
            .terminal_positions_resolved(None)
            .into_iter()
            .max_by_key(|(_, point)| point.x)
            .expect("left terminal");
        let right_terminal = schematic
            .design
            .document()
            .components
            .iter()
            .find(|component| component.id == right)
            .expect("right component")
            .terminal_positions_resolved(None)
            .into_iter()
            .min_by_key(|(_, point)| point.x)
            .expect("right terminal");
        let wire = schematic
            .add_wire(vec![left_terminal.1, right_terminal.1])
            .expect("internal wire");
        schematic
            .design
            .document_mut_for_test()
            .connections
            .push(WireConnection::new(wire, 0, left, left_terminal.0));
        schematic
            .design
            .document_mut_for_test()
            .connections
            .push(WireConnection::new(wire, 1, right, right_terminal.0));
        let label = schematic.add_net_label(left_terminal.1, "sense".to_owned());
        schematic.session.editor.selection.select_component(left);
        schematic.session.editor.selection.select_component(right);

        let plan = schematic
            .plan_hierarchy_extraction(
                &terminals(&schematic),
                &connectivity(&schematic),
                &bounds(&schematic),
            )
            .expect("hierarchy connectivity plan");
        let sheet_move = plan
            .sheet_move_connectivity(&schematic.design.document(), schematic.topology_version())
            .expect("verified internal sheet move");

        assert!(sheet_move.boundaries.is_empty());
        assert_eq!(
            sheet_move.moved_object_ids,
            [left, right, wire, label]
                .into_iter()
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn child_port_rails_use_resolved_authored_symbol_bounds() {
        let mut schematic = SchematicState::default();
        let selected = schematic.add_component(ComponentType::Resistor, Point::origin());
        schematic.add_component(ComponentType::Resistor, Point::new(80, 0));
        schematic.add_wire(vec![Point::new(20, 0), Point::new(60, 0)]);
        schematic
            .session
            .editor
            .selection
            .select_component(selected);
        let mut resolved_bounds = bounds(&schematic);
        resolved_bounds.insert(selected, (-200, -100, 200, 100));

        let plan = schematic
            .plan_hierarchy_extraction(
                &terminals(&schematic),
                &connectivity(&schematic),
                &resolved_bounds,
            )
            .expect("plan");
        let candidate = schematic
            .materialize_hierarchy_extraction(&plan, "work", "wide_symbol", "schematic")
            .expect("candidate");
        let child_port_terminal = candidate
            .child
            .design
            .document()
            .components
            .iter()
            .find(|component| component.kind == ComponentType::Port)
            .expect("child port")
            .terminal_positions()[0]
            .1;
        assert!(
            child_port_terminal.x >= 240,
            "port rail must clear the resolved 200-unit authored symbol extent"
        );
    }

    #[test]
    fn hierarchy_candidates_preserve_editor_state_without_mutating_source() {
        use crate::state::{Rotation, Tool, Wire};
        use rspice_design::schematic::history::SchematicSnapshot;
        use rspice_schematic_editor::session::placement::PendingPartModel;

        let mut schematic = SchematicState::default();
        assert!(schematic.with_undo("seed", |state| {
            state.add_component(ComponentType::Resistor, Point::origin());
        }));
        let id = schematic.design.document().components[0].id;
        schematic.session.editor.selection.select_only_component(id);
        schematic.design.document_mut_for_test().grid_size = 25;
        schematic.session.editor.snap_engine.grid_size = 77;
        schematic.session.editor.zoom = 2.5;
        schematic.session.editor.pan = (11.0, -7.0);
        schematic.session.editor.needs_fit = true;
        schematic.session.current_file = Some("hierarchy-source.rsch".into());
        schematic.session.editor.preview_rotation = Rotation::R90;
        schematic.session.editor.preview_mirror_h = true;
        schematic.session.editor.pending_part_model = Some(PendingPartModel {
            tool: Tool::Place(ComponentType::Resistor),
            model: "pending".to_owned(),
            variant: None,
        });
        let mut invalid_wire = Wire::segment(99, Point::origin(), Point::new(10, 0));
        invalid_wire.points.clear();
        schematic.session.editor.clipboard.wires.push(invalid_wire);
        schematic.session.is_dirty = false;
        let before = SchematicSnapshot::capture(&schematic.design.document());
        let selection = schematic.session.editor.selection.clone();
        let cursor = schematic.identity_cursor();
        let topology = schematic.topology_version();
        let plan = schematic
            .plan_hierarchy_extraction(
                &terminals(&schematic),
                &connectivity(&schematic),
                &bounds(&schematic),
            )
            .unwrap();
        let candidate = schematic
            .materialize_hierarchy_extraction(&plan, "work", "child", "schematic")
            .unwrap();
        assert!(before.is_equal_document(&schematic.design.document()));
        assert_eq!(schematic.session.editor.selection, selection);
        assert_eq!(schematic.identity_cursor(), cursor);
        assert_eq!(schematic.session.editor.clipboard.wires.len(), 1);
        assert!(!schematic.session.is_dirty);
        let parent = candidate.parent;
        assert_eq!(parent.session.editor.zoom, schematic.session.editor.zoom);
        assert_eq!(parent.session.editor.pan, schematic.session.editor.pan);
        assert_eq!(parent.session.current_file, schematic.session.current_file);
        assert!(parent.session.editor.needs_fit);
        assert_eq!(
            parent.session.editor.pending_part_model,
            schematic.session.editor.pending_part_model
        );
        assert_eq!(parent.session.editor.snap_engine.grid_size, 77);
        assert_eq!(parent.undo_description(), Some("seed"));
        assert_eq!(parent.content_version(), schematic.content_version());
        assert_eq!(parent.topology_version(), topology + 2);
        assert!(parent.session.editor.clipboard.wires.is_empty());
        assert_eq!(
            parent.session.editor.selection.components,
            std::collections::HashSet::from([candidate.instance_id])
        );
        assert!(parent.session.is_dirty);
        let instance = &parent.design.document().components[0];
        assert_eq!(instance.rotation, Rotation::R0);
        assert!(instance.mirror_h);
        assert!(candidate.child.session.editor.selection.is_empty());
        assert!(candidate.child.session.editor.pending_part_model.is_none());
        assert!(!candidate.child.can_undo());
        assert_eq!(candidate.child.design.document().grid_size, 25);
        assert_eq!(
            candidate.child.session.editor.snap_engine.grid_size,
            SchematicState::default()
                .session
                .editor
                .snap_engine
                .grid_size
        );
        assert!(candidate.child.session.is_dirty);
    }
}
