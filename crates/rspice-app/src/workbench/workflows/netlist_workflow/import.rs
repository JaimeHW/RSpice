//! Applying an accepted import to the project.
//!
//! The transaction either installs the whole canonical document — source,
//! descriptor, dependency closure — or leaves the project untouched.

use super::*;

#[cfg(test)]
pub(crate) fn apply_imported_netlist(
    state: &mut AppState,
    source: String,
    source_path: Option<std::path::PathBuf>,
    display_name: &str,
) -> bool {
    let metadata = NetlistImportMetadata {
        encoding: crate::state::NetlistTextEncoding::Utf8,
        line_ending: crate::state::NetlistLineEnding::detect(&source),
        dialect: crate::state::NetlistSourceDialect::RSpice,
        compatibility_reviewed: false,
        raw_sha256: sha256(source.as_bytes()),
    };
    apply_imported_netlist_transaction(
        state,
        source,
        source_path,
        display_name,
        metadata,
        Vec::new(),
    )
}

#[derive(Debug, Clone, Copy)]
pub(super) struct NetlistImportMetadata {
    pub(super) encoding: crate::state::NetlistTextEncoding,
    pub(super) line_ending: crate::state::NetlistLineEnding,
    pub(super) dialect: crate::state::NetlistSourceDialect,
    pub(super) compatibility_reviewed: bool,
    pub(super) raw_sha256: [u8; 32],
}

