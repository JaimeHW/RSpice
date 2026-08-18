//! Buttons: the labeled control button (`Button`) and the square tool button
//! (`IconButton`).

use egui::{Rect, Response, Sense, Stroke, Ui, WidgetInfo, WidgetType, vec2};

use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight, mix};
use crate::ui::tokens::{self, Tokens};

use super::mark_response_disabled;

fn trailing_shortcut(text: &str) -> Option<&str> {
    let (prefix, suffix) = text.rsplit_once('(')?;
    let shortcut = suffix.strip_suffix(')')?.trim();
    (!prefix.trim().is_empty() && !shortcut.is_empty() && shortcut.len() <= 24).then_some(shortcut)
}

/// A labeled button: bordered by default, filled when [`Button::accent`].
///
/// Matches the design's `.button` / `.button.primary`: control height, 10 px
/// horizontal padding, 1 px border, 11 px label, with an optional 13 px
/// leading icon.
pub struct Button<'a> {
    label: &'a str,
    icon: Option<Icon>,
    accent: bool,
    ghost: bool,
    destructive: bool,
    enabled: bool,
    min_width: f32,
    min_height: f32,
    max_width: Option<f32>,
    square: bool,
    accessible_label: Option<&'a str>,
}

impl<'a> Button<'a> {
    /// A standard bordered button.
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            icon: None,
            accent: false,
            ghost: false,
            destructive: false,
            enabled: true,
            min_width: 0.0,
            min_height: 0.0,
            max_width: None,
            square: false,
            accessible_label: None,
        }
    }

    /// Add a leading 13 px icon.
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Render as the primary accent action.
    pub fn accent(mut self) -> Self {
        self.accent = true;
        self
    }

    /// Keep the standard border while using a transparent resting fill.
    pub fn ghost(mut self) -> Self {
        self.ghost = true;
        self
    }

    /// Mark the action as destructive.
    ///
    /// With [`Button::accent`] it fills with the error color — the dialog
    /// grammar's destructive primary. On a bordered button it tones the border
    /// and the label instead, which is the design's `.button.danger`: the one
    /// destructive action in a row of ordinary ones, marked without being
    /// promoted to the surface's primary.
    pub fn destructive(mut self, destructive: bool) -> Self {
        self.destructive = destructive;
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

    /// Enforce a minimum height, used by responsive surfaces whose final
    /// cascade raises interactive controls to the 44 px touch target.
    pub fn min_height(mut self, min_height: f32) -> Self {
        self.min_height = min_height;
        self
    }

    /// Constrain the control to a responsive cell and wrap its visible label.
    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = Some(max_width.max(1.0));
        self
    }

    /// Supply a contextual accessible name without changing visible copy.
    /// Drop the corner radius.
    ///
    /// For a full-bleed button that sits flush against a panel edge: a rounded
    /// corner there reads as a control floating inside the panel rather than
    /// as the panel's own last row.
    pub fn square(mut self) -> Self {
        self.square = true;
        self
    }

    pub fn accessible_label(mut self, accessible_label: &'a str) -> Self {
        self.accessible_label = Some(accessible_label);
        self
    }

    /// The label galley and the width the control is laid out at.
    ///
    /// One owner for that arithmetic, because two surfaces ask for it: the
    /// paint below, and a caller sizing a neighbouring control against the
    /// space this one leaves.
    fn measure(&self, ui: &mut Ui, fg: egui::Color32) -> (std::sync::Arc<egui::Galley>, f32) {
        let t = Tokens::get(ui.ctx());
        let font_id = theme::sans(
            tokens::FS_1,
            if self.accent {
                FontWeight::SemiBold
            } else {
                FontWeight::Regular
            },
        );
        let mut job = egui::text::LayoutJob::default();
        if let Some(max_width) = self.max_width {
            job.wrap.max_width = (max_width - 20.0).max(1.0);
        }
        job.append(
            self.label,
            0.0,
            egui::TextFormat {
                font_id,
                color: fg,
                ..Default::default()
            },
        );
        let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
        let icon_w = if self.icon.is_some() { 13.0 + 6.0 } else { 0.0 };
        let unconstrained_width = (galley.size().x + icon_w + 20.0).max(self.min_width);
        let width = self
            .max_width
            .map_or(unconstrained_width, |max_width| {
                unconstrained_width.min(max_width)
            })
            .max(if t.metrics.is_touch() {
                tokens::TOUCH_TARGET
            } else {
                0.0
            });
        (galley, width)
    }

    /// The width [`Button::show`] will take, asked before it is shown.
    ///
    /// A toolbar with one control that grows into whatever the fixed controls
    /// leave has to know that remainder before it lays the growing one out. The
    /// only honest source for it is the arithmetic the paint uses; a width
    /// authored beside the row is a second account of it, free to drift the
    /// first time a label changes.
    pub fn measured_width(&self, ui: &mut Ui) -> f32 {
        self.measure(ui, egui::Color32::PLACEHOLDER).1
    }

    /// Show the button.
    pub fn show(self, ui: &mut Ui) -> Response {
        let t = Tokens::get(ui.ctx());
        let c = &t.color;

        let fg = if self.accent {
            if self.destructive {
                // Dark ink over the error fill, mirroring accent_ink.
                mix(c.err, egui::Color32::BLACK, 0.82)
            } else {
                c.accent_ink
            }
        } else if self.destructive {
            c.err
        } else {
            c.text
        };
        let (galley, width) = self.measure(ui, fg);
        let icon_w = if self.icon.is_some() { 13.0 + 6.0 } else { 0.0 };
        let content_width = galley.size().x + icon_w;
        let height = t
            .metrics
            .ctl_h
            .max(galley.size().y + 8.0)
            .max(self.min_height);
        let (rect, mut response) = ui.allocate_exact_size(
            vec2(width, height),
            if self.enabled {
                Sense::click()
            } else {
                Sense::hover()
            },
        );
        if !self.enabled {
            mark_response_disabled(&mut response);
        }

        response.widget_info(|| {
            WidgetInfo::labeled(
                WidgetType::Button,
                self.enabled && ui.is_enabled(),
                self.accessible_label.unwrap_or(self.label),
            )
        });
        if !ui.is_rect_visible(rect) {
            return response;
        }

        let hover = ui.ctx().animate_bool_with_time(
            response.id,
            response.hovered() && self.enabled,
            ui.style().animation_time,
        );
        let pressed = response.is_pointer_button_down_on() && self.enabled;

        let (fill, stroke_color) = if self.accent {
            // Hover brightens, press darkens the fill (accent or err).
            let base = if self.destructive { c.err } else { c.accent };
            let fill = if pressed {
                mix(base, egui::Color32::BLACK, 0.06)
            } else {
                mix(base, egui::Color32::WHITE, hover * 0.07)
            };
            (fill, fill)
        } else if self.ghost {
            // Transparent at rest; the standard border remains visible.
            let fill = if pressed {
                c.bg_active
            } else {
                mix(egui::Color32::TRANSPARENT, c.bg_hover, hover)
            };
            (fill, mix(c.border, c.border_strong, hover))
        } else {
            let fill = if pressed {
                c.bg_active
            } else {
                mix(c.bg_panel_2, c.bg_hover, hover)
            };
            // A destructive bordered action carries the error tone on its
            // border as well as its label, so the mark survives a row of
            // buttons read at a glance.
            let border = if self.destructive {
                c.err
            } else {
                mix(c.border, c.border_strong, hover)
            };
            (fill, border)
        };

        let opacity = if self.enabled { 1.0 } else { 0.4 };
        let painter = ui.painter();
        painter.rect(
            rect,
            if self.square {
                egui::CornerRadius::ZERO
            } else {
                t.radius.into()
            },
            fill.gamma_multiply(opacity),
            Stroke::new(1.0, stroke_color.gamma_multiply(opacity)),
            egui::StrokeKind::Inside,
        );

        let mut x = rect.center().x - content_width * 0.5;
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
        if self.enabled {
            response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
        }
        theme::paint_focus_ring_outset(ui, &response, rect);
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
    toggle: bool,
    size: egui::Vec2,
}

