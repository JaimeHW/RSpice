//! The setup cards beside the analysis form: design variables and outputs.
//!
//! These are plan-scoped rather than analysis-scoped — a variable or saved
//! output belongs to the whole plan, so the cards read from the plan and not
//! from whichever analysis happens to be selected. They present rows and
//! delegate every edit to the plan workflows rather than mutating here.

use super::*;

pub(super) fn setup_tables(ui: &mut Ui, app: &mut RSpiceApp) {
    let plan_id = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .map(|plan| plan.id())
        .ok();
    let payload = plan_id
        .and_then(|plan_id| app.state.workspace.active_plan_data(plan_id).cloned())
        .unwrap_or_default();
    let design_variables = payload.design_variables;
    let saved_outputs = payload.saved_outputs;
    let saved_output_reports = app
        .simulation_controller
        .saved_outputs_preflight(&app.state, &saved_outputs);
    let specifications = if plan_id.is_some() {
        payload.specs
    } else {
        app.state.workspace.specs.clone()
    };
    let dataset = selected_output_dataset(&app.state.simulation).cloned();
    let row_height = Tokens::get(ui.ctx()).metrics.row_h;
    let variable_rows = design_variables.len().max(1);
    let output_rows = (saved_outputs.len() + specifications.len()).max(1);
    let variable_card_height = setup_card_height(row_height, variable_rows);
    let output_card_height = setup_card_height(row_height, output_rows);
    let split = ui.available_width() > SIMULATION_STACK_BREAKPOINT;
    let mut add_variable = false;
    let mut add_output = false;

    if split {
        let card_height = variable_card_height.max(output_card_height);
        let divider = 1.0;
        let usable = (ui.available_width() - divider).max(1.0);
        let left_width = usable * 0.46;
        let right_width = usable - left_width;
        let row = ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = divider;
            ui.allocate_ui_with_layout(
                vec2(left_width, card_height),
                Layout::top_down(Align::Min),
                |ui| {
                    add_variable = design_variables_card(
                        ui,
                        &design_variables,
                        card_height,
                        SetupCardBorder::SplitLeft,
                    );
                },
            );
            ui.allocate_ui_with_layout(
                vec2(right_width, card_height),
                Layout::top_down(Align::Min),
                |ui| {
                    add_output = outputs_specifications_card(
                        ui,
                        &saved_outputs,
                        &saved_output_reports,
                        &specifications,
                        dataset.as_ref(),
                        card_height,
                        SetupCardBorder::SplitRight,
                    );
                },
            );
        });
        ui.painter().vline(
            row.response.rect.left() + left_width + divider * 0.5,
            row.response.rect.y_range(),
            Stroke::new(1.0, Tokens::get(ui.ctx()).color.border),
        );
    } else {
        add_variable = design_variables_card(
            ui,
            &design_variables,
            variable_card_height,
            SetupCardBorder::All,
        );
        add_output = outputs_specifications_card(
            ui,
            &saved_outputs,
            &saved_output_reports,
            &specifications,
            dataset.as_ref(),
            output_card_height,
            SetupCardBorder::All,
        );
    }

    if add_variable {
        app.state.workbench.simulation_workflow = Some(SimulationWorkflowDialog::DesignVariable(
            DesignVariableDraft::default(),
        ));
    } else if add_output {
        app.state.workbench.simulation_workflow = Some(SimulationWorkflowDialog::SavedOutput(
            SavedOutputDraft::default(),
        ));
    }
}

pub(super) fn setup_card_height(row_height: f32, rows: usize) -> f32 {
    SETUP_CARD_HEADER_HEIGHT + SETUP_TABLE_HEADER_HEIGHT + row_height * rows.max(1) as f32
}

pub(super) fn design_variables_card(
    ui: &mut Ui,
    variables: &[crate::state::DesignVariable],
    height: f32,
    border: SetupCardBorder,
) -> bool {
    setup_table_card(ui, "design-variables", height, border, |ui| {
        let add = setup_card_header(ui, "Design variables", "Add design variable");
        setup_table_row(
            ui,
            [0.36, 0.32, 0.32],
            ["NAME", "VALUE", "SCOPE"],
            true,
            [None, None, None],
        );
        if variables.is_empty() {
            setup_table_row(
                ui,
                [0.36, 0.32, 0.32],
                ["No design variables configured", "—", "optional"],
                false,
                [None, None, None],
            );
        } else {
            for variable in variables {
                setup_table_row(
                    ui,
                    [0.36, 0.32, 0.32],
                    [
                        variable.name.as_str(),
                        variable.expression.as_str(),
                        variable.scope.label(),
                    ],
                    false,
                    [None, None, None],
                );
            }
        }
        add
    })
}

