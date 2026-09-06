//! The retained result handle.
//!
//! One handle carries every result of one browser call: a scalar request has
//! a single result, an authored deck has one per coordinate/analysis pair plus
//! one per attached post-process. JavaScript reads descriptor-only metadata,
//! then requests bounded point windows whose numeric columns cross the
//! boundary as typed arrays. No export copies a whole result into ordinary
//! JavaScript arrays.

use rspice_core::execution::result_document::ResultCoordinate;
use rspice_core::execution::{
    AnalysisResultDocument, AxisKind, DeckPlan, ResultDocumentError, RunAxis, RunAxisValue,
    RunCoordinate, StepAxisTarget,
};
use rspice_core::{AbortSignal, ResourceKind, ResourceLimits};
use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::DetailedWasmResult;
use crate::abort::aborted_error;
use crate::document::{AnalysisIdentity, ResultMetadata, result_metadata, window_transfer_values};
use crate::errors::{WasmError, wasm_error_to_js};
use crate::events::{BusEventRow, DigitalBusDescriptor, DigitalEventRow, DigitalNodeDescriptor};
use crate::js_interop::{serialize_result_window_to_js, serialize_to_js};
use crate::options::{DEFAULT_MAX_RESULT_JSON_BYTES, DEFAULT_MAX_TRANSFER_VALUES};

/// Schema identifier of the handle's own metadata envelope. The results it
/// carries keep the core document's schema and version.
pub const BROWSER_RESULT_SCHEMA: &str = "rspice-browser-result";
/// Version of the handle metadata envelope.
pub const BROWSER_RESULT_VERSION: u32 = 2;

/// One planned run axis, without its values.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunAxisDescriptor {
    pub kind: &'static str,
    pub name: String,
    pub step_target: Option<StepTargetDescriptor>,
    pub value_count: usize,
    /// Parameter names one DATA row binds; empty for every other axis kind.
    pub data_bindings: Vec<String>,
}

/// The typed target one authored numeric `.STEP` dimension changes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StepTargetDescriptor {
    Parameter {
        name: String,
    },
    Device {
        name: String,
        parameter: Option<String>,
    },
    Model {
        name: String,
        parameter: String,
    },
    Temperature,
}

/// Compact per-result entry in the handle's metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultSummary {
    pub index: usize,
    pub result_kind: &'static str,
    pub analysis: AnalysisIdentity,
    pub parent_analysis: Option<AnalysisIdentity>,
    pub coordinate_index: Option<usize>,
    pub coordinate_id: Option<String>,
    pub output_namespace: Option<String>,
    pub checkpoint_namespace: Option<String>,
    pub point_count: usize,
    pub signal_count: usize,
    pub scalar_count: usize,
    pub device_state_count: usize,
    pub total_value_count: usize,
}

/// The handle's own metadata: the plan it executed and what it retained.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HandleMetadata<'a> {
    pub schema: &'static str,
    pub schema_version: u32,
    pub axes: &'a [RunAxisDescriptor],
    pub planned_analyses: Vec<AnalysisIdentity>,
    pub coordinates: &'a [ResultCoordinate],
    pub results: Vec<ResultSummary>,
    pub result_count: usize,
    pub maximum_window_values: usize,
    pub maximum_result_json_bytes: f64,
}

/// Every result of one browser call, retained in WebAssembly memory.
#[derive(Debug)]
#[wasm_bindgen]
pub struct WasmResultHandle {
    axes: Vec<RunAxisDescriptor>,
    planned_analyses: Vec<rspice_core::execution::AnalysisInstanceId>,
    coordinates: Vec<ResultCoordinate>,
    results: Vec<AnalysisResultDocument>,
    maximum_window_values: usize,
    maximum_result_json_bytes: u64,
}

