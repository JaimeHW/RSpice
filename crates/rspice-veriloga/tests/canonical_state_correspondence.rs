//! The map between the two lowerings of a module, checked against the models.
//!
//! [`HirExecutedCorrespondence`] exists for one reason: a CFG names its state
//! operators by body-copy expression id, and
//! [`rspice_veriloga::canonical_ir::CanonicalStateLayout`] numbers the executed
//! copy, because that is the copy whose records the runtime allocates and the
//! checkpoint serializes. Everything here is about whether that map is total
//! over the operators that actually exist, and whether it lands each one on the
//! *same* operator rather than merely on *an* operator.
//!
//! The focused tests pin the shapes. The corpus tests are the ones that matter:
//! a correspondence that works on a two-line fixture and misses one `ddt` in
//! BSIM-CMG is worth nothing, and the only way to know is to walk every model
//! and refuse to round up.

use rspice_veriloga::canonical_ir::cfg::{CfgStateSite, CfgValueKind};
use rspice_veriloga::canonical_ir::cfg_lower::CfgModel;
use rspice_veriloga::canonical_ir::{
    CanonicalIrArtifact, CanonicalStateFamily, CanonicalStateLayout, CanonicalStateOperator,
    CfgInvalidationClass, CfgStateAllocation, CfgStateAllocationError, ExprId, schedule_cfg,
};
use rspice_veriloga::ir::TransitionSiteId;
use rspice_veriloga::rust_backend::discover_veriloga_sources;
use rspice_veriloga::source::Span;
use rspice_veriloga::{CompilerOptions, VerilogACompiler};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

fn artifact(source: &str) -> CanonicalIrArtifact {
    VerilogACompiler::default()
        .compile_canonical_ir(source)
        .expect("fixture must compile to canonical IR")
}

