//! The analysis list beside the editor: its rows, group headers, index discs
//! and switches, and where each analysis stands in the plan's newest run.

use super::*;

pub(super) const ANALYSIS_ROW_HEIGHT: f32 = 53.0;
pub(super) const ANALYSIS_GROUP_HEADER_HEIGHT: f32 = 26.0;
const ANALYSIS_INDEX_DIAMETER: f32 = 22.0;
const ANALYSIS_ROW_LEFT_PADDING: f32 = 9.0;
const ANALYSIS_INDEX_LABEL_GAP: f32 = 8.0;
const ANALYSIS_ROW_TRAILING_PADDING: f32 = 9.0;
const ANALYSIS_LABEL_MIN_WIDTH: f32 = 64.0;
const ANALYSIS_SWITCH_MAX_HIT_WIDTH: f32 = 44.0;
const ANALYSIS_SWITCH_LABEL_GAP: f32 = 7.0;

#[derive(Clone)]
pub(super) struct AnalysisStackRow {
    id: AnalysisInstanceId,
    pub(super) kind: AnalysisKind,
    /// What this instance is called. Resolved once, when the row is built, so
    /// the title, the accessibility name, and the enable switch cannot drift
    /// apart within a frame.
    name: String,
    enabled: bool,
    lifecycle: AnalysisLifecycleState,
    /// Where this instance stands in the plan's newest run, when that run
    /// authorized a task for it. `None` when the plan has never run, or when
    /// this instance was added since.
    run_state: Option<InstanceRunState>,
    summary: String,
    /// What this instance costs the queue: points, tasks, modelled duration.
    /// `None` for a disabled instance, which contributes nothing and is priced
    /// nowhere. Projected from [`workload::PlanWorkload`], the one owner of
    /// that arithmetic, so a row and the Run Set page's rate table cannot
    /// disagree about the same instance.
    workload: Option<String>,
    issue_count: usize,
}

#[derive(Debug, Clone, Copy)]
enum StackAction {
    Select(AnalysisInstanceId),
    SetEnabled(AnalysisInstanceId, bool),
}

pub(super) const fn analysis_row_content_min_width() -> f32 {
    ANALYSIS_ROW_LEFT_PADDING
        + ANALYSIS_INDEX_DIAMETER
        + ANALYSIS_INDEX_LABEL_GAP
        + ANALYSIS_LABEL_MIN_WIDTH
        + ANALYSIS_SWITCH_LABEL_GAP
        + ANALYSIS_SWITCH_MAX_HIT_WIDTH
        + ANALYSIS_ROW_TRAILING_PADDING
}

pub(super) fn ordered_instance_stack(
    ui: &mut Ui,
    cx: &mut Cx<'_, SimulateFeature>,
    participation: &participation::PlanParticipation,
) {
    let rows = match analysis_stack_rows(cx.model, participation) {
        Ok(rows) => rows,
        Err(error) => {
            flat_notice(ui, |ui| {
                page_kit::note_line(ui, "Plan unavailable", page_kit::Tone::Error);
                page_kit::note_line(ui, &error, page_kit::Tone::Dim);
            });
            return;
        }
    };
    let active = cx.state.active_analysis_instance;
    let mut action = None;
    let mut add_analysis = false;
    let t = Tokens::get(ui.ctx());
    egui::Frame::new().fill(t.color.bg_panel).show(ui, |ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        ui.set_width(ui.available_width());
        if rows.is_empty() {
            // The notice used to name the navigator and stop there, which made
            // it a dead end on the two states where the navigator is not on
            // screen — the panel collapsed, and focus mode. The empty rail is
            // where a plan is started, so it carries the action itself.
            flat_notice(ui, |ui| {
                page_kit::note_line(ui, "No analysis instances", page_kit::Tone::Text);
                ui.add_space(4.0);
                add_analysis = Button::new(SimulateCommand::AddAnalysis.spec().label)
                    .accent()
                    .min_width(ui.available_width())
                    .show(ui)
                    .clicked();
            });
        } else {
            let mut displayed_position = 0usize;
            let mut active_row = None;
            for group in ANALYSIS_CATEGORY_ORDER {
                let members = rows
                    .iter()
                    .filter(|row| analysis_catalog_group(row.kind) == group)
                    .collect::<Vec<_>>();
                if members.is_empty() {
                    continue;
                }
                analysis_group_header(ui, group, members.len());
                for row in members {
                    displayed_position += 1;
                    let top = ui.cursor().top();
                    let is_active = active == Some(row.id);
                    if let Some(row_action) =
                        analysis_stack_row(ui, row, displayed_position, is_active)
                    {
                        action = Some(row_action);
                    }
                    if is_active {
                        active_row = Some(Rect::from_x_y_ranges(
                            ui.max_rect().x_range(),
                            top..=ui.cursor().top(),
                        ));
                    }
                }
            }
            reveal_newly_active_row(ui, active, active_row);
        }
    });

    if add_analysis {
        cx.execute(SimulateCommand::AddAnalysis.key());
    }

    match action {
        Some(StackAction::Select(id)) => {
            cx.state.active_analysis_instance = Some(id);
        }
        Some(StackAction::SetEnabled(id, enabled)) => {
            apply_analysis_action(cx, id, AnalysisAction::SetEnabled(enabled));
        }
        None => {}
    }
}

