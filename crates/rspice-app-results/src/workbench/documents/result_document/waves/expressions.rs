//! Per-strip expression traces: editing them and evaluating them.
//!
//! An expression is re-evaluated only when its text or the data behind it
//! changes, so the cache key carries both. One evaluated off the frame keeps
//! drawing the curve it last landed until the job for the new version lands,
//! under the identity of the version it was evaluated at. An expression that
//! fails to resolve is reported on its own row rather than being dropped, so
//! the strip never silently loses a trace the engineer asked for.

use super::*;
use crate::state::ComplexExpressionPolicy;
use crate::workbench::documents::result_document::{ExpressionJob, ResultsParts};
use rspice_results_ui::waves::cache::apply_waveform_visibility;
use rspice_results_ui::waves::expression_editor::Action;
use rspice_results_ui::waves::lanes::EventLaneLayout;
pub(super) use rspice_results_ui::waves::stack::{expr_color, expr_palette_slot, expression_label};

/// The baseline `@baseline` reads over the active run, and a fingerprint of
/// it to fold into an expression's version, so a new pin evaluates again.
fn pinned(state: ResultsReads<'_>) -> (Option<crate::product::DatasetId>, u64) {
    let pin = state.model.simulation.active_run().and_then(|run| {
        super::super::document_edit::compare::pinned_baseline(state, run.dataset_id)
    });
    (
        pin,
        pin.map_or(0, |pin| rspice_results_ui::waves::stable_hash(&pin)),
    )
}

/// The inline expression editor row under a strip header (when open for
/// this strip): mono input, Enter/Add commits, Esc closes, and a bounded
/// validation message that moves below the controls on compact surfaces.
pub(super) fn expr_editor_row(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    analysis_key: AnalysisPresentationKey,
    analysis_index: usize,
) {
    let scope_note = state
        .results
        .session
        .sample_selection
        .as_ref()
        .and_then(|selection| {
            let run = state.model.simulation.active_run()?;
            let analysis = run.analyses.get(analysis_index)?;
            (selection.dataset_id == run.dataset_id && selection.analysis_sequence == analysis.id)
                .then(|| {
                    if selection.source_indices.is_empty() {
                        "Scope: no samples selected"
                    } else if selection.family_render_plan().is_some() {
                        "Scope: each selected curve, evaluated independently"
                    } else {
                        "Scope: selected samples"
                    }
                })
        });
    // The unit the text reads in, shown where the reader may declare one.
    let inferred = state
        .results
        .session
        .expr_editor
        .as_ref()
        .filter(|editor| editor.analysis == analysis_key && !editor.text.trim().is_empty())
        .and_then(|editor| {
            let run = state.model.simulation.active_run()?;
            let analysis = run.analyses.get(analysis_index)?;
            let x_unit = state
                .results
                .models
                .last_built()?
                .iter()
                .find(|model| model.analysis_key == analysis_key)
                .map(|model| model.x_unit.clone())?;
            Some(expression_unit(editor.text.trim(), analysis, &x_unit))
        });
    let session = &mut state.results.session;
    let Some(editor) = session
        .expr_editor
        .as_mut()
        .filter(|editor| editor.analysis == analysis_key)
    else {
        return;
    };

    let action = rspice_results_ui::waves::expression_editor::show(
        ui,
        editor,
        &mut session.expr_editor_unit,
        inferred,
        scope_note,
    );

    match action {
        Action::None => {}
        Action::Cancel => state.results.session.expr_editor = None,
        Action::Commit => {
            let text = state
                .results
                .session
                .expr_editor
                .as_ref()
                .map(|e| e.text.trim().to_owned())
                .unwrap_or_default();
            if text.is_empty() {
                state.results.session.expr_editor = None;
                return;
            }
            let sample_selection = state.results.session.sample_selection.clone();
            let (baseline, pin) = pinned(state.results_reads());
            let places = Places::of(state, (analysis_key, analysis_index), None);
            // An expression over stored samples is only read here: the strip
            // evaluates it off the frame once it is added, and says there
            // why it could not be, as it says for any other.
            let series = if expression_reads_store(&state.model.simulation, analysis_index, &text) {
                calculator::parser::try_parse(&text)
                    .map(|_| None)
                    .map_err(|error| format!("parse error: {error}"))
            } else {
                evaluate_expression_reading(
                    &state.model.simulation,
                    (analysis_index, baseline),
                    &text,
                    sample_selection.as_ref(),
                    ComplexExpressionPolicy::Rectangular,
                    state.results.session.event_lanes.get(&analysis_key),
                    &places,
                )
                .map(Some)
            };
            match series {
                Ok(series) => {
                    if let Some(series) = series {
                        state.results.analysis_expr_cache.insert(
                            (analysis_key, text.clone()),
                            ExprSeries {
                                version: expression_version(
                                    state.model.simulation.view.data_version,
                                    sample_selection.as_ref(),
                                    ComplexExpressionPolicy::Rectangular,
                                ) ^ pin,
                                series: Ok(series),
                            },
                        );
                    }
                    let declared = state.results.session.expr_editor_unit.trim().to_owned();
                    let units = &mut state.results.session.expression_units;
                    let unit_changed = if declared.is_empty() {
                        units.remove(&(analysis_key, text.clone())).is_some()
                    } else {
                        units.insert((analysis_key, text.clone()), declared.clone())
                            != Some(declared)
                    };
                    let added = state
                        .results
                        .add_expression_trace(&state.model.simulation, analysis_key, text)
                        .expect("the expression editor is bound to a retained analysis")
                        || unit_changed;
                    if added {
                        state.model.workspace.content.visualization_documents_dirty = true;
                    }
                    state.results.session.expr_editor = None;
                }
                Err(error) => {
                    if let Some(editor) = state.results.session.expr_editor.as_mut() {
                        editor.error = Some(error);
                        editor.want_focus = true;
                    }
                }
            }
        }
    }
}

