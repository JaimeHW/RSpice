use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;

use rspice_veriloga::ast::{
    AnalogOperator, BranchAccess, Expression, LaplaceKind, NumberLit, PortDirection,
};
use rspice_veriloga::canonical_ir::{
    BranchId, BranchUnknownId, ContributionId, EquationId, ExprId, ModuleId, NodeId, ParamId,
    PortId, ScheduleId, SourceId, StateId, ValueId, VariableId,
};
use rspice_veriloga::canonical_ir::{
    CanonicalIrArtifact, CanonicalMetadata, CompilerPhase, DerivativeLane, DerivativeLaneKind,
    DiagnosticSeverity, HirAnalogOperator, HirContributionKind, HirExprKind, HirLaplaceKind,
    HirLoop, HirModel, HirStatement, InvalidationClass, IrDiagnostic, MirAnalysisDomain,
    MirEquationKind, MirModel, MirStateSlot, OptBinaryOp, OptDerivative, OptEvalInputs, OptModel,
    OptOp, OptSchedule, OptUnaryOp, OptValue, OptValueKind, OptValueType, SourceSpanRef,
    StableDigest,
};
use rspice_veriloga::semantic::{AnalyzedContribution, AnalyzedModule, AnalyzedPort, SymbolTable};
use rspice_veriloga::source::Span;
use rspice_veriloga::types::ValueType;
use rspice_veriloga::{Lexer, Parser, SemanticAnalyzer, SourceMap, VerilogACompiler};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

fn analyze_fixture(
    source: &str,
    module_name: &str,
) -> rspice_veriloga::CompileResult<rspice_veriloga::semantic::AnalyzedModule> {
    let mut source_map = SourceMap::new();
    let source_id = source_map.add_source_mut("<fixture>", source);
    let tokens = Lexer::new(source, source_id).collect_tokens()?;
    let source_file = Parser::new(&tokens).parse()?;
    let analyzed = SemanticAnalyzer::new().analyze(&source_file)?;
    analyzed
        .modules
        .get(module_name)
        .cloned()
        .ok_or_else(|| rspice_veriloga::CompileError::ModuleSelection(module_name.to_string()))
}

fn validation_messages(hir: &HirModel) -> Vec<String> {
    hir.validate()
        .expect_err("malformed HIR must fail validation")
        .into_iter()
        .map(|diagnostic| diagnostic.message)
        .collect()
}

fn assert_validation_message(hir: &HirModel, expected_substring: &str) {
    let messages = validation_messages(hir);
    assert!(
        messages
            .iter()
            .any(|message| message.contains(expected_substring)),
        "expected diagnostic containing {expected_substring:?}, got {messages:?}"
    );
}

fn mir_validation_messages(mir: &MirModel) -> Vec<String> {
    mir.validate()
        .expect_err("malformed MIR must fail validation")
        .into_iter()
        .map(|diagnostic| diagnostic.message)
        .collect()
}

fn assert_mir_validation_message(mir: &MirModel, expected_substring: &str) {
    let messages = mir_validation_messages(mir);
    assert!(
        messages
            .iter()
            .any(|message| message.contains(expected_substring)),
        "expected diagnostic containing {expected_substring:?}, got {messages:?}"
    );
}

fn opt_validation_messages(opt: &OptModel) -> Vec<String> {
    opt.validate()
        .expect_err("malformed OptIR must fail validation")
        .into_iter()
        .map(|diagnostic| diagnostic.message)
        .collect()
}

fn assert_opt_validation_message(opt: &OptModel, expected_substring: &str) {
    let messages = opt_validation_messages(opt);
    assert!(
        messages
            .iter()
            .any(|message| message.contains(expected_substring)),
        "expected diagnostic containing {expected_substring:?}, got {messages:?}"
    );
}

fn set_newton_ops(opt: &mut OptModel, ops: Vec<OptOp>) {
    opt.schedules
        .retain(|schedule| schedule.invalidation == InvalidationClass::NewtonIteration);
    for (index, schedule) in opt.schedules.iter_mut().enumerate() {
        schedule.id = ScheduleId::from(index);
    }
    let newton = opt
        .schedules
        .iter_mut()
        .find(|schedule| schedule.invalidation == InvalidationClass::NewtonIteration)
        .expect("NewtonIteration schedule");
    newton.ops = ops;
}

fn lower_tiny_resistor_mir() -> MirModel {
    let analyzed = analyze_fixture(tiny_resistor_source(), "tiny_res").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", tiny_resistor_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);

    MirModel::from_hir(&hir).expect("lower MIR")
}

fn lower_fixture_parts(
    source: &'static str,
    module_name: &str,
) -> (CanonicalMetadata, HirModel, MirModel, OptModel) {
    let analyzed = analyze_fixture(source, module_name).expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", source);
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);
    let mir = MirModel::from_hir(&hir).expect("lower MIR");
    let opt = OptModel::from_hir_and_mir(&hir, &mir).expect("lower OptIR");

    (metadata, hir, mir, opt)
}

fn lower_tiny_resistor_parts() -> (CanonicalMetadata, HirModel, MirModel, OptModel) {
    lower_fixture_parts(tiny_resistor_source(), "tiny_res")
}

fn lower_internal_node_mir() -> MirModel {
    let analyzed = analyze_fixture(internal_node_source(), "has_mid").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", internal_node_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);

    MirModel::from_hir(&hir).expect("lower MIR")
}

fn contribution_branch_access_id(hir: &HirModel) -> ExprId {
    let root_id = hir.contributions[0].expression.id;
    let HirExprKind::Binary { left, .. } = &hir.expressions[usize::from(root_id)].kind else {
        panic!("expected contribution expression to be binary");
    };

    *left
}

fn tiny_resistor_source() -> &'static str {
    r#"
module tiny_res(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1000.0 from (0:inf);
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#
}

fn multi_module_source() -> &'static str {
    r#"
module first_res(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ V(p, n);
endmodule

module second_res(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ 2.0 * V(p, n);
endmodule
"#
}

fn dynamic_array_source() -> &'static str {
    r#"
module dyn_array(p, n);
    inout p, n;
    electrical p, n;
    parameter integer pick = 1;
    parameter real scale = 2.0;
    real xs[0:3];
    analog begin
        xs[pick] = scale;
        I(p, n) <+ xs[pick] * V(p, n);
    end
endmodule
"#
}

fn scalar_assignment_source() -> &'static str {
    r#"
module scalar_assign(p, n);
    inout p, n;
    electrical p, n;
    real x;
    analog begin
        x = 1.0;
        I(p, n) <+ x * V(p, n);
    end
endmodule
"#
}

fn integer_parameter_loop_accumulator_source() -> &'static str {
    r#"
module integer_parameter_loop_accum(p, n);
    inout p, n;
    electrical p, n;
    parameter integer nf = 4 from [0:inf);
    integer i;
    real acc;
    analog begin
        i = 0;
        acc = 0.0;
        while (i < nf) begin
            acc = acc + V(p, n);
            i = i + 1;
        end
        I(p, n) <+ acc;
    end
endmodule
"#
}

fn mixed_dynamic_assignment_source() -> &'static str {
    r#"
module mixed_dynamic_assignment(p, n);
    inout p, n;
    electrical p, n;
    parameter real c = 1e-12 from [0:inf);
    real x, q;
    analog begin
        x = V(p, n) + 3.0;
        I(p, n) <+ x;
        q = c * V(p, n);
        I(p, n) <+ ddt(q);
    end
endmodule
"#
}

fn chunked_dynamic_assignment_source(count: usize) -> String {
    assert!(count > 0);
    let mut source = String::from(
        r#"
module chunked_dynamic_assignments(p, n);
    inout p, n;
    electrical p, n;
    parameter real c = 1e-12 from [0:inf);
"#,
    );
    for index in 0..count {
        source.push_str(&format!("    real x{index};\n"));
    }
    source.push_str(
        r#"    real q;
    analog begin
        x0 = V(p, n);
"#,
    );
    for index in 1..count {
        source.push_str(&format!("        x{index} = x{} + 1.0;\n", index - 1));
    }
    source.push_str(&format!("        I(p, n) <+ x{};\n", count - 1));
    source.push_str(
        r#"        q = c * V(p, n);
        I(p, n) <+ ddt(q);
    end
endmodule
"#,
    );
    source
}

fn internal_node_source() -> &'static str {
    r#"
module has_mid(p, n);
    inout p, n;
    electrical p, n;
    electrical mid;
    analog begin
        I(p, mid) <+ V(p, mid);
        I(mid, n) <+ V(mid, n);
    end
endmodule
"#
}

fn multiply_by_one_source() -> &'static str {
    r#"
module mul_one(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ V(p, n) * 1.0;
endmodule
"#
}

fn constant_arithmetic_source() -> &'static str {
    r#"
module const_arith(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ (2.0 + 3.0) * V(p, n);
endmodule
"#
}

fn negative_constant_gain_source() -> &'static str {
    r#"
module neg_const_gain(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ -1.0 * V(p, n);
endmodule
"#
}

fn sine_current_source() -> &'static str {
    r#"
module sin_i(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ sin(V(p, n));
endmodule
"#
}

fn atan_current_source() -> &'static str {
    r#"
module atan_i(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ atan(V(p, n));
endmodule
"#
}

fn asinh_current_source() -> &'static str {
    r#"
module asinh_i(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ asinh(V(p, n));
endmodule
"#
}

fn named_branch_potential_source() -> &'static str {
    r#"
module branch_potential(p, n);
    inout p, n;
    electrical p, n;
    branch (p, n) res;
    analog V(res) <+ 1.0;
endmodule
"#
}

fn named_branch_access_source() -> &'static str {
    r#"
module named_branch_access(p, n);
    inout p, n;
    electrical p, n;
    electrical x;
    branch (x) probe;
    analog begin
        I(probe) <+ V(probe);
        I(p, n) <+ I(probe);
    end
endmodule
"#
}

fn implicit_terminal_branch_access_source() -> &'static str {
    r#"
module implicit_terminal_branch_access(p, n);
    inout p, n;
    electrical p, n;
    real ip;
    analog begin
        ip = I(<p>);
        I(p, n) <+ ip;
    end
endmodule
"#
}

fn hir_validation_surface_source() -> &'static str {
    r#"
module validation_surface(p, n);
    inout p, n;
    electrical p, n;
    electrical mid;
    parameter real gain = 1.0;
    branch (p, mid) probe;
    analog begin
        I(probe) <+ gain * V(probe);
        I(mid, n) <+ V(mid, n);
    end
endmodule
"#
}

fn ground_alias_source() -> &'static str {
    r#"
module ground_alias(p);
    inout p;
    electrical p;
    electrical mid;
    ground earth;
    analog I(mid, earth) <+ V(mid);
endmodule
"#
}

fn ground_positive_source() -> &'static str {
    r#"
module ground_positive(p);
    inout p;
    electrical p;
    ground earth;
    analog I(earth, p) <+ V(p);
endmodule
"#
}

fn ground_positive_named_branch_source() -> &'static str {
    r#"
module ground_positive_branch(p);
    inout p;
    electrical p;
    ground earth;
    branch (earth, p) res;
    analog V(res) <+ 1.0;
endmodule
"#
}

struct TempSourceFile {
    path: std::path::PathBuf,
}

struct TempSourceDir {
    path: std::path::PathBuf,
}

impl TempSourceFile {
    fn new(contents: &str) -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("rspice-canonical-ir-{unique}.va"));
        std::fs::write(&path, contents).expect("write fixture");

        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl TempSourceDir {
    fn new() -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("rspice-canonical-ir-dir-{unique}"));
        std::fs::create_dir(&path).expect("create fixture dir");

        Self { path }
    }

    fn write_file(&self, name: &str, contents: &str) -> std::path::PathBuf {
        let path = self.path.join(name);
        std::fs::write(&path, contents).expect("write fixture");
        path
    }
}

impl Drop for TempSourceFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

impl Drop for TempSourceDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

#[test]
fn compiler_can_emit_canonical_ir_without_bytecode_runtime() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(tiny_resistor_source())
        .expect("compile canonical IR");

    assert_eq!(artifact.hir.module_name.as_str(), "tiny_res");
    assert_eq!(artifact.mir.equations.len(), 1);
    assert!(artifact.dump_text().contains("canonical-veriloga-ir"));
}

#[test]
fn compiler_can_emit_canonical_ir_for_selected_module() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir_module(multi_module_source(), Some("second_res"))
        .expect("compile selected module");

    assert_eq!(artifact.hir.module_name.as_str(), "second_res");
    assert_eq!(artifact.mir.module_name.as_str(), "second_res");
}

#[test]
fn compiler_rejects_ambiguous_canonical_ir_module_selection() {
    let err = VerilogACompiler::default()
        .compile_canonical_ir(multi_module_source())
        .expect_err("multi-module source requires a module name");

    assert_eq!(
        err.to_string(),
        "Module selection error: the file declares multiple modules: first_res, second_res; select one by name"
    );
}

#[test]
fn canonical_ir_compile_preserves_parse_errors() {
    let err = VerilogACompiler::default()
        .compile_canonical_ir("module broken(")
        .expect_err("invalid source must fail");

    assert!(
        matches!(err, rspice_veriloga::CompileError::Parser(_)),
        "expected parser error, got {err:?}"
    );
}

#[test]
fn compiler_can_emit_file_canonical_ir_with_metadata() {
    let fixture = TempSourceFile::new(tiny_resistor_source());
    let canonical_path = fixture
        .path()
        .canonicalize()
        .expect("canonical fixture path");
    let canonical_path_text = canonical_path.display().to_string();

    let compiled = VerilogACompiler::default()
        .compile_file_canonical_ir_with_metadata(fixture.path(), None)
        .expect("compile canonical IR from file");

    assert!(compiled.artifact.validate().is_ok());
    assert_eq!(compiled.dependencies, vec![canonical_path]);
    assert_eq!(
        compiled.artifact.metadata.source_package.as_str(),
        canonical_path_text
    );
    assert_eq!(
        compiled.artifact.hir.source_package.as_str(),
        canonical_path_text
    );
    assert!(
        compiled
            .artifact
            .dump_text()
            .contains(&format!("source_package={canonical_path_text}"))
    );
}

