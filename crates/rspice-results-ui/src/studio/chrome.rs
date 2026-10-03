//! Studio header, status strip, and desktop/compact/touch section controls.

use super::{
    COMPACT_BREAKPOINT, TOUCH_DOCK_HEIGHT, bar_content_height,
    widgets::{paint_bottom_rule, separator},
};
use egui::{
    Align, Color32, Frame, Grid, Id, Layout, Margin, Rect, RichText, ScrollArea, Sense, Stroke, Ui,
    Vec2, vec2,
};
use rspice_results::studio_presentation::{VisualizationSection, VisualizationTouchPane};
use rspice_ui_kit::{
    panels::WorkbenchIcon,
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::Button,
};

const SUMMARY: &str = "Compose waveform, tabular, statistical, RF, eye, field, and report-page views with family slicing, exact axes, annotations, measurements, and large-data policies.";
const EVIDENCE: &str = "Implemented visualization entities retain dataset and analysis identities, viewer type, pane/page placement, links, exact markers, scalar measurements, and annotations without mutating source samples.";
const OWNERSHIP: &str =
    "Owns result-document presentation entities, not solver data or release decisions.";
const WORKSPACE_HEADER_HEIGHT: f32 = 58.0;
const WORKSPACE_HEADER_VERTICAL_MARGIN: f32 = 7.0;
const SECTION_NAVIGATION_HEIGHT: f32 = 36.0;

const fn uses_horizontal_kpi_strip(width: f32, coarse_pointer: bool, touch_screen: bool) -> bool {
    width <= COMPACT_BREAKPOINT || coarse_pointer || touch_screen
}

/// Returns whether the user requested the previous navigation origin.
pub fn workspace_header(ui: &mut Ui, configuration: Result<(), &str>, has_origin: bool) -> bool {
    let mut back_requested = false;
    let t = Tokens::get(ui.ctx());
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
                        && has_origin
                        && Button::new("← Source")
                            .ghost()
                            .show(ui)
                            .on_hover_text("Return to the exact navigation origin")
                            .clicked()
                    {
                        back_requested = true;
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
                        if let Err(reason) = configuration {
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
                        if let Err(reason) = configuration {
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
    back_requested
}

pub fn status_label(ui: &mut Ui, label: &str, color: Color32) -> egui::Response {
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

/// Counts prepared from the retained bindings and exact source spans.
pub struct StatusCounts {
    pub dataset_count: usize,
    pub pane_count: usize,
    pub linked_groups: usize,
    pub expression_count: usize,
    pub samples: usize,
    pub revision: u64,
}

pub fn status_strip(ui: &mut Ui, counts: StatusCounts, coarse_pointer: bool) {
    let StatusCounts {
        dataset_count,
        pane_count,
        linked_groups,
        expression_count,
        samples,
        revision,
    } = counts;
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
            format!("{linked_groups} linked groups · revision {}", revision),
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
    let horizontal_strip =
        uses_horizontal_kpi_strip(ui.available_width(), coarse_pointer, touch_screen);
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

fn status_metric_card(ui: &mut Ui, t: &Tokens, width: f32, label: &str, value: &str, detail: &str) {
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

pub fn engineering_count(value: usize) -> String {
    match value {
        1_000_000_000.. => format!("{:.2}B", value as f64 / 1_000_000_000.0),
        1_000_000.. => format!("{:.2}M", value as f64 / 1_000_000.0),
        1_000.. => format!("{:.1}k", value as f64 / 1_000.0),
        _ => value.to_string(),
    }
}

pub fn section_navigation(ui: &mut Ui, selected: &mut VisualizationSection) {
    let t = Tokens::get(ui.ctx());
    let bar = Frame::NONE.fill(t.color.bg_panel).show(ui, |ui| {
        ScrollArea::horizontal()
            .id_salt("visualization.sections")
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::ZERO;
                    for (index, section) in VisualizationSection::ALL.into_iter().enumerate() {
                        let active = *selected == section;
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
                            *selected = section;
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
                                *selected = section;
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

pub fn compact_section_picker(
    ui: &mut Ui,
    selected: &mut VisualizationSection,
    touch_pane: &mut VisualizationTouchPane,
) {
    let t = Tokens::get(ui.ctx());
    ScrollArea::vertical()
        .id_salt("visualization.compact-sections")
        .show(ui, |ui| {
            for (index, section) in VisualizationSection::ALL.into_iter().enumerate() {
                let active = *selected == section;
                let response = ui.add_sized(
                    [ui.available_width(), 44.0],
                    egui::Button::new(format!("{:02}  {}", index + 1, section.label()))
                        .selected(active),
                );
                if response.clicked() {
                    *selected = section;
                    *touch_pane = VisualizationTouchPane::Stage;
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

pub fn touch_dock(ui: &mut Ui, touch_pane: &mut VisualizationTouchPane) {
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
                        let active = *touch_pane == pane;
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
                            *touch_pane = if active {
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
