//! What the new pipeline produces on real models.
//!
//! The audit that started the rebuild claimed the old backend's output scales
//! as operations x guard depth x derivative lanes, and that this is what makes
//! the largest models too big to emit. This is where that claim gets tested
//! against the replacement: lower, differentiate, simplify, emit, and count.
//!
//! Numbers, not assertions, are the product. A size that regresses is caught by
//! the recorded ceilings; a size that improves is meant to be read and the
//! ceiling lowered.

use rspice_veriloga::canonical_ir::cfg_lower::CfgModel;
use rspice_veriloga::canonical_ir::hir::HirRegion;
use rspice_veriloga::canonical_ir::schedule::split as split_cfg;
use rspice_veriloga::canonical_ir::{
    AdSeed, CanonicalIrArtifact, ValueId, differentiate, optimize_cfg, schedule_cfg,
};
use rspice_veriloga::rust_backend::discover_veriloga_sources;
use rspice_veriloga::rust_backend::emit::{EmitBindings, emit_body};
use rspice_veriloga::{CompilerOptions, VerilogACompiler};
use std::path::{Path, PathBuf};
use std::time::Instant;

/// Real models small enough to run on every build.
///
/// A three-terminal CMC resistor and the CMC diode: both are foundry-released
/// compact models with real guard structure, and both compile in seconds.
const QUICK_MODELS: &[(&str, &str, &str)] = &[
    ("r3_cmc", "cmc/r3_cmc_release1.1.2_2023Jun16", "r3_cmc.va"),
    (
        "DIODE_CMC",
        "cmc/diode_cmc_3.0_20250714/vacode",
        "diode_cmc.va",
    ),
    // Here because it is the corpus's worst case for emitted size, and a
    // regression on it is the one most worth catching early.
    ("EPFL_HEMT_10a", "epfl_hemt_3.0.0/vacode", "epfl_hemt.va"),
];

/// Ceilings on emitted bytes, measured. Lower them when the number drops.
const QUICK_CEILINGS: &[(&str, usize)] = &[
    // Measured 2026-07-29, with 3-6% headroom. The checked-in generated source
    // also includes device ABI and metadata around these emitted bodies.
    //
    // Both dropped again when derivative lanes were packed — r3_cmc 157,737 ->
    // 103,911, DIODE_CMC 559,701 -> 483,095. Invalidation staging and scalar
    // emission for one-lane shapes then brought them to 86,467 and 406,176.
    // DIODE_CMC gains least from scalar lanes: most of its live sets are two
    // wide, and its bulk is primal code from 530 source conditionals.
    ("r3_cmc", 90_000),
    ("DIODE_CMC", 420_000),
    // Was 6,260,006 while the function inliner still folded a branching body
    // into `guard ? value : previous`; 83,023 once it inlines as statements,
    // and 41,066 once lanes are packed, against 78,303 from the tier being
    // replaced. Current staged scalar-or-packed emission is 42,395. The
    // ceiling is deliberately tight: this model is the corpus's canary for
    // that class of blow-up.
    ("EPFL_HEMT_10a", 45_000),
];

#[test]
fn real_models_survive_the_whole_pipeline() {
    let root = model_root();
    for (module, directory, file) in QUICK_MODELS {
        let path = directory
            .split('/')
            .fold(root.clone(), |path, part| path.join(part))
            .join(file);
        assert!(path.exists(), "model fixture missing: {}", path.display());

        let report = run(&root, &path, module);
        report.print();

        let ceiling = QUICK_CEILINGS
            .iter()
            .find(|(name, _)| *name == *module)
            .map(|(_, ceiling)| *ceiling)
            .expect("every quick model needs a recorded ceiling");
        assert!(
            report.emitted_bytes <= ceiling,
            "{module} emitted {} bytes, above the recorded ceiling of {ceiling}",
            report.emitted_bytes
        );
    }
}

/// The same over the whole shipped tree. Slow, and the point is the census.
#[test]
#[ignore = "compiles every shipped model through the new pipeline; run with --ignored"]
fn the_whole_corpus_reports_its_size() {
    let root = model_root();
    let candidates = discover_veriloga_sources(&root).expect("model tree must be discoverable");
    for candidate in &candidates {
        for module in &candidate.modules {
            let report = std::panic::catch_unwind(|| run(&root, &candidate.path, module));
            match report {
                Ok(report) => report.print(),
                // The reason, not just the fact. A census that reports "did not
                // complete" for nine models leaves nine investigations to start
                // from nothing; the panic payload already says which stage gave
                // up and on what.
                Err(payload) => eprintln!("{module:>24}  did not complete: {}", reason(&payload)),
            }
        }
    }
}

