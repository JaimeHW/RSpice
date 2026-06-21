//! Full-compilation regression pin for an optional BSIM4.8 Verilog-A model.
//!
//! When present, bsim4.va exercises the entire pipeline: heavy macro
//! preprocessing, the full parser, guarded-dataflow lowering, runtime
//! (nf-bounded) loops, analog function inlining, branch-current unknowns
//! for its collapse/impedance voltage contributions, and shadow-based
//! Jacobian generation. Any regression that breaks one of those features
//! fails this test immediately.

mod support;

use rspice_veriloga::VerilogACompiler;
use std::path::Path;

#[test]
fn bsim4_optional_source_prefers_configured_external_path() {
    let path = std::env::temp_dir().join(format!(
        "rspice-bsim4-env-path-{}-{}.va",
        std::process::id(),
        "frontier"
    ));
    std::fs::write(&path, "module bsim4va; endmodule\n").expect("write temporary VA source");

    let actual =
        support::optional_bsim4_va_path_from(Some(path.clone().into_os_string()), Path::new(""));
    let _ = std::fs::remove_file(&path);

    assert_eq!(actual, Some(path));
}

#[test]
fn bsim4_compiles_end_to_end() {
    let Some(path) = support::optional_bsim4_va_path(env!("CARGO_MANIFEST_DIR")) else {
        eprintln!("bsim4.va not present; skipping optional full-compile pin");
        return;
    };

    let model = VerilogACompiler::default()
        .compile_file(path.as_path())
        .expect("BSIM4.8 must compile end to end");

    // Structural pins: drain/gate/source/bulk terminals, the model's
    // documented internal node set, and the substrate/gate-resistance
    // branch network introduced by its voltage contributions.
    assert_eq!(model.num_terminals, 4, "d/g/s/b terminals");
    assert_eq!(model.internal_nodes, 7, "di/si/gi/gm/dbulk/sbulk/bi");
    assert!(
        model.parameters.len() > 800,
        "BSIM4.8 carries ~900 parameters, got {}",
        model.parameters.len()
    );
    assert!(
        !model.branch_sources.is_empty(),
        "rdsmod/rgatemod/rbodymod voltage contributions need branch unknowns"
    );
    assert!(
        !model.stamp_programs.is_empty() && !model.assignment_steps.is_empty(),
        "stamp and evaluation programs generated"
    );
}
