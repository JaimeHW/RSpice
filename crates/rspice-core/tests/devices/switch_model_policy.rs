//! Build-time policy for native switch model parameters.
//!
//! Recognized SW/CSW parameters must be evaluated natively from model-card
//! expressions or rejected. They must not disappear into default switch values.

use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect};
use rspice_core::netlist::Netlist;
use std::sync::Arc;

fn run(deck: &str) -> Result<(), String> {
    let netlist = Netlist::parse(deck).expect("deck parses");
    Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .map(|_| ())
        .map_err(|err| err.to_string())
}

fn op_voltage(deck: &str, node: &str) -> f64 {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let op = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("operating point solves");
    let idx = op
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} missing from OP result {:?}", op.node_names));
    op.node_voltages[idx]
}

#[test]
fn xyce_generic_switch_control_time_expression_matches_gswitch_oracle() {
    let deck = "* Xyce generic SWITCH CONTROL expression syntax\n\
                is 0 1 pwl(0 0 4u 0.040)\n\
                vmon 1 1a 0v\n\
                r1 1a 0 100\n\
                vmon1 2 3 0\n\
                r2 3 0 100\n\
                sw1 1 2 sw off control={if(time>2u,1,0)}\n\
                .model sw SWITCH (ON=1 OFF=0 RON=1 ROFF=1E6)\n\
                .tran 0.02u 4u\n\
                .end\n";

    let netlist = Netlist::parse(deck).expect("Xyce generic SWITCH CONTROL deck parses");
    let tran = Engine::new(SimulationConfig::default())
        .run_tran(&netlist, 4.0e-6, 2.0e-8)
        .expect("native generic SWITCH CONTROL transient runs");
    let i_vmon = tran
        .try_branch_current_waveform_named("vmon")
        .expect("missing I(VMON) waveform");
    let i_vmon1 = tran
        .try_branch_current_waveform_named("vmon1")
        .expect("missing I(VMON1) waveform");

    for (time, expected_vmon, expected_vmon1) in [
        (1.82e-6, 1.819_818_04e-2, 1.819_636_07e-6),
        (2.20e-6, 1.105_47e-2, 1.094_53e-2),
        (4.0e-6, 2.009_950_25e-2, 1.990_049_75e-2),
    ] {
        let got_vmon = interpolate(&tran.time, i_vmon, time);
        let got_vmon1 = interpolate(&tran.time, i_vmon1, time);
        assert_abs_or_rel_close("Xyce generic SWITCH I(VMON)", got_vmon, expected_vmon);
        assert_abs_or_rel_close("Xyce generic SWITCH I(VMON1)", got_vmon1, expected_vmon1);
    }
}

#[test]
fn xyce_generic_switch_hysteresis_matches_gswitch_oracle() {
    let deck = "* Xyce GSWITCH hysteresis regression shape\n\
                v1 1 0 5v\n\
                r2 2 0 100\n\
                sw1 1 2 sw off control={sin((time/2u))+0.1*sin((pi*5*(time/2u)))}\n\
                .model sw SWITCH (ON=1 ONH=0.55 OFF=0 OFFH=0.25 RON=1 ROFF=100)\n\
                .tran 0 8u 0 0.08u\n\
                .end\n";

    let netlist = Netlist::parse(deck).expect("Xyce generic SWITCH hysteresis deck parses");
    let (grid, step_sizes) = xyce_gswitch_hyst_reference_schedule();
    let config = SimulationConfig {
        locked_time_grid: Some(Arc::new(grid)),
        locked_time_step_sizes: Some(Arc::new(step_sizes)),
        ..Default::default()
    };
    let tran = Engine::new(config)
        .run_tran(&netlist, 8.0e-6, 1.0e-9)
        .expect("native generic SWITCH hysteresis transient runs");
    let v2 = tran
        .try_voltage_waveform_named("2")
        .expect("missing V(2) waveform");

    for (time, expected_current) in [
        (0.0, 2.5e-2),
        (6.971_780_94e-7, 2.511_253_11e-2),
        (2.681_127_39e-6, 4.950_495_05e-2),
        (5.161_127_39e-6, 4.950_495_05e-2),
        (5.387_369_92e-6, 3.838_527_08e-2),
        (6.125_297_88e-6, 2.5e-2),
    ] {
        let got_current = interpolate(&tran.time, v2, time) / 100.0;
        assert_abs_or_rel_close(
            "Xyce generic SWITCH hysteresis current",
            got_current,
            expected_current,
        );
    }
}

