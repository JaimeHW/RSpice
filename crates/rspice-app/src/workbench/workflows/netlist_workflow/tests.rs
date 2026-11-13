//! Netlist workflow tests.

use super::bundle::*;
#[cfg(not(target_arch = "wasm32"))]
use super::external_change::*;
use super::platform::*;
use super::staging::*;
use super::*;
use rspice_simulation::netlist_preparation::owned_source::compose_owned_netlist_execution_source;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod vendor_profiles;

#[test]
fn dialect_detection_uses_active_body_records_and_export_headers() {
    use crate::state::NetlistSourceDialect;
    for title in [".control", ".alter", ".probe", ".end"] {
        let source = format!("{title}\nV1 out 0 1\n.op\n.end; done\n.control\n");
        assert_eq!(
            super::staging::detect_netlist_dialect(&source).0,
            NetlistSourceDialect::RSpice
        );
    }
    assert_eq!(
        super::staging::detect_netlist_dialect("deck\n.control; script\nop\n.endc; done\n.end\n").0,
        NetlistSourceDialect::Spice3Ngspice
    );
    assert_eq!(
        super::staging::detect_netlist_dialect("simulator lang=spice; exported\n.op\n.end\n").0,
        NetlistSourceDialect::Spectre
    );
}

#[test]
fn netlist_import_and_conflict_resolution_have_no_panic_shortcuts() {
    for source in [
        include_str!("staging.rs"),
        include_str!("external_change.rs"),
    ] {
        let production = crate::source_guard::production_source(source);
        for forbidden in [".expect(", ".unwrap(", "panic!(", "unreachable!("] {
            assert!(
                !production.contains(forbidden),
                "netlist workflow production code contains panic shortcut {forbidden}"
            );
        }
    }
}

const GENERATED_BASE: &str = "* generated\n.option reltol=1e-3\n.param gain=10\n.include \"models/a.lib\"\n.lib \"models/b.lib\" TT\n+ section=fast\nV1 out 0 1\nR1 out 0 1k\n.op\n.measure op vout FIND V(out)\n.save V(out)\n.end\n";

#[test]
fn started_netlist_import_routes_every_calling_page_to_its_completion_owner() {
    let mut state = AppState::default();
    state.workbench.workspace = crate::workbench::state::Workspace::Models;
    state.code.code_workspace.page =
        crate::workbench::documents::code_workspace::CodeWorkspacePage::Automation;

    assert!(!route_started_netlist_import(
        &mut state.workbench,
        &mut state.code.code_workspace.page,
        false
    ));
    assert_eq!(
        state.workbench.workspace,
        crate::workbench::state::Workspace::Models
    );
    assert_eq!(
        state.code.code_workspace.page,
        crate::workbench::documents::code_workspace::CodeWorkspacePage::Automation
    );

    assert!(route_started_netlist_import(
        &mut state.workbench,
        &mut state.code.code_workspace.page,
        true
    ));
    assert_eq!(
        state.workbench.workspace,
        crate::workbench::state::Workspace::Netlist
    );
    assert_eq!(
        state.code.code_workspace.page,
        crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist
    );
}

#[test]
fn import_decoder_and_encoder_preserve_supported_file_boundaries() {
    let source = "* caf\u{00e9}\r\nV1 out 0 1\r\n.op\r\n.end\r\n";
    let utf16 = crate::state::NetlistTextEncoding::Utf16LeBom
        .encode(source)
        .expect("UTF-16 encoding succeeds");
    let (decoded, encoding) = decode_import_bytes(&utf16).expect("UTF-16 import decodes");

    assert_eq!(decoded, source);
    assert_eq!(encoding, crate::state::NetlistTextEncoding::Utf16LeBom);
    assert_eq!(
        crate::state::NetlistLineEnding::detect(&decoded),
        crate::state::NetlistLineEnding::Crlf
    );
    assert_eq!(encoding.encode(&decoded).unwrap(), utf16);

    let latin1 = crate::state::NetlistTextEncoding::Latin1
        .encode(source)
        .expect("Latin-1 fixture is representable");
    let (decoded, encoding) = decode_import_bytes(&latin1).expect("Latin-1 import decodes");
    assert_eq!(encoding, crate::state::NetlistTextEncoding::Latin1);
    assert_eq!(decoded, source);
    assert_eq!(encoding.encode(&decoded).unwrap(), latin1);
    assert!(encoding.encode("\u{20ac}").is_err());
}

