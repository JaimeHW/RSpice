//! The studio's chrome: header, status strip, and section navigation.
//!
//! The status strip reports the document's own counts and the binding it is
//! showing, not a summary recomputed here, so what the chrome says and what
//! the stage draws can never disagree. Section navigation is the same list on
//! desktop, compact, and touch — the presentation changes, the set of sections
//! does not.

use super::*;

pub(super) fn workspace_header(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let configuration = visualization_configuration_status(&app.state);
    let (configuration_label, configuration_color) = if configuration.is_ok() {
        ("configuration valid", t.color.ok)
    } else {
        ("configuration blocked", t.color.warn)
    };
    let wide = ui.available_width() > 1_120.0;
    let phone = ui.available_width() <= 600.0;
    let show_origin = ui.available_width() <= 760.0;
    let bar = Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(14, 7))
        .show(ui, |ui| {
            ui.set_min_height(bar_content_height(
                WORKSPACE_HEADER_HEIGHT,
                WORKSPACE_HEADER_VERTICAL_MARGIN,
            ));
            ui.horizontal(|ui| {
                let (mark, _) = ui.allocate_exact_size(Vec2::splat(34.0), Sense::hover());
                ui.painter().rect(
                    mark,
                    0.0,
                    t.color.accent_dim,
                    Stroke::new(1.0, t.color.accent),
                    egui::StrokeKind::Inside,
                );
                ui.painter().text(
                    mark.center(),
                    egui::Align2::CENTER_CENTER,
                    "XY",
                    theme::mono(tokens::FS_1, FontWeight::SemiBold),
                    t.color.accent,
                );
                ui.add_space(4.0);
                ui.vertical(|ui| {
                    if show_origin
                        && app.state.workbench.previous_route().is_some()
                        && Button::new("← Source")
                            .ghost()
                            .show(ui)
                            .on_hover_text("Return to the exact navigation origin")
                            .clicked()
                    {
                        let _ = app
                            .state
                            .workbench
                            .navigate_back(RouteTransitionSource::User);
                    }
                    ui.label(
                        RichText::new("RESULT DOCUMENT · VIEWER-SPECIFIC CONTROLS")
                            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                            .color(t.color.text_faint),
                    );
                    ui.label(
                        RichText::new("Lab characterization data display")
                            .font(theme::sans(tokens::FS_3, FontWeight::SemiBold))
                            .color(t.color.text),
                    );
                    if phone {
                        let response = status_label(ui, configuration_label, configuration_color);
                        if let Err(reason) = &configuration {
                            response.on_hover_text(reason);
                        }
                    }
                    if wide {
                        ui.label(
                            RichText::new(SUMMARY)
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_faint),
                        );
                    }
                });
                if !phone {
                    ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                        let response = status_label(ui, configuration_label, configuration_color);
                        if let Err(reason) = &configuration {
                            response.on_hover_text(reason);
                        }
                        if wide {
                            ui.add_space(10.0);
                            ui.vertical(|ui| {
                                ui.label(
                                    RichText::new("RESULT PRESENTATION")
                                        .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                                        .color(t.color.text_faint),
                                );
                                ui.label(
                                    RichText::new(OWNERSHIP)
                                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                        .color(t.color.text_dim),
                                );
                            });
                        }
                    });
                }
            });
        });
    paint_bottom_rule(ui, bar.response.rect, t.color.border_strong);
}

pub(super) fn status_label(ui: &mut Ui, label: &str, color: Color32) -> egui::Response {
    ui.horizontal(|ui| {
        let (dot, _) = ui.allocate_exact_size(vec2(7.0, 13.0), Sense::hover());
        ui.painter().circle_filled(dot.center(), 3.0, color);
        ui.label(
            RichText::new(label)
                .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                .color(color),
        );
    })
    .response
}

