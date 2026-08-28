//! Per-strip expression traces: editing them and evaluating them.
//!
//! An expression is re-evaluated only when its text or the data behind it
//! changes, so the cache key carries both — a stale result can never be shown
//! against new data. An expression that fails to resolve is reported on its
//! own row rather than being dropped, so the strip never silently loses a
//! trace the engineer asked for.

use super::*;

/// Palette color for the i-th trace slot of a strip (waveforms, then
/// expressions).
pub(super) fn expr_color(tokens: &Tokens, slot: usize) -> egui::Color32 {
    tokens.color.traces[slot % tokens.color.traces.len()]
}

/// The palette slot the strip's `slot`-th expression draws in.
///
/// Waveform traces take the leading slots and expressions the ones after —
/// and "the ones after" has to be counted the same way wherever the colour is
/// asked for. The legend counted only the active run's traces while the
/// canvas counted every trace it held, overlays included, so the moment a
/// strip carried a second run the chip beside an expression was a different
/// colour from the curve it named.
pub(super) fn expr_palette_slot(model: &StripModel, slot: usize) -> usize {
    model.traces.len() + slot
}

pub(super) const EXPR_EDITOR_PADDING_X: f32 = 10.0;
pub(super) const EXPR_EDITOR_PADDING_Y: f32 = 5.0;
pub(super) const EXPR_EDITOR_GAP: f32 = 8.0;
pub(super) const EXPR_EDITOR_COMPACT_WIDTH: f32 = 560.0;
pub(super) const EXPR_EDITOR_MIN_INLINE_INPUT: f32 = 160.0;
pub(super) const EXPR_EDITOR_ERROR_HEIGHT: f32 = 20.0;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(super) struct EditorSpan {
    pub(super) start: f32,
    pub(super) width: f32,
}

