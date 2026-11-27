//! A saved result document prints as it shows: made with New result
//! document from a run, its pane prints the strips, values, captions and
//! key the same run's sheet prints, and Export takes the same page.

use super::availability::app_showing;
use crate::feature_tests::hardcopy::{hardcopy_parts, resolve_retained_hardcopy_source};
use crate::workbench::RSpiceApp;
use rspice_app_hardcopy::workbench::app::dialogs::hardcopy::publish::active_retained_source_selection;
use rspice_app_hardcopy::workbench::app::dialogs::hardcopy::{HardcopySection, HardcopyWorkflow};
use rspice_app_results::workbench::documents::result_document::ResultViewer;
use rspice_app_results::workbench::documents::result_document::create_document::commit;
use rspice_app_state::product::{ContentDigest, ResultDocumentId};
use rspice_app_state::results::visualization_document::{
    AnnotationAnchor, AxisOrientation, AxisScale, DocumentEdit, EntityRef, NewAxis, NewTrace,
};
use rspice_app_state::state::{
    AnalysisResult, AnalysisType, RetainedTrace, SimulationRun, SimulationRunLifecycle,
};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::state::workbench_session::WorkspaceDocumentId;
use rspice_hardcopy::render::{
    HardcopyRenderer, HardcopyScene, HardcopySceneMetadata, SceneFont, ScenePrimitive,
    scene_from_resolved,
};
use rspice_hardcopy::sources::{
    HardcopySemanticDocument, ResolvedHardcopyDocument, SemanticAxisKind, SemanticPlot,
};
use rspice_hardcopy_contract::{
    BackgroundMode, Bleed, ColorMapping, DecorationSetup, DuplexMode, FontPolicy, HardcopyPlan,
    HardcopyPlanId, HardcopyScope, HardcopySetup, Length, Orientation, OutputFormat, PageMargins,
    PaperSize, PhysicalPageSetup, PrinterJobSettings, PrinterMediaSource, PrinterRasterGeometry,
    RenderSetup, RenderTarget, ScaleMode, SchematicHardcopySetup, StandardPaper, TilingMode,
    TilingSetup, Watermark,
};
use rspice_results::document_creation::ResultDocumentLayout;
use rspice_results_ui::create_document::CreateResultDocumentDialogState;

/// A finished transient keeping a voltage swinging 0.45 V to 2.85 V and a
/// current swinging -1.6 mA to 2.4 mA: two units on one sheet.
fn mixed_units_run() -> SimulationRun {
    let time: Vec<f64> = (0..400).map(|index| f64::from(index) * 0.25e-9).collect();
    let phase = |t: f64| t * 1.0e9 * std::f64::consts::TAU / 25.0;
    let volts: Vec<f64> = time.iter().map(|&t| 1.65 + 1.2 * phase(t).sin()).collect();
    let amps: Vec<f64> = time
        .iter()
        .map(|&t| 2.0e-3 * phase(t).cos() + 0.4e-3)
        .collect();
    let mut run = SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            RetainedTrace::new("V(out)", time.clone(), volts, "#00aaff").with_unit("V"),
            RetainedTrace::new("I(R1)", time, amps, "#ffaa00").with_unit("A"),
        ]),
    );
    run.lifecycle = SimulationRunLifecycle::Completed;
    run
}

/// A finished AC sweep of a one-pole low pass, 1 Hz to 1 MHz, keeping its
/// linear magnitude and its phase as an AC run retains them.
fn low_pass_run() -> SimulationRun {
    let frequency: Vec<f64> = (0..=60)
        .map(|index| 10f64.powf(f64::from(index) / 10.0))
        .collect();
    let ratio = |f: f64| f / 1.0e3;
    let magnitude: Vec<f64> = frequency
        .iter()
        .map(|&f| 1.0 / (1.0 + ratio(f).powi(2)).sqrt())
        .collect();
    let phase: Vec<f64> = frequency
        .iter()
        .map(|&f| -ratio(f).atan().to_degrees())
        .collect();
    let mut run = SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(vec![
            RetainedTrace::new("|V(out)|", frequency.clone(), magnitude, "#00aaff"),
            RetainedTrace::new("phase(V(out))", frequency, phase, "#ffaa00").with_unit("\u{b0}"),
        ]),
    );
    run.lifecycle = SimulationRunLifecycle::Completed;
    run
}