pub(super) fn expression_version(
    data_version: u64,
    selection: Option<&SourceSampleSelection>,
    complex_policy: ComplexExpressionPolicy,
) -> u64 {
    data_version
        ^ selection
            .map(SourceSampleSelection::fingerprint)
            .unwrap_or_default()
            .rotate_left(23)
        ^ if complex_policy.is_legacy() {
            0
        } else {
            0x5093_00da_9d16_5c37
        }
}

pub(super) use rspice_results_ui::waves::pane::ResolvedExpr;

/// Refresh the expression cache for a strip at the current data version and
/// hand back plottable series (visible expressions, successful evaluations).
pub(super) fn resolve_strip_exprs(
    state: &mut ResultsParts<'_>,
    model: &StripModel,
    tokens: &Tokens,
) -> Vec<ResolvedExpr> {
    let exprs: Vec<(usize, ExprTrace)> = state
        .results
        .session
        .analysis_exprs
        .get(&model.analysis_key)
        .map(|list| list.iter().cloned().enumerate().collect())
        .unwrap_or_default();
    if exprs.is_empty() {
        return Vec::new();
    }

    let sample_selection = state.results.session.sample_selection.clone();
    let lanes = state
        .results
        .session
        .event_lanes
        .get(&model.analysis_key)
        .cloned();
    // `analog()` reads a bus in its radix's signedness and may name the
    // reader's own buses, so a change to either evaluates again.
    let bus_reading = lanes.as_ref().map_or(0, bus_reading_fingerprint);
    let (baseline, pin) = pinned(state.results_reads());
    let places = Places::of(
        state,
        (model.analysis_key, model.analysis_index),
        Some(model),
    );
    let mut resolved = Vec::new();
    for (slot, expr) in exprs {
        let version = expression_version(
            state.model.simulation.view.data_version,
            sample_selection.as_ref(),
            expr.complex_policy,
        ) ^ bus_reading.rotate_left(41)
            ^ pin
            ^ places.version_for(&expr.text).rotate_left(7);
        let key = (model.analysis_key, expr.text.clone());
        refresh_expression(
            state,
            &key,
            &ExpressionRead {
                analysis_index: model.analysis_index,
                baseline,
                selection: sample_selection.as_ref(),
                complex_policy: expr.complex_policy,
                lanes: lanes.as_ref(),
                places: &places,
                version,
                report: true,
            },
        );
        // A `digital()` conversion is a logic lane above the panes.
        if !expr.visible || draws_as_logic(&expr.text) {
            continue;
        }
        // Until a job lands this version, the curve is the one it last
        // landed, under that version's identity.
        let cached = state
            .results
            .analysis_expr_cache
            .get(&key)
            .and_then(|cached| Some((cached.version, cached.series.as_ref().ok()?.clone())));
        let Some((evaluated, outputs)) = cached else {
            continue;
        };
        let base_color = expr_color(tokens, expr_palette_slot(model, slot));
        let base_cache_key =
            (expr_cache_key(model.analysis_key, &expr.text) ^ evaluated.rotate_left(7)) | (1 << 63);
        for output in outputs {
            let group = match output.source {
                ExpressionSource::Analysis | ExpressionSource::SelectedSamples => None,
                ExpressionSource::FamilyMember { ordinal } => {
                    let group = sample_selection
                        .as_ref()
                        .and_then(SourceSampleSelection::family_render_plan)
                        .and_then(|plan| plan.groups().get(ordinal))
                        .filter(|group| group.ordinal == ordinal);
                    // A member the reader has deselected is not drawn. This
                    // runs every frame, so it is not reported.
                    if group.is_none() {
                        continue;
                    }
                    group
                }
            };
            let base_label = expression_label(&expr, output.waveform.complex.is_some());
            let base_label = match &output.reading {
                Some(reading) => format!("{base_label} · {reading}"),
                None => base_label,
            };
            let x = output.waveform.data.x;
            let y = output.waveform.data.y;
            let family_style = group.map(|group| group.style);
            let cache_key =
                base_cache_key ^ group.map_or(0, |group| group.stable_key.rotate_left(19));
            // The evaluated version is folded into the memo key: an expression
            // re-evaluated against a new family selection produces different
            // coordinates at the same data version, and a shape held over from
            // the previous selection would route the reduction by a sweep that
            // is no longer there.
            let shape = state
                .results
                .session
                .derived
                .shape_or(cache_key, || SweepShape::of_values(&x));
            // Cached beside the shape, under the same identity: the pane's
            // automatic fit wants an expression's bounds on every frame, and
            // resolving a strip happens twice per frame, so scanning for them
            // here cost two full passes over the evaluated series each time.
            let y_extremes = state
                .results
                .session
                .derived
                .range_or(cache_key, || super::super::finite_extremes(&y));
            resolved.push(ResolvedExpr {
                slot,
                x,
                shape,
                y_extremes,
                y,
                color: family_style.map_or(base_color, |style| family_color(style, base_color)),
                cache_key,
                label: group.map_or_else(
                    || base_label.clone(),
                    |group| format!("{base_label} · {}", group.label),
                ),
                family_style,
                presentation_key: group.map_or(0, |group| group.stable_key),
                reading: output.reading.clone(),
            });
        }
    }
    resolved
}