#[test]
fn file_canonical_ir_metadata_uses_root_path_when_include_sorts_first() {
    let fixture = TempSourceDir::new();
    let include_path = fixture.write_file("aaa_include.va", "`define ROOT_GAIN 1.0\n");
    let root_path = fixture.write_file(
        "zzz_root.va",
        r#"
`include "aaa_include.va"
module root_with_include(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ `ROOT_GAIN * V(p, n);
endmodule
"#,
    );
    let canonical_include_path = include_path.canonicalize().expect("canonical include path");
    let canonical_root_path = root_path.canonicalize().expect("canonical root path");
    let canonical_root_path_text = canonical_root_path.display().to_string();

    let compiled = VerilogACompiler::default()
        .compile_file_canonical_ir_with_metadata(&root_path, None)
        .expect("compile canonical IR from file");

    assert_eq!(
        compiled.dependencies,
        vec![canonical_include_path, canonical_root_path]
    );
    assert_eq!(
        compiled.artifact.metadata.source_package.as_str(),
        canonical_root_path_text
    );
    assert_eq!(
        compiled.artifact.hir.source_package.as_str(),
        canonical_root_path_text
    );
    assert!(compiled.artifact.validate().is_ok());
}

#[test]
fn typed_ids_are_dense_copyable_and_displayable() {
    let module = ModuleId::new(7);
    let source = SourceId::new(3);
    let port = PortId::new(2);
    let param = ParamId::new(5);

    assert_eq!(module.index(), 7);
    assert_eq!(source.index(), 3);
    assert_eq!(port.index(), 2);
    assert_eq!(param.index(), 5);
    assert_eq!(module.to_string(), "ModuleId(7)");
    assert_eq!(port.next(), PortId::new(3));
}

#[test]
fn typed_ids_expose_expected_trait_surface() {
    fn assert_id_traits<T>()
    where
        T: Copy + Ord + Hash + Serialize + for<'de> Deserialize<'de>,
    {
    }

    assert_id_traits::<ModuleId>();
}

#[test]
fn typed_ids_convert_to_and_from_usize() {
    let id = ParamId::from(42usize);

    assert_eq!(usize::from(id), 42);
}

#[test]
#[should_panic(expected = "canonical IR id overflow")]
fn next_panics_on_overflow() {
    let _ = ModuleId::new(u32::MAX).next();
}

#[cfg(target_pointer_width = "64")]
#[test]
#[should_panic(expected = "canonical IR id index exceeds u32::MAX")]
fn from_usize_panics_when_index_exceeds_u32_max() {
    let _ = ModuleId::from(u32::MAX as usize + 1);
}

#[test]
fn metadata_digest_is_stable_and_hex_encoded() {
    let vectors = [
        ("", "cbf29ce484222325"),
        ("a", "af63dc4c8601ec8c"),
        ("module tiny; endmodule", "b6b5ff4fe150c2db"),
    ];

    for (text, expected) in vectors {
        assert_eq!(StableDigest::from_text(text).as_hex(), expected);
    }

    let digest = StableDigest::from_text("module tiny; endmodule");
    assert_eq!(digest.as_hex().len(), 16);
    assert_eq!(digest, StableDigest::from_text("module tiny; endmodule"));
    assert_ne!(digest, StableDigest::from_text("module other; endmodule"));

    let metadata = CanonicalMetadata::for_source("fixture", "module tiny; endmodule");
    assert_eq!(metadata.schema_version, 1);
    assert_eq!(metadata.source_package.as_str(), "fixture");
    assert_eq!(metadata.source_digest.as_str(), digest.as_hex());
}

#[test]
fn diagnostics_are_phase_aware_and_source_spanned() {
    let span = SourceSpanRef {
        source_file_id: 0,
        start: 12,
        end: 20,
    };
    let diagnostic =
        IrDiagnostic::error(CompilerPhase::MirValidation, "missing equation row", span);

    assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
    assert_eq!(diagnostic.phase, CompilerPhase::MirValidation);
    assert_eq!(diagnostic.message, "missing equation row");
    assert_eq!(diagnostic.span, Some(span));

    let rendered = diagnostic.to_string();
    assert!(rendered.contains("MirValidation"));
    assert!(rendered.contains("0:12-20"));
}

#[test]
fn diagnostics_can_be_global_without_source_span() {
    let diagnostic = IrDiagnostic::global_error(CompilerPhase::Artifact, "schedule has cycle");

    assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
    assert_eq!(diagnostic.phase, CompilerPhase::Artifact);
    assert_eq!(diagnostic.message, "schedule has cycle");
    assert_eq!(diagnostic.span, None);

    let rendered = diagnostic.to_string();
    assert!(rendered.contains("Artifact"));
    assert!(rendered.contains("global"));
}

#[test]
fn hir_lowering_preserves_analyzed_module_surface() {
    let analyzed = analyze_fixture(tiny_resistor_source(), "tiny_res").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", tiny_resistor_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);

    assert_eq!(hir.module_name.as_str(), "tiny_res");
    assert_eq!(hir.ports.len(), 2);
    assert_eq!(hir.ports[0].name.as_str(), "p");
    assert_eq!(hir.ports[1].name.as_str(), "n");
    assert_eq!(hir.parameters.len(), 1);
    assert_eq!(hir.parameters[0].name.as_str(), "r");
    assert_eq!(hir.parameters[0].default, Some(1000.0));
    assert_eq!(hir.contributions.len(), 1);
    assert_eq!(hir.contributions[0].kind, HirContributionKind::Current);
    assert!(hir.validate().is_ok());
}

#[test]
fn mir_lowering_makes_contributions_explicit_equations() {
    let mir = lower_tiny_resistor_mir();

    assert_eq!(mir.module_name.as_str(), "tiny_res");
    assert_eq!(mir.nodes.len(), 2);
    assert_eq!(mir.nodes[0].name.as_str(), "p");
    assert_eq!(mir.nodes[0].id, NodeId::new(0));
    assert!(mir.nodes[0].is_external);
    assert_eq!(mir.nodes[1].name.as_str(), "n");
    assert_eq!(mir.nodes[1].id, NodeId::new(1));
    assert!(mir.nodes[1].is_external);

    assert_eq!(mir.equations.len(), 1);
    assert_eq!(mir.equations[0].id, EquationId::new(0));
    assert_eq!(mir.equations[0].contribution, ContributionId::new(0));
    assert_eq!(mir.equations[0].kind, MirEquationKind::Current);
    assert_eq!(mir.equations[0].branch.label.as_str(), "p,n");
    assert_eq!(mir.equations[0].branch.declared_name.as_deref(), None);
    assert_eq!(mir.equations[0].branch.pos_node, Some(NodeId::new(0)));
    assert_eq!(mir.equations[0].branch.neg_node, Some(NodeId::new(1)));
    assert!(
        mir.equations[0]
            .active_domains
            .contains(&MirAnalysisDomain::Dc)
    );
    assert!(mir.validate().is_ok());
}

#[test]
fn opt_lowering_builds_newton_schedule_from_mir_equations() {
    let mir = lower_tiny_resistor_mir();
    let opt = OptModel::from_mir(&mir).expect("lower OptIR");

    assert_eq!(opt.module_name.as_str(), "tiny_res");

    let newton = opt
        .schedules
        .iter()
        .find(|schedule| schedule.invalidation == InvalidationClass::NewtonIteration)
        .expect("NewtonIteration schedule");

    assert_eq!(
        newton.ops.last(),
        Some(&OptOp::EvaluateEquation {
            equation: EquationId::new(0)
        })
    );
    assert!(opt.validate().is_ok());
}

#[test]
fn opt_lowering_builds_scalar_graph_for_tiny_resistor_expression() {
    let mir = lower_tiny_resistor_mir();
    let opt = OptModel::from_mir(&mir).expect("lower OptIR");

    assert!(opt.values.len() >= 5);
    assert_eq!(
        opt.values[0].kind,
        OptValueKind::NodePotential {
            node: NodeId::new(0)
        }
    );
    assert_eq!(
        opt.values[1].kind,
        OptValueKind::NodePotential {
            node: NodeId::new(1)
        }
    );
    assert_eq!(
        opt.values[2].kind,
        OptValueKind::Binary {
            op: OptBinaryOp::Sub,
            left: ValueId::new(0),
            right: ValueId::new(1),
        }
    );
    assert_eq!(
        opt.values[3].kind,
        OptValueKind::Parameter {
            parameter: ParamId::new(0)
        }
    );
    assert_eq!(
        opt.values[4].kind,
        OptValueKind::Binary {
            op: OptBinaryOp::Div,
            left: ValueId::new(2),
            right: ValueId::new(3),
        }
    );

    let newton = opt
        .schedules
        .iter()
        .find(|schedule| schedule.invalidation == InvalidationClass::NewtonIteration)
        .expect("NewtonIteration schedule");
    assert_eq!(
        newton.ops,
        vec![
            OptOp::ComputeValue {
                value: ValueId::new(4)
            },
            OptOp::EvaluateEquation {
                equation: EquationId::new(0)
            }
        ]
    );

    let snapshot = opt
        .evaluate(&OptEvalInputs {
            parameters: vec![1000.0],
            node_potentials: vec![5.0, 2.0],
            branch_flows: Vec::new(),
        })
        .expect("evaluate lowered graph");
    assert_eq!(snapshot.real(ValueId::new(4)), Some(0.003));
}

#[test]
fn opt_lowering_adds_sparse_derivatives_for_tiny_resistor_expression() {
    let mir = lower_tiny_resistor_mir();
    let opt = OptModel::from_mir(&mir).expect("lower OptIR");

    let root = ValueId::new(4);
    let snapshot = opt
        .evaluate(&OptEvalInputs {
            parameters: vec![1000.0],
            node_potentials: vec![5.0, 2.0],
            branch_flows: Vec::new(),
        })
        .expect("evaluate lowered graph");

    assert_eq!(
        snapshot.derivative(root, DerivativeLane::node(NodeId::new(0))),
        Some(0.001)
    );
    assert_eq!(
        snapshot.derivative(root, DerivativeLane::node(NodeId::new(1))),
        Some(-0.001)
    );
}

#[test]
fn opt_lowering_reuses_common_scalar_values_across_equations() {
    let mir = lower_internal_node_mir();
    let opt = OptModel::from_mir(&mir).expect("lower OptIR");

    let mid_potential_count = opt
        .values
        .iter()
        .filter(|value| {
            value.kind
                == OptValueKind::NodePotential {
                    node: NodeId::new(2),
                }
        })
        .count();

    assert_eq!(mid_potential_count, 1);
}

#[test]
fn opt_lowering_resolves_straight_line_scalar_assignment() {
    let (_, _, _, opt) = lower_fixture_parts(scalar_assignment_source(), "scalar_assign");
    let newton = opt
        .schedules
        .iter()
        .find(|schedule| schedule.invalidation == InvalidationClass::NewtonIteration)
        .expect("NewtonIteration schedule");
    let Some(OptOp::ComputeValue { value: root }) = newton.ops.first() else {
        panic!("assignment-fed current should have a scalar root: {newton:?}");
    };

    let snapshot = opt
        .evaluate(&OptEvalInputs {
            parameters: Vec::new(),
            node_potentials: vec![5.0, 2.0],
            branch_flows: Vec::new(),
        })
        .expect("evaluate assignment-fed scalar graph");

    assert_eq!(snapshot.real(*root), Some(3.0));
    assert!(
        opt.values
            .iter()
            .all(|value| !matches!(value.kind, OptValueKind::EquationValue { .. })),
        "assignment-fed scalar graph should not fall back to legacy equation values: {:?}",
        opt.values
    );
}

#[test]
fn opt_lowering_scalarizes_nonnegative_integer_parameter_counted_accumulator_loop() {
    let (_, _, _, opt) = lower_fixture_parts(
        integer_parameter_loop_accumulator_source(),
        "integer_parameter_loop_accum",
    );
    let newton = opt
        .schedules
        .iter()
        .find(|schedule| schedule.invalidation == InvalidationClass::NewtonIteration)
        .expect("NewtonIteration schedule");
    let Some(OptOp::ComputeValue { value: root }) = newton.ops.first() else {
        panic!("integer parameter counted loop should have a scalar root: {newton:?}");
    };

    let snapshot = opt
        .evaluate(&OptEvalInputs {
            parameters: vec![4.0],
            node_potentials: vec![5.0, 2.0],
            branch_flows: Vec::new(),
        })
        .expect("evaluate integer counted loop scalar graph");

    assert_eq!(snapshot.real(*root), Some(12.0));
    assert_eq!(
        snapshot.derivative(*root, DerivativeLane::node(NodeId::new(0))),
        Some(4.0)
    );
    assert_eq!(
        snapshot.derivative(*root, DerivativeLane::node(NodeId::new(1))),
        Some(-4.0)
    );
    assert!(
        opt.values
            .iter()
            .all(|value| !matches!(value.kind, OptValueKind::EquationValue { .. })),
        "integer counted loop should not fall back to legacy equation values: {:?}",
        opt.values
    );
}

#[test]
fn opt_lowering_keeps_assignment_fed_static_roots_in_dynamic_models() {
    let (_, _, _, opt) = lower_fixture_parts(
        mixed_dynamic_assignment_source(),
        "mixed_dynamic_assignment",
    );
    let newton = opt
        .schedules
        .iter()
        .find(|schedule| schedule.invalidation == InvalidationClass::NewtonIteration)
        .expect("NewtonIteration schedule");
    let Some(OptOp::ComputeValue { value: root }) = newton.ops.first() else {
        panic!("assignment-fed static current should have a scalar root: {newton:?}");
    };

    let snapshot = opt
        .evaluate(&OptEvalInputs {
            parameters: vec![1.0e-12],
            node_potentials: vec![5.0, 2.0],
            branch_flows: Vec::new(),
        })
        .expect("evaluate mixed dynamic scalar graph");

    assert_eq!(snapshot.real(*root), Some(6.0));
    assert_eq!(
        snapshot.derivative(*root, DerivativeLane::node(NodeId::new(0))),
        Some(1.0)
    );
    assert_eq!(
        snapshot.derivative(*root, DerivativeLane::node(NodeId::new(1))),
        Some(-1.0)
    );
}

