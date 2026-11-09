//! The undo history of the edits a command makes to a source bundle.
//!
//! Format, Rename, Replace and Replace source from file rewrite documents
//! under the editor, in as many files as they reach, so the editor's own
//! history cannot answer for them. Each is one step here. An Undo or Redo the
//! editor has no step for takes the newest step of the bundle it shows.

use crate::state::ProjectSourceId;
use crate::workbench::framework::requests::StandardCommand;
use rspice_project::ProjectWorkspace;

/// Steps kept across every bundle. The oldest falls away first.
const RETAINED_STEPS: usize = 64;

#[derive(Debug, Clone)]
struct FileEdit {
    logical_path: String,
    before: String,
    after: String,
}

#[derive(Debug, Clone)]
struct SourceEditStep {
    bundle_id: ProjectSourceId,
    files: Vec<FileEdit>,
}

#[derive(Debug, Clone, Default)]
pub struct SourceEditHistory {
    undo: Vec<SourceEditStep>,
    redo: Vec<SourceEditStep>,
}

impl SourceEditHistory {
    /// Write `replacements` to the bundle in one transaction, as one step.
    /// Returns how many files changed; none changes nothing here either.
    pub(super) fn replace(
        &mut self,
        content: &mut ProjectWorkspace,
        bundle_id: ProjectSourceId,
        replacements: Vec<(String, String)>,
    ) -> Result<usize, String> {
        let bundle = content
            .project_sources
            .get_bundle(bundle_id)
            .ok_or_else(|| "The source bundle no longer exists.".to_owned())?;
        let files = replacements
            .iter()
            .filter_map(|(logical_path, after)| {
                let before = bundle.file_content(logical_path)?;
                (before != after).then(|| FileEdit {
                    logical_path: logical_path.clone(),
                    before: before.to_owned(),
                    after: after.clone(),
                })
            })
            .collect::<Vec<_>>();
        let changed = content
            .replace_project_source_bundle_files_transactionally(bundle_id, replacements)
            .map_err(|error| error.to_string())?;
        if changed > 0 {
            self.redo.retain(|step| step.bundle_id != bundle_id);
            self.undo.push(SourceEditStep { bundle_id, files });
            if self.undo.len() > RETAINED_STEPS {
                self.undo.remove(0);
            }
        }
        Ok(changed)
    }

    /// Undo or redo the newest step of `bundle_id`. Returns whether a step
    /// was taken. A file edited since the step refuses it, and the step
    /// stays: undoing that edit makes it possible again.
    pub(crate) fn step(
        &mut self,
        content: &mut ProjectWorkspace,
        bundle_id: ProjectSourceId,
        command: StandardCommand,
    ) -> Result<bool, String> {
        let undoing = command == StandardCommand::Undo;
        let from = if undoing { &self.undo } else { &self.redo };
        let Some(index) = from.iter().rposition(|step| step.bundle_id == bundle_id) else {
            return Ok(false);
        };
        let bundle = content
            .project_sources
            .get_bundle(bundle_id)
            .ok_or_else(|| "The source bundle no longer exists.".to_owned())?;
        let step = &from[index];
        for file in &step.files {
            let expected = if undoing { &file.after } else { &file.before };
            if bundle.file_content(&file.logical_path) != Some(expected.as_str()) {
                return Err(format!(
                    "{} has changed since, so the edit across its files was left as it is.",
                    file.logical_path
                ));
            }
        }
        let replacements = step
            .files
            .iter()
            .map(|file| {
                let text = if undoing { &file.before } else { &file.after };
                (file.logical_path.clone(), text.clone())
            })
            .collect::<Vec<_>>();
        content
            .replace_project_source_bundle_files_transactionally(bundle_id, replacements)
            .map_err(|error| error.to_string())?;
        let (from, to) = if undoing {
            (&mut self.undo, &mut self.redo)
        } else {
            (&mut self.redo, &mut self.undo)
        };
        let step = from.remove(index);
        to.push(step);
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        CellViewRef, ProjectSourceBundle, ProjectSourceDependency, ProjectSourceFile,
        ProjectSourceLanguage, ProjectSourceOwner, ProjectSourceRegistry,
    };

