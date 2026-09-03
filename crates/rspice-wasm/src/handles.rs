//! Retained result handles.
//!
//! JavaScript reads descriptor-only metadata once, then requests bounded
//! point windows whose numeric columns cross the boundary as typed arrays.
//! No export copies a whole result into ordinary JavaScript arrays.

use rspice_core::{AbortSignal, ResourceKind, ResourceLimits};
use wasm_bindgen::prelude::*;

use crate::DetailedWasmResult;
use crate::abort::aborted_error;
use crate::deck_result_document::DeckResultDocument;
use crate::errors::resource_limit_error;
use crate::errors::{WasmError, wasm_error_to_js};
use crate::js_interop::{
    serialize_deck_fft_bin_window_to_js, serialize_deck_fft_harmonic_window_to_js,
    serialize_result_window_to_js, serialize_stb_result_window_to_js, serialize_to_js,
};
use crate::options::DEFAULT_MAX_TRANSFER_VALUES;
use crate::result_document::{AnalogResultDocument, AnalogResultMetadata, AnalogResultWindow};
use crate::stb_result_document::{
    self, StbDocumentError, StbResultDocument, StbResultMetadata, StbResultWindow,
};

/// A versioned analog result retained in WebAssembly memory.
///
/// JavaScript reads the descriptor-only metadata once, then calls
/// `readWindow(start, count)` to transfer a bounded slice of every aligned
/// numeric column as typed arrays. This avoids serializing a second full copy
/// of a large result into ordinary JavaScript arrays.
#[derive(Debug)]
#[wasm_bindgen]
pub struct WasmAnalogResultHandle {
    pub(crate) document: AnalogResultDocument,
    pub(crate) maximum_window_values: usize,
}

/// Versioned results from a complete authored analog deck.
///
/// The handle retains every coordinate-local result in WebAssembly memory.
/// Only descriptors and caller-bounded numeric windows cross into JavaScript.
#[derive(Debug)]
#[wasm_bindgen]
pub struct WasmDeckResultHandle {
    pub(crate) document: DeckResultDocument,
    pub(crate) maximum_window_values: usize,
}

/// Versioned STB result retained in WebAssembly memory.
///
/// Metadata contains the six scalar margins and all descriptors. Large
/// primary, Bode, and optional Nyquist columns cross the boundary only through
/// bounded typed-array windows.
#[derive(Debug)]
#[wasm_bindgen]
pub struct WasmStbResultHandle {
    pub(crate) document: StbResultDocument,
    pub(crate) maximum_window_values: usize,
}

pub(crate) fn stb_metadata_error(error: StbDocumentError) -> Box<WasmError> {
    match error {
        StbDocumentError::Aborted => Box::new(WasmError::from_simulation_error(
            rspice_core::engine::SimulationError::Aborted,
        )),
        StbDocumentError::Invalid(message) => Box::new(WasmError::new(
            message,
            "invalid_result_document",
            "result_validation",
        )),
        StbDocumentError::Allocation(message) => Box::new(WasmError::new(
            message,
            "result_allocation_failed",
            "result_transfer",
        )),
    }
}

pub(crate) fn stb_window_error(error: StbDocumentError) -> Box<WasmError> {
    match error {
        StbDocumentError::Aborted => Box::new(WasmError::from_simulation_error(
            rspice_core::engine::SimulationError::Aborted,
        )),
        StbDocumentError::Invalid(message) => Box::new(WasmError::new(
            message,
            "invalid_result_window",
            "result_transfer",
        )),
        StbDocumentError::Allocation(message) => Box::new(WasmError::new(
            message,
            "result_allocation_failed",
            "result_transfer",
        )),
    }
}

