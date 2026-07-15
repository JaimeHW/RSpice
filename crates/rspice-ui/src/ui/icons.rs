//! Vector icon set.
//!
//! Icons are 1.7 pt stroke drawings authored on a 24 × 24 grid, transcribed
//! from the design specification's SVG paths. They are painted directly as
//! tessellated polylines — no rasterized assets, so they stay crisp at every
//! DPI and inherit color from the caller.

use egui::{Painter, Pos2, Rect, Shape, Stroke, Vec2, pos2, vec2};

/// Logical canvas size icons are authored on.
const GRID: f32 = 24.0;

/// One drawing primitive in icon space.
enum Seg {
    /// Open polyline through the listed points.
    Line(&'static [[f32; 2]]),
    /// Closed polygon through the listed points.
    Poly(&'static [[f32; 2]]),
    /// Filled convex polygon through the listed points.
    Fill(&'static [[f32; 2]]),
    /// Circle outline: center, radius.
    Circle([f32; 2], f32),
    /// Filled dot: center, radius.
    Dot([f32; 2], f32),
    /// Circular arc: center, radius, start angle (deg), sweep (deg).
    Arc([f32; 2], f32, f32, f32),
}

/// The application icon vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Icon {
    /// Selection tool (cursor arrow).
    Select,
    /// Wire drawing tool.
    Wire,
    /// Net label tool.
    NetLabel,
    /// Pin / port placement tool.
    Pin,
    /// Net probe tool.
    Probe,
    /// Symbol circle body tool.
    SymbolCircle,
    /// Symbol arc body tool.
    SymbolArc,
    /// Symbol arrow body tool.
    SymbolArrow,
    /// Symbol dot body tool.
    SymbolDot,
    /// Zoom in.
    ZoomIn,
    /// Zoom out.
    ZoomOut,
    /// Zoom to fit.
    ZoomFit,
    /// Grid toggle (line grid / generic grid).
    Grid,
    /// Dot-grid variant of the grid toggle.
    GridDots,
    /// Undo.
    Undo,
    /// Redo.
    Redo,
    /// Design-check (ERC) checkmark.
    Check,
    /// Document / netlist file.
    File,
    /// Project or directory browser.
    Folder,
    /// Add or create.
    Add,
    /// Stop (square).
    Stop,
    /// Run (play triangle).
    Run,
    /// Library workspace (shelf).
    Library,
    /// Schematic workspace (wire bend).
    Schematic,
    /// Simulate workspace (pulse).
    Simulate,
    /// Results workspace (bars).
    Results,
    /// Ground symbol.
    Ground,
    /// Supply flag symbol.
    Supply,
    /// Trash / clear.
    Trash,
    /// Chevron up.
    ChevronUp,
    /// Chevron down.
    ChevronDown,
    /// Close (×).
    Close,
    /// Application brand mark (the RSpice forward waveform).
    Brand,
}

impl Icon {
    /// Stable text alternative used by icon-only controls when a more
    /// specific tooltip has not been supplied by the caller.
    pub const fn accessible_label(self) -> &'static str {
        match self {
            Self::Select => "Select",
            Self::Wire => "Draw wire",
            Self::NetLabel => "Place net label",
            Self::Pin => "Place pin",
            Self::Probe => "Probe",
            Self::SymbolCircle => "Draw circle",
            Self::SymbolArc => "Draw arc",
            Self::SymbolArrow => "Place arrow",
            Self::SymbolDot => "Place dot",
            Self::ZoomIn => "Zoom in",
            Self::ZoomOut => "Zoom out",
            Self::ZoomFit => "Zoom to fit",
            Self::Grid => "Grid",
            Self::GridDots => "Dot grid",
            Self::Undo => "Undo",
            Self::Redo => "Redo",
            Self::Check => "Run checks",
            Self::File => "Document",
            Self::Folder => "Browse",
            Self::Add => "Add",
            Self::Stop => "Stop",
            Self::Run => "Run",
            Self::Library => "Library workspace",
            Self::Schematic => "Schematic workspace",
            Self::Simulate => "Simulation workspace",
            Self::Results => "Results workspace",
            Self::Ground => "Ground",
            Self::Supply => "Supply",
            Self::Trash => "Delete",
            Self::ChevronUp => "Move up",
            Self::ChevronDown => "Move down",
            Self::Close => "Close",
            Self::Brand => "RSpice",
        }
    }

    fn segments(self) -> &'static [Seg] {
        use Seg::*;
        match self {
            Icon::Select => &[Poly(&[[5.0, 3.0], [19.0, 11.0], [12.5, 12.5], [9.0, 19.0]])],
            Icon::Wire => &[
                Line(&[[4.0, 18.0], [11.0, 18.0], [11.0, 6.0], [20.0, 6.0]]),
                Dot([4.0, 18.0], 1.6),
                Dot([20.0, 6.0], 1.6),
            ],
            Icon::NetLabel => &[
                Line(&[[5.0, 7.0], [5.0, 5.0], [19.0, 5.0], [19.0, 7.0]]),
                Line(&[[12.0, 5.0], [12.0, 19.0]]),
                Line(&[[9.0, 19.0], [15.0, 19.0]]),
            ],
            Icon::Pin => &[Poly(&[
                [4.0, 9.0],
                [14.0, 9.0],
                [18.0, 12.0],
                [14.0, 15.0],
                [4.0, 15.0],
            ])],
            Icon::Probe => &[
                Circle([10.0, 10.0], 5.5),
                Line(&[[14.5, 14.5], [20.0, 20.0]]),
            ],
            Icon::SymbolCircle => &[Circle([12.0, 12.0], 6.0)],
            Icon::SymbolArc => &[
                Arc([12.0, 12.0], 7.0, -35.0, 250.0),
                Dot([18.0, 8.0], 1.2),
                Dot([8.0, 18.0], 1.2),
            ],
            Icon::SymbolArrow => &[
                Line(&[[5.0, 18.0], [16.0, 7.0]]),
                Fill(&[[14.0, 3.0], [21.0, 5.0], [19.0, 12.0]]),
            ],
            Icon::SymbolDot => &[Dot([12.0, 12.0], 4.2)],
            Icon::ZoomIn => &[
                Circle([11.0, 11.0], 7.0),
                Line(&[[11.0, 8.0], [11.0, 14.0]]),
                Line(&[[8.0, 11.0], [14.0, 11.0]]),
                Line(&[[16.5, 16.5], [21.0, 21.0]]),
            ],
            Icon::ZoomOut => &[
                Circle([11.0, 11.0], 7.0),
                Line(&[[8.0, 11.0], [14.0, 11.0]]),
                Line(&[[16.5, 16.5], [21.0, 21.0]]),
            ],
            Icon::ZoomFit => &[
                Line(&[[4.0, 9.0], [4.0, 4.0], [9.0, 4.0]]),
                Line(&[[20.0, 9.0], [20.0, 4.0], [15.0, 4.0]]),
                Line(&[[4.0, 15.0], [4.0, 20.0], [9.0, 20.0]]),
                Line(&[[20.0, 15.0], [20.0, 20.0], [15.0, 20.0]]),
            ],
            Icon::Grid => &[
                Line(&[[4.0, 9.0], [20.0, 9.0]]),
                Line(&[[4.0, 15.0], [20.0, 15.0]]),
                Line(&[[9.0, 4.0], [9.0, 20.0]]),
                Line(&[[15.0, 4.0], [15.0, 20.0]]),
            ],
            Icon::GridDots => &[
                Dot([5.0, 5.0], 1.4),
                Dot([12.0, 5.0], 1.4),
                Dot([19.0, 5.0], 1.4),
                Dot([5.0, 12.0], 1.4),
                Dot([12.0, 12.0], 1.4),
                Dot([19.0, 12.0], 1.4),
                Dot([5.0, 19.0], 1.4),
                Dot([12.0, 19.0], 1.4),
                Dot([19.0, 19.0], 1.4),
            ],
            Icon::Undo => &[
                Line(&[[8.0, 5.0], [3.0, 10.0], [8.0, 15.0]]),
                // M3 10 H14 then arc right half-circle down to (14,22), end H11.
                Line(&[[3.0, 10.0], [14.0, 10.0]]),
                Arc([14.0, 16.0], 6.0, -90.0, 180.0),
                Line(&[[14.0, 22.0], [11.0, 22.0]]),
            ],
            Icon::Redo => &[
                Line(&[[16.0, 5.0], [21.0, 10.0], [16.0, 15.0]]),
                Line(&[[21.0, 10.0], [10.0, 10.0]]),
                Arc([10.0, 16.0], 6.0, -90.0, -180.0),
                Line(&[[10.0, 22.0], [13.0, 22.0]]),
            ],
            Icon::Check => &[Line(&[[20.0, 7.0], [10.0, 17.0], [6.0, 13.0]])],
            Icon::File => &[
                Poly(&[
                    [7.0, 4.0],
                    [14.0, 4.0],
                    [18.0, 8.0],
                    [18.0, 20.0],
                    [7.0, 20.0],
                ]),
                Line(&[[14.0, 4.0], [14.0, 8.0], [18.0, 8.0]]),
                Line(&[[10.0, 13.0], [15.0, 13.0]]),
                Line(&[[10.0, 16.0], [15.0, 16.0]]),
            ],
            Icon::Folder => &[
                Poly(&[
                    [3.0, 7.0],
                    [10.0, 7.0],
                    [12.0, 10.0],
                    [21.0, 10.0],
                    [19.0, 19.0],
                    [3.0, 19.0],
                ]),
                Line(&[[3.0, 7.0], [3.0, 19.0]]),
            ],
            Icon::Add => &[
                Line(&[[12.0, 5.0], [12.0, 19.0]]),
                Line(&[[5.0, 12.0], [19.0, 12.0]]),
            ],
            // Transport glyphs are filled — a stroked square reads as a
            // missing-glyph box, not a stop control.
            Icon::Stop => &[Fill(&[[7.0, 7.0], [17.0, 7.0], [17.0, 17.0], [7.0, 17.0]])],
            Icon::Run => &[Fill(&[[7.0, 4.0], [20.0, 12.0], [7.0, 20.0]])],
            Icon::Library => &[
                Poly(&[[4.0, 5.0], [20.0, 5.0], [20.0, 9.0], [4.0, 9.0]]),
                Poly(&[[4.0, 13.0], [20.0, 13.0], [20.0, 19.0], [4.0, 19.0]]),
                Line(&[[8.0, 5.0], [8.0, 9.0]]),
                Line(&[[8.0, 13.0], [8.0, 19.0]]),
            ],
            Icon::Schematic => &[
                Line(&[
                    [4.0, 18.0],
                    [9.0, 18.0],
                    [9.0, 12.0],
                    [15.0, 12.0],
                    [15.0, 6.0],
                    [20.0, 6.0],
                ]),
                Dot([4.0, 18.0], 1.5),
                Dot([20.0, 6.0], 1.5),
            ],
            Icon::Simulate => &[Line(&[
                [4.0, 12.0],
                [8.0, 12.0],
                [10.0, 6.0],
                [14.0, 18.0],
                [16.0, 12.0],
                [20.0, 12.0],
            ])],
            Icon::Results => &[
                Line(&[[4.0, 20.0], [4.0, 8.0]]),
                Line(&[[10.0, 20.0], [10.0, 4.0]]),
                Line(&[[16.0, 20.0], [16.0, 11.0]]),
                Line(&[[21.0, 20.0], [3.0, 20.0]]),
            ],
            Icon::Ground => &[
                Line(&[[12.0, 4.0], [12.0, 11.0]]),
                Line(&[[5.0, 11.0], [19.0, 11.0]]),
                Line(&[[8.0, 15.0], [16.0, 15.0]]),
                Line(&[[10.0, 19.0], [14.0, 19.0]]),
            ],
            Icon::Supply => &[
                Line(&[[12.0, 20.0], [12.0, 7.0]]),
                Line(&[[6.0, 7.0], [18.0, 7.0]]),
            ],
            Icon::Trash => &[
                Line(&[[5.0, 7.0], [19.0, 7.0]]),
                Line(&[[9.0, 7.0], [9.0, 5.0], [15.0, 5.0], [15.0, 7.0]]),
                Line(&[[7.0, 7.0], [8.0, 20.0], [16.0, 20.0], [17.0, 7.0]]),
            ],
            Icon::ChevronUp => &[Line(&[[6.0, 14.0], [12.0, 8.0], [18.0, 14.0]])],
            Icon::ChevronDown => &[Line(&[[6.0, 10.0], [12.0, 16.0], [18.0, 10.0]])],
            Icon::Close => &[
                Line(&[[6.0, 6.0], [18.0, 18.0]]),
                Line(&[[18.0, 6.0], [6.0, 18.0]]),
            ],
            Icon::Brand => &[Line(&[[7.0, 8.0], [17.0, 12.0], [7.0, 16.0]])],
        }
    }

