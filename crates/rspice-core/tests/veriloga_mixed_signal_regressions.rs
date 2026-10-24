#![cfg(feature = "veriloga")]

use rspice_core::{Engine, Netlist};
use std::sync::atomic::{AtomicU64, Ordering};

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct ModelFile(std::path::PathBuf);
impl ModelFile {
    fn new(source: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "rspice_unified_audit_{}_{}.va",
            std::process::id(),
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::write(&path, source).unwrap();
        Self(path)
    }
    fn path(&self) -> String {
        self.0.display().to_string().replace('\\', "/")
    }
}
impl Drop for ModelFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

/// An authored conductance must have the same transient weighting as a native
/// resistor. Include both native and authored charge, plus an initialized idt,
/// so a model route cannot silently receive only part of the integrator rule.
#[test]
fn mixed_rc_and_integrator_follow_the_native_trapezoidal_equations() {
    use rspice_core::SimulationConfig;
    use rspice_core::engine::SpiceDialect;
    use rspice_core::numerics::integration::{IntegrationMethod, TransientErrorControl};
    use std::sync::Arc;

    let model = ModelFile::new(
        r#"
module mixed_rc(r, rc, integrated, drive);
    inout r, rc, integrated, drive;
    electrical r, rc, integrated, drive;
    reg enabled;
    initial enabled=1;
    analog begin
        I(r)<+enabled*1e-3*V(r);
        I(rc)<+enabled*1e-3*V(rc)+ddt(1e-9*V(rc));
        V(integrated)<+idt(1e6*V(drive),0.0);
    end
endmodule
"#,
    );
    let deck = Netlist::parse(&format!(
        "* identical native and mixed transient equations\n\
         Inative 0 native PWL(0 0 4u 4m)\n\
         Rnative native 0 1k\nCnative native 0 1n\n\
         Ir 0 r PWL(0 0 4u 4m)\nCr r 0 1n\n\
         Irc 0 rc PWL(0 0 4u 4m)\n\
         Vdrive drive 0 PWL(0 0 4u 4)\n\
         Iintegral 0 native_integral PWL(0 0 4u 4)\n\
         Cintegral native_integral 0 1u\n\
         X1 r rc integrated drive mixed_rc\n\
         .va \"{}\" mixed_rc\n.end\n",
        model.path()
    ))
    .unwrap();
    let grid: Arc<Vec<f64>> = Arc::new((0..=200).map(|i| i as f64 * 1e-8).collect());
    for spice_dialect in [SpiceDialect::Xyce, SpiceDialect::Ngspice] {
        let result = Engine::new(SimulationConfig {
            spice_dialect,
            integration_method: IntegrationMethod::Trapezoidal,
            // On this fixed grid, make order promotion independent of the
            // voltage LTE estimate so this check actually reaches order two.
            transient_error_control: TransientErrorControl::NonlinearIterations,
            locked_time_grid: Some(grid.clone()),
            ..Default::default()
        })
        .run_tran(&deck, 2e-6, 1e-8)
        .unwrap_or_else(|error| panic!("{spice_dialect:?}: {error}"));
        assert_eq!(result.time, *grid);
        let values = |name: &str| {
            &result.voltages[result
                .node_names
                .iter()
                .position(|node| node.eq_ignore_ascii_case(name))
                .unwrap()]
        };
        for (actual, reference) in [
            ("r", "native"),
            ("rc", "native"),
            ("integrated", "native_integral"),
        ] {
            let worst = values(actual)
                .iter()
                .zip(values(reference))
                .map(|(a, b)| (a - b).abs())
                .fold(0.0_f64, f64::max);
            assert!(
                worst < 1e-8,
                "{spice_dialect:?}: {actual} differs from {reference} by {worst:e}"
            );
        }
        // Independent closed forms: the current ramps at 1000 A/s through
        // R=1 kohm, C=1 nF; the ideal integrator's input ramps at 1e12 V/s^2.
        // Allow the first-order startup intervals on this 10 ns grid.
        let t = *result.time.last().unwrap();
        let rc_expected = 1e6 * (t - 1e-6 * (1.0 - (-t / 1e-6).exp()));
        let rc = *values("rc").last().unwrap();
        let integrated = *values("integrated").last().unwrap();
        let integral_expected = 0.5e12 * t * t;
        assert!(
            (rc - rc_expected).abs() < 3e-4,
            "{spice_dialect:?}: RC final {rc} versus analytic {rc_expected}"
        );
        assert!(
            (integrated - integral_expected).abs() < 3e-4,
            "{spice_dialect:?}: integral final {integrated} versus analytic {integral_expected}"
        );
    }
}

#[test]
fn initial_digital_read_uses_the_solved_time_zero_voltage() {
    let model = ModelFile::new(
        "module sample(p,q); input p; electrical p; output q; reg q; initial q=(V(p)>0.999); analog I(p)<+0; endmodule",
    );
    let deck = Netlist::parse(&format!(
        "* initial sample\nV1 p 0 1\nX1 p q sample\n.va \"{}\" sample\n.end",
        model.path()
    ))
    .unwrap();
    let result = Engine::default().run_tran(&deck, 2e-9, 0.2e-9).unwrap();
    let q = result
        .node_names
        .iter()
        .position(|node| node.eq_ignore_ascii_case("q"))
        .unwrap();
    assert!(
        result.voltages[q]
            .iter()
            .all(|voltage| (*voltage - 3.3).abs() < 1e-8),
        "{:?}",
        result.voltages[q]
    );
}

#[test]
fn scheduled_digital_read_uses_analog_value_at_its_own_time() {
    let model = ModelFile::new(
        r#"
module sample_at_ten(p,q);
    input p; electrical p;
    output q; reg q;
    initial begin q=0; #10 q=(V(p)>0.999); end
    analog I(p)<+0;
endmodule
"#,
    );
    let deck = Netlist::parse(&format!("* sample a known ramp at 10 ns\nV1 p 0 PWL(0 0 20n 2)\nX1 p q sample_at_ten\n.va \"{}\" sample_at_ten\n.end\n", model.path())).unwrap();
    let result = Engine::default().run_tran(&deck, 12e-9, 0.2e-9).unwrap();
    let p = result
        .node_names
        .iter()
        .position(|s| s.eq_ignore_ascii_case("p"))
        .unwrap();
    let q = result
        .node_names
        .iter()
        .position(|s| s.eq_ignore_ascii_case("q"))
        .unwrap();
    for (index, time) in result
        .time
        .iter()
        .enumerate()
        .filter(|(_, time)| **time >= 9.5e-9 && **time <= 10.5e-9)
    {
        println!(
            "scheduled sample: t={time:e}, p={}, q={}",
            result.voltages[p][index], result.voltages[q][index]
        );
    }
    assert!(
        *result.voltages[q].last().unwrap() > 3.0,
        "V(p,10 ns)=1 V, so the sampled comparison must be true; final q={}",
        result.voltages[q].last().unwrap()
    );
}

#[test]
fn digital_variable_drives_analog_equation() {
    let model = ModelFile::new(
        r#"
module digital_to_analog(out);
    output out; electrical out;
    reg state;
    initial begin state=0; #1 state=1; end
    analog V(out)<+state;
endmodule
"#,
    );
    let deck = Netlist::parse(&format!(
        "* shared variable\nX1 out digital_to_analog\n.va \"{}\" digital_to_analog\n.end\n",
        model.path()
    ))
    .unwrap();
    let result = Engine::default().run_tran(&deck, 3e-9, 0.2e-9).unwrap();
    let out = result
        .node_names
        .iter()
        .position(|s| s.eq_ignore_ascii_case("out"))
        .unwrap();
    assert!(
        (*result.voltages[out].last().unwrap() - 1.0).abs() < 1e-9,
        "digital state=1 must produce 1 V, got {}",
        result.voltages[out].last().unwrap()
    );
}

