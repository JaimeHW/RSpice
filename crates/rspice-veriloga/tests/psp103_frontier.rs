//! Compilation frontier for PSP 103.6 (NXP/CEA/TU Delft, via the IHP
//! SG13G2 open PDK): the industry-standard surface-potential MOSFET
//! model. 460 kB of Verilog-A across 15 include files exercises the
//! preprocessor, ddx(), heavy analog functions, and JUNCAP200 junctions.

use rspice_veriloga::VerilogACompiler;
use std::path::{Path, PathBuf};

fn psp_path() -> Option<PathBuf> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("models")
        .join("veriloga")
        .join("psp103")
        .join("psp103.va");
    path.exists().then_some(path)
}

#[test]
fn psp103_compiles_end_to_end() {
    let Some(path) = psp_path() else {
        eprintln!("psp103.va not present; skipping frontier");
        return;
    };

    let model = VerilogACompiler::default()
        .compile_file(&path)
        .expect("PSP 103.6 must compile");

    assert_eq!(model.num_terminals, 4, "d g s b");
    assert!(
        model.parameters.len() >= 400,
        "PSP 103.6 carries hundreds of parameters, got {}",
        model.parameters.len()
    );
    assert!(!model.stamp_programs.is_empty());
    eprintln!(
        "PSP 103.6: {} params, {} variables, {} stamps, {} internal nodes, {} branch unknowns",
        model.parameters.len(),
        model.num_variables,
        model.stamp_programs.len(),
        model.internal_nodes,
        model.branch_sources.len()
    );
}
