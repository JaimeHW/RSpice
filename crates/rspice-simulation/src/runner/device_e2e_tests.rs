//! End-to-end device qualification: every palette device's generated
//! netlist must parse, build, and SOLVE in rspice-core — not merely
//! string-match. Each bench places real components, wires their
//! terminals, generates the deck through the production
//! `NetlistGenerator`, and runs the engine through the same spec path
//! the simulate action uses.

use super::spec::run_spec_request;
use crate::engine_bridge::EngineBridge;
use crate::execution_artifact::ResolvedExecutionDependencies;
use crate::execution_options::SpecExecutionOptions;
use crate::netlist_gen::{FixtureSourceFiles, NetlistGenerator, NetlistSourceData};
use rspice_design::schematic::component::Component;
use rspice_design::schematic::component_type::ComponentType;
use rspice_design::schematic::net_label::NetLabel;
use rspice_design::schematic::owned::Schematic;
use rspice_design::schematic::wire::Wire;
use rspice_design_model::Point;
use rspice_simulation_contract::analysis_spec::AnalysisSpec;

struct Bench {
    state: Schematic,
    next_id: u64,
}

impl Bench {
    fn new() -> Self {
        Self {
            state: Schematic::default(),
            next_id: 1,
        }
    }

    fn place(
        &mut self,
        kind: ComponentType,
        x: i32,
        y: i32,
        name: &str,
        value: &str,
        params: &str,
    ) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        let mut component = Component::new(id, kind, Point::new(x, y)).with_name_value(name, value);
        component.params = params.to_owned();
        self.state
            .document_mut_for_test()
            .components
            .push(component);
        self.state.document().components.len() - 1
    }

    fn terminal(&self, index: usize, terminal: usize) -> Point {
        self.state.document().components[index].terminal_positions()[terminal].1
    }

    fn connect(&mut self, from: (usize, usize), to: (usize, usize)) {
        let a = self.terminal(from.0, from.1);
        let b = self.terminal(to.0, to.1);
        let id = self.next_id;
        self.next_id += 1;
        self.state
            .document_mut_for_test()
            .wires
            .push(Wire::segment(id, a, b));
    }

    fn ground(&mut self, at: (usize, usize)) {
        let point = self.terminal(at.0, at.1);
        let id = self.next_id;
        self.next_id += 1;
        // The ground terminal sits 10 units above its anchor position.
        self.state
            .document_mut_for_test()
            .components
            .push(Component::new(
                id,
                ComponentType::Ground,
                Point::new(point.x, point.y + 10),
            ));
    }

    /// Name the net a terminal stands on, with a label on the terminal.
    fn label(&mut self, at: (usize, usize), name: &str) {
        let point = self.terminal(at.0, at.1);
        let id = self.next_id;
        self.next_id += 1;
        self.state
            .document_mut_for_test()
            .net_labels
            .push(NetLabel::new(id, point, name));
    }

    fn netlist(&self) -> String {
        self.netlist_with_parameters(&[])
    }

    /// The deck with `parameter_lines` where a run writes its design
    /// variables.
    fn netlist_with_parameters(&self, parameter_lines: &[String]) -> String {
        let result =
            NetlistGenerator::new(&self.state, NetlistSourceData::new(&FixtureSourceFiles))
                .finish(&[], parameter_lines);
        assert!(
            result.errors.is_empty(),
            "netlist generation errors: {:?}\n{}",
            result.errors,
            result.netlist
        );
        result.netlist
    }
}

fn solve_op(netlist: &str) {
    let result = run_spec_request(
        &EngineBridge::new(),
        AnalysisSpec::LegacyDcOp,
        SpecExecutionOptions::default(),
        netlist,
        None,
        &ResolvedExecutionDependencies::default(),
        &rspice_core::abort_signal::NoAbort,
    );
    assert!(
        result.is_ok(),
        "OP solve failed: {result:?}\ndeck:\n{netlist}"
    );
}

fn solve_tran(netlist: &str, stop_time: f64, step_time: f64) {
    solve_tran_with_uic(netlist, stop_time, step_time, false)
}

fn solve_tran_with_uic(netlist: &str, stop_time: f64, step_time: f64, uic: bool) {
    let result = run_spec_request(
        &EngineBridge::new(),
        AnalysisSpec::Transient {
            stop_time,
            step_time,
            start_time: 0.0,
            max_timestep: None,
            uic,
        },
        SpecExecutionOptions::default(),
        netlist,
        None,
        &ResolvedExecutionDependencies::default(),
        &rspice_core::abort_signal::NoAbort,
    );
    assert!(
        result.is_ok(),
        "transient solve failed: {result:?}\ndeck:\n{netlist}"
    );
}

#[test]
fn mesfet_bench_solves_op() {
    let mut bench = Bench::new();
    let vd = bench.place(ComponentType::VoltageSource, 0, 0, "VD", "2", "");
    let z = bench.place(ComponentType::Nmesfet, 200, 0, "Z1", "", "");
    bench.connect((vd, 0), (z, 0)); // VD+ -> drain
    bench.ground((vd, 1));
    bench.ground((z, 1)); // gate
    bench.ground((z, 2)); // source
    solve_op(&bench.netlist());
}

