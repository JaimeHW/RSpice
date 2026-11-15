//! Artifact precision and sealing regression coverage.

use super::PreparedVerilogARuntime;

pub(crate) fn standalone_connection_fixture() -> (super::PreparedVerilogARuntimeSet, String) {
    use rspice_design::project_sources::{
        MAX_PROJECT_CODE_SOURCE_BYTES, MAX_PROJECT_SOURCE_BUNDLE_BYTES,
        MAX_PROJECT_SOURCE_DEPENDENCY_DEPTH, MAX_PROJECT_SOURCE_FILES,
        MAX_PROJECT_SOURCE_LOGICAL_PATH_BYTES,
    };
    let mut rules = rspice_veriloga::connect::library::BUILTIN_CONNECT_MODULES
        .iter()
        .map(|(_, source)| *source)
        .collect::<String>();
    rules.push_str("\nconnectrules Low; connect d2a #(.vsup(1.0)); endconnectrules\nconnectrules High; connect d2a #(.vsup(5.0)); endconnectrules\n");
    let (_, library) = crate::model_import::import_source_bundle(
        "standalone-ui-connections.lib",
        None,
        vec![
            ("root.lib".into(), b".va \"driver.vams\" UI_DRIVER\n.va \"rules.vams\" UI_CONNECTIONS\n".to_vec()),
            ("driver.vams".into(), b"module ui_driver(p,q); inout p; electrical p; output q; reg q; initial q=1; analog I(p)<+0; endmodule\n".to_vec()),
            ("rules.vams".into(), rules.into_bytes()),
        ],
        None,
        rspice_model_library::source_bundle::ImportLimits {
            max_files: MAX_PROJECT_SOURCE_FILES,
            max_total_bytes: MAX_PROJECT_SOURCE_BUNDLE_BYTES,
        },
        rspice_veriloga::VirtualCompileLimits {
            max_files: MAX_PROJECT_SOURCE_FILES,
            max_path_bytes: MAX_PROJECT_SOURCE_LOGICAL_PATH_BYTES,
            max_file_bytes: MAX_PROJECT_CODE_SOURCE_BYTES,
            max_total_source_bytes: MAX_PROJECT_SOURCE_BUNDLE_BYTES,
            max_include_depth: MAX_PROJECT_SOURCE_DEPENDENCY_DEPTH,
            max_expanded_bytes: MAX_PROJECT_SOURCE_BUNDLE_BYTES.saturating_mul(2),
            max_module_name_bytes: 128,
        },
    ).expect("checked model-source import");
    let mut catalog = rspice_model_library::ModelCatalog::default();
    catalog.add_library(library);
    let sealed = crate::model_sources::seal_catalog_execution_sources(
        &catalog,
        &rspice_model_library::ModelResolutionRecords::default(),
    )
    .unwrap();
    let authority = sealed
        .reference_model_library_veriloga_authority(rspice_vocabulary::product::ProcessCorner::TT)
        .unwrap()
        .unwrap();
    let sources =
        crate::project_veriloga::compile_model_library_source_runtimes(&authority).unwrap();
    let mut deck = "standalone UI connections\nV1 p 0 1\nX1 p q UI_DRIVER\n.options connectrules=Low connectrules_source=ui_connections\n.tran 0.2n 2n\n.end\n".to_owned();
    for source in sources.sources() {
        assert!(!std::path::Path::new(source.source_key()).exists());
        crate::netlist_preparation::append_project_veriloga_directive(
            &mut deck,
            source.source_key(),
            source.netlist_alias(),
        );
    }
    (sources, deck)
}

/// Constants whose shortest decimal form a text encoding parses back
/// exactly only with care (`serde_json` needs its `float_roundtrip`
/// feature). `1.3806505e-23` is Boltzmann's constant verbatim from shipped
/// Verilog-A models; without that care it reads back as
/// `1.3806504999999999e-23`.
const LOSSY_MODEL_CONSTANTS: [(&str, f64); 3] = [
    ("kb", 1.380_650_5e-23),
    ("q", 1.602_176_634e-19),
    ("scale", 6.25e41),
];

