//! The project feature's dialogs: the library, cell and view transactions, a
//! new project's identity, and the project's save, history, technology and
//! recovery workflows; the Project Launcher, the Examples dialog, and the
//! Project workspace's pages and filters.

use crate::state::ViewType;
use crate::workbench::app::dialogs::library::new_view_opening;
use crate::workbench::app::{DesignManagementDialogState, ProjectRevisionHistoryDialogState};
use crate::workbench::app_state::Model;

/// What the project feature keeps open between frames.
#[derive(Debug, Clone, Default)]
pub(crate) struct ProjectState {
    /// New Cell creation dialog
    pub new_cell_dialog: bool,
    /// New Cell name input
    pub new_cell_name: String,
    /// New Cell target library
    pub new_cell_library: String,
    /// New Cell description
    pub new_cell_description: String,
    /// New Cell view types to create
    pub new_cell_create_schematic: bool,
    /// Create symbol view for new cell
    pub new_cell_create_symbol: bool,
    /// Create testbench view for new cell
    pub new_cell_create_testbench: bool,
    /// Create a Verilog-A view, and the symbol its module generates
    pub new_cell_create_veriloga: bool,
    /// New Cell validation error message
    pub new_cell_error: Option<String>,
    /// Exact library catalog revision captured when New Cell opened.
    pub(crate) new_cell_library_revision: u64,
    /// New View creation dialog
    pub new_view_dialog: bool,
    /// New View target library
    pub new_view_library: String,
    /// New View target cell
    pub new_view_cell: String,
    /// New View name input
    pub new_view_name: String,
    /// New View type selection
    pub new_view_type: crate::state::ViewType,
    /// New View validation error message
    pub new_view_error: Option<String>,
    /// Exact library catalog revision captured when New View opened.
    pub(crate) new_view_library_revision: u64,
    /// Copy Cell dialog
    pub copy_cell_dialog: bool,
    /// Copy Cell source library
    pub copy_cell_source_library: String,
    /// Copy Cell source cell
    pub copy_cell_source_cell: String,
    /// Copy Cell target library
    pub copy_cell_target_library: String,
    /// Copy Cell new name input
    pub copy_cell_name: String,
    /// Copy Cell validation error message
    pub copy_cell_error: Option<String>,
    /// Exact library catalog revision captured when Copy Cell opened.
    pub(crate) copy_cell_library_revision: u64,
    /// Rename Cell dialog
    pub rename_cell_dialog: bool,
    /// Rename Cell target library
    pub rename_cell_library: String,
    /// Rename Cell current name
    pub rename_cell_current: String,
    /// Rename Cell new name input
    pub rename_cell_name: String,
    /// Rename Cell validation error message
    pub rename_cell_error: Option<String>,
    /// Exact library catalog revision captured when Rename Cell opened.
    pub(crate) rename_cell_library_revision: u64,
    /// Rename View dialog
    pub rename_view_dialog: bool,
    /// Rename View target library
    pub rename_view_library: String,
    /// Rename View target cell
    pub rename_view_cell: String,
    /// Rename View current name
    pub rename_view_current: String,
    /// Rename View new name input
    pub rename_view_name: String,
    /// Rename View validation error message
    pub rename_view_error: Option<String>,
    /// Exact library catalog revision captured when Rename View opened.
    pub(crate) rename_view_library_revision: u64,
    /// New Library dialog
    pub new_library_dialog: bool,
    /// New Library name input
    pub new_library_name: String,
    /// New Library validation error message
    pub new_library_error: Option<String>,
    /// Exact library catalog revision captured when New Library opened.
    pub(crate) new_library_library_revision: u64,
    /// Rename Library dialog
    pub rename_library_dialog: bool,
    /// Rename Library current name
    pub rename_library_current: String,
    /// Rename Library new name input
    pub rename_library_name: String,
    /// Rename Library validation error message
    pub rename_library_error: Option<String>,
    /// Exact library catalog revision captured when Rename Library opened.
    pub(crate) rename_library_library_revision: u64,
    /// Delete Library review
    pub delete_library_dialog: bool,
    /// Delete Library target
    pub delete_library_target: String,
    /// Delete Library blocking reason, retained from the last commit attempt
    pub delete_library_error: Option<String>,
    /// Exact library catalog revision captured when Delete Library opened.
    pub(crate) delete_library_library_revision: u64,
    /// Pending cell deletion (library, cell_name)
    pub(crate) pending_delete_cell: Option<(String, String)>,
    /// Pending view deletion (library, cell, view_name)
    pub(crate) pending_delete_view: Option<(String, String, String)>,
    /// Exact, revision-bound destructive review for deleting one cell or view.
    pub(crate) library_deletion_review:
        crate::workbench::app::dialogs::library::pending_deletions::LibraryDeletionReviewState,

