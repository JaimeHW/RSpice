//! Rendering a planned workflow's specification.
//!
//! A planned workflow is authored content, not a live capability, and this
//! renders it as exactly what the fixture says — its ownership, sequence, and
//! notes are shown verbatim rather than inferred from product state. Nothing
//! here reads the workbench, which is what keeps a planned entry from ever
//! reading as something the product currently does.

use super::*;

pub(super) fn render_planned_specification(
    ui: &mut Ui,
    row: &data::PlannedWorkflowRow,
    specification: &data::PlannedWorkflowSpecification,
) {
    ui.spacing_mut().item_spacing.y = 0.0;
    let frame = data::PLANNED_WORKFLOW_FRAME;

    planned_warning_banner(
        ui,
        &format!(
            "{}{}{}",
            frame.unavailable_heading_prefix,
            row.status.as_str(),
            frame.unavailable_heading_suffix
        ),
        frame.unavailable_explanation,
    );
    workflow_purpose_line(ui, specification.purpose);
    planned_ownership_and_sequence(ui, row, frame);

    settings_section_label(ui, specification.content_section_title);
    render_planned_content(ui, specification);
    if let Some(chips) = specification.chip_section {
        render_planned_chips(ui, chips);
    }
    planned_note_grid(
        ui,
        frame.validation_section_title,
        specification.validation_recovery,
        frame.outputs_section_title,
        specification.outputs_provenance,
    );
    planned_info_banner(ui, frame.implementation_boundary);
}

pub(super) fn workflow_purpose_line(ui: &mut Ui, purpose: &str) {
    let t = Tokens::get(ui.ctx());
    let response = Frame::new()
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(12, 8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.set_min_height(19.0);
            ui.horizontal_top(|ui| {
                ui.spacing_mut().item_spacing.x = 8.0;
                let (icon_rect, icon_response) =
                    ui.allocate_exact_size(vec2(15.0, 15.0), Sense::hover());
                super::super::design_system::WorkbenchIcon::Info.paint(
                    ui.painter(),
                    Rect::from_center_size(icon_rect.center(), vec2(13.0, 13.0)),
                    t.color.info,
                );
                icon_response
                    .widget_info(|| WidgetInfo::labeled(WidgetType::Image, true, "Information"));
                ui.add(planned_wrapped_label(
                    purpose,
                    t.color.text_dim,
                    1.45 * tokens::FS_0,
                ));
            });
        })
        .response;
    ui.painter().extend(egui::Shape::dashed_line(
        &[response.rect.left_bottom(), response.rect.right_bottom()],
        Stroke::new(1.0, t.color.border_strong),
        3.0,
        3.0,
    ));
}

pub(super) fn planned_ownership_and_sequence(
    ui: &mut Ui,
    row: &data::PlannedWorkflowRow,
    frame: data::PlannedWorkflowFrame,
) {
    planned_split(
        ui,
        true,
        true,
        |ui| planned_ownership_panel(ui, row, frame),
        |ui| planned_sequence_panel(ui, frame),
    );
}

pub(super) fn planned_ownership_panel(
    ui: &mut Ui,
    row: &data::PlannedWorkflowRow,
    frame: data::PlannedWorkflowFrame,
) {
    planned_panel(
        ui,
        frame.ownership_section_title,
        Some(PlannedPanelStatus::Ok(frame.ownership_state)),
        |ui| {
            planned_property_list(ui, |ui| {
                for (label, value) in frame
                    .ownership_field_labels
                    .into_iter()
                    .zip([row.owner, row.entry, row.group])
                {
                    compact_property(ui, label, value);
                }
            });
        },
    );
}

pub(super) fn planned_sequence_panel(ui: &mut Ui, frame: data::PlannedWorkflowFrame) {
    planned_panel(
        ui,
        frame.task_sequence_section_title,
        Some(PlannedPanelStatus::Plain(frame.task_sequence_route_state)),
        |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(7.0, 7.0);
                for step in frame.task_steps {
                    mini_badge(ui, step);
                }
            });
            ui.add_space(10.0);
        },
    );
}

pub(super) fn planned_note_grid(
    ui: &mut Ui,
    left_title: &str,
    left_body: &str,
    right_title: &str,
    right_body: &str,
) {
    planned_split(
        ui,
        true,
        true,
        |ui| planned_note(ui, left_title, left_body),
        |ui| planned_note(ui, right_title, right_body),
    );
}

pub(super) fn planned_note(ui: &mut Ui, title: &str, body: &str) {
    let t = Tokens::get(ui.ctx());
    ui.set_width(ui.available_width());
    ui.label(
        RichText::new(title)
            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text),
    );
    ui.add_space(4.0);
    ui.add(planned_wrapped_label(
        body,
        t.color.text_dim,
        1.45 * tokens::FS_0,
    ));
}

