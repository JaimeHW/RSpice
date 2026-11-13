//! Smith chart source qualification, selection and cache coordination.

use std::sync::Arc;

use egui::Ui;
use num_complex::Complex64;
use rspice_results::network_matrix::trace_identity;
use rspice_results::network_matrix::two_port::TwoPortSweep;

use super::{ResultsParts, ResultsReads};
use crate::state::{
    ActiveViewer, SpecializedViewerAnalysisIdentity, SpecializedViewerCacheProvenance,
};
#[cfg(test)]
use crate::workbench::documents::result_document::test_host::ResultsTestHost;

use super::frame_work;
use rspice_results_ui::smith_chart::circles::{ChartCircle, TwoPortFigures};
use rspice_results_ui::smith_chart::view::{self, SmithBarOffer, SmithPlotTrace};

/// Whether an analysis retains a complex AC or harmonic-balance response the
/// Smith sheet can read against the pane's Z₀.
fn retains_response(analysis: &crate::state::AnalysisResult) -> bool {
    rspice_results::renderability::retains_smith_response(analysis)
}

pub(super) fn analysis_is_renderable(analysis: &crate::state::AnalysisResult) -> bool {
    rspice_results::renderability::smith_analysis_is_renderable(analysis, frame_work::note_scan)
        || (retains_response(analysis) && analysis.validate_retained_evidence().is_ok())
}

/// What the tab strip says about this sheet, in the sheet's own words.
///
/// The verdict comes from the workspace memo rather than a fresh walk, which
/// is why it is spelled through the structural gate and not through
/// [`analysis_is_renderable`].
pub(super) fn availability(state: ResultsReads<'_>) -> super::ViewerAvailability {
    if state.model.simulation.active_run().is_some_and(|run| {
        state
            .model
            .simulation
            .active_analysis()
            .is_some_and(|analysis| {
                (retains_response(analysis)
                    || super::analysis_answers_structural_gate(
                        state,
                        run.dataset_id,
                        analysis,
                        super::StructuralGate::SParameterStructure,
                    ))
                    && super::analysis_evidence_is_valid(state, run.dataset_id, analysis)
            })
    }) {
        super::ViewerAvailability::available(
            "Retained complex traces are available: network coefficients against their port \
             impedances, or responses against the pane's Z₀",
        )
    } else {
        super::ViewerAvailability::unavailable(
            "Requires a complex trace: SP, PSP or HBSP coefficients, or an AC or HB response",
        )
    }
}

/// The same question with the retained-evidence verdict left to the caller.
///
/// Split out because the per-frame callers — the tab strip's availability
/// gate and the cache synchronizer — take that verdict from the workspace
/// memo instead of walking every retained complex sample again.
pub(super) fn structure_is_renderable(analysis: &crate::state::AnalysisResult) -> bool {
    rspice_results::renderability::smith_structure_is_renderable(analysis, frame_work::note_scan)
        || retains_response(analysis)
}

fn clear(state: &mut ResultsParts<'_>) -> bool {
    state.analysis.smith_chart_state.clear_traces();
    state
        .analysis
        .clear_specialized_viewer_cache_authority(ActiveViewer::SmithChart);
    false
}

