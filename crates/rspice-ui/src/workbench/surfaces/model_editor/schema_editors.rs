//! Editing a model's schema: parameters, sections, correlations, temperature.
//!
//! Every editor here is gated on the document being writable and shows the
//! same fields read-only when it is not, rather than hiding them — a reviewer
//! looking at a locked model sees exactly what an author would edit. Bounds
//! and schema entries are parsed as they are typed so an invalid value is
//! rejected at the field, never carried into the draft.

use super::*;

pub(super) fn parameter_schema_dialog(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    _records: &PersistedModelRecords,
) {
    if !app.state.workbench.model_editor.parameter_schema_open {
        return;
    }
    let writable = !app.state.workbench.safe_mode.project_read_only();
    let choice = Dialog::new(
        "MODEL · TYPED PARAMETER SCHEMA",
        "Parameter schema",
        "Close",
    )
    .description("Edit typed declarations and exact process-section overrides. Changes remain transactional until Save model revision succeeds.")
    .size(DialogSize::Transaction)
    .fixed_height(680.0)
    .show(ctx, |ui| {
        section_override_editor(ui, app, writable);
        parameter_schema_editor(ui, app, writable);
    });
    if matches!(choice, DialogChoice::Primary | DialogChoice::Cancelled) {
        app.state.workbench.model_editor.parameter_schema_open = false;
    }
}

pub(super) fn section_override_editor(ui: &mut Ui, app: &mut RSpiceApp, writable: bool) {
    let (sections, parameters) = app.state.workbench.model_editor.draft.as_ref().map_or_else(
        || (Vec::new(), Vec::new()),
        |draft| {
            (
                draft
                    .metadata
                    .sections
                    .iter()
                    .map(|section| section.name.clone())
                    .collect::<Vec<_>>(),
                draft
                    .metadata
                    .parameters
                    .iter()
                    .map(|parameter| parameter.name.clone())
                    .collect::<Vec<_>>(),
            )
        },
    );
    dialog_subheading(ui, "Process-section value override");
    if sections.is_empty() || parameters.is_empty() {
        promotion_note(
            ui,
            None,
            if sections.is_empty() {
                "Create a named process section before authoring section-specific parameter values."
            } else {
                "The model declares no typed parameters that can be overridden."
            },
        );
        return;
    }

    let [section_rect, parameter_rect, value_rect] = qualification_three_rects(ui);
    let mut selection_changed = false;
    {
        let editor = &mut app.state.workbench.model_editor;
        selection_changed |= section_override_combo(
            ui,
            section_rect,
            "Process section",
            "model-parameter-schema-section",
            &mut editor.parameter_schema_section,
            &sections,
            writable,
        );
        selection_changed |= section_override_combo(
            ui,
            parameter_rect,
            "Parameter",
            "model-parameter-schema-parameter",
            &mut editor.parameter_schema_parameter,
            &parameters,
            writable,
        );
    }
    if selection_changed {
        app.state
            .workbench
            .model_editor
            .refresh_parameter_schema_override_editor();
    }
    qualification_text_field(
        ui,
        value_rect,
        "Section value",
        &mut app
            .state
            .workbench
            .model_editor
            .parameter_schema_override_value,
        true,
        writable,
    );

    let override_exists = app
        .state
        .workbench
        .model_editor
        .parameter_schema_override_exists();
    let (actions_rect, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width().max(1.0), 38.0),
        Sense::hover(),
    );
    let mut actions = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(actions_rect.shrink2(Vec2::new(0.0, 4.0)))
            .layout(Layout::left_to_right(Align::Center)),
    );
    actions.spacing_mut().item_spacing.x = 6.0;
    if Button::new(if override_exists {
        "Update override"
    } else {
        "Add override"
    })
    .accent()
    .enabled(writable)
    .show(&mut actions)
    .clicked()
    {
        app.state
            .workbench
            .model_editor
            .commit_parameter_schema_override();
    }
    if Button::new("Remove override")
        .enabled(writable && override_exists)
        .show(&mut actions)
        .on_disabled_hover_text(if override_exists {
            "Section overrides cannot be changed while the project is read-only."
        } else {
            "The selected parameter inherits its value and has no override to remove."
        })
        .clicked()
    {
        app.state
            .workbench
            .model_editor
            .remove_parameter_schema_override();
    }
    let error = app
        .state
        .workbench
        .model_editor
        .parameter_schema_override_error
        .clone();
    promotion_note(
        ui,
        error.as_deref(),
        if override_exists {
            "The selected value is an explicit section delta. Removing it restores inherited resolution through the declared parent graph."
        } else {
            "The selected value currently resolves through the section's parent graph; Add override creates one explicit typed delta."
        },
    );
}

