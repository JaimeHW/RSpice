//! Waveform instrument presentation over active-pane host operations.

use super::{
    ExportRequests, ResultBarMetrics, export_menu, instrument_control, instrument_separator,
};
use egui::Ui;
use rspice_ui_kit::{
    icons::Icon,
    plot::InteractionMode,
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::IconButton,
};

/// Mutually exclusive primary pointer tool for instrument-style result plots.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ResultPlotTool {
    /// Normal result interaction: cursors/markers plus wheel zoom and panning.
    #[default]
    Cursor,
    /// Primary drag draws a zoom rectangle.
    BoxZoom,
    /// Primary drag pans the plot viewport.
    Pan,
    /// Primary click or drag places the active pane's horizontal cursor.
    HorizontalCursor,
}

impl ResultPlotTool {
    pub const fn interaction_mode(self) -> InteractionMode {
        match self {
            Self::Cursor => InteractionMode::All,
            Self::BoxZoom => InteractionMode::Zoom,
            Self::Pan => InteractionMode::Pan,
            Self::HorizontalCursor => InteractionMode::Select,
        }
    }
}

/// Current local controls; queried again after each host request.
#[derive(Debug, Clone, Copy)]
pub struct InstrumentControls {
    pub plot_tool: ResultPlotTool,
    pub cursor_armed: bool,
    pub show_spec_limits: bool,
    pub show_family_envelope: bool,
    pub show_minor_grid: bool,
}

/// Availability resolved against the active retained pane before drawing.
pub struct InstrumentAvailability {
    pub limits_available: bool,
    pub envelope_available: bool,
    pub marker_available: bool,
}

/// A request applied by the host to its current active pane and local tools.
pub enum InstrumentAction {
    Cursor,
    Tool(ResultPlotTool),
    Zoom(f64),
    Fit,
    ToggleLimits,
    ToggleEnvelope,
    ToggleGrid,
    DropMarker,
    RestoreStrips,
}

/// Immediate host operations preserve control order within a frame.
pub trait WaveInstrumentHost {
    fn controls(&self) -> InstrumentControls;
    fn request(&mut self, action: InstrumentAction);
    fn export(&mut self, requests: ExportRequests);
    fn inline_cursor_readout(&mut self) -> Option<String>;
    fn hidden_strip_count(&self) -> usize;
}

pub fn show(ui: &mut Ui, availability: InstrumentAvailability, host: &mut impl WaveInstrumentHost) {
    let t = Tokens::get(ui.ctx());
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        host.export(export_menu(ui));
        // The collapsed readout's numbers ride the bar's right edge, where
        // the strip that owns them would otherwise be.
        if let Some(readout) = host.inline_cursor_readout() {
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(readout)
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            )
            .on_hover_text("Cursor readout · expand the strip for per-trace values");
        }
        let remaining = ui.available_size();
        ui.allocate_ui_with_layout(
            remaining,
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                egui::ScrollArea::horizontal()
                    .id_salt("rspice.results.wave-instrument")
                    .auto_shrink([false, true])
                    .scroll_bar_visibility(
                        egui::scroll_area::ScrollBarVisibility::AlwaysHidden,
                    )
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 4.0;
                            let cursor_active = host.controls().plot_tool
                                == ResultPlotTool::Cursor
                                && host.controls().cursor_armed;
                            if instrument_control(ui, "A|B", cursor_active, "A/B cursor tool")
                                .clicked()
                            {
                                host.request(InstrumentAction::Cursor);
                            }
                            if instrument_control(
                                ui,
                                "H",
                                host.controls().plot_tool == ResultPlotTool::HorizontalCursor,
                                "Horizontal cursor - click or drag in the active pane",
                            )
                            .clicked()
                            {
                                host.request(InstrumentAction::Tool(ResultPlotTool::HorizontalCursor));
                            }

                            // Box zoom and pan are gestures, so they carry
                            // glyphs; the lettered controls on this bar are
                            // the named modes beside them.
                            instrument_separator(ui);
                            if IconButton::new(Icon::BoxZoom)
                                .side(ResultBarMetrics::of(ui).instrument_control)
                                .on(host.controls().plot_tool == ResultPlotTool::BoxZoom)
                                .tooltip("Box zoom - drag a region")
                                .show(ui)
                                .clicked()
                            {
                                host.request(InstrumentAction::Tool(ResultPlotTool::BoxZoom));
                            }
                            if IconButton::new(Icon::Pan)
                                .side(ResultBarMetrics::of(ui).instrument_control)
                                .on(host.controls().plot_tool == ResultPlotTool::Pan)
                                .tooltip("Pan viewport - drag the plot")
                                .show(ui)
                                .clicked()
                            {
                                host.request(InstrumentAction::Tool(ResultPlotTool::Pan));
                            }

                            // Viewport controls carry their glyphs like the
                            // mockup: magnitude and fit are gestures, and the
                            // lettered controls beside them are named modes.
                            instrument_separator(ui);
                            if IconButton::new(Icon::ZoomIn)
                                .side(ResultBarMetrics::of(ui).instrument_control)
                                .tooltip("Zoom active pane in 2x")
                                .show(ui)
                                .clicked()
                            {
                                host.request(InstrumentAction::Zoom(0.5));
                            }
                            if IconButton::new(Icon::ZoomOut)
                                .side(ResultBarMetrics::of(ui).instrument_control)
                                .tooltip("Zoom active pane out 2x")
                                .show(ui)
                                .clicked()
                            {
                                host.request(InstrumentAction::Zoom(2.0));
                            }
                            if IconButton::new(Icon::ZoomFit)
                                .side(ResultBarMetrics::of(ui).instrument_control)
                                .tooltip("Fit active waveform pane")
                                .show(ui)
                                .clicked()
                            {
                                host.request(InstrumentAction::Fit);
                            }

                            instrument_separator(ui);
                            if ui
                                .add_enabled_ui(availability.limits_available, |ui| {
                                    instrument_control(
                                        ui,
                                        "LIM",
                                        host.controls().show_spec_limits,
                                        "Show exact compatible project specification limits",
                                    )
                                })
                                .inner
                                .clicked()
                            {
                                host.request(InstrumentAction::ToggleLimits);
                            }
                            if ui
                                .add_enabled_ui(availability.envelope_available, |ui| {
                                    instrument_control(
                                        ui,
                                        "ENV",
                                        host.controls().show_family_envelope,
                                        "Show min/max envelope from retained family samples",
                                    )
                                })
                                .inner
                                .clicked()
                            {
                                host.request(InstrumentAction::ToggleEnvelope);
                            }
                            if instrument_control(
                                ui,
                                "GRID",
                                host.controls().show_minor_grid,
                                "Show minor waveform grid",
                            )
                            .clicked()
                            {
                                host.request(InstrumentAction::ToggleGrid);
                            }
                            if ui
                                .add_enabled_ui(availability.marker_available, |ui| {
                                    instrument_control(
                                        ui,
                                        "+M",
                                        false,
                                        "Drop marker at cursor A on selected or nearest visible trace",
                                    )
                                })
                                .inner
                                .clicked()
                            {
                                host.request(InstrumentAction::DropMarker);
                            }

                            let hidden = host.hidden_strip_count();
                            if hidden > 0 {
                                instrument_separator(ui);
                                if instrument_control(
                                    ui,
                                    &format!("{hidden} HIDDEN"),
                                    true,
                                    "Restore closed waveform strips",
                                )
                                .clicked()
                                {
                                    host.request(InstrumentAction::RestoreStrips);
                                }
                            }
                        });
                    });
            },
        );
    });
}
