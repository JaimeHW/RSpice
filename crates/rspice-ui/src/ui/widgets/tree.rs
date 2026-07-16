//! Tree rows — the list/tree row used across hierarchy trees, cell lists,
//! signal browsers and run history.

use std::ops::Range;

use egui::{Rect, Response, Sense, Stroke, Ui, WidgetInfo, WidgetType, vec2};

use crate::ui::theme::{self, FontWeight, mix};
use crate::ui::tokens::{self, Tokens};

/// What happened to a [`TreeRow`] this frame.
pub struct TreeRowResult {
    /// The row's interaction response (click, double-click, context menu).
    pub response: Response,
    /// `true` if an embedded checkbox changed value this frame.
    pub checkbox_changed: bool,
}

/// A full-width interactive row with optional twist (expand) arrow, leading
/// color dot or checkbox, and right-aligned mono metadata.
pub struct TreeRow<'a> {
    label: &'a str,
    meta: Option<&'a str>,
    /// `Some(expanded)` shows a twist arrow.
    twist: Option<bool>,
    chip_dot: Option<egui::Color32>,
    checkbox: Option<&'a mut bool>,
    indent: u8,
    selected: bool,
    mono_label: bool,
    dim: bool,
    height: Option<f32>,
    highlight_query: Option<&'a str>,
}

