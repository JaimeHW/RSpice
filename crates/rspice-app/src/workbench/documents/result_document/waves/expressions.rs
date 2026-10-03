//! Per-strip expression traces: editing them and evaluating them.
//!
//! An expression is re-evaluated only when its text or the data behind it
//! changes, so the cache key carries both — a stale result can never be shown
//! against new data. An expression that fails to resolve is reported on its
//! own row rather than being dropped, so the strip never silently loses a
//! trace the engineer asked for.

use super::*;
use crate::state::ComplexExpressionPolicy;
use rspice_results_ui::waves::cache::apply_waveform_visibility;
use rspice_results_ui::waves::expression_editor::Action;
pub(super) use rspice_results_ui::waves::stack::{expr_color, expr_palette_slot, expression_label};

/// The inline expression editor row under a strip header (when open for
/// this strip): mono input, Enter/Add commits, Esc closes, and a bounded
/// validation message that moves below the controls on compact surfaces.
pub(super) fn expr_editor_row(
    ui: &mut Ui,
    state: &mut AppState,
    analysis_key: AnalysisPresentationKey,
    analysis_index: usize,
) {
    let scope_note = state
        .ui
        .results
        .session
        .sample_selection
        .as_ref()
        .and_then(|selection| {
            let run = state.simulation.active_run()?;
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
    let Some(editor) = state
        .ui
        .results
        .session
        .expr_editor
        .as_mut()
        .filter(|editor| editor.analysis == analysis_key)
    else {
        return;
    };

    let action = rspice_results_ui::waves::expression_editor::show(ui, editor, scope_note);

    match action {
        Action::None => {}
        Action::Cancel => state.ui.results.session.expr_editor = None,
        Action::Commit => {
            let text = state
                .ui
                .results
                .session
                .expr_editor
                .as_ref()
                .map(|e| e.text.trim().to_owned())
                .unwrap_or_default();
            if text.is_empty() {
                state.ui.results.session.expr_editor = None;
                return;
            }
            let sample_selection = state.ui.results.session.sample_selection.clone();
            let series = evaluate_expression(
                &state.simulation,
                analysis_index,
                &text,
                sample_selection.as_ref(),
            );
            match series {
                Ok(series) => {
                    state.ui.results.analysis_expr_cache.insert(
                        (analysis_key, text.clone()),
                        ExprSeries {
                            version: expression_version(
                                state.simulation.data_version,
                                sample_selection.as_ref(),
                                ComplexExpressionPolicy::Rectangular,
                            ),
                            series: Ok(series),
                        },
                    );
                    let added = state
                        .ui
                        .results
                        .add_expression_trace(&state.simulation, analysis_key, text)
                        .expect("the expression editor is bound to a retained analysis");
                    if added {
                        state.workspace.content.visualization_documents_dirty = true;
                    }
                    state.ui.results.session.expr_editor = None;
                }
                Err(error) => {
                    if let Some(editor) = state.ui.results.session.expr_editor.as_mut() {
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
    state: &mut AppState,
    model: &StripModel,
    tokens: &Tokens,
) -> Vec<ResolvedExpr> {
    let exprs: Vec<(usize, ExprTrace)> = state
        .ui
        .results
        .session
        .analysis_exprs
        .get(&model.analysis_key)
        .map(|list| list.iter().cloned().enumerate().collect())
        .unwrap_or_default();
    if exprs.is_empty() {
        return Vec::new();
    }

    let sample_selection = state.ui.results.session.sample_selection.clone();
    let mut resolved = Vec::new();
    for (slot, expr) in exprs {
        let version = expression_version(
            state.simulation.data_version,
            sample_selection.as_ref(),
            expr.complex_policy,
        );
        let key = (model.analysis_key, expr.text.clone());
        let fresh = state
            .ui
            .results
            .analysis_expr_cache
            .get(&key)
            .is_some_and(|s| s.version == version);
        if !fresh {
            let series = evaluate_expression_with_policy(
                &state.simulation,
                model.analysis_index,
                &expr.text,
                sample_selection.as_ref(),
                expr.complex_policy,
            );
            if let Err(error) = &series {
                state.push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
                    "expression `{}`: {}",
                    expr.text, error
                )));
            }
            state
                .ui
                .results
                .analysis_expr_cache
                .insert(key.clone(), ExprSeries { version, series });
        }
        if !expr.visible {
            continue;
        }
        let cached = state
            .ui
            .results
            .analysis_expr_cache
            .get(&key)
            .and_then(|cached| cached.series.as_ref().ok())
            .cloned();
        let Some(outputs) = cached else {
            continue;
        };
        let base_color = expr_color(tokens, expr_palette_slot(model, slot));
        let base_cache_key =
            (expr_cache_key(model.analysis_key, &expr.text) ^ version.rotate_left(7)) | (1 << 63);
        for output in outputs {
            let group = match output.source {
                ExpressionSource::Analysis | ExpressionSource::SelectedSamples => None,
                ExpressionSource::FamilyMember { ordinal } => {
                    let group = sample_selection
                        .as_ref()
                        .and_then(SourceSampleSelection::family_render_plan)
                        .and_then(|plan| plan.groups().get(ordinal))
                        .filter(|group| group.ordinal == ordinal);
                    if group.is_none() {
                        state.push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
                            "expression `{}`: its evaluated family member is no longer selected",
                            expr.text
                        )));
                        continue;
                    }
                    group
                }
            };
            let base_label = expression_label(&expr, output.waveform.complex.is_some());
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
                .ui
                .results
                .session
                .derived
                .shape_or(cache_key, || SweepShape::of(&x));
            // Cached beside the shape, under the same identity: the pane's
            // automatic fit wants an expression's bounds on every frame, and
            // resolving a strip happens twice per frame, so scanning for them
            // here cost two full passes over the evaluated series each time.
            let y_extremes = state
                .ui
                .results
                .session
                .derived
                .range_or(cache_key, || super::super::finite_extremes(&y));
            resolved.push(ResolvedExpr {
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
            });
        }
    }
    resolved
}

/// Stable decimation-cache identity for an expression trace. The high bit
/// keeps it out of the waveform trace_key space.
pub(super) fn expr_cache_key(analysis: AnalysisPresentationKey, text: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    (analysis, text).hash(&mut hasher);
    hasher.finish() | (1 << 63)
}

/// Flip a source waveform's quick-view visibility without mutating result data.
pub(crate) fn toggle_visibility(
    state: &mut AppState,
    analysis_index: usize,
    waveform_index: usize,
) {
    let Some(run) = state.simulation.active_run() else {
        return;
    };
    let Some(analysis) = run.analyses.get(analysis_index) else {
        return;
    };
    let Some(waveform) = analysis.waveforms.get(waveform_index) else {
        return;
    };
    let name = waveform.name.clone();
    let dataset_default = waveform.visible;
    let key = SourceWaveformPresentationKey::new(
        AnalysisPresentationKey::new(run.dataset_id, analysis),
        name.clone(),
    );
    if let Some(context) = state
        .ui
        .results
        .session
        .persistent_pane_context
        .filter(|context| context.analysis == key.analysis())
    {
        let retained = state
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
                && let Err(error) = state.workspace.content.transact_visualization_document(
                    context.document_id,
                    revision,
                    edits,
                )
            {
                state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                    "Could not retain trace visibility: {error}"
                )));
                return;
            }
            if now_visible {
                state.ui.results.session.note_recent_signal(key);
            }
            return;
        }
    }
    let now_visible = state
        .ui
        .results
        .toggle_waveform_visibility(key.clone(), dataset_default);
    // Revealing a trace is a deliberate act; feed the browser's Recent scope.
    if now_visible {
        state.ui.results.session.note_recent_signal(key);
    }
}