fn lower(artifact: &CanonicalIrArtifact) -> CfgModel {
    match CfgModel::from_hir(&artifact.hir, &artifact.mir) {
        Ok(model) => model,
        Err(diagnostics) => panic!(
            "{}",
            diagnostics
                .iter()
                .map(|diagnostic| diagnostic.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        ),
    }
}

/// The case a positional pairing gets wrong, written out.
///
/// The module contributes before it assigns, so the executed order (assignments
/// then contributions) and the body order (source order) disagree: the `ddt`
/// written *first* owns the *second* integration slot. Pairing the two per-family
/// lists by position would hand each `ddt` the other's history, and a resumed
/// checkpoint would integrate the wrong operand — silently, and only in transient.
#[test]
fn a_module_that_contributes_before_it_assigns_still_pairs_each_operator_with_its_own_record() {
    let artifact = artifact(
        r#"
module crossed(p, n);
  inout p, n;
  electrical p, n;
  parameter real g = 1.0;
  real q;
  analog begin
    I(p, n) <+ ddt(V(p, n));
    if (g > 0.0) begin
      q = ddt(V(p, n) * 2.0);
    end else begin
      q = 0.0;
    end
    I(p, n) <+ q + idt(V(p, n));
  end
endmodule
"#,
    );
    let cfg = lower(&artifact);
    let allocation = CfgStateAllocation::build(&artifact.hir, &cfg.function)
        .unwrap_or_else(|errors| panic!("{}", render_allocation_errors(&errors)));

    // Three integration records, in the order the executed copy numbers them:
    // the assignment's `ddt` first, then the two contributions' operators.
    assert_eq!(
        allocation.family_len(CanonicalStateFamily::Integration),
        3,
        "one record per site, not per emission"
    );

    // The CFG meets them in source order. The slots it gets back are *not* that
    // order, which is the whole point.
    let slots: Vec<u32> = cfg_state_operators(&cfg)
        .into_iter()
        .map(|(operator, kind)| {
            allocation
                .slot(operator)
                .unwrap_or_else(|| panic!("{} at {operator} owns no record", kind.name()))
        })
        .collect();
    assert_eq!(
        slots,
        vec![1, 0, 2],
        "the contribution written first owns slot 1, because the executed copy \
         runs assignments before contributions"
    );
}

/// A guard folded around an assignment must not be walked into.
///
/// The executed copy of a guarded assignment is `guard ? written : previous`,
/// and its `previous` arm re-reads the target — which for a `ddt`-valued
/// variable is not a `ddt` at all. Pairing the region against the whole executed
/// root instead of its then-arm would put the operator on the wrong node, so the
/// congruence check would reject the run and the operator would come back
/// unmapped rather than wrong. This asserts it comes back *right*.
#[test]
fn a_guarded_assignment_pairs_with_the_then_arm_of_its_executed_copy() {
    let artifact = artifact(
        r#"
module guarded(p, n);
  inout p, n;
  electrical p, n;
  parameter real on = 1.0;
  real q;
  analog begin
    q = 0.0;
    if (on > 0.5) begin
      q = ddt(V(p, n));
    end
    I(p, n) <+ q;
  end
endmodule
"#,
    );
    let cfg = lower(&artifact);
    let allocation = CfgStateAllocation::build(&artifact.hir, &cfg.function)
        .unwrap_or_else(|errors| panic!("{}", render_allocation_errors(&errors)));
    let layout = CanonicalStateLayout::from_hir(&artifact.hir);

    let operators = cfg_state_operators(&cfg);
    assert_eq!(operators.len(), 1, "one ddt in the module");
    let (operator, kind) = operators[0];
    assert_eq!(kind, CanonicalStateOperator::Ddt);

    let executed = artifact
        .hir
        .executed_correspondence
        .executed(operator)
        .expect("the guarded assignment's ddt must map");
    assert_ne!(
        executed, operator,
        "the two copies are different expressions"
    );
    assert_eq!(
        layout.site(executed).map(|site| site.kind),
        Some(CanonicalStateOperator::Ddt),
        "the map must land on a ddt, not on the guard or the fallback"
    );
    assert_eq!(allocation.slot(operator), Some(0));
}

/// A loop condition is folded with `&&`, not with a select, and the walk off it
/// has to know which.
#[test]
fn a_guarded_loop_condition_pairs_with_the_right_operand_of_its_executed_copy() {
    let artifact = artifact(
        r#"
module looped(p, n);
  inout p, n;
  electrical p, n;
  parameter real n_iter = 3.0;
  parameter real on = 1.0;
  real acc;
  real i;
  analog begin
    acc = 0.0;
    if (on > 0.5) begin
      i = 0.0;
      while (i < n_iter) begin
        acc = acc + V(p, n);
        i = i + 1.0;
      end
    end
    I(p, n) <+ acc;
  end
endmodule
"#,
    );
    // The loop body's assignments are inside the region tree and must pair; a
    // failure here shows up as a body expression with no executed counterpart.
    let unmapped = unmapped_body_expressions(&artifact);
    assert!(
        unmapped.is_empty(),
        "loop-body expressions must pair: {unmapped:?}"
    );
}

/// The correspondence is a map, not a merge.
///
/// Two syntactically identical operators at different program points are two
/// records, and a map that collapsed them would make one `ddt` integrate the
/// other's operand.
#[test]
fn two_identical_operators_at_different_points_keep_separate_records() {
    let artifact = artifact(
        r#"
module twice(p, n);
  inout p, n;
  electrical p, n;
  parameter real on = 1.0;
  real a;
  real b;
  analog begin
    a = ddt(V(p, n));
    b = ddt(V(p, n));
    I(p, n) <+ a + b;
  end
endmodule
"#,
    );
    let cfg = lower(&artifact);
    let allocation = CfgStateAllocation::build(&artifact.hir, &cfg.function)
        .unwrap_or_else(|errors| panic!("{}", render_allocation_errors(&errors)));

    let operators = cfg_state_operators(&cfg);
    assert_eq!(operators.len(), 2);
    let slots: BTreeSet<u32> = operators
        .iter()
        .map(|(operator, _)| allocation.slot(*operator).expect("mapped"))
        .collect();
    assert_eq!(
        slots,
        BTreeSet::from([0, 1]),
        "two textually identical ddt sites own two records"
    );
    assert_eq!(allocation.family_len(CanonicalStateFamily::Integration), 2);
}

/// A module with no dynamic operators allocates nothing and refuses nothing.
#[test]
fn a_static_module_needs_no_records_and_reports_no_refusals() {
    let artifact = artifact(
        r#"
module resistor(p, n);
  inout p, n;
  electrical p, n;
  parameter real r = 1000.0;
  analog I(p, n) <+ V(p, n) / r;
endmodule
"#,
    );
    let cfg = lower(&artifact);
    let allocation =
        CfgStateAllocation::build(&artifact.hir, &cfg.function).expect("no operators to refuse");
    for family in [
        CanonicalStateFamily::Integration,
        CanonicalStateFamily::CrossDetector,
        CanonicalStateFamily::TransitionFilter,
        CanonicalStateFamily::ZiFilter,
    ] {
        assert_eq!(allocation.family_len(family), 0, "{family:?}");
    }
    assert!(
        allocation.agrees_with_emission_allocation(&artifact.hir),
        "a module whose assignments own nothing cannot double-allocate"
    );
}

/// Every kind that owns a record must be scheduled at Newton scope.
///
/// The two lists are written in different modules for different readers —
/// `state_site` says "this owns runtime state", `leaf_class` says "this cannot
/// be cached" — and they are the same list. When they drift, a stateful operator
/// gets computed once per model card and its waveform freezes, which is the
/// shipped `transition` defect W-A fixed. This is the check that catches the
/// next one at the point a kind is added rather than at the point a model
/// misbehaves.
#[test]
fn every_state_bearing_cfg_kind_is_scheduled_at_newton_scope() {
    // One module holding one of everything the CFG can name a record for, so
    // the check is over real values rather than over constructed enum variants.
    // `zi_*` is assigned to a variable rather than contributed: the front end
    // refuses a direct contribution whose transition time is not strictly
    // positive (VAMS-2023 4.5.12), and this fixture is about scheduling.
    //
    // `transition` is absent because the CFG level cannot lower it — see
    // `the_cfg_level_refuses_transition_by_name`.
    let artifact = artifact(
        r#"
module every_operator(p, n);
  inout p, n;
  electrical p, n;
  real filtered;
  analog begin
    filtered = zi_nd(V(p, n), '{0.25}, '{1.0, -0.75}, 1e-6, 0.0);
    I(p, n) <+ ddt(V(p, n))
             + idt(V(p, n))
             + idtmod(V(p, n), 0.0, 2.0, 0.0)
             + absdelay(V(p, n), 1e-9)
             + slew(V(p, n), 1e6)
             + cross(V(p, n) - 1.0, 1)
             + above(V(p, n), 1.0)
             + last_crossing(V(p, n) - 2.0, 1)
             + laplace_nd(V(p, n), '{1.0}, '{1.0, 1e-9})
             + filtered;
  end
endmodule
"#,
    );
    let cfg = lower(&artifact);
    let schedule = schedule_cfg(&cfg.function);

    let mut kinds = BTreeSet::new();
    for value in &cfg.function.values {
        let Some(site) = value.kind.state_site() else {
            continue;
        };
        if let CfgStateSite::Operator(_, kind) = site {
            kinds.insert(kind.name());
        }
        let class = schedule.class(value.id);
        assert_eq!(
            class,
            CfgInvalidationClass::Newton,
            "{:?} owns a state record but is scheduled at {class:?}; a coarser \
             scope computes its history once and freezes it",
            value.kind
        );
    }
    // Every operator the CFG level can lower a record for. `transition` and
    // `$table_model` are the two `CanonicalStateOperator` variants absent, both
    // because the level cannot lower them at all.
    assert_eq!(
        kinds,
        BTreeSet::from([
            "ddt", "idt", "idtmod", "absdelay", "slew", "cross", "above", "laplace", "zi"
        ]),
        "the fixture must exercise every operator-keyed record the CFG can name"
    );

    // And the allocation names one for each of them.
    let allocation = CfgStateAllocation::build(&artifact.hir, &cfg.function)
        .unwrap_or_else(|errors| panic!("{}", render_allocation_errors(&errors)));
    for (operator, kind) in cfg_state_operators(&cfg) {
        assert!(
            allocation.slot(operator).is_some(),
            "{} at {operator} owns no record",
            kind.name()
        );
    }
}

/// `transition` never reaches the CFG level, and the allocator refuses it by
/// name if one ever does.
///
/// Two separate facts, both worth pinning. The front end lowers `transition` to
/// an ordinary `Call` — the parser produces no `AnalogOperator::Transition` node
/// and the semantic analyzer's arms for one are unreachable — so `cfg_lower`
/// meets it in the call arm and says so. And `CfgValueKind::Transition` is
/// nevertheless a live IR variant that `ad` and `cfg_eval` handle, so the
/// allocator has to answer for it: it is the one operator the CFG names by its
/// own site identity rather than by expression, the two lowerings mint different
/// ordinals for one source site, and the correspondence is a map over
/// expressions. Refused, never guessed.
#[test]
fn the_cfg_level_refuses_transition_by_name() {
    let ramped = artifact(
        r#"
module ramped(p, n);
  inout p, n;
  electrical p, n;
  parameter real tr = 1e-9;
  analog I(p, n) <+ transition(V(p, n) > 1.0 ? 1.0 : 0.0, 0.0, tr, tr);
endmodule
"#,
    );
    let diagnostics = CfgModel::from_hir(&ramped.hir, &ramped.mir)
        .expect_err("the CFG level cannot lower transition");
    assert!(
        diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("transition")),
        "the refusal must name the construct: {diagnostics:?}"
    );

    // Now the allocator's own refusal, over a CFG that does carry one. Built by
    // planting the value rather than by compiling one, because the front end
    // cannot produce it — which is exactly why the refusal needs a test.
    let base = artifact(
        r#"
module plain(p, n);
  inout p, n;
  electrical p, n;
  analog I(p, n) <+ ddt(V(p, n));
endmodule
"#,
    );
    let mut cfg = lower(&base);
    let victim = cfg
        .function
        .values
        .iter()
        .position(|value| matches!(value.kind, CfgValueKind::Ddt { .. }))
        .expect("the fixture has a ddt");
    let CfgValueKind::Ddt { input, .. } = cfg.function.values[victim].kind else {
        unreachable!("just matched")
    };
    cfg.function.values[victim].kind = CfgValueKind::Transition {
        site: TransitionSiteId::from_span(Span::dummy()),
        input,
        delay: input,
        rise: input,
        fall: input,
    };

    let errors = CfgStateAllocation::build(&base.hir, &cfg.function)
        .expect_err("a transition must be refused, not allocated");
    assert!(
        errors
            .iter()
            .any(|error| matches!(error, CfgStateAllocationError::UnsupportedTransition { .. })),
        "the refusal must name transition: {}",
        render_allocation_errors(&errors)
    );
}

