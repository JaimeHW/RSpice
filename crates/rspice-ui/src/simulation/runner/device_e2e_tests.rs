//! End-to-end device qualification: every palette device's generated
//! netlist must parse, build, and SOLVE in rspice-core — not merely
//! string-match. Each bench places real components, wires their
//! terminals, generates the deck through the production
//! `NetlistGenerator`, and runs the engine through the same spec path
//! the simulate action uses.

use super::SpecExecutionOptions;
use super::spec::run_spec_request;
use crate::simulation::engine_bridge::EngineBridge;
use crate::simulation::execution::ResolvedExecutionDependencies;
use crate::simulation::multi_run::AnalysisSpec;
use crate::simulation::netlist_gen::generate_netlist;
use crate::state::{Component, ComponentType, Point, SchematicState, Wire};

struct Bench {
    state: SchematicState,
    next_id: u64,
}

impl Bench {
    fn new() -> Self {
        Self {
            state: SchematicState::default(),
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
        self.state.components.push(component);
        self.state.components.len() - 1
    }

    fn terminal(&self, index: usize, terminal: usize) -> Point {
        self.state.components[index].terminal_positions()[terminal].1
    }

    fn connect(&mut self, from: (usize, usize), to: (usize, usize)) {
        let a = self.terminal(from.0, from.1);
        let b = self.terminal(to.0, to.1);
        let id = self.next_id;
        self.next_id += 1;
        self.state.wires.push(Wire::segment(id, a, b));
    }

    fn ground(&mut self, at: (usize, usize)) {
        let point = self.terminal(at.0, at.1);
        let id = self.next_id;
        self.next_id += 1;
        // The ground terminal sits 10 units above its anchor position.
        self.state.components.push(Component::new(
            id,
            ComponentType::Ground,
            Point::new(point.x, point.y + 10),
        ));
    }

    fn netlist(&self) -> String {
        let result = generate_netlist(&self.state);
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
