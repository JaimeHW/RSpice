//! Workbench-specific visual primitives.
//!
//! These controls are intentionally small and stateless.  They render from
//! the shared palette and typography assets, but none of the retired layout's
//! layout or widget implementations are reused.

use egui::{Align2, Color32, Pos2, Rect, Response, Sense, Shape, Stroke, Ui, Vec2};
use std::hash::Hash;
use unicode_segmentation::UnicodeSegmentation;

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

// The chrome bars size themselves from their own content and tokens, so the
// fixed heights that used to sit here (title 35, toolbar 45, document bar 34,
// status 25, phone nav 54, touch target 44) had no reader — a second, silently
// diverging copy of numbers `workbench::chrome` already owns.
pub const ACTIVITY_RAIL_W: f32 = 51.0;
pub const PANEL_HEADER_H: f32 = 39.0;
/// Desktop panel tabs use the upgraded mockup's compact 25 px track.
/// Coarse-pointer layouts still raise this to the shared touch target.
pub const PANEL_TABS_H: f32 = 25.0;
/// Section heads keep a 24 px single-line rhythm and grow only when the
/// title/metadata pair genuinely needs a second line.
pub const PANEL_SECTION_H: f32 = 24.0;

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
    Instance,
    Wire,
    Bus,
    BusTap,
    Junction,
    Label,
    Pin,
    Text,
    Probe,
    Rotate,
    Mirror,
    MirrorVertical,
    Undo,
    Redo,
    ZoomOut,
    ZoomIn,
    ZoomFit,
    Grid,
    Visibility,
    Check,
    Focus,
    Run,
    Stop,
    ChevronDown,
    ChevronRight,
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
    ArrowRight,
    ArrowUp,
    Supply,
    Layers,
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
    Star,
    StarFilled,
}

/// Compact semantic status marks painted as vector geometry.
///
/// These deliberately replace Unicode check/triangle characters in dense
/// engineering rows. The bundled text faces are not the authority for icon
/// coverage, and a missing fallback must never turn a status into a tofu box.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusMark {
    Success,
    Warning,
    Failure,
    Neutral,
}

