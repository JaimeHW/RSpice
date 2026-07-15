//! Canonical product capability and platform matrix.
//!
//! The mockup owns the information architecture and copy for this manager.
//! Rust owns the fail-closed execution boundary: design-fixture rows are
//! visibly identified as reference data and never become runtime availability
//! or production qualification evidence.

use egui::{
    Align, Color32, Frame, Label, Layout, Margin, Rect, RichText, Sense, Stroke, Ui, Vec2,
    WidgetInfo, WidgetType, pos2, vec2,
};

use crate::common::{RSpiceApp, app::ConsoleMessage};
use crate::ui::{
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::{Button, Dialog, DialogChoice, DialogInitialFocus, DialogSize, select},
};

use super::{
    CapabilityWorkflowId, RouteTransitionSource, SurfaceId, SurfaceRoute,
    feature_availability_data as data,
    state::{
        CapabilityMatrixDrilldown, CapabilityMatrixSection, CapabilityMatrixState,
        EngineeringProfile, InteroperabilityDomain, InteroperabilitySection,
        InteroperabilitySupportLevel,
    },
};

#[derive(Debug)]
enum MatrixAction {
    OpenRoute(SurfaceRoute),
}

/// Render the manager only while its canonical route owns the foreground.
pub fn show(ctx: &egui::Context, app: &mut RSpiceApp) {
    let current_route = app.state.workbench.current_route();
    if current_route.surface_id() != SurfaceId::FeatureAvailability {
        return;
    }

    // The dialog edits application-session view state, not engineering data.
    // Keeping a local draft also avoids borrowing the application through the
    // modal closure while route and log actions are collected transactionally.
    let mut matrix = app.state.workbench.capability_matrix.clone();
    let engineering_profile = app.state.workbench.engineering_profile;
    let mut route_return_reanchor = false;
    if let Some(workflow) = current_route.capability_workflow_id() {
        match workflow {
            CapabilityWorkflowId::InteroperabilityMatrix => {
                matrix.drilldown = Some(CapabilityMatrixDrilldown::Interoperability);
            }
            CapabilityWorkflowId::PlatformLifecycle => {
                matrix.drilldown = Some(CapabilityMatrixDrilldown::PlatformLifecycle);
            }
            CapabilityWorkflowId::TouchEditGuide => {
                matrix.drilldown = Some(CapabilityMatrixDrilldown::TouchEditGuide);
            }
            _ if data::planned_workflow_row(workflow.as_str()).is_some()
                && data::planned_workflow_specification(workflow.as_str()).is_some() =>
            {
                matrix.drilldown = Some(CapabilityMatrixDrilldown::PlannedWorkflow(
                    workflow.as_str().to_owned(),
                ));
            }
            _ => {
                app.state.push_user_message(ConsoleMessage::warning(format!(
                    "Capability workflow `{}` is unavailable because its governed specification is not registered.",
                    workflow.as_str()
                )));
                close_to_source(app);
                return;
            }
        }
    } else if matrix.drilldown.is_some() {
        route_return_reanchor = true;
        matrix.drilldown = None;
        matrix.drilldown_scroll_offset = 0.0;
    }
    if matches!(
        matrix.drilldown.as_ref(),
        Some(CapabilityMatrixDrilldown::PlannedWorkflow(id))
            if data::planned_workflow_row(id).is_none()
                || data::planned_workflow_specification(id).is_none()
    ) {
        matrix.drilldown = None;
        matrix.drilldown_scroll_offset = 0.0;
    }

    let drilldown = matrix.drilldown.clone();
    let planned_content = drilldown.as_ref().and_then(|drilldown| match drilldown {
        CapabilityMatrixDrilldown::PlannedWorkflow(id) => {
            data::planned_workflow_row(id).zip(data::planned_workflow_specification(id))
        }
        CapabilityMatrixDrilldown::Interoperability
        | CapabilityMatrixDrilldown::TouchEditGuide
        | CapabilityMatrixDrilldown::PlatformLifecycle => None,
    });
    let matrix_route = SurfaceRoute::surface(SurfaceId::FeatureAvailability);
    let workflow_from_matrix = current_route.capability_workflow_id().is_some()
        && app.state.workbench.previous_route() == Some(matrix_route);
    let planned_from_matrix = planned_content.is_some() && workflow_from_matrix;
    let (kicker, title, primary) = match (drilldown.as_ref(), planned_content) {
        (_, Some((row, _))) => (
            format!(
                "{} · {} · {}",
                row.group.to_uppercase(),
                data::PLANNED_WORKFLOW_FRAME.dialog_eyebrow_status,
                row.status.as_str().to_uppercase()
            ),
            format!(
                "{}{}",
                row.label,
                data::PLANNED_WORKFLOW_FRAME.dialog_title_suffix
            ),
            if planned_from_matrix {
                "Back to capability matrix"
            } else {
                "Close"
            },
        ),
        (Some(CapabilityMatrixDrilldown::Interoperability), None) => (
            data::INTEROPERABILITY_DIALOG_EYEBROW.to_owned(),
            data::INTEROPERABILITY_DIALOG_TITLE.to_owned(),
            "Close",
        ),
        (Some(CapabilityMatrixDrilldown::PlatformLifecycle), None) => (
            data::PLATFORM_LIFECYCLE_DIALOG_EYEBROW.to_owned(),
            data::PLATFORM_LIFECYCLE_DIALOG_TITLE.to_owned(),
            "Close",
        ),
        (Some(CapabilityMatrixDrilldown::TouchEditGuide), None) => (
            data::TOUCH_EDIT_GUIDE_DIALOG_EYEBROW.to_owned(),
            data::TOUCH_EDIT_GUIDE_DIALOG_TITLE.to_owned(),
            "Close",
        ),
        (None, None) => (
            "HELP · VERSIONED PRODUCT CONTRACT".to_owned(),
            "Product capability and platform matrix".to_owned(),
            "Close",
        ),
        (Some(CapabilityMatrixDrilldown::PlannedWorkflow(_)), None) => (
            "HELP · VERSIONED PRODUCT CONTRACT".to_owned(),
            "Product capability and platform matrix".to_owned(),
            "Close",
        ),
    };
    let mut body_scroll_offset = if drilldown.is_some() {
        matrix.drilldown_scroll_offset
    } else {
        matrix.scroll_offset
    };
    let mut pending_action = None;
    let touch_layout = app.state.workbench.coarse_pointer || ctx.content_rect().width() <= 820.0;
    let initial_focus = match (drilldown.as_ref(), planned_content) {
        (None, None) => DialogInitialFocus::BodyControl,
        (_, Some(_)) if planned_from_matrix => DialogInitialFocus::Primary,
        _ => DialogInitialFocus::Close,
    };
    let description = workflow_description(drilldown.as_ref());
    let dialog = Dialog::new(&kicker, &title, primary)
        .description(description)
        .initial_focus(initial_focus)
        .size(DialogSize::CapabilityReview)
        .flush_body()
        .primary_on_enter(false)
        .body_scroll_offset(&mut body_scroll_offset);
    let dialog = if planned_from_matrix {
        dialog.ghost("Cancel")
    } else {
        dialog
    };
    let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
        if touch_layout {
            ui.spacing_mut().interact_size.y = 44.0;
        }
        match drilldown.as_ref() {
            Some(CapabilityMatrixDrilldown::PlannedWorkflow(_)) => {
                if let Some((row, specification)) = planned_content {
                    render_planned_specification(ui, row, specification);
                }
                None
            }
            Some(CapabilityMatrixDrilldown::Interoperability) => {
                render_interoperability(ui, &mut matrix);
                None
            }
            Some(CapabilityMatrixDrilldown::PlatformLifecycle) => {
                render_platform_lifecycle(ui, &mut pending_action);
                None
            }
            Some(CapabilityMatrixDrilldown::TouchEditGuide) => {
                render_touch_edit_guide(ui);
                None
            }
            None => Some(render_matrix(
                ui,
                &mut matrix,
                &mut pending_action,
                engineering_profile,
                route_return_reanchor,
            )),
        }
    });
    if drilldown != matrix.drilldown {
        if drilldown.is_none() {
            matrix.scroll_offset = body_scroll_offset;
        }
        matrix.drilldown_scroll_offset = 0.0;
    } else if drilldown.is_some() {
        matrix.drilldown_scroll_offset = body_scroll_offset;
    } else {
        matrix.scroll_offset = body_scroll_offset;
    }
    app.state.workbench.capability_matrix = matrix;

    if let Some(action) = pending_action {
        match action {
            MatrixAction::OpenRoute(route) => {
                if let Err(error) = app
                    .state
                    .workbench
                    .navigate(route, RouteTransitionSource::User)
                {
                    app.state.push_user_message(ConsoleMessage::warning(format!(
                        "Capability route was not opened: {error}"
                    )));
                }
            }
        }
    }

    if current_route.capability_workflow_id().is_some() {
        if planned_content.is_some() {
            match choice {
                DialogChoice::Primary if planned_from_matrix => close_workflow_to_matrix(app),
                DialogChoice::Primary | DialogChoice::Ghost | DialogChoice::Cancelled => {
                    close_planned_workflow(app, planned_from_matrix);
                }
                DialogChoice::None | DialogChoice::Secondary => {}
            }
        } else {
            match choice {
                DialogChoice::Primary | DialogChoice::Cancelled => {
                    if workflow_from_matrix {
                        close_workflow_to_matrix(app);
                    } else {
                        close_to_source(app);
                    }
                }
                DialogChoice::None | DialogChoice::Secondary | DialogChoice::Ghost => {}
            }
        }
    } else if matches!(choice, DialogChoice::Primary | DialogChoice::Cancelled) {
        if drilldown.is_some() {
            app.state.workbench.capability_matrix.drilldown = None;
            app.state
                .workbench
                .capability_matrix
                .drilldown_scroll_offset = 0.0;
        } else {
            close_to_source(app);
        }
    }
}

