//! Exact, transactional source-file lifecycle for project source bundles.

#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]
#![cfg_attr(not(test), deny(clippy::panic, clippy::unreachable))]

use super::CodeWorkspaceParts;
use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::{
    ProjectSourceBundle, ProjectSourceFile, ProjectSourceLanguage, ProjectSourceOwner,
};
use rspice_app_services::workbench::slots::platform::PlatformState;
use rspice_app_state::workbench::app_state::model::Model;

use super::CodeWorkspaceRuntimeState;
use super::{
    CodeSourceFileAction, CodeSourceFileDialogState, CodeSourceHistoryState, VerilogAFileSelection,
};

pub fn source_file_mutation_block_reason(
    parts: &CodeWorkspaceParts<'_>,
    language: ProjectSourceLanguage,
) -> Option<&'static str> {
    source_file_mutation_block_reason_in(parts.model, parts.code, parts.platform, language)
}

/// [`source_file_mutation_block_reason`], over the model, the Code
/// workspace's runtime state and the platform's.
pub fn source_file_mutation_block_reason_in(
    model: &Model,
    code: &CodeWorkspaceRuntimeState,
    platform: &PlatformState,
    language: ProjectSourceLanguage,
) -> Option<&'static str> {
    if !platform.project_lifecycle.is_open() {
        return Some("Open a project before changing project source files.");
    }
    if model.safe_mode.project_read_only() {
        return Some("The project is open read-only.");
    }
    match language {
        ProjectSourceLanguage::VerilogA if code.veriloga.pending.is_some() => {
            Some("Wait for the Verilog-A compile to finish or cancel it first.")
        }
        _ => None,
    }
}

/// Resolve edit authority from persisted document ownership and semantic
/// roles. File extensions and demonstration filenames are never authority.
pub fn source_document_is_editable(
    model: &Model,
    code: &CodeWorkspaceRuntimeState,
    platform: &PlatformState,
    language: ProjectSourceLanguage,
    logical_path: &str,
) -> bool {
    let Some(bundle_id) = model
        .workspace
        .active_source_bundle(language)
        .map(ProjectSourceBundle::id)
    else {
        return false;
    };
    source_bundle_document_is_editable_in(model, code, platform, language, bundle_id, logical_path)
}

/// Resolve edit authority for an exact source bundle, over the model, the
/// Code workspace's runtime state and the platform's. A Verilog-A bundle
/// belongs to its cell view.
pub fn source_bundle_document_is_editable_in(
    model: &Model,
    code: &CodeWorkspaceRuntimeState,
    platform: &PlatformState,
    language: ProjectSourceLanguage,
    bundle_id: crate::state::ProjectSourceId,
    logical_path: &str,
) -> bool {
    if source_file_mutation_block_reason_in(model, code, platform, language).is_some() {
        return false;
    }
    source_bundle_contains_document_in(model, platform, language, bundle_id, logical_path)
}

/// Whether an exact retained document belongs to an authenticated source
/// owner in the currently open project. Read-only policy is intentionally not
/// part of this predicate so find, navigation, and language inspection remain
/// available in safe mode.
pub fn source_bundle_contains_document(
    parts: &CodeWorkspaceParts<'_>,
    language: ProjectSourceLanguage,
    bundle_id: crate::state::ProjectSourceId,
    logical_path: &str,
) -> bool {
    source_bundle_contains_document_in(
        parts.model,
        parts.platform,
        language,
        bundle_id,
        logical_path,
    )
}

/// [`source_bundle_contains_document`], over the model and the platform's
/// state.
pub(crate) fn source_bundle_contains_document_in(
    model: &Model,
    platform: &PlatformState,
    language: ProjectSourceLanguage,
    bundle_id: crate::state::ProjectSourceId,
    logical_path: &str,
) -> bool {
    if !platform.project_lifecycle.is_open() {
        return false;
    }
    model
        .workspace
        .content
        .project_sources
        .get_bundle(bundle_id)
        .is_some_and(|bundle| {
            let owner_matches = match bundle.owner() {
                ProjectSourceOwner::CodeWorkspace {
                    language: owner_language,
                } => *owner_language == language,
                ProjectSourceOwner::CellView { .. } => language == ProjectSourceLanguage::VerilogA,
            };
            bundle.language() == language && owner_matches && bundle.contains_file(logical_path)
        })
}

