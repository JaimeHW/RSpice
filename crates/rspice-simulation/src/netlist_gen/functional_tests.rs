//! Functional parts placed on a schematic from their catalog binding,
//! netlisted through the workspace hierarchy, simulated and probed: the
//! whole path a reader takes from the Component shelf to a waveform.

use std::collections::HashMap;

use rspice_core::engine::Engine;
use rspice_core::netlist::Netlist;
use rspice_design::hierarchy::HierarchySource;
use rspice_design::library::LibraryCatalog;
use rspice_design::schematic::component::Component;
use rspice_design::schematic::component_type::ComponentType;
use rspice_design::schematic::functional_catalog::{functional_library_binding, functional_part};
use rspice_design::schematic::net_label::NetLabel;
use rspice_design::schematic::owned::Schematic;
use rspice_design_model::Point;

use super::{FixtureSourceFiles, NetlistSourceData, generate_netlist_hierarchical};

/// A schematic built part by part, every terminal tied to its net by a label.
#[derive(Default)]
struct Bench {
    schematic: Schematic,
    next_id: u64,
}

impl Bench {
    fn place(&mut self, component: Component, nets: &[&str]) {
        let terminals = component.terminal_positions();
        assert_eq!(terminals.len(), nets.len(), "{}", component.name);
        let document = self.schematic.document_mut_for_test();
        for ((_, point), net) in terminals.into_iter().zip(nets) {
            self.next_id += 1;
            document
                .net_labels
                .push(NetLabel::new(1000 + self.next_id, point, *net));
        }
        document.components.push(component);
    }

    fn origin(&mut self) -> Point {
        self.next_id += 1;
        Point::new(400 * self.next_id as i32, 0)
    }

    /// Place a functional part from its catalog binding.
    fn part(&mut self, stable_id: &str, name: &str, params: &str, nets: &[&str]) {
        let descriptor = functional_part(stable_id).expect("catalog part");
        let mut component = Component::new(self.next_id, ComponentType::CellInstance, self.origin())
            .with_library_cell(functional_library_binding(descriptor));
        component.name = name.to_owned();
        component.params = params.to_owned();
        self.place(component, nets);
    }

    fn primitive(&mut self, kind: ComponentType, name: &str, value: &str, nets: &[&str]) {
        let component =
            Component::new(self.next_id, kind, self.origin()).with_name_value(name, value);
        self.place(component, nets);
    }

    fn deck(&self) -> Netlist {
        let libraries = LibraryCatalog::default();
        let mut buffers = HashMap::new();
        buffers.insert("user/top/schematic".to_owned(), self.schematic.clone());
        let hierarchy = HierarchySource::from_workspace(&libraries, &buffers);
        let generated = generate_netlist_hierarchical(
            &self.schematic,
            &[],
            &hierarchy,
            &NetlistSourceData::new(&FixtureSourceFiles),
        );
        assert!(generated.errors.is_empty(), "{:?}", generated.errors);
        Netlist::parse(&generated.netlist)
            .unwrap_or_else(|error| panic!("{error}\n{}", generated.netlist))
    }

    fn operating_point(&self, node: &str) -> f64 {
        Engine::default()
            .run_dc_op(&self.deck())
            .expect("operating point solves")
            .try_voltage_named(node)
            .unwrap_or_else(|| panic!("no node {node}"))
    }
}

fn supplies(bench: &mut Bench, positive: &str, negative: &str) {
    bench.primitive(ComponentType::VoltageSource, "VP", positive, &["vp", "0"]);
    bench.primitive(ComponentType::VoltageSource, "VN", negative, &["vn", "0"]);
}

#[test]
fn a_placed_op_amp_amplifies_by_its_feedback_ratio() {
    let mut bench = Bench::default();
    supplies(&mut bench, "15", "-15");
    bench.primitive(ComponentType::VoltageSource, "VIN", "0.5", &["in", "0"]);
    bench.part(
        "rspice.functional.opamp",
        "X1",
        "AVOL=1Meg",
        &["in", "fb", "vp", "vn", "out"],
    );
    bench.primitive(ComponentType::Resistor, "RF", "9k", &["out", "fb"]);
    bench.primitive(ComponentType::Resistor, "RG", "1k", &["fb", "0"]);
    let gain = 10.0 / (1.0 + 10.0 / 1e6);
    let out = bench.operating_point("out");
    assert!((out - 0.5 * gain).abs() < 1e-6, "{out}");
}