// ---------------------------------------------------------------------------
// Corpus
// ---------------------------------------------------------------------------

/// The subset that compiles fast enough to walk on an ordinary run.
///
/// Chosen for shape rather than for size: `vbic_1.3` has `ddt` under control
/// flow and self-heating, `epfl_hemt_3.0.0` is the module whose conditionals
/// produce the largest guard-folded trees in the corpus, `ekv26_2.6` and the
/// `cmc` entries carry `$limit` and the resistor/diode families.
const FAST_CORPUS: &[&str] = &[
    "vbic_1.3",
    "angelov_2.0",
    "epfl_hemt_3.0.0",
    "ekv26_2.6",
    "cmc/r3_cmc_release1.1.2_2023Jun16",
    "cmc/r2_cmc_v1.0.2",
    "cmc/diode_cmc_3.0_20250714",
];

#[test]
fn the_fast_corpus_maps_every_cfg_state_operator_onto_its_own_executed_record() {
    let root = model_root();
    let roots: Vec<PathBuf> = FAST_CORPUS
        .iter()
        .map(|relative| {
            let path = relative
                .split('/')
                .fold(root.clone(), |path, part| path.join(part));
            assert!(
                path.exists(),
                "fast-corpus entry missing: {}",
                path.display()
            );
            path
        })
        .collect();

    let census = survey(&root, &roots);
    census.report("fast corpus");
    assert!(
        census.modules > 0,
        "the fast corpus must produce modules to check"
    );
    assert!(
        census.refusals.is_empty(),
        "every CFG state operator must name its own executed record:\n{}",
        census.refusals.join("\n")
    );
}

