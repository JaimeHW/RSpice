//! Waveform pane legend, unit tag, visibility controls and Y-axis actions.

use super::pane::SwatchStyle;
use super::{ReadoutPolicy, StripModel, StripTrace, UnitPane, cursor_interpolation};
use crate::strip::HEADER_PADDING_X;
use egui::Ui;
use rspice_ui_kit::{
    icons::Icon,
    plot::{paint_trace_swatch, sample::sample_at_with_shape},
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::{IconButton, chip},
};
use rspice_vocabulary::quantity::QuantityPresentationPolicy;
use std::borrow::Cow;

pub const WAVE_PANE_HEADER_HEIGHT: f32 = 25.0;

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

/// Display state qualified by the host for this pane and its own cursor domain.
pub struct PaneHeader {
    pub height: f32,
    pub active: bool,
    pub log_y: bool,
    pub log_y_available: bool,
    pub cursor_a: Option<(f64, ReadoutPolicy, QuantityPresentationPolicy)>,
    /// The strip gutter's name column, when the strip has lanes: the unit
    /// tag takes exactly that width, so the legend starts on the edge the
    /// lane names end on.
    pub name_width: Option<f32>,
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
    /// The name column's new width, while the reader drags its edge here.
    pub resize_name: Option<f32>,
}

/// The three regions of a pane header, across its row.
struct PaneHeaderRegions {
    unit: egui::Rect,
    legend: egui::Rect,
    actions: egui::Rect,
}

/// Where those regions sit.
///
/// Arithmetic rather than inline layout because the trailing edge is the
/// whole point: this row and the strip header above it are two rows of chrome
/// in one card, and this row used to run its log toggle into the pane edge
/// while the row above it stopped [`HEADER_PADDING_X`] short. A function can
/// be asserted about without standing up a strip, a host and a projection.
fn header_regions(rect: egui::Rect, unit_width: f32, action_width: f32) -> PaneHeaderRegions {
    // The unit tag's own inset is its floor as well as its start: a pane
    // narrower than the inset handed the tag a rectangle whose right edge was
    // left of its left one, which clips to nothing and reads as a widget that
    // vanished rather than one that had no room.
    let unit_left = rect.left() + 5.0;
    let unit = egui::Rect::from_min_max(
        egui::pos2(unit_left, rect.top() + 1.5),
        egui::pos2(
            (rect.left() + unit_width).min(rect.right()).max(unit_left),
            rect.bottom() - 1.5,
        ),
    );
    let actions_right = (rect.right() - HEADER_PADDING_X).max(unit.right());
    let actions = egui::Rect::from_min_max(
        egui::pos2((actions_right - action_width).max(unit.right()), rect.top()),
        egui::pos2(actions_right, rect.bottom()),
    );
    let legend = egui::Rect::from_min_max(
        egui::pos2(unit.right() + 3.0, rect.top()),
        egui::pos2(
            (actions.left() - 3.0).max(unit.right() + 3.0),
            rect.bottom(),
        ),
    );
    PaneHeaderRegions {
        unit,
        legend,
        actions,
    }
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
        name_width,
    } = input;
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let height = height.clamp(0.0, WAVE_PANE_HEADER_HEIGHT);
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, c.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, c.border),
    );
    if height < 18.0 {
        return UnitPaneHeaderResponse::default();
    }

    let action_width = 58.0;
    // Fit the pane's actual unit tag: nV/√Hz is wider than the quantity tags
    // the old fixed 46 px assumed, and a clipped unit misreads as a new unit.
    // An expression whose unit cannot be stated has a pane of its own and
    // says so, with the reason, rather than show a unit it does not have. A
    // plain number's pane has no unit to name: its axis goes unmarked, as a
    // unitless axis does in ViVA and ADS, and the tag is left out rather than
    // drawn as a control with nothing on it.
    let unit_chip = if pane.unlabeled {
        Some("no unit")
    } else {
        Some(pane.unit).filter(|unit| !unit.is_empty())
    };
    let unit_hover = if pane.unlabeled {
        let reason = model
            .expression_panes
            .iter()
            .filter(|expression| pane.expressions.contains(&expression.slot))
            .find_map(|expression| expression.unknown.as_deref())
            .unwrap_or("its unit cannot be stated");
        format!("No unit: {reason}, so this expression has an axis of its own")
    } else {
        format!("{} unit-scoped Y axis", pane.unit)
    };
    let unit_label_width = unit_chip.map(|unit_chip| {
        ui.fonts_mut(|fonts| {
            fonts
                .layout_no_wrap(
                    unit_chip.to_owned(),
                    theme::mono(tokens::FS_0, FontWeight::Regular),
                    c.text,
                )
                .size()
                .x
        })
    });
    // With lanes the tag column is the gutter's name column whatever it
    // holds, so the legend starts on the edge the lane names end on.
    let unit_width = name_width
        .unwrap_or_else(|| unit_label_width.map_or(0.0, |width| (width + 28.0).max(46.0)));
    let PaneHeaderRegions {
        unit: unit_rect,
        legend: legend_rect,
        actions: actions_rect,
    } = header_regions(rect, unit_width, action_width);

    if let Some(unit_chip) = unit_chip {
        ui.scope_builder(
            egui::UiBuilder::new()
                .max_rect(unit_rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
            |ui| {
                ui.set_clip_rect(unit_rect);
                let response = chip(ui, unit_chip, active).on_hover_text(unit_hover);
                if response.clicked() {
                    host.activate_pane();
                }
            },
        );
    }

    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(legend_rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
        |ui| {
            ui.set_clip_rect(legend_rect);
            ui.spacing_mut().item_spacing.x = 3.0;
            let traces: Vec<&StripTrace> = model
                .traces
                .iter()
                .take(model.signal_trace_count)
                .filter(|trace| trace_belongs_to_pane(model, pane, ordinal, trace))
                .collect();
            let (plotted, hidden): (Vec<&StripTrace>, Vec<&StripTrace>) =
                traces.into_iter().partition(|trace| trace.visible);
            // A name gives way only past a share of the row: hierarchical
            // names keep their leaf, cut from the middle as ViVA cuts them.
            let name_budget = (legend_rect.width() * 0.4).max(72.0);
            let chips: Vec<LegendEntry> = plotted
                .iter()
                .map(|trace| {
                    legend_entry(
                        ui,
                        model,
                        trace,
                        &cursor_a_value,
                        name_budget,
                        &display_name,
                    )
                })
                .collect();
            let hidden_label = format!("{} hidden", hidden.len());
            let reserve = |label: &str| control_width(ui, label) + 3.0;
            let hidden_width = if hidden.is_empty() {
                0.0
            } else {
                reserve(&hidden_label)
            };
            let fitted = fitted_chips(&chips, legend_rect.width() - hidden_width, |overflow| {
                reserve(&format!("+{overflow}"))
            });
            for (trace, entry) in plotted.iter().zip(&chips).take(fitted) {
                legend_chip(ui, trace, entry, &display_name, host);
            }
            if fitted < chips.len() {
                let overflow = &plotted[fitted..];
                ui.menu_button(format!("+{}", overflow.len()), |ui| {
                    for (trace, entry) in overflow.iter().zip(&chips[fitted..]) {
                        ui.horizontal(|ui| legend_chip(ui, trace, entry, &display_name, host));
                    }
                })
                .response
                .on_hover_text(format!(
                    "{} more plotted in the {} pane",
                    overflow.len(),
                    pane.unit
                ));
            }
            if !hidden.is_empty() {
                ui.menu_button(&hidden_label, |ui| {
                    for trace in &hidden {
                        let mut shown = false;
                        if ui
                            .checkbox(&mut shown, &*display_name(&trace.name))
                            .clicked()
                        {
                            host.toggle_trace_visibility(trace);
                        }
                    }
                })
                .response
                .on_hover_text(format!("Hidden traces of the {} pane", pane.unit));
            }
        },
    );

    let mut output = UnitPaneHeaderResponse {
        resize_name: name_width.and_then(|width| {
            super::logic::name_column_grip(
                ui,
                rect,
                width,
                ui.id()
                    .with(("pane-name-grip", model.analysis_key, pane.unit)),
            )
        }),
        ..UnitPaneHeaderResponse::default()
    };
    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(actions_rect)
            .layout(egui::Layout::right_to_left(egui::Align::Center)),
        |ui| {
            ui.set_clip_rect(actions_rect);
            let log = ui
                .add_enabled_ui(log_y_available, |ui| chip(ui, "log", log_y))
                .inner
                .on_hover_text(if log_y_available {
                    "Toggle logarithmic Y axis"
                } else {
                    "Logarithmic Y requires strictly positive visible values"
                });
            if log.clicked() {
                output.toggle_log_y = true;
            }
            if IconButton::new(Icon::ZoomFit)
                .side(19.0)
                .tooltip("Autoscale Y to visible traces")
                .show(ui)
                .clicked()
            {
                output.autoscale_y = true;
            }
        },
    );
    output
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

