#![cfg(feature = "veriloga")]

use rspice_core::{Engine, Netlist};
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static FIXTURE_ID: AtomicU64 = AtomicU64::new(0);

fn write_model(source: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!(
        "rspice_coherent_noise_{}_{}_{}.va",
        std::process::id(),
        std::thread::current().name().unwrap_or("test"),
        FIXTURE_ID.fetch_add(1, Ordering::Relaxed),
    ));
    let mut file = std::fs::File::create(&path).expect("create Verilog-A noise fixture");
    file.write_all(source.as_bytes())
        .expect("write Verilog-A noise fixture");
    path
}

fn dynamic_range_source() -> &'static str {
    r#"
`include "disciplines.vams"
module dynamic_range_noise(p, n);
    inout p, n;
    electrical p, n;
    real process;
    analog begin
        process = white_noise(1.0, "residual");
        I(p, n) <+ 1.0e16 * process;
        I(p, n) <+ process;
        I(p, n) <+ -1.0e16 * process;
    end
endmodule
"#
}

fn deck_path(path: &std::path::Path) -> String {
    path.display().to_string().replace('\\', "/")
}

fn shared_twice_source() -> &'static str {
    r#"
`include "disciplines.vams"
module shared_twice(p, n);
    inout p, n;
    electrical p, n;
    real process;
    analog begin
        process = white_noise(1.0, "shared");
        I(p, n) <+ process;
        I(p, n) <+ process;
    end
endmodule
"#
}

