//! Waveform pane legend, visibility controls and Y-axis actions.

use super::pane::SwatchStyle;
use super::{ReadoutPolicy, StripModel, StripTrace, UnitPane, cursor_interpolation};
use crate::strip::HEADER_PADDING_X;
use egui::Ui;
use rspice_results::visualization_document::Abscissa;
use rspice_ui_kit::{
    icons::Icon,
    plot::{paint_trace_swatch, sample::sample_at_with_shape},
    theme,
    tokens::Tokens,
    widgets::{IconButton, chip},
};
use rspice_vocabulary::quantity::QuantityPresentationPolicy;
use std::borrow::Cow;

pub const WAVE_PANE_HEADER_HEIGHT: f32 = 25.0;

/// The strip header's icon side and the gap between its trailing actions:
/// the pane's icon takes the same rectangle as the strip's maximize icon
/// above it.
const ACTION_ICON_SIDE: f32 = 22.0;
const ACTION_ITEM_GAP: f32 = 1.0;
/// The space, beyond the item gap, between the icon and the scale chip left
/// of it, as the strip header spaces its own chip.
const SCALE_CHIP_SPACE: f32 = 4.0;

/// Source-qualified actions applied immediately in legend order.
///
/// Selection is queried for each chip, including after an earlier chip changes it.
pub trait PaneHeaderHost {
    fn trace_selected(&self, trace: &StripTrace) -> bool;
    fn toggle_trace_visibility(&mut self, trace: &StripTrace);
    fn select_trace(&mut self, trace: &StripTrace);
    fn activate_pane(&mut self);
    /// Add an expression to the strip, as a trace's menu asks.
    fn add_expression(&mut self, _text: String) {}
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
    /// Whether it is drawn. A hidden one waits behind the pane's hidden
    /// control, as a hidden signal does.
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

fn trace_belongs_to_pane(
    model: &StripModel,
    pane: &UnitPane,
    ordinal: usize,
    trace: &StripTrace,
) -> bool {
    if !trace.kind.is_phase() {
        return model.trace_unit(trace) == pane.unit;
    }
    let has_magnitude = model
        .traces
        .iter()
        .any(|candidate| !candidate.kind.is_phase() && model.trace_unit(candidate) == "dB");
    if has_magnitude {
        pane.unit == "dB"
    } else {
        ordinal == 0
    }
}

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
}

/// The two regions of a pane header, across its row.
struct PaneHeaderRegions {
    legend: egui::Rect,
    actions: egui::Rect,
}

/// Where those regions sit.
///
/// Arithmetic rather than inline layout because the edges are the whole
/// point: this row and the strip header above it are two rows of chrome in
/// one card, and both start and stop [`HEADER_PADDING_X`] in from the pane's
/// edges. This row used to run its log toggle into the pane edge while the
/// row above it stopped short. A function can be asserted about without
/// standing up a strip, a host and a projection.
fn header_regions(rect: egui::Rect, action_width: f32) -> PaneHeaderRegions {
    // The inset is a floor as well as a start: a pane narrower than its two
    // insets must not hand a region a rectangle whose right edge is left of
    // its left one, which clips to nothing and reads as a widget that
    // vanished rather than one that had no room.
    let left = (rect.left() + HEADER_PADDING_X).min(rect.right());
    let actions_right = (rect.right() - HEADER_PADDING_X).max(left);
    let actions = egui::Rect::from_min_max(
        egui::pos2((actions_right - action_width).max(left), rect.top()),
        egui::pos2(actions_right, rect.bottom()),
    );
    let legend = egui::Rect::from_min_max(
        egui::pos2(left, rect.top()),
        egui::pos2((actions.left() - 3.0).max(left), rect.bottom()),
    );
    PaneHeaderRegions { legend, actions }
}