pub(crate) fn generated_bundle_fixture() -> Vec<u8> {
    use crate::product::{ContentDigest, ObjectRevision};
    use crate::state::{
        DependencyMetadata, GeneratedArtifact, GeneratedProvenance, GenerationInput, SourceLocator,
    };

    let root = "bundle fixture\n.include \"models/base.lib\"\nV1 out 0 1\n.op\n.end\n";
    let base = SourceLocator::try_new("models/base.lib", "base.lib").unwrap();
    let child = SourceLocator::try_new("models/devices/core.lib", "core.lib").unwrap();
    let dependencies = vec![
        DependencyMetadata::unresolved_direct_to(0, "models/base.lib", base.clone())
            .unwrap()
            .with_authority(crate::state::DependencySourceAuthority::Vendor)
            .resolve_utf8(b".include \"devices/core.lib\"\n.model base nmos level=1\n".to_vec())
            .unwrap(),
        DependencyMetadata::unresolved_transitive_to(base, 0, "devices/core.lib", child)
            .unwrap()
            .with_authority(crate::state::DependencySourceAuthority::TechnologyPackage)
            .resolve_utf8(b".model core nmos level=1 vto=0.45\n".to_vec())
            .unwrap(),
    ];
    let artifact = GeneratedArtifact::try_from_utf8(
        GeneratedProvenance::try_new(
            "netlist-bundle-import-test",
            GenerationInput::new(
                ObjectRevision::INITIAL,
                ContentDigest::from_bytes([0x5a; 32]),
            ),
        )
        .unwrap(),
        root.as_bytes().to_vec(),
        dependencies,
        Vec::new(),
    )
    .unwrap();
    // SPICE is already the canonical dialect, so the fixture renders each
    // document unchanged and the bundle under test is exactly what the
    // generator produced.
    crate::io::build_generated_bundle(
        &artifact,
        crate::io::NetlistFormat::Spice,
        true,
        str::to_owned,
    )
    .unwrap()
}

