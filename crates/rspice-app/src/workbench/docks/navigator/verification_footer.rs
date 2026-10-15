//! Verification-flow summaries and status rendering for the navigator footer.

use super::*;

#[cfg(test)]
pub(super) fn active_mc_sample_trail(simulation: &crate::state::SimulationState) -> usize {
    simulation
        .yield_results_for_active_dataset()
        .unwrap_or(&[])
        .iter()
        .map(|result| result.trail.len())
        .max()
        .unwrap_or(0)
}

/// A flow row's height, and where its status line sits, for `detail_lines`
/// wrapped lines of detail and a status line or none.
///
/// The label, then the detail, then the status, each on the row's own pitch; a
/// line the flow has nothing to say on is not drawn and takes no height.
pub(super) fn flow_row_geometry(detail_lines: usize, has_status: bool) -> (f32, f32) {
    let detail_top = |line: usize| FLOW_DETAIL_TOP + line as f32 * FLOW_DETAIL_LINE_HEIGHT;
    let status_top = match detail_lines {
        0 => FLOW_DETAIL_TOP,
        lines => FLOW_STATUS_TOP + (lines - 1) as f32 * FLOW_DETAIL_LINE_HEIGHT,
    };
    let last_top = if has_status {
        status_top
    } else {
        detail_top(detail_lines.saturating_sub(1))
    };
    (last_top + (FLOW_ROW_HEIGHT - FLOW_STATUS_TOP), status_top)
}

pub(super) fn flow_row(ui: &mut Ui, flow: &VerificationFlowPresentation, selected: bool) -> bool {
    let VerificationFlowPresentation {
        label,
        detail,
        status,
        problem,
        glyph,
    } = flow;
    let t = Tokens::get(ui.ctx());
    let text_width = (ui.available_width() - FLOW_TEXT_LEFT - 9.0).max(1.0);
    let detail_galley = ui.painter().layout(
        detail.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
        text_width,
    );
    let detail_lines = if detail.is_empty() {
        0
    } else {
        detail_galley.rows.len().max(1)
    };
    let (row_height, status_top) = flow_row_geometry(detail_lines, !status.is_empty());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_height),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        let spoken = [label.as_str(), detail.as_str(), status.as_str()]
            .into_iter()
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join(". ");
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            true,
            selected,
            problem.map_or_else(|| spoken.clone(), |problem| problem.announce(&spoken)),
        )
    });
    if selected || response.hovered() {
        ui.painter().rect_filled(
            rect,
            0.0,
            if selected {
                t.color.accent_dim
            } else {
                t.color.bg_hover
            },
        );
    }
    if selected {
        ui.painter().rect_filled(
            egui::Rect::from_min_max(rect.min, egui::pos2(rect.left() + 2.0, rect.bottom())),
            0.0,
            t.color.accent,
        );
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    // The leading slot holds the flow's problem, in its own mark and colour,
    // or the flow's neutral page glyph. There is no tinted disc behind either.
    let lead = egui::pos2(rect.left() + 17.5, rect.top() + FLOW_LABEL_TOP + 6.5);
    match problem {
        Some(problem) => paint_problem_mark(
            ui.painter(),
            egui::Rect::from_center_size(lead, egui::Vec2::splat(11.0)),
            *problem,
        ),
        None => {
            ui.painter().text(
                lead,
                egui::Align2::CENTER_CENTER,
                glyph,
                theme::mono(tokens::FS_0, FontWeight::Medium),
                t.color.text_dim,
            );
        }
    }
    let text_left = rect.left() + FLOW_TEXT_LEFT;
    let text_right = rect.right() - 9.0;
    let clip = egui::Rect::from_x_y_ranges(text_left..=text_right, rect.y_range());
    let painter = ui.painter().with_clip_rect(clip);
    painter.text(
        egui::pos2(text_left, rect.top() + FLOW_LABEL_TOP),
        egui::Align2::LEFT_TOP,
        label,
        theme::sans(tokens::FS_0, FontWeight::Medium),
        t.color.text,
    );
    if detail_lines > 0 {
        painter.galley(
            egui::pos2(text_left, rect.top() + FLOW_DETAIL_TOP),
            detail_galley,
            t.color.text_faint,
        );
    }
    if !status.is_empty() {
        painter.text(
            egui::pos2(text_left, rect.top() + status_top),
            egui::Align2::LEFT_TOP,
            status,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        );
    }
    theme::paint_focus_ring(ui, &response, rect);
    response.clicked()
}

pub(super) fn models(ui: &mut Ui, app: &mut RSpiceApp) {
    // "Model ownership" titled a page index. Nothing under it is about who
    // owns a model.
    section_header(ui, "Pages", None);
    for page in ModelsPage::ALL {
        if nav_row(
            ui,
            WorkbenchIcon::Models,
            page.label(),
            app.state.workbench.models_page == page,
            None,
        ) {
            app.state.workbench.models_page = page;
        }
    }
    section_header(
        ui,
        "Loaded model libraries",
        Some(&app.state.model_library_manager.library_count().to_string()),
    );
    let libraries: Vec<_> = app
        .state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .map(|library| (library.name.clone(), library.model_count()))
        .collect();
    for (name, count) in libraries {
        let selected = app.state.model_library_manager.selected_library.as_deref() == Some(&name);
        if nav_row(
            ui,
            WorkbenchIcon::Models,
            &name,
            selected,
            Some(&count.to_string()),
        ) {
            app.state.select_model_library(&name);
            app.state.workbench.models_page = ModelsPage::Models;
            app.state.workbench.selected_model = None;
        }
    }
}

