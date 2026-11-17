//! New project — the reviewed identity, place and technology a session's
//! next project is created with.
//!
//! On the desktop, Create makes the project's own folder in the chosen
//! location and writes the project file there at once, so checkpoints and
//! autosave have somewhere to live from the first edit. In the browser
//! nothing is written until the first save, which asks where. The names are
//! checked against the same contracts the project file itself enforces, so
//! an identity reviewed here can never be refused at save time. The library
//! mirrors a slug of the project name until the reader edits it, after which
//! the two are independent — a mirror that keeps overwriting a deliberate
//! choice is worse than no mirror.

use std::path::PathBuf;

use crate::state::workspace::DEFAULT_PROJECT_LIBRARY;
use crate::workbench::surfaces::project::AttachableTechnology;
use crate::workbench::workflows::project_workflow::NewProjectParams;

/// What the footer says once everything is filled in and valid.
#[cfg(not(target_arch = "wasm32"))]
pub const NEW_PROJECT_READY: &str = "Create writes the project file now.";
/// What the footer says once everything is filled in and valid.
#[cfg(target_arch = "wasm32")]
pub const NEW_PROJECT_READY: &str = "Nothing is written until you save.";

/// What Create waits for while the location is empty.
pub const LOCATION_WAIT: &str = "Choose a folder for the project.";

/// Draft identity for one New project transaction.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NewProjectDialogState {
    pub open: bool,
    pub name: String,
    /// The folder the project's own folder is made in. Desktop only.
    pub location: String,
    pub library: String,
    /// Set the moment the reader edits Library, which retires the mirror.
    pub library_edited: bool,
    pub top_cell: String,
    /// The technologies this session can attach, as of the dialog's opening.
    pub technologies: Vec<AttachableTechnology>,
    /// The one chosen; `None` creates the project with no technology.
    pub technology: Option<AttachableTechnology>,
    /// A refusal from a create attempt, distinct from live field validation.
    pub error: Option<String>,
}

impl NewProjectDialogState {
    /// The identity the reviewed fields describe, trimmed exactly as creation
    /// will record it.
    pub fn params(&self) -> NewProjectParams {
        NewProjectParams {
            name: self.name.trim().to_owned(),
            root_library: self.library.trim().to_owned(),
            top_cell: self.top_cell.trim().to_owned(),
        }
    }

    /// Re-derive Library from the project name. A reader who has typed in
    /// Library owns it from then on: a suggestion that keeps overwriting a
    /// deliberate choice is worse than no suggestion.
    pub fn mirror_library_from_name(&mut self) {
        if !self.library_edited {
            self.library = library_slug(&self.name);
        }
    }

    /// The project file Create writes: `<location>/<stem>/<stem>.rspiceproj`,
    /// where the stem is the project name with every character a file name
    /// cannot carry on some system folded to `_`. Refuses a location that is
    /// not a whole path, since a relative one would land wherever the
    /// application happened to start.
    pub fn project_file(&self) -> Result<PathBuf, String> {
        let location = self.location.trim();
        if location.is_empty() {
            return Err(LOCATION_WAIT.to_owned());
        }
        let location = PathBuf::from(location);
        if !location.is_absolute() {
            return Err("The location must be a full path to a folder.".to_owned());
        }
        let name = self.name.trim();
        if name.is_empty() {
            return Err("Type a project name.".to_owned());
        }
        let stem = project_file_stem(name);
        Ok(location
            .join(&stem)
            .join(format!("{stem}.{PROJECT_FILE_EXTENSION}")))
    }

    /// The first field still empty, in the order the dialog shows them, as
    /// what creation waits for.
    fn missing(&self) -> Option<&'static str> {
        let params = self.params();
        if params.name.is_empty() {
            return params.missing();
        }
        if cfg!(not(target_arch = "wasm32")) && self.location.trim().is_empty() {
            return Some(LOCATION_WAIT);
        }
        params.missing()
    }

    /// Whether Create can go ahead, or what it waits for or refuses.
    pub fn creatable(&self) -> Result<(), String> {
        if let Some(wait) = self.missing() {
            return Err(wait.to_owned());
        }
        self.params().validate()?;
        #[cfg(not(target_arch = "wasm32"))]
        self.project_file()?;
        Ok(())
    }

    /// What the footer says. An empty field has refused nothing, so the
    /// footer names what Create waits for as a plain fact; a filled field the
    /// project file could not record, or a refused create, is stated as a
    /// refusal; a complete identity says what Create does.
    pub fn footer(&self) -> Result<&'static str, String> {
        if let Some(wait) = self.missing() {
            return Ok(wait);
        }
        self.creatable()?;
        match &self.error {
            Some(refusal) => Err(refusal.clone()),
            None => Ok(NEW_PROJECT_READY),
        }
    }

    /// The Technology field's text for the current choice.
    pub fn technology_label(&self) -> &str {
        self.technology
            .as_ref()
            .map_or(NO_TECHNOLOGY, |technology| technology.label.as_str())
    }
}

/// The Technology choice that attaches nothing.
pub const NO_TECHNOLOGY: &str = "None";

const PROJECT_FILE_EXTENSION: &str = "rspiceproj";

/// A project name as a file and folder name every system takes: the
/// characters Windows refuses folded to `_`, no trailing dot or space, and
/// a reserved device name given a `_` after it.
fn project_file_stem(name: &str) -> String {
    let folded: String = name
        .chars()
        .map(|character| {
            if character.is_control() || "<>:\"/\\|?*".contains(character) {
                '_'
            } else {
                character
            }
        })
        .collect();
    let trimmed = folded.trim_end_matches(['.', ' ']).trim_start();
    if trimmed.is_empty() {
        return "project".to_owned();
    }
    let device = trimmed
        .split('.')
        .next()
        .unwrap_or_default()
        .to_ascii_uppercase();
    let reserved = matches!(device.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || ((device.starts_with("COM") || device.starts_with("LPT"))
            && device.len() == 4
            && device.as_bytes()[3].is_ascii_digit());
    if reserved {
        let (device, rest) = trimmed.split_at(device.len());
        format!("{device}_{rest}")
    } else {
        trimmed.to_owned()
    }
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
