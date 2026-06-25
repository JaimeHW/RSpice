//! Compilation frontiers for the CMC standard models shipped with the
//! shipped CMC corpus: the r3_cmc 3-terminal resistor and the standalone
//! JUNCAP200 junction model.

use rspice_veriloga::VerilogACompiler;
use std::path::{Path, PathBuf};

fn model_path(parts: &[&str]) -> PathBuf {
    let mut path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("models")
        .join("veriloga")
        .join("cmc");
    for part in parts {
        path = path.join(part);
    }
    assert!(
        path.exists(),
        "required shipped CMC model fixture missing: {}",
        path.display()
    );
    path
}

#[test]
fn r3_cmc_compiles_end_to_end() {
    let path = model_path(&["r3_cmc_release1.1.2_2023Jun16", "r3_cmc.va"]);
    let model = VerilogACompiler::default()
        .compile_file(&path)
        .expect("r3_cmc must compile");
    eprintln!(
        "r3_cmc: {} terminals, {} params, {} stamps",
        model.num_terminals,
        model.parameters.len(),
        model.stamp_programs.len()
    );
    assert!(model.parameters.len() >= 40);
    assert!(!model.stamp_programs.is_empty());
}

#[test]
fn juncap200_compiles_end_to_end() {
    let path = model_path(&["PSP104.1.0_vacode", "vacode", "juncap200.va"]);
    let model = VerilogACompiler::default()
        .compile_file(&path)
        .expect("JUNCAP200 must compile");
    eprintln!(
        "juncap200: {} terminals, {} params, {} stamps",
        model.num_terminals,
        model.parameters.len(),
        model.stamp_programs.len()
    );
    assert!(!model.stamp_programs.is_empty());
}