#[test]
fn digital_switch_branches_preserve_both_source_modes() {
    let model = ModelFile::new(
        "`timescale 1ns/1ps
        module switched(p); inout p; electrical p; reg state;
        initial begin state=0; #1 state=1; #1 state=0; end
        analog if(state) V(p)<+2*I(p); else I(p)<+3*V(p); endmodule",
    );
    let deck = Netlist::parse(&format!(
        "* digital switch modes\nI1 0 out DC 1\nX1 out switched\n.va \"{}\" switched\n.end",
        model.path()
    ))
    .unwrap();
    for spice_dialect in [
        rspice_core::engine::SpiceDialect::Ngspice,
        rspice_core::engine::SpiceDialect::Xyce,
    ] {
        let engine = Engine::new(rspice_core::SimulationConfig {
            spice_dialect,
            ..Default::default()
        });
        let result = engine
            .run_tran(&deck, 3e-9, 0.1e-9)
            .unwrap_or_else(|error| panic!("{spice_dialect:?}: {error}"));
        let out = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap();
        // `state` steps at 1 ns and 2 ns, and each step acts from the first
        // instant after its edge: the edge's own timepoint solves the mode
        // that held up to it.
        for (time, voltage) in result.time.iter().zip(&result.voltages[out]) {
            let expected = if *time > 1e-9 + 1e-18 && *time <= 2e-9 + 1e-18 {
                2.0
            } else {
                1.0 / 3.0
            };
            assert!(
                (voltage - expected).abs() < 1e-8,
                "{spice_dialect:?}: t={time}, V={voltage}, expected {expected}"
            );
        }
    }
}

#[test]
fn digital_state_controls_analog_conductance_and_its_jacobian() {
    let model = ModelFile::new(
        "module switched(p); inout p; electrical p; reg state; initial begin state=0; #1 state=1; end analog if(state) I(p)<+V(p)/1000; endmodule",
    );
    let deck = Netlist::parse(&format!("* state-dependent conductance\nV1 vdd 0 1\nR1 vdd p 1000\nX1 p switched\n.va \"{}\" switched\n.end", model.path())).unwrap();
    let result = Engine::default().run_tran(&deck, 3e-9, 0.2e-9).unwrap();
    let p = result
        .node_names
        .iter()
        .position(|node| node.eq_ignore_ascii_case("p"))
        .unwrap();
    // `state` steps at 1 ns and acts from the first instant after it: the
    // edge's own timepoint solves the open circuit that held up to it.
    for (time, voltage) in result.time.iter().zip(&result.voltages[p]) {
        let expected = if *time <= 1e-9 + 1e-18 { 1.0 } else { 0.5 };
        assert!((voltage - expected).abs() < 1e-7, "t={time}, p={voltage}");
    }
}

#[test]
fn mixed_instance_parameters_are_executable() {
    let model = ModelFile::new(
        r#"
module param_device(p,q);
    inout p; electrical p;
    output q; reg q;
    parameter real resistance=1000 from (0:inf);
    parameter real ratio=resistance/1000;
    initial q=(ratio>2.5);
    analog I(p)<+V(p)/(resistance*($param_given(resistance)?1:2));
endmodule
"#,
    );
    let deck = Netlist::parse(&format!("* parameter support\n.param r=2000\nV1 vdd 0 1\nR1 vdd p1 1000\nR2 vdd p2 1000\nX1 p1 q1 param_device resistance={{r}}\nX2 p2 q2 param_device resistance=3000\n.va \"{}\" param_device\n.end\n", model.path())).unwrap();
    let result = Engine::default().run_tran(&deck, 2e-9, 0.2e-9).unwrap();
    for (name, expected) in [("p1", 2.0 / 3.0), ("p2", 0.75), ("q1", 0.0), ("q2", 3.3)] {
        let index = result
            .node_names
            .iter()
            .position(|node| node.eq_ignore_ascii_case(name))
            .unwrap();
        assert!(
            result.voltages[index]
                .iter()
                .all(|value| (value - expected).abs() < 1e-8),
            "{name} must be {expected}, got {:?}",
            result.voltages[index]
        );
    }
}

#[test]
fn pure_analog_and_spice_devices_share_the_solution() {
    let model = ModelFile::new(
        r#"
module va_resistor(p,n);
    inout p,n; electrical p,n;
    analog I(p,n)<+V(p,n)/1000;
endmodule
"#,
    );
    let deck = Netlist::parse(&format!("* same MNA control\nV1 in 0 2\nR1 in out 1000\nX1 out 0 va_resistor\n.va \"{}\" va_resistor\n.end\n", model.path())).unwrap();
    let result = Engine::default().run_tran(&deck, 2e-9, 0.2e-9).unwrap();
    let out = result
        .node_names
        .iter()
        .position(|s| s.eq_ignore_ascii_case("out"))
        .unwrap();
    assert!(result.voltages[out].iter().all(|v| (v - 1.0).abs() < 1e-9));
}

#[test]
fn high_impedance_output_releases_the_analog_net() {
    let model = ModelFile::new(
        r#"
module released_output(p,q);
    input p; electrical p;
    output q; reg q;
    initial q=1'bz;
    analog I(p)<+0;
endmodule
"#,
    );
    let deck = Netlist::parse(&format!("* pull up a released output\nV1 p 0 3.3\nRpull p q 1000\nX1 p q released_output\n.va \"{}\" released_output\n.end\n", model.path())).unwrap();
    let result = Engine::default().run_tran(&deck, 2e-9, 0.2e-9).unwrap();
    let q = result
        .node_names
        .iter()
        .position(|s| s.eq_ignore_ascii_case("q"))
        .unwrap();
    assert!(
        (*result.voltages[q].last().unwrap() - 3.3).abs() < 1e-6,
        "released output with 1 kohm pull-up must reach 3.3 V; final q={}",
        result.voltages[q].last().unwrap()
    );
}

#[test]
fn output_release_and_unknown_drive_are_distinct_across_timesteps() {
    let model = ModelFile::new(
        "module tristate(p,q); input p; electrical p; output q; reg q; initial begin q=0; #1 q=1'bz; #1 q=1'bx; #1 q=1; end analog I(p)<+0; endmodule",
    );
    let deck = Netlist::parse(&format!("* tri-state sequence\nV1 p 0 3.3\nRpull p q 1000\nX1 p q tristate\n.va \"{}\" tristate\n.end", model.path())).unwrap();
    let result = Engine::default().run_tran(&deck, 4e-9, 0.2e-9).unwrap();
    let q = result
        .node_names
        .iter()
        .position(|node| node.eq_ignore_ascii_case("q"))
        .unwrap();
    // Each drive, its impedance included, acts from the first instant after
    // the timepoint that set it: the point on an edge holds the drive before it.
    for (time, voltage) in result.time.iter().zip(&result.voltages[q]) {
        let expected = if *time <= 1e-9 + 1e-18 {
            3.3 * 20.0 / 1020.0
        } else if *time <= 2e-9 + 1e-18 {
            3.3
        } else if *time <= 3e-9 + 1e-18 {
            (3.3 * 20.0 + 1.65 * 1000.0) / 1020.0
        } else {
            3.3
        };
        assert!(
            (voltage - expected).abs() < 1e-7,
            "t={time}, expected={expected}, q={voltage}"
        );
    }
}

