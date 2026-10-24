//! The shell's host: which owner draws at each contribution point, and what
//! the containers draw around the points that depend on an owner's state.
//!
//! The containers name no owner; this table does. Each point resolves to its
//! owner's function in the order the arms are listed and on the conditions
//! they test, the first arm that matches winning, so an arm listed earlier
//! takes a point from one listed later. The owner draws in the `Ui` the
//! container handed over.
//!
//! The containers hold the commands they offer by key. Here, and only here,
//! a key becomes the command it names, which then answers as it always has.

mod menus;
mod palette_icons;
mod status_order;

use egui::Ui;
use egui::os::OperatingSystem;

use crate::state::ViewType;
use crate::state::command_catalog::{CommandKey, CommandPlatform};
use crate::ui::icons::Icon;
use crate::workbench::app_state::Model;
use crate::workbench::chrome::document_bar::WorkspaceDocument;
use crate::workbench::chrome::title_bar::application_menus::{self, ApplicationMenu};
use crate::workbench::chrome::title_bar::menu_entries::{MenuEntry, menu_keys};
use crate::workbench::commands::vocabulary::{
    Command, CommandRegistry, command_registry, legacy_command,
};
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::framework::command_registry::CommandAvailability;
use crate::workbench::framework::command_registry::{Registry, ids};
use crate::workbench::framework::host::{
    DocumentHost, Host, InspectorScope, NavigatorSection, Point, RailEntry, StatusCell,
    ToolbarScope, ToolbarSlot,
};
use crate::workbench::framework::workbench_host::WorkbenchHost;
use crate::workbench::layout::LayoutSpec;
use crate::workbench::state::{
    ProjectPage, VerificationPage, WorkbenchSession, Workspace, WorkspaceDocumentId,
};
use crate::workbench::{AppState, MessageId, RSpiceApp};
use menus::{COMPACT_MENU, SOURCE_EDITOR_MENU};

/// A route whose surface brings its own toolbar, in place of its
/// workspace's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SurfaceOwner {
    ReportAuthoring,
    ModelEditor,
}

/// The commands the registry leaves out that a container still holds by key:
/// the report route's own toolbar actions. No menu, shortcut profile or
/// Repeat reaches them.
const UNLISTED: [Command; 3] = [
    Command::SaveReportDocument,
    Command::AddReportPage,
    Command::ReportPageProperties,
];

/// The command a container's `key` names: a registered one, found by key, or
/// one of the few the registry leaves out.
fn command(key: CommandKey) -> Option<Command> {
    legacy_command(key).or_else(|| UNLISTED.into_iter().find(|command| command.key() == key))
}

impl Host for RSpiceApp {
    type WorkspaceKey = Workspace;
    type SurfaceKey = SurfaceOwner;
    type Layout = LayoutSpec;
    type StatusFrame = status_order::StatusReadings;
    type Menu = ApplicationMenu;
    type MenuEntry = MenuEntry;

