//! Waveform pane legend, visibility controls and Y-axis actions.

use super::pane::SwatchStyle;
use super::{ReadoutPolicy, StripModel, StripTrace, UnitPane, cursor_interpolation};
use crate::strip::{self, HEADER_PADDING_X};
use egui::Ui;
use rspice_results::visualization_document::{Abscissa, Strip};
use rspice_ui_kit::{
    icons::Icon,
    plot::paint_trace_swatch,
    theme,
    tokens::Tokens,
    widgets::{IconButton, chip, chip_width},
};
use rspice_vocabulary::quantity::QuantityPresentationPolicy;
use std::borrow::Cow;

pub const WAVE_PANE_HEADER_HEIGHT: f32 = 25.0;

/// How tall the pane's trace list grows before it scrolls.
const TRACE_LIST_HEIGHT: f32 = 320.0;

/// The height of the row's controls: the header chip's and the strip
/// header's icon side.
const ACTION_ICON_SIDE: f32 = 22.0;

/// Source-qualified actions applied immediately in legend order.
///
/// Selection is queried for each chip, including after an earlier chip changes it.
pub trait PaneHeaderHost {
    fn trace_selected(&self, trace: &StripTrace) -> bool;
    fn toggle_trace_visibility(&mut self, trace: &StripTrace);
    fn select_trace(&mut self, trace: &StripTrace);
    /// Add `trace` to the traces selected, or take it out, as a Ctrl-click
    /// on its legend entry asks: the selected traces are styled together.
    fn extend_selection(&mut self, trace: &StripTrace) {
        self.select_trace(trace);
    }
    fn activate_pane(&mut self);
    /// Add an expression to the strip, as a trace's menu asks.
    fn add_expression(&mut self, _text: String) {}
    /// Whether the host runs a trace's menu's style, measure, spectrum and
    /// calculator entries, which the menu then offers.
    fn offers_trace_commands(&self) -> bool {
        false
    }
    /// Run `command` on `trace`, as its menu asks.
    fn trace_command(&mut self, _trace: &StripTrace, _command: TraceCommand) {}
}

/// What a trace's menu hands the host to do with the trace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceCommand {
    /// Draw the trace, and the traces selected with it, in this style.
    Style(rspice_results::visualization_document::TraceStyle),
    /// Measure the trace.
    Measure,
    /// Take the trace's spectrum.
    Spectrum,
    /// Hand the trace to the waveform calculator.
    SendToCalculator,
}

/// One of the strip's expressions on this pane, as its legend entry states
/// it: a pane names every curve it draws, an expression's as a signal's.
pub struct PaneExpression {
    /// Its place in the strip's expression list.
    pub slot: usize,
    /// The expression as typed, which a conversion of it reads.
    pub text: String,
    /// The name its entry gives it.
    pub label: String,
    pub color: egui::Color32,
    /// Its dash and marker, as the pane draws its curve.
    pub swatch: SwatchStyle,
    /// Whether it is drawn. A hidden one waits in the pane's trace list,
    /// as a hidden signal does.
    pub visible: bool,
    /// What it reads at cursor A, in the pane's unit, when A is placed.
    pub at_cursor: Option<String>,
    /// How many curves it draws: one for each member of a family.
    pub curves: usize,
    /// Its values, where Digital's threshold starts.
    pub values: rspice_results::waveform::SharedWaveformValues,
}

/// Display state qualified by the host for this pane and its own cursor domain.
pub struct PaneHeader {
    pub height: f32,
    pub active: bool,
    /// Whether the legend reads in the dim text colour: on a sheet of
    /// several strips, every strip but the active one, so the strip the
    /// instrument bar acts on is the one whose names read at full contrast.
    pub dimmed: bool,
    pub log_y: bool,
    pub log_y_available: bool,
    pub cursor_a: Option<(f64, ReadoutPolicy, QuantityPresentationPolicy)>,
    /// The strip's expressions on this pane, drawn ones first.
    pub expressions: Vec<PaneExpression>,
    /// Whether the pane draws anything. A pane with nothing to draw is its
    /// header alone, which offers no autoscale and no log toggle for an axis
    /// it does not draw.
    pub plotted: bool,
    /// What the pane's curves may be plotted against, when the host offers
    /// a choice.
    pub abscissa: Option<AbscissaMenu>,
    /// How many window controls the strip header above keeps: the row's fit
    /// icon stands under the leftmost, its scale chip on the same rule.
    pub window_slots: usize,
    /// Whether the row arranges the stack: its band drags the strip up or
    /// down and its menu splits and merges strips; a legend entry drags onto
    /// another strip and its menu moves the curve.
    pub arrange: bool,
    /// The strip's axes there is no room to draw beside its plot, each
    /// named by its unit and its curves: the trace list names them.
    pub folded_axes: Vec<String>,
}

/// What a unit pane's curves may be plotted against: one of the strip's
/// signals, one of its expressions, or an expression typed here.
#[derive(Debug, Clone, Default)]
pub struct AbscissaMenu {
    /// What the curves are plotted against already, in a strip drawn against
    /// a signal or an expression.
    pub current: Option<Abscissa>,
    /// The strip's signals, by source name: the pane's own first.
    pub signals: Vec<String>,
    /// The strip's expressions.
    pub expressions: Vec<String>,
}

