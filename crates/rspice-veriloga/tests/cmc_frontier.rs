//! Compilation frontiers for the CMC standard models shipped with the
//! IHP SG13G2 open PDK beyond PSP itself: the r3_cmc 3-terminal resistor
//! and the standalone JUNCAP200 junction model.

use rspice_veriloga::VerilogACompiler;
use std::path::{Path, PathBuf};

fn model_path(parts: &[&str]) -> Option<PathBuf> {
    let mut path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("models")
        .join("veriloga");
    for part in parts {
        path = path.join(part);
    }
    path.exists().then_some(path)
}

#[test]
fn r3_cmc_compiles_end_to_end() {
    let Some(path) = model_path(&["r3_cmc", "r3_cmc.va"]) else {
        eprintln!("r3_cmc.va not present; skipping frontier");
        return;
    };
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
    let Some(path) = model_path(&["psp103", "juncap200.va"]) else {
        eprintln!("juncap200.va not present; skipping frontier");
        return;
    };
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