impl<'a> IconButton<'a> {
    /// A tool button for `icon`.
    pub fn new(icon: Icon) -> Self {
        Self {
            icon,
            on: false,
            enabled: true,
            tooltip: None,
            toggle: false,
            size: vec2(28.0, 28.0),
        }
    }

    /// Latched ("on") state — accent wash background, accent icon.
    pub fn on(mut self, on: bool) -> Self {
        self.on = on;
        self.toggle = true;
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
        self.size = vec2(side, side);
        self
    }

    /// Override width and height independently for CSS contracts such as the
    /// desktop title/dialog icon button (28 × 27 px).
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.size = vec2(width, height);
        self
    }

    /// Show the button.
    pub fn show(self, ui: &mut Ui) -> Response {
        let t = Tokens::get(ui.ctx());
        let c = &t.color;
        let size = if t.metrics.is_touch() {
            vec2(
                self.size.x.max(tokens::TOUCH_TARGET),
                self.size.y.max(tokens::TOUCH_TARGET),
            )
        } else {
            self.size
        };

        let (rect, mut response) = ui.allocate_exact_size(
            size,
            if self.enabled {
                Sense::click()
            } else {
                Sense::hover()
            },
        );
        if !self.enabled {
            mark_response_disabled(&mut response);
        }
        let accessible_label = self.tooltip.unwrap_or_else(|| self.icon.accessible_label());
        response.widget_info(|| {
            if self.toggle {
                WidgetInfo::selected(
                    WidgetType::Button,
                    self.enabled && ui.is_enabled(),
                    self.on,
                    accessible_label,
                )
            } else {
                WidgetInfo::labeled(
                    WidgetType::Button,
                    self.enabled && ui.is_enabled(),
                    accessible_label,
                )
            }
        });
        if let Some(shortcut) = trailing_shortcut(accessible_label) {
            ui.ctx().accesskit_node_builder(response.id, |node| {
                node.set_keyboard_shortcut(shortcut);
            });
        }
        if !ui.is_rect_visible(rect) {
            return response;
        }

        let hover = ui.ctx().animate_bool_with_time(
            response.id,
            response.hovered() && self.enabled,
            ui.style().animation_time,
        );
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
            // An icon button carries no visible label, so the tooltip is the
            // only place its name is written. It has to survive being
            // disabled, which is the state a reader is most likely to be
            // hovering it to understand.
            response = if self.enabled {
                response.on_hover_text(tooltip)
            } else {
                response.on_disabled_hover_text(tooltip)
            };
        }
        if self.enabled {
            response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
        }
        theme::paint_focus_ring_outset(ui, &response, rect);
        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn accesskit_nodes(
        mut add_contents: impl FnMut(&mut egui::Ui),
    ) -> Vec<(egui::accesskit::NodeId, egui::accesskit::Node)> {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        ctx.run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| add_contents(ui));
        })
        .platform_output
        .accesskit_update
        .expect("AccessKit tree update")
        .nodes
    }

    /// Every string these buttons and their tooltips paint, in one pass.
    fn painted_text(mut add_contents: impl FnMut(&mut egui::Ui)) -> Vec<String> {
        fn walk(shape: &egui::epaint::Shape, out: &mut Vec<String>) {
            match shape {
                egui::epaint::Shape::Text(text) => out.push(text.galley.text().to_owned()),
                egui::epaint::Shape::Vec(shapes) => {
                    for shape in shapes {
                        walk(shape, out);
                    }
                }
                _ => {}
            }
        }

        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        // A tooltip opens on the pointer, and a headless pass has none. This
        // is egui's own switch for laying every popup out anyway; it does not
        // bypass the check that decides *which* tooltip a response owns, so a
        // disabled tooltip still only appears if the response reports itself
        // disabled.
        ctx.memory_mut(|memory| memory.set_everything_is_visible(true));
        // Two passes: a tooltip is an `Area`, which cannot place itself until
        // a previous pass has measured it.
        let mut shapes = Vec::new();
        for _ in 0..2 {
            shapes = ctx
                .run_ui(Default::default(), |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| add_contents(ui));
                })
                .shapes;
        }

        let mut painted = Vec::new();
        for clipped in &shapes {
            walk(&clipped.shape, &mut painted);
        }
        painted
    }

    #[test]
    fn a_disabled_button_shows_the_reason_its_call_site_attached() {
        let painted = painted_text(|ui| {
            Button::new("Run simulation")
                .enabled(false)
                .show(ui)
                .on_disabled_hover_text("No analysis is selected");
        });

        assert!(
            painted.iter().any(|text| text == "No analysis is selected"),
            "the disabled reason was never painted: {painted:?}"
        );
    }

    #[test]
    fn an_enabled_button_withholds_the_disabled_reason() {
        let painted = painted_text(|ui| {
            Button::new("Run simulation")
                .show(ui)
                .on_disabled_hover_text("No analysis is selected");
        });

        assert!(
            !painted.iter().any(|text| text == "No analysis is selected"),
            "a reachable button explained why it was blocked: {painted:?}"
        );
    }

    #[test]
    fn a_disabled_icon_button_keeps_the_tooltip_that_names_it() {
        let painted = painted_text(|ui| {
            IconButton::new(Icon::Grid)
                .tooltip("Grid visibility")
                .enabled(false)
                .show(ui);
        });

        assert!(
            painted.iter().any(|text| text == "Grid visibility"),
            "an icon-only button lost its only label: {painted:?}"
        );
    }

    #[test]
    fn labeled_button_publishes_button_role_and_name() {
        let nodes = accesskit_nodes(|ui| {
            Button::new("Run simulation").show(ui);
        });

        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Button && node.label() == Some("Run simulation")
        }));
    }

    #[test]
    fn latched_icon_button_publishes_toggle_state() {
        let nodes = accesskit_nodes(|ui| {
            IconButton::new(Icon::Grid)
                .tooltip("Grid visibility")
                .on(true)
                .show(ui);
        });

        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Button
                && node.label() == Some("Grid visibility")
                && node.toggled() == Some(egui::accesskit::Toggled::True)
        }));
    }

    #[test]
    fn shortcut_suffix_is_exposed_without_guessing_non_shortcut_tooltips() {
        assert_eq!(trailing_shortcut("Run simulation (F5)"), Some("F5"));
        assert_eq!(
            trailing_shortcut("Redo (Ctrl+Shift+Z)"),
            Some("Ctrl+Shift+Z")
        );
        assert_eq!(trailing_shortcut("Grid: dots - click for lines"), None);
    }

    #[test]
    fn labeled_button_accepts_exact_responsive_minimum_height() {
        let button = Button::new("Clear read").min_height(44.0);
        assert_eq!(button.min_height, 44.0);
    }
}
