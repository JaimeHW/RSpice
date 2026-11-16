//! New, Rename, Move, Duplicate and Delete for one file of a Verilog-A or
//! Automation bundle, each its own compact command form.
//!
//! The Files rail and Manage source document both open these. A form states
//! the file it acts on, the name it is given, and what else the change
//! rewrites — the `include a new file adds, the includes a rename updates or
//! a delete removes — and refuses before the click rather than after it: a
//! name the bundle would reject is said under the field in plain words, and
//! a file that cannot be deleted says why in the footer.
//!
//! The draft is the Code workspace's `source_file_dialog`; committing goes
//! through the same transaction every other surface uses.

#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]
#![cfg_attr(not(test), deny(clippy::panic, clippy::unreachable))]

use egui::{Context, Id};

use crate::state::project_sources::ProjectSourceError;
use crate::state::{
    ProjectSourceBundle, ProjectSourceDocument, ProjectSourceFile, ProjectSourceLanguage,
    project_source_paths_equal,
};
use crate::ui::widgets::{CommandForm, DialogChoice, FieldLine};
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::code::commands::{LifecycleAction, SourceDocumentRequest};
use crate::workbench::documents::code_workspace::{
    self, CodeSourceFileAction, CodeSourceFileDialogState, CodeWorkspaceRuntimeState,
};
use crate::workbench::{MessageCatalog, MessageId};

use super::source_document::role_label;

/// The name field every form of this family edits.
const NAME_FIELD: &str = "workbench.source-file.name";
/// The picker of the file a new file is included from.
const INCLUDED_FROM: &str = "workbench.source-file.included-from";

