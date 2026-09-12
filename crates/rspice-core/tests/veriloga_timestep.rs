//! Engine pins for Verilog-A transient timestep control.
//!
//! `$bound_step` caps the step the engine takes while the device is
//! active, so a transient over a quiet circuit must produce steps no
//! larger than the bound (the stepper would otherwise stride far wider).
#![cfg(feature = "veriloga")]

use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect};
use rspice_core::netlist::Netlist;
use rspice_veriloga_runtime::GENERATED_DDT_TIMESTEP_FLOOR;
use std::io::Write;

fn write_model(name: &str, source: &str) -> String {
    let dir = std::env::temp_dir().join("rspice_va_timestep_tests");
    std::fs::create_dir_all(&dir).expect("temp dir");
    let path = dir.join(name);
    let mut file = std::fs::File::create(&path).expect("create model file");
    file.write_all(source.as_bytes()).expect("write model");
    path.display().to_string().replace('\\', "/")
}

const BOUNDED_RES: &str = r#"
`include "disciplines.vams"
module bres(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1000.0 from (0:inf);
    parameter real maxstep = 1.0e-7 from [0:inf);
    analog begin
        $bound_step(maxstep);
        I(p, n) <+ V(p, n) / r;
    end
endmodule
"#;

#[test]
fn switch_branch_currents_follow_both_transient_mode_changes() {
    let model = write_model(
        &format!("switch_modes_{}.va", std::process::id()),
        "module switched(p,c); inout p,c; electrical p,c; analog if(V(c)>0.5) V(p)<+2*I(p); else I(p)<+3*V(p); endmodule",
    );
    let netlist=Netlist::parse(&format!("* switched source transient\nI1 0 out DC 1\nVC control 0 PWL(0 0 1n 0 1.01n 1 2n 1 2.01n 0 3n 0)\nX1 out control switched\n.va \"{model}\" switched\n.end\n")).unwrap();
    for spice_dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
        let result = Engine::new(SimulationConfig {
            spice_dialect,
            ..SimulationConfig::default()
        })
        .run_tran(&netlist, 3e-9, 1e-10)
        .unwrap_or_else(|error| panic!("{spice_dialect:?}: {error}"));
        let out = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap();
        let control = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("control"))
            .unwrap();
        let mut modes = [false; 2];
        for (voltage, control) in result.voltages[out].iter().zip(&result.voltages[control]) {
            let potential = *control > 0.5;
            modes[usize::from(potential)] = true;
            let expected = if potential { 2.0 } else { 1.0 / 3.0 };
            assert!(
                (voltage - expected).abs() < 1e-8,
                "{spice_dialect:?}: control={control}, output={voltage}, expected {expected}"
            );
        }
        assert_eq!(modes, [true, true]);
        assert!((result.voltages[out].last().unwrap() - 1.0 / 3.0).abs() < 1e-8);
    }
    let _ = std::fs::remove_file(model);
}

#[test]
fn nonconvergence_stops_at_the_veriloga_integration_floor() {
    let model = write_model(
        &format!("floor_recovery_{}.va", std::process::id()),
        "module floor_recovery(p); inout p; electrical p; analog begin if ($abstime >= 1e-9) I(p)<+V(p)*V(p)+V(p)+1; else I(p)<+V(p); end endmodule\n",
    );
    let netlist = Netlist::parse(&format!(
        "integration floor\nX1 p floor_recovery\n.va \"{model}\" floor_recovery\n.end\n"
    ))
    .unwrap();
    for spice_dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
        let error = Engine::new(SimulationConfig {
            spice_dialect,
            ..SimulationConfig::default()
        })
        .run_tran(&netlist, 3e-9, 1e-10)
        .expect_err("the post-switch equation has no real equilibrium");
        assert!(
            matches!(error, rspice_core::SimulationError::ConvergenceFailed(_)),
            "{spice_dialect:?} recovery must stop before requesting unsupported integration coefficients: {error}"
        );
    }
    let _ = std::fs::remove_file(model);
}

#[test]
fn analog_timer_near_the_model_floor_lands_without_an_invalid_equalized_step() {
    let model = write_model(
        &format!("floor_timer_{}.va", std::process::id()),
        "module floor_timer(p); inout p; electrical p; integer q; analog begin @(initial_step) q=0; @(timer(2.5e-20)) begin q=1; $discontinuity(0); end V(p)<+q; end endmodule",
    );
    let deck = Netlist::parse(&format!(
        "floor timer\nX1 out floor_timer\nR1 out 0 1k\n.va \"{model}\" floor_timer\n.end\n"
    ))
    .unwrap();
    let floor = GENERATED_DDT_TIMESTEP_FLOOR;
    for spice_dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
        let result = Engine::new(SimulationConfig {
            spice_dialect,
            ..SimulationConfig::default()
        })
        .run_tran(&deck, 5.0 * floor, 1.25 * floor)
        .unwrap_or_else(|error| panic!("{spice_dialect:?}: {error}"));
        let event = 2.5e-20;
        let point = result
            .time
            .iter()
            .position(|time| *time == event)
            .expect("exact timer landing");
        let out = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap();
        assert!(
            result.voltages[out][..point]
                .iter()
                .all(|v| v.abs() < 1e-10)
        );
        assert!(
            result.voltages[out][point..]
                .iter()
                .all(|v| (*v - 1.0).abs() < 1e-10)
        );
        for dt in result.step_sizes.iter().skip(1) {
            assert!(
                *dt >= floor && *dt <= 1.25 * floor,
                "{spice_dialect:?}: dt={dt:.18e}"
            );
        }
    }
    let _ = std::fs::remove_file(model);
}

/// A re-arming analog timer finer than the solver's floor paces the whole
/// analysis at that floor, and the run still reaches a round `tstop`.
///
/// `timer(0, 1f)` asks for another activation a femtosecond after every point
/// the solver lands on, and ngspice's floor for a millisecond maximum step is
/// ten femtoseconds (`delmin = 1e-11 x tmax`), so no analog instant separates
/// the accepted point from the target and every one of them is landed on the
/// floor instead. Ten thousand of those floor points is exactly the hundred
/// picoseconds this analysis runs for.
///
/// The timer also bounds the step it asks for, so the solver's candidate
/// maximum is the floor as well, and that is what made this deck end three
/// points in. Deriving each landing from the point before it and nudging it up
/// until the interval measures a floor *by subtraction* puts it a couple of ulps
/// beyond one floor, and a bound pinned at the floor has no step that reaches
/// it — one falls short of the target and two overshoot by a whole floor:
/// `cannot integrate to mandatory time 3.0000000000000005e-14s from
/// 2.0000000000000000e-14s: interval 1.0000000000000005e-14s cannot satisfy
/// model minimum 1.0000000000000000e-14s, maximum 1.0000000000000000e-3s and
/// current bound 1.0000000000000000e-14s` — an interval that looks exactly like
/// the floor it is said not to satisfy. The same nudge accumulates about half an
/// ulp per point over a long march, which is what used to leave a round `tstop`
/// unreachable; both are the chain, and the landings are points on one grid
/// instead.
///
/// The module's own output is deliberately constant: this is about the width
/// of the steps, not about anything the timer does when it fires.
#[test]
fn a_sub_minimum_analog_timer_marches_to_a_round_stop_time() {
    const TSTOP: f64 = 1.0e-10;
    const MAX_STEP: f64 = 1.0e-3;
    // `delmin = 1e-11 x tmax`, and the analysis is exactly this many of them.
    let floor = MAX_STEP * 1.0e-11;
    assert_eq!(TSTOP / floor, 10_000.0);

    let model = write_model(
        &format!("floor_march_{}.va", std::process::id()),
        "module floor_march(p); inout p; electrical p; integer q; \
         analog begin @(initial_step) q = 1; @(timer(0.0, 1.0e-15)) q = 1; V(p) <+ q; end \
         endmodule\n",
    );
    let deck = Netlist::parse(&format!(
        "floor march\nX1 out floor_march\nR1 out 0 1k\n.va \"{model}\" floor_march\n.end\n"
    ))
    .expect("the deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_tran(&deck, TSTOP, MAX_STEP)
        .unwrap_or_else(|error| panic!("the femtosecond timer must reach tstop: {error}"));

    let last = result.time.last().copied().unwrap_or(0.0);
    assert!(
        last >= TSTOP - floor,
        "the run must reach tstop {TSTOP:e}s, it stopped at {last:e}s"
    );
    // One accepted point per floor step, which is what makes this a march
    // rather than a handful of landings: a different floor would show up here
    // as a different count long before the residual below.
    assert!(
        (9_000..=11_000).contains(&result.time.len()),
        "the analysis must be paced at the {floor:e}s floor, it took {} accepted points",
        result.time.len()
    );
    // The accepted points are the floor grid the run started on, so the residual
    // against `k x delmin` never leaves the ulps of one multiply-add.
    let drifted = result
        .time
        .iter()
        .map(|time| (time, (time / floor).round()))
        .filter(|(_, index)| *index >= 1.0)
        .map(|(time, index)| (time, time - index * floor))
        .max_by(|(_, left), (_, right)| left.abs().total_cmp(&right.abs()));
    if let Some((time, residual)) = drifted {
        let ulp = time.next_up() - time;
        println!(
            "worst floor-grid residual: {residual:+.3e}s ({:+.1} ulps)",
            residual / ulp
        );
        assert!(
            residual.abs() <= 64.0 * f64::EPSILON * TSTOP,
            "a floor march may not drift off its grid, saw {residual:+.3e}s at t={time:e}s"
        );
    }
    let _ = std::fs::remove_file(model);
}

#[test]
fn bound_step_at_or_below_the_integration_floor_uses_the_minimum() {
    let model = write_model(
        &format!("floor_bound_{}.va", std::process::id()),
        BOUNDED_RES,
    );
    let floor = GENERATED_DDT_TIMESTEP_FLOOR;
    for spice_dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
        for (bound, maximum) in [
            (0.0, 10.0 * floor),
            (0.5 * floor, 10.0 * floor),
            (floor, 10.0 * floor),
            (1e-7, floor),
        ] {
            let netlist = Netlist::parse(&format!(
                "bounded minimum\nv1 in 0 dc 1\nr1 in out 1k\nX1 out 0 bres maxstep={bound:e}\n.va \"{model}\" bres\n.end\n"
            ))
            .unwrap();
            let result = Engine::new(SimulationConfig {
                spice_dialect,
                ..SimulationConfig::default()
            })
            .run_tran(&netlist, 100.0 * floor, maximum)
            .unwrap_or_else(|error| panic!("{spice_dialect:?}, bound={bound:e}: {error}"));
            assert_eq!(result.time.last(), Some(&(100.0 * floor)));
            assert!(result.time.len() >= 100, "{:?}", result.time);
            for pair in result.time.windows(2) {
                let dt = pair[1] - pair[0];
                assert!((dt / floor - 1.0).abs() < 1e-10, "dt={dt:e}");
            }
            let out = result
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("out"))
                .unwrap();
            for voltage in &result.voltages[out] {
                assert!((voltage - 0.5).abs() < 1e-8, "{voltage}");
            }
        }
    }
    let _ = std::fs::remove_file(model);
}