#[test]
fn opt_lowering_keeps_ddt_operand_roots_in_dynamic_models() {
    let (_, _, _, opt) = lower_fixture_parts(
        mixed_dynamic_assignment_source(),
        "mixed_dynamic_assignment",
    );
    let newton = opt
        .schedules
        .iter()
        .find(|schedule| schedule.invalidation == InvalidationClass::NewtonIteration)
        .expect("NewtonIteration schedule");

    assert!(
        matches!(newton.ops.as_slice(), [
            OptOp::ComputeValue { .. },
            OptOp::EvaluateEquation { equation: eq0 },
            OptOp::ComputeValue { .. },
            OptOp::EvaluateEquation { equation: eq1 },
        ] if *eq0 == EquationId::new(0) && *eq1 == EquationId::new(1)),
        "static and ddt equations should both have scalar roots: {newton:?}"
    );
}

#[test]
fn opt_lowering_keeps_large_assignment_chain_roots_in_dynamic_models() {
    let source = chunked_dynamic_assignment_source(320);
    let analyzed =
        analyze_fixture(&source, "chunked_dynamic_assignments").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", &source);
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);
    let mir = MirModel::from_hir(&hir).expect("lower MIR");
    let opt = OptModel::from_hir_and_mir(&hir, &mir).expect("lower OptIR");
    let newton = opt
        .schedules
        .iter()
        .find(|schedule| schedule.invalidation == InvalidationClass::NewtonIteration)
        .expect("NewtonIteration schedule");

    let Some(OptOp::ComputeValue { .. }) = newton.ops.first() else {
        panic!("large assignment-fed static current should have a scalar root: {newton:?}");
    };
}

#[test]
fn opt_lowering_adds_sparse_derivatives_for_sine_current() {
    let (_, _, _, opt) = lower_fixture_parts(sine_current_source(), "sin_i");
    let newton = opt
        .schedules
        .iter()
        .find(|schedule| schedule.invalidation == InvalidationClass::NewtonIteration)
        .expect("NewtonIteration schedule");
    let Some(OptOp::ComputeValue { value: root }) = newton.ops.first() else {
        panic!("sine current should have a scalar root: {newton:?}");
    };

    let snapshot = opt
        .evaluate(&OptEvalInputs {
            parameters: Vec::new(),
            node_potentials: vec![0.75, 0.25],
            branch_flows: Vec::new(),
        })
        .expect("evaluate sine scalar graph");
    let expected_value = 0.5_f64.sin();
    let expected_derivative = 0.5_f64.cos();

    assert!((snapshot.real(*root).expect("root value") - expected_value).abs() < 1.0e-15);
    assert!(
        (snapshot
            .derivative(*root, DerivativeLane::node(NodeId::new(0)))
            .expect("positive node derivative")
            - expected_derivative)
            .abs()
            < 1.0e-15
    );
    assert!(
        (snapshot
            .derivative(*root, DerivativeLane::node(NodeId::new(1)))
            .expect("negative node derivative")
            + expected_derivative)
            .abs()
            < 1.0e-15
    );
}

#[test]
fn opt_lowering_adds_sparse_derivatives_for_inverse_math_currents() {
    let cases = [
        (
            atan_current_source(),
            "atan_i",
            0.5_f64.atan(),
            1.0 / (1.0 + 0.5_f64 * 0.5_f64),
        ),
        (
            asinh_current_source(),
            "asinh_i",
            0.5_f64.asinh(),
            1.0 / (1.0 + 0.5_f64 * 0.5_f64).sqrt(),
        ),
    ];

    for (source, module, expected_value, expected_derivative) in cases {
        let (_, _, _, opt) = lower_fixture_parts(source, module);
        let newton = opt
            .schedules
            .iter()
            .find(|schedule| schedule.invalidation == InvalidationClass::NewtonIteration)
            .expect("NewtonIteration schedule");
        let Some(OptOp::ComputeValue { value: root }) = newton.ops.first() else {
            panic!("inverse math current should have a scalar root: {newton:?}");
        };

        let snapshot = opt
            .evaluate(&OptEvalInputs {
                parameters: Vec::new(),
                node_potentials: vec![0.75, 0.25],
                branch_flows: Vec::new(),
            })
            .expect("evaluate inverse math scalar graph");

        assert!((snapshot.real(*root).expect("root value") - expected_value).abs() < 1.0e-15);
        assert!(
            (snapshot
                .derivative(*root, DerivativeLane::node(NodeId::new(0)))
                .expect("positive node derivative")
                - expected_derivative)
                .abs()
                < 1.0e-15
        );
        assert!(
            (snapshot
                .derivative(*root, DerivativeLane::node(NodeId::new(1)))
                .expect("negative node derivative")
                + expected_derivative)
                .abs()
                < 1.0e-15
        );
    }
}

#[test]
fn opt_lowering_strength_reduces_multiply_by_one() {
    let (_, _, _, opt) = lower_fixture_parts(multiply_by_one_source(), "mul_one");
    let newton = opt
        .schedules
        .iter()
        .find(|schedule| schedule.invalidation == InvalidationClass::NewtonIteration)
        .expect("NewtonIteration schedule");

    assert_eq!(
        newton.ops.first(),
        Some(&OptOp::ComputeValue {
            value: ValueId::new(2)
        })
    );
    assert!(
        opt.values.iter().all(|value| {
            !matches!(
                value.kind,
                OptValueKind::Binary {
                    op: OptBinaryOp::Mul,
                    ..
                }
            )
        }),
        "multiply by one should not remain in scalar OptIR: {:?}",
        opt.values
    );
}

#[test]
fn opt_lowering_constant_folds_real_arithmetic() {
    let (_, _, _, opt) = lower_fixture_parts(constant_arithmetic_source(), "const_arith");

    assert!(
        opt.values
            .iter()
            .any(|value| value.kind == OptValueKind::RealConstant(5.0)),
        "constant arithmetic should fold to one scalar value: {:?}",
        opt.values
    );
    assert!(
        opt.values.iter().all(|value| {
            !matches!(
                value.kind,
                OptValueKind::Binary {
                    op: OptBinaryOp::Add,
                    left,
                    right,
                } if matches!(
                    (&opt.values[usize::from(left)].kind, &opt.values[usize::from(right)].kind),
                    (OptValueKind::RealConstant(_), OptValueKind::RealConstant(_))
                )
            )
        }),
        "constant-only additions should not remain in scalar OptIR: {:?}",
        opt.values
    );
}

#[test]
fn opt_lowering_eliminates_dead_folded_constant_operands() {
    let (_, _, _, opt) = lower_fixture_parts(constant_arithmetic_source(), "const_arith");

    assert!(
        opt.values.iter().all(|value| {
            !matches!(
                value.kind,
                OptValueKind::RealConstant(value) if value == 2.0 || value == 3.0
            )
        }),
        "folded constant operands should not remain live in scalar OptIR: {:?}",
        opt.values
    );
    assert!(opt.validate().is_ok());
}

#[test]
fn opt_lowering_constant_folds_unary_negation() {
    let (_, _, _, opt) = lower_fixture_parts(negative_constant_gain_source(), "neg_const_gain");

    assert!(
        opt.values
            .iter()
            .any(|value| value.kind == OptValueKind::RealConstant(-1.0)),
        "negative literal should fold to one scalar value: {:?}",
        opt.values
    );
    assert!(
        opt.values.iter().all(|value| {
            !matches!(
                value.kind,
                OptValueKind::Unary {
                    op: OptUnaryOp::Neg,
                    input,
                } if matches!(opt.values[usize::from(input)].kind, OptValueKind::RealConstant(_))
            )
        }),
        "constant unary negation should not remain in scalar OptIR: {:?}",
        opt.values
    );
}

#[test]
fn opt_validation_rejects_missing_newton_schedule() {
    let mir = lower_tiny_resistor_mir();
    let mut opt = OptModel::from_mir(&mir).expect("lower OptIR");
    opt.schedules
        .retain(|schedule| schedule.invalidation != InvalidationClass::NewtonIteration);

    assert_opt_validation_message(&opt, "exactly one NewtonIteration schedule");
}

#[test]
fn opt_validation_rejects_non_dense_schedule_id() {
    let mir = lower_tiny_resistor_mir();
    let mut opt = OptModel::from_mir(&mir).expect("lower OptIR");
    opt.schedules[0].id = ScheduleId::new(9);

    assert_opt_validation_message(&opt, "OptIR schedule IDs must be dense");
}

#[test]
fn opt_validation_rejects_non_dense_value_id() {
    let mir = lower_tiny_resistor_mir();
    let mut opt = OptModel::from_mir(&mir).expect("lower OptIR");
    opt.values.push(OptValue {
        id: ValueId::new(1),
        value_type: OptValueType::Real,
        kind: OptValueKind::RealConstant(1.0),
        derivatives: Vec::new(),
    });

    assert_opt_validation_message(&opt, "OptIR value IDs must be dense");
}

#[test]
fn opt_validation_accepts_scalar_value_graph() {
    let mir = lower_tiny_resistor_mir();
    let mut opt = OptModel::from_mir(&mir).expect("lower OptIR");
    opt.values = vec![
        OptValue {
            id: ValueId::new(0),
            value_type: OptValueType::Real,
            kind: OptValueKind::NodePotential {
                node: NodeId::new(0),
            },
            derivatives: vec![OptDerivative {
                lane: DerivativeLane::node(NodeId::new(0)),
                value: ValueId::new(2),
            }],
        },
        OptValue {
            id: ValueId::new(1),
            value_type: OptValueType::Real,
            kind: OptValueKind::Parameter {
                parameter: ParamId::new(0),
            },
            derivatives: Vec::new(),
        },
        OptValue {
            id: ValueId::new(2),
            value_type: OptValueType::Real,
            kind: OptValueKind::RealConstant(1.0),
            derivatives: Vec::new(),
        },
        OptValue {
            id: ValueId::new(3),
            value_type: OptValueType::Real,
            kind: OptValueKind::Binary {
                op: OptBinaryOp::Div,
                left: ValueId::new(0),
                right: ValueId::new(1),
            },
            derivatives: vec![OptDerivative {
                lane: DerivativeLane {
                    kind: DerivativeLaneKind::Node,
                    index: 0,
                },
                value: ValueId::new(2),
            }],
        },
    ];

    set_newton_ops(
        &mut opt,
        vec![
            OptOp::ComputeValue {
                value: ValueId::new(3),
            },
            OptOp::EvaluateEquation {
                equation: EquationId::new(0),
            },
        ],
    );

    assert!(opt.validate().is_ok());
}

#[test]
fn opt_validation_rejects_scalar_operand_out_of_range() {
    let mir = lower_tiny_resistor_mir();
    let mut opt = OptModel::from_mir(&mir).expect("lower OptIR");
    opt.values = vec![OptValue {
        id: ValueId::new(0),
        value_type: OptValueType::Real,
        kind: OptValueKind::Unary {
            op: OptUnaryOp::Neg,
            input: ValueId::new(9),
        },
        derivatives: Vec::new(),
    }];
    set_newton_ops(
        &mut opt,
        vec![
            OptOp::ComputeValue {
                value: ValueId::new(0),
            },
            OptOp::EvaluateEquation {
                equation: EquationId::new(0),
            },
        ],
    );

    assert_opt_validation_message(&opt, "operand ValueId(9) is out of range");
}

#[test]
fn opt_validation_rejects_forward_scalar_operand() {
    let mir = lower_tiny_resistor_mir();
    let mut opt = OptModel::from_mir(&mir).expect("lower OptIR");
    opt.values = vec![
        OptValue {
            id: ValueId::new(0),
            value_type: OptValueType::Real,
            kind: OptValueKind::Unary {
                op: OptUnaryOp::Neg,
                input: ValueId::new(1),
            },
            derivatives: Vec::new(),
        },
        OptValue {
            id: ValueId::new(1),
            value_type: OptValueType::Real,
            kind: OptValueKind::RealConstant(1.0),
            derivatives: Vec::new(),
        },
    ];

    assert_opt_validation_message(&opt, "violates scalar value topological order");
}

#[test]
fn opt_validation_rejects_duplicate_derivative_lanes() {
    let mir = lower_tiny_resistor_mir();
    let mut opt = OptModel::from_mir(&mir).expect("lower OptIR");
    opt.values = vec![
        OptValue {
            id: ValueId::new(0),
            value_type: OptValueType::Real,
            kind: OptValueKind::RealConstant(1.0),
            derivatives: Vec::new(),
        },
        OptValue {
            id: ValueId::new(1),
            value_type: OptValueType::Real,
            kind: OptValueKind::NodePotential {
                node: NodeId::new(0),
            },
            derivatives: vec![
                OptDerivative {
                    lane: DerivativeLane::node(NodeId::new(0)),
                    value: ValueId::new(0),
                },
                OptDerivative {
                    lane: DerivativeLane::node(NodeId::new(0)),
                    value: ValueId::new(0),
                },
            ],
        },
    ];

    assert_opt_validation_message(&opt, "duplicate derivative lane");
}

#[test]
fn opt_validation_rejects_out_of_range_derivative_lane() {
    let mir = lower_tiny_resistor_mir();
    let mut opt = OptModel::from_mir(&mir).expect("lower OptIR");
    opt.values = vec![
        OptValue {
            id: ValueId::new(0),
            value_type: OptValueType::Real,
            kind: OptValueKind::RealConstant(1.0),
            derivatives: Vec::new(),
        },
        OptValue {
            id: ValueId::new(1),
            value_type: OptValueType::Real,
            kind: OptValueKind::NodePotential {
                node: NodeId::new(0),
            },
            derivatives: vec![OptDerivative {
                lane: DerivativeLane::node(NodeId::new(opt.node_count)),
                value: ValueId::new(0),
            }],
        },
    ];

    assert_opt_validation_message(&opt, "derivative lane");
    assert_opt_validation_message(&opt, "out of range");
}

#[test]
fn opt_validation_rejects_unsorted_derivative_lanes() {
    let mir = lower_tiny_resistor_mir();
    let mut opt = OptModel::from_mir(&mir).expect("lower OptIR");
    opt.values = vec![
        OptValue {
            id: ValueId::new(0),
            value_type: OptValueType::Real,
            kind: OptValueKind::RealConstant(1.0),
            derivatives: Vec::new(),
        },
        OptValue {
            id: ValueId::new(1),
            value_type: OptValueType::Real,
            kind: OptValueKind::NodePotential {
                node: NodeId::new(0),
            },
            derivatives: vec![
                OptDerivative {
                    lane: DerivativeLane::node(NodeId::new(1)),
                    value: ValueId::new(0),
                },
                OptDerivative {
                    lane: DerivativeLane::node(NodeId::new(0)),
                    value: ValueId::new(0),
                },
            ],
        },
    ];

    assert_opt_validation_message(&opt, "derivative lanes must be sorted");
}