    /// Paint the icon inside `rect`, preserving aspect, with stroke width
    /// scaled relative to the design's 1.7 pt at 16 px.
    pub fn paint(self, painter: &Painter, rect: Rect, color: egui::Color32) {
        let side = rect.width().min(rect.height());
        let scale = side / GRID;
        let origin = rect.center() - vec2(side, side) * 0.5;
        let map = |p: [f32; 2]| -> Pos2 { origin + Vec2::new(p[0], p[1]) * scale };

        // 1.7 pt at the 16 px reference size → scale proportionally, but keep
        // a crisp minimum of 1 pt.
        let width = (1.7 * side / 16.0).max(1.0);
        let stroke = Stroke::new(width, color);

        for seg in self.segments() {
            match seg {
                Seg::Line(pts) => {
                    painter.add(Shape::line(pts.iter().map(|&p| map(p)).collect(), stroke));
                }
                Seg::Poly(pts) => {
                    painter.add(Shape::closed_line(
                        pts.iter().map(|&p| map(p)).collect(),
                        stroke,
                    ));
                }
                Seg::Fill(pts) => {
                    painter.add(Shape::convex_polygon(
                        pts.iter().map(|&p| map(p)).collect(),
                        color,
                        Stroke::NONE,
                    ));
                }
                Seg::Circle(center, r) => {
                    painter.circle_stroke(map(*center), r * scale, stroke);
                }
                Seg::Dot(center, r) => {
                    painter.circle_filled(map(*center), r * scale, color);
                }
                Seg::Arc(center, r, start_deg, sweep_deg) => {
                    let c = map(*center);
                    let r = r * scale;
                    let n = 16;
                    let points: Vec<Pos2> = (0..=n)
                        .map(|i| {
                            let a = (start_deg + sweep_deg * i as f32 / n as f32).to_radians();
                            pos2(c.x + r * a.cos(), c.y + r * a.sin())
                        })
                        .collect();
                    painter.add(Shape::line(points, stroke));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_icons_have_geometry_within_grid() {
        let all = [
            Icon::Select,
            Icon::Wire,
            Icon::NetLabel,
            Icon::Pin,
            Icon::Probe,
            Icon::SymbolCircle,
            Icon::SymbolArc,
            Icon::SymbolArrow,
            Icon::SymbolDot,
            Icon::ZoomIn,
            Icon::ZoomOut,
            Icon::ZoomFit,
            Icon::Grid,
            Icon::Undo,
            Icon::Redo,
            Icon::Check,
            Icon::File,
            Icon::Folder,
            Icon::Add,
            Icon::Stop,
            Icon::Run,
            Icon::Library,
            Icon::Schematic,
            Icon::Simulate,
            Icon::Results,
            Icon::Ground,
            Icon::Supply,
            Icon::Trash,
            Icon::ChevronUp,
            Icon::ChevronDown,
            Icon::Close,
            Icon::Brand,
        ];
        for icon in all {
            let segs = icon.segments();
            assert!(!segs.is_empty());
            for seg in segs {
                let check = |p: &[f32; 2]| {
                    assert!(
                        (0.0..=GRID).contains(&p[0]) && (0.0..=GRID).contains(&p[1]),
                        "{icon:?} point {p:?} outside grid"
                    );
                };
                match seg {
                    Seg::Line(pts) | Seg::Poly(pts) | Seg::Fill(pts) => {
                        pts.iter().for_each(check);
                    }
                    Seg::Circle(c, r) | Seg::Dot(c, r) | Seg::Arc(c, r, _, _) => {
                        assert!(c[0] - r >= -0.5 && c[0] + r <= GRID + 0.5, "{icon:?}");
                        assert!(c[1] - r >= -0.5 && c[1] + r <= GRID + 0.5, "{icon:?}");
                    }
                }
            }
        }
    }
}