/// An open project in Results holding `run` as its only run, shown.
fn app_holding(run: SimulationRun, viewer: ResultViewer) -> RSpiceApp {
    let mut app = app_showing(viewer);
    app.state.model.simulation.retained.runs = vec![run].into();
    assert!(app.state.model.simulation.select_run(0));
    assert!(app.state.model.simulation.select_analysis(0));
    app.state.results.results.session.viewer = viewer;
    app
}

/// New result document over the run's first analysis, on `viewer`, plotting
/// `signals`, committed as the form commits it.
fn new_document(
    app: &mut RSpiceApp,
    name: &str,
    viewer: &str,
    signals: &[&str],
) -> ResultDocumentId {
    let run = app.state.model.simulation.retained.runs[0].clone();
    app.state.results.create_result_document = CreateResultDocumentDialogState {
        open: true,
        name: name.to_owned(),
        name_touched: true,
        dataset_id: Some(run.dataset_id),
        analysis: Some(0),
        viewer_id: viewer.to_owned(),
        layout: ResultDocumentLayout::SinglePane,
        signals: signals.iter().map(|signal| (*signal).to_owned()).collect(),
        validation_error: None,
    };
    let document = commit(&mut app.state.results_parts()).expect("the document commits");
    assert_eq!(
        app.state.workbench.documents.active(Workspace::Results),
        Some(&WorkspaceDocumentId::VisualizationDocument(document))
    );
    document
}

/// What File > Print takes from the Results workspace as it stands, through
/// the dialog's own source selection and the retained path its worker runs.
fn printed(app: &mut RSpiceApp) -> ResolvedHardcopyDocument {
    let (source_key, scope) =
        active_retained_source_selection(&hardcopy_parts(app)).expect("Print has a source");
    resolve_retained_hardcopy_source(&app.state, &source_key, scope).expect("the page resolves")
}

/// The run's own sheet printed, as File > Print takes it with the run shown.
fn run_sheet_printed(app: &mut RSpiceApp) -> ResolvedHardcopyDocument {
    let dataset = app.state.model.simulation.retained.runs[0].dataset_id;
    let mut shown = app.state.clone();
    shown
        .workbench
        .documents
        .activate(WorkspaceDocumentId::ResultDataset(dataset));
    let source_key = format!(
        "project:{}:result-dataset:{dataset}",
        shown.model.workspace.content.project.id().as_uuid()
    );
    resolve_retained_hardcopy_source(&shown, &source_key, HardcopyScope::ActivePlotDocument)
        .expect("the run's sheet prints")
}

fn plot_of(resolved: &ResolvedHardcopyDocument) -> &SemanticPlot {
    let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
        panic!("a pane prints as a plot")
    };
    plot
}

/// The page compiled.
fn page_scene(resolved: &ResolvedHardcopyDocument) -> HardcopyScene {
    scene_from_resolved(
        resolved,
        resolved.default_print_mapping(),
        SchematicHardcopySetup::default(),
        HardcopySceneMetadata::try_new(resolved.authority().display_name(), "RSpice").unwrap(),
    )
    .expect("the page compiles")
}

/// Every word the compiled page letters.
fn page_text(resolved: &ResolvedHardcopyDocument) -> Vec<String> {
    page_scene(resolved)
        .primitives()
        .iter()
        .filter_map(|primitive| match primitive {
            ScenePrimitive::Text { text, .. } => Some(text.clone()),
            _ => None,
        })
        .collect()
}

/// The values an axis of the page names.
fn values(plot: &SemanticPlot, axis: SemanticAxisKind) -> Vec<&str> {
    plot.axis_ticks
        .iter()
        .filter(|tick| tick.axis == axis && !tick.label.is_empty())
        .map(|tick| tick.label.as_str())
        .collect()
}

/// The unit each strip states on its top value, top strip first.
fn strip_units(plot: &SemanticPlot) -> Vec<String> {
    plot.frames
        .iter()
        .map(|frame| {
            plot.axis_ticks
                .iter()
                .filter(|tick| {
                    tick.axis == SemanticAxisKind::Vertical
                        && (frame.minimum.y_um..=frame.maximum.y_um).contains(&tick.start.y_um)
                })
                .filter(|tick| !tick.label.is_empty())
                .min_by_key(|tick| tick.start.y_um)
                .map(|tick| match tick.label.split_once(' ') {
                    Some((_, unit)) => unit.to_owned(),
                    // A degree stands against its number.
                    None => tick
                        .label
                        .trim_start_matches(|c: char| {
                            c.is_ascii_digit() || "+-\u{2212}.".contains(c)
                        })
                        .to_owned(),
                })
                .unwrap_or_default()
        })
        .collect()
}