/// The choices of an [`AbscissaMenu`] under a heading naming the pane's
/// curves, with a field to type an expression. The pane header's context
/// menu and the X axis of a strip drawn against a signal both open this.
pub(crate) fn abscissa_choices(
    ui: &mut Ui,
    unit: &str,
    menu: &AbscissaMenu,
    display_name: &dyn Fn(&str) -> String,
    draft_id: egui::Id,
) -> Option<Abscissa> {
    let mut chosen = None;
    let heading = if unit.is_empty() {
        "Plot the curves against".to_owned()
    } else {
        format!("Plot the {unit} curves against")
    };
    ui.label(egui::RichText::new(heading).weak());
    let signals = menu
        .signals
        .iter()
        .map(|name| Abscissa::Signal { name: name.clone() });
    let expressions = menu
        .expressions
        .iter()
        .map(|text| Abscissa::Expression { text: text.clone() });
    let options: Vec<Abscissa> = signals.chain(expressions).collect();
    egui::ScrollArea::vertical()
        .max_height(240.0)
        .show(ui, |ui| {
            for option in options {
                let label = match &option {
                    Abscissa::Signal { name } => display_name(name),
                    Abscissa::Expression { text } => text.clone(),
                };
                if ui
                    .selectable_label(menu.current.as_ref() == Some(&option), label)
                    .clicked()
                {
                    chosen = Some(option);
                }
            }
        });
    ui.separator();
    let mut draft = ui.data_mut(|data| data.get_temp::<String>(draft_id).unwrap_or_default());
    let field = ui.add(
        egui::TextEdit::singleline(&mut draft)
            .hint_text("Expression, e.g. V(a)-V(b)")
            .desired_width(200.0),
    );
    let entered = field.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter));
    if entered && !draft.trim().is_empty() {
        chosen = Some(Abscissa::Expression {
            text: draft.trim().to_owned(),
        });
        draft.clear();
    }
    ui.data_mut(|data| data.insert_temp(draft_id, draft));
    if chosen.is_some() {
        ui.close();
    }
    chosen
}

pub use rspice_ui_kit::panels::elide_chars as elide;

#[derive(Default)]
pub struct UnitPaneHeaderResponse {
    pub autoscale_y: bool,
    pub toggle_log_y: bool,
    /// The expression, by its place in the strip's list, whose entry was
    /// hidden or shown.
    pub toggle_expression: Option<usize>,
    /// The expression whose entry's Remove was chosen.
    pub remove_expression: Option<usize>,
    /// What the reader chose to plot the pane's curves against.
    pub abscissa: Option<Abscissa>,
    /// The strips a strip or curve menu entry the reader chose arranges the
    /// stack into.
    pub arrange: Option<Vec<Strip>>,
}

/// Where the legend runs in a pane header spanning `rect` whose trailing
/// controls take `action_width` from its right inset.
///
/// Arithmetic rather than inline layout because the edges are the whole
/// point: this row and the strip header above it are two rows of chrome in
/// one card, and both start and stop [`HEADER_PADDING_X`] in from the pane's
/// edges. A function can be asserted about without standing up a strip, a
/// host and a projection.
fn legend_region(rect: egui::Rect, action_width: f32) -> egui::Rect {
    // The inset is a floor as well as a start: a pane narrower than its two
    // insets must not hand the legend a rectangle whose right edge is left
    // of its left one, which clips to nothing and reads as a widget that
    // vanished rather than one that had no room.
    let left = (rect.left() + HEADER_PADDING_X).min(rect.right());
    let actions_left = ((rect.right() - HEADER_PADDING_X).max(left) - action_width).max(left);
    egui::Rect::from_min_max(
        egui::pos2(left, rect.top()),
        egui::pos2((actions_left - 3.0).max(left), rect.bottom()),
    )
}

