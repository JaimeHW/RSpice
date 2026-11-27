//! The Export result dataset… form read the way a reader meets it: the format list
//! in stored order, the analysis question a view of several strips asks, the
//! Plotted or All choice and the trace a PWL source asks for, each checked
//! through the AccessKit tree of a whole rendered frame.

use super::*;
use crate::workbench::EngineeringExportFormat;
use crate::workbench::documents::result_document::test_host::ResultsTestHost;

/// NumPy .npy: one array, so one analysis.
const NPY: usize = 6;

/// The picker's list is the stored choice's numbering, and its "one
/// analysis" flags agree with the formats the export refuses on a
/// multi-analysis view.
#[test]
fn the_picker_offers_every_format_the_export_writes_in_stored_order() {
    let mut state = ResultsTestHost::default();
    for (index, offered) in FORMATS.iter().enumerate() {
        state
            .model
            .preferences
            .set_choice(ChoicePreference::EngineeringExport, index)
            .expect("every listed format is a stored choice");
        let format = state
            .model
            .preferences
            .result_presentation_policy()
            .engineering_export();
        let holds_one = matches!(
            format,
            EngineeringExportFormat::TouchstoneWhereCompatible
                | EngineeringExportFormat::ValueChangeDump
                | EngineeringExportFormat::NumpyArray
                | EngineeringExportFormat::SpicePwl
        );
        assert_eq!(offered.one_analysis, holds_one, "format {index} disagrees");
    }
    assert!(
        state
            .model
            .preferences
            .set_choice(ChoicePreference::EngineeringExport, FORMATS.len())
            .is_err(),
        "the list covers the whole stored domain"
    );
}

/// Export stores the chosen format; Cancel stores nothing.
#[test]
fn export_remembers_the_format_and_cancel_does_not() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut state = ResultsTestHost::default();
    state.results.dataset_export = Some(DatasetExportPicker {
        format: 2,
        several_analyses: false,
    });
    let escape = egui::RawInput {
        events: vec![egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
        ..egui::RawInput::default()
    };
    let mut exported = false;
    let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
        exported |= show_dataset_export_picker(
            ctx,
            &mut state.results.dataset_export,
            &mut state.results.dataset_export_form,
            &mut state.model.preferences,
        )
        .is_some();
    });
    assert!(
        state.results.dataset_export.is_some(),
        "the picker stays open until it is answered"
    );
    let _ = ctx.run_ui(escape, |ctx| {
        exported |= show_dataset_export_picker(
            ctx,
            &mut state.results.dataset_export,
            &mut state.results.dataset_export_form,
            &mut state.model.preferences,
        )
        .is_some();
    });
    assert!(!exported);
    assert!(state.results.dataset_export.is_none());
    assert_eq!(
        state
            .model
            .preferences
            .choice(ChoicePreference::EngineeringExport),
        0,
        "a cancelled picker leaves the remembered format alone"
    );
}

/// Two transients on one strip stack, the second strip active, and the
/// second trace of each hidden. Each analysis holds a different value, so a
/// file says which one it came from.
fn two_strip_state() -> ResultsTestHost {
    use crate::state::{AnalysisResult, AnalysisType, SimulationRun};
    use crate::workbench::documents::result_document::test_fixtures::waveform;

    let mut run = SimulationRun::new(7);
    for id in 1..=2_u64 {
        run.add_analysis(
            AnalysisResult::new(id, AnalysisType::Transient, "Transient").with_waveforms(vec![
                waveform("V(out)", vec![0.0, 1.0e-6], vec![0.0, id as f64]),
                waveform("V(mid)", vec![0.0, 1.0e-6], vec![0.0, 0.5 * id as f64]),
            ]),
        );
    }
    let mut state = ResultsTestHost::default();
    state.model.simulation.retained.runs = vec![run].into();
    state.model.simulation.view.active_run_idx = Some(0);
    state.model.simulation.view.active_analysis_idx = Some(1);
    let dataset_id = state.model.simulation.retained.runs[0].dataset_id;
    state.workbench.documents.activate(
        crate::workbench::state::WorkspaceDocumentId::ResultDataset(dataset_id),
    );
    state.results.results.session.viewer = crate::workbench::ResultViewer::Waves;
    for analysis in 0..2 {
        crate::workbench::documents::result_document::toggle_visibility(
            &mut state.results_parts(),
            analysis,
            1,
        );
    }
    state
}

/// Open the form over `state`'s view on `format`, as Export dataset… does.
fn open_form(state: &mut ResultsTestHost, format: usize) {
    let mut picker = dataset_export_picker(state.results_reads()).expect("a displayed view");
    picker.format = format;
    state.results.dataset_export = Some(picker);
    state.results.dataset_export_form = Some(Box::new(dataset_export_form(state.results_reads())));
}

