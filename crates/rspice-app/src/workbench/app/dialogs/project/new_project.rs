//! New project — the reviewed identity a session's next project is created
//! under.
//!
//! Creating a project writes nothing to disk, so the only decision this
//! transaction carries is identity: the project name, the editable design
//! library, and the top cell. All three are checked against the same contracts
//! the project file itself enforces, so an identity reviewed here can never be
//! refused at save time. The library mirrors a slug of the project name until
//! the reader edits it, after which the two are independent — a mirror that
//! keeps overwriting a deliberate choice is worse than no mirror.

use crate::state::workspace::{DEFAULT_PROJECT_LIBRARY, DEFAULT_TOP_CELL};
use crate::workbench::app_state::AppState;
use crate::workbench::workflows::project_workflow::NewProjectParams;

/// Draft identity for one New project transaction.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct NewProjectDialogState {
    pub(crate) open: bool,
    pub(crate) name: String,
    pub(crate) library: String,
    /// Set the moment the reader edits Library, which retires the mirror.
    pub(crate) library_edited: bool,
    pub(crate) top_cell: String,
    /// A refusal from a create attempt, distinct from live field validation.
    pub(crate) error: Option<String>,
}

impl NewProjectDialogState {
    /// The identity the reviewed fields describe, trimmed exactly as creation
    /// will record it.
    pub(crate) fn params(&self) -> NewProjectParams {
        NewProjectParams {
            name: self.name.trim().to_owned(),
            root_library: self.library.trim().to_owned(),
            top_cell: self.top_cell.trim().to_owned(),
        }
    }

    /// Re-derive Library from the project name. A reader who has typed in
    /// Library owns it from then on: a suggestion that keeps overwriting a
    /// deliberate choice is worse than no suggestion.
    pub(crate) fn mirror_library_from_name(&mut self) {
        if !self.library_edited {
            self.library = library_slug(&self.name);
        }
    }

    /// What the footer says. An empty field has refused nothing, so the
    /// footer names what Create waits for as a plain fact; a filled field the
    /// project file could not record, or a refused create, is stated as a
    /// refusal; a complete identity gets the reminder.
    pub(crate) fn footer(&self) -> Result<&'static str, String> {
        let params = self.params();
        if let Some(wait) = params.missing() {
            return Ok(wait);
        }
        params.validate()?;
        match &self.error {
            Some(refusal) => Err(refusal.clone()),
            None => Ok("Nothing is written to disk until you save."),
        }
    }
}

/// Open the transaction with the default library and top cell. The project
/// name starts empty because it is the one field with no defensible default:
/// requiring it is the point of the dialog.
pub(crate) fn open_new_project_dialog(state: &mut AppState) {
    state.project.new_project = NewProjectDialogState {
        open: true,
        library: DEFAULT_PROJECT_LIBRARY.to_owned(),
        top_cell: DEFAULT_TOP_CELL.to_owned(),
        ..NewProjectDialogState::default()
    };
}

/// Mirror a project name into a library-name segment: lowercase, every
/// character the persisted key format rejects folded to `_`, runs collapsed,
/// and no leading or trailing separator. A name with nothing usable in it
/// leaves the default library rather than an empty segment.
fn library_slug(name: &str) -> String {
    let mut slug = String::with_capacity(name.len());
    for character in name.chars() {
        if character.is_alphanumeric() || character == '_' {
            slug.extend(character.to_lowercase());
        } else if !slug.ends_with('_') && !slug.is_empty() {
            slug.push('_');
        }
    }
    let trimmed = slug.trim_matches('_');
    if trimmed.is_empty() {
        DEFAULT_PROJECT_LIBRARY.to_owned()
    } else {
        trimmed.to_owned()
    }
}

#[cfg(test)]
mod tests;