fn workflow_description(drilldown: Option<&CapabilityMatrixDrilldown>) -> &'static str {
    match drilldown {
        None => {
            "Review intended product scope, implementation evidence, and qualification boundaries in this read-only capability disclosure."
        }
        Some(CapabilityMatrixDrilldown::PlannedWorkflow(_)) => {
            "Inspect the purpose, ownership, sequence, inputs, validation, recovery, outputs, and provenance of this unavailable capability design."
        }
        Some(CapabilityMatrixDrilldown::Interoperability) => {
            "Review versioned import, export, round-trip, and qualification contracts for supported format profiles."
        }
        Some(CapabilityMatrixDrilldown::PlatformLifecycle) => {
            "Review suspension, storage, recovery, and release-eligibility behavior across browser and mobile platforms."
        }
        Some(CapabilityMatrixDrilldown::TouchEditGuide) => {
            "Review touch gestures, snapped placement, inspection, and exact-coordinate fallback for schematic editing."
        }
    }
}

fn close_planned_workflow(app: &mut RSpiceApp, from_matrix: bool) {
    if from_matrix
        && app.state.workbench.back_route_count() >= 2
        && app
            .state
            .workbench
            .navigate_back_steps(2, RouteTransitionSource::User)
            .is_some()
    {
        return;
    }
    if from_matrix {
        replace_with_workspace_fallback(app);
        return;
    }
    close_to_source(app);
}

fn close_workflow_to_matrix(app: &mut RSpiceApp) {
    let matrix_route = SurfaceRoute::surface(SurfaceId::FeatureAvailability);
    if app.state.workbench.previous_route() == Some(matrix_route)
        && app
            .state
            .workbench
            .navigate_back(RouteTransitionSource::User)
            .is_some()
    {
        return;
    }

    if let Err(error) = app
        .state
        .workbench
        .replace_route(matrix_route, RouteTransitionSource::User)
    {
        app.state.push_user_message(ConsoleMessage::warning(format!(
            "Could not return to the capability matrix: {error}"
        )));
    }
}

fn close_to_source(app: &mut RSpiceApp) {
    if app
        .state
        .workbench
        .navigate_back(RouteTransitionSource::User)
        .is_some()
    {
        return;
    }

    replace_with_workspace_fallback(app);
}

fn replace_with_workspace_fallback(app: &mut RSpiceApp) {
    // A direct deep link has no in-app predecessor. Replace it with the
    // retained primary workspace projection so Close never creates a loop.
    let fallback = SurfaceRoute::surface(SurfaceId::from_workspace(app.state.workbench.workspace));
    if let Err(error) = app
        .state
        .workbench
        .replace_route(fallback, RouteTransitionSource::User)
    {
        app.state.push_user_message(ConsoleMessage::warning(format!(
            "Could not close the capability manager: {error}"
        )));
    }
}