pub(super) fn visualization_configuration_status(state: &AppState) -> Result<(), String> {
    let studio = &state.workbench.visualization_studio;
    studio.validate_presentation()?;
    if studio.panes.is_empty() {
        return Err("No visualization pane is bound to an immutable result dataset".to_owned());
    }
    for pane in &studio.panes {
        let run = state
            .simulation
            .runs
            .iter()
            .find(|run| run.dataset_id == pane.dataset_id)
            .ok_or_else(|| format!("Pane {:02} references an unavailable dataset", pane.id))?;
        let analysis = run
            .analyses
            .iter()
            .find(|analysis| analysis.id == pane.analysis_sequence)
            .ok_or_else(|| format!("Pane {:02} references an unavailable analysis", pane.id))?;
        let definition = viewer_document(&pane.viewer_document_id)
            .ok_or_else(|| format!("Pane {:02} references an unknown viewer", pane.id))?;
        let analysis_ids = [analysis_manifest_id(analysis.analysis_type)];
        match viewer_compatibility(
            definition.id,
            ViewerCapabilities {
                analysis_ids: &analysis_ids,
                external_capabilities: &[],
            },
        ) {
            ViewerCompatibility::Compatible => {}
            ViewerCompatibility::MissingAnalysis { .. } => {
                return Err(format!(
                    "Pane {:02} viewer is incompatible with its retained analysis",
                    pane.id
                ));
            }
            ViewerCompatibility::MissingExternalCapability { capability_id } => {
                return Err(format!(
                    "Pane {:02} requires unavailable capability {capability_id}",
                    pane.id
                ));
            }
            ViewerCompatibility::UnknownDocument => {
                return Err(format!("Pane {:02} viewer is not registered", pane.id));
            }
        }
        // A retained pane names both its sheet and its viewer document, so what
        // has to hold is that the two agree — read forwards, from the sheet.
        // Read backwards it would not: three sheets render `viewer-table`, and
        // the inverse can only name one of them, so a retained Specs or OP pane
        // would be rejected as having no renderer.
        if pane.viewer.viewer_document_id() != Some(definition.id) {
            return Err(format!(
                "Pane {:02} has no exact renderer for its retained viewer",
                pane.id
            ));
        }
        if pane.viewer == ResultViewer::PoleZero && retained_pole_zero_payload(analysis).is_none() {
            return Err(format!(
                "Pane {:02} has no valid retained pole-zero payload",
                pane.id
            ));
        }
        if pane.viewer == ResultViewer::Contribution
            && retained_sensitivity_payload(analysis).is_none()
        {
            return Err(format!(
                "Pane {:02} has no valid retained sensitivity payload",
                pane.id
            ));
        }
    }
    for annotation in &studio.annotations {
        if !state.simulation.runs.iter().any(|run| {
            run.dataset_id == annotation.dataset_id
                && run
                    .analyses
                    .iter()
                    .any(|analysis| analysis.id == annotation.analysis_sequence)
        }) {
            return Err(format!(
                "Annotation {:02} references an unavailable source",
                annotation.id
            ));
        }
    }
    for marker in &studio.markers {
        if !state.simulation.runs.iter().any(|run| {
            run.dataset_id == marker.dataset_id
                && run
                    .analyses
                    .iter()
                    .any(|analysis| analysis.id == marker.analysis_sequence)
        }) {
            return Err(format!(
                "Marker {:02} references an unavailable source",
                marker.id
            ));
        }
    }
    for measurement in &studio.measurements {
        if !state.simulation.runs.iter().any(|run| {
            run.dataset_id == measurement.dataset_id
                && run
                    .analyses
                    .iter()
                    .any(|analysis| analysis.id == measurement.analysis_sequence)
        }) {
            return Err(format!(
                "Measurement {:02} references an unavailable source",
                measurement.id
            ));
        }
    }
    Ok(())
}

pub(super) fn status_strip(ui: &mut Ui, app: &RSpiceApp) {
    let studio = &app.state.workbench.visualization_studio;
    let bound_datasets = studio
        .panes
        .iter()
        .map(|pane| pane.dataset_id)
        .collect::<HashSet<_>>();
    let dataset_count = bound_datasets.len();
    let pane_count = studio.panes.len();
    let linked_groups = studio
        .panes
        .iter()
        .flat_map(|pane| [pane.x_link, pane.cursor_group])
        .flatten()
        .collect::<HashSet<_>>()
        .len();
    let expression_count: usize = app.state.ui.results.exprs.values().map(Vec::len).sum();
    let samples: usize = app
        .state
        .simulation
        .runs
        .iter()
        .filter(|run| bound_datasets.contains(&run.dataset_id))
        .flat_map(|run| &run.analyses)
        .flat_map(|analysis| &analysis.waveforms)
        .map(|waveform| waveform.x.len().min(waveform.y.len()))
        .sum();
    let metrics = [
        (
            "Datasets",
            dataset_count.to_string(),
            if dataset_count == 0 {
                "No immutable dataset".to_owned()
            } else {
                format!("1 active · {} overlay", dataset_count.saturating_sub(1))
            },
        ),
        (
            "View panes",
            pane_count.to_string(),
            format!(
                "{linked_groups} linked groups · revision {}",
                studio.revision
            ),
        ),
        (
            "Expressions",
            expression_count.to_string(),
            "calculator-owned".to_owned(),
        ),
        (
            "Sample span",
            engineering_count(samples),
            "exact source samples".to_owned(),
        ),
    ];
    let touch_screen = ui.ctx().input(|input| input.has_touch_screen());
    let horizontal_strip = uses_horizontal_kpi_strip(
        ui.available_width(),
        app.state.workbench.coarse_pointer,
        touch_screen,
    );
    let t = Tokens::get(ui.ctx());
    if horizontal_strip {
        ScrollArea::horizontal()
            .id_salt("visualization.status-strip.mobile")
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::ZERO;
                    for (label, value, detail) in &metrics {
                        status_metric_card(ui, &t, 142.0, label, value, detail);
                    }
                });
            });
        return;
    }
    let card_width = (ui.available_width() / 4.0).max(1.0);
    Grid::new("visualization.status-strip")
        .num_columns(4)
        .spacing(Vec2::ZERO)
        .show(ui, |ui| {
            for (index, (label, value, detail)) in metrics.iter().enumerate() {
                status_metric_card(ui, &t, card_width, label, value, detail);
                if (index + 1) % 4 == 0 {
                    ui.end_row();
                }
            }
        });
}

