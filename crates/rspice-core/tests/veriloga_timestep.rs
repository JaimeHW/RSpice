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
                // Every operating-point aid fails too, and the last one to
                // run reports which aid stopped and where.
                assert!(
                    matches!(
                        error,
                        rspice_core::SimulationError::ConvergenceFailed(_)
                            | rspice_core::SimulationError::ContinuationFailed(_)
                    ),
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

/// A timer whose target falls a hair before a source corner fires instead of
/// ending the run.
///
/// `timer(0, per)` with `per` an ulp to 10 as short of 10 us asks for an event
/// just before the PULSE corner at 10 us. The device bounded each step to end
/// on its event, the breakpoint schedule halved every such step to equalize
/// the approach to the corner, and the run crept up on the corner until it was
/// within the merge tolerance and was snapped onto it, past the event. The
/// next scheduling query then aborted the run: `event 0.0000099999999999999
/// is not strictly after accepted time 0.00001`. Now an event at least one
/// floor (1e-18 s here) short of the corner is landed on, and one closer than
/// that is the corner's instant and fires on it.
///
/// The count is the analytic one: the timer fires at `k * per` for every
/// `k * per` in `[0, 100 us]`, eleven times, so `V(g)` steps from 1 to 11; and
/// the approach to each corner takes a handful of points, not a creep.
#[test]
fn a_timer_target_just_before_a_source_corner_fires_at_the_corner() {
    let model = write_model(
        &format!("corner_timer_{}.va", std::process::id()),
        "`include \"disciplines.vams\"\n\
         module corner_timer(ramp, g); input ramp; output g; electrical ramp, g;\n\
         integer q; parameter real per = 1e-5;\n\
         analog begin @(initial_step) q = 0; @(timer(0, per)) q = q + 1; V(g) <+ q; end\n\
         endmodule\n",
    );
    // 1e-19 s, one ulp, 3e-18 s and 1e-17 s short of the corner.
    for per in [
        9.9999999999999e-6,
        9.999_999_999_999_999e-6,
        9.999999999997e-6,
        9.99999999999e-6,
    ] {
        let deck = Netlist::parse(&format!(
            "corner timer\nvr ramp 0 pulse(0 1 0 9.9u 0.1u 0 10u)\n\
             X1 ramp g corner_timer per={per:e}\nRL g 0 1k\n\
             .va \"{model}\" corner_timer\n.end\n"
        ))
        .expect("the deck parses");
        let result = Engine::new(SimulationConfig::default())
            .run_tran(&deck, 100e-6, 0.1e-6)
            .unwrap_or_else(|error| panic!("per={per:e}: the run must complete: {error}"));
        let g = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("g"))
            .expect("node g");
        assert_eq!(result.time.last(), Some(&100e-6), "per={per:e}");
        for (time, level) in result.time.iter().zip(&result.voltages[g]) {
            let cycles = time / per;
            if (cycles - cycles.round()).abs() < 1e-6 {
                // At a firing (or the corner it moved to) either side may be
                // sampled; the count is checked strictly between them.
                continue;
            }
            assert_eq!(
                *level,
                cycles.floor() + 1.0,
                "per={per:e}: V(g) at t={time:e} counts the firings so far"
            );
        }
        assert_eq!(result.voltages[g].last(), Some(&11.0), "per={per:e}");
        let creep = result
            .time
            .iter()
            .filter(|time| {
                let to_corner = (*time / 1e-5).ceil() * 1e-5 - *time;
                to_corner > 0.0 && to_corner < 1e-12
            })
            .count();
        // Landing the event takes a point or two per corner (37 in all at
        // 1e-17 s). Halving towards each corner took about sixteen per corner
        // inside the last picosecond.
        assert!(
            creep <= 60,
            "per={per:e}: {creep} points crept up on the ten corners"
        );
    }
    let _ = std::fs::remove_file(model);
}

