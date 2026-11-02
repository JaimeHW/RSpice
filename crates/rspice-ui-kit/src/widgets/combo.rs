//! egui's combo box, holding its value at the size every field holds one.

use egui::style::StyleModifier;
use egui::{AsIdSalt, FontId, InnerResponse, Ui, WidgetText};

use crate::theme;

/// egui's combo box, drawing its value and its list at the field size.
///
/// egui sets a combo box's value and the choices in its list in the button
/// style, a size under the one every field and select holds its value at, so
/// a form that mixed the two read as ragged. An application builds its combo
/// boxes here, and a form reads at one size.
#[must_use = "a combo box is shown with `show_ui`"]
pub struct Combo {
    inner: egui::ComboBox,
    font: FontId,
}

impl Combo {
    /// A combo box with no label; its accessible name comes from the row.
    pub fn from_id_salt(id_salt: impl AsIdSalt) -> Self {
        Self {
            inner: egui::ComboBox::from_id_salt(id_salt),
            font: theme::field_sans(),
        }
    }

    /// The value shown while the list is closed.
    pub fn selected_text(mut self, text: impl Into<WidgetText>) -> Self {
        self.inner = self.inner.selected_text(text);
        self
    }

    /// The box's minimum width, its value's room included.
    pub fn width(mut self, width: f32) -> Self {
        self.inner = self.inner.width(width);
        self
    }

    /// Shorten a value too wide for the box instead of widening it.
    pub fn truncate(mut self) -> Self {
        self.inner = self.inner.truncate();
        self
    }

    /// When the open list closes.
    pub fn close_behavior(mut self, close_behavior: egui::PopupCloseBehavior) -> Self {
        self.inner = self.inner.close_behavior(close_behavior);
        self
    }

    /// Show the box; `menu_contents` builds the list when it is open.
    pub fn show_ui<R>(
        self,
        ui: &mut Ui,
        menu_contents: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<Option<R>> {
        let Self { inner, font } = self;
        let list_font = font.clone();
        let inner = inner.popup_style(StyleModifier::new(move |style| {
            style.override_font_id = Some(list_font.clone());
        }));
        // The closed value is laid out in `ui`'s style; set the field size on
        // it for this one call and give the caller's style back after, so the
        // box keeps the id it would have had and nothing beside it moves.
        let previous = ui.style().override_font_id.clone();
        ui.style_mut().override_font_id = Some(font);
        let response = inner.show_ui(ui, menu_contents);
        ui.style_mut().override_font_id = previous;
        response
    }
}

#[cfg(test)]
mod tests {
    use egui::{Context, Rect};

    use super::*;

    fn painted_texts(shapes: &[egui::epaint::ClippedShape]) -> Vec<(String, FontId)> {
        fn walk(shape: &egui::Shape, into: &mut Vec<(String, FontId)>) {
            match shape {
                egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, into)),
                egui::Shape::Text(text) => into.push((
                    text.galley.text().to_owned(),
                    text.galley.job.sections[0].format.font_id.clone(),
                )),
                _ => {}
            }
        }
        let mut texts = Vec::new();
        for clipped in shapes {
            walk(&clipped.shape, &mut texts);
        }
        texts
    }

    /// The value a closed combo box shows and every choice its open list
    /// offers read at the field size, and the caller's style is untouched.
    #[test]
    fn a_combo_box_holds_its_value_and_its_choices_at_the_field_size() {
        let ctx = Context::default();
        crate::Theme::default().apply(&ctx);
        ctx.all_styles_mut(|style| style.animation_time = 0.0);
        let input = |events: Vec<egui::Event>| egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 480.0),
            )),
            events,
            ..Default::default()
        };
        let id = egui::Id::new("combo-size");
        // One pass: what it painted, the box's rect, and the style the
        // caller's `Ui` holds after the box was shown.
        let pass = |ctx: &Context, events: Vec<egui::Event>| {
            let mut after = None;
            let mut rect = Rect::NOTHING;
            let output = ctx.run_ui(input(events), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let mut choice = 0;
                    rect = Combo::from_id_salt(id)
                        .selected_text("Orthogonal")
                        .width(160.0)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut choice, 0, "Orthogonal");
                            ui.selectable_value(&mut choice, 1, "Any angle");
                        })
                        .response
                        .rect;
                    after = Some(ui.style().override_font_id.clone());
                    ui.label("Snap");
                });
            });
            (output, rect, after)
        };
        for _ in 0..2 {
            let _ = pass(&ctx, Vec::new());
        }
        let (closed, rect, after) = pass(&ctx, Vec::new());
        let texts = painted_texts(&closed.shapes);
        let value = texts
            .iter()
            .find(|(text, _)| text == "Orthogonal")
            .expect("the closed value is painted");
        assert_eq!(value.1, theme::field_sans(), "the closed value");
        let label = texts
            .iter()
            .find(|(text, _)| text == "Snap")
            .expect("label");
        assert_eq!(
            label.1,
            egui::TextStyle::Body.resolve(&ctx.global_style()),
            "a label after the box keeps the caller's style"
        );
        assert_eq!(after, Some(None), "the caller's style is given back");

        let at = rect.center();
        let press = |pressed| egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        };
        let _ = pass(&ctx, vec![egui::Event::PointerMoved(at), press(true)]);
        let _ = pass(&ctx, vec![press(false)]);
        let _ = pass(&ctx, Vec::new());
        let (open, _, _) = pass(&ctx, Vec::new());
        let texts = painted_texts(&open.shapes);
        let choice = texts
            .iter()
            .find(|(text, _)| text == "Any angle")
            .expect("the open list paints its choices");
        assert_eq!(choice.1, theme::field_sans(), "a choice in the list");
    }
}
