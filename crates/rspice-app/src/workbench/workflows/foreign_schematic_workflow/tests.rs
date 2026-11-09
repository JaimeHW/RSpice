//! An imported LTspice drawing lands as project content a run reads: its
//! cells, its analyses in a plan, its parameters as design variables, its
//! model text and included files as model libraries, and what has no place
//! as text on its sheet.

use rspice_design::foreign_schematic::{ForeignCard, ForeignCardRole, ForeignTool};
use rspice_design_model::Point;
use rspice_simulation_contract::analysis_draft::AnalysisDraft;
use rspice_simulation_contract::config::NoiseSweepType;
use rspice_simulation_contract::options::IntegrationMethod;

use super::plan::{ForeignPlan, plan_from_cards};
use super::*;
use rspice_simulation_contract::run_set::RunSetDimensionKind;

fn card(role: ForeignCardRole, text: &str) -> ForeignCard {
    ForeignCard {
        position: Point::new(10, 20),
        role,
        text: text.to_owned(),
        tokens: rspice_design::netlist_document::card_tokens(text),
    }
}

#[test]
fn analysis_cards_become_the_drafts_of_their_analyses() {
    use ForeignCardRole::Analysis;
    let plan = plan_cards(vec![
        card(Analysis, ".tran 0 10m 1m 1u uic startup"),
        card(Analysis, ".ac oct 20 10 1meg"),
        card(Analysis, ".dc V1 0 5 0.1 V2 0 1 0.5"),
        card(Analysis, ".noise V(out) V1 dec 10 1 100k"),
        card(Analysis, ".tf V(out) Vin"),
        card(Analysis, ".op"),
        card(Analysis, ".ac list 1 10 100"),
    ]);
    let drafts = plan
        .analyses
        .iter()
        .map(|(_, draft)| draft)
        .collect::<Vec<_>>();
    let AnalysisDraft::Transient(tran) = drafts[0] else {
        panic!("a transient draft, not {:?}", drafts[0]);
    };
    // A zero print step is the source program's "choose one".
    assert_eq!(
        (
            tran.stop.as_str(),
            tran.step.as_str(),
            tran.start.as_str(),
            tran.max_step.as_str(),
            tran.uic
        ),
        ("10m", "10u", "1m", "1u", true)
    );
    let AnalysisDraft::Ac(ac) = drafts[1] else {
        panic!("an AC draft");
    };
    assert_eq!(
        (
            ac.sweep,
            ac.points.as_str(),
            ac.fstart.as_str(),
            ac.fstop.as_str()
        ),
        (1, "20", "10", "1meg")
    );
    let AnalysisDraft::DcSweep(dc) = drafts[2] else {
        panic!("a DC draft");
    };
    assert_eq!(
        (
            dc.source.as_str(),
            dc.stop.as_str(),
            dc.step.as_str(),
            dc.nested
        ),
        ("V1", "5", "0.1", true)
    );
    assert_eq!((dc.source2.as_str(), dc.step2.as_str()), ("V2", "0.5"));
    let AnalysisDraft::Noise(noise) = drafts[3] else {
        panic!("a noise draft");
    };
    assert_eq!(
        (noise.output.as_str(), noise.input.as_str(), noise.sweep),
        ("V(out)", "V1", NoiseSweepType::Decade)
    );
    let AnalysisDraft::TransferFunction(xf) = drafts[4] else {
        panic!("a transfer-function draft");
    };
    assert_eq!(
        (xf.output_expression.as_str(), xf.input_source.as_str()),
        ("V(out)", "Vin")
    );
    assert!(matches!(drafts[5], AnalysisDraft::OperatingPoint(_)));
    assert_eq!(drafts.len(), 6);
    assert_eq!(plan.kept.len(), 1, "the list sweep stays on the sheet");
    assert!(plan.notices[0].contains("`startup`"), "{:?}", plan.notices);

    let setup = crate::workbench::app_state::AppState::default()
        .model
        .sim_setup
        .clone();
    for draft in drafts {
        assert_eq!(
            setup.analysis_draft_validation_error(draft),
            None,
            "{draft:?}"
        );
    }
}

