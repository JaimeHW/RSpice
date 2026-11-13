//! End-to-end policy for floating components at the shared DC-bias boundary.

use rspice_core::StaticMatrix;
use rspice_core::engine::{
    ContinuationAid, ContinuationStage, Engine, SimulationConfig, SimulationError,
    SimulationErrorCategory, SpiceDialect,
};
use rspice_core::netlist::Netlist;
use std::cell::RefCell;
use std::sync::Once;

fn engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

fn parse(deck: &str) -> Netlist {
    Netlist::parse(deck).expect("deck parses")
}

static LOG_INIT: Once = Once::new();

thread_local! {
    static CAPTURED: RefCell<Option<Vec<String>>> = const { RefCell::new(None) };
}

struct CapturingLogger;

impl log::Log for CapturingLogger {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        metadata.level() <= log::Level::Warn
    }

    fn log(&self, record: &log::Record<'_>) {
        if self.enabled(record.metadata()) {
            CAPTURED.with(|captured| {
                if let Some(lines) = captured.borrow_mut().as_mut() {
                    lines.push(record.args().to_string());
                }
            });
        }
    }

    fn flush(&self) {}
}

static LOGGER: CapturingLogger = CapturingLogger;

/// Run `body` with the warnings it logs on this thread collected.
fn warnings_of<T>(body: impl FnOnce() -> T) -> (T, Vec<String>) {
    LOG_INIT.call_once(|| {
        log::set_logger(&LOGGER).expect("the capture logger installs");
        log::set_max_level(log::LevelFilter::Warn);
    });
    CAPTURED.with(|captured| *captured.borrow_mut() = Some(Vec::new()));
    let value = body();
    let lines = CAPTURED.with(|captured| captured.borrow_mut().take().unwrap_or_default());
    (value, lines)
}

fn voltage(result: &rspice_core::solver::SimulationResult, node: &str) -> f64 {
    result
        .try_voltage_named(node)
        .unwrap_or_else(|| panic!("missing voltage for node {node}"))
}