#[test]
fn unsupported_maximum_and_locked_intervals_are_refused() {
    let model = write_model(
        &format!("floor_grid_{}.va", std::process::id()),
        BOUNDED_RES,
    );
    let netlist = Netlist::parse(&format!(
        "minimum grid\nv1 p 0 dc 1\nX1 p 0 bres\n.va \"{model}\" bres\n.end\n"
    ))
    .unwrap();
    let floor = GENERATED_DDT_TIMESTEP_FLOOR;
    for spice_dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
        let error = Engine::new(SimulationConfig {
            spice_dialect,
            ..SimulationConfig::default()
        })
        .run_tran(&netlist, 20.0 * floor, 0.5 * floor)
        .expect_err("the requested maximum cannot fit a supported interval");
        assert!(error.to_string().contains("maximum timestep"), "{error}");

        let error = Engine::new(SimulationConfig {
            spice_dialect,
            locked_time_grid: Some(std::sync::Arc::new(vec![0.5 * floor, 20.0 * floor])),
            ..SimulationConfig::default()
        })
        .run_tran(&netlist, 20.0 * floor, 10.0 * floor)
        .expect_err("the locked target must not be silently skipped");
        assert!(error.to_string().contains("cannot integrate"), "{error}");

        let error = Engine::new(SimulationConfig {
            spice_dialect,
            ..SimulationConfig::default()
        })
        .run_tran(&netlist, 20.5 * floor, floor)
        .expect_err("a genuinely short final interval must not be enlarged");
        assert!(error.to_string().contains("cannot integrate"), "{error}");
    }
    let _ = std::fs::remove_file(model);
}