pub fn paint_status_mark(painter: &egui::Painter, rect: Rect, mark: StatusMark, color: Color32) {
    let side = rect.width().min(rect.height()).max(1.0);
    let center = rect.center();
    let half = side * 0.42;
    let stroke = Stroke::new((side * 0.11).max(1.0), color);
    match mark {
        StatusMark::Success => {
            painter.add(Shape::line(
                vec![
                    Pos2::new(center.x - half, center.y),
                    Pos2::new(center.x - half * 0.22, center.y + half * 0.72),
                    Pos2::new(center.x + half, center.y - half * 0.78),
                ],
                stroke,
            ));
        }
        StatusMark::Warning => {
            painter.add(Shape::closed_line(
                vec![
                    Pos2::new(center.x, center.y - half),
                    Pos2::new(center.x + half, center.y + half * 0.82),
                    Pos2::new(center.x - half, center.y + half * 0.82),
                ],
                stroke,
            ));
        }
        StatusMark::Failure => {
            painter.line_segment(
                [
                    Pos2::new(center.x - half, center.y - half),
                    Pos2::new(center.x + half, center.y + half),
                ],
                stroke,
            );
            painter.line_segment(
                [
                    Pos2::new(center.x + half, center.y - half),
                    Pos2::new(center.x - half, center.y + half),
                ],
                stroke,
            );
        }
        StatusMark::Neutral => {
            painter.circle_filled(center, (side * 0.18).max(1.25), color);
        }
    }
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
            Self::Instance => {
                closed(&[(7.0, 6.0), (17.0, 6.0), (17.0, 18.0), (7.0, 18.0)]);
                line(&[(3.0, 9.0), (7.0, 9.0)]);
                line(&[(3.0, 15.0), (7.0, 15.0)]);
                line(&[(17.0, 9.0), (21.0, 9.0)]);
                line(&[(17.0, 15.0), (21.0, 15.0)]);
            }
            Self::Wire => {
                line(&[(4.0, 18.0), (11.0, 18.0), (11.0, 6.0), (20.0, 6.0)]);
                painter.circle_filled(p(4.0, 18.0), 1.5 * scale, color);
                painter.circle_filled(p(20.0, 6.0), 1.5 * scale, color);
            }
            Self::Bus => {
                line(&[(3.0, 17.0), (17.0, 3.0)]);
                line(&[(7.0, 21.0), (21.0, 7.0)]);
                line(&[(3.0, 9.0), (15.0, 21.0)]);
            }
            Self::BusTap => {
                line(&[(4.0, 4.0), (4.0, 20.0)]);
                line(&[(8.0, 4.0), (8.0, 20.0)]);
                line(&[(8.0, 12.0), (13.0, 12.0), (19.0, 7.0)]);
                painter.circle_stroke(p(19.0, 7.0), 1.8 * scale, stroke);
            }
            Self::Junction => {
                line(&[(3.0, 12.0), (21.0, 12.0)]);
                line(&[(12.0, 3.0), (12.0, 21.0)]);
                painter.circle_filled(p(12.0, 12.0), 2.6 * scale, color);
            }
            Self::Label => {
                line(&[(5.0, 5.0), (19.0, 5.0)]);
                line(&[(12.0, 5.0), (12.0, 20.0)]);
            }
            Self::Pin => {
                closed(&[(9.0, 4.0), (15.0, 4.0), (15.0, 10.0), (9.0, 10.0)]);
                line(&[(12.0, 10.0), (12.0, 20.0)]);
                line(&[(8.0, 20.0), (16.0, 20.0)]);
            }
            Self::Text => {
                line(&[(5.0, 5.0), (19.0, 5.0)]);
                line(&[(12.0, 5.0), (12.0, 20.0)]);
                line(&[(9.0, 20.0), (15.0, 20.0)]);
            }
            Self::Probe => {
                painter.circle_stroke(p(10.0, 10.0), 5.5 * scale, stroke);
                line(&[(14.5, 14.5), (20.0, 20.0)]);
            }
            // Selection transforms share their art with the design-system
            // icon set so the toolbar, the inspector action stack, and the
            // context menu can never drift apart.
            Self::Rotate => crate::ui::icons::Icon::Rotate.paint(painter, rect, color),
            Self::Mirror => crate::ui::icons::Icon::Mirror.paint(painter, rect, color),
            Self::MirrorVertical => {
                line(&[(3.0, 12.0), (21.0, 12.0)]);
                closed(&[(8.0, 9.0), (12.0, 4.0), (16.0, 9.0)]);
                closed(&[(8.0, 15.0), (16.0, 15.0), (12.0, 20.0)]);
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
            Self::Visibility => {
                let mut outline = (0..=12)
                    .map(|step| {
                        let t = step as f32 / 12.0;
                        p(
                            2.0 + 20.0 * t,
                            12.0 - 6.5 * (std::f32::consts::PI * t).sin(),
                        )
                    })
                    .collect::<Vec<_>>();
                outline.extend((0..=12).rev().map(|step| {
                    let t = step as f32 / 12.0;
                    p(
                        2.0 + 20.0 * t,
                        12.0 + 6.5 * (std::f32::consts::PI * t).sin(),
                    )
                }));
                outline.push(outline[0]);
                painter.add(Shape::line(outline, stroke));
                painter.circle_stroke(p(12.0, 12.0), 2.8 * scale, stroke);
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
            Self::ChevronRight => line(&[(9.0, 6.0), (15.0, 12.0), (9.0, 18.0)]),
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
            Self::ArrowRight => {
                line(&[(9.0, 5.0), (16.0, 12.0), (9.0, 19.0)]);
                line(&[(3.0, 12.0), (16.0, 12.0)]);
            }
            Self::ArrowUp => {
                line(&[(5.0, 11.0), (12.0, 4.0), (19.0, 11.0)]);
                line(&[(12.0, 4.0), (12.0, 20.0)]);
            }
            Self::Supply => {
                line(&[(4.0, 8.0), (20.0, 8.0)]);
                line(&[(6.5, 12.0), (17.5, 12.0)]);
                line(&[(9.0, 16.0), (15.0, 16.0)]);
            }
            // Descending into an instance opens the sheet beneath it.
            Self::Layers => {
                closed(&[(12.0, 3.0), (21.0, 8.0), (12.0, 13.0), (3.0, 8.0)]);
                line(&[(3.0, 13.0), (12.0, 18.0), (21.0, 13.0)]);
                line(&[(3.0, 17.0), (12.0, 22.0), (21.0, 17.0)]);
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
            // A five-pointed star on the 24-unit grid, stroked hollow and
            // filled when set. The bundled faces carry neither U+2605 nor
            // U+2606, so a starred row used to paint a missing-glyph box.
            Self::Star | Self::StarFilled => {
                let points = STAR_POINTS.map(|(x, y)| p(x, y)).to_vec();
                if matches!(self, Self::StarFilled) {
                    painter.add(Shape::convex_polygon(points, color, Stroke::NONE));
                } else {
                    closed(&STAR_POINTS);
                }
            }
        }
    }
}

/// Outer radius 9.5, inner 4.0, first point straight up on the 24-unit grid.
const STAR_POINTS: [(f32, f32); 10] = [
    (12.0, 2.5),
    (13.8, 9.2),
    (20.8, 9.2),
    (15.2, 13.4),
    (17.2, 20.1),
    (12.0, 15.9),
    (6.8, 20.1),
    (8.8, 13.4),
    (3.2, 9.2),
    (10.2, 9.2),
];

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
            // `.tool-text-button` inherits the mockup's 12 px body type;
            // compactness comes from its 29 px box, not smaller copy.
            tokens::FS_1,
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
    section_header_with_typography(ui, title, meta, tokens::FS_0, 0.0);
}

/// Schematic-dock section heading from the upgraded design mockup.
///
/// This stays separate from [`section_header`] so the schematic's 13 px,
/// tracked EDA headings do not silently enlarge unrelated workspaces.
pub fn schematic_section_header(ui: &mut Ui, title: &str, meta: Option<&str>) {
    section_header_with_typography(
        ui,
        title,
        meta,
        tokens::FS_2,
        SCHEMATIC_SECTION_TITLE_TRACKING,
    );
}

const SCHEMATIC_SECTION_TITLE_TRACKING: f32 = 0.055 * tokens::FS_2;

fn tracked_galley(
    ui: &Ui,
    text: String,
    font_id: egui::FontId,
    color: Color32,
    extra_letter_spacing: f32,
) -> std::sync::Arc<egui::Galley> {
    let job = egui::text::LayoutJob::single_section(
        text,
        egui::TextFormat {
            font_id,
            color,
            extra_letter_spacing,
            ..Default::default()
        },
    );
    ui.fonts_mut(|fonts| fonts.layout_job(job))
}

fn tracked_wrapped_galley(
    ui: &Ui,
    text: String,
    font_id: egui::FontId,
    color: Color32,
    extra_letter_spacing: f32,
    max_width: f32,
) -> std::sync::Arc<egui::Galley> {
    let mut job = egui::text::LayoutJob::single_section(
        text,
        egui::TextFormat {
            font_id,
            color,
            extra_letter_spacing,
            ..Default::default()
        },
    );
    job.wrap.max_width = max_width.max(1.0);
    job.wrap.max_rows = 2;
    job.wrap.break_anywhere = true;
    ui.fonts_mut(|fonts| fonts.layout_job(job))
}

fn section_header_full_text(title: &str, meta: Option<&str>) -> String {
    meta.map_or_else(|| title.to_owned(), |meta| format!("{title}, {meta}"))
}

