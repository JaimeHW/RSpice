//! What the user owns of the project's descriptor (its name, description,
//! top cell and include search chain) and how an edit of it is applied.
//!
//! Every editor of these fields applies through here: an edit names the
//! fields it changes and is written over the project as it stands then, as
//! one step of the one Undo named by what changed, so a field the edit does
//! not name keeps whatever it holds. A project open read-only refuses every
//! edit before anything is written.

use std::path::{Path, PathBuf};

use crate::state::ProjectDescriptor;
use crate::state::workspace::ProjectProperties;
use crate::workbench::app_state::Model;
use crate::workbench::workflows::project_workflow::project_name_refusal;

/// Why no edit of the project's properties is taken while the project is
/// open read-only; also what a read-only field says on hover.
pub const PROJECT_READ_ONLY: &str = "The project is open read-only.";

/// Why the project's properties cannot be edited now, or `None` when they can.
#[must_use]
pub fn project_properties_refusal(model: &Model) -> Option<&'static str> {
    model
        .safe_mode
        .project_read_only()
        .then_some(PROJECT_READ_ONLY)
}

/// `name` as the project file holds it, without the space around it, or why
/// the file cannot hold it.
pub fn project_name(name: &str) -> Result<String, String> {
    let name = name.trim().to_owned();
    ProjectDescriptor::validate_name(&name).map_err(project_name_refusal)?;
    Ok(name)
}

/// An edit of the project's properties: the fields it changes, and no others.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProjectPropertiesEdit {
    pub name: Option<String>,
    pub description: Option<String>,
    /// The top cell, as its library and cell.
    pub top: Option<(String, String)>,
    pub include_search_paths: Option<Vec<PathBuf>>,
}

impl ProjectPropertiesEdit {
    /// The fields `draft` changes of `opened`. A description is compared as
    /// it is written, without the space around it, so opening an editor and
    /// applying never changes one by itself.
    #[must_use]
    pub fn between(draft: &ProjectProperties, opened: &ProjectProperties) -> Self {
        Self {
            name: (draft.name != opened.name).then(|| draft.name.clone()),
            description: (draft.description != opened.description.trim())
                .then(|| draft.description.clone()),
            top: ((&draft.root_library, &draft.top_cell)
                != (&opened.root_library, &opened.top_cell))
                .then(|| (draft.root_library.clone(), draft.top_cell.clone())),
            include_search_paths: (draft.include_search_paths != opened.include_search_paths)
                .then(|| draft.include_search_paths.clone()),
        }
    }

    /// Whether the edit names no field.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.name.is_none()
            && self.description.is_none()
            && self.top.is_none()
            && self.include_search_paths.is_none()
    }

    /// The step Undo and Redo name: what changed, when one thing did.
    fn step(&self) -> String {
        let mut steps = Vec::new();
        if let Some(name) = &self.name {
            steps.push(format!("rename project to {name}"));
        }
        if self.description.is_some() {
            steps.push("edit project description".to_owned());
        }
        if let Some((library, cell)) = &self.top {
            steps.push(format!("set project top to {library}/{cell}"));
        }
        if self.include_search_paths.is_some() {
            steps.push("change include search paths".to_owned());
        }
        match steps.as_slice() {
            [only] => only.clone(),
            _ => "edit project properties".to_owned(),
        }
    }
}

/// Write the fields `edit` names over the project as it stands, as one step
/// of Undo named by what changed. An edit that names nothing writes nothing;
/// a read-only project, a name the project file cannot hold or a top cell
/// runs cannot start from is refused, and the project is left as it was.
pub fn apply_project_properties(
    model: &mut Model,
    mut edit: ProjectPropertiesEdit,
) -> Result<(), String> {
    if let Some(refusal) = project_properties_refusal(model) {
        return Err(refusal.to_owned());
    }
    if edit.is_empty() {
        return Ok(());
    }
    if let Some(name) = &mut edit.name {
        *name = project_name(name)?;
    }
    if let Some(description) = &mut edit.description {
        *description = description.trim().to_owned();
    }
    let step = edit.step();
    let mut candidate = model.workspace.content.project.properties();
    if let Some(name) = edit.name {
        candidate.name = name;
    }
    if let Some(description) = edit.description {
        candidate.description = description;
    }
    if let Some((library, cell)) = edit.top {
        candidate.root_library = library;
        candidate.top_cell = cell;
    }
    if let Some(paths) = edit.include_search_paths {
        candidate.include_search_paths = paths;
    }
    model.publish_project_properties(candidate, step)
}

/// Why `path` cannot join an include search chain that already lists
/// `listed`, or `None` when it can.
pub fn include_search_path_refusal<'a>(
    mut listed: impl Iterator<Item = &'a str>,
    path: &str,
) -> Option<String> {
    let path = path.trim();
    if path.is_empty() {
        return Some("Name a folder to search.".to_owned());
    }
    listed
        .any(|listed| listed == path)
        .then(|| format!("{path} is already listed."))
}

