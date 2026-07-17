//! Workbench-specific visual primitives.
//!
//! These controls are intentionally small and stateless.  They render from
//! the shared palette and typography assets, but none of the retired layout's
//! layout or widget implementations are reused.

use egui::{Align2, Color32, Pos2, Rect, Response, Sense, Shape, Stroke, Ui, Vec2};
use unicode_segmentation::UnicodeSegmentation;

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

pub const TITLE_BAR_H: f32 = 35.0;
pub const TOOL_BAR_H: f32 = 45.0;
pub const DOCUMENT_BAR_H: f32 = 34.0;
pub const STATUS_BAR_H: f32 = 25.0;
pub const ACTIVITY_RAIL_W: f32 = 51.0;
pub const PHONE_NAV_H: f32 = 54.0;
pub const TOUCH_TARGET: f32 = 44.0;
pub const PANEL_HEADER_H: f32 = 39.0;
pub const PANEL_TABS_H: f32 = 31.0;
pub const PANEL_SECTION_H: f32 = 29.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkbenchIcon {
    Brand,
    Project,
    Design,
    Simulate,
    Results,
    Verify,
    Models,
    Netlist,
    Search,
    Settings,
    Bell,
    User,
    Navigator,
    Inspector,
    Console,
    Select,
    Wire,
    Bus,
    Label,
    Probe,
    Rotate,
    Mirror,
    Undo,
    Redo,
    ZoomOut,
    ZoomIn,
    ZoomFit,
    Grid,
    Check,
    Focus,
    Run,
    Stop,
    ChevronDown,
    Close,
    More,
    Add,
    Folder,
    Save,
    History,
    Refresh,
    Copy,
    Trash,
    ArrowLeft,
    Component,
    Code,
    Terminal,
    Compare,
    Info,
    Warning,
    Success,
    File,
    Sliders,
    Target,
    Export,
}