/// Whether evaluating `text` over analysis `analysis_index` of the active run
/// reads samples a results store keeps: that analysis', or another run's it
/// names with `@`. Such an expression is evaluated off the frame.
pub(super) fn expression_reads_store(
    simulation: &SimulationState,
    analysis_index: usize,
    text: &str,
) -> bool {
    let stored = |analysis: &AnalysisResult| {
        analysis
            .waveforms
            .iter()
            .any(|waveform| waveform.x.is_stored() || waveform.y.is_stored())
    };
    let Some(run) = simulation.active_run() else {
        return false;
    };
    if run.analyses.get(analysis_index).is_some_and(stored) {
        return true;
    }
    calculator::parser::try_parse(text)
        .is_ok_and(|expr| !rspice_results::calculator::datasets::named_runs(&expr).is_empty())
        && simulation
            .retained
            .runs
            .iter()
            .any(|run| run.analyses.iter().any(stored))
}

/// What a strip, or a pane drawn against an expression, reads one at: the
/// version it is read at and what it is evaluated with.
struct ExpressionRead<'a> {
    analysis_index: usize,
    baseline: Option<crate::product::DatasetId>,
    selection: Option<&'a SourceSampleSelection>,
    complex_policy: ComplexExpressionPolicy,
    lanes: Option<&'a EventLaneLayout>,
    places: &'a Places,
    version: u64,
    /// Whether a failed evaluation goes to the log.
    report: bool,
}

/// Bring the cached series of `key` to `read.version`.
///
/// One that reads only samples held in memory is evaluated here, on the
/// frame. One that reads a store is evaluated on a Results job, one job at a
/// time per expression: a job whose version is no longer read is cancelled,
/// and once it ends it is dropped, never drawn; the next frame then starts a
/// job for the version read then, so the latest request replaces any made
/// while the job ran. The series last landed stays cached, and drawn, until
/// a job lands the version read.
fn refresh_expression(
    state: &mut ResultsParts<'_>,
    key: &(AnalysisPresentationKey, String),
    read: &ExpressionRead<'_>,
) {
    land_expression_job(state, key, read);
    let fresh = state
        .results
        .analysis_expr_cache
        .get(key)
        .is_some_and(|cached| cached.version == read.version);
    let text = &key.1;
    if !fresh && expression_reads_store(&state.model.simulation, read.analysis_index, text) {
        if !state.results.expression_jobs.contains_key(key) {
            let work = ExpressionWork::gather(
                &state.model.simulation,
                (read.analysis_index, read.baseline),
                text,
                (read.selection, read.complex_policy),
                read.lanes,
                read.places,
            );
            start_expression_job(state, key.clone(), read, work);
        }
    } else if !fresh {
        let series = evaluate_expression_reading(
            &state.model.simulation,
            (read.analysis_index, read.baseline),
            text,
            read.selection,
            read.complex_policy,
            read.lanes,
            read.places,
        );
        if let Err(error) = &series {
            report_failure(state, read, text, error);
        }
        state.results.analysis_expr_cache.insert(
            key.clone(),
            ExprSeries {
                version: read.version,
                series,
            },
        );
    }
    // A job still out lands on a later frame, so one is due.
    if state.results.expression_jobs.contains_key(key) {
        state.results.expression_frame_due = true;
    }
}