pub(super) fn planned_wrapped_label(text: &str, color: Color32, line_height: f32) -> Label {
    let mut job = egui::text::LayoutJob::default();
    job.append(
        text,
        0.0,
        egui::TextFormat {
            font_id: theme::sans(tokens::FS_0, FontWeight::Regular),
            line_height: Some(line_height),
            color,
            ..Default::default()
        },
    );
    Label::new(job).wrap().selectable(true)
}

#[derive(Debug, Clone, Copy)]
pub(super) enum PlannedPanelStatus<'a> {
    Plain(&'a str),
    Ok(&'a str),
}

pub(super) fn planned_panel(
    ui: &mut Ui,
    title: &str,
    status: Option<PlannedPanelStatus<'_>>,
    body: impl FnOnce(&mut Ui),
) {
    let t = Tokens::get(ui.ctx());
    ui.set_width(ui.available_width());
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 29.0), Sense::hover());
    ui.painter().rect_filled(
        rect,
        0.0,
        Color32::from_rgba_unmultiplied(
            t.color.bg_panel_2.r(),
            t.color.bg_panel_2.g(),
            t.color.bg_panel_2.b(),
            204,
        ),
    );
    let mut header = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect.shrink2(vec2(10.0, 0.0)))
            .layout(Layout::left_to_right(Align::Center)),
    );
    let mut title_job = egui::text::LayoutJob::default();
    title_job.append(
        &title.to_uppercase(),
        0.0,
        egui::TextFormat {
            font_id: theme::sans(tokens::FS_0, FontWeight::SemiBold),
            color: t.color.text_dim,
            extra_letter_spacing: 0.055 * tokens::FS_0,
            ..Default::default()
        },
    );
    header.add(Label::new(title_job).truncate().selectable(true));
    if let Some(status) = status {
        header.with_layout(Layout::right_to_left(Align::Center), |ui| match status {
            PlannedPanelStatus::Plain(text) => {
                ui.add(
                    Label::new(
                        RichText::new(text.to_uppercase())
                            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                            .color(t.color.text_dim),
                    )
                    .truncate()
                    .selectable(true),
                );
            }
            PlannedPanelStatus::Ok(text) => {
                ui.spacing_mut().item_spacing.x = 5.0;
                ui.add(
                    Label::new(
                        RichText::new(text.to_uppercase())
                            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                            .color(t.color.ok),
                    )
                    .truncate()
                    .selectable(true),
                );
                let (dot, _) = ui.allocate_exact_size(vec2(5.0, 5.0), Sense::hover());
                ui.painter().circle_filled(dot.center(), 2.5, t.color.ok);
            }
        });
    }
    body(ui);
}

pub(super) fn planned_split(
    ui: &mut Ui,
    left_padded: bool,
    right_padded: bool,
    left: impl FnOnce(&mut Ui),
    right: impl FnOnce(&mut Ui),
) {
    let border = Tokens::get(ui.ctx()).color.border_strong;
    let wide = ui.available_width() > 760.0;
    ui.add_space(10.0);
    let frame = Frame::NONE.stroke(Stroke::new(1.0, border));
    if wide {
        frame.show(ui, |ui| {
            let response = ui
                .horizontal_top(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    let width = ui.available_width() * 0.5;
                    planned_split_panel(ui, width, left_padded, left);
                    planned_split_panel(ui, ui.available_width(), right_padded, right);
                })
                .response;
            ui.painter().vline(
                response.rect.center().x,
                response.rect.y_range(),
                Stroke::new(1.0, border),
            );
        });
    } else {
        frame.show(ui, |ui| {
            let left_response = planned_split_panel(ui, ui.available_width(), left_padded, left);
            ui.painter().hline(
                left_response.response.rect.x_range(),
                left_response.response.rect.bottom(),
                Stroke::new(1.0, border),
            );
            planned_split_panel(ui, ui.available_width(), right_padded, right);
        });
    }
}

pub(super) fn planned_split_panel(
    ui: &mut Ui,
    width: f32,
    padded: bool,
    content: impl FnOnce(&mut Ui),
) -> egui::InnerResponse<()> {
    ui.allocate_ui_with_layout(Vec2::new(width, 0.0), Layout::top_down(Align::Min), |ui| {
        if padded {
            Frame::NONE.inner_margin(Margin::same(10)).show(ui, content);
        } else {
            content(ui);
        }
    })
}

pub(super) fn planned_property_list(ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
    Frame::NONE
        .inner_margin(Margin {
            left: 10,
            right: 10,
            top: 7,
            bottom: 10,
        })
        .show(ui, content);
}