/// Scroll the stack to the active analysis once each time it changes.
///
/// The stack scrolls on its own beside the editor, so an analysis made active
/// from elsewhere — the catalogue selects what it inserts, a blocker's reveal
/// selects what it names — can be out of its view. Pressing a row changes it
/// too, and that row is already in view, so the scroll is a no-op there.
fn reveal_newly_active_row(ui: &Ui, active: Option<AnalysisInstanceId>, row: Option<Rect>) {
    let (Some(active), Some(row)) = (active, row) else {
        return;
    };
    let revealed = egui::Id::new("workbench.simulate.analysis-stack.revealed");
    if ui.data(|data| data.get_temp::<AnalysisInstanceId>(revealed)) != Some(active) {
        ui.scroll_to_rect(row, None);
        ui.data_mut(|data| data.insert_temp(revealed, active));
    }
}

fn analysis_group_header(ui: &mut Ui, label: &str, count: usize) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        vec2(ui.available_width(), ANALYSIS_GROUP_HEADER_HEIGHT),
        Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    ui.painter().text(
        rect.left_center() + vec2(10.0, 0.0),
        Align2::LEFT_CENTER,
        label,
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text_faint,
    );
    ui.painter().text(
        rect.right_center() - vec2(10.0, 0.0),
        Align2::RIGHT_CENTER,
        count.to_string(),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
}