impl WorkbenchIcon {
    pub fn paint(self, painter: &egui::Painter, rect: Rect, color: Color32) {
        let side = rect.width().min(rect.height());
        let scale = side / 24.0;
        let origin = rect.center() - Vec2::splat(side * 0.5);
        let p = |x: f32, y: f32| Pos2::new(origin.x + x * scale, origin.y + y * scale);
        let stroke = Stroke::new((1.6 * scale).max(1.0), color);
        let line = |points: &[(f32, f32)]| {
            painter.add(Shape::line(
                points.iter().map(|&(x, y)| p(x, y)).collect(),
                stroke,
            ));
        };
        let closed = |points: &[(f32, f32)]| {
            painter.add(Shape::closed_line(
                points.iter().map(|&(x, y)| p(x, y)).collect(),
                stroke,
            ));
        };

        match self {
            Self::Brand => {
                painter.rect_filled(rect, side * 21.0 / 96.0, color);
                let ink = Tokens::get(painter.ctx()).color.accent_ink;
                let terminal_stroke = Stroke::new((2.0 * scale).max(1.2), ink);
                line_colored(painter, &[p(6.05, 9.55), p(8.41, 9.55)], terminal_stroke);
                line_colored(painter, &[p(6.05, 14.45), p(8.41, 14.45)], terminal_stroke);
                line_colored(painter, &[p(15.76, 12.0), p(17.95, 12.0)], terminal_stroke);
                for center in [p(5.0, 9.55), p(5.0, 14.45), p(19.0, 12.0)] {
                    painter.circle_stroke(
                        center,
                        1.05 * scale,
                        Stroke::new((1.0 * scale).max(0.8), ink),
                    );
                }
                painter.add(Shape::convex_polygon(
                    vec![p(8.06, 7.21), p(8.06, 16.79), p(15.94, 12.0)],
                    ink,
                    Stroke::new((1.4 * scale).max(1.0), ink),
                ));
            }
            Self::Project => {
                line(&[(4.0, 10.0), (12.0, 4.0), (20.0, 10.0)]);
                closed(&[(6.0, 9.0), (18.0, 9.0), (18.0, 20.0), (6.0, 20.0)]);
            }
            Self::Design => {
                line(&[
                    (4.0, 18.0),
                    (9.0, 18.0),
                    (9.0, 12.0),
                    (15.0, 12.0),
                    (15.0, 6.0),
                    (20.0, 6.0),
                ]);
                painter.circle_filled(p(4.0, 18.0), 1.5 * scale, color);
                painter.circle_filled(p(20.0, 6.0), 1.5 * scale, color);
            }
            Self::Simulate => line(&[
                (3.0, 13.0),
                (7.0, 13.0),
                (10.0, 5.0),
                (14.0, 19.0),
                (17.0, 11.0),
                (21.0, 11.0),
            ]),
            Self::Results => {
                line(&[
                    (4.0, 18.0),
                    (4.0, 11.0),
                    (9.0, 11.0),
                    (9.0, 6.0),
                    (14.0, 6.0),
                    (14.0, 14.0),
                    (20.0, 14.0),
                ]);
                line(&[(4.0, 20.0), (20.0, 20.0)]);
            }
            Self::Verify => {
                closed(&[
                    (12.0, 3.0),
                    (20.0, 6.0),
                    (18.5, 16.0),
                    (12.0, 21.0),
                    (5.5, 16.0),
                    (4.0, 6.0),
                ]);
                line(&[(8.0, 12.0), (11.0, 15.0), (16.0, 9.0)]);
            }
            Self::Models => {
                closed(&[(4.0, 5.0), (20.0, 5.0), (20.0, 10.0), (4.0, 10.0)]);
                closed(&[(4.0, 14.0), (20.0, 14.0), (20.0, 19.0), (4.0, 19.0)]);
                line(&[(8.0, 5.0), (8.0, 10.0), (8.0, 14.0), (8.0, 19.0)]);
            }
            Self::Netlist => {
                line(&[(8.0, 5.0), (3.0, 12.0), (8.0, 19.0)]);
                line(&[(16.0, 5.0), (21.0, 12.0), (16.0, 19.0)]);
                line(&[(14.0, 4.0), (10.0, 20.0)]);
            }
            Self::Search => {
                painter.circle_stroke(p(10.0, 10.0), 5.5 * scale, stroke);
                line(&[(14.0, 14.0), (20.0, 20.0)]);
            }
            Self::Settings => {
                painter.circle_stroke(p(12.0, 12.0), 3.0 * scale, stroke);
                painter.circle_stroke(p(12.0, 12.0), 8.0 * scale, stroke);
                for (from, to) in [
                    ((12.0, 2.8), (12.0, 5.0)),
                    ((12.0, 19.0), (12.0, 21.2)),
                    ((2.8, 12.0), (5.0, 12.0)),
                    ((19.0, 12.0), (21.2, 12.0)),
                    ((5.5, 5.5), (7.0, 7.0)),
                    ((17.0, 17.0), (18.5, 18.5)),
                    ((18.5, 5.5), (17.0, 7.0)),
                    ((7.0, 17.0), (5.5, 18.5)),
                ] {
                    line(&[from, to]);
                }
            }
            Self::Sliders => {
                for (y, x) in [(6.0, 9.0), (12.0, 15.0), (18.0, 11.0)] {
                    line(&[(4.0, y), (20.0, y)]);
                    painter.circle_filled(p(x, y), 2.0 * scale, color);
                }
            }
            Self::Bell => {
                line(&[(6.0, 16.0), (8.0, 13.0), (8.0, 9.0)]);
                painter.add(Shape::line(
                    (0..=12)
                        .map(|i| {
                            let a = std::f32::consts::PI * i as f32 / 12.0;
                            p(12.0 + 4.0 * a.cos(), 10.0 - 5.0 * a.sin())
                        })
                        .collect(),
                    stroke,
                ));
                line(&[(16.0, 9.0), (16.0, 13.0), (18.0, 16.0), (6.0, 16.0)]);
                painter.circle_filled(p(12.0, 19.0), 1.4 * scale, color);
            }
            Self::User => {
                painter.circle_stroke(p(12.0, 8.0), 3.0 * scale, stroke);
                painter.circle_stroke(p(12.0, 20.0), 7.0 * scale, stroke);
            }
            Self::Navigator => {
                closed(&[(4.0, 5.0), (20.0, 5.0), (20.0, 19.0), (4.0, 19.0)]);
                line(&[(9.0, 5.0), (9.0, 19.0)]);
            }
            Self::Inspector => {
                closed(&[(4.0, 5.0), (20.0, 5.0), (20.0, 19.0), (4.0, 19.0)]);
                line(&[(15.0, 5.0), (15.0, 19.0)]);
            }
            Self::Console => {
                closed(&[(4.0, 5.0), (20.0, 5.0), (20.0, 19.0), (4.0, 19.0)]);
                line(&[(4.0, 14.0), (20.0, 14.0)]);
            }
            Self::Select => closed(&[(5.0, 3.0), (19.0, 11.0), (12.5, 12.5), (9.0, 19.0)]),
            Self::Wire => {
                line(&[(4.0, 18.0), (11.0, 18.0), (11.0, 6.0), (20.0, 6.0)]);
                painter.circle_filled(p(4.0, 18.0), 1.5 * scale, color);
                painter.circle_filled(p(20.0, 6.0), 1.5 * scale, color);
            }
            Self::Bus => {
                line(&[(4.0, 17.0), (10.0, 17.0), (10.0, 7.0), (20.0, 7.0)]);
                line(&[(4.0, 20.0), (13.0, 20.0), (13.0, 10.0), (20.0, 10.0)]);
            }
            Self::Label => {
                line(&[(5.0, 5.0), (19.0, 5.0)]);
                line(&[(12.0, 5.0), (12.0, 20.0)]);
            }
            Self::Probe => {
                painter.circle_stroke(p(10.0, 10.0), 5.5 * scale, stroke);
                line(&[(14.5, 14.5), (20.0, 20.0)]);
            }
            Self::Rotate => {
                painter.circle_stroke(p(12.0, 12.0), 7.0 * scale, stroke);
                line(&[(17.0, 5.0), (21.0, 5.0), (21.0, 9.0)]);
            }
            Self::Mirror => {
                line(&[(12.0, 3.0), (12.0, 21.0)]);
                closed(&[(4.0, 7.0), (10.0, 12.0), (4.0, 17.0)]);
                closed(&[(20.0, 7.0), (14.0, 12.0), (20.0, 17.0)]);
            }
            Self::Undo => {
                line(&[(8.0, 5.0), (3.0, 10.0), (8.0, 15.0)]);
                line(&[(3.0, 10.0), (14.0, 10.0), (19.0, 14.0), (19.0, 19.0)]);
            }
            Self::Redo => {
                line(&[(16.0, 5.0), (21.0, 10.0), (16.0, 15.0)]);
                line(&[(21.0, 10.0), (10.0, 10.0), (5.0, 14.0), (5.0, 19.0)]);
            }
            Self::ZoomOut | Self::ZoomIn => {
                painter.circle_stroke(p(10.0, 10.0), 5.5 * scale, stroke);
                line(&[(14.0, 14.0), (20.0, 20.0)]);
                line(&[(7.0, 10.0), (13.0, 10.0)]);
                if self == Self::ZoomIn {
                    line(&[(10.0, 7.0), (10.0, 13.0)]);
                }
            }
            Self::ZoomFit | Self::Focus => {
                line(&[(4.0, 9.0), (4.0, 4.0), (9.0, 4.0)]);
                line(&[(20.0, 9.0), (20.0, 4.0), (15.0, 4.0)]);
                line(&[(4.0, 15.0), (4.0, 20.0), (9.0, 20.0)]);
                line(&[(20.0, 15.0), (20.0, 20.0), (15.0, 20.0)]);
            }
            Self::Grid => {
                for x in [7.0, 12.0, 17.0] {
                    for y in [7.0, 12.0, 17.0] {
                        painter.circle_filled(p(x, y), 1.0 * scale, color);
                    }
                }
            }
            Self::Check | Self::Success => line(&[(5.0, 12.0), (10.0, 17.0), (20.0, 6.0)]),
            Self::Run => {
                painter.add(Shape::convex_polygon(
                    vec![p(7.0, 4.0), p(20.0, 12.0), p(7.0, 20.0)],
                    color,
                    Stroke::NONE,
                ));
            }
            Self::Stop => {
                painter.rect_filled(Rect::from_min_max(p(7.0, 7.0), p(17.0, 17.0)), 1.0, color);
            }
            Self::ChevronDown => line(&[(6.0, 9.0), (12.0, 15.0), (18.0, 9.0)]),
            Self::Close => {
                line(&[(6.0, 6.0), (18.0, 18.0)]);
                line(&[(18.0, 6.0), (6.0, 18.0)]);
            }
            Self::More => {
                for x in [6.0, 12.0, 18.0] {
                    painter.circle_filled(p(x, 12.0), 1.5 * scale, color);
                }
            }
            Self::Add => {
                line(&[(12.0, 5.0), (12.0, 19.0)]);
                line(&[(5.0, 12.0), (19.0, 12.0)]);
            }
            Self::Folder => closed(&[
                (3.0, 7.0),
                (10.0, 7.0),
                (12.0, 10.0),
                (21.0, 10.0),
                (19.0, 19.0),
                (3.0, 19.0),
            ]),
            Self::Save => {
                closed(&[
                    (4.0, 4.0),
                    (18.0, 4.0),
                    (20.0, 6.0),
                    (20.0, 20.0),
                    (4.0, 20.0),
                ]);
                closed(&[(8.0, 4.0), (16.0, 4.0), (16.0, 10.0), (8.0, 10.0)]);
                closed(&[(8.0, 14.0), (16.0, 14.0), (16.0, 20.0), (8.0, 20.0)]);
            }
            Self::History => {
                painter.circle_stroke(p(12.0, 12.0), 8.0 * scale, stroke);
                line(&[(12.0, 7.0), (12.0, 12.0), (16.0, 14.0)]);
            }
            Self::Refresh => {
                painter.add(Shape::line(
                    (0..=12)
                        .map(|index| {
                            let angle = -2.7 + 4.15 * index as f32 / 12.0;
                            p(12.0 + 8.0 * angle.cos(), 12.0 + 8.0 * angle.sin())
                        })
                        .collect(),
                    stroke,
                ));
                line(&[(20.0, 7.0), (20.0, 12.0), (15.0, 12.0)]);
                painter.add(Shape::line(
                    (0..=12)
                        .map(|index| {
                            let angle = 0.45 + 4.15 * index as f32 / 12.0;
                            p(12.0 + 8.0 * angle.cos(), 12.0 + 8.0 * angle.sin())
                        })
                        .collect(),
                    stroke,
                ));
                line(&[(4.0, 17.0), (4.0, 12.0), (9.0, 12.0)]);
            }
            Self::Copy => {
                painter.rect_stroke(
                    Rect::from_min_max(p(8.0, 8.0), p(19.0, 20.0)),
                    1.0,
                    stroke,
                    egui::StrokeKind::Inside,
                );
                line(&[
                    (15.0, 5.0),
                    (15.0, 4.0),
                    (4.0, 4.0),
                    (4.0, 16.0),
                    (5.0, 16.0),
                ]);
            }
            Self::Trash => {
                line(&[(4.0, 7.0), (20.0, 7.0)]);
                line(&[(9.0, 3.0), (15.0, 3.0), (16.0, 7.0), (8.0, 7.0)]);
                closed(&[(6.0, 7.0), (7.0, 21.0), (17.0, 21.0), (18.0, 7.0)]);
                line(&[(10.0, 11.0), (10.0, 17.0)]);
                line(&[(14.0, 11.0), (14.0, 17.0)]);
            }
            Self::ArrowLeft => {
                line(&[(15.0, 5.0), (8.0, 12.0), (15.0, 19.0)]);
                line(&[(8.0, 12.0), (21.0, 12.0)]);
            }
            Self::Component => {
                line(&[(2.0, 12.0), (6.0, 12.0)]);
                line(&[(18.0, 12.0), (22.0, 12.0)]);
                closed(&[(6.0, 7.0), (6.0, 17.0), (18.0, 12.0)]);
            }
            Self::Code => {
                line(&[(8.0, 5.0), (2.0, 12.0), (8.0, 19.0)]);
                line(&[(16.0, 5.0), (22.0, 12.0), (16.0, 19.0)]);
                line(&[(14.0, 3.0), (10.0, 21.0)]);
            }
            Self::Terminal => {
                line(&[(4.0, 6.0), (9.0, 11.0), (4.0, 16.0)]);
                line(&[(11.0, 17.0), (19.0, 17.0)]);
            }
            Self::Compare => {
                line(&[(8.0, 4.0), (8.0, 20.0)]);
                line(&[(16.0, 4.0), (16.0, 20.0)]);
                line(&[(4.0, 8.0), (8.0, 4.0), (12.0, 8.0)]);
                line(&[(12.0, 16.0), (16.0, 20.0), (20.0, 16.0)]);
            }
            Self::Info => {
                painter.circle_stroke(p(12.0, 12.0), 8.0 * scale, stroke);
                painter.circle_filled(p(12.0, 7.5), 1.0 * scale, color);
                line(&[(12.0, 11.0), (12.0, 17.0)]);
            }
            Self::Warning => {
                closed(&[(12.0, 3.0), (22.0, 20.0), (2.0, 20.0)]);
                line(&[(12.0, 8.0), (12.0, 14.0)]);
                painter.circle_filled(p(12.0, 17.0), 1.0 * scale, color);
            }
            Self::File => {
                closed(&[
                    (6.0, 3.0),
                    (15.0, 3.0),
                    (20.0, 8.0),
                    (20.0, 21.0),
                    (6.0, 21.0),
                ]);
                line(&[(15.0, 3.0), (15.0, 8.0), (20.0, 8.0)]);
            }
            Self::Target => {
                painter.circle_stroke(p(12.0, 12.0), 8.0 * scale, stroke);
                painter.circle_stroke(p(12.0, 12.0), 4.0 * scale, stroke);
                line(&[(12.0, 2.0), (12.0, 6.0)]);
                line(&[(12.0, 18.0), (12.0, 22.0)]);
                line(&[(2.0, 12.0), (6.0, 12.0)]);
                line(&[(18.0, 12.0), (22.0, 12.0)]);
            }
            Self::Export => {
                closed(&[(4.0, 10.0), (4.0, 20.0), (20.0, 20.0), (20.0, 10.0)]);
                line(&[(12.0, 16.0), (12.0, 3.0), (7.0, 8.0)]);
                line(&[(12.0, 3.0), (17.0, 8.0)]);
            }
        }
    }
}

