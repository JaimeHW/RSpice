//! The shared result document, exposed uniformly on every result family.
//!
//! Every family in this binding also answers four questions the same way, over
//! the one [`AnalysisResultDocument`] the CLI, the WASM build and the engine
//! adapter publish:
//!
//! - `signals()` — the complete `SignalDescriptor` inventory: canonical name,
//!   kind, unit, value type, shape, owner, availability, and length.
//! - `scalars()` — every analysis-owned scalar, with its unit and its typed
//!   value, including the determinations that are not numbers.
//! - `device_observables()` — every per-device operating-point history the
//!   family captured, with per-sample presence rather than a placeholder zero.
//! - `document()` — the whole document as a JSON-serializable Python object.
//!
//! The document is projected from the core result on demand rather than kept
//! beside it: a transient's document restates its whole waveform set, and
//! holding both would double the memory of every run to answer a question most
//! callers never ask. Projection is done through the same `AnalysisResultDocument::from_*`
//! builders every other surface calls, so no accessor here decides what a
//! result means.
//!
//! # Pickled results
//!
//! A pickled result carries this binding's own projection of a result, not the
//! core result the document is built from. A result restored from pickled
//! state therefore answers these four accessors with a typed
//! `RSpiceNotImplementedError` naming that fact, rather than assembling a
//! document out of the Python projection — which would be a second, silently
//! different projection of the same evidence. `document()` returns plain
//! Python data and pickles losslessly, so a caller who needs the document to
//! survive a pickle pickles that.

use pyo3::prelude::*;
use rspice_core::abort_signal::AbortSignal;
use rspice_core::execution::result_document::{
    DeviceParameterSeries, DeviceStateSeries, ScalarValue, SeriesAvailability,
};
use rspice_core::execution::{
    AnalysisInstanceId, AnalysisResultDocument, ResultDocumentError, ResultScalar, ResultSignal,
    SignalDescriptor, SignalKind, SignalOwner, SignalShape, SignalUnit, SignalValueType,
};

use crate::abort::run_interruptible_unregistered;

/// Largest JSON encoding of one result document this binding will materialize
/// as a Python object.
///
/// The document is bounded before it is built into Python objects so a
/// pathological result reports a typed limit rather than exhausting the
/// interpreter's memory while a caller waits with no way to interrupt it.
const MAX_DOCUMENT_JSON_BYTES: u64 = 512 * 1024 * 1024;

/// Core evidence one Python result keeps so its shared document can be
/// projected on demand.
///
/// `core` is whatever the family's `AnalysisResultDocument::from_*` builder
/// takes beyond the analysis identity. A family whose Python projection
/// already retains the core result uses `()` here and reads the result from
/// its own field, so no result ever holds two copies of the same evidence.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct DocumentEvidence<T> {
    /// Identity of the authored card this result came from.
    pub(crate) analysis: AnalysisInstanceId,
    pub(crate) core: T,
}

impl<T> DocumentEvidence<T> {
    /// Evidence for a run that authored exactly one card of `kind`.
    ///
    /// A deck route that ran several cards of the family replaces this with
    /// the plan's own identity through [`Self::with_analysis`]; a convenience
    /// call runs one card, for which this is the identity the planner assigns.
    pub(crate) fn sole(kind: rspice_core::execution::AnalysisKind, core: T) -> Self {
        Self {
            analysis: sole_identity(kind),
            core,
        }
    }

    pub(crate) fn with_analysis(mut self, analysis: AnalysisInstanceId) -> Self {
        self.analysis = analysis;
        self
    }
}

/// A result whose shared document can be bound to the authored card the
/// canonical deck plan named for it.
///
/// A convenience call runs exactly one card, so its result keeps the identity
/// [`DocumentEvidence::sole`] gave it. `Engine.run` may execute several cards
/// of one family, and binds each result to the plan's own identity here rather
/// than letting every result claim the family's first ordinal.
pub(crate) trait CarriesDocumentEvidence {
    fn bind_analysis(&mut self, analysis: AnalysisInstanceId);
}

/// Bind one freshly produced result to the identity the plan named, when the
/// producing route knows one.
pub(crate) fn bound<T: CarriesDocumentEvidence>(
    mut result: T,
    analysis: Option<AnalysisInstanceId>,
) -> T {
    if let Some(analysis) = analysis {
        result.bind_analysis(analysis);
    }
    result
}

