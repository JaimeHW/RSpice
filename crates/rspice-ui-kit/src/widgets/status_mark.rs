//! Compact semantic status marks painted as vector geometry.
//!
//! These deliberately replace Unicode check/triangle characters in dense
//! engineering rows. The bundled text faces are not the authority for icon
//! coverage, and a missing fallback must never turn a status into a tofu box.
//!
//! They sit in the kit rather than in the workbench's design system because
//! surfaces below the shell paint them too: the waveform mini a list row draws
//! shows this warning where the engine refuses the card, and a triangle drawn
//! a second time in that painter would be the same mark at a second size and a
//! second weight.

use egui::{Color32, Pos2, Rect, Shape, Stroke};

/// What a status mark says.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusMark {
    Success,
    Warning,
    Failure,
    Neutral,
}

/// Paint one mark, sized to the shorter side of `rect`.
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
