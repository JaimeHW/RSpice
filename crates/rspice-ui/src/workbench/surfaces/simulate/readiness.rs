//! What the Analyses page reports about the plan's readiness.
//!
//! The page is two things sharing a scroll area: an editor for one analysis,
//! and the strips underneath that say what the whole plan is currently fit
//! for. This file is the second. The receipt strip reads the lifecycle ledger,
//! the preflight strip reads the last preparation, and the dependency-repair
//! wording turns a graph diagnostic into the one sentence that names the fix.
//!
//! None of it edits anything — every function here takes `&RSpiceApp` — which
//! is the seam: an editor mutates a draft and reports through a receipt, and a
//! strip reads state and paints it. Keeping them in one file is what let a
//! page grow past its budget without anyone deciding it should.

use super::*;
pub(super) fn lifecycle_receipt_strip(ui: &mut Ui, app: &RSpiceApp) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let tombstones = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .map_or(0, |plan| plan.tombstones().len());
    let width = ui.available_width().max(1.0);
    let compact = width <= 760.0;
    let outcome = &app.state.workbench.analysis_lifecycle_status;
    let detail = outcome.message();
    // A refusal is told from a receipt by the outcome's own severity. Reading
    // the wording instead would make the colour depend on how each of the
    // announcing sites happens to phrase itself.
    let detail_color = if outcome.is_refusal() {
        t.color.err
    } else {
        t.color.text_dim
    };
    let title_width = 94.0;
    let tombstone_width = 104.0;
    let detail_width = if compact {
        (width - 18.0).max(1.0)
    } else {
        (width - 18.0 - title_width - tombstone_width - 16.0).max(1.0)
    };
    let detail_galley = ui.painter().layout(
        detail.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        detail_color,
        detail_width,
    );
    // The receipt is a status band, not an expanding log viewer. Its exact
    // height is stable across short edit receipts and verbose insertion or
    // dependency receipts so a form edit cannot change the scroll extent and
    // move every field in a near-bottom viewport. The complete immutable
    // receipt remains available through the hover text below.
    let height = lifecycle_receipt_height(compact);
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    let painter = ui.painter().with_clip_rect(rect.intersect(ui.clip_rect()));
    painter.rect_filled(rect, 0.0, t.color.bg_panel);
    painter.hline(rect.x_range(), rect.top(), Stroke::new(1.0, t.color.border));
    painter.hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let top_center = if compact {
        rect.top() + 14.0
    } else {
        rect.center().y
    };
    painter.text(
        egui::pos2(rect.left() + 9.0, top_center),
        Align2::LEFT_CENTER,
        "Lifecycle receipt",
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
    let tombstone_text = format!(
        "{tombstones} tombstone{}",
        if tombstones == 1 { "" } else { "s" }
    );
    painter.text(
        egui::pos2(rect.right() - 9.0, top_center),
        Align2::RIGHT_CENTER,
        tombstone_text,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    let detail_position = if compact {
        egui::pos2(rect.left() + 9.0, rect.top() + 27.0)
    } else {
        egui::pos2(rect.left() + 9.0 + title_width + 8.0, rect.top() + 6.0)
    };
    painter.galley(detail_position, detail_galley, detail_color);
    response.on_hover_text(detail)
}

const fn lifecycle_receipt_height(compact: bool) -> f32 {
    if compact { 64.0 } else { 40.0 }
}

pub(super) fn preflight_strip(ui: &mut Ui, app: &RSpiceApp) {
    let configured_root = app.state.workspace.simulation_root_reference();
    let configured_schematic = app
        .state
        .workspace
        .simulation_root_schematic(&app.state.workspace.active_view, &app.state.schematic);
    let topology_ok =
        configured_schematic.is_some_and(|schematic| !schematic.components.is_empty());
    let project_revision = app.state.workspace.project.revision().get();
    let (topology_root, topology_revision, topology_closure) =
        app.state.configured_topology_revision();
    let current_plan = app
        .state
        .sim_setup
        .analysis_plan
        .as_ref()
        .map(|plan| (plan.id(), plan.revision()));
    let retained_report = app
        .state
        .workbench
        .preflight
        .report
        .as_ref()
        .filter(|report| {
            report.is_current_for(
                project_revision,
                &topology_root,
                topology_revision,
                &topology_closure,
                current_plan,
            )
        });
    let netlist_state = retained_report.map(|report| {
        topology_ok
            && !report
                .blockers
                .iter()
                .any(|issue| issue.remediation.blocks_executable_netlist())
    });
    let (enabled_count, configurations_ok, graph_ok) =
        match app.state.sim_setup.stable_analysis_plan() {
            Ok(plan) => {
                let enabled = plan
                    .instances()
                    .iter()
                    .filter(|instance| instance.enabled())
                    .collect::<Vec<_>>();
                (
                    enabled.len(),
                    enabled.iter().all(|instance| {
                        app.state
                            .sim_setup
                            .analysis_draft_validation_error(instance.draft())
                            .is_none()
                    }),
                    plan.validation_issues().is_empty(),
                )
            }
            Err(_) => (0, false, false),
        };
    let active_payload =
        current_plan.and_then(|(plan_id, _)| app.state.workspace.plan_data(plan_id));
    let governed_specifications = active_payload
        .map(|payload| payload.specification_definitions.as_slice())
        .unwrap_or_default();
    let projected_specifications = active_payload
        .map(|payload| payload.specs.as_slice())
        .unwrap_or(&app.state.workspace.specs);
    let specifications_configured = !projected_specifications.is_empty();
    let specifications_ok = specifications_configured
        && if governed_specifications.is_empty() {
            projected_specifications
                .iter()
                .all(|specification| specification.validate().is_ok())
        } else {
            governed_specifications.len() == projected_specifications.len()
                && governed_specifications
                    .iter()
                    .zip(projected_specifications)
                    .all(|(definition, projection)| {
                        definition.validate().is_ok() && definition.projected_entry().eq(projection)
                    })
                && active_payload
                    .is_some_and(|payload| payload.specification_policy.validate().is_ok())
        };
    let items = [
        (
            netlist_state,
            "Netlist",
            if configured_schematic.is_none() {
                format!(
                    "configured root {} is unresolved",
                    configured_root.display_path()
                )
            } else if !topology_ok {
                "configured design is empty".to_owned()
            } else if retained_report.is_none() {
                if app.state.workbench.preflight.report.is_some() {
                    "preflight receipt expired".to_owned()
                } else {
                    "preflight not run".to_owned()
                }
            } else if netlist_state == Some(false) {
                "preflight has design blockers".to_owned()
            } else {
                format!("revision {topology_revision} current")
            },
        ),
        (
            Some(graph_ok),
            "Instance graph",
            if graph_ok {
                format!("dependency ordered · {enabled_count} enabled")
            } else {
                "resolve lifecycle diagnostics".to_owned()
            },
        ),
        (
            specifications_configured.then_some(specifications_ok),
            "Specifications",
            if !specifications_configured {
                "not configured · optional".to_owned()
            } else if specifications_ok {
                format!(
                    "{} governed requirements valid",
                    projected_specifications.len()
                )
            } else {
                "invalid specification bounds".to_owned()
            },
        ),
        (
            retained_report.map(|report| {
                report.is_runnable_for(
                    project_revision,
                    &topology_root,
                    topology_revision,
                    &topology_closure,
                    current_plan,
                )
            }),
            "Execution graph",
            if enabled_count == 0 {
                "enable an analysis instance".to_owned()
            } else if !configurations_ok {
                "correct invalid fields".to_owned()
            } else if !graph_ok {
                "dependency graph blocked".to_owned()
            } else if retained_report.is_none() {
                if app.state.workbench.preflight.report.is_some() {
                    "rerun expired preflight".to_owned()
                } else {
                    "run preflight to authorize dispatch".to_owned()
                }
            } else if retained_report.is_some_and(|report| {
                !report.is_runnable_for(
                    project_revision,
                    &topology_root,
                    topology_revision,
                    &topology_closure,
                    current_plan,
                )
            }) {
                "resolve retained preflight blockers".to_owned()
            } else {
                let task_count = retained_report
                    .and_then(|report| report.prepared.as_ref())
                    .map_or(enabled_count, |prepared| prepared.task_count);
                // The authorized queue, priced the way the Run Set page prices
                // the forecast it was planned against. This is the last cell
                // before a dispatch, and a task count is the one number an
                // operator cannot convert into a decision on its own.
                format!(
                    "{task_count} analysis tasks ready · {}",
                    super::workload::modelled_duration(&app.state, task_count)
                )
            },
        ),
    ];

    let t = Tokens::get(ui.ctx());
    let compact = ui.available_width() <= 760.0;
    let columns = if compact { 2 } else { 4 };
    let rows = items.len().div_ceil(columns);
    let cell_width = ui.available_width() / columns as f32;
    let text_width = (cell_width - 36.0).max(1.0);
    let row_height = items
        .iter()
        .map(|item| {
            let detail = ui.painter().layout(
                item.2.clone(),
                theme::sans(tokens::FS_0, FontWeight::Regular),
                t.color.text_dim,
                text_width,
            );
            (26.0 + detail.size().y).max(PREFLIGHT_CELL_HEIGHT)
        })
        .fold(PREFLIGHT_CELL_HEIGHT, f32::max);
    let size = vec2(ui.available_width(), row_height * rows as f32);
    let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    let cell_width = rect.width() / columns as f32;
    for (index, item) in items.iter().enumerate() {
        let column = index % columns;
        let row = index / columns;
        let cell = Rect::from_min_size(
            rect.min + vec2(cell_width * column as f32, row_height * row as f32),
            vec2(cell_width, row_height),
        );
        preflight_cell(
            ui,
            cell,
            item.0,
            item.1,
            &item.2,
            column + 1 < columns,
            row + 1 < rows,
        );
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
}

pub(super) fn preflight_cell(
    ui: &mut Ui,
    rect: Rect,
    pass: Option<bool>,
    name: &str,
    detail: &str,
    draw_right_border: bool,
    draw_bottom_border: bool,
) {
    let t = Tokens::get(ui.ctx());
    let status = match pass {
        Some(true) => "passed",
        Some(false) => "blocked",
        None => "not available",
    };
    let response = ui.interact(rect, ui.id().with(("preflight-cell", name)), Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            format!("{name}: {status}. {detail}"),
        )
    });
    let painter = ui.painter().with_clip_rect(rect.intersect(ui.clip_rect()));
    if draw_right_border {
        painter.vline(
            rect.right(),
            rect.y_range(),
            Stroke::new(1.0, t.color.border),
        );
    }
    if draw_bottom_border {
        painter.hline(
            rect.x_range(),
            rect.bottom(),
            Stroke::new(1.0, t.color.border),
        );
    }
    let (mark, mark_color) = match pass {
        Some(true) => (StatusMark::Success, t.color.ok),
        Some(false) => (StatusMark::Warning, t.color.err),
        None => (StatusMark::Neutral, t.color.text_faint),
    };
    paint_status_mark(
        &painter,
        Rect::from_center_size(rect.left_center() + vec2(14.0, 0.0), Vec2::splat(11.0)),
        mark,
        mark_color,
    );
    let text_left = rect.left() + 28.0;
    paint_clipped_text(
        ui,
        Rect::from_min_max(
            egui::pos2(text_left, rect.top() + 5.0),
            egui::pos2(rect.right() - 8.0, rect.center().y + 1.0),
        ),
        name,
        theme::sans(tokens::FS_0, FontWeight::Medium),
        t.color.text,
    );
    let detail_galley = ui.painter().layout(
        detail.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
        (rect.right() - 8.0 - text_left).max(1.0),
    );
    painter.galley(
        egui::pos2(text_left, rect.top() + 21.0),
        detail_galley,
        t.color.text_dim,
    );
}