/// Rebuild the mutable drawing cache solely from the selected immutable
/// result. Cache provenance makes repeated frames a no-op and prevents a
/// same-shape result from inheriting another run's RF traces.
pub fn synchronize_active_analysis(state: &mut ResultsParts<'_>) -> bool {
    if state
        .results_reads()
        .viewer_capability(ActiveViewer::SmithChart)
        .available
    {
        return true;
    }
    let Some(run) = state.model.simulation.active_run() else {
        return clear(state);
    };
    let Some(analysis) = state.model.simulation.active_analysis() else {
        return clear(state);
    };
    if !structure_is_renderable(analysis)
        || !super::analysis_evidence_is_valid(state.results_reads(), run.dataset_id, analysis)
    {
        return clear(state);
    }
    let provenance = SpecializedViewerCacheProvenance::for_analysis(run.dataset_id, analysis);
    let mut waveforms = analysis.waveforms.clone();
    waveforms.sort_by(|left, right| left.name.cmp(&right.name));
    state.analysis.smith_chart_state.clear_traces();
    if let Some(crate::state::AnalysisResultFamilyMetadata::SParameter {
        reference_impedances_ohm,
        ..
    }) = analysis.family_metadata.as_ref()
        && rspice_results::renderability::smith_structure_is_renderable(analysis, |_| {})
    {
        let reference_impedances_ohm = reference_impedances_ohm.clone();
        for waveform in waveforms {
            let Some(identity) = trace_identity(&waveform.name) else {
                continue;
            };
            let port_count = if identity.physical_ports {
                reference_impedances_ohm.len()
            } else {
                reference_impedances_ohm.len() / 2
            };
            if identity.output_port > port_count || identity.input_port > port_count {
                continue;
            }
            let reference_impedance_ohm = (identity.physical_ports
                && identity.output_port == identity.input_port)
                .then(|| reference_impedances_ohm[identity.output_port - 1]);
            let Some(complex) = waveform.complex.as_ref() else {
                continue;
            };
            if state
                .analysis
                .smith_chart_state
                .load_sparam_data(
                    &waveform.name,
                    &waveform.x,
                    &complex.real,
                    &complex.imag,
                    reference_impedance_ohm,
                )
                .is_err()
            {
                return clear(state);
            }
        }
    } else {
        // Every retained complex response, so the sheet can list them; a
        // response whose sweep the chart cannot place is left out whole
        // rather than drawn in part.
        let analysis_type = analysis.analysis_type;
        for waveform in waveforms {
            if !matches!(
                rspice_results::renderability::polar_quantity_source(
                    analysis_type,
                    None,
                    &waveform
                ),
                Some(rspice_results::renderability::PolarQuantitySource::Response)
            ) {
                continue;
            }
            let Some(complex) = waveform.complex.as_ref() else {
                continue;
            };
            let _ = state.analysis.smith_chart_state.load_response_data(
                &waveform.name,
                &waveform.x,
                &complex.real,
                &complex.imag,
            );
        }
    }
    if state.analysis.smith_chart_state.traces.is_empty() {
        return clear(state);
    }
    state
        .analysis
        .bind_specialized_viewer_cache(ActiveViewer::SmithChart, provenance);
    true
}

/// The cache owner of the selected result: the key every derived column of
/// this sheet carries, so selecting another analysis of the same run never
/// hands it the previous one's samples.
fn owner_key(state: &ResultsParts<'_>) -> u64 {
    state
        .model
        .active_specialized_viewer_cache_provenance()
        .map_or(0, |provenance| {
            use std::hash::{Hash, Hasher};
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            provenance.dataset_id.hash(&mut hasher);
            match provenance.analysis_identity {
                SpecializedViewerAnalysisIdentity::Prepared(id) => {
                    id.hash(&mut hasher);
                }
                SpecializedViewerAnalysisIdentity::LegacyResultId(id) => id.hash(&mut hasher),
            }
            hasher.finish()
        })
}

fn text_key(text: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    text.hash(&mut hasher);
    hasher.finish()
}

/// An evaluated locus: frequencies, then the real and imaginary components.
type EvaluatedLocus = (Vec<f64>, Vec<f64>, Vec<f64>);

