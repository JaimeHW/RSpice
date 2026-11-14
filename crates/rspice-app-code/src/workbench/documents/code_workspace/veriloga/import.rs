//! Verilog-A/AMS files arriving from disk: File > Import's pick or drop of
//! roots and the files their includes reach, Add files on the Files rail, a
//! Locate for an include the import form misses, and Replace source from
//! file.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::workspace::{
    DeclaredModule, ImportClosure, ImportedInclude, resolve_veriloga_import,
};
use crate::state::{
    LibraryManager, MAX_PROJECT_CODE_SOURCE_BYTES, ProjectSourceBundle, ProjectSourceFile,
    ProjectSourceId, ProjectSourceLanguage,
};
use crate::workbench::documents::code_workspace::CodeWorkspaceParts;
use rspice_app_state::workbench::app_state::model::Model;

use super::super::CodeWorkspaceRuntimeState;
use super::super::page::{VerilogAImportTarget, VerilogAWorkbenchState};
use super::super::{
    VerilogAFileSelection, VerilogAImportFormState, VerilogAReplaceReview, VerilogAReplaceTarget,
};
use super::{invalidate_veriloga_evidence, invalidate_veriloga_evidence_in};
use rspice_app_workbench::workbench::state::WorkbenchSession;

#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
struct BrowserImportCompletion {
    token: crate::workbench::browser::file_import::TextImportToken,
    result: Result<Option<crate::workbench::browser::file_import::PickedTextFile>, String>,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_IMPORT_COMPLETION: std::cell::RefCell<Option<BrowserImportCompletion>> =
        const { std::cell::RefCell::new(None) };
}

/// The pickers' file types: Verilog-A/AMS sources and the headers they
/// include. Native pickers also offer every file.
const VERILOGA_FILE_TYPES: (&str, &[&str]) = (
    "Verilog-A/AMS",
    &["va", "vams", "vh", "h", "inc", "include"],
);

/// A file picked or dropped for import: its name, its text, and the folder it
/// was read from when it came from disk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PickedSource {
    pub name: String,
    pub contents: String,
    pub folder: Option<PathBuf>,
}