#[test]
fn ordinary_adjoint_sums_reused_process_injections_before_squaring() {
    let model = write_model(shared_twice_source());
    let deck = format!(
        "* coherent Verilog-A output noise\nR1 out 0 1\nX1 out 0 shared_twice\n.va \"{}\" shared_twice\n.end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse(&deck).expect("coherent noise deck parses");
    let circuit = Engine::default()
        .build_circuit(&netlist)
        .expect("coherent noise circuit builds");
    let output = circuit.get_node_by_name("out").expect("output node");
    let result = Engine::default()
        .run_noise(&netlist, output, &[1.0e3], 300.15)
        .expect("ordinary noise succeeds");
    assert_eq!(result.len(), 1);
    assert!(
        (result[0].output_noise_density - 4.0).abs() < 1.0e-10,
        "expected |1+1|^2 = 4 V^2/Hz, got {}",
        result[0].output_noise_density
    );
    let _ = std::fs::remove_file(model);
}

#[test]
fn port_covariance_uses_the_same_coherent_process_amplitude() {
    let model = write_model(shared_twice_source());
    let deck = format!(
        "* coherent Verilog-A port noise\nVPORT out 0 0\nX1 out 0 shared_twice\n.va \"{}\" shared_twice\n.end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse(&deck).expect("coherent port-noise deck parses");
    let result = Engine::default()
        .run_port_noise_correlation(&netlist, &["VPORT".to_string()], &[1.0e3], 300.15)
        .expect("port-noise covariance succeeds");
    let covariance = result[0].current_correlation[0][0];
    assert!(covariance.im.abs() < 1.0e-12);
    assert!(
        (covariance.re - 4.0).abs() < 1.0e-10,
        "expected |1+1|^2 = 4 A^2/Hz, got {covariance:?}"
    );
    let _ = std::fs::remove_file(model);
}

#[test]
fn ordinary_adjoint_retains_small_residual_between_large_injections() {
    let model = write_model(dynamic_range_source());
    let deck = format!(
        "* compensated coherent Verilog-A output noise\nR1 out 0 1\nX1 out 0 dynamic_range_noise\n.va \"{}\" dynamic_range_noise\n.end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse(&deck).expect("dynamic-range noise deck parses");
    let circuit = Engine::default()
        .build_circuit(&netlist)
        .expect("dynamic-range noise circuit builds");
    let output = circuit.get_node_by_name("out").expect("output node");
    let result = Engine::default()
        .run_noise(&netlist, output, &[1.0e3], 300.15)
        .expect("dynamic-range ordinary noise succeeds");
    assert!(
        (result[0].output_noise_density - 1.0).abs() < 1.0e-10,
        "expected retained unit residual, got {}",
        result[0].output_noise_density
    );
    let _ = std::fs::remove_file(model);
}

#[test]
fn port_covariance_retains_small_residual_between_large_injections() {
    let model = write_model(dynamic_range_source());
    let deck = format!(
        "* compensated coherent Verilog-A port noise\nVPORT out 0 0\nX1 out 0 dynamic_range_noise\n.va \"{}\" dynamic_range_noise\n.end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse(&deck).expect("dynamic-range port deck parses");
    let result = Engine::default()
        .run_port_noise_correlation(&netlist, &["VPORT".to_string()], &[1.0e3], 300.15)
        .expect("dynamic-range port covariance succeeds");
    let covariance = result[0].current_correlation[0][0];
    assert!(covariance.im.abs() < 1.0e-12);
    assert!(
        (covariance.re - 1.0).abs() < 1.0e-10,
        "expected retained unit residual, got {covariance:?}"
    );
    let _ = std::fs::remove_file(model);
}

#[test]
fn grouped_engine_prepass_does_not_evaluate_untaken_invalid_psd() {
    let model = write_model(
        r#"
`include "disciplines.vams"
module inactive_invalid_noise(p, n);
    inout p, n; electrical p, n;
    parameter real enable = 0.0;
    real hidden;
    analog begin
        if (enable) hidden = white_noise(-1.0, "inactive");
        I(p, n) <+ hidden;
    end
endmodule
"#,
    );
    let deck = format!(
        "* inactive grouped noise\nR1 out 0 1\nX1 out 0 inactive_invalid_noise\n.va \"{}\" inactive_invalid_noise\n.end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse(&deck).expect("inactive noise deck parses");
    let circuit = Engine::default()
        .build_circuit(&netlist)
        .expect("inactive noise circuit builds");
    let output = circuit.get_node_by_name("out").expect("output node");
    let result = Engine::default()
        .run_noise(&netlist, output, &[1.0e3], 300.15)
        .expect("untaken invalid PSD never reaches the eager scalar prepass");
    assert!(
        result[0].contribution_catalog.iter().any(|identity| {
            identity.device.eq_ignore_ascii_case("X1")
                && identity
                    .mechanism
                    .as_deref()
                    .is_some_and(|mechanism| mechanism.eq_ignore_ascii_case("INACTIVE"))
        }),
        "inactive structural process must remain in the contribution catalog: {:?}",
        result[0].contribution_catalog
    );
    let _ = std::fs::remove_file(model);
}

#[test]
fn schema_zero_scalar_noise_remains_visible_to_ordinary_and_port_analysis() {
    let source = r#"
`include "disciplines.vams"
module legacy_scalar_noise(p, n);
    inout p, n; electrical p, n;
    analog I(p, n) <+ white_noise(2.0, "legacy");
endmodule
"#;
    let model_path = write_model(source);
    let compiler =
        rspice_veriloga::VerilogACompiler::new(rspice_veriloga::CompilerOptions::default());
    let mut model = compiler.compile(source).expect("legacy model compiles");
    model.noise_process_schema = 0;
    for noise in &mut model.noise_sources {
        noise.process_id = 0;
        noise.injections.clear();
    }
    #[cfg(not(feature = "veriloga-native"))]
    rspice_core::register_precompiled_veriloga_model(&model_path, model)
        .expect("portable legacy model registers without canonical IR");
    #[cfg(feature = "veriloga-native")]
    {
        let canonical = compiler
            .compile_canonical_ir(source)
            .expect("legacy canonical IR compiles");
        rspice_core::register_precompiled_veriloga_runtime_with_dependencies(
            &model_path,
            std::slice::from_ref(&model_path),
            model,
            canonical,
        )
        .expect("native legacy runtime registers");
    }

    let ordinary_deck = format!(
        "* legacy scalar ordinary noise\nR1 out 0 1\nX1 out 0 legacy_scalar_noise\n.va \"{}\" legacy_scalar_noise\n.end\n",
        deck_path(&model_path)
    );
    let ordinary = Netlist::parse(&ordinary_deck).expect("ordinary legacy deck parses");
    let circuit = Engine::default()
        .build_circuit(&ordinary)
        .expect("ordinary legacy circuit builds");
    let output = circuit.get_node_by_name("out").expect("output node");
    let result = Engine::default()
        .run_noise(&ordinary, output, &[1.0e3], 300.15)
        .expect("ordinary legacy scalar noise succeeds");
    assert!((result[0].output_noise_density - 2.0).abs() < 1.0e-10);

    let port_deck = format!(
        "* legacy scalar port noise\nVPORT out 0 0\nX1 out 0 legacy_scalar_noise\n.va \"{}\" legacy_scalar_noise\n.end\n",
        deck_path(&model_path)
    );
    let port = Netlist::parse(&port_deck).expect("port legacy deck parses");
    let result = Engine::default()
        .run_port_noise_correlation(&port, &["VPORT".to_string()], &[1.0e3], 300.15)
        .expect("port legacy scalar noise succeeds");
    assert!((result[0].current_correlation[0][0].re - 2.0).abs() < 1.0e-10);
    let _ = std::fs::remove_file(model_path);
}