impl WasmResultHandle {
    /// Build a handle over the plan that produced these results.
    ///
    /// The transfer ceiling is the browser default, further reduced by the
    /// caller's own `maxResultValues`, so tightening the resource policy can
    /// never widen what one window transfers.
    pub(crate) fn new(
        plan: &DeckPlan,
        coordinates: Vec<RunCoordinate>,
        results: Vec<AnalysisResultDocument>,
        resource_limits: ResourceLimits,
    ) -> DetailedWasmResult<Self> {
        let mut axes = Vec::new();
        axes.try_reserve_exact(plan.axes().len())
            .map_err(|_| allocation_error("run-axis descriptors"))?;
        for axis in plan.axes() {
            axes.push(run_axis_descriptor(axis)?);
        }
        let mut planned_analyses = Vec::new();
        planned_analyses
            .try_reserve_exact(plan.analyses().len())
            .map_err(|_| allocation_error("planned analysis identities"))?;
        for analysis in plan.analyses() {
            planned_analyses.push(analysis.id());
        }
        let mut projected = Vec::new();
        projected
            .try_reserve_exact(coordinates.len())
            .map_err(|_| allocation_error("run coordinates"))?;
        for coordinate in &coordinates {
            projected.push(ResultCoordinate::from_run_coordinate(coordinate));
        }
        Ok(Self {
            axes,
            planned_analyses,
            coordinates: projected,
            results,
            maximum_window_values: DEFAULT_MAX_TRANSFER_VALUES
                .min(resource_limits.max_result_values),
            maximum_result_json_bytes: DEFAULT_MAX_RESULT_JSON_BYTES,
        })
    }

    /// The retained core documents, in publication order.
    pub fn documents(&self) -> &[AnalysisResultDocument] {
        &self.results
    }

    fn document(&self, index: usize) -> DetailedWasmResult<&AnalysisResultDocument> {
        self.results.get(index).ok_or_else(|| {
            Box::new(WasmError::new(
                format!(
                    "result index {index} is outside the {} retained results",
                    self.results.len()
                ),
                "invalid_result_index",
                "input_validation",
            ))
        })
    }