pub fn open_source_file_dialog(
    parts: &mut CodeWorkspaceParts<'_>,
    language: ProjectSourceLanguage,
    action: CodeSourceFileAction,
    source_path: &str,
    importer_path: &str,
) -> Result<(), String> {
    let bundle_id = parts
        .model
        .workspace
        .active_source_bundle(language)
        .map(ProjectSourceBundle::id)
        .ok_or_else(|| format!("The {} source bundle no longer exists.", language.label()))?;
    open_source_file_dialog_in_bundle(
        parts,
        language,
        bundle_id,
        action,
        source_path,
        importer_path,
    )
}

pub fn open_source_file_dialog_in_bundle(
    parts: &mut CodeWorkspaceParts<'_>,
    language: ProjectSourceLanguage,
    bundle_id: crate::state::ProjectSourceId,
    action: CodeSourceFileAction,
    source_path: &str,
    importer_path: &str,
) -> Result<(), String> {
    if let Some(reason) = source_file_mutation_block_reason(parts, language) {
        return Err(reason.to_owned());
    }
    let authority_path = if action == CodeSourceFileAction::New {
        importer_path
    } else {
        source_path
    };
    if !source_bundle_contains_document(parts, language, bundle_id, authority_path) {
        return Err(format!(
            "The selected source file '{authority_path}' no longer belongs to this project source bundle."
        ));
    }
    let bundle = parts
        .model
        .workspace
        .content
        .project_sources
        .get_bundle(bundle_id)
        .ok_or_else(|| format!("The {} source bundle no longer exists.", language.label()))?;
    if action != CodeSourceFileAction::New && !bundle.contains_file(source_path) {
        return Err(format!(
            "The selected source file '{source_path}' no longer exists."
        ));
    }
    if action == CodeSourceFileAction::Delete
        && crate::state::project_source_paths_equal(source_path, bundle.root().logical_path())
    {
        return Err(
            "The main file can't be deleted. Rename it or replace its source instead.".to_owned(),
        );
    }
    // A Verilog-A file is added by writing its `include into the file that
    // includes it. A file a role binds — the build profile — is
    // configuration, and a directive written into it would corrupt it, so a
    // new file asked for from there is included from the main file instead.
    // A copy stands beside its original: it is included from the file that
    // includes the original, rather than from the original itself.
    let veriloga = language == ProjectSourceLanguage::VerilogA;
    let importer_path = if veriloga && action == CodeSourceFileAction::Duplicate {
        veriloga_includers_of(bundle, source_path)
            .into_iter()
            .find(|includer| bundle.role_for_path(includer).is_none())
            .unwrap_or_else(|| bundle.root().logical_path().to_owned())
    } else if veriloga
        && action == CodeSourceFileAction::New
        && bundle.role_for_path(importer_path).is_some()
    {
        bundle.root().logical_path().to_owned()
    } else {
        importer_path.to_owned()
    };
    let importer_path = importer_path.as_str();
    let proposed_path = match action {
        CodeSourceFileAction::New => {
            unique_suggestion(bundle, importer_path, default_file_name(language))
        }
        CodeSourceFileAction::Duplicate => {
            unique_suggestion(bundle, source_path, duplicate_file_name(source_path))
        }
        CodeSourceFileAction::Rename
        | CodeSourceFileAction::Move
        | CodeSourceFileAction::Delete => source_path.to_owned(),
    };
    parts.code.source_file_dialog = Some(CodeSourceFileDialogState {
        action,
        project_id: parts.model.workspace.content.project.id(),
        bundle_id,
        bundle_revision: bundle.revision().get(),
        closure_digest: bundle.closure_digest(),
        language,
        source_path: source_path.to_owned(),
        importer_path: importer_path.to_owned(),
        proposed_path,
        error: None,
    });
    Ok(())
}

