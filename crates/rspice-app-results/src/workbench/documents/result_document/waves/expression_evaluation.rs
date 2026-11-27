//! Resolve the input family before evaluating and retain each output's axis.

use super::*;
use crate::product::DatasetId;
use crate::state::{ComplexExpressionPolicy, SimulationRun};
use rspice_results::calculator::datasets::{BASELINE, NamedRun, RunDatasets, named_runs};
use rspice_results_ui::waves::lanes::EventLaneLayout;
use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(test)]
pub(super) fn evaluate_expression(
    simulation: &SimulationState,
    analysis_index: usize,
    text: &str,
    selection: Option<&SourceSampleSelection>,
) -> ExpressionSeriesResult {
    evaluate_expression_reading(
        simulation,
        (analysis_index, None),
        text,
        selection,
        ComplexExpressionPolicy::Rectangular,
        None,
        &Places::default(),
    )
}

/// Where the reader's cursors and markers stand on the strip an expression
/// is drawn on, for `xA`, `xB`, `m1.x` and `m1.y`, and the scale its X axis
/// is drawn in, for `yval`.
#[derive(Debug, Clone, Default, PartialEq)]
pub(super) struct Places {
    /// A and B, when they stand on this strip.
    pub cursors: [Option<f64>; 2],
    /// Each marker on the strip: its number, X and Y.
    pub markers: Vec<(u64, f64, f64)>,
    /// The scale of the strip's X axis, so `yval` reads where its cursor
    /// reads; `None` off any strip.
    pub x_scale: Option<XScale>,
}

impl Places {
    /// Where the cursors and markers stand on `model`'s strip, its analysis
    /// `analysis` at `index` in the run: the cursors only when they stand on
    /// it, a marker at its found X with its trace's reading there.
    pub(super) fn of(
        state: &ResultsParts<'_>,
        (analysis, index): (AnalysisPresentationKey, usize),
        model: Option<&StripModel>,
    ) -> Self {
        let session = &state.results.session;
        let cursors = if session.cursor_strip.is_none_or(|strip| strip == index) {
            [session.cursors.a, session.cursors.b]
        } else {
            [None, None]
        };
        let model = model.or_else(|| {
            state
                .results
                .models
                .last_built()
                .and_then(|models| models.iter().find(|model| model.analysis_key == analysis))
        });
        let interpolation = rspice_results_ui::waves::cursor_interpolation(
            state
                .model
                .preferences
                .result_presentation_policy()
                .cursor_interpolation(),
        );
        let markers = model.map_or_else(Vec::new, |model| {
            session
                .strip_markers(analysis)
                .filter_map(|marker| {
                    let x =
                        rspice_results_ui::waves::cursor_values::marker_x(model, session, marker)
                            .ok()?;
                    let trace = model.traces.iter().find(|trace| {
                        !trace.overlay
                            && rspice_results_ui::waves::anchor_key(model, trace)
                                == *marker.anchor()
                    })?;
                    let y = model.trace_value(trace, x, interpolation);
                    Some((marker.0.retained_id.get(), x, y))
                })
                .collect()
        });
        Self {
            cursors,
            markers,
            x_scale: model.map(|model| model.x_scale),
        }
    }

    /// What of these places `text` reads, folded into its version, so an
    /// expression that names a cursor or a marker evaluates again when it
    /// moves and one that names none never does.
    pub(super) fn version_for(&self, text: &str) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        for word in text.split(|c: char| !(c.is_alphanumeric() || matches!(c, '_' | '.'))) {
            match word {
                "xA" => self.cursors[0].map(f64::to_bits).hash(&mut hasher),
                "xB" => self.cursors[1].map(f64::to_bits).hash(&mut hasher),
                // A level measure reads its position or window ends where
                // the drawn line runs, so the axis scale is part of it.
                "yval" | "avg" | "average" | "rms" | "min" | "max" | "pp" => {
                    word.hash(&mut hasher);
                    self.x_scale.hash(&mut hasher);
                }
                _ => {
                    if let Some(reading) =
                        rspice_results::calculator::ast::MarkerReading::parse(word)
                    {
                        word.hash(&mut hasher);
                        self.marker(reading).map(f64::to_bits).hash(&mut hasher);
                    }
                }
            }
        }
        hasher.finish()
    }

    fn marker(&self, reading: rspice_results::calculator::ast::MarkerReading) -> Option<f64> {
        use rspice_results::calculator::ast::MarkerAxis;
        self.markers
            .iter()
            .find(|(number, _, _)| *number == reading.number)
            .map(|&(_, x, y)| match reading.axis {
                MarkerAxis::X => x,
                MarkerAxis::Y => y,
            })
    }
}