const EXACT_CONSTANT_SOURCE: &str = r#"
`include "disciplines.vams"
module rspice_float_roundtrip_probe(p, n);
  inout p, n;
  electrical p, n;
  parameter real kb = 1.3806505e-23;
  parameter real q = 1.602176634e-19;
  parameter real scale = 6.25e41;
  analog I(p, n) <+ (kb + q + scale) * V(p, n);
endmodule
"#;

/// Compile one fixture source and seal it exactly as a project source is
/// sealed. `try_from_virtual_compilation` is the constructor both the
/// project and the PDK paths reach, so a fixture that seals here
/// seals in production.
fn seal_runtime(file: &str, module: &str, source: &str) -> PreparedVerilogARuntime {
    let bundle = rspice_veriloga::VirtualSourceBundle::new(
        file,
        [rspice_veriloga::VirtualSourceFile::new(file, source)],
    )
    .expect("fixture bundle is well formed");
    let compilation = rspice_veriloga::VerilogACompiler::default()
        .compile_virtual_runtime(
            &bundle,
            module,
            rspice_veriloga::VirtualCompileLimits::default(),
        )
        .expect("fixture module compiles");
    PreparedVerilogARuntime::try_from_virtual_compilation(
        format!("__rspice_project__/fixture/{file}"),
        rspice_vocabulary::product::ContentDigest::from_bytes([0x5a; 32]),
        module.to_owned(),
        &compilation,
    )
    .expect("fixture compilation seals into a prepared runtime")
}

/// The model and canonical IR a sealed runtime opens to.
fn opened(
    runtime: &PreparedVerilogARuntime,
) -> (
    rspice_veriloga::CompiledModel,
    rspice_veriloga::canonical_ir::CanonicalIrArtifact,
) {
    runtime.parsed().expect("the sealed artifact opens")
}

fn seal_probe_runtime() -> PreparedVerilogARuntime {
    seal_runtime(
        "probe.va",
        "rspice_float_roundtrip_probe",
        EXACT_CONSTANT_SOURCE,
    )
}

/// A sealed runtime carries its `CompiledModel` in its sealed record and
/// every consumer — `registration`, `terminal_names`, `validate`, and the
/// browser worker's `compile_wasm_jit_artifact` — opens it. The open has to
/// be bit exact, or a device built through the browser WASM JIT is built
/// from constants one unit in the last place away from the ones the native
/// path uses, for the same source and the same `artifact_digest`. The
/// digest cannot catch that: the bytes are intact and only the reading is
/// lossy.
#[test]
fn sealed_model_constants_survive_the_seal() {
    let runtime = seal_probe_runtime();
    let (decoded, _) = opened(&runtime);

    for (name, expected) in LOSSY_MODEL_CONSTANTS {
        let parameter = decoded
            .parameters
            .iter()
            .find(|parameter| parameter.name.as_str() == name)
            .unwrap_or_else(|| panic!("probe model declares parameter '{name}'"));
        assert_eq!(
            parameter.default.to_bits(),
            expected.to_bits(),
            "parameter '{name}' came back as {:e} instead of {expected:e}",
            parameter.default
        );
    }
}

/// The same guarantee for the canonical IR, which is what the browser JIT
/// lowers.
#[test]
fn sealed_canonical_ir_constants_survive_the_seal() {
    let runtime = seal_probe_runtime();
    let (_, decoded) = opened(&runtime);

    for (name, expected) in LOSSY_MODEL_CONSTANTS {
        let default = decoded
            .hir
            .parameters
            .iter()
            .find(|parameter| parameter.name.as_str() == name)
            .unwrap_or_else(|| panic!("probe IR declares parameter '{name}'"))
            .default
            .expect("probe parameters declare a literal default");
        assert_eq!(
            default.to_bits(),
            expected.to_bits(),
            "IR parameter '{name}' came back as {default:e} instead of {expected:e}"
        );
    }
}

/// `$bound_step` caps the next transient step. It lowers to a hidden task
/// variable reset to `+inf` at the top of every evaluation, so *every*
/// module that calls it carries an infinity in its compiled bytecode.
/// When runtimes were sealed as JSON, which has no spelling for one, this
/// module could not be sealed at all until `rspice_veriloga::json_float`.
const BOUND_STEP_SOURCE: &str = include_str!("../../tests/fixtures/bound_step.va");

/// The declaration-side spellings, and the silent ones. Both are folded
/// constants, so neither is announced by anything in the source text.
///
/// `folded` divides by a literal zero. The fold yields `+inf`, which lands
/// in `CompiledParameter.default`, a bare `f64`, and in
/// `HirParameter.default`, an `Option<f64>` that a JSON seal once decoded
/// back as `None`: the default silently gone.
///
/// `unbounded` reaches a *range*. The grammar's `inf` shortcut is not the
/// only way `inf` can appear in a bound: written as part of an expression it
/// is the ordinary constant again, folds to `+inf`, and lands in
/// `CompiledParameter.max` and `HirParamRange.max` — `Option<f64>` again.
///
/// `excluded` reaches the *third* range field, and it is the one place
/// where a bare `inf` really is an infinity: `exclude` takes a plain
/// expression, with none of a bound's absence rule, so `exclude inf` folds
/// to `+inf` and lands in `HirParamRange.exclude` and
/// `CompiledParameter.exclude` — a `Vec<f64>`, filled by the very same
/// closure that fills `min` and `max` (`SemanticAnalyzer::parse_range`).
///
/// `conductance` is the control, and it is here to keep a wrong premise
/// from coming back: `from (0:inf)` does *not* put an infinity anywhere.
/// The parser reads `inf` in a range bound as the absence of a bound
/// (`RangeBound.upper` is an `Option<Expression>` whose `None` means
/// `+inf`), so an open range never becomes a float at all — which is why
/// the shipped corpus, full of open ranges, carries no non-finite float.
const INFINITE_CONSTANT_SOURCE: &str = include_str!("../../tests/fixtures/infinite_constant.va");

/// Every non-finite `PushConst` in a compiled program, however nested,
/// with its assigned variable when the expression writes a variable.
fn non_finite_constants(
    steps: &[rspice_veriloga::codegen::AssignmentStep],
) -> Vec<(Option<usize>, f64)> {
    fn constants(program: &rspice_veriloga::codegen::BytecodeProgram) -> Vec<f64> {
        program
            .instructions
            .iter()
            .filter_map(|instruction| match instruction {
                rspice_veriloga::codegen::Instruction::PushConst(value) if !value.is_finite() => {
                    Some(*value)
                }
                _ => None,
            })
            .collect()
    }

    let mut found = Vec::new();
    for step in steps {
        match step {
            rspice_veriloga::codegen::AssignmentStep::Assign(assignment) => found.extend(
                constants(&assignment.program)
                    .into_iter()
                    .map(|value| (Some(assignment.var_index), value)),
            ),
            rspice_veriloga::codegen::AssignmentStep::AssignIndexed {
                base, index, value, ..
            } => {
                found.extend(constants(index).into_iter().map(|value| (None, value)));
                found.extend(
                    constants(value)
                        .into_iter()
                        .map(|value| (Some(*base), value)),
                );
            }
            rspice_veriloga::codegen::AssignmentStep::Loop { condition, body } => {
                found.extend(constants(condition).into_iter().map(|value| (None, value)));
                found.extend(non_finite_constants(body));
            }
            rspice_veriloga::codegen::AssignmentStep::Initialization { body, .. } => {
                found.extend(non_finite_constants(body));
            }
            rspice_veriloga::codegen::AssignmentStep::Task(task) => found.extend(
                task.expressions()
                    .flat_map(constants)
                    .map(|value| (None, value)),
            ),
        }
    }
    found
}

/// A sealed artifact that opens to a pair sealing back to the same bytes and
/// fingerprint lost nothing at all — not only the non-finite values these
/// fixtures are about.
fn assert_seal_is_its_own_fixed_point(runtime: &PreparedVerilogARuntime) {
    let (model, canonical_ir) = opened(runtime);
    let resealed = rspice_core::seal_veriloga_artifact(&model, &canonical_ir)
        .expect("the opened artifact seals again");
    assert!(
        resealed.bytes.as_slice() == runtime.artifact.as_bytes(),
        "the sealed artifact is not a fixed point of its own encoding"
    );
    assert_eq!(
        resealed.fingerprint,
        *runtime.artifact_fingerprint.as_bytes()
    );
}

#[test]
fn a_bound_step_module_seals_with_its_infinite_step_sentinel_intact() {
    let runtime = seal_runtime(
        "bound-step.va",
        "rspice_bound_step_probe",
        BOUND_STEP_SOURCE,
    );
    assert_seal_is_its_own_fixed_point(&runtime);

    let (decoded, _) = opened(&runtime);
    let found = non_finite_constants(&decoded.assignment_steps);
    assert_eq!(
        found.len(),
        1,
        "the module resets exactly one task variable to a non-finite sentinel, found {found:?}"
    );
    let (var_index, value) = found[0];
    let var_index = var_index.expect("the step sentinel is assigned to a variable");
    assert_eq!(
        decoded.variable_names[var_index].as_str(),
        "$bound_step",
        "the surviving sentinel belongs to the step bound"
    );
    assert_eq!(
        value.to_bits(),
        f64::INFINITY.to_bits(),
        "the step bound came back as {value} instead of +inf"
    );

    // The canonical IR carries the same sentinel as an expression literal,
    // and it is what the browser JIT lowers.
    let (_, ir) = opened(&runtime);
    let sentinels: Vec<f64> = ir
        .hir
        .expressions
        .iter()
        .filter_map(|expression| match &expression.kind {
            rspice_veriloga::canonical_ir::hir::HirExprKind::Number { value, .. }
                if !value.is_finite() =>
            {
                Some(*value)
            }
            _ => None,
        })
        .collect();
    // Lowering may retain separate literals for initialization and the
    // per-evaluation reset. Their count is a compiler detail; this seal
    // must retain every sentinel's exact sign and non-finite value.
    assert!(!sentinels.is_empty(), "the IR lost its step-bound sentinel");
    assert!(
        sentinels
            .iter()
            .all(|value| value.to_bits() == f64::INFINITY.to_bits()),
        "the IR step-bound sentinels arrived as {sentinels:?}"
    );
}

#[test]
fn an_infinite_folded_constant_survives_the_seal_in_both_payloads() {
    let runtime = seal_runtime(
        "infinite-constant.va",
        "rspice_infinite_constant_probe",
        INFINITE_CONSTANT_SOURCE,
    );
    assert_seal_is_its_own_fixed_point(&runtime);

    let (decoded, _) = opened(&runtime);
    let folded = decoded
        .parameters
        .iter()
        .find(|parameter| parameter.name.as_str() == "folded")
        .expect("probe model declares the infinite parameter");
    assert_eq!(
        folded.default.to_bits(),
        f64::INFINITY.to_bits(),
        "the default came back as {} instead of +inf",
        folded.default
    );

    let (_, ir) = opened(&runtime);
    let folded = ir
        .hir
        .parameters
        .iter()
        .find(|parameter| parameter.name.as_str() == "folded")
        .expect("probe IR declares the infinite parameter");
    assert_eq!(
        folded.default.map(f64::to_bits),
        Some(f64::INFINITY.to_bits()),
        "the IR default arrived as {:?}, which is the silent loss",
        folded.default
    );

    let unbounded = ir
        .hir
        .parameters
        .iter()
        .find(|parameter| parameter.name.as_str() == "unbounded")
        .expect("probe IR declares the expression-bounded range")
        .range
        .as_ref()
        .expect("the parameter declares a range");
    assert_eq!(
        unbounded.max.map(f64::to_bits),
        Some(f64::INFINITY.to_bits()),
        "an infinite bound expression arrived as {:?}",
        unbounded.max
    );
    assert_eq!(
        decoded
            .parameters
            .iter()
            .find(|parameter| parameter.name.as_str() == "unbounded")
            .and_then(|parameter| parameter.max)
            .map(f64::to_bits),
        Some(f64::INFINITY.to_bits()),
        "the bound the device range-checks against must be the declared one"
    );
}

/// The third range field, and the one whose loss was *loud*: `exclude` is
/// a `Vec<f64>` of bare elements, which a JSON seal once wrote as `null`
/// and then refused to decode.
#[test]
fn an_excluded_value_that_folds_to_an_infinity_survives_the_seal() {
    let runtime = seal_runtime(
        "infinite-constant.va",
        "rspice_infinite_constant_probe",
        INFINITE_CONSTANT_SOURCE,
    );

    let (decoded, _) = opened(&runtime);
    let excluded = decoded
        .parameters
        .iter()
        .find(|parameter| parameter.name.as_str() == "excluded")
        .expect("probe model declares the excluding parameter");
    assert_eq!(
        excluded
            .exclude
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>(),
        vec![f64::INFINITY.to_bits()],
        "the excluded value arrived as {:?}",
        excluded.exclude
    );

    let (_, ir) = opened(&runtime);
    let range = ir
        .hir
        .parameters
        .iter()
        .find(|parameter| parameter.name.as_str() == "excluded")
        .expect("probe IR declares the excluding parameter")
        .range
        .as_ref()
        .expect("the parameter declares a range");
    assert_eq!(
        range
            .exclude
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>(),
        vec![f64::INFINITY.to_bits()],
        "the IR excluded value arrived as {:?}",
        range.exclude
    );
}

/// The control, and the correction of a premise worth not rediscovering:
/// an open range bound is an *absence*, not an infinity. Nothing about the
/// non-finite encoding applies to it, and the shipped corpus — hundreds of
/// `from (0:inf)` declarations across 43 modules — carries no non-finite
/// float because of this.
#[test]
fn an_open_range_bound_is_an_absence_rather_than_an_infinity() {
    let runtime = seal_runtime(
        "infinite-constant.va",
        "rspice_infinite_constant_probe",
        INFINITE_CONSTANT_SOURCE,
    );
    let (decoded, _) = opened(&runtime);
    let conductance = decoded
        .parameters
        .iter()
        .find(|parameter| parameter.name.as_str() == "conductance")
        .expect("probe model declares the ranged parameter");
    assert_eq!(conductance.min.map(f64::to_bits), Some(0.0_f64.to_bits()));
    assert_eq!(conductance.max, None, "`inf` in a range bound is no bound");

    let (_, ir) = opened(&runtime);
    let range = ir
        .hir
        .parameters
        .iter()
        .find(|parameter| parameter.name.as_str() == "conductance")
        .expect("probe IR declares the ranged parameter")
        .range
        .as_ref()
        .expect("the parameter declares a range");
    assert_eq!(range.min.map(f64::to_bits), Some(0.0_f64.to_bits()));
    assert_eq!(range.max, None);
}

/// Installing a project's runtimes never moves a deck off the runtime it
/// names. An edited module is sealed under a key of its own, as a project
/// source's key carries its content digest, and installing it beside the one
/// a running deck names, as a check of the edited design does while a run
/// executes, leaves that deck elaborating the artifact it names: the
/// registration adds to the session's runtimes and replaces none.
#[test]
fn installing_an_edited_module_leaves_each_deck_the_runtime_it_names() {
    let load = |siemens: &str| {
        format!(
            "`include \"disciplines.vams\"\nmodule load(p, n);\n  inout p, n;\n  \
             electrical p, n;\n  analog I(p, n) <+ {siemens} * V(p, n);\nendmodule\n"
        )
    };
    let running = seal_runtime("installed-while-running.va", "load", &load("1.0e-3"));
    let edited = seal_runtime("installed-after-an-edit.va", "load", &load("2.0e-3"));
    let _installed = [&running, &edited].map(|runtime| {
        super::PreparedVerilogARuntimeSet::try_new(vec![runtime.clone()])
            .expect("one runtime is a set")
            .install()
            .expect("an edited module installs beside the running one")
    });

    for (runtime, conductance) in [(&running, 1.0e-3), (&edited, 2.0e-3)] {
        let directive = crate::netlist_preparation::project_veriloga_directive(
            runtime.source_key(),
            runtime.netlist_alias(),
        );
        let netlist = rspice_core::Netlist::parse(&format!(
            "deck\n{directive}\nV1 n 0 1\nX1 n 0 load\n.op\n.end\n"
        ))
        .expect("the deck parses");
        let current = -rspice_core::Engine::default()
            .run_dc_op(&netlist)
            .expect("the deck solves")
            .branch_current_named("V1")
            .expect("V1 is solved");
        assert!(
            (current - conductance).abs() < 1e-12,
            "{}: {current} A",
            runtime.source_key()
        );
    }
}

