//! Model-qualification vectors at the worker boundary: the model editor's
//! browser session posts each vector here, so its solve runs in this image.

pub(crate) fn run_worker_request_value(
    request: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    let request = request.as_string().ok_or_else(|| {
        wasm_bindgen::JsValue::from_str("the qualification request is not a JSON string")
    })?;
    rspice_simulation::model_qualification::run_qualification_worker_request(&request)
        .map(|response| wasm_bindgen::JsValue::from_str(&response))
        .map_err(|error| wasm_bindgen::JsValue::from_str(&error))
}