fn report_failure(
    state: &mut ResultsParts<'_>,
    read: &ExpressionRead<'_>,
    text: &str,
    error: &str,
) {
    if read.report {
        state.record(
            crate::diagnostics::LogSeverity::Warning,
            crate::diagnostics::LogSource::Studio,
            format!("Could not evaluate {text}: {error}"),
        );
    }
}

/// Start the job that evaluates `key` at `read.version`, or cache why it
/// could not start.
fn start_expression_job(
    state: &mut ResultsParts<'_>,
    key: (AnalysisPresentationKey, String),
    read: &ExpressionRead<'_>,
    work: Result<super::expression_evaluation::ExpressionWork, String>,
) {
    match work {
        Ok(work) => {
            let job = crate::workbench::results_job::ResultsJob::start(
                "results-expression",
                move |job| Ok(work.evaluate(job.cancel_flag())),
            );
            state.results.expression_jobs.insert(
                key,
                ExpressionJob {
                    version: read.version,
                    job,
                },
            );
        }
        Err(error) => {
            report_failure(state, read, &key.1, &error);
            state.results.analysis_expr_cache.insert(
                key,
                ExprSeries {
                    version: read.version,
                    series: Err(error),
                },
            );
        }
    }
}

/// Settle the job evaluating `key`: one still running for a version no
/// longer read is cancelled; one that has ended leaves, and lands in the
/// expression cache only when it evaluated the version read.
fn land_expression_job(
    state: &mut ResultsParts<'_>,
    key: &(AnalysisPresentationKey, String),
    read: &ExpressionRead<'_>,
) {
    use crate::workbench::results_job::JobOutcome;
    let Some(running) = state.results.expression_jobs.get(key) else {
        return;
    };
    if !running.job.is_finished() {
        if running.version != read.version {
            running.job.cancel();
        }
        return;
    }
    let Some(ended) = state.results.expression_jobs.remove(key) else {
        return;
    };
    if ended.version != read.version {
        return;
    }
    let series = match ended.job.take_outcome() {
        Some(JobOutcome::Done(series)) => series,
        Some(JobOutcome::Failed(reason)) => Err(reason),
        Some(JobOutcome::Cancelled) | None => return,
    };
    if let Err(error) = &series {
        report_failure(state, read, &key.1, error);
    }
    state.results.analysis_expr_cache.insert(
        key.clone(),
        ExprSeries {
            version: ended.version,
            series,
        },
    );
}

