use std::ops::RangeInclusive;

use egui::{CentralPanel, Color32, Context, Frame, SidePanel, TopBottomPanel};

use super::{BottomPanelTab, PanelVisibility, RSpiceApp};
use crate::common::viewer_style::viewer_header_bg_color;

const ICON_RAIL_WIDTH: f32 = 42.0;
const SIDE_PANEL_MARGIN: f32 = 8.0;

const BOTTOM_TAB_HEIGHT: f32 = 28.0;
const BOTTOM_TAB_MIN_WIDTH: f32 = 80.0;
const BOTTOM_TAB_PADDING: f32 = 16.0;
const BOTTOM_TAB_UNDERLINE_HEIGHT: f32 = 2.0;
const BOTTOM_PANEL_MIN_HEIGHT: f32 = 100.0;
const BOTTOM_PANEL_EDGE_MARGIN: f32 = 120.0;

const SIDE_PANEL_HEADER_HEIGHT: f32 = 16.0;

fn compute_bottom_tab_width(text_width: f32) -> f32 {
    (text_width + BOTTOM_TAB_PADDING * 2.0).max(BOTTOM_TAB_MIN_WIDTH)
}

fn bottom_panel_height_range(screen_height: f32) -> RangeInclusive<f32> {
    let min = BOTTOM_PANEL_MIN_HEIGHT;
    let max = (screen_height - BOTTOM_PANEL_EDGE_MARGIN).max(min);
    min..=max
}

fn should_show_project_explorer(panels: &PanelVisibility) -> bool {
    panels.project_explorer && !panels.results_browser
}

fn should_show_library_browser(panels: &PanelVisibility) -> bool {
    panels.library_browser && !panels.project_explorer && !panels.results_browser
}

fn separator_line_bounds(rect: egui::Rect, margin: f32) -> RangeInclusive<f32> {
    (rect.left() - margin)..=(rect.right() + margin)
}

fn bottom_panel_bg_fill() -> Color32 {
    viewer_header_bg_color()
}

fn draw_side_panel_separator(ui: &mut egui::Ui, margin: f32) {
    let separator_rect = ui.available_rect_before_wrap();
    ui.painter().hline(
        separator_line_bounds(separator_rect, margin),
        separator_rect.top(),
        egui::Stroke::new(1.0, Color32::from_rgb(50, 54, 62)),
    );
}

impl RSpiceApp {
    pub(super) fn render_workspace_layout(&mut self, ctx: &Context) {
        self.render_icon_rail_panel(ctx);
        self.render_bottom_panel(ctx);
        self.render_left_browser_panel(ctx);
        self.render_properties_panel(ctx);
        self.render_central_schematic_panel(ctx);
    }

    fn render_icon_rail_panel(&mut self, ctx: &Context) {
        SidePanel::left("icon_rail")
            .resizable(false)
            .exact_width(ICON_RAIL_WIDTH)
            .frame(Frame::none().fill(Color32::from_rgb(22, 24, 30)))
            .show(ctx, |ui| {
                self.render_icon_rail(ui);
            });
    }