/// The retained runs an expression over `analysis` of `run` can read with
/// `@`, each by its name, `Run 3`, and the pinned baseline also as
/// `baseline`: the analysis of each that matches the one evaluated.
fn other_runs<'a>(
    simulation: &'a SimulationState,
    run: &SimulationRun,
    analysis: &AnalysisResult,
    baseline: Option<DatasetId>,
) -> Vec<NamedRun<'a, crate::state::RetainedTrace>> {
    simulation
        .retained
        .runs
        .iter()
        .filter(|other| other.dataset_id != run.dataset_id)
        .filter_map(|other| {
            let matching =
                rspice_results::waveform_comparison::matching_comparison_analysis(analysis, other)?;
            let mut names = vec![format!("Run {}", other.id)];
            if baseline == Some(other.dataset_id) {
                names.push(BASELINE.to_owned());
            }
            Some(NamedRun {
                names,
                waveforms: matching.waveforms.as_slice(),
                coordinate_unit: matching.waveform_coordinate_unit(),
            })
        })
        .collect()
}

/// Evaluate `text` with the strip's buses readable by `analog()`, each in
/// the signedness of the radix `lanes` reads it in, the reader's own buses
/// among them, and the other retained runs readable with `@`, `baseline`
/// naming the pinned one.
pub(super) fn evaluate_expression_reading(
    simulation: &SimulationState,
    (analysis_index, baseline): (usize, Option<DatasetId>),
    text: &str,
    selection: Option<&SourceSampleSelection>,
    complex_policy: ComplexExpressionPolicy,
    lanes: Option<&EventLaneLayout>,
    places: &Places,
) -> ExpressionSeriesResult {
    let run = simulation.active_run().ok_or("analysis no longer exists")?;
    let analysis = run
        .analyses
        .get(analysis_index)
        .ok_or("analysis no longer exists")?;
    let datasets = RunDatasets {
        own: analysis.waveforms.as_slice(),
        coordinate_unit: analysis.waveform_coordinate_unit(),
        runs: other_runs(simulation, run, analysis, baseline),
    };
    let reading = Reading {
        analysis,
        datasets: &datasets,
        lanes,
        places,
        policy: complex_policy,
        stop: &AtomicBool::new(false),
    };
    evaluate_reading(&reading, text, own_selection(selection, run, analysis))
}

/// A pane selection applies only to the exact dataset and analysis it
/// identifies: a Waves document may contain several analysis strips.
fn own_selection<'s>(
    selection: Option<&'s SourceSampleSelection>,
    run: &SimulationRun,
    analysis: &AnalysisResult,
) -> Option<&'s SourceSampleSelection> {
    selection.filter(|selection| {
        selection.dataset_id == run.dataset_id && selection.analysis_sequence == analysis.id
    })
}

/// What an expression is evaluated against: its analysis, the other runs
/// it can name, the strip's buses and places, and the flag that stops it.
struct Reading<'a> {
    analysis: &'a AnalysisResult,
    datasets: &'a RunDatasets<'a, crate::state::RetainedTrace>,
    lanes: Option<&'a EventLaneLayout>,
    places: &'a Places,
    policy: ComplexExpressionPolicy,
    stop: &'a AtomicBool,
}