/// Consume an import request raised by the File menu, the Files rail or the
/// import form. Native and browser pickers converge on the same exact UTF-8
/// project-source transaction.
pub fn poll_veriloga_import(
    model: &mut Model,
    code: &mut CodeWorkspaceRuntimeState,
    wb: &mut WorkbenchSession,
) {
    #[cfg(target_arch = "wasm32")]
    poll_browser_import_completion(model, code, wb);

    if !code.veriloga.import_requested {
        return;
    }
    code.veriloga.import_requested = false;

    #[cfg(not(target_arch = "wasm32"))]
    {
        let veriloga = &code.veriloga;
        let adding = veriloga.import_target.is_some();
        let replacing = veriloga.replace_target.is_some();
        let locating = veriloga
            .import_form
            .as_ref()
            .and_then(|form| form.locating.clone());
        let title = if adding {
            "Add files to the Verilog-A/AMS source".to_owned()
        } else if replacing {
            "Replace Verilog-A/AMS source from file".to_owned()
        } else if let Some(path) = &locating {
            format!("Locate {path}")
        } else {
            "Import Verilog-A/AMS source".to_owned()
        };
        let (types, extensions) = VERILOGA_FILE_TYPES;
        let dialog = rfd::FileDialog::new()
            .add_filter(types, extensions)
            .add_filter("All files", &["*"])
            .set_title(title);
        let paths = if replacing || locating.is_some() {
            dialog.pick_file().map(|path| vec![path])
        } else {
            dialog.pick_files()
        };
        let Some(paths) = paths.filter(|paths| !paths.is_empty()) else {
            abandon_pick(&mut code.veriloga);
            return;
        };
        let picked = paths
            .iter()
            .map(|path| read_picked_source(path))
            .collect::<Result<Vec<_>, _>>();
        match picked {
            Ok(picked) => apply_import(model, code, wb, picked),
            Err(error) => {
                abandon_pick(&mut code.veriloga);
                model.record(
                    LogSeverity::Error,
                    LogSource::Studio,
                    format!("Verilog-A/AMS import failed: {error}"),
                );
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        let token = match crate::workbench::browser::file_import::try_begin_text_import(
            crate::workbench::browser::file_import::BrowserTextImportKind::VerilogA,
        ) {
            Ok(token) => token,
            Err(error) => {
                abandon_pick(&mut code.veriloga);
                wb.prompt(format!("Verilog-A/AMS import failed: {error}"));
                return;
            }
        };
        let (types, extensions) = VERILOGA_FILE_TYPES;
        crate::workbench::browser::file_import::pick_text_file(types, extensions, move |result| {
            if crate::workbench::browser::file_import::text_import_is_current(token) {
                BROWSER_IMPORT_COMPLETION.with(|slot| {
                    *slot.borrow_mut() = Some(BrowserImportCompletion { token, result });
                });
            }
        });
    }
}

/// A pick that delivered nothing: whatever it was bound to is released.
fn abandon_pick(veriloga: &mut VerilogAWorkbenchState) {
    veriloga.import_target = None;
    veriloga.replace_target = None;
    if let Some(form) = veriloga.import_form.as_mut() {
        form.locating = None;
    }
}

/// The text of the file at `path`, refused past the source size limit or
/// when it is not UTF-8.
#[cfg(not(target_arch = "wasm32"))]
fn read_picked_source(path: &Path) -> Result<PickedSource, String> {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.trim().is_empty())
        .ok_or_else(|| format!("{} has no valid UTF-8 file name", path.display()))?;
    let metadata = std::fs::metadata(path).map_err(|error| format!("{name}: {error}"))?;
    if metadata.len() > MAX_PROJECT_CODE_SOURCE_BYTES as u64 {
        return Err(format!(
            "{name} exceeds the supported {MAX_PROJECT_CODE_SOURCE_BYTES}-byte limit"
        ));
    }
    let bytes = std::fs::read(path).map_err(|error| format!("{name}: {error}"))?;
    if bytes.len() > MAX_PROJECT_CODE_SOURCE_BYTES {
        return Err(format!(
            "{name} exceeds the supported {MAX_PROJECT_CODE_SOURCE_BYTES}-byte limit"
        ));
    }
    let contents =
        String::from_utf8(bytes).map_err(|error| format!("{name} is not valid UTF-8: {error}"))?;
    Ok(PickedSource {
        name: name.to_owned(),
        contents,
        folder: path.parent().map(Path::to_path_buf),
    })
}

#[cfg(target_arch = "wasm32")]
fn poll_browser_import_completion(
    model: &mut Model,
    code: &mut CodeWorkspaceRuntimeState,
    wb: &mut WorkbenchSession,
) {
    let Some(completion) = BROWSER_IMPORT_COMPLETION.with(|slot| slot.borrow_mut().take()) else {
        return;
    };
    if !crate::workbench::browser::file_import::finish_text_import(completion.token) {
        return;
    }
    match completion.result {
        Ok(Some(file)) => apply_import(
            model,
            code,
            wb,
            vec![PickedSource {
                name: file.name,
                contents: file.contents,
                folder: None,
            }],
        ),
        Ok(None) => abandon_pick(&mut code.veriloga),
        Err(error) => {
            abandon_pick(&mut code.veriloga);
            model.record(
                LogSeverity::Error,
                LogSource::Studio,
                format!("Verilog-A/AMS import failed: {error}"),
            );
        }
    }
}

/// Deliver picked files to whatever asked for them: Add files, a
/// replacement, a Locate on the import form, or else File > Import.
fn apply_import(
    model: &mut Model,
    code: &mut CodeWorkspaceRuntimeState,
    wb: &mut WorkbenchSession,
    picked: Vec<PickedSource>,
) {
    // This is also the single completion point deterministic tests use.
    // Consume the request here as well as in the polling adapter so no
    // successful, failed, or stale completion can leave the workspace
    // permanently locked in "pending".
    let veriloga = &mut code.veriloga;
    veriloga.import_requested = false;
    if let Some(target) = veriloga.import_target.take() {
        veriloga.replace_target = None;
        if !import_target_is_current(model, &target) {
            model.record(
                LogSeverity::Error,
                LogSource::Studio,
                "The files were not added: the source changed while the picker was open.",
            );
            return;
        }
        let mut added = Vec::new();
        for file in picked {
            let logical_path = suggested_import_path(&target.importer_path, &file.name);
            match add_bundle_file(
                model,
                code,
                target.bundle_id,
                &target.importer_path,
                logical_path.clone(),
                file.contents,
            ) {
                Ok(true) => added.push(logical_path),
                Ok(false) => {}
                Err(error) => {
                    model.record(
                        LogSeverity::Error,
                        LogSource::Studio,
                        format!("{} was not added: {error}", file.name),
                    );
                }
            }
        }
        if !added.is_empty() {
            model.record(
                LogSeverity::Info,
                LogSource::Studio,
                format!("Added {} to the source.", added.join(", ")),
            );
        }
        return;
    }
    if let Some(target) = veriloga.replace_target.take() {
        // "Replace source from file…" replaces nothing on arrival: the
        // picked file waits for the reader to confirm which file it replaces.
        let Some(file) = picked.into_iter().next() else {
            return;
        };
        let Some(bundle) = current_replace_bundle(model, &target) else {
            model.record(
                LogSeverity::Error,
                LogSource::Studio,
                format!(
                    "Replacing {} from {} was discarded because the source changed while the picker was open.",
                    target.logical_path, file.name
                ),
            );
            return;
        };
        let restorable =
            replacement_keeps_previous_text(bundle, &target.logical_path, &file.contents);
        code.veriloga.replace_review = Some(VerilogAReplaceReview {
            target,
            file_name: file.name,
            contents: file.contents,
            restorable,
        });
        return;
    }
    if let Some(form) = veriloga.import_form.as_mut()
        && let Some(path) = form.locating.take()
    {
        if let Some(file) = picked.into_iter().next() {
            form.locate(path, file.contents);
        }
        return;
    }
    // File > Import never touches an open view: the file becomes cell views
    // of its own, and the form says which, and where, before any is made.
    if let Err(error) = open_veriloga_import_forms(model, &mut code.veriloga, picked) {
        wb.prompt(error);
    }
}

/// Whether the bundle Add files was asked for is still the revision the
/// rail showed, and still holds the file that was in front.
fn import_target_is_current(model: &Model, target: &VerilogAImportTarget) -> bool {
    let content = &model.workspace.content;
    content.project.id() == target.project_id
        && content
            .project_sources
            .get_bundle(target.bundle_id)
            .is_some_and(|bundle| {
                bundle.revision().get() == target.bundle_revision
                    && bundle.closure_digest() == target.closure_digest
                    && bundle.contains_file(&target.importer_path)
            })
}

/// Ask for files to add to `bundle_id`, beside `importer_path`, the file in
/// front of the reader. The picker is bound to that exact revision. The
/// caller has checked that the project's sources may change.
pub fn request_veriloga_add_files(
    model: &Model,
    veriloga: &mut VerilogAWorkbenchState,
    bundle_id: ProjectSourceId,
    importer_path: &str,
) -> Result<(), String> {
    if veriloga.import_requested {
        return Err("A Verilog-A/AMS file is already being chosen.".to_owned());
    }
    let content = &model.workspace.content;
    let bundle = content
        .project_sources
        .get_bundle(bundle_id)
        .ok_or_else(|| "The source is no longer in the project.".to_owned())?;
    let target = VerilogAImportTarget {
        project_id: content.project.id(),
        bundle_id,
        bundle_revision: bundle.revision().get(),
        closure_digest: bundle.closure_digest(),
        importer_path: importer_path.to_owned(),
    };
    veriloga.import_target = Some(target);
    veriloga.replace_target = None;
    veriloga.import_requested = true;
    Ok(())
}

/// Ask for the file that holds `path`, a missing include of the open import
/// form.
pub fn request_veriloga_import_locate(veriloga: &mut VerilogAWorkbenchState, path: &str) {
    if veriloga.import_requested {
        return;
    }
    let Some(form) = veriloga.import_form.as_mut() else {
        return;
    };
    form.locating = Some(path.to_owned());
    veriloga.import_target = None;
    veriloga.replace_target = None;
    veriloga.import_requested = true;
}

/// What the picker delivers when a file is chosen, for tests that cannot
/// open a native picker.
#[cfg(any(test, feature = "test-support"))]
pub fn complete_veriloga_file_pick(
    parts: &mut CodeWorkspaceParts<'_>,
    file_name: &str,
    contents: &str,
) {
    complete_veriloga_files_pick(parts, &[(file_name, contents)], None);
}

/// What the picker delivers when files are chosen from `folder`.
#[cfg(any(test, feature = "test-support"))]
pub fn complete_veriloga_files_pick(
    parts: &mut CodeWorkspaceParts<'_>,
    files: &[(&str, &str)],
    folder: Option<&Path>,
) {
    apply_import(
        parts.model,
        parts.code,
        parts.wb,
        files
            .iter()
            .map(|(name, contents)| PickedSource {
                name: (*name).to_owned(),
                contents: (*contents).to_owned(),
                folder: folder.map(Path::to_path_buf),
            })
            .collect(),
    );
}

/// Whether `name` can be the root of an import: a `.va` or `.vams` file.
fn is_veriloga_root(name: &str) -> bool {
    Path::new(name)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .is_some_and(|extension| {
            extension.eq_ignore_ascii_case("va") || extension.eq_ignore_ascii_case("vams")
        })
}

/// Open File > Import's form for each root among `picked`, the first now
/// and the rest in turn. A root is a `.va` or `.vams` file no other picked
/// root includes; every picked file is a file each form can resolve.
fn open_veriloga_import_forms(
    model: &Model,
    veriloga: &mut VerilogAWorkbenchState,
    picked: Vec<PickedSource>,
) -> Result<(), String> {
    let files = picked
        .iter()
        .map(|file| (file.name.clone(), file.contents.clone()))
        .collect::<BTreeMap<_, _>>();
    let candidates = picked
        .iter()
        .filter(|file| is_veriloga_root(&file.name))
        .collect::<Vec<_>>();
    let included = candidates
        .iter()
        .flat_map(|candidate| {
            let mut files = files.clone();
            resolve_veriloga_import(&candidate.name, &mut files, &|_| None)
                .files
                .into_iter()
                .filter(|file| file.include == ImportedInclude::Found)
                .map(|file| file.path.to_lowercase())
        })
        .collect::<BTreeSet<_>>();
    let mut forms = candidates
        .into_iter()
        .filter(|candidate| !included.contains(&candidate.name.to_lowercase()))
        .map(|root| {
            VerilogAImportFormState::read(
                model,
                root.name.clone(),
                files.clone(),
                root.folder.clone(),
            )
        })
        .collect::<Vec<_>>();
    if forms.is_empty() {
        return Err(match picked.as_slice() {
            [one] => format!(
                "{} is not a Verilog-A/AMS root file; import a .va or .vams file.",
                one.name
            ),
            _ => "None of the files is a Verilog-A/AMS root file; import a .va or .vams file."
                .to_owned(),
        });
    }
    let first = forms.remove(0);
    veriloga.import_form = Some(first);
    veriloga.import_queue = forms;
    Ok(())
}

/// Close File > Import's form, opening the next root of the same pick or
/// drop when one is waiting.
pub fn close_veriloga_import_form(veriloga: &mut VerilogAWorkbenchState) {
    veriloga.import_form =
        (!veriloga.import_queue.is_empty()).then(|| veriloga.import_queue.remove(0));
}

/// The bundle a replacement was asked for, while it is exactly the revision
/// the editor showed and still holds the file.
fn current_replace_bundle<'a>(
    model: &'a Model,
    target: &VerilogAReplaceTarget,
) -> Option<&'a ProjectSourceBundle> {
    (target.project_id == model.workspace.content.project.id())
        .then(|| {
            model
                .workspace
                .content
                .project_sources
                .get_bundle(target.bundle_id)
        })
        .flatten()
        .filter(|bundle| {
            bundle.revision().get() == target.bundle_revision
                && bundle.closure_digest() == target.closure_digest
                && bundle.contains_file(&target.logical_path)
        })
}