/// Add `path` at the end of the project's include search chain, as one step
/// of Undo.
pub fn add_include_search_path(model: &mut Model, path: &str) -> Result<(), String> {
    let mut paths = model
        .workspace
        .content
        .project
        .properties()
        .include_search_paths;
    let listed = paths
        .iter()
        .map(|listed| listed.display().to_string())
        .collect::<Vec<_>>();
    if let Some(refusal) = include_search_path_refusal(listed.iter().map(String::as_str), path) {
        return Err(refusal);
    }
    paths.push(PathBuf::from(path.trim()));
    apply_project_properties(
        model,
        ProjectPropertiesEdit {
            include_search_paths: Some(paths),
            ..ProjectPropertiesEdit::default()
        },
    )
}

/// Take the `index`th path out of the project's include search chain, as one
/// step of Undo.
pub fn remove_include_search_path(model: &mut Model, index: usize) -> Result<(), String> {
    let mut paths = model
        .workspace
        .content
        .project
        .properties()
        .include_search_paths;
    if index >= paths.len() {
        return Err("That include search path is no longer listed.".to_owned());
    }
    paths.remove(index);
    apply_project_properties(
        model,
        ProjectPropertiesEdit {
            include_search_paths: Some(paths),
            ..ProjectPropertiesEdit::default()
        },
    )
}

/// The paths of `authored` no folder answers to, a relative one read from
/// `data_root`; none where the platform cannot say what is present.
#[must_use]
pub fn missing_include_search_paths(authored: &[PathBuf], data_root: Option<&Path>) -> Vec<String> {
    use rspice_simulation::netlist_preparation::IncludeSearchChain;
    if !IncludeSearchChain::states_presence() {
        return Vec::new();
    }
    IncludeSearchChain::resolve(authored, data_root)
        .entries()
        .iter()
        .filter(|entry| !entry.exists())
        .map(|entry| entry.authored().display().to_string())
        .collect()
}

/// When the project file was last written, on the reader's clock.
#[cfg(not(target_arch = "wasm32"))]
#[must_use]
pub fn saved_at(path: &Path) -> Option<String> {
    let modified = std::fs::metadata(path).ok()?.modified().ok()?;
    let unix_ms = modified
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?
        .as_millis()
        .try_into()
        .ok()?;
    let stamp = crate::time_compat::local_date_time(unix_ms)?;
    Some(format!(
        "{:04}-{:02}-{:02} {:02}:{:02}",
        stamp.year(),
        u8::from(stamp.month()),
        stamp.day(),
        stamp.hour(),
        stamp.minute()
    ))
}

/// The browser keeps a project in its own store, which has no file time.
#[cfg(target_arch = "wasm32")]
#[must_use]
pub fn saved_at(_: &Path) -> Option<String> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::project::tests::harness;

    fn paths(model: &Model) -> Vec<PathBuf> {
        model
            .workspace
            .content
            .project
            .properties()
            .include_search_paths
    }

    fn read_only(model: &mut Model) {
        model.safe_mode.active = true;
        model.safe_mode.applied.open_project_read_only = true;
    }

    /// Each edit writes the fields it names, as the project file holds them,
    /// over what the edit before it wrote.
    #[test]
    fn an_edit_writes_only_the_fields_it_names_over_the_project_as_it_stands() {
        let mut harness = harness();
        harness.open_project_session();
        let model = &mut harness.model;
        let before = model.workspace.content.project.properties();
        apply_project_properties(
            model,
            ProjectPropertiesEdit {
                description: Some("  Bias for the converter tests ".to_owned()),
                ..ProjectPropertiesEdit::default()
            },
        )
        .unwrap();
        apply_project_properties(
            model,
            ProjectPropertiesEdit {
                name: Some(" Bandgap reference ".to_owned()),
                ..ProjectPropertiesEdit::default()
            },
        )
        .unwrap();
        let after = model.workspace.content.project.properties();
        assert_eq!(after.name, "Bandgap reference");
        assert_eq!(after.description, "Bias for the converter tests");
        assert_eq!(
            (after.root_library, after.top_cell),
            (before.root_library, before.top_cell)
        );
    }

    /// A read-only project refuses an edit, and an include path added or
    /// taken out, with the words a read-only field says, and keeps its name.
    #[test]
    fn a_read_only_project_refuses_every_edit() {
        let mut harness = harness();
        harness.open_project_session();
        let model = &mut harness.model;
        let name = model.workspace.content.project.properties().name;
        read_only(model);
        let refused = Err(PROJECT_READ_ONLY.to_owned());
        let rename = ProjectPropertiesEdit {
            name: Some("Renamed".to_owned()),
            ..ProjectPropertiesEdit::default()
        };
        assert_eq!(apply_project_properties(model, rename), refused);
        assert_eq!(add_include_search_path(model, "models"), refused);
        assert_eq!(model.workspace.content.project.properties().name, name);
    }

    /// A path joins the chain once, at its end, and leaves by its place.
    #[test]
    fn an_include_search_path_is_listed_once_and_removed_by_its_place() {
        let mut harness = harness();
        harness.open_project_session();
        let model = &mut harness.model;
        add_include_search_path(model, "models").unwrap();
        add_include_search_path(model, " vendor ").unwrap();
        assert_eq!(
            add_include_search_path(model, "models"),
            Err("models is already listed.".to_owned())
        );
        assert_eq!(
            paths(model),
            [PathBuf::from("models"), PathBuf::from("vendor")]
        );
        remove_include_search_path(model, 0).unwrap();
        assert_eq!(paths(model), [PathBuf::from("vendor")]);
        assert!(remove_include_search_path(model, 1).is_err());
    }
}