/// Draw the legend using design-aware names while keeping source identities intact.
pub fn show_header(
    ui: &mut Ui,
    model: &StripModel,
    pane: &UnitPane,
    ordinal: usize,
    input: PaneHeader,
    display_name: impl for<'a> Fn(&'a str) -> Cow<'a, str>,
    host: &mut impl PaneHeaderHost,
) -> UnitPaneHeaderResponse {
    let PaneHeader {
        height,
        active,
        log_y,
        log_y_available,
        cursor_a: cursor_a_value,
        expressions,
        plotted,
        abscissa,
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
        egui::Sense::click(),
    );
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
    let actions_width_id = egui::Id::new(("pane-actions-width", model.analysis_key));
    let action_width = if plotted {
        let space = header_regions(rect, f32::INFINITY).actions;
        let drawn = ui.scope_builder(
            egui::UiBuilder::new()
                .max_rect(space)
                .layout(egui::Layout::right_to_left(egui::Align::Center)),
            |ui| {
                ui.set_clip_rect(space);
                // The row ends as the strip header above it does: the icon
                // in the strip's icon column, then the axis' scale chip the
                // same gap to its left, so every scale toggle of the strip
                // stands in one right-hand column.
                ui.spacing_mut().item_spacing.x = ACTION_ITEM_GAP;
                if IconButton::new(Icon::ZoomFit)
                    .side(ACTION_ICON_SIDE)
                    .tooltip("Fit Y to the traces in view")
                    .show(ui)
                    .clicked()
                {
                    output.autoscale_y = true;
                }
                ui.add_space(SCALE_CHIP_SPACE);
                let log = ui
                    .add_enabled_ui(log_y_available, |ui| chip(ui, "log Y", log_y))
                    .inner
                    .on_hover_text(if log_y_available {
                        "Toggle logarithmic Y axis"
                    } else {
                        "Logarithmic Y needs a positive value to draw"
                    });
                if log.clicked() {
                    output.toggle_log_y = true;
                }
            },
        );
        let width = space.right() - drawn.response.rect.left();
        ui.ctx()
            .data_mut(|data| data.insert_temp(actions_width_id, width));
        width
    } else {
        // A pane with nothing plotted draws no buttons but keeps their
        // width, so its hidden control stands in one column with those of
        // the panes above and below it.
        ui.ctx()
            .data(|data| data.get_temp::<f32>(actions_width_id))
            .unwrap_or(0.0)
    };
    let PaneHeaderRegions {
        legend: legend_rect,
        ..
    } = header_regions(rect, action_width);

    let mut traces: Vec<&StripTrace> = model
        .traces
        .iter()
        .take(model.signal_trace_count)
        .filter(|trace| trace_belongs_to_pane(model, pane, ordinal, trace))
        // The signal a pane is drawn against is its X, not a curve.
        .filter(|trace| {
            !drawn_against
                .is_some_and(|name| super::xy::is_own_abscissa(&trace.source_waveform_name, name))
        })
        .collect();
    sweep_members_in_value_order(&mut traces, |trace| &trace.source_waveform_name);
    let (plotted_traces, hidden): (Vec<&StripTrace>, Vec<&StripTrace>) =
        traces.into_iter().partition(|trace| trace.visible);
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
    let chips: Vec<LegendEntry> = entries
        .iter()
        .map(|entry| match entry {
            Entry::Signal(trace) => legend_entry(
                ui,
                model,
                trace,
                &cursor_a_value,
                name_budget,
                &display_name,
            ),
            Entry::Expression(expression) => expression_entry(ui, expression, name_budget),
        })
        .collect();

    // The hidden control is laid out next, whole, at the legend's right end:
    // the plotted entries share what is left, and the ones that do not fit
    // go behind the overflow control instead of under it.
    let hidden_count = hidden.len() + hidden_expressions.len();
    let mut entries_right = legend_rect.right();
    if hidden_count > 0 {
        let drawn = ui.scope_builder(
            egui::UiBuilder::new()
                .max_rect(legend_rect)
                .layout(egui::Layout::right_to_left(egui::Align::Center)),
            |ui| {
                ui.menu_button(format!("{hidden_count} hidden"), |ui| {
                    for trace in &hidden {
                        let mut shown = false;
                        if ui
                            .checkbox(&mut shown, &*display_name(&trace.name))
                            .clicked()
                        {
                            host.toggle_trace_visibility(trace);
                        }
                    }
                    for expression in &hidden_expressions {
                        let mut shown = false;
                        if ui
                            .checkbox(&mut shown, &expression.label)
                            .on_hover_text(&expression.text)
                            .clicked()
                        {
                            output.toggle_expression = Some(expression.slot);
                        }
                    }
                })
                .response
                .on_hover_text(format!("Hidden traces of the {} pane", pane_name(pane)));
            },
        );
        entries_right = drawn.response.rect.left() - 3.0;
    }
    let entries_rect = egui::Rect::from_min_max(
        legend_rect.min,
        egui::pos2(entries_right.max(legend_rect.left()), legend_rect.bottom()),
    );
    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(entries_rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
        |ui| {
            ui.set_clip_rect(entries_rect);
            ui.spacing_mut().item_spacing.x = 3.0;
            let fitted = fitted_chips(&chips, entries_rect.width(), |overflow| {
                control_width(ui, &format!("+{overflow}")) + 3.0
            });
            for (entry, chip) in entries.iter().zip(&chips).take(fitted) {
                entry.show(ui, chip, &display_name, host, &mut output);
            }
            if fitted < chips.len() {
                let overflow = &entries[fitted..];
                ui.menu_button(format!("+{}", overflow.len()), |ui| {
                    for (entry, chip) in overflow.iter().zip(&chips[fitted..]) {
                        ui.horizontal(|ui| entry.show(ui, chip, &display_name, host, &mut output));
                    }
                })
                .response
                .on_hover_text(format!(
                    "{} more plotted in the {} pane",
                    overflow.len(),
                    pane_name(pane)
                ));
            }
        },
    );
    if let Some(menu) = abscissa.as_ref() {
        let draft_id = ui
            .id()
            .with(("abscissa-menu-draft", model.analysis_key, pane.unit));
        if let Some(chosen) = egui::Popup::context_menu(&band)
            .show(|ui| abscissa_choices(ui, pane.unit, menu, &display, draft_id))
            .and_then(|inner| inner.inner)
        {
            output.abscissa = Some(chosen);
        }
    }
    output
}

