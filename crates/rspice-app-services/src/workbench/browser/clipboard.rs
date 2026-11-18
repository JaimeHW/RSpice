//! The clipboard's text for a menu's Paste in the browser.
//!
//! A browser hands a page the clipboard on the keyboard's paste, as a paste
//! event. A menu's Paste has no such event: the page has to ask the
//! asynchronous clipboard for its text, which the browser may give, ask the
//! reader about first, or refuse: on a page that is not a secure context,
//! when the reader or a site setting declines, or in a browser without the
//! call. The answer arrives after the frame that asked.

use wasm_bindgen::{JsCast, JsValue};

/// Ask the browser's clipboard for its text. `answer` is given the text, or
/// `None` when the browser refused.
pub fn read_text(answer: impl FnOnce(Option<String>) + 'static) {
    let Some(promise) = read_text_promise() else {
        answer(None);
        return;
    };
    wasm_bindgen_futures::spawn_local(async move {
        let text = wasm_bindgen_futures::JsFuture::from(promise)
            .await
            .ok()
            .and_then(|text| text.as_string());
        answer(text);
    });
}

/// `navigator.clipboard.readText()`, where the page may call it. Each step
/// is looked up rather than bound, so a browser without one refuses instead
/// of throwing.
fn read_text_promise() -> Option<js_sys::Promise> {
    let window = web_sys::window()?;
    if !window.is_secure_context() {
        return None;
    }
    let navigator = js_sys::Reflect::get(&window, &JsValue::from_str("navigator")).ok()?;
    let clipboard = js_sys::Reflect::get(&navigator, &JsValue::from_str("clipboard")).ok()?;
    if clipboard.is_undefined() || clipboard.is_null() {
        return None;
    }
    let read = js_sys::Reflect::get(&clipboard, &JsValue::from_str("readText"))
        .ok()?
        .dyn_into::<js_sys::Function>()
        .ok()?;
    read.call0(&clipboard)
        .ok()?
        .dyn_into::<js_sys::Promise>()
        .ok()
}