#[test]
fn parameters_options_and_temperature_find_their_plan_places() {
    let plan = plan_cards(vec![
        card(ForeignCardRole::Analysis, ".tran 1m"),
        card(ForeignCardRole::Parameter, ".param R=1k Gain={R/100}"),
        // LTspice also reads spaced and bare assignments.
        card(
            ForeignCardRole::Parameter,
            ".param Vdd = 15 sel 3 RG = {RF / 2} ; the supply",
        ),
        card(
            ForeignCardRole::Options,
            ".options reltol=1e-4 method=gear plotwinsize=0 cshunt=1p",
        ),
        card(ForeignCardRole::Temperature, ".temp 50"),
        card(ForeignCardRole::Temperature, ".temp 0 25 50"),
        card(ForeignCardRole::ModelText, ".model D1N D(Is=1n)"),
        card(ForeignCardRole::Step, ".step param R 1k 2k 1k"),
        card(ForeignCardRole::SourceOnly, ".backanno"),
        card(ForeignCardRole::Other, "K1 L1 L2 1"),
    ]);
    let variables = plan
        .variables
        .iter()
        .map(|(name, value)| (name.as_str(), value.as_str()))
        .collect::<Vec<_>>();
    assert_eq!(variables, [("R", "1k"), ("Vdd", "15"), ("sel", "3")]);
    assert_eq!(
        plan.model_text,
        [
            ".param Gain={R/100}",
            ".param RG={RF / 2}",
            ".model D1N D(Is=1n)"
        ]
    );
    assert_eq!(plan.options.reltol, 1e-4);
    assert_eq!(plan.options.method, IntegrationMethod::Gear2);
    assert_eq!(plan.temperature, Some(50.0));
    let kept = plan
        .kept
        .iter()
        .map(|(card, _)| card.text.as_str())
        .collect::<Vec<_>>();
    assert_eq!(kept, [".options cshunt=1p", "K1 L1 L2 1"]);
    assert!(plan.notices[0].contains("plotwinsize=0"));
    // A temperature list and a stepped parameter are axes of the run set.
    assert_eq!(
        axes(&plan),
        [
            (RunSetDimensionKind::Temperature, vec![0.0, 25.0, 50.0]),
            (RunSetDimensionKind::Parameter, vec![1e3, 2e3]),
        ]
    );
}

/// Every assignment of a parameter block reaches the plan: an expression
/// spaced around its operators, and several assignments on a card, as
/// QSPICE and LTspice read them (a = 3, c = 4, b = 5, d = 6, g = 4, f = 3
/// in batch runs of these cards on both, LTspice 26.1.1).
#[test]
fn a_parameter_card_reads_spaced_expressions_and_every_assignment() {
    let plan = plan_cards(vec![
        card(
            ForeignCardRole::Parameter,
            ".param a = 1 + 2 c = 4 b 5 d = a *\n+ 2",
        ),
        card(ForeignCardRole::Parameter, ".param g = 2*(1 + 1) f=3"),
    ]);
    assert_eq!(
        plan.variables,
        [
            ("c".to_owned(), "4".to_owned()),
            ("b".to_owned(), "5".to_owned()),
            ("f".to_owned(), "3".to_owned()),
        ]
    );
    assert_eq!(
        plan.model_text,
        [".param a=1 + 2 d=a * 2", ".param g=2*(1 + 1)"]
    );
    assert!(plan.kept.is_empty(), "{:?}", plan.kept);
}

fn plan_cards(cards: Vec<ForeignCard>) -> ForeignPlan {
    plan_from_cards(cards, &SchematicDocument::default(), |_| None)
}

fn axes(plan: &ForeignPlan) -> Vec<(RunSetDimensionKind, Vec<f64>)> {
    plan.sweeps
        .iter()
        .map(|(_, dimension)| (dimension.kind, dimension.canonical_values()))
        .collect()
}

