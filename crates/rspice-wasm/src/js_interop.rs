//! JavaScript value decoding and typed-array publication.
//!
//! Numeric result columns cross the boundary as `Float64Array` or
//! `Uint8Array`. Optional data is published as explicit `null`, never as an
//! omitted field or a plausible zero.

use rspice_core::execution::ResultWindow;
use rspice_core::execution::result_document::{AxisValues, SeriesWindowValues};
use serde::Serialize;
use wasm_bindgen::{JsCast, prelude::*};

use crate::DetailedWasmResult;
use crate::abort::{JsExecutionRequest, JsSharedCancellationControl};
use crate::errors::WasmError;
use crate::options::{MAX_TIMEOUT_MILLISECONDS, WasmExecutionOptions};

pub(crate) fn js_object_keys(value: &JsValue) -> Vec<String> {
    js_sys::Object::keys(value.unchecked_ref::<js_sys::Object>())
        .iter()
        .filter_map(|key| key.as_string())
        .collect()
}

pub(crate) fn optional_js_property(
    object: &JsValue,
    name: &str,
) -> DetailedWasmResult<Option<JsValue>> {
    let value = js_sys::Reflect::get(object, &JsValue::from_str(name)).map_err(|_| {
        Box::new(WasmError::invalid_argument(format!(
            "could not read execution option '{name}'"
        )))
    })?;
    // A missing JavaScript property reads as `undefined`. Preserve an
    // explicitly authored `null` so the field's type validator rejects it
    // instead of silently converting malformed input into a default.
    Ok((!value.is_undefined()).then_some(value))
}

pub(crate) fn shared_cancellation_from_js(
    value: JsValue,
) -> DetailedWasmResult<JsSharedCancellationControl> {
    if !value.is_object() || value.is_array() {
        return Err(Box::new(WasmError::invalid_argument(
            "cancellation must be an object".to_string(),
        )));
    }
    for key in js_object_keys(&value) {
        if !matches!(key.as_str(), "mechanism" | "view" | "index") {
            return Err(Box::new(WasmError::invalid_argument(format!(
                "unknown cancellation option '{key}'"
            ))));
        }
    }

    let mechanism = optional_js_property(&value, "mechanism")?
        .and_then(|value| value.as_string())
        .ok_or_else(|| {
            Box::new(WasmError::invalid_argument(
                "cancellation.mechanism must be the string 'sharedInt32'".to_string(),
            ))
        })?;
    if mechanism != "sharedInt32" {
        return Err(Box::new(WasmError::unsupported_cancellation(mechanism)));
    }

    let view = optional_js_property(&value, "view")?
        .and_then(|value| value.dyn_into::<js_sys::Int32Array>().ok())
        .ok_or_else(|| {
            Box::new(WasmError::invalid_argument(
                "cancellation.view must be an Int32Array over SharedArrayBuffer".to_string(),
            ))
        })?;
    let buffer = js_sys::Reflect::get(&view, &JsValue::from_str("buffer")).map_err(|_| {
        Box::new(WasmError::invalid_argument(
            "could not inspect cancellation.view.buffer".to_string(),
        ))
    })?;
    if !buffer.is_instance_of::<js_sys::SharedArrayBuffer>() {
        return Err(Box::new(WasmError::invalid_argument(
            "cancellation.view must use SharedArrayBuffer, not ArrayBuffer".to_string(),
        )));
    }

    let index = match optional_js_property(&value, "index")? {
        None => 0,
        Some(value) => {
            let number = value.as_f64().filter(|number| {
                number.is_finite()
                    && *number >= 0.0
                    && number.fract() == 0.0
                    && *number <= f64::from(u32::MAX)
            });
            number.ok_or_else(|| {
                Box::new(WasmError::invalid_argument(
                    "cancellation.index must be a non-negative integer".to_string(),
                ))
            })? as u32
        }
    };
    if index >= view.length() {
        return Err(Box::new(WasmError::invalid_argument(format!(
            "cancellation.index {index} is outside a view of length {}",
            view.length()
        ))));
    }
    js_sys::Atomics::load(&view, index).map_err(|_| {
        Box::new(WasmError::invalid_argument(
            "cancellation.view does not support Atomics.load".to_string(),
        ))
    })?;

    Ok(JsSharedCancellationControl { view, index })
}