pub(super) fn outputs_specifications_card(
    ui: &mut Ui,
    outputs: &[crate::state::SavedOutput],
    output_reports: &[crate::simulation::SavedOutputPreflightReport],
    specifications: &[crate::state::SpecEntry],
    dataset: Option<&crate::state::SimulationRun>,
    height: f32,
    border: SetupCardBorder,
) -> bool {
    setup_table_card(ui, "outputs-specifications", height, border, |ui| {
        let add = setup_card_header(ui, "Outputs & specifications", "Add saved output");
        setup_table_row(
            ui,
            [0.42, 0.29, 0.29],
            ["EXPRESSION", "SPEC", "STATUS"],
            true,
            [None, None, None],
        );
        if outputs.is_empty() && specifications.is_empty() {
            setup_table_row(
                ui,
                [0.42, 0.29, 0.29],
                [
                    "No saved outputs or specifications configured",
                    "—",
                    "optional",
                ],
                false,
                [None, None, None],
            );
        } else {
            let t = Tokens::get(ui.ctx());
            for (index, output) in outputs.iter().enumerate() {
                let report = output_reports.get(index);
                let (status, color) = match report.map(|report| report.semantic_status()) {
                    Some(SavedOutputSemanticStatus::Valid { .. }) => ("valid", t.color.ok),
                    Some(SavedOutputSemanticStatus::RuntimeBound { .. }) => {
                        ("runtime-bound", t.color.warn)
                    }
                    Some(SavedOutputSemanticStatus::Invalid { reason }) => {
                        (reason.as_str(), t.color.err)
                    }
                    None => ("preflight report unavailable", t.color.err),
                };
                setup_table_row(
                    ui,
                    [0.42, 0.29, 0.29],
                    [
                        output.source_expression.as_str(),
                        output.status_label(),
                        status,
                    ],
                    false,
                    [None, None, Some(color)],
                );
            }
            for spec in specifications {
                let limit = specification_limit(spec);
                let evidence =
                    dataset.and_then(|run| measurement_in_output_dataset(run, &spec.measurement));
                let (status, color) = match evidence {
                    Some(evidence) if !evidence.measurement_passed => {
                        ("measurement failed", t.color.err)
                    }
                    Some(evidence) if spec.passes(evidence.value) => ("pass", t.color.ok),
                    Some(_) => ("fail", t.color.err),
                    None => ("no evidence", t.color.warn),
                };
                setup_table_row(
                    ui,
                    [0.42, 0.29, 0.29],
                    [spec.measurement.as_str(), limit.as_str(), status],
                    false,
                    [None, None, Some(color)],
                );
            }
        }
        add
    })
}

pub(super) fn setup_table_card(
    ui: &mut Ui,
    card_id: &'static str,
    height: f32,
    border: SetupCardBorder,
    body: impl FnOnce(&mut Ui) -> bool,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let response = egui::Frame::new().fill(t.color.bg_app).show(ui, |ui| {
        ui.set_width(width);
        ui.set_min_height(height);
        ui.spacing_mut().item_spacing.y = 0.0;
        // The mockup keeps all three columns in the card at narrow widths.
        // A fixed-width nested ScrollArea both hid the trailing column and
        // allowed sibling cards to alias egui scroll state; proportional rows
        // already provide the intended responsive behavior without state.
        // Keep every stateful descendant in a stable card-local namespace as
        // well. This prevents sibling setup cards from ever sharing widget or
        // scroll state if either card gains another interactive child later.
        ui.push_id(("simulation-setup-card", card_id), body).inner
    });
    let rect = response.response.rect;
    match border {
        SetupCardBorder::All => {
            ui.painter().rect_stroke(
                rect,
                0.0,
                Stroke::new(1.0, t.color.border),
                egui::StrokeKind::Inside,
            );
        }
        SetupCardBorder::SplitLeft | SetupCardBorder::SplitRight => {
            ui.painter().hline(
                rect.x_range(),
                rect.top() + 0.5,
                Stroke::new(1.0, t.color.border),
            );
            ui.painter().hline(
                rect.x_range(),
                rect.bottom() - 0.5,
                Stroke::new(1.0, t.color.border),
            );
            let x = if border == SetupCardBorder::SplitLeft {
                rect.left() + 0.5
            } else {
                rect.right() - 0.5
            };
            ui.painter()
                .vline(x, rect.y_range(), Stroke::new(1.0, t.color.border));
        }
    };
    response.inner
}

