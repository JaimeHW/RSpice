//! Strip chrome — the 28 px header (kind tag · subtitle · legend chips ·
//! actions) over a document-well body. Every plot in the product wears this.

use egui::{Label, Rect, Sense, Stroke, Ui, WidgetInfo, WidgetType, vec2};

use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight, mix};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::IconButton;

/// Height of a strip header.
pub const HEADER_HEIGHT: f32 = 28.0;

const HEADER_PADDING_X: f32 = 10.0;
const HEADER_ACTION_GAP: f32 = 6.0;
const HEADER_METADATA_GAP: f32 = 8.0;
const HEADER_LEGEND_GAP: f32 = 4.0;
const HEADER_ICON_SIDE: f32 = 22.0;
const HEADER_ACTION_ITEM_GAP: f32 = 1.0;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct HorizontalSpan {
    start: f32,
    width: f32,
}

impl HorizontalSpan {
    fn end(self) -> f32 {
        self.start + self.width
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct StripContentLayout {
    kind: HorizontalSpan,
    subtitle: HorizontalSpan,
    legend: HorizontalSpan,
}

fn strip_content_layout(
    available_width: f32,
    kind_natural_width: f32,
    subtitle_natural_width: f32,
    has_legend: bool,
) -> StripContentLayout {
    let available_width = available_width.max(0.0);
    if available_width == 0.0 {
        return StripContentLayout::default();
    }

    // Preserve a readable kind tag, but never let arbitrary analysis labels
    // claim the legend's or the trailing actions' rectangle.
    let kind_cap = if has_legend {
        (available_width * 0.28).clamp(32.0, 96.0)
    } else {
        (available_width * 0.40).clamp(32.0, 128.0)
    }
    .min(available_width);
    let kind_width = kind_natural_width.max(0.0).min(kind_cap);
    let kind = HorizontalSpan {
        start: 0.0,
        width: kind_width,
    };

    if !has_legend {
        let gap = if kind_width > 0.0 && subtitle_natural_width > 0.0 {
            HEADER_METADATA_GAP.min((available_width - kind.end()).max(0.0))
        } else {
            0.0
        };
        let subtitle = HorizontalSpan {
            start: kind.end() + gap,
            width: subtitle_natural_width
                .max(0.0)
                .min((available_width - kind.end() - gap).max(0.0)),
        };
        return StripContentLayout {
            kind,
            subtitle,
            legend: HorizontalSpan::default(),
        };
    }

    let after_kind = (available_width - kind.end()).max(0.0);
    let metadata_gap = if kind_width > 0.0 && subtitle_natural_width > 0.0 {
        HEADER_METADATA_GAP.min(after_kind)
    } else {
        0.0
    };
    // The legend is the flexible/scrolling owner. Reserve at most 60 px for
    // it, then let a bounded subtitle use only the surplus.
    let legend_reserve = 60.0_f32.min(available_width * 0.24).min(after_kind);
    let legend_gap = HEADER_LEGEND_GAP
        .min((available_width - kind.end() - metadata_gap - legend_reserve).max(0.0));
    let subtitle_cap = (available_width * 0.32).min(180.0);
    let subtitle_budget =
        (available_width - kind.end() - metadata_gap - legend_gap - legend_reserve).max(0.0);
    let subtitle_width = subtitle_natural_width
        .max(0.0)
        .min(subtitle_cap)
        .min(subtitle_budget);

    let subtitle = HorizontalSpan {
        start: kind.end()
            + if subtitle_width > 0.0 {
                metadata_gap
            } else {
                0.0
            },
        width: subtitle_width,
    };
    let metadata_end = if subtitle_width > 0.0 {
        subtitle.end()
    } else {
        kind.end()
    };
    let actual_legend_gap = HEADER_LEGEND_GAP.min((available_width - metadata_end).max(0.0));
    let legend = HorizontalSpan {
        start: metadata_end + actual_legend_gap,
        width: (available_width - metadata_end - actual_legend_gap).max(0.0),
    };

    StripContentLayout {
        kind,
        subtitle,
        legend,
    }
}

fn trailing_actions_width(
    icon_side: f32,
    closable: bool,
    zoomed: bool,
    fit_chip_width: f32,
) -> f32 {
    let icon_count = 1 + usize::from(closable);
    let icons_width = icon_count as f32 * icon_side
        + icon_count.saturating_sub(1) as f32 * HEADER_ACTION_ITEM_GAP;
    icons_width
        + if zoomed {
            2.0 * HEADER_ACTION_ITEM_GAP + 4.0 + fit_chip_width
        } else {
            0.0
        }
}

/// One legend chip's description.
pub struct LegendChip<'a> {
    /// Trace name.
    pub name: &'a str,
    /// Trace color swatch.
    pub color: egui::Color32,
    /// Whether the trace is visible.
    pub on: bool,
}

/// What strip-header interactions happened this frame.
#[derive(Default)]
pub struct StripHeaderResponse {
    /// Index of the legend chip that was clicked.
    pub legend_clicked: Option<usize>,
    /// Index of the legend chip whose "Remove" context action was chosen.
    pub legend_removed: Option<usize>,
    /// The maximize/restore action was clicked.
    pub maximize_clicked: bool,
    /// The close action was clicked.
    pub close_clicked: bool,
    /// The FIT action (shown while zoomed) was clicked.
    pub fit_clicked: bool,
    /// The "+ expr" action was clicked.
    pub add_expr_clicked: bool,
}

/// Strip header builder. `StripHeader::new(...).closable(true).show(ui)`.
pub struct StripHeader<'a> {
    kind: &'a str,
    subtitle: &'a str,
    legend: &'a [LegendChip<'a>],
    maximized: bool,
    closable: bool,
    zoomed: bool,
    expr_action: bool,
    removable_from: usize,
}

