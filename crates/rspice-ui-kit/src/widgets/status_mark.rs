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
//!
//! A warning or a failure the reader has to notice is a [`Problem`]: a small
//! coloured mark in front of neutral text, the grammar of a compiler's problem
//! list. The text itself is never coloured, so the mark is the one thing on the
//! line that carries severity.

use egui::{Color32, Pos2, Rect, Response, Sense, Shape, Stroke, Ui, vec2};

use crate::palette::Palette;
use crate::theme::{self, FontWeight};
use crate::tokens::{self, Tokens};

/// What a status mark says.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusMark {
    Success,
    Warning,
    Failure,
    Neutral,
}

/// A problem the reader has to notice.
///
/// It owns its colour as well as its shape, so a caller cannot pair a cross
/// with the warning tone or a triangle with the error one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Problem {
    /// Something to look at before relying on the result: a triangle in `warn`.
    Warning,
    /// Something that stops the work: a cross in `err`.
    Failure,
}

impl Problem {
    /// The colour the mark is painted in, and nothing else on its line.
    pub fn color(self, palette: &Palette) -> Color32 {
        match self {
            Self::Warning => palette.warn,
            Self::Failure => palette.err,
        }
    }

    /// The vector shape the mark is drawn as.
    pub fn mark(self) -> StatusMark {
        match self {
            Self::Warning => StatusMark::Warning,
            Self::Failure => StatusMark::Failure,
        }
    }

    /// What assistive technology says in front of the text, in place of the
    /// mark it cannot see.
    pub fn announcement(self) -> &'static str {
        match self {
            Self::Warning => "Warning",
            Self::Failure => "Error",
        }
    }

    /// `text` as a screen reader is told it: the severity, then the words.
    pub fn announce(self, text: &str) -> String {
        format!("{}: {text}", self.announcement())
    }
}

/// Side of a problem mark set in front of a line of text.
pub(crate) const PROBLEM_MARK_SIDE: f32 = 11.0;
/// From the mark's left edge to where its text starts: the mark and the gap
/// after it. Every row that sets a mark before its value uses this advance, so
/// a marked line and an unmarked one beside it differ by exactly this much.
pub(crate) const PROBLEM_MARK_ADVANCE: f32 = 17.0;

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

/// Paint a problem's mark, in the problem's own colour from the active theme.
pub fn paint_problem_mark(painter: &egui::Painter, rect: Rect, problem: Problem) {
    let palette = Tokens::get(painter.ctx()).color;
    paint_status_mark(painter, rect, problem.mark(), problem.color(&palette));
}

/// A problem stated on its own line: the mark, then `text` in `text_dim`.
///
/// The text wraps to the width the line is given, the mark sits on its first
/// line, and a screen reader hears "Warning: …" or "Error: …" for the whole.
pub fn problem_line(ui: &mut Ui, problem: Problem, text: &str) -> Response {
    problem_line_inset(ui, problem, text, 0.0, 0.0)
}