impl EditorSpan {
    pub(super) fn end(self) -> f32 {
        self.start + self.width
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(super) struct ExprEditorLayout {
    pub(super) label: EditorSpan,
    pub(super) input: EditorSpan,
    pub(super) error: EditorSpan,
    pub(super) add: EditorSpan,
    pub(super) stack_error: bool,
}

pub(super) fn expr_editor_layout(
    available_width: f32,
    label_natural_width: f32,
    add_natural_width: f32,
    error_natural_width: Option<f32>,
) -> ExprEditorLayout {
    let available_width = available_width.max(0.0);
    // Reserve the commit action from the right edge before allocating any
    // free-form text. This is the invariant the former fixed 340 px input
    // violated on phone-sized strips.
    let add_width = add_natural_width.max(0.0).min(available_width);
    let add = EditorSpan {
        start: available_width - add_width,
        width: add_width,
    };
    let before_add = if add_width > 0.0 {
        (add.start - EXPR_EDITOR_GAP).max(0.0)
    } else {
        available_width
    };
    let label_width = label_natural_width.max(0.0).min(before_add);
    let label = EditorSpan {
        start: 0.0,
        width: label_width,
    };
    let input_start = if label_width > 0.0 {
        (label.end() + EXPR_EDITOR_GAP).min(before_add)
    } else {
        0.0
    };

    let stack_error = error_natural_width.is_some()
        && (available_width <= EXPR_EDITOR_COMPACT_WIDTH
            || before_add - input_start < EXPR_EDITOR_MIN_INLINE_INPUT + 88.0);
    let mut input_end = before_add;
    let mut error = EditorSpan::default();
    if let Some(error_natural_width) = error_natural_width.filter(|_| !stack_error) {
        let error_budget =
            (before_add - input_start - EXPR_EDITOR_MIN_INLINE_INPUT - EXPR_EDITOR_GAP).max(0.0);
        let error_width = error_natural_width
            .max(0.0)
            .min(available_width * 0.28)
            .min(error_budget);
        if error_width > 0.0 {
            error = EditorSpan {
                start: before_add - error_width,
                width: error_width,
            };
            input_end = (error.start - EXPR_EDITOR_GAP).max(input_start);
        }
    }

    ExprEditorLayout {
        label,
        input: EditorSpan {
            start: input_start,
            width: (input_end - input_start).max(0.0),
        },
        error,
        add,
        stack_error,
    }
}

/// The inline expression editor row under a strip header (when open for
/// this strip): mono input, Enter/Add commits, Esc closes, and a bounded
/// validation message that moves below the controls on compact surfaces.
pub(super) fn expr_editor_row(
    ui: &mut Ui,
    state: &mut AppState,
    analysis_key: AnalysisPresentationKey,
    analysis_index: usize,
) {
    let Some(editor) = state
        .ui
        .results
        .expr_editor
        .as_mut()
        .filter(|editor| editor.analysis == analysis_key)
    else {
        return;
    };

    let t = Tokens::get(ui.ctx());
    let c = t.color;

    enum Action {
        None,
        Commit,
        Cancel,
    }
    let mut action = Action::None;

    let label_font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let input_font = theme::mono(tokens::FS_1, FontWeight::Regular);
    let error_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let label_width = ui
        .painter()
        .layout_no_wrap("expr".to_owned(), label_font.clone(), c.text_dim)
        .size()
        .x;
    let add_width = ui
        .painter()
        .layout_no_wrap(
            "Add".to_owned(),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            c.text,
        )
        .size()
        .x
        + 20.0;
    let error_width = editor.error.as_ref().map(|error| {
        ui.painter()
            .layout_no_wrap(error.clone(), error_font.clone(), c.err)
            .size()
            .x
    });
    let inner_width = (ui.available_width() - 2.0 * EXPR_EDITOR_PADDING_X).max(0.0);
    let layout = expr_editor_layout(inner_width, label_width, add_width, error_width);
    let control_row_height = t.metrics.ctl_h + 2.0 * EXPR_EDITOR_PADDING_Y;
    let total_height = control_row_height
        + if layout.stack_error {
            EXPR_EDITOR_ERROR_HEIGHT
        } else {
            0.0
        };
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), total_height),
        egui::Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, c.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, c.border),
    );

    let control_rect = egui::Rect::from_min_size(
        egui::pos2(
            rect.left() + EXPR_EDITOR_PADDING_X,
            rect.top() + EXPR_EDITOR_PADDING_Y,
        ),
        egui::vec2(inner_width, t.metrics.ctl_h),
    );
    let span_rect = |span: EditorSpan| {
        egui::Rect::from_min_max(
            egui::pos2(control_rect.left() + span.start, control_rect.top()),
            egui::pos2(control_rect.left() + span.end(), control_rect.bottom()),
        )
    };
    let label_rect = span_rect(layout.label);
    ui.painter().with_clip_rect(label_rect).text(
        label_rect.left_center(),
        egui::Align2::LEFT_CENTER,
        "expr",
        label_font,
        c.text_dim,
    );

    let input_rect = span_rect(layout.input);
    let response = ui
        .scope_builder(
            egui::UiBuilder::new()
                .max_rect(input_rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
            |ui| {
                ui.set_clip_rect(input_rect);
                ui.add_sized(
                    input_rect.size(),
                    egui::TextEdit::singleline(&mut editor.text)
                        .font(input_font)
                        .hint_text("V(out)/V(in) - dB(V(out)) - deriv(V(out))")
                        .desired_width(input_rect.width()),
                )
            },
        )
        .inner;
    if editor.want_focus {
        response.request_focus();
        editor.want_focus = false;
    }
    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
        action = Action::Commit;
    }
    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
        action = Action::Cancel;
    }
    let add_rect = span_rect(layout.add);
    let add_clicked = ui
        .scope_builder(
            egui::UiBuilder::new()
                .max_rect(add_rect)
                .layout(egui::Layout::right_to_left(egui::Align::Center)),
            |ui| {
                ui.set_clip_rect(add_rect);
                crate::ui::widgets::Button::new("Add")
                    .min_width(add_rect.width())
                    .max_width(add_rect.width())
                    .show(ui)
            },
        )
        .inner
        .clicked();
    if add_clicked {
        action = Action::Commit;
    }
    if let Some(error) = &editor.error {
        let error_rect = if layout.stack_error {
            egui::Rect::from_min_max(
                egui::pos2(
                    control_rect.left() + layout.input.start,
                    rect.top() + control_row_height,
                ),
                egui::pos2(control_rect.right(), rect.bottom()),
            )
        } else {
            span_rect(layout.error)
        };
        if error_rect.width() > 0.0 {
            ui.scope_builder(
                egui::UiBuilder::new()
                    .max_rect(error_rect)
                    .layout(egui::Layout::left_to_right(egui::Align::Center)),
                |ui| {
                    ui.set_clip_rect(error_rect);
                    ui.add_sized(
                        error_rect.size(),
                        egui::Label::new(egui::RichText::new(error).font(error_font).color(c.err))
                            .truncate(),
                    )
                    .on_hover_text(error);
                },
            );
        }
    }

    match action {
        Action::None => {}
        Action::Cancel => state.ui.results.expr_editor = None,
        Action::Commit => {
            let text = state
                .ui
                .results
                .expr_editor
                .as_ref()
                .map(|e| e.text.trim().to_owned())
                .unwrap_or_default();
            if text.is_empty() {
                state.ui.results.expr_editor = None;
                return;
            }
            let sample_selection = state.ui.results.sample_selection.clone();
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
                        state.workspace.visualization_documents_dirty = true;
                    }
                    state.ui.results.expr_editor = None;
                }
                Err(error) => {
                    if let Some(editor) = state.ui.results.expr_editor.as_mut() {
                        editor.error = Some(error);
                        editor.want_focus = true;
                    }
                }
            }
        }
    }
}