/// Draw the open file transaction as its command form. Returns the commit
/// for the shell to carry out; Cancel and Esc close the draft here.
pub(in crate::workbench) fn render(
    code: &mut CodeWorkspaceRuntimeState,
    model: &Model,
    platform: &PlatformState,
    messages: MessageCatalog,
    ctx: &Context,
) -> Option<SourceDocumentRequest> {
    let draft = code.source_file_dialog.clone()?;
    let bundle = model
        .workspace
        .content
        .project_sources
        .get_bundle(draft.bundle_id);
    let blocked =
        code_workspace::source_file_mutation_block_reason_in(model, code, platform, draft.language);
    let stale = bundle.is_none_or(|bundle| {
        draft.project_id != model.workspace.content.project.id()
            || bundle.revision().get() != draft.bundle_revision
            || bundle.closure_digest() != draft.closure_digest
    });
    let stale_text = messages.text(MessageId::CodeSourceFileStale);
    let veriloga = draft.language == ProjectSourceLanguage::VerilogA;
    let includers = match bundle {
        Some(bundle) if veriloga => {
            code_workspace::veriloga_includers_of(bundle, &draft.source_path)
        }
        _ => Vec::new(),
    };
    let hosts = match bundle {
        Some(bundle) if veriloga => code_workspace::veriloga_include_hosts(bundle),
        _ => Vec::new(),
    };

    let deleting = draft.action == CodeSourceFileAction::Delete;
    let refusal = bundle
        .filter(|_| deleting)
        .and_then(|bundle| delete_refusal(bundle, &draft.source_path, messages));
    let name = (!deleting).then(|| check_name(bundle, &draft, messages));

    let (title, primary) = titles(&draft, messages);
    let enter_name = messages.text(MessageId::CodeSourceFileEnterName);
    let keeps_copy = messages.text(MessageId::CodeSourceFileHistoryKeepsCopy);
    let status: Result<Option<&str>, &str> = if let Some(reason) = blocked {
        Err(reason)
    } else if stale {
        Err(&stale_text)
    } else if let Some(refusal) = refusal.as_deref() {
        Err(refusal)
    } else if let (true, Some(error)) = (deleting, draft.error.as_deref()) {
        Err(error)
    } else if deleting {
        Ok(Some(&keeps_copy))
    } else if name == Some(NameCheck::Empty) {
        Ok(Some(&enter_name))
    } else {
        Ok(None)
    };
    let ready = blocked.is_none()
        && !stale
        && refusal.is_none()
        && draft.error.is_none()
        && name.as_ref().is_none_or(|name| *name == NameCheck::Ready);

    // What the name field's line says: the commit's failure, else why the
    // name is refused, else what the change rewrites besides the file.
    let rewrites = match draft.action {
        CodeSourceFileAction::Rename | CodeSourceFileAction::Move => {
            include_extent(&includers, messages, IncludeEdit::Update)
        }
        CodeSourceFileAction::Delete => include_extent(&includers, messages, IncludeEdit::Remove),
        CodeSourceFileAction::New | CodeSourceFileAction::Duplicate => None,
    };
    let name_line = draft.error.clone().or_else(|| match &name {
        Some(NameCheck::Refused(reason)) => Some(reason.clone()),
        _ => None,
    });
    let adds = messages.format(
        MessageId::CodeSourceFileAddsInclude,
        &[("file", &draft.importer_path)],
    );

    let mut proposed = draft.proposed_path.clone();
    let mut importer = draft.importer_path.clone();
    let mut edited = false;
    let choice = CommandForm::new(&title, &primary)
        .primary_enabled(ready)
        .fields_enabled(blocked.is_none())
        .status(status)
        .show(ctx, |rows| {
            let field = Id::new(NAME_FIELD);
            let file = messages.text(MessageId::CodeSourceFileFile);
            match draft.action {
                CodeSourceFileAction::Delete => {
                    rows.value_mono(&file, &draft.source_path);
                    if let Some(rewrites) = rewrites.as_deref() {
                        rows.field_line(None, FieldLine::Derived(rewrites));
                    }
                    return None;
                }
                CodeSourceFileAction::New => {}
                CodeSourceFileAction::Rename
                | CodeSourceFileAction::Move
                | CodeSourceFileAction::Duplicate => {
                    rows.value_mono(&file, &draft.source_path);
                }
            }
            let label = messages.text(match draft.action {
                CodeSourceFileAction::New => MessageId::CodeSourceFileName,
                CodeSourceFileAction::Move => MessageId::CodeSourceFileNewPath,
                _ => MessageId::CodeSourceFileNewName,
            });
            let response = rows.text(&label, field, &mut proposed, "");
            edited |= response.changed();
            // The form keeps the whole name selected for the keyboard to land
            // on; a name, unlike a path, keeps its folder and extension.
            if draft.action != CodeSourceFileAction::Move && !response.has_focus() {
                select_stem(ctx, field, &proposed);
            }
            let line = match (name_line.as_deref(), rewrites.as_deref()) {
                (Some(error), _) => FieldLine::Error(error),
                (None, Some(rewrites)) => FieldLine::Derived(rewrites),
                (None, None) => FieldLine::Blank,
            };
            rows.field_line(Some(field), line);
            let adds_file = matches!(
                draft.action,
                CodeSourceFileAction::New | CodeSourceFileAction::Duplicate
            );
            if veriloga && adds_file {
                let label = messages.text(MessageId::CodeSourceFileIncludedFrom);
                if hosts.len() >= 2 {
                    if let Some(index) = rows.select(&label, INCLUDED_FROM, &importer, &hosts)
                        && let Some(host) = hosts.get(index)
                    {
                        edited |= *host != importer;
                        importer.clone_from(host);
                    }
                } else {
                    rows.value_mono(&label, &importer);
                }
                rows.field_line(None, FieldLine::Derived(&adds));
            }
            Some(field)
        });
    if let Some(open) = code.source_file_dialog.as_mut() {
        open.proposed_path = proposed;
        open.importer_path = importer;
        if edited {
            open.error = None;
        }
    }
    match choice {
        DialogChoice::Primary if ready => Some(SourceDocumentRequest::Document {
            action: LifecycleAction::Commit,
            language: draft.language,
            logical_path: draft.source_path,
            close: false,
        }),
        DialogChoice::Ghost | DialogChoice::Secondary | DialogChoice::Cancelled => {
            code.source_file_dialog = None;
            None
        }
        DialogChoice::Primary | DialogChoice::None => None,
    }
}