#[test]
fn crossing_does_not_execute_an_unrelated_future_timer_early() {
    let model = ModelFile::new(
        r#"
module future_timer(p,clk,q,edge_seen,delayed);
    input p; electrical p;
    input clk; wire clk;
    output q; reg q;
    output edge_seen,delayed; reg edge_seen,delayed;
    reg seen;
    initial begin q=0; #4 q=1; end
    initial begin seen=0; edge_seen=0; delayed=0; end
    always @(posedge clk) seen<=1;
    always @(posedge seen) begin #0 edge_seen=1; #1 delayed=1; end
    analog I(p)<+0;
endmodule
"#,
    );
    let deck = Netlist::parse(&format!("* A/D crossing before independent timer\nV1 p 0 1\nVclk clk 0 PWL(0 0 3.5n 0 3.6n 3.3 6n 3.3)\nX1 p clk q edge delayed future_timer\n.va \"{}\" future_timer\n.end\n", model.path())).unwrap();
    let result = Engine::default().run_tran(&deck, 5.5e-9, 0.05e-9).unwrap();
    for (name, earliest, latest) in [
        ("edge", 3.5e-9, 3.7e-9),
        ("q", 4e-9, 4e-9),
        ("delayed", 5e-9, 5e-9),
    ] {
        let node = result
            .node_names
            .iter()
            .position(|node| node.eq_ignore_ascii_case(name))
            .unwrap();
        // A drive acts from the first instant after the timepoint that moved
        // it, so the edge is the accepted point the rise leaves from: the last
        // sample at the old level sits on it and the first above 3 V follows.
        let index = result.voltages[node]
            .iter()
            .position(|value| *value > 3.0)
            .unwrap();
        let edge = result.time[index - 1];
        assert!(
            edge >= earliest - 1e-18 && edge <= latest + 1e-18,
            "{name} rose from {edge}, reaching 3 V at {}",
            result.time[index]
        );
    }
    let q = result
        .node_names
        .iter()
        .position(|s| s.eq_ignore_ascii_case("q"))
        .unwrap();
    let early: Vec<_> = result
        .time
        .iter()
        .zip(&result.voltages[q])
        .filter(|(t, v)| **t < 4e-9 - 1e-18 && **v > 3.0)
        .collect();
    println!("timer output before 4 ns: {early:?}");
    assert!(
        early.is_empty(),
        "an unrelated #4 timer must not drive the circuit before 4 ns"
    );
}

#[test]
fn connect_rules_survive_a_verilog_include_wrapper() {
    let mut source = String::from(
        r#"
module rules_device(p,clk,q);
    input p; electrical p;
    input clk; wire clk;
    output q; wire q;
    assign q=clk;
    analog I(p)<+0;
endmodule
"#,
    );
    for (_, module) in rspice_veriloga::connect::library::BUILTIN_CONNECT_MODULES {
        source.push_str(module);
    }
    source.push_str("\nconnectrules supply_rules;\nconnect a2d #(.vsup(1.0));\nconnect d2a #(.vsup(1.0));\nendconnectrules\n");
    let direct = ModelFile::new(&source);
    let wrapper = ModelFile::new(&format!("`include \"{}\"\n", direct.path()));
    let run = |model: &ModelFile| {
        let deck = Netlist::parse(&format!("* include must preserve electrical conversion\nV1 p 0 1\nVclk clk 0 1\nX1 p clk q rules_device\n.va \"{}\" rules_device\n.end\n", model.path())).unwrap();
        let result = Engine::default().run_tran(&deck, 2e-9, 0.2e-9).unwrap();
        let q = result
            .node_names
            .iter()
            .position(|s| s.eq_ignore_ascii_case("q"))
            .unwrap();
        *result.voltages[q].last().unwrap()
    };
    let direct_q = run(&direct);
    let wrapped_q = run(&wrapper);
    println!("connect rules: direct q={direct_q}, wrapped q={wrapped_q}");
    assert!(
        (direct_q - 1.0).abs() < 1e-9,
        "direct rules must select 1 V conversion"
    );
    assert!(
        (wrapped_q - direct_q).abs() < 1e-9,
        "an include wrapper must not silently discard the selected connect rules"
    );
}

/// The R1.4 domain-edge module with a discrete half bolted on, so the builder
/// routes it to `MixedSignalHost` instead of the plain analog device.
///
/// The continuous equation is the same `ln(V(p,n)+0.1)` R1.4 pinned on the
/// analog route: defined at every accepted point of this deck and undefined at
/// the iterate plain Newton proposes when a 1 fs, -5 V edge lands inside one
/// step. The `reg`/`initial` pair is what makes the artifact's discrete plan
/// non-empty; it contributes nothing to the analog half.
const MIXED_DOMAIN_EDGE_MODEL: &str = r#"
module mixed_domain_edge(p, n, q);
    inout p, n;
    electrical p, n;
    output q;
    reg q;
    initial begin q = 0; #1 q = 1; end
    analog I(p, n) <+ 1.0e-3 * ln(V(p, n) + 0.1);
endmodule
"#;

/// A non-finite analog evaluation inside a mixed host is a rejected Newton
/// iterate, exactly as it is for a plain analog instance.
///
/// Before R1.14 the mixed route had no classification at all: `analog_error`
/// formatted every `try_stamp` failure into `MixedSignalError::Analog`, which
/// `mixed_error` widened into an ordinary `SimulationError::Circuit`, so the
/// same overshoot R1.4 taught the analog route to retry still ended the run
/// here. The two halves of the same module must not disagree about it.
#[test]
fn a_nonfinite_mixed_analog_trial_is_rejected_and_the_step_retried() {
    use rspice_core::SimulationConfig;

    let model = ModelFile::new(MIXED_DOMAIN_EDGE_MODEL);
    let deck = Netlist::parse(&format!(
        "* a Newton trial overshoots a log singularity inside a mixed host\n\
         V1 in 0 PULSE(0 -5 0 1e-15 1e-15 1 2)\n\
         R1 in p 1k\n\
         Rq q 0 1k\n\
         X1 p 0 q mixed_domain_edge\n\
         .va \"{}\" mixed_domain_edge\n\
         .end\n",
        model.path()
    ))
    .expect("parse the mixed domain-edge deck");
    let result = Engine::new(SimulationConfig {
        transient_initial_timestep: Some(1.0e-9),
        ..Default::default()
    })
    .run_tran(&deck, 3.0e-9, 1.0e-9)
    .expect("a non-finite mixed analog trial must reject the iterate, not the run");

    let trace = result.voltages[result
        .node_names
        .iter()
        .position(|node| node.eq_ignore_ascii_case("p"))
        .unwrap_or_else(|| panic!("node p in {:?}", result.node_names))]
    .clone();
    let source = |time: f64| -> f64 {
        if time >= 1.0e-15 {
            -5.0
        } else {
            -5.0 * time / 1.0e-15
        }
    };
    assert!(
        result.time.get(1).copied().unwrap_or(1.0) < 5.0e-16,
        "the rejected trial must have cut the step: {:?}",
        &result.time[..result.time.len().min(4)]
    );
    for (&time, &voltage) in result.time.iter().zip(&trace) {
        assert!(
            voltage > -0.1,
            "t={time}: accepted V(p)={voltage} is outside the module's domain"
        );
        let residual = (voltage - source(time)) * 1.0e-3 + 1.0e-3 * (voltage + 0.1).ln();
        assert!(
            residual.abs() < 1.0e-6,
            "t={time}: V(p)={voltage} leaves KCL residual {residual}"
        );
    }
    assert!(
        result.time.last().copied().unwrap_or(0.0) >= 3.0e-9 - 1.0e-18,
        "the run must reach tstop: {:?}",
        result.time.last()
    );
}