#[test]
fn generated_bundle_import_rejects_tampered_member_bytes() {
    let mut bytes = generated_bundle_fixture();
    let needle = b"vto=0.45";
    let offset = bytes
        .windows(needle.len())
        .position(|window| window == needle)
        .expect("fixture contains retained model bytes");
    bytes[offset] ^= 0x01;

    let error = parse_generated_netlist_bundle(&bytes).expect_err("tamper must fail closed");
    assert!(error.contains("CRC-32 verification"), "{error}");
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn three_way_merge_combines_independent_lines_and_marks_overlaps() {
    let base = ".param a=1\n.param b=2\n.op\n";
    let local = ".param a=10\n.param b=2\n.op\n";
    let external = ".param a=1\n.param b=20\n.op\n";
    let (merged, conflicts) = three_way_merge_source(Some(base), local, external);
    assert_eq!(conflicts, 0);
    assert_eq!(merged, ".param a=10\n.param b=20\n.op\n");

    let external = ".param a=100\n.param b=2\n.op\n";
    let (merged, conflicts) = three_way_merge_source(Some(base), local, external);
    assert_eq!(conflicts, 1);
    assert!(merged.contains("<<<<<<< RSPICE LOCAL"));
    assert!(merged.contains(".param a=10"));
    assert!(merged.contains(".param a=100"));

    let base = "a\nb\nc\n";
    let local = "a\ninserted locally\nb\nc\n";
    let external = "a\nb\nchanged externally\n";
    let (merged, conflicts) = three_way_merge_source(Some(base), local, external);
    assert_eq!(conflicts, 0);
    assert_eq!(merged, "a\ninserted locally\nb\nchanged externally\n");
}

pub(crate) fn state_with_owned_strategy(
    generated_source: &str,
    authored_source: &str,
    strategy: crate::state::OwnedNetlistEditStrategy,
) -> AppState {
    use crate::product::ObjectRevision;
    use crate::state::{
        GeneratedArtifact, GeneratedProvenance, GenerationInput, NetlistDocument,
        NetlistDocumentId, content_digest,
    };

    let provenance = GeneratedProvenance::try_new(
        "rspice-netlist-workflow-test",
        GenerationInput::new(ObjectRevision::INITIAL, content_digest("test-inputs")),
    )
    .expect("valid generated provenance");
    let artifact = GeneratedArtifact::try_from_utf8(
        provenance,
        generated_source.as_bytes().to_vec(),
        Vec::new(),
        Vec::new(),
    )
    .expect("valid generated artifact");
    let generated = NetlistDocument::from_generated(NetlistDocumentId::new(), artifact)
        .expect("valid generated document");
    let mut owned = generated
        .create_editable_copy(NetlistDocumentId::new(), generated.content_digest())
        .expect("editable copy");
    owned
        .replace_editable_source(owned.content_digest(), authored_source.as_bytes().to_vec())
        .expect("authored source");

    let mut state = AppState::default();
    state.model.workspace.content.netlist_source = Some(authored_source.to_owned());
    state.model.workspace.content.netlist_document = Some(owned);
    state.model.workspace.content.netlist_descriptor = Some(crate::state::OwnedNetlistDescriptor {
        deck_id: uuid::Uuid::new_v4(),
        artifact_name: "owned.cir".to_owned(),
        strategy,
        source_encoding: crate::state::NetlistTextEncoding::Utf8,
        source_line_ending: crate::state::NetlistLineEnding::detect(authored_source),
        imported_dialect: None,
        compatibility_reviewed: false,
        execution_profile: Some(crate::state::NetlistExecutionProfile::RSpiceCanonicalV1),
        external_file_sha256: None,
        save_history: Vec::new(),
        revision_history: Vec::new(),
        owned_includes: Vec::new(),
    });
    state
}

#[test]
fn owned_source_strategy_executes_exact_authored_bytes_without_generated_composition() {
    let authored = "* independently owned\r\nV9 out 0 9\r\n.tran 1n 10n\r\n.end\r\n";
    let state = state_with_owned_strategy(
        GENERATED_BASE,
        authored,
        crate::state::OwnedNetlistEditStrategy::OwnedSource,
    );

    let composed = compose_owned_netlist_execution_source(
        state.model.workspace.content.netlist_descriptor.as_ref(),
        state
            .model
            .workspace
            .content
            .netlist_document
            .as_ref()
            .and_then(|document| document.generated_artifact()),
        authored,
    )
    .expect("owned source is executable");

    assert_eq!(composed.as_bytes(), authored.as_bytes());
}

#[test]
fn parameter_option_override_retains_base_and_appends_override_before_end() {
    let authored = "* project corner\n.param gain=22\n+ trim=0.5\n.options method=gear\n.temp 85\n";
    let state = state_with_owned_strategy(
        GENERATED_BASE,
        authored,
        crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride,
    );

    let composed = compose_owned_netlist_execution_source(
        state.model.workspace.content.netlist_descriptor.as_ref(),
        state
            .model
            .workspace
            .content
            .netlist_document
            .as_ref()
            .and_then(|document| document.generated_artifact()),
        authored,
    )
    .expect("parameter override is executable");

    assert_eq!(
        composed,
        "* generated\n.option reltol=1e-3\n.param gain=10\n.include \"models/a.lib\"\n.lib \"models/b.lib\" TT\n+ section=fast\nV1 out 0 1\nR1 out 0 1k\n.op\n.measure op vout FIND V(out)\n.save V(out)\n* project corner\n.param gain=22\n+ trim=0.5\n.options method=gear\n.temp 85\n.end\n"
    );
}

#[test]
fn include_order_override_replaces_all_base_include_cards_and_continuations() {
    let authored =
        ".lib \"models/b.lib\" SS\n.include \"models/a.lib\"\n.veriloga \"models/device.va\"\n";
    let state = state_with_owned_strategy(
        GENERATED_BASE,
        authored,
        crate::state::OwnedNetlistEditStrategy::IncludeOrderOverride,
    );

    let composed = compose_owned_netlist_execution_source(
        state.model.workspace.content.netlist_descriptor.as_ref(),
        state
            .model
            .workspace
            .content
            .netlist_document
            .as_ref()
            .and_then(|document| document.generated_artifact()),
        authored,
    )
    .expect("include-order override is executable");

    assert_eq!(
        composed,
        "* generated\n.option reltol=1e-3\n.param gain=10\nV1 out 0 1\nR1 out 0 1k\n.op\n.measure op vout FIND V(out)\n.save V(out)\n.lib \"models/b.lib\" SS\n.include \"models/a.lib\"\n.veriloga \"models/device.va\"\n.end"
    );
    assert!(!composed.contains("section=fast"));
}

#[test]
fn analysis_only_deck_replaces_base_analysis_measurement_and_output_cards() {
    let authored = ".tran 1n 10n\n.measure tran vmax MAX V(out)\n.probe V(out) I(V1)\n";
    let state = state_with_owned_strategy(
        GENERATED_BASE,
        authored,
        crate::state::OwnedNetlistEditStrategy::AnalysisOnlyDeck,
    );

    let composed = compose_owned_netlist_execution_source(
        state.model.workspace.content.netlist_descriptor.as_ref(),
        state
            .model
            .workspace
            .content
            .netlist_document
            .as_ref()
            .and_then(|document| document.generated_artifact()),
        authored,
    )
    .expect("analysis-only deck is executable");

    assert_eq!(
        composed,
        "* generated\n.option reltol=1e-3\n.param gain=10\n.include \"models/a.lib\"\n.lib \"models/b.lib\" TT\n+ section=fast\nV1 out 0 1\nR1 out 0 1k\n.tran 1n 10n\n.measure tran vmax MAX V(out)\n.probe V(out) I(V1)\n.end"
    );
    assert!(!composed.contains(".op\n"));
    assert!(!composed.contains(".save V(out)"));
}

#[test]
fn narrow_override_rejects_device_cards_and_cross_strategy_directives() {
    let device = "Roverride out 0 2k\n";
    let state = state_with_owned_strategy(
        GENERATED_BASE,
        device,
        crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride,
    );
    let error = compose_owned_netlist_execution_source(
        state.model.workspace.content.netlist_descriptor.as_ref(),
        state
            .model
            .workspace
            .content
            .netlist_document
            .as_ref()
            .and_then(|document| document.generated_artifact()),
        device,
    )
    .expect_err("device card must fail closed");
    assert!(error.contains("'roverride'"));
    assert!(error.contains("line 1"));

    let wrong_strategy = ".include \"models/other.lib\"\n";
    let state = state_with_owned_strategy(
        GENERATED_BASE,
        wrong_strategy,
        crate::state::OwnedNetlistEditStrategy::AnalysisOnlyDeck,
    );
    let error = compose_owned_netlist_execution_source(
        state.model.workspace.content.netlist_descriptor.as_ref(),
        state
            .model
            .workspace
            .content
            .netlist_document
            .as_ref()
            .and_then(|document| document.generated_artifact()),
        wrong_strategy,
    )
    .expect_err("cross-strategy directive must fail closed");
    assert!(error.contains("'.include'"));
    assert!(error.contains("line 1"));
}

#[test]
fn narrow_override_rejects_orphan_continuation() {
    let authored = "+ sweep=fast\n.param gain=22\n";
    let state = state_with_owned_strategy(
        GENERATED_BASE,
        authored,
        crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride,
    );

    let error = compose_owned_netlist_execution_source(
        state.model.workspace.content.netlist_descriptor.as_ref(),
        state
            .model
            .workspace
            .content
            .netlist_document
            .as_ref()
            .and_then(|document| document.generated_artifact()),
        authored,
    )
    .expect_err("orphan continuation must fail closed");

    assert_eq!(
        error,
        "Override line 1 is a continuation without an allowed owning card."
    );
}

#[test]
fn narrow_override_requires_retained_generated_base() {
    let authored = ".tran 1n 10n\n";
    let mut state = AppState::default();
    state.model.workspace.content.netlist_descriptor = Some(crate::state::OwnedNetlistDescriptor {
        deck_id: uuid::Uuid::new_v4(),
        artifact_name: "analysis.cir".to_owned(),
        strategy: crate::state::OwnedNetlistEditStrategy::AnalysisOnlyDeck,
        source_encoding: crate::state::NetlistTextEncoding::Utf8,
        source_line_ending: crate::state::NetlistLineEnding::Lf,
        imported_dialect: None,
        compatibility_reviewed: false,
        execution_profile: Some(crate::state::NetlistExecutionProfile::RSpiceCanonicalV1),
        external_file_sha256: None,
        save_history: Vec::new(),
        revision_history: Vec::new(),
        owned_includes: Vec::new(),
    });

    let error = compose_owned_netlist_execution_source(
        state.model.workspace.content.netlist_descriptor.as_ref(),
        state
            .model
            .workspace
            .content
            .netlist_document
            .as_ref()
            .and_then(|document| document.generated_artifact()),
        authored,
    )
    .expect_err("missing generated base must fail closed");

    assert_eq!(
        error,
        "Narrow override has no retained generated base artifact."
    );
}

#[test]
fn narrow_override_rejects_generated_base_without_end_terminator() {
    let authored = ".param gain=22\n";
    let state = state_with_owned_strategy(
        "* malformed generated base\nR1 out 0 1k\n.op\n",
        authored,
        crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride,
    );

    let error = compose_owned_netlist_execution_source(
        state.model.workspace.content.netlist_descriptor.as_ref(),
        state
            .model
            .workspace
            .content
            .netlist_document
            .as_ref()
            .and_then(|document| document.generated_artifact()),
        authored,
    )
    .expect_err("missing end terminator must fail closed");

    assert_eq!(error, "Retained generated base has no .end terminator.");
}

#[test]
fn deterministic_netlist_format_preserves_line_ending_policy() {
    assert_eq!(
        super::normalize_owned_netlist_whitespace("deck  \r\nR1 1 0 1k\t\r\n.end"),
        "deck  \r\nR1 1 0 1k\r\n.end\r\n"
    );
    assert_eq!(
        super::normalize_owned_netlist_whitespace("deck\n.end\n"),
        "deck\n.end\n"
    );
}
