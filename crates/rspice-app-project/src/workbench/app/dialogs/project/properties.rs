//! Project properties: what the user owns of the project's descriptor, its
//! name, description, top cell and include search chain, beside where its
//! file is, its revision and when it was saved, which are read only.
//!
//! One column at the form family's reading width, with nothing to scroll.
//! Apply commits every field the reader changed as one step of the one
//! Undo, over the project as it stands then, so a field left alone keeps
//! whatever it holds.

use std::path::{Path, PathBuf};

use egui::{Context, Id};

use crate::state::workspace::ProjectProperties;
use crate::ui::widgets::{CommandForm, DialogChoice, FieldLine, ListWords};
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::framework::feature::Cx;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::health;
use crate::workbench::project::properties::{
    PROJECT_READ_ONLY, ProjectPropertiesEdit, apply_project_properties,
    include_search_path_refusal, missing_include_search_paths, project_name, saved_at,
};

/// Wide enough for the longest label, "Include search paths".
const LABEL_COLUMN: f32 = 136.0;
/// Lines the description field shows.
const DESCRIPTION_LINES: usize = 3;

fn name_id() -> Id {
    Id::new("rspice.project-properties.name")
}

fn description_id() -> Id {
    Id::new("rspice.project-properties.description")
}

fn include_id() -> Id {
    Id::new("rspice.project-properties.include")
}

/// The open dialog's draft, and what it states about the project's file.
#[derive(Debug, Clone, Default)]
pub struct ProjectPropertiesDialogState {
    /// The properties the dialog opened on; `None` while it is closed.
    opened: Option<ProjectProperties>,
    name: String,
    description: String,
    top: (String, String),
    /// The cells the project can take as its top, listed as the dialog opened.
    top_choices: Vec<(String, String)>,
    include_search_paths: Vec<String>,
    include_draft: String,
    include_refusal: Option<String>,
    /// The listed include paths no folder answers to, read when the list
    /// changes rather than on every frame.
    missing_include_paths: Vec<String>,
    /// The folder a relative include path is read from.
    data_root: Option<PathBuf>,
    location: Option<String>,
    revision: u64,
    saved: Option<String>,
    refusal: Option<String>,
}

impl ProjectPropertiesDialogState {
    pub const fn is_open(&self) -> bool {
        self.opened.is_some()
    }

    /// Open on the project as it stands.
    pub fn open(&mut self, model: &Model, platform: &PlatformState) {
        let project = &model.workspace.content.project;
        let properties = project.properties();
        *self = Self {
            name: properties.name.clone(),
            description: properties.description.clone(),
            top: (properties.root_library.clone(), properties.top_cell.clone()),
            top_choices: model.project_top_candidates(),
            include_search_paths: properties
                .include_search_paths
                .iter()
                .map(|path| path.display().to_string())
                .collect(),
            data_root: project.data_root().map(Path::to_path_buf),
            location: health::location(model, platform),
            revision: project.revision().get(),
            saved: project.path.as_deref().and_then(saved_at),
            opened: Some(properties),
            ..Self::default()
        };
        self.read_include_presence();
    }

    pub fn close(&mut self) {
        *self = Self::default();
    }

    /// The properties the fields describe, as Apply would write them, or why
    /// the project file could not hold them.
    fn draft(&self) -> Result<ProjectProperties, String> {
        Ok(ProjectProperties {
            name: project_name(&self.name)?,
            description: self.description.trim().to_owned(),
            root_library: self.top.0.clone(),
            top_cell: self.top.1.clone(),
            include_search_paths: self
                .include_search_paths
                .iter()
                .map(PathBuf::from)
                .collect(),
        })
    }

    /// Add the path the include field holds to the end of the chain.
    fn add_include_path(&mut self) {
        let path = self.include_draft.trim().to_owned();
        if let Some(refusal) =
            include_search_path_refusal(self.include_search_paths.iter().map(String::as_str), &path)
        {
            self.include_refusal = Some(refusal);
            return;
        }
        self.include_search_paths.push(path);
        self.include_draft.clear();
        self.include_refusal = None;
        self.read_include_presence();
    }

    fn remove_include_path(&mut self, index: usize) {
        if index < self.include_search_paths.len() {
            self.include_search_paths.remove(index);
            self.include_refusal = None;
            self.read_include_presence();
        }
    }

    fn read_include_presence(&mut self) {
        let authored = self
            .include_search_paths
            .iter()
            .map(PathBuf::from)
            .collect::<Vec<_>>();
        self.missing_include_paths =
            missing_include_search_paths(&authored, self.data_root.as_deref());
    }
}