/// A session runs one project again and again, and each run installs its
/// runtimes. The first install checks and opens the payload; a later one
/// does not read it, holding the artifact the first one opened by the key and
/// the fingerprint the runtime was sealed with, so a request whose payload
/// was emptied in transit runs that same artifact.
#[test]
fn a_second_install_holds_the_installed_artifact_without_reading_its_payload() {
    let runtime = seal_runtime(
        "installed-twice.va",
        "installed_twice",
        "`include \"disciplines.vams\"\nmodule installed_twice(p, n);\n  inout p, n;\n  \
         electrical p, n;\n  analog I(p, n) <+ 1.0e-3 * V(p, n);\nendmodule\n",
    );
    let key = std::path::Path::new(runtime.source_key());
    let set = super::PreparedVerilogARuntimeSet::try_new(vec![runtime.clone()])
        .expect("one runtime is a set");
    let first = set.install().expect("the first install opens the payload");
    let (model, _) = first.runtime_artifact(key).expect("the install holds it");
    drop(first);

    let mut emptied = serde_json::to_value(&set).expect("encode the set");
    emptied["runtimes"][0]["artifact"] = serde_json::Value::String(String::new());
    let emptied: super::PreparedVerilogARuntimeSet =
        serde_json::from_value(emptied).expect("decode the emptied set");
    let second = emptied
        .install()
        .expect("the installed artifact is held by its identity");
    let (again, _) = second
        .runtime_artifact(key)
        .expect("the second install holds it");
    assert!(
        std::sync::Arc::ptr_eq(&model, &again),
        "the artifact the first install parsed"
    );
}