/// Draw the legend using design-aware names while keeping source identities intact.
pub fn show_header(
    ui: &mut Ui,
    model: &StripModel,
    pane: &UnitPane,
    input: PaneHeader,
    display_name: impl for<'a> Fn(&'a str) -> Cow<'a, str>,
    host: &mut impl PaneHeaderHost,
) -> UnitPaneHeaderResponse {
    let PaneHeader {
        height,
        active,
        dimmed,
        log_y,
        log_y_available,
        cursor_a: cursor_a_value,
        expressions,
        plotted,
        abscissa,
        window_slots,
        arrange,
        folded_axes,
    } = input;
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let height = height.clamp(0.0, WAVE_PANE_HEADER_HEIGHT);
    // The band is the pane's as much as its plot is: a click anywhere on it
    // that no control of the row takes makes the pane the active one, as a
    // click on the plot does, and its context menu is where the pane is
    // drawn against another signal. The row's own controls are laid out over
    // it and take their clicks first.
    let (rect, band) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        if arrange {
            egui::Sense::click_and_drag()
        } else {
            egui::Sense::click()
        },
    );
    if arrange {
        // Dragged, the band carries its strip to another place in the stack.
        band.dnd_set_drag_payload(super::strip_drag::HeaderDrag {
            analysis: model.analysis_key,
            identity: pane.unit.to_owned(),
        });
    }
    band.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::Button,
            true,
            active,
            format!("{} pane", pane_name(pane)),
        )
    });
    if band.clicked() || band.secondary_clicked() {
        host.activate_pane();
    }
    ui.painter().rect_filled(rect, 0.0, c.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, c.border),
    );
    if height < 18.0 {
        return UnitPaneHeaderResponse::default();
    }

    // In a strip drawn against a signal, that signal is the X axis, not a
    // curve of the legend.
    let display = |name: &str| display_name(name).into_owned();
    let drawn_against = match abscissa.as_ref().and_then(|menu| menu.current.as_ref()) {
        Some(Abscissa::Signal { name }) => Some(name.as_str()),
        _ => None,
    };
    // The pane's unit is its Y axis' caption, not a tag repeated here: the
    // row starts with the legend.
    let mut output = UnitPaneHeaderResponse::default();
    // The right-hand controls are laid out first and take the width they
    // need, so the legend is measured against what is left of the row and no
    // chip of it runs under them.
    let action_width = if plotted {
        // The row ends as the strip header above it does: the fit icon in
        // the place of the strip's maximize control, and the axis' scale
        // chip on the rule the strip's "log X" ends on, so every scale
        // toggle of the strip stands in one right-hand column.
        let icon = strip::window_control_rect(rect, window_slots.max(1) - 1);
        let chip_right = strip::scale_chip_right(rect, window_slots);
        let chip_width = chip_width(ui, "log Y");
        let centred = |left: f32, width: f32| {
            egui::Rect::from_center_size(
                egui::pos2(left + 0.5 * width, rect.center().y),
                egui::vec2(width, ACTION_ICON_SIDE),
            )
        };
        let fit = ui
            .scope_builder(
                egui::UiBuilder::new()
                    .max_rect(centred(icon.left(), icon.width()))
                    .id_salt("pane-fit-y"),
                |ui| {
                    IconButton::new(Icon::ZoomFit)
                        .side(ACTION_ICON_SIDE)
                        .tooltip("Fit Y to the traces in view")
                        .show(ui)
                },
            )
            .inner;
        if fit.clicked() {
            output.autoscale_y = true;
        }
        let log = ui
            .scope_builder(
                egui::UiBuilder::new()
                    .max_rect(centred(chip_right - chip_width, chip_width))
                    .id_salt("pane-log-y"),
                |ui| {
                    ui.add_enabled_ui(log_y_available, |ui| chip(ui, "log Y", log_y))
                        .inner
                },
            )
            .inner
            .on_hover_text(if log_y_available {
                "Toggle logarithmic Y axis"
            } else {
                "Logarithmic Y needs a positive value to draw"
            });
        if log.clicked() {
            output.toggle_log_y = true;
        }
        (rect.right() - HEADER_PADDING_X) - (chip_right - chip_width)
    } else {
        // A pane with nothing plotted draws no axis controls, and its legend
        // has the row.
        0.0
    };
    let legend_rect = legend_region(rect, action_width);

    // The row names exactly what the pane draws, the same traces its fit
    // and its curves read, and offers the hidden signals of its unit. A
    // magnitude's phase is drawn in the phase pane, so it is named there,
    // and the signal a pane is drawn against is its X, not a curve.
    let curve = |trace: &&StripTrace| {
        !drawn_against
            .is_some_and(|name| super::xy::is_own_abscissa(&trace.source_waveform_name, name))
    };
    let mut plotted_traces: Vec<&StripTrace> = pane
        .all_traces()
        .filter(|index| *index < model.signal_trace_count)
        .filter_map(|index| model.traces.get(index))
        .filter(curve)
        .collect();
    let mut hidden: Vec<&StripTrace> = model
        .traces
        .iter()
        .take(model.signal_trace_count)
        .filter(|trace| {
            !trace.visible && !pane.unlabeled && model.trace_unit(trace) == pane.quantity_unit()
        })
        .filter(curve)
        .collect();
    sweep_members_in_value_order(&mut plotted_traces, |trace| &trace.source_waveform_name);
    sweep_members_in_value_order(&mut hidden, |trace| &trace.source_waveform_name);
    let (drawn_expressions, hidden_expressions): (Vec<_>, Vec<_>) = expressions
        .iter()
        .partition(|expression| expression.visible);
    // The pane names every curve it draws: its signals, then the strip's
    // expressions routed to it.
    let entries: Vec<Entry<'_>> = plotted_traces
        .iter()
        .map(|trace| Entry::Signal(trace))
        .chain(drawn_expressions.iter().map(|e| Entry::Expression(e)))
        .collect();
    // A name gives way only past a share of the row: hierarchical names keep
    // their leaf, cut from the middle as ViVA cuts them.
    let name_budget = (legend_rect.width() * 0.4).max(72.0);
    // The row names the entries that fit and counts the rest, so the entries
    // are measured in order only until the measured ones fill it: a pane of a
    // million curves measures the handful of names its row can hold.
    let mut chips: Vec<LegendEntry> = Vec::new();
    let mut measured = 0.0;
    for entry in &entries {
        if measured > legend_rect.width() {
            break;
        }
        let chip = match entry {
            Entry::Signal(trace) => legend_entry(
                ui,
                model,
                trace,
                &cursor_a_value,
                name_budget,
                &display_name,
            ),
            Entry::Expression(expression) => expression_entry(ui, expression, name_budget),
        };
        measured += chip.width;
        chips.push(chip);
    }

    // Whatever the row cannot name waits behind one control, the pane's
    // trace list, right after the last entry that fits: the drawn curves
    // there is no room for, and the ones not drawn. The far right of the row
    // is the strip's column of scale and window controls, not the legend's.
    let hidden_count = hidden.len() + hidden_expressions.len();
    let hidden_entries: Vec<Entry<'_>> = hidden
        .iter()
        .map(|trace| Entry::Signal(trace))
        .chain(hidden_expressions.iter().map(|e| Entry::Expression(e)))
        .collect();
    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(legend_rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
        |ui| {
            ui.set_clip_rect(legend_rect);
            // The entries take the header controls' height. The theme's
            // control height is taller than the band, and a row of them
            // grew past it and sat a pixel and a half below its centre.
            let spacing = ui.spacing_mut();
            spacing.item_spacing.x = 3.0;
            spacing.interact_size.y = ACTION_ICON_SIDE;
            spacing.button_padding.y = spacing.button_padding.y.min(2.0);
            let folded = folded_axes.len();
            let fitted = fitted_chips(&chips, entries.len(), legend_rect.width(), |overflow| {
                trace_list_label(overflow, hidden_count, folded)
                    .map_or(0.0, |label| chip_width(ui, &label) + 3.0)
            });
            for (entry, chip) in entries.iter().zip(&chips).take(fitted) {
                entry.show(
                    ui,
                    chip,
                    (model, arrange, dimmed),
                    &display_name,
                    host,
                    &mut output,
                );
            }
            let Some(label) = trace_list_label(entries.len() - fitted, hidden_count, folded) else {
                return;
            };
            // Centred on the band as the scale chip and the icon are, and
            // under its own id, so the list stays open while a toggle in it
            // moves an entry onto or off the row in front of it.
            let width = chip_width(ui, &label);
            let slot = egui::Rect::from_center_size(
                egui::pos2(ui.cursor().left() + 0.5 * width, legend_rect.center().y),
                egui::vec2(width, ACTION_ICON_SIDE),
            );
            let list = ui
                .scope_builder(
                    egui::UiBuilder::new()
                        .max_rect(slot)
                        .id_salt("pane-trace-list"),
                    |ui| chip(ui, &label, false),
                )
                .inner
                .on_hover_text(format!("Traces of the {} pane", pane_name(pane)));
            egui::Popup::menu(&list)
                .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
                .show(|ui| {
                    // The axes the strip has no room to draw lead: their
                    // curves are drawn, but read against nothing beside
                    // the plot until the strip is wider.
                    if !folded_axes.is_empty() {
                        ui.weak("No room beside the plot for");
                        for axis in &folded_axes {
                            ui.label(axis);
                        }
                        ui.separator();
                    }
                    // However many curves the pane has, the list finds one:
                    // typed letters narrow it, and it scrolls.
                    let filter_id = list.id.with("filter");
                    let mut filter = ui
                        .data_mut(|data| data.get_temp::<String>(filter_id))
                        .unwrap_or_default();
                    ui.add(
                        egui::TextEdit::singleline(&mut filter)
                            .hint_text("Filter traces")
                            .desired_width(200.0),
                    );
                    ui.data_mut(|data| data.insert_temp(filter_id, filter.clone()));
                    let needle = filter.trim().to_lowercase();
                    let matches = |entry: &&Entry<'_>| {
                        needle.is_empty()
                            || entry
                                .listed_name(&display_name)
                                .to_lowercase()
                                .contains(&needle)
                    };
                    let shown: Vec<&Entry<'_>> = entries.iter().filter(matches).collect();
                    let waiting: Vec<&Entry<'_>> = hidden_entries.iter().filter(matches).collect();
                    egui::ScrollArea::vertical()
                        .max_height(TRACE_LIST_HEIGHT)
                        .show(ui, |ui| {
                            for entry in &shown {
                                trace_list_row(ui, entry, &display_name, host, &mut output);
                            }
                            if !shown.is_empty() && !waiting.is_empty() {
                                ui.separator();
                            }
                            for entry in &waiting {
                                trace_list_row(ui, entry, &display_name, host, &mut output);
                            }
                            if shown.is_empty() && waiting.is_empty() {
                                ui.weak("No trace matches");
                            }
                        });
                });
        },
    );
    if abscissa.is_some() || arrange {
        let draft_id = ui
            .id()
            .with(("abscissa-menu-draft", model.analysis_key, pane.unit));
        let strip = model
            .unit_panes()
            .iter()
            .position(|held| held.unit == pane.unit);
        if let Some((strips, chosen)) = crate::context_menu::context_menu(&band, |ui| {
            let strips = strip
                .filter(|_| arrange)
                .and_then(|index| super::strip_drag::strip_menu(ui, model, index));
            let chosen = abscissa.as_ref().and_then(|menu| {
                if arrange {
                    ui.separator();
                }
                abscissa_choices(ui, pane.quantity_unit(), menu, &display, draft_id)
            });
            (strips, chosen)
        })
        .map(|inner| inner.inner)
        {
            output.arrange = strips;
            output.abscissa = chosen;
        }
    }
    output
}