fn line_colored(painter: &egui::Painter, points: &[Pos2], stroke: Stroke) {
    painter.add(Shape::line(points.to_vec(), stroke));
}

pub fn icon_button(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    size: Vec2,
) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), label)
    });
    if ui.is_rect_visible(rect) {
        let enabled = ui.is_enabled();
        let highlighted = enabled && (selected || response.hovered());
        let fill = if highlighted {
            t.color.bg_hover
        } else {
            Color32::TRANSPARENT
        };
        if highlighted {
            ui.painter().rect(
                rect,
                t.radius,
                fill,
                Stroke::new(1.0, t.color.border),
                egui::StrokeKind::Inside,
            );
        }
        if selected {
            ui.painter().rect_filled(
                Rect::from_min_max(
                    Pos2::new(rect.left() + 4.0, rect.bottom() - 4.0),
                    Pos2::new(rect.right() - 4.0, rect.bottom() - 3.0),
                ),
                0.0,
                t.color.accent,
            );
        }
        let icon_rect = Rect::from_center_size(rect.center(), Vec2::splat(16.0));
        icon.paint(
            ui.painter(),
            icon_rect,
            if !enabled {
                t.color.text_faint
            } else if highlighted {
                t.color.text
            } else {
                t.color.text_dim
            },
        );
        theme::paint_focus_ring_outset(ui, &response, rect);
    }
    response.on_hover_text(label)
}