/// A saved transient document of a voltage and a current prints the page
/// the run's sheet prints: a volt strip and a milliamp strip under one time
/// axis, each valued, the abscissa named with its unit and the traces keyed.
#[test]
fn a_saved_transient_document_prints_the_strips_its_run_sheet_prints() {
    let mut app = app_holding(mixed_units_run(), ResultViewer::Waves);
    let sheet = run_sheet_printed(&mut app);
    new_document(
        &mut app,
        "Mixed units",
        "viewer-waveform",
        &["V(out)", "I(R1)"],
    );
    let page = printed(&mut app);
    let plot = plot_of(&page);

    assert_eq!(
        page.authority().display_name(),
        "Mixed units \u{b7} Waveform"
    );
    assert_eq!(plot.frames.len(), 2, "one strip per unit");
    assert_eq!(
        strip_units(plot),
        ["V", "mA"],
        "{:?}",
        values(plot, SemanticAxisKind::Vertical)
    );
    assert!(values(plot, SemanticAxisKind::Horizontal).len() >= 3);
    assert!(values(plot, SemanticAxisKind::Vertical).len() >= 6);
    let text = page_text(&page);
    assert!(text.iter().any(|text| text == "TIME \u{b7} s"), "{text:?}");
    // The abscissa's name is set as the screen sets it beside its values: in
    // their face, a size smaller and fainter.
    let scene = page_scene(&page);
    let lettered = |words: &str| {
        scene
            .primitives()
            .iter()
            .find_map(|primitive| match primitive {
                ScenePrimitive::Text {
                    text,
                    font,
                    size,
                    color,
                    ..
                } if text == words => Some((*font, *size, *color)),
                _ => None,
            })
            .unwrap_or_else(|| panic!("{words} is lettered"))
    };
    let (font, size, color) = lettered("TIME \u{b7} s");
    let (value_font, value_size, value_color) =
        lettered(values(plot, SemanticAxisKind::Horizontal)[1]);
    assert!(matches!(font, SceneFont::Monospace) && font == value_font);
    assert!(size < value_size, "{size:?} against {value_size:?}");
    assert_ne!(color, value_color);
    let keyed = |plot: &SemanticPlot| {
        plot.traces
            .iter()
            .map(|trace| (trace.key_label().to_owned(), trace.palette_slot))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        keyed(plot)
            .into_iter()
            .map(|(key, _)| key)
            .collect::<Vec<_>>(),
        ["V(out)", "I(R1)"]
    );

    // The same axes, values, strips, curves and key as the run's own sheet.
    let sheet_plot = plot_of(&sheet);
    assert_eq!(plot.frames, sheet_plot.frames);
    assert_eq!(plot.axis_ticks, sheet_plot.axis_ticks);
    assert_eq!(plot.captions, sheet_plot.captions);
    assert_eq!(keyed(plot), keyed(sheet_plot));
    for (printed, sheet) in plot.traces.iter().zip(&sheet_plot.traces) {
        assert_eq!(
            printed.paths, sheet.paths,
            "{} is drawn alike",
            printed.label
        );
    }
}

/// A saved AC document's Bode pane prints its frequency on decades, named
/// with its unit, its magnitude in decibels and its phase in degrees.
#[test]
fn a_saved_ac_document_prints_its_frequency_on_decades() {
    let mut app = app_holding(low_pass_run(), ResultViewer::Bode);
    let sheet = run_sheet_printed(&mut app);
    new_document(
        &mut app,
        "Low pass",
        "viewer-bode",
        &["|V(out)|", "phase(V(out))"],
    );
    let page = printed(&mut app);
    let plot = plot_of(&page);

    assert_eq!(plot.x_scale, AxisScale::Logarithmic);
    let decades = values(plot, SemanticAxisKind::Horizontal);
    for decade in ["10", "100", "1k", "10k", "100k"] {
        assert!(decades.contains(&decade), "{decade} is named: {decades:?}");
    }
    assert_eq!(
        strip_units(plot),
        ["dB", "\u{b0}"],
        "{:?}",
        values(plot, SemanticAxisKind::Vertical)
    );
    let text = page_text(&page);
    assert!(text.iter().any(|text| text == "FREQ \u{b7} Hz"), "{text:?}");

    // The same decades, strips and values as the run's own Bode sheet.
    let sheet_plot = plot_of(&sheet);
    assert_eq!(plot.frames, sheet_plot.frames);
    assert_eq!(plot.axis_ticks, sheet_plot.axis_ticks);

    // The figures the response card reads print under the plot, and the
    // gain is read against its 0 dB line, as on the run's own Bode sheet.
    assert_eq!(plot.captions, sheet_plot.captions);
    assert!(
        plot.captions
            .iter()
            .any(|caption| caption.text.contains("bandwidth")),
        "{:?}",
        plot.captions
    );
    assert_eq!(plot.guides, sheet_plot.guides);
    let references = plot
        .guides
        .iter()
        .filter(|guide| guide.reference)
        .collect::<Vec<_>>();
    assert_eq!(references.len(), 1, "one 0 dB line, on the gain strip");
    let gain = &plot.frames[0];
    assert!(
        references[0]
            .paths
            .iter()
            .flatten()
            .all(|point| (gain.minimum.y_um..=gain.maximum.y_um).contains(&point.y_um)),
        "the 0 dB line runs across the gain strip"
    );
}