/// A foundation card bound deliberately must solve as readily as one fallen
/// back to. Only the diode, bipolar, MOSFET and JFET bind sites resolve a
/// card-less foundation name from the embedded library, so a MESFET or VDMOS
/// bound by name in the Models workspace reaches the engine as an unknown
/// model unless the deck carries the card text with it.
#[test]
fn explicitly_bound_foundation_cards_solve_for_families_with_no_bind_fallback() {
    for (kind, model, extra_terminals) in [
        (ComponentType::Nmesfet, "RSPICE_NMESFET", 0),
        (ComponentType::NVdmos, "RSPICE_NVDMOS", 1),
    ] {
        let mut bench = Bench::new();
        let vd = bench.place(ComponentType::VoltageSource, 0, 0, "VD", "5", "");
        let vg = bench.place(ComponentType::VoltageSource, 0, 200, "VG", "5", "");
        let device = bench.place(kind, 200, 0, "Z1", "", &format!("model={model}"));
        bench.connect((vd, 0), (device, 0));
        bench.connect((vg, 0), (device, 1));
        bench.ground((device, 2));
        for terminal in 0..extra_terminals {
            bench.ground((device, 3 + terminal));
        }
        bench.ground((vd, 1));
        bench.ground((vg, 1));
        let netlist = bench.netlist();
        assert!(
            netlist.contains(model),
            "{model} should reach the deck:\n{netlist}"
        );
        solve_op(&netlist);
    }
}

#[test]
fn vdmos_bench_solves_op() {
    let mut bench = Bench::new();
    let vd = bench.place(ComponentType::VoltageSource, 0, 0, "VD", "5", "");
    let vg = bench.place(ComponentType::VoltageSource, 0, 200, "VG", "5", "");
    let m = bench.place(ComponentType::NVdmos, 200, 0, "M1", "", "");
    bench.connect((vd, 0), (m, 0)); // drain
    bench.connect((vg, 0), (m, 1)); // gate
    bench.ground((m, 2)); // source
    bench.ground((m, 3)); // bulk
    bench.ground((vd, 1));
    bench.ground((vg, 1));
    solve_op(&bench.netlist());
}

#[test]
fn soi_mosfet_bench_solves_op() {
    let mut bench = Bench::new();
    let vd = bench.place(ComponentType::VoltageSource, 0, 0, "VD", "1.5", "");
    let vg = bench.place(ComponentType::VoltageSource, 0, 200, "VG", "1", "");
    let m = bench.place(ComponentType::NmosSoi, 200, 0, "M1", "", "");
    bench.connect((vd, 0), (m, 0)); // drain
    bench.connect((vg, 0), (m, 1)); // gate
    bench.ground((m, 2)); // source
    bench.ground((m, 3)); // back gate E
    bench.ground((m, 4)); // body contact P
    bench.ground((vd, 1));
    bench.ground((vg, 1));
    solve_op(&bench.netlist());
}

#[test]
fn substrate_bjt_bench_solves_op() {
    let mut bench = Bench::new();
    let vc = bench.place(ComponentType::VoltageSource, 0, 0, "VC", "3", "");
    let vb = bench.place(ComponentType::VoltageSource, 0, 200, "VB", "0.7", "");
    let q = bench.place(ComponentType::NpnBjt4, 200, 0, "Q1", "", "");
    bench.connect((vc, 0), (q, 0)); // collector
    bench.connect((vb, 0), (q, 1)); // base
    bench.ground((q, 2)); // emitter
    bench.ground((q, 3)); // substrate
    bench.ground((vc, 1));
    bench.ground((vb, 1));
    solve_op(&bench.netlist());
}

#[test]
fn thermal_vbic_bjt_bench_solves_op() {
    let mut bench = Bench::new();
    let vc = bench.place(ComponentType::VoltageSource, 0, 0, "VC", "3", "");
    let vb = bench.place(ComponentType::VoltageSource, 0, 200, "VB", "0.7", "");
    let q = bench.place(ComponentType::NpnBjt5, 200, 0, "Q1", "", "");
    bench.connect((vc, 0), (q, 0)); // collector
    bench.connect((vb, 0), (q, 1)); // base
    bench.ground((q, 2)); // emitter
    bench.ground((q, 3)); // substrate
    bench.ground((q, 4)); // thermal node at ambient
    bench.ground((vc, 1));
    bench.ground((vb, 1));
    solve_op(&bench.netlist());
}

#[test]
fn diode_default_model_bench_solves_op() {
    let mut bench = Bench::new();
    let v = bench.place(ComponentType::VoltageSource, 0, 0, "V1", "0.7", "");
    let d = bench.place(ComponentType::Diode, 200, 0, "D1", "", "");
    bench.connect((v, 0), (d, 0));
    bench.ground((d, 1));
    bench.ground((v, 1));
    solve_op(&bench.netlist());
}

#[test]
fn saturable_inductor_bench_solves_op() {
    let mut bench = Bench::new();
    let v = bench.place(ComponentType::VoltageSource, 0, 0, "V1", "1", "");
    let r = bench.place(ComponentType::Resistor, 200, 0, "R1", "10", "");
    let l = bench.place(ComponentType::SaturableInductor, 400, 0, "L1", "1m", "");
    bench.connect((v, 0), (r, 0));
    bench.connect((r, 1), (l, 0));
    bench.ground((l, 1));
    bench.ground((v, 1));
    solve_op(&bench.netlist());
}

#[test]
fn iswitch_bench_solves_op() {
    let mut bench = Bench::new();
    let vmain = bench.place(ComponentType::VoltageSource, 0, 0, "VMAIN", "5", "");
    let w = bench.place(ComponentType::ISwitch, 200, 0, "W1", "", "");
    let rload = bench.place(ComponentType::Resistor, 400, 0, "RLOAD", "100", "");
    // Control loop: drive a current through the sense-coil pins.
    let vctrl = bench.place(ComponentType::VoltageSource, 0, 300, "VCTRL", "1", "");
    let rctrl = bench.place(ComponentType::Resistor, 400, 300, "RCTRL", "100", "");
    bench.connect((vmain, 0), (w, 0));
    bench.connect((w, 1), (rload, 0));
    bench.ground((rload, 1));
    bench.ground((vmain, 1));
    bench.connect((vctrl, 0), (w, 2)); // coil c+
    bench.connect((w, 3), (rctrl, 0)); // coil c- -> return resistor
    bench.ground((rctrl, 1));
    bench.ground((vctrl, 1));
    solve_op(&bench.netlist());
}