/// Evaluate a reader's expression on the active analysis as a complex
/// locus: frequencies and rectangular components. A real-valued result is a
/// locus on the real axis; a scalar is not a locus at all.
fn evaluate_expression(
    analysis: &crate::state::AnalysisResult,
    text: &str,
) -> Result<EvaluatedLocus, String> {
    use crate::analysis::calculator;
    let expression =
        calculator::parser::try_parse(text).map_err(|error| format!("parse error: {error}"))?;
    let context = calculator::WaveformsContext::with_policy(
        &analysis.waveforms,
        crate::state::ComplexExpressionPolicy::Rectangular,
    );
    let value = calculator::evaluator::evaluate(&expression, &context)
        .map_err(|error| error.to_string())?;
    let waveform = rspice_results::calculator::retained::evaluated_waveform(value, text, None)
        .map_err(|_| "a constant is not a locus".to_owned())?;
    let (real, imag) = match &waveform.complex {
        Some(complex) => (complex.real.to_vec(), complex.imag.to_vec()),
        None => (waveform.y.to_vec(), vec![0.0; waveform.y.len()]),
    };
    let frequencies = waveform.x.to_vec();
    if frequencies.is_empty() || frequencies.len() != real.len() || frequencies.len() != imag.len()
    {
        return Err("the expression produced no samples".to_owned());
    }
    if !frequencies.windows(2).all(|pair| pair[0] < pair[1]) {
        return Err("the expression's sweep is not strictly increasing".to_owned());
    }
    Ok((frequencies, real, imag))
}

/// The two-port of the selected analysis, resolved once per owner.
fn two_port(state: &mut ResultsParts<'_>, owner: u64) -> Option<Arc<TwoPortSweep>> {
    let sheet = &mut state.results.session.smith;
    if let Some((cached_owner, sweep)) = &sheet.two_port_cache
        && *cached_owner == owner
    {
        return sweep.clone();
    }
    let sweep = state
        .model
        .simulation
        .active_analysis()
        .and_then(|analysis| TwoPortSweep::resolve(&**analysis))
        .map(Arc::new);
    state.results.session.smith.two_port_cache = Some((owner, sweep.clone()));
    sweep
}

/// The sheet's controls, left-aligned in its bar.
pub(super) fn domain_bar(ui: &mut Ui, state: &mut ResultsParts<'_>) -> bool {
    let owner = owner_key(state);
    let sweep = two_port(state, owner);
    let retained = state
        .analysis
        .smith_chart_state
        .traces
        .iter()
        .filter(|trace| trace.response)
        .map(|trace| trace.name.clone())
        .collect::<Vec<_>>();
    let is_response_sheet = state
        .model
        .simulation
        .active_analysis()
        .is_some_and(retains_response);
    let offer = SmithBarOffer {
        responses: is_response_sheet.then_some(retained.as_slice()),
        two_port: sweep.is_some(),
        noise: sweep.as_ref().is_some_and(|sweep| sweep.noise.is_some()),
    };
    let entered = view::domain_bar(ui, &mut state.results.session.smith, offer);
    if let Some(text) = entered {
        let outcome = state
            .model
            .simulation
            .active_analysis()
            .ok_or_else(|| "no analysis is selected".to_owned())
            .and_then(|analysis| evaluate_expression(analysis, &text));
        let sheet = &mut state.results.session.smith;
        match outcome {
            Ok(_) if sheet.responses.contains(&text) => {}
            Ok(_) => sheet.responses.push(text),
            Err(error) => {
                sheet.expression_error = Some(format!("{text}: {error}"));
                sheet.expression_draft = Some(text);
            }
        }
    }
    true
}

/// One frame's traces, circles and figures.
struct Prepared<'a> {
    traces: Vec<SmithPlotTrace<'a>>,
    figures: Option<TwoPortFigures>,
    circles: Vec<ChartCircle>,
}

