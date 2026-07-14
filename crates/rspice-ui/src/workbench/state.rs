//! Persistent and transient state for the RSpice workbench.
//!
//! This module deliberately contains no egui code.  It is the single owner
//! for navigation, dock visibility, responsive drawers, and the selection of
//! the task surface inside each canonical workspace.

use serde::{Deserialize, Serialize};

/// The seven canonical workspaces from the product contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum Workspace {
    Project,
    #[default]
    Design,
    Simulate,
    Results,
    Verify,
    Models,
    Netlist,
}

impl Workspace {
    pub const ALL: [Self; 7] = [
        Self::Project,
        Self::Design,
        Self::Simulate,
        Self::Results,
        Self::Verify,
        Self::Models,
        Self::Netlist,
    ];

    pub const PHONE_PRIMARY: [Self; 4] =
        [Self::Design, Self::Simulate, Self::Results, Self::Verify];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Project => "Project",
            Self::Design => "Design",
            Self::Simulate => "Simulate",
            Self::Results => "Results",
            Self::Verify => "Verify",
            Self::Models => "Models",
            Self::Netlist => "Netlist",
        }
    }

    pub const fn owner_label(self) -> &'static str {
        match self {
            Self::Project => "Project",
            Self::Design => "Design",
            Self::Simulate => "Simulation plan",
            Self::Results => "Result document",
            Self::Verify => "Verification evidence",
            Self::Models => "Model binding",
            Self::Netlist => "Automation pipeline",
        }
    }

    pub const fn shortcut(self) -> &'static str {
        match self {
            Self::Project => "Alt+1",
            Self::Design => "Alt+2",
            Self::Simulate => "Alt+3",
            Self::Results => "Alt+4",
            Self::Verify => "Alt+5",
            Self::Models => "Alt+6",
            Self::Netlist => "Alt+7",
        }
    }

    pub const fn navigator_title(self) -> &'static str {
        match self {
            Self::Project => "Project navigator",
            Self::Design => "Design navigator",
            Self::Simulate => "Simulation plan",
            Self::Results => "Results navigator",
            Self::Verify => "Verification navigator",
            Self::Models => "Models and libraries",
            Self::Netlist => "Code and automation",
        }
    }

    pub const fn inspector_title(self) -> &'static str {
        match self {
            Self::Project => "Project details",
            Self::Design => "Inspector",
            Self::Simulate => "Analysis inspector",
            Self::Results => "Data inspector",
            Self::Verify => "Evidence inspector",
            Self::Models => "Model inspector",
            Self::Netlist => "Source inspector",
        }
    }
}

/// Width class used by layout composition.  Breakpoints are based on task
/// density rather than an assumed device or operating system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidthClass {
    Phone,
    Tablet,
    Desktop,
    Wide,
}

impl WidthClass {
    pub const fn for_width(width: f32) -> Self {
        if width < 600.0 {
            Self::Phone
        } else if width < 960.0 {
            Self::Tablet
        } else if width < 1440.0 {
            Self::Desktop
        } else {
            Self::Wide
        }
    }

    pub const fn is_phone(self) -> bool {
        matches!(self, Self::Phone)
    }

    pub const fn uses_drawers(self) -> bool {
        matches!(self, Self::Phone | Self::Tablet)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Drawer {
    Navigator,
    Inspector,
    Workspaces,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ConsolePage {
    #[default]
    Console,
    Problems,
    Measurements,
    TaskLog,
}

/// The two Design navigator tabs specified by the workbench mockup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum DesignPanel {
    #[default]
    Navigator,
    ComponentShelf,
}

/// Sort order offered by the project launcher. The recent-files store is
/// already maintained newest-first, so `LastOpened` preserves that durable
/// ordering without fabricating timestamps that the application does not
/// persist.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProjectLauncherSort {
    #[default]
    LastOpened,
    Name,
}

impl ProjectLauncherSort {
    pub const fn label(self) -> &'static str {
        match self {
            Self::LastOpened => "Last opened",
            Self::Name => "Name",
        }
    }
}