#[test]
fn ccvs_and_cccs_bench_solves_op() {
    let mut bench = Bench::new();
    let v = bench.place(ComponentType::VoltageSource, 0, 0, "V1", "1", "");
    let rc = bench.place(ComponentType::Resistor, 200, 0, "RC", "1k", "");
    let h = bench.place(ComponentType::Ccvs, 400, 0, "H1", "100", "");
    let rl = bench.place(ComponentType::Resistor, 600, 0, "RL", "1k", "");
    // Control loop: V1 -> RC -> control pins (synthesized sense source).
    bench.connect((v, 0), (rc, 0));
    bench.connect((rc, 1), (h, 2)); // C+
    bench.ground((h, 3)); // C-
    bench.ground((v, 1));
    // Output loop.
    bench.connect((h, 0), (rl, 0)); // O+
    bench.ground((h, 1)); // O-
    bench.ground((rl, 1));

    let f = bench.place(ComponentType::Cccs, 400, 300, "F1", "2.5", "");
    let v2 = bench.place(ComponentType::VoltageSource, 0, 300, "V2", "1", "");
    let rc2 = bench.place(ComponentType::Resistor, 200, 300, "RC2", "1k", "");
    let rl2 = bench.place(ComponentType::Resistor, 600, 300, "RL2", "1k", "");
    bench.connect((v2, 0), (rc2, 0));
    bench.connect((rc2, 1), (f, 2));
    bench.ground((f, 3));
    bench.ground((v2, 1));
    bench.connect((f, 0), (rl2, 0));
    bench.ground((f, 1));
    bench.ground((rl2, 1));
    solve_op(&bench.netlist());
}

/// A current-controlled source with no controlling element of its own is
/// written beside a 0 V sense source, `VSENSE_H1` for `H1`. A source drawn
/// `SENSE_H1` is written under that name too, so the deck holds two cards of
/// one name and the engine refuses it. The design check reads the same names
/// and refuses the run first.
#[test]
fn a_source_written_as_a_sense_source_is_refused_before_the_run() {
    let mut bench = Bench::new();
    let v = bench.place(ComponentType::VoltageSource, 0, 0, "SENSE_H1", "1", "");
    let rc = bench.place(ComponentType::Resistor, 200, 0, "RC", "1k", "");
    let h = bench.place(ComponentType::Ccvs, 400, 0, "H1", "100", "");
    let rl = bench.place(ComponentType::Resistor, 600, 0, "RL", "1k", "");
    bench.connect((v, 0), (rc, 0));
    bench.connect((rc, 1), (h, 2));
    bench.ground((h, 3));
    bench.ground((v, 1));
    bench.connect((h, 0), (rl, 0));
    bench.ground((h, 1));
    bench.ground((rl, 1));

    let netlist = bench.netlist();
    let cards = netlist
        .lines()
        .filter(|line| line.split_whitespace().next() == Some("VSENSE_H1"))
        .count();
    assert_eq!(cards, 2, "{netlist}");
    assert!(
        matches!(
            rspice_core::netlist::Netlist::parse(&netlist),
            Err(rspice_core::netlist::ParseError::DuplicateName { .. })
        ),
        "{netlist}"
    );

    let views = std::collections::HashMap::<
        String,
        rspice_design::schematic::document::SchematicDocument,
    >::new();
    let hierarchy = rspice_design::hierarchy::HierarchySource::from_buffers(&views);
    let root = rspice_design_model::cell_view::CellViewRef::new("work", "top", "schematic");
    assert!(
        crate::preparation::check_generated_design(
            &bench.state,
            &root,
            &hierarchy,
            rspice_design::drc::DrcConfig::default(),
        )
        .is_err(),
        "the design check passes a deck the engine refuses:\n{netlist}"
    );
    let findings = rspice_design::drc::run_check_with_hierarchy(
        &bench.state,
        &hierarchy,
        rspice_design::drc::DrcConfig::default(),
    );
    let duplicates = findings
        .violations()
        .iter()
        .filter(|finding| {
            finding.violation_type == rspice_design::drc::DrcViolationType::DuplicateName
        })
        .map(|finding| finding.message.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        duplicates,
        ["SENSE_H1 and H1's sense source are both written VSENSE_H1"]
    );
}

/// A current-controlled source may name its controlling element by the name
/// drawn on it. The deck writes a source drawn `sense` as `Vsense`, so the
/// reference has to follow: in the plain card, and in the behavioural form a
/// limit lowers it to, whose expression reads the current as `i(...)`.
#[test]
fn a_controlling_source_named_as_drawn_solves() {
    let mut bench = Bench::new();
    let sense = bench.place(ComponentType::VoltageSource, 0, 0, "sense", "1", "");
    let rc = bench.place(ComponentType::Resistor, 200, 0, "RC", "1k", "");
    bench.connect((sense, 0), (rc, 0));
    bench.ground((rc, 1));
    bench.ground((sense, 1));

    let f = bench.place(ComponentType::Cccs, 400, 300, "F1", "2.5", "vref=sense");
    let rf = bench.place(ComponentType::Resistor, 600, 300, "RF", "1k", "");
    bench.connect((f, 0), (rf, 0));
    bench.ground((f, 1));
    bench.ground((rf, 1));

    let h = bench.place(
        ComponentType::Ccvs,
        400,
        600,
        "H1",
        "100",
        "vref=sense vmax=10",
    );
    let rh = bench.place(ComponentType::Resistor, 600, 600, "RH", "1k", "");
    bench.connect((h, 0), (rh, 0));
    bench.ground((h, 1));
    bench.ground((rh, 1));

    let netlist = bench.netlist();
    assert!(
        netlist
            .lines()
            .any(|line| line.starts_with("F1 ") && line.contains(" Vsense ")),
        "{netlist}"
    );
    assert!(netlist.contains("i(Vsense)"), "{netlist}");
    solve_op(&netlist);
}

