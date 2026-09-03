//! WebAssembly wrapper for the RSpice simulation core.
//!
//! The crate keeps the browser-facing API intentionally thin. It owns no
//! result schema of its own: every analysis produces the shared
//! [`rspice_core::execution::AnalysisResultDocument`], and this crate only
//! decides how that document crosses the JavaScript boundary.
//!
//! The modules are split by role so each one can be read on its own:
//!
//! - [`options`]: browser resource policy, execution options, compression policy
//! - [`abort`]: deadlines, the shared-memory cancellation control, and the
//!   composed abort source every runner polls
//! - [`errors`]: the structured [`WasmError`] and its JavaScript projection
//! - [`dto`]: parser-diagnostic and readiness summaries
//! - [`document`]: the descriptor-only projection of one core result document
//! - [`handles`]: the retained handle that publishes bounded typed-array windows
//! - [`js_interop`]: JavaScript value decoding and typed-array publication
//! - [`runners`]: the authored-deck route and the direct entry points
//! - [`exports`]: the `#[wasm_bindgen]` shims

mod abort;
mod document;
mod dto;
mod errors;
mod exports;
mod handles;
mod hb_request;
mod js_interop;
mod options;
mod runners;
mod support;

pub use document::{
    AnalysisIdentity, AxisDescriptor, AxisValueTypeView, DeviceParameterDescriptor,
    DeviceStateDescriptor, PayloadDescriptor, ResultMetadata, SignalDescriptorView, SignalKindView,
    SignalOwnerView, SignalShapeView, SignalUnitView, SignalValueTypeView,
};
pub use dto::{
    NetlistSummary, WasmDiagnostic, WasmHealthReport, WasmSourceLocation, WasmStartupDiagnostic,
    WasmStartupDirectiveScope,
};
pub use errors::{WasmError, WasmUnresolvedOutputSymbol};
pub use handles::{
    BROWSER_RESULT_SCHEMA, BROWSER_RESULT_VERSION, HandleMetadata, ResultSummary,
    RunAxisDescriptor, StepTargetDescriptor, WasmResultHandle,
};
pub use options::{WasmCompressionOptions, WasmExecutionOptions, WasmResourceLimits};
pub use runners::deck::{
    DeckExecution, run_authored_deck_document_detailed,
    run_authored_deck_document_with_options_and_abort_detailed,
};
pub use runners::direct::{
    health_check_with_options_and_abort_detailed, health_check_with_options_detailed,
    run_ac_document_detailed, run_ac_document_with_options_and_abort_detailed,
    run_dc_sweep_document_detailed, run_dc_sweep_document_with_options_and_abort_detailed,
    run_noise_document_detailed, run_noise_document_with_options_and_abort_detailed,
    run_operating_point_document_detailed,
    run_operating_point_document_with_options_and_abort_detailed, run_transient_document_detailed,
    run_transient_document_with_options_and_abort_detailed, summarize_netlist,
    summarize_netlist_detailed, summarize_netlist_with_options_and_abort_detailed,
    summarize_netlist_with_options_detailed,
};

pub(crate) type WasmResult<T> = Result<T, String>;
pub(crate) type DetailedWasmResult<T> = Result<T, Box<WasmError>>;