impl WasmStbResultHandle {
    pub(crate) fn new_with_abort(
        document: StbResultDocument,
        resource_limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> DetailedWasmResult<Self> {
        match document.validate_with_abort(abort) {
            Ok(()) => {}
            Err(stb_result_document::StbDocumentError::Aborted) => {
                return Err(Box::new(WasmError::from_simulation_error(
                    rspice_core::engine::SimulationError::Aborted,
                )));
            }
            Err(stb_result_document::StbDocumentError::Invalid(message)) => {
                return Err(Box::new(WasmError::new(
                    message,
                    "invalid_result_document",
                    "result_validation",
                )));
            }
            Err(stb_result_document::StbDocumentError::Allocation(message)) => {
                return Err(Box::new(WasmError::new(
                    message,
                    "result_allocation_failed",
                    "result_validation",
                )));
            }
        }
        let retained_values = document.retained_numeric_value_count().map_err(|message| {
            Box::new(WasmError::new(
                message,
                "invalid_result_document",
                "result_validation",
            ))
        })?;
        if retained_values > resource_limits.max_result_values {
            return Err(resource_limit_error(
                ResourceKind::ResultValues,
                retained_values,
                resource_limits.max_result_values,
            ));
        }
        Ok(Self {
            document,
            maximum_window_values: resource_limits
                .max_result_values
                .min(DEFAULT_MAX_TRANSFER_VALUES),
        })
    }

    /// Access the canonical Rust document without crossing the JS boundary.
    pub fn document(&self) -> &StbResultDocument {
        &self.document
    }

    pub(crate) fn metadata_snapshot(&self) -> DetailedWasmResult<StbResultMetadata> {
        self.document
            .metadata(self.maximum_window_values)
            .map_err(stb_metadata_error)
    }

    pub(crate) fn window_snapshot(
        &self,
        start: usize,
        count: usize,
    ) -> DetailedWasmResult<StbResultWindow> {
        self.document
            .window(start, count, self.maximum_window_values)
            .map_err(stb_window_error)
    }
}

#[wasm_bindgen]
impl WasmStbResultHandle {
    #[wasm_bindgen(getter, js_name = pointCount)]
    pub fn point_count(&self) -> usize {
        self.document.point_count
    }

    #[wasm_bindgen(getter, js_name = analysisId)]
    pub fn analysis_id(&self) -> String {
        self.document.analysis.id.clone()
    }

    /// Return STB descriptors, units, margins, status, and transfer ceiling
    /// without copying any per-frequency sample column.
    #[wasm_bindgen(js_name = metadata)]
    pub fn metadata_js(&self) -> Result<JsValue, JsValue> {
        let metadata = self
            .metadata_snapshot()
            .map_err(|error| wasm_error_to_js(*error))?;
        serialize_to_js(&metadata).map_err(|_| {
            wasm_error_to_js(WasmError::new(
                "failed to serialize STB result metadata".to_owned(),
                "result_serialization_failed",
                "result_transfer",
            ))
        })
    }

    /// Transfer one bounded half-open frequency range as typed arrays.
    #[wasm_bindgen(js_name = readWindow)]
    pub fn read_window_js(&self, start: usize, count: usize) -> Result<JsValue, JsValue> {
        let window = self
            .window_snapshot(start, count)
            .map_err(|error| wasm_error_to_js(*error))?;
        serialize_stb_result_window_to_js(&window).map_err(|_| {
            wasm_error_to_js(WasmError::new(
                "failed to serialize STB result window".to_owned(),
                "result_serialization_failed",
                "result_transfer",
            ))
        })
    }
}

impl WasmAnalogResultHandle {
    pub(crate) fn new(
        document: AnalogResultDocument,
        resource_limits: ResourceLimits,
    ) -> DetailedWasmResult<Self> {
        document.validate().map_err(|message| {
            Box::new(WasmError::new(
                message,
                "invalid_result_document",
                "result_validation",
            ))
        })?;
        let retained_values = document.retained_numeric_value_count();
        if retained_values > resource_limits.max_result_values {
            return Err(resource_limit_error(
                ResourceKind::ResultValues,
                retained_values,
                resource_limits.max_result_values,
            ));
        }
        Ok(Self {
            document,
            maximum_window_values: resource_limits
                .max_result_values
                .min(DEFAULT_MAX_TRANSFER_VALUES),
        })
    }

    /// Access the canonical Rust document without crossing the JS boundary.
    pub fn document(&self) -> &AnalogResultDocument {
        &self.document
    }