/// What a caught panic was about. `run` panics with a formatted message naming
/// the stage, so the payload is a `String` or a `&str` in practice.
fn reason(payload: &Box<dyn std::any::Any + Send>) -> String {
    if let Some(message) = payload.downcast_ref::<String>() {
        return message.clone();
    }
    payload
        .downcast_ref::<&str>()
        .map_or_else(|| "panicked with a payload of no known type".to_string(), |message| (*message).to_string())
}

struct Report {
    module: String,
    nodes: usize,
    equations: usize,
    /// Expressions the shared front end produced. Separates "this model is
    /// large" from "the CFG lowering made it large".
    hir_expressions: usize,
    /// Conditionals in the structured body, which is where blocks come from.
    /// A count far above the `if`s in the source means something upstream —
    /// function inlining, macro expansion, loop unrolling — multiplied them.
    hir_conditionals: usize,
    hir_assignments: usize,
    lowered_values: usize,
    differentiated_values: usize,
    simplified_values: usize,
    blocks: usize,
    /// Values per invalidation class, coarsest first. What the per-class split
    /// has to work with: everything outside the last column is work a Newton
    /// iteration should never repeat.
    classes: [usize; 5],
    /// `(class, emitted bytes, cached values)` per stage.
    stage_bytes: Vec<(&'static str, usize, usize)>,
    emitted_bytes: usize,
    /// Milliseconds in the existing front end, then in each new pass. Split
    /// because the front end is shared with the backend being replaced, and a
    /// cost that lives there says nothing about this pipeline.
    front_end_ms: u128,
    lower_ms: u128,
    differentiate_ms: u128,
    optimize_ms: u128,
    emit_ms: u128,
}

impl Report {
    fn print(&self) {
        eprintln!(
            "{:>24}  {:>2} nodes  {:>3} eqs  hir {:>7}  {:>5} ifs  {:>6} assigns  \
             cfg {:>7}  +ad {:>8}  opt {:>7}  \
             {:>5} blocks  model {:>7} inst {:>7} temp {:>6} step {:>5} newt {:>7} \
             ({:>4.1}% newton)  \
             {:>9} bytes   front {:>6}ms  lower {:>5}ms  ad {:>5}ms  \
             opt {:>6}ms  emit {:>5}ms",
            self.module,
            self.nodes,
            self.equations,
            self.hir_expressions,
            self.hir_conditionals,
            self.hir_assignments,
            self.lowered_values,
            self.differentiated_values,
            self.simplified_values,
            self.blocks,
            self.classes[0],
            self.classes[1],
            self.classes[2],
            self.classes[3],
            self.classes[4],
            100.0 * self.classes[4] as f64 / self.classes.iter().sum::<usize>().max(1) as f64,
            self.emitted_bytes,
            self.front_end_ms,
            self.lower_ms,
            self.differentiate_ms,
            self.optimize_ms,
            self.emit_ms,
        );
        let stages: Vec<String> = self
            .stage_bytes
            .iter()
            .map(|(class, bytes, cached)| format!("{class} {bytes} B / {cached} cached"))
            .collect();
        eprintln!("{:>24}  stages: {}", "", stages.join("  |  "));
    }
}

fn run(root: &Path, path: &Path, module: &str) -> Report {
    let started = Instant::now();
    let artifact = compile(root, path, module);
    let front_end_ms = started.elapsed().as_millis();

    let started = Instant::now();
    let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir)
        .unwrap_or_else(|diagnostics| panic!("{module}: lowering: {diagnostics:?}"));
    let lower_ms = started.elapsed().as_millis();
    let lowered_values = cfg.function.values.len();

    let lanes: Vec<AdSeed> = (0..artifact.mir.nodes.len())
        .map(|index| AdSeed::NodePotential(index.into()))
        .chain(
            (0..artifact.mir.branch_unknowns.len())
                .map(|index| AdSeed::BranchUnknownFlow(index.into())),
        )
        .collect();
    let started = Instant::now();
    let mut differentiated = differentiate(&cfg.function, &lanes)
        .unwrap_or_else(|error| panic!("{module}: differentiation: {error}"));
    let differentiate_ms = started.elapsed().as_millis();
    let differentiated_values = differentiated.function.values.len();

