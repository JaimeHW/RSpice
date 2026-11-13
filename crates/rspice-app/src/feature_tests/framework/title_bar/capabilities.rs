//! Tests for what the menus reach beyond the plain edits: selecting every
//! object of a kind, stepping back through the views a canvas showed,
//! highlighting a net, copying a picture of the drawing and annotating a
//! transient's voltages.

use super::*;
use crate::schematic::view::sheet_visibility::SchematicObjectKind;
use crate::state::{
    ComponentType, DesignNote, DesignNoteKind, DocumentationShape, DocumentationShapeGeometry,
    Point, Selection,
};
use crate::workbench::framework::requests::ClipboardImage;
use crate::workbench::hardcopy::commands::HardcopyCommand;

/// A sheet holding objects of every kind Edit ▸ Select ▸ names, each kind's
/// expected selection beside it.
fn sheet_with_every_kind() -> (RSpiceApp, Vec<(SchematicObjectKind, Selection)>) {
    let mut app = title_test_app();
    let schematic = &mut app.state.model.schematic;
    let resistor = schematic.add_component(ComponentType::Resistor, Point::new(0, 0));
    let ground = schematic.add_component(ComponentType::Ground, Point::new(0, 100));
    let pin = schematic.add_component(ComponentType::Port, Point::new(-100, 0));
    let wire = schematic
        .add_wire(vec![Point::new(-100, 0), Point::new(-40, 0)])
        .expect("a wire is added");
    let label = schematic.add_net_label(Point::new(-70, 0), "IN".to_owned());
    let document = schematic.document_mut_for_test();
    document.design_notes.push(
        DesignNote::new(900, Point::new(0, 200), DesignNoteKind::PlainText, "note")
            .expect("valid note"),
    );
    document.documentation_shapes.push(
        DocumentationShape::new(
            901,
            DocumentationShapeGeometry::Line {
                start: Point::new(0, 300),
                end: Point::new(100, 300),
            },
        )
        .expect("valid line"),
    );
    let expect = |fill: &dyn Fn(&mut Selection)| {
        let mut selection = Selection::default();
        fill(&mut selection);
        selection
    };
    let expected = vec![
        (
            SchematicObjectKind::Instances,
            expect(&|selection| {
                selection.select_component(resistor);
                selection.select_component(ground);
            }),
        ),
        (
            SchematicObjectKind::Wires,
            expect(&|selection| selection.select_wire(wire)),
        ),
        (
            SchematicObjectKind::NetLabels,
            expect(&|selection| selection.select_net_label(label)),
        ),
        (
            SchematicObjectKind::Pins,
            expect(&|selection| selection.select_component(pin)),
        ),
        (
            SchematicObjectKind::Text,
            expect(&|selection| selection.select_design_note(900)),
        ),
        (
            SchematicObjectKind::Shapes,
            expect(&|selection| selection.select_documentation_shape(901)),
        ),
    ];
    (app, expected)
}

/// Each Edit ▸ Select ▸ row selects every object of its kind on the sheet
/// and nothing else, in place of what was selected, whatever the selection
/// filter lets a click pick.
#[test]
fn select_by_kind_selects_exactly_that_kind() {
    let (mut app, expected) = sheet_with_every_kind();
    assert_eq!(
        expected.iter().map(|(kind, _)| *kind).collect::<Vec<_>>(),
        SchematicObjectKind::ALL
    );
    app.state.design.schematic_selection_filter.instances = false;
    app.execute(Command::SelectAll.key());
    for (kind, selection) in expected {
        let key = DesignCommand::SelectOfKind(kind).key();
        assert_eq!(
            app.availability(key),
            CommandAvailability::Available,
            "{kind:?}"
        );
        app.execute(key);
        assert_eq!(
            app.state.model.schematic.session.editor.selection, selection,
            "{kind:?}"
        );
    }
}

/// A kind the sheet has none of is offered greyed, saying so.
#[test]
fn select_by_kind_is_grey_for_a_kind_the_sheet_lacks() {
    let mut app = title_test_app();
    app.state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(0, 0));
    assert!(app.is_enabled(DesignCommand::SelectOfKind(SchematicObjectKind::Instances).key()));
    assert_eq!(
        app.availability(DesignCommand::SelectOfKind(SchematicObjectKind::Pins).key()),
        CommandAvailability::Disabled("the sheet has no pins")
    );
}

/// One frame of the schematic canvas, `time` seconds into the session.
fn canvas_frame(ctx: &Context, app: &mut RSpiceApp, time: f64) {
    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(800.0, 600.0),
            )),
            time: Some(time),
            ..Default::default()
        },
        |ui| {
            crate::schematic::view::render_schematic_view(
                ui,
                &mut crate::schematic::view::canvas_frame(&mut app.state),
                None,
            )
        },
    );
}

