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