    // Everything a stamp needs: the residual of each equation and every live
    // lane of it.
    let mut wanted = cfg.residuals.clone();
    for residual in &cfg.residuals {
        for lane in 0..lanes.len() {
            if let Some(derivative) = differentiated.derivative(*residual, lane) {
                wanted.push(derivative);
            }
        }
    }
    let started = Instant::now();
    let (optimized, wanted) = optimize_cfg(&differentiated.function, &wanted);
    let optimize_ms = started.elapsed().as_millis();

    let schedule = schedule_cfg(&optimized);
    let classes = schedule.census();
    // Emitted per stage, which is what the per-file limit is actually about:
    // the whole-body figure is the sum, and the largest stage is the file the
    // gate has to pass.
    let stages = split_cfg(&optimized, &schedule, &wanted)
        .unwrap_or_else(|error| panic!("{module}: split: {error}"));
    let mut stage_bytes = Vec::new();
    for stage in &stages {
        let produced: Vec<ValueId> = stage.outputs.iter().flatten().copied().collect();
        let (body, _) = emit_body(&stage.function, &produced, &EmitBindings::default())
            .unwrap_or_else(|error| panic!("{module}: {} stage: {error}", stage.class.name()));
        stage_bytes.push((stage.class.name(), body.len(), stage.exports.len()));
    }

    let started = Instant::now();
    let (body, _) = emit_body(&optimized, &wanted, &EmitBindings::default())
        .unwrap_or_else(|error| panic!("{module}: emission: {error}"));
    let emit_ms = started.elapsed().as_millis();

    Report {
        module: module.to_string(),
        nodes: artifact.mir.nodes.len(),
        equations: artifact.mir.equations.len(),
        hir_expressions: artifact.hir.expressions.len(),
        hir_conditionals: count_regions(&artifact.hir.body).0,
        hir_assignments: count_regions(&artifact.hir.body).1,
        lowered_values,
        differentiated_values,
        simplified_values: optimized.values.len(),
        blocks: optimized.blocks.len(),
        classes,
        stage_bytes,
        emitted_bytes: body.len(),
        front_end_ms,
        lower_ms,
        differentiate_ms,
        optimize_ms,
        emit_ms,
    }
}

fn compile(root: &Path, path: &Path, module: &str) -> CanonicalIrArtifact {
    let mut options = CompilerOptions::default();
    options.include_paths.push(root.to_path_buf());
    if let Some(profile) = profile_for(root, path) {
        options.defines = profile.0;
        options.undefines = profile.1;
    }
    VerilogACompiler::new(options)
        .compile_file_canonical_ir_with_metadata(path, Some(module))
        .unwrap_or_else(|error| panic!("{module}: front end: {error}"))
        .artifact
}

/// The compile profile discovery already worked out for this source.
fn profile_for(root: &Path, path: &Path) -> Option<(Vec<(String, Option<String>)>, Vec<String>)> {
    let directory = path.parent()?;
    let candidates = discover_veriloga_sources(directory).ok()?;
    let _ = root;
    candidates
        .into_iter()
        .find(|candidate| candidate.path == path)
        .map(|candidate| {
            (
                candidate.compile_profile.defines,
                candidate.compile_profile.undefines,
            )
        })
}

/// Conditionals and assignments in a structured body, counted recursively.
fn count_regions(regions: &[HirRegion]) -> (usize, usize) {
    let mut conditionals = 0;
    let mut assignments = 0;
    for region in regions {
        match region {
            HirRegion::Assignment(_) => assignments += 1,
            HirRegion::Contribution(_) => {}
            HirRegion::Conditional {
                then_body,
                else_body,
                ..
            } => {
                conditionals += 1;
                for body in [then_body, else_body] {
                    let (nested_conditionals, nested_assignments) = count_regions(body);
                    conditionals += nested_conditionals;
                    assignments += nested_assignments;
                }
            }
            HirRegion::Loop { body, .. } => {
                let (nested_conditionals, nested_assignments) = count_regions(body);
                conditionals += nested_conditionals;
                assignments += nested_assignments;
            }
        }
    }
    (conditionals, assignments)
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