fn render_planned_specification(
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

fn workflow_purpose_line(ui: &mut Ui, purpose: &str) {
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
                super::design_system::WorkbenchIcon::Info.paint(
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

fn planned_ownership_and_sequence(
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

fn planned_ownership_panel(
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

fn planned_sequence_panel(ui: &mut Ui, frame: data::PlannedWorkflowFrame) {
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

fn planned_note_grid(
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

fn planned_note(ui: &mut Ui, title: &str, body: &str) {
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

fn planned_wrapped_label(text: &str, color: Color32, line_height: f32) -> Label {
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
enum PlannedPanelStatus<'a> {
    Plain(&'a str),
    Ok(&'a str),
}

fn planned_panel(
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

fn planned_split(
    ui: &mut Ui,
    left_padded: bool,
    right_padded: bool,
    left: impl FnOnce(&mut Ui),
    right: impl FnOnce(&mut Ui),
) {
    let border = Tokens::get(ui.ctx()).color.border_strong;
    let wide = ui.ctx().content_rect().width() > 760.0;
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

fn planned_split_panel(
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

fn planned_property_list(ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
    Frame::NONE
        .inner_margin(Margin {
            left: 10,
            right: 10,
            top: 7,
            bottom: 10,
        })
        .show(ui, content);
}

fn compact_property(ui: &mut Ui, label: &str, value: &str) {
    let t = Tokens::get(ui.ctx());
    compact_property_row(ui, label, value, t.color.text);
}

fn compact_property_row(ui: &mut Ui, label: &str, value: &str, value_color: Color32) {
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

fn mini_badge(ui: &mut Ui, text: &str) {
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

fn render_planned_content(ui: &mut Ui, specification: &data::PlannedWorkflowSpecification) {
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

fn render_planned_table(ui: &mut Ui, table: &data::PlannedWorkflowTable) {
    let narrow = ui.ctx().content_rect().width() <= 820.0;
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

fn render_planned_table_contents(ui: &mut Ui, table: &data::PlannedWorkflowTable) {
    let response = ui
        .scope(|ui| render_planned_table_contents_inner(ui, table))
        .response;
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Table);
        node.set_label(table.headers.join("; "));
    });
}

fn render_planned_table_contents_inner(ui: &mut Ui, table: &data::PlannedWorkflowTable) {
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

fn render_planned_properties(
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

fn render_planned_chips(ui: &mut Ui, section: data::PlannedWorkflowChipSection) {
    settings_section_label(ui, section.title);
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(7.0, 7.0);
        for chip in section.chips {
            mini_badge(ui, chip);
        }
    });
    ui.add_space(10.0);
}

fn compact_property_styled(
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

fn planned_rich_text(
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

fn render_matrix(
    ui: &mut Ui,
    matrix: &mut CapabilityMatrixState,
    pending_action: &mut Option<MatrixAction>,
    engineering_profile: EngineeringProfile,
    route_return_reanchor: bool,
) -> egui::Id {
    // The mockup's workflow body is a flush document: direct children own
    // their borders and padding, so egui must not insert implicit gaps.
    ui.spacing_mut().item_spacing.y = 0.0;
    active_profile_banner(ui, engineering_profile.label());
    let document_compact = ui.available_width() <= 820.0;
    let reflow_target = matrix
        .last_document_compact
        .replace(document_compact)
        .filter(|previous| *previous != document_compact)
        .map(|_| matrix.section);
    let (toolbar_jump_target, section_picker_id) = render_toolbar(ui, matrix);
    let jump_target = toolbar_jump_target
        .or(reflow_target)
        .or(route_return_reanchor.then_some(matrix.section));

    render_claim_resolution(ui);

    section_anchor(ui, CapabilityMatrixSection::Platforms, jump_target);
    render_platforms(ui, pending_action);

    section_anchor(ui, CapabilityMatrixSection::PlannedDesigns, jump_target);
    render_planned_workflows(ui, pending_action);

    section_anchor(ui, CapabilityMatrixSection::Analyses, jump_target);
    render_analyses(ui);

    section_anchor(ui, CapabilityMatrixSection::Workspaces, jump_target);
    render_workspaces(ui, engineering_profile);
    section_picker_id
}

fn render_toolbar(
    ui: &mut Ui,
    matrix: &mut CapabilityMatrixState,
) -> (Option<CapabilityMatrixSection>, egui::Id) {
    let t = Tokens::get(ui.ctx());
    let previous_section = matrix.section;
    let mut section_picker_id = None;
    let response = Frame::NONE
        .inner_margin(Margin::symmetric(12, 10))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.set_min_height(34.0);
            let compact = ui.ctx().content_rect().width() <= 560.0;
            if compact {
                setting_row_copy(ui);
                ui.add_space(10.0);
                section_picker_id = Some(section_picker(ui, matrix));
            } else {
                let gap = 12.0;
                let content_width = (ui.available_width() - gap).max(0.0);
                let left_width = (content_width * (0.38 / 1.38)).max(150.0);
                let right_width = (content_width - left_width).max(0.0);
                ui.horizontal_top(|ui| {
                    ui.spacing_mut().item_spacing.x = gap;
                    ui.allocate_ui_with_layout(
                        Vec2::new(left_width, 0.0),
                        Layout::top_down(Align::Min),
                        setting_row_copy,
                    );
                    ui.allocate_ui_with_layout(
                        Vec2::new(right_width, 0.0),
                        Layout::top_down(Align::Min),
                        |ui| section_picker_id = Some(section_picker(ui, matrix)),
                    );
                });
            }
        })
        .response;
    ui.painter().hline(
        response.rect.x_range(),
        response.rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    (
        (matrix.section != previous_section).then_some(matrix.section),
        section_picker_id.expect("the capability toolbar always renders its section select"),
    )
}

fn setting_row_copy(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        RichText::new("Jump to matrix section")
            .font(theme::sans(tokens::FS_0, FontWeight::Medium))
            .color(t.color.text),
    );
    ui.add_space(3.0);
    ui.label(
        RichText::new("Move within this read-only capability document.")
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_faint),
    );
}

fn section_anchor(
    ui: &mut Ui,
    section: CapabilityMatrixSection,
    jump_target: Option<CapabilityMatrixSection>,
) {
    let response = ui
        .push_id(("capability-matrix-section-anchor", section), |ui| {
            ui.allocate_response(Vec2::new(ui.available_width(), 0.0), Sense::hover())
        })
        .inner;
    if jump_target == Some(section) {
        response.scroll_to_me(Some(Align::Min));
    }
}

fn section_picker(ui: &mut Ui, matrix: &mut CapabilityMatrixState) -> egui::Id {
    let labels = CapabilityMatrixSection::ALL
        .iter()
        .map(|section| section.label().to_owned())
        .collect::<Vec<_>>();
    // `select` allocates exactly one control before opening its popup, so the
    // next auto id is the stable focus target for this render pass.
    let response_id = ui.next_auto_id();
    if let Some(index) = select(
        ui,
        "product-readiness.capability-matrix.section",
        "Capability matrix section",
        matrix.section.label(),
        &labels,
        ui.available_width().max(1.0),
    ) {
        matrix.section = CapabilityMatrixSection::ALL[index];
    }
    response_id
}

fn render_claim_resolution(ui: &mut Ui) {
    let all_rows = &data::CAPABILITY_CLAIM_PROJECTIONS;
    let permitted = all_rows.iter().filter(|row| row.label_allowed).count();
    let blocked = all_rows.len() - permitted;

    warning_banner(
        ui,
        &format!(
            "Fail-closed capability resolver · {}",
            data::CAPABILITY_FIXTURE_REVISION
        ),
        data::CAPABILITY_FIXTURE_BOUNDARY,
    );
    summary_stats(
        ui,
        all_rows.len(),
        permitted,
        blocked,
        data::CAPABILITY_FIXTURE_AS_OF,
    );
    matrix_table(
        ui,
        "capability-claim-table",
        960.0,
        &[
            "Resolved customer copy",
            "Subject / stage",
            "Currentness",
            "Applicable source",
            "Reason codes",
        ],
        |ui| {
            for row in all_rows {
                let reasons = if row.reason_codes.is_empty() {
                    "all exact bindings current".to_owned()
                } else {
                    row.reason_codes.join(" · ")
                };
                let subject = format!("{} · {}", row.subject_kind, row.asserted_stage);
                let cells = [
                    Cell::new(
                        "Resolved customer copy",
                        row.resolved_label,
                        Some(row.case_id),
                    )
                    .emphasized()
                    .secondary_mono(),
                    Cell::new("Subject / stage", &subject, None),
                    Cell::colored(
                        "Currentness",
                        row.state.as_str(),
                        Some(if row.label_allowed {
                            "label permitted by this design fixture"
                        } else {
                            "protected label not rendered as product truth"
                        }),
                        readiness_color(ui, row.state),
                    )
                    .emphasized(),
                    Cell::new("Applicable source", row.applicable_source_summary, None),
                    Cell::new("Reason codes", &reasons, None),
                ];
                matrix_data_row(ui, &cells, None);
            }
        },
    );
    matrix_disclaimer(
        ui,
        "Contract-test-vector authority is intentionally excluded from this customer projection. It can prove resolver behavior, never product readiness.",
    );
}

fn render_platforms(ui: &mut Ui, pending_action: &mut Option<MatrixAction>) {
    settings_section_label(ui, "Platform task and execution contract");
    matrix_table(
        ui,
        "capability-platform-table",
        960.0,
        &[
            "Platform",
            "Design capability contract",
            "Fixture currentness",
            "Qualification boundary",
            "Sign-off rule",
        ],
        |ui| {
            for row in data::PLATFORM_AVAILABILITY_ROWS {
                let cells = [
                    Cell::new("Platform", row.label, Some(row.id))
                        .emphasized()
                        .secondary_mono(),
                    Cell::new(
                        "Design capability contract",
                        row.capability_mode_summary,
                        None,
                    ),
                    Cell::colored(
                        "Fixture currentness",
                        row.fixture_state.as_str(),
                        Some(row.fixture_qualification),
                        readiness_color(ui, row.fixture_state),
                    )
                    .emphasized(),
                    Cell::colored(
                        "Qualification boundary",
                        row.qualification_boundary,
                        None,
                        Tokens::get(ui.ctx()).color.warn,
                    ),
                    Cell::new("Sign-off rule", row.sign_off_rule, None),
                ];
                matrix_data_row(ui, &cells, None);
            }
        },
    );
    workflow_toolbar(ui, |ui| {
        if Button::new("Format interoperability…").show(ui).clicked() {
            *pending_action = Some(MatrixAction::OpenRoute(SurfaceRoute::capability_workflow(
                CapabilityWorkflowId::InteroperabilityMatrix,
            )));
        }
        if Button::new("Lifecycle behavior…").show(ui).clicked() {
            *pending_action = Some(MatrixAction::OpenRoute(SurfaceRoute::capability_workflow(
                CapabilityWorkflowId::PlatformLifecycle,
            )));
        }
    });
}

fn render_planned_workflows(ui: &mut Ui, pending_action: &mut Option<MatrixAction>) {
    settings_section_label(ui, "Planned and deferred workflows · GUI designs complete");
    info_banner(
        ui,
        "These routes expose implementation-ready interaction designs without advertising unavailable capability. Their intended operational menu locations remain hidden until the exact engine or producer, persistence, platform, entitlement, and qualification gates pass.",
    );
    matrix_table(
        ui,
        "capability-planned-table",
        960.0,
        &[
            "Workflow",
            "Canonical owner",
            "Intended entry point",
            "GUI design",
            "Capability",
            "",
        ],
        |ui| {
            for row in data::PLANNED_WORKFLOW_ROWS {
                let capability = format!("{} · execution unavailable", row.status.as_str());
                let accessible_action = format!("Inspect design: {}", row.label);
                let cells = [
                    Cell::new("Workflow", row.label, Some(row.group)).emphasized(),
                    Cell::new("Canonical owner", row.owner, None),
                    Cell::new("Intended entry point", row.entry, None),
                    Cell::colored(
                        "GUI design",
                        "purpose-built GUI specified",
                        None,
                        Tokens::get(ui.ctx()).color.ok,
                    ),
                    Cell::colored(
                        "Capability",
                        &capability,
                        None,
                        Tokens::get(ui.ctx()).color.warn,
                    ),
                ];
                if matrix_data_row(
                    ui,
                    &cells,
                    Some(MatrixRowAction::new("Inspect design", &accessible_action)),
                ) {
                    *pending_action = Some(MatrixAction::OpenRoute(
                        SurfaceRoute::capability_workflow(row.workflow),
                    ));
                }
            }
        },
    );
}

fn render_analyses(ui: &mut Ui) {
    settings_section_label(ui, "Analysis catalog");
    matrix_table(
        ui,
        "capability-analysis-table",
        760.0,
        &["Code", "Analysis", "Intended tier", "Evidence requirement"],
        |ui| {
            for row in data::ANALYSIS_AVAILABILITY_ROWS {
                matrix_data_row(
                    ui,
                    &[
                        Cell::new("Code", row.code, None),
                        Cell::new("Analysis", row.title, None),
                        Cell::new("Intended tier", row.intended_tier.as_str(), None),
                        Cell::new("Evidence requirement", row.evidence_requirement(), None),
                    ],
                    None,
                );
            }
        },
    );
}

fn render_workspaces(ui: &mut Ui, engineering_profile: EngineeringProfile) {
    settings_section_label(ui, "Owned engineering workspaces");
    matrix_table(
        ui,
        "capability-workspace-table",
        1120.0,
        &[
            "Workspace",
            "Canonical owner",
            "Profile",
            "Interaction evidence",
            "Engine / service",
            "Evidence role",
        ],
        |ui| {
            for row in data::SPECIALIST_WORKSPACE_ROWS {
                let availability = row.runtime_availability();
                let profile = if row.shown_in_profile(engineering_profile) {
                    "shown"
                } else {
                    "profile hidden"
                };
                let interaction_evidence = if availability.can_open() {
                    "registered · production route executor available"
                } else {
                    "specification registered · operational route unavailable"
                };
                let cells = [
                    Cell::new("Workspace", row.label(), None),
                    Cell::new("Canonical owner", row.owner.owner_label(), None),
                    Cell::new("Profile", profile, None),
                    Cell::new("Interaction evidence", interaction_evidence, None),
                    Cell::colored(
                        "Engine / service",
                        row.engine_service_boundary(),
                        None,
                        Tokens::get(ui.ctx()).color.warn,
                    ),
                    Cell::new("Evidence role", row.evidence_role, None),
                ];
                matrix_data_row(ui, &cells, None);
            }
        },
    );
}

fn settings_section_label(ui: &mut Ui, title: &str) {
    let t = Tokens::get(ui.ctx());
    // In the governed workflow markup these labels are direct workflow-body
    // children, not descendants of `.settings-content`; the mockup therefore
    // renders them as ordinary body labels without the Preferences-only inset.
    ui.label(
        RichText::new(title)
            .font(theme::sans(tokens::FS_2, FontWeight::Regular))
            .color(t.color.text),
    );
}

fn matrix_table(
    ui: &mut Ui,
    id: &'static str,
    _minimum_width: f32,
    columns: &[&str],
    body: impl FnOnce(&mut Ui),
) {
    // `.data-table` is a fixed-layout, 100%-wide table in the mockup. Its
    // columns contract with the workflow rather than creating independent
    // horizontal scrollers at arbitrary Rust-only widths.
    let table = ui
        .push_id(id, |ui| {
            ui.scope(|ui| {
                ui.set_width(ui.available_width());
                let header_rect = table_header(ui, columns);
                body(ui);
                let table_rect = Rect::from_min_max(
                    header_rect.min,
                    pos2(
                        header_rect.right(),
                        ui.cursor().top().max(header_rect.bottom()),
                    ),
                );
                paint_sticky_table_header(ui, table_rect, header_rect, columns);
            })
        })
        .inner;
    ui.ctx().accesskit_node_builder(table.response.id, |node| {
        node.set_role(egui::accesskit::Role::Table);
        node.set_label(id);
    });
}

#[derive(Debug, Clone, Copy)]
struct MatrixRowAction<'a> {
    label: &'a str,
    accessible_label: &'a str,
}

impl<'a> MatrixRowAction<'a> {
    const fn new(label: &'a str, accessible_label: &'a str) -> Self {
        Self {
            label,
            accessible_label,
        }
    }
}

fn matrix_data_row(ui: &mut Ui, cells: &[Cell<'_>], action: Option<MatrixRowAction<'_>>) -> bool {
    let t = Tokens::get(ui.ctx());
    let mut action_clicked = false;
    let row_height = action
        .map(|_| (t.metrics.ctl_h + 8.0).max(t.metrics.row_h))
        .unwrap_or(t.metrics.row_h);
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), row_height), Sense::hover());

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

        let column_count = cells.len() + usize::from(action.is_some());
        let column_width = rect.width() / column_count.max(1) as f32;
        let mut x = rect.left();
        for (index, cell) in cells.iter().enumerate() {
            let right = if index + 1 == column_count {
                rect.right()
            } else {
                (x + column_width).min(rect.right())
            };
            render_matrix_cell(
                ui,
                Rect::from_min_max(pos2(x, rect.top()), pos2(right, rect.bottom())),
                cell,
                response.hovered(),
            );
            x = right;
        }
        if let Some(action) = action {
            let action_rect = Rect::from_min_max(
                pos2(x.min(rect.right()), rect.top()),
                pos2(rect.right(), rect.bottom()),
            );
            let content_rect = action_rect.shrink2(vec2(8.0, 4.0));
            let mut action_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(content_rect)
                    .layout(Layout::left_to_right(Align::Center)),
            );
            action_ui.set_clip_rect(action_rect.intersect(ui.clip_rect()));
            let button_response = Button::new(action.label)
                .ghost()
                .max_width(content_rect.width())
                .accessible_label(action.accessible_label)
                .show(&mut action_ui);
            action_clicked = button_response.clicked();
        }
    }

    let accessible_label = cells
        .iter()
        .map(|cell| format!("{}: {}", cell.label, cell.primary))
        .collect::<Vec<_>>()
        .join("; ");
    response
        .widget_info(|| WidgetInfo::labeled(WidgetType::Label, ui.is_enabled(), &accessible_label));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Row);
        node.set_label(accessible_label.clone());
    });
    action_clicked
}

fn render_matrix_cell(ui: &mut Ui, rect: Rect, cell: &Cell<'_>, row_hovered: bool) {
    let t = Tokens::get(ui.ctx());
    let base_color = cell.color.unwrap_or(if row_hovered {
        t.color.text
    } else {
        t.color.text_dim
    });
    let mut job = egui::text::LayoutJob::default();
    job.append(
        cell.primary,
        0.0,
        egui::TextFormat {
            font_id: theme::sans(
                tokens::FS_0,
                if cell.emphasized {
                    FontWeight::SemiBold
                } else {
                    FontWeight::Regular
                },
            ),
            color: base_color,
            ..Default::default()
        },
    );
    if let Some(secondary) = cell.secondary {
        job.append(
            " ",
            0.0,
            egui::TextFormat {
                font_id: theme::sans(9.0, FontWeight::Regular),
                color: base_color,
                ..Default::default()
            },
        );
        job.append(
            secondary,
            0.0,
            egui::TextFormat {
                font_id: if cell.secondary_mono {
                    theme::mono(9.0, FontWeight::Regular)
                } else {
                    theme::sans(9.0, FontWeight::Regular)
                },
                color: base_color,
                ..Default::default()
            },
        );
    }

    let content_rect = rect.shrink2(vec2(8.0, 0.0));
    let mut cell_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(content_rect)
            .layout(Layout::left_to_right(Align::Center)),
    );
    cell_ui.set_clip_rect(rect.intersect(ui.clip_rect()));
    let response = cell_ui.add_sized(
        vec2(content_rect.width(), content_rect.height()),
        Label::new(job).truncate().selectable(true),
    );
    cell_ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Cell);
    });
}

struct Cell<'a> {
    label: &'a str,
    primary: &'a str,
    secondary: Option<&'a str>,
    color: Option<Color32>,
    emphasized: bool,
    secondary_mono: bool,
}

