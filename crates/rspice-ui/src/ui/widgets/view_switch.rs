//! A compact switch between a few ways of looking at one thing.
//!
//! [`segmented`](super::segmented) is a form control: control height, a border
//! of its own, an accent underline, at home in a preference row. A plot's strip
//! needs something smaller and quieter — three words that choose the span a
//! waveform is seen through, sitting beside readouts in a 24 point row, where a
//! control-height box with a yellow selection outweighs the data it is about.
//!
//! So this is a track and a thumb. The track is an inset well as tall as the
//! text needs; the chosen option sits on a raised thumb that slides to it, and
//! nothing in it is accent-coloured, because on an instrument the accent
//! belongs to the trace. An option that cannot be chosen stays in place, faint,
//! and says why under the pointer: a switch whose options come and go makes the
//! reader relearn where the others are.

use egui::{Rect, Sense, Stroke, Ui, WidgetInfo, WidgetType, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

/// Height of the track. Four points shorter than the strips it sits in, so it
/// reads as an object in the row rather than as the row.
const TRACK_HEIGHT: f32 = 20.0;
/// Inset of the thumb from the edges of the track.
const THUMB_INSET: f32 = 2.0;
const OPTION_PADDING: f32 = 10.0;
const OPTION_MINIMUM_WIDTH: f32 = 44.0;
const TRACK_RADIUS: u8 = 5;
const THUMB_RADIUS: u8 = 3;
/// How long the thumb takes to reach a newly chosen option.
const SLIDE_SECONDS: f32 = 0.12;

/// One option of a [`view_switch`].
#[derive(Debug, Clone, Copy)]
pub(crate) struct ViewOption<'a> {
    /// The word on the option.
    pub label: &'a str,
    /// Why the option cannot be chosen right now, when it cannot.
    pub unavailable: Option<&'a str>,
}