impl<'a> StripHeader<'a> {
    /// A header with the kind tag, subtitle, and legend; all actions off.
    pub fn new(kind: &'a str, subtitle: &'a str, legend: &'a [LegendChip<'a>]) -> Self {
        Self {
            kind,
            subtitle,
            legend,
            maximized: false,
            closable: false,
            zoomed: false,
            expr_action: false,
            removable_from: usize::MAX,
        }
    }

    /// Show the maximize action in its active (restore) state.
    pub fn maximized(mut self, on: bool) -> Self {
        self.maximized = on;
        self
    }

    /// Show the close action.
    pub fn closable(mut self, on: bool) -> Self {
        self.closable = on;
        self
    }

    /// Show the FIT action (the plot has a zoom/pan override active).
    pub fn zoomed(mut self, on: bool) -> Self {
        self.zoomed = on;
        self
    }

    /// Show the "+ expr" action after the legend.
    pub fn expr_action(mut self, on: bool) -> Self {
        self.expr_action = on;
        self
    }

    /// Legend chips from this index on get a "Remove" context-menu action
    /// (user expression traces).
    pub fn removable_from(mut self, index: usize) -> Self {
        self.removable_from = index;
        self
    }

    /// Render the header across the available width.
    pub fn show(self, ui: &mut Ui) -> StripHeaderResponse {
        let t = Tokens::get(ui.ctx());
        let c = t.color;
        let mut out = StripHeaderResponse::default();

        let width = ui.available_width();
        let (rect, _) = ui.allocate_exact_size(vec2(width, HEADER_HEIGHT), Sense::hover());
        if !ui.is_rect_visible(rect) {
            return out;
        }
        let painter = ui.painter();
        painter.rect_filled(rect, 0.0, c.bg_panel);
        painter.hline(
            rect.x_range(),
            rect.bottom() - 0.5,
            Stroke::new(1.0, c.border),
        );

        let inner = rect.shrink2(vec2(HEADER_PADDING_X, 0.0));
        let fit_chip_width = if self.zoomed {
            action_chip_width(ui, "FIT")
        } else {
            0.0
        };
        let actions_width =
            trailing_actions_width(HEADER_ICON_SIDE, self.closable, self.zoomed, fit_chip_width)
                .min(inner.width());
        let actions_rect = Rect::from_min_max(
            egui::pos2(inner.right() - actions_width, inner.top()),
            inner.right_bottom(),
        );
        let content_right = (actions_rect.left() - HEADER_ACTION_GAP).max(inner.left());
        let content_rect =
            Rect::from_min_max(inner.left_top(), egui::pos2(content_right, inner.bottom()));

        // Kind tag: mono semibold, letterspaced. Metadata is measured before
        // rendering so it receives a bounded region instead of consuming the
        // legend or action budget.
        let mut job = egui::text::LayoutJob::default();
        job.append(
            self.kind,
            0.0,
            egui::TextFormat {
                font_id: theme::mono(tokens::FS_0, FontWeight::Medium),
                color: c.text,
                extra_letter_spacing: 0.08 * tokens::FS_0,
                ..Default::default()
            },
        );
        let kind_natural_width = ui.fonts_mut(|fonts| fonts.layout_job(job.clone()).size().x);
        let subtitle_font = theme::sans(tokens::FS_0, FontWeight::Regular);
        let subtitle_natural_width = if self.subtitle.is_empty() {
            0.0
        } else {
            ui.fonts_mut(|fonts| {
                fonts
                    .layout_no_wrap(
                        self.subtitle.to_owned(),
                        subtitle_font.clone(),
                        c.text_faint,
                    )
                    .size()
                    .x
            })
        };
        let layout = strip_content_layout(
            content_rect.width(),
            kind_natural_width,
            subtitle_natural_width,
            !self.legend.is_empty() || self.expr_action,
        );
        let span_rect = |span: HorizontalSpan| {
            Rect::from_min_max(
                egui::pos2(content_rect.left() + span.start, content_rect.top()),
                egui::pos2(content_rect.left() + span.end(), content_rect.bottom()),
            )
        };
        truncated_label(
            ui,
            span_rect(layout.kind),
            Label::new(job).truncate(),
            self.kind,
        );
        if layout.subtitle.width > 0.0 {
            truncated_label(
                ui,
                span_rect(layout.subtitle),
                Label::new(
                    egui::RichText::new(self.subtitle)
                        .font(subtitle_font)
                        .color(c.text_faint),
                )
                .truncate(),
                self.subtitle,
            );
        }

        // Chips own exactly the remaining clipped width; drag/wheel scrolling
        // keeps every trace reachable without crossing into the action rect.
        let removable_from = self.removable_from;
        let legend = self.legend;
        let expr_action = self.expr_action;
        let legend_rect = span_rect(layout.legend);
        let mut legend_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(legend_rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        legend_ui.set_clip_rect(legend_rect);
        egui::ScrollArea::horizontal()
            .id_salt(legend_ui.id().with("strip.legend"))
            .max_width(layout.legend.width)
            .auto_shrink([false, true])
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
            .show(&mut legend_ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 4.0;
                    for (index, chip) in legend.iter().enumerate() {
                        let response = legend_chip(ui, chip);
                        if response.clicked() {
                            out.legend_clicked = Some(index);
                        }
                        if index >= removable_from {
                            response.context_menu(|ui| {
                                if ui.button("Remove expression").clicked() {
                                    out.legend_removed = Some(index);
                                    ui.close();
                                }
                            });
                        }
                    }
                    if expr_action {
                        ui.add_space(2.0);
                        if action_chip(
                            ui,
                            "+ expr",
                            "Add an expression trace - V(out)/V(in), dB(), deriv()...",
                        )
                        .clicked()
                        {
                            out.add_expr_clicked = true;
                        }
                    }
                });
            });

        ui.scope_builder(
            egui::UiBuilder::new()
                .max_rect(actions_rect)
                .layout(egui::Layout::right_to_left(egui::Align::Center)),
            |ui| {
                ui.set_clip_rect(actions_rect);
                ui.spacing_mut().item_spacing.x = HEADER_ACTION_ITEM_GAP;
                if self.closable
                    && IconButton::new(Icon::Close)
                        .side(HEADER_ICON_SIDE)
                        .tooltip("Close strip")
                        .show(ui)
                        .clicked()
                {
                    out.close_clicked = true;
                }
                let tip = if self.maximized {
                    "Restore strips"
                } else {
                    "Maximize strip"
                };
                if IconButton::new(Icon::ZoomFit)
                    .side(HEADER_ICON_SIDE)
                    .on(self.maximized)
                    .tooltip(tip)
                    .show(ui)
                    .clicked()
                {
                    out.maximize_clicked = true;
                }
                if self.zoomed {
                    ui.add_space(4.0);
                    if action_chip(
                        ui,
                        "FIT",
                        "Restore automatic view (double-click the plot does the same)",
                    )
                    .clicked()
                    {
                        out.fit_clicked = true;
                    }
                }
            },
        );

        out
    }
}

