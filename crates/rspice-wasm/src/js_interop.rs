//! JavaScript value decoding and typed-array publication.
//!
//! Numeric result columns cross the boundary as `Float64Array`,
//! `Uint32Array`, or `Uint8Array`. Optional data is published as explicit
//! `null`, never as an omitted field or a plausible zero.

use serde::Serialize;
use wasm_bindgen::{JsCast, prelude::*};

use crate::DetailedWasmResult;
use crate::abort::{JsExecutionRequest, JsSharedCancellationControl};
use crate::deck_result_document::{DeckFftBinWindow, DeckFftHarmonicWindow};
use crate::dto::{TransientFftBinsSnapshot, TransientFftHarmonicsSnapshot, TransientSnapshot};
use crate::errors::WasmError;
use crate::options::{MAX_TIMEOUT_MILLISECONDS, WasmCompressionOptions, WasmExecutionOptions};
use crate::result_document::{AnalogResultWindow, SignalWindowValues};
use crate::stb_result_document::StbResultWindow;

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
            "resourceLimits" | "timeoutMilliseconds" | "cancellation"
        ) {
            return Err(Box::new(WasmError::invalid_argument(format!(
                "unknown execution option '{key}'"
            ))));
        }
    }

    let serializable = js_sys::Object::new();
    for name in ["resourceLimits"] {
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

pub(crate) fn compression_options_from_js(
    value: JsValue,
) -> DetailedWasmResult<WasmCompressionOptions> {
    if value.is_undefined() || value.is_null() {
        return Ok(WasmCompressionOptions::default());
    }
    serde_wasm_bindgen::from_value(value).map_err(|error| {
        Box::new(WasmError::invalid_argument(format!(
            "invalid transient compression options: {error}"
        )))
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

pub(crate) fn set_float64_array_entry(
    array: &js_sys::Array,
    index: usize,
    values: &[f64],
    name: &str,
) -> Result<(), JsValue> {
    let index = u32::try_from(index).map_err(|_| {
        JsValue::from_str(&format!(
            "serialization failed: result `{name}` index exceeds JavaScript array bounds"
        ))
    })?;
    let values = js_sys::Float64Array::from(values);
    array.set(index, values.into());
    Ok(())
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

pub(crate) fn publish_optional_waveforms_as_typed_arrays(
    object: &JsValue,
    name: &str,
    waveforms: &[Option<Vec<f64>>],
) -> Result<(), JsValue> {
    let serialized = js_array_property(object, name)?;
    for (index, waveform) in waveforms.iter().enumerate() {
        if let Some(values) = waveform {
            set_float64_array_entry(&serialized, index, values, name)?;
        }
    }
    Ok(())
}

pub(crate) fn publish_trace_values_as_typed_arrays<T>(
    object: &JsValue,
    name: &str,
    traces: &[T],
    values: impl Fn(&T) -> &[f64],
) -> Result<(), JsValue> {
    let serialized = js_array_property(object, name)?;
    for (index, trace) in traces.iter().enumerate() {
        let js_trace = serialized.get(u32::try_from(index).map_err(|_| {
            JsValue::from_str(&format!(
                "serialization failed: result `{name}` index exceeds JavaScript array bounds"
            ))
        })?);
        set_float64_array(&js_trace, "values", values(trace))?;
    }
    Ok(())
}

pub(crate) fn set_uint32_array(
    object: &JsValue,
    name: &str,
    values: &[usize],
) -> Result<(), JsValue> {
    let values = values
        .iter()
        .copied()
        .map(u32::try_from)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| {
            JsValue::from_str(&format!(
                "serialization failed: transient FFT index `{name}` exceeds Uint32Array"
            ))
        })?;
    let values = js_sys::Uint32Array::from(values.as_slice());
    js_sys::Reflect::set(object, &JsValue::from_str(name), values.as_ref())
        .map(|_| ())
        .map_err(|_| {
            JsValue::from_str(&format!(
                "serialization failed: cannot publish transient FFT typed array `{name}`"
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

pub(crate) fn publish_fft_bins_as_typed_arrays(
    object: &JsValue,
    bins: &TransientFftBinsSnapshot,
) -> Result<(), JsValue> {
    set_uint32_array(object, "indices", &bins.indices)?;
    set_float64_array(object, "frequencies", &bins.frequencies)?;
    set_float64_array(object, "real", &bins.real)?;
    set_float64_array(object, "imaginary", &bins.imaginary)?;
    set_float64_array(object, "magnitudes", &bins.magnitudes)?;
    set_float64_array(object, "phase_degrees", &bins.phase_degrees)
}

pub(crate) fn publish_fft_harmonics_as_typed_arrays(
    object: &JsValue,
    harmonics: &TransientFftHarmonicsSnapshot,
) -> Result<(), JsValue> {
    set_uint32_array(object, "ranks", &harmonics.ranks)?;
    set_uint32_array(object, "bins", &harmonics.bins)?;
    set_float64_array(object, "frequencies", &harmonics.frequencies)?;
    set_float64_array(object, "magnitudes", &harmonics.magnitudes)?;
    set_float64_array(object, "magnitudes_db", &harmonics.magnitudes_db)?;
    set_float64_array(object, "phase_degrees", &harmonics.phase_degrees)
}

/// Serialize transient analog and FFT numeric columns as compact,
/// interoperable JavaScript typed arrays. Optional projected waveforms,
/// compression provenance, and FFT fields are deliberately encoded as `null`,
/// not omitted or `undefined`, so consumers can distinguish absence explicitly.
pub(crate) fn serialize_transient_to_js(snapshot: &TransientSnapshot) -> Result<JsValue, JsValue> {
    let serializer = serde_wasm_bindgen::Serializer::new().serialize_missing_as_null(true);
    let serialized = snapshot
        .serialize(&serializer)
        .map_err(|error| JsValue::from_str(&format!("serialization failed: {error}")))?;
    set_float64_array(&serialized, "time", &snapshot.time)?;
    set_float64_array(&serialized, "step_sizes", &snapshot.step_sizes)?;
    publish_optional_waveforms_as_typed_arrays(&serialized, "voltages", &snapshot.voltages)?;
    publish_optional_waveforms_as_typed_arrays(
        &serialized,
        "branch_currents",
        &snapshot.branch_currents,
    )?;
    publish_trace_values_as_typed_arrays(
        &serialized,
        "device_op_traces",
        &snapshot.device_op_traces,
        |trace| &trace.values,
    )?;
    publish_trace_values_as_typed_arrays(
        &serialized,
        "store_traces",
        &snapshot.store_traces,
        |trace| &trace.values,
    )?;

    let fft_results = js_array_property(&serialized, "fft_results")?;

    for (index, fft) in snapshot.fft_results.iter().enumerate() {
        let js_fft = fft_results.get(index as u32);
        let js_bins = js_property(&js_fft, "bins")?;
        publish_fft_bins_as_typed_arrays(&js_bins, &fft.bins)?;

        if let Some(metrics) = &fft.metrics {
            let js_metrics = js_property(&js_fft, "metrics")?;
            let js_harmonics = js_property(&js_metrics, "largest_harmonics")?;
            publish_fft_harmonics_as_typed_arrays(&js_harmonics, &metrics.largest_harmonics)?;
        }
    }

    Ok(serialized)
}

/// Serialize only a bounded analog-result window, replacing every numeric
/// Serde array with its compact JavaScript typed-array representation.
pub(crate) fn serialize_result_window_to_js(
    window: &AnalogResultWindow,
) -> Result<JsValue, JsValue> {
    let serializer = serde_wasm_bindgen::Serializer::new().serialize_missing_as_null(true);
    let serialized = window
        .serialize(&serializer)
        .map_err(|error| JsValue::from_str(&format!("serialization failed: {error}")))?;

    let axes = js_array_property(&serialized, "axes")?;
    for (index, axis) in window.axes.iter().enumerate() {
        let object = axes.get(u32::try_from(index).map_err(|_| {
            JsValue::from_str("serialization failed: result axis index exceeds JavaScript bounds")
        })?);
        set_float64_array(&object, "values", &axis.values)?;
    }

    let signals = js_array_property(&serialized, "signals")?;
    for (index, signal) in window.signals.iter().enumerate() {
        let object = signals.get(u32::try_from(index).map_err(|_| {
            JsValue::from_str("serialization failed: result signal index exceeds JavaScript bounds")
        })?);
        let values = js_property(&object, "values")?;
        match &signal.values {
            SignalWindowValues::Real {
                values: samples,
                validity,
            } => {
                set_float64_array(&values, "values", samples)?;
                set_uint8_array(&values, "validity", validity)?;
            }
            SignalWindowValues::Complex {
                real,
                imaginary,
                validity,
            } => {
                set_float64_array(&values, "real", real)?;
                set_float64_array(&values, "imaginary", imaginary)?;
                set_uint8_array(&values, "validity", validity)?;
            }
        }
    }
    Ok(serialized)
}

pub(crate) fn serialize_deck_fft_bin_window_to_js(
    window: &DeckFftBinWindow,
) -> Result<JsValue, JsValue> {
    let serialized = serialize_to_js(window)?;
    set_uint32_array(&serialized, "indices", &window.indices)?;
    set_float64_array(&serialized, "frequencies", &window.frequencies)?;
    set_float64_array(&serialized, "real", &window.real)?;
    set_float64_array(&serialized, "imaginary", &window.imaginary)?;
    set_float64_array(&serialized, "magnitudes", &window.magnitudes)?;
    set_float64_array(&serialized, "phaseDegrees", &window.phase_degrees)?;
    Ok(serialized)
}

pub(crate) fn serialize_deck_fft_harmonic_window_to_js(
    window: &DeckFftHarmonicWindow,
) -> Result<JsValue, JsValue> {
    let serialized = serialize_to_js(window)?;
    set_uint32_array(&serialized, "ranks", &window.ranks)?;
    set_uint32_array(&serialized, "bins", &window.bins)?;
    set_float64_array(&serialized, "frequencies", &window.frequencies)?;
    set_float64_array(&serialized, "magnitudes", &window.magnitudes)?;
    set_float64_array(&serialized, "magnitudesDb", &window.magnitudes_db)?;
    set_float64_array(&serialized, "phaseDegrees", &window.phase_degrees)?;
    Ok(serialized)
}

/// Serialize one bounded STB window, replacing every retained per-frequency
/// numeric column with a `Float64Array` while leaving optional Nyquist absence
/// explicit as `null`.
pub(crate) fn serialize_stb_result_window_to_js(
    window: &StbResultWindow,
) -> Result<JsValue, JsValue> {
    let serializer = serde_wasm_bindgen::Serializer::new().serialize_missing_as_null(true);
    let serialized = window
        .serialize(&serializer)
        .map_err(|error| JsValue::from_str(&format!("serialization failed: {error}")))?;

    let primary = js_property(&serialized, "primary")?;
    set_float64_array(&primary, "frequencies", &window.primary.frequencies)?;
    let primary_loop_gain = js_property(&primary, "loopGain")?;
    set_float64_array(&primary_loop_gain, "real", &window.primary.loop_gain.real)?;
    set_float64_array(
        &primary_loop_gain,
        "imaginary",
        &window.primary.loop_gain.imaginary,
    )?;

    let bode = js_property(&serialized, "bode")?;
    set_float64_array(&bode, "frequencies", &window.bode.frequencies)?;
    set_float64_array(&bode, "magnitudes", &window.bode.magnitudes)?;
    set_float64_array(&bode, "magnitudesDb", &window.bode.magnitudes_db)?;
    set_float64_array(&bode, "phaseDegrees", &window.bode.phase_degrees)?;
    let bode_loop_gain = js_property(&bode, "loopGain")?;
    set_float64_array(&bode_loop_gain, "real", &window.bode.loop_gain.real)?;
    set_float64_array(
        &bode_loop_gain,
        "imaginary",
        &window.bode.loop_gain.imaginary,
    )?;

    if let Some(nyquist) = &window.nyquist {
        let js_nyquist = js_property(&serialized, "nyquist")?;
        set_float64_array(&js_nyquist, "real", &nyquist.real)?;
        set_float64_array(&js_nyquist, "imaginary", &nyquist.imaginary)?;
        set_float64_array(&js_nyquist, "frequencies", &nyquist.frequencies)?;
    }
    Ok(serialized)
}