/// Build the frame's traces from the drawing cache and the reader's list,
/// with every Γ column cached under a key that names what it means.
fn prepare<'a>(
    smith: &'a rspice_results_ui::smith_chart::SmithChartState,
    sheet: &'a rspice_results_ui::smith_chart::SmithSheetState,
    derived: &mut rspice_results_ui::derived::DerivedSeries,
    analysis: Option<&crate::state::AnalysisResult>,
    owner: u64,
    sweep: Option<&TwoPortSweep>,
    pin: Option<(usize, usize)>,
) -> Prepared<'a> {
    let mut traces = Vec::new();
    let empty = || Arc::new(Vec::new());
    // Network coefficients: every retained term, in the inventory's order.
    //
    // The cache and the traces live in disjoint halves of the session, so the
    // arrays are built by reference. The key carries the cache's owning result
    // as well as the trace ordinal: selecting another analysis of the same run
    // reloads the loci without moving the data version, and a bare ordinal
    // would hand the new selection the previous one's coefficients.
    for (index, trace) in smith
        .traces
        .iter()
        .enumerate()
        .filter(|(_, trace)| !trace.response && !trace.points.is_empty())
    {
        let points = &trace.points;
        let key = owner ^ ((index as u64) << 8);
        let frequencies = derived.get_or(key ^ 0x501_0002, || {
            Arc::new(points.iter().map(|p| p.frequency).collect::<Vec<_>>())
        });
        let real = derived.get_or(key ^ 0x501_0000, || {
            Arc::new(points.iter().map(|p| p.s.re).collect::<Vec<_>>())
        });
        let imaginary = derived.get_or(key ^ 0x501_0001, || {
            Arc::new(points.iter().map(|p| p.s.im).collect::<Vec<_>>())
        });
        traces.push(SmithPlotTrace {
            index,
            name: &trace.name,
            frequencies,
            real,
            imaginary,
            reference_impedance_ohm: trace.reference_impedance_ohm,
            visible: trace.visible && !sheet.hidden.contains(&trace.name),
            removable: false,
            cache_key: rspice_ui_kit::plot::trace_cache_key(0x501_00F0, index),
        });
    }
    // Responses the reader listed, read through the pane's reading and Z₀.
    let reading = sheet.reading;
    let reference = sheet.reference_ohm;
    let reading_key = ((reading.index() as u64) << 56) ^ reference.to_bits().rotate_left(17);
    for (position, text) in sheet.responses.iter().enumerate() {
        let key = owner ^ text_key(text).rotate_left(9);
        let retained = smith
            .traces
            .iter()
            .find(|trace| trace.response && trace.name == *text);
        let (frequencies, raw_real, raw_imag) = if let Some(trace) = retained {
            let points = &trace.points;
            (
                derived.get_or(key ^ 0x502_0002, || {
                    Arc::new(points.iter().map(|p| p.frequency).collect::<Vec<_>>())
                }),
                derived.get_or(key ^ 0x502_0000, || {
                    Arc::new(points.iter().map(|p| p.s.re).collect::<Vec<_>>())
                }),
                derived.get_or(key ^ 0x502_0001, || {
                    Arc::new(points.iter().map(|p| p.s.im).collect::<Vec<_>>())
                }),
            )
        } else {
            let mut stash = None;
            let frequencies = derived.get_or(key ^ 0x503_0002, || {
                match analysis.map(|analysis| evaluate_expression(analysis, text)) {
                    Some(Ok((frequencies, real, imag))) => {
                        stash = Some((real, imag));
                        Arc::new(frequencies)
                    }
                    _ => empty(),
                }
            });
            let evaluated = |part: usize| {
                stash
                    .as_ref()
                    .map(|(real, imag)| {
                        if part == 0 {
                            real.clone()
                        } else {
                            imag.clone()
                        }
                    })
                    .or_else(|| {
                        analysis
                            .and_then(|analysis| evaluate_expression(analysis, text).ok())
                            .map(|(_, real, imag)| if part == 0 { real } else { imag })
                    })
                    .unwrap_or_default()
            };
            let real = derived.get_or(key ^ 0x503_0000, || Arc::new(evaluated(0)));
            let imag = derived.get_or(key ^ 0x503_0001, || Arc::new(evaluated(1)));
            (frequencies, real, imag)
        };
        if frequencies.is_empty()
            || frequencies.len() != raw_real.len()
            || frequencies.len() != raw_imag.len()
        {
            continue;
        }
        let gamma = |part: usize| {
            Arc::new(
                raw_real
                    .iter()
                    .zip(raw_imag.iter())
                    .map(|(re, im)| {
                        let gamma = reading.reflection(Complex64::new(*re, *im), reference);
                        if part == 0 { gamma.re } else { gamma.im }
                    })
                    .collect::<Vec<_>>(),
            )
        };
        let mapped = key ^ reading_key;
        let real = derived.get_or(mapped ^ 0x504_0000, || gamma(0));
        let imaginary = derived.get_or(mapped ^ 0x504_0001, || gamma(1));
        let index = smith.traces.len() + position;
        traces.push(SmithPlotTrace {
            index,
            name: text,
            frequencies,
            real,
            imaginary,
            reference_impedance_ohm: Some(reference),
            visible: true,
            removable: true,
            cache_key: rspice_ui_kit::plot::trace_cache_key(
                0x501_00F1 ^ (mapped as u32 as u64),
                index,
            ),
        });
    }
    // The circles follow the marker's frequency, else the sweep's middle.
    let figures = sweep.and_then(|sweep| {
        let marked = pin
            .and_then(|(slot, index)| {
                traces
                    .get(slot)
                    .and_then(|trace| trace.frequencies.get(index))
            })
            .and_then(|frequency| sweep.nearest_index(*frequency));
        TwoPortFigures::at(sweep, marked.unwrap_or(sweep.frequencies.len() / 2))
    });
    let circles = figures
        .map(|figures| figures.circles(sheet.circles, &sheet.gain_db, &sheet.noise_db))
        .unwrap_or_default();
    Prepared {
        traces,
        figures,
        circles,
    }
}

