//! Front-end cost profile over the shipped Verilog-A corpus.
//!
//! One line per compiled module reporting wall time, every recorded pipeline
//! phase, the two canonical phase digests, a bytecode identity digest, and the
//! arena sizes that explain the cost. The digests are what make this harness
//! usable as an optimisation gate: a pass may only get cheaper if
//! `hir_digest`, `mir_digest` and `bytecode_digest` are unchanged, so the
//! table doubles as the before/after identity record.
//!
//! Timing is evidence, never identity, so nothing here is asserted. Run it
//! with:
//!
//! ```text
//! cargo test --release -p rspice-veriloga --test frontend_profile \
//!     -- --ignored --nocapture --test-threads=1
//! ```
//!
//! `RSPICE_FRONTEND_PROFILE_FILTER` narrows the run to modules whose name
//! contains the given substring. `RSPICE_VERILOGA_PHASE_TRACE` and
//! `RSPICE_VERILOGA_COMPILE_TIMINGS` are honoured by the compiler itself and
//! add the finer intra-phase spans to stderr.

use std::path::{Path, PathBuf};
use std::time::Instant;

use rspice_veriloga::metrics::PipelinePhase;
use rspice_veriloga::rust_backend::discover_veriloga_sources;
use rspice_veriloga::{CompilerOptions, VerilogACompiler};

/// Digests a serialization as it is produced.
///
/// The identity this harness reports is a BLAKE3 over the compiled model's
/// JSON encoding. Buffering that encoding first would make the harness itself
/// the largest allocation in the run — a HiSIM-class model encodes to well
/// over a gigabyte — which perturbs the timings the same run is trying to
/// measure. `serde_json::to_vec` is `to_writer` into a `Vec`, so the bytes,
/// and therefore the digest, are the same either way.
struct DigestWriter {
    hasher: blake3::Hasher,
    bytes: usize,
}

impl std::io::Write for DigestWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.hasher.update(buf);
        self.bytes += buf.len();
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn shipped_model_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("models")
        .join("veriloga")
}

/// Every phase the runtime route can record, in pipeline order, so a table row
/// has the same columns whether or not a model exercised a phase.
const REPORTED_PHASES: &[PipelinePhase] = &[
    PipelinePhase::Preprocess,
    PipelinePhase::Lex,
    PipelinePhase::Parse,
    PipelinePhase::Semantic,
    PipelinePhase::BytecodeGeneration,
    PipelinePhase::HirLowering,
    PipelinePhase::MirLowering,
    PipelinePhase::CanonicalNoisePlanning,
    PipelinePhase::IntegrityValidation,
];

#[test]
#[ignore = "profile harness; run with --release -- --ignored --nocapture --test-threads=1"]
fn shipped_model_frontend_profile() {
    let root = shipped_model_root();
    let candidates = discover_veriloga_sources(&root).expect("discover shipped Verilog-A sources");
    let filter = std::env::var("RSPICE_FRONTEND_PROFILE_FILTER").ok();

    let mut models = 0_usize;
    let mut total_wall = 0.0_f64;
    let mut phase_totals = vec![0.0_f64; REPORTED_PHASES.len()];

    for candidate in &candidates {
        for module in &candidate.modules {
            if filter
                .as_deref()
                .is_some_and(|filter| !module.contains(filter))
            {
                continue;
            }

            let mut options = CompilerOptions::default();
            options.include_paths.push(root.clone());
            options.defines = candidate.compile_profile.defines.clone();
            options.undefines = candidate.compile_profile.undefines.clone();
            let compiler = VerilogACompiler::new(options);

            let started = Instant::now();
            let runtime = compiler
                .compile_file_runtime_with_metadata(&candidate.path, Some(module))
                .unwrap_or_else(|error| {
                    panic!("compile {} :: {module}: {error}", candidate.path.display())
                });
            let wall = started.elapsed().as_secs_f64();

            let artifact = &runtime.canonical_ir;
            let mut bytecode = DigestWriter {
                hasher: blake3::Hasher::new(),
                bytes: 0,
            };
            serde_json::to_writer(&mut bytecode, &runtime.model).expect("serialize compiled model");
            let bytecode_digest = bytecode.hasher.finalize().to_hex().to_string();

            let mut line = format!("frontend-profile model={module} wall={wall:.3}");
            for (slot, phase) in REPORTED_PHASES.iter().enumerate() {
                let seconds = runtime.metrics.phase_elapsed(*phase).as_secs_f64();
                phase_totals[slot] += seconds;
                line.push_str(&format!(" phase.{phase}={seconds:.3}"));
            }
            line.push_str(&format!(
                " hir_digest={} mir_digest={} bytecode_digest={bytecode_digest}",
                artifact.hir_digest, artifact.mir_digest
            ));
            line.push_str(&format!(
                " hir_expressions={} hir_contributions={} hir_statements={} hir_body={} \
                 hir_variables={} hir_arrays={} hir_branches={} \
                 mir_equations={} mir_expressions={} mir_state_slots={} \
                 stamp_programs={} assignment_steps={} noise_assignment_steps={} \
                 noise_sources={} canonical_noise_sources={} bytecode_bytes={}",
                artifact.hir.expressions.len(),
                artifact.hir.contributions.len(),
                artifact.hir.statements.len(),
                artifact.hir.body.len(),
                artifact.hir.variables.len(),
                artifact.hir.arrays.len(),
                artifact.hir.branches.len(),
                artifact.mir.equations.len(),
                artifact.mir.expressions.len(),
                artifact.mir.state_slots.len(),
                runtime.model.stamp_programs.len(),
                runtime.model.assignment_steps.len(),
                runtime.model.noise_assignment_steps.len(),
                runtime.model.noise_sources.len(),
                artifact.noise_sources.sources.len(),
                bytecode.bytes,
            ));
            println!("{line}");

            models += 1;
            total_wall += wall;
        }
    }

    let mut totals = format!("frontend-profile-total models={models} wall={total_wall:.3}");
    for (slot, phase) in REPORTED_PHASES.iter().enumerate() {
        totals.push_str(&format!(" phase.{phase}={:.3}", phase_totals[slot]));
    }
    println!("{totals}");

    assert!(models > 0, "the profile filter matched no shipped module");
}