/// The final value of each requested quantity after a short transient, found
/// under the very name it was requested by. A request the deck cannot answer
/// fails the run or comes back absent, and either fails the caller.
fn final_values(netlist: &str, requests: &[String]) -> Vec<f64> {
    let end = netlist
        .to_ascii_lowercase()
        .rfind("\n.end")
        .expect("the deck ends with .end");
    let deck = format!(
        "{}\n.save {}{}",
        &netlist[..end],
        requests.join(" "),
        &netlist[end..]
    );
    let result = run_spec_request(
        &EngineBridge::new(),
        AnalysisSpec::Transient {
            stop_time: 1e-6,
            step_time: 1e-7,
            start_time: 0.0,
            max_timestep: None,
            uic: false,
        },
        SpecExecutionOptions::default(),
        &deck,
        None,
        &ResolvedExecutionDependencies::default(),
        &rspice_core::abort_signal::NoAbort,
    );
    let Ok(crate::results::SimulationResult::Transient { waveforms, .. }) = result else {
        panic!("transient solve failed: {result:?}\ndeck:\n{deck}");
    };
    requests
        .iter()
        .map(|request| {
            let bare = request
                .strip_prefix("V(")
                .and_then(|inner| inner.strip_suffix(')'))
                .unwrap_or(request);
            waveforms
                .values()
                .find(|waveform| {
                    waveform.name.eq_ignore_ascii_case(request)
                        || waveform.name.eq_ignore_ascii_case(bare)
                })
                .and_then(|waveform| waveform.y_values.last().copied())
                .unwrap_or_else(|| {
                    panic!(
                        "{request} is absent from {:?}\ndeck:\n{deck}",
                        waveforms.keys().collect::<Vec<_>>()
                    )
                })
        })
        .collect()
}

/// A VCVS with an output limit is written as the behavioural source `B1`, and
/// the engine knows it by that name alone. A current probe on it asks for that
/// element; asked for as drawn, it names a source the deck does not have.
#[test]
fn a_current_probe_on_a_limited_vcvs_reads_the_behavioural_source() {
    let mut bench = Bench::new();
    let v = bench.place(ComponentType::VoltageSource, 0, 0, "V1", "1", "");
    let e = bench.place(ComponentType::Vcvs, 400, 0, "E1", "2", "vmax=10 vmin=-10");
    let load = bench.place(ComponentType::Resistor, 600, 0, "RL", "1k", "");
    bench.connect((v, 0), (e, 2));
    bench.ground((e, 3));
    bench.ground((v, 1));
    bench.connect((e, 0), (load, 0));
    bench.ground((e, 1));
    bench.ground((load, 1));

    let probed = bench.state.document().components[e].written_element_name();
    let current = final_values(&bench.netlist(), &[format!("I({probed})")])[0];

    // 2 V across 1 kΩ, delivered out of the source's positive terminal.
    assert!(
        (current.abs() - 2e-3).abs() < 1e-9,
        "I({probed}) = {current}"
    );
}

/// A cell instance drawn `amp`, holding a divider whose midpoint is the net
/// `n1` and whose lower leg is a resistor drawn `load`. The deck writes them
/// `Xamp` and `Rload`, and the engine scopes the inside of the cell by the
/// written instance name.
fn amp_divider() -> (
    rspice_design::schematic::owned::Schematic,
    std::collections::HashMap<String, Schematic>,
) {
    use rspice_design::schematic::component::LibraryCellInstance;
    use rspice_design::schematic::net_label::NetLabel;

    let mut amp = Bench::new();
    let port = amp.place(ComponentType::Port, 0, 0, "", "IN", "");
    let upper = amp.place(ComponentType::Resistor, 200, 0, "R1", "1k", "");
    let lower = amp.place(ComponentType::Resistor, 400, 0, "load", "1k", "");
    amp.connect((port, 0), (upper, 0));
    amp.connect((upper, 1), (lower, 0));
    amp.ground((lower, 1));
    let midpoint = amp.terminal(upper, 1);
    amp.state
        .document_mut_for_test()
        .net_labels
        .push(NetLabel::new(90, midpoint, "n1"));

    let mut top = Bench::new();
    let source = top.place(ComponentType::VoltageSource, 0, 0, "V1", "2", "");
    let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
    binding.terminal_order = vec!["IN".to_owned()];
    let instance = top.state.add_library_cell_component(
        crate::netlist_gen::test_placement(Point::new(400, 0)),
        binding,
    );
    let instance = top
        .state
        .document()
        .components
        .iter()
        .position(|component| component.id == instance)
        .expect("placed instance");
    top.state.document_mut_for_test().components[instance].name = "amp".to_owned();
    top.next_id = 100;
    top.connect((source, 0), (instance, 0));
    top.ground((source, 1));

    (
        top.state,
        std::collections::HashMap::from([("work/amp/schematic".to_owned(), amp.state)]),
    )
}