/// Borrow a result's document evidence, or say why it has none.
pub(crate) fn evidence<'result, T>(
    evidence: &'result Option<DocumentEvidence<T>>,
    family: &str,
) -> PyResult<&'result DocumentEvidence<T>> {
    evidence
        .as_ref()
        .ok_or_else(|| restored_result_error(family))
}

/// One entry of a result's shared signal inventory.
///
/// This is the schema of a series, not its samples: a series the run did not
/// retain still appears here with its unit and owner, and says so through
/// `availability`. Reading a value out of an unretained series is impossible
/// rather than silently zero.
#[pyclass(
    name = "SignalDescriptor",
    module = "rspice",
    frozen,
    skip_from_py_object
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PySignalDescriptor {
    /// Canonical lower-case name, such as `v(out)` or `@m1[id]`.
    #[pyo3(get)]
    pub name: String,
    /// Spelling intended for display, such as `V(out)`.
    #[pyo3(get)]
    pub display_name: String,
    /// `voltage`, `current`, `device_observable`, `scalar`, or `digital`.
    #[pyo3(get)]
    pub kind: String,
    /// Unit tag: `volt`, `ampere`, ..., `unspecified`, or `custom`.
    #[pyo3(get)]
    pub unit: String,
    /// Symbol of a `custom` unit; `None` for every declared unit.
    #[pyo3(get)]
    pub unit_symbol: Option<String>,
    /// `real`, `complex`, or `logic`.
    #[pyo3(get)]
    pub value_type: String,
    /// `scalar`, `vector`, or `matrix`.
    #[pyo3(get)]
    pub shape: String,
    /// What the signal belongs to: `node`, `branch`, `device`, or `analysis`.
    #[pyo3(get)]
    pub owner_kind: String,
    /// Name of the owning node, branch, or device; `None` for an
    /// analysis-owned quantity, which belongs to the run rather than to a
    /// circuit object.
    #[pyo3(get)]
    pub owner: Option<String>,
    /// `available`, `not_projected`, or `absent_at_coordinate`.
    #[pyo3(get)]
    pub availability: String,
    /// Samples this series spans, which is the result's point count.
    #[pyo3(get)]
    pub point_count: usize,
}

#[pymethods]
impl PySignalDescriptor {
    /// True when the series was computed and retained.
    #[getter]
    fn is_available(&self) -> bool {
        self.availability == "available"
    }

    fn __repr__(&self) -> String {
        format!(
            "SignalDescriptor(name='{}', kind='{}', unit='{}', owner={}, availability='{}')",
            self.name,
            self.kind,
            self.unit,
            self.owner
                .as_ref()
                .map_or_else(|| "None".to_owned(), |owner| format!("'{owner}'")),
            self.availability
        )
    }
}

/// One analysis-owned scalar of a result.
///
/// `value` is the typed value: a float, a complex, an int, a bool, a string,
/// or `None`. A quantity the analysis proved has no finite value — the gain
/// margin of an unconditionally stable loop, for instance — carries `None`
/// with `unavailable_reason` set, which is a determination and not a missing
/// computation.
#[pyclass(name = "ResultScalar", module = "rspice", frozen, skip_from_py_object)]
#[derive(Debug, Clone)]
pub struct PyResultScalar {
    /// Canonical lower-case name, such as `gain_margin_db`.
    #[pyo3(get)]
    pub name: String,
    /// Spelling intended for display.
    #[pyo3(get)]
    pub display_name: String,
    /// Unit tag, or `None` when the quantity declares no unit at all.
    #[pyo3(get)]
    pub unit: Option<String>,
    /// Symbol of a `custom` unit.
    #[pyo3(get)]
    pub unit_symbol: Option<String>,
    /// How the value is represented: `real`, `complex`, `integer`, `count`,
    /// `boolean`, `text`, or `unavailable`.
    #[pyo3(get)]
    pub representation: String,
    /// Why a real quantity has no finite value: `positive_infinity`,
    /// `negative_infinity`, or `no_crossover`.
    #[pyo3(get)]
    pub unavailable_reason: Option<String>,
    value: ScalarValue,
}

