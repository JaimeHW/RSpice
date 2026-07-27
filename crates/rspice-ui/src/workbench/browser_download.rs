use std::path::Path;

// Synthetic anchor downloads expose no standard completion event. Keep the
// Blob URL alive well beyond the navigation task so every supported engine can
// consume it, then release it to bound memory use.
const DOWNLOAD_OBJECT_URL_REVOKE_DELAY_MS: i32 = 60_000;

pub(crate) fn download_text_file(path: &Path, contents: &str) -> Result<(), String> {
    use wasm_bindgen::JsCast as _;

    let filename = path
        .file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.trim().is_empty())
        .unwrap_or("rspice-export.txt");
    let window = web_sys::window().ok_or_else(|| "Browser window is unavailable".to_string())?;
    let document = window
        .document()
        .ok_or_else(|| "Browser document is unavailable".to_string())?;
    let body = document
        .body()
        .ok_or_else(|| "Browser document body is unavailable".to_string())?;

    let parts = js_sys::Array::new();
    parts.push(&wasm_bindgen::JsValue::from_str(contents));
    let options = web_sys::BlobPropertyBag::new();
    options.set_type("text/plain;charset=utf-8");
    let blob = web_sys::Blob::new_with_str_sequence_and_options(&parts, &options)
        .map_err(js_error_message)?;
    let url = web_sys::Url::create_object_url_with_blob(&blob).map_err(js_error_message)?;
    let url = ObjectUrlLease::new(url);
    let anchor = document.create_element("a").map_err(js_error_message)?;
    anchor
        .set_attribute("href", url.as_str())
        .map_err(js_error_message)?;
    anchor
        .set_attribute("download", filename)
        .map_err(js_error_message)?;
    anchor
        .set_attribute("style", "display: none")
        .map_err(js_error_message)?;

    let clickable = anchor
        .dyn_ref::<web_sys::HtmlElement>()
        .ok_or_else(|| "Browser download anchor is not clickable".to_string())?;
    if let Err(error) = body.append_child(&anchor) {
        let _ = body.remove_child(&anchor);
        return Err(js_error_message(error));
    }
    clickable.click();
    if let Err(error) = body.remove_child(&anchor) {
        warn_download_cleanup("remove temporary download anchor", &js_error_message(error));
    }
    url.schedule_revoke_after_handoff(&window);
    Ok(())
}

/// Hand arbitrary bytes to the browser download manager with exact file-type
/// metadata. A successful return means the user agent accepted the synthetic
/// click; it does not claim that the user kept or finished the download.
pub(crate) fn download_bytes_file(
    path: &Path,
    contents: &[u8],
    mime_type: &str,
) -> Result<(), String> {
    use wasm_bindgen::JsCast as _;

    let filename = path
        .file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.trim().is_empty())
        .unwrap_or("rspice-export.dat");
    let mime_type = mime_type.trim();
    if mime_type.is_empty() || mime_type.chars().any(char::is_control) {
        return Err("Browser download MIME type is invalid".to_string());
    }
    let window = web_sys::window().ok_or_else(|| "Browser window is unavailable".to_string())?;
    let document = window
        .document()
        .ok_or_else(|| "Browser document is unavailable".to_string())?;
    let body = document
        .body()
        .ok_or_else(|| "Browser document body is unavailable".to_string())?;

    let bytes = js_sys::Uint8Array::from(contents);
    let parts = js_sys::Array::new();
    parts.push(&bytes);
    let options = web_sys::BlobPropertyBag::new();
    options.set_type(mime_type);
    let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(&parts, &options)
        .map_err(js_error_message)?;
    let url = web_sys::Url::create_object_url_with_blob(&blob).map_err(js_error_message)?;
    let url = ObjectUrlLease::new(url);
    let anchor = document.create_element("a").map_err(js_error_message)?;
    anchor
        .set_attribute("href", url.as_str())
        .map_err(js_error_message)?;
    anchor
        .set_attribute("download", filename)
        .map_err(js_error_message)?;
    anchor
        .set_attribute("style", "display: none")
        .map_err(js_error_message)?;

    let clickable = anchor
        .dyn_ref::<web_sys::HtmlElement>()
        .ok_or_else(|| "Browser download anchor is not clickable".to_string())?;
    if let Err(error) = body.append_child(&anchor) {
        let _ = body.remove_child(&anchor);
        return Err(js_error_message(error));
    }
    clickable.click();
    if let Err(error) = body.remove_child(&anchor) {
        warn_download_cleanup("remove temporary download anchor", &js_error_message(error));
    }
    url.schedule_revoke_after_handoff(&window);
    Ok(())
}

