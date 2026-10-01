//! Typed hardcopy request and response transfer at the worker boundary.

use rspice_hardcopy::sources::MAX_WORKER_SNAPSHOT_BYTES;
use rspice_hardcopy::worker::{HardcopyWorkerRequest, MAX_REQUEST_BUFFERS, execute_request};

pub(crate) fn run_worker_request_value(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    use js_sys::{Array, Object, Reflect, Uint8Array};
    use wasm_bindgen::JsCast as _;

    let metadata = Reflect::get(&value, &wasm_bindgen::JsValue::from_str("metadata"))?;
    let request: HardcopyWorkerRequest = serde_wasm_bindgen::from_value(metadata)
        .map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))?;
    let transferred = Reflect::get(&value, &wasm_bindgen::JsValue::from_str("buffers"))?
        .dyn_into::<Array>()
        .map_err(|_| {
            wasm_bindgen::JsValue::from_str("Hardcopy worker buffers must be an array.")
        })?;
    if transferred.length() as usize > MAX_REQUEST_BUFFERS {
        return Err(wasm_bindgen::JsValue::from_str(
            "Hardcopy worker request has too many binary buffers.",
        ));
    }
    let mut buffers = Vec::with_capacity(transferred.length() as usize);
    let mut aggregate_bytes = 0usize;
    for index in 0..transferred.length() {
        let view = transferred
            .get(index)
            .dyn_into::<Uint8Array>()
            .map_err(|_| {
                wasm_bindgen::JsValue::from_str(
                    "Hardcopy worker request buffers must be Uint8Array values.",
                )
            })?;
        let byte_length = view.byte_length() as usize;
        aggregate_bytes = aggregate_bytes.checked_add(byte_length).ok_or_else(|| {
            wasm_bindgen::JsValue::from_str("Hardcopy worker request buffer size overflowed.")
        })?;
        if byte_length > MAX_WORKER_SNAPSHOT_BYTES || aggregate_bytes > MAX_WORKER_SNAPSHOT_BYTES {
            return Err(wasm_bindgen::JsValue::from_str(
                "Hardcopy worker request exceeds its binary transport budget.",
            ));
        }
        buffers.push(view.to_vec());
    }
    let response = execute_request(request, buffers)
        .map_err(|error| wasm_bindgen::JsValue::from_str(&error))?;
    let object = Object::new();
    Reflect::set(
        &object,
        &wasm_bindgen::JsValue::from_str("protocolVersion"),
        &wasm_bindgen::JsValue::from_f64(f64::from(response.protocol_version())),
    )?;
    Reflect::set(
        &object,
        &wasm_bindgen::JsValue::from_str("id"),
        &wasm_bindgen::JsValue::from_f64(f64::from(response.id())),
    )?;
    Reflect::set(
        &object,
        &wasm_bindgen::JsValue::from_str("epoch"),
        &wasm_bindgen::JsValue::from_str(response.epoch()),
    )?;
    Reflect::set(
        &object,
        &wasm_bindgen::JsValue::from_str("generation"),
        &wasm_bindgen::JsValue::from_str(response.generation()),
    )?;
    Reflect::set(
        &object,
        &wasm_bindgen::JsValue::from_str("operation"),
        &wasm_bindgen::JsValue::from_str(response.operation().as_str()),
    )?;
    let output = Array::new();
    for buffer in response.into_buffers() {
        output.push(&Uint8Array::from(buffer.as_slice()));
    }
    Reflect::set(
        &object,
        &wasm_bindgen::JsValue::from_str("buffers"),
        &output,
    )?;
    Ok(object.into())
}