    fn contribute(&mut self, point: Point<Workspace, SurfaceOwner, LayoutSpec>, ui: &mut Ui) {
        let app = self;
        match point {
            // The stage: a route that opens a surface of its own takes it
            // from the workspace's page.
            Point::Surface(workspace) => {
                use crate::workbench::surfaces::{
                    automation, design, library_cellview_manager, model_correlation, model_editor,
                    models, netlist, pdk_technology_admin, project, report_authoring, results,
                    simulate, stimulus, verify,
                };

                if app.state.workbench.current_route().surface_id()
                    == super::SurfaceId::ReportAuthoring
                {
                    report_authoring::show(ui, app);
                    return;
                }
                if app.state.workbench.current_route().surface_id() == super::SurfaceId::ModelEditor
                {
                    model_editor::show(ui, app);
                    return;
                }
                if app.state.workbench.current_route().surface_id()
                    == super::SurfaceId::ModelCorrelation
                {
                    model_correlation::show(ui, app);
                    return;
                }
                if app.state.workbench.current_route().surface_id()
                    == super::SurfaceId::PdkTechnologyAdmin
                {
                    pdk_technology_admin::show(ui, app);
                    return;
                }
                if app.state.workbench.current_route().surface_id()
                    == super::SurfaceId::LibraryCellviewManager
                {
                    library_cellview_manager::show(ui, app);
                    return;
                }
                match workspace {
                    Workspace::Project => project::show(ui, app),
                    Workspace::Design => design::show(ui, app),
                    Workspace::Simulate => simulate::show(ui, app),
                    Workspace::Stimulus => stimulus::show(ui, &mut app.state),
                    Workspace::Results => results::show(ui, app),
                    Workspace::Verify => verify::show(ui, app),
                    Workspace::Models => models::show(ui, app),
                    Workspace::Netlist => {
                        netlist::prepare_workspace(app);
                        match app.state.code.code_workspace.page {
                            crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist => {
                                netlist::show_prepared(ui, app)
                            }
                            crate::workbench::documents::code_workspace::CodeWorkspacePage::Automation => {
                                automation::show(ui, app)
                            }
                        }
                    }
                }
            }
            Point::SplitResults => {
                crate::workbench::documents::result_document::show_compact_split(ui, app);
            }
            // A workspace's navigator, below the heading and the filter field.
            Point::Navigator(workspace) => {
                use crate::workbench::docks::navigator::models::models;
                use crate::workbench::docks::navigator::netlist::{code_workspace_pages, netlist};
                use crate::workbench::docks::navigator::project::project;
                use crate::workbench::docks::navigator::results::results;
                use crate::workbench::docks::navigator::simulate::simulate;
                use crate::workbench::docks::navigator::verify::verify;
                use crate::workbench::docks::navigator::{
                    design, source_bundle, stimulus, symbol, veriloga,
                };

                match workspace {
                    // The page tabs, then the page's outline. The outline
                    // belongs to the page: an automation pipeline and a SPICE
                    // deck are different objects and read differently, so
                    // neither borrows the other's tree.
                    Workspace::Netlist => {
                        use crate::workbench::documents::code_workspace::CodeWorkspacePage;

                        code_workspace_pages(ui, app);
                        match app.state.code.code_workspace.page {
                            CodeWorkspacePage::Netlist => netlist(ui, app),
                            CodeWorkspacePage::Automation => source_bundle::automation(ui, app),
                        }
                    }
                    // A symbol cellview is a design document with its own
                    // structure: an ordered pin contract instead of a
                    // hierarchy of instances.
                    Workspace::Design
                        if app.state.model.workspace.content.active_view_type()
                            == ViewType::Symbol =>
                    {
                        symbol::show(ui, app);
                    }
                    Workspace::Design
                        if app.state.model.workspace.content.active_view_type()
                            == ViewType::VerilogA =>
                    {
                        veriloga::show(ui, app);
                    }
                    Workspace::Design => design::show(ui, app),
                    Workspace::Results => results(ui, app),
                    Workspace::Project => project(ui, app),
                    Workspace::Simulate => simulate(ui, app),
                    Workspace::Stimulus => stimulus::show(ui, &mut app.state),
                    Workspace::Verify => verify(ui, app),
                    Workspace::Models => models(ui, app),
                }
            }
            Point::Inspector(InspectorScope::AnyWorkspace) => {
                crate::workbench::docks::inspector::results::results(ui, app);
            }
            // A workspace's own inspector section.
            Point::Inspector(InspectorScope::Workspace(workspace)) => {
                use crate::workbench::docks::inspector::code_workspace::code_workspace_inspector;
                use crate::workbench::docks::inspector::project::project;
                use crate::workbench::docks::inspector::results::results;
                use crate::workbench::docks::inspector::verify::verify;
                use crate::workbench::docks::inspector::{
                    design, models, simulation, stimulus, symbol, veriloga,
                };

                match workspace {
                    Workspace::Project => project(ui, app),
                    // A symbol cellview is edited against its pin contract,
                    // not against a schematic selection.
                    Workspace::Design
                        if app.state.model.workspace.content.active_view_type()
                            == crate::state::ViewType::Symbol =>
                    {
                        symbol::show(ui, &mut app.state);
                    }
                    // A Verilog-A module is inspected for what it compiles
                    // to, not for a schematic selection it does not have.
                    Workspace::Design
                        if app.state.model.workspace.content.active_view_type()
                            == crate::state::ViewType::VerilogA =>
                    {
                        veriloga::show(ui, app);
                    }
                    Workspace::Design => design::show(ui, app),
                    Workspace::Simulate => simulation::simulate(ui, app),
                    Workspace::Stimulus => stimulus::show(ui, &mut app.state),
                    Workspace::Results => results(ui, app),
                    Workspace::Verify => verify(ui, app),
                    Workspace::Models => models::show(ui, app),
                    Workspace::Netlist => code_workspace_inspector(ui, app),
                }
            }
            // The toolbar's lane: the tools of whoever its scope names.
            Point::Toolbar(ToolbarSlot::Tools(scope), layout) => {
                use crate::workbench::chrome::toolbar::code::{netlist_tools, veriloga_tools};
                use crate::workbench::chrome::toolbar::design::{
                    design_tools, stimulus_tools, symbol_tools,
                };
                use crate::workbench::chrome::toolbar::hardcopy::report_authoring_tools;
                use crate::workbench::chrome::toolbar::models::{model_editor_tools, models_tools};
                use crate::workbench::chrome::toolbar::project::{no_project_tools, project_tools};
                use crate::workbench::chrome::toolbar::results::results_tools;
                use crate::workbench::chrome::toolbar::simulate::{
                    simulation_tools, verification_tools,
                };

                match scope {
                    ToolbarScope::NoProject => no_project_tools(ui, app, layout),
                    ToolbarScope::Surface(SurfaceOwner::ReportAuthoring) => {
                        report_authoring_tools(ui, app, layout);
                    }
                    ToolbarScope::Surface(SurfaceOwner::ModelEditor) => {
                        model_editor_tools(ui, app, layout);
                    }
                    ToolbarScope::Workspace(workspace) => match workspace {
                        Workspace::Project => project_tools(ui, app, layout),
                        Workspace::Design
                            if app.state.model.workspace.content.active_view_type()
                                == ViewType::Symbol =>
                        {
                            symbol_tools(ui, app, layout);
                        }
                        Workspace::Design
                            if app.state.model.workspace.content.active_view_type()
                                == ViewType::VerilogA =>
                        {
                            veriloga_tools(ui, app, layout);
                        }
                        Workspace::Design => design_tools(ui, app, layout),
                        Workspace::Simulate => simulation_tools(ui, app, layout),
                        Workspace::Stimulus => stimulus_tools(ui, app, layout),
                        Workspace::Results => results_tools(ui, app, layout),
                        Workspace::Verify => verification_tools(ui, app, layout),
                        Workspace::Models => models_tools(ui, app, layout),
                        Workspace::Netlist => netlist_tools(ui, app, layout),
                    },
                }
            }
            // The trailing group: the run controls, then the plan chip and the
            // PVT selector where the layout has room for them.
            Point::Toolbar(ToolbarSlot::Trailing, layout) => {
                use crate::workbench::chrome::toolbar::run_config_chip::run_config_selector;
                use crate::workbench::chrome::toolbar::simulate::{pvt_selector, run_controls};

                run_controls(ui, app, layout);
                if layout.show_run_config_selector {
                    run_config_selector(ui, app, layout.run_control_height);
                }
                if layout.show_pvt_selector {
                    pvt_selector(ui, app, layout.toolbar_control_height);
                }
            }
            // The live session's presence: the cluster reads the application
            // and hands back what the reader asked of it, which runs here.
            Point::Presence { large_targets } => {
                use crate::workbench::chrome::presence::{
                    PresenceAction, session_presence_cluster,
                };

                for action in session_presence_cluster(ui, app, large_targets) {
                    match action {
                        PresenceAction::OpenDialog => app.open_live_session_dialog(),
                        PresenceAction::EndSession => app.cloud_account.end_live_session(),
                        PresenceAction::LeaveSession => {
                            app.cloud_account.leave_live_session();
                        }
                        PresenceAction::RequestLease(doc) => {
                            app.live_session.request_lease(&doc);
                        }
                        PresenceAction::ReleaseLease(doc) => {
                            app.live_session.release_lease(&doc);
                        }
                    }
                }
            }
        }
    }