/// Paint the switch and report the option the reader chose, if they chose a
/// different one.
pub(crate) fn view_switch(
    ui: &mut Ui,
    id_salt: &'static str,
    options: &[ViewOption<'_>],
    selected: usize,
) -> Option<usize> {
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let chosen_font = theme::sans(tokens::FS_0, FontWeight::Medium);
    let widths = options
        .iter()
        .map(|option| {
            let text = ui
                .painter()
                .layout_no_wrap(option.label.to_owned(), chosen_font.clone(), t.color.text)
                .size()
                .x;
            (text + 2.0 * OPTION_PADDING).max(OPTION_MINIMUM_WIDTH)
        })
        .collect::<Vec<_>>();
    let (track, _) = ui.allocate_exact_size(
        vec2(widths.iter().sum::<f32>() + 2.0 * THUMB_INSET, TRACK_HEIGHT),
        Sense::hover(),
    );
    ui.painter().rect(
        track,
        TRACK_RADIUS,
        t.color.bg_inset,
        Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );

    let mut left = track.left() + THUMB_INSET;
    let cells = widths
        .iter()
        .map(|width| {
            let cell = Rect::from_min_max(
                egui::pos2(left, track.top() + THUMB_INSET),
                egui::pos2(left + width, track.bottom() - THUMB_INSET),
            );
            left = cell.right();
            cell
        })
        .collect::<Vec<_>>();

    // The thumb goes down before the words, and travels: its left edge and its
    // width are both eased, so moving between a short word and a long one is
    // one motion rather than a jump and a stretch.
    if let Some(target) = cells.get(selected) {
        let id = ui.make_persistent_id((id_salt, "thumb"));
        let x = ui
            .ctx()
            .animate_value_with_time(id.with("x"), target.left(), SLIDE_SECONDS);
        let width = ui
            .ctx()
            .animate_value_with_time(id.with("w"), target.width(), SLIDE_SECONDS);
        let thumb = Rect::from_min_size(egui::pos2(x, target.top()), vec2(width, target.height()));
        ui.painter().rect(
            thumb,
            THUMB_RADIUS,
            t.color.bg_active,
            Stroke::new(1.0, t.color.border_strong),
            egui::StrokeKind::Inside,
        );
    }

    let mut chosen = None;
    for (index, (option, cell)) in options.iter().zip(&cells).enumerate() {
        let available = option.unavailable.is_none();
        let active = index == selected;
        let response = ui.interact(
            *cell,
            ui.make_persistent_id((id_salt, index)),
            if available {
                Sense::click()
            } else {
                Sense::hover()
            },
        );
        response.widget_info(|| {
            WidgetInfo::selected(WidgetType::RadioButton, available, active, option.label)
        });
        let color = if !available {
            t.color.text_faint.gamma_multiply(0.55)
        } else if active || response.hovered() {
            t.color.text
        } else {
            t.color.text_dim
        };
        ui.painter().text(
            cell.center(),
            egui::Align2::CENTER_CENTER,
            option.label,
            if active {
                chosen_font.clone()
            } else {
                font.clone()
            },
            color,
        );
        theme::paint_focus_ring(ui, &response, *cell);

        let keyed = if response.has_focus() {
            ui.input(|input| {
                if input.key_pressed(egui::Key::ArrowLeft) {
                    neighbour(options, index, false)
                } else if input.key_pressed(egui::Key::ArrowRight) {
                    neighbour(options, index, true)
                } else {
                    None
                }
            })
        } else {
            None
        };
        if let Some(next) = keyed {
            let next_id = ui.make_persistent_id((id_salt, next));
            ui.memory_mut(|memory| memory.request_focus(next_id));
            chosen = Some(next);
        } else if available && response.clicked() && !active {
            chosen = Some(index);
        }
        if let Some(reason) = option.unavailable {
            response.on_hover_text(reason);
        }
    }
    chosen.filter(|index| *index != selected)
}

/// The next option an arrow key lands on: the nearest one in that direction
/// that can be chosen, passing over the ones that cannot.
fn neighbour(options: &[ViewOption<'_>], from: usize, forward: bool) -> Option<usize> {
    let mut at = from;
    loop {
        at = if forward {
            at.checked_add(1).filter(|next| *next < options.len())?
        } else {
            at.checked_sub(1)?
        };
        if options[at].unavailable.is_none() {
            return Some(at);
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    const OPTIONS: [ViewOption<'static>; 3] = [
        ViewOption {
            label: "Fit",
            unavailable: None,
        },
        ViewOption {
            label: "Period",
            unavailable: Some("This family has no fundamental period"),
        },
        ViewOption {
            label: "Transient",
            unavailable: None,
        },
    ];

    /// Every option is announced as one of a set, with which cannot be chosen.
    #[test]
    fn options_are_announced_with_their_availability() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(600.0, 200.0))),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let _ = view_switch(ui, "widgets.test.view", &OPTIONS, 2);
                });
            },
        );
        let mut nodes = output
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update")
            .nodes
            .into_iter()
            .filter(|(_, node)| node.role() == egui::accesskit::Role::RadioButton)
            .map(|(_, node)| {
                (
                    node.label().unwrap_or_default().to_owned(),
                    node.is_disabled(),
                )
            })
            .collect::<Vec<_>>();
        nodes.sort();
        assert_eq!(
            nodes,
            [
                ("Fit".to_owned(), false),
                ("Period".to_owned(), true),
                ("Transient".to_owned(), false),
            ]
        );
    }

    /// An arrow key passes over an option that cannot be chosen.
    #[test]
    fn arrows_skip_what_cannot_be_chosen() {
        assert_eq!(neighbour(&OPTIONS, 0, true), Some(2));
        assert_eq!(neighbour(&OPTIONS, 2, false), Some(0));
        assert_eq!(neighbour(&OPTIONS, 2, true), None);
        assert_eq!(neighbour(&OPTIONS, 0, false), None);
    }
}
