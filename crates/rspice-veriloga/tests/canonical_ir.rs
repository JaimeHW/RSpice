use serde::{Deserialize, Serialize};
use std::hash::Hash;

use rspice_veriloga::canonical_ir::{
    BranchId, ContributionId, ModuleId, NodeId, ParamId, PortId, SourceId, VariableId,
};
use rspice_veriloga::canonical_ir::{
    CanonicalMetadata, CompilerPhase, DiagnosticSeverity, HirContributionKind, HirModel,
    HirStatement, IrDiagnostic, SourceSpanRef, StableDigest,
};
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
fn hir_lowering_preserves_dynamic_array_assignment_target_and_index() {
    let analyzed = analyze_fixture(dynamic_array_source(), "dyn_array").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", dynamic_array_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);
    let array = analyzed.arrays.get("xs").expect("array layout");

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
}