    pub(crate) fn metadata_snapshot(&self) -> DetailedWasmResult<HandleMetadata<'_>> {
        let mut results = Vec::new();
        results
            .try_reserve_exact(self.results.len())
            .map_err(|_| allocation_error("result summaries"))?;
        for (index, document) in self.results.iter().enumerate() {
            let namespaces = document.namespaces();
            results.push(ResultSummary {
                index,
                result_kind: document.result_kind().tag(),
                analysis: AnalysisIdentity::new(document.analysis()),
                parent_analysis: document.parent_analysis().map(AnalysisIdentity::new),
                coordinate_index: document
                    .coordinate()
                    .map(rspice_core::execution::result_document::ResultCoordinate::ordinal),
                coordinate_id: document
                    .coordinate()
                    .map(|coordinate| coordinate.id().to_string()),
                output_namespace: namespaces.map(|namespaces| namespaces.output.clone()),
                checkpoint_namespace: namespaces.map(|namespaces| namespaces.checkpoint.clone()),
                point_count: document.point_count(),
                signal_count: document.signals().len(),
                scalar_count: document.scalars().len(),
                device_state_count: document.device_states().len(),
                total_value_count: document.total_value_count(),
            });
        }
        let mut planned_analyses = Vec::new();
        planned_analyses
            .try_reserve_exact(self.planned_analyses.len())
            .map_err(|_| allocation_error("planned analysis descriptors"))?;
        for id in &self.planned_analyses {
            planned_analyses.push(AnalysisIdentity::new(*id));
        }
        Ok(HandleMetadata {
            schema: BROWSER_RESULT_SCHEMA,
            schema_version: BROWSER_RESULT_VERSION,
            axes: &self.axes,
            planned_analyses,
            coordinates: &self.coordinates,
            results,
            result_count: self.results.len(),
            maximum_window_values: self.maximum_window_values,
            #[allow(clippy::cast_precision_loss)]
            // JavaScript numbers are f64; the byte ceiling is far below 2^53.
            maximum_result_json_bytes: self.maximum_result_json_bytes as f64,
        })
    }

    pub(crate) fn result_metadata_snapshot(
        &self,
        index: usize,
    ) -> DetailedWasmResult<ResultMetadata<'_>> {
        let document = self.document(index)?;
        result_metadata(document, self.maximum_window_values)
    }

    pub(crate) fn window_snapshot(
        &self,
        index: usize,
        start: usize,
        count: usize,
    ) -> DetailedWasmResult<rspice_core::execution::ResultWindow> {
        let document = self.document(index)?;
        if count == 0 {
            return Err(window_error(
                "a result window must contain at least one point".to_owned(),
            ));
        }
        let requested = window_transfer_values(document, count)?;
        if requested > self.maximum_window_values {
            return Err(window_error(format!(
                "result window requires {requested} numeric/validity values but the transfer limit is {}",
                self.maximum_window_values
            )));
        }
        document
            .window(start, count)
            .map_err(|error| window_error(error.to_string()))
    }

    pub(crate) fn digital_nodes_snapshot(
        &self,
        index: usize,
    ) -> DetailedWasmResult<Vec<DigitalNodeDescriptor>> {
        crate::events::digital_nodes(self.document(index)?)
    }

    pub(crate) fn digital_events_snapshot(
        &self,
        index: usize,
        node: &str,
    ) -> DetailedWasmResult<Vec<DigitalEventRow>> {
        crate::events::digital_events(self.document(index)?, node, self.maximum_window_values)
    }

    pub(crate) fn digital_buses_snapshot(
        &self,
        index: usize,
    ) -> DetailedWasmResult<Vec<DigitalBusDescriptor>> {
        crate::events::digital_buses(self.document(index)?)
    }

    pub(crate) fn bus_events_snapshot(
        &self,
        index: usize,
        name: &str,
    ) -> DetailedWasmResult<Vec<BusEventRow>> {
        crate::events::bus_events(self.document(index)?, name, self.maximum_window_values)
    }

    pub(crate) fn vcd_snapshot(&self, index: usize) -> DetailedWasmResult<String> {
        crate::events::vcd_text(self.document(index)?)
    }

    pub(crate) fn result_json_snapshot(
        &self,
        index: usize,
        abort: &dyn AbortSignal,
    ) -> DetailedWasmResult<String> {
        let document = self.document(index)?;
        document
            .to_json_with_abort(abort, self.maximum_result_json_bytes)
            .map_err(|error| match error {
                ResultDocumentError::Aborted => aborted_error(),
                ResultDocumentError::ArtifactTooLarge { limit_bytes } => Box::new(
                    WasmError::resource_limit(
                        format!(
                            "result {index} does not fit in the {limit_bytes}-byte browser export budget"
                        ),
                        rspice_core::ResourceLimitError {
                            resource: ResourceKind::ResultValues,
                            requested: document.total_value_count(),
                            limit: self.maximum_window_values,
                        },
                    ),
                ),
                other => Box::new(WasmError::new(
                    other.to_string(),
                    "invalid_result_document",
                    "result_validation",
                )),
            })
    }
}

#[wasm_bindgen]
impl WasmResultHandle {
    /// Number of retained results.
    #[wasm_bindgen(js_name = resultCount)]
    pub fn result_count(&self) -> usize {
        self.results.len()
    }

    /// Number of canonical run coordinates the plan produced.
    #[wasm_bindgen(js_name = coordinateCount)]
    pub fn coordinate_count(&self) -> usize {
        self.coordinates.len()
    }

    /// The plan, the coordinates, and a compact summary of every result.
    #[wasm_bindgen(js_name = metadata)]
    pub fn metadata_js(&self) -> Result<JsValue, JsValue> {
        let metadata = self
            .metadata_snapshot()
            .map_err(|error| wasm_error_to_js(*error))?;
        serialize_to_js(&metadata)
    }