/// A `cross` event that steps a linear output by 20 V, with or without a
/// timer elsewhere in the module.
fn event_step_model(name: &str, timer: &str, contribution: &str, step: f64) -> String {
    write_model(
        &format!("{name}_{}.va", std::process::id()),
        &format!(
            "`include \"disciplines.vams\"\n\
             module ev(in, o1); input in; output o1; electrical in, o1;\n\
             real ta, tt;\n\
             analog begin\n\
             @(initial_step) begin ta = 0; tt = 0; end\n\
             @(cross(V(in) - 0.9, +1)) ta = {step:e};\n\
             {timer}\n\
             {contribution}\n\
             end\n\
             endmodule\n"
        ),
    )
}

fn run_event_step(model: &str) -> (rspice_core::engine::TransientResult, Engine) {
    let deck = Netlist::parse(&format!(
        "cross step\nv1 in 0 sin(0 1 1k)\nX1 in o1 ev\n.va \"{model}\" ev\n.end\n"
    ))
    .expect("the deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let result = engine
        .run_tran(&deck, 1.5e-3, 10e-6)
        .unwrap_or_else(|error| panic!("{model}: the run must complete: {error}"));
    (result, engine)
}

/// B-4. `@(cross(V(in) - 0.9, +1)) ta = 20; V(o1) <+ ta;` on a 1 kHz, 1 V sine
/// is a linear circuit, but with any `@(timer(...))` in the module the run
/// failed with "Convergence failed after 103 iterations" at
/// t = 178.2168534356 us. Each refinement that moves a straddling candidate
/// back onto the root counted as a Newton recovery retry; the second one armed
/// the global nodal trust region, whose 0.2 V-per-iteration clamp walks o1 at
/// most 16.5 V in the 100-iteration budget, so the landing failed Newton every
/// time it took two refinements and the chase died at the 1e-16 s floor. The
/// timer only offset the step grid. The Norton form failed the same way.
///
/// The root is analytic: sin(2 pi 1k t) = 0.9 at t = asin(0.9) / (2000 pi)
/// = 178.2168534356 us. o1 is 0 V before it and 20 V from it on.
#[test]
fn a_twenty_volt_cross_step_lands_on_its_root_with_or_without_a_timer() {
    let root = 0.9_f64.asin() / (2000.0 * std::f64::consts::PI);
    for (form, contribution) in [
        ("thevenin", "V(o1) <+ ta;"),
        ("norton", "I(o1) <+ -ta / 1k; I(o1) <+ V(o1) / 1k;"),
    ] {
        for timer in ["", "@(timer(10u)) tt = 1;", "@(timer(1u)) tt = 1;"] {
            let model = event_step_model(&format!("cross_step_{form}"), timer, contribution, 20.0);
            let (result, _) = run_event_step(&model);
            let o1 = result
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("o1"))
                .expect("node o1");
            assert_eq!(result.time.last(), Some(&1.5e-3), "{form} {timer}");
            for (time, level) in result.time.iter().zip(&result.voltages[o1]) {
                let expected = if *time < root - 1e-12 {
                    0.0
                } else if *time > root + 1e-12 {
                    20.0
                } else {
                    continue;
                };
                assert!(
                    (level - expected).abs() < 1e-9,
                    "{form} {timer}: V(o1) at t={time:e} is {level}, expected {expected}"
                );
            }
            let landing = result
                .time
                .iter()
                .zip(&result.voltages[o1])
                .find(|(_, level)| (*level - 20.0).abs() < 1e-9)
                .map(|(time, _)| *time)
                .expect("o1 reaches 20 V");
            assert!(
                (landing - root).abs() < 1e-12,
                "{form} {timer}: the event lands at {landing:e}, the root is {root:e}"
            );
            let _ = std::fs::remove_file(model);
        }
    }
}