#[test]
fn stepped_runs_are_run_set_axes_and_measurements_are_plan_measurements() {
    use ForeignCardRole::{Analysis, Measurement, Step};
    let mut document = SchematicDocument::default();
    document.components.push(
        rspice_design::schematic::component::Component::new(
            1,
            rspice_design::schematic::component_type::ComponentType::VoltageSource,
            Point::new(0, 0),
        )
        .with_name_value("V1".to_owned(), "DC 5".to_owned()),
    );
    let plan = plan_from_cards(
        vec![
            card(Measurement, ".meas peak MAX V(out) ; the highest output"),
            card(Analysis, ".tran 0 1m"),
            card(Step, ".step dec param C 1n 100n 1"),
            card(Step, ".step param X .1 .3 .1"),
            card(Step, ".step V1 list 1 2"),
            card(Step, ".step NPN 2N2222(Vaf) 50 100 25"),
            card(Step, ".step param C list 1 2"),
            card(Step, ".step V9 1 2 1"),
            card(Measurement, ".meas AC gain FIND V(out) AT 1k"),
            card(
                Measurement,
                ".measure tran fall FIND time WHEN V(out)=0.5 FALL=1",
            ),
            card(Measurement, ".meas TRAN late FIND V(out) AT fall"),
        ],
        &document,
        |_| None,
    );
    assert_eq!(
        axes(&plan),
        [
            (RunSetDimensionKind::Parameter, vec![1e-9, 1e-8, 1e-7]),
            (RunSetDimensionKind::Parameter, vec![0.1, 0.2, 0.3]),
            (RunSetDimensionKind::Source, vec![1.0, 2.0]),
        ]
    );
    let (_, stepped) = &plan.sweeps[1];
    assert_eq!(
        stepped
            .values
            .iter()
            .map(|value| value.lexical.as_str())
            .collect::<Vec<_>>(),
        ["100m", "200m", "300m"],
        "a sum's binary residue is not a value of its own"
    );
    assert_eq!(plan.sweeps[2].1.source, "netlist-source:V1");
    // A parameter the drawing only steps is a design variable at its first
    // value, so the reference point runs too.
    assert!(
        plan.variables
            .iter()
            .any(|(name, value)| name == "C" && value == "1n"),
        "{:?}",
        plan.variables
    );
    let statements = plan
        .measurements
        .iter()
        .map(|(_, definition)| {
            assert!(definition.define_measurement);
            definition.expression.as_str()
        })
        .collect::<Vec<_>>();
    assert_eq!(
        statements,
        [
            ".MEAS TRAN peak MAX V(out)",
            ".MEAS TRAN fall FIND time WHEN V(out)=0.5 FALL=1",
            ".MEAS TRAN late FIND V(out) AT fall"
        ]
    );
    let kept = plan
        .kept
        .iter()
        .map(|(card, reason)| {
            (
                card.text.split_whitespace().nth(1).unwrap_or_default(),
                reason.as_str(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(kept.len(), 4, "{kept:?}");
    assert!(kept[0].1.contains("model parameter"), "{kept:?}");
    assert!(kept[1].1.contains("already steps"), "{kept:?}");
    assert!(kept[2].1.contains("`V9`"), "{kept:?}");
    assert!(kept[3].1.contains("no AC analysis"), "{kept:?}");
}

#[test]
fn sweeps_and_measurements_without_an_analysis_stay_on_the_sheet() {
    let plan = plan_cards(vec![
        card(ForeignCardRole::Step, ".step param R list 1 2"),
        card(ForeignCardRole::Measurement, ".meas TRAN peak MAX V(out)"),
    ]);
    assert!(plan.sweeps.is_empty() && plan.measurements.is_empty());
    assert_eq!(plan.kept.len(), 2);
}

#[test]
fn an_analysis_the_plan_refuses_stays_on_the_sheet() {
    let plan = plan_from_cards(
        vec![card(ForeignCardRole::Analysis, ".tran 1m")],
        &SchematicDocument::default(),
        |_| Some("refused".to_owned()),
    );
    assert!(plan.analyses.is_empty());
    assert_eq!(plan.kept[0].1, "refused");
}

/// A drawing folder with an LTspice schematic, a symbol RSpice has no part
/// for beside it, and a model file it includes.
fn drawing_folder() -> PathBuf {
    let folder = std::env::temp_dir().join(format!(
        "rspice-ltspice-import-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time after epoch")
            .as_nanos()
    ));
    std::fs::create_dir_all(&folder).expect("drawing folder");
    std::fs::write(
        folder.join("buffer.asy"),
        "Version 4\nSymbolType CELL\nRECTANGLE Normal -32 -32 32 32\nSYMATTR Prefix X\n\
         SYMATTR Value BUF1\nPIN -32 0 NONE 0\nPINATTR PinName IN\nPINATTR SpiceOrder 1\n\
         PIN 32 0 NONE 0\nPINATTR PinName OUT\nPINATTR SpiceOrder 2\n",
    )
    .expect("symbol");
    std::fs::write(
        folder.join("buffers.lib"),
        ".subckt BUF1 in out\nE1 out 0 in 0 1\n.ends BUF1\n",
    )
    .expect("model file");
    std::fs::write(
        folder.join("filter.asc"),
        "Version 4\nSHEET 1 880 680\n\
         WIRE 96 96 32 96\nWIRE 208 96 176 96\nWIRE 288 96 208 96\nWIRE 320 96 288 96\n\
         FLAG 32 176 0\nFLAG 208 160 0\nFLAG 288 96 out\nFLAG 384 96 buffered\n\
         SYMBOL voltage 32 80 R0\nSYMATTR InstName V1\nSYMATTR Value PULSE(0 1 0 1n 1n 5m 10m)\n\
         SYMBOL res 192 80 R90\nSYMATTR InstName R1\nSYMATTR Value {R}\n\
         SYMBOL cap 192 96 R0\nSYMATTR InstName C1\nSYMATTR Value 1u\n\
         SYMBOL buffer 352 96 R0\nSYMATTR InstName U1\n\
         TEXT 32 240 Left 2 !.tran 0 20m 0 10u\\n.param R=1k\\n.include buffers.lib\n\
         TEXT 32 270 Left 2 !.step param R list 1k 2k\\n.meas TRAN vmax MAX V(out)\n\
         TEXT 32 300 Left 2 !.model D1N D(Is=1n)\\nK1 L1 L2 1\n",
    )
    .expect("schematic");
    folder
}

#[test]
fn an_imported_schematic_is_project_content_a_run_reads() {
    let folder = drawing_folder();
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    let library = app
        .state
        .model
        .workspace
        .content
        .active_view
        .library
        .clone();

    let summary =
        import_foreign_schematic_file(&mut app, &folder.join("filter.asc"), ForeignTool::Ltspice)
            .expect("the schematic imports");
    assert!(summary.contains("LTspice"), "{summary}");

    let model = &app.state.model;
    let log = model
        .log_buffer
        .entries()
        .map(|entry| entry.message.clone())
        .collect::<Vec<_>>();
    let cells = model
        .library_manager
        .get_library(&library)
        .expect("the active library")
        .cells
        .values()
        .map(|cell| cell.name.clone())
        .collect::<Vec<_>>();
    assert!(cells.iter().any(|cell| cell == "filter"), "{cells:?}");
    assert!(cells.iter().any(|cell| cell == "buffer"), "{cells:?}");
    // An empty project simulates what it imported.
    assert_eq!(model.workspace.content.project.top_cell, "filter");
    assert_eq!(model.workspace.content.active_view.cell, "filter");

    assert_eq!(
        model.sim_setup.active_plan_name().as_str(),
        "filter",
        "{log:#?}"
    );
    let plan = model
        .sim_setup
        .stable_analysis_plan()
        .expect("the imported plan");
    let [instance] = plan.instances() else {
        panic!("one analysis, not {}", plan.instances().len());
    };
    let AnalysisDraft::Transient(tran) = instance.draft() else {
        panic!("a transient analysis");
    };
    assert_eq!((tran.stop.as_str(), tran.max_step.as_str()), ("20m", "10u"));
    let variables = &model
        .workspace
        .content
        .plan_data(plan.id())
        .expect("the imported plan's payload")
        .design_variables;
    assert_eq!(
        variables
            .iter()
            .map(|variable| (variable.name.as_str(), variable.expression.as_str()))
            .collect::<Vec<_>>(),
        [("R", "1k")]
    );

    let libraries = model
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .map(|library| library.name.clone())
        .collect::<Vec<_>>();
    assert!(
        libraries.iter().any(|name| name == "filter"),
        "{libraries:?}"
    );
    assert!(
        libraries.iter().any(|name| name == "buffers"),
        "{libraries:?}"
    );

    let document = model.schematic.document();
    let notes = document
        .design_notes
        .iter()
        .map(|note| note.text.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        notes,
        [
            ".tran 0 20m 0 10u\n.param R=1k\n.include buffers.lib",
            ".step param R list 1k 2k\n.meas TRAN vmax MAX V(out)",
            ".model D1N D(Is=1n)\nK1 L1 L2 1",
        ]
    );

    // The stepped parameter is an axis of the plan's run set and the
    // measurement a plan measurement; both reach a prepared run as they are.
    let axes = model
        .sim_setup
        .run_set
        .enabled_dimensions()
        .map(|dimension| (dimension.source.clone(), dimension.canonical_values()))
        .collect::<Vec<_>>();
    assert_eq!(axes, [("design-variable:R".to_owned(), vec![1e3, 2e3])]);
    let measurements = model
        .workspace
        .content
        .plan_data(plan.id())
        .expect("the imported plan's payload")
        .specification_definitions
        .iter()
        .map(|definition| definition.expression.clone())
        .collect::<Vec<_>>();
    assert_eq!(measurements, [".MEAS TRAN vmax MAX V(out)"]);

    let checked = app
        .state
        .run_active_design_checks()
        .expect("the imported schematic is checked");
    assert!(!checked.has_errors(), "{checked:#?}");
    let prepared = crate::simulation::controller::SimulationController::new()
        .prepare_run_set_for_preflight(&app.state)
        .expect("the imported schematic prepares a run with no further step");
    assert_eq!((prepared.pvt_point_count, prepared.task_count), (2, 2));
    std::fs::remove_dir_all(&folder).ok();
}

/// A directive block stays on the sheet as the drawing lettered it, whole,
/// while the plan takes the analysis it holds: what a run reads and what the
/// sheet shows are separate.
#[test]
fn a_directive_block_stays_on_the_sheet_while_the_plan_takes_its_analysis() {
    let folder = drawing_folder();
    let file = folder.join("lines.asc");
    std::fs::write(
        &file,
        "Version 4\nSHEET 1 880 680\n\
         SYMBOL res 32 64 R0\nSYMATTR InstName R1\nSYMATTR Value 1k\n\
         TEXT 32 240 Left 2 !.tran 1m\\nK1 L1 L2 1\n",
    )
    .expect("schematic");
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    import_foreign_schematic_file(&mut app, &file, ForeignTool::Ltspice).expect("imports");
    std::fs::remove_dir_all(&folder).ok();
    let model = &app.state.model;
    let notes = &model.schematic.document().design_notes;
    assert!(
        notes.iter().any(|note| note.text == ".tran 1m\nK1 L1 L2 1"),
        "{notes:?}"
    );
    let plan = model
        .sim_setup
        .stable_analysis_plan()
        .expect("the imported plan");
    assert!(
        plan.instances()
            .iter()
            .any(|instance| matches!(instance.draft(), AnalysisDraft::Transient(_))),
        "the plan still holds the analysis"
    );
}

/// What a sheet paints, as text: every stroke and every run of lettering.
fn painted_sheet(state: &mut AppState) -> Vec<String> {
    let symbols = crate::schematic::symbols::SymbolLibrary::load_embedded().ok();
    let ctx = egui::Context::default();
    ctx.set_os(egui::os::OperatingSystem::Nix);
    crate::ui::Theme::default().apply(&ctx);
    let mut output = None;
    for _ in 0..3 {
        let editor = &mut state.model.schematic.session.editor;
        editor.needs_fit = false;
        editor.needs_drawing_sheet_fit = false;
        editor.zoom = 2.0;
        editor.pan = (40.0, 40.0);
        let input = crate::ui::raster::display_input(egui::vec2(1200.0, 900.0), 1.0);
        output = Some(ctx.run_ui(input, |ui| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ui, |ui| {
                    crate::schematic::view::render_schematic_view(ui, state, symbols.as_ref());
                });
        }));
    }
    fn walk(shape: &egui::Shape, out: &mut Vec<String>) {
        match shape {
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, out)),
            egui::Shape::Text(text) => out.push(format!("{:?} {}", text.pos, text.galley.text())),
            egui::Shape::LineSegment { .. } | egui::Shape::Path(_) | egui::Shape::Circle(_) => {
                out.push(format!("{shape:?}"));
            }
            _ => {}
        }
    }
    let mut painted = Vec::new();
    for shape in output.expect("three frames").shapes {
        walk(&shape.shape, &mut painted);
    }
    painted
}