/// How the row names its pane to a reader: by its unit, or as the pane of an
/// expression whose unit cannot be stated, or of plain numbers.
fn pane_name<'a>(pane: &UnitPane<'a>) -> &'a str {
    if pane.unlabeled {
        crate::waves::pane::UNKNOWN_UNIT
    } else if pane.axis.is_empty() {
        crate::waves::pane::NO_UNIT
    } else {
        pane.axis
    }
}

/// One plotted trace's legend chip, measured before it is laid out so the
/// row can tell how many fit.
struct LegendEntry {
    label: String,
    width: f32,
}

const LEGEND_SWATCH: egui::Vec2 = egui::vec2(18.0, 19.0);

fn legend_entry(
    ui: &Ui,
    model: &StripModel,
    trace: &StripTrace,
    cursor_a: &Option<(f64, ReadoutPolicy, QuantityPresentationPolicy)>,
    name_budget: f32,
    display_name: &impl for<'a> Fn(&'a str) -> Cow<'a, str>,
) -> LegendEntry {
    let font = egui::TextStyle::Button.resolve(ui.style());
    let shown = display_name(&trace.name);
    let name = rspice_ui_kit::panels::elide_middle(ui, &shown, &font, name_budget);
    // The instrument idiom: a trace states its own value at cursor A right
    // where its name is, so reading one curve never costs a trip to the
    // readout register.
    let label = match cursor_a {
        Some((x, presentation, policy)) => {
            let digits = usize::from(presentation.displayed_significant_digits().get());
            let value = super::trace_value_at(
                trace,
                *x,
                cursor_interpolation(presentation.cursor_interpolation()),
            );
            format!(
                "{name}  {}",
                model.format_trace_value(trace, value, digits, *policy)
            )
        }
        None => name,
    };
    let width = LEGEND_SWATCH.x + 3.0 + control_width(ui, &label) + 3.0;
    LegendEntry { label, width }
}