fn section_header_with_typography(
    ui: &mut Ui,
    title: &str,
    meta: Option<&str>,
    title_size: f32,
    title_tracking: f32,
) {
    let t = Tokens::get(ui.ctx());
    let title_font = theme::sans(title_size, FontWeight::SemiBold);
    let meta_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let title = title.to_uppercase();
    let content_width = (ui.available_width() - 20.0).max(0.0);
    let title_width = tracked_galley(
        ui,
        title.clone(),
        title_font.clone(),
        t.color.text_dim,
        title_tracking,
    )
    .size()
    .x;
    let meta_width = meta.map_or(0.0, |meta| {
        ui.painter()
            .layout_no_wrap(meta.to_owned(), meta_font.clone(), t.color.text_faint)
            .size()
            .x
    });
    let wraps = section_header_wraps(content_width, title_width, meta_width, meta.is_some());
    let height = if wraps {
        PANEL_SECTION_WRAPPED_H
    } else {
        PANEL_SECTION_H
    };
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), height), Sense::hover());
    let accessible_label = section_header_full_text(&title, meta);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            accessible_label.clone(),
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Heading);
        node.set_label(accessible_label.clone());
        node.set_level(3);
    });
    let _ = response.on_hover_text(accessible_label);
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
    let content_left = rect.left() + 10.0;
    let content_right = rect.right() - 10.0;
    let content_width = (content_right - content_left).max(0.0);
    if wraps {
        if meta.is_none() {
            let title_rect = Rect::from_min_max(
                Pos2::new(content_left, rect.top()),
                Pos2::new(content_right, rect.bottom()),
            );
            let title_galley = tracked_wrapped_galley(
                ui,
                title,
                title_font,
                t.color.text_dim,
                title_tracking,
                title_rect.width(),
            );
            ui.painter().with_clip_rect(title_rect).galley(
                Pos2::new(
                    title_rect.left(),
                    title_rect.center().y - title_galley.size().y * 0.5,
                ),
                title_galley,
                t.color.text_dim,
            );
            return;
        }
        let title_rect = Rect::from_min_max(
            Pos2::new(content_left, rect.top()),
            Pos2::new(content_right, rect.top() + PANEL_SECTION_H),
        );
        let title = elide_text(ui, &title, &title_font, title_rect.width());
        let title_galley = tracked_galley(ui, title, title_font, t.color.text_dim, title_tracking);
        ui.painter().with_clip_rect(title_rect).galley(
            Pos2::new(
                title_rect.left(),
                title_rect.center().y - title_galley.size().y * 0.5,
            ),
            title_galley,
            t.color.text_dim,
        );
        if let Some(meta) = meta {
            let meta_rect = Rect::from_min_max(
                Pos2::new(content_left, rect.top() + PANEL_SECTION_H - 1.0),
                Pos2::new(content_right, rect.bottom()),
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
        return;
    }
    let (title_region_width, meta_region_width) =
        section_header_column_widths(content_width, title_width, meta_width, meta.is_some());
    let title_right = content_left + title_region_width;
    let title_rect = Rect::from_min_max(
        Pos2::new(content_left, rect.top()),
        Pos2::new(title_right.max(content_left), rect.bottom()),
    );
    let title = elide_text(ui, &title, &title_font, title_rect.width());
    let title_galley = tracked_galley(ui, title, title_font, t.color.text_dim, title_tracking);
    ui.painter().with_clip_rect(title_rect).galley(
        Pos2::new(
            title_rect.left(),
            title_rect.center().y - title_galley.size().y * 0.5,
        ),
        title_galley,
        t.color.text_dim,
    );
    if let Some(meta) = meta {
        let meta_rect = Rect::from_min_max(
            Pos2::new(content_right - meta_region_width, rect.top()),
            Pos2::new(content_right, rect.bottom()),
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

const SECTION_HEADER_COLUMN_GAP: f32 = 8.0;
const PANEL_SECTION_WRAPPED_H: f32 = 42.0;

fn section_header_wraps(
    available_width: f32,
    desired_title_width: f32,
    desired_meta_width: f32,
    has_meta: bool,
) -> bool {
    let available_width = available_width.max(0.0);
    desired_title_width > available_width
        || (has_meta
            && desired_title_width + SECTION_HEADER_COLUMN_GAP + desired_meta_width
                > available_width)
}

/// Allocate a section header from the measured copy instead of a fixed ratio.
///
/// Short title/metadata pairs remain complete at the mockup's 228 px navigator
/// width. Only genuinely over-constrained pairs are elided, with the title
/// retaining the slightly larger share because it identifies the section.
fn section_header_column_widths(
    available_width: f32,
    desired_title_width: f32,
    desired_meta_width: f32,
    has_meta: bool,
) -> (f32, f32) {
    let available_width = available_width.max(0.0);
    if !has_meta {
        return (available_width, 0.0);
    }

    let columns_width = (available_width - SECTION_HEADER_COLUMN_GAP).max(0.0);
    if desired_title_width + desired_meta_width <= columns_width {
        return (
            (columns_width - desired_meta_width).max(0.0),
            desired_meta_width.max(0.0),
        );
    }

    let meta_region_width = desired_meta_width.max(0.0).min(columns_width * 0.45);
    (
        (columns_width - meta_region_width).max(0.0),
        meta_region_width,
    )
}

pub fn property_row(ui: &mut Ui, label: &str, value: &str) -> Response {
    let t = Tokens::get(ui.ctx());
    property_row_with_tone(ui, label, value, t.color.text, None, tokens::FS_0)
}

/// Read-only schematic property row. The upgraded inspector uses body-sized
/// mono values while retaining caption-sized labels.
pub fn schematic_property_row(ui: &mut Ui, label: &str, value: &str) -> Response {
    let t = Tokens::get(ui.ctx());
    property_row_with_tone(ui, label, value, t.color.text, None, tokens::FS_1)
}

/// Property row whose value communicates an explicit semantic tone.
pub fn property_row_toned(ui: &mut Ui, label: &str, value: &str, value_tone: Color32) -> Response {
    property_row_with_tone(ui, label, value, value_tone, None, tokens::FS_0)
}

/// Property row with a font-independent semantic mark before its value.
pub fn property_row_status(
    ui: &mut Ui,
    label: &str,
    value: &str,
    value_tone: Color32,
    mark: StatusMark,
) -> Response {
    property_row_with_tone(ui, label, value, value_tone, Some(mark), tokens::FS_0)
}

/// Status-bearing counterpart to [`schematic_property_row`].
pub fn schematic_property_row_status(
    ui: &mut Ui,
    label: &str,
    value: &str,
    value_tone: Color32,
    mark: StatusMark,
) -> Response {
    property_row_with_tone(ui, label, value, value_tone, Some(mark), tokens::FS_1)
}

/// Horizontal padding inside a property row.
const PROPERTY_ROW_PAD: f32 = 10.0;
/// Gap between a property row's label and value columns.
const PROPERTY_ROW_GAP: f32 = 8.0;
/// Share of the column space the label takes while the row is narrow enough
/// for the share to stay readable.
const PROPERTY_LABEL_FRACTION: f32 = 0.4;
/// Widest the label column is ever drawn.
///
/// The proportional share alone tracks the row forever, so a wide row pushes
/// every value roughly 40% across and opens a gap that reads as unfinished
/// layout — most visible when a two-column dialog collapses to one column at
/// its breakpoint and an aside's property list inherits the full dialog width.
/// The cap clears the widest label the workbench ships with room to spare, so
/// no label that fits today begins to elide; below roughly 470 px of row it
/// never binds, leaving inspector docks and narrow asides untouched.
const PROPERTY_LABEL_MAX_W: f32 = 176.0;
/// Shortest a property row is ever drawn.
const PROPERTY_ROW_MIN_H: f32 = 29.0;

/// Column widths shared by every property row so a read-only row and an
/// editable row in the same list line up to the pixel.
fn property_row_columns(width: f32) -> (f32, f32, f32) {
    let inner_width = (width - 2.0 * PROPERTY_ROW_PAD).max(1.0);
    let gap = PROPERTY_ROW_GAP.min(inner_width);
    let columns_width = (inner_width - gap).max(1.0);
    let label_column = (columns_width * PROPERTY_LABEL_FRACTION).min(PROPERTY_LABEL_MAX_W);
    let value_column = (columns_width - label_column).max(1.0);
    (label_column, gap, value_column)
}

/// The horizontal geometry a control row needs to sit inside a property list:
/// where the label starts, and where the value column it has to line up with
/// begins. Both measured from the row's left edge.
///
/// A row that carries a live control rather than a painted value cannot use
/// [`property_row`], but it is read as part of the same block, and a block with
/// two label columns reads as two blocks. The only honest source for that
/// offset is the arithmetic the painted rows already use.
pub fn property_row_control_columns(width: f32) -> (f32, f32) {
    let (label_column, gap, _) = property_row_columns(width);
    (PROPERTY_ROW_PAD, PROPERTY_ROW_PAD + label_column + gap)
}

/// The trailing inset a control row leaves so its right edge meets the value
/// column's.
pub const PROPERTY_ROW_TRAILING_PAD: f32 = PROPERTY_ROW_PAD;

/// Vertical inset above and below a wrapped property row's text.
const PROPERTY_ROW_WRAP_PAD: f32 = 6.0;

/// Property row whose right-hand column is a sentence or a list rather than a
/// value.
///
/// The single-line row elides, which is right for a value: a clipped number is
/// obviously clipped, and the whole one is a click away. It is wrong for prose.
/// "keep each member's own contra…" is a rule stated up to its first clause,
/// and nothing else on the surface finishes it. This row keeps the same label
/// column and lets the value column take the lines it needs. Sans rather than
/// the mono a value gets — the distinction is the point: a mono cell holds a
/// value, this one holds a sentence.
pub fn property_row_wrapped(ui: &mut Ui, label: &str, text: &str) -> Response {
    let t = Tokens::get(ui.ctx());
    let label_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let value_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let width = ui.available_width().max(1.0);
    let (label_column, gap, value_column) = property_row_columns(width);
    let label_galley = ui.painter().layout_no_wrap(
        elide_text(ui, label, &label_font, label_column),
        label_font,
        t.color.text_dim,
    );
    let value_galley = ui
        .painter()
        .layout(text.to_owned(), value_font, t.color.text, value_column);
    let height = (value_galley.size().y + 2.0 * PROPERTY_ROW_WRAP_PAD)
        .max(t.metrics.ctl_h.max(PROPERTY_ROW_MIN_H));
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());
    ui.painter().galley(
        Pos2::new(
            rect.left() + PROPERTY_ROW_PAD,
            rect.top() + PROPERTY_ROW_WRAP_PAD,
        ),
        label_galley,
        t.color.text_dim,
    );
    ui.painter().galley(
        Pos2::new(
            rect.left() + PROPERTY_ROW_PAD + label_column + gap,
            rect.top() + PROPERTY_ROW_WRAP_PAD,
        ),
        value_galley,
        t.color.text,
    );
    response
}

/// Editable twin of [`property_row`]: the same label column and value
/// column, with a compact mono input in place of the painted value.
///
/// `invalid` outlines the input in the error tone — the typed text has not
/// been applied to the design.
pub fn property_row_input(ui: &mut Ui, label: &str, value: &mut String, invalid: bool) -> Response {
    property_row_input_with_hint(ui, label, value, "", invalid)
}

/// Editable property row that keeps inherited/default copy inside the input
/// without making that presentation text part of the authoritative value.
///
/// This is important for override fields: focusing an inherited value must
/// not itself materialize an override, while the first typed character must.
pub fn property_row_input_with_hint(
    ui: &mut Ui,
    label: &str,
    value: &mut String,
    hint: &str,
    invalid: bool,
) -> Response {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (label_column, gap, value_column) = property_row_columns(width);
    let height = t.metrics.ctl_h.max(PROPERTY_ROW_MIN_H);
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());
    let label_rect = Rect::from_min_max(
        Pos2::new(rect.left() + PROPERTY_ROW_PAD, rect.top()),
        Pos2::new(rect.left() + PROPERTY_ROW_PAD + label_column, rect.bottom()),
    );
    ui.painter().with_clip_rect(label_rect).text(
        Pos2::new(label_rect.left(), label_rect.center().y),
        Align2::LEFT_CENTER,
        label,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    let value_rect = Rect::from_min_max(
        Pos2::new(
            label_rect.right() + gap,
            rect.center().y - t.metrics.ctl_h * 0.5,
        ),
        Pos2::new(
            rect.right() - PROPERTY_ROW_PAD,
            rect.center().y + t.metrics.ctl_h * 0.5,
        ),
    );
    let mut edit = egui::TextEdit::singleline(value)
        .font(egui::TextStyle::Monospace)
        .margin(egui::Margin::symmetric(8, 4))
        .desired_width(value_column);
    if !hint.is_empty() {
        edit = edit.hint_text(hint);
    }
    if invalid {
        edit = edit.text_color(t.color.err);
    }
    let response = ui.put(value_rect, edit);
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(label);
        node.set_description(if invalid {
            "Editable engineering value with a validation error"
        } else {
            "Editable engineering value"
        });
        if invalid {
            node.set_invalid(egui::accesskit::Invalid::True);
        } else {
            node.clear_invalid();
        }
    });
    if invalid {
        ui.painter().rect_stroke(
            value_rect,
            t.radius,
            egui::Stroke::new(1.0, t.color.err),
            egui::StrokeKind::Inside,
        );
    }
    response
}

/// Editable property row with one compact trailing action, used by instance
/// values that can open a non-destructive engineering workflow such as the
/// parameter-tuning sandbox. The action owns fixed width so focusing or
/// editing the text field never shifts adjacent rows.
pub fn property_row_input_action(
    ui: &mut Ui,
    label: &str,
    value: &mut String,
    invalid: bool,
    action_icon: WorkbenchIcon,
    action_label: &str,
    action_enabled: bool,
    action_disabled_reason: Option<&str>,
) -> (Response, Response) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (label_column, gap, value_column) = property_row_columns(width);
    let height = t.metrics.ctl_h.max(PROPERTY_ROW_MIN_H);
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());
    let label_rect = Rect::from_min_max(
        Pos2::new(rect.left() + PROPERTY_ROW_PAD, rect.top()),
        Pos2::new(rect.left() + PROPERTY_ROW_PAD + label_column, rect.bottom()),
    );
    ui.painter().with_clip_rect(label_rect).text(
        Pos2::new(label_rect.left(), label_rect.center().y),
        Align2::LEFT_CENTER,
        label,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );

    let action_size = t.metrics.ctl_h;
    let action_gap = 4.0;
    let action_rect = Rect::from_min_max(
        Pos2::new(
            rect.right() - PROPERTY_ROW_PAD - action_size,
            rect.center().y - action_size * 0.5,
        ),
        Pos2::new(
            rect.right() - PROPERTY_ROW_PAD,
            rect.center().y + action_size * 0.5,
        ),
    );
    let value_rect = Rect::from_min_max(
        Pos2::new(
            label_rect.right() + gap,
            rect.center().y - t.metrics.ctl_h * 0.5,
        ),
        Pos2::new(
            action_rect.left() - action_gap,
            rect.center().y + t.metrics.ctl_h * 0.5,
        ),
    );
    let mut edit = egui::TextEdit::singleline(value)
        .font(egui::TextStyle::Monospace)
        .desired_width((value_column - action_size - action_gap).max(1.0))
        .margin(egui::Margin::symmetric(7, 3));
    if invalid {
        edit = edit.text_color(t.color.err);
    }
    let edit_response = ui
        .scope_builder(
            egui::UiBuilder::new()
                .max_rect(value_rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
            |ui| ui.add(edit),
        )
        .inner;
    ui.ctx().accesskit_node_builder(edit_response.id, |node| {
        node.set_label(label);
        node.set_description(if invalid {
            "Editable engineering value with a validation error"
        } else {
            "Editable engineering value"
        });
        if invalid {
            node.set_invalid(egui::accesskit::Invalid::True);
        } else {
            node.clear_invalid();
        }
    });
    if invalid {
        ui.painter().rect_stroke(
            value_rect,
            t.radius,
            Stroke::new(1.0, t.color.err),
            egui::StrokeKind::Inside,
        );
    }

    let action_response = ui.interact(
        action_rect,
        ui.id().with(("property-row-action", action_label)),
        if action_enabled {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    action_response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            action_enabled,
            action_label.to_owned(),
        )
    });
    ui.ctx().accesskit_node_builder(action_response.id, |node| {
        node.set_label(action_label);
        if !action_enabled && let Some(reason) = action_disabled_reason {
            node.set_description(reason);
        }
    });
    let action_fill = if action_response.hovered() && action_enabled {
        t.color.bg_hover
    } else {
        t.color.bg_elevated
    };
    ui.painter().rect(
        action_rect,
        t.radius,
        action_fill,
        Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    action_icon.paint(
        ui.painter(),
        Rect::from_center_size(action_rect.center(), Vec2::splat(15.0)),
        if action_enabled {
            t.color.text_dim
        } else {
            t.color.text_faint
        },
    );
    theme::paint_focus_ring_outset(ui, &action_response, action_rect);
    let action_response = if action_enabled {
        action_response.on_hover_text(action_label)
    } else if let Some(reason) = action_disabled_reason {
        action_response.on_hover_text(reason)
    } else {
        action_response.on_hover_text(action_label)
    };
    (edit_response, action_response)
}