fn analysis_stack_row(
    ui: &mut Ui,
    row: &AnalysisStackRow,
    position: usize,
    selected: bool,
) -> Option<StackAction> {
    let t = Tokens::get(ui.ctx());
    let row_height = ANALYSIS_ROW_HEIGHT;
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), row_height), Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            // The run chip is painted, so it reaches a screen reader only if
            // the row's own name carries it. A status a sighted reader can see
            // and an assistive one cannot is not a status the row has.
            match row.run_state {
                Some(state) => format!(
                    "Select {} analysis instance {}, {} in the last run",
                    row.name,
                    row.id,
                    state.label()
                ),
                None => format!("Select {} analysis instance {}", row.name, row.id),
            },
        )
    });
    let painter = ui.painter().with_clip_rect(rect.intersect(ui.clip_rect()));
    let fill = if selected {
        t.color.accent_dim
    } else if response.hovered() {
        t.color.bg_hover
    } else {
        Color32::TRANSPARENT
    };
    painter.rect_filled(rect, 0.0, fill);
    painter.hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    if selected {
        painter.vline(
            rect.left() + 1.0,
            rect.y_range(),
            Stroke::new(2.0, t.color.accent),
        );
    }

    let opacity = if row.enabled { 1.0 } else { 0.72 };
    let index_center = egui::pos2(
        rect.left() + ANALYSIS_ROW_LEFT_PADDING + ANALYSIS_INDEX_DIAMETER * 0.5,
        rect.top() + 20.0,
    );
    painter.circle_filled(
        index_center,
        ANALYSIS_INDEX_DIAMETER * 0.5,
        if selected {
            t.color.accent_dim
        } else {
            t.color.bg_inset
        },
    );
    painter.circle_stroke(
        index_center,
        ANALYSIS_INDEX_DIAMETER * 0.5,
        Stroke::new(
            1.0,
            if selected {
                t.color.accent
            } else {
                t.color.border
            },
        ),
    );
    painter.text(
        index_center,
        Align2::CENTER_CENTER,
        format!("{position:02}"),
        theme::mono(tokens::FS_0, FontWeight::Medium),
        if selected {
            t.color.accent
        } else {
            t.color.text_faint.gamma_multiply(opacity)
        },
    );

    let switch_hit_size = if t.metrics.ctl_h >= ANALYSIS_SWITCH_MAX_HIT_WIDTH {
        ANALYSIS_SWITCH_MAX_HIT_WIDTH
    } else {
        ANALYSIS_SWITCH_WIDTH
    };
    let switch_hit = Rect::from_center_size(
        egui::pos2(
            rect.right() - ANALYSIS_ROW_TRAILING_PADDING - switch_hit_size * 0.5,
            rect.center().y,
        ),
        Vec2::splat(switch_hit_size),
    );
    let toggle = ui.interact(switch_hit, response.id.with("enabled"), Sense::click());
    toggle.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::Checkbox,
            ui.is_enabled(),
            row.enabled,
            format!("Enable {} instance {}", row.name, row.id),
        )
    });
    paint_switch(ui, switch_hit.center(), row.enabled, toggle.hovered(), rect);

    let text_left = rect.left()
        + ANALYSIS_ROW_LEFT_PADDING
        + ANALYSIS_INDEX_DIAMETER
        + ANALYSIS_INDEX_LABEL_GAP;
    let text_right = switch_hit.left() - ANALYSIS_SWITCH_LABEL_GAP;
    // The code says which kind, the name says which instance. An unnamed
    // instance resolves its name to the kind label, so this row reads exactly
    // as it did before anyone named anything.
    let first_line = format!("{} · {}", row.kind.stable_id().to_uppercase(), row.name);
    let second_line = row.summary.as_str();
    // A blocked dependency is the row's one problem and leads the line with its
    // mark. A production analysis is the default and says nothing about it; any
    // other availability is named, neutrally.
    let blocked = row.issue_count > 0;
    let status = if blocked {
        Some("dependency blocked")
    } else {
        Some(availability_label(row.kind)).filter(|label| *label != "Production")
    };
    let status_line = match status {
        Some(status) => format!("{status} · {}", row.lifecycle),
        None => row.lifecycle.to_string(),
    };
    let line_top = rect.top() + 7.0;
    paint_text_elided_to_fit(
        ui,
        Rect::from_min_max(
            egui::pos2(text_left, line_top),
            egui::pos2(text_right, line_top + 13.0),
        ),
        &first_line,
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text.gamma_multiply(opacity),
    );
    // What this instance costs the queue, at the end of the meta line. The rail
    // is where an operator decides which instances to leave enabled, and the
    // price of that decision was only readable one page away.
    let meta_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let cost_width = row.workload.as_ref().map_or(0.0, |cost| {
        ui.painter()
            .layout_no_wrap(cost.clone(), meta_font.clone(), t.color.text)
            .size()
            .x
            + ANALYSIS_SWITCH_LABEL_GAP
    });
    paint_text_elided_to_fit(
        ui,
        Rect::from_min_max(
            egui::pos2(text_left, line_top + 14.0),
            egui::pos2((text_right - cost_width).max(text_left), line_top + 27.0),
        ),
        second_line,
        meta_font.clone(),
        t.color.text_faint.gamma_multiply(opacity),
    );
    if let Some(cost) = &row.workload {
        paint_text_elided_to_fit(
            ui,
            Rect::from_min_max(
                egui::pos2(
                    (text_right - cost_width + ANALYSIS_SWITCH_LABEL_GAP).max(text_left),
                    line_top + 14.0,
                ),
                egui::pos2(text_right, line_top + 27.0),
            ),
            cost,
            meta_font,
            t.color.text_dim.gamma_multiply(opacity),
        );
    }
    // The run chip sits at the end of the status line rather than in the line,
    // because it is the one fact here that changes without the plan changing.
    // A finished run is the normal state and says nothing; the results are in
    // Results. A failed one carries its mark.
    let run_chip = row.run_state.and_then(|state| match state {
        InstanceRunState::Complete => None,
        InstanceRunState::Failed => Some((state.label(), t.color.text_dim, true)),
        InstanceRunState::Running => Some((state.label(), t.color.text_dim, false)),
        InstanceRunState::Queued => Some((state.label(), t.color.text_faint, false)),
    });
    let chip_font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let chip_width = run_chip.map_or(0.0, |(label, _, failed)| {
        ui.painter()
            .layout_no_wrap(label.to_owned(), chip_font.clone(), t.color.text)
            .size()
            .x
            + ANALYSIS_SWITCH_LABEL_GAP
            + if failed { ROW_MARK_ADVANCE } else { 0.0 }
    });
    let status_top = line_top + 28.0;
    let status_bottom = rect.bottom() - 3.0;
    let status_center = (status_top + status_bottom) * 0.5;
    let status_left = if blocked {
        paint_row_mark(ui, text_left, status_center, Problem::Failure)
    } else {
        text_left
    };
    paint_text_elided_to_fit(
        ui,
        Rect::from_min_max(
            egui::pos2(status_left, status_top),
            egui::pos2((text_right - chip_width).max(status_left), status_bottom),
        ),
        &status_line,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint.gamma_multiply(opacity),
    );
    if let Some((label, color, failed)) = run_chip {
        let chip_left = (text_right - chip_width + ANALYSIS_SWITCH_LABEL_GAP).max(text_left);
        let chip_left = if failed {
            paint_row_mark(ui, chip_left, status_center, Problem::Failure)
        } else {
            chip_left
        };
        paint_text_elided_to_fit(
            ui,
            Rect::from_min_max(
                egui::pos2(chip_left, status_top),
                egui::pos2(text_right.max(chip_left), status_bottom),
            ),
            label,
            chip_font,
            color.gamma_multiply(opacity),
        );
    }
    // The canonical rail hides horizontal overflow. Keep keyboard focus
    // indicators inside the row as well, so they cannot bleed through the
    // one-point pane divider into the analysis editor.
    theme::paint_focus_ring(ui, &response, rect);
    theme::paint_focus_ring(ui, &toggle, switch_hit.intersect(rect));

    if toggle.clicked() {
        Some(StackAction::SetEnabled(row.id, !row.enabled))
    } else if response.clicked() {
        Some(StackAction::Select(row.id))
    } else {
        None
    }
}