/// Evaluate `text` as the abscissa a pane is drawn against: once per family
/// member the strip shows, on the active run, in the unit the reader
/// declared for it or the one unit algebra states.
pub(super) fn resolve_abscissa_expression(
    state: &mut ResultsParts<'_>,
    model: &StripModel,
    text: &str,
) -> Result<rspice_results_ui::waves::xy::ResolvedAbscissa, String> {
    use rspice_results_ui::waves::xy::{AbscissaSeries, MemberKey, ResolvedAbscissa};
    let session = &state.results.session;
    // Read as the strip reads the same text when it also plots it, so the
    // two share one evaluation.
    let complex_policy = session
        .analysis_exprs
        .get(&model.analysis_key)
        .and_then(|exprs| exprs.iter().find(|expr| expr.text == text))
        .map_or(ComplexExpressionPolicy::Rectangular, |expr| {
            expr.complex_policy
        });
    let sample_selection = session.sample_selection.clone();
    let lanes = session.event_lanes.get(&model.analysis_key).cloned();
    let bus_reading = lanes.as_ref().map_or(0, bus_reading_fingerprint);
    let (baseline, pin) = pinned(state.results_reads());
    let places = Places::of(
        state,
        (model.analysis_key, model.analysis_index),
        Some(model),
    );
    let version = expression_version(
        state.model.simulation.view.data_version,
        sample_selection.as_ref(),
        complex_policy,
    ) ^ bus_reading.rotate_left(41)
        ^ pin
        ^ places.version_for(text).rotate_left(7);
    let key = (model.analysis_key, text.to_owned());
    refresh_expression(
        state,
        &key,
        &ExpressionRead {
            analysis_index: model.analysis_index,
            baseline,
            selection: sample_selection.as_ref(),
            complex_policy,
            lanes: lanes.as_ref(),
            places: &places,
            version,
            report: false,
        },
    );
    // Until a job lands this version, the pane is drawn against the series
    // it last landed, under that version's identity.
    let (evaluated, outputs) = match state.results.analysis_expr_cache.get(&key) {
        Some(cached) => (cached.version, cached.series.clone()?),
        None if state.results.expression_jobs.contains_key(&key) => {
            return Err(format!("{text} is being evaluated"));
        }
        None => return Err(format!("{text} was not evaluated")),
    };
    let run = state
        .model
        .simulation
        .active_run()
        .ok_or("the analysis no longer exists")?;
    let unit = state
        .results
        .session
        .expression_units
        .get(&key)
        .cloned()
        .or_else(|| {
            run.analyses
                .get(model.analysis_index)
                .and_then(|analysis| expression_unit(text, analysis, &model.x_unit).ok())
        })
        .unwrap_or_default();
    let base_key = expr_cache_key(model.analysis_key, text) ^ evaluated.rotate_left(7);
    let mut series = Vec::new();
    for output in outputs {
        if output.waveform.complex.is_some() {
            return Err(format!(
                "{text} is complex; draw against its real part, imaginary part or magnitude"
            ));
        }
        let group = match output.source {
            ExpressionSource::Analysis | ExpressionSource::SelectedSamples => None,
            ExpressionSource::FamilyMember { ordinal } => {
                let Some(group) = sample_selection
                    .as_ref()
                    .and_then(SourceSampleSelection::family_render_plan)
                    .and_then(|plan| plan.groups().get(ordinal))
                    .filter(|group| group.ordinal == ordinal)
                else {
                    continue;
                };
                Some(group)
            }
        };
        let presentation_key = group.map_or(0, |group| group.stable_key);
        series.push(AbscissaSeries {
            member: MemberKey {
                run_id: run.id,
                overlay: false,
                presentation_key,
            },
            grid: output.waveform.data.x,
            values: output.waveform.data.y,
            cache_key: base_key ^ presentation_key.rotate_left(19),
        });
    }
    Ok(ResolvedAbscissa { unit, series })
}

/// Add `text` to the strip's expressions, as a trace's or lane's menu asks
/// for a conversion. One already there is not added twice.
pub(in crate::workbench::documents::result_document) fn add_strip_expression(
    state: &mut ResultsParts<'_>,
    analysis: AnalysisPresentationKey,
    text: String,
) {
    match state
        .results
        .add_expression_trace(&state.model.simulation, analysis, text)
    {
        Ok(true) => state.model.workspace.content.visualization_documents_dirty = true,
        Ok(false) => {}
        Err(error) => {
            state.record(
                crate::diagnostics::LogSeverity::Warning,
                crate::diagnostics::LogSource::Studio,
                error,
            );
        }
    }
}

pub(super) use rspice_results_ui::waves::logic::convert::draws_as_logic;

/// What of a strip's lane layout `analog()` reads: each bus's radix and the
/// reader's buses.
pub(super) fn bus_reading_fingerprint(lanes: &EventLaneLayout) -> u64 {
    let radixes: Vec<(&String, bool)> = lanes
        .buses
        .iter()
        .map(|(label, view)| {
            (
                label,
                view.radix == rspice_results_ui::waves::logic::BusRadix::Signed,
            )
        })
        .collect();
    let users: Vec<(&String, &Vec<String>)> = lanes
        .user_buses
        .iter()
        .map(|bus| (&bus.name, &bus.members))
        .collect();
    rspice_results_ui::waves::stable_hash(&(radixes, users))
}

/// What unit algebra reads about one analysis: the unit each signal it names
/// reads in, as the strip labels that signal, and the coordinate the strip
/// runs over.
struct AnalysisUnits<'a> {
    analysis: &'a AnalysisResult,
    x_unit: &'a str,
    /// Whether a name that is none of the analysis' waveforms states no
    /// unit, rather than the analysis' default one.
    retained_only: bool,
}

impl rspice_results::calculator::evaluator::UnitContext for AnalysisUnits<'_> {
    fn signal_unit(&self, signal: &str, _dataset: Option<&str>) -> Option<String> {
        let waveform =
            rspice_results::calculator::retained::find_waveform(&self.analysis.waveforms, signal);
        if self.retained_only && waveform.is_none() {
            return None;
        }
        let name = waveform.map_or(signal, |waveform| waveform.name.as_str());
        let unit = rspice_results_ui::waves::browser_signal_unit(
            name,
            waveform.and_then(|waveform| waveform.unit.as_deref()),
            rspice_results_ui::waves::analysis_default_unit(self.analysis.analysis_type),
        );
        Some(unit.to_owned())
    }

    fn x_unit(&self) -> String {
        self.x_unit.to_owned()
    }
}

