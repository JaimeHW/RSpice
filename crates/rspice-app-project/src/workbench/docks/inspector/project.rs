//! The Project workspace's inspector: the project itself while nothing on
//! the page is selected, and otherwise the selected library, cell view,
//! dependency or History row; the configuration on the Configuration page.

pub mod configuration;
mod dependency;
mod library;
mod properties;
mod recovery;

use egui::Ui;
use rspice_app_state::veriloga_library::BuiltinVerilogAModel;

use crate::state::command_catalog::CommandKey;
use crate::workbench::app::dialogs::project::ProjectCheckpointRecoveryState;
use crate::workbench::app_state::{Model, SlotFacts};
use crate::workbench::lifecycle::project_lifecycle::LifecycleView;
use crate::workbench::project::DependencyLibrary;
use crate::workbench::project::commands::ProjectRequest;
use crate::workbench::project::configuration_state::{
    ConfigurationPageState, ConfigurationSelection,
};
use crate::workbench::project::properties::ProjectPropertiesEdit;
use crate::workbench::project::revision_history::HistoryPageState;
use crate::workbench::project::workspace_state::ProjectPage;

use dependency::project_dependency;
pub use library::project_library;
use properties::project_properties;
use recovery::{history_checkpoints, history_record};

/// What the Project workspace's inspector reads and writes: the project
/// feature's page and the selections its rails keep, the configuration
/// page, the checkpoint catalog and the History page's journal rows, and
/// the model, with what the design checks
/// read of the other features' state and the project as its lifecycle reads
/// it.
pub struct ProjectInspector<'a> {
    pub page: ProjectPage,
    pub configuration: &'a mut ConfigurationPageState,
    pub dependency_selection: &'a Option<DependencyLibrary>,
    pub checkpoint_recovery: &'a ProjectCheckpointRecoveryState,
    pub history: &'a HistoryPageState,
    /// The History row selected: a checkpoint's identity, a damaged one's
    /// key or a journal record's.
    pub history_selection: &'a Option<String>,
    /// Whether the Library page shows the built-in Verilog-A library rather
    /// than one of the project's.
    pub builtin_library_open: bool,
    /// The model selected in that library.
    pub builtin_model: Option<&'static BuiltinVerilogAModel>,
    /// Which of the Library page's selected cell and view its find lists.
    pub library_shown: LibraryShown,
    pub model: &'a Model,
    pub facts: SlotFacts,
    /// Read for how many documents a save would write.
    pub lifecycle: LifecycleView<'a>,
    /// Whether the project's Name takes the keys as the inspector draws:
    /// File > Project properties… asked for it.
    pub focus_name: bool,
    /// What the deck a deck-driven project runs says of itself, where its
    /// text is at hand.
    pub deck: Option<std::sync::Arc<crate::workbench::surfaces::project::overview::DeckSummary>>,
}

/// Which of the Library page's selected cell and view its find lists. A
/// filter can hide either; the page does not act on what it hides, and the
/// inspector does not describe it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LibraryShown {
    pub cell: bool,
    pub view: bool,
}

impl LibraryShown {
    /// No filter hides anything.
    pub const ALL: Self = Self {
        cell: true,
        view: true,
    };
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
    /// Apply this edit of the project's properties.
    Properties(ProjectPropertiesEdit),
    /// Open this dependency where it is edited, as its row's Open does.
    OpenDependency(DependencyLibrary),
    /// Apply this description to a library, or with `cell` to that cell, as
    /// its properties are applied.
    LibraryDescription {
        library: String,
        cell: Option<String>,
        description: String,
    },
    /// A command over a library kept in a folder of its own: its mode, or
    /// Break lock… on one of its files.
    LibraryFile(crate::workbench::project::library_dialogs::library_files::LibraryFileIntent),
    /// Keep this note on the checkpoint, as the History page's Note… does.
    CheckpointNote(uuid::Uuid, String),
    /// Keep automatic checkpoints inside this many MiB, as the Preferences
    /// set it.
    CheckpointBudget(u32),
}

pub(in crate::workbench) fn project(
    ui: &mut Ui,
    inspector: &mut ProjectInspector<'_>,
) -> Option<ProjectInspectorAction> {
    let nothing_selected = inspects_project(
        inspector.page,
        inspector.model,
        inspector.builtin_library_open,
        inspector.dependency_selection,
        inspector.history_selection,
    );
    match inspector.page {
        ProjectPage::Overview => project_properties(ui, inspector),
        ProjectPage::Configuration => {
            configuration::project_configuration(ui, inspector.configuration, inspector.model);
            None
        }
        // Where the checkpoints are kept, and the space they are kept in,
        // stand under the project.
        ProjectPage::History if nothing_selected => {
            let action = project_properties(ui, inspector);
            let checkpoints = history_checkpoints(ui, inspector);
            action.or(checkpoints)
        }
        _ if nothing_selected => project_properties(ui, inspector),
        ProjectPage::Library => project_library(ui, inspector),
        ProjectPage::Dependencies => project_dependency(ui, inspector),
        ProjectPage::History => history_record(ui, inspector),
    }
}

/// Whether the inspector beside `page` shows the project itself: always on
/// the Overview; on the Library Manager, Dependencies and History while
/// nothing on the page is selected. The Configuration page shows its
/// configuration instead.
pub fn inspects_project(
    page: ProjectPage,
    model: &Model,
    builtin_library_open: bool,
    dependency_selection: &Option<DependencyLibrary>,
    history_selection: &Option<String>,
) -> bool {
    match page {
        ProjectPage::Overview => true,
        ProjectPage::Library => {
            !builtin_library_open && model.library_manager.current_library().is_none()
        }
        ProjectPage::Configuration => false,
        ProjectPage::Dependencies => dependency_selection.is_none(),
        ProjectPage::History => history_selection.is_none(),
    }
}
