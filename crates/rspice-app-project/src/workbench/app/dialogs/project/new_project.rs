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
//!
//! Each field answers for itself: what it holds that Create cannot use is
//! said under it. What the file system says about the location and the
//! project's folder is asked when either changes, not on every frame.

use std::path::{Path, PathBuf};

use crate::state::workspace::DEFAULT_PROJECT_LIBRARY;
use crate::workbench::surfaces::project::AttachableTechnology;
use crate::workbench::workflows::project_workflow::{
    NewProjectParams, project_name_refusal, validate_identity_segment,
};

/// What the Location field says while it is empty.
pub const LOCATION_WAIT: &str = "Choose a folder for the project.";

/// What the Location field says of a path that does not start at a root.
pub const LOCATION_NOT_WHOLE: &str = "The location must be a full path to a folder.";

/// A field that can refuse what it holds, in the order the dialog shows them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NewProjectField {
    Name,
    /// The desktop's only: the browser asks where at the first save.
    Location,
    Library,
    TopCell,
}

impl NewProjectField {
    /// Every field, in the dialog's order.
    pub const ALL: [Self; 4] = [Self::Name, Self::Location, Self::Library, Self::TopCell];
}

/// What the file system last said about a location and a project folder,
/// each kept with the path it was asked about.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FolderChecks {
    location: Option<(PathBuf, Option<String>)>,
    folder: Option<(PathBuf, Option<String>)>,
}

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
    /// The file system's answers for the location and folder last asked
    /// about.
    pub folder_checks: FolderChecks,
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

    /// Offer the default project name, numbered past any project folder of
    /// that name already in the location, with the library following it.
    pub fn offer_free_name(&mut self) {
        let base = NewProjectParams::default().name;
        self.name.clone_from(&base);
        let mut number = 1;
        while self
            .project_file()
            .is_ok_and(|file| file.parent().is_some_and(Path::exists))
        {
            number += 1;
            self.name = format!("{base} {number}");
        }
        self.mirror_library_from_name();
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
            return Err(LOCATION_NOT_WHOLE.to_owned());
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

    /// Why `field` cannot be used as it stands, in one sentence; `None` when
    /// it can.
    pub fn refusal(&self, field: NewProjectField) -> Option<String> {
        let params = self.params();
        match field {
            NewProjectField::Name => crate::state::ProjectDescriptor::validate_name(&params.name)
                .err()
                .map(project_name_refusal),
            NewProjectField::Location => self.location_refusal(),
            NewProjectField::Library => {
                validate_identity_segment("library", &params.root_library).err()
            }
            NewProjectField::TopCell => {
                validate_identity_segment("top cell", &params.top_cell).err()
            }
        }
    }

    /// Whether Create can go ahead, or the first field's refusal in the
    /// dialog's order.
    pub fn creatable(&self) -> Result<(), String> {
        if let Some(refusal) = NewProjectField::ALL
            .into_iter()
            .find_map(|field| self.refusal(field))
        {
            return Err(refusal);
        }
        self.params().validate()
    }

    /// What the footer says: why Create cannot go ahead, or why the last
    /// create was refused. Nothing while Create can go ahead.
    pub fn footer(&self) -> Option<String> {
        self.creatable().err().or_else(|| self.error.clone())
    }

    /// Ask the file system about the location and the project's folder when
    /// either has changed since it was last asked, and keep the answers.
    /// Run once a frame, it asks nothing while neither changes.
    pub fn refresh_folder_checks(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            use crate::workbench::workflows::project_workflow::{
                new_project_folder_refusal, new_project_location_refusal,
            };
            let location = PathBuf::from(self.location.trim());
            let checks = &mut self.folder_checks;
            if location.is_absolute()
                && checks
                    .location
                    .as_ref()
                    .is_none_or(|(asked, _)| *asked != location)
            {
                let refusal = new_project_location_refusal(&location);
                checks.location = Some((location, refusal));
            }
            if let Ok(file) = self.project_file()
                && self
                    .folder_checks
                    .folder
                    .as_ref()
                    .is_none_or(|(asked, _)| *asked != file)
            {
                let refusal = new_project_folder_refusal(&file);
                self.folder_checks.folder = Some((file, refusal));
            }
        }
    }

    /// Why the location cannot take the project's folder: empty, not a whole
    /// path, a folder that is not there or cannot be written to, or one
    /// already holding a folder of the project's name with something in it.
    #[cfg(not(target_arch = "wasm32"))]
    fn location_refusal(&self) -> Option<String> {
        use crate::workbench::workflows::project_workflow::{
            new_project_folder_refusal, new_project_location_refusal,
        };
        let location = self.location.trim();
        if location.is_empty() {
            return Some(LOCATION_WAIT.to_owned());
        }
        let location = Path::new(location);
        if !location.is_absolute() {
            return Some(LOCATION_NOT_WHOLE.to_owned());
        }
        let refused = match &self.folder_checks.location {
            Some((asked, refusal)) if asked == location => refusal.clone(),
            _ => new_project_location_refusal(location),
        };
        if refused.is_some() {
            return refused;
        }
        let file = self.project_file().ok()?;
        match &self.folder_checks.folder {
            Some((asked, refusal)) if *asked == file => refusal.clone(),
            _ => new_project_folder_refusal(&file),
        }
    }

    /// The browser asks where at the first save, so it has no location.
    #[cfg(target_arch = "wasm32")]
    fn location_refusal(&self) -> Option<String> {
        None
    }

    /// The Technology field's text for the current choice.
    pub fn technology_label(&self) -> &str {
        self.technology
            .as_ref()
            .map_or(NO_TECHNOLOGY, |technology| technology.name.as_str())
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