pub(super) fn section_override_combo(
    ui: &mut Ui,
    rect: Rect,
    label: &str,
    salt: &str,
    value: &mut String,
    options: &[String],
    enabled: bool,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let label_rect = Rect::from_min_max(rect.min, Pos2::new(rect.right(), rect.top() + 16.0));
    paint_elided(
        ui,
        label_rect,
        label,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
        0.0,
    );
    let field_rect = Rect::from_min_max(
        Pos2::new(rect.left(), rect.top() + 20.0),
        Pos2::new(rect.right(), rect.bottom()),
    );
    let mut field = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(field_rect)
            .layout(Layout::left_to_right(Align::Center)),
    );
    let before = value.clone();
    let response = field
        .add_enabled_ui(enabled, |ui| {
            egui::ComboBox::from_id_salt(salt)
                .selected_text(value.as_str())
                .width(field_rect.width())
                .show_ui(ui, |ui| {
                    for option in options {
                        ui.selectable_value(value, option.clone(), option);
                    }
                })
        })
        .inner;
    ui.ctx()
        .accesskit_node_builder(response.response.id, |node| {
            node.set_label(label);
        });
    *value != before
}

pub(super) fn parameter_schema_editor(ui: &mut Ui, app: &mut RSpiceApp, writable: bool) {
    const COLUMNS: [(&str, f32); 5] = [
        ("Parameter", 0.16),
        ("Type", 0.14),
        ("Unit", 0.13),
        ("Bounds", 0.23),
        ("Description", 0.34),
    ];
    let mut changed = false;
    ScrollArea::horizontal()
        .id_salt("model-editor-schema-dialog")
        .auto_shrink([false, true])
        .show(ui, |ui| {
            let width = ui.available_width().max(TABLE_MIN_W);
            table_header(ui, width, &COLUMNS);
            let Some(draft) = app.state.workbench.model_editor.draft.as_mut() else {
                table_empty(ui, width, "No open model parameter schema.");
                return;
            };
            if draft.parameters.is_empty() {
                table_empty(ui, width, "The candidate declares no parameters.");
                return;
            }
            for (index, parameter) in draft.parameters.iter_mut().enumerate() {
                let cells = interactive_table_row(ui, width, &COLUMNS, false);
                paint_cell(ui, cells[0], &DisplayCell::mono(&parameter.name));
                let type_rect = cells[1].shrink2(Vec2::new(4.0, 3.0));
                let mut type_ui = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(type_rect)
                        .layout(Layout::left_to_right(Align::Center)),
                );
                let before_kind = parameter.kind;
                let type_response = type_ui
                    .add_enabled_ui(writable, |ui| {
                        egui::ComboBox::from_id_salt(("model-parameter-kind", index))
                            .selected_text(match parameter.kind {
                                ModelParameterKind::Numeric => "numeric",
                                ModelParameterKind::String => "string",
                            })
                            .width((type_rect.width() - 24.0).max(1.0))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut parameter.kind,
                                    ModelParameterKind::Numeric,
                                    "numeric",
                                );
                                ui.selectable_value(
                                    &mut parameter.kind,
                                    ModelParameterKind::String,
                                    "string",
                                );
                            })
                    })
                    .inner;
                ui.ctx()
                    .accesskit_node_builder(type_response.response.id, |node| {
                        node.set_label(format!("Type for {}", parameter.name));
                    });
                changed |= parameter.kind != before_kind;
                changed |= schema_text_edit_enabled(
                    ui,
                    cells[2],
                    &mut parameter.unit,
                    &format!("Unit for {}", parameter.name),
                    writable,
                );
                changed |= bounds_edit_enabled(
                    ui,
                    cells[3],
                    &mut parameter.lower_bound,
                    &mut parameter.upper_bound,
                    &parameter.name,
                    writable,
                );
                changed |= schema_text_edit_enabled(
                    ui,
                    cells[4],
                    &mut parameter.description,
                    &format!("Description for {}", parameter.name),
                    writable,
                );
            }
        });
    if changed {
        app.state
            .workbench
            .model_editor
            .invalidate_candidate_evidence();
    }
}