#[test]
fn opt_validation_rejects_schedule_value_out_of_range() {
    let mir = lower_tiny_resistor_mir();
    let mut opt = OptModel::from_mir(&mir).expect("lower OptIR");
    opt.schedules[1].ops.insert(
        0,
        OptOp::ComputeValue {
            value: ValueId::new(42),
        },
    );

    assert_opt_validation_message(&opt, "ComputeValue ValueId(42) is out of range");
}

#[test]
fn opt_reference_evaluator_evaluates_scalar_value_graph() {
    let mir = lower_tiny_resistor_mir();
    let mut opt = OptModel::from_mir(&mir).expect("lower OptIR");
    opt.values = vec![
        OptValue {
            id: ValueId::new(0),
            value_type: OptValueType::Real,
            kind: OptValueKind::NodePotential {
                node: NodeId::new(0),
            },
            derivatives: Vec::new(),
        },
        OptValue {
            id: ValueId::new(1),
            value_type: OptValueType::Real,
            kind: OptValueKind::NodePotential {
                node: NodeId::new(1),
            },
            derivatives: Vec::new(),
        },
        OptValue {
            id: ValueId::new(2),
            value_type: OptValueType::Real,
            kind: OptValueKind::Binary {
                op: OptBinaryOp::Sub,
                left: ValueId::new(0),
                right: ValueId::new(1),
            },
            derivatives: Vec::new(),
        },
        OptValue {
            id: ValueId::new(3),
            value_type: OptValueType::Real,
            kind: OptValueKind::Parameter {
                parameter: ParamId::new(0),
            },
            derivatives: Vec::new(),
        },
        OptValue {
            id: ValueId::new(4),
            value_type: OptValueType::Real,
            kind: OptValueKind::Binary {
                op: OptBinaryOp::Div,
                left: ValueId::new(2),
                right: ValueId::new(3),
            },
            derivatives: Vec::new(),
        },
    ];
    set_newton_ops(
        &mut opt,
        vec![
            OptOp::ComputeValue {
                value: ValueId::new(4),
            },
            OptOp::EvaluateEquation {
                equation: EquationId::new(0),
            },
        ],
    );

    let snapshot = opt
        .evaluate(&OptEvalInputs {
            parameters: vec![1000.0],
            node_potentials: vec![5.0, 2.0],
            branch_flows: Vec::new(),
        })
        .expect("evaluate OptIR");

    assert_eq!(snapshot.real(ValueId::new(4)).expect("real value"), 0.003);
}

#[test]
fn opt_reference_evaluator_exposes_sparse_derivative_values() {
    let mir = lower_tiny_resistor_mir();
    let mut opt = OptModel::from_mir(&mir).expect("lower OptIR");
    opt.values = vec![
        OptValue {
            id: ValueId::new(0),
            value_type: OptValueType::Real,
            kind: OptValueKind::RealConstant(1.0),
            derivatives: Vec::new(),
        },
        OptValue {
            id: ValueId::new(1),
            value_type: OptValueType::Real,
            kind: OptValueKind::NodePotential {
                node: NodeId::new(0),
            },
            derivatives: vec![OptDerivative {
                lane: DerivativeLane::node(NodeId::new(0)),
                value: ValueId::new(0),
            }],
        },
    ];
    set_newton_ops(
        &mut opt,
        vec![
            OptOp::ComputeValue {
                value: ValueId::new(1),
            },
            OptOp::EvaluateEquation {
                equation: EquationId::new(0),
            },
        ],
    );

    let snapshot = opt
        .evaluate(&OptEvalInputs {
            parameters: vec![1000.0],
            node_potentials: vec![5.0, 2.0],
            branch_flows: Vec::new(),
        })
        .expect("evaluate OptIR");

    assert_eq!(
        snapshot.derivative(ValueId::new(1), DerivativeLane::node(NodeId::new(0))),
        Some(1.0)
    );
    assert_eq!(
        snapshot.derivative(ValueId::new(1), DerivativeLane::node(NodeId::new(1))),
        None
    );
}

#[test]
fn opt_reference_evaluator_evaluates_diode_like_expression() {
    let mir = lower_tiny_resistor_mir();
    let mut opt = OptModel::from_mir(&mir).expect("lower OptIR");
    opt.parameter_count = 2;
    opt.values = vec![
        OptValue {
            id: ValueId::new(0),
            value_type: OptValueType::Real,
            kind: OptValueKind::NodePotential {
                node: NodeId::new(0),
            },
            derivatives: Vec::new(),
        },
        OptValue {
            id: ValueId::new(1),
            value_type: OptValueType::Real,
            kind: OptValueKind::Parameter {
                parameter: ParamId::new(1),
            },
            derivatives: Vec::new(),
        },
        OptValue {
            id: ValueId::new(2),
            value_type: OptValueType::Real,
            kind: OptValueKind::Binary {
                op: OptBinaryOp::Div,
                left: ValueId::new(0),
                right: ValueId::new(1),
            },
            derivatives: Vec::new(),
        },
        OptValue {
            id: ValueId::new(3),
            value_type: OptValueType::Real,
            kind: OptValueKind::Unary {
                op: OptUnaryOp::Exp,
                input: ValueId::new(2),
            },
            derivatives: Vec::new(),
        },
        OptValue {
            id: ValueId::new(4),
            value_type: OptValueType::Real,
            kind: OptValueKind::RealConstant(1.0),
            derivatives: Vec::new(),
        },
        OptValue {
            id: ValueId::new(5),
            value_type: OptValueType::Real,
            kind: OptValueKind::Binary {
                op: OptBinaryOp::Sub,
                left: ValueId::new(3),
                right: ValueId::new(4),
            },
            derivatives: Vec::new(),
        },
        OptValue {
            id: ValueId::new(6),
            value_type: OptValueType::Real,
            kind: OptValueKind::Parameter {
                parameter: ParamId::new(0),
            },
            derivatives: Vec::new(),
        },
        OptValue {
            id: ValueId::new(7),
            value_type: OptValueType::Real,
            kind: OptValueKind::Binary {
                op: OptBinaryOp::Mul,
                left: ValueId::new(6),
                right: ValueId::new(5),
            },
            derivatives: Vec::new(),
        },
    ];
    set_newton_ops(
        &mut opt,
        vec![
            OptOp::ComputeValue {
                value: ValueId::new(7),
            },
            OptOp::EvaluateEquation {
                equation: EquationId::new(0),
            },
        ],
    );

    let snapshot = opt
        .evaluate(&OptEvalInputs {
            parameters: vec![1.0e-12, 0.026],
            node_potentials: vec![0.026, 0.0],
            branch_flows: Vec::new(),
        })
        .expect("evaluate OptIR");

    let expected = 1.0e-12 * (std::f64::consts::E - 1.0);
    let actual = snapshot.real(ValueId::new(7)).expect("diode current");
    assert!((actual - expected).abs() < 1.0e-24);
}

#[test]
fn opt_reference_evaluator_evaluates_conditional_select() {
    let mir = lower_tiny_resistor_mir();
    let mut opt = OptModel::from_mir(&mir).expect("lower OptIR");
    opt.values = vec![
        OptValue {
            id: ValueId::new(0),
            value_type: OptValueType::Real,
            kind: OptValueKind::NodePotential {
                node: NodeId::new(0),
            },
            derivatives: Vec::new(),
        },
        OptValue {
            id: ValueId::new(1),
            value_type: OptValueType::Real,
            kind: OptValueKind::RealConstant(0.0),
            derivatives: Vec::new(),
        },
        OptValue {
            id: ValueId::new(2),
            value_type: OptValueType::Boolean,
            kind: OptValueKind::Binary {
                op: OptBinaryOp::Gt,
                left: ValueId::new(0),
                right: ValueId::new(1),
            },
            derivatives: Vec::new(),
        },
        OptValue {
            id: ValueId::new(3),
            value_type: OptValueType::Real,
            kind: OptValueKind::RealConstant(10.0),
            derivatives: Vec::new(),
        },
        OptValue {
            id: ValueId::new(4),
            value_type: OptValueType::Real,
            kind: OptValueKind::RealConstant(-10.0),
            derivatives: Vec::new(),
        },
        OptValue {
            id: ValueId::new(5),
            value_type: OptValueType::Real,
            kind: OptValueKind::Select {
                condition: ValueId::new(2),
                then_value: ValueId::new(3),
                else_value: ValueId::new(4),
            },
            derivatives: Vec::new(),
        },
    ];
    set_newton_ops(
        &mut opt,
        vec![
            OptOp::ComputeValue {
                value: ValueId::new(5),
            },
            OptOp::EvaluateEquation {
                equation: EquationId::new(0),
            },
        ],
    );

    let snapshot = opt
        .evaluate(&OptEvalInputs {
            parameters: vec![1000.0],
            node_potentials: vec![5.0, 0.0],
            branch_flows: Vec::new(),
        })
        .expect("evaluate OptIR");

    assert_eq!(snapshot.boolean(ValueId::new(2)), Some(true));
    assert_eq!(snapshot.real(ValueId::new(5)), Some(10.0));
}

#[test]
fn opt_validation_rejects_schedules_out_of_invalidation_order() {
    let mir = lower_tiny_resistor_mir();
    let mut opt = OptModel::from_mir(&mir).expect("lower OptIR");
    opt.schedules.swap(0, 1);
    opt.schedules[0].id = ScheduleId::new(0);
    opt.schedules[1].id = ScheduleId::new(1);

    assert_opt_validation_message(&opt, "schedule order");
}

#[test]
fn opt_validation_rejects_equation_op_out_of_range() {
    let mir = lower_tiny_resistor_mir();
    let mut opt = OptModel::from_mir(&mir).expect("lower OptIR");
    let newton = opt
        .schedules
        .iter_mut()
        .find(|schedule| schedule.invalidation == InvalidationClass::NewtonIteration)
        .expect("NewtonIteration schedule");
    newton.ops[0] = OptOp::EvaluateEquation {
        equation: EquationId::new(opt.equation_count),
    };

    assert_opt_validation_message(&opt, "is out of range for 1 equations");
}

#[test]
fn opt_validation_rejects_duplicate_equation_op_in_schedule() {
    let mir = lower_tiny_resistor_mir();
    let mut opt = OptModel::from_mir(&mir).expect("lower OptIR");
    let newton = opt
        .schedules
        .iter_mut()
        .find(|schedule| schedule.invalidation == InvalidationClass::NewtonIteration)
        .expect("NewtonIteration schedule");
    newton.ops.push(OptOp::EvaluateEquation {
        equation: EquationId::new(0),
    });

    assert_opt_validation_message(&opt, "duplicate equation EquationId(0)");
}

#[test]
fn opt_validation_rejects_duplicate_invalidation_schedule() {
    let mir = lower_tiny_resistor_mir();
    let mut opt = OptModel::from_mir(&mir).expect("lower OptIR");
    opt.schedules.push(OptSchedule {
        id: ScheduleId::from(opt.schedules.len()),
        invalidation: InvalidationClass::InstanceStatic,
        ops: Vec::new(),
    });

    assert_opt_validation_message(&opt, "duplicate schedule for invalidation InstanceStatic");
}

#[test]
fn opt_validation_rejects_empty_module_name() {
    let mir = lower_tiny_resistor_mir();
    let mut opt = OptModel::from_mir(&mir).expect("lower OptIR");
    opt.module_name = "".into();

    assert_opt_validation_message(&opt, "OptIR module name must not be empty");
}

#[test]
fn opt_lowering_adds_instance_static_schedule_before_newton_for_parameters() {
    let mir = lower_tiny_resistor_mir();
    let opt = OptModel::from_mir(&mir).expect("lower OptIR");

    assert_eq!(opt.schedules.len(), 2);
    assert_eq!(
        opt.schedules[0].invalidation,
        InvalidationClass::InstanceStatic
    );
    assert_eq!(
        opt.schedules[1].invalidation,
        InvalidationClass::NewtonIteration
    );
}

#[test]
fn opt_lowering_schedules_parameter_dependent_derivatives_as_instance_static() {
    let (_, _, _, opt) = lower_tiny_resistor_parts();
    let newton = opt
        .schedules
        .iter()
        .find(|schedule| schedule.invalidation == InvalidationClass::NewtonIteration)
        .expect("NewtonIteration schedule");
    let Some(OptOp::ComputeValue { value: root }) = newton.ops.first() else {
        panic!("tiny resistor should have a scalar root: {newton:?}");
    };
    let positive_derivative = opt.values[usize::from(*root)]
        .derivatives
        .iter()
        .find(|derivative| derivative.lane == DerivativeLane::node(NodeId::new(0)))
        .expect("positive terminal derivative")
        .value;
    let instance_static = opt
        .schedules
        .iter()
        .find(|schedule| schedule.invalidation == InvalidationClass::InstanceStatic)
        .expect("InstanceStatic schedule");

    assert!(
        instance_static.ops.contains(&OptOp::ComputeValue {
            value: positive_derivative
        }),
        "expected derivative {positive_derivative} in InstanceStatic schedule: {instance_static:?}"
    );
    assert!(
        !newton.ops.contains(&OptOp::ComputeValue {
            value: positive_derivative
        }),
        "parameter-only derivative should not be scheduled in Newton: {newton:?}"
    );
}

#[test]
fn opt_lowering_omits_instance_static_schedule_without_parameters() {
    let mir = lower_internal_node_mir();
    let opt = OptModel::from_mir(&mir).expect("lower OptIR");

    assert!(
        opt.schedules
            .iter()
            .all(|schedule| schedule.invalidation != InvalidationClass::InstanceStatic)
    );
    assert_eq!(opt.schedules.len(), 1);
    assert_eq!(
        opt.schedules[0].invalidation,
        InvalidationClass::NewtonIteration
    );
    assert!(opt.validate().is_ok());
}