pub(super) fn status_metric_card(
    ui: &mut Ui,
    t: &Tokens,
    width: f32,
    label: &str,
    value: &str,
    detail: &str,
) {
    Frame::NONE
        .fill(t.color.bg_app)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(Margin::symmetric(10, 6))
        .show(ui, |ui| {
            ui.set_min_width((width - 1.0).max(1.0));
            ui.set_max_width((width - 1.0).max(1.0));
            ui.set_min_height(38.0);
            ui.with_layout(Layout::top_down(Align::Min), |ui| {
                ui.label(
                    RichText::new(label.to_uppercase())
                        .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                        .color(t.color.text_faint),
                );
                ui.label(
                    RichText::new(value)
                        .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                        .color(t.color.text),
                );
                ui.label(
                    RichText::new(detail)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                );
            });
        });
}

pub(super) fn engineering_count(value: usize) -> String {
    match value {
        1_000_000_000.. => format!("{:.2}B", value as f64 / 1_000_000_000.0),
        1_000_000.. => format!("{:.2}M", value as f64 / 1_000_000.0),
        1_000.. => format!("{:.1}k", value as f64 / 1_000.0),
        _ => value.to_string(),
    }
}

pub(super) fn section_navigation(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let bar = Frame::NONE.fill(t.color.bg_panel).show(ui, |ui| {
        ScrollArea::horizontal()
            .id_salt("visualization.sections")
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::ZERO;
                    for (index, section) in VisualizationSection::ALL.into_iter().enumerate() {
                        let active = app.state.workbench.visualization_studio.section == section;
                        let id = Id::new(("visualization.section", section));
                        let response = ui
                            .push_id(id, |ui| {
                                let (rect, response) = ui.allocate_exact_size(
                                    vec2(142.0, SECTION_NAVIGATION_HEIGHT),
                                    Sense::click(),
                                );
                                let fill = if active {
                                    t.color.bg_active
                                } else if response.hovered() {
                                    t.color.bg_hover
                                } else {
                                    Color32::TRANSPARENT
                                };
                                ui.painter().rect_filled(rect, 0.0, fill);
                                ui.painter().vline(
                                    rect.right(),
                                    rect.y_range(),
                                    Stroke::new(1.0, t.color.border),
                                );
                                if active {
                                    ui.painter().hline(
                                        rect.x_range(),
                                        rect.bottom() - 1.0,
                                        Stroke::new(2.0, t.color.accent),
                                    );
                                }
                                ui.painter().text(
                                    rect.left_center() + vec2(10.0, 0.0),
                                    egui::Align2::LEFT_CENTER,
                                    format!("{:02}  {}", index + 1, section.label()),
                                    theme::sans(tokens::FS_1, FontWeight::Medium),
                                    if active {
                                        t.color.text
                                    } else {
                                        t.color.text_dim
                                    },
                                );
                                response
                            })
                            .inner;
                        response.widget_info(|| {
                            egui::WidgetInfo::selected(
                                egui::WidgetType::SelectableLabel,
                                ui.is_enabled(),
                                active,
                                section.label(),
                            )
                        });
                        ui.ctx().accesskit_node_builder(response.id, |node| {
                            node.set_role(egui::accesskit::Role::Tab);
                            node.set_selected(active);
                            node.set_label(section.label());
                        });
                        if response.clicked() {
                            app.state.workbench.visualization_studio.section = section;
                        }
                        if active && response.has_focus() {
                            let next = ui.input_mut(|input| {
                                if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight) {
                                    Some((index + 1) % VisualizationSection::ALL.len())
                                } else if input
                                    .consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft)
                                {
                                    Some(
                                        (index + VisualizationSection::ALL.len() - 1)
                                            % VisualizationSection::ALL.len(),
                                    )
                                } else if input.consume_key(egui::Modifiers::NONE, egui::Key::Home)
                                {
                                    Some(0)
                                } else if input.consume_key(egui::Modifiers::NONE, egui::Key::End) {
                                    Some(VisualizationSection::ALL.len() - 1)
                                } else {
                                    None
                                }
                            });
                            if let Some(next) = next {
                                let section = VisualizationSection::ALL[next];
                                app.state.workbench.visualization_studio.section = section;
                                ui.ctx().memory_mut(|memory| {
                                    memory
                                        .request_focus(Id::new(("visualization.section", section)))
                                });
                            }
                        }
                        theme::paint_focus_ring(ui, &response, response.rect);
                    }
                });
            });
    });
    paint_bottom_rule(ui, bar.response.rect, t.color.border_strong);
}

