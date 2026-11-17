//! The Project workspace's inspector: the overview, library, configuration,
//! dependency and recovery pages.

pub mod configuration;
mod dependency;
mod library;
mod overview;
mod recovery;

use egui::Ui;
use rspice_app_state::veriloga_library::BuiltinVerilogAModel;

use crate::state::command_catalog::CommandKey;
use crate::ui::widgets::Button;
use crate::workbench::app::dialogs::project::ProjectCheckpointRecoveryState;
use crate::workbench::app_state::{Model, SlotFacts};
use crate::workbench::lifecycle::project_lifecycle::LifecycleView;
use crate::workbench::project::DependencyLibrary;
use crate::workbench::project::commands::ProjectRequest;
use crate::workbench::project::configuration_state::{
    ConfigurationPageState, ConfigurationSelection,
};
use crate::workbench::project::workspace_state::ProjectPage;

use dependency::project_dependency;
pub use library::project_library;
use overview::project_overview;
use recovery::project_recovery;

/// What the Project workspace's inspector reads and writes: the project
/// feature's page and the selections its rails keep, the configuration
/// page, the recovery catalog and the model, with what the design checks
/// read of the other features' state and the project as its lifecycle reads
/// it.
pub struct ProjectInspector<'a> {
    pub page: ProjectPage,
    pub configuration: &'a mut ConfigurationPageState,
    pub dependency_selection: &'a Option<DependencyLibrary>,
    pub checkpoint_recovery: &'a ProjectCheckpointRecoveryState,
    pub checkpoint_selection: &'a Option<String>,
    /// Whether the Library page shows the built-in Verilog-A library rather
    /// than one of the project's.
    pub builtin_library_open: bool,
    /// The model selected in that library.
    pub builtin_model: Option<&'static BuiltinVerilogAModel>,
    pub model: &'a Model,
    pub facts: SlotFacts,
    /// Read for how many documents a save would write.
    pub lifecycle: LifecycleView<'a>,
}

/// What the reader asked of the inspector this frame, raised once it has
/// drawn.
pub enum ProjectInspectorAction {
    /// Run the command this key names.
    Execute(CommandKey),
    /// Use this configuration for runs. The feature commits it as the
    /// inspector returns, and its refusal is stated on the Configuration
    /// page.
    UseForRuns(ConfigurationSelection),
    /// Hand the shell what only it carries out.
    Shell(ProjectRequest),
    /// Open this dependency where it is edited, as its row's Open does.
    OpenDependency(DependencyLibrary),
}

pub(in crate::workbench) fn project(
    ui: &mut Ui,
    inspector: &mut ProjectInspector<'_>,
) -> Option<ProjectInspectorAction> {
    match inspector.page {
        ProjectPage::Overview => project_overview(ui, inspector),
        ProjectPage::Library => project_library(ui, inspector),
        ProjectPage::Configuration => {
            configuration::project_configuration(ui, inspector.configuration, inspector.model)
                .map(ProjectInspectorAction::UseForRuns)
        }
        ProjectPage::Dependencies => project_dependency(ui, inspector),
        ProjectPage::Recovery => {
            project_recovery(ui, inspector);
            None
        }
    }
}

fn inspector_action(ui: &mut Ui, label: &str) -> bool {
    inspector_action_enabled(ui, label, true)
}

/// One of the inspector's actions: the kit's button, as the page beside it
/// draws its own, across the inspector's width.
fn inspector_action_enabled(ui: &mut Ui, label: &str, enabled: bool) -> bool {
    let width = ui.available_width().max(1.0);
    let clicked = Button::new(label)
        .enabled(enabled)
        .min_width(width)
        .max_width(width)
        .show(ui)
        .clicked();
    ui.add_space(4.0);
    clicked
}