/// One entry of a pane's legend: a signal, or one of the strip's expressions.
enum Entry<'a> {
    Signal(&'a StripTrace),
    Expression(&'a PaneExpression),
}

impl Entry<'_> {
    /// The name the pane's trace list gives the entry, which its filter
    /// reads.
    fn listed_name(&self, display_name: &impl for<'a> Fn(&'a str) -> Cow<'a, str>) -> String {
        match self {
            Self::Signal(trace) => display_name(&trace.name).into_owned(),
            Self::Expression(expression) => expression.label.clone(),
        }
    }

    fn show(
        &self,
        ui: &mut Ui,
        chip: &LegendEntry,
        (model, arrange, dimmed): (&StripModel, bool, bool),
        display_name: &impl for<'a> Fn(&'a str) -> Cow<'a, str>,
        host: &mut impl PaneHeaderHost,
        output: &mut UnitPaneHeaderResponse,
    ) {
        // The curve the entry stands for, when the row arranges the stack.
        let curve = arrange
            .then(|| match self {
                Self::Signal(trace) => model
                    .traces
                    .iter()
                    .position(|held| std::ptr::eq(held, *trace))
                    .map(super::strip_layout::Curve::Trace),
                Self::Expression(expression) => {
                    Some(super::strip_layout::Curve::Expression(expression.slot))
                }
            })
            .flatten()
            .map(|curve| (model, curve));
        match self {
            Self::Signal(trace) => {
                legend_chip(ui, (trace, chip, dimmed), curve, display_name, host, output)
            }
            Self::Expression(expression) => {
                expression_chip(ui, (expression, chip, dimmed), curve, host, output);
            }
        }
    }
}

/// A nested sweep's member, as its columns name it: `I(VD) [VG=1.5; point
/// 2]` is the member of `I(VD)` at VG = 1.5. `None` for any other name.
fn sweep_member(name: &str) -> Option<(&str, &str, f64)> {
    let (base, rest) = name.split_once(" [")?;
    let (assignment, point) = rest.strip_suffix(']')?.split_once("; point ")?;
    point.parse::<usize>().ok()?;
    let (source, value) = assignment.split_once('=')?;
    Some((base, source, value.parse::<f64>().ok()?))
}