/// [`problem_line`] set `inset` in from both edges, with `pad_y` above and
/// below its text.
pub(crate) fn problem_line_inset(
    ui: &mut Ui,
    problem: Problem,
    text: &str,
    inset: f32,
    pad_y: f32,
) -> Response {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let wrap_width = (width - 2.0 * inset - PROBLEM_MARK_ADVANCE).max(1.0);
    let galley = ui.painter().layout(
        text.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
        wrap_width,
    );
    let (rect, response) =
        ui.allocate_exact_size(vec2(width, galley.size().y + 2.0 * pad_y), Sense::hover());
    let announcement = problem.announce(text);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &announcement)
    });
    if !ui.is_rect_visible(rect) {
        return response;
    }
    let text_top = rect.top() + pad_y;
    let first_line = galley
        .rows
        .first()
        .map_or(galley.size().y * 0.5, |row| row.rect().center().y);
    paint_problem_mark(
        ui.painter(),
        Rect::from_center_size(
            Pos2::new(
                rect.left() + inset + PROBLEM_MARK_SIDE * 0.5,
                text_top + first_line,
            ),
            egui::Vec2::splat(PROBLEM_MARK_SIDE),
        ),
        problem,
    );
    ui.painter().galley(
        Pos2::new(rect.left() + inset + PROBLEM_MARK_ADVANCE, text_top),
        galley,
        t.color.text_dim,
    );
    response
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every painted colour in a pass's shapes, text included.
    fn painted_colors(shape: &Shape, colors: &mut Vec<Color32>) {
        match shape {
            Shape::Vec(shapes) => {
                for shape in shapes {
                    painted_colors(shape, colors);
                }
            }
            Shape::LineSegment { stroke, .. } => colors.push(stroke.color),
            Shape::Path(path) => {
                if let egui::epaint::ColorMode::Solid(color) = path.stroke.color {
                    colors.push(color);
                }
            }
            Shape::Text(text) => {
                colors.push(text.fallback_color);
                colors.extend(text.galley.job.sections.iter().map(|s| s.format.color));
            }
            _ => {}
        }
    }

    fn paint(mut add: impl FnMut(&mut Ui)) -> Vec<Shape> {
        let ctx = egui::Context::default();
        crate::Theme::default().apply(&ctx);
        ctx.run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| add(ui));
        })
        .shapes
        .into_iter()
        .map(|clipped| clipped.shape)
        .collect()
    }

    /// The mark carries the severity's colour, chosen by the problem rather
    /// than by its caller.
    #[test]
    fn a_problem_mark_is_painted_in_the_problems_own_colour() {
        let palette = Tokens::default().color;
        assert_eq!(Problem::Warning.color(&palette), palette.warn);
        assert_eq!(Problem::Failure.color(&palette), palette.err);
        assert_eq!(Problem::Warning.mark(), StatusMark::Warning);
        assert_eq!(Problem::Failure.mark(), StatusMark::Failure);
        for problem in [Problem::Warning, Problem::Failure] {
            let mut colors = Vec::new();
            for shape in paint(|ui| {
                let (rect, _) = ui.allocate_exact_size(egui::Vec2::splat(14.0), Sense::hover());
                paint_problem_mark(ui.painter(), rect, problem);
            }) {
                painted_colors(&shape, &mut colors);
            }
            assert!(!colors.is_empty(), "{problem:?} painted nothing");
            assert!(
                colors.iter().all(|color| *color == problem.color(&palette)),
                "{problem:?} painted {colors:?}"
            );
        }
    }

    /// The words of a problem line are neutral: only the mark is coloured.
    #[test]
    fn a_problem_lines_text_is_dim_and_only_its_mark_is_coloured() {
        let palette = Tokens::default().color;
        for problem in [Problem::Warning, Problem::Failure] {
            let shapes = paint(|ui| {
                problem_line(ui, problem, "Port number 1 is claimed twice");
            });
            let text_colors = shapes
                .iter()
                .filter_map(|shape| match shape {
                    Shape::Text(text) => Some(text.fallback_color),
                    _ => None,
                })
                .collect::<Vec<_>>();
            assert_eq!(text_colors, vec![palette.text_dim], "{problem:?}");
            let mut mark_colors = Vec::new();
            for shape in shapes
                .iter()
                .filter(|shape| !matches!(shape, Shape::Text(_)))
            {
                painted_colors(shape, &mut mark_colors);
            }
            assert!(!mark_colors.is_empty(), "{problem:?} painted no mark");
            assert!(
                mark_colors
                    .iter()
                    .all(|color| *color == problem.color(&palette)),
                "{problem:?} mark {mark_colors:?}"
            );
        }
    }

    /// A screen reader hears the severity the mark shows.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn a_problem_line_announces_its_severity_before_its_text() {
        let ctx = egui::Context::default();
        crate::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let nodes = ctx
            .run_ui(Default::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    problem_line(ui, Problem::Warning, "Two ports share index 1");
                    problem_line(ui, Problem::Failure, "The card does not parse");
                });
            })
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update")
            .nodes;
        for wanted in [
            "Warning: Two ports share index 1",
            "Error: The card does not parse",
        ] {
            assert!(
                nodes
                    .iter()
                    .any(|(_, node)| node.value() == Some(wanted) || node.label() == Some(wanted)),
                "nothing announces {wanted:?}"
            );
        }
    }

    /// A long problem wraps, and its mark stays on the first line.
    #[test]
    fn a_wrapped_problem_line_keeps_its_mark_on_the_first_line() {
        let shapes = paint(|ui| {
            ui.allocate_ui(egui::vec2(160.0, 200.0), |ui| {
                problem_line(
                    ui,
                    Problem::Failure,
                    "A refusal long enough to need several lines at this width, which wraps",
                );
            });
        });
        let text = shapes
            .iter()
            .find_map(|shape| match shape {
                Shape::Text(text) => Some(text.clone()),
                _ => None,
            })
            .expect("the line paints its text");
        assert!(text.galley.rows.len() > 1, "the text did not wrap");
        let first_row = text.galley.rows[0].rect().translate(text.pos.to_vec2());
        let mark = shapes
            .iter()
            .filter(|shape| matches!(shape, Shape::LineSegment { .. }))
            .map(Shape::visual_bounding_rect)
            .fold(Rect::NOTHING, Rect::union);
        assert!(mark.is_positive(), "no mark painted");
        assert!(
            (mark.center().y - first_row.center().y).abs() <= 1.0,
            "mark {mark:?} is not on the first line {first_row:?}"
        );
        assert!(mark.right() < text.pos.x, "the mark overlaps the text");
    }
}