fn truncated_label(ui: &mut Ui, rect: Rect, label: Label, full_text: &str) {
    if rect.width() <= 0.0 || rect.height() <= 0.0 {
        return;
    }
    let response = ui
        .scope_builder(
            egui::UiBuilder::new()
                .max_rect(rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
            |ui| {
                ui.set_clip_rect(rect);
                ui.add_sized(rect.size(), label)
            },
        )
        .inner;
    response.on_hover_text(full_text);
}

fn action_chip_width(ui: &mut Ui, label: &str) -> f32 {
    ui.fonts_mut(|fonts| {
        fonts
            .layout_no_wrap(
                label.to_owned(),
                theme::mono(tokens::FS_0, FontWeight::Medium),
                egui::Color32::WHITE,
            )
            .size()
            .x
            + 14.0
    })
}

/// A small mono action chip ("FIT", "+ expr") in the strip header.
fn action_chip(ui: &mut Ui, label: &str, tooltip: &str) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let galley = ui.fonts_mut(|f| {
        f.layout_no_wrap(
            label.to_owned(),
            theme::mono(tokens::FS_0, FontWeight::Medium),
            c.accent,
        )
    });
    let (rect, response) =
        ui.allocate_exact_size(vec2(galley.size().x + 14.0, 18.0), Sense::click());
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, ui.is_enabled(), tooltip));
    if !ui.is_rect_visible(rect) {
        return response;
    }

    let hover =
        ui.ctx()
            .animate_bool_with_time(response.id, response.hovered(), ui.style().animation_time);
    let painter = ui.painter();
    painter.rect(
        rect,
        t.radius,
        mix(c.bg_panel, c.bg_hover, hover),
        Stroke::new(1.0, c.accent_dim),
        egui::StrokeKind::Inside,
    );
    painter.galley(
        egui::pos2(
            rect.center().x - galley.size().x * 0.5,
            rect.center().y - galley.size().y * 0.5,
        ),
        galley,
        c.accent,
    );

    theme::paint_focus_ring(ui, &response, rect);

    response
        .on_hover_text(tooltip)
        .on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// A legend chip: 14×3 color swatch + mono name; dimmed when off.