/// Present the qualified traces and apply local viewer outcomes.
pub fn show(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    let quantity_policy = state.model.preferences.quantity_presentation_policy();
    let presentation = state.results_reads().presentation_policy();
    let digits = usize::from(presentation.displayed_significant_digits().get());
    let owner = owner_key(state);
    let sweep = two_port(state, owner);
    let viewer = super::ResultViewer::Smith;
    let mut pin = state.results.session.rf_pin.get(&viewer).copied();
    let analysis = state.model.simulation.active_analysis();
    let session = &mut state.results.session;
    let prepared = prepare(
        &state.analysis.smith_chart_state,
        &session.smith,
        &mut session.derived,
        analysis,
        owner,
        sweep.as_deref(),
        pin,
    );
    let mut view = session.plot_view(viewer, 0);
    let chart = view::SmithChart {
        traces: &prepared.traces,
        sheet: &session.smith,
        figures: prepared.figures.as_ref(),
        circles: &prepared.circles,
    };
    let frame = view::show(
        ui,
        &chart,
        &mut view,
        &mut pin,
        &mut session.cache,
        &quantity_policy,
        digits,
    );
    let toggled = frame
        .as_ref()
        .and_then(|frame| frame.toggled)
        .and_then(|slot| prepared.traces.get(slot))
        .map(|trace| (trace.name.to_owned(), trace.removable));
    let removed = frame
        .as_ref()
        .and_then(|frame| frame.removed)
        .and_then(|slot| prepared.traces.get(slot))
        .filter(|trace| trace.removable)
        .map(|trace| trace.name.to_owned());
    let response = frame.map(|frame| frame.plot);
    drop(prepared);
    let results = &mut state.results;
    if let Some((name, removable)) = toggled
        && !removable
    {
        let hidden = &mut results.session.smith.hidden;
        if !hidden.remove(&name) {
            hidden.insert(name);
        }
    }
    if let Some(name) = removed {
        results.session.smith.responses.retain(|text| *text != name);
        results.session.rf_pin.remove(&viewer);
    }
    if let Some(response) = response {
        super::record_drawn_axes(results, viewer, &response);
        if response.view.any() {
            *results.session.plot_view_mut(viewer, 0) = view;
        }
        if response.response.clicked() {
            match pin {
                Some(hit) => {
                    results.session.rf_pin.insert(viewer, hit);
                }
                None => {
                    results.session.rf_pin.remove(&viewer);
                }
            }
        }
    }
}