/// End to end through the real netlist and engine: the inside of a cell drawn
/// without its `X` is read through the app's own addressing of the drawn path.
#[test]
fn a_cell_drawn_without_its_prefix_is_read_through_its_drawn_path() {
    use rspice_app_types::hierarchy_path::{InstancePath, OccurrenceProbeSpelling, ProbeTarget};

    let (top, buffers) = amp_divider();
    let hierarchy = rspice_design::hierarchy::HierarchySource::from_buffers(&buffers);
    let generated = crate::netlist_gen::generate_netlist_hierarchical(
        &top,
        &[],
        &hierarchy,
        &crate::netlist_gen::NetlistSourceData::new(&crate::netlist_gen::FixtureSourceFiles),
    );
    assert!(
        generated.errors.is_empty(),
        "{:?}\n{}",
        generated.errors,
        generated.netlist
    );

    let node = ProbeTarget::parse("/amp/n1")
        .expect("drawn probe")
        .engine_name()
        .expect("engine node name");
    let load = &buffers["work/amp/schematic"].document().components[2];
    let current = OccurrenceProbeSpelling::for_element(
        &InstancePath::parse("/amp").expect("drawn occurrence"),
        'I',
        &load.name,
        &load.written_element_name(),
    )
    .expect("an ASCII occurrence spells");
    assert_eq!(current.display(), "I(/amp/load)");
    let values = final_values(
        &generated.netlist,
        &[format!("V({node})"), current.engine().to_owned()],
    );

    assert!((values[0] - 1.0).abs() < 1e-9, "V({node}) = {}", values[0]);
    assert!(
        (values[1].abs() - 1e-3).abs() < 1e-12,
        "{} = {}",
        current.engine(),
        values[1]
    );
}

/// Every card the deck writes is parsed under the name the authority gives
/// its component, whatever the drawing called it: a prefix added, a source
/// lowered to a behavioural card, a reference resolved to what it names.
#[test]
fn every_element_is_parsed_under_its_written_name() {
    let mut bench = Bench::new();
    let sense = bench.place(ComponentType::VoltageSource, 0, 0, "sense", "1", "");
    let misnamed = bench.place(ComponentType::Resistor, 200, 0, "C4", "1k", "");
    let unnamed = bench.place(ComponentType::Resistor, 400, 0, "", "1k", "");
    bench.connect((sense, 0), (misnamed, 0));
    bench.connect((misnamed, 1), (unnamed, 0));
    bench.ground((unnamed, 1));
    bench.ground((sense, 1));
    for (row, (kind, name, params)) in [
        (ComponentType::Vcvs, "E1", "vmax=5"),
        (ComponentType::Vcvs, "E2", ""),
        (ComponentType::Vccs, "G3", "poly=\"0,1m\""),
        (ComponentType::Ccvs, "H4", "vref=sense vmin=-5"),
        (ComponentType::Cccs, "F5", "vref=sense"),
    ]
    .into_iter()
    .enumerate()
    {
        let y = 300 * (row as i32 + 1);
        let source = bench.place(kind, 400, y, name, "2", params);
        let load = bench.place(
            ComponentType::Resistor,
            600,
            y,
            &format!("RL{row}"),
            "1k",
            "",
        );
        bench.connect((source, 0), (load, 0));
        bench.ground((source, 1));
        bench.ground((load, 1));
        if matches!(kind, ComponentType::Vcvs | ComponentType::Vccs) {
            bench.connect((sense, 0), (source, 2));
            bench.ground((source, 3));
        }
    }
    let rails = bench.place(
        ComponentType::OpAmp,
        400,
        2000,
        "U1",
        "1e5",
        "vmax=3 vmin=-3",
    );
    let rail_load = bench.place(ComponentType::Resistor, 600, 2000, "RU", "1k", "");
    bench.connect((sense, 0), (rails, 0));
    bench.ground((rails, 1));
    bench.connect((rails, 2), (rail_load, 0));
    bench.ground((rail_load, 1));

    let netlist = bench.netlist();
    let parsed = rspice_core::netlist::Netlist::parse(&netlist).expect("the deck parses");
    for component in bench.state.document().components.iter() {
        if component.kind == ComponentType::Ground {
            continue;
        }
        let written = component.written_element_name();
        assert!(
            parsed
                .elements
                .iter()
                .any(|element| element.name.eq_ignore_ascii_case(&written)),
            "{} drawn {:?} is not the element {written}:\n{netlist}",
            component.kind.display_name(),
            component.name
        );
    }
    solve_op(&netlist);
}

#[test]
fn lossy_ltra_line_bench_solves_op() {
    let mut bench = Bench::new();
    let v = bench.place(ComponentType::VoltageSource, 0, 0, "V1", "1", "");
    let rs = bench.place(ComponentType::Resistor, 200, 0, "RS", "50", "");
    let line = bench.place(
        ComponentType::LossyTransmissionLine,
        400,
        0,
        "O1",
        "",
        "r=12.45 l=8.972n c=0.468p len=16",
    );
    let rl = bench.place(ComponentType::Resistor, 600, 0, "RL", "75", "");
    bench.connect((v, 0), (rs, 0));
    bench.connect((rs, 1), (line, 0)); // a+
    bench.ground((line, 1)); // a-
    bench.connect((line, 2), (rl, 0)); // b+
    bench.ground((line, 3)); // b-
    bench.ground((rl, 1));
    bench.ground((v, 1));
    solve_op(&bench.netlist());
}

#[test]
fn lossy_txl_line_bench_solves_op() {
    let mut bench = Bench::new();
    let v = bench.place(ComponentType::VoltageSource, 0, 0, "V1", "1", "");
    let line = bench.place(
        ComponentType::LossyTransmissionLine,
        200,
        0,
        "O1",
        "",
        "kind=txl r=1 l=1n g=0 c=1p len=1",
    );
    let rl = bench.place(ComponentType::Resistor, 400, 0, "RL", "50", "");
    bench.connect((v, 0), (line, 0));
    bench.ground((line, 1));
    bench.connect((line, 2), (rl, 0));
    bench.ground((line, 3));
    bench.ground((rl, 1));
    bench.ground((v, 1));
    solve_op(&bench.netlist());
}