    fn navigator_section(&self, workspace: Workspace) -> NavigatorSection {
        let app = self;
        let title = match workspace {
            Workspace::Project => match app.state.project.project_page {
                ProjectPage::Overview | ProjectPage::Library => "Project",
                ProjectPage::Configuration => "Configurations",
                ProjectPage::Dependencies => "Dependencies",
                ProjectPage::Recovery => "Recovery",
            },
            Workspace::Design => "Design navigator",
            Workspace::Simulate => "Simulation setup",
            Workspace::Stimulus => "Library browser",
            Workspace::Results => "Data browser",
            Workspace::Verify => "Verification flows",
            Workspace::Models => "Library browser",
            Workspace::Netlist => {
                use crate::workbench::documents::code_workspace::CodeWorkspacePage;
                match app.state.code.code_workspace.page {
                    CodeWorkspacePage::Netlist => "Netlist outline",
                    CodeWorkspacePage::Automation => "Scripts",
                }
            }
        };
        let placeholder = match workspace {
            Workspace::Project => match app.state.project.project_page {
                ProjectPage::Overview | ProjectPage::Library => "Filter libraries, cells, views…",
                ProjectPage::Configuration => "Filter configurations…",
                ProjectPage::Dependencies => "Filter dependencies…",
                ProjectPage::Recovery => "Filter checkpoints…",
            },
            Workspace::Design => "Find instance, net or port…",
            Workspace::Simulate => "Filter setup…",
            Workspace::Stimulus => "Search definitions…",
            Workspace::Results => "Find signal, expression or run…",
            Workspace::Verify => "Filter flows, specs or samples…",
            Workspace::Models => "Search model, device or library…",
            Workspace::Netlist => "Find symbol or line…",
        };
        NavigatorSection {
            title,
            placeholder,
            // The design navigator draws its search inside its own band. The
            // data browser leads with its tab band and owns the search below
            // it, because the query and the kind/sort facets filter the tab
            // that is showing — a search above the tabs would read as
            // filtering all three at once.
            container_draws_search: !matches!(workspace, Workspace::Design | Workspace::Results),
        }
    }