/// Pin a note to a place a quarter down the current's strip, halfway
/// across it, and another to the point 50 ns, 1 V of the voltage's data.
fn pin_notes(app: &mut RSpiceApp, document_id: ResultDocumentId) {
    let document = app
        .state
        .model
        .workspace
        .content
        .visualization_documents
        .iter_mut()
        .find(|document| document.id() == document_id)
        .expect("the project owns it");
    let pane_id = document.panes()[0].id;
    document
        .transact(
            document.revision(),
            vec![
                DocumentEdit::AddAnnotation {
                    pane_id,
                    anchor: AnnotationAnchor::Pane {
                        x_fraction: 0.5,
                        y_fraction: 0.25,
                        unit: Some("A".to_owned()),
                    },
                    text: "Settled".to_owned(),
                },
                DocumentEdit::AddAnnotation {
                    pane_id,
                    anchor: AnnotationAnchor::Data {
                        x: 50.0e-9,
                        y: 1.0,
                        unit: "V".to_owned(),
                    },
                    text: "Mid-rail".to_owned(),
                },
            ],
        )
        .expect("the notes are pinned");
}

/// A note pinned to a place in the pane prints in the plot of the strip it
/// names, its words starting from that place, and a note pinned to a point
/// of the data prints on that point, as the screen stands each of them.
#[test]
fn notes_pinned_to_a_place_or_a_point_print_where_the_screen_stands_them() {
    let mut app = app_holding(mixed_units_run(), ResultViewer::Waves);
    let document_id = new_document(&mut app, "Notes", "viewer-waveform", &["V(out)", "I(R1)"]);
    pin_notes(&mut app, document_id);
    let page = printed(&mut app);
    let plot = plot_of(&page);
    let note = |text: &str| {
        plot.annotations
            .iter()
            .find(|annotation| annotation.text == text)
            .unwrap_or_else(|| panic!("{text} prints: {:?}", plot.annotations))
    };

    let settled = note("Settled");
    assert!(settled.placed, "a place note is lettered where it stands");
    let at = settled.position.expect("it has a place");
    let current = &plot.frames[1];
    let quarter = current.minimum.y_um + (current.maximum.y_um - current.minimum.y_um) / 4;
    let middle = (current.minimum.x_um + current.maximum.x_um) / 2;
    assert!(
        (quarter..quarter + 3_000).contains(&at.y_um),
        "a quarter down the current's strip: {at:?} in {current:?}"
    );
    assert!(
        (middle..middle + 2_000).contains(&at.x_um),
        "from halfway across it: {at:?} in {current:?}"
    );

    let mid_rail = note("Mid-rail");
    assert!(!mid_rail.placed, "a point note is led off its point");
    assert_eq!(mid_rail.source_x_bits, Some(50.0e-9_f64.to_bits()));
    assert_eq!(mid_rail.source_y_bits, Some(1.0_f64.to_bits()));
    let at = mid_rail.position.expect("its point is in the window");
    let volts = &plot.frames[0];
    assert!((volts.minimum.y_um..=volts.maximum.y_um).contains(&at.y_um));
    let across =
        (at.x_um - volts.minimum.x_um) as f64 / (volts.maximum.x_um - volts.minimum.x_um) as f64;
    assert!(
        (across - 50.0 / 99.75).abs() < 0.01,
        "50 ns along a 99.75 ns window: {across}"
    );

    let text = page_text(&page);
    for words in ["Settled", "Mid-rail"] {
        assert!(text.iter().any(|text| text == words), "{words}: {text:?}");
    }

    // The point is marked with a dot, as the screen marks it, where the
    // leader to its words starts.
    let scene = page_scene(&page);
    let primitives = scene.primitives();
    let words = primitives
        .iter()
        .find_map(|primitive| match primitive {
            ScenePrimitive::Text { origin, text, .. } if text == "Mid-rail" => Some(*origin),
            _ => None,
        })
        .expect("the point's note is lettered");
    let point = primitives
        .iter()
        .find_map(|primitive| match primitive {
            ScenePrimitive::Line { from, to, .. } if *to == words => Some(*from),
            _ => None,
        })
        .expect("a leader runs to its words");
    assert!(
        primitives.iter().any(|primitive| matches!(
            primitive,
            ScenePrimitive::Circle { center, fill: Some(_), .. } if *center == point
        )),
        "a dot marks the point"
    );
}