fn evaluate_reading(
    reading: &Reading<'_>,
    text: &str,
    selection: Option<&SourceSampleSelection>,
) -> ExpressionSeriesResult {
    let expr =
        calculator::parser::try_parse(text).map_err(|error| format!("parse error: {error}"))?;
    if let Some(selection) = selection {
        if selection.source_indices.is_empty() {
            return Err("the expression selection contains no samples".to_owned());
        }
        if let Some(plan) = selection.family_render_plan() {
            return plan
                .groups()
                .iter()
                .map(|group| {
                    evaluate_scope(
                        reading,
                        (text, &expr),
                        ExpressionSource::FamilyMember {
                            ordinal: group.ordinal,
                        },
                        Some(&group.source_indices),
                        Some(&group.x_values),
                    )
                    .map_err(|error| format!("{}: {error}", group.label))
                })
                .collect();
        }
        return evaluate_scope(
            reading,
            (text, &expr),
            ExpressionSource::SelectedSamples,
            Some(&selection.source_indices),
            None,
        )
        .map(|output| vec![output]);
    }
    evaluate_scope(
        reading,
        (text, &expr),
        ExpressionSource::Analysis,
        None,
        None,
    )
    .map(|output| vec![output])
}

/// What an expression over stored samples is evaluated from off the frame:
/// its analysis and the other runs it can name, held by the job, with what
/// it reads of the strip.
pub(super) struct ExpressionWork {
    analysis: AnalysisResult,
    runs: Vec<OwnedRun>,
    text: String,
    selection: Option<SourceSampleSelection>,
    policy: ComplexExpressionPolicy,
    lanes: Option<EventLaneLayout>,
    places: Places,
}

/// Another run an expression can name, held by an [`ExpressionWork`]: its
/// names, its matching analysis' waveforms and their X unit.
struct OwnedRun {
    names: Vec<String>,
    waveforms: Vec<crate::state::RetainedTrace>,
    coordinate_unit: Option<String>,
}

impl ExpressionWork {
    /// What [`evaluate_expression_reading`] reads, held for a job: a stored
    /// series is a window of its store, so holding it reads nothing.
    pub(super) fn gather(
        simulation: &SimulationState,
        (analysis_index, baseline): (usize, Option<DatasetId>),
        text: &str,
        (selection, complex_policy): (Option<&SourceSampleSelection>, ComplexExpressionPolicy),
        lanes: Option<&EventLaneLayout>,
        places: &Places,
    ) -> Result<Self, String> {
        let run = simulation.active_run().ok_or("analysis no longer exists")?;
        let analysis = run
            .analyses
            .get(analysis_index)
            .ok_or("analysis no longer exists")?;
        let runs = other_runs(simulation, run, analysis, baseline)
            .into_iter()
            .map(|named| OwnedRun {
                names: named.names,
                waveforms: named.waveforms.to_vec(),
                coordinate_unit: named.coordinate_unit.map(str::to_owned),
            })
            .collect();
        Ok(Self {
            analysis: analysis.clone(),
            runs,
            text: text.to_owned(),
            selection: own_selection(selection, run, analysis).cloned(),
            policy: complex_policy,
            lanes: lanes.cloned(),
            places: places.clone(),
        })
    }

    /// The expression's series, as [`evaluate_expression_reading`] reads
    /// them, or an end once `stop` is raised.
    pub(super) fn evaluate(&self, stop: &AtomicBool) -> ExpressionSeriesResult {
        let datasets = RunDatasets {
            own: self.analysis.waveforms.as_slice(),
            coordinate_unit: self.analysis.waveform_coordinate_unit(),
            runs: self
                .runs
                .iter()
                .map(|run| NamedRun {
                    names: run.names.clone(),
                    waveforms: run.waveforms.as_slice(),
                    coordinate_unit: run.coordinate_unit.as_deref(),
                })
                .collect(),
        };
        let reading = Reading {
            analysis: &self.analysis,
            datasets: &datasets,
            lanes: self.lanes.as_ref(),
            places: &self.places,
            policy: self.policy,
            stop,
        };
        evaluate_reading(&reading, &self.text, self.selection.as_ref())
    }
}