    /// Descriptors, units, availability, and provenance of one result.
    #[wasm_bindgen(js_name = resultMetadata)]
    pub fn result_metadata_js(&self, result_index: usize) -> Result<JsValue, JsValue> {
        let metadata = self
            .result_metadata_snapshot(result_index)
            .map_err(|error| wasm_error_to_js(*error))?;
        serialize_to_js(&metadata)
    }

    /// A bounded half-open window of one result's aligned numeric columns.
    ///
    /// Axis and sample columns are `Float64Array`; every signal also carries a
    /// `Uint8Array` validity mask. A zero validity entry marks an explicitly
    /// unavailable sample, so the aligned numeric placeholder must not be
    /// interpreted as a measurement.
    #[wasm_bindgen(js_name = readWindow)]
    pub fn read_window_js(
        &self,
        result_index: usize,
        start: usize,
        count: usize,
    ) -> Result<JsValue, JsValue> {
        let window = self
            .window_snapshot(result_index, start, count)
            .map_err(|error| wasm_error_to_js(*error))?;
        serialize_result_window_to_js(&window)
    }

    /// Every XSPICE digital event node of one result.
    ///
    /// Each entry carries the node name, how many changes it committed, and
    /// the bus that claims it when the document declares one. No points cross
    /// here; `digitalEvents` fetches those one node at a time.
    #[wasm_bindgen(js_name = digitalNodes)]
    pub fn digital_nodes_js(&self, result_index: usize) -> Result<JsValue, JsValue> {
        let nodes = self
            .digital_nodes_snapshot(result_index)
            .map_err(|error| wasm_error_to_js(*error))?;
        serialize_to_js(&nodes)
    }

    /// One digital node's whole committed event history, as typed rows.
    ///
    /// A row is `{time, state, strength, code}`: the accepted time in seconds,
    /// the document's own state and strength spellings, and the `0..=12`
    /// XSPICE event code encoding the same pair. Only changes are recorded, so
    /// a row is a value the run committed rather than a sample of a grid.
    ///
    /// The history is charged against the same transfer ceiling a window is,
    /// two values per row, and fails closed with
    /// `code: "invalid_result_window"` when it does not fit. An unknown node
    /// fails with `code: "unknown_event_node"`.
    #[wasm_bindgen(js_name = digitalEvents)]
    pub fn digital_events_js(
        &self,
        result_index: usize,
        node_name: &str,
    ) -> Result<JsValue, JsValue> {
        let events = self
            .digital_events_snapshot(result_index, node_name)
            .map_err(|error| wasm_error_to_js(*error))?;
        serialize_to_js(&events)
    }

    /// Every digital bus one result declares, in declaration order.
    ///
    /// An entry is `{name, msb, lsb, members, source}`: the word's name, the
    /// range exactly as it was declared, the member trace names from the
    /// declared MSB to the declared LSB, and who declared it — `engine` for a
    /// vector boundary port of a mixed Verilog-AMS module, `schematic` for a
    /// drawing's own declaration over the deck it generated, `import` for one
    /// read out of a foreign artifact. No point crosses here; `busEvents`
    /// fetches the word one bus at a time.
    #[wasm_bindgen(js_name = digitalBuses)]
    pub fn digital_buses_js(&self, result_index: usize) -> Result<JsValue, JsValue> {
        let buses = self
            .digital_buses_snapshot(result_index)
            .map_err(|error| wasm_error_to_js(*error))?;
        serialize_to_js(&buses)
    }

    /// One declared bus's whole history, as the word at each of its events.
    ///
    /// A row is `{time, bits, value}`: the accepted time in seconds, one
    /// `0..=12` XSPICE event code per member declared MSB first, and the same
    /// word in VCD's four states. `bits` keeps the drive strength; `value`
    /// does not. A member that did not change at this time carries the value
    /// it held; one the run has not stated a value for yet is `null` in `bits`
    /// and `x` in `value`.
    ///
    /// A row is charged `1 + width` values against the same transfer ceiling a
    /// window obeys, computed from the members' committed changes, and fails
    /// closed with `code: "invalid_result_window"` when it does not fit. An
    /// undeclared bus fails with `code: "unknown_event_bus"`.
    #[wasm_bindgen(js_name = busEvents)]
    pub fn bus_events_js(&self, result_index: usize, bus_name: &str) -> Result<JsValue, JsValue> {
        let events = self
            .bus_events_snapshot(result_index, bus_name)
            .map_err(|error| wasm_error_to_js(*error))?;
        serialize_to_js(&events)
    }