/// Selectable twin of [`property_row`], with the same fixed label/value
/// columns as read-only rows and text inputs. `options` carries a durable
/// value and its presentation label; the returned flag is true only when the
/// selected value changed.
pub fn property_row_combo(
    ui: &mut Ui,
    label: &str,
    id_source: impl Hash + std::fmt::Debug,
    selected: &mut String,
    options: &[(String, String)],
    enabled: bool,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (label_column, gap, value_column) = property_row_columns(width);
    let height = t.metrics.ctl_h.max(PROPERTY_ROW_MIN_H);
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());
    let label_rect = Rect::from_min_max(
        Pos2::new(rect.left() + PROPERTY_ROW_PAD, rect.top()),
        Pos2::new(rect.left() + PROPERTY_ROW_PAD + label_column, rect.bottom()),
    );
    ui.painter().with_clip_rect(label_rect).text(
        Pos2::new(label_rect.left(), label_rect.center().y),
        Align2::LEFT_CENTER,
        label,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    let value_rect = Rect::from_min_max(
        Pos2::new(
            label_rect.right() + gap,
            rect.center().y - t.metrics.ctl_h * 0.5,
        ),
        Pos2::new(
            rect.right() - PROPERTY_ROW_PAD,
            rect.center().y + t.metrics.ctl_h * 0.5,
        ),
    );
    let selected_label = options
        .iter()
        .find(|(value, _)| value == selected)
        .map_or_else(|| selected.clone(), |(_, display)| display.clone());
    let before = selected.clone();
    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(value_rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
        |ui| {
            ui.set_width(value_rect.width());
            ui.add_enabled_ui(enabled, |ui| {
                let output = egui::ComboBox::from_id_salt(id_source)
                    .width(value_column)
                    .selected_text(selected_label)
                    .show_ui(ui, |ui| {
                        for (value, display) in options {
                            ui.selectable_value(selected, value.clone(), display);
                        }
                    });
                ui.ctx()
                    .accesskit_node_builder(output.response.id, |node| node.set_label(label));
            });
        },
    );
    *selected != before
}

