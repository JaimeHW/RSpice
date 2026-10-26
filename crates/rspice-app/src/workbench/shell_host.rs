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

use crate::diagnostics::LogAnchor;
use crate::state::command_catalog::{CommandKey, CommandPlatform};
use crate::state::{CellViewRef, ViewType};
use crate::ui::icons::Icon;
use crate::ui::widgets::NotificationAction;
use crate::workbench::app::ConfirmationResponse;
use crate::workbench::app::dialogs::confirmation::ConfirmationHost;
use crate::workbench::app_state::problems_view::ProblemsView;
use crate::workbench::app_state::waivers::WaiverTarget;
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::chrome::title_bar::application_menus::{self, ApplicationMenu};
use crate::workbench::chrome::title_bar::menu_entries::{MenuEntry, menu_keys};
use crate::workbench::commands::vocabulary::{
    Command, CommandRegistry, command_registry, legacy_command,
};
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::features;
use crate::workbench::framework::command_registry::CommandAvailability;
use crate::workbench::framework::command_registry::{Registry, ids};
use crate::workbench::framework::host::{
    DocumentHost, Host, InspectorScope, NavigatorSection, Point, ProblemsHost, RailEntry,
    StatusCell, ToolbarScope, ToolbarSlot,
};
use crate::workbench::framework::workbench_host::{ConsoleHost, WorkbenchHost};
use crate::workbench::hardcopy::HardcopyFeature;
use crate::workbench::layout::LayoutSpec;
use crate::workbench::lifecycle::project_lifecycle::{ProjectCloseDestination, RevertReviewToken};
use crate::workbench::sheets::SheetsFeature;
use crate::workbench::state::workbench_session::WorkspaceDocument;
use crate::workbench::state::{
    ProjectPage, VerificationPage, WorkbenchSession, Workspace, WorkspaceDocumentId,
};
use crate::workbench::{AppState, MessageId, RSpiceApp};
use menus::{COMPACT_MENU, SOURCE_EDITOR_MENU, application_menu_entries};

