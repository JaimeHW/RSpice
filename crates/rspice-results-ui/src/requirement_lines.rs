//! A requirement drawn on a result is a way to where it is judged: a click on
//! its line, or the line's context menu, asks to open it in Verify.
//!
//! The viewers keep drawing their lines as they always have; this only lays
//! a thin target over each, after the plot, so it takes the click and the
//! right-click while a drag beside or across it still reaches the plot.

use egui::{CursorIcon, Rect, Sense, Ui, WidgetInfo, WidgetType};
use rspice_ui_kit::plot::{PlotResponse, XScale};

/// How far either side of a line the pointer still lands on it, in points.
const REACH: f32 = 4.0;

/// One requirement line as a plot draws it, in data space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RequirementLine {
    /// A vertical line at `x`, the plot's full height.
    Vertical { x: f64 },
    /// A horizontal line at `y` across `from..to` of the abscissa.
    Horizontal { y: f64, from: f64, to: f64 },
}

impl RequirementLine {
    /// A horizontal line the plot's full width.
    pub const fn across(y: f64) -> Self {
        Self::Horizontal {
            y,
            from: f64::NEG_INFINITY,
            to: f64::INFINITY,
        }
    }

    /// Where the line lies on screen, widened to a target; `None` when it is
    /// outside what the plot drew.
    fn target(self, plot: &PlotResponse, x_scale: XScale) -> Option<Rect> {
        let rect = plot.plot_rect;
        let ((x0, x1), (y0, y1)) = plot.axes;
        let screen_x = |x: f64| {
            let t = x_scale.normalize(x.clamp(x0.min(x1), x0.max(x1)), x0, x1);
            t.is_finite().then(|| rect.left() + t as f32 * rect.width())
        };
        let target = match self {
            Self::Vertical { x } => {
                let t = x_scale.normalize(x, x0, x1);
                if !(0.0..=1.0).contains(&t) {
                    return None;
                }
                let x = rect.left() + t as f32 * rect.width();
                Rect::from_x_y_ranges(x - REACH..=x + REACH, rect.y_range())
            }
            Self::Horizontal { y, from, to } => {
                let t = (y - y0) / (y1 - y0);
                if !(0.0..=1.0).contains(&t) {
                    return None;
                }
                let y = rect.bottom() - t as f32 * rect.height();
                let (left, right) = (screen_x(from)?, screen_x(to)?);
                Rect::from_x_y_ranges(left.min(right)..=left.max(right), y - REACH..=y + REACH)
            }
        };
        (target.width() > 0.0).then_some(target)
    }
}

/// Lay a target over each line, labelled with its `action` ("Open gain's
/// specification in Verify"), and return the index of the line the reader
/// clicked or chose the action on, this frame.
pub fn follow(
    ui: &mut Ui,
    plot: &PlotResponse,
    x_scale: XScale,
    lines: &[(RequirementLine, String)],
) -> Option<usize> {
    let mut followed = None;
    for (index, (line, action)) in lines.iter().enumerate() {
        let Some(target) = line.target(plot, x_scale) else {
            continue;
        };
        let response = ui
            .interact(
                target,
                plot.response.id.with(("requirement-line", index)),
                Sense::click(),
            )
            .on_hover_cursor(CursorIcon::PointingHand)
            .on_hover_text(action.as_str());
        response.widget_info(|| WidgetInfo::labeled(WidgetType::Link, true, action.as_str()));
        rspice_ui_kit::theme::paint_focus_ring(ui, &response, target);
        if response.clicked() {
            followed = Some(index);
        }
        egui::Popup::context_menu(&response).show(|ui| {
            if ui.button(action.as_str()).clicked() {
                followed = Some(index);
            }
        });
    }
    followed
}

/// A plot that draws requirement lines, and the specification one of them
/// asked to open this frame, by the measurement it bounds.
pub struct PlotWithRequirements {
    pub plot: PlotResponse,
    pub open_specification: Option<String>,
}

/// What a line that states the specification measured as `measurement`
/// offers.
pub fn specification_action(measurement: &str) -> String {
    format!("Open {measurement}'s specification in Verify")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_ui_kit::plot::{self, Axis, PlotSpec};

    /// Which line a primary click at the data point `at` follows, on a plot
    /// of 0..10 by 0..10 with a vertical line at 5 and a horizontal one at
    /// 2 across 0..4 of the abscissa.
    fn followed_at(at: (f64, f64)) -> Option<usize> {
        let lines = [
            (RequirementLine::Vertical { x: 5.0 }, "vertical".to_owned()),
            (
                RequirementLine::Horizontal {
                    y: 2.0,
                    from: 0.0,
                    to: 4.0,
                },
                "horizontal".to_owned(),
            ),
        ];
        let ctx = egui::Context::default();
        rspice_ui_kit::Theme::default().apply(&ctx);
        let mut cache = plot::DecimationCache::default();
        let mut target = None;
        let mut followed = None;
        for frame in 0..4 {
            let mut input = rspice_ui_kit::raster::display_input(egui::vec2(900.0, 500.0), 1.0);
            if let (1 | 2, Some(at)) = (frame, target) {
                input.events = vec![
                    egui::Event::PointerMoved(at),
                    egui::Event::PointerButton {
                        pos: at,
                        button: egui::PointerButton::Primary,
                        pressed: frame == 1,
                        modifiers: egui::Modifiers::NONE,
                    },
                ];
            }
            let _ = ctx.run_ui(input, |root| {
                egui::CentralPanel::default().show(root, |ui| {
                    let spec = PlotSpec::new(
                        Axis::linear(0.0, 10.0, ""),
                        XScale::Linear,
                        Axis::linear(0.0, 10.0, ""),
                    );
                    let response = plot::show(ui, &spec, &mut cache, None, None);
                    if frame == 0 {
                        let ((x0, x1), (y0, y1)) = response.axes;
                        let rect = response.plot_rect;
                        target = Some(egui::pos2(
                            rect.left() + ((at.0 - x0) / (x1 - x0)) as f32 * rect.width(),
                            rect.bottom() - ((at.1 - y0) / (y1 - y0)) as f32 * rect.height(),
                        ));
                    }
                    followed = followed.or(follow(ui, &response, XScale::Linear, &lines));
                });
            });
        }
        followed
    }

    /// A click on a line follows it, along its whole drawn length; a click
    /// beside it, or past the end of a segment, is the plot's.
    #[test]
    fn a_click_on_a_requirement_line_follows_it() {
        assert_eq!(followed_at((5.0, 8.0)), Some(0));
        assert_eq!(followed_at((1.0, 2.0)), Some(1));
        assert_eq!(followed_at((3.5, 2.0)), Some(1));
        assert_eq!(followed_at((7.0, 2.0)), None, "past the segment's end");
        assert_eq!(followed_at((6.0, 6.0)), None, "beside every line");
    }
}
