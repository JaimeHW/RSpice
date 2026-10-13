//! The summary has the deck's nets: one per node, however the drawing reaches
//! it.

use super::*;
use crate::schematic::component_type::ComponentType;
use crate::schematic::net_label::NetLabel;
use crate::schematic::wire::Wire;
use rspice_design_model::Point;

fn placed(id: u64, kind: ComponentType, name: &str, pos: Point) -> Component {
    let mut component = Component::new(id, kind, pos);
    component.name = name.to_owned();
    component
}

fn terminal(component: &Component, index: usize) -> Point {
    terminal_positions_with_hierarchy(component, None)[index].1
}

/// A wire from `from` down to `to`'s height, then across to it.
fn elbow(id: u64, from: Point, to: Point) -> Wire {
    Wire::new(id, vec![from, Point::new(from.x, to.y), to])
}

fn ground_nets(nets: &[DesignNet]) -> Vec<&DesignNet> {
    nets.iter().filter(|net| net.name == "0").collect()
}

#[test]
fn separate_ground_symbols_are_one_ground_net() {
    let r1 = placed(1, ComponentType::Resistor, "R1", Point::new(0, 0));
    let r2 = placed(2, ComponentType::Resistor, "R2", Point::new(400, 0));
    let g1 = placed(3, ComponentType::Ground, "G1", Point::new(100, 300));
    let g2 = placed(4, ComponentType::Ground, "G2", Point::new(500, 300));
    let wires = vec![
        elbow(10, terminal(&r1, 1), terminal(&g1, 0)),
        elbow(11, terminal(&r2, 1), terminal(&g2, 0)),
    ];
    let schematic = SchematicDocument {
        components: vec![r1, r2, g1, g2],
        wires,
        ..SchematicDocument::default()
    };

    let nets = design_nets(&schematic);
    let grounds = ground_nets(&nets);
    assert_eq!(
        grounds.len(),
        1,
        "two ground symbols are the deck's one node 0, not two nets: {:?}",
        nets.iter().map(|net| &net.name).collect::<Vec<_>>()
    );
    let ground = grounds[0];
    assert_eq!(ground.class, NetClass::Ground);
    let mut wire_ids = ground.wire_ids.clone();
    wire_ids.sort_unstable();
    assert_eq!(wire_ids, vec![10, 11], "selecting ground lights every ground wire");
    let mut members: Vec<u64> = ground
        .terminals
        .iter()
        .map(|terminal| terminal.component_id)
        .collect();
    members.sort_unstable();
    members.dedup();
    assert_eq!(members, vec![1, 2, 3, 4]);
}

#[test]
fn a_net_labelled_zero_is_the_ground_net() {
    let r1 = placed(1, ComponentType::Resistor, "R1", Point::new(0, 0));
    let r2 = placed(2, ComponentType::Resistor, "R2", Point::new(400, 0));
    let g1 = placed(3, ComponentType::Ground, "G1", Point::new(500, 300));
    let label_at = Point::new(100, 300);
    let wires = vec![
        elbow(10, terminal(&r1, 1), label_at),
        elbow(11, terminal(&r2, 1), terminal(&g1, 0)),
    ];
    let schematic = SchematicDocument {
        components: vec![r1, r2, g1],
        wires,
        net_labels: vec![NetLabel::new(20, label_at, "0")],
        ..SchematicDocument::default()
    };

    let nets = design_nets(&schematic);
    let grounds = ground_nets(&nets);
    assert_eq!(
        grounds.len(),
        1,
        "a `0` label names the node a ground symbol does: {:?}",
        nets.iter().map(|net| &net.name).collect::<Vec<_>>()
    );
    let mut wire_ids = grounds[0].wire_ids.clone();
    wire_ids.sort_unstable();
    assert_eq!(wire_ids, vec![10, 11]);
}