fn evaluate_scope(
    reading: &Reading<'_>,
    (text, expr): (&str, &calculator::ast::CalculatorExpr),
    source: ExpressionSource,
    indices: Option<&[usize]>,
    axis: Option<&[f64]>,
) -> Result<ExpressionWaveform, String> {
    let analysis = reading.analysis;
    let mut ctx = calculator::WaveformsContext::with_policy(&analysis.waveforms, reading.policy)
        .with_datasets(reading.datasets);
    if let Some(indices) = indices {
        ctx = ctx
            .with_sample_projection(indices, axis)
            .map_err(|error| error.to_string())?;
    }
    let scalar_axis = if let Some(axis) = axis {
        axis.to_vec()
    } else if let Some(waveform) = analysis.waveforms.first() {
        if let Some(indices) = indices {
            indices
                .iter()
                .map(|index| {
                    waveform
                        .x
                        .sample(*index)
                        .ok_or_else(|| "selected rows exceed the retained analysis axis".to_owned())
                })
                .collect::<Result<Vec<_>, _>>()?
        } else {
            waveform
                .x
                .ends()
                .map(|(first, last)| vec![first, last])
                .unwrap_or_default()
        }
    } else {
        Vec::new()
    };
    let ctx = EventWords {
        values: ctx,
        analysis,
        lanes: reading.lanes,
        places: reading.places,
    };
    // Over the analysis' own rows, an expression read sample by sample reads
    // its stored operands a block at a time, never whole.
    if indices.is_none()
        && let Some(axis) = analysis.waveforms.first().map(|waveform| &waveform.x)
        && let Some(values) = calculator::pointwise::evaluate_in_blocks(
            expr,
            &ctx,
            axis,
            rspice_results::waveform::BLOCK_SAMPLES,
            |_, _| !reading.stop.load(Ordering::Relaxed),
        )
    {
        let values = values.map_err(|error| error.to_string())?;
        let value = rspice_results::waveform::RetainedWaveform::new(text, axis.clone(), values);
        return Ok(ExpressionWaveform {
            source,
            waveform: calculator::evaluated_trace(value),
            reading: None,
        });
    }
    let value = calculator::evaluator::evaluate(expr, &ctx).map_err(|error| error.to_string())?;
    let scalar_axis = rspice_results::waveform::WaveformValues::from(scalar_axis);
    let waveform = calculator::evaluated_waveform(value, text, Some(&scalar_axis))?;
    if waveform.x.is_empty() {
        return Err("expression produced no samples".to_owned());
    }
    // A signal read from another run carries interpolation onto this axis,
    // so the trace says which run and how.
    let readings = named_runs(expr)
        .iter()
        .filter_map(|(signal, run)| reading.datasets.reading(signal, run))
        .collect::<Vec<_>>();
    Ok(ExpressionWaveform {
        source,
        waveform,
        reading: (!readings.is_empty()).then(|| readings.join("; ")),
    })
}

/// An analysis' values, with its digital nets and buses read from their exact
/// event histories for `analog()`.
struct EventWords<'a, C> {
    values: C,
    analysis: &'a AnalysisResult,
    lanes: Option<&'a EventLaneLayout>,
    places: &'a Places,
}

impl<C: calculator::evaluator::EvaluationContext> calculator::evaluator::EvaluationContext
    for EventWords<'_, C>
{
    fn get_waveform(
        &self,
        signal: &str,
        dataset: Option<&str>,
    ) -> Result<calculator::evaluator::CalcValue, calculator::evaluator::EvaluationError> {
        self.values.get_waveform(signal, dataset)
    }

    fn get_magnitude(
        &self,
        signal: &str,
        dataset: Option<&str>,
    ) -> Result<calculator::evaluator::CalcValue, calculator::evaluator::EvaluationError> {
        self.values.get_magnitude(signal, dataset)
    }

    fn stored_columns(
        &self,
        signal: &str,
        dataset: Option<&str>,
    ) -> Option<(
        &rspice_results::waveform::WaveformValues,
        &rspice_results::waveform::WaveformValues,
    )> {
        self.values.stored_columns(signal, dataset)
    }

    fn drawn_x_scale(&self) -> Option<XScale> {
        self.places.x_scale
    }

    fn get_event_word(
        &self,
        signal: &str,
        _dataset: Option<&str>,
    ) -> Result<calculator::functions::EventWord, calculator::evaluator::EvaluationError> {
        event_word(self.analysis, self.lanes, signal)
    }

    fn cursor(&self, cursor: rspice_results::calculator::ast::Cursor) -> Option<f64> {
        use rspice_results::calculator::ast::Cursor;
        match cursor {
            Cursor::A => self.places.cursors[0],
            Cursor::B => self.places.cursors[1],
        }
    }

    fn marker(&self, reading: rspice_results::calculator::ast::MarkerReading) -> Option<f64> {
        self.places.marker(reading)
    }
}