/// A pane that reads its current against a second, right-hand axis prints,
/// each unit in its own strip, and a strip the reader set logarithmic is
/// ruled by its decades.
#[test]
fn a_pane_with_a_second_vertical_axis_prints() {
    let mut app = app_holding(mixed_units_run(), ResultViewer::Waves);
    let document_id = new_document(
        &mut app,
        "Two axes",
        "viewer-waveform",
        &["V(out)", "I(R1)"],
    );
    let document = app
        .state
        .model
        .workspace
        .content
        .visualization_documents
        .iter_mut()
        .find(|document| document.id() == document_id)
        .expect("the project owns it");
    let current = document
        .traces()
        .iter()
        .find(|trace| trace.label == "I(R1)")
        .cloned()
        .expect("the current is plotted");
    let volts = document
        .traces()
        .iter()
        .find(|trace| trace.label == "V(out)")
        .map(|trace| trace.y_axis_id)
        .expect("the voltage is plotted");
    let created = document
        .transact(
            document.revision(),
            vec![DocumentEdit::AddAxis(NewAxis {
                pane_id: current.pane_id,
                label: "Current".to_owned(),
                orientation: AxisOrientation::VerticalRight,
                scale: AxisScale::Linear,
                unit: Some("A".to_owned()),
                range: None,
            })],
        )
        .unwrap();
    let EntityRef::Axis(right) = created.created[0] else {
        panic!("an axis is created")
    };
    document
        .transact(
            document.revision(),
            vec![
                DocumentEdit::Remove(EntityRef::Trace(current.id)),
                DocumentEdit::AddTrace(NewTrace {
                    pane_id: current.pane_id,
                    binding: current.binding,
                    signal_key: current.signal_key.to_string(),
                    coordinate_key: current.coordinate_key.to_string(),
                    row_predicates: current.row_predicates.to_vec(),
                    x_axis_id: current.x_axis_id,
                    y_axis_id: right,
                    label: current.label.clone(),
                }),
                DocumentEdit::SetAxisScale {
                    axis_id: volts,
                    scale: AxisScale::Logarithmic,
                },
            ],
        )
        .unwrap();

    let page = printed(&mut app);
    let plot = plot_of(&page);
    assert_eq!(
        strip_units(plot),
        ["V", "mA"],
        "{:?}",
        values(plot, SemanticAxisKind::Vertical)
    );
    // A logarithmic strip carries its interior mantissas as minor rules; a
    // linear one rules only the values it names.
    let minor = |strip: usize| {
        let frame = &plot.frames[strip];
        plot.axis_ticks.iter().any(|tick| {
            tick.axis == SemanticAxisKind::Vertical
                && !tick.major
                && (frame.minimum.y_um..=frame.maximum.y_um).contains(&tick.start.y_um)
        })
    };
    assert!(minor(0), "the volt strip is ruled by its decades");
    assert!(!minor(1), "the milliamp strip stays linear");
}

/// Export takes the page Print takes, and both PDF and SVG carry it.
#[test]
fn a_saved_document_exports_the_page_it_prints_as_pdf_and_svg() {
    let mut app = app_holding(mixed_units_run(), ResultViewer::Waves);
    new_document(
        &mut app,
        "Mixed units",
        "viewer-waveform",
        &["V(out)", "I(R1)"],
    );
    let page = printed(&mut app);
    for format in [OutputFormat::PdfVector, OutputFormat::SvgVector] {
        let publication = HardcopyRenderer::render_resolved(
            &plan(&page, format),
            &page,
            HardcopySceneMetadata::try_new(page.authority().display_name(), "RSpice").unwrap(),
        )
        .unwrap_or_else(|error| panic!("{format:?} renders: {error:?}"));
        let bytes = publication
            .parts()
            .iter()
            .flat_map(|part| part.bytes().iter().copied())
            .collect::<Vec<_>>();
        assert!(!bytes.is_empty(), "{format:?} writes a file");
        if format == OutputFormat::SvgVector {
            let svg = String::from_utf8(bytes).expect("an SVG is text");
            assert!(svg.contains("TIME \u{b7} s"), "the SVG names its abscissa");
            assert!(svg.contains("mA"), "the SVG states the current's unit");
        }
    }
}