/// The digest of an artifact that names no min:typ:max selection:
/// `rspice.project-veriloga-runtime/v4` over its six fields, each
/// length-prefixed, and nothing after them.
fn digest_without_a_selection(
    runtime: &PreparedVerilogARuntime,
) -> rspice_vocabulary::product::ContentDigest {
    use sha2::{Digest as _, Sha256};
    assert!(runtime.external_cells.is_none(), "a fixture without leaves");
    let mut hasher = Sha256::new();
    hasher.update(b"rspice.project-veriloga-runtime/v4\0");
    for bytes in [
        runtime.source_key.as_bytes(),
        runtime.source_digest.as_bytes(),
        runtime.module_name.as_bytes(),
        runtime.netlist_alias.as_bytes(),
        runtime.artifact.as_bytes(),
        runtime.artifact_fingerprint.as_bytes(),
    ] {
        hasher.update((bytes.len() as u64).to_le_bytes());
        hasher.update(bytes);
    }
    rspice_vocabulary::product::ContentDigest::from_bytes(hasher.finalize().into())
}

const HELD: &str = "module held(p, n); inout p, n; electrical p, n;\n\
                    parameter real g = 1.0e-3:2.0e-3:3.0e-3;\n\
                    analog I(p, n) <+ g * V(p, n);\nendmodule\n";

