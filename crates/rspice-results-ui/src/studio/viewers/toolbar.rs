//! Studio tools and live viewport controls with immediate host actions.

use super::super::{bar_content_height, widgets::paint_bottom_rule};
use egui::{Align, Frame, Layout, Margin, RichText, ScrollArea, Ui};
use rspice_results::studio_presentation::ViewerTool;
use rspice_ui_kit::{
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::Button,
};

#[derive(Clone, Copy)]
pub enum ToolbarAction {
    SelectTool(ViewerTool),
    TraceManager,
    Axes,
    AddCursor,
    AddMarker,
    Measurement,
    Annotation,
    Export,
    Fit,
    Zoom(f32),
}

pub trait ToolbarHost {
    fn waveform_coordinates(&self) -> bool;
    fn magnification_available(&self) -> bool;
    fn tool(&self) -> ViewerTool;
    fn fit_block_reason(&self) -> Option<&'static str>;
    fn magnification_readout(&mut self, tokens: &Tokens) -> String;
    fn request(&mut self, action: ToolbarAction);
}

const VIEWER_TOOLBAR_HEIGHT: f32 = 36.0;

const VIEWER_TOOLBAR_VERTICAL_MARGIN: f32 = 5.0;

pub fn show(ui: &mut Ui, host: &mut impl ToolbarHost, compact: bool) {
    let t = Tokens::get(ui.ctx());
    let bar = Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(8, 5))
        .show(ui, |ui| {
            ui.set_min_height(if compact {
                44.0
            } else {
                bar_content_height(VIEWER_TOOLBAR_HEIGHT, VIEWER_TOOLBAR_VERTICAL_MARGIN)
            });
            ScrollArea::horizontal()
                .id_salt("visualization.viewer-toolbar")
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 6.0;
                        let waveform_coordinates = host.waveform_coordinates();
                        let magnifiable = host.magnification_available();
                        for tool in ViewerTool::ALL {
                            let active = host.tool() == tool;
                            if ui
                                .add_sized(
                                    [
                                        if compact { 64.0 } else { 54.0 },
                                        if compact { 42.0 } else { 26.0 },
                                    ],
                                    egui::Button::new(tool.label()).selected(active),
                                )
                                .clicked()
                            {
                                host.request(ToolbarAction::SelectTool(tool));
                            }
                        }
                        toolbar_action(ui, "Add trace", || {
                            host.request(ToolbarAction::TraceManager)
                        });
                        toolbar_action(ui, "Edit axis", || {
                            host.request(ToolbarAction::Axes);
                        });
                        toolbar_action_enabled(
                            ui,
                            "Add cursor",
                            waveform_coordinates,
                            "Exact source cursors are available in the waveform renderer",
                            || host.request(ToolbarAction::AddCursor),
                        );
                        toolbar_action_enabled(
                            ui,
                            "Add marker",
                            waveform_coordinates,
                            "Exact source markers are available in the waveform renderer",
                            || host.request(ToolbarAction::AddMarker),
                        );
                        toolbar_action(ui, "Measure", || {
                            host.request(ToolbarAction::Measurement);
                        });
                        toolbar_action(ui, "Annotate", || {
                            host.request(ToolbarAction::Annotation);
                        });
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            toolbar_action(ui, "Export…", || host.request(ToolbarAction::Export));
                            toolbar_action(ui, "Axes & display…", || {
                                host.request(ToolbarAction::Axes);
                            });
                            let fit_blocker = host.fit_block_reason();
                            let fit =
                                ui.add_enabled(fit_blocker.is_none(), egui::Button::new("Fit"));
                            let fit = if let Some(reason) = fit_blocker {
                                fit.on_disabled_hover_text(reason)
                            } else {
                                fit
                            };
                            if fit.clicked() {
                                host.request(ToolbarAction::Fit);
                            }
                            if ui
                                .add_enabled(magnifiable, egui::Button::new("+"))
                                .on_hover_text("Zoom in")
                                .clicked()
                            {
                                host.request(ToolbarAction::Zoom(1.25));
                            }
                            ui.label(
                                RichText::new(host.magnification_readout(&t))
                                    .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                                    .color(t.color.text_dim),
                            );
                            if ui
                                .add_enabled(magnifiable, egui::Button::new("−"))
                                .on_hover_text("Zoom out")
                                .clicked()
                            {
                                host.request(ToolbarAction::Zoom(0.8));
                            }
                        });
                    });
                });
        });
    paint_bottom_rule(ui, bar.response.rect, t.color.border_strong);
}

fn toolbar_action(ui: &mut Ui, label: &'static str, action: impl FnOnce()) {
    if Button::new(label).ghost().show(ui).clicked() {
        action();
    }
}

fn toolbar_action_enabled(
    ui: &mut Ui,
    label: &'static str,
    enabled: bool,
    unavailable_reason: &'static str,
    action: impl FnOnce(),
) {
    if ui
        .add_enabled(enabled, egui::Button::new(label))
        .on_disabled_hover_text(unavailable_reason)
        .clicked()
    {
        action();
    }
}
