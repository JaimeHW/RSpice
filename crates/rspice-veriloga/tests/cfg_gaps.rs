//! The shipped models the new pipeline does not yet carry, and why.
//!
//! The corpus census reports these as "did not complete" among forty-odd that
//! do. That is the right shape for a census and the wrong shape for fixing
//! them: nine failures with no reasons is nine investigations starting from
//! nothing. This runs only those models, prints how far each one got and what
//! stopped it, and asserts nothing.
//!
//! It shrinks as they are fixed, and is deleted when it is empty.

use rspice_veriloga::canonical_ir::cfg_lower::CfgModel;
use rspice_veriloga::canonical_ir::{AdSeed, differentiate, optimize_cfg};
use rspice_veriloga::rust_backend::discover_veriloga_sources;
use rspice_veriloga::rust_backend::emit::{EmitBindings, emit_body};
use rspice_veriloga::{CompilerOptions, VerilogACompiler};
use std::path::{Path, PathBuf};

/// Module, directory under `models/veriloga`, file.
///
/// Was nine, then two, now one.
///
/// Six were a single construct — a model reading back its own contributed
/// current, as `I(di, si)` on a branch nothing solves for or as `I(<b>)` on a
/// port — and one more (`asmhemt`) was the first half of it.
///
/// `bsimsoi_va` (BSIM-SOI 4.6.1) was the eighth and is now in the census too. It
/// reads `MJSWG` before assigning it, from a block-local declaration shadowing a
/// parameter of the same name, and was held here deliberately rather than
/// defaulted to zero. What settled it was not a decision about this model:
/// `r3_cmc` turned out to do the same thing, Verilog-AMS initialises an analog
/// variable to zero, and every production compiler runs both. So an undefined
/// read is zero *and a warning* — which lets this lower while keeping the
/// shadowed-name question visible, which was the whole reason to hold it.
const GAPS: &[(&str, &str, &str)] = &[
    // `idt`, which is the state-slot allocation Phase 2 records as unfinished.
    (
        "PSPNQS104VA",
        "cmc/PSP104.1.0_vacode/vacode",
        "psp104_nqs.va",
    ),
];

#[test]
#[ignore = "compiles the models that do not yet complete; run with --ignored"]
fn the_remaining_gaps_report_what_stops_them() {
    let root = model_root();
    for (module, directory, file) in GAPS {
        let path = directory
            .split('/')
            .fold(root.clone(), |path, part| path.join(part))
            .join(file);
        if !path.exists() {
            eprintln!("{module:>14}  no such fixture: {}", path.display());
            continue;
        }
        eprintln!("{module:>14}  {}", diagnose(&root, &path, module));
    }
}

fn diagnose(root: &Path, path: &Path, module: &str) -> String {
    let mut options = CompilerOptions::default();
    options.include_paths.push(root.to_path_buf());
    if let Some(directory) = path.parent()
        && let Ok(candidates) = discover_veriloga_sources(directory)
        && let Some(candidate) = candidates.into_iter().find(|entry| entry.path == path)
    {
        options.defines = candidate.compile_profile.defines;
        options.undefines = candidate.compile_profile.undefines;
    }

    let compiled = match VerilogACompiler::new(options)
        .compile_file_canonical_ir_with_metadata(path, Some(module))
    {
        Ok(compiled) => compiled,
        Err(error) => return format!("front end: {error}"),
    };
    let artifact = compiled.artifact;

    let cfg = match CfgModel::from_hir(&artifact.hir, &artifact.mir) {
        Ok(cfg) => cfg,
        Err(diagnostics) => {
            let mut reasons: Vec<String> = diagnostics
                .iter()
                .map(|diagnostic| format!("{diagnostic:?}"))
                .collect();
            reasons.sort();
            reasons.dedup();
            return format!(
                "lowering: {} diagnostics, {} distinct: {}",
                diagnostics.len(),
                reasons.len(),
                reasons.join(" | ")
            );
        }
    };

    let lanes: Vec<AdSeed> = (0..artifact.mir.nodes.len())
        .map(|index| AdSeed::NodePotential(index.into()))
        .chain(
            (0..artifact.mir.branch_unknowns.len())
                .map(|index| AdSeed::BranchUnknownFlow(index.into())),
        )
        .collect();
    let mut differentiated = match differentiate(&cfg.function, &lanes) {
        Ok(differentiated) => differentiated,
        Err(error) => return format!("differentiation: {error}"),
    };

    let mut wanted = cfg.residuals.clone();
    for residual in &cfg.residuals.clone() {
        wanted.extend(differentiated.derivative_row(*residual).into_iter().flatten());
    }
    let (optimized, wanted) = optimize_cfg(&differentiated.function, &wanted);

    match emit_body(&optimized, &wanted, &EmitBindings::default()) {
        Ok((body, _)) => format!(
            "completes after all: {} values, {} blocks, {} bytes",
            optimized.values.len(),
            optimized.blocks.len(),
            body.len()
        ),
        Err(error) => format!("emission: {error}"),
    }
}

fn model_root() -> PathBuf {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("models")
        .join("veriloga");
    assert!(root.exists(), "model tree missing: {}", root.display());
    root
}