pub(super) fn compact_property(ui: &mut Ui, label: &str, value: &str) {
    let t = Tokens::get(ui.ctx());
    compact_property_row(ui, label, value, t.color.text);
}

pub(super) fn compact_property_row(ui: &mut Ui, label: &str, value: &str, value_color: Color32) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), 29.0), Sense::hover());
    let gap = 8.0;
    let usable = (rect.width() - gap).max(1.0);
    let label_width = if usable >= 170.0 {
        (usable * 0.4).clamp(74.0, usable - 96.0)
    } else {
        usable * 0.4
    };
    let label_rect = Rect::from_min_max(
        rect.min,
        pos2((rect.left() + label_width).min(rect.right()), rect.bottom()),
    );
    let value_rect = Rect::from_min_max(
        pos2((label_rect.right() + gap).min(rect.right()), rect.top()),
        rect.max,
    );
    let mut label_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(label_rect)
            .layout(Layout::left_to_right(Align::Center)),
    );
    let label_response = label_ui.add_sized(
        label_rect.size(),
        Label::new(
            RichText::new(label)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        )
        .truncate()
        .selectable(true),
    );
    label_ui
        .ctx()
        .accesskit_node_builder(label_response.id, |node| {
            node.set_role(egui::accesskit::Role::Cell);
        });
    let mut value_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(value_rect)
            .layout(Layout::left_to_right(Align::Center)),
    );
    let value_response = value_ui.add_sized(
        value_rect.size(),
        Label::new(
            RichText::new(value)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(value_color),
        )
        .truncate()
        .selectable(true),
    );
    value_ui
        .ctx()
        .accesskit_node_builder(value_response.id, |node| {
            node.set_role(egui::accesskit::Role::Cell);
        });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Row);
        node.set_label(format!("{label}: {value}"));
    });
}

pub(super) fn mini_badge(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .inner_margin(Margin::symmetric(10, 0))
        .show(ui, |ui| {
            ui.set_min_height(26.0);
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.add(
                    Label::new(
                        RichText::new(text)
                            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                            .color(t.color.text_faint),
                    )
                    .selectable(true),
                );
            });
        });
}

pub(super) fn render_planned_content(
    ui: &mut Ui,
    specification: &data::PlannedWorkflowSpecification,
) {
    match specification.content_layout {
        data::PlannedWorkflowContentLayout::TableStackAndProperties => {
            if !specification.properties.is_empty() {
                planned_split(
                    ui,
                    false,
                    true,
                    |ui| {
                        for table in specification.tables {
                            render_planned_table(ui, table);
                        }
                    },
                    |ui| {
                        render_planned_properties(
                            ui,
                            specification.property_section_title,
                            specification.properties,
                        );
                    },
                );
            } else {
                for table in specification.tables {
                    render_planned_table(ui, table);
                }
            }
        }
        data::PlannedWorkflowContentLayout::SplitTables => {
            if specification.tables.len() >= 2 {
                planned_split(
                    ui,
                    false,
                    false,
                    |ui| render_planned_table(ui, &specification.tables[0]),
                    |ui| render_planned_table(ui, &specification.tables[1]),
                );
                for table in specification.tables.iter().skip(2) {
                    render_planned_table(ui, table);
                }
            } else {
                for table in specification.tables {
                    render_planned_table(ui, table);
                }
            }
            if !specification.properties.is_empty() {
                ui.add_space(10.0);
                render_planned_properties(
                    ui,
                    specification.property_section_title,
                    specification.properties,
                );
            }
        }
    }
}

pub(super) fn render_planned_table(ui: &mut Ui, table: &data::PlannedWorkflowTable) {
    let narrow = ui.available_width() <= 820.0;
    if narrow {
        egui::ScrollArea::horizontal()
            .id_salt(("planned-capability-table", table.headers))
            .auto_shrink([false, true])
            .show(ui, |ui| {
                ui.set_min_width(660.0);
                render_planned_table_contents(ui, table);
            });
    } else {
        render_planned_table_contents(ui, table);
    }
}

pub(super) fn render_planned_table_contents(ui: &mut Ui, table: &data::PlannedWorkflowTable) {
    let response = ui
        .scope(|ui| render_planned_table_contents_inner(ui, table))
        .response;
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Table);
        node.set_label(table.headers.join("; "));
    });
}