pub fn labeled_icon_button(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    width: f32,
) -> Response {
    let height = Tokens::get(ui.ctx()).metrics.ctl_h.max(28.0);
    labeled_icon_button_sized(ui, icon, label, selected, width, height)
}

pub fn labeled_icon_button_sized(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    width: f32,
    height: f32,
) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), label)
    });
    if ui.is_rect_visible(rect) {
        let enabled = ui.is_enabled();
        let fill = if enabled && selected {
            t.color.accent_dim
        } else if enabled && response.hovered() {
            t.color.bg_hover
        } else {
            Color32::TRANSPARENT
        };
        ui.painter().rect_filled(rect, t.radius, fill);
        icon.paint(
            ui.painter(),
            Rect::from_center_size(
                Pos2::new(rect.left() + 15.0, rect.center().y),
                Vec2::splat(16.0),
            ),
            if !enabled {
                t.color.text_faint
            } else if selected {
                t.color.accent
            } else {
                t.color.text_dim
            },
        );
        let label_font = theme::sans(
            // `.tool-text-button` inherits the mockup's 13 px body type;
            // compactness comes from its 29 px box, not smaller copy.
            tokens::FS_2,
            if selected {
                FontWeight::SemiBold
            } else {
                FontWeight::Regular
            },
        );
        let text_rect = Rect::from_min_max(
            Pos2::new(rect.left() + 29.0, rect.top()),
            Pos2::new(rect.right() - 8.0, rect.bottom()),
        );
        let visible_label = elide_text(ui, label, &label_font, text_rect.width().max(0.0));
        ui.painter().with_clip_rect(text_rect).text(
            Pos2::new(rect.left() + 29.0, rect.center().y),
            Align2::LEFT_CENTER,
            visible_label,
            label_font,
            if !enabled {
                t.color.text_faint
            } else if selected {
                t.color.text
            } else {
                t.color.text_dim
            },
        );
        theme::paint_focus_ring_outset(ui, &response, rect);
    }
    response.on_hover_text(label)
}