/// Every AccessKit node the form publishes over a few frames.
fn form_nodes(
    state: &mut ResultsTestHost,
) -> std::collections::HashMap<egui::accesskit::NodeId, egui::accesskit::Node> {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut nodes = std::collections::HashMap::new();
    for _ in 0..3 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1440.0, 900.0),
                )),
                ..Default::default()
            },
            |ctx| {
                show_dataset_export_picker(
                    ctx,
                    &mut state.results.dataset_export,
                    &mut state.results.dataset_export_form,
                    &mut state.model.preferences,
                );
            },
        );
        nodes.extend(
            output
                .platform_output
                .accesskit_update
                .expect("AccessKit tree update")
                .nodes,
        );
    }
    nodes
}

/// Every text the form paints on its last frame, notes included.
fn painted(state: &mut ResultsTestHost) -> Vec<String> {
    fn texts(shape: &egui::Shape, into: &mut Vec<String>) {
        match shape {
            egui::Shape::Text(text) => into.push(text.galley.text().to_owned()),
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| texts(shape, into)),
            _ => {}
        }
    }
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut painted = Vec::new();
    for _ in 0..3 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1440.0, 900.0),
                )),
                ..Default::default()
            },
            |ctx| {
                show_dataset_export_picker(
                    ctx,
                    &mut state.results.dataset_export,
                    &mut state.results.dataset_export_form,
                    &mut state.model.preferences,
                );
            },
        );
        painted.clear();
        for clipped in &output.shapes {
            texts(&clipped.shape, &mut painted);
        }
    }
    painted
}

fn spoken(
    nodes: &std::collections::HashMap<egui::accesskit::NodeId, egui::accesskit::Node>,
) -> Vec<String> {
    nodes
        .values()
        .filter_map(|node| node.label().or(node.value()).map(str::to_owned))
        .collect()
}

/// A one-analysis format over two strips asks which analysis, offering the
/// active strip's, and no longer sends the reader to maximize one; the
/// signal choice counts what each answer writes.
#[test]
fn a_view_of_two_strips_asks_which_analysis_and_offers_the_active_one() {
    let mut state = two_strip_state();
    open_form(&mut state, NPY);
    let nodes = form_nodes(&mut state);
    let analysis = nodes
        .values()
        .find(|node| {
            node.role() == egui::accesskit::Role::ComboBox && node.label() == Some("Analysis")
        })
        .expect("the form asks which analysis");
    assert_eq!(analysis.value(), Some("Transient (2)"));
    let words = spoken(&nodes);
    assert!(
        words.iter().any(|word| word == "Plotted (1)")
            && words.iter().any(|word| word == "All (2)"),
        "{words:?}"
    );
    assert!(
        !words.iter().any(|word| word.contains("Maximize")),
        "{words:?}"
    );
}

/// Over the members of a `.step`, the form offers all of them at once, which a table writes as
/// one long table; a format that holds one analysis says it writes the member chosen.
#[test]
fn a_stepped_view_offers_all_members_and_a_one_member_format_says_so() {
    use rspice_results::provenance::{AnalysisResultFamilyMember, AnalysisResultStepCoordinate};

    let mut state = two_strip_state();
    for (index, analysis) in state.model.simulation.retained.runs[0]
        .analyses
        .iter_mut()
        .enumerate()
    {
        analysis.import_source = Some(rspice_results::result_import::ResultImportSource {
            source_name: "stepped.raw".to_owned(),
            format: rspice_results::result_import::ResultImportFormat::SpiceRaw,
            coordinate: None,
            family_member: Some(AnalysisResultFamilyMember::StepPoint {
                index,
                coordinate: vec![AnalysisResultStepCoordinate {
                    name: "r".to_owned(),
                    value: 1000.0 * (index + 1) as f64,
                    unit: String::new(),
                }],
            }),
            event_dump: None,
        });
    }
    let analysis_question =
        |nodes: &std::collections::HashMap<egui::accesskit::NodeId, egui::accesskit::Node>| {
            nodes
                .values()
                .find(|node| {
                    node.role() == egui::accesskit::Role::ComboBox
                        && node.label() == Some("Analysis")
                })
                .and_then(|node| node.value().map(str::to_owned))
        };

    open_form(&mut state, 0);
    let nodes = form_nodes(&mut state);
    // The choice counts the members; the signals are counted once, as each member holds them.
    assert_eq!(
        analysis_question(&nodes).as_deref(),
        Some("All members (2)")
    );
    let words = spoken(&nodes);
    assert!(
        words.iter().any(|word| word == "Plotted (1)")
            && words.iter().any(|word| word == "All (2)"),
        "{words:?}"
    );
    let texts = painted(&mut state);
    assert!(!texts.iter().any(|text| text == ONE_MEMBER_NOTE));
    assert!(
        !texts
            .iter()
            .any(|text| FORMATS.iter().any(|format| format.hint == Some(text))),
        "CSV needs no hint: {texts:?}"
    );

    open_form(&mut state, NPY);
    let nodes = form_nodes(&mut state);
    // The active strip's member, named by its parameter value.
    assert_eq!(
        analysis_question(&nodes).as_deref(),
        Some("Transient \u{00b7} r=2k")
    );
    let texts = painted(&mut state);
    assert!(
        texts.iter().any(|text| text == ONE_MEMBER_NOTE),
        "{texts:?}"
    );
    // One array has no names: the reader is told before choosing it.
    let hint = FORMATS[NPY]
        .hint
        .expect("NumPy array states its column order");
    assert!(texts.iter().any(|text| text == hint), "{texts:?}");
}