/// The Xyce accepted-step sequence for `gswitchHyst1.cir`, and the step sizes
/// that produced it.
///
/// A `.prn` table records the points Xyce accepted, so the differences between
/// consecutive rows *are* its step sizes. Supplying them matters: the engine
/// reads `locked_time_step_sizes` as the caller's claim that this grid is an
/// accepted-step sequence rather than a sampling of one, and only then does it
/// refuse to subdivide. The generic switch needs that refusal — it reads its
/// hysteresis band from the store vector two accepted points back, so an extra
/// point between two reference samples silently selects the wrong band.
fn xyce_gswitch_hyst_reference_schedule() -> (Vec<f64>, Vec<f64>) {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/xyce/OutputData/GSWITCH/gswitchHyst1.cir.prn");
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    let grid: Vec<f64> = text
        .lines()
        .skip(1)
        .filter_map(|line| {
            let mut fields = line.split_whitespace();
            fields.next()?;
            fields.next()?.parse::<f64>().ok()
        })
        .collect();
    let mut steps = Vec::with_capacity(grid.len());
    let mut previous = 0.0;
    for &point in &grid {
        steps.push(point - previous);
        previous = point;
    }
    (grid, steps)
}

fn interpolate(time: &[f64], values: &[f64], target: f64) -> f64 {
    assert_eq!(time.len(), values.len());
    if target <= time[0] {
        return values[0];
    }
    for index in 1..time.len() {
        if time[index] >= target {
            let t0 = time[index - 1];
            let t1 = time[index];
            let f = if t1 > t0 {
                (target - t0) / (t1 - t0)
            } else {
                0.0
            };
            return values[index - 1] + f * (values[index] - values[index - 1]);
        }
    }
    *values.last().unwrap()
}

fn assert_abs_or_rel_close(label: &str, got: f64, expected: f64) {
    let abs = (got - expected).abs();
    let tol = 2.0e-6_f64.max(5.0e-3 * expected.abs());
    assert!(
        abs <= tol,
        "{label}: got {got:.12e}, expected {expected:.12e}, abs {abs:.3e} > tol {tol:.3e}"
    );
}

#[test]
fn vswitch_model_expressions_drive_native_switch_params() {
    let deck = "* expression-valued SW model parameters\n\
                vin in 0 dc 10\n\
                vctrl ctrl 0 dc 5\n\
                s1 in out ctrl 0 smod\n\
                rload out 0 90\n\
                .model smod SW (RONBASE=10 RON={RONBASE} ROFF={1e6} VT={1} VH=0 SMOOTH={0.01})\n\
                .op\n\
                .end\n";

    let vout = op_voltage(deck, "out");
    assert!(
        (vout - 9.0).abs() < 1e-9,
        "RON={{RONBASE}} should produce the 10 ohm / 90 ohm divider, got V(out)={vout}"
    );
}

#[test]
fn unresolved_vswitch_model_parameter_fails_closed() {
    let deck = "* unresolved SW model expression must not default\n\
                vin in 0 dc 10\n\
                vctrl ctrl 0 dc 5\n\
                s1 in out ctrl 0 smod\n\
                rload out 0 90\n\
                .model smod SW (RON={missing_ron} ROFF=1e6 VT=1)\n\
                .op\n\
                .end\n";

    let message = run(deck).expect_err("unresolved RON must fail instead of using default RON");
    assert!(
        message.contains("RON") && message.contains("could not be resolved"),
        "error should identify unresolved switch RON: {message}"
    );
}

#[test]
fn string_vswitch_model_parameter_fails_closed() {
    let deck = "* string SW model parameter must not default\n\
                vin in 0 dc 10\n\
                vctrl ctrl 0 dc 5\n\
                s1 in out ctrl 0 smod\n\
                rload out 0 90\n\
                .model smod SW (RON=\"10\" ROFF=1e6 VT=1)\n\
                .op\n\
                .end\n";

    let message = run(deck).expect_err("string RON must fail instead of using default RON");
    assert!(
        message.contains("RON") && message.contains("non-numeric"),
        "error should identify non-numeric switch RON: {message}"
    );
}