#[test]
fn coupled_cpl_line_bench_solves_op() {
    let mut bench = Bench::new();
    let v = bench.place(ComponentType::VoltageSource, 0, 0, "V1", "1", "");
    let line = bench.place(ComponentType::CoupledTransmissionLine, 200, 0, "P1", "", "");
    let r1 = bench.place(ComponentType::Resistor, 400, 0, "R1", "50", "");
    let r2 = bench.place(ComponentType::Resistor, 400, 200, "R2", "50", "");
    let r3 = bench.place(ComponentType::Resistor, 0, 300, "R3", "50", "");
    bench.connect((v, 0), (line, 0)); // a1
    bench.connect((line, 1), (r3, 0)); // a2 -> local termination
    bench.ground((r3, 1));
    bench.ground((line, 2)); // near reference
    bench.connect((line, 3), (r1, 0)); // b1
    bench.connect((line, 4), (r2, 0)); // b2
    bench.ground((r1, 1));
    bench.ground((r2, 1));
    bench.ground((line, 5)); // far reference
    bench.ground((v, 1));
    solve_op(&bench.netlist());
}

#[test]
fn memristor_bench_solves_tran() {
    // The TEAM memristor carries a hidden state node whose steady-state row is
    // degenerate at the default exponents, so the core gauges it to XON at the
    // operating point. That makes the palette device start from a real DC
    // solve, without UIC.
    let mut bench = Bench::new();
    let v = bench.place(ComponentType::VoltageSource, 0, 0, "V1", "0.2", "");
    let mr = bench.place(ComponentType::Memristor, 200, 0, "MR1", "", "");
    bench.connect((v, 0), (mr, 0));
    bench.ground((mr, 1));
    bench.ground((v, 1));
    solve_tran_with_uic(&bench.netlist(), 1e-7, 1e-9, false);
}

/// The engine files a memristor under its card's keyword, so a current probe
/// built from the component asks for `I(YMEMRISTOR!MR1)`. The drawn `MR1`
/// would name a MOSFET the deck does not hold.
#[test]
fn a_current_probe_on_a_memristor_reads_the_element_the_engine_files() {
    let mut bench = Bench::new();
    let v = bench.place(ComponentType::VoltageSource, 0, 0, "V1", "0.2", "");
    let mr = bench.place(ComponentType::Memristor, 200, 0, "MR1", "", "");
    bench.connect((v, 0), (mr, 0));
    bench.ground((mr, 1));
    bench.ground((v, 1));

    let probed = bench.state.document().components[mr].engine_element_name();
    assert_eq!(probed, "YMEMRISTOR!MR1");
    let values = final_values(
        &bench.netlist(),
        &[format!("I({probed})"), "I(V1)".to_owned()],
    );

    // The source's branch current is the memristor's, entering at its
    // positive terminal.
    assert!(values[0].abs() > 1e-9, "I({probed}) = {}", values[0]);
    assert!(
        (values[0] + values[1]).abs() <= 1e-9 * values[0].abs(),
        "I({probed}) = {}, I(V1) = {}",
        values[0],
        values[1]
    );
}

#[test]
fn rf_port_bench_solves_op() {
    let mut bench = Bench::new();
    let port = bench.place(ComponentType::RfPort, 0, 0, "P1", "", "dc=2");
    let rl = bench.place(ComponentType::Resistor, 200, 0, "RL", "50", "");
    bench.connect((port, 0), (rl, 0));
    bench.ground((rl, 1));
    bench.ground((port, 1));
    solve_op(&bench.netlist());
}

/// A schematic RF Port has to be able to author a large-signal drive, and the
/// card it emits has to keep the generator behind the reference impedance.
/// Switching to the ngspice `portnum=` spelling to carry a power would move the
/// source onto the plane and change the circuit, so `P` carries it instead.
#[test]
fn rf_port_authors_an_available_power_drive_on_its_own_card() {
    let mut bench = Bench::new();
    let port = bench.place(
        ComponentType::RfPort,
        0,
        0,
        "P1",
        "",
        "port=1 z0=50 pwr=1m freq=1e9 phase=90",
    );
    let rl = bench.place(ComponentType::Resistor, 200, 0, "RL", "50", "");
    bench.connect((port, 0), (rl, 0));
    bench.ground((rl, 1));
    bench.ground((port, 1));

    let netlist = bench.netlist();
    let card = netlist
        .lines()
        .find(|line| line.trim_start().starts_with("P1 "))
        .unwrap_or_else(|| panic!("no P1 card in:\n{netlist}"));
    for expected in ["PORT=1", "Z0=50", "PWR=1m", "FREQ=1e9", "PHASE=90"] {
        assert!(card.contains(expected), "{expected} missing from `{card}`");
    }

    solve_op(&netlist);
}

/// Frequency and phase describe a drive; with no power there is none to
/// describe, and emitting them would make the card claim a generator the
/// schematic never asked for.
#[test]
fn rf_port_omits_drive_timing_when_no_power_is_set() {
    let mut bench = Bench::new();
    let port = bench.place(
        ComponentType::RfPort,
        0,
        0,
        "P1",
        "",
        "port=1 z0=50 freq=1e9 phase=90",
    );
    let rl = bench.place(ComponentType::Resistor, 200, 0, "RL", "50", "");
    bench.connect((port, 0), (rl, 0));
    bench.ground((rl, 1));
    bench.ground((port, 1));

    let netlist = bench.netlist();
    let card = netlist
        .lines()
        .find(|line| line.trim_start().starts_with("P1 "))
        .unwrap_or_else(|| panic!("no P1 card in:\n{netlist}"));
    assert!(
        !card.contains("FREQ") && !card.contains("PHASE") && !card.contains("PWR"),
        "unpowered port still claims a drive: `{card}`"
    );
}