pub fn commit_source_file_dialog(parts: &mut CodeWorkspaceParts<'_>) -> Result<String, String> {
    let draft = parts
        .code
        .source_file_dialog
        .clone()
        .ok_or_else(|| "No source-file transaction is open.".to_owned())?;
    if let Some(reason) = source_file_mutation_block_reason(parts, draft.language) {
        return Err(reason.to_owned());
    }
    if draft.project_id != parts.model.workspace.content.project.id() {
        return Err(
            "The active project changed while the source-file transaction was open.".to_owned(),
        );
    }
    let bundle = parts
        .model
        .workspace
        .content
        .project_sources
        .get_bundle(draft.bundle_id)
        .ok_or_else(|| {
            "The source bundle was removed while the transaction was open.".to_owned()
        })?;
    let authority_path = if draft.action == CodeSourceFileAction::New {
        draft.importer_path.as_str()
    } else {
        draft.source_path.as_str()
    };
    if !source_bundle_contains_document(parts, draft.language, draft.bundle_id, authority_path) {
        return Err(
            "The file change no longer applies to a source bundle in the project.".to_owned(),
        );
    }
    if bundle.language() != draft.language
        || bundle.revision().get() != draft.bundle_revision
        || bundle.closure_digest() != draft.closure_digest
    {
        return Err(
            "The source bundle changed while the transaction was open. Review the current tree and try again."
                .to_owned(),
        );
    }

    let proposed = draft.proposed_path.trim();
    let after_delete = veriloga_includers_of(bundle, &draft.source_path)
        .into_iter()
        .next()
        .unwrap_or_else(|| bundle.root().logical_path().to_owned());
    let changed = match draft.action {
        CodeSourceFileAction::New => {
            let file = ProjectSourceFile::try_new(proposed, new_file_template(draft.language))
                .map_err(|error| error.to_string())?;
            parts
                .model
                .workspace
                .content
                .add_project_source_bundle_file(draft.bundle_id, &draft.importer_path, file)
                .map_err(|error| error.to_string())?
        }
        CodeSourceFileAction::Duplicate => {
            let content = bundle
                .file_content(&draft.source_path)
                .ok_or_else(|| {
                    format!("The source file '{}' no longer exists.", draft.source_path)
                })?
                .to_owned();
            let file =
                ProjectSourceFile::try_new(proposed, content).map_err(|error| error.to_string())?;
            parts
                .model
                .workspace
                .content
                .add_project_source_bundle_file(draft.bundle_id, &draft.importer_path, file)
                .map_err(|error| error.to_string())?
        }
        CodeSourceFileAction::Rename | CodeSourceFileAction::Move => parts
            .model
            .workspace
            .content
            .rename_project_source_bundle_file(draft.bundle_id, &draft.source_path, proposed)
            .map_err(|error| error.to_string())?,
        CodeSourceFileAction::Delete => parts
            .model
            .workspace
            .content
            .remove_project_source_bundle_file(draft.bundle_id, &draft.source_path)
            .map_err(|error| error.to_string())?,
    };
    if !changed {
        parts.code.source_file_dialog = None;
        return Ok("Nothing changed.".to_owned());
    }

    // A deleted file leaves the reader on the file that included it, else
    // on the main file.
    let selected_path = match draft.action {
        CodeSourceFileAction::Delete => after_delete,
        _ => proposed.to_owned(),
    };
    match draft.language {
        ProjectSourceLanguage::VerilogA => {
            parts.code.veriloga.selected_file = Some(VerilogAFileSelection {
                bundle_id: draft.bundle_id,
                logical_path: selected_path,
            });
            super::veriloga::invalidate_veriloga_evidence_in(
                parts.model,
                parts.code,
                draft.bundle_id,
            );
        }
    }
    parts.code.source_file_dialog = None;
    let message = match draft.action {
        CodeSourceFileAction::New => format!("Created {proposed}."),
        CodeSourceFileAction::Duplicate => {
            format!("Duplicated {} as {proposed}.", draft.source_path)
        }
        CodeSourceFileAction::Rename => {
            format!("Renamed {} to {proposed}.", draft.source_path)
        }
        CodeSourceFileAction::Move => {
            format!("Moved {} to {proposed}.", draft.source_path)
        }
        CodeSourceFileAction::Delete => format!("Deleted {}.", draft.source_path),
    };
    parts
        .model
        .record(LogSeverity::Info, LogSource::Studio, message.clone());
    Ok(message)
}

pub fn open_source_history(
    parts: &mut CodeWorkspaceParts<'_>,
    language: ProjectSourceLanguage,
    source_path: &str,
) -> Result<(), String> {
    let bundle_id = parts
        .model
        .workspace
        .active_source_bundle(language)
        .map(ProjectSourceBundle::id)
        .ok_or_else(|| format!("The {} source bundle no longer exists.", language.label()))?;
    open_source_history_in_bundle(parts, language, bundle_id, source_path)
}

