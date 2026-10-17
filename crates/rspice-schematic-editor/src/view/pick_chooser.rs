//! The overlapping-object chooser: a long click on a point that two or more
//! objects claim lists them, and the chosen one becomes the selection
//! (D1 section 9). Rows are in pick order, so the first row is what a plain
//! click there takes.

use egui::{
    Align2, Context, Frame, Id, Key, LayerId, Margin, Modifiers, Order, PopupAnchor, Pos2, Rect,
    RectAlign, Sense, Stroke, Vec2,
};
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::Tokens;

use super::pointer_target::PointerTarget;

/// At most this many rows; with more candidates the last row counts the
/// rest and asks for a closer zoom.
pub const MAX_ROWS: usize = 10;
const ROW_HEIGHT: f32 = 22.0;
const TOUCH_ROW_HEIGHT: f32 = 44.0;
const KIND_WIDTH: f32 = 64.0;
const SIDE_PADDING: f32 = 8.0;
const MIN_WIDTH: f32 = 160.0;
const MAX_WIDTH: f32 = 280.0;
const FONT_SIZE: f32 = 11.0;
/// The chooser opens this far right of and below the click.
const ANCHOR_OFFSET: Vec2 = Vec2::new(8.0, 8.0);

/// One candidate: what kind of object it is and which one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChooserRow {
    pub target: PointerTarget,
    /// For example `R1`, `VOUT`, `wire 412 · VOUT` or `(120, 40)`.
    pub identity: String,
}

/// What the chooser asks of the selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChooserOutcome {
    /// Enter or a click: the selection becomes this object.
    Replace(PointerTarget),
    /// Shift+Enter or Shift+click: this object is toggled in the selection.
    Toggle(PointerTarget),
    /// Esc or a click outside: closed with no change.
    Dismiss,
}

/// The chooser's word for a kind of object.
pub const fn kind_label(target: PointerTarget) -> &'static str {
    match target {
        PointerTarget::Component(_) => "Instance",
        PointerTarget::Wire(_) => "Wire",
        PointerTarget::Bus(_) => "Bus",
        PointerTarget::BusTap(_) => "Tap",
        PointerTarget::Junction(_) => "Junction",
        PointerTarget::NetLabel(_) => "Net label",
        PointerTarget::DesignNote(_) => "Note",
        PointerTarget::DocumentationShape(_) => "Shape",
        PointerTarget::Probe(_) => "Probe",
    }
}

/// An open chooser.
#[derive(Debug, Clone, PartialEq)]
pub struct PickChooser {
    /// Where the long click was released.
    pub anchor: Pos2,
    pub rows: Vec<ChooserRow>,
    /// The focused row, which the canvas pre-highlights.
    pub focused: usize,
    /// Rows sized for a finger.
    pub touch: bool,
}

impl PickChooser {
    /// A chooser over `rows` with the first row focused, or none when fewer
    /// than two objects compete (that is an ordinary click).
    pub fn open(anchor: Pos2, rows: Vec<ChooserRow>, touch: bool) -> Option<Self> {
        (rows.len() >= 2).then_some(Self {
            anchor,
            rows,
            focused: 0,
            touch,
        })
    }

    /// The rows a person can choose: all of them up to ten, else the first
    /// nine, the tenth row counting the rest.
    pub fn choosable(&self) -> usize {
        if self.rows.len() > MAX_ROWS {
            MAX_ROWS - 1
        } else {
            self.rows.len()
        }
    }

    /// The text of the disabled last row, when there are more candidates
    /// than rows.
    pub fn overflow_text(&self) -> Option<String> {
        (self.rows.len() > MAX_ROWS)
            .then(|| format!("{} more · zoom in", self.rows.len() - self.choosable()))
    }

    /// The object the focused row names.
    pub fn focused_target(&self) -> Option<PointerTarget> {
        self.rows
            .get(self.focused.min(self.choosable().saturating_sub(1)))
            .map(|row| row.target)
    }