fn property_row_with_tone(
    ui: &mut Ui,
    label: &str,
    value: &str,
    value_tone: Color32,
    mark: Option<StatusMark>,
    value_size: f32,
) -> Response {
    let t = Tokens::get(ui.ctx());
    let full_label = label;
    let full_value = value;
    let label_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let value_font = theme::mono(value_size, FontWeight::Regular);
    let width = ui.available_width().max(1.0);
    let (label_column, gap, value_column) = property_row_columns(width);
    let display_label = elide_text(ui, label, &label_font, label_column);
    let label_galley = ui
        .painter()
        .layout_no_wrap(display_label, label_font, t.color.text_dim);
    let status_prefix = if mark.is_some() { 17.0 } else { 0.0 };
    let value_width = (value_column - status_prefix).max(1.0);
    let display_value = elide_text(ui, value, &value_font, value_width);
    let value_galley = ui
        .painter()
        .layout_no_wrap(display_value, value_font, value_tone);
    // Read-only property cells share the editable row's invariant geometry.
    // Both columns are one line and ellipsized so changing the selected object
    // can never move the rows below it.
    let height = t.metrics.ctl_h.max(PROPERTY_ROW_MIN_H);
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());
    let label_rect = Rect::from_min_max(
        Pos2::new(rect.left() + PROPERTY_ROW_PAD, rect.top()),
        Pos2::new(rect.left() + PROPERTY_ROW_PAD + label_column, rect.bottom()),
    );
    let value_rect = Rect::from_min_max(
        Pos2::new(label_rect.right() + gap, rect.top()),
        Pos2::new(rect.right() - PROPERTY_ROW_PAD, rect.bottom()),
    );
    ui.painter().with_clip_rect(label_rect).galley(
        Pos2::new(
            label_rect.left(),
            label_rect.center().y - label_galley.size().y * 0.5,
        ),
        label_galley,
        t.color.text_dim,
    );
    if let Some(mark) = mark {
        paint_status_mark(
            &ui.painter().with_clip_rect(value_rect),
            Rect::from_center_size(
                Pos2::new(value_rect.left() + 5.5, rect.center().y),
                Vec2::splat(11.0),
            ),
            mark,
            value_tone,
        );
    }
    ui.painter().with_clip_rect(value_rect).galley(
        Pos2::new(
            value_rect.left() + status_prefix,
            value_rect.center().y - value_galley.size().y * 0.5,
        ),
        value_galley,
        value_tone,
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(full_label);
        node.set_value(full_value);
    });
    response.on_hover_text(format!("{full_label}: {full_value}"))
}