/// Put the members of each nested sweep in `items` in the order of their
/// parameter value, numerically, in the places the members already hold:
/// VG = 1, 1.5, 2 rather than the 1.5, 1, 2 their names sort to. Every
/// other item keeps its place.
pub(crate) fn sweep_members_in_value_order<T: Copy>(items: &mut [T], name: impl Fn(&T) -> &str) {
    // Each item's family (its base name and swept source) and value, owned:
    // the items move while these are read.
    let members: Vec<Option<(String, f64)>> = items
        .iter()
        .map(|item| {
            sweep_member(name(item))
                .map(|(base, source, value)| (format!("{base}\0{source}"), value))
        })
        .collect();
    let mut placed = vec![false; items.len()];
    for first in 0..items.len() {
        let Some((family, _)) = &members[first] else {
            continue;
        };
        if placed[first] {
            continue;
        }
        let places: Vec<usize> = (first..items.len())
            .filter(|index| {
                members[*index]
                    .as_ref()
                    .is_some_and(|(other, _)| other == family)
            })
            .collect();
        let value = |index: usize| members[index].as_ref().map_or(0.0, |member| member.1);
        let mut order = places.clone();
        order.sort_by(|a, b| value(*a).total_cmp(&value(*b)).then(a.cmp(b)));
        let sorted: Vec<T> = order.iter().map(|index| items[*index]).collect();
        for (place, item) in places.iter().zip(sorted) {
            items[*place] = item;
            placed[*place] = true;
        }
    }
}

/// An expression's entry, measured as a signal's is: its name, its value at
/// cursor A, and how many curves it stands for when it draws a family.
fn expression_entry(ui: &Ui, expression: &PaneExpression, name_budget: f32) -> LegendEntry {
    let font = egui::TextStyle::Button.resolve(ui.style());
    let name = rspice_ui_kit::panels::elide_middle(ui, &expression.label, &font, name_budget);
    let label = match &expression.at_cursor {
        Some(value) => format!("{name}  {value}"),
        None => name,
    };
    let label = if expression.curves > 1 {
        format!("{label}  ×{}", expression.curves)
    } else {
        label
    };
    let width = LEGEND_SWATCH.x + 3.0 + control_width(ui, &label) + 3.0;
    LegendEntry { label, width }
}

/// An expression's chip, as a signal's: its swatch hides it, its name
/// activates the pane, and its menu offers Digital, as a trace's does, and
/// Remove. The expression as typed is on hover.
fn expression_chip(
    ui: &mut Ui,
    (expression, entry, dimmed): (&PaneExpression, &LegendEntry, bool),
    curve: Option<(&StripModel, super::strip_layout::Curve)>,
    host: &mut impl PaneHeaderHost,
    output: &mut UnitPaneHeaderResponse,
) {
    let t = Tokens::get(ui.ctx());
    let (swatch, swatch_response) = ui.allocate_exact_size(LEGEND_SWATCH, egui::Sense::click());
    paint_trace_swatch(
        ui.painter(),
        swatch,
        expression.color,
        expression.swatch.look(t.color_safe_traces),
        t.color.bg_panel,
    );
    swatch_response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::Button,
            true,
            true,
            format!("Toggle {} visibility", expression.text),
        )
    });
    theme::paint_focus_ring(ui, &swatch_response, swatch);
    if swatch_response
        .on_hover_text(format!("Hide {}", expression.text))
        .clicked()
    {
        output.toggle_expression = Some(expression.slot);
    }
    let label = ui
        .selectable_label(false, legend_text(ui, &entry.label, dimmed))
        .on_hover_text(&expression.text);
    let label = drag_curve(label, curve);
    if label.clicked() {
        host.activate_pane();
    }
    crate::context_menu::context_menu(&label, |ui| {
        if let Some((model, curve)) = curve {
            output.arrange = output
                .arrange
                .take()
                .or_else(|| super::strip_drag::curve_menu(ui, model, curve));
            ui.separator();
        }
        rspice_ui_kit::widgets::menu_button(ui, "Digital", |ui| {
            if let Some(text) = super::logic::convert::digital_form_of(ui, &expression.text, || {
                expression
                    .values
                    .fetch_whole()
                    .ready()
                    .and_then(|values| super::logic::convert::mid_range(&values))
            }) {
                host.add_expression(text);
            }
        });
        ui.separator();
        if ui.button("Remove expression").clicked() {
            output.remove_expression = Some(expression.slot);
            ui.close();
        }
    });
}

/// The width a button-like control labelled `label` takes.
fn control_width(ui: &Ui, label: &str) -> f32 {
    // Measured as egui lays a button out: its label in the widget text font
    // (the body face unless the style overrides it, not the `Button` text
    // style) inside its frame's margins, at the widest of its states. The
    // `Button` style measured every chip a twelfth narrow, so a crowded row
    // ran past its end and cut the control after it short.
    use egui::widget_style::{Classes, WidgetState};
    let style = ui.style();
    let classes = Classes::default();
    let margin = [
        WidgetState::Inactive,
        WidgetState::Hovered,
        WidgetState::Active,
    ]
    .into_iter()
    .map(|state| {
        let margin = style.button_style(&classes, state).frame.inner_margin;
        f32::from(margin.left) + f32::from(margin.right)
    })
    .fold(0.0, f32::max);
    let font = style
        .button_style(&classes, WidgetState::Inactive)
        .text_style
        .font_id;
    ui.painter()
        .layout_no_wrap(label.to_owned(), font, egui::Color32::WHITE)
        .size()
        .x
        + margin
}

/// How many of `chips`, the measured first of the row's `count` entries, the
/// row shows in `width`: as many as leave room for the trace list after them,
/// whose width for `n` entries left over is `list_width(n)`, nothing when it
/// is not drawn.
fn fitted_chips(
    chips: &[LegendEntry],
    count: usize,
    width: f32,
    list_width: impl Fn(usize) -> f32,
) -> usize {
    let mut used = 0.0;
    for (index, chip) in chips.iter().enumerate() {
        if used + chip.width + list_width(count - index - 1) > width {
            return index;
        }
        used += chip.width;
    }
    chips.len()
}