pub fn section_header(ui: &mut Ui, title: &str, meta: Option<&str>) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), PANEL_SECTION_H),
        Sense::hover(),
    );
    ui.painter().rect_filled(
        rect,
        0.0,
        Color32::from_rgba_unmultiplied(
            t.color.bg_panel_2.r(),
            t.color.bg_panel_2.g(),
            t.color.bg_panel_2.b(),
            204,
        ),
    );
    ui.painter()
        .hline(rect.x_range(), rect.top(), Stroke::new(1.0, t.color.border));
    let title_font = theme::sans(tokens::FS_0, FontWeight::SemiBold);
    let meta_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let title_right = if meta.is_some() {
        rect.left() + rect.width() * 0.58
    } else {
        rect.right()
    };
    let title_rect = Rect::from_min_max(
        Pos2::new(rect.left() + 10.0, rect.top()),
        Pos2::new((title_right - 6.0).max(rect.left() + 10.0), rect.bottom()),
    );
    let title = elide_text(ui, &title.to_uppercase(), &title_font, title_rect.width());
    ui.painter().with_clip_rect(title_rect).text(
        title_rect.left_center(),
        Align2::LEFT_CENTER,
        title,
        title_font,
        t.color.text_dim,
    );
    if let Some(meta) = meta {
        let meta_rect = Rect::from_min_max(
            Pos2::new(title_right, rect.top()),
            Pos2::new(rect.right() - 10.0, rect.bottom()),
        );
        let meta = elide_text(ui, meta, &meta_font, meta_rect.width());
        ui.painter().with_clip_rect(meta_rect).text(
            meta_rect.right_center(),
            Align2::RIGHT_CENTER,
            meta,
            meta_font,
            t.color.text_faint,
        );
    }
}