/// `held` compiled keeping the `delay_select` value of its min:typ:max
/// conductance, and sealed under that selection.
fn seal_held(
    delay_select: rspice_veriloga::canonical_ir::digital::DigitalDelaySelect,
) -> PreparedVerilogARuntime {
    let bundle = rspice_veriloga::VirtualSourceBundle::new(
        "held.va",
        [rspice_veriloga::VirtualSourceFile::new("held.va", HELD)],
    )
    .expect("fixture bundle is well formed");
    let compilation = rspice_veriloga::VerilogACompiler::new(rspice_veriloga::CompilerOptions {
        delay_select,
        ..Default::default()
    })
    .compile_virtual_runtime(
        &bundle,
        "held",
        rspice_veriloga::VirtualCompileLimits::default(),
    )
    .expect("fixture module compiles");
    PreparedVerilogARuntime::try_from_virtual_compilation_against(
        "__rspice_project__/fixture/held.va".to_owned(),
        rspice_vocabulary::product::ContentDigest::from_bytes([0x5a; 32]),
        "HELD".to_owned(),
        &compilation,
        None,
        delay_select,
    )
    .expect("fixture compilation seals into a prepared runtime")
}

/// A typical artifact names no selection, in its digest or in its bytes, and
/// the digest of a fixed payload is pinned.
#[test]
fn a_typical_artifact_names_no_selection() {
    use rspice_veriloga::canonical_ir::digital::DigitalDelaySelect;
    let pinned = super::runtime_artifact_digest(
        "__rspice_project__/fixture/held.va",
        rspice_vocabulary::product::ContentDigest::from_bytes([0x5a; 32]),
        "held",
        "HELD",
        b"artifact",
        rspice_vocabulary::product::ContentDigest::from_bytes([0xa5; 32]),
        None,
        DigitalDelaySelect::Typ,
    );
    assert_eq!(
        pinned.to_string(),
        "98b453fae098038cc578df0b587280644694547bdbcda64426cdb8ebe5433863"
    );

    let typical = seal_held(DigitalDelaySelect::Typ);
    assert_eq!(
        typical.artifact_digest(),
        digest_without_a_selection(&typical)
    );
    let json = serde_json::to_string(&typical).expect("encode the artifact");
    assert!(!json.contains("delay_select"), "{json}");
    let decoded: PreparedVerilogARuntime = serde_json::from_str(&json).expect("decode");
    assert_eq!(decoded, typical);
}

