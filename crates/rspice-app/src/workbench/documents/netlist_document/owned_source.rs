//! Opening the project-owned source.
//!
//! The owned source is created from the generated primary the first time it
//! is opened, and opening an existing one never rewrites its bytes.

// Through the parent rather than `crate::workbench`, the same route
// `comparison` takes.
use super::{ActiveNetlistDocument, Model, NetlistDocumentState};
use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::NetlistDocumentId;

/// Open (or explicitly create) the project-owned source derived from the
/// immutable generated primary. Existing owned bytes are never overwritten.
/// It works over the model and the netlist editor's state.
pub(crate) fn open_owned_source_in(model: &mut Model, netlist: &mut NetlistDocumentState) -> bool {
    if model.workspace.content.netlist_source.is_none() {
        return false;
    }
    if netlist.owned_document.is_none() {
        let Some(generated) = model.netlist_artifacts.generated_document.as_ref() else {
            return false;
        };
        let source = model
            .workspace
            .content
            .netlist_source
            .as_deref()
            .unwrap_or_else(|| generated.source());
        let mut owned = match generated
            .create_editable_copy(NetlistDocumentId::new(), generated.content_digest())
        {
            Ok(document) => document,
            Err(error) => {
                model.record(
                    LogSeverity::Error,
                    LogSource::Studio,
                    format!("The editable SPICE copy could not be created: {error}"),
                );
                return false;
            }
        };
        let transition = if let Some(path) = model.workspace.content.netlist_source_path.as_ref() {
            let display_name = path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| path.display().to_string());
            crate::state::SourceLocator::try_new(path.display().to_string(), display_name)
                .and_then(|locator| locator.with_native_origin(path.display().to_string()))
                .and_then(|locator| {
                    owned.import_source(owned.content_digest(), locator, source.as_bytes().to_vec())
                })
                .and_then(|_| owned.make_editable(owned.content_digest()))
        } else {
            owned.replace_editable_source(owned.content_digest(), source.as_bytes().to_vec())
        };
        if let Err(error) = transition {
            model.record(
                LogSeverity::Error,
                LogSource::Studio,
                format!("The editable SPICE copy could not be created: {error}"),
            );
            return false;
        }
        netlist.owned_document = Some(owned);
    }
    let Some(source) = netlist
        .owned_document
        .as_ref()
        .map(|document| document.source().to_owned())
    else {
        return false;
    };
    model.workspace.content.netlist_source = Some(source.clone());
    model.workspace.content.netlist_document = netlist.owned_document.clone();
    if model.workspace.content.netlist_descriptor.is_none() {
        let artifact_name = model
            .workspace
            .content
            .netlist_source_path
            .as_deref()
            .and_then(std::path::Path::file_name)
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| "top_override.sp".to_owned());
        model.workspace.content.netlist_descriptor = Some(crate::state::OwnedNetlistDescriptor {
            deck_id: uuid::Uuid::new_v4(),
            artifact_name,
            strategy: crate::state::OwnedNetlistEditStrategy::OwnedSource,
            source_encoding: crate::state::NetlistTextEncoding::Utf8,
            source_line_ending: crate::state::NetlistLineEnding::detect(&source),
            imported_dialect: None,
            compatibility_reviewed: false,
            execution_profile: Some(crate::state::NetlistExecutionProfile::RSpiceCanonicalV1),
            external_file_sha256: None,
            save_history: Vec::new(),
            revision_history: Vec::new(),
            owned_includes: Vec::new(),
        });
    }
    netlist.active_document = ActiveNetlistDocument::OwnedSource;
    netlist.active_dependency_identity = None;
    netlist.active_dependency_root = None;
    netlist.active_document_initialized = true;
    model.simulation.source.netlist_content = source;
    netlist.revision = netlist.revision.wrapping_add(1);
    netlist.completion_open = false;
    netlist.completion_dismissed_at = None;
    model.invalidate_source_evidence();
    true
}