#[test]
#[ignore = "walks the whole shipped model tree, which takes tens of minutes"]
fn the_whole_corpus_maps_every_cfg_state_operator_onto_its_own_executed_record() {
    let root = model_root();
    let census = survey(&root, std::slice::from_ref(&root));
    census.report("full corpus");
    assert!(
        census.refusals.is_empty(),
        "every CFG state operator must name its own executed record:\n{}",
        census.refusals.join("\n")
    );
}

#[derive(Default)]
struct Census {
    modules: usize,
    operators: usize,
    /// Per model: how many state operators it has, by kind.
    per_model: BTreeMap<String, BTreeMap<&'static str, usize>>,
    refusals: Vec<String>,
}

impl Census {
    fn report(&self, label: &str) {
        eprintln!(
            "state correspondence {label}: {} modules, {} state operators, {} refusals",
            self.modules,
            self.operators,
            self.refusals.len()
        );
        for (model, kinds) in &self.per_model {
            if kinds.is_empty() {
                continue;
            }
            let summary = kinds
                .iter()
                .map(|(kind, count)| format!("{kind}={count}"))
                .collect::<Vec<_>>()
                .join(" ");
            eprintln!("  {model}: {summary}");
        }
        for refusal in &self.refusals {
            eprintln!("  REFUSED {refusal}");
        }
    }
}