pub(crate) fn download_href(filename: &str, href: &str) -> Result<(), String> {
    use wasm_bindgen::JsCast as _;

    let filename = if filename.trim().is_empty() {
        "rspice-export.dat"
    } else {
        filename.trim()
    };
    let window = web_sys::window().ok_or_else(|| "Browser window is unavailable".to_string())?;
    let document = window
        .document()
        .ok_or_else(|| "Browser document is unavailable".to_string())?;
    let body = document
        .body()
        .ok_or_else(|| "Browser document body is unavailable".to_string())?;
    let anchor = document.create_element("a").map_err(js_error_message)?;
    anchor
        .set_attribute("href", href)
        .map_err(js_error_message)?;
    anchor
        .set_attribute("download", filename)
        .map_err(js_error_message)?;
    anchor
        .set_attribute("style", "display: none")
        .map_err(js_error_message)?;

    let clickable = anchor
        .dyn_ref::<web_sys::HtmlElement>()
        .ok_or_else(|| "Browser download anchor is not clickable".to_string())?;
    if let Err(error) = body.append_child(&anchor) {
        let _ = body.remove_child(&anchor);
        return Err(js_error_message(error));
    }
    clickable.click();
    if let Err(error) = body.remove_child(&anchor) {
        warn_download_cleanup("remove temporary download anchor", &js_error_message(error));
    }
    Ok(())
}

struct ObjectUrlLease {
    url: Option<String>,
}

impl ObjectUrlLease {
    fn new(url: String) -> Self {
        Self { url: Some(url) }
    }

    fn as_str(&self) -> &str {
        self.url.as_deref().expect("object URL lease is active")
    }

    fn schedule_revoke_after_handoff(mut self, window: &web_sys::Window) {
        let url = self.url.take().expect("object URL lease is active");
        if let Err(error) = schedule_object_url_revoke(window, url) {
            // The click already happened. Retaining the Blob until document
            // unload is safer than revoking it early and invalidating a slow
            // browser download.
            warn_download_cleanup("schedule Blob URL revocation", &error);
        }
    }
}

impl Drop for ObjectUrlLease {
    fn drop(&mut self) {
        let Some(url) = self.url.take() else {
            return;
        };
        if let Err(error) = web_sys::Url::revoke_object_url(&url) {
            warn_download_cleanup(
                "revoke an unused Blob URL after setup failed",
                &js_error_message(error),
            );
        }
    }
}

fn schedule_object_url_revoke(window: &web_sys::Window, url: String) -> Result<(), String> {
    use wasm_bindgen::JsCast as _;

    let callback = wasm_bindgen::closure::Closure::<dyn FnMut()>::once(move || {
        if let Err(error) = web_sys::Url::revoke_object_url(&url) {
            web_sys::console::warn_1(&error);
        }
    });
    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            DOWNLOAD_OBJECT_URL_REVOKE_DELAY_MS,
        )
        .map_err(js_error_message)?;
    callback.forget();
    Ok(())
}

fn warn_download_cleanup(operation: &str, error: &str) {
    web_sys::console::warn_1(&wasm_bindgen::JsValue::from_str(&format!(
        "Browser download started, but could not {operation}: {error}"
    )));
}

fn js_error_message(error: wasm_bindgen::JsValue) -> String {
    error
        .as_string()
        .unwrap_or_else(|| "Browser download failed".to_string())
}