    /// One result's event histories as a Value Change Dump.
    ///
    /// The same bytes `rspice run -f vcd` publishes for the same run: one
    /// `$scope module events`, digital nodes as one-bit wires, real event
    /// nodes as `real` variables, each on its own event timeline. VCD has four
    /// bit states and no drive strength, so the twelve resolved states
    /// collapse onto `0`, `1`, `x` and `z`; `digitalEvents` carries the band.
    ///
    /// A result with no event history fails with
    /// `code: "no_event_history"` rather than returning a dump that declares
    /// no signal.
    #[wasm_bindgen(js_name = toVcd)]
    pub fn to_vcd_js(&self, result_index: usize) -> Result<String, JsValue> {
        self.vcd_snapshot(result_index)
            .map_err(|error| wasm_error_to_js(*error))
    }

    /// The complete core result document for one result, as JSON.
    ///
    /// This is the lossless export path: it is bounded by an explicit byte
    /// budget and fails closed rather than truncating.
    #[wasm_bindgen(js_name = resultJson)]
    pub fn result_json_js(&self, result_index: usize) -> Result<String, JsValue> {
        self.result_json_snapshot(result_index, &rspice_core::NoAbort)
            .map_err(|error| wasm_error_to_js(*error))
    }
}

fn run_axis_descriptor(axis: &RunAxis) -> DetailedWasmResult<RunAxisDescriptor> {
    let kind = match axis.kind() {
        AxisKind::Alter => "alter",
        AxisKind::Data => "data",
        AxisKind::Step => "step",
        AxisKind::Temperature => "temperature",
        other => {
            return Err(Box::new(WasmError::new(
                format!("canonical run-axis kind {other:?} has no browser representation"),
                "unsupported_deck_axis",
                "unsupported_feature",
            )));
        }
    };
    let mut data_bindings = Vec::new();
    if let Some(RunAxisValue::DataRow(bindings)) = axis.values().first() {
        data_bindings
            .try_reserve_exact(bindings.len())
            .map_err(|_| allocation_error("DATA axis binding descriptors"))?;
        for binding in bindings {
            data_bindings.push(binding.name().to_owned());
        }
    }
    Ok(RunAxisDescriptor {
        kind,
        name: axis.name().to_owned(),
        step_target: axis.step_target().map(step_target_descriptor).transpose()?,
        value_count: axis.values().len(),
        data_bindings,
    })
}

fn step_target_descriptor(target: &StepAxisTarget) -> DetailedWasmResult<StepTargetDescriptor> {
    Ok(match target {
        StepAxisTarget::Parameter { name } => {
            StepTargetDescriptor::Parameter { name: name.clone() }
        }
        StepAxisTarget::Device { name, parameter } => StepTargetDescriptor::Device {
            name: name.clone(),
            parameter: parameter.clone(),
        },
        StepAxisTarget::Model { name, parameter } => StepTargetDescriptor::Model {
            name: name.clone(),
            parameter: parameter.clone(),
        },
        StepAxisTarget::Temperature => StepTargetDescriptor::Temperature,
        other => {
            return Err(Box::new(WasmError::new(
                format!("canonical STEP target {other:?} has no browser representation"),
                "unsupported_deck_axis",
                "unsupported_feature",
            )));
        }
    })
}

fn window_error(message: String) -> Box<WasmError> {
    Box::new(WasmError::new(
        message,
        "invalid_result_window",
        "result_transfer",
    ))
}