    fn render_bottom_panel(&mut self, ctx: &Context) {
        if !self.state.panels.bottom_panel {
            return;
        }

        let height_range = bottom_panel_height_range(ctx.screen_rect().height());

        // Use waveform_height only as the initial default — do NOT feed the
        // panel's actual rect height back into default_height each frame.
        // egui stores resize deltas relative to default_height, so tracking
        // the actual height creates a compounding feedback loop that forces
        // the panel to its maximum size.
        let default_height = self.state.panel_sizes.waveform_height;

        TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .default_height(default_height)
            .height_range(height_range)
            .frame(Frame::none().fill(bottom_panel_bg_fill()))
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;

                let accent_color = Color32::from_rgb(100, 150, 255);
                let inactive_text = Color32::from_rgb(160, 165, 175);
                let active_text = Color32::WHITE;
                let hover_text = Color32::from_rgb(200, 205, 215);

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;

                    for &tab in BottomPanelTab::all() {
                        let selected = self.state.panels.active_bottom_tab == tab;

                        let text = tab.name();
                        let galley = ui.fonts(|f| {
                            f.layout_no_wrap(
                                text.to_string(),
                                egui::FontId::proportional(12.0),
                                active_text,
                            )
                        });
                        let tab_width = compute_bottom_tab_width(galley.rect.width());

                        let (rect, response) = ui.allocate_exact_size(
                            egui::vec2(tab_width, BOTTOM_TAB_HEIGHT),
                            egui::Sense::click(),
                        );

                        let text_color = if selected {
                            active_text
                        } else if response.hovered() {
                            hover_text
                        } else {
                            inactive_text
                        };

                        let painter = ui.painter();

                        if response.hovered() && !selected {
                            painter.rect_filled(
                                rect,
                                egui::Rounding::ZERO,
                                Color32::from_rgba_unmultiplied(255, 255, 255, 8),
                            );
                        }

                        painter.text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            text,
                            egui::FontId::proportional(12.0),
                            text_color,
                        );

                        if selected {
                            let underline_rect = egui::Rect::from_min_size(
                                egui::pos2(rect.min.x, rect.max.y - BOTTOM_TAB_UNDERLINE_HEIGHT),
                                egui::vec2(rect.width(), BOTTOM_TAB_UNDERLINE_HEIGHT),
                            );
                            painter.rect_filled(underline_rect, egui::Rounding::ZERO, accent_color);
                        }

                        if response.clicked() {
                            self.state.panels.active_bottom_tab = tab;
                        }
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let close_size = egui::vec2(BOTTOM_TAB_HEIGHT, BOTTOM_TAB_HEIGHT);
                        let (close_rect, close_response) =
                            ui.allocate_exact_size(close_size, egui::Sense::click());

                        let close_color = if close_response.hovered() {
                            Color32::from_rgb(255, 100, 100)
                        } else {
                            inactive_text
                        };

                        if close_response.hovered() {
                            ui.painter().rect_filled(
                                close_rect,
                                egui::Rounding::same(2.0),
                                Color32::from_rgba_unmultiplied(255, 80, 80, 30),
                            );
                        }

                        let center = close_rect.center();
                        let cross_size = 4.0;
                        let stroke = egui::Stroke::new(1.5, close_color);
                        ui.painter().line_segment(
                            [
                                center + egui::vec2(-cross_size, -cross_size),
                                center + egui::vec2(cross_size, cross_size),
                            ],
                            stroke,
                        );
                        ui.painter().line_segment(
                            [
                                center + egui::vec2(cross_size, -cross_size),
                                center + egui::vec2(-cross_size, cross_size),
                            ],
                            stroke,
                        );

                        if close_response.on_hover_text("Close panel").clicked() {
                            self.state.panels.bottom_panel = false;
                        }
                    });
                });

                let separator_rect = ui.available_rect_before_wrap();
                ui.painter().hline(
                    separator_rect.x_range(),
                    separator_rect.top(),
                    egui::Stroke::new(1.0, Color32::from_rgb(50, 54, 62)),
                );

                match self.state.panels.active_bottom_tab {
                    BottomPanelTab::Log => self.render_log_panel(ui),
                    BottomPanelTab::Waveform => self.render_waveform_panel(ui),
                    BottomPanelTab::Automation => self.render_automation_panel(ui),
                }
            });
    }

    fn render_left_browser_panel(&mut self, ctx: &Context) {
        if should_show_project_explorer(&self.state.panels) {
            self.render_project_explorer_panel(ctx);
            return;
        }

        if should_show_library_browser(&self.state.panels) {
            self.render_library_browser_panel(ctx);
            return;
        }

        if self.state.panels.results_browser {
            self.render_results_browser_panel(ctx);
        }
    }

    fn render_project_explorer_panel(&mut self, ctx: &Context) {
        SidePanel::left("left_browser")
            .resizable(true)
            .default_width(self.state.panel_sizes.browser_width)
            .width_range(150.0..=400.0)
            .frame(
                Frame::none()
                    .fill(Color32::from_rgb(30, 33, 40))
                    .inner_margin(egui::Margin::same(SIDE_PANEL_MARGIN)),
            )
            .show(ctx, |ui| {
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), SIDE_PANEL_HEADER_HEIGHT),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.label(
                            egui::RichText::new("Project Explorer")
                                .size(14.0)
                                .color(Color32::from_rgb(180, 180, 190)),
                        );
                    },
                );

                draw_side_panel_separator(ui, SIDE_PANEL_MARGIN);
                ui.add_space(8.0);
                crate::panels::render_project_explorer(ui, &mut self.state);
            });
    }

    fn render_library_browser_panel(&mut self, ctx: &Context) {
        SidePanel::left("left_browser")
            .resizable(true)
            .default_width(self.state.panel_sizes.browser_width)
            .width_range(150.0..=400.0)
            .frame(
                Frame::none()
                    .fill(Color32::from_rgb(30, 33, 40))
                    .inner_margin(egui::Margin::same(SIDE_PANEL_MARGIN)),
            )
            .show(ctx, |ui| {
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), SIDE_PANEL_HEADER_HEIGHT),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.label(
                            egui::RichText::new("Library Browser")
                                .size(14.0)
                                .color(Color32::from_rgb(180, 180, 190)),
                        );
                    },
                );

                draw_side_panel_separator(ui, SIDE_PANEL_MARGIN);
                ui.add_space(8.0);
                crate::panels::render_library_browser(ui, &mut self.state);
            });
    }

    fn render_results_browser_panel(&mut self, ctx: &Context) {
        SidePanel::left("left_browser")
            .resizable(true)
            .default_width(self.state.panel_sizes.browser_width)
            .width_range(150.0..=400.0)
            .frame(
                Frame::none()
                    .fill(Color32::from_rgb(30, 33, 40))
                    .inner_margin(egui::Margin::same(SIDE_PANEL_MARGIN)),
            )
            .show(ctx, |ui| {
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), SIDE_PANEL_HEADER_HEIGHT),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.label(
                            egui::RichText::new("Simulation Results")
                                .size(14.0)
                                .color(Color32::from_rgb(180, 180, 190)),
                        );
                    },
                );

                draw_side_panel_separator(ui, SIDE_PANEL_MARGIN);
                ui.add_space(8.0);
                crate::panels::render_results_browser(ui, &mut self.state);
            });
    }

    fn render_properties_panel(&mut self, ctx: &Context) {
        if !self.state.panels.properties {
            return;
        }

        SidePanel::right("properties")
            .resizable(true)
            .default_width(self.state.panel_sizes.properties_width)
            .width_range(180.0..=400.0)
            .frame(
                Frame::none()
                    .fill(Color32::from_rgb(30, 33, 40))
                    .inner_margin(egui::Margin::same(SIDE_PANEL_MARGIN)),
            )
            .show(ctx, |ui| {
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), SIDE_PANEL_HEADER_HEIGHT),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.label(
                            egui::RichText::new("Properties")
                                .size(14.0)
                                .color(Color32::from_rgb(180, 180, 190)),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let close_size = egui::vec2(18.0, 18.0);
                            let (close_rect, close_response) =
                                ui.allocate_exact_size(close_size, egui::Sense::click());

                            let inactive_text = Color32::from_rgb(120, 125, 135);
                            let close_color = if close_response.hovered() {
                                Color32::from_rgb(255, 100, 100)
                            } else {
                                inactive_text
                            };

                            if close_response.hovered() {
                                ui.painter().rect_filled(
                                    close_rect,
                                    egui::Rounding::same(2.0),
                                    Color32::from_rgba_unmultiplied(255, 80, 80, 30),
                                );
                            }

                            let center = close_rect.center();
                            let cross_size = 4.0;
                            let stroke = egui::Stroke::new(1.5, close_color);
                            ui.painter().line_segment(
                                [
                                    center + egui::vec2(-cross_size, -cross_size),
                                    center + egui::vec2(cross_size, cross_size),
                                ],
                                stroke,
                            );
                            ui.painter().line_segment(
                                [
                                    center + egui::vec2(cross_size, -cross_size),
                                    center + egui::vec2(-cross_size, cross_size),
                                ],
                                stroke,
                            );

                            if close_response.on_hover_text("Close panel").clicked() {
                                self.state.panels.properties = false;
                            }
                        });
                    },
                );

                draw_side_panel_separator(ui, SIDE_PANEL_MARGIN);
                ui.add_space(8.0);
                crate::panels::render_properties_panel(ui, &mut self.state);
            });
    }

    fn render_central_schematic_panel(&mut self, ctx: &Context) {
        CentralPanel::default()
            .frame(Frame::none().fill(Color32::from_rgb(18, 20, 24)))
            .show(ctx, |ui| {
                crate::schematic::view::render_schematic_view(
                    ui,
                    &mut self.state,
                    self.symbol_library.as_ref(),
                );
            });
    }
}
