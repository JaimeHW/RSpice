//! Buttons: the labeled control button (`Button`) and the square tool button
//! (`IconButton`).

use egui::{Rect, Response, Sense, Stroke, Ui, vec2};

use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight, mix};
use crate::ui::tokens::{self, Tokens};

/// A labeled button: bordered by default, filled when [`Button::accent`].
///
/// Matches the design's `.btn` / `.btn-accent`: control height, 12 px
/// horizontal padding, 1 px strong border, medium-weight 12 px label, with an
/// optional 13 px leading icon and a dimmed trailing hint (e.g. a shortcut).
pub struct Button<'a> {
    label: &'a str,
    icon: Option<Icon>,
    hint: Option<&'a str>,
    accent: bool,
    enabled: bool,
    min_width: f32,
}

impl<'a> Button<'a> {
    /// A standard bordered button.
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            icon: None,
            hint: None,
            accent: false,
            enabled: true,
            min_width: 0.0,
        }
    }

    /// Add a leading 13 px icon.
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Add a dimmed trailing hint (keyboard shortcut).
    pub fn hint(mut self, hint: &'a str) -> Self {
        self.hint = Some(hint);
        self
    }

    /// Render as the primary accent action.
    pub fn accent(mut self) -> Self {
        self.accent = true;
        self
    }

    /// Enable or disable interaction (disabled renders at 40 % opacity).
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Enforce a minimum width (label centered).
    pub fn min_width(mut self, min_width: f32) -> Self {
        self.min_width = min_width;
        self
    }

    /// Show the button.
    pub fn show(self, ui: &mut Ui) -> Response {
        let t = Tokens::get(ui.ctx());
        let c = &t.color;

        // Instrument renders accent actions in uppercase letterspaced mono —
        // a bench-instrument cue from the design spec.
        let instrument_accent = self.accent && t.direction == tokens::Direction::Instrument;
        let (text, font_id, letter_spacing) = if instrument_accent {
            (
                self.label.to_uppercase(),
                theme::mono(tokens::FS_0, FontWeight::Medium),
                0.07 * tokens::FS_0,
            )
        } else {
            (
                self.label.to_owned(),
                theme::sans(tokens::FS_1, FontWeight::Medium),
                0.0,
            )
        };

        let fg = if self.accent { c.accent_ink } else { c.text };
        let galley = {
            let mut job = egui::text::LayoutJob::default();
            job.append(
                &text,
                0.0,
                egui::TextFormat {
                    font_id: font_id.clone(),
                    color: fg,
                    extra_letter_spacing: letter_spacing,
                    ..Default::default()
                },
            );
            ui.fonts(|f| f.layout_job(job))
        };
        let hint_galley = self.hint.map(|h| {
            ui.fonts(|f| f.layout_no_wrap(h.to_owned(), font_id.clone(), fg.gamma_multiply(0.65)))
        });

        let icon_w = if self.icon.is_some() { 13.0 + 6.0 } else { 0.0 };
        let hint_w = hint_galley
            .as_ref()
            .map_or(0.0, |g| g.size().x + 5.0);
        let width = (galley.size().x + icon_w + hint_w + 24.0).max(self.min_width);
        let (rect, mut response) = ui.allocate_exact_size(
            vec2(width, t.metrics.ctl_h),
            if self.enabled {
                Sense::click()
            } else {
                Sense::hover()
            },
        );

        if !ui.is_rect_visible(rect) {
            return response;
        }

        let hover = ui
            .ctx()
            .animate_bool_with_time(response.id, response.hovered() && self.enabled, 0.16);
        let pressed = response.is_pointer_button_down_on() && self.enabled;

        let (fill, stroke_color) = if self.accent {
            // Hover brightens, press darkens the accent fill.
            let fill = if pressed {
                mix(c.accent, egui::Color32::BLACK, 0.06)
            } else {
                mix(c.accent, egui::Color32::WHITE, hover * 0.07)
            };
            (fill, fill)
        } else {
            let fill = if pressed {
                c.bg_active
            } else {
                mix(c.bg_panel, c.bg_hover, hover)
            };
            (fill, c.border_strong)
        };

        let opacity = if self.enabled { 1.0 } else { 0.4 };
        let painter = ui.painter();
        painter.rect(
            rect,
            t.radius,
            fill.gamma_multiply(opacity),
            Stroke::new(1.0, stroke_color.gamma_multiply(opacity)),
        );

        let mut x = rect.left() + 12.0;
        if let Some(icon) = self.icon {
            let icon_rect =
                Rect::from_center_size(egui::pos2(x + 6.5, rect.center().y), vec2(13.0, 13.0));
            icon.paint(painter, icon_rect, fg.gamma_multiply(opacity));
            x += 13.0 + 6.0;
        }
        let text_y = rect.center().y - galley.size().y * 0.5;
        painter.galley(
            egui::pos2(x, text_y),
            galley.clone(),
            fg.gamma_multiply(opacity),
        );
        x += galley.size().x;
        if let Some(hint) = hint_galley {
            painter.galley(
                egui::pos2(x + 5.0, rect.center().y - hint.size().y * 0.5),
                hint,
                fg.gamma_multiply(0.65 * opacity),
            );
        }

        if self.enabled {
            response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
        }
        response
    }
}

