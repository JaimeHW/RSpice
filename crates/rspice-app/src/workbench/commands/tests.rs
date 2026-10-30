//! Tests for command dispatch and menu availability.
//!
//! The cases here mostly assert absence: a command whose behaviour is not
//! implemented must not be offered, and one whose preconditions are unmet must
//! report unavailable rather than dispatch and fail.
//!
//! They are grouped by the surface whose commands they hold to account. This
//! file keeps only the fixtures more than one group builds on, so each group
//! can be read as the whole claim about its surface.

mod catalog;
mod code_pages;
mod design_menu;
mod drawing_sheets;
mod execution;
mod project;
mod rows;
mod schematic_editing;
mod schematic_placement;
mod sheets;
mod simulation_plan;
mod view_state;

use super::vocabulary::{CommandSpec, command_catalog};
use super::*;
use crate::product::{AnalysisInstanceId, ContentDigest, SimulationPlanId};
use crate::state::ModelsPage;
use crate::state::Wire;
use crate::state::{
    AnalysisResult, AnalysisResultProvenance, AnalysisResultSourceDomain, AnalysisType,
    PreparedRunReceipt, PreparedRunTaskReceipt, PreparedSourceCheckReceipt,
};
use crate::workbench::framework::command_registry::CommandAvailability;
use crate::workbench::project::workspace_state::ProjectPage;
use crate::workbench::results::commands::ResultsCommand;
use crate::workbench::settings::commands::SettingsCommand;
use crate::workbench::sheets::commands::SheetsCommand;
use crate::workbench::state::{SimulationPage, VerificationPage};

fn app_with_selected_authored_symbol() -> RSpiceApp {
    use crate::state::{
        Cell, Component, Library, LibraryCellInstance, Point, PortDirection, PortSpec,
        SymbolDocument, SymbolPin, View, ViewType, Wire,
    };

    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;

    let document = SymbolDocument {
        pins: vec![
            SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(70, 20))),
            SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, -10))),
        ],
        ..SymbolDocument::default()
    };
    let mut symbol_view = View::new("symbol", ViewType::Symbol);
    document
        .store_in_view(&mut symbol_view)
        .expect("authored symbol stores");
    let mut cell = Cell::new("amp");
    cell.add_view(symbol_view);
    let mut library = Library::new("command_test");
    library.add_cell(cell);
    app.state.model.library_manager.add_library(library);

    let interface = [
        PortSpec {
            name: "IN".to_owned(),
            direction: PortDirection::In,
        },
        PortSpec {
            name: "OUT".to_owned(),
            direction: PortDirection::Out,
        },
    ];
    let mut binding = LibraryCellInstance::new("command_test", "amp", "schematic");
    binding.bind_interface(&interface);
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(
            Component::new(701, ComponentType::CellInstance, Point::new(100, 50))
                .with_library_cell(binding),
        );
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(Wire::segment(702, Point::new(60, 40), Point::new(60, 0)));
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_component(701);
    app
}

/// The active plan's identity and its first instance, which a seeded run is
/// sealed against so the return edge has something exact to resolve.
pub(crate) fn plan_binding(app: &RSpiceApp) -> (SimulationPlanId, AnalysisInstanceId) {
    let plan = app
        .state
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("the test instance owns a stable analysis plan");
    (
        plan.id(),
        plan.instances()
            .first()
            .expect("the default plan holds one instance")
            .id(),
    )
}

/// Which plan a seeded prepared run is sealed against.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum RunOrigin {
    /// The app's own active plan, so the return edge resolves.
    ActivePlan,
    /// A plan that is not the active one — a retained dataset from a plan the
    /// project has since switched away from.
    ForeignPlan,
    /// No plan at all: a deck run by hand.
    ManualDeck,
}

/// A project with one retained, prepared run sealed against `origin`.
pub(crate) fn app_with_prepared_run(origin: RunOrigin) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    let (active_plan, analysis_id) = plan_binding(&app);
    let project_revision = app.state.model.workspace.content.project.revision();
    let (domain, plan_id, source_check) = match origin {
        RunOrigin::ActivePlan => (
            AnalysisResultSourceDomain::SimulationPlan,
            Some(active_plan),
            PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0x33; 32])),
        ),
        RunOrigin::ForeignPlan => (
            AnalysisResultSourceDomain::SimulationPlan,
            Some(SimulationPlanId::new()),
            PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0x33; 32])),
        ),
        RunOrigin::ManualDeck => (
            AnalysisResultSourceDomain::ManualDeck,
            None,
            PreparedSourceCheckReceipt::ManualSourceCheck(ContentDigest::from_bytes([0x33; 32])),
        ),
    };
    let receipt = PreparedRunReceipt::new(crate::state::PreparedRunReceiptInput {
        source_domain: domain,
        simulation_plan_id: plan_id,
        project_revision,
        prepared_snapshot_digest: ContentDigest::from_bytes([0x11; 32]),
        source_content_digest: ContentDigest::from_bytes([0x22; 32]),
        source_check_receipt: source_check,
        project_model_sources: Vec::new(),
        specifications: Vec::new(),
        specification_policy: rspice_results::specification::PreparedSpecificationPolicy::default(),
        tasks: vec![
            PreparedRunTaskReceipt::new(
                analysis_id,
                project_revision,
                Vec::new(),
                1,
                ContentDigest::from_bytes([0x44; 32]),
            )
            .expect("valid prepared task"),
        ],
    })
    .expect("valid prepared run receipt");
    app.state
        .model
        .simulation
        .start_prepared_run(receipt)
        .unwrap()
        .add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "retained TRAN").with_provenance(
                AnalysisResultProvenance::new_with_source_domain(
                    AnalysisResultSourceDomain::SimulationPlan,
                    analysis_id,
                    project_revision,
                    ContentDigest::from_bytes([0x44; 32]),
                    Vec::new(),
                )
                .expect("valid prepared analysis provenance"),
            ),
        );
    app.state.model.simulation.view.active_run_idx = Some(0);
    app.state.model.simulation.view.active_analysis_idx = Some(0);
    app
}
