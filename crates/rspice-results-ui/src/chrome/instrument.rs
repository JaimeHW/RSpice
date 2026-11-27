//! Waveform instrument presentation over active-pane host operations.

use super::{
    ExportRequests, ResultBarMetrics, export_menu, instrument_control, instrument_segment,
    instrument_separator,
};
use egui::Ui;
use rspice_ui_kit::{
    icons::Icon,
    plot::InteractionMode,
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::{Button, IconButton},
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

/// What the bar's strip tools read for the active strip: whether one is
/// active for them to act on, its X scale, and whether a strip is maximized.
#[derive(Debug, Clone, Default)]
pub struct StripTools {
    /// Whether a strip is active for the tools to act on.
    pub active: bool,
    /// Whether the active strip's X axis is logarithmic.
    pub log_x: bool,
    /// Why the active strip's X axis cannot be logarithmic, when it cannot.
    pub log_x_refusal: Option<String>,
    /// Whether a strip stands maximized.
    pub maximized: bool,
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
    /// Open the expression editor on the active strip.
    AddExpression,
    /// Turn the active strip's X axis between linear and logarithmic.
    ToggleLogX,
    /// Maximize the active strip, or restore every strip.
    ToggleMaximize,
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
    /// The name of the strip the bar's zoom and fit act on, which their
    /// tooltips give: with several strips on the sheet, that is where the
    /// reader learns which one they will move.
    fn active_strip(&self) -> Option<String>;
    /// What the bar's strip tools (+ Expression, log X and maximize) read,
    /// on a sheet of strips; `None` where the sheet draws none.
    fn strip_tools(&self) -> Option<StripTools> {
        None
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
                    let more = Button::new(MORE)
                        .menu()
                        .show(ui)
                        .on_hover_text("The instrument controls the bar has no room for");
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

/// The plot tools a page beside a document offers on its toolbar band, at
/// the band's control height: the A/B and horizontal cursors, then zoom and
/// fit on the active strip. Box zoom and pan stay gestures (a right drag, a
/// Shift drag), and the display toggles stay on the Results sheet. The
/// cursor tools are the bar's segments, lit by the neutral fill, and the
/// glyphs its flat tools, so the band reads as the bar does.
pub fn show_plot_tools(ui: &mut Ui, host: &mut impl WaveInstrumentHost) {
    let availability = InstrumentAvailability {
        limits_available: false,
        envelope_available: false,
        marker_available: false,
    };
    let controls: Vec<BarControl> = bar_controls(&availability, &*host)
        .into_iter()
        .filter(|control| matches!(control.group, CURSOR_GROUP | ZOOM_GROUP))
        .collect();
    let height = ui.spacing().interact_size.y;
    ui.spacing_mut().item_spacing.x = CONTROL_GAP;
    let mut group = None;
    for control in &controls {
        if group.is_some_and(|group| group != control.group) {
            super::instrument_separator(ui);
        }
        group = Some(control.group);
        let response = if let ControlKind::Glyph(icon) = &control.kind {
            IconButton::new(*icon)
                .side(height)
                .on(control.on)
                .tooltip(&control.tooltip)
                .show(ui)
        } else {
            let label = control.kind.label().unwrap_or_default();
            instrument_segment(ui, label, control.on, &control.tooltip, height)
        };
        if response.clicked()
            && let Some(action) = control.action()
        {
            host.request(action);
        }
    }
}

/// The groups of the bar's controls a page beside a document offers: the
/// cursor tools, and zoom and fit.
const CURSOR_GROUP: u8 = 0;
const ZOOM_GROUP: u8 = 2;
/// The active strip's tools, + Expression and log X, after the display
/// toggles; then maximize, with the restore of closed strips beside it.
const STRIP_GROUP: u8 = 5;
const WINDOW_GROUP: u8 = 6;
/// What the bar's X scale toggle is called in its menu row and to
/// assistive technology.
const LOG_X_NAME: &str = "Logarithmic X axis";

/// The gap between two controls of the bar.
const CONTROL_GAP: f32 = 4.0;
/// The width a separator takes, its gap included.
const SEPARATOR_WIDTH: f32 = 7.0 + CONTROL_GAP;
/// The control the bar folds the controls it has no room for behind.
const MORE: &str = "More";

/// What one control of the bar is.
#[derive(Debug, Clone, PartialEq)]
enum ControlKind {
    /// A cursor tool, one segment of the bar's tool group: lit by a neutral
    /// fill, as a segmented control's segment is.
    Segment(String),
    /// A lettered toggle: the kit's bordered button, lit while it is on.
    Toggle(String),
    /// A lettered action: the kit's bordered button.
    Action(String),
    /// A glyph, drawn flat as a toolbar's tools are.
    Glyph(Icon),
    /// A glyph that latches a state of the sheet's windows, maximize: lit by
    /// the neutral fill and the full ink, as no accent is drawn for it.
    Latch(Icon),
}

impl ControlKind {
    /// The words the control reads, or none for a glyph.
    fn label(&self) -> Option<&str> {
        match self {
            Self::Segment(label) | Self::Toggle(label) | Self::Action(label) => Some(label),
            Self::Glyph(_) | Self::Latch(_) => None,
        }
    }
}

/// One control of the bar, which it draws in its row while there is room
/// and in its "More" menu once there is not.
struct BarControl {
    /// The controls of one group stand together between separators.
    group: u8,
    kind: ControlKind,
    /// What the control's menu row reads, and its tooltip on the bar.
    name: String,
    tooltip: String,
    /// What a lettered control is called to assistive technology, where its
    /// letters alone ("log X") would not say it.
    accessible: Option<&'static str>,
    /// Why the control cannot act, which it says on hover while it is off.
    unavailable: String,
    on: bool,
    enabled: bool,
    action: fn() -> Option<InstrumentAction>,
}

impl BarControl {
    fn action(&self) -> Option<InstrumentAction> {
        (self.action)()
    }

    /// The control, saying `reason` on hover while it cannot act.
    fn unavailable(mut self, reason: String) -> Self {
        self.unavailable = reason;
        self
    }

    /// `response`, saying why the control cannot act while it is off.
    fn explained(&self, response: egui::Response) -> egui::Response {
        if self.unavailable.is_empty() {
            response
        } else {
            response.on_disabled_hover_text(&self.unavailable)
        }
    }

    /// What the control says on hover while it is off: why it cannot act,
    /// or else what it does.
    fn disabled_hover(&self) -> &str {
        if self.unavailable.is_empty() {
            &self.tooltip
        } else {
            &self.unavailable
        }
    }

    /// The width the control takes in the bar, its gap included.
    fn width(&self, ui: &Ui) -> f32 {
        let side = ResultBarMetrics::of(ui).instrument_control;
        let own = match &self.kind {
            ControlKind::Glyph(_) | ControlKind::Latch(_) => side,
            ControlKind::Toggle(label) | ControlKind::Action(label) => {
                Button::new(label).measured_width(ui)
            }
            ControlKind::Segment(label) => {
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
        match &self.kind {
            ControlKind::Toggle(label) => {
                let button = Button::new(label).selected(self.on).enabled(self.enabled);
                match self.accessible {
                    Some(name) => button.accessible_label(name),
                    None => button,
                }
                .show(ui)
                .on_hover_text(&self.tooltip)
                .on_disabled_hover_text(self.disabled_hover())
            }
            ControlKind::Action(label) => Button::new(label)
                .enabled(self.enabled)
                .show(ui)
                .on_hover_text(&self.tooltip)
                .on_disabled_hover_text(self.disabled_hover()),
            ControlKind::Segment(label) => self.explained(
                ui.add_enabled_ui(self.enabled, |ui| {
                    instrument_control(ui, label, self.on, &self.tooltip)
                })
                .inner,
            ),
            ControlKind::Glyph(icon) => self.explained(
                ui.add_enabled_ui(self.enabled, |ui| {
                    IconButton::new(*icon)
                        .side(ResultBarMetrics::of(ui).instrument_control)
                        .on(self.on)
                        .tooltip(&self.tooltip)
                        .show(ui)
                })
                .inner,
            ),
            ControlKind::Latch(icon) => self.explained(
                ui.add_enabled_ui(self.enabled, |ui| {
                    IconButton::new(*icon)
                        .side(ResultBarMetrics::of(ui).instrument_control)
                        .on(self.on)
                        .plain()
                        .tooltip(&self.tooltip)
                        .show(ui)
                })
                .inner,
            ),
        }
    }

    fn show_in_menu(&self, ui: &mut Ui) -> egui::Response {
        let response = ui
            .add_enabled(
                self.enabled,
                egui::Button::selectable(self.on, self.name.as_str()),
            )
            .on_hover_text(&self.tooltip);
        self.explained(response)
    }
}

/// The bar's controls, in order, as the controls and availability read now.
fn bar_controls(
    availability: &InstrumentAvailability,
    host: &impl WaveInstrumentHost,
) -> Vec<BarControl> {
    let controls = host.controls();
    let strip = host.active_strip().map_or_else(
        || "the active strip".to_owned(),
        |name| format!("the {name} strip"),
    );
    let control = |group, kind, name: &str, tooltip: &str, on, enabled, action| BarControl {
        group,
        kind,
        name: name.to_owned(),
        tooltip: tooltip.to_owned(),
        accessible: None,
        unavailable: String::new(),
        on,
        enabled,
        action,
    };
    let mut bar = vec![
        control(
            0,
            ControlKind::Segment("Cursors".to_owned()),
            "Cursors",
            "Cursors A and B: click a pane to place them and read every curve at two X values and between them",
            controls.plot_tool == ResultPlotTool::Cursor && controls.cursor_armed,
            true,
            || Some(InstrumentAction::Cursor),
        ),
        control(
            0,
            ControlKind::Segment("Y cursors".to_owned()),
            "Y cursors",
            "Horizontal cursors: click or drag in the active pane to read Y values; click again to remove them",
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
            &format!("Zoom {strip} in 2x"),
            false,
            true,
            || Some(InstrumentAction::Zoom(0.5)),
        ),
        control(
            2,
            ControlKind::Glyph(Icon::ZoomOut),
            "Zoom out",
            &format!("Zoom {strip} out 2x"),
            false,
            true,
            || Some(InstrumentAction::Zoom(2.0)),
        ),
        control(
            2,
            ControlKind::Glyph(Icon::ZoomFit),
            "Fit",
            &format!("Fit {strip}"),
            false,
            true,
            || Some(InstrumentAction::Fit),
        ),
        control(
            3,
            ControlKind::Toggle("Limits".to_owned()),
            "Limits",
            "Show exact compatible project specification limits",
            controls.show_spec_limits,
            availability.limits_available,
            || Some(InstrumentAction::ToggleLimits),
        )
        .unavailable(format!(
            "No project specification limit applies to the signals and units of {strip}"
        )),
        control(
            3,
            ControlKind::Toggle("Envelope".to_owned()),
            "Envelope",
            "Show min/max envelope from retained family samples",
            controls.show_family_envelope,
            availability.envelope_available,
            || Some(InstrumentAction::ToggleEnvelope),
        )
        .unavailable(format!(
            "{} draws no stepped or Monte Carlo family to bound",
            capitalized(&strip)
        )),
        control(
            3,
            ControlKind::Toggle("Grid".to_owned()),
            "Grid",
            "Show minor waveform grid",
            controls.show_minor_grid,
            true,
            || Some(InstrumentAction::ToggleGrid),
        ),
        control(
            3,
            ControlKind::Action("Marker".to_owned()),
            "Drop marker",
            "Drop a marker at cursor A on the selected or nearest visible trace (Shift+M)",
            false,
            availability.marker_available,
            || Some(InstrumentAction::DropMarker),
        )
        .unavailable("Place cursor A on a trace to drop a marker there".to_owned()),
    ];
    // The active strip's own tools: what its row carried before the bar
    // took them. They act on the active strip, as zoom and fit do.
    if let Some(tools) = host.strip_tools() {
        bar.push(control(
            STRIP_GROUP,
            ControlKind::Action(crate::strip::EXPRESSION_ACTION_LABEL.to_owned()),
            crate::strip::EXPRESSION_ACTION_LABEL,
            crate::strip::EXPRESSION_ACTION_TIP,
            false,
            tools.active,
            || Some(InstrumentAction::AddExpression),
        ));
        let log_x_tip = match &tools.log_x_refusal {
            Some(reason) => format!("Logarithmic X is unavailable: {reason}"),
            None => format!("Draw {strip}'s X axis in decades"),
        };
        bar.push(BarControl {
            accessible: Some(LOG_X_NAME),
            ..control(
                STRIP_GROUP,
                ControlKind::Toggle(crate::strip::X_SCALE_LABEL.to_owned()),
                LOG_X_NAME,
                &log_x_tip,
                tools.log_x,
                tools.active && tools.log_x_refusal.is_none(),
                || Some(InstrumentAction::ToggleLogX),
            )
        });
        let (glyph, name) = if tools.maximized {
            (Icon::Restore, "Restore strips")
        } else {
            (Icon::Maximize, "Maximize strip")
        };
        bar.push(control(
            WINDOW_GROUP,
            ControlKind::Latch(glyph),
            name,
            name,
            tools.maximized,
            tools.active || tools.maximized,
            || Some(InstrumentAction::ToggleMaximize),
        ));
    }
    let hidden = host.hidden_strip_count();
    if hidden > 0 {
        let label = format!("{hidden} hidden");
        bar.push(control(
            WINDOW_GROUP,
            ControlKind::Action(label.clone()),
            &label,
            "Restore closed waveform strips",
            true,
            true,
            || Some(InstrumentAction::RestoreStrips),
        ));
    }
    bar
}

/// `text` with its first letter a capital, as a sentence opens.
fn capitalized(text: &str) -> String {
    let mut letters = text.chars();
    letters
        .next()
        .map(|first| first.to_uppercase().chain(letters).collect())
        .unwrap_or_default()
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
    let more = Button::new(MORE).menu().measured_width(ui) + CONTROL_GAP + SEPARATOR_WIDTH;
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