    fn inspector_scope(&self, workspace: Workspace) -> InspectorScope<Workspace> {
        if split_selected_trace_is_inspected(self) {
            InspectorScope::AnyWorkspace
        } else {
            InspectorScope::Workspace(workspace)
        }
    }

    fn inspector_title(&self, scope: InspectorScope<Workspace>) -> String {
        let app = self;
        let InspectorScope::Workspace(workspace) = scope else {
            return "Result details".to_owned();
        };
        let messages = app.state.workbench.messages();
        match workspace {
            Workspace::Project => match app.state.project.project_page {
                ProjectPage::Overview => "Project overview".to_owned(),
                ProjectPage::Library => "Cell view".to_owned(),
                ProjectPage::Configuration => "Configuration".to_owned(),
                ProjectPage::Dependencies => "Dependency".to_owned(),
                ProjectPage::Recovery => "Checkpoint".to_owned(),
            },
            Workspace::Verify
                if app.state.simulate.verification_page == VerificationPage::Yield =>
            {
                "Yield details".to_owned()
            }
            Workspace::Verify => app.state.simulate.verification_page.label().to_owned(),
            Workspace::Netlist => match app.state.code.code_workspace.page {
                crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist => {
                    messages.text(MessageId::CodeInspectorNetlistTitle)
                }
                crate::workbench::documents::code_workspace::CodeWorkspacePage::Automation => {
                    messages.text(MessageId::CodeInspectorAutomationTitle)
                }
            },
            // A result sheet's context panel is named by the sheet it is
            // reading, the way every other workspace names its subject.
            Workspace::Results => {
                format!(
                    "{} details",
                    app.state.results.results.session.viewer.tab_label()
                )
            }
            _ => workspace.inspector_title().to_owned(),
        }
    }

    fn toolbar_scope(&self, workspace: Workspace) -> ToolbarScope<Workspace, SurfaceOwner> {
        let app = self;
        if !app.state.platform.project_lifecycle.is_open() {
            return ToolbarScope::NoProject;
        }
        if app.state.workbench.current_route().surface_id() == super::SurfaceId::ReportAuthoring {
            return ToolbarScope::Surface(SurfaceOwner::ReportAuthoring);
        }
        if app.state.workbench.current_route().surface_id() == super::SurfaceId::ModelEditor {
            return ToolbarScope::Surface(SurfaceOwner::ModelEditor);
        }
        ToolbarScope::Workspace(workspace)
    }