pub(super) fn compact_section_picker(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    ScrollArea::vertical()
        .id_salt("visualization.compact-sections")
        .show(ui, |ui| {
            for (index, section) in VisualizationSection::ALL.into_iter().enumerate() {
                let active = app.state.workbench.visualization_studio.section == section;
                let response = ui.add_sized(
                    [ui.available_width(), 44.0],
                    egui::Button::new(format!("{:02}  {}", index + 1, section.label()))
                        .selected(active),
                );
                if response.clicked() {
                    app.state.workbench.visualization_studio.section = section;
                    app.state.workbench.visualization_studio.touch_pane =
                        VisualizationTouchPane::Stage;
                }
            }
            ui.add_space(10.0);
            Frame::NONE
                .fill(t.color.bg_inset)
                .inner_margin(Margin::same(10))
                .show(ui, |ui| {
                    ui.label(
                        RichText::new("EVIDENCE CONTRACT")
                            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                            .color(t.color.text_faint),
                    );
                    ui.label(
                        RichText::new(EVIDENCE)
                            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                            .color(t.color.text_dim),
                    );
                });
        });
}

pub(super) fn touch_dock(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().min(ui.clip_rect().width()).max(1.0);
    separator(ui, t.color.border_strong);
    ui.allocate_ui_with_layout(
        vec2(width, TOUCH_DOCK_HEIGHT - 1.0),
        Layout::top_down(Align::Min),
        |ui| {
            Frame::NONE.fill(t.color.bg_panel).show(ui, |ui| {
                ui.set_width(width);
                ui.set_min_height(TOUCH_DOCK_HEIGHT - 1.0);
                ui.columns(3, |columns| {
                    let controls = [
                        (
                            VisualizationTouchPane::Sections,
                            "Sections",
                            WorkbenchIcon::Grid,
                        ),
                        (
                            VisualizationTouchPane::Inspector,
                            "Inspect",
                            WorkbenchIcon::Sliders,
                        ),
                        (
                            VisualizationTouchPane::Actions,
                            "Actions",
                            WorkbenchIcon::More,
                        ),
                    ];
                    for (column, (pane, label, icon)) in columns.iter_mut().zip(controls) {
                        let active = app.state.workbench.visualization_studio.touch_pane == pane;
                        let (rect, response) = column.allocate_exact_size(
                            vec2(column.available_width(), TOUCH_DOCK_HEIGHT - 1.0),
                            Sense::click(),
                        );
                        response.widget_info(|| {
                            egui::WidgetInfo::selected(
                                egui::WidgetType::Button,
                                column.is_enabled(),
                                active,
                                label,
                            )
                        });
                        column.painter().rect_filled(
                            rect,
                            0.0,
                            if active {
                                t.color.bg_active
                            } else {
                                t.color.bg_panel
                            },
                        );
                        icon.paint(
                            column.painter(),
                            Rect::from_center_size(
                                rect.center_top() + vec2(0.0, 15.0),
                                Vec2::splat(16.0),
                            ),
                            if active {
                                t.color.accent
                            } else {
                                t.color.text_dim
                            },
                        );
                        column.painter().text(
                            rect.center_bottom() - vec2(0.0, 7.0),
                            egui::Align2::CENTER_BOTTOM,
                            label,
                            theme::sans(tokens::FS_0, FontWeight::Medium),
                            if active {
                                t.color.text
                            } else {
                                t.color.text_dim
                            },
                        );
                        if response.clicked() {
                            app.state.workbench.visualization_studio.touch_pane = if active {
                                VisualizationTouchPane::Stage
                            } else {
                                pane
                            };
                        }
                        theme::paint_focus_ring(column, &response, rect);
                    }
                });
            });
        },
    );
}