/// A saved project holds everything its imported sheet draws: with the drawing
/// and every symbol it read gone, the sheet paints the same.
#[test]
fn an_imported_sheet_draws_the_same_with_its_source_gone() {
    use crate::state::ComponentType;
    use crate::workbench::workflows::project_workflow::{
        ProjectLoadOrigin, apply_loaded_project, save_project_to_path,
    };

    let folder = drawing_folder();
    std::fs::write(
        folder.join("res.asy"),
        "Version 4\nSymbolType CELL\n\
         LINE Normal 16 16 16 32\nLINE Normal 16 80 16 96\nRECTANGLE Normal 4 32 28 80\n\
         ARC Normal 0 30 32 62 4 56 28 36\n\
         WINDOW 0 40 40 Left 2\nWINDOW 3 40 72 Left 2\n\
         SYMATTR Value R\nSYMATTR Prefix R\n\
         PIN 16 16 NONE 0\nPINATTR PinName A\nPINATTR SpiceOrder 1\n\
         PIN 16 96 NONE 0\nPINATTR PinName B\nPINATTR SpiceOrder 2\n",
    )
    .expect("symbol");
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    import_foreign_schematic_file(&mut app, &folder.join("filter.asc"), ForeignTool::Ltspice)
        .expect("the schematic imports");
    let resistor = app
        .state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|part| part.kind == ComponentType::Resistor)
        .expect("the resistor");
    assert!(resistor.can_swap_to_rspice_symbol(), "it draws its source");
    let before = painted_sheet(&mut app.state);
    assert!(before.len() > 4, "the sheet paints its part: {before:?}");

    let project = std::env::temp_dir().join(format!(
        "rspice-imported-sheet-{}-{}.rspiceproj",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time after epoch")
            .as_nanos()
    ));
    assert!(save_project_to_path(&mut app.state, &project), "it saves");
    std::fs::remove_dir_all(&folder).expect("the drawing is gone");
    let loaded = crate::io::load_project_file(&project).expect("it reloads");

    let mut reopened = RSpiceApp::test_instance();
    assert!(apply_loaded_project(
        &mut reopened.state,
        loaded,
        ProjectLoadOrigin::BrowserImport("reopened.rspiceproj"),
    ));
    std::fs::remove_file(&project).ok();
    assert_eq!(painted_sheet(&mut reopened.state), before);
}

