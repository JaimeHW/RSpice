//! The shell's host: which owner draws at each contribution point, and what
//! the containers draw around the points that depend on an owner's state.
//!
//! The containers name no owner; this table does. Each point resolves to its
//! owner's function in the order the arms are listed and on the conditions
//! they test, the first arm that matches winning, so an arm listed earlier
//! takes a point from one listed later. The owner draws in the `Ui` the
//! container handed over.

use egui::Ui;

use crate::state::ViewType;
use crate::workbench::framework::host::{Host, InspectorScope, NavigatorSection, Point};
use crate::workbench::state::{ProjectPage, VerificationPage, Workspace};
use crate::workbench::{MessageId, RSpiceApp};

impl Host for RSpiceApp {
    type WorkspaceKey = Workspace;

    fn contribute(&mut self, point: Point<Workspace>, ui: &mut Ui) {
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