pub fn property_row(ui: &mut Ui, label: &str, value: &str) -> Response {
    let t = Tokens::get(ui.ctx());
    property_row_with_tone(ui, label, value, t.color.text)
}

/// Property row whose value communicates an explicit semantic tone.
pub fn property_row_toned(ui: &mut Ui, label: &str, value: &str, value_tone: Color32) -> Response {
    property_row_with_tone(ui, label, value, value_tone)
}

fn property_row_with_tone(ui: &mut Ui, label: &str, value: &str, value_tone: Color32) -> Response {
    let t = Tokens::get(ui.ctx());
    let full_label = label;
    let full_value = value;
    let label_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let value_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let width = ui.available_width().max(1.0);
    let inner_width = (width - 20.0).max(1.0);
    let gap = 8.0_f32.min(inner_width);
    let columns_width = (inner_width - gap).max(1.0);
    let label_column = columns_width * 0.4;
    let value_column = (columns_width - label_column).max(1.0);
    let label_galley =
        ui.painter()
            .layout(label.to_owned(), label_font, t.color.text_dim, label_column);
    let value_galley = ui
        .painter()
        .layout(value.to_owned(), value_font, value_tone, value_column);
    let height = (label_galley.size().y.max(value_galley.size().y) + 12.0).max(29.0);
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());
    let label_rect = Rect::from_min_max(
        Pos2::new(rect.left() + 10.0, rect.top()),
        Pos2::new(rect.left() + 10.0 + label_column, rect.bottom()),
    );
    let value_rect = Rect::from_min_max(
        Pos2::new(label_rect.right() + gap, rect.top()),
        Pos2::new(rect.right() - 10.0, rect.bottom()),
    );
    ui.painter().with_clip_rect(label_rect).galley(
        Pos2::new(label_rect.left(), label_rect.top() + 6.0),
        label_galley,
        t.color.text_dim,
    );
    ui.painter().with_clip_rect(value_rect).galley(
        Pos2::new(value_rect.left(), value_rect.top() + 6.0),
        value_galley,
        value_tone,
    );
    response.on_hover_text(format!("{full_label}: {full_value}"))
}