pub(crate) fn open_source_history_in_bundle(
    parts: &mut CodeWorkspaceParts<'_>,
    language: ProjectSourceLanguage,
    bundle_id: crate::state::ProjectSourceId,
    source_path: &str,
) -> Result<(), String> {
    if !source_bundle_contains_document(parts, language, bundle_id, source_path) {
        return Err(format!(
            "The selected source file '{source_path}' no longer belongs to this project source bundle."
        ));
    }
    let bundle = parts
        .model
        .workspace
        .content
        .project_sources
        .get_bundle(bundle_id)
        .ok_or_else(|| format!("The {} source bundle no longer exists.", language.label()))?;
    if !bundle.contains_file(source_path) {
        return Err(format!(
            "The selected source file '{source_path}' no longer exists."
        ));
    }
    parts.code.source_history = Some(CodeSourceHistoryState {
        project_id: parts.model.workspace.content.project.id(),
        bundle_id,
        bundle_revision: bundle.revision().get(),
        closure_digest: bundle.closure_digest(),
        language,
        source_path: source_path.to_owned(),
        selected_revision: bundle
            .revision_history()
            .last()
            .map(|snapshot| snapshot.revision().get()),
        error: None,
    });
    Ok(())
}

pub fn commit_source_history_restore(parts: &mut CodeWorkspaceParts<'_>) -> Result<String, String> {
    let draft = parts
        .code
        .source_history
        .clone()
        .ok_or_else(|| "No source-history transaction is open.".to_owned())?;
    if parts.model.workspace.content.project.id() != draft.project_id {
        return Err("The active project changed while source history was open.".to_owned());
    }
    if let Some(reason) = source_file_mutation_block_reason(parts, draft.language) {
        return Err(reason.to_owned());
    }
    let retained_revision = draft
        .selected_revision
        .ok_or_else(|| "Select a retained revision to restore.".to_owned())?;
    let bundle = parts
        .model
        .workspace
        .content
        .project_sources
        .get_bundle(draft.bundle_id)
        .ok_or_else(|| "The source bundle no longer exists.".to_owned())?;
    if !source_bundle_contains_document(parts, draft.language, draft.bundle_id, &draft.source_path)
    {
        return Err(
            "Source history no longer applies to a source bundle in the project.".to_owned(),
        );
    }
    if bundle.revision().get() != draft.bundle_revision
        || bundle.closure_digest() != draft.closure_digest
    {
        return Err(
            "The source bundle changed while history was open. Reopen history before restoring."
                .to_owned(),
        );
    }
    let expected_current = bundle.revision();
    let retained_revision = crate::product::ObjectRevision::new(retained_revision)
        .map_err(|error| format!("The retained revision is invalid: {error}"))?;
    let changed = parts
        .model
        .workspace
        .content
        .restore_project_source_bundle_revision(
            draft.bundle_id,
            expected_current,
            retained_revision,
        )
        .map_err(|error| error.to_string())?;
    if !changed {
        parts.code.source_history = None;
        return Ok("The selected revision already matches the current source graph.".to_owned());
    }

    let restored_bundle = parts
        .model
        .workspace
        .content
        .project_sources
        .get_bundle(draft.bundle_id)
        .ok_or_else(|| "The restored source bundle is unavailable.".to_owned())?;
    let selected_path = if restored_bundle.contains_file(&draft.source_path) {
        draft.source_path.clone()
    } else {
        restored_bundle.root().logical_path().to_owned()
    };
    match draft.language {
        ProjectSourceLanguage::VerilogA => {
            parts.code.veriloga.selected_file = Some(VerilogAFileSelection {
                bundle_id: draft.bundle_id,
                logical_path: selected_path,
            });
            super::veriloga::invalidate_veriloga_evidence_in(
                parts.model,
                parts.code,
                draft.bundle_id,
            );
        }
    }
    parts.code.source_history = None;
    let message = format!(
        "Restored {} source bundle revision {} as a new revision.",
        draft.language.label(),
        retained_revision.get()
    );
    parts
        .model
        .record(LogSeverity::Info, LogSource::Studio, message.clone());
    Ok(message)
}

fn default_file_name(language: ProjectSourceLanguage) -> &'static str {
    match language {
        ProjectSourceLanguage::VerilogA => "untitled.va",
    }
}

