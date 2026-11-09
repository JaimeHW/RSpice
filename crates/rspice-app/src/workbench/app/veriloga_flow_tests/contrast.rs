//! A contrast theme, chosen in the application or set by Windows, through
//! whole frames.

use super::language_tools::op_amp;
use super::*;
use crate::ui::palette::{ContrastInks, INSTRUMENT_DARK, INSTRUMENT_HIGH_CONTRAST, high_contrast};
use crate::ui::tokens::Tokens;

const LIGHT_THEME: ContrastInks = ContrastInks {
    window: egui::Color32::from_rgb(0xff, 0xfa, 0xef),
    window_text: egui::Color32::from_rgb(0x3d, 0x3d, 0x3d),
    highlight: egui::Color32::from_rgb(0x90, 0x39, 0x09),
    highlight_text: egui::Color32::from_rgb(0xff, 0xf5, 0xe3),
    link: egui::Color32::from_rgb(0x1c, 0x5e, 0x75),
    disabled_text: egui::Color32::from_rgb(0x67, 0x67, 0x67),
};

fn panel_color(user: &User) -> egui::Color32 {
    Tokens::get(&user.ctx).color.bg_panel
}

#[test]
fn high_contrast_is_a_color_mode_and_covers_the_canvas() {
    let mut user = op_amp();
    assert_eq!(panel_color(&user), INSTRUMENT_DARK.bg_panel);

    user.app.state.workbench.theme.mode = crate::ui::Mode::HighContrast;
    user.app.state.workbench.theme.canvas_theme = crate::ui::EngineeringCanvasTheme::Light;
    user.settle();

    let tokens = Tokens::get(&user.ctx);
    assert_eq!(tokens.color, INSTRUMENT_HIGH_CONTRAST);
    assert_eq!(tokens.mode, crate::ui::Mode::Dark, "it draws a dark ground");
}

#[test]
fn a_windows_contrast_theme_takes_precedence_and_lets_go() {
    let mut user = op_amp();
    user.app.state.workbench.theme.mode = crate::ui::Mode::Dark;
    user.app.read_system_contrast = || Some(LIGHT_THEME);
    user.settle();

    let tokens = Tokens::get(&user.ctx);
    assert_eq!(tokens.color, high_contrast(LIGHT_THEME));
    assert_eq!(
        tokens.mode,
        crate::ui::Mode::Light,
        "it draws a light ground"
    );
    assert!(!user.ctx.global_style().visuals.dark_mode);

    user.app.read_system_contrast = || None;
    user.settle();
    assert_eq!(panel_color(&user), INSTRUMENT_DARK.bg_panel);
    assert!(user.ctx.global_style().visuals.dark_mode);
}