/// A 28 × 28 square tool button holding a 16 px stroke icon.
///
/// Matches the design's `.icon-btn`: transparent at rest, hover fill, active
/// fill, and an "on" state (accent wash + accent icon) for latched tools.
pub struct IconButton<'a> {
    icon: Icon,
    on: bool,
    enabled: bool,
    tooltip: Option<&'a str>,
    side: f32,
}

impl<'a> IconButton<'a> {
    /// A tool button for `icon`.
    pub fn new(icon: Icon) -> Self {
        Self {
            icon,
            on: false,
            enabled: true,
            tooltip: None,
            side: 28.0,
        }
    }

    /// Latched ("on") state — accent wash background, accent icon.
    pub fn on(mut self, on: bool) -> Self {
        self.on = on;
        self
    }

    /// Enable or disable interaction (disabled renders at 35 % opacity).
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Hover tooltip.
    pub fn tooltip(mut self, tooltip: &'a str) -> Self {
        self.tooltip = Some(tooltip);
        self
    }

    /// Override the square side length (default 28).
    pub fn side(mut self, side: f32) -> Self {
        self.side = side;
        self
    }

    /// Show the button.
    pub fn show(self, ui: &mut Ui) -> Response {
        let t = Tokens::get(ui.ctx());
        let c = &t.color;

        let (rect, mut response) = ui.allocate_exact_size(
            vec2(self.side, self.side),
            if self.enabled {
                Sense::click()
            } else {
                Sense::hover()
            },
        );
        if !ui.is_rect_visible(rect) {
            return response;
        }

        let hover = ui
            .ctx()
            .animate_bool_with_time(response.id, response.hovered() && self.enabled, 0.16);
        let pressed = response.is_pointer_button_down_on() && self.enabled;

        let opacity = if self.enabled { 1.0 } else { 0.35 };
        let (fill, icon_color) = if self.on {
            (c.accent_dim, c.accent)
        } else if pressed {
            (c.bg_active, c.text)
        } else {
            (
                mix(egui::Color32::TRANSPARENT, c.bg_hover, hover),
                mix(c.text_dim, c.text, hover),
            )
        };

        let painter = ui.painter();
        if fill != egui::Color32::TRANSPARENT {
            painter.rect_filled(rect, t.radius, fill.gamma_multiply(opacity));
        }
        let icon_rect = Rect::from_center_size(rect.center(), vec2(16.0, 16.0));
        self.icon
            .paint(painter, icon_rect, icon_color.gamma_multiply(opacity));

        if let Some(tooltip) = self.tooltip {
            response = response.on_hover_text(tooltip);
        }
        if self.enabled {
            response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
        }
        response
    }
}