/// Evaluate one expression against an analysis' waveforms. Scalars become a
/// constant trace across the analysis' x span.
pub(super) fn evaluate_expression(
    simulation: &SimulationState,
    analysis_index: usize,
    text: &str,
    selection: Option<&SourceSampleSelection>,
) -> WaveformSeriesResult {
    let Some(run) = simulation.active_run() else {
        return Err("analysis no longer exists".to_owned());
    };
    let Some(analysis) = run.analyses.get(analysis_index) else {
        return Err("analysis no longer exists".to_owned());
    };
    let selection = selection.filter(|selection| {
        selection.dataset_id == run.dataset_id && selection.analysis_sequence == analysis.id
    });

    let ctx = calculator::WaveformsContext::new(&analysis.waveforms);
    let expr = match calculator::parser::try_parse(text) {
        Ok(expr) => expr,
        Err(error) => return Err(format!("parse error: {error}")),
    };
    match calculator::evaluator::evaluate(&expr, &ctx) {
        Ok(calculator::CalcValue::Waveform(x, y)) if !x.is_empty() => {
            let (x, y) = match selection {
                None => (x, y),
                Some(selection)
                    if x.len() == y.len()
                        && selection
                            .source_indices
                            .last()
                            .is_none_or(|index| *index < x.len()) =>
                {
                    (
                        selection
                            .source_indices
                            .iter()
                            .map(|index| x[*index])
                            .collect(),
                        selection
                            .source_indices
                            .iter()
                            .map(|index| y[*index])
                            .collect(),
                    )
                }
                Some(_) => {
                    return Err(
                        "expression sample count does not match the retained family manifest"
                            .to_owned(),
                    );
                }
            };
            Ok((x.into(), y.into()))
        }
        Ok(calculator::CalcValue::Waveform(..)) => Err("expression produced no samples".to_owned()),
        Ok(calculator::CalcValue::Scalar(value)) => {
            if let Some(selection) = selection {
                let selected_x = analysis.waveforms.first().and_then(|waveform| {
                    selected_series_pair(&waveform.x, &waveform.y, Some(selection)).map(|(x, _)| x)
                });
                return match selected_x {
                    Some(x) if !x.is_empty() => {
                        let y = vec![value; x.len()];
                        Ok((x, y.into()))
                    }
                    _ => Err("scalar result with no selected X rows".to_owned()),
                };
            }
            let span = analysis.waveforms.first().and_then(|waveform| {
                let (x, _) = selected_series_pair(&waveform.x, &waveform.y, selection)?;
                (x.len() >= 2).then(|| (x[0], x[x.len() - 1]))
            });
            match span {
                Some((x0, x1)) => Ok((vec![x0, x1].into(), vec![value, value].into())),
                None => Err("scalar result with no x span".to_owned()),
            }
        }
        Err(error) => Err(error.to_string()),
    }
}

pub(super) fn expression_version(
    data_version: u64,
    selection: Option<&SourceSampleSelection>,
) -> u64 {
    data_version
        ^ selection
            .map(SourceSampleSelection::fingerprint)
            .unwrap_or_default()
            .rotate_left(23)
}