pub(super) fn nav_row_indented_styled(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: Option<&str>,
    level: usize,
    mono: bool,
) -> Response {
    let t = Tokens::get(ui.ctx());
    nav_row_indented_styled_with_metrics(
        ui,
        icon,
        label,
        selected,
        meta,
        level,
        mono,
        t.metrics.row_h,
        tokens::FS_0,
        tokens::FS_0,
        false,
        false,
        egui::Sense::click(),
    )
}

pub(super) fn nav_row_indented_styled_with_metrics(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: Option<&str>,
    level: usize,
    mono: bool,
    height: f32,
    label_size: f32,
    meta_size: f32,
    expanded: bool,
    child_guide: bool,
    sense: egui::Sense,
) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(egui::vec2(ui.available_width(), height), sense);
    // The meta column is half of what the row says. `bridge_drive  SIN · r1`
    // states a family and a revision, `Libraries` rows state their cell count
    // and whether they are writable, and a name built from the label alone
    // hands a screen reader the half that is not the answer. Same shape as
    // `ui::widgets::tree`, which is where the label-comma-meta spelling the
    // rest of the product announces comes from.
    let announced = meta.map_or_else(|| label.to_owned(), |meta| format!("{label}, {meta}"));
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            &announced,
        )
    });
    if selected || response.hovered() {
        ui.painter().rect_filled(
            rect,
            0.0,
            if selected {
                t.color.accent_dim
            } else {
                t.color.bg_hover
            },
        );
    }
    if selected {
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                rect.left_top(),
                egui::pos2(rect.left() + 2.0, rect.bottom()),
            ),
            0.0,
            t.color.accent,
        );
    }
    let schematic_metrics = (height - SCHEMATIC_NAV_ROW_HEIGHT).abs() <= f32::EPSILON
        && (label_size - SCHEMATIC_NAV_LABEL_SIZE).abs() <= f32::EPSILON;
    let child_offset = if child_guide { 19.0 } else { 0.0 };
    if child_guide {
        ui.painter().vline(
            rect.left() + 19.0,
            rect.y_range(),
            egui::Stroke::new(1.0, t.color.border),
        );
    }
    // `.nav-children` owns its 19 px hierarchy offset; its rows reset their
    // own padding to the root-row contract instead of accumulating levels.
    let indent = if child_guide {
        0.0
    } else {
        14.0 * level as f32
    };
    if expanded {
        WorkbenchIcon::ChevronDown.paint(
            ui.painter(),
            egui::Rect::from_center_size(
                egui::pos2(
                    rect.left()
                        + child_offset
                        + if schematic_metrics { 14.0 } else { 12.0 }
                        + indent,
                    rect.center().y,
                ),
                egui::vec2(9.0, 9.0),
            ),
            t.color.text_faint,
        );
    }
    icon.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::pos2(
                rect.left() + child_offset + if schematic_metrics { 33.5 } else { 31.0 } + indent,
                rect.center().y,
            ),
            egui::vec2(15.0, 15.0),
        ),
        if selected {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    let meta_width = meta.map_or(0.0, |meta| {
        ui.painter()
            .layout_no_wrap(
                meta.to_owned(),
                theme::mono(meta_size, FontWeight::Regular),
                t.color.text_faint,
            )
            .size()
            .x
    });
    let label_left =
        rect.left() + child_offset + if schematic_metrics { 47.0 } else { 45.0 } + indent;
    let label_right = if meta.is_some() {
        rect.right() - 14.0 - meta_width
    } else {
        rect.right() - 8.0
    };
    ui.painter()
        .with_clip_rect(egui::Rect::from_x_y_ranges(
            label_left..=label_right.max(label_left),
            rect.y_range(),
        ))
        .text(
            egui::pos2(label_left, rect.center().y),
            egui::Align2::LEFT_CENTER,
            label,
            if mono {
                theme::mono(label_size, FontWeight::Regular)
            } else {
                theme::sans(label_size, FontWeight::Regular)
            },
            if selected {
                t.color.text
            } else {
                t.color.text_dim
            },
        );
    if let Some(meta) = meta {
        ui.painter().text(
            egui::pos2(rect.right() - 8.0, rect.center().y),
            egui::Align2::RIGHT_CENTER,
            meta,
            theme::mono(meta_size, FontWeight::Regular),
            t.color.text_faint,
        );
    }
    theme::paint_focus_ring(ui, &response, rect);
    response
}

pub(super) fn muted(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .inner_margin(egui::Margin {
            left: EMPTY_HINT_PADDING_X,
            right: EMPTY_HINT_PADDING_X,
            top: EMPTY_HINT_PADDING_Y,
            bottom: EMPTY_HINT_PADDING_Y,
        })
        .show(ui, |ui| {
            ui.set_width(ui.available_width().max(1.0));
            ui.add(
                egui::Label::new(
                    egui::RichText::new(text)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                )
                .wrap()
                .halign(Align::Center),
            );
        });
}

#[cfg(test)]
mod tests;
