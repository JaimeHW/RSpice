//! The Library page's inspector drawn through the whole state: what it
//! announces for the open built-in library and for a Verilog-A view.

use egui::Ui;
use rspice_app_state::veriloga_library::{BUILTIN_VERILOGA_MODELS, BuiltinVerilogAModel};

use crate::workbench::app_state::AppState;
use rspice_app_project::workbench::docks::inspector::project::{
    LibraryShown, ProjectInspector, project_library,
};
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_services::workbench::lifecycle::project_lifecycle::LifecycleView;
use rspice_app_state::state::ViewType;
use rspice_app_state::workbench::app_state::model::SlotFacts;

/// Everything the Library page's inspector announces for `state`.
fn announced(
    state: &mut AppState,
    builtin_library_open: bool,
    builtin_model: Option<&'static BuiltinVerilogAModel>,
) -> Vec<String> {
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let input = || egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(300.0, 640.0),
        )),
        ..Default::default()
    };
    let mut pass = |ctx: &mut Ui| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let inspector = ProjectInspector {
                page: ProjectPage::Library,
                configuration: &mut state.project.configuration,
                dependency_selection: &state.project.project_dependency_selection,
                checkpoint_recovery: &state.project.project_checkpoint_recovery,
                checkpoint_selection: &state.project.project_checkpoint_selection,
                builtin_library_open,
                builtin_model,
                library_shown: LibraryShown::ALL,
                model: &state.model,
                facts: SlotFacts {
                    preflight_report_retained: false,
                    review_markers: state.design.schematic_visibility.review_markers,
                },
                lifecycle: LifecycleView {
                    model: &state.model,
                    platform: &state.platform,
                    workspace: state.workbench.workspace,
                    presentation: &state.results.results,
                },
            };
            let _ = project_library(ui, &inspector);
        });
    };
    // Two passes: the first builds the fonts the rows are measured in.
    let _ = ctx.run_ui(input(), &mut pass);
    ctx.run_ui(input(), &mut pass)
        .platform_output
        .accesskit_update
        .expect("AccessKit tree")
        .nodes
        .into_iter()
        .flat_map(|(_, node)| {
            [
                node.label().map(str::to_owned),
                node.value().map(str::to_owned),
            ]
        })
        .flatten()
        .collect()
}

#[test]
fn the_open_built_in_library_is_what_the_inspector_describes() {
    let model = &BUILTIN_VERILOGA_MODELS[0];
    let mut state = AppState::default();

    let said = announced(&mut state, true, Some(model));

    for row in [
        "Module",
        model.module,
        "Version",
        model.version,
        "Access",
        "read-only",
    ] {
        assert!(said.iter().any(|text| text == row), "{row:?} in {said:?}");
    }
    for row in [
        "View type",
        "Location",
        "State",
        "Checks",
        "Used by",
        "Open selected view",
    ] {
        assert!(!said.iter().any(|text| text == row), "{row:?} in {said:?}");
    }
}

#[test]
fn a_verilog_a_view_reads_as_verilog_a() {
    let mut state = AppState::default();
    let mut cell = rspice_app_state::state::Cell::new("amp");
    cell.add_view(rspice_app_state::state::View::new(
        "veriloga",
        ViewType::VerilogA,
    ));
    let mut library = rspice_app_state::state::Library::new("analog");
    library.add_cell(cell);
    let libraries = &mut state.model.library_manager;
    libraries.add_library(library);
    libraries.select_view("analog", "amp", "veriloga");

    let said = announced(&mut state, false, None);

    assert!(said.iter().any(|text| text == "Verilog-A"), "{said:?}");
    assert!(!said.iter().any(|text| text == "veriloga"), "{said:?}");
}

/// The band cuts the selection's path to the width its controls leave; the
/// inspector states it whole, with what the cell is for and the file the view
/// is kept in.
#[test]
fn the_selection_reads_its_whole_path_description_and_file() {
    let mut state = AppState::default();
    let mut cell = rspice_app_state::state::Cell::new("bandgap_reference_with_trim");
    cell.description = "Trimmed 1.2 V reference".to_owned();
    let mut view = rspice_app_state::state::View::new("schematic", ViewType::Schematic);
    let file = std::path::PathBuf::from("analog/bandgap/schematic.rsch");
    view.file_path = Some(file.clone());
    cell.add_view(view);
    let mut library = rspice_app_state::state::Library::new("analog");
    library.add_cell(cell);
    let libraries = &mut state.model.library_manager;
    libraries.add_library(library);
    libraries.select_view("analog", "bandgap_reference_with_trim", "schematic");

    let said = announced(&mut state, false, None);

    for row in [
        "Path",
        "analog/bandgap_reference_with_trim/schematic",
        "Description",
        "Trimmed 1.2 V reference",
        "File",
        &file.display().to_string(),
    ] {
        assert!(said.iter().any(|text| text == row), "{row:?} in {said:?}");
    }
}
