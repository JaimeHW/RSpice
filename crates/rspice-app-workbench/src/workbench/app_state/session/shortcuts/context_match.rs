//! Whether a shortcut context is live for what is in front of the reader.

use crate::state::command_catalog::ShortcutContext;
use crate::workbench::app_state::Model;
use crate::workbench::state::workbench_session::WorkbenchSession;

/// Whether a shortcut context is live, which the session answers: the context
/// is the model's vocabulary, but the workspace in front of the reader that it
/// is matched against is the session's.
pub trait ShortcutContextMatch {
    /// Whether this context is live for the workspace and document in front
    /// of the reader, as menus and the palette ask it.
    fn matches(self, workbench: &WorkbenchSession, model: &Model) -> bool;
}

impl ShortcutContextMatch for ShortcutContext {
    fn matches(self, workbench: &WorkbenchSession, model: &Model) -> bool {
        use crate::state::ViewType;
        use crate::workbench::state::Workspace;

        match self {
            Self::Global | Self::ApplicationChrome | Self::RunnableProject => true,
            // Results is here for the same reason the design canvases are:
            // the magnification and copy commands are carried out on whatever
            // the reader is looking at, and a result sheet is one of those
            // surfaces. Membership is not permission — every command in this
            // context still answers `is_enabled` for the surface in front of
            // it, and the schematic-editing family answers `false` on a
            // result sheet.
            Self::EditContext => {
                workbench.workspace == Workspace::Results
                    || match model.workspace.content.active_view_type() {
                        ViewType::Schematic | ViewType::Testbench => {
                            workbench.workspace == Workspace::Design
                        }
                        ViewType::Symbol => {
                            matches!(workbench.workspace, Workspace::Design | Workspace::Models)
                        }
                        _ => false,
                    }
            }
            Self::EngineeringCanvas => {
                workbench.workspace == Workspace::Design
                    && matches!(
                        model.workspace.content.active_view_type(),
                        ViewType::Schematic | ViewType::Testbench
                    )
            }
            Self::SymbolCanvas => {
                matches!(workbench.workspace, Workspace::Design | Workspace::Models)
                    && model.workspace.content.active_view_type() == ViewType::Symbol
            }
            Self::DesignWorkspace => workbench.workspace == Workspace::Design,
            Self::SimulationWorkspace => workbench.workspace == Workspace::Simulate,
            Self::ResultsWorkspace => workbench.workspace == Workspace::Results,
            Self::VerificationWorkspace => workbench.workspace == Workspace::Verify,
            Self::ViolationNavigation => {
                matches!(workbench.workspace, Workspace::Design | Workspace::Verify)
            }
            Self::SourceEditor => workbench.active_source_editor.is_some(),
        }
    }
}