/// A mixed module whose continuous half is one conductance and whose discrete
/// half toggles one bit. Two halves, one instance, so `m=` has something to
/// scale and something to leave alone.
const MIXED_MULTIPLICITY_MODEL: &str = r#"
module mres_toggle(p, n, q);
    inout p, n;
    electrical p, n;
    output q;
    reg q;
    parameter real r = 1000.0 from (0:inf);
    initial begin q = 0; #1 q = 1; end
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#;

/// One transient run of [`MIXED_MULTIPLICITY_MODEL`] on a locked grid.
///
/// Returns the divider node the module loads and the whole trace of the node
/// its bridged output drives. The grid is locked so two runs sample the same
/// times and their digital traces can be compared point by point; the analog
/// half is DC, so its node is returned as the single value the run held.
fn mixed_multiplicity_run(model: &ModelFile, instances: &str) -> (f64, Vec<f64>) {
    use rspice_core::SimulationConfig;
    use std::sync::Arc;

    let deck = Netlist::parse(&format!(
        "* mixed instance multiplicity\n\
         V1 in 0 dc 1\n\
         Rs in a 1k\n\
         Rload q 0 1k\n\
         {instances}\
         .va \"{}\" mres_toggle\n\
         .end\n",
        model.path()
    ))
    .unwrap_or_else(|error| panic!("{instances}: {error}"));
    let grid: Arc<Vec<f64>> = Arc::new((0..=20).map(|index| index as f64 * 1e-10).collect());
    let result = Engine::new(SimulationConfig {
        locked_time_grid: Some(grid.clone()),
        ..Default::default()
    })
    .run_tran(&deck, 2e-9, 1e-10)
    .unwrap_or_else(|error| panic!("{instances}: {error}"));
    // Every grid point, and the one breakpoint the converter adds between
    // two of them: `q`'s 1 ns edge takes no time and completes a picosecond
    // later, as an XSPICE breakpoint may split a locked interval.
    let mut expected = grid.to_vec();
    expected.insert(11, 1e-9 + 1e-12);
    assert_eq!(result.time, expected);
    let trace = |name: &str| {
        result.voltages[result
            .node_names
            .iter()
            .position(|node| node.eq_ignore_ascii_case(name))
            .unwrap_or_else(|| panic!("node {name} in {:?}", result.node_names))]
        .clone()
    };
    let divider = trace("a");
    assert!(
        divider
            .iter()
            .all(|value| (value - divider[0]).abs() < 1e-12),
        "{instances}: the divider is DC and must not move: {divider:?}"
    );
    (divider[0], trace("q"))
}

/// `m=` on a mixed Verilog-AMS instance is the reserved instance parameter it
/// already is on a plain Verilog-A instance: the module's continuous half
/// stamps as m parallel copies. Its discrete half has no multiplicity, and
/// neither does the D/A bridge that carries that half onto a net.
#[test]
fn m_scales_the_analog_half_of_a_mixed_instance_and_leaves_its_digital_half_alone() {
    let model = ModelFile::new(MIXED_MULTIPLICITY_MODEL);

    let (plain, plain_q) = mixed_multiplicity_run(&model, "X1 a 0 q mres_toggle r=1k\n");
    let (multiplied, multiplied_q) =
        mixed_multiplicity_run(&model, "X1 a 0 q mres_toggle r=1k m=2\n");
    let (explicit, _) = mixed_multiplicity_run(
        &model,
        "Xa a 0 q mres_toggle r=1k\nXb a 0 qb mres_toggle r=1k\nRloadb qb 0 1k\n",
    );

    // 1 V over a 1 kohm series resistor into the module: one copy halves it,
    // two parallel copies leave a third.
    assert!((plain - 0.5).abs() < 1e-12, "1k over 1k: {plain}");
    assert!(
        (multiplied - 1.0 / 3.0).abs() < 1e-12,
        "1k over two parallel 1k: {multiplied}"
    );
    assert!(
        (multiplied - explicit).abs() < 1e-12,
        "m=2 must equal two explicit instances: {multiplied} vs {explicit}"
    );

    // The current the series resistor carries, over the voltage the module's
    // own terminals see: the module's conductance, exactly doubled.
    let conductance = |divider: f64| (1.0 - divider) / (1000.0 * divider);
    let (single, doubled) = (conductance(plain), conductance(multiplied));
    assert!(
        (doubled - 2.0 * single).abs() < 1e-12,
        "m=2 must double the mixed module's conductance: {doubled} vs 2*{single}"
    );

    // The 1 kohm load on `q` is what makes this comparison read the bridge's
    // own source conductance rather than only the level it drives, so a
    // bridge scaled by m would move the trace by about a percent.
    assert_eq!(plain_q.len(), multiplied_q.len());
    let worst = plain_q
        .iter()
        .zip(&multiplied_q)
        .map(|(plain, multiplied)| (plain - multiplied).abs())
        .fold(0.0_f64, f64::max);
    assert!(
        worst < 1e-9,
        "m must touch neither the digital trace nor its bridge: worst {worst:e}"
    );
    assert!(
        *plain_q.first().unwrap() < 0.5 && *plain_q.last().unwrap() > 1.0,
        "the discrete half must actually toggle: {plain_q:?}"
    );
}

/// An X-line `m` composes onto the instances inside the subcircuit it names,
/// and a mixed X-card is one of those instances like any other.
#[test]
fn m_reaches_a_mixed_instance_through_a_subcircuit_wrapper() {
    let model = ModelFile::new(MIXED_MULTIPLICITY_MODEL);

    let (direct, direct_q) = mixed_multiplicity_run(&model, "X1 a 0 q mres_toggle r=1k m=2\n");
    let (wrapped, wrapped_q) = mixed_multiplicity_run(
        &model,
        "Xw a 0 q wrap m=2\n.subckt wrap p n d\nX1 p n d mres_toggle r=1k\n.ends\n",
    );

    assert!(
        (wrapped - direct).abs() < 1e-12,
        "an X-line m must reach the mixed X inside it: {wrapped} vs {direct}"
    );
    let worst = direct_q
        .iter()
        .zip(&wrapped_q)
        .map(|(direct, wrapped)| (direct - wrapped).abs())
        .fold(0.0_f64, f64::max);
    assert!(
        worst < 1e-9,
        "the wrapper must not change the digital trace: worst {worst:e}"
    );
}

