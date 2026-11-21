//! A context menu the keyboard opens too: Shift+F10 on the widget that has
//! the keyboard opens what a right click opens, as every desktop toolkit
//! does, so a plot's or a trace's menu needs no pointer.

use egui::{InnerResponse, Key, Modifiers, Popup, Response, SetOpenCommand, Ui};

/// Show `add_contents` as `response`'s context menu: opened by a right click
/// on it, or by Shift+F10 while it has the keyboard, then beside it.
pub(crate) fn context_menu<R>(
    response: &Response,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> Option<InnerResponse<R>> {
    let from_keys = response.has_focus()
        && response
            .ctx
            .input_mut(|input| input.consume_key(Modifiers::SHIFT, Key::F10));
    let id = Popup::default_response_id(response);
    let mut popup = Popup::context_menu(response).id(id);
    if from_keys {
        popup = popup.open_memory(Some(SetOpenCommand::Bool(true)));
    }
    if Popup::position_of_id(&response.ctx, id).is_none() {
        popup = popup.anchor(response);
    }
    popup.show(add_contents)
}
