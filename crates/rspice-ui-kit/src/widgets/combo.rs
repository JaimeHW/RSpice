//! egui's combo box, holding its value at the size every field holds one.

use egui::style::StyleModifier;
use egui::{AsIdSalt, FontId, InnerResponse, Ui, WidgetText};

use crate::theme;
use crate::tokens::Tokens;

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
        let inner = inner
            .popup_style(StyleModifier::new(move |style| {
                style.override_font_id = Some(list_font.clone());
            }))
            // The select's own chevron: egui's filled triangle marked every
            // combo box as the odd control out in a form of selects.
            .icon(|ui, rect, _visuals, _open| {
                super::select::paint_select_chevron(
                    ui.painter(),
                    rect.center(),
                    Tokens::get(ui.ctx()).color.text_dim,
                );
            });
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
