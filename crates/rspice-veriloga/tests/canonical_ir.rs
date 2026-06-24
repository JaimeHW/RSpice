use serde::{Deserialize, Serialize};
use std::hash::Hash;

use rspice_veriloga::canonical_ir::{
    CanonicalMetadata, CompilerPhase, DiagnosticSeverity, HirContributionKind, HirModel,
    IrDiagnostic, SourceSpanRef, StableDigest,
};
use rspice_veriloga::canonical_ir::{ModuleId, ParamId, PortId, SourceId};
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