impl ConsolePage {
    pub const ALL: [Self; 4] = [
        Self::Console,
        Self::Problems,
        Self::Measurements,
        Self::TaskLog,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Console => "Console",
            Self::Problems => "Problems",
            Self::Measurements => "Measurements",
            Self::TaskLog => "Task log",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ProjectPage {
    #[serde(alias = "Overview", alias = "Activity")]
    #[default]
    Dashboard,
    Configuration,
    Technology,
    Dependencies,
    Recovery,
}

impl ProjectPage {
    pub const ALL: [Self; 5] = [
        Self::Dashboard,
        Self::Configuration,
        Self::Technology,
        Self::Dependencies,
        Self::Recovery,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Dashboard => "Project overview",
            Self::Configuration => "Testbench configuration",
            Self::Technology => "Technology & PDK",
            Self::Dependencies => "Dependency manifest",
            Self::Recovery => "Recovery center",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum VerificationPage {
    #[default]
    Cockpit,
    Specifications,
    Checks,
    Reliability,
    History,
}

impl VerificationPage {
    pub const ALL: [Self; 5] = [
        Self::Cockpit,
        Self::Specifications,
        Self::Checks,
        Self::Reliability,
        Self::History,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Cockpit => "Verification cockpit",
            Self::Specifications => "Specification matrix",
            Self::Checks => "Design checks",
            Self::Reliability => "Reliability and SOA",
            Self::History => "Run history",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ModelsPage {
    #[default]
    Catalog,
    Libraries,
    Pdk,
    Behavioral,
    Qualification,
}

impl ModelsPage {
    pub const ALL: [Self; 5] = [
        Self::Catalog,
        Self::Libraries,
        Self::Pdk,
        Self::Behavioral,
        Self::Qualification,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Catalog => "Model catalog",
            Self::Libraries => "Libraries and cellviews",
            Self::Pdk => "PDK configuration",
            Self::Behavioral => "Verilog-A / AMS",
            Self::Qualification => "Qualification",
        }
    }
}

/// New workbench session state.  Durable layout preferences are serialized;
/// one-frame requests and open drawers are intentionally transient.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkbenchState {
    #[serde(default)]
    pub workspace: Workspace,
    #[serde(default = "default_true")]
    pub navigator_visible: bool,
    #[serde(default = "default_true")]
    pub inspector_visible: bool,
    #[serde(default = "default_true")]
    pub console_visible: bool,
    #[serde(default)]
    pub console_maximized: bool,
    #[serde(default)]
    pub focus_mode: bool,
    /// Current viewport full-screen intent. Runtime-owned because the host
    /// window, browser, or mobile shell decides whether the request can be
    /// honored and must never restore a stale platform window state.
    #[serde(skip)]
    pub full_screen: bool,
    /// The Project Launcher is an application-level modal, not a workspace.
    #[serde(skip)]
    pub project_launcher_open: bool,
    /// Search text is transient and never becomes project state.
    #[serde(skip)]
    pub project_launcher_query: String,
    #[serde(skip)]
    pub project_launcher_sort: ProjectLauncherSort,
    /// Focus is requested only on the frame in which the launcher opens.
    #[serde(skip)]
    pub focus_project_launcher_search: bool,
    #[serde(default = "default_navigator_width")]
    pub navigator_width: f32,
    #[serde(default = "default_inspector_width")]
    pub inspector_width: f32,
    #[serde(default = "default_console_height")]
    pub console_height: f32,
    #[serde(default)]
    pub console_page: ConsolePage,
    #[serde(default)]
    pub design_panel: DesignPanel,
    #[serde(default)]
    pub project_page: ProjectPage,
    #[serde(default)]
    pub verification_page: VerificationPage,
    #[serde(default)]
    pub models_page: ModelsPage,
    /// Analysis row whose configuration is shown in the simulation plan.
    #[serde(default = "default_analysis_index")]
    pub active_analysis: usize,
    /// Selected specification row in the verification matrix.
    #[serde(default)]
    pub selected_spec: Option<usize>,
    /// Selected model name within the currently selected model library.
    #[serde(default)]
    pub selected_model: Option<String>,
    /// Process corner selected by the workbench-level PVT control.
    #[serde(default = "default_corner")]
    pub corner: String,
    /// Filter local to the simulation analysis catalog.
    #[serde(default)]
    pub analysis_query: String,
    #[serde(default)]
    pub navigator_query: String,
    #[serde(default)]
    pub command_query: String,
    /// Transactional project-name editor buffer. Runtime-only so a partially
    /// entered value can never be restored as authoritative project identity.
    #[serde(skip)]
    pub project_name_draft: String,
    /// Last project metadata validation error.
    #[serde(skip)]
    pub project_name_error: Option<String>,
    #[serde(skip)]
    pub drawer: Option<Drawer>,
    #[serde(skip)]
    pub workspace_history: Vec<Workspace>,
    /// One-frame request to focus the workspace navigator filter.
    #[serde(skip)]
    pub focus_navigator_search: bool,
    /// Filter text in the component chooser.
    #[serde(skip)]
    pub placement_query: String,
    /// One-frame request to focus the component chooser filter.
    #[serde(skip)]
    pub focus_placement_search: bool,
}

const fn default_true() -> bool {
    true
}

const fn default_navigator_width() -> f32 {
    306.0
}

const fn default_inspector_width() -> f32 {
    326.0
}

const fn default_console_height() -> f32 {
    174.0
}

const fn default_analysis_index() -> usize {
    crate::common::simulation_analysis_tabs::TAB_TRANSIENT
}

fn default_corner() -> String {
    "tt".to_owned()
}

impl Default for WorkbenchState {
    fn default() -> Self {
        Self {
            workspace: Workspace::Design,
            navigator_visible: true,
            inspector_visible: true,
            console_visible: true,
            console_maximized: false,
            focus_mode: false,
            full_screen: false,
            project_launcher_open: false,
            project_launcher_query: String::new(),
            project_launcher_sort: ProjectLauncherSort::LastOpened,
            focus_project_launcher_search: false,
            navigator_width: default_navigator_width(),
            inspector_width: default_inspector_width(),
            console_height: default_console_height(),
            console_page: ConsolePage::Console,
            design_panel: DesignPanel::Navigator,
            project_page: ProjectPage::Dashboard,
            verification_page: VerificationPage::Cockpit,
            models_page: ModelsPage::Catalog,
            active_analysis: default_analysis_index(),
            selected_spec: None,
            selected_model: None,
            corner: default_corner(),
            analysis_query: String::new(),
            navigator_query: String::new(),
            command_query: String::new(),
            project_name_draft: String::new(),
            project_name_error: None,
            drawer: None,
            workspace_history: vec![Workspace::Design],
            focus_navigator_search: false,
            placement_query: String::new(),
            focus_placement_search: false,
        }
    }
}

impl WorkbenchState {
    pub fn open_project_launcher(&mut self) {
        self.project_launcher_open = true;
        self.focus_project_launcher_search = true;
    }

    pub fn activate(&mut self, workspace: Workspace) {
        if self.workspace == workspace {
            self.drawer = None;
            return;
        }
        self.workspace = workspace;
        self.drawer = None;
        self.workspace_history.retain(|entry| *entry != workspace);
        self.workspace_history.push(workspace);
        self.workspace_history.truncate(16);
    }

    pub fn cycle_workspace(&mut self, backwards: bool) {
        let current = Workspace::ALL
            .iter()
            .position(|workspace| *workspace == self.workspace)
            .unwrap_or(0);
        let next = if backwards {
            (current + Workspace::ALL.len() - 1) % Workspace::ALL.len()
        } else {
            (current + 1) % Workspace::ALL.len()
        };
        self.activate(Workspace::ALL[next]);
    }

    pub fn toggle_drawer(&mut self, drawer: Drawer) {
        self.drawer = (self.drawer != Some(drawer)).then_some(drawer);
    }

    pub fn reset_layout(&mut self) {
        self.navigator_visible = true;
        self.inspector_visible = true;
        self.console_visible = true;
        self.console_maximized = false;
        self.focus_mode = false;
        self.navigator_width = default_navigator_width();
        self.inspector_width = default_inspector_width();
        self.console_height = default_console_height();
        self.drawer = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_workspace_order_and_shortcuts_are_stable() {
        assert_eq!(Workspace::ALL.len(), 7);
        for (index, workspace) in Workspace::ALL.into_iter().enumerate() {
            assert_eq!(workspace.shortcut(), format!("Alt+{}", index + 1));
        }
    }

    #[test]
    fn responsive_width_classes_preserve_phone_and_desktop_boundaries() {
        assert_eq!(WidthClass::for_width(390.0), WidthClass::Phone);
        assert_eq!(WidthClass::for_width(834.0), WidthClass::Tablet);
        assert_eq!(WidthClass::for_width(1280.0), WidthClass::Desktop);
        assert_eq!(WidthClass::for_width(1728.0), WidthClass::Wide);
    }

    #[test]
    fn workspace_history_is_deduplicated_and_bounded() {
        let mut state = WorkbenchState::default();
        state.activate(Workspace::Results);
        state.activate(Workspace::Design);

        assert_eq!(state.workspace, Workspace::Design);
        assert_eq!(
            state.workspace_history,
            vec![Workspace::Results, Workspace::Design]
        );
    }
}
