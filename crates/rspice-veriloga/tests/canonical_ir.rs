use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;

use rspice_veriloga::ast::{
    AnalogOperator, BranchAccess, Expression, LaplaceKind, NumberLit, PortDirection,
};
use rspice_veriloga::canonical_ir::{
    BranchId, ContributionId, EquationId, ExprId, ModuleId, NodeId, ParamId, PortId, SourceId,
    StateId, VariableId,
};
use rspice_veriloga::canonical_ir::{
    CanonicalMetadata, CompilerPhase, DiagnosticSeverity, HirContributionKind, HirExprKind,
    HirLaplaceKind, HirLoop, HirModel, HirStatement, IrDiagnostic, MirAnalysisDomain,
    MirEquationKind, MirModel, MirStateSlot, SourceSpanRef, StableDigest,
};
use rspice_veriloga::semantic::{AnalyzedContribution, AnalyzedModule, AnalyzedPort, SymbolTable};
use rspice_veriloga::source::Span;
use rspice_veriloga::types::ValueType;
use rspice_veriloga::{Lexer, Parser, SemanticAnalyzer, SourceMap};

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

fn lower_tiny_resistor_mir() -> MirModel {
    let analyzed = analyze_fixture(tiny_resistor_source(), "tiny_res").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", tiny_resistor_source());
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
    assert_eq!(mir.equations[0].branch.pos_node, NodeId::new(0));
    assert_eq!(mir.equations[0].branch.neg_node, Some(NodeId::new(1)));
    assert!(
        mir.equations[0]
            .active_domains
            .contains(&MirAnalysisDomain::Dc)
    );
    assert!(mir.validate().is_ok());
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
    assert_eq!(mir.equations[0].branch.pos_node, NodeId::new(0));
    assert_eq!(mir.equations[0].branch.neg_node, Some(NodeId::new(1)));
}

#[test]
fn mir_validation_rejects_branch_participation_out_of_range() {
    let mut mir = lower_tiny_resistor_mir();
    mir.equations[0].branch.pos_node = NodeId::new(99);

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