/// What the plan's newest run says about one instance.
///
/// Deliberately no `Running` for an analysis that has produced nothing yet.
/// The only identity-backed evidence that a particular instance is executing is
/// the provisional result a live transient writes; everything else the session
/// knows — `is_running`, `progress`, the status line — is about the *run*, and
/// spreading it across every card would tell the reader that thirty analyses
/// are executing at once. A card that says "queued" while its analysis is in
/// fact running understates; a card that says "running" when it is not is a
/// lie, and the chip is worth having only if it never tells one.
#[derive(Clone, Copy, PartialEq, Eq)]
enum InstanceRunState {
    /// A live partial result exists for this instance: it is executing now.
    Running,
    Complete,
    Failed,
    /// The run's authorized queue holds a task for this instance that has not
    /// produced a result.
    Queued,
}

impl InstanceRunState {
    const fn label(self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::Complete => "complete",
            Self::Failed => "failed",
            Self::Queued => "queued",
        }
    }
}

/// Resolve every instance's standing in the plan's newest run, once per frame.
///
/// Keyed by the provenance identity the result carries rather than by the
/// task's position in the receipt. Positional matching desynchronizes the
/// moment a task is skipped for a failed prerequisite, and a chip that shifts
/// one row down is worse than no chip: it attributes a failure to an analysis
/// that never ran.
fn instance_run_states(model: &Model) -> Vec<(AnalysisInstanceId, InstanceRunState)> {
    let Some(PriorPlanRun { index, .. }) = prior_plan_run(model) else {
        return Vec::new();
    };
    let Some(run) = model.simulation.retained.runs.get(index) else {
        return Vec::new();
    };
    let Some(receipt) = run.prepared_receipt() else {
        return Vec::new();
    };
    receipt
        .tasks()
        .iter()
        .map(|task| {
            let id = task.instance_id();
            let state = match run.find_analysis_by_source_instance(id) {
                // A live partial is written `success == false` with a reserved
                // message, so it has to be told from a real failure before the
                // success bit is read at all.
                Some(result) if result.is_live_partial() => InstanceRunState::Running,
                Some(result) if result.success => InstanceRunState::Complete,
                Some(_) => InstanceRunState::Failed,
                None => InstanceRunState::Queued,
            };
            (id, state)
        })
        .collect()
}

pub(super) fn analysis_stack_rows(
    model: &Model,
    participation: &participation::PlanParticipation,
) -> Result<Vec<AnalysisStackRow>, String> {
    let setup = &model.sim_setup;
    let plan = setup.stable_analysis_plan()?;
    let issues = plan.validation_issues();
    let run_states = instance_run_states(model);
    // Resolved once for the whole rail rather than once per row: the projection
    // prices every enabled instance in a single pass, and asking it per row
    // would expand the declared space as many times as there are analyses.
    // Priced against the participation the route already resolved, so the rail
    // costs no expansion of its own.
    let workload = workload::PlanWorkload::resolve_with(model, participation).ok();
    Ok(plan
        .instances()
        .iter()
        .map(|instance| {
            let closure_ids = dependency_closure_ids(plan, instance.id());
            AnalysisStackRow {
                id: instance.id(),
                kind: instance.kind(),
                name: instance.display_name().to_owned(),
                enabled: instance.enabled(),
                lifecycle: instance.lifecycle(),
                run_state: run_states
                    .iter()
                    .find(|(id, _)| *id == instance.id())
                    .map(|(_, state)| *state),
                summary: setup.analysis_draft_summary(instance.draft()),
                workload: workload
                    .as_ref()
                    .and_then(|plan| plan.row_cost_for(instance.id())),
                issue_count: issues
                    .iter()
                    .filter(|issue| {
                        closure_ids
                            .iter()
                            .any(|closure_id| issue_applies_to(issue, *closure_id))
                    })
                    .count(),
            }
        })
        .collect())
}