/// A route whose surface brings its own toolbar, in place of its
/// workspace's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceOwner {
    ReportAuthoring,
    ModelEditor,
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
                    models, netlist, pdk_technology_admin, project, results, simulate, stimulus,
                    verify,
                };

                if app.state.workbench.current_route().surface_id()
                    == super::SurfaceId::ReportAuthoring
                {
                    features::show_surface::<HardcopyFeature>(app, ui);
                    app.after_contribution();
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
                // A hand-off a page raised on its way into another workspace
                // is answered once the page has drawn, as a feature's command
                // has its own answered once it returns.
                app.after_contribution();
            }
            Point::SplitResults => {
                crate::workbench::documents::result_document::show_compact_split(ui, app);
                app.after_contribution();
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
                    Workspace::Results => {
                        results(ui, app);
                        app.after_contribution();
                    }
                    Workspace::Project => project(ui, app),
                    Workspace::Simulate => simulate(ui, app),
                    Workspace::Stimulus => stimulus::show(ui, &mut app.state),
                    Workspace::Verify => verify(ui, app),
                    Workspace::Models => models(ui, app),
                }
            }
            Point::Inspector(InspectorScope::AnyWorkspace) => {
                crate::workbench::docks::inspector::results::results(ui, app);
                app.after_contribution();
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
                    Workspace::Results => {
                        results(ui, app);
                        app.after_contribution();
                    }
                    Workspace::Verify => verify(ui, app),
                    Workspace::Models => models::show(ui, app),
                    Workspace::Netlist => code_workspace_inspector(ui, app),
                }
            }
            // The section that inspects a workspace's document itself: the
            // drawing sheet, which the sheets feature draws.
            Point::Inspector(InspectorScope::Document(Workspace::Design)) => {
                features::show_inspector::<SheetsFeature>(app, ui);
                app.after_contribution();
            }
            Point::Inspector(InspectorScope::Document(_)) => {}
            // The toolbar's lane: the tools of whoever its scope names.
            Point::Toolbar(ToolbarSlot::Tools(scope), layout) => {
                use crate::workbench::chrome::toolbar::code::{netlist_tools, veriloga_tools};
                use crate::workbench::chrome::toolbar::design::{
                    design_tools, stimulus_tools, symbol_tools,
                };
                use crate::workbench::chrome::toolbar::feature_tools;
                use crate::workbench::chrome::toolbar::models::{model_editor_tools, models_tools};
                use crate::workbench::chrome::toolbar::project::{no_project_tools, project_tools};
                use crate::workbench::chrome::toolbar::results::results_tools;
                use crate::workbench::chrome::toolbar::simulate::{
                    simulation_tools, verification_tools,
                };

                match scope {
                    ToolbarScope::NoProject => no_project_tools(ui, app, layout),
                    ToolbarScope::Surface(SurfaceOwner::ReportAuthoring) => {
                        feature_tools(
                            ui,
                            app,
                            features::toolbar_tools::<HardcopyFeature>(),
                            layout,
                        );
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
            pins_scroll_bar: match workspace {
                Workspace::Verify => Some(
                    crate::workbench::docks::navigator::verify::verification_navigator_requires_scroll,
                ),
                _ => None,
            },
        }
    }

    fn commit_inspector_section(&mut self, scope: InspectorScope<Workspace>, ctx: &egui::Context) {
        if scope == InspectorScope::Document(Workspace::Design) {
            features::commit_inspector::<SheetsFeature>(self, ctx);
            self.after_contribution();
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
        let workspace = match scope {
            InspectorScope::AnyWorkspace => return "Result details".to_owned(),
            InspectorScope::Workspace(workspace) | InspectorScope::Document(workspace) => workspace,
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
                    unseen_result_count(
                        app.state.results.results_seen_version,
                        &app.state.model.simulation,
                        Workspace::Results,
                        active,
                    )
                }),
                on_visible: Some(|app| {
                    mark_results_seen(
                        &mut app.state.results.results_seen_version,
                        &app.state.model.simulation,
                    )
                }),
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

    // A key no legacy command answers to is a feature's, or nobody's: the
    // dispatcher by key answers both.
    fn availability(&self, key: CommandKey) -> CommandAvailability {
        legacy_command(key).map_or_else(
            || self.command_availability(key),
            |command| command.availability(self),
        )
    }

    fn is_enabled(&self, key: CommandKey) -> bool {
        legacy_command(key).map_or_else(
            || self.command_key_enabled(key),
            |command| command.is_enabled(self),
        )
    }

    fn checked(&self, key: CommandKey) -> Option<bool> {
        legacy_command(key).map_or_else(
            || features::checked(&self.state, key),
            |command| command.checked(self),
        )
    }

    fn label(&self, key: CommandKey) -> &'static str {
        legacy_command(key).map_or_else(
            || features::spec(key).map_or_else(|| command_registry().label(key), |spec| spec.label),
            |command| command.spec().label,
        )
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
        match legacy_command(key) {
            Some(command) => command.execute(self),
            None => self.execute_command_key(key),
        }
    }

    fn command_icon(&self, key: CommandKey) -> Option<WorkbenchIcon> {
        legacy_command(key).map_or_else(|| menus::feature_command_icon(key), menus::command_icon)
    }

    fn palette_icon(&self, key: CommandKey) -> Option<Icon> {
        legacy_command(key).map_or_else(
            || palette_icons::feature_palette_icon(key),
            palette_icons::palette_icon,
        )
    }

    fn open_workspace(&self, workspace: Workspace) -> CommandKey {
        Command::OpenWorkspace(workspace).key()
    }

    fn menu_entries(&self, menu: ApplicationMenu) -> &'static [MenuEntry] {
        application_menu_entries(menu)
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
            menu_keys(application_menu_entries(menu), &mut keys);
        }
        menu_keys(COMPACT_MENU, &mut keys);
        menu_keys(SOURCE_EDITOR_MENU, &mut keys);
        command_registry().validate(
            ids::named()
                .chain(keys.into_iter().map(|key| ("a menu", key)))
                .chain(crate::workbench::settings::named_command_keys()),
        )
    }

    fn perform_notification_action(&mut self, action: NotificationAction) {
        crate::workbench::commands::result_navigation::perform_notification_action(self, action);
    }

    fn open_run_log(&mut self, run_sequence: u64) {
        crate::workbench::commands::result_navigation::open_run_log(
            &self.state.model,
            &mut self.state.workbench,
            run_sequence,
        );
    }

    fn simulation_input_changed(&mut self) {
        self.invalidate_simulation_preflight();
    }
}

impl ProblemsHost for AppState {
    fn apply_quick_fix(&mut self, finding: uuid::Uuid) -> Result<String, String> {
        crate::workbench::documents::netlist_document::language::apply_quick_fix_for_finding(
            self, finding,
        )
    }
}

impl ConsoleHost for AppState {
    fn model(&self) -> &Model {
        &self.model
    }

    fn model_mut(&mut self) -> &mut Model {
        &mut self.model
    }

    fn workbench(&self) -> &WorkbenchSession {
        &self.workbench
    }

    fn workbench_mut(&mut self) -> &mut WorkbenchSession {
        &mut self.workbench
    }