/// Serialize the active Waves cursor readout for the platform clipboard.
/// This is the Edit → Copy consumer for the Units copied-value policy.
pub(crate) fn copy_cursor_text(state: &mut AppState) -> Option<String> {
    let x = state.ui.results.session.cursors.a?;
    state.ui.results.synchronize_wave_caches(&state.simulation);
    let presentation = state.ui.preferences.result_presentation_policy();
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let interpolation = cursor_interpolation(presentation.cursor_interpolation());
    let sample_selection = state.ui.results.session.sample_selection.clone();
    let hidden_family_traces = state.ui.results.session.hidden_family_traces.clone();
    let waveform_visibility = state.ui.results.session.waveform_visibility.clone();
    let mut models = build_models(
        &state.simulation,
        &mut state.ui.results.session.derived,
        &Tokens::default(),
        state.ui.results.session.phase_continuous,
        presentation.complex_number_display(),
        sample_selection.as_ref(),
        &hidden_family_traces,
    );
    apply_waveform_visibility(
        &mut models,
        state.simulation.active_run().map(AsRef::as_ref),
        &waveform_visibility,
        &hidden_family_traces,
    );
    // Same order as the cached path: the extent is a memo of the traces the
    // strip draws, and the overrides above decide which those are.
    super::extent::resolve_x_ranges(&mut models);
    let model = state
        .ui
        .results
        .session
        .cursor_strip
        .and_then(|index| models.iter().find(|model| model.analysis_index == index))?;

    let mut text = String::new();
    append_copied_cursor(&mut text, "A", x, model, interpolation, quantity_policy);
    if let Some(b) = state.ui.results.session.cursors.b {
        text.push('\n');
        append_copied_cursor(&mut text, "B", b, model, interpolation, quantity_policy);
    }
    Some(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::WaveformData;

    #[test]
    fn complex_quick_trace_upgrade_refreshes_cached_values_without_rewriting_evidence() {
        let mut state = AppState::default();
        state.simulation.start_run().add_analysis(
            crate::state::AnalysisResult::new(1, crate::state::AnalysisType::Ac, "AC")
                .with_waveforms(vec![
                    WaveformData::new("|V(a)|", vec![1.0, 2.0], vec![1.0; 2], "#fff")
                        .with_complex_components("V(a)", vec![1.0; 2], vec![0.0; 2]),
                    WaveformData::new("|V(b)|", vec![1.0, 2.0], vec![1.0; 2], "#fff")
                        .with_complex_components("V(b)", vec![-1.0; 2], vec![0.0; 2]),
                ]),
        );
        state.simulation.complete_run();
        state.ui.results.session.viewer = super::super::super::ResultViewer::Bode;
        let digest = state
            .simulation
            .active_run()
            .unwrap()
            .dataset_content_digest();
        let policy = state.ui.preferences.result_presentation_policy();
        let models = cached_models(
            &state.simulation,
            &mut state.ui.results,
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
            .ui
            .results
            .session
            .analysis_exprs
            .insert(model.analysis_key, vec![legacy]);
        state
            .ui
            .results
            .session
            .sync_expression_projection(model.analysis_key, 0);
        let old = resolve_strip_exprs(&mut state, model, &Tokens::default());
        assert_eq!(old[0].y.as_slice(), &[0.0; 2]);
        assert!(old[0].label.starts_with("legacy magnitude"));
        assert!(
            state
                .ui
                .results
                .add_expression_trace(
                    &state.simulation,
                    model.analysis_key,
                    "V(a)-V(b)".to_owned()
                )
                .unwrap()
        );
        let current = resolve_strip_exprs(&mut state, model, &Tokens::default());
        assert_eq!(current[0].y.as_slice(), &[2.0; 2]);
        assert_eq!(current[0].label, "mag(V(a)-V(b))");
        assert_ne!(old[0].cache_key, current[0].cache_key);
        assert_eq!(
            state
                .simulation
                .active_run()
                .unwrap()
                .dataset_content_digest(),
            digest
        );
        assert!(
            !state
                .ui
                .results
                .add_expression_trace(
                    &state.simulation,
                    model.analysis_key,
                    "V(a)-V(b)".to_owned()
                )
                .unwrap()
        );
    }
}
