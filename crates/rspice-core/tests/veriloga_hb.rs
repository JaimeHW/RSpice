#![cfg(feature = "veriloga")]

use rspice_core::analysis::harmonic_balance::HbConfig;
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

fn write_model(name: &str, source: &str) -> String {
    let mut path = std::env::temp_dir();
    path.push(format!("rspice_hb_{}_{}.va", name, std::process::id()));
    std::fs::write(&path, source).expect("write temporary Verilog-A model");
    path.to_string_lossy().replace('\\', "/")
}

#[test]
fn dynamic_and_noisy_veriloga_fails_closed_until_hb_metadata_is_exact() {
    let model = write_model(
        "dynamic_noise",
        r#"
`include "disciplines.vams"

module va_hb_dynamic_noise(p, n);
    inout p, n;
    electrical p, n;
    parameter real g = 1.0e-3;
    parameter real c = 1.0e-12;
    analog I(p, n) <+ g * V(p, n) + ddt(c * V(p, n))
        + white_noise(1.0e-18, "thermal");
endmodule
"#,
    );

    let deck = format!(
        "* veriloga HB exact-capability diagnostic\n\
         V1 in 0 DC 1 SIN(1 0 1meg)\n\
         R1 in 0 1k\n\
         XVA in 0 va_hb_dynamic_noise\n\
         .va \"{model}\" va_hb_dynamic_noise\n\
         .end\n"
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let result = std::panic::catch_unwind(|| {
        Engine::new(SimulationConfig::default())
            .run_hb(&netlist, HbConfig::new(1.0e6).with_harmonics(3))
    });

    let _ = std::fs::remove_file(model);

    let result = result.expect("unsupported Verilog-A HB capability must not panic");
    let err = result.expect_err("HB must not omit Verilog-A charge or noise contributions");
    let text = err.to_string();
    assert!(
        text.contains("runtime Verilog-A")
            && text.contains("exact HB charge/noise capability metadata"),
        "diagnostic should identify the unavailable exact Verilog-A HB capability, got: {text}"
    );
}