    fn problems_view(&self) -> &ProblemsView {
        &self.problems_view
    }

    fn problems_view_mut(&mut self) -> &mut ProblemsView {
        &mut self.problems_view
    }

    fn jump_to_log_anchor(&mut self, anchor: LogAnchor) {
        AppState::jump_to_log_anchor(self, anchor);
    }

    fn log_anchor_refusal(&self, anchor: &LogAnchor) -> Option<String> {
        AppState::log_anchor_refusal(self, anchor)
    }

    fn waive_finding(&mut self, target: &WaiverTarget, reason: &str) -> Result<(), String> {
        AppState::waive_finding(self, target, reason)
    }

    fn remove_finding_waiver(&mut self, target: &WaiverTarget) -> Result<(), String> {
        AppState::remove_finding_waiver(self, target)
    }
}

impl WorkbenchHost for RSpiceApp {
    type Session = AppState;

    fn session(&self) -> &AppState {
        &self.state
    }

    fn session_mut(&mut self) -> &mut AppState {
        &mut self.state
    }

    fn workbench(&self) -> &WorkbenchSession {
        &self.state.workbench
    }

    fn workbench_mut(&mut self) -> &mut WorkbenchSession {
        &mut self.state.workbench
    }

    fn model(&self) -> &Model {
        &self.state.model
    }

    fn model_mut(&mut self) -> &mut Model {
        &mut self.state.model
    }

    fn platform(&self) -> &PlatformState {
        &self.state.platform
    }

    fn run_preflight_checking(&self) -> bool {
        self.state.runs.preflight.checking.is_some()
    }

    fn application_modal_open(&self) -> bool {
        self.state.application_modal_open()
    }

    fn model_draft_unsaved(&self) -> bool {
        self.state.models.model_editor_has_unsaved_changes()
    }

    fn unapplied_stimulus_drafts(&self) -> Vec<String> {
        self.state.design.stimulus_editor.dirty_definitions()
    }

    fn open_cell_view(&mut self, reference: CellViewRef) {
        self.state.open_workspace_view(reference);
    }

    fn show_catalog_model(&mut self, library: &str, model: String) {
        let state = &mut self.state;
        state.select_model_library(library);
        state.model.model_library_manager.filter_text = model.clone();
        state.model.model_library_manager.filter_type = None;
        state.models.selected_model = Some(model);
        state.models.models_page = crate::state::ModelsPage::Models;
        state
            .workbench
            .activate(crate::workbench::state::Workspace::Models);
    }

    fn show_signal(&mut self, run_index: usize, analysis_index: usize, waveform_index: usize) {
        let state = &mut self.state;
        state.model.simulation.select_run(run_index);
        state.model.simulation.select_analysis(analysis_index);
        state.model.simulation.retained.runs[run_index].analyses[analysis_index].waveforms
            [waveform_index]
            .visible = true;
        if let Some(waveform) = state
            .model
            .simulation
            .view
            .waveforms
            .get_mut(waveform_index)
        {
            waveform.visible = true;
        }
        state.model.simulation.view.data_version =
            state.model.simulation.view.data_version.wrapping_add(1);
        state.results.results.session.viewer =
            rspice_results::result_presentation::ResultViewer::Waves;
        state
            .results
            .results
            .restore_analysis_strip(&state.model.simulation, analysis_index);
        state.results.results.session.maximized_strip = None;
        state
            .workbench
            .activate(crate::workbench::state::Workspace::Results);
    }
}

impl ConfirmationHost for RSpiceApp {
    fn answer_confirmation(&mut self, response: ConfirmationResponse) {
        self.handle_confirmation_response(response);
    }

    fn revert_active_document(&mut self, token: &RevertReviewToken) {
        crate::workbench::workflows::project_workflow::confirm_revert_active_document(
            &mut self.state,
            token,
        );
    }

    fn close_project_after_save(&mut self, destination: ProjectCloseDestination) {
        self.begin_close_project_after_save(destination);
    }

    fn close_project_discarding(&mut self, destination: ProjectCloseDestination) {
        crate::workbench::workflows::project_workflow::close_project_discard(
            &mut self.state,
            destination,
        );
    }

    #[cfg(target_arch = "wasm32")]
    fn pending_browser_file_operation(&self) -> Option<String> {
        crate::workbench::workflows::project_workflow::browser_file_operation_label(&self.state)
    }