/// How the row names its pane to a reader: by its unit, or as the pane of an
/// expression whose unit cannot be stated, or of plain numbers.
fn pane_name<'a>(pane: &UnitPane<'a>) -> &'a str {
    if pane.unlabeled {
        "no-unit"
    } else if pane.unit.is_empty() {
        "unitless"
    } else {
        pane.unit
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
            let value = sample_at_with_shape(
                &trace.x,
                &trace.y,
                &trace.shape,
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
    fn show(
        &self,
        ui: &mut Ui,
        chip: &LegendEntry,
        display_name: &impl for<'a> Fn(&'a str) -> Cow<'a, str>,
        host: &mut impl PaneHeaderHost,
        output: &mut UnitPaneHeaderResponse,
    ) {
        match self {
            Self::Signal(trace) => legend_chip(ui, trace, chip, display_name, host),
            Self::Expression(expression) => expression_chip(ui, expression, chip, host, output),
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
    expression: &PaneExpression,
    entry: &LegendEntry,
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
        .selectable_label(false, entry.label.as_str())
        .on_hover_text(&expression.text);
    if label.clicked() {
        host.activate_pane();
    }
    label.context_menu(|ui| {
        ui.menu_button("Digital", |ui| {
            if let Some(text) = super::logic::convert::digital_form_of(
                ui,
                &expression.text,
                expression.values.as_slice(),
            ) {
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
    // ran past its end and cut the hidden control to "10 hidd".
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

/// How many of `chips` the row shows in `width`: all of them when they fit,
/// otherwise as many as leave room for the chip that counts the rest.
fn fitted_chips(chips: &[LegendEntry], width: f32, overflow_width: impl Fn(usize) -> f32) -> usize {
    let total: f32 = chips.iter().map(|chip| chip.width).sum();
    if total <= width {
        return chips.len();
    }
    let mut used = 0.0;
    for (index, chip) in chips.iter().enumerate() {
        if used + chip.width + overflow_width(chips.len() - index) > width {
            return index;
        }
        used += chip.width;
    }
    chips.len()
}

/// A plotted trace's chip: its swatch, which hides it, then its name and
/// value, which select it. The full name is on hover.
fn legend_chip(
    ui: &mut Ui,
    trace: &StripTrace,
    entry: &LegendEntry,
    display_name: &impl for<'a> Fn(&'a str) -> Cow<'a, str>,
    host: &mut impl PaneHeaderHost,
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
        .selectable_label(selected, entry.label.as_str())
        .on_hover_text(&*shown);
    if label.clicked() {
        host.select_trace(trace);
    }
    label.context_menu(|ui| {
        ui.menu_button("Digital", |ui| {
            if let Some(text) = super::logic::convert::digital_form(
                ui,
                &trace.source_waveform_name,
                trace.y.as_slice(),
            ) {
                host.add_expression(text);
            }
        });
    });
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

    /// Both rows of a strip's chrome stop on one rule.
    ///
    /// The pane header's actions used to be laid out against `rect.right()`,
    /// so the log toggle sat flush with the pane edge under a strip header
    /// whose own controls stopped 10 px short — two rows of one card trailing
    /// at 10 px and 0 px.
    #[test]
    fn the_pane_header_trails_on_the_strip_header_s_own_inset() {
        let rect = egui::Rect::from_min_size(egui::pos2(0.0, 28.0), egui::vec2(900.0, 25.0));
        let regions = header_regions(rect, 58.0);
        assert!(
            (regions.legend.left() - rect.left() - HEADER_PADDING_X).abs() < f32::EPSILON,
            "the legend starts at {}, not {} in from the pane edge",
            regions.legend.left(),
            HEADER_PADDING_X
        );
        assert!(
            (rect.right() - regions.actions.right() - HEADER_PADDING_X).abs() < f32::EPSILON,
            "actions end at {}, not {} from the pane edge",
            regions.actions.right(),
            HEADER_PADDING_X
        );
        assert!(regions.actions.width() >= 58.0, "the actions lost room");
        assert!(
            regions.legend.right() <= regions.actions.left(),
            "the legend runs into the actions"
        );
    }

    /// A pane too narrow for both regions gives up the legend, and never
    /// hands anything a negative rectangle.
    #[test]
    fn a_narrow_pane_header_collapses_without_inverting_a_region() {
        for width in [0.0_f32, 12.0, 46.0, 60.0, 120.0] {
            let rect = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(width, 25.0));
            let regions = header_regions(rect, 58.0);
            for region in [regions.legend, regions.actions] {
                assert!(region.width() >= 0.0, "{width}: inverted {region:?}");
            }
            assert!(regions.actions.left() >= regions.legend.right());
        }
    }
}