fn duplicate_file_name(path: &str) -> String {
    let (directory, file) = path
        .rsplit_once('/')
        .map_or(("", path), |(dir, file)| (dir, file));
    let (stem, extension) = file
        .rsplit_once('.')
        .map_or((file, ""), |(stem, ext)| (stem, ext));
    let file = if extension.is_empty() {
        format!("{stem}_copy")
    } else {
        format!("{stem}_copy.{extension}")
    };
    if directory.is_empty() {
        file
    } else {
        format!("{directory}/{file}")
    }
}

fn unique_suggestion(
    bundle: &crate::state::ProjectSourceBundle,
    anchor: &str,
    file_name: impl AsRef<str>,
) -> String {
    let directory = anchor
        .rsplit_once('/')
        .map_or("", |(directory, _)| directory);
    let requested = file_name.as_ref();
    let (stem, extension) = requested
        .rsplit_once('.')
        .map_or((requested, ""), |(stem, extension)| (stem, extension));
    for index in 0_u32..10_000 {
        let suffix = if index == 0 {
            String::new()
        } else {
            format!("_{index}")
        };
        let file = if extension.is_empty() {
            format!("{stem}{suffix}")
        } else {
            format!("{stem}{suffix}.{extension}")
        };
        let candidate = if directory.is_empty() {
            file
        } else {
            format!("{directory}/{file}")
        };
        if !bundle.contains_file(&candidate) {
            return candidate;
        }
    }
    format!("{requested}.new")
}

/// The files a new or duplicated Verilog-A file can be included from: the
/// main file and every other file no role binds, main file first. A file a
/// role binds is configuration, and an `include written into it would
/// corrupt it.
pub fn veriloga_include_hosts(bundle: &ProjectSourceBundle) -> Vec<String> {
    std::iter::once(bundle.root().logical_path())
        .chain(
            bundle
                .files()
                .iter()
                .map(ProjectSourceFile::logical_path)
                .filter(|path| bundle.role_for_path(path).is_none()),
        )
        .map(str::to_owned)
        .collect()
}

/// The files whose `include of `logical_path` a rename rewrites or a delete
/// removes, in the bundle's order.
pub fn veriloga_includers_of(bundle: &ProjectSourceBundle, logical_path: &str) -> Vec<String> {
    bundle
        .dependencies()
        .iter()
        .filter(|edge| crate::state::project_source_paths_equal(edge.imported(), logical_path))
        .map(|edge| edge.importer().to_owned())
        .collect()
}