    #[cfg(target_arch = "wasm32")]
    fn cancel_browser_file_operation(&mut self) {
        crate::workbench::workflows::project_workflow::cancel_pending_browser_file_operation(
            &mut self.state,
        );
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
            Workspace::Results => result_documents(&state.model.simulation),
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
                activate_visualization_document(&mut state.results_parts(), document_id)
            }
            WorkspaceDocumentId::ResultDataset(dataset_id) => {
                activate_result_dataset(&mut state.model.simulation, dataset_id)
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

    fn commit_pending_edit(&mut self) -> bool {
        self.commit_pending_inspector_edit()
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
            (ids::CANCEL, Command::Cancel),
            (ids::CLEAR_CONSOLE, Command::ClearConsole),
            (ids::COMMAND_PALETTE, Command::CommandPalette),
            (ids::COPY, Command::Copy),
            (
                ids::DESIGN_WORKSPACE,
                Command::OpenWorkspace(Workspace::Design),
            ),
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
            (ids::PROJECT_LAUNCHER, Command::ProjectLauncher),
            (
                ids::PROJECT_RECOVERY,
                Command::ProjectPage(ProjectPage::Recovery),
            ),
            (
                ids::PROJECT_WORKSPACE,
                Command::OpenWorkspace(Workspace::Project),
            ),
            (ids::RESET_ACTIVE_VIEW, Command::ResetActiveView),
            (ids::RESET_LAYOUT, Command::ResetLayout),
            (
                ids::RESULTS_WORKSPACE,
                Command::OpenWorkspace(Workspace::Results),
            ),
            (ids::SAVE, Command::Save),
            (
                ids::SIMULATION_ANALYSES,
                Command::SimulationPage(crate::workbench::state::SimulationPage::Analyses),
            ),
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
            (ids::ZOOM_FIT, Command::ZoomFit),
            (ids::ZOOM_IN, Command::ZoomIn),
            (ids::ZOOM_OUT, Command::ZoomOut),
        ] {
            assert_eq!(id, command.key(), "{command:?}");
            assert_eq!(legacy_command(id), Some(command), "{command:?}");
        }
    }

    /// Each key framework code names by an id for a results command is the
    /// key that command declares.
    #[test]
    fn each_results_id_names_its_command() {
        use crate::workbench::ResultViewer;
        use crate::workbench::results::commands::ResultsCommand;
        for (id, command) in [
            (ids::CALCULATOR, ResultsCommand::WaveformCalculator),
            (ids::EDIT_SPECIFICATIONS, ResultsCommand::EditSpecifications),
            (ids::EXPORT_RESULTS, ResultsCommand::ExportWaveformsCsv),
            (
                ids::EXPRESSION_DIAGNOSTICS,
                ResultsCommand::ExpressionDiagnostics,
            ),
            (ids::FAMILY_SLICING, ResultsCommand::FamilySlicing),
            (ids::MEASUREMENT_LIBRARY, ResultsCommand::MeasurementLibrary),
            (ids::OPEN_RUN_IN_RESULTS, ResultsCommand::OpenRunInResults),
            (
                ids::RESULT_OPERATING_POINT,
                ResultsCommand::ResultViewer(ResultViewer::Op),
            ),
            (
                ids::RESULT_SPECIFICATIONS,
                ResultsCommand::ResultViewer(ResultViewer::Specs),
            ),
            (
                ids::RESULT_WAVEFORMS,
                ResultsCommand::ResultViewer(ResultViewer::Waves),
            ),
            (ids::TOGGLE_SPLIT_VIEW, ResultsCommand::ToggleResultsSplit),
        ] {
            assert_eq!(id, command.key(), "{command:?}");
            assert_eq!(ResultsCommand::from_key(id), Some(command), "{command:?}");
            assert_eq!(legacy_command(id), None, "{command:?}");
        }
    }

    /// The commands a feature leaves out of the registry answer by the key
    /// the toolbar holds, under the label they declare, and no id names them.
    #[test]
    fn a_left_out_command_answers_by_key() {
        use crate::workbench::hardcopy::commands::HardcopyCommand;

        let app = RSpiceApp::test_instance();
        for left_out in HardcopyCommand::REPORT_TOOLBAR {
            let key = left_out.key();
            assert_eq!(legacy_command(key), None, "{left_out:?}");
            assert_eq!(
                command_registry().resolve(key.as_str()),
                None,
                "{left_out:?}"
            );
            assert_eq!(
                app.availability(key),
                CommandAvailability::Disabled("command is unavailable in this context"),
                "off the report route, {left_out:?} is the feature's to refuse"
            );
            assert!(!app.is_enabled(key), "{left_out:?}");
            assert_eq!(app.label(key), left_out.declared_spec().label);
        }
        assert_eq!(
            app.availability(CommandKey::new("no-such-command")),
            CommandAvailability::Hidden
        );
    }
}
