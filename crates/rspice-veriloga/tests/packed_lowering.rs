//! What the packed lowering would emit for real compact models.
//!
//! The width-parameterized backend replaces one value per live derivative lane
//! with one `[f64; L]` binding per differentiated value, so its cost is set by
//! how much of the primal graph actually depends on an unknown. That number is
//! model-dependent and cannot be guessed — an earlier attempt to size the
//! rewrite against the *finished* artifact reported no saving at all, because
//! that graph already has the expansion baked into it and nothing left to
//! distinguish primal work from derivative work.
//!
//! These measure the primal graph instead, and assert the properties the
//! rewrite depends on rather than exact counts, which move whenever the
//! upstream model version does.

use rspice_veriloga::canonical_ir::{OptModel, OptValueKind};
use rspice_veriloga::{CompilerOptions, VerilogACompiler};
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
    assert!(path.exists(), "model fixture missing: {}", path.display());
    path
}

struct Shape {
    primal: OptModel,
    scalarized_values: usize,
}

fn shape_of(parts: &[&str], module: &str) -> Shape {
    let path = model_path(parts);
    let source = std::fs::read_to_string(&path).expect("read model source");
    let mut options = CompilerOptions::default();
    options.include_paths.push(
        path.parent()
            .expect("model lives in a directory")
            .to_path_buf(),
    );
    let artifact = VerilogACompiler::new(options)
        .compile_canonical_ir_module(&source, Some(module))
        .expect("compile model to canonical IR");

    let primal = OptModel::primal_from_hir_and_mir(&artifact.hir, &artifact.mir)
        .expect("lower primal OptIR");

    Shape {
        primal,
        scalarized_values: artifact.opt.values.len(),
    }
}

#[test]
fn expanding_derivatives_dominates_a_compact_model_graph() {
    // The premise of the whole rewrite: most of what the existing backends
    // emit is the derivative expansion, not the model's own arithmetic. If
    // these came out close together there would be nothing to win by packing.
    let shape = shape_of(
        &["BSIM-BULK107.2.1_02112025", "code", "bsimbulk.va"],
        "bsimbulk",
    );
    let primal_values = shape.primal.values.len();

    eprintln!(
        "bsimbulk: primal={} scalarized={} expansion={:.1}x",
        primal_values,
        shape.scalarized_values,
        shape.scalarized_values as f64 / primal_values as f64
    );

    assert!(
        shape.scalarized_values > primal_values * 3,
        "expansion should dominate: primal={primal_values} scalarized={}",
        shape.scalarized_values
    );
}

#[test]
fn primal_graph_carries_no_derivatives_for_a_production_model() {
    // The unit test for this uses a two-terminal fixture. A production compact
    // model exercises limiters, ddt, bounded loops and analysis queries, any of
    // which could smuggle a derivative in through a path the fixture misses.
    let shape = shape_of(
        &["BSIM-BULK107.2.1_02112025", "code", "bsimbulk.va"],
        "bsimbulk",
    );

    assert!(
        shape
            .primal
            .values
            .iter()
            .all(|value| value.derivatives.is_empty()),
        "primal lowering leaked a derivative into a production model graph"
    );
    assert!(
        shape
            .primal
            .values
            .iter()
            .any(|value| matches!(value.kind, OptValueKind::NodePotential { .. })),
        "a MOSFET must read at least one node potential"
    );
}