/// The form opens with no field focused, so no choice wears the focus ring
/// before the reader has touched the keyboard.
#[test]
fn the_form_opens_with_no_field_focused() {
    let mut state = two_strip_state();
    open_form(&mut state, NPY);
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut nodes = std::collections::HashMap::new();
    let mut focus = None;
    for _ in 0..3 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1440.0, 900.0),
                )),
                ..Default::default()
            },
            |ctx| {
                show_dataset_export_picker(
                    ctx,
                    &mut state.results.dataset_export,
                    &mut state.results.dataset_export_form,
                    &mut state.model.preferences,
                );
            },
        );
        let update = output
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update");
        focus = Some(update.focus);
        nodes.extend(update.nodes);
    }
    let fields = nodes
        .iter()
        .filter(|(_, node)| node.role() == egui::accesskit::Role::ComboBox)
        .map(|(id, _)| *id)
        .collect::<Vec<_>>();
    assert!(fields.len() >= 2, "Format and Analysis");
    assert!(
        !fields.contains(&focus.expect("a frame ran")),
        "a field took the focus at open"
    );
}

/// A view of one analysis has nothing to ask about the analysis.
#[test]
fn a_view_of_one_analysis_asks_no_analysis_question() {
    let mut state = two_strip_state();
    let run = state.model.simulation.active_run().unwrap();
    state.results.results.session.maximized_strip = Some(
        crate::workbench::documents::result_document::AnalysisPresentationKey::new(
            run.dataset_id,
            &run.analyses[0],
        ),
    );
    open_form(&mut state, RAW_BINARY);
    let words = spoken(&form_nodes(&mut state));
    assert!(!words.iter().any(|word| word == "Analysis"), "{words:?}");
    assert!(words.iter().any(|word| word == "Plotted (1)"), "{words:?}");
}

/// The analysis written is the one chosen: for a one-analysis format the
/// active strip's by default, another when the reader picks it, and every
/// displayed one for a format left on "Every displayed analysis".
#[test]
fn the_choice_names_the_analysis_the_reader_picked() {
    let mut state = two_strip_state();
    open_form(&mut state, NPY);
    let form = state.results.dataset_export_form.as_mut().unwrap();
    assert_eq!(form.choice(NPY).analysis, Some(1), "the active strip");
    assert_eq!(form.choice(0).analysis, None, "CSV writes the stack");
    assert_eq!(
        form.choice(RAW_BINARY).analysis,
        None,
        "raw writes a plot of each"
    );
    form.analysis = Some(0);
    assert_eq!(form.choice(NPY).analysis, Some(0));
    assert_eq!(form.choice(0).analysis, Some(0), "CSV of one strip");
}

/// Exporting through the form writes the chosen analysis of a stacked view
/// in a one-analysis format, its plotted traces or all of them.
#[test]
fn the_form_exports_the_chosen_analysis_and_signals() {
    let mut state = two_strip_state();
    state
        .model
        .preferences
        .set_choice(ChoicePreference::EngineeringExport, RAW_ASCII)
        .unwrap();
    for (analysis, signals, names, out) in [
        (0, ExportedSignals::Plotted, vec!["V(out)"], 1.0),
        (1, ExportedSignals::All, vec!["V(out)", "V(mid)"], 2.0),
    ] {
        let io = crate::workbench::documents::result_document::test_fixtures::MockExportWorkflowIo::default();
        super::super::action_export_dataset_with_io(
            &mut state.results_parts(),
            &io,
            &DatasetExportChoice {
                analysis: Some(analysis),
                signals,
                trace: None,
            },
        );
        let files = io.byte_files.borrow();
        assert_eq!(files.len(), 1, "{:?}", state.prompt_text());
        let reopened = crate::workbench::workflows::result_import_workflow::parse_result_dataset(
            "waveforms.raw",
            &files[0].1,
        )
        .unwrap();
        assert_eq!(
            reopened
                .waveforms
                .iter()
                .map(|waveform| waveform.name.as_str())
                .collect::<Vec<_>>(),
            names
        );
        assert_eq!(
            reopened.waveforms[0].y.read_whole().unwrap().as_ref(),
            &[0.0, out]
        );
    }
}