/// The unit an expression reads in by unit algebra, as its pane names it
/// (empty for a plain number), or why none can be stated.
pub(super) fn expression_unit(
    text: &str,
    analysis: &AnalysisResult,
    x_unit: &str,
) -> Result<String, String> {
    let expr =
        calculator::parser::try_parse(text).map_err(|error| format!("parse error: {error}"))?;
    rspice_results::calculator::evaluator::infer_unit(
        &expr,
        &AnalysisUnits {
            analysis,
            x_unit,
            retained_only: false,
        },
    )
    .map(|unit| unit.label())
}

/// The unit a measurement's value reads in by unit algebra, from the
/// analysis' waveforms alone: a name that is none of them (a family member's
/// own measurement, say) states no unit, so the value reads as a number
/// rather than in the analysis' default unit. `None` when no unit can be
/// stated.
pub(crate) fn measurement_unit(
    text: &str,
    analysis: &AnalysisResult,
    x_unit: &str,
) -> Option<String> {
    let expr = calculator::parser::try_parse(text).ok()?;
    rspice_results::calculator::evaluator::infer_unit(
        &expr,
        &AnalysisUnits {
            analysis,
            x_unit,
            retained_only: true,
        },
    )
    .ok()
    .map(|unit| unit.label())
}

/// Send each strip's expressions to the pane of their unit: the unit the
/// reader declared, else the one unit algebra states. One whose unit cannot
/// be stated gets an unlabeled pane of its own, and a difference from
/// another run a pane of its own below the pane of its unit. A hidden one is routed too,
/// so the pane of its unit can offer it again. An expression that failed to
/// evaluate draws nothing, so it opens no pane, and a `digital()` conversion
/// is a logic lane, not a value on any pane.
pub(super) fn route_expressions(
    models: &mut [StripModel],
    simulation: &SimulationState,
    exprs: &std::collections::HashMap<AnalysisPresentationKey, Vec<ExprTrace>>,
    declared: &std::collections::HashMap<(AnalysisPresentationKey, String), String>,
    failed: impl Fn(AnalysisPresentationKey, &str) -> bool,
) {
    let Some(run) = simulation.active_run() else {
        return;
    };
    for model in models {
        let (Some(exprs), Some(analysis)) = (
            exprs.get(&model.analysis_key),
            run.analyses.get(model.analysis_index),
        ) else {
            continue;
        };
        model.expression_panes = exprs
            .iter()
            .enumerate()
            .filter(|(_, expr)| {
                !draws_as_logic(&expr.text) && !failed(model.analysis_key, &expr.text)
            })
            .map(|(slot, expr)| {
                let declared = declared.get(&(model.analysis_key, expr.text.clone()));
                match declared.map_or_else(
                    || expression_unit(&expr.text, analysis, &model.x_unit),
                    |unit| Ok(unit.clone()),
                ) {
                    Ok(unit) => rspice_results_ui::waves::ExpressionPane {
                        slot,
                        apart: calculator::parser::try_parse(&expr.text)
                            .is_ok_and(|parsed| {
                                rspice_results::calculator::datasets::is_run_difference(&parsed)
                            })
                            .then(|| format!("{unit} \u{2014} {}", expr.text)),
                        unit,
                        unknown: None,
                        visible: expr.visible,
                        text: expr.text.clone(),
                    },
                    Err(reason) => rspice_results_ui::waves::ExpressionPane {
                        visible: expr.visible,
                        ..rspice_results_ui::waves::ExpressionPane::unlabeled(
                            slot, &expr.text, reason,
                        )
                    },
                }
            })
            .collect();
    }
}

/// Stable decimation-cache identity for an expression trace. The high bit
/// keeps it out of the waveform trace_key space.
pub(super) fn expr_cache_key(analysis: AnalysisPresentationKey, text: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    (analysis, text).hash(&mut hasher);
    hasher.finish() | (1 << 63)
}

/// Whether a source waveform is drawn: as the open saved pane says when it
/// holds the waveform, otherwise as the quick view does.
pub fn waveform_plotted(
    model: &rspice_app_state::workbench::app_state::model::Model,
    results: &crate::workbench::documents::result_document::ResultsState,
    key: &SourceWaveformPresentationKey,
    name: &str,
    dataset_default: bool,
) -> bool {
    if let Some(context) = results
        .session
        .persistent_pane_context
        .filter(|context| context.analysis == key.analysis())
        && let Some(document) = model
            .workspace
            .content
            .visualization_document(context.document_id)
    {
        let mut traces = document
            .traces()
            .iter()
            .filter(|trace| trace.pane_id == context.pane_id && trace.label == name)
            .peekable();
        if traces.peek().is_some() {
            return traces.all(|trace| trace.visible);
        }
    }
    results.session.waveform_visibility(key, dataset_default)
}