/// B-4, the passing case: without a timer the same module completed, but it
/// landed the crossing through nine Newton failures, each spending the whole
/// 100-iteration budget under the trust region. A converged candidate moved
/// onto an event root is not a Newton failure, and neither is the landing:
/// a 178 V step costs exactly the iterations and rejections a 1 mV step does.
#[test]
fn a_cross_step_costs_no_newton_failure_whatever_its_size() {
    let cost = |step: f64| {
        let model = event_step_model("cross_step_cost", "", "V(o1) <+ ta;", step);
        let (result, engine) = run_event_step(&model);
        let quality = engine.convergence_quality();
        let _ = std::fs::remove_file(model);
        (
            result.time.len(),
            quality.total_iterations,
            quality.timestep_reductions,
        )
    };
    let (points, iterations, reductions) = cost(178.0);
    // Before the fix: (216, 1164, 37) against (164, 196, 21).
    assert_eq!((points, iterations, reductions), cost(1e-3));
    // One failed attempt alone spends the whole 100-iteration ngspice budget;
    // the run, operating point included, spends fewer than one per point
    // beyond that.
    assert!(
        iterations < points + 100,
        "{iterations} Newton iterations for {points} points"
    );
}

/// B-4, the power-electronics case: a clocked PWM (timer sets the gate, the
/// ramp crossing the control resets it) driving 20 or 24 V failed with
/// "Convergence failed after 118 iterations" while 12 V passed. The ramp
/// rises 0 to 1 V over 9.9 us of each 10 us period, so the 0.43 V control is
/// crossed at 4.257 us into every cycle: the gate is vdrv on [k*10u, k*10u +
/// 4.257u) and 0 V for the rest of the cycle.
#[test]
fn a_clocked_pwm_drives_twenty_four_volts_every_cycle() {
    let model = write_model(
        &format!("pwm_{}.va", std::process::id()),
        "`include \"disciplines.vams\"\n\
         module pwm(ctl, ramp, g); input ctl, ramp; output g; electrical ctl, ramp, g;\n\
         parameter real vdrv = 20; parameter real period = 10u;\n\
         integer q;\n\
         analog begin\n\
         @(initial_step) q = 0;\n\
         @(timer(0, period)) q = 1;\n\
         @(cross(V(ramp) - V(ctl), +1)) q = 0;\n\
         V(g) <+ vdrv * q;\n\
         end\n\
         endmodule\n",
    );
    for vdrv in [12.0, 20.0, 24.0] {
        let deck = Netlist::parse(&format!(
            "pwm\nvr ramp 0 pulse(0 1 0 9.9u 0.1u 0 10u)\nvc ctl 0 0.43\n\
             X1 ctl ramp g pwm vdrv={vdrv}\nrl g 0 1k\n.va \"{model}\" pwm\n.end\n"
        ))
        .expect("the deck parses");
        let result = Engine::new(SimulationConfig::default())
            .run_tran(&deck, 100e-6, 0.1e-6)
            .unwrap_or_else(|error| panic!("vdrv={vdrv}: the run must complete: {error}"));
        let g = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("g"))
            .expect("node g");
        assert_eq!(result.time.last(), Some(&100e-6), "vdrv={vdrv}");
        let mut high = 0;
        for (time, level) in result.time.iter().zip(&result.voltages[g]) {
            let phase = time - (time / 10e-6).floor() * 10e-6;
            let expected = if phase > 1e-12 && phase < 4.257e-6 - 1e-12 {
                high += 1;
                vdrv
            } else if phase > 4.257e-6 + 1e-12 && phase < 10e-6 - 1e-12 {
                0.0
            } else {
                continue;
            };
            assert!(
                (level - expected).abs() < 1e-9,
                "vdrv={vdrv}: V(g) at t={time:e} is {level}, expected {expected}"
            );
        }
        assert!(high >= 10, "vdrv={vdrv}: the gate is high in every cycle");
    }
    let _ = std::fs::remove_file(model);
}