    fn title_cell(&self, workspace: Workspace) -> String {
        use crate::workbench::chrome::title_bar::code::code_title_cell;
        use crate::workbench::chrome::title_bar::design::design_title_cell;

        let app = self;
        match workspace {
            Workspace::Project => "Project overview".to_owned(),
            Workspace::Design => design_title_cell(&app.state),
            Workspace::Simulate => "Simulation plan".to_owned(),
            Workspace::Stimulus => "Stimulus Library".to_owned(),
            Workspace::Results => "Results".to_owned(),
            Workspace::Verify => "Verification".to_owned(),
            Workspace::Models => "Models & PDKs".to_owned(),
            Workspace::Netlist => code_title_cell(&app.state),
        }
    }

    fn account_holder(&self) -> Option<String> {
        crate::workbench::user_identity::account_holder(
            self.cloud_account.snapshot(),
            self.state.cloud.license.as_ref(),
        )
    }

    fn rail_entry(&self, workspace: Workspace) -> RailEntry<RSpiceApp> {
        use crate::workbench::chrome::activity_rail::results::{
            mark_results_seen, unseen_result_count,
        };

        match workspace {
            // Results not yet seen badge the button, and showing Results is
            // seeing them.
            Workspace::Results => RailEntry {
                badge: Some(|app, active| {
                    unseen_result_count(&app.state, Workspace::Results, active)
                }),
                on_visible: Some(|app| mark_results_seen(&mut app.state)),
            },
            _ => RailEntry {
                badge: None,
                on_visible: None,
            },
        }
    }

    fn status_frame(&mut self, ctx: &egui::Context) -> status_order::StatusReadings {
        let availability = self.simulation_controller.engine_availability();
        status_order::status_readings(&mut self.state, availability, ctx)
    }

    fn status_cells(&self) -> &'static [StatusCell<RSpiceApp>] {
        status_order::STATUS_ORDER
    }

    fn command_registry(&self) -> &'static Registry {
        command_registry()
    }

    fn availability(&self, key: CommandKey) -> CommandAvailability {
        command(key).map_or(CommandAvailability::Hidden, |command| {
            command.availability(self)
        })
    }

    fn is_enabled(&self, key: CommandKey) -> bool {
        command(key).is_some_and(|command| command.is_enabled(self))
    }

    fn checked(&self, key: CommandKey) -> Option<bool> {
        command(key).and_then(|command| command.checked(self))
    }

    fn label(&self, key: CommandKey) -> &'static str {
        command(key).map_or(key.as_str(), |command| command.spec().label)
    }

    fn shortcut_label(
        &self,
        key: CommandKey,
        platform: CommandPlatform,
        os: OperatingSystem,
    ) -> String {
        self.state
            .model
            .preferences
            .shortcuts()
            .resolved_label(&CommandRegistry, key, platform, os)
    }

    fn execute(&mut self, key: CommandKey) {
        if let Some(command) = command(key) {
            command.execute(self);
        }
    }

    fn command_icon(&self, key: CommandKey) -> Option<WorkbenchIcon> {
        command(key).and_then(menus::command_icon)
    }

    fn palette_icon(&self, key: CommandKey) -> Option<Icon> {
        command(key).and_then(palette_icons::palette_icon)
    }

    fn open_workspace(&self, workspace: Workspace) -> CommandKey {
        Command::OpenWorkspace(workspace).key()
    }

    fn menu_entries(&self, menu: ApplicationMenu) -> &'static [MenuEntry] {
        menu.entries()
    }

    fn compact_menu(&self) -> &'static [MenuEntry] {
        COMPACT_MENU
    }

    fn source_editor_menu(&self) -> &'static [MenuEntry] {
        SOURCE_EDITOR_MENU
    }

    fn validate_commands(&self) -> Result<(), String> {
        let mut keys = Vec::new();
        for menu in application_menus::ALL_MENUS {
            menu_keys(menu.entries(), &mut keys);
        }
        menu_keys(COMPACT_MENU, &mut keys);
        menu_keys(SOURCE_EDITOR_MENU, &mut keys);
        command_registry().validate(ids::named().chain(keys.into_iter().map(|key| ("a menu", key))))
    }
}

