//! A save that met a project file changed since this window opened it: what
//! differs, document by document, and on which side, so the reader chooses
//! knowing what each answer keeps.

use std::path::{Path, PathBuf};

use rspice_project::persistence::PersistenceError;
use rspice_project::persistence::native::DestinationAuthority;

use super::{
    LifecycleParts, LifecycleSourceMut, ProjectDocumentId, ProjectLifecycleError, open_access,
    snapshot,
};
use crate::product::ContentDigest;
use crate::state::write_policy::ProjectHolder;
use crate::workbench::lifecycle::project_checkpoint::differences_between;

/// Where a document differs from the version this window opened.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictSide {
    /// Changed in this window only.
    Here,
    /// Changed in the saved file only.
    Saved,
    /// Changed in both.
    Both,
}

impl ConflictSide {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Here => "Changed here",
            Self::Saved => "Changed in the saved file",
            Self::Both => "Changed here and in the saved file",
        }
    }
}

/// One document the save would write differently from the saved file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConflictDifference {
    pub document: ProjectDocumentId,
    pub side: ConflictSide,
}

/// The saved file a save met, and how it differs from this window's work.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaveConflict {
    pub path: PathBuf,
    pub project_name: String,
    /// Who has the project open now, when another window holds it.
    pub holder: Option<ProjectHolder>,
    pub differences: Vec<ConflictDifference>,
    /// The saved file's exact content as it was compared: "Save over it"
    /// replaces this and nothing newer.
    observed: ContentDigest,
}

impl SaveConflict {
    /// What the dialog says was found.
    pub fn sentence(&self) -> String {
        let who = self.holder.as_ref().map_or_else(String::new, |holder| {
            format!(" {} has it open.", sentence_case(&holder.display_name()))
        });
        let documents = match self.differences.len() {
            1 => "1 document".to_owned(),
            count => format!("{count} documents"),
        };
        format!(
            "\u{201c}{}\u{201d} was saved from another window after this one opened it.{who} \
             The changes here and the saved file differ in {documents}.",
            self.project_name
        )
    }
}

fn sentence_case(text: &str) -> String {
    let mut characters = text.chars();
    characters.next().map_or_else(String::new, |first| {
        first.to_uppercase().chain(characters).collect()
    })
}

/// What a failed save of the open project to its own file met, when the file
/// had changed since this window opened it; `None` for any other failure, or
/// when the saved file cannot be read to compare.
pub fn conflict_for(
    state: &LifecycleParts<'_>,
    error: &ProjectLifecycleError,
    path: &Path,
    authority: DestinationAuthority,
) -> Option<SaveConflict> {
    let ProjectLifecycleError::Persistence(PersistenceError::ExternalChange) = error else {
        return None;
    };
    if authority != DestinationAuthority::Canonical {
        return None;
    }
    prepare(state, path).ok()
}

/// Read the saved file a save to `path` met and compare it, document by
/// document, with what this window opened and with what it holds now.
fn prepare(state: &LifecycleParts<'_>, path: &Path) -> Result<SaveConflict, ProjectLifecycleError> {
    let opened = state.platform.project_lifecycle.accepted_snapshot()?;
    let here = snapshot(state)?;
    let bytes =
        crate::io::project_io::read_project_bytes(path).map_err(PersistenceError::Project)?;
    let observed = bytes.digest();
    let saved = crate::io::ProjectSnapshot::from_decoded(
        bytes
            .decode(Some(path), crate::state::workspace::WorkspaceSourceFiles)
            .map_err(PersistenceError::Project)?,
    );
    let changed_here =
        differences_between(&here, &opened).map_err(ProjectLifecycleError::InvalidState)?;
    let changed_there =
        differences_between(&saved, &opened).map_err(ProjectLifecycleError::InvalidState)?;
    let mut differences: Vec<ConflictDifference> = changed_here
        .iter()
        .map(|difference| ConflictDifference {
            document: difference.document.clone(),
            side: if changed_there
                .iter()
                .any(|there| there.document == difference.document)
            {
                ConflictSide::Both
            } else {
                ConflictSide::Here
            },
        })
        .collect();
    differences.extend(
        changed_there
            .iter()
            .filter(|there| {
                !changed_here
                    .iter()
                    .any(|here| here.document == there.document)
            })
            .map(|there| ConflictDifference {
                document: there.document.clone(),
                side: ConflictSide::Saved,
            }),
    );
    // A window holding the lease itself was let in with "Open anyway": the
    // other window's name is not on record then.
    let holder = state
        .platform
        .project_lifecycle
        .open_lease
        .is_none()
        .then(|| open_access::holder_of(path))
        .flatten();
    Ok(SaveConflict {
        path: path.to_path_buf(),
        project_name: here.file.workspace.project.name().to_owned(),
        holder,
        differences,
        observed,
    })
}

/// Make the next save of this window's project replace the saved file the
/// conflict compared, and nothing newer: its content is what the save
/// expects to find.
pub fn accept_saved_file_for_overwrite(
    state: &mut dyn LifecycleSourceMut,
    conflict: &SaveConflict,
) {
    let state = &mut state.lifecycle_parts();
    if let Some(binding) = state.platform.project_lifecycle.accepted_binding.as_mut()
        && binding.canonical_path == conflict.path
    {
        binding.accepted_digest = conflict.observed;
    }
}
