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
        false,
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

pub(super) fn apply_imported_netlist_transaction(
    state: &mut AppState,
    source: String,
    source_path: Option<std::path::PathBuf>,
    display_name: &str,
    initializing_netlist_project: bool,
    metadata: NetlistImportMetadata,
    dependencies: Vec<crate::state::DependencyMetadata>,
) -> bool {
    if source.trim().is_empty() {
        state.push_user_message(ConsoleMessage::error(format!(
            "SPICE deck import failed: {display_name} is empty"
        )));
        return false;
    }

    if !state.project_lifecycle.project_open {
        state.push_user_message(ConsoleMessage::error(
            "SPICE deck import requires an open project",
        ));
        return false;
    }
    if state.workbench.safe_mode.project_read_only() && !initializing_netlist_project {
        state.push_user_message(ConsoleMessage::error(
            "SPICE deck import is unavailable because the project is open read-only",
        ));
        return false;
    }
    if state.simulation.has_active_execution() {
        state.push_user_message(ConsoleMessage::error(
            "SPICE deck import is blocked while a simulation execution owns the project",
        ));
        return false;
    }

    let (document, descriptor) = match canonical_import_document(
        state,
        &source,
        source_path.as_deref(),
        display_name,
        metadata,
        dependencies,
    ) {
        Ok(canonical) => canonical,
        Err(error) => {
            state.push_user_message(ConsoleMessage::error(format!(
                "SPICE deck import failed: {error}"
            )));
            return false;
        }
    };
    // Importing a new source deck changes future execution authority but does
    // not delete immutable datasets produced by earlier sources. Every run
    // carries its own provenance, so retained history remains truthful and
    // reviewable after this project-owned document changes.
    state.design_execution_epoch = state.design_execution_epoch.wrapping_add(1);
    state.ui.netlist = Default::default();
    state.workbench.netlist_open_documents.clear();
    let source_digest =
        crate::workbench::documents::netlist_document::source_content_digest(&source);
    state.workspace.netlist_source = Some(source.clone());
    state.workspace.netlist_document = Some(document.clone());
    state.workspace.netlist_descriptor = Some(descriptor);
    state.workspace.netlist_source_path = source_path;
    state.workspace.set_netlist_source_dirty(true);
    state.simulation.netlist_content = source;
    state.ui.netlist.owned_document = Some(document);
    state.ui.netlist.externally_saved_content_digest = Some(source_digest);
    state.ui.netlist.active_document =
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource;
    state.ui.netlist.active_document_initialized = true;
    state.ui.netlist.revision = state.ui.netlist.revision.wrapping_add(1);
    crate::workbench::documents::netlist_document::invalidate_source_evidence(
        &mut state.ui.netlist,
    );
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Netlist);
    state.push_user_message(ConsoleMessage::info(format!(
        "Imported SPICE deck: {display_name}"
    )));
    true
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
    if state.simulation.has_active_execution() {
        return Some("a simulation execution still owns the project");
    }
    if mode == NetlistImportMode::ImportIntoProject && !state.project_lifecycle.project_open {
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
        state.push_user_message(ConsoleMessage::error(format!(
            "Netlist project open failed: {display_name} is empty"
        )));
        return false;
    }
    if state.simulation.has_active_execution() {
        state.push_user_message(ConsoleMessage::error(
            "Netlist project open is blocked while a simulation execution owns the project",
        ));
        return false;
    }

    // Construct and validate the replacement off to the side. The currently
    // open project, its dirty documents and all retained evidence remain
    // untouched unless the complete netlist-first project is ready to commit.
    let mut candidate = state.clone();
    crate::workbench::workflows::project_workflow::create_new_project(&mut candidate);
    let proposed_name = std::path::Path::new(display_name)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .filter(|stem| !stem.trim().is_empty())
        .unwrap_or("Netlist Project");
    if candidate
        .workspace
        .project
        .rename(proposed_name.to_owned())
        .is_err()
    {
        let _ = candidate
            .workspace
            .project
            .rename("Netlist Project".to_owned());
    }
    if !apply_imported_netlist_transaction(
        &mut candidate,
        source,
        source_path,
        display_name,
        true,
        metadata,
        dependencies,
    ) {
        state.push_user_message(ConsoleMessage::error(
            "Netlist project open failed; the current project was left unchanged",
        ));
        return false;
    }
    candidate.push_user_message(ConsoleMessage::info(format!(
        "Opened netlist-first project: {display_name}"
    )));
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
            false,
            metadata,
            dependencies,
        ),
    }
}

pub(super) fn canonical_import_document(
    state: &AppState,
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
    use crate::state::{
        GeneratedArtifact, GeneratedProvenance, GenerationInput, NetlistDocument,
        NetlistDocumentId, SourceLocator,
    };

    let source_digest =
        crate::workbench::documents::netlist_document::source_content_digest(source);
    let provenance = GeneratedProvenance::try_new(
        "rspice-import-baseline/v1",
        GenerationInput::new(state.workspace.project.revision(), source_digest),
    )
    .map_err(|error| error.to_string())?;
    let baseline = GeneratedArtifact::try_from_utf8(
        provenance,
        source.as_bytes().to_vec(),
        dependencies.clone(),
        Vec::new(),
    )
    .map_err(|error| error.to_string())?;
    let mut document = NetlistDocument::from_generated(NetlistDocumentId::new(), baseline)
        .map_err(|error| error.to_string())?;

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
    document
        .import_source(
            document.content_digest(),
            locator,
            source.as_bytes().to_vec(),
        )
        .map_err(|error| error.to_string())?;
    document
        .make_editable(document.content_digest())
        .map_err(|error| error.to_string())?;
    if !dependencies.is_empty() {
        document
            .acknowledge_dependencies(document.content_digest(), dependencies)
            .map_err(|error| error.to_string())?;
    }

    let mut descriptor = crate::state::OwnedNetlistDescriptor {
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