impl WorkbenchHost for RSpiceApp {
    fn workbench(&self) -> &WorkbenchSession {
        &self.state.workbench
    }

    fn workbench_mut(&mut self) -> &mut WorkbenchSession {
        &mut self.state.workbench
    }

    fn model(&self) -> &Model {
        &self.state.model
    }

    fn state(&self) -> &AppState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut AppState {
        &mut self.state
    }
}

impl DocumentHost for AppState {
    type WorkspaceKey = Workspace;
    type DocumentId = WorkspaceDocumentId;
    type Document = WorkspaceDocument;

    fn workspace_documents(&self, workspace: Workspace) -> Vec<WorkspaceDocument> {
        use crate::workbench::chrome::document_bar::code::netlist_workspace_documents;
        use crate::workbench::chrome::document_bar::design::design_documents;
        use crate::workbench::chrome::document_bar::models::models_documents;
        use crate::workbench::chrome::document_bar::project::project_documents;
        use crate::workbench::chrome::document_bar::results::result_documents;
        use crate::workbench::chrome::document_bar::simulate::{
            simulation_plan_documents, verification_documents,
        };

        let state = self;
        match workspace {
            Workspace::Project => project_documents(),
            Workspace::Design => design_documents(state),
            Workspace::Simulate => simulation_plan_documents(),
            // The library is one project-owned record, and the shell has no
            // document identity for it yet — the instrument that edits a
            // definition is what will open one. Naming a tab here before that
            // exists would put a document in the window session that nothing can
            // detach, close, or save.
            Workspace::Stimulus => Vec::new(),
            Workspace::Results => result_documents(state),
            Workspace::Verify => verification_documents(),
            Workspace::Models => models_documents(),
            Workspace::Netlist => netlist_workspace_documents(state),
        }
    }

    fn page_document(&self, workspace: Workspace) -> Option<WorkspaceDocumentId> {
        use crate::workbench::chrome::document_bar::code::active_netlist_document_id;

        match workspace {
            Workspace::Netlist => active_netlist_document_id(self),
            _ => None,
        }
    }

    fn activate_owned_document(&mut self, document: &WorkspaceDocumentId) -> bool {
        use crate::workbench::chrome::document_bar::code::{
            activate_generated_netlist, activate_netlist_comparison, activate_netlist_dependency,
            activate_netlist_run_snapshot, activate_owned_netlist,
        };
        use crate::workbench::chrome::document_bar::design::activate_cell_view;
        use crate::workbench::chrome::document_bar::results::{
            activate_result_dataset, activate_visualization_document,
        };

        let state = self;
        match document {
            WorkspaceDocumentId::CellView(reference) => activate_cell_view(state, reference),
            WorkspaceDocumentId::VisualizationDocument(document_id) => {
                activate_visualization_document(state, document_id)
            }
            WorkspaceDocumentId::ResultDataset(dataset_id) => {
                activate_result_dataset(state, dataset_id)
            }
            WorkspaceDocumentId::NetlistGenerated(document_id) => {
                activate_generated_netlist(state, document_id)
            }
            WorkspaceDocumentId::NetlistOwned(document_id) => {
                activate_owned_netlist(state, document_id)
            }
            WorkspaceDocumentId::NetlistDependency {
                root,
                logical_identity,
            } => activate_netlist_dependency(state, root, logical_identity),
            WorkspaceDocumentId::NetlistComparison => activate_netlist_comparison(state),
            WorkspaceDocumentId::NetlistRunSnapshot => activate_netlist_run_snapshot(state),
            WorkspaceDocumentId::Project
            | WorkspaceDocumentId::SimulationPlan
            | WorkspaceDocumentId::Verification
            | WorkspaceDocumentId::Models
            | WorkspaceDocumentId::NetlistSource => true,
        }
    }

    fn close_owned_document(&mut self, document: &WorkspaceDocumentId) -> Option<bool> {
        let state = self;
        match document {
            // A cell view closes as the view that shows, so it is brought
            // forward first.
            WorkspaceDocumentId::CellView(_) => Some(
                crate::workbench::chrome::document_bar::activate_document_by_id(state, document)
                    && crate::workbench::workflows::project_workflow::close_active_document(state),
            ),
            WorkspaceDocumentId::NetlistDependency { .. } => {
                state.code.netlist_open_documents.remove(document);
                None
            }
            _ => None,
        }
    }
}