/// Imports `drawing` as `name`.asc and answers the sheet it lies on, and
/// whether everything drawn lies inside that sheet's drawing area and clear
/// of its title block.
fn import_onto_sheet(name: &str, drawing: &str) -> (String, bool, bool) {
    let folder = drawing_folder();
    let file = folder.join(format!("{name}.asc"));
    std::fs::write(&file, drawing).expect("schematic");
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    import_foreign_schematic_file(&mut app, &file, ForeignTool::Ltspice).expect("imports");
    std::fs::remove_dir_all(&folder).ok();
    let model = &mut app.state.model;
    let sheet = model.active_drawing_sheet();
    let own = model.active_sheet_catalog().is_some();
    let (min_x, min_y, max_x, max_y) = model
        .frame_symbol_context()
        .content_bounds(model.schematic.document())
        .expect("something drawn");
    let area = sheet.geometry.drawing_area;
    let inside = f64::from(min_x) >= area.min_x
        && f64::from(min_y) >= area.min_y
        && f64::from(max_x) <= area.max_x
        && f64::from(max_y) <= area.max_y;
    let clear = sheet
        .geometry
        .title_block
        .is_none_or(|block| f64::from(max_x) < block.min_x || f64::from(max_y) < block.min_y);
    let pins_on_grid = model
        .schematic
        .document()
        .components
        .iter()
        .flat_map(|part| part.terminal_positions())
        .all(|(_, point)| point.x % 10 == 0 && point.y % 10 == 0);
    assert!(pins_on_grid);
    (
        format!(
            "{} {}",
            sheet.format.authored_size.label(),
            sheet.format.orientation.label()
        ) + if own { " (own)" } else { "" },
        inside,
        clear,
    )
}