fn elide_text(ui: &Ui, text: &str, font: &egui::FontId, max_width: f32) -> String {
    if max_width <= 0.0 {
        return String::new();
    }
    let fits = |candidate: &str| {
        ui.painter()
            .layout_no_wrap(candidate.to_owned(), font.clone(), Color32::WHITE)
            .size()
            .x
            <= max_width
    };
    if fits(text) {
        return text.to_owned();
    }
    let ellipsis = "…";
    if !fits(ellipsis) {
        return String::new();
    }
    let graphemes = UnicodeSegmentation::graphemes(text, true).collect::<Vec<_>>();
    let mut low = 0;
    let mut high = graphemes.len();
    while low < high {
        let middle = (low + high).div_ceil(2);
        let candidate = format!("{}{}", graphemes[..middle].concat(), ellipsis);
        if fits(&candidate) {
            low = middle;
        } else {
            high = middle - 1;
        }
    }
    format!("{}{}", graphemes[..low].concat(), ellipsis)
}

pub fn card(ui: &mut Ui, title: &str, body: impl FnOnce(&mut Ui)) {
    card_with_body_margin(ui, title, 11, body);
}

/// Card whose body is already a property list. [`property_row`] owns the
/// mockup's ten-pixel cell inset, so this variant deliberately adds no second
/// card-body inset.
pub fn property_card(ui: &mut Ui, title: &str, body: impl FnOnce(&mut Ui)) {
    card_with_body_margin(ui, title, 0, body);
}

fn card_with_body_margin(ui: &mut Ui, title: &str, body_margin: i8, body: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (head_rect, _) = ui.allocate_exact_size(Vec2::new(width, 37.0), Sense::hover());
    ui.painter().hline(
        head_rect.x_range(),
        head_rect.top(),
        Stroke::new(1.0, t.color.border_strong),
    );
    ui.painter().hline(
        head_rect.x_range(),
        head_rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let title_font = theme::sans(tokens::FS_0, FontWeight::SemiBold);
    let title_rect = Rect::from_min_max(
        Pos2::new(head_rect.left() + 11.0, head_rect.top()),
        Pos2::new(head_rect.right() - 11.0, head_rect.bottom()),
    );
    let title = elide_text(ui, title, &title_font, title_rect.width());
    ui.painter().with_clip_rect(title_rect).text(
        title_rect.left_center(),
        Align2::LEFT_CENTER,
        title,
        title_font,
        t.color.text,
    );
    egui::Frame::new()
        .inner_margin(egui::Margin::same(body_margin))
        .show(ui, |ui| {
            ui.set_width((width - f32::from(body_margin) * 2.0).max(1.0));
            ui.with_layout(egui::Layout::top_down(egui::Align::Min), body);
        });
}

pub fn status_dot(ui: &mut Ui, color: Color32, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal_top(|ui| {
        let (rect, _) = ui.allocate_exact_size(Vec2::splat(10.0), Sense::hover());
        ui.painter().circle_filled(rect.center(), 3.0, color);
        ui.add(
            egui::Label::new(
                egui::RichText::new(text)
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            )
            .wrap(),
        );
    });
}

pub fn divider(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 1.0), Sense::hover());
    ui.painter().hline(
        rect.x_range(),
        rect.center().y,
        Stroke::new(1.0, t.color.border),
    );
}

pub fn heading(ui: &mut Ui, eyebrow: &str, title: &str, description: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(eyebrow.to_uppercase())
            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
            .color(t.color.text_faint),
    );
    ui.add_space(2.0);
    ui.label(
        egui::RichText::new(title)
            .font(theme::sans(15.0, FontWeight::SemiBold))
            .color(t.color.text),
    );
    ui.label(
        egui::RichText::new(description)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
}

/// Exact compact heading used by the mockup's Code & Automation title rows.
///
/// This is deliberately separate from [`heading`]: code workspaces use the
/// mockup's three-point vertical rhythm and tracked mono eyebrow while other
/// workbench headings retain their existing composition.
pub fn code_workspace_heading(ui: &mut Ui, eyebrow: &str, title: &str, description: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(eyebrow.to_uppercase())
            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
            .extra_letter_spacing(0.09 * tokens::FS_0)
            .color(t.color.text_faint),
    );
    ui.add_space(3.0);
    ui.label(
        egui::RichText::new(title)
            .font(theme::sans(15.0, FontWeight::SemiBold))
            .color(t.color.text),
    );
    ui.add_space(3.0);
    ui.add(
        egui::Label::new(
            egui::RichText::new(description)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        )
        .wrap(),
    );
}