/// The any-workspace inspector section applies: a trace is selected in the
/// results split beside the schematic, and the inspector reads it.
fn split_selected_trace_is_inspected(app: &RSpiceApp) -> bool {
    app.state.workbench.workspace == Workspace::Design
        && app.state.workbench.results_split_visible(
            app.state.platform.project_lifecycle.is_open(),
            app.state
                .model
                .simulation
                .retained
                .has_retained_result_dataset(),
        )
        && app
            .state
            .results
            .results
            .valid_selected_trace(&app.state.model.simulation)
            .is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_registry_answers_every_key_the_menus_and_the_framework_name() {
        assert_eq!(RSpiceApp::test_instance().validate_commands(), Ok(()));
    }

    /// Each key framework code names by an id is the key of the command the
    /// id is named for.
    #[test]
    fn each_framework_id_names_its_command() {
        for (id, command) in [
            (ids::ACCOUNT_ORGANIZATION, Command::AccountOrganization),
            (ids::CLEAR_CONSOLE, Command::ClearConsole),
            (ids::COMMAND_PALETTE, Command::CommandPalette),
            (
                ids::DESIGN_WORKSPACE,
                Command::OpenWorkspace(Workspace::Design),
            ),
            (ids::MEASUREMENT_LIBRARY, Command::MeasurementLibrary),
            (
                ids::MODELS_WORKSPACE,
                Command::OpenWorkspace(Workspace::Models),
            ),
            (
                ids::NETLIST_WORKSPACE,
                Command::OpenWorkspace(Workspace::Netlist),
            ),
            (ids::OPEN_CONSOLE, Command::OpenConsole),
            (ids::OPEN_NOTIFICATIONS, Command::OpenNotifications),
            (ids::PREFERENCES, Command::Preferences),
            (ids::PROJECT_LAUNCHER, Command::ProjectLauncher),
            (
                ids::PROJECT_RECOVERY,
                Command::ProjectPage(ProjectPage::Recovery),
            ),
            (
                ids::PROJECT_WORKSPACE,
                Command::OpenWorkspace(Workspace::Project),
            ),
            (ids::RESET_LAYOUT, Command::ResetLayout),
            (
                ids::RESULTS_WORKSPACE,
                Command::OpenWorkspace(Workspace::Results),
            ),
            (ids::SAVE, Command::Save),
            (
                ids::SIMULATE_WORKSPACE,
                Command::OpenWorkspace(Workspace::Simulate),
            ),
            (ids::SPECIALIST_TOOL_BROWSER, Command::SpecialistToolBrowser),
            (
                ids::STIMULUS_WORKSPACE,
                Command::OpenWorkspace(Workspace::Stimulus),
            ),
            (ids::TOGGLE_CONSOLE, Command::ToggleConsole),
            (
                ids::TOGGLE_CONSOLE_MAXIMIZED,
                Command::ToggleConsoleMaximized,
            ),
            (ids::TOGGLE_FOCUS_MODE, Command::ToggleFocusMode),
            (
                ids::VERIFY_WORKSPACE,
                Command::OpenWorkspace(Workspace::Verify),
            ),
            (ids::WORKSPACE_LAYOUTS, Command::WorkspaceLayouts),
        ] {
            assert_eq!(id, command.key(), "{command:?}");
            assert_eq!(legacy_command(id), Some(command), "{command:?}");
        }
    }

    /// The commands the registry leaves out answer by key exactly as they
    /// answer by value, and nothing else answers to their keys.
    #[test]
    fn a_left_out_command_answers_by_key() {
        let app = RSpiceApp::test_instance();
        for left_out in UNLISTED {
            assert_eq!(legacy_command(left_out.key()), None, "{left_out:?}");
            assert_eq!(command(left_out.key()), Some(left_out));
            assert_eq!(
                app.availability(left_out.key()),
                left_out.availability(&app)
            );
            assert_eq!(app.label(left_out.key()), left_out.spec().label);
        }
        assert_eq!(
            app.availability(CommandKey::new("no-such-command")),
            CommandAvailability::Hidden
        );
    }
}