pub(super) fn schema_text_edit(ui: &mut Ui, rect: Rect, value: &mut String, label: &str) -> bool {
    schema_text_edit_enabled(ui, rect, value, label, true)
}

pub(super) fn schema_text_edit_enabled(
    ui: &mut Ui,
    rect: Rect,
    value: &mut String,
    label: &str,
    enabled: bool,
) -> bool {
    let edit_rect = rect.shrink2(Vec2::new(4.0, 3.0));
    schema_text_edit_exact(ui, edit_rect, value, label, enabled)
}

pub(super) fn schema_text_edit_exact(
    ui: &mut Ui,
    edit_rect: Rect,
    value: &mut String,
    label: &str,
    enabled: bool,
) -> bool {
    let mut edit_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(edit_rect)
            .layout(Layout::left_to_right(Align::Center)),
    );
    let response = edit_ui
        .add_enabled_ui(enabled, |ui| {
            ui.add_sized(
                edit_rect.size(),
                egui::TextEdit::singleline(value)
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular)),
            )
        })
        .inner;
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(label);
    });
    response.changed()
}

pub(super) fn bounds_edit_enabled(
    ui: &mut Ui,
    rect: Rect,
    lower: &mut String,
    upper: &mut String,
    parameter_name: &str,
    enabled: bool,
) -> bool {
    let inner = rect.shrink2(Vec2::new(4.0, 3.0));
    let gap = 4.0;
    let field_width = ((inner.width() - gap) * 0.5).max(1.0);
    let lower_rect = Rect::from_min_size(inner.min, Vec2::new(field_width, inner.height()));
    let upper_rect = Rect::from_min_size(
        Pos2::new(lower_rect.right() + gap, inner.top()),
        Vec2::new(field_width, inner.height()),
    );
    schema_text_edit_exact(
        ui,
        lower_rect,
        lower,
        &format!("Lower bound for {parameter_name}"),
        enabled,
    ) | schema_text_edit_exact(
        ui,
        upper_rect,
        upper,
        &format!("Upper bound for {parameter_name}"),
        enabled,
    )
}