#[pymethods]
impl PyResultScalar {
    /// The typed value, or `None` when the analysis computed none.
    #[getter]
    fn value<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyAny>>> {
        Ok(match &self.value {
            ScalarValue::Real { value } => match value {
                Some(value) => Some(value.into_pyobject(py)?.into_any()),
                None => None,
            },
            ScalarValue::Complex { value } => value.map(|value| {
                pyo3::types::PyComplex::from_doubles(py, value.real, value.imaginary).into_any()
            }),
            ScalarValue::Integer { value } => Some(value.into_pyobject(py)?.into_any()),
            ScalarValue::Count { value } => Some(value.into_pyobject(py)?.into_any()),
            ScalarValue::Boolean { value } => Some(value.into_pyobject(py)?.to_owned().into_any()),
            ScalarValue::Text { value } => Some(value.into_pyobject(py)?.into_any()),
            ScalarValue::Unavailable { .. } => None,
        })
    }

    fn __repr__(&self) -> String {
        format!(
            "ResultScalar(name='{}', representation='{}', unit={})",
            self.name,
            self.representation,
            self.unit
                .as_ref()
                .map_or_else(|| "None".to_owned(), |unit| format!("'{unit}'"))
        )
    }
}

/// One per-device operating-point history captured by a result.
///
/// A sample the run did not capture is absent from `values` and `False` in
/// `validity`; it is never a plausible zero.
#[pyclass(
    name = "DeviceObservable",
    module = "rspice",
    frozen,
    skip_from_py_object
)]
#[derive(Debug, Clone)]
pub struct PyDeviceObservable {
    /// Instance name as written in the netlist.
    #[pyo3(get)]
    pub device: String,
    /// Device family label, when the producing result declares one.
    #[pyo3(get)]
    pub device_kind: Option<String>,
    /// Observable name, such as `id` or `gm`.
    #[pyo3(get)]
    pub parameter: String,
    /// Unit tag, or `None` when the observable declares no unit.
    #[pyo3(get)]
    pub unit: Option<String>,
    /// Symbol of a `custom` unit.
    #[pyo3(get)]
    pub unit_symbol: Option<String>,
    /// Operating region at each point, or an empty list when the family
    /// reports none. An entry is `None` where the region was not captured.
    #[pyo3(get)]
    pub regions: Vec<Option<String>>,
    values: Vec<Option<f64>>,
}

#[pymethods]
impl PyDeviceObservable {
    /// Sample values in analysis order, with `nan` where a sample is absent.
    ///
    /// Pair this with `validity`: `nan` here means "no sample", and the mask
    /// is what says so without a caller having to test for a magic value.
    #[getter]
    fn values<'py>(&self, py: Python<'py>) -> Bound<'py, numpy::PyArray1<f64>> {
        use numpy::ToPyArray;

        self.values
            .iter()
            .map(|value| value.unwrap_or(f64::NAN))
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    /// Per-sample presence. A `False` entry is an absent sample.
    #[getter]
    fn validity(&self) -> Vec<bool> {
        self.values.iter().map(Option::is_some).collect()
    }

    fn __len__(&self) -> usize {
        self.values.len()
    }

    fn __repr__(&self) -> String {
        format!(
            "DeviceObservable(device='{}', parameter='{}', samples={})",
            self.device,
            self.parameter,
            self.values.len()
        )
    }
}

//=============================================================================
// Projection from the shared document
//=============================================================================

/// Stable tag of one signal unit, matching the document's own JSON spelling.
fn unit_tag(unit: &SignalUnit) -> (String, Option<String>) {
    match unit {
        SignalUnit::Volt => ("volt".to_owned(), None),
        SignalUnit::Ampere => ("ampere".to_owned(), None),
        SignalUnit::Ohm => ("ohm".to_owned(), None),
        SignalUnit::Siemens => ("siemens".to_owned(), None),
        SignalUnit::Watt => ("watt".to_owned(), None),
        SignalUnit::Hertz => ("hertz".to_owned(), None),
        SignalUnit::Second => ("second".to_owned(), None),
        SignalUnit::Degree => ("degree".to_owned(), None),
        SignalUnit::Radian => ("radian".to_owned(), None),
        SignalUnit::Dimensionless => ("dimensionless".to_owned(), None),
        SignalUnit::Logic => ("logic".to_owned(), None),
        SignalUnit::Unspecified => ("unspecified".to_owned(), None),
        SignalUnit::Custom(symbol) => ("custom".to_owned(), Some(symbol.clone())),
        // `SignalUnit` is `#[non_exhaustive]`: a unit this build does not know
        // is reported as unknown rather than silently folded onto a unit it is
        // not.
        other => (format!("unknown:{other:?}"), None),
    }
}