/// The selection is part of every artifact's identity, of a design with a
/// min:typ:max expression or without one alike: min, typ and max seal three
/// artifacts, and a minimum artifact stripped of its selection no longer
/// matches its digest.
#[test]
fn each_selection_seals_an_artifact_of_its_own() {
    use rspice_veriloga::canonical_ir::digital::DigitalDelaySelect;
    let selections = [
        DigitalDelaySelect::Min,
        DigitalDelaySelect::Typ,
        DigitalDelaySelect::Max,
    ];
    let sealed = selections.map(seal_held);
    let fixed = selections.map(|delay_select| {
        super::runtime_artifact_digest(
            "__rspice_project__/fixture/held.va",
            rspice_vocabulary::product::ContentDigest::from_bytes([0x5a; 32]),
            "held",
            "HELD",
            b"artifact",
            rspice_vocabulary::product::ContentDigest::from_bytes([0xa5; 32]),
            None,
            delay_select,
        )
    });
    for digests in [
        sealed.each_ref().map(|runtime| runtime.artifact_digest()),
        fixed,
    ] {
        assert_ne!(digests[0], digests[1]);
        assert_ne!(digests[1], digests[2]);
        assert_ne!(digests[0], digests[2]);
    }
    for (runtime, delay_select) in sealed.iter().zip(selections) {
        assert_eq!(runtime.delay_select(), delay_select);
        runtime.validate().expect("each artifact validates");
        let registration = runtime.registration().expect("each artifact registers");
        assert_eq!(registration.delay_select, delay_select);
    }

    let mut json: serde_json::Value = serde_json::to_value(&sealed[0]).expect("encode");
    assert_eq!(json["delay_select"], "Min");
    let decoded: PreparedVerilogARuntime =
        serde_json::from_value(json.clone()).expect("decode the minimum artifact");
    assert_eq!(decoded, sealed[0]);
    json.as_object_mut()
        .expect("an object")
        .remove("delay_select");
    let stripped: PreparedVerilogARuntime =
        serde_json::from_value(json).expect("decode the stripped artifact");
    assert!(
        stripped.validate().is_err(),
        "a stripped selection is refused"
    );
}