#[test]
fn an_imported_drawing_lies_inside_the_smallest_sheet_that_holds_it() {
    // Far from the origin, and small: the project's A4 sheet holds it.
    let small = "Version 4
SHEET 1 880 680
         WIRE -3984 -3008 -4096 -3008
FLAG -4096 -2928 0
FLAG -3984 -2928 0
         SYMBOL voltage -4096 -3024 R0
SYMATTR InstName V1
SYMATTR Value 1
         SYMBOL res -4000 -3024 R0
SYMATTR InstName R1
SYMATTR Value 1k
";
    assert_eq!(
        import_onto_sheet("small", small),
        ("ISO A4 landscape".to_owned(), true, true)
    );
    // Two parts 2400 units apart need more than A4 and A3 hold.
    let wide = "Version 4
SHEET 1 880 680
         WIRE 2416 0 0 0
FLAG 0 80 0
FLAG 2416 80 0
         SYMBOL voltage 0 -16 R0
SYMATTR InstName V1
SYMATTR Value 1
         SYMBOL res 2400 -16 R0
SYMATTR InstName R1
SYMATTR Value 1k
";
    let (sheet, inside, clear) = import_onto_sheet("wide", wide);
    assert!(sheet.ends_with("landscape (own)"), "{sheet}");
    assert!(
        !sheet.starts_with("ISO A4") && !sheet.starts_with("ISO A3"),
        "{sheet}"
    );
    assert!(inside && clear, "{sheet}");
}