impl<'a> Cell<'a> {
    const fn new(label: &'a str, primary: &'a str, secondary: Option<&'a str>) -> Self {
        Self {
            label,
            primary,
            secondary,
            color: None,
            emphasized: false,
            secondary_mono: false,
        }
    }

    const fn colored(
        label: &'a str,
        primary: &'a str,
        secondary: Option<&'a str>,
        color: Color32,
    ) -> Self {
        Self {
            label,
            primary,
            secondary,
            color: Some(color),
            emphasized: false,
            secondary_mono: false,
        }
    }

    const fn emphasized(mut self) -> Self {
        self.emphasized = true;
        self
    }

    const fn secondary_mono(mut self) -> Self {
        self.secondary_mono = true;
        self
    }
}

fn table_header(ui: &mut Ui, columns: &[&str]) -> Rect {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), 27.0), Sense::hover());
    response.widget_info(|| {
        WidgetInfo::labeled(WidgetType::Label, ui.is_enabled(), columns.join("; "))
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Row);
    });
    if !ui.is_rect_visible(rect) {
        return rect;
    }
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );

    let column_width = rect.width() / columns.len().max(1) as f32;
    let mut x = rect.left();
    for (index, label) in columns.iter().enumerate() {
        let right = if index + 1 == columns.len() {
            rect.right()
        } else {
            (x + column_width).min(rect.right())
        };
        let cell_rect = Rect::from_min_max(pos2(x, rect.top()), pos2(right, rect.bottom()));
        let content_rect = cell_rect.shrink2(vec2(8.0, 0.0));
        let mut cell_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(content_rect)
                .layout(Layout::left_to_right(Align::Center)),
        );
        cell_ui.set_clip_rect(cell_rect.intersect(ui.clip_rect()));
        let mut job = egui::text::LayoutJob::default();
        job.append(
            &label.to_uppercase(),
            0.0,
            egui::TextFormat {
                font_id: theme::sans(tokens::FS_0, FontWeight::Medium),
                color: t.color.text_faint,
                extra_letter_spacing: 0.04 * tokens::FS_0,
                ..Default::default()
            },
        );
        let response = cell_ui.add_sized(
            vec2(content_rect.width(), content_rect.height()),
            Label::new(job).truncate().selectable(true),
        );
        cell_ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_role(egui::accesskit::Role::ColumnHeader);
        });
        x = right;
    }
    rect
}

fn paint_sticky_table_header(ui: &Ui, table_rect: Rect, header_rect: Rect, columns: &[&str]) {
    let clip = ui.clip_rect();
    let sticky_top = clip
        .top()
        .max(header_rect.top())
        .min((table_rect.bottom() - header_rect.height()).max(header_rect.top()));
    if sticky_top <= header_rect.top() + 0.5 || sticky_top >= table_rect.bottom() {
        return;
    }

    let rect = Rect::from_min_size(pos2(header_rect.left(), sticky_top), header_rect.size());
    let t = Tokens::get(ui.ctx());
    let painter = ui.painter().with_clip_rect(rect.intersect(clip));
    painter.rect_filled(rect, 0.0, t.color.bg_panel_2);
    painter.hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let column_width = rect.width() / columns.len().max(1) as f32;
    for (index, label) in columns.iter().enumerate() {
        let left = rect.left() + column_width * index as f32;
        let right = if index + 1 == columns.len() {
            rect.right()
        } else {
            left + column_width
        };
        let mut job = egui::text::LayoutJob::default();
        job.append(
            &label.to_uppercase(),
            0.0,
            egui::TextFormat {
                font_id: theme::sans(tokens::FS_0, FontWeight::Medium),
                color: t.color.text_faint,
                extra_letter_spacing: 0.04 * tokens::FS_0,
                ..Default::default()
            },
        );
        job.wrap.max_width = (right - left - 16.0).max(1.0);
        job.wrap.max_rows = 1;
        job.wrap.break_anywhere = true;
        let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
        painter
            .with_clip_rect(Rect::from_min_max(
                pos2(left + 8.0, rect.top()),
                pos2((right - 8.0).max(left + 8.0), rect.bottom()),
            ))
            .galley(
                pos2(left + 8.0, rect.center().y - galley.size().y * 0.5),
                galley,
                t.color.text_faint,
            );
    }
}