    pub(crate) fn metadata_snapshot(&self) -> AnalogResultMetadata {
        self.document.metadata(self.maximum_window_values)
    }

    pub(crate) fn window_snapshot(
        &self,
        start: usize,
        count: usize,
    ) -> DetailedWasmResult<AnalogResultWindow> {
        self.document
            .window(start, count, self.maximum_window_values)
            .map_err(|message| {
                Box::new(WasmError::new(
                    message,
                    "invalid_result_window",
                    "result_transfer",
                ))
            })
    }
}

impl WasmDeckResultHandle {
    pub(crate) fn new_with_abort(
        document: DeckResultDocument,
        resource_limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> DetailedWasmResult<Self> {
        document.validate_with_abort(abort).map_err(|message| {
            if abort.is_aborted() {
                aborted_error()
            } else {
                Box::new(WasmError::new(
                    message,
                    "invalid_deck_result_document",
                    "result_validation",
                ))
            }
        })?;
        let retained_values = document
            .retained_numeric_value_count_with_abort(abort)
            .map_err(|message| {
                if abort.is_aborted() {
                    aborted_error()
                } else {
                    Box::new(WasmError::new(
                        message,
                        "invalid_deck_result_document",
                        "result_validation",
                    ))
                }
            })?;
        if retained_values > resource_limits.max_result_values {
            return Err(resource_limit_error(
                ResourceKind::ResultValues,
                retained_values,
                resource_limits.max_result_values,
            ));
        }
        Ok(Self {
            document,
            maximum_window_values: resource_limits
                .max_result_values
                .min(DEFAULT_MAX_TRANSFER_VALUES),
        })
    }

    /// Access the canonical Rust document without crossing the JS boundary.
    pub fn document(&self) -> &DeckResultDocument {
        &self.document
    }
}

#[wasm_bindgen]
impl WasmDeckResultHandle {
    #[wasm_bindgen(getter, js_name = coordinateCount)]
    pub fn coordinate_count(&self) -> usize {
        self.document.coordinates.len()
    }

    #[wasm_bindgen(getter, js_name = resultCount)]
    pub fn result_count(&self) -> usize {
        self.document.results.len()
    }

    #[wasm_bindgen(getter, js_name = fftResultCount)]
    pub fn fft_result_count(&self) -> usize {
        self.document.fft_results.len()
    }

    /// Return aggregate axes, coordinates, stable namespaces, and compact
    /// result summaries without copying any numeric result column.
    #[wasm_bindgen(js_name = metadata)]
    pub fn metadata_js(&self) -> Result<JsValue, JsValue> {
        let metadata = self
            .document
            .metadata(self.maximum_window_values)
            .map_err(|message| {
                wasm_error_to_js(WasmError::new(
                    message,
                    "invalid_deck_result_document",
                    "result_validation",
                ))
            })?;
        serialize_to_js(&metadata).map_err(|_| {
            wasm_error_to_js(WasmError::new(
                "failed to serialize deck result metadata".to_owned(),
                "result_serialization_failed",
                "result_transfer",
            ))
        })
    }

    /// Return the coordinate-local schema for one analog result.
    #[wasm_bindgen(js_name = resultMetadata)]
    pub fn result_metadata_js(&self, result_index: usize) -> Result<JsValue, JsValue> {
        let metadata = self
            .document
            .result_metadata(result_index, self.maximum_window_values)
            .map_err(|message| {
                wasm_error_to_js(WasmError::new(
                    message,
                    "invalid_result_index",
                    "result_transfer",
                ))
            })?;
        serialize_to_js(&metadata).map_err(|_| {
            wasm_error_to_js(WasmError::new(
                "failed to serialize coordinate-local result metadata".to_owned(),
                "result_serialization_failed",
                "result_transfer",
            ))
        })
    }