pub(super) fn new_section_dialog(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    records: &PersistedModelRecords,
) {
    if !app.state.workbench.model_editor.new_section_open {
        return;
    }
    let writable = !app.state.workbench.safe_mode.project_read_only();
    let primary_enabled = writable && !app.state.workbench.model_editor.new_section_name.is_empty();
    let choice = Dialog::new(
        "MODEL · PROCESS SECTION",
        "New process section",
        "Add section",
    )
    .description(
        "Create one validated inheritance node. The section remains part of the candidate until Save model revision commits it atomically.",
    )
    .fixed_height(300.0)
    .secondary("Cancel")
    .primary_enabled(primary_enabled)
    .show(ctx, |ui| {
        ui.set_enabled(writable);
        model_text_field(
            ui,
            "Section name",
            "Case-insensitive SPICE section identity",
            &mut app.state.workbench.model_editor.new_section_name,
        );
        let parent_names = records
            .metadata
            .as_ref()
            .map(|metadata| {
                metadata
                    .sections
                    .iter()
                    .map(|section| section.name.clone())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        model_parent_field(
            ui,
            &mut app.state.workbench.model_editor.new_section_parent,
            &parent_names,
        );
        if let Some(error) = app
            .state
            .workbench
            .model_editor
            .new_section_error
            .as_deref()
        {
            blocker_note(ui, error);
        }
    });
    match choice {
        DialogChoice::Primary => {
            app.state.workbench.model_editor.commit_new_section();
        }
        DialogChoice::Secondary | DialogChoice::Cancelled => {
            app.state.workbench.model_editor.new_section_open = false;
            app.state.workbench.model_editor.new_section_error = None;
        }
        DialogChoice::None | DialogChoice::Ghost => {}
    }
}

pub(super) fn model_text_field(ui: &mut Ui, label: &str, help: &str, value: &mut String) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, 54.0), Sense::hover());
    let label_rect = Rect::from_min_max(
        Pos2::new(rect.left(), rect.top()),
        Pos2::new(rect.left() + 190.0_f32.min(width * 0.34), rect.bottom()),
    );
    ui.painter().with_clip_rect(label_rect).text(
        label_rect.left_top(),
        Align2::LEFT_TOP,
        label,
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
    ui.painter().with_clip_rect(label_rect).text(
        Pos2::new(label_rect.left(), label_rect.top() + 19.0),
        Align2::LEFT_TOP,
        help,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    let field_rect = Rect::from_min_max(
        Pos2::new(label_rect.right() + 10.0, rect.top()),
        Pos2::new(rect.right(), rect.top() + t.metrics.ctl_h),
    );
    let mut field = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(field_rect)
            .layout(Layout::left_to_right(Align::Center)),
    );
    let response = field.add_sized(
        field_rect.size(),
        egui::TextEdit::singleline(value).font(theme::mono(tokens::FS_0, FontWeight::Regular)),
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(label);
    });
}

pub(super) fn model_parent_field(ui: &mut Ui, value: &mut String, parents: &[String]) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, 54.0), Sense::hover());
    let label_w = 190.0_f32.min(width * 0.34);
    ui.painter().with_clip_rect(rect).text(
        rect.left_top(),
        Align2::LEFT_TOP,
        "Parent section",
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
    ui.painter()
        .with_clip_rect(Rect::from_min_max(
            rect.min,
            Pos2::new(rect.left() + label_w, rect.bottom()),
        ))
        .text(
            Pos2::new(rect.left(), rect.top() + 19.0),
            Align2::LEFT_TOP,
            "Base model when no parent is selected",
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        );
    let field_rect = Rect::from_min_max(
        Pos2::new(rect.left() + label_w + 10.0, rect.top()),
        Pos2::new(rect.right(), rect.top() + t.metrics.ctl_h),
    );
    let mut field = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(field_rect)
            .layout(Layout::left_to_right(Align::Center)),
    );
    let response = egui::ComboBox::from_id_salt("model-editor-new-section-parent")
        .selected_text(if value.is_empty() {
            "Base model"
        } else {
            value.as_str()
        })
        .width(field_rect.width())
        .show_ui(&mut field, |ui| {
            ui.selectable_value(value, String::new(), "Base model");
            for parent in parents {
                ui.selectable_value(value, parent.clone(), parent);
            }
        });
    ui.ctx()
        .accesskit_node_builder(response.response.id, |node| {
            node.set_label("Parent section");
        });
}