fn allocation_error(object: &'static str) -> Box<WasmError> {
    Box::new(WasmError::new(
        format!("could not allocate {object}"),
        "result_allocation_failed",
        "result_projection",
    ))
}

#[cfg(test)]
mod tests {
    use rspice_core::NoAbort;
    use rspice_core::execution::result_document::SeriesWindowValues;

    use super::*;
    use crate::runners::deck::run_authored_deck_document_detailed;

    const TRANSIENT: &str = "browser window deck\n\
V1 in 0 PULSE(0 1 0 1n 1n 20n 40n)\n\
R1 in out 1k\n\
C1 out 0 1p\n\
.TRAN 1n 20n\n\
.END\n";

    fn handle(source: &str, limits: ResourceLimits) -> WasmResultHandle {
        let execution = run_authored_deck_document_detailed(source)
            .unwrap_or_else(|error| panic!("deck must run: {}", error.message));
        WasmResultHandle::new(
            &execution.plan,
            execution.coordinates,
            execution.results,
            limits,
        )
        .expect("the handle retains the executed plan")
    }

    /// A window is a half-open slice of every aligned column, and each signal
    /// carries the validity mask that says which numbers are measurements.
    #[test]
    fn a_window_transfers_aligned_columns_with_their_validity_masks() {
        let handle = handle(TRANSIENT, crate::options::browser_resource_limits());
        let point_count = handle.documents()[0].point_count();
        let window = handle
            .window_snapshot(0, 1, 3)
            .expect("a three-point window is inside the result");

        assert_eq!(window.start, 1);
        assert_eq!(window.count, 3);
        assert_eq!(window.point_count, point_count);
        assert_eq!(window.axes.len(), 1);
        assert!(!window.signals.is_empty());
        for signal in &window.signals {
            let validity = match &signal.values {
                SeriesWindowValues::Real { values, validity } => {
                    assert_eq!(values.len(), 3);
                    validity
                }
                SeriesWindowValues::Complex {
                    real,
                    imaginary,
                    validity,
                } => {
                    assert_eq!(real.len(), 3);
                    assert_eq!(imaginary.len(), 3);
                    validity
                }
                SeriesWindowValues::Logic { samples, validity } => {
                    assert_eq!(samples.len(), 3);
                    validity
                }
            };
            assert_eq!(validity.len(), 3, "every window column carries its mask");
        }
    }

    /// Empty, out-of-range, and overflowing windows fail closed with the
    /// documented code instead of clamping to something plausible.
    #[test]
    fn empty_and_out_of_range_windows_fail_closed() {
        let handle = handle(TRANSIENT, crate::options::browser_resource_limits());
        let point_count = handle.documents()[0].point_count();

        for (start, count) in [(0, 0), (point_count, 1), (0, point_count + 1)] {
            let error = *handle
                .window_snapshot(0, start, count)
                .expect_err("an invalid window must fail closed");
            assert_eq!(error.code, "invalid_result_window");
            assert_eq!(error.category, "result_transfer");
        }
        let error = *handle
            .window_snapshot(handle.result_count(), 0, 1)
            .expect_err("an unknown result index must fail closed");
        assert_eq!(error.code, "invalid_result_index");
    }

    /// A tighter `maxResultValues` tightens the transfer ceiling; it can never
    /// widen it.
    #[test]
    fn the_transfer_budget_is_the_stricter_of_the_two_ceilings() {
        let mut limits = crate::options::browser_resource_limits();
        limits.max_result_values = 8;
        let handle = handle(TRANSIENT, limits);
        let metadata = handle
            .result_metadata_snapshot(0)
            .expect("result metadata projects");
        assert_eq!(metadata.maximum_window_values, 8);

        let error = *handle
            .window_snapshot(0, 0, 4)
            .expect_err("an over-budget window must fail closed");
        assert_eq!(error.code, "invalid_result_window");
        assert!(
            error.message.contains("transfer limit is 8"),
            "the failure quotes the effective ceiling: {}",
            error.message
        );
    }