pub(crate) fn execution_request_from_js(value: JsValue) -> DetailedWasmResult<JsExecutionRequest> {
    if value.is_undefined() || value.is_null() {
        return Ok(JsExecutionRequest {
            options: WasmExecutionOptions::default(),
            timeout_milliseconds: None,
            cancellation: None,
        });
    }
    if !value.is_object() || value.is_array() {
        return Err(Box::new(WasmError::invalid_argument(
            "execution options must be an object".to_string(),
        )));
    }
    for key in js_object_keys(&value) {
        if !matches!(
            key.as_str(),
            "resourceLimits" | "transientCompression" | "timeoutMilliseconds" | "cancellation"
        ) {
            return Err(Box::new(WasmError::invalid_argument(format!(
                "unknown execution option '{key}'"
            ))));
        }
    }

    let serializable = js_sys::Object::new();
    for name in ["resourceLimits", "transientCompression"] {
        if let Some(field) = optional_js_property(&value, name)? {
            js_sys::Reflect::set(&serializable, &JsValue::from_str(name), &field).map_err(
                |_| {
                    Box::new(WasmError::invalid_argument(format!(
                        "could not decode execution option '{name}'"
                    )))
                },
            )?;
        }
    }
    let options: WasmExecutionOptions = serde_wasm_bindgen::from_value(serializable.into())
        .map_err(|error| {
            Box::new(WasmError::invalid_argument(format!(
                "invalid execution options: {error}"
            )))
        })?;
    let timeout_milliseconds = optional_js_property(&value, "timeoutMilliseconds")?
        .map(|value| {
            let number = value.as_f64().filter(|number| {
                number.is_finite()
                    && *number >= 0.0
                    && number.fract() == 0.0
                    && *number <= f64::from(MAX_TIMEOUT_MILLISECONDS)
            });
            number.map(|number| number as u32).ok_or_else(|| {
                Box::new(WasmError::invalid_argument(format!(
                    "timeoutMilliseconds must be an integer from 0 through {MAX_TIMEOUT_MILLISECONDS}"
                )))
            })
        })
        .transpose()?;
    let cancellation = optional_js_property(&value, "cancellation")?
        .map(shared_cancellation_from_js)
        .transpose()?;
    Ok(JsExecutionRequest {
        options,
        timeout_milliseconds,
        cancellation,
    })
}

pub(crate) fn serialize_to_js<T: Serialize>(value: &T) -> Result<JsValue, JsValue> {
    serde_wasm_bindgen::to_value(value)
        .map_err(|err| JsValue::from_str(&format!("serialization failed: {err}")))
}

pub(crate) fn js_property(object: &JsValue, name: &str) -> Result<JsValue, JsValue> {
    js_sys::Reflect::get(object, &JsValue::from_str(name)).map_err(|_| {
        JsValue::from_str(&format!(
            "serialization failed: result property `{name}` is unavailable"
        ))
    })
}

pub(crate) fn set_float64_array(
    object: &JsValue,
    name: &str,
    values: &[f64],
) -> Result<(), JsValue> {
    let values = js_sys::Float64Array::from(values);
    js_sys::Reflect::set(object, &JsValue::from_str(name), values.as_ref())
        .map(|_| ())
        .map_err(|_| {
            JsValue::from_str(&format!(
                "serialization failed: cannot publish result typed array `{name}`"
            ))
        })
}

pub(crate) fn js_array_property(object: &JsValue, name: &str) -> Result<js_sys::Array, JsValue> {
    js_property(object, name)?
        .dyn_into::<js_sys::Array>()
        .map_err(|_| {
            JsValue::from_str(&format!(
                "serialization failed: result property `{name}` is not an array"
            ))
        })
}

pub(crate) fn set_uint8_array(object: &JsValue, name: &str, values: &[u8]) -> Result<(), JsValue> {
    let values = js_sys::Uint8Array::from(values);
    js_sys::Reflect::set(object, &JsValue::from_str(name), values.as_ref())
        .map(|_| ())
        .map_err(|_| {
            JsValue::from_str(&format!(
                "serialization failed: cannot publish typed validity array `{name}`"
            ))
        })
}

