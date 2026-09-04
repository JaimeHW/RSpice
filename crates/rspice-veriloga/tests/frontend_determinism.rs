//! One source compiles to one program, whatever order the hash maps walk in.
//!
//! Rust's default hasher is seeded per `RandomState`, and a fresh seed is
//! drawn for every map a process builds, so compiling one source twice inside
//! one process already walks every `HashMap` in a different order. A pass
//! whose *output* order comes from such a walk — an emission order, a slot
//! number, a declaration list — therefore shows up here as two different
//! programs from one source, and shows up in production as a compile that
//! fails one time in N with an internal error naming a temporary.
//!
//! These tests are the cheap standing check for that class. They do not need a
//! seeded hasher or a repeated process: the per-map seed does the shuffling.

use rspice_veriloga::ir::DeviceIR;
use rspice_veriloga::rust_backend::discover_veriloga_sources;
use rspice_veriloga::{
    CompiledModel, CompilerOptions, Lexer, Parser, SemanticAnalyzer, SourceMap, VerilogACompiler,
};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

/// A model whose arrays are declared out of alphabetical order and are read
/// through a runtime index, so every array carries a derivative shadow run.
///
/// The run bases are handed out one array at a time, so the order the arrays
/// are walked in decides the variable slot every runtime-indexed derivative
/// read addresses. Nothing else in the module varies.
const RUNTIME_INDEXED_ARRAYS: &str = r#"
`include "disciplines.vams"
module ordered_arrays(p, n);
    inout p, n;
    electrical p, n;
    parameter integer count = 3 from [1:8];
    real zc[0:7];
    real ma[0:7];
    real ab[0:7];
    real total;
    integer i;
    analog begin
        total = 0.0;
        for (i = 0; i < count; i = i + 1) begin
            zc[i] = V(p, n) * (i + 1);
            ma[i] = zc[i] * V(p, n);
            ab[i] = ma[i] + zc[i];
            total = total + ab[i] * 1.0e-3;
        end
        I(p, n) <+ total;
    end
endmodule
"#;

/// FNV-1a over a compiled model's `Debug` rendering, hashed as it is written.
///
/// `Debug` is used rather than an encoding because it is total over the model
/// — every slot, program and name — and because it renders floats exactly.
/// The bytes are folded in as they arrive so a shipped model's rendering is
/// never held as one string.
struct ProgramDigest(u64);

impl ProgramDigest {
    fn of(model: &CompiledModel) -> String {
        let mut digest = Self(0xcbf2_9ce4_8422_2325);
        write!(digest, "{model:?}").expect("hashing a Debug rendering cannot fail");
        format!("{:016x}", digest.0)
    }
}

impl std::fmt::Write for ProgramDigest {
    fn write_str(&mut self, text: &str) -> std::fmt::Result {
        for byte in text.as_bytes() {
            self.0 ^= u64::from(*byte);
            self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3);
        }
        Ok(())
    }
}

/// Compile one source `rounds` times in this process and report the distinct
/// programs it produced, each with the first round that produced it.
fn distinct_programs(source: &str, rounds: usize) -> Vec<(usize, String)> {
    let mut seen: Vec<(usize, String)> = Vec::new();
    for round in 0..rounds {
        let model = VerilogACompiler::new(CompilerOptions::default())
            .compile(source)
            .unwrap_or_else(|error| panic!("round {round} failed to compile: {error}"));
        let digest = ProgramDigest::of(&model);
        if !seen.iter().any(|(_, known)| *known == digest) {
            seen.push((round, digest));
        }
    }
    seen
}

fn analyzed_module(source: &str, module: &str) -> rspice_veriloga::semantic::AnalyzedModule {
    let mut sources = SourceMap::new();
    let source_id = sources.add_source_mut("<determinism>", source);
    let tokens = Lexer::new(source, source_id)
        .collect_tokens()
        .expect("tokenize");
    let parsed = Parser::new(&tokens).parse().expect("parse");
    let analyzed = SemanticAnalyzer::new().analyze(&parsed).expect("analyze");
    analyzed
        .modules
        .get(module)
        .cloned()
        .expect("the module is declared")
}

#[test]
fn repeated_compiles_of_one_source_agree_on_the_program() {
    let programs = distinct_programs(RUNTIME_INDEXED_ARRAYS, 24);
    assert_eq!(
        programs.len(),
        1,
        "one source compiled to {} different programs across 24 compiles in one process, \
         first seen at rounds {:?}; a pass is ordering its output by hash iteration",
        programs.len(),
        programs.iter().map(|(round, _)| *round).collect::<Vec<_>>()
    );
}

#[test]
fn array_layout_order_is_a_function_of_the_module_not_of_hash_order() {
    // Declared zc, ma, ab; sorted is ab, ma, zc. A walk of the analyzed
    // module's array map would answer neither reliably.
    let module = analyzed_module(RUNTIME_INDEXED_ARRAYS, "ordered_arrays");
    let ir = DeviceIR::from_analyzed(&module).expect("the module builds an IR");
    let names: Vec<&str> = ir.arrays.iter().map(|array| array.name.as_str()).collect();
    assert_eq!(
        names,
        vec!["ab", "ma", "zc"],
        "array layouts must be ordered by name, as the canonical HIR orders them"
    );

    // The shadow runs are handed bases in that same order, so the ordering
    // reaches the program rather than stopping at a lookup table.
    let first_run = ir
        .variables
        .iter()
        .position(|variable| variable.name.starts_with("ab[0]@d"))
        .expect("the first array's shadow run exists");
    for later in ["ma[0]@d", "zc[0]@d"] {
        let position = ir
            .variables
            .iter()
            .position(|variable| variable.name.starts_with(later))
            .expect("every live array carries a shadow run");
        assert!(
            position > first_run,
            "shadow runs follow the array order: {later} was allocated before ab's"
        );
    }
}

/// The model root the shipped-model censuses compile.
fn shipped_model_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("models")
        .join("veriloga")
}

/// Compile shipped modules the way the censuses do, `rounds` times each.
///
/// Ignored because it compiles real CMC sources: `hisimsotb_va` alone is a
/// few seconds a round, the whole corpus minutes. Two knobs widen it after
/// touching a pass that orders variables, slots or emission:
///
/// * `RSPICE_DETERMINISM_ROUNDS` — compiles per module (default 2);
/// * `RSPICE_DETERMINISM_MODULE` — the module to compile, or `all` for every
///   shipped module (default `hisimsotb_va`, the one a census caught a
///   nondeterministic `Unknown variable` on).
#[test]
#[ignore = "recompiles shipped CMC models; seconds per round, minutes for `all`"]
fn shipped_modules_compile_to_one_program_each() {
    let rounds = std::env::var("RSPICE_DETERMINISM_ROUNDS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(2);
    let wanted =
        std::env::var("RSPICE_DETERMINISM_MODULE").unwrap_or_else(|_| "hisimsotb_va".to_string());
    let root = shipped_model_root();
    let candidates = discover_veriloga_sources(&root).expect("discover shipped Verilog-A sources");

    let mut compiled = 0_usize;
    let mut divergent: Vec<String> = Vec::new();
    for candidate in &candidates {
        for module in &candidate.modules {
            if wanted != "all" && *module != wanted {
                continue;
            }
            compiled += 1;
            let mut seen: Vec<String> = Vec::new();
            for round in 0..rounds {
                let mut options = CompilerOptions::default();
                options.include_paths.push(root.clone());
                options.defines = candidate.compile_profile.defines.clone();
                options.undefines = candidate.compile_profile.undefines.clone();
                let started = std::time::Instant::now();
                let model = VerilogACompiler::new(options)
                    .compile_file_module(&candidate.path, Some(module))
                    .unwrap_or_else(|error| {
                        panic!("{module} round {round} failed to compile: {error}")
                    });
                let digest = ProgramDigest::of(&model);
                eprintln!(
                    "{module} round {round}: {digest} in {:.1}s",
                    started.elapsed().as_secs_f64()
                );
                if !seen.contains(&digest) {
                    seen.push(digest);
                }
            }
            if seen.len() > 1 {
                divergent.push(format!("{module}: {seen:?}"));
            }
        }
    }

    assert!(compiled > 0, "no shipped module matched '{wanted}'");
    assert!(
        divergent.is_empty(),
        "{} of {compiled} shipped modules compiled to more than one program \
         across {rounds} compiles: {divergent:?}",
        divergent.len()
    );
}