    /// Up and Down move the focus and wrap; Enter replaces the selection,
    /// Shift+Enter toggles; Esc dismisses. The keys are consumed so the
    /// canvas does not also act on them.
    pub fn handle_keys(&mut self, ctx: &Context) -> Option<ChooserOutcome> {
        let count = self.choosable();
        if count == 0 {
            return Some(ChooserOutcome::Dismiss);
        }
        let (down, up, toggle, replace, escape) = ctx.input_mut(|input| {
            (
                input.consume_key(Modifiers::NONE, Key::ArrowDown),
                input.consume_key(Modifiers::NONE, Key::ArrowUp),
                input.consume_key(Modifiers::SHIFT, Key::Enter),
                input.consume_key(Modifiers::NONE, Key::Enter),
                input.consume_key(Modifiers::NONE, Key::Escape),
            )
        });
        if down {
            self.focused = (self.focused + 1) % count;
        }
        if up {
            self.focused = (self.focused + count - 1) % count;
        }
        let target = self.focused_target()?;
        if escape {
            Some(ChooserOutcome::Dismiss)
        } else if toggle {
            Some(ChooserOutcome::Toggle(target))
        } else if replace {
            Some(ChooserOutcome::Replace(target))
        } else {
            None
        }
    }

    /// Show the chooser for this frame: keys first, then the rows. Hovering
    /// a row focuses it. Returns what the person chose, if they did.
    pub fn show(&mut self, ctx: &Context) -> Option<ChooserOutcome> {
        if let Some(outcome) = self.handle_keys(ctx) {
            return Some(outcome);
        }
        let t = Tokens::get(ctx);
        let kind_font = theme::sans(FONT_SIZE, FontWeight::Regular);
        let identity_font = theme::mono(FONT_SIZE, FontWeight::Regular);
        let row_height = if self.touch {
            TOUCH_ROW_HEIGHT
        } else {
            ROW_HEIGHT
        };
        let overflow = self.overflow_text();
        let identity_width = ctx.fonts_mut(|fonts| {
            self.rows[..self.choosable()]
                .iter()
                .map(|row| row.identity.as_str())
                .chain(overflow.as_deref())
                .map(|text| {
                    fonts
                        .layout_no_wrap(text.to_owned(), identity_font.clone(), t.color.text)
                        .size()
                        .x
                })
                .fold(0.0, f32::max)
        });
        let width =
            (SIDE_PADDING + KIND_WIDTH + identity_width + SIDE_PADDING).clamp(MIN_WIDTH, MAX_WIDTH);
        let id = Id::new("rspice.schematic.pick_chooser");
        let frame = Frame::new()
            .fill(t.color.bg_elevated)
            .stroke(Stroke::new(1.0, t.color.border_strong))
            .shadow(t.shadow())
            .inner_margin(Margin::symmetric(0, 4));
        let shift = ctx.input(|input| input.modifiers.shift);
        let mut open = true;
        let mut outcome = None;
        let mut focused = self.focused;
        let choosable = self.choosable();
        egui::Popup::new(
            id,
            ctx.clone(),
            PopupAnchor::Position(self.anchor + ANCHOR_OFFSET),
            LayerId::new(Order::Foreground, id),
        )
        .open_bool(&mut open)
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .align(RectAlign::BOTTOM_START)
        .align_alternatives(&[RectAlign::TOP_START, RectAlign::BOTTOM_END, RectAlign::TOP_END])
        .gap(0.0)
        .width(width)
        .frame(frame)
        .show(|ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            for (index, row) in self.rows[..choosable].iter().enumerate() {
                let (rect, response) =
                    ui.allocate_exact_size(Vec2::new(width, row_height), Sense::click());
                if response.hovered() {
                    focused = index;
                }
                if index == focused {
                    ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
                }
                paint_row(
                    ui.painter(),
                    rect,
                    kind_label(row.target),
                    &row.identity,
                    (&kind_font, &identity_font),
                    (t.color.text_dim, t.color.text),
                );
                if response.clicked() {
                    outcome = Some(if shift {
                        ChooserOutcome::Toggle(row.target)
                    } else {
                        ChooserOutcome::Replace(row.target)
                    });
                }
            }
            if let Some(text) = &overflow {
                let (rect, _) = ui.allocate_exact_size(Vec2::new(width, row_height), Sense::hover());
                paint_row(
                    ui.painter(),
                    rect,
                    "",
                    text,
                    (&kind_font, &identity_font),
                    (t.color.text_faint, t.color.text_faint),
                );
            }
        });
        self.focused = focused;
        outcome.or_else(|| (!open).then_some(ChooserOutcome::Dismiss))
    }
}