/// Zoom previous steps the canvas back through the views it showed, newest
/// first, and is grey once there is none left to go back to.
#[test]
fn zoom_previous_steps_back_through_the_views_the_canvas_showed() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = title_test_app();
    app.state.workbench.activate(Workspace::Design);
    let view = |app: &RSpiceApp| {
        let editor = &app.state.model.schematic.session.editor;
        (editor.zoom, editor.pan)
    };
    let key = Command::ZoomPrevious.key();
    canvas_frame(&ctx, &mut app, 0.0);
    let opened = view(&app);
    assert_eq!(
        app.availability(key),
        CommandAvailability::Disabled("the view has not changed yet")
    );

    app.execute(Command::ZoomIn.key());
    canvas_frame(&ctx, &mut app, 1.0);
    let nearer = view(&app);
    app.execute(Command::ZoomIn.key());
    canvas_frame(&ctx, &mut app, 2.0);
    assert_ne!(view(&app), nearer);
    assert_ne!(nearer, opened);

    assert!(app.is_enabled(key));
    app.execute(key);
    canvas_frame(&ctx, &mut app, 3.0);
    assert_eq!(view(&app), nearer);
    app.execute(key);
    canvas_frame(&ctx, &mut app, 4.0);
    assert_eq!(view(&app), opened);
    assert!(!app.is_enabled(key));
}

/// The Results workspace over a finished transient, `V(out)` on the Waves
/// sheet with its pane active, and the key of the analysis it draws.
fn waveform_sheet_app() -> (
    RSpiceApp,
    crate::workbench::documents::result_document::AnalysisPresentationKey,
) {
    use crate::state::{AnalysisResult, AnalysisType, RetainedTrace};
    use crate::workbench::documents::result_document::{
        AnalysisPresentationKey, WavePanePresentationKey,
    };
    let mut app = title_test_app();
    app.state.open_project_session();
    app.state.workbench.activate(Workspace::Results);
    let time: Vec<f64> = (0..=200).map(|step| f64::from(step) * 5.0e-6).collect();
    let volts: Vec<f64> = time
        .iter()
        .map(|t| 5.0 + 0.9 * (std::f64::consts::TAU * 2.0e3 * t).sin())
        .collect();
    app.state.model.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient")
            .with_waveforms(vec![RetainedTrace::new("V(out)", time, volts, "#ffbd2e")]),
    );
    app.state.model.simulation.view.active_analysis_idx = Some(0);
    let run = app
        .state
        .model
        .simulation
        .active_run()
        .expect("retained run");
    let analysis = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
    let session = &mut app.state.results.results.session;
    session.viewer = crate::workbench::ResultViewer::Waves;
    session.active_wave_pane = Some(WavePanePresentationKey {
        analysis,
        unit: "V".to_owned(),
    });
    (app, analysis)
}

/// One frame of the Results workspace, `time` seconds into the session.
fn results_frame(ctx: &Context, app: &mut RSpiceApp, time: f64) {
    let selected_specification = app.state.simulate.selected_specification.clone();
    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1200.0, 800.0),
            )),
            time: Some(time),
            ..Default::default()
        },
        |ui| {
            crate::workbench::documents::result_document::show(
                ui,
                &mut app.state.results_parts(),
                &mut app.simulation_controller,
                selected_specification.as_deref(),
            );
        },
    );
}

/// Zoom previous steps a waveform sheet back through the windows it showed,
/// newest first, the way it steps a drawing back, and is grey once there is
/// none left to go back to.
#[test]
fn zoom_previous_steps_a_waveform_sheet_back_through_the_views_it_showed() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let (mut app, analysis) = waveform_sheet_app();
    let view = |app: &RSpiceApp| {
        let pane = app.state.results.results.session.analysis_plot_view_pane(
            crate::workbench::ResultViewer::Waves,
            analysis,
            0,
        );
        (pane.x, pane.y)
    };
    let key = Command::ZoomPrevious.key();
    results_frame(&ctx, &mut app, 0.0);
    let opened = view(&app);
    assert_eq!(
        app.availability(key),
        CommandAvailability::Disabled("the view has not changed yet")
    );

    app.execute(Command::ZoomIn.key());
    results_frame(&ctx, &mut app, 1.0);
    let nearer = view(&app);
    app.execute(Command::ZoomIn.key());
    results_frame(&ctx, &mut app, 2.0);
    assert_ne!(view(&app), nearer);
    assert_ne!(nearer, opened);

    assert!(app.is_enabled(key));
    app.execute(key);
    results_frame(&ctx, &mut app, 3.0);
    assert_eq!(view(&app), nearer);
    app.execute(key);
    results_frame(&ctx, &mut app, 4.0);
    assert_eq!(view(&app), opened);
    assert!(!app.is_enabled(key));
}

