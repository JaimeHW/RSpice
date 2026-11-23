//! The project's include search paths, edited from their rows: added,
//! changed to another folder, removed and moved in the search order. Each
//! edit applies through the one path every edit of the project's properties
//! takes: one step of Undo, refused while the project is open read-only.

use std::path::{Path, PathBuf};

use crate::workbench::app_state::Model;
use crate::workbench::project::properties::{
    ProjectPropertiesEdit, apply_project_properties, include_search_path_refusal,
    project_properties_refusal,
};

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
///
/// One folder can be spelled two ways: through a link (macOS keeps its
/// temporary folders under `/var`, which is `/private/var`) or by a short
/// name on Windows. When the spellings differ, the folders' canonical forms
/// on disk decide whether one holds the other.
pub(crate) fn stored_include_path(folder: &Path, project_folder: Option<&Path>) -> PathBuf {
    let inside = project_folder.and_then(|project| {
        folder
            .strip_prefix(project)
            .map(Path::to_path_buf)
            .ok()
            .or_else(|| {
                let folder = std::fs::canonicalize(folder).ok()?;
                let project = std::fs::canonicalize(project).ok()?;
                folder.strip_prefix(project).map(Path::to_path_buf).ok()
            })
    });
    inside.map_or_else(
        || folder.to_path_buf(),
        |inside| {
            if inside.as_os_str().is_empty() {
                PathBuf::from(".")
            } else {
                inside
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
    if let Some(refusal) = project_properties_refusal(model) {
        return Err(refusal.to_owned());
    }
    let mut paths = model
        .workspace
        .content
        .project
        .properties()
        .include_search_paths;
    let listed: Vec<String> = paths
        .iter()
        .map(|path| path.display().to_string())
        .collect();
    let position = |written: &str| {
        listed
            .iter()
            .position(|listed| listed == written)
            .ok_or_else(|| format!("{written} is no longer an include search path."))
    };
    let done = match edit {
        IncludePathEdit::Add(folder) => {
            let shown = folder.display().to_string();
            if let Some(refusal) =
                include_search_path_refusal(listed.iter().map(String::as_str), &shown)
            {
                return Err(refusal);
            }
            paths.push(folder);
            format!("Added include search path {shown}.")
        }
        IncludePathEdit::Change(written, folder) => {
            let at = position(&written)?;
            let shown = folder.display().to_string();
            if shown == written {
                return Err(format!("{written} already names that folder."));
            }
            if let Some(refusal) =
                include_search_path_refusal(listed.iter().map(String::as_str), &shown)
            {
                return Err(refusal);
            }
            paths[at] = folder;
            format!("Include search path {written} is now {shown}.")
        }
        IncludePathEdit::Remove(written) => {
            paths.remove(position(&written)?);
            format!("Removed include search path {written}.")
        }
        IncludePathEdit::Move(written, up) => {
            let at = position(&written)?;
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
            format!(
                "Include search path {written} is now searched {}.",
                ordinal(to + 1)
            )
        }
    };
    apply_project_properties(
        model,
        ProjectPropertiesEdit {
            include_search_paths: Some(paths),
            ..ProjectPropertiesEdit::default()
        },
    )?;
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

#[cfg(all(test, unix))]
mod tests {
    use std::path::{Path, PathBuf};

    use super::stored_include_path;

    /// A folder picked through a link to the project folder is stored from
    /// the project folder, as one picked by the project's own spelling is.
    #[test]
    fn a_folder_spelled_through_a_link_is_stored_from_the_project_folder() {
        let made = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |since| since.as_nanos());
        let root = std::env::temp_dir().join(format!(
            "rspice-include-spelling-{}-{made}",
            std::process::id()
        ));
        let project = root.join("project");
        std::fs::create_dir_all(project.join("models")).expect("the folders are made");
        let link = root.join("link");
        std::os::unix::fs::symlink(&project, &link).expect("the link is made");

        let picked = link.join("models");
        assert_eq!(
            stored_include_path(&picked, Some(&project)),
            PathBuf::from("models")
        );
        assert_eq!(
            stored_include_path(&link, Some(&project)),
            PathBuf::from(".")
        );
        let outside = root.join("elsewhere");
        std::fs::create_dir_all(&outside).expect("the folder is made");
        assert_eq!(stored_include_path(&outside, Some(&project)), outside);
        assert_eq!(
            stored_include_path(Path::new("/nowhere/models"), Some(&project)),
            PathBuf::from("/nowhere/models")
        );
        std::fs::remove_dir_all(root).ok();
    }
}