    /// Transfer one bounded half-open window from one coordinate-local analog
    /// result as typed numeric and validity arrays.
    #[wasm_bindgen(js_name = readWindow)]
    pub fn read_window_js(
        &self,
        result_index: usize,
        start: usize,
        count: usize,
    ) -> Result<JsValue, JsValue> {
        let window = self
            .document
            .result_window(result_index, start, count, self.maximum_window_values)
            .map_err(|message| {
                wasm_error_to_js(WasmError::new(
                    message,
                    "invalid_result_window",
                    "result_transfer",
                ))
            })?;
        serialize_result_window_to_js(&window).map_err(|_| {
            wasm_error_to_js(WasmError::new(
                "failed to serialize coordinate-local result window".to_owned(),
                "result_serialization_failed",
                "result_transfer",
            ))
        })
    }

    /// Return complete scalar FFT configuration and metrics without copying
    /// bin or harmonic numeric columns.
    #[wasm_bindgen(js_name = fftMetadata)]
    pub fn fft_metadata_js(&self, fft_index: usize) -> Result<JsValue, JsValue> {
        let metadata = self
            .document
            .fft_metadata(fft_index, self.maximum_window_values)
            .map_err(|message| {
                wasm_error_to_js(WasmError::new(
                    message,
                    "invalid_fft_result_index",
                    "result_transfer",
                ))
            })?;
        serialize_to_js(&metadata).map_err(|_| {
            wasm_error_to_js(WasmError::new(
                "failed to serialize deck FFT metadata".to_owned(),
                "result_serialization_failed",
                "result_transfer",
            ))
        })
    }

    /// Transfer one bounded half-open FFT-bin window as typed arrays.
    #[wasm_bindgen(js_name = readFftBins)]
    pub fn read_fft_bins_js(
        &self,
        fft_index: usize,
        start: usize,
        count: usize,
    ) -> Result<JsValue, JsValue> {
        let window = self
            .document
            .fft_bin_window(fft_index, start, count, self.maximum_window_values)
            .map_err(|message| {
                wasm_error_to_js(WasmError::new(
                    message,
                    "invalid_fft_result_window",
                    "result_transfer",
                ))
            })?;
        serialize_deck_fft_bin_window_to_js(&window).map_err(|_| {
            wasm_error_to_js(WasmError::new(
                "failed to serialize deck FFT bin window".to_owned(),
                "result_serialization_failed",
                "result_transfer",
            ))
        })
    }

    /// Transfer one bounded half-open magnitude-ranked harmonic window.
    #[wasm_bindgen(js_name = readFftHarmonics)]
    pub fn read_fft_harmonics_js(
        &self,
        fft_index: usize,
        start: usize,
        count: usize,
    ) -> Result<JsValue, JsValue> {
        let window = self
            .document
            .fft_harmonic_window(fft_index, start, count, self.maximum_window_values)
            .map_err(|message| {
                wasm_error_to_js(WasmError::new(
                    message,
                    "invalid_fft_result_window",
                    "result_transfer",
                ))
            })?;
        serialize_deck_fft_harmonic_window_to_js(&window).map_err(|_| {
            wasm_error_to_js(WasmError::new(
                "failed to serialize deck FFT harmonic window".to_owned(),
                "result_serialization_failed",
                "result_transfer",
            ))
        })
    }
}

#[wasm_bindgen]
impl WasmAnalogResultHandle {
    #[wasm_bindgen(getter, js_name = pointCount)]
    pub fn point_count(&self) -> usize {
        self.document.point_count
    }

    #[wasm_bindgen(getter, js_name = analysisId)]
    pub fn analysis_id(&self) -> String {
        self.document.analysis.id.clone()
    }

    /// Return descriptors, units, identity, explicit coordinate absence, and
    /// the transfer ceiling without copying result samples.
    #[wasm_bindgen(js_name = metadata)]
    pub fn metadata_js(&self) -> Result<JsValue, JsValue> {
        serialize_to_js(&self.metadata_snapshot())
    }

    /// Transfer one bounded, half-open point range as typed numeric and
    /// validity arrays. Missing samples carry validity zero; their numeric
    /// slots are placeholders and must not be interpreted.
    #[wasm_bindgen(js_name = readWindow)]
    pub fn read_window_js(&self, start: usize, count: usize) -> Result<JsValue, JsValue> {
        let window = self
            .window_snapshot(start, count)
            .map_err(|error| wasm_error_to_js(*error))?;
        serialize_result_window_to_js(&window)
    }
}
