//! Tree rows — the list/tree row used across hierarchy trees, cell lists,
//! signal browsers and run history.

use egui::{Rect, Response, Sense, Stroke, Ui, WidgetInfo, WidgetType, vec2};

use crate::ui::theme::{self, FontWeight, mix};
use crate::ui::tokens::{self, Tokens};

/// What happened to a [`TreeRow`] this frame.
pub struct TreeRowResult {
    /// The row's interaction response (click, double-click, context menu).
    pub response: Response,
    /// `true` if an embedded switch changed value this frame.
    pub switch_changed: bool,
}

/// A full-width interactive row with optional twist (expand) arrow, leading
/// color dot or switch, and right-aligned mono metadata.
pub struct TreeRow<'a> {
    label: &'a str,
    meta: Option<&'a str>,
    /// `Some(expanded)` shows a twist arrow.
    twist: Option<bool>,
    chip_dot: Option<egui::Color32>,
    switch: Option<&'a mut bool>,
    indent: u8,
    selected: bool,
    mono_label: bool,
    height: Option<f32>,
}

impl<'a> TreeRow<'a> {
    /// A row with the given primary label.
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            meta: None,
            twist: None,
            chip_dot: None,
            switch: None,
            indent: 0,
            selected: false,
            mono_label: false,
            height: None,
        }
    }

    /// Right-aligned faint mono metadata text.
    pub fn meta(mut self, meta: &'a str) -> Self {
        self.meta = Some(meta);
        self
    }

    /// Show a twist (expand/collapse) arrow; `expanded` selects ▾ vs ▸.
    #[cfg(test)]
    pub fn twist(mut self, expanded: bool) -> Self {
        self.twist = Some(expanded);
        self
    }

    /// Leading 8 × 8 color dot (trace color, run status).
    pub fn chip_dot(mut self, color: egui::Color32) -> Self {
        self.chip_dot = Some(color);
        self
    }

    /// Embed an on/off switch; clicking the row toggles it.
    ///
    /// This painted egui's tick box until the whole product's booleans were
    /// one control. The switch it paints now is the design system's own, from
    /// [`crate::ui::widgets::paint_switch`] — so the form row that falls back
    /// to this shape in a narrow column no longer shows a reader an idiom
    /// nothing else in the application uses.
    pub fn switch(mut self, value: &'a mut bool) -> Self {
        self.switch = Some(value);
        self
    }

    /// Indent level (0, 1, 2 → 0 / 16 / 32 px).
    pub fn indent(mut self, level: u8) -> Self {
        self.indent = level;
        self
    }

    /// Render in the selected state (accent wash).
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Render the primary label in the mono face (cell names, signals).
    pub fn mono(mut self) -> Self {
        self.mono_label = true;
        self
    }

    /// Override the row height (defaults to the density row height).
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    /// Show the row, filling the available width.
    pub fn show(self, ui: &mut Ui) -> TreeRowResult {
        let t = Tokens::get(ui.ctx());
        let c = &t.color;
        let row_h = self.height.unwrap_or(t.metrics.row_h);

        let width = ui.available_width();
        let (rect, mut response) = ui.allocate_exact_size(vec2(width, row_h), Sense::click());

        // A self-painted control gets none of egui's disabled handling: this
        // row allocates its own rect and reads its own response, so the
        // enabled bit is read once here and carried into the toggle, the paint
        // and the announcement alike.
        let enabled = ui.is_enabled();
        let mut switch_changed = false;
        let has_switch = self.switch.is_some();
        let mut switch_value_after = self.switch.as_ref().map(|v| **v);
        if let Some(value) = self.switch {
            if enabled && response.clicked() {
                *value = !*value;
                switch_changed = true;
                response.mark_changed();
            }
            switch_value_after = Some(*value);
        }
        let accessible_label = self.meta.map_or_else(
            || self.label.to_owned(),
            |meta| format!("{}, {}", self.label, meta),
        );
        response.widget_info(|| {
            if let Some(value) = switch_value_after.filter(|_| has_switch) {
                // `Checkbox` is the role a two-state control publishes, and it
                // is what the switches elsewhere in the product announce: the
                // paint changed, what a reader's assistive technology is
                // offered did not.
                WidgetInfo::selected(WidgetType::Checkbox, enabled, value, &accessible_label)
            } else if self.twist.is_some() {
                WidgetInfo::labeled(WidgetType::CollapsingHeader, enabled, &accessible_label)
            } else {
                WidgetInfo::labeled(WidgetType::SelectableLabel, enabled, &accessible_label)
            }
        });
        ui.ctx().accesskit_node_builder(response.id, |node| {
            if !has_switch {
                node.set_role(egui::accesskit::Role::TreeItem);
                node.set_selected(self.selected);
                node.set_level(usize::from(self.indent) + 1);
                if let Some(expanded) = self.twist {
                    node.set_expanded(expanded);
                }
            }
        });

        if !ui.is_rect_visible(rect) {
            return TreeRowResult {
                response,
                switch_changed,
            };
        }

        let hover = ui.ctx().animate_bool_with_time(
            response.id,
            response.hovered(),
            ui.style().animation_time,
        );

        let painter = ui.painter();
        if self.selected {
            painter.rect_filled(rect, t.radius, c.accent_dim);
        } else if hover > 0.0 {
            painter.rect_filled(
                rect,
                t.radius,
                mix(egui::Color32::TRANSPARENT, c.bg_hover, hover),
            );
        }

        let text_color = if self.selected {
            c.text
        } else {
            mix(c.text_dim, c.text, hover)
        };

        let mut x = rect.left() + 6.0 + f32::from(self.indent) * 16.0;
        let cy = rect.center().y;

        if let Some(expanded) = self.twist {
            // Painted triangle — text glyphs (▾/▸) are not covered by the
            // embedded fonts and render as tofu boxes.
            let center = egui::pos2(x + 4.0, cy);
            let r = 3.2;
            let points = if expanded {
                vec![
                    center + vec2(-r, -r * 0.5),
                    center + vec2(r, -r * 0.5),
                    center + vec2(0.0, r * 0.9),
                ]
            } else {
                vec![
                    center + vec2(-r * 0.5, -r),
                    center + vec2(r * 0.9, 0.0),
                    center + vec2(-r * 0.5, r),
                ]
            };
            painter.add(egui::Shape::convex_polygon(
                points,
                c.text_faint,
                Stroke::NONE,
            ));
            x += 12.0;
        }

        if let Some(value) = switch_value_after {
            super::paint_switch(
                ui,
                egui::pos2(x + super::SWITCH_WIDTH * 0.5, cy),
                value,
                enabled && response.hovered(),
                rect,
            );
            x += super::SWITCH_WIDTH + 6.0;
        }

        if let Some(dot) = self.chip_dot {
            let dot_rect = Rect::from_center_size(egui::pos2(x + 4.0, cy), vec2(8.0, 8.0));
            painter.rect_filled(dot_rect, 2.0, dot);
            x += 8.0 + 6.0;
        }

        let label_font = if self.mono_label {
            theme::mono(tokens::FS_1, FontWeight::Regular)
        } else {
            theme::sans(tokens::FS_1, FontWeight::Regular)
        };
        let meta_galley = self.meta.map(|m| {
            ui.fonts_mut(|f| {
                f.layout_job(egui::text::LayoutJob::simple_singleline(
                    m.to_owned(),
                    theme::mono(tokens::FS_0, FontWeight::Regular),
                    egui::Color32::PLACEHOLDER,
                ))
            })
        });
        let meta_w = meta_galley.as_ref().map_or(0.0, |g| g.size().x + 8.0);

        // Lay out with the placeholder color and tint at paint time: the
        // hover animation sweeps `text_color` through ten shades, and a
        // color-keyed galley would miss egui's cache on every one.
        // Truncate (never wrap): rows are fixed-height, and a long name
        // spilling onto a second line corrupts the list rhythm. The full
        // name surfaces as a tooltip instead.
        let label_galley = ui.fonts_mut(|f| {
            let mut job = egui::text::LayoutJob::simple_singleline(
                self.label.to_owned(),
                label_font,
                egui::Color32::PLACEHOLDER,
            );
            job.wrap = egui::text::TextWrapping::truncate_at_width(
                (rect.right() - 6.0 - meta_w - x).max(8.0),
            );
            f.layout_job(job)
        });
        let label_elided = label_galley.elided;
        painter.galley(
            egui::pos2(x, cy - label_galley.size().y * 0.5),
            label_galley,
            text_color,
        );

        if let Some(galley) = meta_galley {
            painter.galley(
                egui::pos2(
                    rect.right() - 6.0 - galley.size().x,
                    cy - galley.size().y * 0.5,
                ),
                galley,
                c.text_faint,
            );
        }

        theme::paint_focus_ring_outset(ui, &response, rect);

        let mut response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
        if label_elided {
            response = response.on_hover_text(self.label);
        }
        TreeRowResult {
            response,
            switch_changed,
        }
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

    #[test]
    fn tree_row_publishes_tree_state_without_toggle_state() {
        let nodes = accesskit_nodes(|ui| {
            TreeRow::new("amplifier")
                .meta("sheet")
                .twist(true)
                .indent(2)
                .selected(true)
                .show(ui);
        });

        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::TreeItem
                && node.label() == Some("amplifier, sheet")
                && node.is_selected() == Some(true)
                && node.is_expanded() == Some(true)
                && node.level() == Some(3)
                && node.toggled().is_none()
        }));
    }

    /// A row inside a disabled `Ui` does not move when it is clicked.
    ///
    /// A self-painted control gets none of egui's disabled handling: the row
    /// allocates its own rect and reads its own response, so the enabled bit
    /// is honoured here or nowhere. The DC sweep draws its two mutually
    /// exclusive settings exactly this way — each inside
    /// `add_enabled_ui(!the_other, ..)` — so a row that toggled anyway would
    /// let a reader configure the pair the form exists to keep apart.
    #[test]
    fn a_disabled_switch_row_does_not_toggle_when_it_is_clicked() {
        fn press(enabled: bool) -> bool {
            let ctx = egui::Context::default();
            crate::ui::Theme::default().apply(&ctx);
            let mut value = false;
            let mut at = egui::Pos2::ZERO;
            // Two passes to measure the row against the fonts, then one that
            // presses its centre.
            for pass in 0..3 {
                let events = if pass == 2 {
                    vec![
                        egui::Event::PointerMoved(at),
                        egui::Event::PointerButton {
                            pos: at,
                            button: egui::PointerButton::Primary,
                            pressed: true,
                            modifiers: egui::Modifiers::default(),
                        },
                        egui::Event::PointerButton {
                            pos: at,
                            button: egui::PointerButton::Primary,
                            pressed: false,
                            modifiers: egui::Modifiers::default(),
                        },
                    ]
                } else {
                    Vec::new()
                };
                let mut measured = Rect::NOTHING;
                // The frame's output is not the subject here: what the pass is
                // for is the value the row wrote and the rect it landed in.
                let _ = ctx.run_ui(
                    egui::RawInput {
                        screen_rect: Some(Rect::from_min_size(
                            egui::Pos2::ZERO,
                            vec2(240.0, 200.0),
                        )),
                        events,
                        ..Default::default()
                    },
                    |ctx| {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            ui.add_enabled_ui(enabled, |ui| {
                                measured = TreeRow::new("Nested sweep")
                                    .switch(&mut value)
                                    .show(ui)
                                    .response
                                    .rect;
                            });
                        });
                    },
                );
                at = measured.center();
            }
            value
        }

        assert!(
            press(true),
            "an enabled switch row must toggle on the click that asked"
        );
        assert!(
            !press(false),
            "a disabled one must not, self-painted or otherwise"
        );
    }

    #[test]
    fn switch_tree_row_publishes_toggle_state() {
        let mut checked = true;
        let nodes = accesskit_nodes(|ui| {
            TreeRow::new("V(out)").switch(&mut checked).show(ui);
        });

        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::CheckBox
                && node.label() == Some("V(out)")
                && node.toggled() == Some(egui::accesskit::Toggled::True)
        }));
    }
}