pub(super) fn render_planned_table_contents_inner(ui: &mut Ui, table: &data::PlannedWorkflowTable) {
    let table_top = ui.cursor().top();
    let header_rect = table_header(ui, table.headers);
    for row in table.rows {
        let t = Tokens::get(ui.ctx());
        let (rect, response) =
            ui.allocate_exact_size(vec2(ui.available_width(), t.metrics.row_h), Sense::hover());
        response.widget_info(|| {
            WidgetInfo::labeled(
                WidgetType::Label,
                ui.is_enabled(),
                row.iter()
                    .map(|cell| cell.text)
                    .collect::<Vec<_>>()
                    .join("; "),
            )
        });
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_role(egui::accesskit::Role::Row);
        });
        if ui.is_rect_visible(rect) {
            if response.hovered() {
                ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
            }
            ui.painter().hline(
                rect.x_range(),
                rect.bottom(),
                Stroke::new(
                    1.0,
                    Color32::from_rgba_unmultiplied(
                        t.color.border.r(),
                        t.color.border.g(),
                        t.color.border.b(),
                        191,
                    ),
                ),
            );
        }
        let cell_width = rect.width() / row.len().max(1) as f32;
        for (index, cell) in row.iter().enumerate() {
            let left = rect.left() + index as f32 * cell_width;
            let right = if index + 1 == row.len() {
                rect.right()
            } else {
                left + cell_width
            };
            let cell_rect = Rect::from_min_max(pos2(left, rect.top()), pos2(right, rect.bottom()));
            let content_rect = cell_rect.shrink2(vec2(8.0, 0.0));
            let mut cell_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(content_rect)
                    .layout(Layout::left_to_right(Align::Center)),
            );
            cell_ui.set_clip_rect(cell_rect.intersect(ui.clip_rect()));
            let response = cell_ui.add_sized(
                vec2(content_rect.width(), content_rect.height()),
                Label::new(planned_rich_text(
                    &cell_ui,
                    cell.text,
                    cell.style,
                    response.hovered(),
                ))
                .truncate()
                .selectable(true),
            );
            cell_ui.ctx().accesskit_node_builder(response.id, |node| {
                node.set_role(egui::accesskit::Role::Cell);
            });
        }
    }
    let table_rect = Rect::from_min_max(
        pos2(header_rect.left(), table_top),
        pos2(
            header_rect.right(),
            ui.cursor().top().max(header_rect.bottom()),
        ),
    );
    paint_sticky_table_header(ui, table_rect, header_rect, table.headers);
}

pub(super) fn render_planned_properties(
    ui: &mut Ui,
    title: Option<&str>,
    properties: &[data::PlannedWorkflowProperty],
) {
    planned_panel(
        ui,
        title.unwrap_or("Configuration properties"),
        None,
        |ui| {
            planned_property_list(ui, |ui| {
                for property in properties {
                    compact_property_styled(ui, property.label, property.value, property.style);
                }
            });
        },
    );
}

pub(super) fn render_planned_chips(ui: &mut Ui, section: data::PlannedWorkflowChipSection) {
    settings_section_label(ui, section.title);
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(7.0, 7.0);
        for chip in section.chips {
            mini_badge(ui, chip);
        }
    });
    ui.add_space(10.0);
}

pub(super) fn compact_property_styled(
    ui: &mut Ui,
    label: &str,
    value: &str,
    style: data::PlannedWorkflowTextStyle,
) {
    let color = match style {
        data::PlannedWorkflowTextStyle::Success => Tokens::get(ui.ctx()).color.ok,
        data::PlannedWorkflowTextStyle::Warning => Tokens::get(ui.ctx()).color.warn,
        data::PlannedWorkflowTextStyle::Plain | data::PlannedWorkflowTextStyle::Monospace => {
            Tokens::get(ui.ctx()).color.text
        }
    };
    compact_property_row(ui, label, value, color);
}

pub(super) fn planned_rich_text(
    ui: &Ui,
    text: &str,
    style: data::PlannedWorkflowTextStyle,
    row_hovered: bool,
) -> RichText {
    let t = Tokens::get(ui.ctx());
    let (font, color) = match style {
        data::PlannedWorkflowTextStyle::Plain => (
            theme::sans(tokens::FS_0, FontWeight::Regular),
            if row_hovered {
                t.color.text
            } else {
                t.color.text_dim
            },
        ),
        data::PlannedWorkflowTextStyle::Monospace => (
            theme::mono(tokens::FS_0, FontWeight::Regular),
            if row_hovered {
                t.color.text
            } else {
                t.color.text_dim
            },
        ),
        data::PlannedWorkflowTextStyle::Success => {
            (theme::sans(tokens::FS_0, FontWeight::Regular), t.color.ok)
        }
        data::PlannedWorkflowTextStyle::Warning => {
            (theme::sans(tokens::FS_0, FontWeight::Regular), t.color.warn)
        }
    };
    RichText::new(text).font(font).color(color)
}