#[test]
fn placed_k_coupling_bench_solves_op() {
    let mut bench = Bench::new();
    let v = bench.place(ComponentType::VoltageSource, 0, 0, "V1", "1", "");
    let r1 = bench.place(ComponentType::Resistor, 200, 0, "R1", "10", "");
    let l1 = bench.place(ComponentType::Inductor, 400, 0, "L1", "1m", "");
    let l2 = bench.place(ComponentType::Inductor, 400, 200, "L2", "1m", "");
    // Offset row so the connecting wire cannot chain through L2's other
    // terminal (horizontal wires merge any terminal lying on the segment).
    let r2 = bench.place(ComponentType::Resistor, 600, 300, "R2", "100", "");
    bench.place(
        ComponentType::CoupledInductor,
        600,
        0,
        "K1",
        "0.9",
        "inductors=\"L1 L2\"",
    );
    bench.connect((v, 0), (r1, 0));
    bench.connect((r1, 1), (l1, 0));
    bench.ground((l1, 1));
    bench.ground((v, 1));
    bench.connect((l2, 0), (r2, 0));
    bench.ground((r2, 1));
    bench.ground((l2, 1));
    solve_op(&bench.netlist());
}

#[test]
fn xspice_gain_and_limiter_bench_solves_op() {
    let mut bench = Bench::new();
    let v = bench.place(ComponentType::VoltageSource, 0, 0, "V1", "1", "");
    let gain = bench.place(ComponentType::XspiceGain, 200, 0, "A1", "", "gain=2");
    let limiter = bench.place(ComponentType::XspiceLimiter, 400, 0, "A2", "", "");
    let rl = bench.place(ComponentType::Resistor, 600, 0, "RL", "1k", "");
    bench.connect((v, 0), (gain, 0));
    bench.connect((gain, 1), (limiter, 0));
    bench.connect((limiter, 1), (rl, 0));
    bench.ground((rl, 1));
    bench.ground((v, 1));
    solve_op(&bench.netlist());
}

#[test]
fn xspice_and_gate_bench_solves_tran_with_auto_bridges() {
    let mut bench = Bench::new();
    let va = bench.place(ComponentType::VoltageSource, 0, 0, "VA", "3.3", "");
    let vb = bench.place(ComponentType::VoltageSource, 0, 200, "VB", "3.3", "");
    let gate = bench.place(ComponentType::XspiceAndGate, 200, 0, "A1", "", "");
    let rl = bench.place(ComponentType::Resistor, 400, 0, "RL", "1k", "");
    bench.connect((va, 0), (gate, 0));
    bench.connect((vb, 0), (gate, 1));
    bench.connect((gate, 2), (rl, 0));
    bench.ground((rl, 1));
    bench.ground((va, 1));
    bench.ground((vb, 1));
    solve_tran(&bench.netlist(), 1e-7, 1e-9);
}

#[test]
fn xspice_dff_bench_solves_tran() {
    let mut bench = Bench::new();
    let vd = bench.place(ComponentType::VoltageSource, 0, 0, "VD", "3.3", "");
    let clk = bench.place(
        ComponentType::VoltageSourcePulse,
        0,
        200,
        "VCLK",
        "0",
        "v2=3.3 td=5n tr=1n tf=1n pw=20n per=40n",
    );
    let dff = bench.place(ComponentType::XspiceDFlipFlop, 200, 0, "A1", "", "");
    let rq = bench.place(ComponentType::Resistor, 400, 0, "RQ", "1k", "");
    let rqb = bench.place(ComponentType::Resistor, 400, 200, "RQB", "1k", "");
    bench.connect((vd, 0), (dff, 0));
    bench.connect((clk, 0), (dff, 1));
    bench.connect((dff, 2), (rq, 0));
    bench.connect((dff, 3), (rqb, 0));
    bench.ground((rq, 1));
    bench.ground((rqb, 1));
    bench.ground((vd, 1));
    bench.ground((clk, 1));
    solve_tran(&bench.netlist(), 2e-7, 1e-9);
}

#[test]
fn xspice_sr_latch_bench_solves_tran() {
    let mut bench = Bench::new();
    let vs = bench.place(ComponentType::VoltageSource, 0, 0, "VS", "3.3", "");
    let vr = bench.place(ComponentType::VoltageSource, 0, 200, "VR", "0", "");
    let ven = bench.place(ComponentType::VoltageSource, 0, 400, "VEN", "3.3", "");
    let latch = bench.place(ComponentType::XspiceSrLatch, 200, 0, "A1", "", "");
    let rq = bench.place(ComponentType::Resistor, 400, 0, "RQ", "1k", "");
    let rqb = bench.place(ComponentType::Resistor, 400, 200, "RQB", "1k", "");
    bench.connect((vs, 0), (latch, 0));
    bench.connect((vr, 0), (latch, 1));
    bench.connect((ven, 0), (latch, 2));
    bench.connect((latch, 3), (rq, 0));
    bench.connect((latch, 4), (rqb, 0));
    bench.ground((rq, 1));
    bench.ground((rqb, 1));
    bench.ground((vs, 1));
    bench.ground((vr, 1));
    bench.ground((ven, 1));
    solve_tran(&bench.netlist(), 1e-7, 1e-9);
}

#[test]
fn noise_current_source_bench_solves_tran() {
    let mut bench = Bench::new();
    let i = bench.place(ComponentType::CurrentSourceNoise, 0, 0, "I1", "", "");
    let r = bench.place(ComponentType::Resistor, 200, 0, "R1", "1k", "");
    bench.connect((i, 0), (r, 0));
    bench.ground((i, 1));
    bench.ground((r, 1));
    solve_tran(&bench.netlist(), 1e-5, 1e-7);
}