fn survey(include_root: &Path, roots: &[PathBuf]) -> Census {
    let mut census = Census::default();
    for search_root in roots {
        let candidates =
            discover_veriloga_sources(search_root).expect("model tree must be discoverable");
        for candidate in &candidates {
            let mut options = CompilerOptions::default();
            options.include_paths.push(include_root.to_path_buf());
            options.defines = candidate.compile_profile.defines.clone();
            options.undefines = candidate.compile_profile.undefines.clone();
            let compiler = VerilogACompiler::new(options);

            for module in &candidate.modules {
                let Ok(compiled) =
                    compiler.compile_file_canonical_ir_with_metadata(&candidate.path, Some(module))
                else {
                    // A module the front end refuses is a different concern;
                    // this measures the map, not the parser.
                    continue;
                };
                let artifact = compiled.artifact;
                let Ok(cfg) = CfgModel::from_hir(&artifact.hir, &artifact.mir) else {
                    // Likewise a module the CFG level cannot lower yet: it has
                    // no CFG operators to map.
                    continue;
                };
                census.modules += 1;
                let mut kinds: BTreeMap<&'static str, usize> = BTreeMap::new();
                for (_, kind) in cfg_state_operators(&cfg) {
                    *kinds.entry(kind.name()).or_default() += 1;
                    census.operators += 1;
                }
                let transitions = cfg
                    .function
                    .values
                    .iter()
                    .filter_map(|value| value.kind.state_site())
                    .filter(|site| matches!(site, CfgStateSite::Transition(_)))
                    .count();
                if transitions > 0 {
                    *kinds.entry("transition").or_default() += transitions;
                    census.operators += transitions;
                }
                census.per_model.insert(module.to_string(), kinds);

                if let Err(errors) = CfgStateAllocation::build(&artifact.hir, &cfg.function) {
                    for error in errors {
                        census.refusals.push(format!("{module}: {error}"));
                    }
                }
            }
        }
    }
    census
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Every distinct operator-keyed state site the CFG names, in value order.
fn cfg_state_operators(cfg: &CfgModel) -> Vec<(ExprId, CanonicalStateOperator)> {
    let mut seen = BTreeSet::new();
    let mut operators = Vec::new();
    for value in &cfg.function.values {
        if let Some(CfgStateSite::Operator(operator, kind)) = value.kind.state_site()
            && seen.insert(operator)
        {
            operators.push((operator, kind));
        }
    }
    operators
}

/// Body expressions the correspondence does not cover, as `(id, kind label)`.
fn unmapped_body_expressions(artifact: &CanonicalIrArtifact) -> Vec<(usize, String)> {
    // Body expressions are the tail of the arena: the executed copy is lowered
    // first, so the first body id is the lowest `body_start` any run reports.
    let Some(first_body) = artifact
        .hir
        .executed_correspondence
        .spans()
        .iter()
        .map(|span| span.body_start)
        .min()
    else {
        return Vec::new();
    };
    let mut unmapped = Vec::new();
    for expression in artifact.hir.expressions.iter().skip(first_body as usize) {
        let id = expression.id;
        if usize::from(id) >= artifact.hir.expressions.len() {
            continue;
        }
        if artifact.hir.executed_correspondence.executed(id).is_none() {
            unmapped.push((usize::from(id), format!("{:?}", expression.kind)));
        }
    }
    unmapped
}

fn render_allocation_errors(errors: &[CfgStateAllocationError]) -> String {
    errors
        .iter()
        .map(|error| error.to_string())
        .collect::<Vec<_>>()
        .join("\n")
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
