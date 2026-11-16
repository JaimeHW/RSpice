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
    assert_eq!(
        wire_ids,
        vec![10, 11],
        "selecting ground lights every ground wire"
    );
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

/// The kind of the net `component`'s pin `index` is on.
fn kind_at(nets: &[DesignNet], component: &Component, pin: &str) -> NetDomain {
    nets.iter()
        .find(|net| {
            net.terminals.iter().any(|terminal| {
                terminal.component_id == component.id && terminal.pin.eq_ignore_ascii_case(pin)
            })
        })
        .unwrap_or_else(|| panic!("no net on {}.{pin}", component.name))
        .domain
}

/// An XSPICE gate placed from the shelf by its own kind carries no frozen
/// contract, so its pins are typed from the engine's code-model table: an
/// inverter's ports are digital, and two of them meet on a logic net. A
/// resistor on an output makes that net one the run converts.
#[test]
fn a_code_model_gate_placed_by_its_kind_reads_its_ports_from_the_engine() {
    let a1 = placed(1, ComponentType::XspiceInverter, "A1", Point::new(0, 0));
    let a2 = placed(2, ComponentType::XspiceInverter, "A2", Point::new(400, 0));
    let r1 = placed(3, ComponentType::Resistor, "R1", Point::new(800, 0));
    let wires = vec![
        elbow(10, terminal(&a1, 1), terminal(&a2, 0)),
        elbow(11, terminal(&a2, 1), terminal(&r1, 0)),
    ];
    let schematic = SchematicDocument {
        components: vec![a1.clone(), a2.clone(), r1],
        wires,
        ..SchematicDocument::default()
    };
    let nets = design_nets(&schematic);
    assert_eq!(kind_at(&nets, &a1, "out"), NetDomain::Logic);
    assert_eq!(
        kind_at(&nets, &a2, "out"),
        NetDomain::Mixed(EventOnlyNetKind::Digital)
    );
}

/// A pin of a placed schematic takes the kind of its net inside the master,
/// as the flattened deck does: the master's port `en` reaches only an
/// inverter's input, so the parent's net between another inverter and that
/// pin is a logic net, not an electrical one.
#[test]
fn a_placed_schematics_pin_reads_the_kind_of_its_net_inside() {
    let port = placed(1, ComponentType::Port, "P1", Point::new(0, 0)).with_name_value("P1", "en");
    let inner = placed(2, ComponentType::XspiceInverter, "A1", Point::new(400, 0));
    let r_out = placed(3, ComponentType::Resistor, "R1", Point::new(800, 0));
    let master = SchematicDocument {
        wires: vec![
            elbow(10, terminal(&port, 0), terminal(&inner, 0)),
            elbow(11, terminal(&inner, 1), terminal(&r_out, 0)),
        ],
        components: vec![port, inner, r_out],
        ..SchematicDocument::default()
    };
    let mut binding = LibraryCellInstance::new("work", "blk", "schematic");
    binding.bind_interface(&[rspice_design_model::port::PortSpec {
        name: "en".to_owned(),
        direction: PortDirection::In,
    }]);
    let block =
        placed(5, ComponentType::CellInstance, "X1", Point::new(400, 0)).with_library_cell(binding);
    let driver = placed(6, ComponentType::XspiceInverter, "A2", Point::new(0, 0));
    let top = SchematicDocument {
        wires: vec![elbow(
            20,
            terminal(&driver, 1),
            terminal_positions_with_hierarchy(&block, None)[0].1,
        )],
        components: vec![driver.clone(), block],
        ..SchematicDocument::default()
    };
    let buffers = HashMap::from([
        ("work/blk/schematic".to_owned(), master),
        ("work/top/schematic".to_owned(), top.clone()),
    ]);
    let nets = design_nets_with_hierarchy(&top, &HierarchySource::from_buffers(&buffers));
    assert_eq!(kind_at(&nets, &driver, "out"), NetDomain::Logic);
}