/// Whether replacing `logical_path` with `contents` leaves the text it holds
/// now in the bundle's revision history, where the source history restores
/// it. Measured on a copy, by doing the replacement.
fn replacement_keeps_previous_text(
    bundle: &ProjectSourceBundle,
    logical_path: &str,
    contents: &str,
) -> bool {
    let Some(previous) = bundle.file_content(logical_path) else {
        return false;
    };
    let mut replaced = bundle.clone();
    replaced
        .replace_file_content(logical_path, contents.to_owned())
        .is_ok_and(|changed| changed)
        && replaced
            .revision_snapshot(bundle.revision())
            .and_then(|snapshot| snapshot.file_content(logical_path))
            == Some(previous)
}

/// Ask for a file to replace the one the editor is showing.
///
/// Opens the picker bound to that exact file; the replacement itself waits
/// for the confirmation the picked file opens.
pub fn request_veriloga_source_replacement(
    parts: &mut CodeWorkspaceParts<'_>,
    logical_path: &str,
) -> Result<(), String> {
    if let Some(reason) =
        super::super::source_file_mutation_block_reason(parts, ProjectSourceLanguage::VerilogA)
    {
        return Err(reason.to_owned());
    }
    if parts.code.veriloga.import_requested {
        return Err("A Verilog-A/AMS file is already being chosen.".to_owned());
    }
    let selected = super::selected_veriloga_source_in(parts.model)?;
    if !crate::workbench::documents::code_workspace::source_bundle_document_is_editable_in(
        parts.model,
        parts.code,
        parts.platform,
        ProjectSourceLanguage::VerilogA,
        selected.bundle().id(),
        logical_path,
    ) {
        return Err(format!("{logical_path} cannot be edited."));
    }
    parts.code.veriloga.replace_target = Some(VerilogAReplaceTarget {
        project_id: parts.model.workspace.content.project.id(),
        view: parts.model.workspace.content.active_view.clone(),
        bundle_id: selected.bundle().id(),
        bundle_revision: selected.bundle().revision().get(),
        closure_digest: selected.bundle().closure_digest(),
        logical_path: logical_path.to_owned(),
    });
    parts.code.veriloga.import_target = None;
    parts.code.veriloga.import_requested = true;
    Ok(())
}