/// Flip a source waveform's quick-view visibility without mutating result data.
pub fn toggle_visibility(
    state: &mut crate::workbench::documents::result_document::ResultsParts<'_>,
    analysis_index: usize,
    waveform_index: usize,
) {
    let Some(run) = state.model.simulation.active_run() else {
        return;
    };
    let Some(analysis) = run.analyses.get(analysis_index) else {
        return;
    };
    let Some(waveform) = analysis.waveforms.get(waveform_index) else {
        return;
    };
    let dataset_default = waveform.visible;
    let name = waveform.name.clone();
    let key = SourceWaveformPresentationKey::new(
        AnalysisPresentationKey::new(run.dataset_id, analysis),
        name.clone(),
    );
    toggle_source_visibility(state, key, name, dataset_default);
}

/// Flip the quick-view visibility of the source `name` that `key` names,
/// whose visibility before the reader touched it is `dataset_default`. The name
/// need not be a column of the analysis: an event net the run keeps only as
/// history is plotted under its column's name all the same.
pub(crate) fn toggle_source_visibility(
    state: &mut crate::workbench::documents::result_document::ResultsParts<'_>,
    key: SourceWaveformPresentationKey,
    name: String,
    dataset_default: bool,
) {
    if let Some(context) = state
        .results
        .session
        .persistent_pane_context
        .filter(|context| context.analysis == key.analysis())
    {
        let retained = state
            .model
            .workspace
            .content
            .visualization_document(context.document_id)
            .map(|document| {
                let traces = document
                    .traces()
                    .iter()
                    .filter(|trace| trace.pane_id == context.pane_id && trace.label == name)
                    .map(|trace| (trace.id, trace.visible))
                    .collect::<Vec<_>>();
                (document.revision(), traces)
            });
        // A pane draws only the signals it lists, so a tick of a signal it
        // does not list puts the signal on it. A net the run keeps only as
        // history is no column to list, and is plotted as its lanes say.
        let column = key.resolve(&state.model.simulation.retained.runs).is_some();
        if let Some((revision, traces)) = &retained
            && traces.is_empty()
            && column
        {
            let edit = crate::results::visualization_document::DocumentEdit::SetSignalsPlotted {
                pane_id: context.pane_id,
                signals: vec![name.clone()],
                plotted: true,
            };
            if let Err(error) = crate::workbench::documents::result_document::view_edits::apply(
                state,
                context.document_id,
                *revision,
                vec![edit],
            ) {
                state
                    .workbench
                    .prompt(format!("Could not plot {name} in the active pane: {error}"));
                return;
            }
            state.results.session.note_recent_signal(key);
            return;
        }
        if let Some((revision, traces)) = retained
            && !traces.is_empty()
        {
            let now_visible = traces.iter().any(|(_, visible)| !visible);
            let edits = traces
                .into_iter()
                .filter_map(|(trace_id, visible)| {
                    (visible != now_visible).then_some(
                        crate::results::visualization_document::DocumentEdit::SetTraceVisibility {
                            trace_id,
                            visible: now_visible,
                        },
                    )
                })
                .collect::<Vec<_>>();
            if !edits.is_empty()
                && let Err(error) = crate::workbench::documents::result_document::view_edits::apply(
                    state,
                    context.document_id,
                    revision,
                    edits,
                )
            {
                state
                    .workbench
                    .prompt(format!("Could not change trace visibility: {error}"));
                return;
            }
            if now_visible {
                state.results.session.note_recent_signal(key);
            }
            return;
        }
    }
    let now_visible = state
        .results
        .toggle_waveform_visibility(key.clone(), dataset_default);
    // Revealing a trace is a deliberate act; feed the browser's Recent scope.
    if now_visible {
        state.results.session.note_recent_signal(key);
    }
}