/// Present the marker, the trace summary and the two-port figures.
pub fn right_panel(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    let presentation = state.results_reads().presentation_policy();
    let quantity_policy = state.model.preferences.quantity_presentation_policy();
    let digits = usize::from(presentation.displayed_significant_digits().get());
    let owner = owner_key(state);
    let sweep = two_port(state, owner);
    let pin = state
        .results
        .session
        .rf_pin
        .get(&super::ResultViewer::Smith)
        .copied();
    let analysis = state.model.simulation.active_analysis();
    let session = &mut state.results.session;
    let prepared = prepare(
        &state.analysis.smith_chart_state,
        &session.smith,
        &mut session.derived,
        analysis,
        owner,
        sweep.as_deref(),
        pin,
    );
    let chart = view::SmithChart {
        traces: &prepared.traces,
        sheet: &session.smith,
        figures: prepared.figures.as_ref(),
        circles: &prepared.circles,
    };
    view::right_panel(ui, &chart, pin, &quantity_policy, digits);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A retained S-parameter analysis whose S11 locus is `values`.
    fn sparam_analysis(
        id: u64,
        label: &str,
        values: &[(f64, f64)],
    ) -> crate::state::AnalysisResult {
        let x: Vec<f64> = (0..values.len())
            .map(|index| 1.0e9 * (index as f64 + 1.0))
            .collect();
        let waveform = crate::state::RetainedTrace::new(
            "S11",
            x.clone(),
            values.iter().map(|(re, _)| *re).collect::<Vec<_>>(),
            "#0af",
        )
        .with_complex_components(
            "S11",
            values.iter().map(|(re, _)| *re).collect::<Vec<_>>(),
            values.iter().map(|(_, im)| *im).collect::<Vec<_>>(),
        );
        crate::state::AnalysisResult::new(id, crate::state::AnalysisType::SParameter, label)
            .with_family_metadata(crate::state::AnalysisResultFamilyMetadata::SParameter {
                noise_reference_temperature_kelvin: None,
                reference_impedances_ohm: vec![50.0, 50.0],
            })
            .with_waveforms(vec![waveform])
    }

    /// A complex retained trace.
    fn complex_trace(name: &str, x: &[f64], values: &[Complex64]) -> crate::state::RetainedTrace {
        crate::state::RetainedTrace::new(
            name,
            x.to_vec(),
            values.iter().map(|value| value.norm()).collect::<Vec<_>>(),
            "#0af",
        )
        .with_complex_components(
            name,
            values.iter().map(|value| value.re).collect::<Vec<_>>(),
            values.iter().map(|value| value.im).collect::<Vec<_>>(),
        )
    }

    /// An AC analysis of a series RC driven by `Vin`: V(in) = 1 and
    /// I(Vin) = −V(in)/Z, so V(in)/I(Vin) is −Z.
    fn ac_analysis() -> (crate::state::AnalysisResult, Vec<Complex64>) {
        let frequencies = [1.0e6, 1.0e7, 1.0e8];
        let impedances = frequencies
            .iter()
            .map(|frequency| {
                Complex64::new(25.0, -1.0 / (std::f64::consts::TAU * frequency * 10.0e-12))
            })
            .collect::<Vec<_>>();
        let voltage = vec![Complex64::new(1.0, 0.0); 3];
        let current = impedances
            .iter()
            .map(|impedance| -1.0 / impedance)
            .collect::<Vec<_>>();
        let analysis = crate::state::AnalysisResult::new(7, crate::state::AnalysisType::Ac, "AC")
            .with_waveforms(vec![
                complex_trace("V(in)", &frequencies, &voltage),
                complex_trace("I(Vin)", &frequencies, &current),
            ]);
        (analysis, impedances)
    }

    fn host_with(analysis: crate::state::AnalysisResult) -> ResultsTestHost {
        let mut run = crate::state::SimulationRun::new(1);
        run.add_analysis(analysis);
        let mut state = ResultsTestHost::default();
        state.model.simulation.retained.runs = vec![run].into();
        assert!(state.model.simulation.select_run(0));
        assert!(state.model.simulation.select_analysis(0));
        state
    }

    /// An AC input impedance goes on the chart: the expression V(in)/I(Vin)
    /// (negated, since SPICE's source current flows into the + terminal),
    /// read as Z against the pane's Z₀, lands at (Z − Z₀)/(Z + Z₀).
    #[test]
    fn an_ac_input_impedance_is_read_onto_the_chart_against_the_pane_reference() {
        let (analysis, impedances) = ac_analysis();
        let mut state = host_with(analysis);
        assert!(
            availability(state.results_reads()).available,
            "an AC response is a Smith source"
        );
        assert!(synchronize_active_analysis(&mut state.results_parts()));
        let names = state
            .results
            .analysis
            .smith_chart_state
            .traces
            .iter()
            .map(|trace| (trace.name.clone(), trace.response))
            .collect::<Vec<_>>();
        assert_eq!(
            names,
            [("I(Vin)".to_owned(), true), ("V(in)".to_owned(), true)]
        );

        let parts = state.results_parts();
        let analysis = parts.model.simulation.active_analysis().unwrap();
        assert!(evaluate_expression(analysis, "-V(in)/I(Vin)").is_ok());
        assert!(evaluate_expression(analysis, "V(nowhere)").is_err());
        let sheet = &mut parts.results.session.smith;
        sheet.responses = vec!["-V(in)/I(Vin)".to_owned()];
        sheet.reading = rspice_results_ui::smith_chart::state::ResponseReading::Impedance;
        sheet.reference_ohm = 75.0;
        let analysis = parts.model.simulation.active_analysis();
        let session = &mut parts.results.session;
        let prepared = prepare(
            &parts.analysis.smith_chart_state,
            &session.smith,
            &mut session.derived,
            analysis,
            1,
            None,
            None,
        );
        let trace = prepared
            .traces
            .iter()
            .find(|trace| trace.removable)
            .expect("the expression is drawn");
        assert_eq!(trace.reference_impedance_ohm, Some(75.0));
        for (index, impedance) in impedances.iter().enumerate() {
            let expected = (impedance - 75.0) / (impedance + 75.0);
            let drawn = Complex64::new(trace.real[index], trace.imaginary[index]);
            assert!((drawn - expected).norm() < 1.0e-12, "{drawn} vs {expected}");
        }
        assert!(prepared.figures.is_none(), "an AC run has no two-port");
    }

    /// The circles come from the run's own two-port at the marker's
    /// frequency.
    #[test]
    fn the_circles_follow_the_marker_frequency() {
        let frequencies = [1.0e8, 1.4e9, 2.0e9];
        let column = |pick: fn(usize) -> Complex64| (0..3).map(pick).collect::<Vec<_>>();
        let s11 = column(|_| Complex64::from_polar(0.533, 176.6f64.to_radians()));
        let s12 = column(|_| Complex64::from_polar(0.06, 58.4f64.to_radians()));
        let s21 = column(|index| Complex64::from_polar(2.8 + index as f64, 64.5f64.to_radians()));
        let s22 = column(|_| Complex64::from_polar(0.604, -58.3f64.to_radians()));
        let analysis =
            crate::state::AnalysisResult::new(3, crate::state::AnalysisType::SParameter, "SP")
                .with_family_metadata(crate::state::AnalysisResultFamilyMetadata::SParameter {
                    noise_reference_temperature_kelvin: None,
                    reference_impedances_ohm: vec![50.0, 50.0],
                })
                .with_waveforms(vec![
                    complex_trace("S11", &frequencies, &s11),
                    complex_trace("S12", &frequencies, &s12),
                    complex_trace("S21", &frequencies, &s21),
                    complex_trace("S22", &frequencies, &s22),
                ]);
        let mut state = host_with(analysis);
        assert!(synchronize_active_analysis(&mut state.results_parts()));
        let mut parts = state.results_parts();
        let owner = owner_key(&parts);
        let sweep = two_port(&mut parts, owner).expect("a two-port");
        parts.results.session.smith.circles.stability = true;
        let analysis = parts.model.simulation.active_analysis();
        let session = &mut parts.results.session;
        let unmarked = prepare(
            &parts.analysis.smith_chart_state,
            &session.smith,
            &mut session.derived,
            analysis,
            owner,
            Some(&sweep),
            None,
        );
        assert_eq!(unmarked.figures.map(|figures| figures.index), Some(1));
        assert_eq!(unmarked.circles.len(), 2);
        // A marker on the last sample of S11 moves the circles there.
        let s11_slot = unmarked
            .traces
            .iter()
            .position(|trace| trace.name == "S11")
            .unwrap();
        drop(unmarked);
        let marked = prepare(
            &parts.analysis.smith_chart_state,
            &session.smith,
            &mut session.derived,
            analysis,
            owner,
            Some(&sweep),
            Some((s11_slot, 2)),
        );
        let figures = marked.figures.expect("figures");
        assert_eq!(figures.index, 2);
        assert_eq!(figures.frequency, 2.0e9);
        let expected = sweep.scattering[2].source_stability_circle().unwrap();
        assert!(
            marked
                .circles
                .iter()
                .any(|circle| circle.circle == expected.circle),
            "the source circle is the last sample's"
        );
    }

    fn draw(state: &mut ResultsTestHost) {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(900.0, 700.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut state.results_parts()));
            },
        );
    }

    /// The Γ-component arrays are cached, and the cache belongs to the result
    /// that filled it. Selecting another analysis of the same run reloads the
    /// loci without moving the data version, so a cache keyed only by trace
    /// ordinal would draw the previous analysis' coefficients under the new
    /// one's name.
    #[test]
    fn the_gamma_arrays_belong_to_the_analysis_that_filled_them() {
        let mut run = crate::state::SimulationRun::new(1);
        run.add_analysis(sparam_analysis(1, "SP low", &[(0.1, 0.0), (0.2, 0.1)]));
        run.add_analysis(sparam_analysis(2, "SP high", &[(-0.6, 0.3), (-0.5, 0.4)]));
        let mut state = ResultsTestHost::default();
        state.model.simulation.retained.runs = vec![run].into();
        assert!(state.model.simulation.select_run(0));
        assert!(state.model.simulation.select_analysis(0));

        assert!(synchronize_active_analysis(&mut state.results_parts()));
        draw(&mut state);
        let first = state.results.analysis.smith_chart_state.traces[0].points[0]
            .s
            .re;
        assert!((first - 0.1).abs() < 1.0e-12, "{first}");

        assert!(state.model.simulation.select_analysis(1));
        assert!(synchronize_active_analysis(&mut state.results_parts()));
        draw(&mut state);
        let second = state.results.analysis.smith_chart_state.traces[0].points[0]
            .s
            .re;
        assert!((second + 0.6).abs() < 1.0e-12, "{second}");

        // The cached component array has to have moved with the selection.
        let owner = state
            .model
            .active_specialized_viewer_cache_provenance()
            .expect("the reloaded cache names its owner");
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        {
            use std::hash::Hash as _;
            owner.dataset_id.hash(&mut hasher);
            match owner.analysis_identity {
                SpecializedViewerAnalysisIdentity::Prepared(id) => {
                    id.hash(&mut hasher);
                }
                SpecializedViewerAnalysisIdentity::LegacyResultId(id) => id.hash(&mut hasher),
            }
        }
        let key = {
            use std::hash::Hasher as _;
            hasher.finish()
        };
        let cached = state
            .results
            .results
            .session
            .derived
            .get_or(key ^ 0x501_0000, || std::sync::Arc::new(Vec::new()));
        assert_eq!(
            cached.first().copied(),
            Some(-0.6),
            "the sheet drew the previous analysis' coefficients"
        );
    }
}