pub(super) fn correlation_matrix_dialog(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    _records: &PersistedModelRecords,
) {
    if !app.state.workbench.model_editor.correlation_matrix_open {
        return;
    }
    let writable = !app.state.workbench.safe_mode.project_read_only();
    let choice = Dialog::new(
        "MODEL · STATISTICAL CORRELATION",
        "Correlation matrix",
        "Apply matrix",
    )
    .description("Edit the candidate's ordered correlation coefficients. Symmetric entries are kept synchronized and the complete positive-semidefinite matrix is validated atomically.")
    .size(DialogSize::Transaction)
    .fixed_height(680.0)
    .secondary("Cancel")
    .primary_enabled(writable)
    .show(ctx, |ui| {
        ui.set_enabled(writable);
        let matrices = app
            .state
            .workbench
            .model_editor
            .correlation_matrix_candidates
            .clone();
        if matrices.is_empty() {
            table_empty(
                ui,
                ui.available_width().max(1.0),
                "No correlation matrix is persisted.",
            );
            return;
        }
        ScrollArea::horizontal()
            .id_salt("model-editor-correlation-matrix-scroll")
            .auto_shrink([false, true])
            .show(ui, |ui| {
        for (matrix_index, matrix) in matrices.iter().enumerate() {
            dialog_subheading(ui, &matrix.group);
            let mut columns = vec![("Variable".to_owned(), 0.24)];
            let remaining = 0.76 / matrix.variables.len().max(1) as f32;
            columns.extend(
                matrix
                    .variables
                    .iter()
                    .map(|name| (name.clone(), remaining)),
            );
            let borrowed = columns
                .iter()
                .map(|(name, width)| (name.as_str(), *width))
                .collect::<Vec<_>>();
            let width = ui.available_width().max(TABLE_MIN_W);
            table_header(ui, width, &borrowed);
            if matrix.variables.is_empty() {
                table_empty(ui, width, "The matrix contains no variables.");
                continue;
            }
            for (row_index, name) in matrix.variables.iter().enumerate() {
                let cells = interactive_table_row(ui, width, &borrowed, false);
                paint_cell(ui, cells[0], &DisplayCell::mono(name));
                for column_index in 0..matrix.variables.len() {
                    if row_index == column_index {
                        paint_cell(ui, cells[column_index + 1], &DisplayCell::mono("1"));
                        continue;
                    }
                    let Some(current) = app
                        .state
                        .workbench
                        .model_editor
                        .correlation_matrix_edits
                        .get(matrix_index)
                        .and_then(|rows| rows.get(row_index))
                        .and_then(|row| row.get(column_index))
                        .cloned()
                    else {
                        paint_cell(
                            ui,
                            cells[column_index + 1],
                            &DisplayCell::toned("missing", MetricTone::Error),
                        );
                        continue;
                    };
                    let mut value = current;
                    if schema_text_edit(
                        ui,
                        cells[column_index + 1],
                        &mut value,
                        &format!(
                            "Correlation {} to {} in {}",
                            matrix.variables[row_index],
                            matrix.variables[column_index],
                            matrix.group
                        ),
                    )
                        && let Some(rows) = app
                            .state
                            .workbench
                            .model_editor
                            .correlation_matrix_edits
                            .get_mut(matrix_index)
                    {
                        rows[row_index][column_index] = value.clone();
                        rows[column_index][row_index] = value;
                    }
                }
            }
        }
            });
        if let Some(error) = app
            .state
            .workbench
            .model_editor
            .correlation_matrix_error
            .as_deref()
        {
            blocker_note(ui, error);
        }
    });
    match choice {
        DialogChoice::Primary => {
            app.state
                .workbench
                .model_editor
                .commit_correlation_matrix_edit();
        }
        DialogChoice::Secondary | DialogChoice::Cancelled => {
            app.state.workbench.model_editor.correlation_matrix_open = false;
            app.state
                .workbench
                .model_editor
                .correlation_matrix_candidates
                .clear();
            app.state
                .workbench
                .model_editor
                .correlation_matrix_edits
                .clear();
            app.state.workbench.model_editor.correlation_matrix_error = None;
        }
        DialogChoice::None | DialogChoice::Ghost => {}
    }
}

pub(super) fn temperature_preview_dialog(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    records: &PersistedModelRecords,
) {
    if !app.state.workbench.model_editor.temperature_preview_open {
        return;
    }
    let choice = Dialog::new(
        "MODEL · TEMPERATURE LAW",
        "Temperature-law plot",
        "Close",
    )
    .description(
        "Evaluate the candidate's declared equation or lookup table over its complete valid range using typed model parameters.",
    )
    .show(ctx, |ui| {
        let Some(metadata) = records.metadata.as_ref() else {
            table_empty(
                ui,
                ui.available_width().max(1.0),
                "No temperature-law metadata is available.",
            );
            return;
        };
        if metadata.temperature_laws.is_empty() {
            table_empty(
                ui,
                ui.available_width().max(1.0),
                "No temperature laws are declared.",
            );
            return;
        }
        for law in &metadata.temperature_laws {
            dialog_subheading(ui, &law.quantity);
            match temperature_samples(metadata, law) {
                Ok(samples) => temperature_plot(ui, law, &samples),
                Err(error) => blocker_note(ui, &error),
            }
        }
    });
    if matches!(choice, DialogChoice::Primary | DialogChoice::Cancelled) {
        app.state.workbench.model_editor.temperature_preview_open = false;
    }
}

