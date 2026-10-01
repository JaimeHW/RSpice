//! Model-source import transfer at the worker boundary.

use rspice_simulation::model_import::{
    BROWSER_MODEL_IMPORT_PROTOCOL_VERSION, BrowserModelImportWorkerMetadata,
    MAX_MODEL_IMPORT_RESPONSE_BYTES,
};

pub(crate) fn run_worker_request_value(
    request: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    use js_sys::{Array, Object, Reflect, Uint8Array};
    use wasm_bindgen::JsValue;

    let metadata = Reflect::get(&request, &JsValue::from_str("metadata"))?;
    let metadata: BrowserModelImportWorkerMetadata = serde_wasm_bindgen::from_value(metadata)
        .map_err(|error| JsValue::from_str(&format!("Invalid model-import metadata: {error}")))?;
    if metadata.protocol_version != BROWSER_MODEL_IMPORT_PROTOCOL_VERSION {
        return Err(JsValue::from_str(&format!(
            "Unsupported model-import protocol {}.",
            metadata.protocol_version
        )));
    }
    let buffers = Reflect::get(&request, &JsValue::from_str("buffers"))?;
    if !Array::is_array(&buffers) {
        return Err(JsValue::from_str(
            "Model-import request buffers are not an array.",
        ));
    }
    let buffers = Array::from(&buffers);
    if usize::try_from(buffers.length()).ok() != Some(metadata.file_names.len()) {
        return Err(JsValue::from_str(
            "Model-import file metadata does not match the transferred buffers.",
        ));
    }
    let mut total_bytes = 0usize;
    let mut files = Vec::with_capacity(metadata.file_names.len());
    for (index, name) in metadata.file_names.iter().enumerate() {
        let value =
            buffers.get(u32::try_from(index).map_err(|_| {
                JsValue::from_str("Model-import buffer index exceeds browser limits.")
            })?);
        let view = Uint8Array::new(&value);
        let length = usize::try_from(view.length())
            .map_err(|_| JsValue::from_str("Model-import buffer exceeds host limits."))?;
        total_bytes = total_bytes
            .checked_add(length)
            .ok_or_else(|| JsValue::from_str("Model-import buffer size overflowed."))?;
        if total_bytes > rspice_design::project_sources::MAX_PROJECT_SOURCE_BUNDLE_BYTES {
            return Err(JsValue::from_str(
                "Model-import source bundle exceeds the supported byte limit.",
            ));
        }
        let mut bytes = vec![0; length];
        view.copy_to(&mut bytes);
        files.push((name.clone(), bytes));
    }

    let (_, library) = rspice_simulation::model_import::import_project_source_bundle(
        &metadata.display_name,
        Some(&metadata.root_name),
        files,
        None,
    )
    .map_err(|error| JsValue::from_str(&error))?;
    let encoded = serde_json::to_vec(&library)
        .map_err(|error| JsValue::from_str(&format!("Could not encode parsed library: {error}")))?;
    if encoded.is_empty() || encoded.len() > MAX_MODEL_IMPORT_RESPONSE_BYTES {
        return Err(JsValue::from_str(
            "Parsed model library exceeds the browser worker response limit.",
        ));
    }
    let length = u32::try_from(encoded.len())
        .map_err(|_| JsValue::from_str("Parsed model library exceeds browser array limits."))?;
    let bytes = Uint8Array::new_with_length(length);
    bytes.copy_from(&encoded);
    let response = Object::new();
    Reflect::set(
        &response,
        &JsValue::from_str("protocolVersion"),
        &JsValue::from_f64(f64::from(BROWSER_MODEL_IMPORT_PROTOCOL_VERSION)),
    )?;
    Reflect::set(&response, &JsValue::from_str("libraryBytes"), &bytes)?;
    Ok(response.into())
}