/// The trace list's label: "+N" for drawn curves the row has no room for,
/// "M hidden" for the ones not drawn and "+K axes" for the strip's axes
/// with no room beside the plot, whichever apply; none when the row names
/// every curve, none is hidden and every axis is drawn.
fn trace_list_label(overflow: usize, hidden: usize, folded_axes: usize) -> Option<String> {
    let parts: Vec<String> = [
        (overflow > 0).then(|| format!("+{overflow}")),
        (hidden > 0).then(|| format!("{hidden} hidden")),
        match folded_axes {
            0 => None,
            1 => Some("+1 axis".to_owned()),
            count => Some(format!("+{count} axes")),
        },
    ]
    .into_iter()
    .flatten()
    .collect();
    (!parts.is_empty()).then(|| parts.join(" · "))
}

/// One row of a pane's trace list: whether it is drawn, its swatch and its
/// whole name. Its box and its name both toggle it, and the list stays open.
fn trace_list_row(
    ui: &mut Ui,
    entry: &Entry<'_>,
    display_name: &impl for<'a> Fn(&'a str) -> Cow<'a, str>,
    host: &mut impl PaneHeaderHost,
    output: &mut UnitPaneHeaderResponse,
) {
    let t = Tokens::get(ui.ctx());
    let (name, color, look, visible, typed) = match entry {
        Entry::Signal(trace) => (
            display_name(&trace.name).into_owned(),
            trace.color,
            SwatchStyle::of(trace).look(t.color_safe_traces),
            trace.visible,
            None,
        ),
        Entry::Expression(expression) => (
            expression.label.clone(),
            expression.color,
            expression.swatch.look(t.color_safe_traces),
            expression.visible,
            Some(expression.text.as_str()),
        ),
    };
    let toggled = ui
        .horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 4.0;
            let mut shown = visible;
            let check = ui.checkbox(&mut shown, "");
            check.widget_info(|| {
                egui::WidgetInfo::selected(egui::WidgetType::Checkbox, true, visible, &name)
            });
            let (swatch, _) = ui.allocate_exact_size(LEGEND_SWATCH, egui::Sense::hover());
            paint_trace_swatch(ui.painter(), swatch, color, look, t.color.bg_panel);
            let mut label = ui.add(
                egui::Label::new(name.as_str())
                    .extend()
                    .sense(egui::Sense::click()),
            );
            if let Some(typed) = typed.filter(|typed| *typed != name) {
                label = label.on_hover_text(typed);
            }
            check.clicked() || label.clicked()
        })
        .inner;
    if toggled {
        match entry {
            Entry::Signal(trace) => host.toggle_trace_visibility(trace),
            Entry::Expression(expression) => output.toggle_expression = Some(expression.slot),
        }
    }
}

/// A plotted trace's chip: its swatch, which hides it, then its name and
/// value, which select it. The full name is on hover.
/// A legend entry's name: in the dim text colour on a strip that is not the
/// active one, as the row's own ink otherwise.
fn legend_text(ui: &Ui, label: &str, dimmed: bool) -> egui::RichText {
    let text = egui::RichText::new(label);
    if dimmed {
        text.color(Tokens::get(ui.ctx()).color.text_dim)
    } else {
        text
    }
}

fn legend_chip(
    ui: &mut Ui,
    (trace, entry, dimmed): (&StripTrace, &LegendEntry, bool),
    curve: Option<(&StripModel, super::strip_layout::Curve)>,
    display_name: &impl for<'a> Fn(&'a str) -> Cow<'a, str>,
    host: &mut impl PaneHeaderHost,
    output: &mut UnitPaneHeaderResponse,
) {
    let t = Tokens::get(ui.ctx());
    let (swatch, swatch_response) = ui.allocate_exact_size(LEGEND_SWATCH, egui::Sense::click());
    paint_trace_swatch(
        ui.painter(),
        swatch,
        trace.color,
        SwatchStyle::of(trace).look(t.color_safe_traces),
        t.color.bg_panel,
    );
    let shown = display_name(&trace.name);
    swatch_response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::Button,
            true,
            trace.visible,
            format!("Toggle {shown} visibility"),
        )
    });
    theme::paint_focus_ring(ui, &swatch_response, swatch);
    if swatch_response
        .on_hover_text(format!("Hide {shown}"))
        .clicked()
    {
        host.toggle_trace_visibility(trace);
    }
    let selected = host.trace_selected(trace);
    let label = ui
        .selectable_label(selected, legend_text(ui, &entry.label, dimmed))
        .on_hover_text(&*shown);
    let label = drag_curve(label, curve);
    if label.clicked() {
        if ui.input(|input| input.modifiers.command) {
            host.extend_selection(trace);
        } else {
            host.select_trace(trace);
        }
    }
    crate::context_menu::context_menu(&label, |ui| {
        trace_menu(ui, trace, curve, host, output);
    });
}