pub(super) fn temperature_samples(
    metadata: &ModelDefinitionMetadata,
    law: &crate::state::model_library::TemperatureLawDefinition,
) -> Result<Vec<(f64, f64)>, String> {
    const SAMPLE_COUNT: usize = 121;
    let minimum = law.valid_range.minimum_c.get();
    let span = law.valid_range.maximum_c.get() - minimum;
    (0..SAMPLE_COUNT)
        .map(|index| {
            let temperature = minimum + span * index as f64 / (SAMPLE_COUNT - 1) as f64;
            metadata
                .evaluate_temperature_law(&law.quantity, temperature)
                .map(|evaluation| (temperature, evaluation.value.get()))
                .map_err(|error| error.to_string())
        })
        .collect()
}

pub(super) fn temperature_plot(
    ui: &mut Ui,
    law: &crate::state::model_library::TemperatureLawDefinition,
    samples: &[(f64, f64)],
) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(320.0);
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, 190.0), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_inset);
    ui.painter().rect_stroke(
        rect,
        0.0,
        Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    let plot = Rect::from_min_max(
        Pos2::new(rect.left() + 48.0, rect.top() + 14.0),
        Pos2::new(rect.right() - 14.0, rect.bottom() - 30.0),
    );
    let minimum_t = law.valid_range.minimum_c.get();
    let maximum_t = law.valid_range.maximum_c.get();
    let (mut minimum_y, mut maximum_y) = samples.iter().fold(
        (f64::INFINITY, f64::NEG_INFINITY),
        |(minimum, maximum), (_, value)| (minimum.min(*value), maximum.max(*value)),
    );
    if (maximum_y - minimum_y).abs() <= f64::EPSILON {
        let padding = maximum_y.abs().max(1.0) * 0.05;
        minimum_y -= padding;
        maximum_y += padding;
    }
    for division in 0..=4 {
        let fraction = division as f32 / 4.0;
        let y = egui::lerp(plot.bottom()..=plot.top(), fraction);
        ui.painter().hline(
            plot.x_range(),
            y,
            Stroke::new(1.0, t.color.border.gamma_multiply(0.55)),
        );
    }
    let points = samples
        .iter()
        .map(|(temperature, value)| {
            let x_fraction = ((*temperature - minimum_t) / (maximum_t - minimum_t)) as f32;
            let y_fraction = ((*value - minimum_y) / (maximum_y - minimum_y)) as f32;
            Pos2::new(
                egui::lerp(plot.left()..=plot.right(), x_fraction),
                egui::lerp(plot.bottom()..=plot.top(), y_fraction),
            )
        })
        .collect::<Vec<_>>();
    for pair in points.windows(2) {
        ui.painter()
            .line_segment([pair[0], pair[1]], Stroke::new(1.6, t.color.accent));
    }
    ui.painter().text(
        Pos2::new(plot.left(), rect.bottom() - 17.0),
        Align2::LEFT_CENTER,
        format!("{minimum_t} °C"),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    ui.painter().text(
        Pos2::new(plot.right(), rect.bottom() - 17.0),
        Align2::RIGHT_CENTER,
        format!("{maximum_t} °C"),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    ui.painter().text(
        Pos2::new(plot.left() - 7.0, plot.top()),
        Align2::RIGHT_TOP,
        format!("{maximum_y:.6e}"),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    ui.painter().text(
        Pos2::new(plot.left() - 7.0, plot.bottom()),
        Align2::RIGHT_BOTTOM,
        format!("{minimum_y:.6e}"),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
}