fn planned_warning_banner(ui: &mut Ui, title: &str, body: &str) {
    planned_banner(ui, Some(title), body, true);
}

fn planned_info_banner(ui: &mut Ui, body: &str) {
    planned_banner(ui, None, body, false);
}

fn planned_banner(ui: &mut Ui, title: Option<&str>, body: &str, warning: bool) {
    let t = Tokens::get(ui.ctx());
    let (fill, text, border) = if warning {
        let alpha = if t.mode == tokens::Mode::Dark { 28 } else { 26 };
        (
            Color32::from_rgba_unmultiplied(
                t.color.warn.r(),
                t.color.warn.g(),
                t.color.warn.b(),
                alpha,
            ),
            t.color.warn,
            Color32::from_rgba_unmultiplied(
                t.color.warn.r(),
                t.color.warn.g(),
                t.color.warn.b(),
                140,
            ),
        )
    } else {
        (t.color.bg_inset, t.color.text_dim, t.color.border_strong)
    };
    Frame::NONE.inner_margin(Margin::same(8)).show(ui, |ui| {
        let response = Frame::NONE
            .fill(fill)
            .corner_radius(t.radius)
            .inner_margin(Margin::same(8))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing.x = 7.0;
                    if let Some(title) = title {
                        ui.add(
                            Label::new(
                                RichText::new(title)
                                    .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                                    .color(text),
                            )
                            .selectable(true),
                        );
                    }
                    ui.add(
                        Label::new(
                            RichText::new(body)
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(text),
                        )
                        .wrap()
                        .selectable(true),
                    );
                });
            })
            .response;
        paint_dashed_rect(ui, response.rect, Stroke::new(1.0, border));
    });
}

fn paint_dashed_rect(ui: &Ui, rect: Rect, stroke: Stroke) {
    for points in [
        [rect.left_top(), rect.right_top()],
        [rect.right_top(), rect.right_bottom()],
        [rect.right_bottom(), rect.left_bottom()],
        [rect.left_bottom(), rect.left_top()],
    ] {
        ui.painter()
            .extend(egui::Shape::dashed_line(&points, stroke, 3.0, 3.0));
    }
}

fn warning_banner(ui: &mut Ui, title: &str, body: &str) {
    let t = Tokens::get(ui.ctx());
    let alpha = if t.mode == tokens::Mode::Dark { 28 } else { 26 };
    let fill = Color32::from_rgba_unmultiplied(
        t.color.warn.r(),
        t.color.warn.g(),
        t.color.warn.b(),
        alpha,
    );
    let response = Frame::new()
        .fill(fill)
        .inner_margin(Margin::same(8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 7.0;
                ui.label(
                    RichText::new(title)
                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                        .color(t.color.warn),
                );
                ui.add(
                    Label::new(
                        RichText::new(body)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.warn),
                    )
                    .wrap(),
                );
            });
        });
    paint_dashed_bottom_border(
        ui,
        response.response.rect,
        Stroke::new(
            1.0,
            Color32::from_rgba_unmultiplied(
                t.color.warn.r(),
                t.color.warn.g(),
                t.color.warn.b(),
                140,
            ),
        ),
    );
}

fn active_profile_banner(ui: &mut Ui, profile: &str) {
    let t = Tokens::get(ui.ctx());
    let alpha = if t.mode == tokens::Mode::Dark { 28 } else { 26 };
    let fill = Color32::from_rgba_unmultiplied(
        t.color.warn.r(),
        t.color.warn.g(),
        t.color.warn.b(),
        alpha,
    );
    let response = Frame::new()
        .fill(fill)
        .inner_margin(Margin::same(8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 7.0;
                let regular = theme::sans(tokens::FS_0, FontWeight::Regular);
                ui.label(
                    RichText::new("Active profile ·")
                        .font(regular.clone())
                        .color(t.color.warn),
                );
                ui.label(
                    RichText::new(profile)
                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                        .color(t.color.warn),
                );
                ui.add(
                    Label::new(
                        RichText::new(". Profiles change everyday navigation only. The matrix separates intended scope from current implementation evidence; no row is a release claim without a bound qualification record.")
                            .font(regular)
                            .color(t.color.warn),
                    )
                    .wrap(),
                );
            });
        });
    paint_dashed_bottom_border(
        ui,
        response.response.rect,
        Stroke::new(
            1.0,
            Color32::from_rgba_unmultiplied(
                t.color.warn.r(),
                t.color.warn.g(),
                t.color.warn.b(),
                140,
            ),
        ),
    );
}

fn info_banner(ui: &mut Ui, body: &str) {
    let t = Tokens::get(ui.ctx());
    let response = Frame::new()
        .fill(t.color.bg_inset)
        .inner_margin(Margin::same(8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.add(
                Label::new(
                    RichText::new(body)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                )
                .wrap(),
            );
        });
    paint_dashed_bottom_border(
        ui,
        response.response.rect,
        Stroke::new(1.0, t.color.border_strong),
    );
}

fn paint_dashed_bottom_border(ui: &Ui, rect: egui::Rect, stroke: Stroke) {
    let points = [rect.left_bottom(), rect.right_bottom()];
    ui.painter()
        .extend(egui::Shape::dashed_line(&points, stroke, 3.0, 3.0));
}

fn summary_stats(ui: &mut Ui, total: usize, permitted: usize, blocked: usize, as_of: &str) {
    let t = Tokens::get(ui.ctx());
    let compact = ui.available_width() <= 820.0 || ui.ctx().input(|input| input.has_touch_screen());
    let stats = [
        SummaryStat::plain("Product claim vectors", total.to_string()),
        SummaryStat::colored(
            "Permitted labels",
            permitted.to_string(),
            if permitted > 0 {
                t.color.ok
            } else {
                t.color.warn
            },
        ),
        SummaryStat::colored(
            "Blocked labels",
            blocked.to_string(),
            if blocked > 0 {
                t.color.warn
            } else {
                t.color.ok
            },
        ),
        SummaryStat::monospace("As-of", as_of.to_owned()),
    ];
    // The source design declares five desktop tracks for this four-value
    // summary. Preserve that deliberate empty fifth track; mobile reflows to
    // two columns exactly as the mockup does.
    let column_count = if compact { 2 } else { 5 };
    Frame::new()
        .fill(t.color.border)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(t.radius_lg)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.spacing_mut().item_spacing.y = 1.0;
            for row in stats.chunks(column_count) {
                let width = (ui.available_width() - (column_count.saturating_sub(1) as f32))
                    .max(1.0)
                    / column_count as f32;
                ui.horizontal_top(|ui| {
                    ui.spacing_mut().item_spacing.x = 1.0;
                    for stat in row {
                        summary_stat(ui, stat, width, &t);
                    }
                });
            }
        });
    ui.add_space(12.0);
}

struct SummaryStat {
    label: &'static str,
    value: String,
    color: Option<Color32>,
    monospace: bool,
}

impl SummaryStat {
    fn plain(label: &'static str, value: String) -> Self {
        Self {
            label,
            value,
            color: None,
            monospace: false,
        }
    }

    fn colored(label: &'static str, value: String, color: Color32) -> Self {
        Self {
            label,
            value,
            color: Some(color),
            monospace: false,
        }
    }

    fn monospace(label: &'static str, value: String) -> Self {
        Self {
            label,
            value,
            color: None,
            monospace: true,
        }
    }
}

fn summary_stat(ui: &mut Ui, stat: &SummaryStat, width: f32, t: &Tokens) {
    Frame::new()
        .fill(t.color.bg_panel_2)
        .inner_margin(Margin::symmetric(12, 10))
        .show(ui, |ui| {
            ui.set_width((width - 24.0).max(1.0));
            let mut label_job = egui::text::LayoutJob::default();
            label_job.append(
                &stat.label.to_uppercase(),
                0.0,
                egui::TextFormat {
                    font_id: theme::sans(tokens::FS_0, FontWeight::Regular),
                    color: t.color.text_dim,
                    extra_letter_spacing: 0.06 * tokens::FS_0,
                    ..Default::default()
                },
            );
            ui.add(Label::new(label_job).truncate());
            ui.add_space(3.0);
            ui.add(
                Label::new(
                    RichText::new(&stat.value)
                        .font(if stat.monospace {
                            theme::mono(tokens::FS_2, FontWeight::SemiBold)
                        } else {
                            theme::sans(tokens::FS_2, FontWeight::SemiBold)
                        })
                        .color(stat.color.unwrap_or(t.color.text)),
                )
                .truncate(),
            );
        });
}

fn matrix_disclaimer(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.add(
        Label::new(
            RichText::new(text)
                .font(theme::sans(tokens::FS_2, FontWeight::Regular))
                .color(t.color.text_dim),
        )
        .wrap()
        .selectable(true),
    );
}

fn readiness_color(ui: &Ui, state: data::ReadinessState) -> Color32 {
    let color = Tokens::get(ui.ctx()).color;
    match state {
        data::ReadinessState::Current => color.ok,
        data::ReadinessState::Stale => color.warn,
        data::ReadinessState::Unavailable => color.err,
        data::ReadinessState::Unknown => color.text_dim,
    }
}