/// Publish one bounded result window with typed numeric columns.
///
/// The window is serialized from the core `ResultWindow`, then every numeric
/// column is replaced by a typed array. Each signal keeps its validity mask:
/// a zero entry is an explicitly unavailable sample, so the aligned numeric
/// placeholder must never be read as a measurement.
pub(crate) fn serialize_result_window_to_js(window: &ResultWindow) -> Result<JsValue, JsValue> {
    let serializer = serde_wasm_bindgen::Serializer::new().serialize_missing_as_null(true);
    let serialized = window
        .serialize(&serializer)
        .map_err(|error| JsValue::from_str(&format!("serialization failed: {error}")))?;

    // `analysisId` and `end` are the flat spellings the browser contract has
    // always exposed; the core document keeps the structured identity and a
    // point count, and both are preserved alongside them.
    set_string(&serialized, "analysisId", &window.analysis.tag())?;
    set_usize(
        &serialized,
        "end",
        window.start.saturating_add(window.count),
    )?;

    let axes = js_array_property(&serialized, "axes")?;
    for (index, axis) in window.axes.iter().enumerate() {
        let object = axes.get(u32::try_from(index).map_err(|_| {
            JsValue::from_str("serialization failed: result axis index exceeds JavaScript bounds")
        })?);
        match &axis.values {
            AxisValues::Real { values } => set_float64_array(&object, "values", values)?,
            AxisValues::Integer { values } => {
                set_float64_array(&object, "values", &exact_axis_integers(values, &axis.name)?)?;
            }
        }
    }

    let signals = js_array_property(&serialized, "signals")?;
    for (index, signal) in window.signals.iter().enumerate() {
        let object = signals.get(u32::try_from(index).map_err(|_| {
            JsValue::from_str("serialization failed: result signal index exceeds JavaScript bounds")
        })?);
        let values = js_property(&object, "values")?;
        match &signal.values {
            SeriesWindowValues::Real {
                values: samples,
                validity,
            } => {
                set_float64_array(&values, "values", samples)?;
                set_uint8_array(&values, "validity", validity)?;
            }
            SeriesWindowValues::Complex {
                real,
                imaginary,
                validity,
            } => {
                set_float64_array(&values, "real", real)?;
                set_float64_array(&values, "imaginary", imaginary)?;
                set_uint8_array(&values, "validity", validity)?;
            }
            SeriesWindowValues::Logic { validity, .. } => {
                // Logic samples are a state/strength pair, not a number; only
                // the mask is a numeric column.
                set_uint8_array(&values, "validity", validity)?;
            }
        }
    }
    Ok(serialized)
}

/// Widen an integer axis to the JavaScript number domain, refusing any value
/// that would silently lose precision.
fn exact_axis_integers(values: &[i64], axis: &str) -> Result<Vec<f64>, JsValue> {
    const EXACT: i64 = 1 << 53;
    let mut widened = Vec::new();
    widened.try_reserve_exact(values.len()).map_err(|_| {
        JsValue::from_str("serialization failed: cannot allocate an integer axis column")
    })?;
    for value in values {
        if !(-EXACT..=EXACT).contains(value) {
            return Err(JsValue::from_str(&format!(
                "serialization failed: axis `{axis}` coordinate {value} is not exact as a JavaScript number"
            )));
        }
        #[allow(clippy::cast_precision_loss)]
        // The bound above is exactly the exactly-representable integer range.
        widened.push(*value as f64);
    }
    Ok(widened)
}

fn set_string(object: &JsValue, name: &str, value: &str) -> Result<(), JsValue> {
    js_sys::Reflect::set(object, &JsValue::from_str(name), &JsValue::from_str(value))
        .map(|_| ())
        .map_err(|_| {
            JsValue::from_str(&format!(
                "serialization failed: cannot publish result property `{name}`"
            ))
        })
}

fn set_usize(object: &JsValue, name: &str, value: usize) -> Result<(), JsValue> {
    let encoded = u32::try_from(value).map(f64::from).or_else(|_| {
        u64::try_from(value)
            .ok()
            .filter(|value| *value <= (1u64 << 53))
            .map(|value| value as f64)
            .ok_or_else(|| {
                JsValue::from_str(&format!(
                    "serialization failed: result property `{name}` is not exact as a JavaScript number"
                ))
            })
    })?;
    js_sys::Reflect::set(
        object,
        &JsValue::from_str(name),
        &JsValue::from_f64(encoded),
    )
    .map(|_| ())
    .map_err(|_| {
        JsValue::from_str(&format!(
            "serialization failed: cannot publish result property `{name}`"
        ))
    })
}