fn paint_row(
    painter: &egui::Painter,
    rect: Rect,
    kind: &str,
    identity: &str,
    (kind_font, identity_font): (&egui::FontId, &egui::FontId),
    (kind_color, identity_color): (egui::Color32, egui::Color32),
) {
    let left = rect.left() + SIDE_PADDING;
    painter.text(
        Pos2::new(left, rect.center().y),
        Align2::LEFT_CENTER,
        kind,
        kind_font.clone(),
        kind_color,
    );
    painter.text(
        Pos2::new(left + KIND_WIDTH, rect.center().y),
        Align2::LEFT_CENTER,
        identity,
        identity_font.clone(),
        identity_color,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_design_model::Point;

    fn rows(count: u64) -> Vec<ChooserRow> {
        (1..=count)
            .map(|id| ChooserRow {
                target: PointerTarget::Component(id),
                identity: format!("R{id}"),
            })
            .collect()
    }

    fn press(key: Key, modifiers: Modifiers) -> egui::RawInput {
        egui::RawInput {
            modifiers,
            events: vec![egui::Event::Key {
                key,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers,
            }],
            ..Default::default()
        }
    }

    /// One frame with `key` down, through the chooser's key handling.
    fn key(chooser: &mut PickChooser, key: Key, modifiers: Modifiers) -> Option<ChooserOutcome> {
        let ctx = Context::default();
        let mut outcome = None;
        let _ = ctx.run_ui(press(key, modifiers), |ui| {
            outcome = chooser.handle_keys(ui.ctx());
        });
        outcome
    }

    /// One candidate is an ordinary click; two or more open the chooser
    /// with the first, which a plain click takes, focused.
    #[test]
    fn the_chooser_opens_only_over_competing_objects() {
        assert_eq!(PickChooser::open(Pos2::ZERO, rows(1), false), None);
        let chooser = PickChooser::open(Pos2::ZERO, rows(2), false).expect("two compete");
        assert_eq!(chooser.focused_target(), Some(PointerTarget::Component(1)));
    }

    /// Down and Up wrap; Enter replaces the selection with the focused
    /// object, Shift+Enter toggles it, Esc closes with no change.
    #[test]
    fn keys_move_the_focus_with_wrap_and_choose() {
        let mut chooser = PickChooser::open(Pos2::ZERO, rows(3), false).expect("chooser");
        assert_eq!(key(&mut chooser, Key::ArrowUp, Modifiers::NONE), None);
        assert_eq!(chooser.focused_target(), Some(PointerTarget::Component(3)));
        assert_eq!(key(&mut chooser, Key::ArrowDown, Modifiers::NONE), None);
        assert_eq!(chooser.focused_target(), Some(PointerTarget::Component(1)));
        key(&mut chooser, Key::ArrowDown, Modifiers::NONE);
        assert_eq!(
            key(&mut chooser, Key::Enter, Modifiers::NONE),
            Some(ChooserOutcome::Replace(PointerTarget::Component(2)))
        );
        assert_eq!(
            key(&mut chooser, Key::Enter, Modifiers::SHIFT),
            Some(ChooserOutcome::Toggle(PointerTarget::Component(2)))
        );
        assert_eq!(
            key(&mut chooser, Key::Escape, Modifiers::NONE),
            Some(ChooserOutcome::Dismiss)
        );
    }

    /// With eleven or more candidates nine rows choose and the tenth counts
    /// the rest; the focus never lands on it.
    #[test]
    fn more_than_ten_candidates_list_nine_and_count_the_rest() {
        let mut chooser = PickChooser::open(Pos2::ZERO, rows(14), false).expect("chooser");
        assert_eq!(chooser.choosable(), 9);
        assert_eq!(chooser.overflow_text().as_deref(), Some("5 more · zoom in"));
        key(&mut chooser, Key::ArrowUp, Modifiers::NONE);
        assert_eq!(chooser.focused_target(), Some(PointerTarget::Component(9)));
        let ten = PickChooser::open(Pos2::ZERO, rows(10), false).expect("chooser");
        assert_eq!((ten.choosable(), ten.overflow_text()), (10, None));
    }

    /// Every kind has its word.
    #[test]
    fn kinds_read_as_the_design_names_them() {
        assert_eq!(kind_label(PointerTarget::Junction(Point::new(0, 0))), "Junction");
        assert_eq!(kind_label(PointerTarget::NetLabel(1)), "Net label");
        assert_eq!(kind_label(PointerTarget::BusTap(1)), "Tap");
    }
}