fn render_interoperability(ui: &mut Ui, matrix: &mut CapabilityMatrixState) {
    ui.spacing_mut().item_spacing.y = 0.0;
    let tabs = workflow_tabs(
        ui,
        data::INTEROPERABILITY_TABLIST_LABEL,
        &mut matrix.interoperability_section,
    );

    let panel = ui.scope(|ui| match matrix.interoperability_section {
        InteroperabilitySection::FormatMatrix => render_interoperability_format_matrix(ui, matrix),
        InteroperabilitySection::RoundTripContract => render_interoperability_round_trip(ui),
        InteroperabilitySection::Qualification => render_interoperability_qualification(ui),
    });
    for tab_id in tabs.tab_ids {
        ui.ctx().accesskit_node_builder(tab_id, |node| {
            node.push_controlled(panel.response.id.accesskit_id());
        });
    }
    ui.ctx().accesskit_node_builder(panel.response.id, |node| {
        node.set_role(egui::accesskit::Role::TabPanel);
        node.set_label(matrix.interoperability_section.label());
        node.push_labelled_by(tabs.active_tab_id.accesskit_id());
    });
}

fn render_interoperability_format_matrix(ui: &mut Ui, matrix: &mut CapabilityMatrixState) {
    workflow_toolbar(ui, |ui| {
        let domain_options = InteroperabilityDomain::ALL
            .iter()
            .map(|domain| domain.label().to_owned())
            .collect::<Vec<_>>();
        if let Some(index) = select(
            ui,
            "interoperability-domain",
            "Interoperability domain",
            matrix.interoperability_domain.label(),
            &domain_options,
            180.0,
        ) {
            matrix.interoperability_domain = InteroperabilityDomain::ALL[index];
        }

        let support_options = InteroperabilitySupportLevel::ALL
            .iter()
            .map(|support| support.label().to_owned())
            .collect::<Vec<_>>();
        if let Some(index) = select(
            ui,
            "interoperability-support-level",
            "Interoperability support level",
            matrix.interoperability_support_level.label(),
            &support_options,
            180.0,
        ) {
            matrix.interoperability_support_level = InteroperabilitySupportLevel::ALL[index];
        }
    });

    contract_table(
        ui,
        "interoperability-format-table",
        &[
            "Domain / format",
            "Version or dialect",
            "Direction",
            "Release contract",
            "Round-trip / loss policy",
        ],
        |ui| {
            let mut matched = false;
            for row in data::INTEROPERABILITY_FORMAT_ROWS
                .into_iter()
                .filter(|row| {
                    row.matches(
                        matrix.interoperability_domain,
                        matrix.interoperability_support_level,
                    )
                })
            {
                matched = true;
                matrix_data_row(
                    ui,
                    &[
                        Cell::new("Domain / format", row.domain_format, None),
                        Cell::new("Version or dialect", row.version_dialect, None),
                        Cell::new("Direction", row.direction, None),
                        contract_cell(
                            ui,
                            "Release contract",
                            row.release_contract,
                            row.release_tone,
                        ),
                        Cell::new("Round-trip / loss policy", row.round_trip_loss_policy, None),
                    ],
                    None,
                );
            }
            if !matched {
                matrix_data_row(
                    ui,
                    &[Cell::new(
                        "Filter result",
                        INTEROPERABILITY_EMPTY_FILTER_COPY,
                        None,
                    )],
                    None,
                );
            }
        },
    );
}

const INTEROPERABILITY_EMPTY_FILTER_COPY: &str =
    "No format contracts match the selected domain and support level.";

fn render_interoperability_round_trip(ui: &mut Ui) {
    contract_note_grid(ui, &data::INTEROPERABILITY_ROUND_TRIP_NOTES, false, None);
    contract_table(
        ui,
        "interoperability-round-trip-table",
        &["Gate", "Evidence", "Failure behavior"],
        |ui| {
            for row in data::INTEROPERABILITY_ROUND_TRIP_GATES {
                matrix_data_row(
                    ui,
                    &[
                        Cell::new("Gate", row.gate, None),
                        Cell::new("Evidence", row.evidence, None),
                        Cell::new("Failure behavior", row.failure_behavior, None),
                    ],
                    None,
                );
            }
        },
    );
}

fn render_interoperability_qualification(ui: &mut Ui) {
    contract_table(
        ui,
        "interoperability-qualification-table",
        &[
            "Profile",
            "Golden corpus",
            "Required comparison",
            "Platform gate",
            "Release state",
        ],
        |ui| {
            for row in data::INTEROPERABILITY_QUALIFICATION_ROWS {
                matrix_data_row(
                    ui,
                    &[
                        Cell::new("Profile", row.profile, None),
                        Cell::new("Golden corpus", row.golden_corpus, None),
                        Cell::new("Required comparison", row.required_comparison, None),
                        Cell::new("Platform gate", row.platform_gate, None),
                        contract_cell(ui, "Release state", row.release_state, row.release_tone),
                    ],
                    None,
                );
            }
        },
    );
    planned_banner(
        ui,
        None,
        data::INTEROPERABILITY_QUALIFICATION_BOUNDARY,
        true,
    );
}

fn render_platform_lifecycle(ui: &mut Ui, pending_action: &mut Option<MatrixAction>) {
    ui.spacing_mut().item_spacing.y = 0.0;
    planned_banner(ui, None, data::PLATFORM_LIFECYCLE_WARNING, true);
    contract_table(
        ui,
        "platform-lifecycle-table",
        &[
            "Platform event",
            "Protected state",
            "User-visible response",
            "Recovery",
            "Release eligibility",
        ],
        |ui| {
            for row in data::PLATFORM_LIFECYCLE_ROWS {
                matrix_data_row(
                    ui,
                    &[
                        Cell::new("Platform event", row.platform_event, None),
                        Cell::new("Protected state", row.protected_state, None),
                        Cell::new("User-visible response", row.user_visible_response, None),
                        Cell::new("Recovery", row.recovery, None),
                        contract_cell(
                            ui,
                            "Release eligibility",
                            row.release_eligibility,
                            row.eligibility_tone,
                        ),
                    ],
                    None,
                );
            }
        },
    );
    if contract_note_grid(
        ui,
        &data::PLATFORM_LIFECYCLE_NOTES,
        true,
        Some(ContractNoteAction {
            note_index: 0,
            label: data::TOUCH_EDIT_GUIDE_ACTION_LABEL,
        }),
    ) {
        *pending_action = Some(MatrixAction::OpenRoute(SurfaceRoute::capability_workflow(
            CapabilityWorkflowId::TouchEditGuide,
        )));
    }
}

fn render_touch_edit_guide(ui: &mut Ui) {
    ui.spacing_mut().item_spacing.y = 0.0;
    let columns = touch_guide_column_count(ui.available_width());
    let guide = ui.scope(|ui| {
        if columns == 2 {
            for row in data::TOUCH_EDIT_GUIDE_STEPS.chunks(2) {
                ui.horizontal_top(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    let width = ui.available_width() * 0.5;
                    for (column_index, step) in row.iter().enumerate() {
                        touch_guide_step(ui, step, width, column_index == 0);
                    }
                });
            }
        } else {
            for step in data::TOUCH_EDIT_GUIDE_STEPS {
                touch_guide_step(ui, &step, ui.available_width(), false);
            }
        }
    });
    ui.ctx().accesskit_node_builder(guide.response.id, |node| {
        node.set_role(egui::accesskit::Role::List);
        node.set_label("Touch schematic editing steps");
    });

    let concept = touch_guide_concept_banner(ui);
    ui.ctx().accesskit_node_builder(concept.id, |node| {
        node.set_role(egui::accesskit::Role::Note);
        node.set_label(data::TOUCH_EDIT_GUIDE_CONCEPT);
    });
}

const TOUCH_GUIDE_BREAKPOINT: f32 = 620.0;
const TOUCH_GUIDE_SECTION_MIN_HEIGHT: f32 = 96.0;
const TOUCH_GUIDE_SECTION_PADDING: f32 = 13.0;
const TOUCH_GUIDE_MARKER_COLUMN_WIDTH: f32 = 32.0;
const TOUCH_GUIDE_MARKER_DIAMETER: f32 = 30.0;

fn touch_guide_column_count(available_width: f32) -> usize {
    if available_width <= TOUCH_GUIDE_BREAKPOINT {
        1
    } else {
        2
    }
}

