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
use rspice_design::schematic::functional_catalog::{
    apply_functional_preset, functional_library_binding, functional_part,
};
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
        let mut component =
            Component::new(self.next_id, ComponentType::CellInstance, self.origin())
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

    fn operating_points(&self, nodes: &[&str]) -> Vec<f64> {
        let op = Engine::default()
            .run_dc_op(&self.deck())
            .expect("operating point solves");
        nodes
            .iter()
            .map(|node| {
                op.try_voltage_named(node)
                    .unwrap_or_else(|| panic!("no node {node}"))
            })
            .collect()
    }

    fn operating_point(&self, node: &str) -> f64 {
        self.operating_points(&[node])[0]
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

#[test]
fn a_placed_open_collector_comparator_releases_its_output_to_the_pull_up() {
    // The selector's label reaches the subcircuit as its value: an open
    // collector lets a 12 V pull-up take the output above the 5 V supply.
    let mut bench = Bench::default();
    supplies(&mut bench, "5", "0");
    bench.primitive(ComponentType::VoltageSource, "VPU", "12", &["pu", "0"]);
    bench.primitive(ComponentType::VoltageSource, "VIN", "1.1", &["in", "0"]);
    bench.primitive(ComponentType::VoltageSource, "VR", "1", &["ref", "0"]);
    bench.part(
        "rspice.functional.comparator",
        "X1",
        "OC=\"Open collector\"",
        &["in", "ref", "vp", "vn", "out"],
    );
    bench.primitive(ComponentType::Resistor, "RPU", "10k", &["pu", "out"]);
    let out = bench.operating_point("out");
    assert!((out - 12.0).abs() < 1e-6, "{out}");
}

#[test]
fn a_placed_ota_drives_its_bias_current_times_tanh_into_a_load() {
    let mut bench = Bench::default();
    supplies(&mut bench, "5", "-5");
    bench.primitive(ComponentType::CurrentSource, "IB", "100u", &["vp", "iabc"]);
    bench.primitive(ComponentType::VoltageSource, "VIN", "10m", &["in", "0"]);
    bench.part(
        "rspice.functional.ota",
        "X1",
        "",
        &["in", "0", "iabc", "vp", "vn", "out"],
    );
    bench.primitive(ComponentType::Resistor, "RL", "10k", &["out", "0"]);
    // The load in parallel with the 10 MOhm ROUT.
    let load = 1.0 / (1.0 / 10e3 + 1.0 / 10e6);
    let expected = 100e-6 * (10e-3_f64 / 51.7e-3).tanh() * load;
    let out = bench.operating_point("out");
    assert!(
        (out - expected).abs() < 1e-6 * expected,
        "{out} vs {expected}"
    );
}

#[test]
fn placed_logic_parts_settle_at_their_truth_table_levels() {
    // A high and B low into every gate; the Schmitt parts read B and A.
    let mut bench = Bench::default();
    bench.primitive(ComponentType::VoltageSource, "VA", "5", &["a", "0"]);
    bench.primitive(ComponentType::VoltageSource, "VB", "0", &["b", "0"]);
    let parts = [
        ("rspice.functional.and2", &["a", "b", "and"][..], 0.0),
        ("rspice.functional.nand2", &["a", "b", "nand"][..], 5.0),
        ("rspice.functional.or2", &["a", "b", "or"][..], 5.0),
        ("rspice.functional.nor2", &["a", "b", "nor"][..], 0.0),
        ("rspice.functional.xor2", &["a", "b", "xor"][..], 5.0),
        ("rspice.functional.inverter", &["a", "inv"][..], 0.0),
        ("rspice.functional.buffer", &["a", "buf"][..], 5.0),
        (
            "rspice.functional.schmitt_inverter",
            &["b", "sinv"][..],
            5.0,
        ),
        ("rspice.functional.schmitt_buffer", &["a", "sbuf"][..], 5.0),
    ];
    for (index, (part, nets, _)) in parts.iter().enumerate() {
        bench.part(part, &format!("X{index}"), "", nets);
    }
    let outputs = parts
        .iter()
        .map(|(_, nets, _)| nets[nets.len() - 1])
        .collect::<Vec<_>>();
    for ((node, voltage), (part, _, expected)) in outputs
        .iter()
        .zip(bench.operating_points(&outputs))
        .zip(parts)
    {
        assert!(
            (voltage - expected).abs() < 1e-9,
            "{part} {node}: {voltage}"
        );
    }
}

#[test]
fn a_placed_cmos_555_astable_oscillates_at_its_textbook_frequency() {
    // RA=1k, RB=10k, C=10n with the CMOS preset chosen by its label.
    let timer = functional_part("rspice.functional.timer555").expect("555");
    let cmos = apply_functional_preset(timer, "", "CMOS").expect("the CMOS preset");
    let mut bench = Bench::default();
    bench.primitive(ComponentType::VoltageSource, "VCC", "5", &["vcc", "0"]);
    bench.primitive(ComponentType::Resistor, "RA", "1k", &["vcc", "dis"]);
    bench.primitive(ComponentType::Resistor, "RB", "10k", &["dis", "thr"]);
    bench.primitive(ComponentType::Capacitor, "C1", "10n", &["thr", "0"]);
    bench.part(
        "rspice.functional.timer555",
        "X1",
        &cmos,
        &["thr", "thr", "vcc", "out", "dis", "ctl", "vcc", "0"],
    );
    bench.primitive(ComponentType::Capacitor, "CC", "10n", &["ctl", "0"]);
    let tran = Engine::default()
        .run_tran(&bench.deck(), 1e-3, 10e-9)
        .expect("transient solves");
    let out = tran.try_voltage_waveform_named("out").expect("out");
    let rises = out
        .windows(2)
        .enumerate()
        .filter(|(_, pair)| pair[0] < 2.5 && pair[1] >= 2.5)
        .map(|(index, _)| tran.time[index])
        .collect::<Vec<_>>();
    // The CMOS output swings rail to rail; five settled periods.
    assert!(rises.len() >= 7, "{} rising edges", rises.len());
    let period = (rises[6] - rises[1]) / 5.0;
    let analytic = 1.44 / ((1e3 + 2.0 * 10e3) * 10e-9);
    assert!((1.0 / period / analytic - 1.0).abs() < 0.03, "{period}");
    let high = out.iter().copied().fold(f64::MIN, f64::max);
    assert!(high > 4.9, "CMOS output high {high}");
}

#[test]
fn placed_sequential_parts_count_divide_and_hold() {
    // A 1 MHz clock into a divide-by-4 counter and into a phase detector
    // against itself delayed, and a sample-and-hold of a DC level.
    let mut bench = Bench::default();
    bench.primitive(
        ComponentType::VoltageSourcePulse,
        "VC",
        "PULSE(0 5 1u 1n 1n 0.5u 1u)",
        &["clk", "0"],
    );
    bench.primitive(
        ComponentType::VoltageSourcePulse,
        "VD",
        "PULSE(0 5 1.2u 1n 1n 0.5u 1u)",
        &["late", "0"],
    );
    bench.primitive(ComponentType::VoltageSource, "VR", "0", &["rst", "0"]);
    bench.primitive(ComponentType::VoltageSource, "VI", "1.25", &["level", "0"]);
    bench.part(
        "rspice.functional.counter",
        "X1",
        "N=4",
        &["clk", "rst", "div"],
    );
    bench.part(
        "rspice.functional.phase_detector",
        "X2",
        "",
        &["clk", "late", "pump"],
    );
    bench.primitive(ComponentType::Capacitor, "CP", "1n", &["pump", "0"]);
    bench.part(
        "rspice.functional.sample_hold",
        "X3",
        "",
        &["level", "clk", "held"],
    );
    let tran = Engine::default()
        .run_tran(&bench.deck(), 4.9e-6, 1e-9)
        .expect("transient solves");
    let at = |node: &str, time: f64| {
        let wave = tran.try_voltage_waveform_named(node).expect(node);
        let index = tran.time.iter().position(|&t| t >= time).expect("in run");
        wave[index]
    };
    // Divide by 4, high for two counts: low from the second edge to the fourth.
    assert!(at("div", 1.5e-6) > 4.9 && at("div", 3.5e-6) < 0.1);
    // Four 0.2 us leads of 100 uA into 1 nF.
    assert!(
        (at("pump", 4.9e-6) - 0.08).abs() < 0.5e-3,
        "{}",
        at("pump", 4.9e-6)
    );
    assert!(
        (at("held", 4.9e-6) - 1.25).abs() < 1e-3,
        "{}",
        at("held", 4.9e-6)
    );
}