fn legend_chip(ui: &mut Ui, chip: &LegendChip<'_>) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let galley = ui.fonts_mut(|f| {
        f.layout_no_wrap(
            chip.name.to_owned(),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            c.text_dim,
        )
    });
    let (rect, response) = ui.allocate_exact_size(
        vec2(6.0 + 14.0 + 5.0 + galley.size().x + 6.0, 20.0),
        Sense::click(),
    );
    response.widget_info(|| {
        WidgetInfo::selected(
            WidgetType::SelectableLabel,
            ui.is_enabled(),
            chip.on,
            format!("{} trace visibility", chip.name),
        )
    });
    if !ui.is_rect_visible(rect) {
        return response;
    }

    let hover =
        ui.ctx()
            .animate_bool_with_time(response.id, response.hovered(), ui.style().animation_time);
    let opacity = if chip.on { 1.0 } else { 0.38 };
    let painter = ui.painter();
    if hover > 0.0 {
        painter.rect_filled(
            rect,
            t.radius,
            mix(egui::Color32::TRANSPARENT, c.bg_hover, hover),
        );
    }
    let swatch_color = if chip.on { chip.color } else { c.text_faint };
    painter.rect_filled(
        Rect::from_min_size(
            egui::pos2(rect.left() + 6.0, rect.center().y - 1.5),
            vec2(14.0, 3.0),
        ),
        1.0,
        swatch_color.gamma_multiply(opacity),
    );
    painter.galley(
        egui::pos2(
            rect.left() + 6.0 + 14.0 + 5.0,
            rect.center().y - galley.size().y * 0.5,
        ),
        galley,
        mix(c.text_dim, c.text, hover).gamma_multiply(opacity),
    );

    theme::paint_focus_ring(ui, &response, rect);

    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_disjoint(left: HorizontalSpan, right: HorizontalSpan) {
        assert!(
            left.end() <= right.start + f32::EPSILON,
            "spans overlap: {left:?} and {right:?}"
        );
    }

    #[test]
    fn narrow_strip_bounds_metadata_and_gives_legend_the_true_remainder() {
        let layout = strip_content_layout(210.0, 320.0, 640.0, true);

        assert!(layout.kind.width <= 96.0);
        assert!(layout.subtitle.width <= 210.0 * 0.32);
        assert!(layout.legend.width > 0.0);
        assert_disjoint(layout.kind, layout.subtitle);
        assert_disjoint(layout.subtitle, layout.legend);
        assert!(layout.legend.end() <= 210.0 + f32::EPSILON);
    }

    #[test]
    fn strip_layout_never_forces_a_minimum_past_available_width() {
        for width in [0.0, 12.0, 48.0, 120.0, 360.0] {
            let layout = strip_content_layout(width, 400.0, 800.0, true);
            assert!(layout.kind.start >= 0.0);
            assert!(layout.subtitle.start >= 0.0);
            assert!(layout.legend.start >= 0.0);
            assert!(layout.kind.end() <= width + f32::EPSILON);
            assert!(layout.subtitle.end() <= width + f32::EPSILON);
            assert!(layout.legend.end() <= width + f32::EPSILON);
        }
    }

    #[test]
    fn trailing_actions_reserve_exact_rendered_controls_first() {
        let base = trailing_actions_width(22.0, false, false, 0.0);
        let close = trailing_actions_width(22.0, true, false, 0.0);
        let zoomed = trailing_actions_width(22.0, true, true, 31.0);

        assert_eq!(base, 22.0);
        assert_eq!(close, 45.0);
        assert_eq!(zoomed, 82.0);
    }
}
