//! Compilation frontier for PSP 103.6 (NXP/CEA/TU Delft, via the IHP
//! SG13G2 open PDK): the industry-standard surface-potential MOSFET
//! model. 460 kB of Verilog-A across 15 include files exercises the
//! preprocessor, ddx(), heavy analog functions, and JUNCAP200 junctions.

use rspice_veriloga::VerilogACompiler;
use rspice_veriloga::canonical_ir::MirEquationKind;
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
fn psp103_canonical_ir_reports_expected_potential_branch_equations() {
    let Some(path) = psp_path() else {
        eprintln!("psp103.va not present; skipping frontier");
        return;
    };

    let compiled = VerilogACompiler::default()
        .compile_file_canonical_ir_with_metadata(&path, Some("PSP103VA"))
        .expect("PSP 103.6 canonical IR must compile");
    let mut potential_equations = Vec::new();
    for equation in &compiled.artifact.mir.equations {
        if equation.kind == MirEquationKind::Potential {
            potential_equations.push((equation.id.index(), equation.branch.label.to_string()));
        }
    }

    assert_eq!(
        potential_equations,
        vec![
            (17, "G,GP".to_string()),
            (20, "S,SI".to_string()),
            (23, "D,DI".to_string()),
            (26, "BP,BI".to_string()),
            (29, "BS,BI".to_string()),
            (32, "BD,BI".to_string()),
            (35, "B,BI".to_string()),
        ]
    );
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