/// The form's title and its primary, by transaction.
fn titles(draft: &CodeSourceFileDialogState, messages: MessageCatalog) -> (String, String) {
    let (title, primary) = match draft.action {
        CodeSourceFileAction::New => {
            return (
                messages.format(
                    MessageId::CodeSourceFileNewTitle,
                    &[("language", draft.language.label())],
                ),
                messages.text(MessageId::CodeSourceCreatePrimary),
            );
        }
        CodeSourceFileAction::Rename => (
            MessageId::CodeSourceFileRenameTitle,
            MessageId::CodeSourceRenamePrimary,
        ),
        CodeSourceFileAction::Move => (
            MessageId::CodeSourceFileMoveTitle,
            MessageId::CodeSourceMovePrimary,
        ),
        CodeSourceFileAction::Duplicate => (
            MessageId::CodeSourceFileDuplicateTitle,
            MessageId::CodeSourceDuplicatePrimary,
        ),
        CodeSourceFileAction::Delete => (
            MessageId::CodeSourceFileDeleteTitle,
            MessageId::CodeSourceDeletePrimary,
        ),
    };
    (messages.text(title), messages.text(primary))
}

/// Why `logical_path` cannot be deleted from `bundle`, when it cannot: the
/// main file, a file a role binds, or a file that still includes another.
///
/// These are the bundle's own refusals, said before the click. The rail's
/// Delete, Manage source document's and this form's footer all read them
/// from here, so the three can never disagree.
pub(in crate::workbench) fn delete_refusal(
    bundle: &ProjectSourceBundle,
    logical_path: &str,
    messages: MessageCatalog,
) -> Option<String> {
    if project_source_paths_equal(logical_path, bundle.root().logical_path()) {
        return Some(messages.text(MessageId::CodeSourceRootDeleteBlocked));
    }
    if let Some(role) = bundle.role_for_path(logical_path) {
        return Some(messages.format(
            MessageId::CodeSourceFileRoleBound,
            &[
                ("file", logical_path),
                ("role", &role_label(messages, Some(role))),
            ],
        ));
    }
    bundle
        .dependencies()
        .iter()
        .find(|edge| project_source_paths_equal(edge.importer(), logical_path))
        .map(|edge| {
            messages.format(
                MessageId::CodeSourceFileIncludesOthers,
                &[("file", logical_path), ("dependency", edge.imported())],
            )
        })
}

#[derive(Clone, Copy)]
enum IncludeEdit {
    Update,
    Remove,
}

/// What a rename or a delete rewrites in the files that include this one,
/// when any do.
fn include_extent(
    includers: &[String],
    messages: MessageCatalog,
    edit: IncludeEdit,
) -> Option<String> {
    let (one, many) = match edit {
        IncludeEdit::Update => (
            MessageId::CodeSourceFileUpdatesInclude,
            MessageId::CodeSourceFileUpdatesIncludes,
        ),
        IncludeEdit::Remove => (
            MessageId::CodeSourceFileRemovesInclude,
            MessageId::CodeSourceFileRemovesIncludes,
        ),
    };
    match includers {
        [] => None,
        [file] => Some(messages.format(one, &[("file", file)])),
        files => Some(messages.format(many, &[("count", &files.len().to_string())])),
    }
}

/// Where the name the reader typed stands.
#[derive(Debug, Clone, PartialEq, Eq)]
enum NameCheck {
    /// Nothing typed.
    Empty,
    /// The file's own name, which renames nothing.
    Unchanged,
    /// Why the bundle would refuse it, in words.
    Refused(String),
    Ready,
}

/// Judge the draft's name as the commit would, before the commit.
fn check_name(
    bundle: Option<&ProjectSourceBundle>,
    draft: &CodeSourceFileDialogState,
    messages: MessageCatalog,
) -> NameCheck {
    let name = draft.proposed_path.trim();
    if name.is_empty() {
        return NameCheck::Empty;
    }
    let moves = matches!(
        draft.action,
        CodeSourceFileAction::Rename | CodeSourceFileAction::Move
    );
    if moves && name == draft.source_path {
        return NameCheck::Unchanged;
    }
    let refused =
        |error: ProjectSourceError| NameCheck::Refused(name_error(&error, name, messages));
    if let Err(error) = ProjectSourceFile::try_new(name, String::new()) {
        return refused(error);
    }
    let is_root = bundle.is_some_and(|bundle| {
        project_source_paths_equal(&draft.source_path, bundle.root().logical_path())
    });
    if moves
        && is_root
        && let Err(error) = ProjectSourceDocument::try_new(name, draft.language, "")
    {
        return refused(error);
    }
    // The bundle compares paths without case. Giving a file its own name in
    // another case is a rename, not a collision.
    let same_file = moves && project_source_paths_equal(name, &draft.source_path);
    if let Some(bundle) = bundle
        && !same_file
        && bundle.contains_file(name)
    {
        let existing = std::iter::once(bundle.root().logical_path())
            .chain(bundle.files().iter().map(ProjectSourceFile::logical_path))
            .find(|path| project_source_paths_equal(path, name))
            .unwrap_or(name);
        return NameCheck::Refused(
            messages.format(MessageId::CodeSourceFileExists, &[("name", existing)]),
        );
    }
    NameCheck::Ready
}