fn signal_kind_tag(kind: SignalKind) -> String {
    match kind {
        SignalKind::Voltage => "voltage",
        SignalKind::Current => "current",
        SignalKind::DeviceObservable => "device_observable",
        SignalKind::Scalar => "scalar",
        SignalKind::Digital => "digital",
        _ => "unknown",
    }
    .to_owned()
}

fn value_type_tag(value_type: SignalValueType) -> String {
    match value_type {
        SignalValueType::Real => "real",
        SignalValueType::Complex => "complex",
        SignalValueType::Logic => "logic",
        _ => "unknown",
    }
    .to_owned()
}

fn shape_tag(shape: SignalShape) -> String {
    match shape {
        SignalShape::Scalar => "scalar",
        SignalShape::Vector => "vector",
        SignalShape::Matrix => "matrix",
        _ => "unknown",
    }
    .to_owned()
}

fn owner_tags(owner: &SignalOwner) -> (String, Option<String>) {
    match owner {
        SignalOwner::Node(name) => ("node".to_owned(), Some(name.clone())),
        SignalOwner::Branch(name) => ("branch".to_owned(), Some(name.clone())),
        SignalOwner::Device(name) => ("device".to_owned(), Some(name.clone())),
        SignalOwner::Analysis => ("analysis".to_owned(), None),
        _ => ("unknown".to_owned(), None),
    }
}

fn availability_tag(availability: SeriesAvailability) -> String {
    match availability {
        SeriesAvailability::Available => "available",
        SeriesAvailability::NotProjected => "not_projected",
        SeriesAvailability::AbsentAtCoordinate => "absent_at_coordinate",
    }
    .to_owned()
}

fn descriptor_of(
    descriptor: &SignalDescriptor,
    availability: SeriesAvailability,
    point_count: usize,
) -> PySignalDescriptor {
    let (unit, unit_symbol) = unit_tag(descriptor.unit());
    let (owner_kind, owner) = owner_tags(descriptor.owner());
    PySignalDescriptor {
        name: descriptor.canonical_name().to_owned(),
        display_name: descriptor.display_name().to_owned(),
        kind: signal_kind_tag(descriptor.kind()),
        unit,
        unit_symbol,
        value_type: value_type_tag(descriptor.value_type()),
        shape: shape_tag(descriptor.shape()),
        owner_kind,
        owner,
        availability: availability_tag(availability),
        point_count,
    }
}

fn signal_descriptor(signal: &ResultSignal, point_count: usize) -> PySignalDescriptor {
    descriptor_of(signal.descriptor(), signal.availability(), point_count)
}

fn scalar_of(scalar: &ResultScalar) -> PyResultScalar {
    let (unit, unit_symbol) = match scalar.unit() {
        Some(unit) => {
            let (tag, symbol) = unit_tag(unit);
            (Some(tag), symbol)
        }
        None => (None, None),
    };
    let (representation, unavailable_reason) = match scalar.value() {
        ScalarValue::Real { .. } => ("real", None),
        ScalarValue::Complex { .. } => ("complex", None),
        ScalarValue::Integer { .. } => ("integer", None),
        ScalarValue::Count { .. } => ("count", None),
        ScalarValue::Boolean { .. } => ("boolean", None),
        ScalarValue::Text { .. } => ("text", None),
        ScalarValue::Unavailable { reason } => ("unavailable", Some(reason.tag().to_owned())),
    };
    PyResultScalar {
        name: scalar.name().to_owned(),
        display_name: scalar.display_name().to_owned(),
        unit,
        unit_symbol,
        representation: representation.to_owned(),
        unavailable_reason,
        value: scalar.value().clone(),
    }
}

