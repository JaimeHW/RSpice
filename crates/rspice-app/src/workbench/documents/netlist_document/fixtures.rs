//! What the netlist document's cases seed: a generated deck that includes
//! one dependency, copied into the project's own root source and
//! validated. The document's own cases and the code feature's
//! whole-application tests share it.

use super::{ActiveNetlistDocument, Model, NetlistDocumentState};
use crate::product::ObjectRevision;
use crate::state::{
    DependencyMetadata, GeneratedArtifact, GeneratedProvenance, GenerationInput, NetlistDocument,
    NetlistDocumentId, SourceLocator,
};

pub(crate) const ROOT: &str = "owned dependency fixture\n.include \"models/resistor.inc\"\nV1 out 0 1\nR1 out 0 rmodel\n.op\n.end\n";

pub(crate) const ORIGINAL_INCLUDE: &str = ".param rmodel=1k\n";

pub(crate) const INCLUDE_IDENTITY: &str = "models/resistor.inc";

pub(crate) const INCLUDE_NATIVE_ORIGIN: &str = "C:/project/models/resistor.inc";

/// The owned-dependency fixture seeded into a session's model and netlist
/// editor: a generated deck that includes one dependency, copied into the
/// project's own root source, which the editor shows, validated.
pub fn seed_owned_dependency(model: &mut Model, netlist: &mut NetlistDocumentState) {
    let locator = SourceLocator::try_new(INCLUDE_IDENTITY, "resistor.inc")
        .unwrap()
        .with_native_origin(INCLUDE_NATIVE_ORIGIN)
        .unwrap();
    let dependency = DependencyMetadata::unresolved_direct_to(0, INCLUDE_IDENTITY, locator)
        .unwrap()
        .resolve_utf8(ORIGINAL_INCLUDE.as_bytes().to_vec())
        .unwrap();
    let provenance = GeneratedProvenance::try_new(
        "rspice-owned-include-test",
        GenerationInput::new(
            ObjectRevision::INITIAL,
            crate::state::content_digest("owned-include-input"),
        ),
    )
    .unwrap();
    let artifact = GeneratedArtifact::try_from_utf8(
        provenance,
        ROOT.as_bytes().to_vec(),
        vec![dependency],
        Vec::new(),
    )
    .unwrap();
    let generated = NetlistDocument::from_generated(NetlistDocumentId::new(), artifact).unwrap();
    let owned = generated
        .create_editable_copy(NetlistDocumentId::new(), generated.content_digest())
        .unwrap();

    model.workspace.content.netlist_source = Some(ROOT.to_owned());
    model.workspace.content.netlist_document = Some(owned.clone());
    model.workspace.content.netlist_descriptor = Some(crate::state::OwnedNetlistDescriptor {
        deck_id: uuid::Uuid::new_v4(),
        artifact_name: "owned.cir".to_owned(),
        strategy: crate::state::OwnedNetlistEditStrategy::OwnedSource,
        source_encoding: crate::state::NetlistTextEncoding::Utf8,
        source_line_ending: crate::state::NetlistLineEnding::Lf,
        imported_dialect: Some(crate::state::NetlistSourceDialect::RSpice),
        compatibility_reviewed: true,
        execution_profile: Some(crate::state::NetlistExecutionProfile::RSpiceCanonicalV1),
        external_file_sha256: None,
        save_history: Vec::new(),
        revision_history: Vec::new(),
        owned_includes: Vec::new(),
    });
    model.netlist_artifacts.generated_source = ROOT.to_owned();
    model.netlist_artifacts.generated_document = Some(generated);
    netlist.owned_document = Some(owned);
    netlist.active_document = ActiveNetlistDocument::OwnedSource;
    netlist.active_document_initialized = true;
    model.simulation.source.netlist_content = ROOT.to_owned();
    model
        .workspace
        .content
        .validate_simulation_configuration()
        .unwrap();
}