/// The width a button-like control labelled `label` takes.
fn control_width(ui: &Ui, label: &str) -> f32 {
    let font = egui::TextStyle::Button.resolve(ui.style());
    ui.painter()
        .layout_no_wrap(label.to_owned(), font, egui::Color32::WHITE)
        .size()
        .x
        + 2.0 * ui.spacing().button_padding.x
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

    /// Both rows of a strip's chrome stop on one rule.
    ///
    /// The pane header's actions used to be laid out against `rect.right()`,
    /// so the log toggle sat flush with the pane edge under a strip header
    /// whose own controls stopped 10 px short — two rows of one card trailing
    /// at 10 px and 0 px.
    #[test]
    fn the_pane_header_trails_on_the_strip_header_s_own_inset() {
        let rect = egui::Rect::from_min_size(egui::pos2(0.0, 28.0), egui::vec2(900.0, 25.0));
        let regions = header_regions(rect, 46.0, 58.0);
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

    /// A pane too narrow for all three regions gives up the legend, never the
    /// unit tag, and never hands anything a negative rectangle.
    #[test]
    fn a_narrow_pane_header_collapses_without_inverting_a_region() {
        for width in [0.0_f32, 12.0, 46.0, 60.0, 120.0] {
            let rect = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(width, 25.0));
            let regions = header_regions(rect, 46.0, 58.0);
            for region in [regions.unit, regions.legend, regions.actions] {
                assert!(region.width() >= 0.0, "{width}: inverted {region:?}");
            }
            assert!(regions.actions.left() >= regions.unit.right());
        }
    }
}
