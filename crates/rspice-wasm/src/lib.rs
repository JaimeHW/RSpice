//! WebAssembly wrapper for the RSpice simulation core.
//!
//! The crate keeps the browser-facing API intentionally thin: it exposes
//! serializable snapshots that mirror stable simulator concepts while delegating
//! all numerical work to `rspice-core`.
//!
//! The modules are split by role so each one can be read on its own:
//!
//! - [`options`]: browser resource policy, execution options, compression policy
//! - [`abort`]: deadlines, the shared-memory cancellation control, and the
//!   composed [`abort::ConfiguredAbort`] every runner polls
//! - [`errors`]: the structured [`WasmError`] and its JavaScript projection
//! - [`dto`]: serializable snapshots of core results
//! - [`handles`]: retained result handles that publish bounded windows
//! - [`js_interop`]: JavaScript value decoding and typed-array publication
//! - [`runners`]: the direct and authored-deck execution entry points
//! - [`exports`]: the `#[wasm_bindgen]` shims

mod abort;
mod deck_result_document;
mod dto;
mod errors;
mod exports;
mod handles;
mod js_interop;
mod options;
mod result_document;
mod runners;
mod stb_result_document;
mod support;

#[cfg(test)]
mod tests;

pub use deck_result_document::{
    DECK_RESULT_SCHEMA, DECK_RESULT_VERSION, DeckAxisAssignment, DeckAxisDescriptor, DeckAxisValue,
    DeckCoordinateDescriptor, DeckDataBinding, DeckFftBinWindow, DeckFftHarmonicWindow,
    DeckFftMetadata, DeckFftMetricsMetadata, DeckFftSummary, DeckPlannedAnalysisDescriptor,
    DeckResultDocument, DeckResultMetadata, DeckResultSummary,
};
pub use dto::{
    AcPointSnapshot, ComplexSeries, DcOperatingPoint, NetlistSummary,
    TransientCompressionErrorSnapshot, TransientCompressionSnapshot, TransientDeviceOpSnapshot,
    TransientFftBinsSnapshot, TransientFftHarmonicsSnapshot, TransientFftMetricsSnapshot,
    TransientFftSnapshot, TransientSnapshot, TransientStoreSnapshot, WasmDiagnostic,
    WasmHealthReport, WasmSourceLocation, WasmStartupDiagnostic, WasmStartupDirectiveScope,
    transient_snapshot_from_compressed_result, transient_snapshot_from_result,
};
pub use errors::{WasmError, WasmUnresolvedOutputSymbol};
pub use handles::{WasmAnalogResultHandle, WasmDeckResultHandle, WasmStbResultHandle};
pub use options::{WasmCompressionOptions, WasmExecutionOptions, WasmResourceLimits, WasmStbSweep};
pub use result_document::{
    ANALOG_RESULT_SCHEMA, ANALOG_RESULT_VERSION, AnalogAnalysisKind, AnalogResultDocument,
    AnalogResultMetadata, AnalogResultWindow, AnalogSignalKind, AnalysisIdentity, AxisDescriptor,
    AxisSeries, AxisWindow, ComplexSample, DeviceStateDescriptor, DeviceStateSeries,
    SignalDescriptor, SignalOwner, SignalSeries, SignalUnit, SignalValueType, SignalValues,
    SignalWindow, SignalWindowValues,
};
pub use runners::deck::{
    run_authored_deck_document_detailed, run_authored_deck_document_with_options_and_abort_detailed,
};
pub use runners::direct::{
    health_check_with_options_and_abort_detailed, health_check_with_options_detailed,
    run_ac_analysis, run_ac_analysis_detailed, run_ac_analysis_with_options_and_abort_detailed,
    run_ac_analysis_with_options_detailed, run_ac_document_detailed,
    run_ac_document_with_options_and_abort_detailed, run_dc_operating_point,
    run_dc_operating_point_detailed, run_dc_operating_point_with_options_and_abort_detailed,
    run_dc_operating_point_with_options_detailed, run_dc_sweep_document_detailed,
    run_dc_sweep_document_with_options_and_abort_detailed, run_noise_document_detailed,
    run_noise_document_with_options_and_abort_detailed, run_operating_point_document_detailed,
    run_operating_point_document_with_options_and_abort_detailed, run_stb_document_detailed,
    run_stb_document_with_options_and_abort_detailed, run_transient_analysis,
    run_transient_analysis_compressed, run_transient_analysis_compressed_detailed,
    run_transient_analysis_compressed_with_options_and_abort_detailed,
    run_transient_analysis_compressed_with_options_detailed, run_transient_analysis_detailed,
    run_transient_analysis_with_options_and_abort_detailed,
    run_transient_analysis_with_options_detailed, run_transient_document_detailed,
    run_transient_document_with_options_and_abort_detailed, summarize_netlist,
    summarize_netlist_detailed, summarize_netlist_with_options_and_abort_detailed,
    summarize_netlist_with_options_detailed,
};
pub use stb_result_document::{
    STB_RESULT_SCHEMA, STB_RESULT_VERSION, StbAnalysisIdentity, StbAnalysisKind, StbBodeSeries,
    StbBodeWindow, StbComplexSample, StbComplexWindow, StbDocumentError, StbMarginDescriptors,
    StbMarginUnits, StbMargins, StbNyquistSeries, StbNyquistWindow, StbPrimarySeries,
    StbPrimaryWindow, StbResultDocument, StbResultMetadata, StbResultWindow, StbSeriesDescriptor,
    StbUnit, StbValueType,
};

pub(crate) type WasmResult<T> = Result<T, String>;
pub(crate) type DetailedWasmResult<T> = Result<T, Box<WasmError>>;