/// Why a deck was not installed.
///
/// Returned rather than reported, because the caller decides which state
/// hears about it: opening a netlist as a project installs the deck into a
/// candidate it discards on failure, so a reason reported there would be
/// discarded with it.
#[derive(Debug)]
enum ImportFailure {
    /// The gesture was refused before anything was read.
    Refused(&'static str),
    /// The deck could not be installed.
    Failed(String),
}

/// Import a SPICE deck into the open project, reporting the outcome.
pub(super) fn apply_imported_netlist_transaction(
    state: &mut AppState,
    source: String,
    source_path: Option<std::path::PathBuf>,
    display_name: &str,
    metadata: NetlistImportMetadata,
    dependencies: Vec<crate::state::DependencyMetadata>,
) -> bool {
    match install_imported_netlist(
        state,
        source,
        source_path,
        display_name,
        false,
        metadata,
        dependencies,
    ) {
        Ok(()) => {
            state.record(
                LogSeverity::Info,
                LogSource::Netlist,
                format!("Imported SPICE deck {display_name}."),
            );
            true
        }
        Err(ImportFailure::Refused(refusal)) => {
            state.prompt(refusal);
            false
        }
        Err(ImportFailure::Failed(error)) => {
            record_import_failure(state, &error);
            false
        }
    }
}

fn install_imported_netlist(
    state: &mut AppState,
    source: String,
    source_path: Option<std::path::PathBuf>,
    display_name: &str,
    initializing_netlist_project: bool,
    metadata: NetlistImportMetadata,
    dependencies: Vec<crate::state::DependencyMetadata>,
) -> Result<(), ImportFailure> {
    if source.trim().is_empty() {
        return Err(ImportFailure::Failed(format!("{display_name} is empty")));
    }

    if !state.project_lifecycle.is_open() {
        return Err(ImportFailure::Refused(
            "SPICE deck import needs an open project.",
        ));
    }
    if state.workbench.safe_mode.project_read_only() && !initializing_netlist_project {
        return Err(ImportFailure::Refused(
            "SPICE deck import is unavailable: the project is open read-only.",
        ));
    }
    if state.simulation.execution.has_active_execution() {
        return Err(ImportFailure::Refused(
            "SPICE deck import is unavailable while a simulation is running.",
        ));
    }

    let (document, mut descriptor) = canonical_import_document(
        state,
        &source,
        source_path.as_deref(),
        display_name,
        metadata,
        dependencies,
    )
    .map_err(ImportFailure::Failed)?;
    let mut candidate = state.clone();
    if !initializing_netlist_project && candidate.workspace.content.netlist_document.is_some() {
        crate::workbench::documents::netlist_document::retain_active_top_deck(&mut candidate)
            .map_err(ImportFailure::Failed)?;
    }
    descriptor.artifact_name =
        crate::workbench::documents::netlist_document::available_top_deck_path(
            &candidate,
            &descriptor.artifact_name,
        );
    // Importing a new source deck changes future execution authority but does
    // not delete immutable datasets or a previously active top deck.
    candidate.design_execution_epoch = candidate.design_execution_epoch.wrapping_add(1);
    candidate.ui.netlist = Default::default();
    candidate.workbench.netlist_open_documents.clear();
    let source_digest = crate::state::content_digest(&source);
    let reopenable_import = source_path.is_some();
    candidate.workspace.content.netlist_source = Some(source.clone());
    candidate.workspace.content.netlist_document = Some(document.clone());
    candidate.workspace.content.netlist_descriptor = Some(descriptor);
    candidate.workspace.content.netlist_source_path = source_path;
    candidate.workspace.content.set_netlist_source_dirty(true);
    candidate.simulation.source.netlist_content = source;
    candidate.ui.netlist.owned_document = Some(document);
    candidate.ui.netlist.externally_saved_content_digest =
        reopenable_import.then_some(source_digest);
    candidate.ui.netlist.active_document =
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource;
    candidate.ui.netlist.active_document_initialized = true;
    candidate.ui.netlist.revision = candidate.ui.netlist.revision.wrapping_add(1);
    crate::workbench::documents::netlist_document::invalidate_source_evidence(
        &mut candidate.ui.netlist,
    );
    candidate
        .workbench
        .activate(crate::workbench::state::Workspace::Netlist);
    candidate
        .workspace
        .content
        .validate_simulation_configuration()
        .map_err(|error| ImportFailure::Failed(error.to_string()))?;
    *state = candidate;
    Ok(())
}

pub(super) fn record_import_failure(state: &mut AppState, error: &str) {
    state.record(
        LogSeverity::Error,
        LogSource::Netlist,
        format!("SPICE deck import failed: {error}"),
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum NetlistImportMode {
    OpenProject,
    ImportIntoProject,
}

impl NetlistImportMode {
    pub(super) const fn dialog_title(self) -> &'static str {
        match self {
            Self::OpenProject => "Open Netlist Project",
            Self::ImportIntoProject => "Import SPICE Deck",
        }
    }
}

pub(super) fn netlist_import_start_block_reason(
    state: &AppState,
    mode: NetlistImportMode,
) -> Option<&'static str> {
    if state.simulation.execution.has_active_execution() {
        return Some("a simulation execution still owns the project");
    }
    if mode == NetlistImportMode::ImportIntoProject && !state.project_lifecycle.is_open() {
        return Some("no project is open");
    }
    if mode == NetlistImportMode::ImportIntoProject && state.workbench.safe_mode.project_read_only()
    {
        return Some("the project is open read-only");
    }
    None
}

pub(super) fn apply_opened_netlist_project(
    state: &mut AppState,
    source: String,
    source_path: Option<std::path::PathBuf>,
    display_name: &str,
    metadata: NetlistImportMetadata,
    dependencies: Vec<crate::state::DependencyMetadata>,
) -> bool {
    if source.trim().is_empty() {
        state.record(
            LogSeverity::Error,
            LogSource::Netlist,
            format!("Netlist project open failed: {display_name} is empty"),
        );
        return false;
    }
    if state.simulation.execution.has_active_execution() {
        state.prompt("A netlist project cannot be opened while a simulation is running.");
        return false;
    }

    // Construct and validate the replacement off to the side. The currently
    // open project, its dirty documents and all retained evidence remain
    // untouched unless the complete netlist-first project is ready to commit.
    // The candidate states the opened netlist when it is created, so the line
    // reaches the Console only if the candidate is committed.
    let mut candidate = state.clone();
    crate::workbench::workflows::project_workflow::create_netlist_project(
        &mut candidate,
        display_name,
    );
    if let Err(failure) = install_imported_netlist(
        &mut candidate,
        source,
        source_path,
        display_name,
        true,
        metadata,
        dependencies,
    ) {
        let reason = match failure {
            ImportFailure::Refused(refusal) => refusal.trim_end_matches('.').to_owned(),
            ImportFailure::Failed(error) => error,
        };
        state.record(
            LogSeverity::Error,
            LogSource::Netlist,
            format!(
                "Netlist project open failed: {reason}. The current project was left unchanged."
            ),
        );
        return false;
    }
    *state = candidate;
    true
}

pub(super) fn apply_netlist_import_result(
    state: &mut AppState,
    mode: NetlistImportMode,
    source: String,
    source_path: Option<std::path::PathBuf>,
    display_name: &str,
    metadata: NetlistImportMetadata,
    dependencies: Vec<crate::state::DependencyMetadata>,
) -> bool {
    match mode {
        NetlistImportMode::OpenProject => apply_opened_netlist_project(
            state,
            source,
            source_path,
            display_name,
            metadata,
            dependencies,
        ),
        NetlistImportMode::ImportIntoProject => apply_imported_netlist_transaction(
            state,
            source,
            source_path,
            display_name,
            metadata,
            dependencies,
        ),
    }
}

pub(super) fn canonical_import_document(
    _state: &AppState,
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

pub(super) fn sha256(bytes: &[u8]) -> [u8; 32] {
    use sha2::Digest as _;

    let digest = sha2::Sha256::digest(bytes);
    let mut value = [0_u8; 32];
    value.copy_from_slice(&digest);
    value
}