#[test]
fn opt_lowering_preserves_multi_equation_order_in_newton_schedule() {
    let mir = lower_internal_node_mir();
    let opt = OptModel::from_mir(&mir).expect("lower OptIR");
    let newton = opt
        .schedules
        .iter()
        .find(|schedule| schedule.invalidation == InvalidationClass::NewtonIteration)
        .expect("NewtonIteration schedule");

    assert_eq!(
        newton.ops,
        vec![
            OptOp::ComputeValue {
                value: ValueId::new(2)
            },
            OptOp::EvaluateEquation {
                equation: EquationId::new(0)
            },
            OptOp::ComputeValue {
                value: ValueId::new(4)
            },
            OptOp::EvaluateEquation {
                equation: EquationId::new(1)
            }
        ]
    );
    assert!(opt.validate().is_ok());
}

#[test]
fn artifact_dump_is_deterministic_and_contains_phase_summaries() {
    let (metadata, hir, mir, opt) = lower_tiny_resistor_parts();

    let artifact =
        CanonicalIrArtifact::from_parts(metadata, hir, mir, opt).expect("build artifact");
    let first = artifact.dump_text();
    let second = artifact.dump_text();

    assert_eq!(first, second);
    assert!(first.contains("canonical-veriloga-ir"));
    assert!(first.contains("schema_version=1"));
    assert!(first.contains("source_package=fixture"));
    assert!(first.contains("source_digest="));
    assert!(first.contains("compiler_version="));
    assert!(first.contains("hir_digest="));
    assert!(first.contains("mir_digest="));
    assert!(first.contains("opt_digest="));
    assert!(first.contains("hir module=tiny_res ports=2 parameters=1 contributions=1"));
    assert!(first.contains("mir nodes=2 equations=1"));
    assert!(first.contains("opt schedules=2"));
}

#[test]
fn artifact_validation_rejects_mismatched_module_names() {
    let (metadata, hir, mut mir, opt) = lower_tiny_resistor_parts();
    mir.module_name = "other_module".into();

    let diagnostics = CanonicalIrArtifact::from_parts(metadata, hir, mir, opt)
        .expect_err("module name mismatch must fail");

    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic.phase == CompilerPhase::Artifact
            && diagnostic.message.contains("module names must match")
    }));
}

#[test]
fn artifact_validation_rejects_mismatched_opt_equation_count() {
    let (metadata, hir, mir, mut opt) = lower_tiny_resistor_parts();
    opt.equation_count = 2;
    let newton = opt
        .schedules
        .iter_mut()
        .find(|schedule| schedule.invalidation == InvalidationClass::NewtonIteration)
        .expect("NewtonIteration schedule");
    newton.ops.push(OptOp::EvaluateEquation {
        equation: EquationId::new(1),
    });

    let diagnostics = CanonicalIrArtifact::from_parts(metadata, hir, mir, opt)
        .expect_err("OptIR equation count mismatch must fail");

    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic.phase == CompilerPhase::Artifact
            && diagnostic
                .message
                .contains("OptIR equation count 2 must match MIR equation count 1")
    }));
}

#[test]
fn artifact_validation_rejects_metadata_feature_flag_mismatch() {
    let (mut metadata, hir, mir, opt) = lower_tiny_resistor_parts();
    metadata.feature_flags.push("artifact-only".into());

    let diagnostics = CanonicalIrArtifact::from_parts(metadata, hir, mir, opt)
        .expect_err("metadata feature flags mismatch must fail");

    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic.phase == CompilerPhase::Artifact && diagnostic.message.contains("feature_flags")
    }));
}

#[test]
fn artifact_validation_rejects_hir_mir_parameter_mismatch() {
    let (metadata, hir, mut mir, opt) = lower_tiny_resistor_parts();
    mir.parameters[0].name = "conductance".into();

    let diagnostics = CanonicalIrArtifact::from_parts(metadata, hir, mir, opt)
        .expect_err("HIR/MIR parameter mismatch must fail");

    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic.phase == CompilerPhase::Artifact && diagnostic.message.contains("parameter 0")
    }));
}

#[test]
fn artifact_validation_rejects_hir_mir_equation_expression_mismatch() {
    let (metadata, hir, mut mir, opt) = lower_tiny_resistor_parts();
    mir.equations[0].expression.span.end += 1;

    let diagnostics = CanonicalIrArtifact::from_parts(metadata, hir, mir, opt)
        .expect_err("MIR equation expression mismatch must fail");

    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic.phase == CompilerPhase::Artifact
            && diagnostic.message.contains("equation 0 expression")
    }));
}

#[test]
fn artifact_validation_rejects_hir_mir_ground_node_mismatch() {
    let (metadata, hir, mut mir, opt) = lower_fixture_parts(ground_alias_source(), "ground_alias");
    assert!(mir.validate().is_ok());
    mir.ground_nodes = vec!["different_ground".into()];
    assert!(mir.validate().is_ok());

    let diagnostics = CanonicalIrArtifact::from_parts(metadata, hir, mir, opt)
        .expect_err("HIR/MIR ground node mismatch must fail");

    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic.phase == CompilerPhase::Artifact && diagnostic.message.contains("ground nodes")
    }));
}

#[test]
fn artifact_validation_rejects_newton_schedule_mismatch() {
    let analyzed = analyze_fixture(internal_node_source(), "has_mid").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", internal_node_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);
    let mir = MirModel::from_hir(&hir).expect("lower MIR");
    let mut opt = OptModel::from_mir(&mir).expect("lower OptIR");
    let newton = opt
        .schedules
        .iter_mut()
        .find(|schedule| schedule.invalidation == InvalidationClass::NewtonIteration)
        .expect("NewtonIteration schedule");
    let equation_op_positions: Vec<_> = newton
        .ops
        .iter()
        .enumerate()
        .filter_map(|(index, op)| match op {
            OptOp::EvaluateEquation { .. } => Some(index),
            OptOp::ComputeValue { .. } => None,
        })
        .collect();
    newton
        .ops
        .swap(equation_op_positions[0], equation_op_positions[1]);

    let diagnostics = CanonicalIrArtifact::from_parts(metadata, hir, mir, opt)
        .expect_err("Newton schedule mismatch must fail");

    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic.phase == CompilerPhase::Artifact
            && diagnostic.message.contains("NewtonIteration op 0")
    }));
}

#[test]
fn artifact_validation_accepts_opt_without_legacy_instance_schedule() {
    let (metadata, hir, mir, mut opt) = lower_tiny_resistor_parts();
    opt.schedules
        .retain(|schedule| schedule.invalidation != InvalidationClass::InstanceStatic);
    opt.schedules[0].id = ScheduleId::new(0);
    assert!(opt.validate().is_ok());

    let artifact = CanonicalIrArtifact::from_parts(metadata, hir, mir, opt)
        .expect("OptIR without legacy instance schedule");

    assert!(artifact.validate().is_ok());
}

#[test]
fn artifact_validation_accepts_scalar_opt_values_and_compute_ops() {
    let (metadata, hir, mir, mut opt) = lower_tiny_resistor_parts();
    opt.values = vec![
        OptValue {
            id: ValueId::new(0),
            value_type: OptValueType::Real,
            kind: OptValueKind::NodePotential {
                node: NodeId::new(0),
            },
            derivatives: Vec::new(),
        },
        OptValue {
            id: ValueId::new(1),
            value_type: OptValueType::Real,
            kind: OptValueKind::Parameter {
                parameter: ParamId::new(0),
            },
            derivatives: Vec::new(),
        },
        OptValue {
            id: ValueId::new(2),
            value_type: OptValueType::Real,
            kind: OptValueKind::Binary {
                op: OptBinaryOp::Div,
                left: ValueId::new(0),
                right: ValueId::new(1),
            },
            derivatives: Vec::new(),
        },
    ];
    set_newton_ops(
        &mut opt,
        vec![
            OptOp::ComputeValue {
                value: ValueId::new(2),
            },
            OptOp::EvaluateEquation {
                equation: EquationId::new(0),
            },
        ],
    );
    assert!(opt.validate().is_ok());

    let artifact =
        CanonicalIrArtifact::from_parts(metadata, hir, mir, opt).expect("scalar OptIR artifact");

    assert!(artifact.validate().is_ok());
}

#[test]
fn artifact_validation_rejects_mismatched_opt_topology_counts() {
    let (metadata, hir, mir, mut opt) = lower_tiny_resistor_parts();
    opt.node_count += 1;
    assert!(opt.validate().is_ok());

    let diagnostics = CanonicalIrArtifact::from_parts(metadata, hir, mir, opt)
        .expect_err("OptIR topology count mismatch must fail");

    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic.phase == CompilerPhase::Artifact
            && diagnostic
                .message
                .contains("OptIR node count 3 must match MIR node count 2")
    }));
}

#[test]
fn artifact_validation_rejects_invalid_child_ir() {
    let (metadata, hir, mut mir, opt) = lower_tiny_resistor_parts();
    mir.equations[0].active_domains.clear();

    let diagnostics = CanonicalIrArtifact::from_parts(metadata, hir, mir, opt)
        .expect_err("invalid MIR must fail");

    assert!(
        diagnostics
            .iter()
            .any(|diagnostic| diagnostic.phase == CompilerPhase::MirValidation)
    );
}

#[test]
fn artifact_validate_rejects_stale_digest_and_dump_recomputes_digest() {
    let (metadata, hir, mir, opt) = lower_tiny_resistor_parts();
    let mut artifact =
        CanonicalIrArtifact::from_parts(metadata, hir, mir, opt).expect("build artifact");
    let expected_digest = artifact.hir_digest.clone();
    artifact.hir_digest = "bogus".into();

    let diagnostics = artifact.validate().expect_err("stale digest must fail");

    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic.phase == CompilerPhase::Artifact && diagnostic.message.contains("hir_digest")
    }));
    let dump = artifact.dump_text();
    assert!(dump.contains(&format!("hir_digest={expected_digest}")));
    assert!(!dump.contains("hir_digest=bogus"));
}

#[test]
fn artifact_digest_encoding_distinguishes_delimited_alias_lists() {
    let (metadata, mut hir, _, _) = lower_tiny_resistor_parts();
    hir.parameters[0].aliases = vec!["a,b".into()];
    let mir = MirModel::from_hir(&hir).expect("lower comma alias MIR");
    let opt = OptModel::from_mir(&mir).expect("lower comma alias OptIR");
    let comma_alias = CanonicalIrArtifact::from_parts(metadata.clone(), hir, mir, opt)
        .expect("build comma alias artifact");

    let (_, mut hir, _, _) = lower_tiny_resistor_parts();
    hir.parameters[0].aliases = vec!["a".into(), "b".into()];
    let mir = MirModel::from_hir(&hir).expect("lower split alias MIR");
    let opt = OptModel::from_mir(&mir).expect("lower split alias OptIR");
    let split_alias = CanonicalIrArtifact::from_parts(metadata, hir, mir, opt)
        .expect("build split alias artifact");

    assert_ne!(comma_alias.hir_digest, split_alias.hir_digest);
    assert_ne!(comma_alias.mir_digest, split_alias.mir_digest);
}

#[test]
fn artifact_digests_distinguish_same_count_ir_content() {
    let (metadata, hir, mir, opt) = lower_tiny_resistor_parts();
    let baseline =
        CanonicalIrArtifact::from_parts(metadata.clone(), hir.clone(), mir.clone(), opt.clone())
            .expect("build baseline artifact");
    let mut changed_hir = hir;
    changed_hir.ports[0].direction = "output".into();
    let changed = CanonicalIrArtifact::from_parts(metadata, changed_hir, mir, opt)
        .expect("build changed artifact");

    assert_ne!(baseline.hir_digest, changed.hir_digest);
    assert_eq!(baseline.mir_digest, changed.mir_digest);
    assert_eq!(baseline.opt_digest, changed.opt_digest);
}

#[test]
fn opt_lowering_rejects_invalid_mir_before_building_schedules() {
    let mut mir = lower_tiny_resistor_mir();
    mir.module_name = "".into();

    let diagnostics = OptModel::from_mir(&mir).expect_err("invalid MIR must fail before OptIR");

    assert!(
        diagnostics
            .iter()
            .any(|diagnostic| diagnostic.phase == CompilerPhase::MirValidation)
    );
}

#[test]
fn mir_validation_rejects_empty_node_set() {
    let mut mir = lower_tiny_resistor_mir();
    mir.nodes.clear();

    assert_mir_validation_message(&mir, "MIR model must have at least one node");
}

#[test]
fn mir_validation_rejects_non_dense_node_and_equation_ids() {
    let mut mir = lower_tiny_resistor_mir();
    mir.nodes[1].id = NodeId::new(9);
    mir.equations[0].id = EquationId::new(3);

    let messages = mir_validation_messages(&mir);
    assert!(
        messages
            .iter()
            .any(|message| message.contains("MIR node IDs must be dense")),
        "expected dense node diagnostic, got {messages:?}"
    );
    assert!(
        messages
            .iter()
            .any(|message| message.contains("MIR equation IDs must be dense")),
        "expected dense equation diagnostic, got {messages:?}"
    );
}

#[test]
fn mir_validation_rejects_equation_with_empty_active_domains() {
    let mut mir = lower_tiny_resistor_mir();
    mir.equations[0].active_domains.clear();

    assert_mir_validation_message(&mir, "must have at least one active domain");
}

#[test]
fn mir_validation_rejects_equation_contribution_out_of_range() {
    let mut mir = lower_tiny_resistor_mir();
    mir.equations[0].contribution = ContributionId::new(42);

    assert_mir_validation_message(
        &mir,
        "contribution ContributionId(42) must match equation id EquationId(0)",
    );
}

#[test]
fn mir_validation_rejects_duplicate_contribution_refs() {
    let analyzed = analyze_fixture(internal_node_source(), "has_mid").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", internal_node_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);
    let mut mir = MirModel::from_hir(&hir).expect("lower MIR");

    assert_eq!(mir.equations.len(), 2);
    mir.equations[1].contribution = ContributionId::new(0);

    assert_mir_validation_message(
        &mir,
        "contribution ContributionId(0) must match equation id EquationId(1)",
    );
}

#[test]
fn mir_validation_rejects_parameter_alias_name_collision() {
    let mut mir = lower_tiny_resistor_mir();
    mir.parameters[0].aliases.push("other".into());
    let mut other = mir.parameters[0].clone();
    other.id = ParamId::new(1);
    other.name = "other".into();
    other.aliases.clear();
    mir.parameters.push(other);

    assert_mir_validation_message(&mir, "parameter alias 'other' collides with parameter name");
}