/// A mixed host's D/A edge is an event-driven boundary for the transient
/// recovery guards, so a deck may not be recovered differently depending on
/// whether something *else* happens to disarm them.
///
/// `is_excessive_quiet_force_candidate` and `is_stagnant_force_candidate` are
/// armed only on a circuit the engine classifies as static-source driven:
/// both reason from `max_expected_source_delta`, which sees only the analog
/// independent sources, so on a deck whose solution is stepped by a digital
/// edge they read a legitimate post-edge jump as a nonphysical quiet
/// excursion. The classification used to name XSPICE event-driven code models
/// and nothing else.
///
/// A mixed Verilog-AMS output reaches that classification through its own
/// converter. Since 02921aeeb a discrete output is not stamped by the host: it
/// is an event port behind a shared `__rspice_mixed_dac` code model, which is
/// an XSPICE event-driven device, and the first assertion pins that this is
/// what puts every volt a digital edge drives onto the load on the
/// event-driven side of the guards.
///
/// The control for the run comparison is the same deck with one inert `DC 0`
/// source added on a disconnected node. That source moves nothing and predicts
/// nothing — every analog equation, every digital edge and every expected
/// source delta in the two decks is identical — but its nodes enter
/// `voltage_lte_excluded_nodes`, which is the other half of the same
/// classification and disarms the guards. Two runs that differ by nothing but
/// that bit must recover alike.
#[test]
fn a_mixed_da_edge_recovers_the_same_whether_or_not_the_guards_are_disarmed() {
    let model = ModelFile::new(
        r#"
`include "disciplines.vams"
module edge_toggler(p, n, y);
    inout p, n;
    electrical p, n;
    output y;
    reg y;
    initial y = 1'b0;
    always #5 y = ~y;
    analog I(p, n) <+ V(p, n) / 1000000.0;
endmodule
"#,
    );
    // No analog voltage source anywhere: the D/A edge is the only thing that
    // moves this circuit, and a stiff junction clamp on the driven node is
    // what makes the post-edge Newton solve hard enough to reach the
    // force-accept recovery path at all.
    let base = format!(
        "* a mixed D/A edge into a stiff junction clamp\n\
         x1 p 0 y edge_toggler\n\
         rp p 0 1meg\n\
         rs y clamp 0.05\n\
         d1 clamp 0 stiff\n\
         c1 clamp 0 1p\n\
         .model stiff d(is=1e-20 n=0.4 rs=0 cjo=0)\n\
         .va \"{}\" edge_toggler\n",
        model.path()
    );
    let guarded = format!("{base}.end\n");
    // One source that never moves, on a node that touches nothing else.
    let disarmed = format!("{base}vinert inert 0 dc 0\nrinert inert 0 1k\n.end\n");

    let built = Engine::default()
        .build_circuit(&Netlist::parse(&guarded).unwrap())
        .unwrap();
    assert!(
        built.has_xspice_event_driven_devices(),
        "a mixed module's discrete output is bridged by an XSPICE event-driven \
         converter, so the deck is on the event-driven side of the recovery guards \
         by that classification"
    );

    let run = |deck: &str, label: &str| {
        let netlist = Netlist::parse(deck).unwrap();
        let engine = Engine::default();
        let result = engine
            .run_tran(&netlist, 60e-9, 1e-9)
            .unwrap_or_else(|error| panic!("{label}: {error}"));
        let quality = engine.convergence_quality();
        println!(
            "{label}: force_accepted_points={} timestep_reductions={} lte_rejections={} total_iterations={} points={}",
            quality.force_accepted_points,
            quality.timestep_reductions,
            quality.lte_rejections,
            quality.total_iterations,
            result.time.len()
        );
        let clamp = result
            .node_names
            .iter()
            .position(|node| node.eq_ignore_ascii_case("clamp"))
            .expect("the clamped node is reported");
        (quality, result.time.clone(), result.voltages[clamp].clone())
    };

    let (guarded_quality, guarded_time, guarded_clamp) = run(&guarded, "mixed deck");
    let (disarmed_quality, disarmed_time, disarmed_clamp) =
        run(&disarmed, "mixed deck + inert source");

    assert_eq!(
        guarded_quality.force_accepted_points, disarmed_quality.force_accepted_points,
        "a mixed D/A edge must be an event-driven boundary in its own right, so the \
         force-accept recovery path cannot depend on an unrelated inert source"
    );
    assert_eq!(
        guarded_quality.timestep_reductions, disarmed_quality.timestep_reductions,
        "the recovery timestep policy must not change with the classification either"
    );
    assert_eq!(
        guarded_time.len(),
        disarmed_time.len(),
        "an inert source must not change which timepoints are accepted"
    );
    let worst = guarded_clamp
        .iter()
        .zip(&disarmed_clamp)
        .map(|(a, b)| (a - b).abs())
        .fold(0.0_f64, f64::max);
    assert!(
        worst < 1e-9,
        "the clamped node must follow the same trace either way, worst difference {worst:e}"
    );
}

/// A D/A edge lifts a switch's control from 0.1 V to 0.91 V at 10 ns, short of
/// the 0.95 V it turns on at; a slow ramp then carries it across at 14 ns.
/// The switch's step limit (ngspice's `sw/swtrunc.c`) reads the control's
/// change across a step as a rate. With an edge that has no transition time,
/// the change across every step out of 10 ns carries the 0.81 V jump whole,
/// however short the step; read as a rate, it would ask the same tenth of
/// every retry until the step collapsed. The limit takes its reference after
/// the jump instead. With the edge a 1 ps ramp the control is continuous.
/// The step after the edge starts Newton from an extrapolation through the
/// jump, far above the band; inside the band the switch holds the state it
/// was accepted in, not that iterate's. Either way the switch turns on within
/// the 1 ns maximum step of 14 ns (ngspice-46 on the deck with the edges as
/// 1 ps PWL ramps: 14.56 ns, in 90 points), and off at the falling edge, and
/// its change of state is one step of the solution, not a run of points
/// forced across it.
#[test]
fn a_drive_step_onto_a_switch_control_is_not_read_as_a_rate() {
    const MODULE: &str = "`timescale 1ns/1ps
module edge_drive(q);
  output q; reg q;
  initial begin q = 1'b0; #10 q = 1'b1; #20 q = 1'b0; end
endmodule
";
    for ramp in [None, Some(1.0e-12)] {
        let mut source = String::from(MODULE);
        if let Some(ramp) = ramp {
            for (_, module) in rspice_veriloga::connect::library::BUILTIN_CONNECT_MODULES {
                source.push_str(module);
            }
            source.push_str(&format!(
                "\nconnectrules edge_rules;\nconnect d2a #(.vsup(1.8), .trise({ramp:e}), \
                 .tfall({ramp:e}));\nendconnectrules\n"
            ));
        }
        let model = ModelFile::new(&source);
        let deck = format!(
            "* a D/A edge onto a switch control\n.param vcc=1.8\n\
             vr r 0 pwl(0 0 40n 0.4)\nbc c 0 v = 0.45*v(q) + v(r)\n\
             s1 out 0 c 0 swh\n.model swh sw vt=0.9 vh=0.05 ron=50 roff=1e12\n\
             vdd vdd 0 1\nrl vdd out 1k\nx1 q edge_drive\n.va \"{}\" edge_drive\n.end\n",
            model.path()
        );
        let engine = Engine::default();
        let tran = engine
            .run_tran(&Netlist::parse(&deck).unwrap(), 40.0e-9, 1.0e-9)
            .unwrap_or_else(|error| panic!("ramp {ramp:?}: {error}"));
        let out = tran.try_voltage_waveform_named("out").expect("v(out)");
        let on = tran.time[out
            .iter()
            .position(|&v| v < 0.5)
            .expect("the switch turns on")];
        let edge = tran.time.partition_point(|&t| t < 10.0e-9);
        println!(
            "ramp {ramp:?}: {} points, {} timestep reductions, on at {on:e} s; from the rising edge: {:?}",
            tran.time.len(),
            engine.convergence_quality().timestep_reductions,
            &tran.time[edge..(edge + 8).min(tran.time.len())]
        );
        assert!(
            (14.0e-9..=15.0e-9).contains(&on),
            "ramp {ramp:?}: the switch turned on at {on:e} s"
        );
        let after_fall = tran
            .time
            .partition_point(|&t| t <= 30.0e-9 + ramp.unwrap_or(0.0));
        assert!(
            out[after_fall] > 0.9,
            "ramp {ramp:?}: v(out) = {} at {:e} s, after the falling edge",
            out[after_fall],
            tran.time[after_fall]
        );
        assert!(
            tran.time.len() < 150,
            "ramp {ramp:?}: {} points",
            tran.time.len()
        );
    }
}

/// A comparator output resting on its converter's threshold while switches
/// change state, as the SAR's does at the end of sampling: the top plate is
/// grounded by one switch, the output sits 2 nV below `vcc/2`, and one control
/// opens that switch and closes another that pulls the plate down, so the
/// output leaves the threshold the instant they change state. A chord across
/// that step dates the crossing at the accepted point, one solver floor past
/// it; a landing there finds the switches not yet changed, and the next step
/// changes them again at its own end. The control then closed on its
/// threshold one floor-wide landing at a time (here 40 points less than an
/// attosecond apart; in the SAR, with more going on, until the stepper
/// refused the run as pinned at the floor). The crossing belongs to the endpoint, where the step put the
/// change. The control crosses at 10 ns + ln(1.8/0.85) ns = 10.750 ns, and
/// the switch's step limit lets the step that changes it overshoot by up to
/// 50 mV of control, about 60 ps here, as ngspice's does.
#[test]
fn a_crossing_across_a_switch_change_of_state_is_dated_at_the_step() {
    let model = ModelFile::new(
        "`timescale 1ns/1ps
module watch(cmp);
  input cmp; reg seen;
  initial seen = 0;
  always @(posedge cmp) seen = 1;
endmodule
",
    );
    let deck = format!(
        "* a comparator resting on its converter threshold as switches change state\n\
         .param vcc=1.8\nvs s 0 pwl(0 1.8 10n 1.8 10.001n 0)\nrs s sd 1k\ncsd sd 0 1p\n\
         stop top 0 sd 0 swh\nspull m vneg csel 0 swh\nbsel csel 0 v = 1.8 - v(sd)\n\
         vneg vneg 0 -1\nrm m 0 1k\ncc top m 1p\n\
         bcmp cmp 0 v = 0.9*(1 - tanh(1000*v(top))) - 2n\n\
         .model swh sw vt=0.9 vh=0.05 ron=50 roff=1e12\n\
         x1 cmp watch\n.va \"{}\" watch\n.end\n",
        model.path()
    );
    let tran = Engine::default()
        .run_tran(&Netlist::parse(&deck).unwrap(), 20.0e-9, 1.0e-9)
        .unwrap_or_else(|error| panic!("{error}"));
    let cmp = tran.try_voltage_waveform_named("cmp").expect("v(cmp)");
    let rise = tran.time[cmp
        .iter()
        .position(|&v| v > 1.0)
        .expect("the comparator leaves the threshold")];
    let floor_wide = tran
        .time
        .windows(2)
        .filter(|pair| (10.7e-9..=10.9e-9).contains(&pair[1]) && pair[1] - pair[0] < 1.0e-18)
        .count();
    println!(
        "{} points, {floor_wide} floor-wide near the change, comparator up at {rise:e} s",
        tran.time.len()
    );
    assert!(
        (10.75e-9..=10.81e-9).contains(&rise),
        "the comparator left the threshold at {rise:e} s"
    );
    assert_eq!(
        floor_wide, 0,
        "steps shorter than 1 as at the switches' change of state"
    );
}

/// A zero-duration D/A edge is crossed in one step, however much the drive
/// moves the devices behind it.
///
/// C3's 555 drives its discharge transistor's base through 10 kΩ from a
/// Verilog flip-flop's output. Each edge is solved at its own timepoint with
/// the drive's left limit and completes 1 ps later. The node-activity limit
/// read the transistor's terminals across the step leaving the edge as a rate:
/// the drive's whole step is in that interval however short it is, so every
/// retry asked for a twentieth of itself, and the restart fell to the floor
/// before reaching the completion — 4 to 5 rejections an edge, and a refusal
/// once the floor was below what the clock resolves at 0.128 s under a 1 µs
/// maximum step. Here each edge's next point is its completion, the event
/// records keep the ticks' own instants, and the collector follows a run
/// under a twentieth of the maximum step.
#[test]
fn a_zero_duration_d_a_edge_into_a_transistor_is_crossed_in_one_step() {
    let module = std::env::temp_dir().join(format!(
        "rspice_unified_audit_{}_{}.v",
        std::process::id(),
        SEQUENCE.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::write(
        &module,
        "`timescale 1us/1ps\n\
         module toggle(q); output q; reg q;\n\
         \x20 initial q = 0;\n\
         \x20 always #10 q = ~q;\n\
         endmodule\n",
    )
    .unwrap();
    let model = ModelFile(module);
    let deck = Netlist::parse(&format!(
        "* zero-duration D/A edges switching a transistor\n.hdl \"{}\"\n\
         x1 q toggle\nvcc vcc 0 5\nrb q b 10k\nq1 c b 0 npn1\n.model npn1 npn\n\
         rc vcc c 1k\ncc c 0 1n\n.end\n",
        model.path()
    ))
    .unwrap();
    let run = |maximum: f64| {
        Engine::default()
            .run_tran(&deck, 45.0e-6, maximum)
            .unwrap_or_else(|error| panic!("maximum {maximum:e}: {error}"))
    };
    let tran = run(1.0e-6);
    let reference = run(0.05e-6);

    let q = tran
        .digital_traces
        .iter()
        .find(|trace| trace.node_name.to_ascii_lowercase().contains("q"))
        .unwrap_or_else(|| {
            panic!(
                "a trace of q in {:?}",
                tran.digital_traces
                    .iter()
                    .map(|trace| &trace.node_name)
                    .collect::<Vec<_>>()
            )
        });
    let edges: Vec<f64> = q
        .points
        .iter()
        .map(|point| point.time)
        .filter(|time| *time > 0.0)
        .collect();
    // Each tick's seconds are the double nearest its decimal, as a deck's
    // `10u` is; `k * 1e7 * 1e-12` rounds twice and misses three of them.
    let ticks = [1.0e-5, 2.0e-5, 3.0e-5, 4.0e-5];
    assert_eq!(edges, ticks, "each edge is recorded at its tick");

    let collector = tran.try_voltage_waveform_named("c").expect("v(c)");
    let reference_collector = reference.try_voltage_waveform_named("c").expect("v(c)");
    let at_reference = |time: f64| {
        let index = reference.time.partition_point(|sample| *sample < time);
        if reference.time[index] == time {
            return reference_collector[index];
        }
        let (t0, t1) = (reference.time[index - 1], reference.time[index]);
        let (v0, v1) = (reference_collector[index - 1], reference_collector[index]);
        v0 + (v1 - v0) * (time - t0) / (t1 - t0)
    };
    let mut worst: f64 = 0.0;
    for edge in ticks {
        let landing = tran
            .time
            .iter()
            .position(|time| *time == edge)
            .unwrap_or_else(|| panic!("a point on the edge at {edge:e}"));
        let step = tran.time[landing + 1] - edge;
        assert!(
            (step - 1.0e-12).abs() <= 1.0e-15,
            "the step leaving the edge at {edge:e} is {step:e}, not its 1 ps completion"
        );
        let around = landing.saturating_sub(2)..(landing + 6).min(tran.time.len());
        for (time, value) in tran.time[around.clone()].iter().zip(&collector[around]) {
            worst = worst.max((value - at_reference(*time)).abs());
        }
    }
    println!(
        "{} points; worst v(c) against the reference: {worst:e} V",
        tran.time.len()
    );
    assert!(worst <= 1.0e-2, "v(c) is {worst:e} V from the reference");
}

// ---------------------------------------------------------------------------
// One instant, one key
//
// Where code models share event nets with an HDL module, a discrete instant
// less than one coincidence window from an HDL tick image is that tick image
// (`xspice::event_scheduler::Instant`'s "one instant, one key"). The decks
// below put a code-model float sum, an HDL tick and a PULSE corner on one
// nanosecond, the way the sigma-delta corpus testbench does.
// ---------------------------------------------------------------------------

/// A Verilog-AMS source in a scratch file, removed when dropped.
struct AmsFile(std::path::PathBuf);
impl AmsFile {
    fn new(source: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "rspice_one_instant_{}_{}.vams",
            std::process::id(),
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::write(&path, source).unwrap();
        Self(path)
    }
    fn path(&self) -> String {
        self.0.display().to_string().replace('\\', "/")
    }
}
impl Drop for AmsFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

/// The seconds of a picosecond tick, as the HDL kernel names them.
fn picosecond(tick: u64) -> f64 {
    rspice_core::xspice::event_scheduler::TimeResolution::new(-12)
        .unwrap()
        .ticks_to_seconds(tick)
        .unwrap()
}

/// A toggle flip-flop on `clk`: `q <= #1 ~q` at every rising edge.
const TOGGLE: &str = "`include \"disciplines.vams\"\n\
`timescale 1ns/1ps\n\
module toggle(clk, q);\n\
  input clk; wire clk;\n\
  output q; reg q;\n\
  initial q = 1'b0;\n\
  always @(posedge clk) q <= #1 ~q;\n\
endmodule\n";

/// A 200 ns PULSE clock whose rising edge crosses half its level at 101 ns,
/// converted by an `adc_bridge` with its 1 ns delay onto `clk` at 102 ns; the
/// HDL toggle's `#1` puts `q` on 103 ns, and a `d_and` with a 1 ns delay puts
/// the gated clock `gclk` there too, where a `d_dff` samples `q`. Three
/// computations of one instant: the tick image `103000 + 200000 k` ps, the
/// float sum `crossing + 1n + 1n`, and the HDL's `#1` from the wake.
fn gated_clock_deck(source: &AmsFile, tstop: &str) -> Netlist {
    let text = format!(
        "one instant, one key\n\
         .hdl \"{}\"\n\
         vclk aclk 0 pulse(0 1 100n 2n 2n 98n 200n)\n\
         vone aone 0 dc 1\n\
         vzero azero 0 dc 0\n\
         abridge [aclk aone azero] [clk en dlow] adc_buff\n\
         .model adc_buff adc_bridge(in_low=0.5 in_high=0.5)\n\
         agate [clk en] gclk and1\n\
         .model and1 d_and(rise_delay=1n fall_delay=1n)\n\
         aflop q gclk dlow dlow sq sqb flop\n\
         .model flop d_dff(clk_delay=1n set_delay=1n reset_delay=1n ic=0 rise_delay=1n fall_delay=1n)\n\
         xt clk q toggle\n\
         .tran 1n {tstop}\n\
         .end\n",
        source.path()
    );
    Netlist::parse(&text).unwrap_or_else(|error| panic!("deck parses: {error}\n{text}"))
}

/// The instants and levels a digital net took after time zero.
fn edges(result: &rspice_core::engine::TransientResult, node: &str) -> Vec<(f64, bool)> {
    result
        .digital_trace_named(node)
        .unwrap_or_else(|| panic!("no digital trace `{node}`"))
        .iter()
        .filter(|point| point.time > 0.0)
        .map(|point| (point.time, point.value.state.is_high()))
        .collect()
}

/// The `#1` the HDL schedules off a code model's clock and the code model's
/// own 1 ns gate delay are one instant: on every edge the converter puts on
/// its tick image, both are the exact image a nanosecond later, and the
/// flip-flop sampling one on the other samples the same side of it on every
/// such edge rather than whichever ulp order two roundings gave. That side is
/// the old level: `q <= #1 ~q` is a nonblocking update, which lands after the
/// active events of its instant (IEEE 1364-2005 section 11.3), and every
/// code-model output is one of those.
///
/// A converter that reads its input a rounding below the threshold at the
/// crossing switches one solver floor later, off the lattice; the HDL's
/// ceiling then puts `q` a tick later, a different instant from `gclk`, as
/// the language says. Such an edge says nothing about keying and is skipped,
/// but most edges are on their image.
#[test]
fn an_hdl_delay_and_a_gated_clock_on_one_nanosecond_are_one_instant() {
    const CYCLES: u64 = 200;
    let source = AmsFile::new(TOGGLE);
    let deck = gated_clock_deck(&source, "40.2u");
    let result = Engine::default().run_tran(&deck, 40.2e-6, 10e-9).unwrap();
    let rising = |node: &str| {
        edges(&result, node)
            .into_iter()
            .filter(|(_, high)| *high)
            .collect::<Vec<_>>()
    };
    let clk = rising("clk");
    let q = edges(&result, "q");
    let gclk = rising("gclk");
    let sampled = edges(&result, "sq");
    assert!(
        [clk.len(), q.len(), gclk.len()]
            .iter()
            .all(|count| *count as u64 >= CYCLES)
    );
    let mut mixed = Vec::new();
    let mut on_image = 0;
    for k in 0..CYCLES {
        if clk[k as usize].0.to_bits() != picosecond(102_000 + 200_000 * k).to_bits() {
            continue;
        }
        on_image += 1;
        let image = picosecond(103_000 + 200_000 * k);
        let (q_time, q_level) = q[k as usize];
        assert_eq!(q_time.to_bits(), image.to_bits(), "q edge {k}: {q_time:e}");
        let (gate_time, _) = gclk[k as usize];
        assert_eq!(
            gate_time.to_bits(),
            image.to_bits(),
            "gclk edge {k}: {gate_time:e}"
        );
        // What the flop holds once it has answered this edge and before the
        // next one: the level the HDL replaced, or its new one.
        let next = picosecond(103_000 + 200_000 * (k + 1));
        let held = sampled
            .iter()
            .rev()
            .find(|(time, _)| *time > image && *time < next)
            .map_or_else(
                || {
                    sampled
                        .iter()
                        .rev()
                        .find(|(time, _)| *time <= image)
                        .is_some_and(|(_, high)| *high)
                },
                |(_, high)| *high,
            );
        if held == q_level {
            mixed.push(k);
        }
    }
    assert!(
        on_image * 2 >= CYCLES,
        "the converter put only {on_image} of {CYCLES} edges on their image"
    );
    assert!(
        mixed.is_empty(),
        "the flop sampled the new level of q on edges {mixed:?} and the old one on the rest"
    );
}

/// A checkpoint at the accepted point just before a coincident edge, and one
/// on the edge itself, both resume onto the uninterrupted run's axis and
/// events bit for bit. An image from before the format that keys coupled
/// events on the tick lattice is refused, naming the format.
#[test]
fn a_checkpoint_beside_a_coincident_edge_resumes_bit_identical() {
    use rspice_core::engine::TransientStartupMode;
    use rspice_core::engine::{TransientCheckpoint, TransientCheckpointEncoding};
    let source = AmsFile::new(TOGGLE);
    let deck = gated_clock_deck(&source, "4.2u");
    let (tstop, max_step) = (4.2e-6, 10e-9);
    let engine = Engine::default();
    assert!(
        engine
            .preflight_transient_checkpoint(&deck)
            .unwrap()
            .is_resumable()
    );
    let plain = engine.run_tran(&deck, tstop, max_step).unwrap();
    let edge = picosecond(103_000 + 200_000 * 10);
    let on_edge = plain
        .time
        .iter()
        .position(|time| *time == edge)
        .expect("an accepted point on the edge");
    let before = plain.time[on_edge - 1];

    let (full, scheduled) = engine
        .run_tran_checkpoint_schedule_with_startup_mode(
            &deck,
            tstop,
            max_step,
            TransientStartupMode::OperatingPoint,
            &[before, edge],
        )
        .unwrap();
    assert_eq!(full.time, plain.time, "checkpointing moves no point");
    assert_eq!(scheduled.len(), 2);
    let nets = ["clk", "q", "gclk", "sq"];
    for checkpoint in scheduled.iter().map(|scheduled| &scheduled.checkpoint) {
        let seam = checkpoint.time;
        assert!(seam == before || seam == edge, "a checkpoint at {seam:e}");
        let bytes = checkpoint
            .to_bytes(TransientCheckpointEncoding::Unpacked)
            .unwrap();
        let restored = TransientCheckpoint::from_bytes(&bytes).unwrap();
        let (resumed, _) = engine
            .run_tran_resume(&deck, &restored, tstop, max_step)
            .unwrap();
        let at = full.time.iter().position(|time| *time == seam).unwrap();
        let bits = |times: &[f64]| times.iter().map(|time| time.to_bits()).collect::<Vec<_>>();
        assert_eq!(
            bits(&resumed.time),
            bits(&full.time[at..]),
            "axis from {seam:e}"
        );
        for net in nets {
            let after = |result: &rspice_core::engine::TransientResult| {
                edges(result, net)
                    .into_iter()
                    .filter(|(time, _)| *time > seam)
                    .map(|(time, high)| (time.to_bits(), high))
                    .collect::<Vec<_>>()
            };
            assert_eq!(
                after(&resumed),
                after(&full),
                "`{net}` events from {seam:e}"
            );
        }

        let text = String::from_utf8(bytes).unwrap();
        let older = text
            .replacen("RSPICE-CHECKPOINT 63\n", "RSPICE-CHECKPOINT 62\n", 1)
            .lines()
            .filter(|line| !line.starts_with("floor_march "))
            .map(|line| format!("{line}\n"))
            .collect::<String>();
        let older = TransientCheckpoint::from_bytes(older.as_bytes()).expect("a format-62 image");
        let refusal = engine
            .run_tran_resume(&deck, &older, tstop, max_step)
            .expect_err("an older image of a coupled circuit is refused")
            .to_string();
        assert!(refusal.contains("format 63"), "{refusal}");
    }
}

/// An HDL edge on a tick that drives both a `transition()` output and a code
/// model's `dac_bridge` is one breakpoint: one accepted point within a
/// femtosecond of the tick, the tick's own bits, and no step after it shorter
/// than the picosecond a zero-time edge completes in.
#[test]
fn an_hdl_edge_driving_a_transition_and_a_dac_bridge_is_one_breakpoint() {
    let source = AmsFile::new(
        "`include \"disciplines.vams\"\n\
         `timescale 1ns/1ps\n\
         module drive(b, a);\n\
           output b; reg b;\n\
           output a; electrical a;\n\
           initial begin b = 1'b0; #5 b = 1'b1; end\n\
           analog V(a) <+ transition(b ? 1.0 : 0.0, 0, 1n, 1n);\n\
         endmodule\n",
    );
    let text = format!(
        "one breakpoint per tick\n\
         .hdl \"{}\"\n\
         xd b a drive\n\
         adac [b] [ad] dac1\n\
         .model dac1 dac_bridge(out_low=0 out_high=1)\n\
         rl ad 0 1k\n\
         ra a 0 1k\n\
         .tran 0.1n 20n\n\
         .end\n",
        source.path()
    );
    let deck = Netlist::parse(&text).unwrap_or_else(|error| panic!("{error}\n{text}"));
    let result = Engine::default().run_tran(&deck, 20e-9, 0.1e-9).unwrap();
    let tick = picosecond(5_000);
    let near = result
        .time
        .iter()
        .filter(|time| (**time - tick).abs() <= 1.0e-15)
        .collect::<Vec<_>>();
    assert_eq!(near, [&tick], "one point on the tick: {near:?}");
    let landing = result.time.iter().position(|time| *time == tick).unwrap();
    let shortest = result.time[landing..]
        .windows(2)
        .take_while(|pair| pair[0] < tick + 1.0e-9)
        .map(|pair| pair[1] - pair[0])
        .fold(f64::INFINITY, f64::min);
    assert!(
        shortest >= 1.0e-12 * (1.0 - 1.0e-9),
        "a {shortest:e} s step in the nanosecond after the tick"
    );
}

/// A solver floor near the precision itself moves no code-model event by
/// more than the coincidence cap, a thousandth of a tick. The converter's
/// 1.0004 ns delay puts `clk` 0.4 ps past the image of the instant it
/// converted at, inside the 0.9 ps floor a 90 ms maximum step leaves the
/// solver; a snap as wide as that floor would put the edge on the image. The
/// edge is also 0.4 ps past the PULSE corner at 102 ns, less than one floor:
/// the corner is solved at the edge, which is delivered at its own key.
#[test]
fn a_floor_near_the_precision_moves_no_coupled_event_by_more_than_the_cap() {
    let source = AmsFile::new(TOGGLE);
    let text = format!(
        "a semantic floor\n\
         .hdl \"{}\"\n\
         vclk aclk 0 pulse(0 1 100n 2n 2n 98n 200n)\n\
         abridge [aclk] [clk] adc_late\n\
         .model adc_late adc_bridge(in_low=0.5 in_high=0.5 rise_delay=1.0004n fall_delay=1.0004n)\n\
         xt clk q toggle\n\
         .tran 1n 2.2u\n\
         .end\n",
        source.path()
    );
    let deck = Netlist::parse(&text).unwrap_or_else(|error| panic!("{error}\n{text}"));
    let result = Engine::default().run_tran(&deck, 2.2e-6, 0.09).unwrap();
    let cap = 1.0e-12 * 1.0e-3;
    let rising = edges(&result, "clk")
        .into_iter()
        .filter(|(_, high)| *high)
        .map(|(time, _)| time)
        .collect::<Vec<_>>();
    assert!(rising.len() >= 10, "{rising:?}");
    for (k, time) in rising.iter().enumerate() {
        let image = picosecond(102_000 + 200_000 * k as u64);
        // The accepted point the converter switched at: the crossing, or
        // one floor after it when the crossing read a rounding below the
        // threshold.
        let converted = result
            .time
            .iter()
            .copied()
            .min_by(|a, b| {
                (a - (time - 1.0004e-9))
                    .abs()
                    .total_cmp(&(b - (time - 1.0004e-9)).abs())
            })
            .expect("accepted points");
        let sum = converted + 1.0004e-9;
        assert!(
            (time - sum).abs() <= cap,
            "edge {k} at {time:e} moved {:e} s from its {sum:e} s sum",
            time - sum
        );
        assert_ne!(
            time.to_bits(),
            image.to_bits(),
            "edge {k} was snapped onto its image"
        );
    }
}
