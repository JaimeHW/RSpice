//! A browser's folder upload: every file of one folder the reader picks,
//! each by its path inside the folder, read in the background.
//!
//! A browser reaches no folder by path; the folder picker hands over the
//! files themselves, each named by its path from the picked folder, which is
//! how a library folder is embedded or read for a session there.

use js_sys::{Promise, Reflect, Uint8Array};
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use wasm_bindgen_futures::JsFuture;

/// The most files one picked folder may hold.
const MAX_FILES: usize = 20_000;
/// The most bytes they may hold together.
const MAX_BYTES: usize = 256 * 1024 * 1024;

/// The picked folder's name and its files, each by its path inside it.
pub type PickedFolder = (String, Vec<(String, Vec<u8>)>);

/// Ask the reader for a folder, read every file in it, and hand the result
/// to `on_complete`: none when they cancelled. The window is asked to draw
/// again once the read is done.
pub fn pick_folder(on_complete: impl FnOnce(Result<Option<PickedFolder>, String>) + 'static) {
    wasm_bindgen_futures::spawn_local(async move {
        let picked = read_picked_folder().await;
        on_complete(picked);
        super::file_import::request_browser_import_repaint();
    });
}

fn js_error(error: JsValue) -> String {
    error
        .as_string()
        .or_else(|| {
            Reflect::get(&error, &JsValue::from_str("message"))
                .ok()
                .and_then(|message| message.as_string())
        })
        .unwrap_or_else(|| "unknown browser error".to_owned())
}

async fn read_picked_folder() -> Result<Option<PickedFolder>, String> {
    let window =
        web_sys::window().ok_or_else(|| "The browser window is unavailable.".to_owned())?;
    let document = window
        .document()
        .ok_or_else(|| "The browser document is unavailable.".to_owned())?;
    let input = document
        .create_element("input")
        .map_err(js_error)?
        .dyn_into::<web_sys::HtmlInputElement>()
        .map_err(|_| "The browser could not make a folder picker.".to_owned())?;
    input.set_type("file");
    input.set_multiple(true);
    for (name, value) in [
        ("webkitdirectory", ""),
        ("directory", ""),
        ("style", "display:none"),
        ("aria-label", "Select a library folder"),
    ] {
        input.set_attribute(name, value).map_err(js_error)?;
    }
    if !Reflect::has(input.as_ref(), &JsValue::from_str("webkitdirectory")).map_err(js_error)? {
        return Err("This browser cannot pick a folder.".to_owned());
    }
    document
        .body()
        .ok_or_else(|| "The browser document has no body for the folder picker.".to_owned())?
        .append_child(&input)
        .map_err(js_error)?;
    let change_input = input.clone();
    let cancel_input = input.clone();
    let selection = Promise::new(&mut |resolve, _reject| {
        let change_resolve = resolve.clone();
        let change_input = change_input.clone();
        let on_change = Closure::once_into_js(move |_event: web_sys::Event| {
            let _ = change_resolve.call1(&JsValue::UNDEFINED, &JsValue::TRUE);
            change_input.remove();
        });
        let cancel_resolve = resolve.clone();
        let cancel_input = cancel_input.clone();
        let on_cancel = Closure::once_into_js(move |_event: web_sys::Event| {
            let _ = cancel_resolve.call1(&JsValue::UNDEFINED, &JsValue::FALSE);
            cancel_input.remove();
        });
        let _ = Reflect::set(input.as_ref(), &JsValue::from_str("onchange"), &on_change);
        let _ = Reflect::set(input.as_ref(), &JsValue::from_str("oncancel"), &on_cancel);
    });
    input.click();
    let selected = JsFuture::from(selection)
        .await
        .map_err(js_error)?
        .as_bool()
        .unwrap_or(false);
    let _ = Reflect::set(
        input.as_ref(),
        &JsValue::from_str("onchange"),
        &JsValue::NULL,
    );
    let _ = Reflect::set(
        input.as_ref(),
        &JsValue::from_str("oncancel"),
        &JsValue::NULL,
    );
    input.remove();
    if !selected {
        return Ok(None);
    }
    let files = input
        .files()
        .ok_or_else(|| "The browser returned no files for the folder.".to_owned())?;
    let count = usize::try_from(files.length()).unwrap_or(usize::MAX);
    if count == 0 {
        return Ok(None);
    }
    if count > MAX_FILES {
        return Err(format!(
            "The folder holds {count} files; at most {MAX_FILES} can be read."
        ));
    }
    let mut folder: Option<String> = None;
    let mut total = 0usize;
    let mut read = Vec::with_capacity(count);
    for index in 0..files.length() {
        let file = files
            .get(index)
            .ok_or_else(|| format!("The browser left out file {}.", index + 1))?;
        let path = Reflect::get(file.as_ref(), &JsValue::from_str("webkitRelativePath"))
            .map_err(js_error)?
            .as_string()
            .filter(|path| !path.is_empty())
            .ok_or_else(|| format!("The browser did not keep where {} is.", file.name()))?
            .replace('\\', "/");
        let (root, member) = path
            .split_once('/')
            .filter(|(root, member)| !root.is_empty() && !member.is_empty())
            .ok_or_else(|| format!("The browser named '{path}' outside a folder."))?;
        match &folder {
            Some(expected) if expected != root => {
                return Err(format!(
                    "The browser returned two folders, '{expected}' and '{root}'."
                ));
            }
            Some(_) => {}
            None => folder = Some(root.to_owned()),
        }
        let blob: &web_sys::Blob = file.as_ref();
        let buffer = JsFuture::from(blob.array_buffer())
            .await
            .map_err(js_error)?;
        let view = Uint8Array::new(&buffer);
        let length = usize::try_from(view.length()).unwrap_or(usize::MAX);
        total = total.saturating_add(length);
        if total > MAX_BYTES {
            return Err(format!(
                "The folder holds more than {MAX_BYTES} bytes, which is more than can be read."
            ));
        }
        let mut bytes = vec![0; length];
        view.copy_to(&mut bytes);
        read.push((member.to_owned(), bytes));
    }
    read.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(folder.map(|folder| (folder, read)))
}