/// An OFF voltage switch and an OFF current switch, each into 1 MΩ, with
/// the model cards `models` and an optional options line. The leak through
/// each open switch is `1M/(1M + ROFF)`. The controls sit far below every
/// threshold, so the native law's smooth transition contributes nothing.
fn open_switch_leak(
    dialect: SpiceDialect,
    models: &str,
    options: &str,
) -> Result<[f64; 2], String> {
    let deck = format!(
        "* default switch off resistance\n\
         v1 in 0 1\n\
         s1 in a c 0 sw1\n\
         r1 a 0 1meg\n\
         vc c 0 -10\n\
         w1 in b vsense csw1\n\
         r2 b 0 1meg\n\
         is 0 x dc -1\n\
         vsense x 0 0\n\
         {models}\n\
         {options}\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let op = Engine::new(SimulationConfig {
        spice_dialect: dialect,
        ..Default::default()
    })
    .run_dc_op(&netlist)
    .map_err(|error| error.to_string())?;
    let voltage = |node: &str| {
        let index = op
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case(node))
            .unwrap_or_else(|| panic!("node {node} missing from {:?}", op.node_names));
        op.node_voltages[index]
    };
    Ok([voltage("a"), voltage("b")])
}

fn assert_leak(label: &str, leaks: [f64; 2], roff: f64) {
    let expected = 1.0e6 / (1.0e6 + roff);
    for (switch, leak) in ["S", "W"].into_iter().zip(leaks) {
        let relative = (leak - expected).abs() / expected;
        assert!(
            relative <= 1.0e-6,
            "{label} {switch}: open-switch leak {leak:.12e}, ROFF={roff:e} gives {expected:.12e}"
        );
    }
}

/// ngspice gives a SPICE3 `SW`/`CSW` card that names no `ROFF` the circuit
/// GMIN as its off conductance (`sw/swdefs.h:65`, `csw/cswdefs.h:66`). The
/// ngspice-46 leaks of this deck are 9.999990000010001e-07 at the default
/// GMIN and 9.990009990009990e-04 with `.OPTIONS GMIN=1e-9`. The native
/// dialect reads SPICE3 cards the same way.
#[test]
fn spice3_switch_cards_are_off_at_one_over_gmin() {
    let models = ".model sw1 sw vt=0.5\n.model csw1 csw it=0.5m";
    for dialect in [SpiceDialect::Ngspice, SpiceDialect::BestAvailable] {
        let label = format!("{dialect:?}");
        assert_leak(
            &label,
            open_switch_leak(dialect, models, "").expect("default GMIN deck solves"),
            1.0e12,
        );
        assert_leak(
            &format!("{label} GMIN=1e-9"),
            open_switch_leak(dialect, models, ".options gmin=1e-9")
                .expect("raised GMIN deck solves"),
            1.0e9,
        );
        let refusal = open_switch_leak(dialect, models, ".options gmin=0")
            .expect_err("GMIN=0 leaves a SPICE3 card no finite off resistance");
        assert!(
            refusal.contains("ROFF") && refusal.contains("GMIN"),
            "{label}: the refusal names the missing ROFF and the GMIN it would come from: {refusal}"
        );
        // An explicit ROFF is the card's own, whatever GMIN says.
        assert_leak(
            &format!("{label} explicit ROFF"),
            open_switch_leak(
                dialect,
                ".model sw1 sw vt=0.5 roff=1meg\n.model csw1 csw it=0.5m roff=1meg",
                ".options gmin=0",
            )
            .expect("an explicit ROFF needs no GMIN"),
            1.0e6,
        );
    }
}

/// Xyce's switch defaults `ROFF` to 1 MΩ (`N_DEV_SW.C:89`).
#[test]
fn xyce_switch_cards_are_off_at_one_megohm() {
    assert_leak(
        "Xyce",
        open_switch_leak(
            SpiceDialect::Xyce,
            ".model sw1 vswitch (von=1 voff=0)\n.model csw1 iswitch (ion=1m ioff=0)",
            "",
        )
        .expect("Xyce deck solves"),
        1.0e6,
    );
}

/// ngspice reads a PSpice `VSWITCH`/`ISWITCH` card through its PSpice
/// translation (`frontend/inpcompat.c`): the threshold form becomes a SPICE3
/// card with `roff=1.0e12`, the `VON`/`VOFF` form a code model with
/// `r_off=1.0e6`.
#[test]
fn pspice_switch_cards_take_ngspice_s_translation_defaults() {
    for dialect in [SpiceDialect::Ngspice, SpiceDialect::BestAvailable] {
        let label = format!("{dialect:?}");
        assert_leak(
            &format!("{label} threshold form"),
            open_switch_leak(
                dialect,
                ".model sw1 vswitch vt=0.5\n.model csw1 iswitch it=0.5m",
                "",
            )
            .expect("threshold-form deck solves"),
            1.0e12,
        );
        assert_leak(
            &format!("{label} on/off form"),
            open_switch_leak(
                dialect,
                ".model sw1 vswitch von=1 voff=0\n.model csw1 iswitch ion=1m ioff=0",
                "",
            )
            .expect("on/off-form deck solves"),
            1.0e6,
        );
    }
}