/// The exact history of the bus or logic net `D(name)` names: one of the
/// reader's buses, a bus the run declared (with or without its range), or a
/// single net. A name matches whatever its case, as SPICE and `.MEASURE`
/// read it: the engine keeps a logic net as `CLK` where the drawing labels
/// it `clk`.
pub(in crate::workbench::documents::result_document) fn event_word(
    analysis: &AnalysisResult,
    lanes: Option<&EventLaneLayout>,
    signal: &str,
) -> Result<calculator::functions::EventWord, calculator::evaluator::EvaluationError> {
    use calculator::evaluator::EvaluationError;
    use rspice_results::engine_evidence::{VcdBit, event_code_to_vcd_bit};
    use rspice_results_ui::waves::logic::{BusLane, BusOrigin, BusRadix};

    let missing = || EvaluationError::IdentifierNotFound(signal.to_owned());
    let name = signal
        .strip_prefix("D(")
        .and_then(|name| name.strip_suffix(')'))
        .unwrap_or(signal);
    let Some(crate::state::AnalysisResultPayload::TransientEvents {
        digital_traces,
        digital_buses,
        ..
    }) = analysis.result_payload.as_ref()
    else {
        return Err(missing());
    };
    let trace = |net: &str| {
        digital_traces
            .iter()
            .find(|trace| trace.node_name.eq_ignore_ascii_case(net))
    };
    let history = |net: &str| {
        trace(net).map(|trace| {
            trace
                .points
                .iter()
                .map(|point| (point.time_s, point.value_code))
                .collect::<Vec<_>>()
        })
    };
    let user = lanes.and_then(|lanes| {
        lanes
            .user_buses
            .iter()
            .find(|bus| bus.name.eq_ignore_ascii_case(name))
            .map(|bus| (bus.name.clone(), bus.members.clone()))
    });
    let declared = || {
        digital_buses.iter().find_map(|bus| {
            let label = format!("{}[{}:{}]", bus.name, bus.msb, bus.lsb);
            (bus.name.eq_ignore_ascii_case(name) || label.eq_ignore_ascii_case(name)).then(|| {
                (
                    label,
                    bus.members
                        .iter()
                        .map(|member| format!("D({member})"))
                        .collect::<Vec<_>>(),
                )
            })
        })
    };
    let (label, members) = match user.or_else(declared) {
        Some(bus) => bus,
        None if trace(name).is_some() => (signal.to_owned(), vec![format!("D({name})")]),
        None => return Err(missing()),
    };
    let nets: Vec<String> = members
        .iter()
        .map(|member| {
            member
                .strip_prefix("D(")
                .and_then(|net| net.strip_suffix(')'))
                .unwrap_or(member)
                .to_owned()
        })
        .collect();
    let histories = nets
        .iter()
        .map(|net| history(net))
        .collect::<Option<Vec<_>>>()
        .ok_or_else(missing)?;
    let lane = BusLane::assemble(label.clone(), nets, BusOrigin::User, &histories);
    if let Some(refusal) = lane.refusal() {
        return Err(EvaluationError::MathError(format!("{signal}: {refusal}")));
    }
    let words = (0..lane.times().len())
        .filter_map(|index| lane.word(index))
        .map(|codes| {
            codes
                .iter()
                .map(|code| match event_code_to_vcd_bit(*code) {
                    Some(VcdBit::Zero) => Some(false),
                    Some(VcdBit::One) => Some(true),
                    _ => None,
                })
                .collect()
        })
        .collect();
    let end = analysis
        .waveforms
        .iter()
        .filter_map(|waveform| waveform.x.ends().map(|(_, last)| last))
        .chain(lane.times().last().copied())
        .fold(f64::NEG_INFINITY, f64::max);
    Ok(calculator::functions::EventWord {
        times: lane.times().to_vec(),
        words,
        signed: lanes.is_some_and(|lanes| lanes.bus_view(&label).radix == BusRadix::Signed),
        end,
    })
}