fn check_current_feedback_bias(feedback: &str) {
    let netlist = parse(&format!(
        "Current drive with a feedback conductance\nI1 0 out dc 1m ac 1m\n{feedback}\n.end\n"
    ));
    let engine = engine();
    // Feedback supplies 1 mS even though a passive-conduction walk cannot
    // reach ground from OUT. Physical KCL is .001 * V(out) = .001.
    let result = engine
        .run_dc_op(&netlist)
        .expect("feedback determines DC bias");
    assert!((voltage(&result, "out") - 1.0).abs() < 1e-8);
    let circuit = engine.build_circuit(&netlist).unwrap();
    assert!(circuit.fatal_no_dc_path_nodes().is_empty());
    let topology = rspice_core::netlist::analyze_dc_ground_paths(&netlist.elements, None).unwrap();
    assert!(topology.fatal_nodes().next().is_none());
    // Keep the known conduction partition for the post-solve audit. An
    // unresolved source current does not make its output topology unknown.
    assert!(
        topology
            .floating_component_is_certain
            .iter()
            .all(|certain| *certain)
    );
    let ac = engine.run_ac(&netlist, &[0.0, 1e3]).unwrap();
    for point in ac {
        let out = point
            .node_names
            .iter()
            .position(|node| node.eq_ignore_ascii_case("out"))
            .unwrap();
        assert!((point.voltages[out].re - 1.0).abs() < 1e-8);
        assert!(point.voltages[out].im.abs() < 1e-12);
    }
    let sweep = engine
        .run_dc_sweep(&netlist, "I1", -1e-3, 1e-3, 5e-4)
        .unwrap();
    assert_eq!(sweep.len(), 5);
    for (current, point) in sweep {
        assert!((voltage(&point, "out") - 1000.0 * current).abs() < 1e-8);
    }
    let transient = engine.run_tran(&netlist, 2e-6, 1e-6).unwrap();
    let output = transient.try_voltage_waveform_named("out").unwrap();
    assert!(output.len() >= 2);
    assert!(output.iter().all(|value| (value - 1.0).abs() < 1e-8));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn self_controlled_vccs_balances_independent_dc_drive() {
    check_current_feedback_bias("G1 out 0 out 0 1m");
    check_current_feedback_bias("E1 copy 0 out 0 1\nG1 out 0 copy 0 1m");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn behavioral_feedback_balances_independent_dc_drive() {
    check_current_feedback_bias("B1 out 0 i={v(out)/1000}");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn current_controlled_feedback_balances_independent_dc_drive() {
    check_current_feedback_bias(
        "E1 copy 0 out 0 1\nVsense copy sense 0\nRsense sense 0 1k\nF1 out 0 Vsense 1",
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn feedback_balances_current_injected_elsewhere_in_a_conductive_component() {
    let netlist = parse(
        "Feedback grounds a resistor island\nI1 0 drive 1m\nR1 drive out 1k\nG1 out 0 out 0 1m\n.end\n",
    );
    let result = engine().run_dc_op(&netlist).unwrap();
    assert!((voltage(&result, "out") - 1.0).abs() < 1e-8);
    assert!((voltage(&result, "drive") - 2.0).abs() < 1e-8);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn unresolved_controlled_currents_do_not_admit_a_shunt_dependent_bias() {
    for source in ["G1 out 0 ref 0 1m", "F1 out 0 Vref 1", "B1 out 0 i=0"] {
        let netlist = parse(&format!(
            "Unbalanced DC drive\nVref ref 0 0\nI1 0 out 1m\n{source}\nC1 out 0 1u\n.end\n"
        ));
        let error = engine()
            .run_dc_op(&netlist)
            .expect_err("unknown current is not a ground path");
        assert!(
            error.to_string().contains("no DC path to ground"),
            "{source}: {error}"
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn sensing_or_driving_inside_a_floating_component_does_not_balance_its_net_current() {
    for source in [
        "G1 other 0 out 0 1m\nR1 other 0 1k",
        "G1 out other out 0 1m\nR1 out other 1k",
    ] {
        let netlist = parse(&format!(
            "Unbalanced component\nI1 0 out 1m\n{source}\nC1 out 0 1u\n.end\n"
        ));
        let engine = engine();
        let circuit = engine.build_circuit(&netlist).unwrap();
        assert!(
            circuit
                .fatal_no_dc_path_nodes()
                .iter()
                .any(|node| node.eq_ignore_ascii_case("out"))
        );
        let error = engine
            .run_dc_op(&netlist)
            .expect_err("net component current remains nonzero");
        assert!(
            error.to_string().contains("no DC path to ground"),
            "{source}: {error}"
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn homogeneous_dc_retains_a_unique_bias_when_floating_lu_loses_a_pivot() {
    for exponent in [-100, 0, 100] {
        let scale = 2.0_f64.powi(exponent);
        for algebraic_first in [false, true] {
            let first = format!("G11 a 0 a 0 {:.17e}\n", 0.06666666666666668 * scale);
            let second = format!("G22 b 0 b 0 {:.17e}\n", 0.3 * scale);
            let netlist = parse(&format!(
                "Unique zero bias\n{}{}G12 a 0 b 0 {:.17e}\nG21 b 0 a 0 {:.17e}\n.op\n.end\n",
                if algebraic_first { &second } else { &first },
                if algebraic_first { &first } else { &second },
                0.1 * scale,
                0.2 * scale,
            ));
            let mut config = SimulationConfig::default();
            config.convergence_config.gmin_target = 0.0;
            config.convergence_config.gmin_stepping = false;
            config.convergence_config.source_stepping = false;
            let engine = Engine::new_with_resolved_config(config);
            // Exact binary64 determinant is nonzero. Neither a numerical
            // shunt nor a continuation aid is needed to select this bias.
            let result = engine.run_dc_op(&netlist).unwrap_or_else(|error| {
                panic!("scale=2^{exponent}, order={algebraic_first}: {error}")
            });
            assert_eq!(voltage(&result, "a"), 0.0);
            assert_eq!(voltage(&result, "b"), 0.0);
            assert_eq!(engine.convergence_quality().gmin_stepping_count, 0);
            assert_eq!(engine.convergence_quality().source_stepping_count, 0);
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn zero_residual_does_not_admit_an_underdetermined_dc_bias() {
    for exponent in [-100, 0, 100] {
        let scale = 2.0_f64.powi(exponent);
        // Rows are exactly proportional in binary64, so infinitely many
        // solutions have a zero residual, including the all-zero vector.
        let netlist = parse(&format!(
            "Nonunique zero bias\nG11 a 0 a 0 {scale:.17e}\nG12 a 0 b 0 {scale:.17e}\nG21 b 0 a 0 {:.17e}\nG22 b 0 b 0 {:.17e}\n.op\n.end\n",
            2.0 * scale,
            2.0 * scale,
        ));
        let mut config = SimulationConfig::default();
        config.convergence_config.gmin_target = 0.0;
        config.convergence_config.gmin_stepping = false;
        config.convergence_config.source_stepping = false;
        let error = Engine::new_with_resolved_config(config)
            .run_dc_op(&netlist)
            .expect_err("the circuit has no unique bias");
        assert!(error.to_string().contains("singular"), "{error}");
    }
}

/// A loop that nothing drives and the rest of the circuit only senses has
/// the unique bias zero: `a = -k*b` (E1), `c = a` (R5 carries no current)
/// and `b/R4 + gm*c = 0` (G1), so `b*(1/R4 - gm*k) = 0`. No equation of the
/// loop has one unknown, and faer's LU leaves remnants near 1e-33 there
/// beside the 1 V drive. The first deck failed the sparse backward-error
/// check at 1.0 for them; the second was then answered by gmin stepping
/// with every node shunted to ground, and `out` read 0.476 V. Both now
/// solve directly.
#[test]
fn a_sensed_loop_that_nothing_drives_solves_to_zero_on_the_faer_backend() {
    for (title, values, expected_out) in [
        (
            "Sensed undriven loop",
            "R1 in 0 3.46\nR2 out 0 99.1\nR3 in out 66.6\nE1 a 0 b 0 -10.4\nG1 b 0 c 0 2.33m\nR4 b 0 1540\nR5 c a 16.4\nG2 out 0 a 0 -0.176\n",
            99.1 / (99.1 + 66.6),
        ),
        (
            "Sensed undriven loop, rounded",
            "R2 out 0 100\nR3 in out 100\nE1 a 0 b 0 -10\nG1 b 0 c 0 2.2m\nR4 b 0 1.5k\nR5 c a 15\nG2 out 0 a 0 -0.18\n",
            0.5,
        ),
    ] {
        let netlist = parse(&format!(
            "{title}\nV1 in 0 DC 1\n{values}.options solver=faer\n.op\n.end\n"
        ));
        let engine = engine();
        let result = engine
            .run_dc_op(&netlist)
            .unwrap_or_else(|error| panic!("{title}: {error}"));
        for node in ["a", "b", "c"] {
            assert_eq!(voltage(&result, node), 0.0, "{title}: v({node})");
        }
        let out = voltage(&result, "out");
        assert!(
            (out - expected_out).abs() <= 1e-12 * expected_out,
            "{title}: v(out) = {out:e}, expected {expected_out:e}"
        );
        assert_eq!(
            engine.convergence_quality().gmin_stepping_count,
            0,
            "{title}: the bias needs no continuation"
        );
    }
}

/// A conductance of -1e-15 S is the whole circuit at `n`, and without a
/// GMIN floor that system is regular: ngspice-46 prints
/// v(n) = -9.9999999999999988e+08.
#[test]
fn a_negative_conductance_solves_without_the_nodal_floor() {
    let netlist = parse(
        "Negative conductance cancelling the GMIN floor\nI1 0 n 1u\nR1 n 0 -1e15\n.op\n.end\n",
    );
    let result = engine()
        .run_dc_op(&netlist)
        .expect("the circuit's own system is regular");
    let n = voltage(&result, "n");
    assert!((n + 1.0e9).abs() <= 1.0e-6, "v(n) = {n:e}, ngspice-46 -1e9");
}

/// A converged operating point is polished by one more solve of its static
/// system, which was assembled with the nodal floor even when the operating
/// point had been solved without it. The polish accepts any answer within
/// the voltage tolerance, so it moved a leakage-held node back to the
/// floor's: beside a diode, the 1e9/1e9 Ohm divider read 0.49999975 V.
/// ngspice-46 prints v(n) = 4.999999999999999e-01.
#[test]
fn the_polish_of_an_operating_point_solved_without_the_floor_keeps_its_system() {
    let netlist = parse(
        "Leakage-held divider beside a diode\nV1 in 0 1\nR1 in n 1e9\nR2 n 0 1e9\n\
         RD in a 1k\nD1 a 0 dmod\n.model dmod d(is=1e-14)\n.op\n.end\n",
    );
    let result = engine()
        .run_dc_op(&netlist)
        .expect("the diode deck has an operating point");
    let n = voltage(&result, "n");
    assert!((n - 0.5).abs() <= 1.0e-12, "v(n) = {n:e}, ngspice-46 0.5");
}

/// Every point of a DC sweep is an operating point solved without the nodal
/// floor while that solve succeeds. Polished at the 1e-15 S floor, the
/// source feeding a cubic behavioural current read the floor's 1e-15 * V on
/// top of the device's 1e-20 * V^3. ngspice ends each point at
/// CKTdiagGmin = gshunt = 0; Xyce 7.10 prints I(V1) = -1e-20, -3.375e-20,
/// -8e-20.
#[test]
fn a_dc_sweep_point_carries_no_nodal_floor_current() {
    let netlist = parse(
        "Cubic behavioural current
V1 n1 0 1
B1 n1 0 I=1e-20*V(n1)*V(n1)*V(n1)
         .dc V1 1 2 0.5
.end
",
    );
    for dialect in [SpiceDialect::Ngspice, SpiceDialect::BestAvailable] {
        let sweep = Engine::new(SimulationConfig::default().with_spice_dialect(dialect))
            .run_dc_sweep(&netlist, "v1", 1.0, 2.0, 0.5)
            .expect("the sweep solves");
        assert_eq!(sweep.len(), 3, "{dialect:?}");
        for (v, point) in &sweep {
            let current = point.branch_current_named("V1").expect("I(V1)");
            let expected = -1.0e-20 * v * v * v;
            assert!(
                (current - expected).abs() <= 1.0e-30,
                "{dialect:?} V1 = {v}: I(V1) = {current:e}, Xyce 7.10 {expected:e}",
            );
        }
    }
}

/// An XSPICE digital net between two code models is an event net. Its node
/// owns an MNA placeholder row that no analog stamp reaches, and without
/// the nodal floor that row made the system singular ("singular matrix:
/// check node D1"), so the whole circuit went back to the 1e-15 S floor,
/// which pulled the leakage-held divider at `n` to 0.49999975 V. ngspice
/// keeps event nodes out of its analog matrix: ngspice-46 prints
/// v(n) = 4.999999999999999e-01.
#[test]
fn an_event_net_does_not_send_the_operating_point_back_to_the_nodal_floor() {
    let netlist = parse(
        "Event nets beside a leakage-held divider\nV1 in 0 1\n\
         A1 [in] [d1] adc1\n.model adc1 adc_bridge(in_low=0.5 in_high=0.5)\n\
         A2 d1 d2 inv1\n.model inv1 d_inverter\n\
         A3 [d2] [out] dac1\n.model dac1 dac_bridge\n\
         RL out 0 1k\nR1 in n 1e9\nR2 n 0 1e9\n.op\n.end\n",
    );
    let result = engine()
        .run_dc_op(&netlist)
        .expect("the event nets' placeholders are pinned");
    let n = voltage(&result, "n");
    assert!((n - 0.5).abs() <= 1.0e-12, "v(n) = {n:e}, ngspice-46 0.5");
    assert_eq!(voltage(&result, "out"), 0.0);
}

/// Only capacitors reach `b`, so the system without the nodal floor is
/// singular there. It used to go back to the 1e-15 S floor on every node
/// ("singular matrix: check node B"), which pulled the 1e12/1e12 Ohm divider
/// beside it to 0.49975012493753124 V. Spectre installs GMIN on the node
/// with no DC path alone; the rest of the circuit keeps no floor, and the
/// divider reads the 0.5 V that ngspice-46 prints for it alone (with `b` in
/// the deck, ngspice-46's GMIN stepping fails at "singular matrix: check node
/// b"). `b` then takes its charge-neutral 0.5 V. Every point of a sweep
/// keeps the node held and the divider without the floor.
#[test]
fn a_node_without_a_dc_path_alone_takes_the_floor() {
    let netlist = parse(
        "Capacitor-only node beside a leakage-held divider\nV1 in 0 1\nR1 in mid 1e12\n\
         R2 mid 0 1e12\nV2 in2 0 1\nR3 in2 a 1k\nC1 a b 1p\nC2 b 0 1p\n.op\n.end\n",
    );
    let (result, warnings) = warnings_of(|| engine().run_dc_op(&netlist));
    let result = result.expect("the node without a DC path is held");
    let mid = voltage(&result, "mid");
    assert!(
        (mid - 0.5).abs() <= 1.0e-12,
        "v(mid) = {mid:e}, ngspice-46 0.5 for the divider alone"
    );
    let b = voltage(&result, "b");
    assert!((b - 0.5).abs() <= 1.0e-12, "v(b) = {b:e}");
    assert!(
        warnings
            .iter()
            .any(|line| line == "Voltage Node (B) does not have a DC path to ground"),
        "{warnings:?}"
    );
    assert!(
        !warnings.iter().any(|line| line.contains("singular matrix")),
        "{warnings:?}"
    );
    let sweep = engine()
        .run_dc_sweep(&netlist, "v1", 0.5, 1.5, 0.5)
        .expect("the sweep solves");
    assert_eq!(sweep.len(), 3);
    for (v, point) in &sweep {
        let mid = voltage(point, "mid");
        assert!(
            (mid - v / 2.0).abs() <= 1.0e-12,
            "V1 = {v}: v(mid) = {mid:e}"
        );
    }
}

fn waveform(result: &rspice_core::engine::TransientResult, node: &str) -> Vec<f64> {
    result
        .node_index_named(node)
        .and_then(|index| result.try_voltage_waveform(index))
        .unwrap_or_else(|| panic!("node {node} is in the transient result"))
        .to_vec()
}

/// A 1e12/1e12 Ohm divider reads 0.5 V at every point of its transient,
/// t = 0 included, as ngspice-46 prints it (5.000000000000e-01 at all 59
/// points of `.tran 1u 10u`): ngspice integrates at CKTdiagGmin =
/// CKTgshunt, zero unless the deck sets RSHUNT. The transient's operating
/// point used to be solved with the 1e-15 S floor on every node and the
/// transient kept it, so the divider read 0.49975012493753124 V throughout.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn a_transient_carries_no_nodal_floor() {
    let netlist = parse("Leakage-held divider\nV1 in 0 1\nR1 in mid 1e12\nR2 mid 0 1e12\n.end\n");
    let result = engine()
        .run_tran(&netlist, 10e-6, 1e-6)
        .expect("the transient solves");
    let mid = waveform(&result, "mid");
    assert!(mid.len() > 2);
    for (time, value) in result.time.iter().zip(&mid) {
        assert!(
            (value - 0.5).abs() <= 1.0e-12,
            "v(mid) = {value:e} at t = {time:e}, ngspice-46 0.5"
        );
    }
}

/// Only capacitors reach `b`. Its transient's operating point holds `b`
/// alone, as the DC operating point does, and the transient keeps that one
/// hold, so the divider beside it reads 0.5 V at every point, the value
/// ngspice-46 prints for it alone and in a UIC transient of this deck.
/// (Without UIC, ngspice-46's operating point fails at "singular matrix:
/// check node b" and its transient-op fallback puts GMIN on every node, so
/// it starts and stays at 0.333333 V.) `b` starts at its charge-neutral
/// 0.5 V.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn a_transient_holds_only_the_node_without_a_dc_path() {
    let netlist = parse(
        "Capacitor-only node beside a leakage-held divider\nV1 in 0 1\nR1 in mid 1e12\n\
         R2 mid 0 1e12\nV2 in2 0 1\nR3 in2 a 1k\nC1 a b 1p\nC2 b 0 1p\n.end\n",
    );
    let (result, warnings) = warnings_of(|| engine().run_tran(&netlist, 1e-6, 2e-8));
    let result = result.expect("the node without a DC path is held");
    let mid = waveform(&result, "mid");
    for (time, value) in result.time.iter().zip(&mid) {
        assert!(
            (value - 0.5).abs() <= 1.0e-12,
            "v(mid) = {value:e} at t = {time:e}"
        );
    }
    let b = waveform(&result, "b");
    assert!((b[0] - 0.5).abs() <= 1.0e-12, "v(b) = {:e} at t = 0", b[0]);
    assert!(
        warnings
            .iter()
            .any(|line| line == "Voltage Node (B) does not have a DC path to ground"),
        "{warnings:?}"
    );
    assert!(
        !warnings.iter().any(|line| line.contains("singular matrix")),
        "{warnings:?}"
    );
}

/// A UIC transient solves no operating point, so no singular system says
/// which node lacks a DC path: nothing reaches `u` but a controlled current
/// its own voltage does not set, and without a floor its row is empty at
/// every step. ngspice-46 stops there ("singular matrix: check node u",
/// "Timestep too small"). Each node the ground-path walk proves floating
/// takes the hold the operating point would give it, so `u` stays at 0 V
/// and the divider beside it reads 0.5 V.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn a_uic_transient_holds_a_node_without_a_dc_path() {
    let netlist = parse(
        "Zero-gain VCCS node beside a leakage-held divider\nV1 in 0 1\nR1 in mid 1e12\n\
         R2 mid 0 1e12\nV5 in5 0 1\nR5 in5 0 1k\nG1 0 u in5 0 0\n.end\n",
    );
    let result = engine()
        .run_tran_with_startup_mode(
            &netlist,
            1e-6,
            2e-8,
            rspice_core::engine::TransientStartupMode::Uic,
        )
        .expect("the node without a DC path is held");
    let mid = waveform(&result, "mid");
    let last = *mid.last().expect("points");
    assert!(
        (last - 0.5).abs() <= 1.0e-12,
        "v(mid) = {last:e} at the end"
    );
    assert!(waveform(&result, "u").iter().all(|value| *value == 0.0));
}

/// `p`'s own conductances cancel, so the circuit's system is singular and
/// the solve goes back to the 1e-15 S GMIN floor, which `n`'s conductance
/// cancels exactly; every GMIN stage above the floor solves. GMIN stepping
/// used to answer with the stage just above the floor, about 1.1e8 V at
/// `n`, which is the solution of a different circuit. The operating point
/// does not exist (ngspice-46: singular matrix at `p`, GMIN stepping
/// fails), and the refusal names the aid and the stage it could not solve,
/// whether or not source stepping runs after it.
#[test]
fn a_singular_floor_is_refused_rather_than_answered_by_a_shunted_gmin_stage() {
    let netlist = parse(
        "Negative conductance cancelling the GMIN floor\nI1 0 n 1u\nR1 n 0 -1e15\n\
         I2 0 p 1u\nR2 p 0 1e15\nR3 p 0 -1e15\n.op\n.end\n",
    );
    for source_stepping in [true, false] {
        let mut config = SimulationConfig::default();
        config.convergence_config.source_stepping = source_stepping;
        let engine = Engine::new(config);
        let error = engine
            .run_dc_op(&netlist)
            .expect_err("no operating point exists");
        let SimulationError::ContinuationFailed(failure) = &error else {
            panic!("source stepping {source_stepping}: expected an aid failure, got {error}");
        };
        assert_eq!(failure.aid, ContinuationAid::GminStepping, "{error}");
        assert_eq!(failure.unsolved, ContinuationStage::Step(1e-15), "{error}");
        assert!(failure.reached.is_some_and(|gmin| gmin > 1e-15), "{error}");
        assert!(failure.cause.is_some(), "{error}");
        assert_eq!(
            error.descriptor().category,
            SimulationErrorCategory::Convergence
        );
    }
}

#[test]
fn internal_current_source_preserves_physical_differential_solution() {
    let netlist = parse(
        "internal source on floating resistor island\n\
         vref ref 0 0\n\
         i1 a b dc 1m\n\
         r1 a b 1k\n\
         c1 a 0 1u\n\
         c2 b 0 1u\n\
         .op\n\
         .end\n",
    );

    let result = engine()
        .run_dc_op(&netlist)
        .expect("an internal current source does not drive component common mode");
    let differential = voltage(&result, "a") - voltage(&result, "b");
    assert!(
        (differential.abs() - 1.0).abs() <= 1.0e-9,
        "1 mA through 1 kOhm must establish a 1 V differential, got {differential:.17e}"
    );
}

#[test]
fn zero_dc_and_ac_only_current_sources_preserve_warning_only_behavior() {
    for source in ["i1 0 out dc 0", "i1 0 out ac 1"] {
        let netlist = parse(&format!(
            "inactive DC current source\n\
             vref ref 0 0\n\
             {source}\n\
             c1 out 0 1u\n\
             .op\n\
             .end\n"
        ));
        let result = engine()
            .run_dc_op(&netlist)
            .unwrap_or_else(|error| panic!("{source} must not be a fatal DC drive: {error}"));
        assert_eq!(voltage(&result, "out"), 0.0);
    }
}

#[test]
fn installed_zero_waveforms_and_zero_gain_sources_are_warning_only() {
    for source in [
        "i1 0 out pat(1 0 0 1n 1n 1n b0)",
        "g1 0 out ref 0 0",
        "f1 0 out vref 0",
        "b1 0 out i=0",
    ] {
        let netlist = parse(&format!(
            "zero installed current source\n\
             vref ref 0 0\n\
             {source}\n\
             c1 out 0 1u\n\
             .op\n\
             .end\n"
        ));
        let result = engine()
            .run_dc_op(&netlist)
            .unwrap_or_else(|error| panic!("{source} must have zero net DC drive: {error}"));
        assert_eq!(voltage(&result, "out"), 0.0);
    }
}

#[test]
fn loaded_zero_pwl_current_uses_the_installed_snapshot() {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time after epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "rspice-zero-current-{}-{unique}.csv",
        std::process::id()
    ));
    std::fs::write(&path, "0,0\n1,1\n").expect("write PWL fixture");
    let path_text = path.to_string_lossy().replace('\\', "/");
    let netlist = parse(&format!(
        "zero loaded PWL current\n\
         vref ref 0 0\n\
         i1 0 out pwl file=\"{path_text}\"\n\
         c1 out 0 1u\n\
         .op\n\
         .end\n"
    ));
    let result = engine().run_dc_op(&netlist);
    std::fs::remove_file(&path).expect("remove PWL fixture");
    let result = result.expect("the loaded t=0 sample is the authoritative DC current");
    assert_eq!(voltage(&result, "out"), 0.0);
}

#[test]
fn nonzero_controlled_current_is_rejected_from_its_accepted_bias() {
    for source in ["g1 0 out ref 0 1m", "b1 0 out i=1m"] {
        let netlist = parse(&format!(
            "controlled current drives a floating component\n\
             vref ref 0 1\n\
             {source}\n\
             c1 out 0 1u\n\
             .op\n\
             .end\n"
        ));
        let error = engine()
            .run_dc_op(&netlist)
            .expect_err("a nonzero controlled source must not publish a shunt-dependent bias");
        assert!(
            error.to_string().contains("no DC path to ground"),
            "unexpected {source} error: {error}"
        );
    }
}

#[test]
fn dc_sweep_reclassifies_the_installed_current_at_every_point() {
    let netlist = parse(
        "swept source becomes a floating DC drive\n\
         vref ref 0 0\n\
         i1 0 out dc 0\n\
         c1 out 0 1u\n\
         .dc i1 0 1m 1m\n\
         .end\n",
    );
    let error = engine()
        .run_dc_sweep(&netlist, "i1", 0.0, 1.0e-3, 1.0e-3)
        .expect_err("the nonzero sweep point must not reuse zero-source classification");
    assert!(
        error.to_string().contains("no DC path to ground"),
        "unexpected sweep error: {error}"
    );
}

#[test]
fn every_ac_bias_uses_the_shared_floating_component_policy() {
    let netlist = parse(
        "AC analysis still requires a physical DC bias\n\
         i1 0 out dc 1m ac 1\n\
         c1 out 0 1u\n\
         .ac lin 1 1k 1k\n\
         .end\n",
    );
    let error = engine()
        .run_ac(&netlist, &[1.0e3])
        .expect_err("AC must refuse the same current-driven floating DC bias as .OP");
    assert!(
        error.to_string().contains("no DC path to ground"),
        "unexpected AC startup error: {error}"
    );
}

#[test]
fn rshunt_supplies_the_shared_ac_bias_path() {
    let netlist = parse(
        "RSHUNT physically defines the AC operating point\n\
         i1 0 out dc 1m ac 1\n\
         c1 out 0 1u\n\
         .options rshunt=1g\n\
         .ac lin 1 1k 1k\n\
         .end\n",
    );
    let results = engine()
        .run_ac(&netlist, &[1.0e3])
        .expect("RSHUNT must permit every analysis that consumes the DC bias");
    let point = &results[0];
    let out = point
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("OUT node is present");
    // At 1 kHz the capacitor is in parallel, so this deck primarily checks
    // shared bias admission. A separate pure-RSHUNT point below pins the AC
    // transimpedance exactly.
    assert!(point.voltages[out].norm().is_finite());
}

#[test]
fn rshunt_is_a_physical_ac_transimpedance_without_a_synthetic_device() {
    let netlist = parse(
        "RSHUNT-only AC transimpedance\n\
         i1 0 out dc 0 ac 1\n\
         .ac lin 1 1k 1k\n\
         .end\n",
    );
    let engine = Engine::new(SimulationConfig {
        rshunt: Some(1.0e3),
        ..Default::default()
    });
    let circuit = engine.build_circuit(&netlist).expect("circuit builds");
    assert_eq!(
        circuit.global_shunt_conductance().to_bits(),
        1.0e-3_f64.to_bits()
    );
    assert!(circuit.has_global_shunt());
    assert!(circuit.no_dc_path_nodes().is_empty());
    assert!(circuit.fatal_no_dc_path_nodes().is_empty());
    assert!(
        circuit
            .branch_names_sorted()
            .iter()
            .all(|name| !name.to_ascii_uppercase().contains("RSHUNT")),
        "RSHUNT must not create a synthetic output-visible resistor"
    );

    let results = engine.run_ac(&netlist, &[1.0e3]).expect("AC solves");
    let point = &results[0];
    let out = point
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("OUT node is present");
    let transimpedance = point.voltages[out].norm();
    assert!(
        (transimpedance - 1.0e3).abs() <= 2.0e-9,
        "expected 1 kOhm AC transimpedance, got {transimpedance:.17e}"
    );
}

#[test]
fn rshunt_is_stamped_during_uic_transient_without_an_operating_point() {
    let netlist = parse(
        "RSHUNT in UIC transient\n\
         i1 0 out 1m\n\
         c1 out 0 1u\n\
         .options rshunt=1k\n\
         .tran 10u 100u uic\n\
         .end\n",
    );
    let result = engine()
        .run_tran(&netlist, 100.0e-6, 10.0e-6)
        .expect("UIC transient with RSHUNT solves");
    let out = result
        .try_voltage_waveform_named("out")
        .expect("OUT waveform is present");
    assert_eq!(out[0], 0.0);
    assert!(out.iter().all(|value| value.is_finite()));
    assert!(out.last().is_some_and(|value| *value > 0.0 && *value < 1.0));
}

#[test]
fn invalid_programmatic_rshunt_values_fail_configuration_validation() {
    for invalid in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::from_bits(1)] {
        let config = SimulationConfig {
            rshunt: Some(invalid),
            ..Default::default()
        };
        let error = match Engine::try_new(config) {
            Ok(_) => panic!("invalid RSHUNT must fail closed"),
            Err(error) => error,
        };
        assert!(
            error.to_string().contains("rshunt"),
            "unexpected error for {invalid:?}: {error}"
        );
    }
}

#[test]
fn non_uic_transient_uses_dc_policy_while_uic_deliberately_bypasses_it() {
    let ordinary = parse(
        "ordinary transient asks for an operating point\n\
         i1 0 out dc 1m\n\
         c1 out 0 1u\n\
         .tran 1u 2u\n\
         .end\n",
    );
    let error = engine()
        .run_tran(&ordinary, 2.0e-6, 1.0e-6)
        .expect_err("non-UIC transient must enforce the shared DC policy");
    assert!(
        error.to_string().contains("no DC path to ground"),
        "unexpected transient startup error: {error}"
    );

    let uic = parse(
        "UIC transient intentionally skips the operating point\n\
         i1 0 out dc 1m\n\
         c1 out 0 1u\n\
         .tran 1u 2u uic\n\
         .end\n",
    );
    let result = engine()
        .run_tran(&uic, 2.0e-6, 1.0e-6)
        .expect("UIC must remain exempt because it requests no operating point");
    assert!(
        result
            .try_voltage_waveform_named("out")
            .is_some_and(|waveform| waveform.iter().all(|value| value.is_finite()))
    );

    let inactive_at_t0 = parse(
        "DC source value must not override the t0 waveform contract\n\
         i1 0 out dc 1m pulse(0 0 0 1n 1n 1u 2u)\n\
         c1 out 0 1u\n\
         .tran 1n 2n\n\
         .end\n",
    );
    let result = engine()
        .run_tran(&inactive_at_t0, 2.0e-9, 1.0e-9)
        .expect("a zero t=0 waveform must not be rejected from its separate DC value");
    assert_eq!(
        result
            .try_voltage_waveform_named("out")
            .expect("OUT waveform exists")[0],
        0.0
    );
}

#[test]
fn transient_bias_audit_uses_the_t0_waveform_instead_of_the_dc_source_value() {
    let floating = parse(
        "t0 waveform drives a floating transient operating point\n\
         i1 0 out dc 0 pulse(1m 1m 0 1n 1n 1u 2u)\n\
         c1 out 0 1u\n\
         .tran 1n 2n\n\
         .end\n",
    );
    let error = engine()
        .run_tran(&floating, 2.0e-9, 1.0e-9)
        .expect_err("the nonzero t=0 waveform must not publish a GMIN-defined bias");
    assert!(
        error.to_string().contains("no DC path to ground"),
        "unexpected transient startup error: {error}"
    );

    let grounded = parse(
        "t0 waveform has a physical transient bias path\n\
         i1 0 out dc 0 pulse(1m 1m 0 1n 1n 1u 2u)\n\
         r1 out 0 1k\n\
         c1 out 0 1u\n\
         .tran 1n 2n\n\
         .end\n",
    );
    let result = engine()
        .run_tran(&grounded, 2.0e-9, 1.0e-9)
        .expect("a physical resistor must admit the same t=0 waveform");
    assert!(
        result
            .try_voltage_waveform_named("out")
            .is_some_and(|waveform| waveform.iter().all(|value| value.is_finite()))
    );
}

#[test]
fn capacitor_initial_condition_dc_constraint_is_dialect_exact() {
    let netlist = parse(
        "capacitor IC operating-point dialect contract\n\
         i1 0 out 1m\n\
         c1 out 0 1u ic=0\n\
         .op\n\
         .end\n",
    );

    let xyce = Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
    let xyce_circuit = xyce.build_circuit(&netlist).expect("Xyce circuit builds");
    assert!(xyce_circuit.no_dc_path_nodes().is_empty());
    assert!(xyce_circuit.fatal_no_dc_path_nodes().is_empty());
    let xyce_result = xyce
        .run_dc_op(&netlist)
        .expect("Xyce IC branch supplies the exact DC voltage constraint");
    assert_eq!(voltage(&xyce_result, "out"), 0.0);

    for dialect in [SpiceDialect::Ngspice, SpiceDialect::BestAvailable] {
        let engine = Engine::new(SimulationConfig::default().with_spice_dialect(dialect));
        let circuit = engine.build_circuit(&netlist).expect("circuit builds");
        assert_eq!(circuit.no_dc_path_nodes(), ["OUT"]);
        assert_eq!(circuit.fatal_no_dc_path_nodes(), ["OUT"]);
        let error = engine
            .run_dc_op(&netlist)
            .expect_err("transient-only capacitor IC must not invent a DC constraint");
        assert!(
            error.to_string().contains("no DC path to ground"),
            "unexpected {dialect:?} error: {error}"
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn inductor_initial_current_uses_the_selected_dc_dialect() {
    // Xyce 7.10: V(winding)=1.002 V and I(L1)=-.0002 A.
    // ngspice 46: V(winding)=0 V and I(L1)=.1 A.
    let deck =
        parse("DC winding IC\nV1 in 0 1\nR1 in winding 10\nL1 winding 0 5n IC=-.0002\n.end\n");
    for dialect in [
        SpiceDialect::Xyce,
        SpiceDialect::Ngspice,
        SpiceDialect::BestAvailable,
    ] {
        let engine = Engine::new(SimulationConfig::default().with_spice_dialect(dialect));
        let result = engine.run_dc_op(&deck).unwrap();
        let branch = result
            .branch_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("l1"))
            .unwrap();
        let expected_current = if dialect == SpiceDialect::Xyce {
            -0.0002
        } else {
            0.1
        };
        let circuit = engine.build_circuit(&deck).unwrap();
        let mut triplets = circuit.create_matrix();
        let mut rhs = circuit.create_rhs();
        circuit.stamp_dc(&mut triplets, &mut rhs);
        let mut triplet_solution = Vec::new();
        let entries: Vec<_> = triplets.entries().collect();
        StaticMatrix::from_triplets(triplets.nrows(), triplets.ncols(), &entries)
            .unwrap()
            .solve_into(&rhs, &mut triplet_solution)
            .unwrap();
        assert!((triplet_solution[circuit.num_nodes() + branch] - expected_current).abs() < 1e-14);
        assert!(
            (result.branch_currents[branch] - expected_current).abs() < 1e-14,
            "{dialect:?}: I(L1)={:e}, expected {expected_current:e}",
            result.branch_currents[branch]
        );
        assert!((voltage(&result, "winding") - (1.0 - 10.0 * expected_current)).abs() < 1e-12);
        for (source, point) in engine.run_dc_sweep(&deck, "V1", 0.0, 2.0, 0.5).unwrap() {
            let expected_current = if dialect == SpiceDialect::Xyce {
                -0.0002
            } else {
                source / 10.0
            };
            assert!((point.branch_currents[branch] - expected_current).abs() < 1e-14);
            assert!(
                (voltage(&point, "winding") - (source - 10.0 * expected_current)).abs() < 1e-12
            );
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn xyce_winding_ic_requires_a_physical_current_balance() {
    let engine = Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
    let inconsistent = parse(
        "Inconsistent DC winding\nI1 0 out .00025\nL1 out 0 5n IC=.0002\n.options GMIN=0\n.end\n",
    );
    assert_eq!(
        engine
            .build_circuit(&inconsistent)
            .unwrap()
            .no_dc_path_nodes(),
        ["OUT"]
    );
    assert!(engine.run_dc_op(&inconsistent).is_err());

    // RSHUNT is authored conductance, so it may carry the difference.
    let shunted = parse(
        "Shunted DC winding\nI1 0 out .00025\nL1 out 0 5n IC=.0002\n.options GMIN=0 RSHUNT=1k\n.end\n",
    );
    let point = engine.run_dc_op(&shunted).unwrap();
    assert!((voltage(&point, "out") - 0.05).abs() < 1e-12);
    assert!((point.branch_currents[0] - 0.0002).abs() < 1e-14);
    let transient = engine.run_tran(&shunted, 20e-12, 1e-12).unwrap();
    assert!((transient.try_voltage_waveform_named("out").unwrap()[0] - 0.05).abs() < 1e-12);
    assert!((transient.try_branch_current_waveform_named("l1").unwrap()[0] - 0.0002).abs() < 1e-14);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn xyce_winding_ic_participates_in_nonlinear_dc_bias() {
    let engine = Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
    let deck = parse(
        "Nonlinear DC winding\nI1 0 out .00025\nL1 out 0 5n IC=.0002\nD1 out 0 diode\n.model diode D(IS=1e-14)\n.options GMIN=0\n.end\n",
    );
    let point = engine.run_dc_op(&deck).unwrap();
    assert!((point.branch_currents[0] - 0.0002).abs() < 1e-14);
    // Xyce 7.10 independent bias solution at the default 27 C.
    assert!((voltage(&point, "out") - 0.577617386).abs() < 1e-6);
}

/// QSPICE 2026 batch `.op`: its DC solve gives every inductor 1 pOhm, so a
/// loop through a winding solves. 1 A into L1 || (L2 + L3) splits 2/3 and 1/3
/// with V(a) = 6.666666666666666e-13 and V(b) = 3.333333333333333e-13; 1 V
/// across 1 uH draws I(L1) = 1e12 and I(V1) = -1e12; 1 V and 2 V joined by
/// 1 uH give I(L1) = -1e12, I(V2) = -1e12 and I(V1) = 9.99999999999999e11;
/// `.dc V1 0 1 0.5` over 1 uH || 3 uH gives each V1 / 1e-12. Read the native
/// way, RSpice gives that resistance to the windings that close a loop and to
/// no other, and warns naming them: a winding outside every loop stays a
/// short. LTspice differs by its default 1 mOhm series resistance, which it
/// gives every inductor in every analysis.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn a_loop_through_windings_solves_with_each_winding_a_picoohm_at_dc() {
    let close = |label: &str, value: f64, expected: f64| {
        assert!(
            (value - expected).abs() <= 1e-12 * expected.abs(),
            "{label} = {value:e}, QSPICE {expected:e}"
        );
    };
    let current = |result: &rspice_core::solver::SimulationResult, name: &str| {
        result
            .branch_current_named(name)
            .unwrap_or_else(|| panic!("I({name}) is solved"))
    };

    let (point, warnings) = warnings_of(|| {
        engine().run_dc_op(&parse(
            "inductor loop\nI1 0 a 1\nL1 a 0 16m\nL2 a b 97.5m\nL3 b 0 16m\nR1 a 0 10k\n\
             I2 0 c 1\nL4 c 0 1u\nR2 c 0 1k\n.end\n",
        ))
    });
    let point = point.expect("a loop through windings solves");
    close("V(a)", voltage(&point, "a"), 6.666666666666666e-13);
    close("V(b)", voltage(&point, "b"), 3.333333333333333e-13);
    close("I(L1)", current(&point, "L1"), 2.0 / 3.0);
    close("I(L2)", current(&point, "L2"), 1.0 / 3.0);
    close("I(L3)", current(&point, "L3"), 1.0 / 3.0);
    assert_eq!(voltage(&point, "c"), 0.0, "L4 closes no loop");
    close("I(L4)", current(&point, "L4"), 1.0);
    assert!(
        warnings.iter().any(|line| {
            let line = line.to_ascii_lowercase();
            line.contains("inductor(s) l1, l2, l3 as 1e-12 ohm")
                && line.contains("close the loop l1, l2, l3")
        }),
        "{warnings:?}"
    );
    assert!(
        !warnings
            .iter()
            .any(|line| line.to_ascii_lowercase().contains("l4")),
        "{warnings:?}"
    );

    let point = engine()
        .run_dc_op(&parse("V across L\nV1 a 0 1\nL1 a 0 1u\n.end\n"))
        .expect("a source across a winding solves");
    close("I(L1)", current(&point, "L1"), 1e12);
    close("I(V1)", current(&point, "V1"), -1e12);

    let point = engine()
        .run_dc_op(&parse(
            "V L V\nV1 a 0 1\nL1 a b 1u\nV2 b 0 2\nR1 a 0 1k\n.end\n",
        ))
        .expect("a winding between two sources solves");
    close("I(L1)", current(&point, "L1"), -1e12);
    close("I(V2)", current(&point, "V2"), -1e12);
    close("I(V1)", current(&point, "V1"), 9.99999999999999e11);

    let sweep = engine()
        .run_dc_sweep(
            &parse("L loop under a sweep\nV1 a 0 1\nL1 a 0 1u\nL2 a 0 3u\n.end\n"),
            "V1",
            0.5,
            1.0,
            0.5,
        )
        .expect("every point of the sweep solves");
    assert_eq!(sweep.len(), 2);
    for (source, point) in &sweep {
        close("I(L1)", current(point, "L1"), source / 1e-12);
        close("I(L2)", current(point, "L2"), source / 1e-12);
    }

    let circuit = engine()
        .build_circuit(&parse("V across L\nV1 a 0 1\nL1 a 0 1u\n.end\n"))
        .unwrap();
    assert!(circuit.dc_branch_voltage_loops().is_empty());
}

/// Read the ngspice way, a loop of voltage sources and inductors is singular
/// in every DC solve: the DC stamp shorts each winding, the loop's branch rows
/// sum to a constant, and neither GMIN nor RSHUNT stamps a branch row. A loop
/// of voltage sources alone is singular read any way. The circuit names each
/// loop it refuses, and a winding the solve holds at its `IC=` is not part of
/// one.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn a_loop_through_dc_shorted_windings_is_refused_by_every_dc_solve() {
    let cases: [(&str, &[&str], bool); 4] = [
        (
            "V across L\nV1 a 0 1\nL1 a 0 1u\n.end\n",
            &["V1", "L1"],
            true,
        ),
        (
            "V L V\nV1 a 0 1\nL1 a b 1u\nV2 b 0 1\n.end\n",
            &["V1", "V2", "L1"],
            true,
        ),
        (
            "Shunted V L V\nV1 a 0 1\nL1 a b 1u\nV2 b 0 2\nR1 a 0 1k\n.options RSHUNT=1k\n.end\n",
            &["V1", "V2", "L1"],
            true,
        ),
        (
            "V across V\nV1 a 0 1\nV2 a 0 1\n.end\n",
            &["V1", "V2"],
            false,
        ),
    ];
    for (deck, members, through_inductor) in cases {
        let netlist = parse(deck);
        let engine = if through_inductor {
            Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Ngspice))
        } else {
            engine()
        };
        engine
            .run_dc_op(&netlist)
            .expect_err("a loop of fixed branch voltages has no unique DC solution");
        engine
            .run_dc_sweep(&netlist, "V1", 0.0, 1.0, 0.5)
            .expect_err("no point of a DC sweep has a unique solution either");
        let loops = engine
            .build_circuit(&netlist)
            .expect("the loop is refused by the solve, not by construction")
            .dc_branch_voltage_loops();
        assert_eq!(loops.len(), 1, "{deck}: {loops:?}");
        let named: Vec<String> = loops[0]
            .members
            .iter()
            .map(|member| member.to_ascii_uppercase())
            .collect();
        assert_eq!(named, members, "{deck}");
        assert_eq!(loops[0].through_inductor, through_inductor, "{deck}");
    }

    let xyce = Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce));
    xyce.run_dc_op(&parse("V across L\nV1 a 0 1\nL1 a 0 1u\n.end\n"))
        .expect_err("Xyce leaves a loop through a winding singular");
    let held = parse("Held winding\nV1 a 0 1\nL1 a 0 1u IC=1m\n.end\n");
    xyce.run_dc_op(&held)
        .expect("a winding held at its IC is a current in the DC solve");
    assert!(
        xyce.build_circuit(&held)
            .unwrap()
            .dc_branch_voltage_loops()
            .is_empty()
    );
}

/// A node only capacitors reach takes the voltage its capacitors share
/// charging up from rest, its net charge zero: 1 V over 1 pF and 3 pF in
/// series puts 0.25 V between them, at the operating point and where a
/// transient starts. QSPICE and Xyce give 0.25, ngspice-46 0.249999. Islands
/// that capacitors chain together settle together, and an island a resistor
/// joins moves as one.
#[test]
fn a_capacitive_island_takes_its_charge_neutral_voltage() {
    let netlist = parse(
        "capacitive islands
         v1 in 0 dc 1 pulse(1 2 1n 10p 10p 2n 4n)
         r1 in 0 1k
         c1 in island 1p
         c2 island 0 3p
         c3 in upper 1p
         c4 upper lower 1p
         c5 lower 0 1p
         c6 in near 1p
         r6 near far 1k
         c7 far 0 1p
         .end
",
    );
    let expected = [
        ("island", 0.25),
        ("upper", 2.0 / 3.0),
        ("lower", 1.0 / 3.0),
        ("near", 0.5),
        ("far", 0.5),
    ];
    let op = engine()
        .run_dc_op(&netlist)
        .expect("the islands' operating point solves");
    let transient = engine()
        .run_tran(&netlist, 2.0e-9, 1.0e-12)
        .expect("the transient runs");
    for (node, value) in expected {
        let settled = voltage(&op, node);
        assert!((settled - value).abs() <= 1.0e-12, "v({node}) = {settled}");
        let start = transient
            .try_voltage_at_named(node, 0)
            .unwrap_or_else(|| panic!("v({node}) is recorded"));
        assert!(
            (start - value).abs() <= 1.0e-12,
            "v({node}) at t = 0: {start}"
        );
    }
}

/// A floating pair joined by 1 mOhm: 1000 S rounds the 1e-15 S nodal floor
/// away, so its rows cancel exactly. It is still a capacitive divider, and
/// used to be refused as a singular matrix.
#[test]
fn an_island_joined_by_a_tiny_resistor_still_settles() {
    let netlist = parse(
        "island behind a series short
         v1 in 0 1
         c1 in near 1n
         rs near far 1m
         c2 far 0 1n
         .end
",
    );
    let op = engine()
        .run_dc_op(&netlist)
        .expect("the island's operating point solves");
    let transient = engine()
        .run_tran(&netlist, 1.0e-8, 1.0e-9)
        .expect("the transient runs");
    for node in ["near", "far"] {
        let settled = voltage(&op, node);
        assert!((settled - 0.5).abs() <= 1.0e-12, "v({node}) = {settled}");
        let start = transient
            .try_voltage_at_named(node, 0)
            .unwrap_or_else(|| panic!("v({node}) is recorded"));
        assert!(
            (start - 0.5).abs() <= 1.0e-12,
            "v({node}) at t = 0: {start}"
        );
    }
}

/// An initial condition on an island holds it where the condition puts it
/// for the transient's operating point; the charge-neutral voltage is for an
/// island nothing pins.
#[test]
fn an_initial_condition_holds_a_capacitive_island_at_its_value() {
    let netlist = parse(
        "capacitive island under .ic
         v1 in 0 dc 1 pulse(1 2 1n 10p 10p 2n 4n)
         r1 in 0 1k
         c1 in island 1p
         c2 island 0 3p
         .ic v(island)=0.7
         .end
",
    );
    let transient = engine()
        .run_tran(&netlist, 2.0e-9, 1.0e-12)
        .expect("the transient runs");
    let start = transient
        .try_voltage_at_named("island", 0)
        .expect("v(island) is recorded");
    assert!((start - 0.7).abs() <= 1.0e-9, "v(island) at t = 0: {start}");
}

/// The charge-neutral shift is for an island nothing else reads. A node a
/// resistor grounds keeps its DC voltage, and an island a controlled source's
/// control port or a behavioral expression reads keeps the voltage the solve
/// gave it, so what that element drives stays consistent with it.
#[test]
fn a_node_with_a_dc_path_or_a_reader_keeps_its_solved_voltage() {
    let netlist = parse(
        "capacitive nodes that are not islands
         v1 in 0 dc 1 pulse(1 2 1n 10p 10p 2n 4n)
         r1 in 0 1k
         c1 in held 1p
         r2 held 0 1meg
         c3 in sensed 1p
         c4 sensed 0 1p
         e1 copy 0 sensed 0 1
         r3 copy 0 1k
         c5 in read 1p
         c6 read 0 1p
         b1 echo 0 v=v(read)
         r4 echo 0 1k
         .end
",
    );
    let op = engine()
        .run_dc_op(&netlist)
        .expect("the operating point solves");
    let transient = engine()
        .run_tran(&netlist, 2.0e-9, 1.0e-12)
        .expect("the transient runs");
    let start = |node: &str| {
        transient
            .try_voltage_at_named(node, 0)
            .unwrap_or_else(|| panic!("v({node}) is recorded"))
    };
    for node in ["held", "sensed", "read"] {
        let solved = voltage(&op, node);
        assert!(solved.abs() <= 1.0e-12, "v({node}) = {solved}");
        let initial = start(node);
        assert!(initial.abs() <= 1.0e-12, "v({node}) at t = 0: {initial}");
    }
    assert_eq!(voltage(&op, "copy"), voltage(&op, "sensed"));
    assert_eq!(voltage(&op, "echo"), voltage(&op, "read"));
    assert_eq!(start("copy"), start("sensed"));
    assert_eq!(start("echo"), start("read"));
}
