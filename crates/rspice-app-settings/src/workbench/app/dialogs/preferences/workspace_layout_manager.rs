//! Workspace layout: apply a dock arrangement to this workspace or to every
//! workspace, on this device.
//!
//! It is a command form: the layout, what it shows, where it applies, Apply
//! and Cancel. It was a manager surface with no way to close it but Apply or
//! Esc, a table of remembered panel widths in pixels that labelled the Verify
//! workspace "DRC", and a scope row half hidden under the footer.

use egui::Context;

use crate::ui::widgets::{CommandForm, DialogChoice};
use crate::workbench::WorkspacePreset;

use super::PreferencesDialogs;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum LayoutTemplate {
    #[default]
    Engineering,
    Canvas,
    Diagnostics,
    ResultsReview,
}

impl LayoutTemplate {
    #[cfg(test)]
    const ALL: [Self; 4] = [
        Self::Engineering,
        Self::Canvas,
        Self::Diagnostics,
        Self::ResultsReview,
    ];

    const LABELS: [&'static str; 4] = ["Engineering", "Canvas", "Diagnostics", "Results review"];
    const DETAILS: [&'static str; 4] = [
        "Navigator, editor, inspector and console",
        "The editor alone, every dock hidden",
        "A wide console and inspector",
        "A wide plot with measurements, no navigator",
    ];

    const fn from_index(index: usize) -> Self {
        match index {
            1 => Self::Canvas,
            2 => Self::Diagnostics,
            3 => Self::ResultsReview,
            _ => Self::Engineering,
        }
    }

    const fn index(self) -> usize {
        match self {
            Self::Engineering => 0,
            Self::Canvas => 1,
            Self::Diagnostics => 2,
            Self::ResultsReview => 3,
        }
    }

    const fn workspace_preset(self) -> Option<WorkspacePreset> {
        match self {
            Self::Engineering => Some(WorkspacePreset::Engineering),
            Self::Canvas => Some(WorkspacePreset::Canvas),
            Self::Diagnostics => Some(WorkspacePreset::Diagnostics),
            Self::ResultsReview => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum LayoutScope {
    #[default]
    CurrentWorkspace,
    AllWorkspaces,
}

impl LayoutScope {
    const LABELS: [&'static str; 2] = ["This workspace", "Every workspace"];

    const fn index(self) -> usize {
        match self {
            Self::CurrentWorkspace => 0,
            Self::AllWorkspaces => 1,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct WorkspaceLayoutManagerState {
    pub open: bool,
    template: LayoutTemplate,
    scope: LayoutScope,
}

impl WorkspaceLayoutManagerState {
    pub fn open(&mut self, preset: WorkspacePreset) {
        self.open = true;
        self.template = match preset {
            WorkspacePreset::Engineering => LayoutTemplate::Engineering,
            WorkspacePreset::Canvas => LayoutTemplate::Canvas,
            WorkspacePreset::Diagnostics => LayoutTemplate::Diagnostics,
        };
        self.scope = LayoutScope::CurrentWorkspace;
    }

    pub(crate) fn close(&mut self) {
        self.open = false;
    }
}

pub(super) fn render(ctx: &Context, state: &mut PreferencesDialogs<'_>) {
    if !state.workspace_layout_manager.open {
        return;
    }

    let mut draft = state.workspace_layout_manager.clone();
    let mut template = draft.template.index();
    let scopes = LayoutScope::LABELS.map(str::to_owned);
    let choice = CommandForm::new(TITLE, "Apply")
        .describe("Apply a dock arrangement on this device. Project data is unchanged.")
        .show(ctx, |rows| {
            rows.choice("Layout", &LayoutTemplate::LABELS, &mut template);
            rows.derived(LayoutTemplate::DETAILS[template]);
            if let Some(picked) = rows.select(
                "Apply to",
                "preferences.workspace.layout-manager.scope",
                LayoutScope::LABELS[draft.scope.index()],
                &scopes,
            ) {
                draft.scope = if picked == 0 {
                    LayoutScope::CurrentWorkspace
                } else {
                    LayoutScope::AllWorkspaces
                };
            }
            None
        });
    draft.template = LayoutTemplate::from_index(template);

    match choice {
        DialogChoice::Primary => {
            apply_layout(state, draft.template, draft.scope);
            state.workspace_layout_manager.close();
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            state.workspace_layout_manager.close();
        }
        DialogChoice::Secondary | DialogChoice::None => {
            *state.workspace_layout_manager = draft;
        }
    }
}

const TITLE: &str = "Workspace layout";

fn apply_layout(state: &mut PreferencesDialogs<'_>, template: LayoutTemplate, scope: LayoutScope) {
    if let Some(preset) = template.workspace_preset() {
        match scope {
            LayoutScope::CurrentWorkspace => state.workbench.apply_workspace_preset(preset),
            LayoutScope::AllWorkspaces => state.workbench.apply_preset_to_all_workspaces(preset),
        }
        state
            .model
            .preferences
            .workspace_mut()
            .expect("the manager opens only for a compatible Workspace preference domain")
            .set_preset(preset);
    } else {
        match scope {
            LayoutScope::CurrentWorkspace => state.workbench.apply_results_review_layout(),
            LayoutScope::AllWorkspaces => state.workbench.apply_results_review_to_all_workspaces(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_and_scope_order_are_stable() {
        assert_eq!(
            LayoutTemplate::LABELS,
            ["Engineering", "Canvas", "Diagnostics", "Results review"]
        );
        for template in LayoutTemplate::ALL {
            assert_eq!(LayoutTemplate::from_index(template.index()), template);
        }
        assert_eq!(LayoutScope::LABELS, ["This workspace", "Every workspace"]);
    }
}
