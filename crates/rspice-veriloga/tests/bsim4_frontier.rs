//! Compilation-frontier regression pin for the bundled BSIM4.8 model.
//!
//! The 12.6k-line bsim4.va exercises the preprocessor (heavy macro use,
//! conditional blocks, standard headers), the lexer, the full parser, and
//! most of semantic analysis. It currently stops at a known architectural
//! limit: stress-effect for-loops bounded by the `nf` parameter need
//! runtime loops, which the dataflow lowering does not support yet.
//!
//! This test pins that exact frontier. If the model starts failing earlier
//! (lex/parse/preprocess regression) the assertion message changes and the
//! test fails. When runtime loops land, this test should be replaced by a
//! full-compile assertion.

use rspice_veriloga::VerilogACompiler;
use std::path::Path;

#[test]
fn bsim4_reaches_the_known_runtime_loop_frontier() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("models")
        .join("veriloga")
        .join("bsim4.va");
    if !path.exists() {
        eprintln!("bsim4.va not present; skipping frontier pin");
        return;
    }

    let result = VerilogACompiler::default().compile_file(&path);
    let err = match result {
        Ok(_) => {
            // Full compilation means runtime loops landed - tighten this
            // test into result checks instead of deleting it.
            return;
        }
        Err(e) => e.to_string(),
    };

    assert!(
        err.contains("for-loop condition must be a compile-time constant"),
        "BSIM4 must preprocess, lex, and parse cleanly, stopping only at \
         the documented runtime-loop limitation; got: {err}"
    );
}
