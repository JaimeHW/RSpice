//! The text of a JavaScript failure, for the browser worker hosts.

use js_sys::Reflect;
use wasm_bindgen::JsValue;

/// A thrown string as is, else the thrown object's `message`.
pub(crate) fn js_error_message(error: JsValue) -> String {
    error
        .as_string()
        .or_else(|| {
            Reflect::get(&error, &JsValue::from_str("message"))
                .ok()
                .and_then(|message| message.as_string())
        })
        .unwrap_or_else(|| "unknown JavaScript error".to_owned())
}