fn touch_guide_step(
    ui: &mut Ui,
    step: &data::TouchEditGuideStep,
    width: f32,
    right_border: bool,
) -> egui::InnerResponse<()> {
    let t = Tokens::get(ui.ctx());
    ui.allocate_ui_with_layout(Vec2::new(width, 0.0), Layout::top_down(Align::Min), |ui| {
        let section = Frame::NONE
            .inner_margin(Margin::same(TOUCH_GUIDE_SECTION_PADDING as i8))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.set_min_height(
                    TOUCH_GUIDE_SECTION_MIN_HEIGHT - 2.0 * TOUCH_GUIDE_SECTION_PADDING,
                );
                ui.horizontal_top(|ui| {
                    ui.spacing_mut().item_spacing.x = 10.0;
                    touch_guide_marker(ui, step.number);
                    ui.allocate_ui_with_layout(
                        Vec2::new(ui.available_width(), 0.0),
                        Layout::top_down(Align::Min),
                        |ui| {
                            ui.spacing_mut().item_spacing.y = 0.0;
                            ui.add(
                                Label::new(
                                    RichText::new(step.title)
                                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                                        .color(t.color.text),
                                )
                                .wrap()
                                .selectable(true),
                            );
                            ui.add_space(5.0);
                            ui.add(planned_wrapped_label(
                                step.body,
                                t.color.text_dim,
                                1.45 * tokens::FS_0,
                            ));
                        },
                    );
                });
            });
        let rect = section.response.rect;
        ui.painter().hline(
            rect.x_range(),
            rect.bottom(),
            Stroke::new(1.0, t.color.border),
        );
        if right_border {
            ui.painter().vline(
                rect.right(),
                rect.y_range(),
                Stroke::new(1.0, t.color.border),
            );
        }
        ui.ctx()
            .accesskit_node_builder(section.response.id, |node| {
                node.set_role(egui::accesskit::Role::ListItem);
                node.set_label(format!(
                    "Step {}. {}. {}",
                    step.number, step.title, step.body
                ));
            });
    })
}

fn touch_guide_marker(ui: &mut Ui, number: u8) {
    let t = Tokens::get(ui.ctx());
    let (column_rect, response) = ui.allocate_exact_size(
        vec2(TOUCH_GUIDE_MARKER_COLUMN_WIDTH, TOUCH_GUIDE_MARKER_DIAMETER),
        Sense::hover(),
    );
    let marker_rect = Rect::from_min_size(
        column_rect.min,
        vec2(TOUCH_GUIDE_MARKER_DIAMETER, TOUCH_GUIDE_MARKER_DIAMETER),
    );
    if ui.is_rect_visible(marker_rect) {
        let border_alpha = if t.mode == tokens::Mode::Dark {
            110
        } else {
            97
        };
        let border = Color32::from_rgba_unmultiplied(
            t.color.accent.r(),
            t.color.accent.g(),
            t.color.accent.b(),
            border_alpha,
        );
        ui.painter().circle_filled(
            marker_rect.center(),
            TOUCH_GUIDE_MARKER_DIAMETER * 0.5,
            t.color.accent_dim,
        );
        ui.painter().circle_stroke(
            marker_rect.center(),
            (TOUCH_GUIDE_MARKER_DIAMETER - 1.0) * 0.5,
            Stroke::new(1.0, border),
        );
        ui.painter().text(
            marker_rect.center(),
            egui::Align2::CENTER_CENTER,
            number,
            theme::mono(tokens::FS_1, FontWeight::SemiBold),
            t.color.accent,
        );
    }
    response.widget_info(|| {
        WidgetInfo::labeled(WidgetType::Label, ui.is_enabled(), number.to_string())
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::ListMarker);
        node.set_label(number.to_string());
    });
}

fn touch_guide_concept_banner(ui: &mut Ui) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let response = Frame::NONE
        .fill(t.color.bg_inset)
        .inner_margin(Margin::same(8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.add(
                Label::new(
                    RichText::new(data::TOUCH_EDIT_GUIDE_CONCEPT)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                )
                .wrap()
                .selectable(true),
            );
        })
        .response;
    paint_dashed_bottom_border(ui, response.rect, Stroke::new(1.0, t.color.border_strong));
    response
}

struct WorkflowTabsA11y {
    tab_ids: Vec<egui::Id>,
    active_tab_id: egui::Id,
}

fn workflow_tabs(
    ui: &mut Ui,
    label: &'static str,
    active: &mut InteroperabilitySection,
) -> WorkflowTabsA11y {
    let t = Tokens::get(ui.ctx());
    let tab_height = ui.spacing().interact_size.y.max(34.0);
    let mut requested = *active;
    let pending_focus_id = ui.make_persistent_id(("capability-workflow-tab-focus", label));
    let pending_focus = ui
        .ctx()
        .data_mut(|data| data.remove_temp::<InteroperabilitySection>(pending_focus_id));
    let mut next_frame_focus = None;
    let mut tab_ids = Vec::with_capacity(InteroperabilitySection::ALL.len());
    let response = Frame::NONE.fill(t.color.bg_panel).show(ui, |ui| {
        egui::ScrollArea::horizontal()
            .id_salt(("capability-workflow-tabs", label))
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    for (index, section) in InteroperabilitySection::ALL.into_iter().enumerate() {
                        let selected = *active == section;
                        let font = theme::sans(tokens::FS_0, FontWeight::Regular);
                        let text_width = ui.fonts_mut(|fonts| {
                            fonts
                                .layout_no_wrap(
                                    section.label().to_owned(),
                                    font.clone(),
                                    t.color.text,
                                )
                                .size()
                                .x
                        });
                        let (_, rect) =
                            ui.allocate_space(vec2((text_width + 24.0).max(112.0), tab_height));
                        let id = ui.make_persistent_id(("capability-workflow-tab", label, section));
                        tab_ids.push(id);
                        let sense = if selected {
                            Sense::click()
                        } else {
                            Sense::CLICK
                        };
                        let tab = ui.interact(rect, id, sense);
                        if selected && pending_focus == Some(section) {
                            tab.request_focus();
                        }
                        tab.widget_info(|| {
                            WidgetInfo::selected(
                                WidgetType::SelectableLabel,
                                ui.is_enabled(),
                                selected,
                                section.label(),
                            )
                        });
                        ui.ctx().accesskit_node_builder(tab.id, |node| {
                            node.set_role(egui::accesskit::Role::Tab);
                            node.set_selected(selected);
                        });
                        if selected {
                            ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
                            ui.painter().hline(
                                rect.x_range(),
                                rect.bottom() - 1.0,
                                Stroke::new(2.0, t.color.accent),
                            );
                        }
                        ui.painter().vline(
                            rect.right(),
                            rect.y_range(),
                            Stroke::new(1.0, t.color.border),
                        );
                        ui.painter().text(
                            pos2(rect.left() + 12.0, rect.center().y),
                            egui::Align2::LEFT_CENTER,
                            section.label(),
                            font,
                            if selected || tab.hovered() {
                                t.color.text
                            } else {
                                t.color.text_dim
                            },
                        );
                        theme::paint_focus_ring(ui, &tab, rect);
                        if tab.clicked() {
                            requested = section;
                            if !selected {
                                next_frame_focus = Some(section);
                            }
                        }
                        if tab.has_focus() {
                            let target = if ui.input_mut(|input| {
                                input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight)
                            }) {
                                Some((index + 1) % InteroperabilitySection::ALL.len())
                            } else if ui.input_mut(|input| {
                                input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft)
                            }) {
                                Some(
                                    (index + InteroperabilitySection::ALL.len() - 1)
                                        % InteroperabilitySection::ALL.len(),
                                )
                            } else if ui.input_mut(|input| {
                                input.consume_key(egui::Modifiers::NONE, egui::Key::Home)
                            }) {
                                Some(0)
                            } else if ui.input_mut(|input| {
                                input.consume_key(egui::Modifiers::NONE, egui::Key::End)
                            }) {
                                Some(InteroperabilitySection::ALL.len() - 1)
                            } else {
                                None
                            };
                            if let Some(target) = target {
                                requested = InteroperabilitySection::ALL[target];
                                next_frame_focus = Some(requested);
                            }
                        }
                    }
                });
            });
    });
    ui.painter().hline(
        response.response.rect.x_range(),
        response.response.rect.bottom(),
        Stroke::new(1.0, t.color.border_strong),
    );
    ui.ctx()
        .accesskit_node_builder(response.response.id, |node| {
            node.set_role(egui::accesskit::Role::TabList);
            node.set_label(label);
        });
    *active = requested;
    if let Some(section) = next_frame_focus {
        ui.ctx()
            .data_mut(|data| data.insert_temp(pending_focus_id, section));
        ui.ctx().request_repaint();
    }
    let active_index = InteroperabilitySection::ALL
        .iter()
        .position(|section| *section == *active)
        .expect("active interoperability section is catalogued");
    WorkflowTabsA11y {
        active_tab_id: tab_ids[active_index],
        tab_ids,
    }
}

fn workflow_toolbar(ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    let response = Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(10, 6))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.set_min_height(29.0);
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = vec2(6.0, 6.0);
                content(ui);
            });
        });
    ui.painter().hline(
        response.response.rect.x_range(),
        response.response.rect.bottom(),
        Stroke::new(1.0, t.color.border_strong),
    );
    ui.add_space(10.0);
}

fn contract_table(ui: &mut Ui, id: &'static str, columns: &[&str], body: impl FnOnce(&mut Ui)) {
    if ui.ctx().content_rect().width() <= 820.0 {
        egui::ScrollArea::horizontal()
            .id_salt(("capability-contract-table", id))
            .auto_shrink([false, true])
            .show(ui, |ui| {
                ui.set_min_width(660.0);
                matrix_table(ui, id, 660.0, columns, body);
            });
        return;
    }
    matrix_table(ui, id, 0.0, columns, body);
}

fn contract_cell<'a>(ui: &Ui, label: &'a str, text: &'a str, tone: data::ContractTone) -> Cell<'a> {
    let color = Tokens::get(ui.ctx()).color;
    match tone {
        data::ContractTone::Plain => Cell::new(label, text, None),
        data::ContractTone::Success => Cell::colored(label, text, None, color.ok),
        data::ContractTone::Warning => Cell::colored(label, text, None, color.warn),
        data::ContractTone::Error => Cell::colored(label, text, None, color.err),
    }
}