#[test]
fn mir_lowering_preserves_equation_expression_arena() {
    let analyzed = analyze_fixture(tiny_resistor_source(), "tiny_res").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", tiny_resistor_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);
    let mir = MirModel::from_hir(&hir).expect("lower MIR");

    assert_eq!(mir.expressions, hir.expressions);
    assert_eq!(mir.equations[0].expression, hir.contributions[0].expression);

    let HirExprKind::Binary { op, left, right } =
        &mir.expressions[usize::from(mir.equations[0].expression.id)].kind
    else {
        panic!("expected MIR equation expression to resolve to binary");
    };
    assert_eq!(op.as_str(), "Div");
    assert!(matches!(
        mir.expressions[usize::from(*left)].kind,
        HirExprKind::BranchAccess { .. }
    ));
    assert!(matches!(
        mir.expressions[usize::from(*right)].kind,
        HirExprKind::Identifier { .. }
    ));
}

#[test]
fn mir_validation_rejects_equation_expression_ref_out_of_range() {
    let mut mir = lower_tiny_resistor_mir();
    mir.equations[0].expression.id = ExprId::from(mir.expressions.len());

    assert_mir_validation_message(&mir, "equation 0 expression id ExprId");
}

#[test]
fn mir_validation_rejects_equation_expression_kind_mismatch() {
    let mut mir = lower_tiny_resistor_mir();
    mir.equations[0].expression.kind = "identifier".into();

    assert_mir_validation_message(
        &mir,
        "equation 0 expression kind 'identifier' does not match 'binary'",
    );
}

#[test]
fn mir_lowering_preserves_parameter_semantics() {
    let analyzed = analyze_fixture(tiny_resistor_source(), "tiny_res").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", tiny_resistor_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);
    let mir = MirModel::from_hir(&hir).expect("lower MIR");

    assert_eq!(mir.parameters.len(), 1);
    assert_eq!(mir.parameters[0].value_type, hir.parameters[0].value_type);
    assert_eq!(mir.parameters[0].default, Some(1000.0));
    assert_eq!(
        mir.parameters[0].default_expr,
        hir.parameters[0].default_expr
    );
    assert_eq!(mir.parameters[0].range, hir.parameters[0].range);
}

#[test]
fn mir_validation_rejects_parameter_default_expr_out_of_range() {
    let mut mir = lower_tiny_resistor_mir();
    let default_expr = mir.parameters[0]
        .default_expr
        .as_mut()
        .expect("tiny resistor parameter default expr");
    default_expr.id = ExprId::from(mir.expressions.len());

    assert_mir_validation_message(&mir, "parameter 'r' default id ExprId");
}

#[test]
fn mir_lowering_resolves_named_branch_endpoints() {
    let analyzed = analyze_fixture(named_branch_potential_source(), "branch_potential")
        .expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", named_branch_potential_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);
    let mir = MirModel::from_hir(&hir).expect("lower MIR");

    assert_eq!(mir.equations[0].branch.label.as_str(), "p,n");
    assert_eq!(mir.equations[0].branch.pos_node, Some(NodeId::new(0)));
    assert_eq!(mir.equations[0].branch.neg_node, Some(NodeId::new(1)));
}

#[test]
fn mir_lowering_promotes_potential_contributions_to_branch_unknowns() {
    let analyzed = analyze_fixture(named_branch_potential_source(), "branch_potential")
        .expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", named_branch_potential_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);
    let mir = MirModel::from_hir(&hir).expect("lower MIR");

    assert_eq!(mir.branch_unknowns.len(), 1);
    assert_eq!(mir.branch_unknowns[0].id, BranchUnknownId::new(0));
    assert_eq!(mir.branch_unknowns[0].equation, EquationId::new(0));
    assert_eq!(mir.branch_unknowns[0].declared_name.as_deref(), Some("res"));
    assert_eq!(mir.branch_unknowns[0].pos_node, Some(NodeId::new(0)));
    assert_eq!(mir.branch_unknowns[0].neg_node, Some(NodeId::new(1)));
}

#[test]
fn mir_validation_rejects_non_dense_branch_unknown_id() {
    let (_, _, mut mir, _) =
        lower_fixture_parts(named_branch_potential_source(), "branch_potential");
    mir.branch_unknowns[0].id = BranchUnknownId::new(7);

    assert_mir_validation_message(&mir, "MIR branch unknown IDs must be dense");
}

#[test]
fn mir_validation_rejects_duplicate_branch_unknown_equation() {
    let (_, _, mut mir, _) =
        lower_fixture_parts(named_branch_potential_source(), "branch_potential");
    let mut duplicate = mir.branch_unknowns[0].clone();
    duplicate.id = BranchUnknownId::new(1);
    mir.branch_unknowns.push(duplicate);

    assert_mir_validation_message(&mir, "duplicate branch unknown for equation EquationId(0)");
}

#[test]
fn mir_validation_rejects_branch_unknown_for_non_potential_equation() {
    let (_, _, mut mir, _) =
        lower_fixture_parts(named_branch_potential_source(), "branch_potential");
    mir.equations[0].kind = MirEquationKind::Current;

    assert_mir_validation_message(
        &mir,
        "branch unknown BranchUnknownId(0) must reference a potential equation",
    );
}

#[test]
fn mir_validation_rejects_branch_unknown_equation_table_mismatch() {
    let (_, _, mut mir, _) =
        lower_fixture_parts(named_branch_potential_source(), "branch_potential");
    mir.equations[0].id = EquationId::new(7);

    assert_mir_validation_message(
        &mir,
        "branch unknown BranchUnknownId(0) equation EquationId(0) does not match equation table entry EquationId(7)",
    );
}

#[test]
fn mir_validation_rejects_branch_unknown_declared_name_mismatch() {
    let (_, _, mut mir, _) =
        lower_fixture_parts(named_branch_potential_source(), "branch_potential");
    mir.branch_unknowns[0].declared_name = Some("other".into());

    assert_mir_validation_message(
        &mir,
        "branch unknown BranchUnknownId(0) declared name does not match equation EquationId(0) branch",
    );
}

#[test]
fn mir_validation_rejects_branch_unknown_endpoint_mismatch() {
    let (_, _, mut mir, _) =
        lower_fixture_parts(named_branch_potential_source(), "branch_potential");
    mir.branch_unknowns[0].neg_node = None;

    assert_mir_validation_message(
        &mir,
        "branch unknown BranchUnknownId(0) endpoints do not match equation EquationId(0) branch endpoints",
    );
}

#[test]
fn mir_validation_rejects_branch_unknown_pos_node_out_of_range() {
    let (_, _, mut mir, _) =
        lower_fixture_parts(named_branch_potential_source(), "branch_potential");
    mir.branch_unknowns[0].pos_node = Some(NodeId::new(99));

    assert_mir_validation_message(
        &mir,
        "branch unknown BranchUnknownId(0) pos_node NodeId(99) is out of range",
    );
}

#[test]
fn mir_validation_rejects_branch_unknown_without_concrete_endpoint() {
    let (_, _, mut mir, _) =
        lower_fixture_parts(named_branch_potential_source(), "branch_potential");
    mir.branch_unknowns[0].pos_node = None;
    mir.branch_unknowns[0].neg_node = None;

    assert_mir_validation_message(
        &mir,
        "branch unknown BranchUnknownId(0) must have a concrete endpoint",
    );
}

#[test]
fn mir_validation_rejects_branch_participation_out_of_range() {
    let mut mir = lower_tiny_resistor_mir();
    mir.equations[0].branch.pos_node = Some(NodeId::new(99));

    assert_mir_validation_message(&mir, "branch pos_node NodeId(99) is out of range");
}

#[test]
fn mir_validation_rejects_branch_label_endpoint_mismatch() {
    let mut mir = lower_tiny_resistor_mir();
    mir.equations[0].branch.label = "n,p".into();

    assert_mir_validation_message(&mir, "branch label 'n,p' does not match endpoints p,n");
}

#[test]
fn mir_validation_rejects_state_slot_owner_out_of_range() {
    let mut mir = lower_tiny_resistor_mir();
    mir.state_slots.push(MirStateSlot {
        id: StateId::new(0),
        name: "hidden_state".into(),
        owner: EquationId::new(7),
    });

    assert_mir_validation_message(&mir, "owner EquationId(7) is out of range");
}

#[test]
fn mir_validation_rejects_external_node_after_internal_node() {
    let analyzed = analyze_fixture(internal_node_source(), "has_mid").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", internal_node_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);
    let mut mir = MirModel::from_hir(&hir).expect("lower MIR");
    mir.nodes[1].is_external = false;
    mir.nodes[2].is_external = true;

    assert_mir_validation_message(&mir, "external nodes must precede internal nodes");
}

#[test]
fn mir_validation_rejects_duplicate_active_domains() {
    let mut mir = lower_tiny_resistor_mir();
    mir.equations[0].active_domains.push(MirAnalysisDomain::Dc);

    assert_mir_validation_message(&mir, "duplicate active domain Dc");
}

#[test]
fn mir_validation_rejects_empty_module_name() {
    let mut mir = lower_tiny_resistor_mir();
    mir.module_name = "".into();

    assert_mir_validation_message(&mir, "MIR module name must not be empty");
}

#[test]
fn mir_lowering_preserves_branch_table_for_named_accesses() {
    let analyzed = analyze_fixture(named_branch_potential_source(), "branch_potential")
        .expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", named_branch_potential_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);
    let mut mir = MirModel::from_hir(&hir).expect("lower MIR");

    assert_eq!(mir.branches.len(), 1);
    assert_eq!(mir.branches[0].id, BranchId::new(0));
    assert_eq!(mir.branches[0].name.as_str(), "res");
    assert_eq!(mir.branches[0].pos_node, Some(NodeId::new(0)));
    assert_eq!(mir.branches[0].neg_node, Some(NodeId::new(1)));
    assert_eq!(mir.branches[0].discipline.as_str(), "electrical");

    let root_id = mir.equations[0].expression.id;
    mir.expressions[usize::from(root_id)].kind = HirExprKind::NamedBranchAccess {
        access: "V".into(),
        name: "res".into(),
    };
    mir.equations[0].expression.kind = "branch_access".into();

    assert!(mir.expressions.iter().any(|expression| matches!(
        &expression.kind,
        HirExprKind::NamedBranchAccess { name, .. } if name.as_str() == "res"
    )));
    assert!(mir.validate().is_ok());
}

#[test]
fn mir_lowering_canonicalizes_ground_alias_branch_labels() {
    let analyzed = analyze_fixture(ground_alias_source(), "ground_alias").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", ground_alias_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);
    let mir = MirModel::from_hir(&hir).expect("lower MIR");

    assert_eq!(mir.nodes.len(), 2);
    assert_eq!(mir.equations[0].branch.pos_node, Some(NodeId::new(1)));
    assert_eq!(mir.equations[0].branch.neg_node, None);
    assert_eq!(mir.equations[0].branch.label.as_str(), "mid,0");
    assert!(mir.validate().is_ok());
}

#[test]
fn mir_lowering_supports_ground_positive_contribution() {
    let analyzed =
        analyze_fixture(ground_positive_source(), "ground_positive").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", ground_positive_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);
    let mir = MirModel::from_hir(&hir).expect("lower MIR without panicking");

    assert_eq!(mir.nodes.len(), 1);
    assert_eq!(mir.equations[0].branch.pos_node, None);
    assert_eq!(mir.equations[0].branch.neg_node, Some(NodeId::new(0)));
    assert_eq!(mir.equations[0].branch.label.as_str(), "0,p");
    assert!(mir.validate().is_ok());
}

#[test]
fn mir_lowering_supports_ground_positive_named_branch() {
    let analyzed = analyze_fixture(
        ground_positive_named_branch_source(),
        "ground_positive_branch",
    )
    .expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", ground_positive_named_branch_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);
    let mir = MirModel::from_hir(&hir).expect("lower MIR without panicking");

    assert_eq!(mir.branches.len(), 1);
    assert_eq!(mir.branches[0].pos_node, None);
    assert_eq!(mir.branches[0].neg_node, Some(NodeId::new(0)));
    assert_eq!(mir.equations[0].branch.pos_node, None);
    assert_eq!(mir.equations[0].branch.neg_node, Some(NodeId::new(0)));
    assert_eq!(mir.equations[0].branch.label.as_str(), "0,p");
    assert!(mir.validate().is_ok());
}

#[test]
fn mir_validation_rejects_ground_ground_branch_participation() {
    let mut mir = lower_tiny_resistor_mir();
    mir.equations[0].branch.pos_node = None;
    mir.equations[0].branch.neg_node = None;
    mir.equations[0].branch.label = "0,0".into();

    assert_mir_validation_message(&mir, "must have at least one concrete endpoint");
}

#[test]
fn mir_validation_rejects_noncanonical_grounded_branch_label() {
    let mut mir = lower_tiny_resistor_mir();
    mir.equations[0].branch.neg_node = None;
    mir.equations[0].branch.label = "p,gnd".into();

    assert_mir_validation_message(&mir, "branch label 'p,gnd' does not match endpoints p,0");
}

#[test]
fn mir_validation_rejects_expression_child_out_of_range() {
    let mut mir = lower_tiny_resistor_mir();
    let root_id = mir.equations[0].expression.id;
    let dangling_id = ExprId::from(mir.expressions.len());
    let HirExprKind::Binary { left, .. } = &mut mir.expressions[usize::from(root_id)].kind else {
        panic!("expected MIR equation expression to be binary");
    };
    *left = dangling_id;

    assert_mir_validation_message(&mir, "child left ExprId");
}

#[test]
fn mir_validation_rejects_expression_child_postorder_violation() {
    let mut mir = lower_tiny_resistor_mir();
    let root_id = mir.equations[0].expression.id;
    let HirExprKind::Binary { left, .. } = &mut mir.expressions[usize::from(root_id)].kind else {
        panic!("expected MIR equation expression to be binary");
    };
    *left = root_id;

    assert_mir_validation_message(&mir, "violates expression postorder");
}

