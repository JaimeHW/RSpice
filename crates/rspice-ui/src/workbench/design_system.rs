//! Workbench-specific visual primitives.
//!
//! These controls are intentionally small and stateless.  They render from
//! the shared palette and typography assets, but none of the retired layout's
//! layout or widget implementations are reused.

use egui::{Align2, Color32, Pos2, Rect, Response, Sense, Shape, Stroke, Ui, Vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

pub const TITLE_BAR_H: f32 = 35.0;
pub const TOOL_BAR_H: f32 = 45.0;
pub const DOCUMENT_BAR_H: f32 = 34.0;
pub const STATUS_BAR_H: f32 = 25.0;
pub const ACTIVITY_RAIL_W: f32 = 51.0;
pub const PHONE_NAV_H: f32 = 54.0;
pub const TOUCH_TARGET: f32 = 44.0;

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
        theme::paint_focus_ring(ui, &response, rect);
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
        ui.painter().text(
            Pos2::new(rect.left() + 29.0, rect.center().y),
            Align2::LEFT_CENTER,
            label,
            theme::sans(
                // `.tool-text-button` inherits the mockup's 13 px body type;
                // compactness comes from its 29 px box, not smaller copy.
                tokens::FS_2,
                if selected {
                    FontWeight::SemiBold
                } else {
                    FontWeight::Regular
                },
            ),
            if !enabled {
                t.color.text_faint
            } else if selected {
                t.color.text
            } else {
                t.color.text_dim
            },
        );
        theme::paint_focus_ring(ui, &response, rect);
    }
    response
}

pub fn section_header(ui: &mut Ui, title: &str, meta: Option<&str>) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 28.0), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    ui.painter().text(
        Pos2::new(rect.left() + 12.0, rect.center().y),
        Align2::LEFT_CENTER,
        title.to_uppercase(),
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text_dim,
    );
    if let Some(meta) = meta {
        ui.painter().text(
            Pos2::new(rect.right() - 12.0, rect.center().y),
            Align2::RIGHT_CENTER,
            meta,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        );
    }
}

pub fn property_row(ui: &mut Ui, label: &str, value: &str) -> Response {
    let t = Tokens::get(ui.ctx());
    let label_font = theme::sans(tokens::FS_1, FontWeight::Regular);
    let value_font = theme::mono(tokens::FS_1, FontWeight::Medium);
    let label_width = ui
        .painter()
        .layout_no_wrap(label.to_owned(), label_font.clone(), t.color.text_dim)
        .size()
        .x;
    let value_width = ui
        .painter()
        .layout_no_wrap(value.to_owned(), value_font.clone(), t.color.text)
        .size()
        .x;
    let stacked = label_width + value_width + 48.0 > ui.available_width();
    let height = if stacked { 42.0 } else { 25.0 };
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), height), Sense::hover());
    if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    ui.painter().text(
        Pos2::new(
            rect.left() + 12.0,
            if stacked {
                rect.top() + 11.0
            } else {
                rect.center().y
            },
        ),
        Align2::LEFT_CENTER,
        label,
        label_font,
        t.color.text_dim,
    );
    ui.painter().text(
        Pos2::new(
            if stacked {
                rect.left() + 12.0
            } else {
                rect.right() - 12.0
            },
            if stacked {
                rect.bottom() - 11.0
            } else {
                rect.center().y
            },
        ),
        if stacked {
            Align2::LEFT_CENTER
        } else {
            Align2::RIGHT_CENTER
        },
        value,
        value_font,
        t.color.text,
    );
    response
}

pub fn card(ui: &mut Ui, title: &str, body: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(t.radius_lg)
        .inner_margin(egui::Margin::same(14))
        .show(ui, |ui| {
            // A card is always a top-down content surface, even when its
            // parent lays several cards out in a horizontal/wrapping row.
            // Inheriting the parent's layout causes headings, values, and
            // controls to paint over one another at desktop and tablet widths.
            ui.vertical(|ui| {
                ui.label(
                    egui::RichText::new(title)
                        .font(theme::sans(tokens::FS_3, FontWeight::SemiBold))
                        .color(t.color.text),
                );
                ui.add_space(8.0);
                body(ui);
            });
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
            .color(t.color.accent),
    );
    ui.add_space(2.0);
    ui.label(
        egui::RichText::new(title)
            .font(theme::sans(22.0, FontWeight::SemiBold))
            .color(t.color.text),
    );
    ui.label(
        egui::RichText::new(description)
            .font(theme::sans(tokens::FS_2, FontWeight::Regular))
            .color(t.color.text_dim),
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