/// Replace the file a confirmed review names with the picked file, when that
/// file is still exactly what the editor showed.
pub fn commit_veriloga_source_replacement(
    parts: &mut CodeWorkspaceParts<'_>,
) -> Result<(), String> {
    let review = parts
        .code
        .veriloga
        .replace_review
        .take()
        .ok_or_else(|| "No source replacement is waiting for confirmation.".to_owned())?;
    if current_replace_bundle(parts.model, &review.target).is_none() {
        return Err(format!(
            "{} changed after the replacement was chosen; nothing was replaced.",
            review.target.logical_path
        ));
    }
    let changed = parts.code.source_edit_history.replace(
        &mut parts.model.workspace.content,
        review.target.bundle_id,
        vec![(review.target.logical_path.clone(), review.contents)],
    )? > 0;
    if changed {
        invalidate_veriloga_evidence(parts, review.target.bundle_id);
        parts.model.record(
            LogSeverity::Info,
            LogSource::Studio,
            format!(
                "Replaced {} with {}; compile is required.",
                review.target.logical_path, review.file_name
            ),
        );
    } else {
        parts.wb.prompt(format!(
            "{} already holds the text of {}.",
            review.target.logical_path, review.file_name
        ));
    }
    Ok(())
}

impl VerilogAImportFormState {
    /// The form for the root `file_name` among `files`, read once: what its
    /// includes reach, the top modules it declares, and the library the
    /// active design lives in, when it may be written.
    fn read(
        model: &Model,
        file_name: String,
        files: BTreeMap<String, String>,
        folder: Option<PathBuf>,
    ) -> Self {
        let writable = writable_libraries(&model.library_manager);
        let library = [
            &model.workspace.content.active_view.library,
            &model.workspace.content.project.root_library,
        ]
        .into_iter()
        .find(|library| writable.contains(*library))
        .or_else(|| writable.first())
        .cloned()
        .unwrap_or_default();
        let mut form = Self {
            project_id: model.workspace.content.project.id(),
            file_name,
            files,
            folder,
            closure: ImportClosure {
                files: Vec::new(),
                edges: Vec::new(),
                modules: Ok(Vec::new()),
            },
            library,
            chosen: Vec::new(),
            error: None,
            locating: None,
        };
        form.resolve();
        form.chosen = form
            .closure
            .modules
            .iter()
            .flatten()
            .map(|module| module.name.clone())
            .collect();
        form
    }

