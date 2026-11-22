//! The project's include search paths, edited from their rows: added,
//! changed to another folder, removed and moved in the search order. Each
//! edit is one step of the one Undo over the project's properties, refused
//! while the project is open read-only.

use std::path::{Path, PathBuf};

use crate::workbench::app_state::Model;

/// One edit of the include search chain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum IncludePathEdit {
    /// A folder at the end of the chain.
    Add(PathBuf),
    /// The path as stored, and the folder it now names.
    Change(String, PathBuf),
    Remove(String),
    /// The path as stored, one place earlier in the search (`true`) or later.
    Move(String, bool),
}

/// How the project stores `folder`: from the project folder when it is
/// inside it, so a moved project keeps finding it; whole otherwise.
pub(crate) fn stored_include_path(folder: &Path, project_folder: Option<&Path>) -> PathBuf {
    project_folder
        .and_then(|project| folder.strip_prefix(project).ok())
        .map_or_else(
            || folder.to_path_buf(),
            |inside| {
                if inside.as_os_str().is_empty() {
                    PathBuf::from(".")
                } else {
                    inside.to_path_buf()
                }
            },
        )
}

/// Apply `edit` to the project's include search paths as one step of Undo,
/// and say what it did, or why it was refused.
pub(crate) fn edit_include_paths(
    model: &mut Model,
    edit: IncludePathEdit,
) -> Result<String, String> {
    let mut candidate = model.workspace.content.project.properties();
    let paths = &mut candidate.include_search_paths;
    let position = |paths: &[PathBuf], written: &str| {
        paths
            .iter()
            .position(|path| path.display().to_string() == written)
            .ok_or_else(|| format!("{written} is no longer an include search path."))
    };
    let (step, done) = match edit {
        IncludePathEdit::Add(folder) => {
            if paths.contains(&folder) {
                return Err(format!("{} is already listed.", folder.display()));
            }
            let shown = folder.display().to_string();
            paths.push(folder);
            (
                format!("add include search path {shown}"),
                format!("Added include search path {shown}."),
            )
        }
        IncludePathEdit::Change(written, folder) => {
            let at = position(paths, &written)?;
            let shown = folder.display().to_string();
            if shown == written {
                return Err(format!("{written} already names that folder."));
            }
            if paths.contains(&folder) {
                return Err(format!("{shown} is already listed."));
            }
            paths[at] = folder;
            (
                format!("change include search path {written} to {shown}"),
                format!("Include search path {written} is now {shown}."),
            )
        }
        IncludePathEdit::Remove(written) => {
            let at = position(paths, &written)?;
            paths.remove(at);
            (
                format!("remove include search path {written}"),
                format!("Removed include search path {written}."),
            )
        }
        IncludePathEdit::Move(written, up) => {
            let at = position(paths, &written)?;
            let to = if up {
                at.checked_sub(1)
            } else {
                Some(at + 1).filter(|to| *to < paths.len())
            }
            .ok_or_else(|| {
                format!(
                    "{written} is searched {} already.",
                    if up { "first" } else { "last" }
                )
            })?;
            paths.swap(at, to);
            let way = if up { "up" } else { "down" };
            (
                format!("move include search path {written} {way}"),
                format!(
                    "Include search path {written} is now searched {}.",
                    ordinal(to + 1)
                ),
            )
        }
    };
    model.publish_project_properties(candidate, step)?;
    Ok(done)
}

/// "1st", "2nd", "3rd", "4th"…, as a place in the search order.
fn ordinal(place: usize) -> String {
    let suffix = match (place % 10, place % 100) {
        (1, 11) | (2, 12) | (3, 13) => "th",
        (1, _) => "st",
        (2, _) => "nd",
        (3, _) => "rd",
        _ => "th",
    };
    format!("{place}{suffix}")
}

/// Whether the path stored as `written` can move earlier (`up`) or later in
/// the search.
pub(crate) fn can_move(model: &Model, written: &str, up: bool) -> bool {
    let paths = model.workspace.content.project.include_search_paths();
    paths
        .iter()
        .position(|path| path.display().to_string() == written)
        .is_some_and(|at| if up { at > 0 } else { at + 1 < paths.len() })
}