impl ProjectFeature {
    /// Draw Project properties. Apply commits the changed fields and closes
    /// it; a refusal stays in the footer with the fields as typed.
    pub(in crate::workbench) fn render_project_properties_dialog(
        cx: &mut Cx<'_, Self>,
        ctx: &Context,
    ) {
        let dialog = &mut cx.state.project_properties;
        let Some(opened) = dialog.opened.clone() else {
            return;
        };
        let read_only = cx.model.safe_mode.project_read_only();
        let draft = dialog.draft();
        let changed = draft
            .as_ref()
            .is_ok_and(|draft| !ProjectPropertiesEdit::between(draft, &opened).is_empty());
        let refusal = dialog.refusal.clone();
        let status = if read_only {
            Ok(Some(PROJECT_READ_ONLY))
        } else {
            refusal.as_deref().map_or(Ok(None), Err)
        };
        let runs_from = runs_from(cx.model);
        let choice = CommandForm::new("Project properties", "Apply")
            .reading_column()
            .describe("The project's name, description, top cell and include search paths.")
            .label_column(LABEL_COLUMN)
            .primary_on_enter(false)
            .fields_enabled(!read_only)
            .primary_enabled(!read_only && changed)
            .status(status)
            .show(ctx, |rows| {
                let mut edited = rows
                    .text_sans("Name", name_id(), &mut dialog.name, "Project name")
                    .changed();
                rows.field_line(
                    Some(name_id()),
                    match &draft {
                        Err(refusal) => FieldLine::Error(refusal),
                        Ok(_) => FieldLine::Blank,
                    },
                );
                edited |= rows
                    .text_multiline(
                        "Description",
                        description_id(),
                        &mut dialog.description,
                        "What the project is for",
                        DESCRIPTION_LINES,
                    )
                    .changed();
                let labels = dialog
                    .top_choices
                    .iter()
                    .map(|(library, cell)| format!("{library}/{cell}"))
                    .collect::<Vec<_>>();
                let current = format!("{}/{}", dialog.top.0, dialog.top.1);
                if let Some(index) =
                    rows.select("Top cell", "project-properties-top", &current, &labels)
                    && let Some(top) = dialog.top_choices.get(index)
                {
                    dialog.top = top.clone();
                    edited = true;
                }
                // Wrapped, not cut: on a phone's column the cut took the
                // cell runs start at, which is the line's whole point.
                rows.derived(&runs_from);
                let entries = dialog.include_search_paths.clone();
                let edit = rows.list(
                    "Include search paths",
                    include_id(),
                    &entries,
                    &mut dialog.include_draft,
                    ListWords {
                        hint: "Folder, relative to the project folder or absolute",
                        add: "Add include search path",
                        remove: "Remove",
                    },
                );
                if edit.field.changed() {
                    dialog.include_refusal = None;
                }
                if let Some(index) = edit.remove {
                    dialog.remove_include_path(index);
                    edited = true;
                }
                if edit.add {
                    dialog.add_include_path();
                    edited = true;
                }
                let not_found = (!dialog.missing_include_paths.is_empty())
                    .then(|| format!("Not found: {}", dialog.missing_include_paths.join(", ")));
                match (&dialog.include_refusal, &not_found) {
                    (Some(refusal), _) => {
                        rows.field_line(Some(include_id()), FieldLine::Error(refusal));
                    }
                    (None, Some(not_found)) => rows.derived(not_found),
                    (None, None) => rows.derived(
                        "Searched in this order, after the folder of the file that includes.",
                    ),
                }
                rows.section("Project file");
                match &dialog.location {
                    Some(location) => rows.value("Location", location),
                    None => rows.value("Location", "Not saved"),
                }
                rows.value("Revision", &dialog.revision.to_string());
                if let Some(saved) = &dialog.saved {
                    rows.value("Saved", saved);
                }
                if edited {
                    dialog.refusal = None;
                }
                Some(name_id())
            });
        match choice {
            DialogChoice::Primary if !read_only && changed => {
                // The fields the reader changed of those the dialog opened on,
                // written over the project as it stands.
                let applied = cx.state.project_properties.draft().and_then(|draft| {
                    apply_project_properties(
                        cx.model,
                        ProjectPropertiesEdit::between(&draft, &opened),
                    )
                });
                match applied {
                    Ok(()) => cx.state.project_properties.close(),
                    Err(refusal) => cx.state.project_properties.refusal = Some(refusal),
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => cx.state.project_properties.close(),
            DialogChoice::Primary | DialogChoice::Secondary | DialogChoice::None => {}
        }
    }
}

/// What the top cell is to a run, said under its picker: runs start there
/// unless a configuration is used for runs, which names its own top.
fn runs_from(model: &Model) -> String {
    match model.workspace.content.configuration_sets.active() {
        Some(configuration) => format!(
            "Runs use configuration {}, which starts at {}.",
            configuration.name(),
            configuration.root().display_path()
        ),
        None => "Runs start here: no configuration is used for runs.".to_owned(),
    }
}