#[test]
fn bound_step_caps_transient_steps() {
    let model = write_model("bres.va", BOUNDED_RES);
    let deck = format!(
        "* bounded steps over a quiet divider\n\
         v1 in 0 dc 1.0\n\
         r1 in out 1k\n\
         X1 out 0 bres r=1k maxstep=1e-7\n\
         .va \"{model}\" bres\n\
         .end\n"
    );

    let netlist = Netlist::parse(&deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let result = engine
        .run_tran(&netlist, 5e-6, 1e-6)
        .expect("transient completes");

    // Every accepted step after startup must respect the bound (allow
    // a tolerance for the very first ramp-in steps)
    let times = &result.time;
    assert!(
        times.len() >= 40,
        "bound forces many steps: {}",
        times.len()
    );
    let mut max_step: f64 = 0.0;
    for pair in times.windows(2).skip(3) {
        max_step = max_step.max(pair[1] - pair[0]);
    }
    assert!(
        max_step <= 1.0e-7 * 1.5,
        "largest accepted step {max_step:.3e} exceeds the $bound_step cap"
    );
}

#[test]
fn discontinuity_newton_hint_prevents_false_dc_convergence() {
    let model = write_model(
        &format!("newton_hint_{}.va", std::process::id()),
        "module hint(p,n); inout p,n; electrical p,n; parameter real degree=0; analog begin $discontinuity(degree); I(p,n)<+V(p,n); end endmodule",
    );
    for spice_dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
        for degree in [0, 1, -1] {
            let netlist = Netlist::parse(&format!(
                "Newton convergence hint\nI1 0 p 1\nX1 p 0 hint degree={degree}\n.va \"{model}\" hint\n.end\n"
            )).unwrap();
            let result = Engine::new(SimulationConfig {
                spice_dialect,
                ..SimulationConfig::default()
            })
            .run_dc_op(&netlist);
            if degree == -1 {
                let error = result.expect_err(
                    "the active hint must prevent convergence even with zero KCL residual",
                );
                assert!(
                    matches!(error, rspice_core::SimulationError::ConvergenceFailed(_)),
                    "{spice_dialect:?}: {error}"
                );
            } else {
                let point = result.unwrap();
                let node = point
                    .node_names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case("p"))
                    .unwrap();
                assert!(
                    (point.node_voltages[node] - 1.0).abs() < 1e-8,
                    "{spice_dialect:?}, degree={degree}: {:?}",
                    point.node_voltages
                );
            }
        }
    }
    let _ = std::fs::remove_file(model);
}