#[test]
fn mir_validation_rejects_expression_branch_access_missing_node() {
    let mut mir = lower_tiny_resistor_mir();
    let root_id = mir.equations[0].expression.id;
    let HirExprKind::Binary { left, .. } = &mir.expressions[usize::from(root_id)].kind else {
        panic!("expected MIR equation expression to be binary");
    };
    let branch_access_id = *left;
    let HirExprKind::BranchAccess { pos, .. } =
        &mut mir.expressions[usize::from(branch_access_id)].kind
    else {
        panic!("expected branch access expression");
    };
    *pos = "missing".into();

    assert_mir_validation_message(&mir, "unknown branch access node 'missing'");
}

#[test]
fn mir_validation_rejects_expression_named_branch_access_missing_branch() {
    let analyzed = analyze_fixture(named_branch_potential_source(), "branch_potential")
        .expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", named_branch_potential_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);
    let mut mir = MirModel::from_hir(&hir).expect("lower MIR");

    let root_id = mir.equations[0].expression.id;
    mir.expressions[usize::from(root_id)].kind = HirExprKind::NamedBranchAccess {
        access: "V".into(),
        name: "missing".into(),
    };
    mir.equations[0].expression.kind = "branch_access".into();

    assert_mir_validation_message(&mir, "unknown named branch access 'missing'");
}

#[test]
fn mir_validation_rejects_state_slot_name_violations() {
    let mut empty_name = lower_tiny_resistor_mir();
    empty_name.state_slots.push(MirStateSlot {
        id: StateId::new(0),
        name: "".into(),
        owner: EquationId::new(0),
    });
    assert_mir_validation_message(&empty_name, "state slot StateId(0) name must not be empty");

    let mut duplicate_name = lower_tiny_resistor_mir();
    duplicate_name.state_slots.push(MirStateSlot {
        id: StateId::new(0),
        name: "state".into(),
        owner: EquationId::new(0),
    });
    duplicate_name.state_slots.push(MirStateSlot {
        id: StateId::new(1),
        name: "state".into(),
        owner: EquationId::new(0),
    });
    assert_mir_validation_message(&duplicate_name, "duplicate state slot name 'state'");
}

#[test]
fn hir_lowering_preserves_expression_tree_structure() {
    let analyzed = analyze_fixture(tiny_resistor_source(), "tiny_res").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", tiny_resistor_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);
    let contribution_expr = &hir.contributions[0].expression;

    assert_eq!(
        hir.expressions[usize::from(contribution_expr.id)].id,
        contribution_expr.id
    );
    let HirExprKind::Binary { op, left, right } =
        &hir.expressions[usize::from(contribution_expr.id)].kind
    else {
        panic!("expected top-level contribution expression to be binary");
    };
    assert_eq!(op.as_str(), "Div");

    let HirExprKind::BranchAccess { access, pos, neg } = &hir.expressions[usize::from(*left)].kind
    else {
        panic!("expected binary lhs to preserve branch access");
    };
    assert_eq!(access.as_str(), "V");
    assert_eq!(pos.as_str(), "p");
    assert_eq!(neg.as_deref(), Some("n"));

    let HirExprKind::Identifier { name } = &hir.expressions[usize::from(*right)].kind else {
        panic!("expected binary rhs to preserve identifier");
    };
    assert_eq!(name.as_str(), "r");
}

#[test]
fn hir_lowering_preserves_laplace_operand_groups() {
    let span = Span::dummy();
    let number = |value: f64, raw: &str| {
        Expression::Number(NumberLit {
            value,
            raw: raw.into(),
            span,
        })
    };
    let analyzed = AnalyzedModule {
        name: "laplace_filter".into(),
        ports: vec![
            AnalyzedPort {
                name: "p".into(),
                direction: PortDirection::Inout,
                discipline: "electrical".into(),
                nature_potential: Some("voltage".into()),
                nature_flow: Some("current".into()),
            },
            AnalyzedPort {
                name: "n".into(),
                direction: PortDirection::Inout,
                discipline: "electrical".into(),
                nature_potential: Some("voltage".into()),
                nature_flow: Some("current".into()),
            },
        ],
        parameters: Vec::new(),
        param_aliases: Vec::new(),
        variables: Vec::new(),
        branches: Vec::new(),
        contributions: vec![AnalyzedContribution {
            branch: "p,n".into(),
            declared_branch: None,
            is_current: false,
            indirect: false,
            expression: Expression::AnalogOperator(AnalogOperator::Laplace {
                kind: LaplaceKind::NumeratorDenominator {
                    numerator: vec![number(1.0, "1.0"), number(2.0, "2.0")],
                    denominator: vec![number(0.5, "0.5")],
                },
                expr: Box::new(Expression::BranchAccess(BranchAccess::Nodes {
                    access: "I".into(),
                    pos: "p".into(),
                    neg: Some("n".into()),
                    span,
                })),
                span,
            }),
            expr_type: ValueType::Real,
            span,
        }],
        statements: Vec::new(),
        internal_nodes: Vec::new(),
        ground_nodes: Vec::new(),
        arrays: HashMap::new(),
        symbol_table: SymbolTable::new(),
    };
    let metadata = CanonicalMetadata::for_source("fixture", "laplace_filter");
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);

    let contribution_expr = &hir.contributions[0].expression;
    let HirExprKind::Laplace { expr, kind } =
        &hir.expressions[usize::from(contribution_expr.id)].kind
    else {
        panic!("expected top-level contribution expression to preserve Laplace structure");
    };

    assert!(matches!(
        hir.expressions[usize::from(*expr)].kind,
        HirExprKind::BranchAccess { .. }
    ));

    let HirLaplaceKind::NumeratorDenominator {
        numerator,
        denominator,
    } = kind
    else {
        panic!("expected laplace_nd to preserve numerator and denominator groups");
    };

    assert_eq!(numerator.len(), 2);
    assert_eq!(denominator.len(), 1);
    assert!(
        numerator
            .iter()
            .all(|id| usize::from(*id) < hir.expressions.len())
    );
    assert!(
        denominator
            .iter()
            .all(|id| usize::from(*id) < hir.expressions.len())
    );
}

#[test]
fn hir_lowering_preserves_typed_analog_operator_slots() {
    let span = Span::dummy();
    let number = |value: f64, raw: &str| {
        Expression::Number(NumberLit {
            value,
            raw: raw.into(),
            span,
        })
    };
    let analyzed = AnalyzedModule {
        name: "typed_ops".into(),
        ports: vec![
            AnalyzedPort {
                name: "p".into(),
                direction: PortDirection::Inout,
                discipline: "electrical".into(),
                nature_potential: Some("voltage".into()),
                nature_flow: Some("current".into()),
            },
            AnalyzedPort {
                name: "n".into(),
                direction: PortDirection::Inout,
                discipline: "electrical".into(),
                nature_potential: Some("voltage".into()),
                nature_flow: Some("current".into()),
            },
        ],
        parameters: Vec::new(),
        param_aliases: Vec::new(),
        variables: Vec::new(),
        branches: Vec::new(),
        contributions: vec![AnalyzedContribution {
            branch: "p,n".into(),
            declared_branch: None,
            is_current: true,
            indirect: false,
            expression: Expression::AnalogOperator(AnalogOperator::IdtMod {
                expr: Box::new(Expression::BranchAccess(BranchAccess::Nodes {
                    access: "V".into(),
                    pos: "p".into(),
                    neg: Some("n".into()),
                    span,
                })),
                ic: None,
                modulus: Some(Box::new(number(6.28, "6.28"))),
                offset: None,
                abstol: Some(Box::new(number(1e-9, "1e-9"))),
                span,
            }),
            expr_type: ValueType::Real,
            span,
        }],
        statements: Vec::new(),
        internal_nodes: Vec::new(),
        ground_nodes: Vec::new(),
        arrays: HashMap::new(),
        symbol_table: SymbolTable::new(),
    };
    let metadata = CanonicalMetadata::for_source("fixture", "typed_ops");
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);

    let contribution_expr = &hir.contributions[0].expression;
    let HirExprKind::AnalogOperator { op } =
        &hir.expressions[usize::from(contribution_expr.id)].kind
    else {
        panic!("expected top-level contribution expression to preserve analog operator");
    };

    let HirAnalogOperator::IdtMod {
        expr,
        ic,
        modulus,
        offset,
        abstol,
    } = op
    else {
        panic!("expected idtmod operator payload");
    };
    assert!(matches!(
        hir.expressions[usize::from(*expr)].kind,
        HirExprKind::BranchAccess { .. }
    ));
    assert!(ic.is_none());
    assert!(modulus.is_some());
    assert!(offset.is_none());
    assert!(abstol.is_some());
}

#[test]
fn hir_lowering_preserves_dynamic_array_assignment_target_and_index() {
    let analyzed = analyze_fixture(dynamic_array_source(), "dyn_array").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", dynamic_array_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);
    let array = analyzed.arrays.get("xs").expect("array layout");
    let array_len: u32 = hir.arrays[0].len;

    let assignment = hir
        .statements
        .iter()
        .find_map(|statement| match statement {
            HirStatement::Assignment(assignment) if assignment.target_name.as_str() == "xs" => {
                Some(assignment)
            }
            _ => None,
        })
        .expect("dynamic array assignment");

    assert_eq!(array_len, 4);
    assert_eq!(assignment.target, VariableId::from(array.base));
    assert_eq!(assignment.target_name.as_str(), "xs");
    assert_eq!(
        assignment.index.as_ref().map(|expr| expr.kind.as_str()),
        Some("identifier")
    );
    assert_eq!(assignment.expr.kind.as_str(), "identifier");
    assert!(hir.validate().is_ok());
}

#[test]
fn hir_validation_rejects_invalid_contribution_references() {
    let analyzed = analyze_fixture(tiny_resistor_source(), "tiny_res").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", tiny_resistor_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);

    let mut missing_branch = hir.clone();
    missing_branch.contributions[0].branch = "missing".into();
    let diagnostics = missing_branch
        .validate()
        .expect_err("missing branch must fail validation");
    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic
            .message
            .contains("unknown contribution branch 'missing'")
    }));

    let mut missing_node = hir;
    missing_node.contributions[0].branch = "p,missing".into();
    let diagnostics = missing_node
        .validate()
        .expect_err("missing node pair must fail validation");
    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic
            .message
            .contains("unknown contribution branch 'p,missing'")
    }));
}

#[test]
fn hir_validation_rejects_expression_arena_invariants() {
    let analyzed = analyze_fixture(tiny_resistor_source(), "tiny_res").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", tiny_resistor_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);

    let dangling_id = ExprId::from(hir.expressions.len());

    let mut dangling_contribution_ref = hir.clone();
    dangling_contribution_ref.contributions[0].expression.id = dangling_id;
    assert_validation_message(
        &dangling_contribution_ref,
        "expression ref contribution 0 expression id ExprId",
    );

    let mut mismatched_ref_kind = hir.clone();
    mismatched_ref_kind.contributions[0].expression.kind = "identifier".into();
    assert_validation_message(
        &mismatched_ref_kind,
        "expression ref contribution 0 expression kind 'identifier' does not match 'binary'",
    );

    let mut dangling_binary_child = hir.clone();
    let root_id = dangling_binary_child.contributions[0].expression.id;
    let HirExprKind::Binary { left, .. } =
        &mut dangling_binary_child.expressions[usize::from(root_id)].kind
    else {
        panic!("expected contribution expression to be binary");
    };
    *left = dangling_id;
    assert_validation_message(&dangling_binary_child, "child left ExprId");

    let mut non_postorder_child = hir;
    let root_id = non_postorder_child.contributions[0].expression.id;
    let HirExprKind::Binary { left, .. } =
        &mut non_postorder_child.expressions[usize::from(root_id)].kind
    else {
        panic!("expected contribution expression to be binary");
    };
    *left = root_id;
    assert_validation_message(&non_postorder_child, "violates expression postorder");
}

#[test]
fn hir_validation_rejects_unknown_identifier_expression() {
    let analyzed = analyze_fixture(tiny_resistor_source(), "tiny_res").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", tiny_resistor_source());
    let mut hir = HirModel::from_analyzed_module(&metadata, &analyzed);
    let root_id = hir.contributions[0].expression.id;
    let HirExprKind::Binary { right, .. } = hir.expressions[usize::from(root_id)].kind else {
        panic!("expected contribution expression to be binary");
    };
    let HirExprKind::Identifier { name } = &mut hir.expressions[usize::from(right)].kind else {
        panic!("expected binary rhs to be an identifier");
    };
    *name = "__missing".into();

    assert_validation_message(&hir, "unknown identifier '__missing'");
}

#[test]
fn hir_validation_rejects_alias_identifier_expression() {
    let analyzed = analyze_fixture(tiny_resistor_source(), "tiny_res").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", tiny_resistor_source());
    let mut hir = HirModel::from_analyzed_module(&metadata, &analyzed);
    hir.parameters[0].aliases.push("res".into());
    let root_id = hir.contributions[0].expression.id;
    let HirExprKind::Binary { right, .. } = hir.expressions[usize::from(root_id)].kind else {
        panic!("expected contribution expression to be binary");
    };
    let HirExprKind::Identifier { name } = &mut hir.expressions[usize::from(right)].kind else {
        panic!("expected binary rhs to be an identifier");
    };
    *name = "res".into();

    assert_validation_message(&hir, "unknown identifier 'res'");
}

#[test]
fn mir_validation_rejects_unknown_identifier_expression() {
    let mut mir = lower_tiny_resistor_mir();
    let root_id = mir.equations[0].expression.id;
    let HirExprKind::Binary { right, .. } = mir.expressions[usize::from(root_id)].kind else {
        panic!("expected equation expression to be binary");
    };
    let HirExprKind::Identifier { name } = &mut mir.expressions[usize::from(right)].kind else {
        panic!("expected binary rhs to be an identifier");
    };
    *name = "__missing".into();

    assert_mir_validation_message(&mir, "unknown identifier '__missing'");
}

#[test]
fn mir_validation_rejects_value_symbol_table_violations() {
    let mir = lower_tiny_resistor_mir();

    let mut duplicate = mir.clone();
    duplicate.value_symbols.push("r".into());
    assert_mir_validation_message(&duplicate, "duplicate value symbol 'r'");

    let mut unsorted = mir;
    unsorted.value_symbols.push("a".into());
    assert_mir_validation_message(&unsorted, "value symbols must be sorted");
}

