//! Engine pins for Verilog-A transient timestep control.
//!
//! `$bound_step` caps the step the engine takes while the device is
//! active, so a transient over a quiet circuit must produce steps no
//! larger than the bound (the stepper would otherwise stride far wider).
#![cfg(feature = "veriloga")]

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;
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
    parameter real maxstep = 1.0e-7 from (0:inf);
    analog begin
        $bound_step(maxstep);
        I(p, n) <+ V(p, n) / r;
    end
endmodule
"#;

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
    assert!(times.len() >= 40, "bound forces many steps: {}", times.len());
    let mut max_step: f64 = 0.0;
    for pair in times.windows(2).skip(3) {
        max_step = max_step.max(pair[1] - pair[0]);
    }
    assert!(
        max_step <= 1.0e-7 * 1.5,
        "largest accepted step {max_step:.3e} exceeds the $bound_step cap"
    );
}