/// One expression trace resolved for plotting.
pub(super) struct ResolvedExpr {
    pub(super) x: SharedWaveformValues,
    pub(super) y: SharedWaveformValues,
    /// What the expression's abscissa is, on the same terms as a waveform
    /// trace's: an expression over a reverse sweep is still a reverse sweep.
    pub(super) shape: Arc<SweepShape>,
    pub(super) color: egui::Color32,
    pub(super) cache_key: u64,
    pub(super) label: String,
    pub(super) y_extremes: Option<(f64, f64)>,
    pub(super) family_style: Option<FamilyTraceStyle>,
}

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
        .analysis_exprs
        .get(&model.analysis_key)
        .map(|list| list.iter().cloned().enumerate().collect())
        .unwrap_or_default();
    if exprs.is_empty() {
        return Vec::new();
    }

    let sample_selection = state.ui.results.sample_selection.clone();
    let version = expression_version(state.simulation.data_version, sample_selection.as_ref());
    let mut resolved = Vec::new();
    for (slot, expr) in exprs {
        let key = (model.analysis_key, expr.text.clone());
        let fresh = state
            .ui
            .results
            .analysis_expr_cache
            .get(&key)
            .is_some_and(|s| s.version == version);
        if !fresh {
            let series = evaluate_expression(
                &state.simulation,
                model.analysis_index,
                &expr.text,
                sample_selection.as_ref(),
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
            .and_then(|cached| {
                cached
                    .series
                    .as_ref()
                    .ok()
                    .map(|(x, y)| (Arc::clone(x), Arc::clone(y)))
            });
        let Some((x, y)) = cached else {
            continue;
        };
        let Some(projections) = projected_selected_family_series(&x, &y, sample_selection.as_ref())
        else {
            state.push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
                "expression `{}`: selected rows do not match the active family render plan",
                expr.text
            )));
            continue;
        };
        let base_color = expr_color(tokens, expr_palette_slot(model, slot));
        let base_cache_key = expr_cache_key(model.analysis_key, &expr.text);
        let base_label = elide(&expr.text, 24);
        for projection in projections {
            let family_style = projection.group.map(|group| group.style);
            let cache_key = base_cache_key
                ^ projection
                    .group
                    .map_or(0, |group| group.stable_key.rotate_left(19));
            // The evaluated version is folded into the memo key: an expression
            // re-evaluated against a new family selection produces different
            // coordinates at the same data version, and a shape held over from
            // the previous selection would route the reduction by a sweep that
            // is no longer there.
            let shape = state
                .ui
                .results
                .derived
                .shape_or(cache_key ^ version.rotate_left(7), || {
                    SweepShape::of(&projection.x)
                });
            // Cached beside the shape, under the same identity: the pane's
            // automatic fit wants an expression's bounds on every frame, and
            // resolving a strip happens twice per frame, so scanning for them
            // here cost two full passes over the evaluated series each time.
            let y_extremes = state
                .ui
                .results
                .derived
                .range_or(cache_key ^ version.rotate_left(7), || {
                    super::super::finite_extremes(&projection.y)
                });
            resolved.push(ResolvedExpr {
                x: projection.x,
                shape,
                y_extremes,
                y: projection.y,
                color: family_style.map_or(base_color, |style| family_color(style, base_color)),
                cache_key,
                label: projection.group.map_or_else(
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
        .persistent_pane_context
        .filter(|context| context.analysis == key.analysis)
    {
        let retained = state
            .workspace
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
                && let Err(error) = state.workspace.transact_visualization_document(
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
                state.ui.results.note_recent_signal(key);
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
        state.ui.results.note_recent_signal(key);
    }
}

/// Serialize the active Waves cursor readout for the platform clipboard.
/// This is the Edit → Copy consumer for the Units copied-value policy.
pub(crate) fn copy_cursor_text(state: &mut AppState) -> Option<String> {
    let x = state.ui.results.cursors.a?;
    let presentation = state.ui.preferences.result_presentation_policy();
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let interpolation = cursor_interpolation(presentation.cursor_interpolation());
    let sample_selection = state.ui.results.sample_selection.clone();
    let hidden_family_traces = state.ui.results.hidden_family_traces.clone();
    let waveform_visibility = state.ui.results.waveform_visibility.clone();
    let mut models = build_models(
        &state.simulation,
        &mut state.ui.results.derived,
        &Tokens::default(),
        state.ui.results.phase_continuous,
        presentation.complex_number_display(),
        sample_selection.as_ref(),
        &hidden_family_traces,
    );
    apply_waveform_visibility(
        &mut models,
        &state.simulation,
        &waveform_visibility,
        &hidden_family_traces,
    );
    // Same order as the cached path: the extent is a memo of the traces the
    // strip draws, and the overrides above decide which those are.
    super::extent::resolve_x_ranges(&mut models);
    let model = state
        .ui
        .results
        .cursor_strip
        .and_then(|index| models.iter().find(|model| model.analysis_index == index))?;

    let mut text = String::new();
    append_copied_cursor(&mut text, "A", x, model, interpolation, quantity_policy);
    if let Some(b) = state.ui.results.cursors.b {
        text.push('\n');
        append_copied_cursor(&mut text, "B", b, model, interpolation, quantity_policy);
    }
    Some(text)
}
