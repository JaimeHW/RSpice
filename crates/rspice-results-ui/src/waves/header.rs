//! Waveform pane legend, unit tag, visibility controls and Y-axis actions.

use super::{ReadoutPolicy, StripModel, StripTrace, UnitPane, cursor_interpolation};
use egui::Ui;
use rspice_app_types::quantity::QuantityPresentationPolicy;
use rspice_ui_kit::{
    icons::Icon,
    plot::sample::sample_at_with_shape,
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::{IconButton, chip},
};
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
}

/// Display state qualified by the host for this pane and its own cursor domain.
pub struct PaneHeader {
    pub height: f32,
    pub active: bool,
    pub log_y: bool,
    pub log_y_available: bool,
    pub cursor_a: Option<(f64, ReadoutPolicy, QuantityPresentationPolicy)>,
}

/// Shorten a label to `max` characters with a typographic ellipsis.
pub fn elide(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        return text.to_owned();
    }
    let mut out: String = text.chars().take(max.saturating_sub(1)).collect();
    out.push('…');
    out
}

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
    let unit_label_width = ui.fonts_mut(|fonts| {
        fonts
            .layout_no_wrap(
                pane.unit.to_owned(),
                theme::mono(tokens::FS_0, FontWeight::Regular),
                c.text,
            )
            .size()
            .x
    });
    let unit_width = (unit_label_width + 28.0).max(46.0);
    let unit_rect = egui::Rect::from_min_max(
        egui::pos2(rect.left() + 5.0, rect.top() + 1.5),
        egui::pos2(
            (rect.left() + unit_width).min(rect.right()),
            rect.bottom() - 1.5,
        ),
    );
    let actions_rect = egui::Rect::from_min_max(
        egui::pos2(
            (rect.right() - action_width).max(unit_rect.right()),
            rect.top(),
        ),
        rect.right_bottom(),
    );
    let legend_rect = egui::Rect::from_min_max(
        egui::pos2(unit_rect.right() + 3.0, rect.top()),
        egui::pos2(
            (actions_rect.left() - 3.0).max(unit_rect.right() + 3.0),
            rect.bottom(),
        ),
    );

    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(unit_rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
        |ui| {
            ui.set_clip_rect(unit_rect);
            let response = chip(ui, pane.unit, active)
                .on_hover_text(format!("{} unit-scoped Y axis", pane.unit));
            if response.clicked() {
                host.activate_pane();
            }
        },
    );

    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(legend_rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
        |ui| {
            ui.set_clip_rect(legend_rect);
            egui::ScrollArea::horizontal()
                .id_salt(("rspice.results.pane-legend", model.analysis_key, pane.unit))
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing.x = 3.0;
                    let traces = model
                        .traces
                        .iter()
                        .take(model.signal_trace_count)
                        .enumerate()
                        .filter(|(_, trace)| trace_belongs_to_pane(model, pane, ordinal, trace));
                    for (_, trace) in traces {
                        let selected = host.trace_selected(trace);
                        let (swatch, swatch_response) =
                            ui.allocate_exact_size(egui::vec2(13.0, 19.0), egui::Sense::click());
                        ui.painter().hline(
                            egui::Rangef::new(swatch.left() + 1.0, swatch.right() - 1.0),
                            swatch.center().y,
                            egui::Stroke::new(2.0, trace.color),
                        );
                        swatch_response.widget_info(|| {
                            egui::WidgetInfo::selected(
                                egui::WidgetType::Button,
                                true,
                                trace.visible,
                                format!("Toggle {} visibility", display_name(&trace.name)),
                            )
                        });
                        theme::paint_focus_ring(ui, &swatch_response, swatch);
                        if swatch_response.clicked() {
                            host.toggle_trace_visibility(trace);
                        }
                        // The instrument idiom: a trace states its own value
                        // at cursor A right where its name is, so reading one
                        // curve never costs a trip to the readout register.
                        // A corner family draws one chip for the whole group,
                        // so the chip states how many traces it stands for.
                        let family = trace.family_group_ordinal.map(|_| {
                            model
                                .traces
                                .iter()
                                .filter(|candidate| {
                                    candidate.presentation_key == trace.presentation_key
                                        && candidate.family_group_ordinal.is_some()
                                })
                                .count()
                        });
                        let shown = display_name(&trace.name);
                        let label = match &cursor_a_value {
                            Some((x, presentation, policy)) if trace.visible => {
                                let digits =
                                    usize::from(presentation.displayed_significant_digits().get());
                                let value = sample_at_with_shape(
                                    &trace.x,
                                    &trace.y,
                                    &trace.shape,
                                    *x,
                                    cursor_interpolation(presentation.cursor_interpolation()),
                                );
                                format!(
                                    "{}  {}",
                                    elide(&shown, 16),
                                    model.format_trace_value(trace, value, digits, *policy)
                                )
                            }
                            _ => elide(&shown, 20),
                        };
                        let label = match family {
                            Some(count) if count > 1 => format!("{label}  ×{count}"),
                            _ => label,
                        };
                        // A hidden trace keeps its chip so it can be brought
                        // back, but must not read as a curve on the canvas.
                        let label = if trace.visible {
                            egui::RichText::new(label)
                        } else {
                            egui::RichText::new(label)
                                .strikethrough()
                                .color(c.text_faint)
                        };
                        if ui
                            .selectable_label(selected, label)
                            .on_hover_text(&*shown)
                            .clicked()
                        {
                            host.select_trace(trace);
                        }
                    }
                    ui.menu_button("+", |ui| {
                        let mut any = false;
                        for trace in
                            model
                                .traces
                                .iter()
                                .take(model.signal_trace_count)
                                .filter(|trace| {
                                    !trace.visible
                                        && trace_belongs_to_pane(model, pane, ordinal, trace)
                                })
                        {
                            any = true;
                            if ui.button(&*display_name(&trace.name)).clicked() {
                                host.toggle_trace_visibility(trace);
                                ui.close();
                            }
                        }
                        if !any {
                            ui.label("All compatible signals are already shown");
                        }
                    })
                    .response
                    .on_hover_text(format!("Add a signal to the {} pane", pane.unit));
                });
        },
    );

    let mut output = UnitPaneHeaderResponse::default();
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
