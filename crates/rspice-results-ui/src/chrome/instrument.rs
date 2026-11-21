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
    /// Draws the sheet's own controls ahead of the tools; `true` when it drew any.
    fn sheet_controls(&mut self, _ui: &mut Ui) -> bool {
        false
    }
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
                ui.spacing_mut().item_spacing.x = CONTROL_GAP;
                // The sheet's own controls lead the bar and never fold; the
                // instrument controls fold into the room they leave.
                let left = ui.cursor().left();
                if host.sheet_controls(ui) {
                    instrument_separator(ui);
                }
                let room = remaining.x - (ui.cursor().left() - left);
                let controls = bar_controls(&availability, host);
                let (shown, folded) = fold(ui, &controls, room);
                let mut group = None;
                for control in &controls[..shown] {
                    if group.is_some_and(|group| group != control.group) {
                        instrument_separator(ui);
                    }
                    group = Some(control.group);
                    if control.show_on_bar(ui).clicked()
                        && let Some(action) = control.action()
                    {
                        host.request(action);
                    }
                }
                if folded {
                    instrument_separator(ui);
                    let more = instrument_control(
                        ui,
                        MORE,
                        false,
                        "The instrument controls the bar has no room for",
                    );
                    egui::Popup::menu(&more).show(|ui| {
                        let mut group = None;
                        for control in &controls[shown..] {
                            if group.is_some_and(|group| group != control.group) {
                                ui.separator();
                            }
                            group = Some(control.group);
                            if control.show_in_menu(ui).clicked()
                                && let Some(action) = control.action()
                            {
                                host.request(action);
                                ui.close();
                            }
                        }
                    });
                }
            },
        );
    });
}

/// The gap between two controls of the bar.
const CONTROL_GAP: f32 = 4.0;
/// The width a separator takes, its gap included.
const SEPARATOR_WIDTH: f32 = 7.0 + CONTROL_GAP;
/// The control the bar folds the controls it has no room for behind.
const MORE: &str = "More";

/// What one control of the bar is.
#[derive(Debug, Clone, PartialEq)]
enum ControlKind {
    /// A lettered control.
    Text(String),
    /// A glyph.
    Glyph(Icon),
}

/// One control of the bar, which it draws in its row while there is room
/// and in its "More" menu once there is not.
struct BarControl {
    /// The controls of one group stand together between separators.
    group: u8,
    kind: ControlKind,
    /// What the control's menu row reads, and its tooltip on the bar.
    name: String,
    tooltip: &'static str,
    on: bool,
    enabled: bool,
    action: fn() -> Option<InstrumentAction>,
}

impl BarControl {
    fn action(&self) -> Option<InstrumentAction> {
        (self.action)()
    }

    /// The width the control takes in the bar, its gap included.
    fn width(&self, ui: &Ui) -> f32 {
        let side = ResultBarMetrics::of(ui).instrument_control;
        let own = match &self.kind {
            ControlKind::Glyph(_) => side,
            ControlKind::Text(label) => {
                let galley = ui.painter().layout_no_wrap(
                    label.clone(),
                    theme::mono(tokens::FS_0, FontWeight::Medium),
                    egui::Color32::WHITE,
                );
                (galley.size().x + 12.0).max(26.0)
            }
        };
        own + CONTROL_GAP
    }

    fn show_on_bar(&self, ui: &mut Ui) -> egui::Response {
        ui.add_enabled_ui(self.enabled, |ui| match &self.kind {
            ControlKind::Text(label) => instrument_control(ui, label, self.on, self.tooltip),
            ControlKind::Glyph(icon) => IconButton::new(*icon)
                .side(ResultBarMetrics::of(ui).instrument_control)
                .on(self.on)
                .tooltip(self.tooltip)
                .show(ui),
        })
        .inner
    }

    fn show_in_menu(&self, ui: &mut Ui) -> egui::Response {
        ui.add_enabled(
            self.enabled,
            egui::Button::selectable(self.on, self.name.as_str()),
        )
        .on_hover_text(self.tooltip)
    }
}