#[cfg(test)]
pub(super) fn dependency_repair_cta(
    plan: &crate::simulation::plan::SimulationPlan,
    issues: &[AnalysisPlanIssue],
    dependencies: &[AnalysisDependency],
) -> Option<String> {
    let context = AnalysisDependencyRepairContext::exact_periodic_sources(
        "periodic fixture\nVIN_DIFF in 0 SIN(0 1 1k)\nR1 in 0 1k\n.end\n",
    )
    .expect("test periodic-source fixture is exact");
    dependency_repair_cta_with_context(plan, issues, dependencies, &context)
}

pub(super) fn compatible_dependency_repair_label(
    plan: &crate::simulation::plan::SimulationPlan,
    dependent: AnalysisInstanceId,
    prerequisite: AnalysisKind,
    repair_context: &AnalysisDependencyRepairContext,
) -> String {
    let qualifier = if prerequisite == AnalysisKind::Transient
        && plan
            .instance(dependent)
            .is_some_and(|instance| instance.kind() == AnalysisKind::Fourier)
    {
        "compatible "
    } else {
        ""
    };
    let Some(position) = plan
        .instances()
        .iter()
        .position(|instance| instance.id() == dependent)
    else {
        return format!("Repair {} prerequisite", prerequisite.label());
    };
    let before = &plan.instances()[..position];
    let after = &plan.instances()[position + 1..];
    if before.iter().rev().any(|candidate| {
        candidate.enabled()
            && plan.dependency_candidate_is_compatible_with_context(
                dependent,
                prerequisite,
                candidate.id(),
                repair_context,
            )
    }) {
        format!("Bind {qualifier}{}", prerequisite.label())
    } else if before.iter().rev().any(|candidate| {
        plan.dependency_candidate_is_compatible_with_context(
            dependent,
            prerequisite,
            candidate.id(),
            repair_context,
        )
    }) {
        format!("Enable {qualifier}{}", prerequisite.label())
    } else if after.iter().any(|candidate| {
        candidate.enabled()
            && plan.dependency_candidate_is_compatible_with_context(
                dependent,
                prerequisite,
                candidate.id(),
                repair_context,
            )
    }) {
        format!("Move {qualifier}{} earlier", prerequisite.label())
    } else if after.iter().any(|candidate| {
        plan.dependency_candidate_is_compatible_with_context(
            dependent,
            prerequisite,
            candidate.id(),
            repair_context,
        )
    }) {
        format!(
            "Enable and move {qualifier}{} earlier",
            prerequisite.label()
        )
    } else {
        format!("Add {qualifier}{}", prerequisite.label())
    }
}

