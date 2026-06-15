//! Status pills — a small state dot plus a mono label (engine status, ERC
//! state). The running state pulses.

use egui::{Response, Sense, Ui, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

/// Semantic state of a [`Pill`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PillState {
    /// Neutral / idle (faint dot).
    #[default]
    Idle,
    /// Healthy (ok dot).
    Ok,
    /// Running (warn dot, pulsing).
    Running,
    /// Failed (error dot).
    Error,
}

/// A status pill: 7 px dot + 11 px mono label.
pub struct Pill<'a> {
    state: PillState,
    label: &'a str,
}

impl<'a> Pill<'a> {
    /// Create a pill with a semantic state and label.
    pub fn new(state: PillState, label: &'a str) -> Self {
        Self { state, label }
    }

    /// Show the pill.
    pub fn show(self, ui: &mut Ui) -> Response {
        let t = Tokens::get(ui.ctx());
        let c = &t.color;

        let galley = ui.fonts(|f| {
            f.layout_no_wrap(
                self.label.to_owned(),
                theme::mono(tokens::FS_0, FontWeight::Regular),
                c.text_dim,
            )
        });
        let (rect, response) = ui.allocate_exact_size(
            vec2(7.0 + 5.0 + galley.size().x, galley.size().y.max(10.0)),
            Sense::hover(),
        );
        if !ui.is_rect_visible(rect) {
            return response;
        }

        let mut dot_color = match self.state {
            PillState::Idle => c.text_faint,
            PillState::Ok => c.ok,
            PillState::Running => c.warn,
            PillState::Error => c.err,
        };
        if self.state == PillState::Running {
            // 1 s pulse: dip to 35 % opacity at midpoint, like the design.
            let phase = (ui.input(|i| i.time) % 1.0) as f32;
            let wave = (phase * std::f32::consts::TAU).cos() * 0.5 + 0.5; // 1→0→1
            dot_color = dot_color.gamma_multiply(0.35 + 0.65 * wave);
            // Throttled: an uncapped request_repaint() here would re-render
            // the whole IDE at vsync rate for the entire simulation. 10 Hz
            // keeps the pulse smooth and bounds completion-poll latency.
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_millis(100));
        }

        let painter = ui.painter();
        painter.circle_filled(
            egui::pos2(rect.left() + 3.5, rect.center().y),
            3.5,
            dot_color,
        );
        painter.galley(
            egui::pos2(rect.left() + 12.0, rect.center().y - galley.size().y * 0.5),
            galley,
            c.text_dim,
        );
        response
    }
}
