use std::path::Path;

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
    let anchor = document.create_element("a").map_err(js_error_message)?;
    anchor
        .set_attribute("href", &url)
        .map_err(js_error_message)?;
    anchor
        .set_attribute("download", filename)
        .map_err(js_error_message)?;
    anchor
        .set_attribute("style", "display: none")
        .map_err(js_error_message)?;

    body.append_child(&anchor).map_err(js_error_message)?;
    let revoke_result = schedule_object_url_revoke(&window, url.clone());
    let click_result = anchor
        .dyn_ref::<web_sys::HtmlElement>()
        .ok_or_else(|| "Browser download anchor is not clickable".to_string())
        .map(|anchor| anchor.click());
    let remove_result = body.remove_child(&anchor).map(|_| ());

    click_result?;
    remove_result.map_err(js_error_message)?;
    revoke_result?;
    Ok(())
}

pub(crate) fn download_href(filename: &str, href: &str) -> Result<(), String> {
    use wasm_bindgen::JsCast as _;

    let filename = (!filename.trim().is_empty())
        .then(|| filename.trim())
        .unwrap_or("rspice-export.dat");
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

    body.append_child(&anchor).map_err(js_error_message)?;
    let click_result = anchor
        .dyn_ref::<web_sys::HtmlElement>()
        .ok_or_else(|| "Browser download anchor is not clickable".to_string())
        .map(|anchor| anchor.click());
    let remove_result = body.remove_child(&anchor).map(|_| ());

    click_result?;
    remove_result.map_err(js_error_message)?;
    Ok(())
}

fn schedule_object_url_revoke(window: &web_sys::Window, url: String) -> Result<(), String> {
    use wasm_bindgen::JsCast as _;

    let callback = wasm_bindgen::closure::Closure::<dyn FnMut()>::once(move || {
        if let Err(error) = web_sys::Url::revoke_object_url(&url) {
            web_sys::console::warn_1(&error);
        }
    });
    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(callback.as_ref().unchecked_ref(), 0)
        .map_err(js_error_message)?;
    callback.forget();
    Ok(())
}

fn js_error_message(error: wasm_bindgen::JsValue) -> String {
    error
        .as_string()
        .unwrap_or_else(|| "Browser download failed".to_string())
}