/// Serialize the active Waves cursor readout for the platform clipboard.
/// This is the Edit → Copy consumer for the Units copied-value policy.
pub(crate) fn copy_cursor_text(
    parts: &mut crate::workbench::documents::result_document::ResultsParts<'_>,
) -> Option<String> {
    let x = parts.results.session.cursors.a?;
    parts
        .results
        .synchronize_wave_caches(&parts.model.simulation);
    let presentation = parts.results_reads().presentation_policy();
    let quantity_policy = parts.model.preferences.quantity_presentation_policy();
    let interpolation = cursor_interpolation(presentation.cursor_interpolation());
    let sample_selection = parts.results.session.sample_selection.clone();
    let hidden_family_traces = parts.results.session.hidden_family_traces.clone();
    let waveform_visibility = parts.results.session.waveform_visibility.clone();
    let waveform_colors = parts.results.session.waveform_colors.clone();
    let phase_continuous = parts.results.session.presented_phase_continuous();
    let ac = parts.results.session.ac_display;
    let mut models = super::build_shown_models(
        &parts.model.simulation,
        &mut parts.results.session.derived,
        &Tokens::default(),
        rspice_results_ui::waves::ProjectionOptions {
            phase_continuous,
            complex_display: presentation.complex_number_display(),
            ac,
            selection: sample_selection.as_ref(),
            hidden_family_traces: &hidden_family_traces,
            waveform_visibility: &waveform_visibility,
            signals: None,
        },
    );
    apply_waveform_visibility(
        &mut models,
        parts.model.simulation.active_run().map(AsRef::as_ref),
        &waveform_visibility,
        &waveform_colors,
        &hidden_family_traces,
        &Tokens::default().color.traces,
    );
    // Same order as the cached path: the extent is a memo of the traces the
    // strip draws, and the overrides above decide which those are.
    super::extent::resolve_x_ranges(&mut models);
    let model = parts
        .results
        .session
        .cursor_strip
        .and_then(|index| models.iter().find(|model| model.analysis_index == index))?;

    let mut text = String::new();
    append_copied_cursor(&mut text, "A", x, model, interpolation, quantity_policy);
    if let Some(b) = parts.results.session.cursors.b {
        text.push('\n');
        append_copied_cursor(&mut text, "B", b, model, interpolation, quantity_policy);
    }
    Some(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::RetainedTrace;

    #[test]
    fn complex_quick_trace_upgrade_refreshes_cached_values_without_rewriting_evidence() {
        let mut state = ResultsTestHost::default();
        state.model.simulation.start_run().add_analysis(
            crate::state::AnalysisResult::new(1, crate::state::AnalysisType::Ac, "AC")
                .with_waveforms(vec![
                    RetainedTrace::new("|V(a)|", vec![1.0, 2.0], vec![1.0; 2], "#fff")
                        .with_complex_components("V(a)", vec![1.0; 2], vec![0.0; 2]),
                    RetainedTrace::new("|V(b)|", vec![1.0, 2.0], vec![1.0; 2], "#fff")
                        .with_complex_components("V(b)", vec![-1.0; 2], vec![0.0; 2]),
                ]),
        );
        state.model.simulation.complete_run();
        state.results.results.session.viewer = super::super::super::ResultViewer::Bode;
        let digest = state
            .model
            .simulation
            .active_run()
            .unwrap()
            .dataset_content_digest();
        let policy = state.results_reads().presentation_policy();
        let models = cached_models(
            &state.model.simulation,
            &mut state.results.results,
            policy.complex_number_display(),
            &Tokens::default(),
        );
        let model = &models[0];
        let legacy: ExprTrace =
            serde_json::from_str(r#"{"text":"V(a)-V(b)","visible":true}"#).unwrap();
        assert!(legacy.complex_policy.is_legacy());
        assert!(
            serde_json::to_value(&legacy)
                .unwrap()
                .get("complex_policy")
                .is_none()
        );
        state
            .results
            .results
            .session
            .analysis_exprs
            .insert(model.analysis_key, vec![legacy]);
        state
            .results
            .results
            .session
            .sync_expression_projection(model.analysis_key, 0);
        let old = resolve_strip_exprs(&mut state.results_parts(), model, &Tokens::default());
        assert_eq!(old[0].y.read_whole().unwrap().as_ref(), &[0.0; 2]);
        assert!(old[0].label.starts_with("mag("));
        assert!(
            state
                .results
                .results
                .add_expression_trace(
                    &state.model.simulation,
                    model.analysis_key,
                    "V(a)-V(b)".to_owned()
                )
                .unwrap()
        );
        let current = resolve_strip_exprs(&mut state.results_parts(), model, &Tokens::default());
        assert_eq!(current[0].y.read_whole().unwrap().as_ref(), &[2.0; 2]);
        assert_eq!(current[0].label, "mag(V(a)-V(b))");
        assert_ne!(old[0].cache_key, current[0].cache_key);
        assert_eq!(
            state
                .model
                .simulation
                .active_run()
                .unwrap()
                .dataset_content_digest(),
            digest
        );
        assert!(
            !state
                .results
                .results
                .add_expression_trace(
                    &state.model.simulation,
                    model.analysis_key,
                    "V(a)-V(b)".to_owned()
                )
                .unwrap()
        );
    }
}