/// The deck writes every ground name as node 0, so V1 from `vdd` to a ground
/// symbol beside V2 from `vdd` to a `GND!` label is two sources across one
/// pair of nodes. The engine refuses that loop, as the source-to-source rule
/// does before it.
#[test]
fn sources_into_ground_and_into_a_ground_global_are_a_loop_the_engine_refuses() {
    let mut bench = Bench::new();
    let v1 = bench.place(ComponentType::VoltageSource, 0, 0, "V1", "1", "");
    let v2 = bench.place(ComponentType::VoltageSource, 200, 0, "V2", "2", "");
    let load = bench.place(ComponentType::Resistor, 400, 0, "RL", "1k", "");
    bench.connect((v1, 0), (v2, 0));
    bench.connect((v2, 0), (load, 0));
    bench.ground((v1, 1));
    bench.ground((load, 1));
    bench.label((v2, 1), "GND!");
    let netlist = bench.netlist();
    let v2_card = netlist
        .lines()
        .find(|line| line.starts_with("V2 "))
        .expect("V2 is written");
    assert_eq!(v2_card.split_whitespace().nth(2), Some("0"), "{netlist}");

    let refused = run_spec_request(
        &EngineBridge::new(),
        AnalysisSpec::LegacyDcOp,
        SpecExecutionOptions::default(),
        &netlist,
        None,
        &ResolvedExecutionDependencies::default(),
        &rspice_core::abort_signal::NoAbort,
    )
    .expect_err("the engine refuses a loop of ideal sources");
    assert!(
        refused
            .to_string()
            .contains("loops of ideal voltage sources"),
        "{refused}\n{netlist}"
    );
}

/// A drawing returned through a `GND!` label is written with that net as node
/// 0: the deck has no node named `GND!`, and the ngspice dialect it runs
/// under does not fold that name onto ground itself. An objective a form
/// measures against the drawn ground by that name is measured against node
/// 0, and one measured against a node the deck lacks is refused rather than
/// read as 0 V.
#[test]
fn an_output_reference_naming_the_drawn_ground_is_measured_against_node_zero() {
    use crate::analysis_preparation::{AnalysisInputs, analysis_draft_spec};
    use rspice_results::analysis_result::AnalysisResult;
    use rspice_results::run::SimulationRun;
    use rspice_simulation_contract::analysis_draft::AnalysisDraft;
    use rspice_simulation_contract::optimization_draft::{
        OptimizationConfig, OptimizationDialogState, OptimizationVariableConfig,
    };

    let mut bench = Bench::new();
    let v1 = bench.place(ComponentType::VoltageSource, 0, 0, "V1", "2", "");
    let r1 = bench.place(ComponentType::Resistor, 200, 0, "R1", "1k", "");
    let r2 = bench.place(ComponentType::Resistor, 400, 0, "R2", "{RLOAD}", "");
    bench.connect((v1, 0), (r1, 0));
    bench.connect((r1, 1), (r2, 0));
    bench.label((r1, 1), "out");
    bench.label((v1, 1), "GND!");
    bench.label((r2, 1), "GND!");
    let netlist = bench.netlist_with_parameters(&[".param RLOAD=1k".to_owned()]);
    assert!(!netlist.to_ascii_uppercase().contains("GND!"), "{netlist}");

    let optimized_against = |reference: &str| {
        let draft = OptimizationDialogState::from_config(&OptimizationConfig {
            variables: vec![OptimizationVariableConfig {
                name: "RLOAD".to_owned(),
                min: 500.0,
                max: 5_000.0,
                initial: 1_000.0,
            }],
            objective_unit: "V".to_owned(),
            objective_node: "out".to_owned(),
            objective_ref: reference.to_owned(),
            target_value: Some(1.2),
            ..OptimizationConfig::default()
        });
        let inputs: AnalysisInputs<'_, SimulationRun, AnalysisResult> = AnalysisInputs {
            sim_setup: &Default::default(),
            schematic: &Default::default(),
            selected_components: &Default::default(),
            selection_occurrence: None,
            runs: &[],
            active_run: None,
            project_revision: rspice_app_types::product::ObjectRevision::INITIAL,
            plan_payloads: &[],
        };
        let spec = analysis_draft_spec(&inputs, &AnalysisDraft::Optimization(draft))
            .expect("the form builds a request");
        run_spec_request(
            &EngineBridge::new(),
            spec,
            SpecExecutionOptions::default(),
            &netlist,
            None,
            &ResolvedExecutionDependencies::default(),
            &rspice_core::abort_signal::NoAbort,
        )
    };

    // V(out) = 2 V * RLOAD / (1k + RLOAD) is 1.2 V at RLOAD = 1.5k.
    for ground in ["GND!", "ground", "0"] {
        let result = optimized_against(ground);
        let Ok(crate::results::SimulationResult::Optimization {
            best_variables,
            best_objectives,
            ..
        }) = &result
        else {
            panic!("{ground}: {result:?}");
        };
        let [objective] = best_objectives.as_slice() else {
            panic!("{ground}: one objective: {best_objectives:?}");
        };
        assert!(
            (objective.value - 1.2).abs() < 1e-3,
            "{ground}: V(out) = {}",
            objective.value
        );
        let rload = best_variables["RLOAD"];
        assert!((rload - 1500.0).abs() < 5.0, "{ground}: RLOAD = {rload}");
    }

    let refused = optimized_against("nowhere").expect_err("a node the deck lacks is refused");
    assert!(
        refused
            .to_string()
            .contains("reference node 'nowhere' not found"),
        "{refused}"
    );
}