impl<'a> TreeRow<'a> {
    /// A row with the given primary label.
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            meta: None,
            twist: None,
            chip_dot: None,
            checkbox: None,
            indent: 0,
            selected: false,
            mono_label: false,
            dim: false,
            height: None,
            highlight_query: None,
        }
    }

    /// Right-aligned faint mono metadata text.
    pub fn meta(mut self, meta: &'a str) -> Self {
        self.meta = Some(meta);
        self
    }

    /// Show a twist (expand/collapse) arrow; `expanded` selects ▾ vs ▸.
    pub fn twist(mut self, expanded: bool) -> Self {
        self.twist = Some(expanded);
        self
    }

    /// Leading 8 × 8 color dot (trace color, run status).
    pub fn chip_dot(mut self, color: egui::Color32) -> Self {
        self.chip_dot = Some(color);
        self
    }

    /// Embed a checkbox; clicking the row toggles it.
    pub fn checkbox(mut self, value: &'a mut bool) -> Self {
        self.checkbox = Some(value);
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

    /// Render as a ghost row (faint label, no hover brightening) — peeked
    /// master contents that are visible but not editable from here.
    pub fn dim(mut self) -> Self {
        self.dim = true;
        self
    }

    /// Override the row height (defaults to the density row height).
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    /// Highlight ASCII case-insensitive matches in the label and metadata.
    pub fn highlight_query(mut self, query: impl Into<Option<&'a str>>) -> Self {
        self.highlight_query = query.into().filter(|query| !query.is_empty());
        self
    }

    /// Show the row, filling the available width.
    pub fn show(self, ui: &mut Ui) -> TreeRowResult {
        let t = Tokens::get(ui.ctx());
        let c = &t.color;
        let row_h = self.height.unwrap_or(t.metrics.row_h);

        let width = ui.available_width();
        let (rect, mut response) = ui.allocate_exact_size(vec2(width, row_h), Sense::click());

        let mut checkbox_changed = false;
        let has_checkbox = self.checkbox.is_some();
        let mut checkbox_value_after = self.checkbox.as_ref().map(|v| **v);
        if let Some(value) = self.checkbox {
            if response.clicked() {
                *value = !*value;
                checkbox_changed = true;
                response.mark_changed();
            }
            checkbox_value_after = Some(*value);
        }
        let accessible_label = self.meta.map_or_else(
            || self.label.to_owned(),
            |meta| format!("{}, {}", self.label, meta),
        );
        response.widget_info(|| {
            if let Some(value) = checkbox_value_after.filter(|_| has_checkbox) {
                WidgetInfo::selected(
                    WidgetType::Checkbox,
                    ui.is_enabled(),
                    value,
                    &accessible_label,
                )
            } else if self.twist.is_some() {
                WidgetInfo::labeled(
                    WidgetType::CollapsingHeader,
                    ui.is_enabled(),
                    &accessible_label,
                )
            } else {
                WidgetInfo::labeled(
                    WidgetType::SelectableLabel,
                    ui.is_enabled(),
                    &accessible_label,
                )
            }
        });
        ui.ctx().accesskit_node_builder(response.id, |node| {
            if !has_checkbox {
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
                checkbox_changed,
            };
        }

        let hover = ui
            .ctx()
            .animate_bool_with_time(response.id, response.hovered(), 0.16);

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

        let text_color = if self.dim {
            c.text_faint
        } else if self.selected {
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

        if let Some(value) = checkbox_value_after {
            let box_rect = Rect::from_center_size(egui::pos2(x + 6.5, cy), vec2(13.0, 13.0));
            if value {
                painter.rect_filled(box_rect, t.radius.min(2.0), c.accent);
                // Check mark.
                let s = box_rect.width();
                painter.add(egui::Shape::line(
                    vec![
                        box_rect.left_top() + vec2(0.25 * s, 0.55 * s),
                        box_rect.left_top() + vec2(0.42 * s, 0.72 * s),
                        box_rect.left_top() + vec2(0.78 * s, 0.30 * s),
                    ],
                    Stroke::new(1.6, c.accent_ink),
                ));
            } else {
                painter.rect(
                    box_rect,
                    t.radius.min(2.0),
                    c.bg_inset,
                    Stroke::new(1.0, c.border_strong),
                    egui::StrokeKind::Inside,
                );
            }
            x += 13.0 + 6.0;
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
        let highlight_bg = egui::Color32::from_rgba_unmultiplied(
            c.accent.r(),
            c.accent.g(),
            c.accent.b(),
            if self.dim { 26 } else { 54 },
        );
        let meta_galley = self.meta.map(|m| {
            ui.fonts_mut(|f| {
                f.layout_job(highlighted_singleline_job(
                    m,
                    theme::mono(tokens::FS_0, FontWeight::Regular),
                    egui::Color32::PLACEHOLDER,
                    highlight_bg,
                    self.highlight_query,
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
            let mut job = highlighted_singleline_job(
                self.label,
                label_font,
                egui::Color32::PLACEHOLDER,
                highlight_bg,
                self.highlight_query,
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
            checkbox_changed,
        }
    }
}

fn highlighted_singleline_job(
    text: &str,
    font_id: egui::FontId,
    color: egui::Color32,
    highlight_bg: egui::Color32,
    query: Option<&str>,
) -> egui::text::LayoutJob {
    let ranges = query.map_or_else(Vec::new, |query| highlight_match_ranges(text, query));
    if ranges.is_empty() {
        return egui::text::LayoutJob::simple_singleline(text.to_owned(), font_id, color);
    }

    let normal = egui::text::TextFormat::simple(font_id, color);
    let highlight = egui::text::TextFormat {
        background: highlight_bg,
        ..normal.clone()
    };
    let mut job = egui::text::LayoutJob {
        break_on_newline: false,
        ..Default::default()
    };
    let mut cursor = 0;
    for range in ranges {
        if cursor < range.start {
            job.append(&text[cursor..range.start], 0.0, normal.clone());
        }
        job.append(&text[range.clone()], 0.0, highlight.clone());
        cursor = range.end;
    }
    if cursor < text.len() {
        job.append(&text[cursor..], 0.0, normal);
    }
    job
}

fn highlight_match_ranges(text: &str, query: &str) -> Vec<Range<usize>> {
    let needle = query.as_bytes();
    if needle.is_empty() || !needle.iter().all(u8::is_ascii) {
        return Vec::new();
    }

    let haystack = text.as_bytes();
    if needle.len() > haystack.len() {
        return Vec::new();
    }

    let mut ranges = Vec::new();
    let mut start = 0;
    while start + needle.len() <= haystack.len() {
        let matched = haystack[start..start + needle.len()]
            .iter()
            .zip(needle.iter())
            .all(|(hay, needle)| hay.eq_ignore_ascii_case(needle));
        if matched {
            ranges.push(start..start + needle.len());
            start += needle.len();
        } else {
            start += 1;
        }
    }
    ranges
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
        ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| add_contents(ui));
        })
        .platform_output
        .accesskit_update
        .expect("AccessKit tree update")
        .nodes
    }

    #[test]
    fn highlight_ranges_match_case_insensitive_ascii() {
        assert_eq!(highlight_match_ranges("VDD_net", "dd"), vec![1..3]);
    }

    #[test]
    fn highlight_ranges_include_multiple_occurrences() {
        assert_eq!(highlight_match_ranges("abababa", "aba"), vec![0..3, 4..7]);
    }

    #[test]
    fn highlight_ranges_are_empty_for_empty_query() {
        assert!(highlight_match_ranges("VDD_net", "").is_empty());
    }

    #[test]
    fn highlight_ranges_are_empty_when_no_match() {
        assert!(highlight_match_ranges("VDD_net", "clk").is_empty());
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

    #[test]
    fn checkbox_tree_row_publishes_checkbox_state() {
        let mut checked = true;
        let nodes = accesskit_nodes(|ui| {
            TreeRow::new("V(out)").checkbox(&mut checked).show(ui);
        });

        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::CheckBox
                && node.label() == Some("V(out)")
                && node.toggled() == Some(egui::accesskit::Toggled::True)
        }));
    }
}