/// What a new file starts with. A Verilog-A file is included ahead of
/// everything in the file that includes it, so it brings the standard
/// headers it needs with it and compiles on its own terms; the reader's
/// caret goes under them.
fn new_file_template(language: ProjectSourceLanguage) -> String {
    match language {
        ProjectSourceLanguage::VerilogA => {
            "`include \"constants.vams\"\n`include \"disciplines.vams\"\n\n".to_owned()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::code::{
        CodeFeature,
        tests::{CodeHarness, harness},
    };

    #[test]
    fn production_source_lifecycle_has_no_panic_based_state_transitions() {
        let production = crate::source_guard::production_half(include_str!("source_files.rs"));
        for forbidden in [".expect(", ".unwrap(", "panic!("] {
            assert!(
                !production.contains(forbidden),
                "production source lifecycle contains panic shortcut {forbidden}"
            );
        }
    }

    fn veriloga_reference() -> crate::state::CellViewRef {
        crate::state::CellViewRef::new("behavioral", "gain", "veriloga")
    }

    /// A test app with the Verilog-A view `behavioral/gain/veriloga` open
    /// over a one-file bundle.
    fn veriloga_view_app() -> CodeHarness {
        let mut harness = harness();
        let reference = veriloga_reference();
        let mut view = crate::state::View::new("veriloga", crate::state::ViewType::VerilogA);
        view.metadata
            .insert("veriloga.module".to_owned(), "gain".to_owned());
        let mut cell = crate::state::Cell::new(&reference.cell);
        cell.add_view(view);
        let mut library = crate::state::Library::new(&reference.library);
        library.add_cell(cell);
        harness.model.library_manager.add_library(library);
        harness.model.workspace
            .content
            .project_sources
            .insert_bundle(
                ProjectSourceBundle::try_new(
                    ProjectSourceOwner::cell_view(reference.clone()),
                    ProjectSourceLanguage::VerilogA,
                    "gain.va",
                    "module gain(p, n);\n  inout p, n;\n  electrical p, n;\n  analog I(p, n) <+ V(p, n);\nendmodule\n",
                    [],
                    [],
                )
                .expect("a valid cell-view bundle"),
            )
            .expect("one bundle per cell view");
        harness
            .model
            .workspace
            .open_view(reference, crate::state::ViewType::VerilogA);
        harness
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn verilog_a_bundle(harness: &CodeHarness) -> &crate::state::ProjectSourceBundle {
        harness
            .model
            .workspace
            .content
            .project_sources
            .bundle_for_owner(&ProjectSourceOwner::cell_view(veriloga_reference()))
            .expect("the test project has a Verilog-A bundle")
    }

    #[test]
    fn duplicate_names_preserve_directory_and_extension() {
        assert_eq!(duplicate_file_name("models/gain.va"), "models/gain_copy.va");
        assert_eq!(duplicate_file_name("README"), "README_copy");
    }

    #[test]
    fn cell_view_file_transactions_retain_the_exact_veriloga_bundle() {
        let bundle = ProjectSourceBundle::try_new(
            ProjectSourceOwner::cell_view(crate::state::CellViewRef::new(
                "behavioral",
                "gain",
                "veriloga",
            )),
            ProjectSourceLanguage::VerilogA,
            "cell/gain.va",
            "module gain; endmodule\n",
            [],
            [],
        )
        .unwrap();
        let bundle_id = bundle.id();
        let mut harness = harness();
        harness
            .model
            .workspace
            .content
            .project_sources
            .insert_bundle(bundle)
            .unwrap();

        open_source_file_dialog_in_bundle(
            &mut CodeFeature::workspace_parts(&mut harness.cx()),
            ProjectSourceLanguage::VerilogA,
            bundle_id,
            CodeSourceFileAction::Duplicate,
            "cell/gain.va",
            "cell/gain.va",
        )
        .unwrap();
        harness
            .state
            .code_workspace
            .source_file_dialog
            .as_mut()
            .unwrap()
            .proposed_path = "cell/gain_copy.va".to_owned();
        commit_source_file_dialog(&mut CodeFeature::workspace_parts(&mut harness.cx())).unwrap();

        let exact = harness
            .model
            .workspace
            .content
            .project_sources
            .get_bundle(bundle_id)
            .unwrap();
        assert!(exact.contains_file("cell/gain_copy.va"));
        assert_eq!(
            harness
                .state
                .code_workspace
                .veriloga
                .selected_file
                .as_ref()
                .map(|selection| selection.bundle_id),
            Some(bundle_id)
        );
        open_source_history_in_bundle(
            &mut CodeFeature::workspace_parts(&mut harness.cx()),
            ProjectSourceLanguage::VerilogA,
            bundle_id,
            "cell/gain.va",
        )
        .unwrap();
        assert_eq!(
            harness
                .state
                .code_workspace
                .source_history
                .as_ref()
                .map(|history| history.bundle_id),
            Some(bundle_id)
        );
    }

    #[test]
    fn editor_authority_rejects_closed_read_only_and_missing_sources() {
        let mut harness = veriloga_view_app();
        let verilog_owner = ProjectSourceOwner::cell_view(veriloga_reference());
        let verilog_root = harness
            .model
            .workspace
            .content
            .project_sources
            .bundle_for_owner(&verilog_owner)
            .expect("test Verilog-A bundle")
            .root()
            .logical_path()
            .to_owned();
        let editable = |harness: &CodeHarness, path: &str| {
            source_document_is_editable(
                &harness.model,
                &harness.state.code_workspace,
                &harness.platform,
                ProjectSourceLanguage::VerilogA,
                path,
            )
        };

        assert!(editable(&harness, &verilog_root));
        assert!(!editable(&harness, "missing/model.va"));

        harness.close_project_session();
        assert!(!editable(&harness, &verilog_root));

        harness.open_project_session();
        harness.model.safe_mode.activate(
            crate::state::write_policy::LocalSafeModeOptions {
                open_project_read_only: true,
                ..Default::default()
            },
            String::new(),
        );
        assert!(!editable(&harness, &verilog_root));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn source_file_actions_are_persisted_transactionally() {
        let mut harness = veriloga_view_app();
        harness.model.workspace.content.mark_project_sources_clean();
        let root = verilog_a_bundle(&harness).root().logical_path().to_owned();

        open_source_file_dialog(
            &mut CodeFeature::workspace_parts(&mut harness.cx()),
            ProjectSourceLanguage::VerilogA,
            CodeSourceFileAction::New,
            &root,
            &root,
        )
        .unwrap();
        harness
            .state
            .code_workspace
            .source_file_dialog
            .as_mut()
            .unwrap()
            .proposed_path = "models/helper.va".to_owned();
        commit_source_file_dialog(&mut CodeFeature::workspace_parts(&mut harness.cx())).unwrap();
        assert!(verilog_a_bundle(&harness).contains_file("models/helper.va"));
        assert!(harness.model.workspace.content.project_sources_dirty);

        open_source_file_dialog(
            &mut CodeFeature::workspace_parts(&mut harness.cx()),
            ProjectSourceLanguage::VerilogA,
            CodeSourceFileAction::Rename,
            "models/helper.va",
            &root,
        )
        .unwrap();
        harness
            .state
            .code_workspace
            .source_file_dialog
            .as_mut()
            .unwrap()
            .proposed_path = "models/renamed.va".to_owned();
        commit_source_file_dialog(&mut CodeFeature::workspace_parts(&mut harness.cx())).unwrap();
        assert!(!verilog_a_bundle(&harness).contains_file("models/helper.va"));
        assert!(verilog_a_bundle(&harness).contains_file("models/renamed.va"));

        open_source_file_dialog(
            &mut CodeFeature::workspace_parts(&mut harness.cx()),
            ProjectSourceLanguage::VerilogA,
            CodeSourceFileAction::Move,
            "models/renamed.va",
            &root,
        )
        .unwrap();
        harness
            .state
            .code_workspace
            .source_file_dialog
            .as_mut()
            .unwrap()
            .proposed_path = "library/renamed.va".to_owned();
        commit_source_file_dialog(&mut CodeFeature::workspace_parts(&mut harness.cx())).unwrap();
        assert!(!verilog_a_bundle(&harness).contains_file("models/renamed.va"));
        assert!(verilog_a_bundle(&harness).contains_file("library/renamed.va"));

        open_source_file_dialog(
            &mut CodeFeature::workspace_parts(&mut harness.cx()),
            ProjectSourceLanguage::VerilogA,
            CodeSourceFileAction::Duplicate,
            "library/renamed.va",
            &root,
        )
        .unwrap();
        let duplicate = harness
            .state
            .code_workspace
            .source_file_dialog
            .as_ref()
            .unwrap()
            .proposed_path
            .clone();
        commit_source_file_dialog(&mut CodeFeature::workspace_parts(&mut harness.cx())).unwrap();
        assert!(verilog_a_bundle(&harness).contains_file(&duplicate));

        open_source_file_dialog(
            &mut CodeFeature::workspace_parts(&mut harness.cx()),
            ProjectSourceLanguage::VerilogA,
            CodeSourceFileAction::Delete,
            &duplicate,
            &root,
        )
        .unwrap();
        commit_source_file_dialog(&mut CodeFeature::workspace_parts(&mut harness.cx())).unwrap();
        assert!(!verilog_a_bundle(&harness).contains_file(&duplicate));
    }

    /// New source in a cell view's bundle refuses a path that differs from a
    /// file it holds only by Unicode case, the root's included, and leaves the
    /// bundle as it was.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn new_source_in_a_cell_bundle_rejects_unicode_case_alias_paths() {
        let mut harness = veriloga_view_app();
        let root = verilog_a_bundle(&harness).root().logical_path().to_owned();
        let create = |harness: &mut CodeHarness, path: &str| {
            open_source_file_dialog(
                &mut CodeFeature::workspace_parts(&mut harness.cx()),
                ProjectSourceLanguage::VerilogA,
                CodeSourceFileAction::New,
                &root,
                &root,
            )
            .unwrap();
            harness
                .state
                .code_workspace
                .source_file_dialog
                .as_mut()
                .unwrap()
                .proposed_path = path.to_owned();
            let result =
                commit_source_file_dialog(&mut CodeFeature::workspace_parts(&mut harness.cx()));
            harness.state.code_workspace.source_file_dialog = None;
            result
        };
        create(&mut harness, "Models/étage.va").expect("a new path is created");
        let before = verilog_a_bundle(&harness).clone();

        for alias in ["MODELS/ÉTAGE.VA", "models/Étage.va", "GAIN.VA"] {
            let error = create(&mut harness, alias).expect_err(alias);
            assert!(error.contains("case-insensitively"), "{alias}: {error}");
            assert_eq!(
                verilog_a_bundle(&harness),
                &before,
                "{alias} changed nothing"
            );
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn source_file_commit_rejects_a_stale_bundle_transaction() {
        let mut harness = veriloga_view_app();
        let root = verilog_a_bundle(&harness).root().logical_path().to_owned();
        let bundle_id = verilog_a_bundle(&harness).id();
        let before = verilog_a_bundle(&harness).closure_digest();
        open_source_file_dialog(
            &mut CodeFeature::workspace_parts(&mut harness.cx()),
            ProjectSourceLanguage::VerilogA,
            CodeSourceFileAction::New,
            &root,
            &root,
        )
        .unwrap();
        harness
            .model
            .workspace
            .content
            .replace_project_source_bundle_file(
                bundle_id,
                &root,
                "module changed; endmodule\n".to_owned(),
            )
            .unwrap();

        let error = commit_source_file_dialog(&mut CodeFeature::workspace_parts(&mut harness.cx()))
            .unwrap_err();
        assert!(error.contains("changed while the transaction was open"));
        assert_ne!(verilog_a_bundle(&harness).closure_digest(), before);
        assert!(!verilog_a_bundle(&harness).contains_file("untitled.va"));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn source_history_restores_the_exact_bundle_as_a_new_revision() {
        let mut harness = veriloga_view_app();
        let bundle_id = verilog_a_bundle(&harness).id();
        let root = verilog_a_bundle(&harness).root().logical_path().to_owned();
        let original = verilog_a_bundle(&harness).root().content().to_owned();
        harness
            .model
            .workspace
            .content
            .replace_project_source_bundle_file(
                bundle_id,
                &root,
                "module changed; endmodule\n".to_owned(),
            )
            .unwrap();
        let edited_revision = verilog_a_bundle(&harness).revision();

        open_source_history(
            &mut CodeFeature::workspace_parts(&mut harness.cx()),
            ProjectSourceLanguage::VerilogA,
            &root,
        )
        .unwrap();
        commit_source_history_restore(&mut CodeFeature::workspace_parts(&mut harness.cx()))
            .unwrap();

        assert_eq!(verilog_a_bundle(&harness).root().content(), original);
        assert!(verilog_a_bundle(&harness).revision().get() > edited_revision.get());
        assert!(harness.state.code_workspace.source_history.is_none());
        assert!(harness.model.workspace.content.project_sources_dirty);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn source_history_restore_rejects_a_newer_source_closure() {
        let mut harness = veriloga_view_app();
        let bundle_id = verilog_a_bundle(&harness).id();
        let root = verilog_a_bundle(&harness).root().logical_path().to_owned();
        harness
            .model
            .workspace
            .content
            .replace_project_source_bundle_file(
                bundle_id,
                &root,
                "module first; endmodule\n".to_owned(),
            )
            .unwrap();
        open_source_history(
            &mut CodeFeature::workspace_parts(&mut harness.cx()),
            ProjectSourceLanguage::VerilogA,
            &root,
        )
        .unwrap();
        harness
            .model
            .workspace
            .content
            .replace_project_source_bundle_file(
                bundle_id,
                &root,
                "module newer; endmodule\n".to_owned(),
            )
            .unwrap();

        let error =
            commit_source_history_restore(&mut CodeFeature::workspace_parts(&mut harness.cx()))
                .unwrap_err();
        assert!(error.contains("changed while history was open"));
        assert_eq!(
            verilog_a_bundle(&harness).root().content(),
            "module newer; endmodule\n"
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn bundle_root_delete_is_rejected_before_a_dialog_opens() {
        let mut harness = veriloga_view_app();
        let root = verilog_a_bundle(&harness).root().logical_path().to_owned();
        let error = open_source_file_dialog(
            &mut CodeFeature::workspace_parts(&mut harness.cx()),
            ProjectSourceLanguage::VerilogA,
            CodeSourceFileAction::Delete,
            &root,
            &root,
        )
        .unwrap_err();
        assert!(error.contains("main file can't be deleted"), "{error}");
        assert!(harness.state.code_workspace.source_file_dialog.is_none());
    }
}