pub(super) fn dependency_closure_ids(
    plan: &crate::simulation::plan::SimulationPlan,
    root: AnalysisInstanceId,
) -> HashSet<AnalysisInstanceId> {
    let mut closure = HashSet::new();
    if plan
        .instance(root)
        .is_some_and(|instance| !instance.enabled())
    {
        closure.insert(root);
        return closure;
    }
    let mut pending = vec![root];
    while let Some(id) = pending.pop() {
        if !closure.insert(id) {
            continue;
        }
        if let Some(instance) = plan.instance(id) {
            pending.extend(instance.dependencies().iter().filter_map(|dependency| {
                let target = dependency.target();
                let role_is_unique = instance
                    .dependencies()
                    .iter()
                    .filter(|candidate| candidate.prerequisite() == dependency.prerequisite())
                    .count()
                    == 1;
                if target == id
                    || !role_is_unique
                    || !instance
                        .prerequisite_roles()
                        .contains(&dependency.prerequisite())
                {
                    return None;
                }
                plan.instance(target)
                    .filter(|candidate| candidate.kind() == dependency.prerequisite())
                    .map(|candidate| candidate.id())
            }));
        }
    }
    closure
}

pub(super) fn format_plan_issue(issue: &AnalysisPlanIssue) -> String {
    match issue {
        AnalysisPlanIssue::NoEnabledInstances => {
            "The simulation plan has no enabled analysis instances.".to_owned()
        }
        AnalysisPlanIssue::DuplicateInstanceId { id } => {
            format!("Stable analysis identity {id} is duplicated.")
        }
        AnalysisPlanIssue::DuplicateTombstoneId { id } => {
            format!("Retired analysis identity {id} has duplicate tombstones.")
        }
        AnalysisPlanIssue::ReusedTombstonedId { id } => {
            format!("Retired analysis identity {id} was reused by an active instance.")
        }
        AnalysisPlanIssue::KindDraftMismatch {
            id,
            expected,
            actual,
        } => format!(
            "Analysis {id} requires a {} draft, but contains {}.",
            expected.label(),
            actual.label()
        ),
        AnalysisPlanIssue::InvalidInstanceRevision { id } => {
            format!("Analysis {id} has an invalid revision range.")
        }
        AnalysisPlanIssue::InvalidInstanceName { id } => {
            format!("Analysis {id} carries a name this plan cannot show.")
        }
        AnalysisPlanIssue::DuplicateInstanceName { id, name } => {
            format!("Analysis {id} shares the name \"{name}\" with another analysis.")
        }
        AnalysisPlanIssue::InvalidLifecycle { id, state, enabled } => {
            format!("Analysis {id} lifecycle {state} conflicts with enabled state {enabled}.")
        }
        AnalysisPlanIssue::MissingPrerequisite {
            dependent,
            prerequisite,
        } => format!(
            "Analysis {dependent} requires an earlier enabled {} instance.",
            prerequisite.label()
        ),
        AnalysisPlanIssue::UnexpectedDependencyRole {
            dependent,
            prerequisite,
        } => format!(
            "Analysis {dependent} does not accept {} as a prerequisite.",
            prerequisite.label()
        ),
        AnalysisPlanIssue::DuplicateDependencyRole {
            dependent,
            prerequisite,
        } => format!(
            "Analysis {dependent} binds {} more than once.",
            prerequisite.label()
        ),
        AnalysisPlanIssue::SelfDependency { dependent } => {
            format!("Analysis {dependent} cannot depend on itself.")
        }
        AnalysisPlanIssue::DanglingDependency { dependent, target } => {
            format!("Analysis {dependent} references missing prerequisite instance {target}.")
        }
        AnalysisPlanIssue::WrongDependencyKind {
            dependent,
            prerequisite,
            target,
            actual,
        } => format!(
            "Analysis {dependent} requires {} at {target}, but that instance is {}.",
            prerequisite.label(),
            actual.label()
        ),
        AnalysisPlanIssue::DisabledDependency { dependent, target } => {
            format!("Analysis {dependent} prerequisite instance {target} is disabled.")
        }
        AnalysisPlanIssue::DependencyNotEarlier { dependent, target } => {
            format!("Analysis {dependent} prerequisite instance {target} must appear earlier.")
        }
        AnalysisPlanIssue::IncompatibleDependencyConfiguration {
            dependent,
            prerequisite,
            target,
            detail,
        } => format!(
            "Analysis {dependent} cannot use {} instance {target}: {detail}.",
            prerequisite.label()
        ),
        AnalysisPlanIssue::DependencyCycle { members } => format!(
            "Analysis dependency cycle contains {} instance{}.",
            members.len(),
            if members.len() == 1 { "" } else { "s" }
        ),
        AnalysisPlanIssue::InvalidTombstoneRevision { id } => {
            format!("Retired analysis identity {id} has an invalid revision range.")
        }
        AnalysisPlanIssue::InvalidReceiptSequence { sequence } => {
            format!("Lifecycle receipt sequence {sequence} is not contiguous.")
        }
        AnalysisPlanIssue::InvalidReceiptRevision { sequence } => {
            format!("Lifecycle receipt {sequence} has an invalid revision transition.")
        }
        AnalysisPlanIssue::DanglingReceiptInstance { sequence, id } => {
            format!("Lifecycle receipt {sequence} references unknown analysis instance {id}.")
        }
        AnalysisPlanIssue::ReceiptKindMismatch {
            sequence,
            expected,
            actual,
        } => format!(
            "Lifecycle receipt {sequence} identifies {}, but its retained analysis is {}.",
            actual.label(),
            expected.label()
        ),
        AnalysisPlanIssue::EmptyReceiptDetail { sequence } => {
            format!("Lifecycle receipt {sequence} has no status detail.")
        }
        AnalysisPlanIssue::InvalidNextReceiptSequence { expected, actual } => {
            format!("Next lifecycle receipt sequence is {actual}; expected {expected}.")
        }
    }
}