/// With nothing in the analysis plotted, Plotted would write an empty file:
/// the form says so and holds Export back until the reader chooses All.
#[test]
fn nothing_plotted_holds_export_back_and_says_why() {
    let mut state = two_strip_state();
    for analysis in 0..2 {
        crate::workbench::documents::result_document::toggle_visibility(
            &mut state.results_parts(),
            analysis,
            0,
        );
    }
    open_form(&mut state, RAW_BINARY);
    let nodes = form_nodes(&mut state);
    let export = nodes
        .values()
        .find(|node| {
            node.role() == egui::accesskit::Role::Button && node.label() == Some("Export\u{2026}")
        })
        .expect("the Export button");
    assert!(export.is_disabled());
    let words = spoken(&nodes);
    assert!(
        words.iter().any(|word| word.contains("Choose All")),
        "{words:?}"
    );
    state.results.dataset_export_form.as_mut().unwrap().signals = ExportedSignals::All;
    let nodes = form_nodes(&mut state);
    assert!(
        !nodes
            .values()
            .find(|node| node.label() == Some("Export\u{2026}"))
            .unwrap()
            .is_disabled()
    );
}

/// A PWL source asks which trace in place of the signal choice, offering
/// the plotted one first, and writes the one picked.
#[test]
fn a_pwl_source_asks_for_its_trace_in_place_of_the_signals() {
    let mut state = two_strip_state();
    open_form(&mut state, PWL);
    let nodes = form_nodes(&mut state);
    let trace = nodes
        .values()
        .find(|node| {
            node.role() == egui::accesskit::Role::ComboBox && node.label() == Some("Trace")
        })
        .expect("the form asks which trace");
    assert_eq!(trace.value(), Some("V(out)"));
    let words = spoken(&nodes);
    assert!(
        !words.iter().any(|word| word.starts_with("Plotted")),
        "{words:?}"
    );
    let form = state.results.dataset_export_form.as_mut().unwrap();
    assert_eq!(form.choice(PWL).trace.as_deref(), Some("V(out)"));
    form.trace = Some("V(mid)".to_owned());
    assert_eq!(form.choice(PWL).trace.as_deref(), Some("V(mid)"));
    assert_eq!(
        form.choice(RAW_BINARY).trace,
        None,
        "only a PWL source names one"
    );
}

/// Over a frequency sweep a PWL source has no trace to take: the form says
/// so and holds Export back.
#[test]
fn a_pwl_source_over_a_frequency_sweep_says_why_it_cannot() {
    use crate::state::{AnalysisResult, AnalysisType, SimulationRun};
    use crate::workbench::documents::result_document::test_fixtures::waveform;

    let mut run = SimulationRun::new(3);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "AC Analysis").with_waveforms(vec![
            waveform("|V(out)|", vec![1.0e3, 1.0e4], vec![1.0, 0.5]).with_complex_components(
                "V(out)",
                vec![1.0, 0.3],
                vec![0.0, -0.4],
            ),
        ]),
    );
    let mut state = ResultsTestHost::default();
    state.model.simulation.retained.runs = vec![run].into();
    state.model.simulation.view.active_run_idx = Some(0);
    state.model.simulation.view.active_analysis_idx = Some(0);
    let dataset_id = state.model.simulation.retained.runs[0].dataset_id;
    state.workbench.documents.activate(
        crate::workbench::state::WorkspaceDocumentId::ResultDataset(dataset_id),
    );
    state.results.results.session.viewer = crate::workbench::ResultViewer::Bode;
    open_form(&mut state, PWL);
    let nodes = form_nodes(&mut state);
    assert!(
        nodes
            .values()
            .find(|node| node.label() == Some("Export\u{2026}"))
            .is_some_and(egui::accesskit::Node::is_disabled)
    );
    let words = spoken(&nodes);
    assert!(
        words.iter().any(|word| word.contains("waveform in time")),
        "{words:?}"
    );
}