/// A refused name in the reader's words rather than the bundle's.
fn name_error(error: &ProjectSourceError, name: &str, messages: MessageCatalog) -> String {
    match error {
        ProjectSourceError::InvalidLogicalPathCharacter { .. }
        | ProjectSourceError::InvalidFileNameCharacters { .. } => {
            messages.text(MessageId::CodeSourceFileNameCharacters)
        }
        ProjectSourceError::ReservedLogicalPath { .. }
        | ProjectSourceError::ReservedFileName { .. } => {
            messages.text(MessageId::CodeSourceFileNameReserved)
        }
        ProjectSourceError::LogicalPathTooLong { limit, .. } => messages.format(
            MessageId::CodeSourceFileNameTooLong,
            &[("limit", &limit.to_string())],
        ),
        ProjectSourceError::InvalidFileNameExtension { accepted, .. } => messages.format(
            MessageId::CodeSourceFileMainExtension,
            &[("accepted", accepted)],
        ),
        ProjectSourceError::DuplicateLogicalPath { logical_path } => {
            messages.format(MessageId::CodeSourceFileExists, &[("name", logical_path)])
        }
        ProjectSourceError::InvalidLogicalPath { .. }
        | ProjectSourceError::InvalidFileNamePath { .. }
        | ProjectSourceError::InvalidFileNameWhitespace { .. } => {
            path_shape(name).map_or_else(|| error.to_string(), |shape| messages.text(shape))
        }
        other => other.to_string(),
    }
}

/// What is wrong with the shape of a path the bundle refused as malformed,
/// read off the path itself.
fn path_shape(name: &str) -> Option<MessageId> {
    if name.contains('\\') {
        return Some(MessageId::CodeSourceFileNameBackslash);
    }
    if name.starts_with('/') || name.chars().nth(1) == Some(':') {
        return Some(MessageId::CodeSourceFileNameAbsolute);
    }
    name.split('/').find_map(|part| {
        if part.is_empty() || part == "." || part == ".." {
            Some(MessageId::CodeSourceFileNameEmptyPart)
        } else if part != part.trim() {
            Some(MessageId::CodeSourceFileNameEdgeSpace)
        } else if part.ends_with('.') {
            Some(MessageId::CodeSourceFileNameTrailingPeriod)
        } else {
            None
        }
    })
}

/// The characters of `path`'s leaf stem: from after its last `/` up to the
/// leaf's last `.`, or the whole leaf when it has no `.` or starts with one.
fn stem_range(path: &str) -> (usize, usize) {
    let leaf_start = path.rfind('/').map_or(0, |slash| slash + 1);
    let end = match path[leaf_start..].rfind('.') {
        None | Some(0) => path.len(),
        Some(dot) => leaf_start + dot,
    };
    (
        path[..leaf_start].chars().count(),
        path[..end].chars().count(),
    )
}

/// Select only the stem of `value` in the field `id`, for the keyboard to
/// land on as every file manager does: typing replaces `untitled` and keeps
/// `.va`.
fn select_stem(ctx: &Context, id: Id, value: &str) {
    if value.is_empty() {
        return;
    }
    let (start, end) = stem_range(value);
    let mut state = egui::TextEdit::load_state(ctx, id).unwrap_or_default();
    state
        .cursor
        .set_char_range(Some(egui::text::CCursorRange::two(
            egui::text::CCursor::new(start),
            egui::text::CCursor::new(end),
        )));
    egui::TextEdit::store_state(ctx, id, state);
}

#[cfg(test)]
mod tests;