pub(super) fn setup_card_header(ui: &mut Ui, title: &str, add_tooltip: &str) -> bool {
    let t = Tokens::get(ui.ctx());
    let (header, _) = ui.allocate_exact_size(
        vec2(ui.available_width(), SETUP_CARD_HEADER_HEIGHT),
        Sense::hover(),
    );
    ui.painter().hline(
        header.x_range(),
        header.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    paint_clipped_text(
        ui,
        Rect::from_min_max(
            header.left_top() + vec2(11.0, 0.0),
            egui::pos2(header.right() - 39.0, header.bottom()),
        ),
        title,
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
    let add_rect = Rect::from_center_size(
        egui::pos2(header.right() - 17.5, header.center().y),
        vec2(28.0, 27.0),
    );
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(add_rect)
            .layout(Layout::top_down(Align::Min)),
    );
    IconButton::new(Icon::Add)
        .size(28.0, 27.0)
        .tooltip(add_tooltip)
        .show(&mut child)
        .clicked()
}

pub(super) fn setup_table_row(
    ui: &mut Ui,
    fractions: [f32; 3],
    cells: [&str; 3],
    header: bool,
    cell_colors: [Option<Color32>; 3],
) {
    let t = Tokens::get(ui.ctx());
    let height = if header {
        SETUP_TABLE_HEADER_HEIGHT
    } else {
        t.metrics.row_h
    };
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::hover());
    if header {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let rects = setup_table_column_rects(rect, fractions);
    let first = rects[0].right();
    let second = rects[1].right();
    for x in [first, second] {
        ui.painter()
            .vline(x, rect.y_range(), Stroke::new(1.0, t.color.border));
    }
    let font = if header {
        theme::sans(tokens::FS_0, FontWeight::Medium)
    } else {
        theme::mono(tokens::FS_0, FontWeight::Regular)
    };
    let default_color = if header {
        t.color.text_faint
    } else {
        t.color.text_dim
    };
    for index in 0..3 {
        paint_clipped_text(
            ui,
            rects[index].shrink2(vec2(8.0, 0.0)),
            cells[index],
            font.clone(),
            cell_colors[index].unwrap_or(default_color),
        );
    }
}

pub(super) fn setup_table_column_rects(rect: Rect, fractions: [f32; 3]) -> [Rect; 3] {
    let first = rect.left() + rect.width() * fractions[0];
    let second = first + rect.width() * fractions[1];
    [
        Rect::from_min_max(rect.min, egui::pos2(first, rect.bottom())),
        Rect::from_min_max(
            egui::pos2(first, rect.top()),
            egui::pos2(second, rect.bottom()),
        ),
        Rect::from_min_max(egui::pos2(second, rect.top()), rect.max),
    ]
}

pub(super) fn specification_limit(spec: &crate::state::SpecEntry) -> String {
    match (spec.min, spec.max) {
        (Some(minimum), Some(maximum)) => {
            format!("{minimum:.6}…{maximum:.6} {}", spec.unit)
        }
        (Some(minimum), None) => format!("≥ {minimum:.6} {}", spec.unit),
        (None, Some(maximum)) => format!("≤ {maximum:.6} {}", spec.unit),
        (None, None) => "waveform".to_owned(),
    }
}

pub(super) fn selected_output_dataset(
    simulation: &crate::state::SimulationState,
) -> Option<&crate::state::SimulationRun> {
    simulation.active_run()
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct OutputMeasurementEvidence {
    pub(super) value: f64,
    pub(super) measurement_passed: bool,
}

pub(super) fn measurement_in_output_dataset(
    run: &crate::state::SimulationRun,
    name: &str,
) -> Option<OutputMeasurementEvidence> {
    run.analyses
        .iter()
        .rev()
        .filter(|analysis| analysis.success && analysis.provenance.is_some())
        .find_map(|analysis| {
            analysis
                .measurements
                .iter()
                .rev()
                .find(|measurement| measurement.name.eq_ignore_ascii_case(name))
                .and_then(|measurement| {
                    measurement
                        .value
                        .filter(|value| value.is_finite())
                        .map(|value| OutputMeasurementEvidence {
                            value,
                            measurement_passed: measurement.passed && measurement.error.is_none(),
                        })
                })
        })
}