pub(crate) fn elide_text(ui: &Ui, text: &str, font: &egui::FontId, max_width: f32) -> String {
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
            // Some workbench surfaces deliberately retain a wider minimum
            // document extent for horizontal scrolling. A title bar belongs
            // to the visible pane, not that off-screen content extent.
            let available = ui.available_rect_before_wrap();
            let visible = available.intersect(ui.clip_rect()).intersect(ui.max_rect());
            // When the clip boundary, rather than the ordinary available
            // rectangle, limits the row, Frame has already advanced past its
            // left inset but its right inset still lies beyond the clip.
            // Reserve that trailing eight points explicitly.
            let clipped_on_right = visible.right() + f32::EPSILON < available.right();
            let visible_width =
                (visible.width() - if clipped_on_right { 8.0 } else { 0.0 }).max(1.0);
            ui.set_width(visible_width);
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

/// Empty document state with a compact, caller-owned recovery action row.
/// Actions are part of the state rather than permanent workspace chrome and
/// disappear as soon as the missing source authority is created or imported.
pub fn empty_state_with_actions(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    title: &str,
    description: &str,
    mut actions: impl FnMut(&mut Ui),
) {
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
        ui.add_space(12.0);
        let measure_origin = ui.next_widget_position();
        let mut measure_ui = ui.new_child(
            egui::UiBuilder::new()
                .id_salt("empty-state-recovery-action-measure")
                .max_rect(Rect::from_min_size(
                    measure_origin,
                    Vec2::new(ui.available_width(), ui.spacing().interact_size.y),
                ))
                .layout(egui::Layout::left_to_right(egui::Align::Center))
                .sizing_pass()
                .invisible(),
        );
        measure_ui.spacing_mut().item_spacing.x = 6.0;
        actions(&mut measure_ui);
        let action_width = measure_ui.min_rect().width();
        ui.horizontal(|ui| {
            let leading_space = ((ui.available_width() - action_width) * 0.5).max(0.0);
            if leading_space > 0.0 {
                ui.add_space(leading_space);
            }
            ui.scope(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                actions(ui);
            });
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workspace_title_row_uses_visible_pane_not_offscreen_content_extent() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut measured_width = f32::INFINITY;
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, egui::vec2(900.0, 700.0))),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let content_rect =
                        Rect::from_min_max(egui::pos2(306.0, 80.0), egui::pos2(1_206.0, 700.0));
                    let mut surface = ui.new_child(egui::UiBuilder::new().max_rect(content_rect));
                    surface.set_clip_rect(Rect::from_min_max(
                        content_rect.min,
                        egui::pos2(900.0, 700.0),
                    ));
                    workspace_title_row(&mut surface, |row| {
                        measured_width = row.available_width();
                    });
                });
            },
        );

        assert!(
            measured_width <= 578.0,
            "title content escaped the 594 px visible pane: {measured_width}"
        );
    }

    fn shape_contains_text(shape: &Shape) -> bool {
        match shape {
            Shape::Text(_) => true,
            Shape::Vec(shapes) => shapes.iter().any(shape_contains_text),
            _ => false,
        }
    }

    #[test]
    fn semantic_status_marks_are_font_independent_vector_geometry() {
        let ctx = egui::Context::default();
        let output = ctx.run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                for mark in [
                    StatusMark::Success,
                    StatusMark::Warning,
                    StatusMark::Failure,
                    StatusMark::Neutral,
                ] {
                    let (rect, _) = ui.allocate_exact_size(Vec2::splat(14.0), Sense::hover());
                    paint_status_mark(ui.painter(), rect, mark, Color32::WHITE);
                }
            });
        });

        let status_shapes = output
            .shapes
            .iter()
            .filter(|shape| {
                matches!(
                    &shape.shape,
                    Shape::Path(_) | Shape::LineSegment { .. } | Shape::Circle(_)
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            status_shapes
                .iter()
                .filter(|shape| matches!(&shape.shape, Shape::Path(_)))
                .count(),
            2
        );
        assert_eq!(
            status_shapes
                .iter()
                .filter(|shape| matches!(&shape.shape, Shape::LineSegment { .. }))
                .count(),
            2
        );
        assert_eq!(
            status_shapes
                .iter()
                .filter(|shape| matches!(&shape.shape, Shape::Circle(_)))
                .count(),
            1
        );
        assert!(
            output
                .shapes
                .iter()
                .all(|shape| !shape_contains_text(&shape.shape))
        );
    }

    #[test]
    fn section_header_uses_measured_copy_before_eliding() {
        let (title, meta) = section_header_column_widths(210.0, 47.0, 91.0, true);
        assert_eq!(meta, 91.0);
        assert!((title + SECTION_HEADER_COLUMN_GAP + meta - 210.0).abs() <= 0.001);
        assert!(title >= 47.0);
        assert!(!section_header_wraps(210.0, 47.0, 91.0, true));
    }

    #[test]
    fn overconstrained_section_header_wraps_instead_of_eliding_both_columns() {
        let (title, meta) = section_header_column_widths(100.0, 80.0, 80.0, true);
        assert!((title - 50.6).abs() <= 0.001);
        assert!((meta - 41.4).abs() <= 0.001);
        assert!((title + SECTION_HEADER_COLUMN_GAP + meta - 100.0).abs() <= 0.001);
        assert!(section_header_wraps(100.0, 80.0, 80.0, true));

        assert_eq!(
            section_header_column_widths(100.0, 180.0, 0.0, false),
            (100.0, 0.0)
        );
        assert!(section_header_wraps(100.0, 180.0, 0.0, false));
        assert_eq!(
            section_header_full_text("LONG ENGINEERING TITLE", Some("complete metadata")),
            "LONG ENGINEERING TITLE, complete metadata"
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn property_controls_and_section_headers_expose_their_full_accessible_names() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut value = "R1".to_owned();
        let mut tunable_value = "1k".to_owned();
        let mut selected = "tt".to_owned();
        let read_only_value = "vendor_analog/OPA189/precision_zero_drift";
        let options = vec![
            ("tt".to_owned(), "Typical".to_owned()),
            ("ff".to_owned(), "Fast".to_owned()),
        ];

        let nodes = ctx
            .run_ui(Default::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.set_width(160.0);
                    section_header(
                        ui,
                        "Long engineering section title",
                        Some("complete metadata"),
                    );
                    property_row_input(ui, "Instance", &mut value, false);
                    property_row_input_action(
                        ui,
                        "Value",
                        &mut tunable_value,
                        false,
                        WorkbenchIcon::Sliders,
                        "Tune value",
                        true,
                        None,
                    );
                    property_row_combo(
                        ui,
                        "Model section",
                        "accessible-model-section",
                        &mut selected,
                        &options,
                        true,
                    );
                    property_row(ui, "Library cell", read_only_value);
                });
            })
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update")
            .nodes;

        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Heading
                && node.label() == Some("LONG ENGINEERING SECTION TITLE, complete metadata")
                && node.level() == Some(3)
        }));
        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::TextInput && node.label() == Some("Instance")
        }));
        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::TextInput && node.label() == Some("Value")
        }));
        assert!(
            nodes
                .iter()
                .any(|(_, node)| node.label() == Some("Model section"))
        );
        assert!(nodes.iter().any(|(_, node)| {
            node.label() == Some("Library cell") && node.value() == Some(read_only_value)
        }));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn empty_state_recovery_actions_are_exposed_as_named_buttons() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();

        let nodes = ctx
            .run_ui(
                egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(Pos2::ZERO, egui::vec2(900.0, 700.0))),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        empty_state_with_actions(
                            ui,
                            WorkbenchIcon::Code,
                            "No project source workspace",
                            "Create or import one to continue.",
                            |ui| {
                                let _ = ui.button("Create source workspace…");
                                let _ = ui.button("Import root source…");
                            },
                        );
                    });
                },
            )
            .platform_output
            .accesskit_update
            .expect("AccessKit empty-state tree")
            .nodes;

        for label in ["Create source workspace…", "Import root source…"] {
            assert!(nodes.iter().any(|(_, node)| {
                node.role() == egui::accesskit::Role::Button && node.label() == Some(label)
            }));
        }
    }

    #[test]
    fn empty_state_recovery_action_group_is_centered_under_the_copy() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut content_center = 0.0;
        let mut first = Rect::NOTHING;
        let mut second = Rect::NOTHING;

        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, egui::vec2(900.0, 700.0))),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    content_center = ui.available_rect_before_wrap().center().x;
                    empty_state_with_actions(
                        ui,
                        WorkbenchIcon::Code,
                        "No project source workspace",
                        "Create or import one to continue.",
                        |ui| {
                            first = ui.button("Create source workspace…").rect;
                            second = ui.button("Import root source…").rect;
                        },
                    );
                });
            },
        );

        let group_center = first.union(second).center().x;
        assert!(
            (group_center - content_center).abs() <= 1.0,
            "recovery action group centered at {group_center}, expected {content_center}"
        );
    }

    #[test]
    fn long_read_only_cells_do_not_reflow_property_rows() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut short_height = 0.0;
        let mut long_value_height = 0.0;
        let mut long_label_height = 0.0;

        let _ = ctx.run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.set_width(180.0);
                short_height = property_row(ui, "View", "schematic").rect.height();
                long_value_height = property_row(
                    ui,
                    "Library cell",
                    "vendor_analog/OPA189/precision_zero_drift/amplifier_symbol",
                )
                .rect
                .height();
                long_label_height = property_row(
                    ui,
                    "Inherited technology and model binding authority",
                    "project",
                )
                .rect
                .height();
            });
        });

        assert_eq!(short_height, PROPERTY_ROW_MIN_H);
        assert_eq!(long_value_height, short_height);
        assert_eq!(long_label_height, short_height);
    }

    #[test]
    fn narrow_property_rows_keep_the_proportional_label_split() {
        // The inspector dock clamps its width to 278..=440, and the widest
        // in-surface inspector is 330. None of them reach the cap, so every
        // narrow property list lays out exactly as it did before the cap.
        for width in [278.0_f32, 281.6, 312.0, 330.0, 440.0] {
            let (label_column, gap, value_column) = property_row_columns(width);
            let inner = width - 2.0 * PROPERTY_ROW_PAD;
            let columns = inner - PROPERTY_ROW_GAP;
            assert!(
                (label_column - columns * PROPERTY_LABEL_FRACTION).abs() <= 0.001,
                "the cap bound at {width} px and narrowed a dock-width label column"
            );
            assert!((label_column + gap + value_column - inner).abs() <= 0.001);
        }
    }

    #[test]
    fn wide_property_rows_stop_pushing_the_value_column_rightwards() {
        // A two-column dialog that collapses at its 820 pt breakpoint hands its
        // aside the full dialog width. Without the cap each value would start
        // ~40% across, leaving a gap that reads as unfinished layout.
        let inner = 780.0 - 2.0 * PROPERTY_ROW_PAD;
        let (label_column, gap, value_column) = property_row_columns(780.0);
        assert_eq!(label_column, PROPERTY_LABEL_MAX_W);
        assert!(label_column < (inner - PROPERTY_ROW_GAP) * PROPERTY_LABEL_FRACTION);
        assert!((label_column + gap + value_column - inner).abs() <= 0.001);

        // The cap is an upper bound, not a fixed column: the label column never
        // grows past it however wide the row gets.
        assert_eq!(property_row_columns(2_000.0).0, PROPERTY_LABEL_MAX_W);
    }

    #[test]
    fn the_label_column_cap_clears_the_widest_label_the_workbench_ships() {
        // Widest property-row label in the workbench, from the simulation plan
        // manager's aside. The cap has to clear it, or capping the column would
        // trade a layout gap for newly elided labels on wide rows.
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut widest = f32::INFINITY;
        let _ = ctx.run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                widest = ui
                    .painter()
                    .layout_no_wrap(
                        "Variables, outputs, specifications".to_owned(),
                        theme::sans(tokens::FS_0, FontWeight::Regular),
                        Color32::WHITE,
                    )
                    .size()
                    .x;
            });
        });

        assert!(
            widest <= PROPERTY_LABEL_MAX_W,
            "the widest shipped label needs {widest} px but the cap allows \
             {PROPERTY_LABEL_MAX_W}"
        );
    }

    #[test]
    fn schematic_section_heading_uses_the_mockup_tracking() {
        assert_eq!(tokens::FS_2, 13.0);
        assert!((SCHEMATIC_SECTION_TITLE_TRACKING - 0.715).abs() <= 0.001);
    }

    #[test]
    fn card_content_remains_vertical_inside_a_horizontal_parent() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut first = egui::Rect::NOTHING;
        let mut second = egui::Rect::NOTHING;

        let _ = ctx.run_ui(Default::default(), |ctx| {
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