#[test]
fn mir_validation_rejects_alias_identifier_expression() {
    let analyzed = analyze_fixture(tiny_resistor_source(), "tiny_res").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", tiny_resistor_source());
    let mut hir = HirModel::from_analyzed_module(&metadata, &analyzed);
    hir.parameters[0].aliases.push("res".into());
    let mut mir = MirModel::from_hir(&hir).expect("lower alias MIR");
    let root_id = mir.equations[0].expression.id;
    let HirExprKind::Binary { right, .. } = mir.expressions[usize::from(root_id)].kind else {
        panic!("expected equation expression to be binary");
    };
    let HirExprKind::Identifier { name } = &mut mir.expressions[usize::from(right)].kind else {
        panic!("expected binary rhs to be an identifier");
    };
    *name = "res".into();

    assert_mir_validation_message(&mir, "unknown identifier 'res'");
}

#[test]
fn hir_validation_rejects_dangling_assignment_index_expression_ref() {
    let analyzed = analyze_fixture(dynamic_array_source(), "dyn_array").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", dynamic_array_source());
    let mut hir = HirModel::from_analyzed_module(&metadata, &analyzed);
    let dangling_id = ExprId::from(hir.expressions.len());

    let HirStatement::Assignment(assignment) = &mut hir.statements[0] else {
        panic!("expected dynamic array assignment");
    };
    assignment.index.as_mut().expect("assignment index").id = dangling_id;

    assert_validation_message(&hir, "expression ref assignment 'xs' index id ExprId");
}

#[test]
fn hir_validation_rejects_unknown_branch_access_nodes() {
    let analyzed = analyze_fixture(tiny_resistor_source(), "tiny_res").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", tiny_resistor_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);

    let mut missing_pos = hir.clone();
    let branch_access_id = contribution_branch_access_id(&missing_pos);
    let HirExprKind::BranchAccess { pos, .. } =
        &mut missing_pos.expressions[usize::from(branch_access_id)].kind
    else {
        panic!("expected branch access expression");
    };
    *pos = "missing".into();
    assert_validation_message(&missing_pos, "unknown branch access node 'missing'");

    let mut missing_neg = hir;
    let branch_access_id = contribution_branch_access_id(&missing_neg);
    let HirExprKind::BranchAccess { neg, .. } =
        &mut missing_neg.expressions[usize::from(branch_access_id)].kind
    else {
        panic!("expected branch access expression");
    };
    *neg = Some("missing".into());
    assert_validation_message(&missing_neg, "unknown branch access node 'missing'");
}

#[test]
fn hir_validation_rejects_unknown_named_branch_access() {
    let analyzed = analyze_fixture(named_branch_potential_source(), "branch_potential")
        .expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", named_branch_potential_source());
    let mut hir = HirModel::from_analyzed_module(&metadata, &analyzed);
    let root_id = hir.contributions[0].expression.id;

    hir.expressions[usize::from(root_id)].kind = HirExprKind::NamedBranchAccess {
        access: "V".into(),
        name: "missing".into(),
    };
    hir.contributions[0].expression.kind = "branch_access".into();

    assert_validation_message(&hir, "unknown named branch access 'missing'");
}

#[test]
fn hir_validation_rejects_malformed_branch_declarations() {
    let analyzed = analyze_fixture(named_branch_potential_source(), "branch_potential")
        .expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", named_branch_potential_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);

    let mut empty_name = hir.clone();
    empty_name.branches[0].name = "".into();
    assert_validation_message(&empty_name, "branch name must not be empty");

    let mut duplicate_name = hir.clone();
    let mut duplicate_branch = duplicate_name.branches[0].clone();
    duplicate_branch.id = BranchId::new(1);
    duplicate_name.branches.push(duplicate_branch);
    assert_validation_message(&duplicate_name, "duplicate branch name 'res'");

    let mut unknown_pos = hir.clone();
    unknown_pos.branches[0].pos_node = "missing".into();
    assert_validation_message(&unknown_pos, "branch 'res' pos_node 'missing' is unknown");

    let mut unknown_neg = hir;
    unknown_neg.branches[0].neg_node = "missing".into();
    assert_validation_message(&unknown_neg, "branch 'res' neg_node 'missing' is unknown");
}

#[test]
fn hir_validation_rejects_parameter_alias_namespace_violations() {
    let analyzed = analyze_fixture(tiny_resistor_source(), "tiny_res").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", tiny_resistor_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);

    let mut empty_alias = hir.clone();
    empty_alias.parameters[0].aliases.push("".into());
    assert_validation_message(&empty_alias, "parameter alias for 'r' must not be empty");

    let mut alias_name_collision = hir.clone();
    alias_name_collision.parameters[0].aliases.push("r".into());
    assert_validation_message(
        &alias_name_collision,
        "parameter alias 'r' collides with parameter name",
    );

    let mut duplicate_parameter_name = hir;
    let mut duplicate = duplicate_parameter_name.parameters[0].clone();
    duplicate.id = ParamId::new(1);
    duplicate.aliases.clear();
    duplicate_parameter_name.parameters.push(duplicate);
    assert_validation_message(&duplicate_parameter_name, "duplicate parameter name 'r'");
}

#[test]
fn hir_validation_rejects_assignment_target_name_and_shape_mismatches() {
    let analyzed =
        analyze_fixture(scalar_assignment_source(), "scalar_assign").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", scalar_assignment_source());
    let mut scalar_hir = HirModel::from_analyzed_module(&metadata, &analyzed);
    let HirStatement::Assignment(scalar_assignment) = &mut scalar_hir.statements[0] else {
        panic!("expected scalar assignment");
    };
    scalar_assignment.target_name = "not_x".into();
    scalar_assignment.index = Some(scalar_assignment.expr.clone());

    let diagnostics = scalar_hir
        .validate()
        .expect_err("scalar assignment mismatches must fail validation");
    let messages: Vec<_> = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.message.as_str())
        .collect();
    assert!(
        messages
            .iter()
            .any(|message| message.contains("target name 'not_x' does not match variable 'x'"))
    );
    assert!(
        messages
            .iter()
            .any(|message| message.contains("scalar assignment 'not_x' must not have an index"))
    );

    let analyzed = analyze_fixture(dynamic_array_source(), "dyn_array").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", dynamic_array_source());
    let mut array_hir = HirModel::from_analyzed_module(&metadata, &analyzed);
    let HirStatement::Assignment(array_assignment) = &mut array_hir.statements[0] else {
        panic!("expected array assignment");
    };
    array_assignment.target_name = "xs[0]".into();

    let diagnostics = array_hir
        .validate()
        .expect_err("array assignment name mismatch must fail validation");
    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic
            .message
            .contains("indexed assignment target name 'xs[0]' does not match array 'xs'")
    }));

    let mut missing_index_hir = HirModel::from_analyzed_module(
        &CanonicalMetadata::for_source("fixture", dynamic_array_source()),
        &analyzed,
    );
    let HirStatement::Assignment(array_assignment) = &mut missing_index_hir.statements[0] else {
        panic!("expected array assignment");
    };
    array_assignment.index = None;

    let diagnostics = missing_index_hir
        .validate()
        .expect_err("array assignment without index must fail validation");
    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic
            .message
            .contains("array assignment 'xs' must include an index")
    }));
}

#[test]
fn hir_lowering_preserves_internal_node_metadata() {
    let analyzed = analyze_fixture(internal_node_source(), "has_mid").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", internal_node_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);

    assert_eq!(hir.internal_nodes.len(), 1);
    assert_eq!(hir.internal_nodes[0].id, NodeId::new(0));
    assert_eq!(hir.internal_nodes[0].name.as_str(), "mid");
    assert_eq!(hir.internal_nodes[0].discipline.as_str(), "electrical");
    assert_eq!(hir.internal_nodes[0].index, 0);
    assert!(hir.validate().is_ok());
}

#[test]
fn mir_lowering_appends_internal_nodes_after_external_ports() {
    let analyzed = analyze_fixture(internal_node_source(), "has_mid").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", internal_node_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);
    let mir = MirModel::from_hir(&hir).expect("lower MIR");

    assert_eq!(mir.nodes.len(), 3);
    assert_eq!(mir.nodes[0].id, NodeId::new(0));
    assert_eq!(mir.nodes[0].name.as_str(), "p");
    assert!(mir.nodes[0].is_external);
    assert_eq!(mir.nodes[1].id, NodeId::new(1));
    assert_eq!(mir.nodes[1].name.as_str(), "n");
    assert!(mir.nodes[1].is_external);
    assert_eq!(mir.nodes[2].id, NodeId::new(2));
    assert_eq!(mir.nodes[2].name.as_str(), "mid");
    assert!(!mir.nodes[2].is_external);
    assert!(mir.validate().is_ok());
}

#[test]
fn hir_lowering_represents_named_branch_potential_contribution() {
    let analyzed = analyze_fixture(named_branch_potential_source(), "branch_potential")
        .expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", named_branch_potential_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);

    assert_eq!(hir.branches.len(), 1);
    assert_eq!(hir.branches[0].name.as_str(), "res");
    assert_eq!(hir.contributions.len(), 1);
    assert_eq!(hir.contributions[0].branch.as_str(), "p,n");
    assert_eq!(hir.contributions[0].kind, HirContributionKind::Potential);
    assert!(hir.validate().is_ok());
}

#[test]
fn hir_lowering_canonicalizes_single_argument_named_branch_accesses() {
    let analyzed = analyze_fixture(named_branch_access_source(), "named_branch_access")
        .expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", named_branch_access_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);

    assert!(hir.expressions.iter().any(|expression| matches!(
        &expression.kind,
        HirExprKind::NamedBranchAccess { access, name }
            if access.as_str() == "I" && name.as_str() == "probe"
    )));
    assert!(hir.expressions.iter().any(|expression| matches!(
        &expression.kind,
        HirExprKind::NamedBranchAccess { access, name }
            if access.as_str() == "V" && name.as_str() == "probe"
    )));
    assert!(hir.validate().is_ok());
}

#[test]
fn hir_lowering_preserves_angle_bracket_terminal_current_accesses() {
    let analyzed = analyze_fixture(
        implicit_terminal_branch_access_source(),
        "implicit_terminal_branch_access",
    )
    .expect("analyze fixture");
    let metadata =
        CanonicalMetadata::for_source("fixture", implicit_terminal_branch_access_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);

    assert!(hir.branches.is_empty());
    assert!(hir.expressions.iter().any(|expression| matches!(
        &expression.kind,
        HirExprKind::BranchAccess { access, pos, neg }
            if access.as_str() == "I" && pos.as_str() == "p" && neg.is_none()
    )));
    assert!(!hir.expressions.iter().any(|expression| matches!(
        &expression.kind,
        HirExprKind::NamedBranchAccess { access, name }
            if access.as_str() == "I" && name.as_str() == "p"
    )));
    assert!(hir.validate().is_ok());
}

#[test]
fn hir_validation_rejects_malformed_structure() {
    let analyzed = analyze_fixture(hir_validation_surface_source(), "validation_surface")
        .expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", hir_validation_surface_source());
    let mut hir = HirModel::from_analyzed_module(&metadata, &analyzed);

    hir.ports[1].id = PortId::new(7);
    hir.parameters[0].aliases.push("shared".into());
    hir.parameters.push(hir.parameters[0].clone());
    hir.parameters[1].id = ParamId::new(1);
    hir.parameters[1].aliases.push("shared".into());
    hir.branches[0].id = BranchId::new(7);
    hir.contributions[0].id = ContributionId::new(9);
    hir.internal_nodes[0].id = NodeId::new(11);
    hir.internal_nodes[0].index = 17;

    let diagnostics = hir.validate().expect_err("malformed HIR must fail");
    let messages: Vec<_> = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.message.as_str())
        .collect();

    assert!(
        messages
            .iter()
            .any(|message| message.contains("port IDs must be dense"))
    );
    assert!(
        messages
            .iter()
            .any(|message| message.contains("duplicate parameter alias 'shared'"))
    );
    assert!(
        messages
            .iter()
            .any(|message| message.contains("branch IDs must be dense"))
    );
    assert!(
        messages
            .iter()
            .any(|message| message.contains("contribution IDs must be dense"))
    );
    assert!(
        messages
            .iter()
            .any(|message| message.contains("internal node IDs must be dense"))
    );
    assert!(
        messages
            .iter()
            .any(|message| message.contains("internal node index 17 does not match id NodeId(11)"))
    );
}

#[test]
fn hir_validation_rejects_malformed_array_contribution_and_statement_paths() {
    let analyzed = analyze_fixture(dynamic_array_source(), "dyn_array").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", dynamic_array_source());
    let mut hir = HirModel::from_analyzed_module(&metadata, &analyzed);
    let variable_count = hir.variables.len();

    hir.arrays[0].base = VariableId::from(variable_count);
    hir.contributions[0].branch = "".into();

    let mut invalid_assignment = hir
        .statements
        .iter()
        .find_map(|statement| match statement {
            HirStatement::Assignment(assignment) => Some(assignment.clone()),
            HirStatement::Loop(_) => None,
        })
        .expect("assignment statement");
    invalid_assignment.target = VariableId::from(variable_count);

    let mut nested_invalid_assignment = invalid_assignment.clone();
    nested_invalid_assignment.target = VariableId::from(variable_count + 1);
    let nested_loop = HirStatement::Loop(HirLoop {
        condition: nested_invalid_assignment.expr.clone(),
        body: vec![HirStatement::Assignment(nested_invalid_assignment)],
        span: invalid_assignment.span,
    });
    hir.statements = vec![HirStatement::Assignment(invalid_assignment), nested_loop];

    let diagnostics = hir.validate().expect_err("malformed HIR must fail");
    let messages: Vec<_> = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.message.as_str())
        .collect();

    assert!(
        messages
            .iter()
            .any(|message| message.contains("array 'xs' base"))
    );
    assert!(
        messages
            .iter()
            .any(|message| message.contains("contribution branch name must not be empty"))
    );
    assert!(messages.iter().any(|message| {
        message.contains(&format!("assignment target VariableId({variable_count})"))
    }));
    assert!(messages.iter().any(|message| message.contains(&format!(
        "assignment target VariableId({})",
        variable_count + 1
    ))));
}