/// File > Print opens ready to print, on a saved document and on the run's
/// own sheet alike. The system printer is sent each page as pixels, so a
/// fresh dialog asks for no selectable text the printer cannot keep: the
/// only thing it waits on is a printer, and once one is chosen the page
/// previews and Print is enabled.
#[test]
#[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
fn a_fresh_print_dialog_previews_the_page_and_enables_print() {
    let mut app = app_holding(mixed_units_run(), ResultViewer::Waves);
    let sheet = run_sheet_printed(&mut app);
    new_document(
        &mut app,
        "Mixed units",
        "viewer-waveform",
        &["V(out)", "I(R1)"],
    );
    let document = printed(&mut app);
    for page in [document, sheet] {
        let name = page.authority().display_name().to_owned();
        let dialog = &mut app.state.hardcopy.hardcopy;
        dialog
            .open_resolved(HardcopyWorkflow::Print, page.clone(), None)
            .expect("Print opens");
        assert_eq!(dialog.format, OutputFormat::NativePrinter, "{name}");
        assert!(
            !dialog.searchable_text && !dialog.embed_fonts,
            "{name}: a printer page keeps no text"
        );
        assert_eq!(
            dialog.error.as_deref(),
            Some("choose a printer to print to"),
            "{name}: a printer is all Print waits on"
        );
        assert_eq!(dialog.error_section, Some(HardcopySection::Output));

        dialog.set_printer(
            "Microsoft Print to PDF".to_owned(),
            PrinterJobSettings::try_new(
                ContentDigest::from_bytes([0x50; 32]),
                "1",
                PrinterRasterGeometry::try_new(1_000, 800, 0, 0, 1_000, 800).unwrap(),
                PrinterMediaSource::AutomaticCompatibleTray,
                600,
                DuplexMode::Off,
                1,
                false,
            )
            .unwrap(),
        );
        assert_eq!(dialog.error, None, "{name}");
        let plan = dialog
            .preview_plan
            .clone()
            .unwrap_or_else(|| panic!("{name}: Print is enabled"));
        let preview = HardcopyRenderer::render_preview_pages_resolved(
            &plan,
            &page,
            HardcopySceneMetadata::try_new(&name, "RSpice").unwrap(),
            &[0],
            72,
            || false,
        )
        .unwrap_or_else(|error| panic!("{name} previews: {error:?}"));
        assert_eq!(preview.len(), 1, "{name}: the first page previews");
        dialog.close();
    }
}

/// A landscape Letter page fitted to its printable area, rendered to
/// `format`.
fn plan(source: &ResolvedHardcopyDocument, format: OutputFormat) -> HardcopyPlan {
    let vector = !matches!(format, OutputFormat::Png { .. });
    let setup = HardcopySetup::try_new(
        PhysicalPageSetup::try_new(
            PaperSize::Standard(StandardPaper::Letter),
            PageMargins::uniform(Length::from_micrometres(10_000)),
            Bleed::None,
            Orientation::Landscape,
        )
        .unwrap(),
        ScaleMode::FitPrintableArea,
        TilingSetup::try_new(TilingMode::Automatic, Length::ZERO, true).unwrap(),
        RenderSetup::try_new(
            RenderTarget::ExportArtifact,
            format,
            ColorMapping::PrintSafeEngineeringPalette,
            BackgroundMode::White,
            // A vector file keeps its words as text, as Export writes it.
            FontPolicy::new(vector, vector),
            true,
        )
        .unwrap(),
        DecorationSetup::try_new(true, true, true, Watermark::None).unwrap(),
        source.default_print_mapping().clone(),
    )
    .unwrap();
    let extent = source.content_extent_for_setup(setup.schematic()).unwrap();
    HardcopyPlan::compile_with_id(
        HardcopyPlanId::try_from_uuid(uuid::Uuid::from_u128(7)).unwrap(),
        source.authority().clone(),
        setup,
        extent,
    )
    .unwrap()
}