    /// Reviewed identity for the project File > New is about to create.
    pub(crate) new_project:
        crate::workbench::app::dialogs::project::new_project::NewProjectDialogState,

    /// Validated active-document save revision transaction.
    pub(crate) check_and_save:
        crate::workbench::app::dialogs::check_and_save::CheckAndSaveDialogState,

    /// Project-owned sheet, variant, annotation, and hierarchy manager.
    pub(crate) design_management: DesignManagementDialogState,

    /// Hash-linked validated schematic revisions and semantic audit workflow.
    pub(crate) project_revision_history: ProjectRevisionHistoryDialogState,

    /// Project-owned technology attachment transaction.
    pub(crate) technology_attachment:
        crate::workbench::app::dialogs::project::TechnologyAttachmentDialogState,

    /// Cached, integrity-verified full-project recovery catalog.
    pub(crate) project_checkpoint_recovery:
        crate::workbench::app::dialogs::project::ProjectCheckpointRecoveryState,

    /// The Project Launcher is an application-level modal, not a workspace.
    pub project_launcher_open: bool,
    /// Search text is transient and never becomes project state.
    pub project_launcher_query: String,
    pub project_launcher_sort: crate::workbench::project::workspace_state::ProjectLauncherSort,
    pub project_launcher_filter: crate::workbench::project::workspace_state::ProjectLauncherFilter,
    /// Selected startup page and scanned native checkpoint catalog are
    /// application-modal runtime state, never project or user preferences.
    pub project_launcher_page: crate::workbench::project::workspace_state::ProjectLauncherPage,
    /// The Examples dialog, while it is open: the example chosen in it.
    pub(crate) examples_selection: Option<usize>,
    pub project_page: crate::workbench::project::workspace_state::ProjectPage,
    /// Device-local filters and row selections for the Project workspace.
    /// They affect presentation only and never participate in project
    /// revisioning, execution snapshots, or recovery checkpoints.
    pub project_library_filter: String,
    pub project_dependency_filter: String,
    pub project_dependency_selection: Option<String>,
    pub project_checkpoint_selection: Option<String>,
    /// Device-local projection retained by the Library/Cellview specialist
    /// route. Authoritative library, cellview, symbol, and form records remain
    /// in the project domains.
    pub library_cellview_page: crate::workbench::project::workspace_state::LibraryCellviewPage,
    /// The Configuration page: the configuration in front of the reader and
    /// its last refusal.
    pub(crate) configuration:
        crate::workbench::project::configuration_state::ConfigurationPageState,
    /// New configuration, Clone, Rename and Delete.
    pub(crate) configuration_dialog:
        crate::workbench::app::dialogs::project::configuration_dialogs::ConfigurationDialogState,
}

impl ProjectState {
    /// Open New View on the exact cell `library`/`cell`, against the library
    /// revision `model` holds now, preset to `view_type`, or for `None` to
    /// the first type the cell does not have yet.
    ///
    /// The target is resolved before any dialog state changes. This keeps a
    /// stale row click from opening an apparently valid transaction against a
    /// different or read-only object.
    pub(crate) fn open_new_view_dialog(
        &mut self,
        model: &Model,
        library: &str,
        cell: &str,
        view_type: Option<ViewType>,
    ) -> Result<(), String> {
        let view_type = new_view_opening(model, library, cell, view_type)?;
        self.new_view_library = library.to_owned();
        self.new_view_cell = cell.to_owned();
        self.new_view_name = view_type.display_name().to_owned();
        self.new_view_type = view_type;
        self.new_view_error = None;
        self.new_view_library_revision = model.library_manager.revision();
        self.new_view_dialog = true;
        Ok(())
    }

    /// Open the Examples dialog on its first circuit.
    pub(crate) fn open_examples(&mut self) {
        self.examples_selection = Some(0);
    }

    /// The open project is being replaced or closed: a Check and save review
    /// and the Configuration page's selection belong to the project going
    /// away.
    pub(crate) fn on_project_changed(&mut self) {
        self.check_and_save.close();
        self.configuration = Default::default();
        self.configuration_dialog = Default::default();
    }
}