/// A plotted trace's menu: hide it, move it to another strip or axis,
/// measure it, take its spectrum, hand it to the calculator, read it as a
/// logic signal, or take it off the plot, as ViVA's trace menu offers.
fn trace_menu(
    ui: &mut Ui,
    trace: &StripTrace,
    curve: Option<(&StripModel, super::strip_layout::Curve)>,
    host: &mut impl PaneHeaderHost,
    output: &mut UnitPaneHeaderResponse,
) {
    if ui.button("Hide").clicked() {
        host.toggle_trace_visibility(trace);
        ui.close();
    }
    if host.offers_trace_commands() {
        let mut style = None;
        rspice_ui_kit::widgets::menu_button(ui, "Style", |ui| {
            style = super::style_menu::style_menu(ui, trace.style);
        });
        if let Some(style) = style {
            host.trace_command(trace, TraceCommand::Style(style));
        }
    }
    if let Some((model, curve)) = curve {
        output.arrange = output
            .arrange
            .take()
            .or_else(|| super::strip_drag::curve_menu(ui, model, curve));
    }
    ui.separator();
    if host.offers_trace_commands() {
        let mut command = None;
        if ui.button("Measure\u{2026}").clicked() {
            command = Some(TraceCommand::Measure);
        }
        // A spectrum is taken of a waveform over time.
        let over_time = curve.is_some_and(|(model, _)| {
            model.analysis_type == rspice_results::analysis_type::AnalysisType::Transient
        });
        if over_time && ui.button("Spectrum").clicked() {
            command = Some(TraceCommand::Spectrum);
        }
        if ui.button("Send to calculator").clicked() {
            command = Some(TraceCommand::SendToCalculator);
        }
        if let Some(command) = command {
            host.trace_command(trace, command);
            ui.close();
        }
    }
    rspice_ui_kit::widgets::menu_button(ui, "Digital", |ui| {
        if let Some(text) =
            super::logic::convert::digital_form(ui, &trace.source_waveform_name, || {
                trace_middle(trace)
            })
        {
            host.add_expression(text);
        }
    });
    if let Some((model, curve)) = curve {
        ui.separator();
        let remove = ui.button("Remove").on_hover_text(
            "Take the curve off the plot; plotted again from the trace list, it goes back to its unit's strip",
        );
        if remove.clicked() {
            if let Ok(strips) = super::strip_layout::remove(model, curve) {
                output.arrange = Some(strips);
            }
            host.toggle_trace_visibility(trace);
            ui.close();
        }
    }
}

/// A legend entry's name, which a drag carries onto another strip or
/// between two when the row arranges the stack.
fn drag_curve(
    label: egui::Response,
    curve: Option<(&StripModel, super::strip_layout::Curve)>,
) -> egui::Response {
    let Some((model, curve)) = curve else {
        return label;
    };
    let label = label.interact(egui::Sense::click_and_drag());
    label.dnd_set_drag_payload(super::strip_drag::CurveDrag {
        analysis: model.analysis_key,
        curve,
    });
    if label.dragged() {
        label.ctx.set_cursor_icon(egui::CursorIcon::Grabbing);
    }
    label
}

/// The middle of `trace`'s range, where its `digital()` threshold starts. A
/// stored trace's comes from the store's summaries, without reading its column.
pub(super) fn trace_middle(trace: &StripTrace) -> Option<f64> {
    match trace.y.extent_over(&trace.x, None) {
        Some(extent) if extent.min <= extent.max => Some(0.5 * (extent.min + extent.max)),
        Some(_) => None,
        None => trace
            .y
            .fetch_whole()
            .ready()
            .and_then(|values| super::logic::convert::mid_range(&values)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A nested sweep's members read in the order of their parameter value,
    /// not of their names, in the places they held; the other quantity's
    /// members and a plain signal keep theirs.
    #[test]
    fn a_sweeps_members_read_in_the_order_of_their_value() {
        let mut names = [
            "I(VD) [VG=1.5; point 2]",
            "I(VD) [VG=10; point 4]",
            "V(D)",
            "I(VD) [VG=1; point 1]",
            "I(VD) [VG=2; point 3]",
            "V(G) [VG=1.5; point 2]",
            "V(G) [VG=1; point 1]",
        ];
        sweep_members_in_value_order(&mut names, |name| name);
        assert_eq!(
            names,
            [
                "I(VD) [VG=1; point 1]",
                "I(VD) [VG=1.5; point 2]",
                "V(D)",
                "I(VD) [VG=2; point 3]",
                "I(VD) [VG=10; point 4]",
                "V(G) [VG=1; point 1]",
                "V(G) [VG=1.5; point 2]",
            ]
        );
    }

    /// A pane too narrow for both regions gives up the legend, and never
    /// hands anything a negative rectangle.
    #[test]
    fn a_narrow_pane_header_collapses_without_inverting_a_region() {
        for width in [0.0_f32, 12.0, 46.0, 60.0, 120.0] {
            let rect = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(width, 25.0));
            let legend = legend_region(rect, 58.0);
            assert!(legend.width() >= 0.0, "{width}: inverted {legend:?}");
            assert!(legend.right() <= rect.right().max(HEADER_PADDING_X));
        }
    }
}