    /// Walk the root's includes again: over the files the import holds, and
    /// the root's folder for any it does not hold yet.
    fn resolve(&mut self) {
        let folder = self.folder.clone();
        let read = |logical: &str| read_beside_root(folder.as_deref(), logical);
        let before = self
            .closure
            .modules
            .iter()
            .flatten()
            .map(|module| module.name.clone())
            .collect::<Vec<_>>();
        self.closure = resolve_veriloga_import(&self.file_name, &mut self.files, &read);
        // A module the walk reads for the first time, once a located file
        // completes the source, is chosen like every module was at first.
        let arrived = self
            .closure
            .modules
            .iter()
            .flatten()
            .filter(|module| !before.contains(&module.name))
            .map(|module| module.name.clone())
            .collect::<Vec<_>>();
        self.chosen.extend(arrived);
        self.error = None;
    }

    /// `contents` is the missing file `path`: keep it and walk the includes
    /// again, which may reach files it includes in turn.
    pub(crate) fn locate(&mut self, path: String, contents: String) {
        self.files.insert(path, contents);
        self.resolve();
    }

    /// The top modules that get a cell, in the order the file declares them.
    pub(crate) fn chosen_modules(&self) -> Vec<DeclaredModule> {
        self.closure
            .modules
            .iter()
            .flatten()
            .filter(|module| self.chosen.contains(&module.name))
            .cloned()
            .collect()
    }
}