#[derive(Debug, Clone, Copy)]
struct ContractNoteAction {
    note_index: usize,
    label: &'static str,
}

fn contract_note_grid(
    ui: &mut Ui,
    notes: &[data::ContractNote],
    top_space: bool,
    action: Option<ContractNoteAction>,
) -> bool {
    if notes.is_empty() {
        return false;
    }
    if top_space {
        ui.add_space(10.0);
    }
    let t = Tokens::get(ui.ctx());
    let wide = ui.available_width() > 760.0;
    let mut horizontal_separators = Vec::new();
    let mut action_clicked = false;
    let response = Frame::NONE
        .stroke(Stroke::new(1.0, t.color.border_strong))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            if wide {
                let row_count = notes.len().div_ceil(2);
                for (row_index, row) in notes.chunks(2).enumerate() {
                    let row_response = ui.horizontal_top(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        let width = ui.available_width() * 0.5;
                        for (column_index, note) in row.iter().enumerate() {
                            let note_index = row_index * 2 + column_index;
                            contract_note(
                                ui,
                                note,
                                width,
                                action.filter(|action| action.note_index == note_index),
                                &mut action_clicked,
                            );
                        }
                    });
                    if row_index + 1 < row_count {
                        horizontal_separators.push(row_response.response.rect.bottom());
                    }
                }
            } else {
                for (index, note) in notes.iter().enumerate() {
                    let note_response = contract_note(
                        ui,
                        note,
                        ui.available_width(),
                        action.filter(|action| action.note_index == index),
                        &mut action_clicked,
                    );
                    if index + 1 < notes.len() {
                        horizontal_separators.push(note_response.response.rect.bottom());
                    }
                }
            }
        });
    let rect = response.response.rect;
    if wide {
        ui.painter().vline(
            rect.center().x,
            rect.y_range(),
            Stroke::new(1.0, t.color.border_strong),
        );
    }
    for y in horizontal_separators {
        ui.painter()
            .hline(rect.x_range(), y, Stroke::new(1.0, t.color.border_strong));
    }
    action_clicked
}

fn contract_note(
    ui: &mut Ui,
    note: &data::ContractNote,
    width: f32,
    action: Option<ContractNoteAction>,
    action_clicked: &mut bool,
) -> egui::InnerResponse<()> {
    let t = Tokens::get(ui.ctx());
    ui.allocate_ui_with_layout(Vec2::new(width, 0.0), Layout::top_down(Align::Min), |ui| {
        Frame::NONE.inner_margin(Margin::same(10)).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.add(
                Label::new(
                    RichText::new(note.title)
                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                        .color(t.color.text),
                )
                .selectable(true),
            );
            ui.add_space(4.0);
            ui.add(planned_wrapped_label(
                note.body,
                t.color.text_dim,
                1.45 * tokens::FS_0,
            ));
            if let Some(action) = action {
                *action_clicked |= Button::new(action.label).show(ui).clicked();
            }
        });
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn accesskit_nodes(
        mut add_contents: impl FnMut(&mut egui::Ui),
    ) -> Vec<(egui::accesskit::NodeId, egui::accesskit::Node)> {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| add_contents(ui));
        })
        .platform_output
        .accesskit_update
        .expect("AccessKit tree update")
        .nodes
    }

    #[test]
    fn touch_guide_reflows_at_the_mockup_breakpoint() {
        assert_eq!(TOUCH_GUIDE_SECTION_MIN_HEIGHT, 96.0);
        assert_eq!(TOUCH_GUIDE_SECTION_PADDING, 13.0);
        assert_eq!(TOUCH_GUIDE_MARKER_COLUMN_WIDTH, 32.0);
        assert_eq!(TOUCH_GUIDE_MARKER_DIAMETER, 30.0);
        assert_eq!(touch_guide_column_count(0.0), 1);
        assert_eq!(touch_guide_column_count(620.0), 1);
        assert_eq!(touch_guide_column_count(620.01), 2);
        assert_eq!(touch_guide_column_count(1_200.0), 2);
    }

    #[test]
    fn capability_document_descriptions_are_specific_and_read_only_truthful() {
        let planned = CapabilityMatrixDrilldown::PlannedWorkflow("source-load-pull".to_owned());
        let interoperability = CapabilityMatrixDrilldown::Interoperability;
        let lifecycle = CapabilityMatrixDrilldown::PlatformLifecycle;
        let touch = CapabilityMatrixDrilldown::TouchEditGuide;
        let descriptions = [
            workflow_description(None),
            workflow_description(Some(&planned)),
            workflow_description(Some(&interoperability)),
            workflow_description(Some(&lifecycle)),
            workflow_description(Some(&touch)),
        ];

        assert_eq!(
            descriptions
                .into_iter()
                .collect::<std::collections::HashSet<_>>()
                .len(),
            5
        );
        assert!(descriptions[0].contains("read-only capability disclosure"));
        assert!(descriptions[1].contains("unavailable capability design"));
        assert!(descriptions[2].contains("versioned import, export, round-trip"));
        assert!(descriptions[3].contains("suspension, storage, recovery"));
        assert!(descriptions[4].contains("exact-coordinate fallback"));
        assert!(descriptions.into_iter().all(|description| {
            !description.contains("transaction action") && !description.contains("mutation")
        }));
    }

    #[test]
    fn touch_guide_publishes_numbered_steps_and_precision_note() {
        let nodes = accesskit_nodes(render_touch_edit_guide);

        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::List
                && node.label() == Some("Touch schematic editing steps")
        }));
        let steps = nodes
            .iter()
            .filter(|(_, node)| node.role() == egui::accesskit::Role::ListItem)
            .collect::<Vec<_>>();
        assert_eq!(steps.len(), data::TOUCH_EDIT_GUIDE_STEPS.len());
        for step in data::TOUCH_EDIT_GUIDE_STEPS {
            let expected = format!("Step {}. {}. {}", step.number, step.title, step.body);
            assert!(
                steps
                    .iter()
                    .any(|(_, node)| node.label() == Some(&expected))
            );
        }
        assert_eq!(
            nodes
                .iter()
                .filter(|(_, node)| node.role() == egui::accesskit::Role::ListMarker)
                .count(),
            data::TOUCH_EDIT_GUIDE_STEPS.len()
        );
        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Note
                && node.label() == Some(data::TOUCH_EDIT_GUIDE_CONCEPT)
        }));
    }

    #[test]
    fn empty_interoperability_filter_is_an_accessible_state_not_a_capability_row() {
        let nodes = accesskit_nodes(|ui| {
            let mut matrix = CapabilityMatrixState {
                interoperability_support_level: InteroperabilitySupportLevel::Planned,
                ..CapabilityMatrixState::default()
            };
            render_interoperability_format_matrix(ui, &mut matrix);
        });

        let empty_rows = nodes
            .iter()
            .filter(|(_, node)| {
                node.role() == egui::accesskit::Role::Row
                    && node.label().is_some_and(|label| {
                        label.contains(INTEROPERABILITY_EMPTY_FILTER_COPY)
                            && label.contains("Filter result")
                    })
            })
            .count();
        assert_eq!(empty_rows, 1);
        assert!(!nodes.iter().any(|(_, node)| {
            node.label()
                .is_some_and(|label| label.contains("planned format contract"))
        }));
    }

    #[test]
    fn interoperability_tabs_have_one_tab_stop_and_explicit_panel_relationships() {
        let nodes = accesskit_nodes(|ui| {
            let mut matrix = CapabilityMatrixState::default();
            render_interoperability(ui, &mut matrix);
        });
        let tabs = nodes
            .iter()
            .filter(|(_, node)| node.role() == egui::accesskit::Role::Tab)
            .collect::<Vec<_>>();
        assert_eq!(tabs.len(), InteroperabilitySection::ALL.len());
        assert_eq!(
            tabs.iter()
                .filter(|(_, node)| node.supports_action(egui::accesskit::Action::Focus))
                .count(),
            1
        );

        let (panel_id, panel) = nodes
            .iter()
            .find(|(_, node)| node.role() == egui::accesskit::Role::TabPanel)
            .expect("active interoperability panel is exposed");
        for (_, tab) in &tabs {
            assert_eq!(tab.controls(), &[*panel_id]);
        }
        let (active_tab_id, _) = tabs
            .iter()
            .find(|(_, node)| node.is_selected() == Some(true))
            .expect("one interoperability tab is selected");
        assert_eq!(panel.labelled_by(), &[*active_tab_id]);
    }

    #[test]
    fn unavailable_specialist_workspaces_publish_evidence_without_fake_actions() {
        let nodes = accesskit_nodes(|ui| render_workspaces(ui, EngineeringProfile::All));

        assert!(
            !nodes
                .iter()
                .any(|(_, node)| node.role() == egui::accesskit::Role::Button)
        );
        assert_eq!(
            nodes
                .iter()
                .filter(|(_, node)| node.role() == egui::accesskit::Role::ColumnHeader)
                .count(),
            6
        );
        assert!(nodes.iter().any(|(_, node)| {
            node.label()
                .is_some_and(|label| label.contains("operational route unavailable"))
        }));
    }
}