/// Every way a module names ground: a one-node probe and contribution, a
/// branch from an internal node to ground, a noise source into ground and a
/// charge on a one-node branch.
const GROUND_REFERENCE_SOURCE: &str = r#"`include "disciplines.vams"
module ground_refs(a, b);
  inout a, b;
  electrical a, b, n;
  parameter real r = 1e3;
  analog begin
    I(a) <+ V(a) / r + I(b, a) * 0.0;
    I(b, a) <+ V(b, a) / r;
    I(n) <+ V(n) / r - V(a) / r;
    I(a) <+ white_noise(4.0e-21, "thermal");
    I(b) <+ ddt(1.0e-12 * V(b));
  end
endmodule
"#;

/// A runtime is sealed on one machine and read on another: the browser
/// worker that reads it is 32-bit. So neither the model nor the IR may hold
/// an integer a 32-bit `usize` cannot: ground, the node index the compiler
/// writes for "no node", is 4294967295 on every target, and a natively
/// sealed model decodes in the worker to the values (and digest) it would
/// seal itself. Ground was `usize::MAX`, which this host wrote
/// 18446744073709551615.
#[test]
fn a_sealed_runtime_holds_no_index_wider_than_the_worker_reads() {
    fn widest(value: &serde_json::Value) -> u64 {
        match value {
            serde_json::Value::Number(number) => number.as_u64().unwrap_or(0),
            serde_json::Value::Array(items) => items.iter().map(widest).max().unwrap_or(0),
            serde_json::Value::Object(fields) => fields.values().map(widest).max().unwrap_or(0),
            _ => 0,
        }
    }
    let runtime = seal_runtime("ground.va", "ground_refs", GROUND_REFERENCE_SOURCE);
    let (model, canonical_ir) = opened(&runtime);
    let model_json = serde_json::to_string(&model).expect("the model writes as JSON");
    for (what, value) in [
        ("model", serde_json::to_value(&model)),
        ("canonical IR", serde_json::to_value(&canonical_ir)),
    ] {
        let value = value.expect("the artifact writes as JSON");
        assert!(
            widest(&value) <= u64::from(u32::MAX),
            "the sealed {what} holds {}, wider than a 32-bit index",
            widest(&value)
        );
    }
    let ground = u32::MAX.to_string();
    assert!(
        model_json.contains(&format!("\"PushVoltage\":[0,{ground}]")),
        "ground is written as {ground}: {}",
        model_json
    );
}
