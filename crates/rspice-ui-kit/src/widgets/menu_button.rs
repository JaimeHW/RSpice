//! A menu button whose submenu row ends in the workbench's own chevron.
//!
//! egui marks a submenu row with U+23F5, a glyph no bundled face has, so its
//! own submenu button draws a missing-glyph box on every platform. Every menu
//! button in the application goes through [`menu_button`] instead of
//! [`egui::Ui::menu_button`], so every submenu draws the arrow the title bar's
//! menus draw.

use egui::containers::menu::{MenuButton, MenuState, SubMenu, is_in_menu};
use egui::{Atom, InnerResponse, IntoAtoms, Ui, Vec2};

use crate::panels::WorkbenchIcon;
use crate::tokens::Tokens;

/// The chevron's side, the same as the title bar's submenu rows.
const CHEVRON_SIDE: f32 = 12.0;

/// A button that opens a menu of `add_contents`: inside a menu, a submenu row
/// ending in a chevron that opens it beside the row; anywhere else, a button
/// that opens it below.
pub fn menu_button<'a, R>(
    ui: &mut Ui,
    atoms: impl IntoAtoms<'a>,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<Option<R>> {
    if !is_in_menu(ui) {
        let (response, inner) = MenuButton::new(atoms).ui(ui, add_contents);
        return InnerResponse::new(inner.map(|shown| shown.inner), response);
    }
    let row_id = ui.next_auto_id();
    let chevron_id = row_id.with("submenu chevron");
    let open = MenuState::from_ui(ui, |state, _| {
        state.open_item == Some(SubMenu::id_from_widget_id(row_id))
    });
    // An open submenu's row keeps the look of a pressed button, as egui's
    // own submenu row does.
    let inactive = ui.style().visuals.widgets.inactive;
    if open {
        ui.style_mut().visuals.widgets.inactive = ui.style().visuals.widgets.open;
    }
    let laid = egui::Button::new(atoms)
        .right_text(Atom::custom(chevron_id, Vec2::splat(CHEVRON_SIDE)))
        .atom_ui(ui);
    ui.style_mut().visuals.widgets.inactive = inactive;
    if let Some(rect) = laid.rect(chevron_id) {
        let faint = Tokens::get(ui.ctx()).color.text_faint;
        let color = if laid.response.enabled() {
            faint
        } else {
            faint.gamma_multiply(0.4)
        };
        WorkbenchIcon::ChevronRight.paint(ui.painter(), rect, color);
    }
    let inner = SubMenu::new().show(ui, &laid.response, add_contents);
    InnerResponse::new(inner.map(|shown| shown.inner), laid.response)
}

#[cfg(test)]
mod tests {
    use egui::containers::menu::SubMenuButton;
    use egui::{CentralPanel, Context, Popup, PopupCloseBehavior, RawInput, Shape};

    use super::*;

    /// Every glyph a menu row draws, from one frame with the row's menu open.
    fn glyphs_in_menu(row: impl Fn(&mut Ui)) -> Vec<char> {
        let ctx = Context::default();
        crate::fonts::install(&ctx);
        let mut glyphs = Vec::new();
        for _ in 0..3 {
            let output = ctx.run_ui(RawInput::default(), |ui| {
                CentralPanel::default().show(ui, |ui| {
                    let anchor = ui.button("Menu");
                    Popup::menu(&anchor)
                        .open(true)
                        .close_behavior(PopupCloseBehavior::IgnoreClicks)
                        .show(|ui| row(ui));
                });
            });
            glyphs.clear();
            for clipped in output.shapes {
                if let Shape::Text(text) = clipped.shape {
                    glyphs.extend(text.galley.text().chars());
                }
            }
        }
        glyphs
    }

    #[test]
    fn a_submenu_row_letters_only_characters_the_bundled_faces_have() {
        let glyphs = glyphs_in_menu(|ui| {
            menu_button(ui, "Code actions", |ui| ui.label("Format document"));
        });
        assert!(
            glyphs.iter().collect::<String>().contains("Code actions"),
            "the row draws its label: {glyphs:?}"
        );
        let missing: Vec<char> = glyphs
            .iter()
            .copied()
            .filter(|c| rspice_fonts::needs_a_glyph(*c) && !rspice_fonts::embedded_covers(*c))
            .collect();
        assert!(
            missing.is_empty(),
            "a submenu row draws characters no bundled face has, as boxes: {missing:?}"
        );
    }

    #[test]
    fn egui_own_submenu_row_draws_an_arrow_no_bundled_face_has() {
        // The reason the application never uses egui's submenu button: if a
        // bundled face gains the arrow, this fails and the helper can go.
        assert!(!rspice_fonts::embedded_covers(
            SubMenuButton::RIGHT_ARROW.chars().next().unwrap()
        ));
        let glyphs = glyphs_in_menu(|ui| {
            SubMenuButton::new("Code actions").ui(ui, |ui| ui.label("Format document"));
        });
        assert!(glyphs.contains(&'\u{23F5}'), "{glyphs:?}");
    }
}