/// Highlight net lights the net a selected wire is on, the one the canvas
/// lights when the wire is clicked, and says why it cannot when the
/// selection names no net or more than one.
#[test]
fn highlight_net_lights_the_net_the_selection_is_on() {
    let mut app = title_test_app();
    rspice_app_state::workbench::examples::load_example(
        "RC Lowpass Filter",
        &mut app.state.model.schematic,
    );
    let key = DesignCommand::HighlightNet.key();
    assert_eq!(
        app.availability(key),
        CommandAvailability::Disabled("select a wire, a net label or a pin")
    );

    app.execute(DesignCommand::SelectOfKind(SchematicObjectKind::Wires).key());
    assert_eq!(
        app.availability(key),
        CommandAvailability::Disabled("the selection is on more than one net")
    );

    let wire = app.state.model.schematic.document().wires[0].id;
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_wire(wire);
    assert!(app.is_enabled(key));
    app.execute(key);
    let highlight = &app.state.model.schematic.session.editor.net_highlight;
    assert!(highlight.active);
    assert!(highlight.highlighted_wires.contains(&wire));
    assert_eq!(
        highlight.selected_net_name.as_deref(),
        crate::schematic::view::selected_net(
            &app.state.model,
            app.state.design.schematic_visibility.net_highlighting,
        )
        .ok()
        .as_deref()
    );
}

/// Copy as image draws the selection, or with nothing selected the whole
/// sheet in view, through the hardcopy renderer and leaves the picture for
/// the clipboard: ink on a white border, the selection's picture the smaller,
/// the status bar saying what was copied.
#[test]
fn copy_as_image_draws_the_selection_or_else_the_view() {
    let mut app = title_test_app();
    let schematic = &mut app.state.model.schematic;
    let resistor = schematic.add_component(ComponentType::Resistor, Point::new(0, 0));
    schematic
        .add_wire(vec![Point::new(-300, 0), Point::new(-40, 0)])
        .expect("a wire is added");
    app.state.workbench.activate(Workspace::Design);
    let reference = app.state.model.workspace.content.active_view.clone();
    app.state
        .workbench
        .documents
        .activate(crate::workbench::state::WorkspaceDocumentId::CellView(
            reference,
        ));
    let key = HardcopyCommand::CopyAsImage.key();
    assert_eq!(app.availability(key), CommandAvailability::Available);
    let copy = |app: &mut RSpiceApp| {
        app.execute(key);
        let ClipboardImage(image) = app
            .state
            .workbench
            .outbox
            .take::<ClipboardImage>()
            .unwrap_or_else(|| {
                panic!(
                    "no picture: {:?}",
                    app.state.workbench.prompt.as_ref().map(|p| &p.text)
                )
            });
        image
    };

    let view = copy(&mut app);
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_component(resistor);
    let selection = copy(&mut app);
    for image in [&view, &selection] {
        let [width, height] = image.size;
        assert_eq!(image.pixels[0], egui::Color32::WHITE);
        assert_eq!(image.pixels[width * height - 1], egui::Color32::WHITE);
        assert!(
            image
                .pixels
                .iter()
                .any(|pixel| pixel.r() < 128 && pixel.a() == 255),
            "a picture with no ink"
        );
    }
    assert!(
        selection.size[0] < view.size[0] / 2 && selection.size[1] < view.size[1] / 2,
        "the selection {:?} against the view {:?}",
        selection.size,
        view.size
    );
    assert_eq!(
        app.state
            .workbench
            .prompt
            .as_ref()
            .map(|prompt| &*prompt.text),
        Some(&*format!(
            "Copied the selection as a {} \u{00d7} {} picture.",
            selection.size[0], selection.size[1]
        ))
    );
}

/// Annotate transient voltages is a check-mark row beside the DC one: it
/// takes the DC operating point's place on the canvas, says in the status bar
/// which instant it reads (or why there is none), and turns off again.
#[test]
fn annotate_transient_voltages_takes_the_operating_points_place() {
    let mut app = title_test_app();
    let transient = DesignCommand::AnnotateTransientVoltages.key();
    let dc = DesignCommand::AnnotateOperatingPoint.key();
    assert!(app.is_enabled(transient));
    assert_eq!(
        (app.checked(transient), app.checked(dc)),
        (Some(false), Some(true))
    );

    app.execute(transient);
    assert_eq!(
        (app.checked(transient), app.checked(dc)),
        (Some(true), Some(false))
    );
    assert_eq!(
        app.state
            .workbench
            .prompt
            .as_ref()
            .map(|prompt| &*prompt.text),
        Some("No transient result is selected: run a transient analysis to annotate its voltages.")
    );

    app.execute(transient);
    assert_eq!(
        (app.checked(transient), app.checked(dc)),
        (Some(false), Some(false))
    );
}