fn device_observables_of(state: &DeviceStateSeries) -> Vec<PyDeviceObservable> {
    state
        .parameters()
        .iter()
        .map(|parameter: &DeviceParameterSeries| {
            let (unit, unit_symbol) = match &parameter.unit {
                Some(unit) => {
                    let (tag, symbol) = unit_tag(unit);
                    (Some(tag), symbol)
                }
                None => (None, None),
            };
            PyDeviceObservable {
                device: state.device_name().to_owned(),
                device_kind: state.device_kind().map(str::to_owned),
                parameter: parameter.name.clone(),
                unit,
                unit_symbol,
                regions: state.regions().to_vec(),
                values: parameter.values.clone(),
            }
        })
        .collect()
}

//=============================================================================
// Uniform accessors
//=============================================================================

/// Map one document projection failure onto the binding's error contract.
pub(crate) fn document_error(error: ResultDocumentError) -> PyErr {
    match error {
        ResultDocumentError::Aborted => {
            crate::errors::SimulationError::new_err("result document projection was cancelled")
        }
        other => crate::errors::SimulationError::new_err(format!(
            "the shared result document could not be projected from this result: {other}"
        )),
    }
}

/// The typed refusal a result restored from pickled state answers with.
pub(crate) fn restored_result_error(family: &str) -> PyErr {
    crate::errors::not_implemented_error(format!(
        "this {family} result was restored from pickled state, which carries the Python \
         projection of a result rather than the core result the shared document is built from; \
         re-run the analysis, or pickle result.document() when the document itself must survive \
         a round trip"
    ))
}

/// Build one result's shared document on an interruptible worker.
///
/// The projection walks every retained sample, so it is exactly the kind of
/// call that must not hold the GIL or ignore Ctrl-C.
pub(crate) fn build<F>(py: Python<'_>, project: F) -> PyResult<AnalysisResultDocument>
where
    F: FnOnce(&dyn AbortSignal) -> Result<AnalysisResultDocument, ResultDocumentError> + Send,
{
    run_interruptible_unregistered(py, |abort| Ok(project(abort)))?.map_err(document_error)
}

/// The shared signal inventory of one result.
pub(crate) fn signals(document: &AnalysisResultDocument) -> Vec<PySignalDescriptor> {
    let point_count = document.point_count();
    document
        .signals()
        .iter()
        .map(|signal| signal_descriptor(signal, point_count))
        .collect()
}

/// Every analysis-owned scalar of one result.
pub(crate) fn scalars(document: &AnalysisResultDocument) -> Vec<PyResultScalar> {
    document.scalars().iter().map(scalar_of).collect()
}

/// Every per-device observable history of one result.
pub(crate) fn device_observables(document: &AnalysisResultDocument) -> Vec<PyDeviceObservable> {
    document
        .device_states()
        .iter()
        .flat_map(device_observables_of)
        .collect()
}

/// The whole document as a JSON-serializable Python object.
///
/// The encoding is bounded and cancellable, then decoded by the interpreter's
/// own `json` module, so the object a caller receives is exactly the document
/// the other surfaces publish rather than a second hand-built projection.
pub(crate) fn json_view<'py>(
    py: Python<'py>,
    document: &AnalysisResultDocument,
) -> PyResult<Bound<'py, PyAny>> {
    let encoded = run_interruptible_unregistered(py, |abort| {
        Ok(document.to_json_with_abort(abort, MAX_DOCUMENT_JSON_BYTES))
    })?
    .map_err(|error| match error {
        ResultDocumentError::ArtifactTooLarge { limit_bytes } => {
            crate::errors::value_error(format!(
                "this result's shared document exceeds the {limit_bytes}-byte view limit; \
                 read it through signals(), scalars() and device_observables() instead"
            ))
        }
        other => document_error(other),
    })?;
    py.import("json")?.call_method1("loads", (encoded,))
}

