//! Applying an accepted import to the project: its parts.
//!
//! The canonical document an import installs, why a deck cannot be
//! installed, and installing one into a candidate's parts. The shell's
//! netlist import transactions install the candidate whole or leave the
//! project untouched.

use super::*;

#[derive(Debug, Clone, Copy)]
pub struct NetlistImportMetadata {
    pub encoding: crate::state::NetlistTextEncoding,
    pub line_ending: crate::state::NetlistLineEnding,
    pub dialect: crate::state::NetlistSourceDialect,
    pub compatibility_reviewed: bool,
    pub raw_sha256: [u8; 32],
}

/// Why a deck was not installed.
///
/// Returned rather than reported, because the caller decides which state
/// hears about it: opening a netlist as a project installs the deck into a
/// candidate it discards on failure, so a reason reported there would be
/// discarded with it.
#[derive(Debug)]
pub enum ImportFailure {
    /// The gesture was refused before anything was read.
    Refused(&'static str),
    /// The deck could not be installed.
    Failed(String),
}

/// Why a deck cannot be installed, read before anything is: an empty deck,
/// no open project, a read-only project (unless the deck initializes a new
/// netlist project) or a running simulation. It reads the model and whether
/// a project is open.
pub fn imported_deck_refusal(
    model: &Model,
    project_open: bool,
    source: &str,
    display_name: &str,
    initializing_netlist_project: bool,
) -> Result<(), ImportFailure> {
    if source.trim().is_empty() {
        return Err(ImportFailure::Failed(format!("{display_name} is empty")));
    }

    if !project_open {
        return Err(ImportFailure::Refused(
            "SPICE deck import needs an open project.",
        ));
    }
    if model.safe_mode.project_read_only() && !initializing_netlist_project {
        return Err(ImportFailure::Refused(
            "SPICE deck import is unavailable: the project is open read-only.",
        ));
    }
    if model.simulation.execution.has_active_execution() {
        return Err(ImportFailure::Refused(
            "SPICE deck import is unavailable while a simulation is running.",
        ));
    }
    Ok(())
}

/// Install an imported deck's canonical document and descriptor into the
/// parts of a candidate project: the model, the netlist editor's state, the
/// include tabs and the runs. The previous top deck is retained unless the
/// deck initializes a new netlist project, the editor starts over on the
/// owned source, and the result must validate; the shell installs the
/// candidate only if this succeeds.
pub fn install_imported_deck(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut std::collections::HashSet<crate::workbench::state::WorkspaceDocumentId>,
    runs: &mut rspice_app_runs::workbench::slots::runs::RunsState,
    document: crate::state::NetlistDocument,
    mut descriptor: crate::state::OwnedNetlistDescriptor,
    source: String,
    source_path: Option<std::path::PathBuf>,
    initializing_netlist_project: bool,
) -> Result<(), ImportFailure> {
    if !initializing_netlist_project && model.workspace.content.netlist_document.is_some() {
        crate::workbench::documents::netlist_document::retain_active_top_deck(model)
            .map_err(ImportFailure::Failed)?;
    }
    descriptor.artifact_name = crate::workbench::documents::netlist_document::unique_top_deck_path(
        model,
        &descriptor.artifact_name,
    );
    // Importing a new source deck changes future execution authority but does
    // not delete immutable datasets or a previously active top deck.
    model.design_execution_epoch = model.design_execution_epoch.wrapping_add(1);
    *netlist = Default::default();
    *model.netlist_artifacts = Default::default();
    *runs.manual_run = Default::default();
    open_documents.clear();
    let source_digest = crate::state::content_digest(&source);
    let reopenable_import = source_path.is_some();
    model.workspace.content.netlist_source = Some(source.clone());
    model.workspace.content.netlist_document = Some(document.clone());
    model.workspace.content.netlist_descriptor = Some(descriptor);
    model.workspace.content.netlist_source_path = source_path;
    model.workspace.content.set_netlist_source_dirty(true);
    model.simulation.source.netlist_content = source;
    netlist.owned_document = Some(document);
    netlist.externally_saved_content_digest = reopenable_import.then_some(source_digest);
    netlist.active_document =
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource;
    netlist.active_document_initialized = true;
    netlist.revision = netlist.revision.wrapping_add(1);
    model.invalidate_source_evidence();
    model
        .workspace
        .content
        .validate_simulation_configuration()
        .map_err(|error| ImportFailure::Failed(error.to_string()))?;
    Ok(())
}

pub fn record_import_failure(model: &mut Model, error: &str) {
    model.record(
        LogSeverity::Error,
        LogSource::Netlist,
        format!("SPICE deck import failed: {error}"),
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetlistImportMode {
    OpenProject,
    ImportIntoProject,
}

impl NetlistImportMode {
    pub const fn dialog_title(self) -> &'static str {
        match self {
            Self::OpenProject => "Open Netlist Project",
            Self::ImportIntoProject => "Import SPICE Deck",
        }
    }
}

/// Why an import in `mode` cannot start, over the model and whether a
/// project is open.
pub fn netlist_import_start_block_reason(
    model: &Model,
    project_open: bool,
    mode: NetlistImportMode,
) -> Option<&'static str> {
    if model.simulation.execution.has_active_execution() {
        return Some("a simulation execution still owns the project");
    }
    if mode == NetlistImportMode::ImportIntoProject && !project_open {
        return Some("no project is open");
    }
    if mode == NetlistImportMode::ImportIntoProject && model.safe_mode.project_read_only() {
        return Some("the project is open read-only");
    }
    None
}

pub fn canonical_import_document(
    source: &str,
    source_path: Option<&std::path::Path>,
    display_name: &str,
    metadata: NetlistImportMetadata,
    dependencies: Vec<crate::state::DependencyMetadata>,
) -> Result<
    (
        crate::state::NetlistDocument,
        crate::state::OwnedNetlistDescriptor,
    ),
    String,
> {
    use crate::state::{NetlistDocument, NetlistDocumentId, SourceLocator};

    let artifact_name = source_path
        .and_then(std::path::Path::file_name)
        .map(|name| name.to_string_lossy().into_owned())
        .or_else(|| {
            let candidate = display_name.trim();
            (!candidate.is_empty()
                && !candidate.chars().any(char::is_control)
                && !candidate.contains('/')
                && !candidate.contains('\\'))
            .then(|| candidate.to_owned())
        })
        .unwrap_or_else(|| "imported.sp".to_owned());
    let logical_identity = source_path.map_or_else(
        || format!("browser-import/{artifact_name}"),
        |path| path.display().to_string(),
    );
    let mut locator = SourceLocator::try_new(logical_identity, artifact_name.clone())
        .map_err(|error| error.to_string())?;
    if let Some(path) = source_path {
        locator = locator
            .with_native_origin(path.display().to_string())
            .map_err(|error| error.to_string())?;
    }
    let mut document = NetlistDocument::from_imported_source(
        NetlistDocumentId::new(),
        locator,
        source.as_bytes().to_vec(),
        dependencies,
    )
    .map_err(|error| error.to_string())?;
    document
        .make_editable(document.content_digest())
        .map_err(|error| error.to_string())?;

    let mut descriptor = crate::state::OwnedNetlistDescriptor {
        deck_id: uuid::Uuid::new_v4(),
        artifact_name,
        strategy: crate::state::OwnedNetlistEditStrategy::OwnedSource,
        source_encoding: metadata.encoding,
        source_line_ending: metadata.line_ending,
        imported_dialect: Some(metadata.dialect),
        compatibility_reviewed: metadata.compatibility_reviewed,
        execution_profile: metadata.dialect.execution_profile(),
        external_file_sha256: source_path.map(|_| metadata.raw_sha256),
        save_history: Vec::new(),
        revision_history: Vec::new(),
        owned_includes: Vec::new(),
    };
    descriptor.retain_revision(&document, "Imported source baseline")?;
    Ok((document, descriptor))
}

pub fn sha256(bytes: &[u8]) -> [u8; 32] {
    use sha2::Digest as _;

    let digest = sha2::Sha256::digest(bytes);
    let mut value = [0_u8; 32];
    value.copy_from_slice(&digest);
    value
}