    const ROOT: &str = "`include \"defs.vams\"\nmodule m; endmodule\n";
    const DEFS: &str = "// definitions\n";

    fn two_files() -> (ProjectWorkspace, ProjectSourceId) {
        let bundle = ProjectSourceBundle::try_new(
            ProjectSourceOwner::cell_view(CellViewRef::new("lib", "m", "veriloga")),
            ProjectSourceLanguage::VerilogA,
            "m.va",
            ROOT,
            [ProjectSourceFile::try_new("defs.vams", DEFS).unwrap()],
            [ProjectSourceDependency::try_new("m.va", "defs.vams").unwrap()],
        )
        .unwrap();
        let id = bundle.id();
        let mut workspace = ProjectWorkspace::default();
        workspace.project_sources = ProjectSourceRegistry::try_from_bundles([bundle]).unwrap();
        (workspace, id)
    }

    fn text(workspace: &ProjectWorkspace, id: ProjectSourceId, path: &str) -> String {
        workspace
            .project_sources
            .get_bundle(id)
            .unwrap()
            .file_content(path)
            .unwrap()
            .to_owned()
    }

    fn both() -> Vec<(String, String)> {
        vec![
            ("m.va".to_owned(), format!("{ROOT}// a\n")),
            ("defs.vams".to_owned(), format!("{DEFS}// b\n")),
        ]
    }

    #[test]
    fn an_edit_across_files_undoes_and_redoes_in_one_step() {
        let (mut workspace, id) = two_files();
        let mut history = SourceEditHistory::default();
        assert_eq!(history.replace(&mut workspace, id, both()).unwrap(), 2);

        assert!(
            history
                .step(&mut workspace, id, StandardCommand::Undo)
                .unwrap()
        );
        assert_eq!(text(&workspace, id, "m.va"), ROOT);
        assert_eq!(text(&workspace, id, "defs.vams"), DEFS);

        assert!(
            history
                .step(&mut workspace, id, StandardCommand::Redo)
                .unwrap()
        );
        assert_eq!(text(&workspace, id, "defs.vams"), format!("{DEFS}// b\n"));
        assert!(
            !history
                .step(&mut workspace, id, StandardCommand::Redo)
                .unwrap(),
            "nothing is left to redo"
        );
    }

    /// A file written since the step is not overwritten, and the step waits
    /// until that writing is taken back.
    #[test]
    fn a_step_over_a_file_edited_since_is_refused_and_kept() {
        let (mut workspace, id) = two_files();
        let mut history = SourceEditHistory::default();
        history.replace(&mut workspace, id, both()).unwrap();
        let renamed = text(&workspace, id, "defs.vams");
        workspace
            .replace_project_source_bundle_file(id, "defs.vams", "typed over\n".to_owned())
            .unwrap();

        let refused = history
            .step(&mut workspace, id, StandardCommand::Undo)
            .unwrap_err();
        assert!(refused.contains("defs.vams"), "{refused}");
        assert_eq!(text(&workspace, id, "m.va"), format!("{ROOT}// a\n"));

        workspace
            .replace_project_source_bundle_file(id, "defs.vams", renamed)
            .unwrap();
        assert!(
            history
                .step(&mut workspace, id, StandardCommand::Undo)
                .unwrap()
        );
        assert_eq!(text(&workspace, id, "m.va"), ROOT);
    }

    #[test]
    fn a_replacement_that_changes_nothing_is_no_step() {
        let (mut workspace, id) = two_files();
        let mut history = SourceEditHistory::default();
        let unchanged = vec![("m.va".to_owned(), ROOT.to_owned())];
        assert_eq!(history.replace(&mut workspace, id, unchanged).unwrap(), 0);
        assert!(
            !history
                .step(&mut workspace, id, StandardCommand::Undo)
                .unwrap()
        );
    }
}