/// Analysis identity of a result produced by a run that authored one card.
pub(crate) fn sole_identity(kind: rspice_core::execution::AnalysisKind) -> AnalysisInstanceId {
    rspice_core::execution::sole_analysis_identity(kind)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::execution::AnalysisKind;
    use rspice_core::execution::result_document::{
        OperatingPointPayload, ResultPayload, SeriesValues,
    };

    /// Units a `SignalKind::Scalar` descriptor may legally carry.
    ///
    /// `Logic` is excluded because a logic-unit signal is a digital signal,
    /// which this binding deliberately does not map.
    const SCALAR_UNITS: [SignalUnit; 11] = [
        SignalUnit::Volt,
        SignalUnit::Ampere,
        SignalUnit::Ohm,
        SignalUnit::Siemens,
        SignalUnit::Watt,
        SignalUnit::Hertz,
        SignalUnit::Second,
        SignalUnit::Degree,
        SignalUnit::Radian,
        SignalUnit::Dimensionless,
        SignalUnit::Unspecified,
    ];

    fn scalar_signal(name: &str, unit: SignalUnit) -> ResultSignal {
        ResultSignal::new(
            SignalDescriptor::new(
                name,
                name,
                SignalKind::Scalar,
                unit,
                SignalValueType::Real,
                SignalShape::Scalar,
                SignalOwner::Analysis,
            )
            .expect("the fixture descriptor is well formed"),
            None,
            SeriesAvailability::Available,
            SeriesValues::Real {
                samples: vec![Some(1.0)],
            },
        )
        .expect("the fixture series is well formed")
    }

    /// One document carrying one scalar signal per unit under test.
    fn unit_document() -> AnalysisResultDocument {
        let mut units = SCALAR_UNITS.to_vec();
        units.push(SignalUnit::Custom("degC".to_owned()));
        let signals = units
            .iter()
            .enumerate()
            .map(|(index, unit)| scalar_signal(&format!("probe_{index}"), unit.clone()))
            .collect::<Vec<_>>();
        AnalysisResultDocument::builder(
            sole_identity(AnalysisKind::Op),
            ResultPayload::Op(OperatingPointPayload {
                observables: Vec::new(),
            }),
            1,
        )
        .signals(signals)
        .build()
        .expect("the fixture document validates")
    }

    /// The unit a Python descriptor reports must be the one the document's own
    /// JSON spells, or the two surfaces disagree about the same signal.
    #[test]
    fn every_unit_tag_is_the_documents_own_json_spelling() {
        let document = unit_document();
        let encoded = document.to_json().expect("the fixture document encodes");
        let projected = signals(&document);
        assert_eq!(projected.len(), SCALAR_UNITS.len() + 1);
        for descriptor in &projected {
            let published = match &descriptor.unit_symbol {
                Some(symbol) => {
                    format!(
                        "{{\"unit\":\"{}\",\"symbol\":\"{symbol}\"}}",
                        descriptor.unit
                    )
                }
                None => format!("{{\"unit\":\"{}\"}}", descriptor.unit),
            };
            assert!(
                encoded.contains(&published),
                "descriptor '{}' reports unit {published}, which the published document does not \
                 spell that way: {encoded}",
                descriptor.name
            );
            assert_eq!(descriptor.kind, "scalar");
            assert_eq!(descriptor.owner_kind, "analysis");
            assert!(descriptor.owner.is_none());
            assert_eq!(descriptor.availability, "available");
            assert_eq!(descriptor.point_count, 1);
        }
        assert!(
            projected
                .iter()
                .any(|descriptor| descriptor.unit_symbol.as_deref() == Some("degC")),
            "the fixture covers a custom unit"
        );
    }

    /// An unretained series keeps its schema and says it was not retained,
    /// which is what stops a caller reading it as a zero.
    #[test]
    fn an_unprojected_series_keeps_its_descriptor_and_declares_itself_absent() {
        let signal = ResultSignal::new(
            SignalDescriptor::new(
                "unretained",
                "Unretained",
                SignalKind::Scalar,
                SignalUnit::Volt,
                SignalValueType::Real,
                SignalShape::Scalar,
                SignalOwner::Analysis,
            )
            .expect("the fixture descriptor is well formed"),
            None,
            SeriesAvailability::NotProjected,
            SeriesValues::Real {
                samples: vec![None],
            },
        )
        .expect("an unretained series is well formed");
        let document = AnalysisResultDocument::builder(
            sole_identity(AnalysisKind::Op),
            ResultPayload::Op(OperatingPointPayload {
                observables: Vec::new(),
            }),
            1,
        )
        .signals(vec![signal])
        .build()
        .expect("the fixture document validates");
        let [descriptor] = signals(&document).try_into().expect("one descriptor");
        assert_eq!(descriptor.availability, "not_projected");
        assert_eq!(descriptor.unit, "volt");
    }
}