/// The file `logical` under the root's `folder`, when there is one and it
/// holds a UTF-8 file within the source size limit. `logical` never climbs
/// out of the folder: the walk resolves `..` inside the bundle's own tree.
fn read_beside_root(folder: Option<&Path>, logical: &str) -> Option<String> {
    let path = folder?.join(logical);
    let metadata = std::fs::metadata(&path).ok()?;
    if !metadata.is_file() || metadata.len() > MAX_PROJECT_CODE_SOURCE_BYTES as u64 {
        return None;
    }
    String::from_utf8(std::fs::read(path).ok()?).ok()
}

/// The libraries a cell can be created in, by name.
pub fn writable_libraries(libraries: &LibraryManager) -> Vec<String> {
    libraries
        .libraries_sorted()
        .into_iter()
        .filter(|library| !library.read_only)
        .map(|library| library.name.clone())
        .collect()
}

/// Files dropped on the workbench: each root among them opens the import
/// form, with the other files and the root's folder to resolve its includes.
pub fn import_dropped_veriloga_sources(
    model: &Model,
    veriloga: &mut VerilogAWorkbenchState,
    dropped: Vec<PickedSource>,
) -> Result<(), String> {
    if model.safe_mode.project_read_only() {
        return Err("The active project is read-only.".to_owned());
    }
    if veriloga.import_requested || veriloga.pending.is_some() {
        return Err("A Verilog-A/AMS import or compile transaction is already active.".to_owned());
    }
    if let Some(file) = dropped
        .iter()
        .find(|file| file.contents.len() > MAX_PROJECT_CODE_SOURCE_BYTES)
    {
        return Err(format!(
            "{} exceeds the supported {}-byte limit.",
            file.name, MAX_PROJECT_CODE_SOURCE_BYTES
        ));
    }
    // A file dropped on an editor is a file to import, not a replacement for
    // the source under the pointer: it goes to the import form like any other.
    open_veriloga_import_forms(model, veriloga, dropped)
}

fn suggested_import_path(importer_path: &str, file_name: &str) -> String {
    importer_path.rsplit_once('/').map_or_else(
        || file_name.to_owned(),
        |(parent, _)| format!("{parent}/{file_name}"),
    )
}

pub(super) fn add_bundle_file(
    model: &mut Model,
    code: &mut CodeWorkspaceRuntimeState,
    bundle_id: ProjectSourceId,
    importer_path: &str,
    logical_path: String,
    content: String,
) -> Result<bool, String> {
    let file = ProjectSourceFile::try_new(logical_path.clone(), content)
        .map_err(|error| error.to_string())?;
    let changed = model
        .workspace
        .content
        .project_sources
        .add_bundle_file(bundle_id, importer_path, file)
        .map_err(|error| error.to_string())?;
    if changed {
        code.veriloga.selected_file = Some(VerilogAFileSelection {
            bundle_id,
            logical_path,
        });
        invalidate_veriloga_evidence_in(model, code, bundle_id);
    }
    Ok(changed)
}
