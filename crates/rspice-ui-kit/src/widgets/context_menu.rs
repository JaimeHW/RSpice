//! A context menu a key can raise as well as a right-click.

use egui::{Id, Popup, Pos2, Response};

fn raised_at_id(popup_id: Id) -> Id {
    popup_id.with("raised at")
}

/// Open `response`'s context menu at `position`, for a key that raises it
/// where no pointer says where.
pub fn raise_context_menu(response: &Response, position: Pos2) {
    let popup_id = Popup::default_response_id(response);
    response
        .ctx
        .data_mut(|data| data.insert_temp(raised_at_id(popup_id), position));
    Popup::open_id(&response.ctx, popup_id);
}

/// `response`'s context menu: where the keyboard raised it if a key did, and
/// at the pointer after a right-click.
pub fn context_menu_popup(response: &Response) -> Popup<'_> {
    let id = raised_at_id(Popup::default_response_id(response));
    if response.secondary_clicked() {
        response.ctx.data_mut(|data| data.remove_temp::<Pos2>(id));
    }
    let popup = Popup::context_menu(response);
    match response.ctx.data(|data| data.get_temp::<Pos2>(id)) {
        Some(position) => popup.at_position(position),
        None => popup,
    }
}