/// One canonical section of the embedded Code-workspace inspector.
///
/// The section owns its 29 px header and bottom divider. `status` uses the
/// mockup's row-status grammar: a five-point dot, six-point gap, and mono text.
pub fn code_inspector_section(
    ui: &mut Ui,
    title: &str,
    status: Option<(&str, Color32)>,
    body: impl FnOnce(&mut Ui),
) {
    let t = Tokens::get(ui.ctx());
    let shown = ui.scope(|ui| {
        ui.spacing_mut().item_spacing = Vec2::ZERO;
        let (rect, _) = ui.allocate_exact_size(
            Vec2::new(ui.available_width(), PANEL_SECTION_H),
            Sense::hover(),
        );
        ui.painter().rect_filled(
            rect,
            0.0,
            Color32::from_rgba_unmultiplied(
                t.color.bg_panel_2.r(),
                t.color.bg_panel_2.g(),
                t.color.bg_panel_2.b(),
                204,
            ),
        );

        let status_left = if let Some((label, tone)) = status {
            let galley = ui.painter().layout_no_wrap(
                label.to_owned(),
                theme::mono(tokens::FS_0, FontWeight::Medium),
                tone,
            );
            let label_pos = Pos2::new(
                rect.right() - 10.0 - galley.size().x,
                rect.center().y - galley.size().y * 0.5,
            );
            ui.painter().galley(label_pos, galley, tone);
            let dot_center = Pos2::new(label_pos.x - 8.5, rect.center().y);
            ui.painter().circle_filled(dot_center, 2.5, tone);
            dot_center.x - 2.5
        } else {
            rect.right()
        };

        let title_font = theme::sans(tokens::FS_0, FontWeight::SemiBold);
        let title_job = egui::text::LayoutJob::single_section(
            title.to_uppercase(),
            egui::TextFormat {
                font_id: title_font,
                color: t.color.text_dim,
                extra_letter_spacing: 0.055 * tokens::FS_0,
                ..Default::default()
            },
        );
        let title_galley = ui.fonts_mut(|fonts| fonts.layout_job(title_job));
        ui.painter()
            .with_clip_rect(Rect::from_min_max(
                Pos2::new(rect.left() + 10.0, rect.top()),
                Pos2::new((status_left - 6.0).max(rect.left() + 10.0), rect.bottom()),
            ))
            .galley(
                Pos2::new(
                    rect.left() + 10.0,
                    rect.center().y - title_galley.size().y * 0.5,
                ),
                title_galley,
                t.color.text_dim,
            );
        body(ui);
    });
    ui.painter().hline(
        shown.response.rect.x_range(),
        shown.response.rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
}

/// Canonical seven/ten-point vertical inset around inspector property rows.
pub fn code_inspector_property_list(ui: &mut Ui, body: impl FnOnce(&mut Ui)) {
    ui.add_space(7.0);
    body(ui);
    ui.add_space(10.0);
}

/// Full-width title row used by non-project workspaces in the reference shell.
pub fn workspace_title_row(ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    let shown = egui::Frame::new()
        .inner_margin(egui::Margin::same(8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width().max(1.0));
            content(ui);
        });
    ui.painter().hline(
        shown.response.rect.x_range(),
        shown.response.rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
}

pub fn empty_state(ui: &mut Ui, icon: WorkbenchIcon, title: &str, description: &str) {
    let t = Tokens::get(ui.ctx());
    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.add_space((ui.available_height() * 0.18).min(100.0));
        let (rect, _) = ui.allocate_exact_size(Vec2::splat(48.0), Sense::hover());
        icon.paint(ui.painter(), rect.shrink(8.0), t.color.text_faint);
        ui.add_space(8.0);
        ui.label(
            egui::RichText::new(title)
                .font(theme::sans(tokens::FS_4, FontWeight::SemiBold))
                .color(t.color.text),
        );
        ui.label(
            egui::RichText::new(description)
                .font(theme::sans(tokens::FS_2, FontWeight::Regular))
                .color(t.color.text_dim),
        );
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_content_remains_vertical_inside_a_horizontal_parent() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut first = egui::Rect::NOTHING;
        let mut second = egui::Rect::NOTHING;

        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.horizontal(|ui| {
                    card(ui, "Status", |ui| {
                        first = ui.label("first").rect;
                        second = ui.label("second").rect;
                    });
                });
            });
        });

        assert!(second.top() >= first.bottom());
    }
}
