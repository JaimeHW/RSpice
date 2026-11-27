//! A context menu the keyboard opens too: Shift+F10 or the Menu key on the
//! widget that has the keyboard opens what a right click opens, as every
//! desktop toolkit does, so a plot's or a trace's menu needs no pointer.

use egui::{InnerResponse, Key, Modifiers, Popup, Response, SetOpenCommand, Ui};

/// Show `add_contents` as `response`'s context menu: opened by a right click
/// on it, or by Shift+F10 or the Menu key while it has the keyboard, then
/// beside it.
pub(crate) fn context_menu<R>(
    response: &Response,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> Option<InnerResponse<R>> {
    let from_keys = response.has_focus()
        && (response
            .ctx
            .input_mut(|input| input.consume_key(Modifiers::SHIFT, Key::F10))
            || rspice_ui_kit::input::menu_key_pressed(&response.ctx));
    let id = Popup::default_response_id(response);
    let mut popup = Popup::context_menu(response).id(id);
    if from_keys {
        popup = popup.open_memory(Some(SetOpenCommand::Bool(true)));
    }
    // A right click opens the menu at the pointer, which egui keeps from
    // the click on. Only a menu the keyboard opened has no such point and
    // stands beside the widget; the frame of the click has none stored yet
    // either, so the click itself must not count as one.
    if !response.secondary_clicked() && Popup::position_of_id(&response.ctx, id).is_none() {
        popup = popup.anchor(response);
    }
    popup.show(add_contents)
}

#[cfg(test)]
mod tests {
    use super::context_menu;

    /// A focused plot area and its menu, one frame: whether the menu drew.
    fn plot_frame(ctx: &egui::Context, focus: bool) -> bool {
        let mut shown = false;
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(800.0, 600.0),
                )),
                ..Default::default()
            },
            |ui| {
                let plot = ui.allocate_response(egui::vec2(400.0, 300.0), egui::Sense::click());
                if focus {
                    plot.request_focus();
                }
                shown = context_menu(&plot, |ui| ui.label("Autoscale")).is_some();
            },
        );
        shown
    }

    /// The Menu key on the focused plot opens the menu a right click opens,
    /// as Shift+F10 does; on a plot without the keyboard it opens nothing.
    #[test]
    fn the_menu_key_opens_a_focused_plots_menu() {
        let ctx = egui::Context::default();
        // The first frame lays the plot out; the second gives it the keyboard.
        plot_frame(&ctx, true);
        assert!(!plot_frame(&ctx, true), "nothing opened it yet");
        rspice_ui_kit::input::press_menu_key(&ctx);
        plot_frame(&ctx, true);
        assert!(
            plot_frame(&ctx, true),
            "the Menu key opened no menu on the focused plot"
        );

        let ctx = egui::Context::default();
        plot_frame(&ctx, false);
        rspice_ui_kit::input::press_menu_key(&ctx);
        plot_frame(&ctx, false);
        assert!(
            !plot_frame(&ctx, false),
            "the Menu key opened the menu of a plot without the keyboard"
        );
    }
}