    /// The lossless JSON export is the core document, unchanged.
    #[test]
    fn the_json_export_round_trips_the_core_document() {
        let handle = handle(TRANSIENT, crate::options::browser_resource_limits());
        let json = handle
            .result_json_snapshot(0, &NoAbort)
            .expect("the lossless export encodes");
        let decoded = AnalysisResultDocument::from_json(&json).expect("the export decodes");
        assert_eq!(&decoded, &handle.documents()[0]);
    }

    /// A transient that captures event nodes answers the typed accessors, and
    /// the payload descriptor's counts are the same run's.
    #[test]
    fn the_event_accessors_and_the_payload_counts_describe_one_run() {
        const EVENT_DECK: &str = "browser event handle deck
v1 in 0 pulse(0 5 0 1n 1n 5n 10n)
abridge1 [in] [d] adc
adac [d] [out] dac
aobs out rnode obs
rout out 0 1k
.model adc adc_bridge(in_low=1 in_high=4)
.model dac dac_bridge(out_low=0 out_high=5 out_undef=2.5)
.model obs v_to_real(gain=2)
.tran 1n 20n
.end
";
        let handle = handle(EVENT_DECK, crate::options::browser_resource_limits());
        let nodes = handle
            .digital_nodes_snapshot(0)
            .expect("a transient answers the inventory");
        assert!(!nodes.is_empty());

        let metadata = handle
            .result_metadata_snapshot(0)
            .expect("result metadata projects");
        assert_eq!(metadata.payload.digital_node_count, nodes.len());
        assert_eq!(
            metadata.payload.digital_event_count,
            nodes.iter().map(|node| node.event_count).sum::<usize>()
        );
        assert_eq!(metadata.payload.digital_bus_count, 0);
        assert!(metadata.payload.real_node_count > 0);
        assert!(metadata.payload.real_event_count > 0);

        let rows = handle
            .digital_events_snapshot(0, &nodes[0].node_name)
            .expect("a named node answers");
        assert_eq!(rows.len(), nodes[0].event_count);

        let dump = handle.vcd_snapshot(0).expect("the run dumps");
        assert!(dump.contains("$scope module events $end"), "{dump}");

        let error = *handle
            .digital_nodes_snapshot(handle.result_count())
            .expect_err("an unknown result index must fail closed");
        assert_eq!(error.code, "invalid_result_index");
    }

    /// A tighter `maxResultValues` tightens an event history the way it
    /// tightens a window; the two obey one budget.
    #[test]
    fn the_transfer_budget_bounds_an_event_history_too() {
        const EVENT_DECK: &str = "browser event budget deck
v1 in 0 pulse(0 5 0 1n 1n 5n 10n)
abridge1 [in] [d] adc
rin in 0 1k
.model adc adc_bridge(in_low=1 in_high=4)
.tran 1n 20n
.end
";
        let mut limits = crate::options::browser_resource_limits();
        limits.max_result_values = 1;
        let handle = handle(EVENT_DECK, limits);
        let nodes = handle
            .digital_nodes_snapshot(0)
            .expect("a transient answers the inventory");
        let error = *handle
            .digital_events_snapshot(0, &nodes[0].node_name)
            .expect_err("a one-value budget cannot carry a history");
        assert_eq!(error.code, "invalid_result_window");
        assert!(
            error.message.contains("the limit is 1"),
            "{}",
            error.message
        );
    }

    /// Projection is cancellable: a pre-set abort stops the export instead of
    /// returning a truncated string.
    #[test]
    fn a_pre_set_abort_cancels_the_json_export() {
        let handle = handle(TRANSIENT, crate::options::browser_resource_limits());
        let error = *handle
            .result_json_snapshot(0, &rspice_core::abort_signal::ImmediateAbort)
            .expect_err("a cancelled export must not return a document");
        assert_eq!(error.code, "aborted");
        assert_eq!(error.category, "cancellation");
    }
}