/// The bar's controls, in order, as the controls and availability read now.
fn bar_controls(
    availability: &InstrumentAvailability,
    host: &impl WaveInstrumentHost,
) -> Vec<BarControl> {
    let controls = host.controls();
    let control = |group, kind, name: &str, tooltip, on, enabled, action| BarControl {
        group,
        kind,
        name: name.to_owned(),
        tooltip,
        on,
        enabled,
        action,
    };
    let mut bar = vec![
        control(
            0,
            ControlKind::Text("A|B".to_owned()),
            "A/B cursors",
            "A/B cursor tool",
            controls.plot_tool == ResultPlotTool::Cursor && controls.cursor_armed,
            true,
            || Some(InstrumentAction::Cursor),
        ),
        control(
            0,
            ControlKind::Text("H".to_owned()),
            "Horizontal cursor",
            "Horizontal cursor - click or drag in the active pane",
            controls.plot_tool == ResultPlotTool::HorizontalCursor,
            true,
            || Some(InstrumentAction::Tool(ResultPlotTool::HorizontalCursor)),
        ),
        // Box zoom and pan are gestures, so they carry glyphs; the lettered
        // controls on this bar are the named modes beside them.
        control(
            1,
            ControlKind::Glyph(Icon::BoxZoom),
            "Box zoom",
            "Box zoom - drag a region",
            controls.plot_tool == ResultPlotTool::BoxZoom,
            true,
            || Some(InstrumentAction::Tool(ResultPlotTool::BoxZoom)),
        ),
        control(
            1,
            ControlKind::Glyph(Icon::Pan),
            "Pan",
            "Pan viewport - drag the plot",
            controls.plot_tool == ResultPlotTool::Pan,
            true,
            || Some(InstrumentAction::Tool(ResultPlotTool::Pan)),
        ),
        control(
            2,
            ControlKind::Glyph(Icon::ZoomIn),
            "Zoom in",
            "Zoom active pane in 2x",
            false,
            true,
            || Some(InstrumentAction::Zoom(0.5)),
        ),
        control(
            2,
            ControlKind::Glyph(Icon::ZoomOut),
            "Zoom out",
            "Zoom active pane out 2x",
            false,
            true,
            || Some(InstrumentAction::Zoom(2.0)),
        ),
        control(
            2,
            ControlKind::Glyph(Icon::ZoomFit),
            "Fit",
            "Fit active waveform pane",
            false,
            true,
            || Some(InstrumentAction::Fit),
        ),
        control(
            3,
            ControlKind::Text("Limits".to_owned()),
            "Limits",
            "Show exact compatible project specification limits",
            controls.show_spec_limits,
            availability.limits_available,
            || Some(InstrumentAction::ToggleLimits),
        ),
        control(
            3,
            ControlKind::Text("Envelope".to_owned()),
            "Envelope",
            "Show min/max envelope from retained family samples",
            controls.show_family_envelope,
            availability.envelope_available,
            || Some(InstrumentAction::ToggleEnvelope),
        ),
        control(
            3,
            ControlKind::Text("Grid".to_owned()),
            "Grid",
            "Show minor waveform grid",
            controls.show_minor_grid,
            true,
            || Some(InstrumentAction::ToggleGrid),
        ),
        control(
            3,
            ControlKind::Text("+M".to_owned()),
            "Drop marker",
            "Drop marker at cursor A on selected or nearest visible trace",
            false,
            availability.marker_available,
            || Some(InstrumentAction::DropMarker),
        ),
    ];
    let hidden = host.hidden_strip_count();
    if hidden > 0 {
        let label = format!("{hidden} hidden");
        bar.push(control(
            4,
            ControlKind::Text(label.clone()),
            &label,
            "Restore closed waveform strips",
            true,
            true,
            || Some(InstrumentAction::RestoreStrips),
        ));
    }
    bar
}

/// How many of `controls` the bar draws in `width`, and whether the rest
/// fold behind "More": the controls in order while they fit, with room
/// kept for "More" whenever any is left out.
fn fold(ui: &Ui, controls: &[BarControl], width: f32) -> (usize, bool) {
    let widths: Vec<f32> = controls.iter().map(|control| control.width(ui)).collect();
    let separated = |index: usize| index > 0 && controls[index - 1].group != controls[index].group;
    let total: f32 = widths
        .iter()
        .enumerate()
        .map(|(index, width)| {
            width
                + if separated(index) {
                    SEPARATOR_WIDTH
                } else {
                    0.0
                }
        })
        .sum();
    if total <= width {
        return (controls.len(), false);
    }
    let more = BarControl {
        group: u8::MAX,
        kind: ControlKind::Text(MORE.to_owned()),
        name: String::new(),
        tooltip: "",
        on: false,
        enabled: true,
        action: || None,
    }
    .width(ui)
        + SEPARATOR_WIDTH;
    let mut used = 0.0;
    let mut shown = 0;
    for (index, control_width) in widths.iter().enumerate() {
        let needed = control_width
            + if separated(index) {
                SEPARATOR_WIDTH
            } else {
                0.0
            };
        if used + needed + more > width {
            break;
        }
        used += needed;
        shown = index + 1;
    }
    (shown, true)
}
