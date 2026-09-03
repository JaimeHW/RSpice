//! The browser adapter over `rspice_core::execution::AnalysisResultDocument`.
//!
//! There is exactly one result document in this crate, and it is the core's.
//! This module only projects it for JavaScript: a small metadata object of
//! identity, descriptors, units, availability and provenance, and bounded
//! point windows whose numeric columns are published as typed arrays by
//! [`crate::js_interop`].
//!
//! Nothing here decides what a result *means*. Every tag it writes comes from
//! a core `tag()`, a core `Serialize` implementation, or an exhaustive mapping
//! that fails closed on a core variant this build does not know.

use rspice_core::execution::result_document::{
    AxisValues, CompressionReportDocument, DeviceStateSeries, FftChildReference, ResultAxis,
    ResultCoordinate, ResultNamespaces, ResultScalar, ResultSignal, SeriesAvailability,
    SeriesQualifier,
};
use rspice_core::execution::{
    AnalysisInstanceId, AnalysisResultDocument, ResultAxisKind, ResultPayload, SignalDescriptor,
    SignalKind, SignalOwner, SignalShape, SignalUnit, SignalValueType,
};
use serde::Serialize;

use crate::DetailedWasmResult;
use crate::errors::WasmError;

/// One `kind`/`ordinal` analysis identity, spelled the way core spells it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalysisIdentity {
    /// Canonical instance tag such as `ac-002`.
    pub id: String,
    /// Canonical analysis-kind tag such as `ac`.
    pub kind: &'static str,
    /// One-based position among the analyses of this kind.
    pub ordinal: usize,
}

impl AnalysisIdentity {
    pub(crate) fn new(id: AnalysisInstanceId) -> Self {
        Self {
            id: id.tag(),
            kind: id.kind().tag(),
            ordinal: id.ordinal() as usize + 1,
        }
    }
}

/// A physical unit, in the core document's own wire spelling.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "unit", rename_all = "snake_case")]
pub enum SignalUnitView {
    Volt,
    Ampere,
    Ohm,
    Siemens,
    Watt,
    Hertz,
    Second,
    Degree,
    Radian,
    Dimensionless,
    Logic,
    Custom { symbol: String },
}

impl SignalUnitView {
    fn project(unit: &SignalUnit) -> DetailedWasmResult<Self> {
        Ok(match unit {
            SignalUnit::Volt => Self::Volt,
            SignalUnit::Ampere => Self::Ampere,
            SignalUnit::Ohm => Self::Ohm,
            SignalUnit::Siemens => Self::Siemens,
            SignalUnit::Watt => Self::Watt,
            SignalUnit::Hertz => Self::Hertz,
            SignalUnit::Second => Self::Second,
            SignalUnit::Degree => Self::Degree,
            SignalUnit::Radian => Self::Radian,
            SignalUnit::Dimensionless => Self::Dimensionless,
            SignalUnit::Logic => Self::Logic,
            SignalUnit::Custom(symbol) => Self::Custom {
                symbol: symbol.clone(),
            },
            other => return Err(unknown_variant("signal unit", format!("{other:?}"))),
        })
    }

    fn project_optional(unit: Option<&SignalUnit>) -> DetailedWasmResult<Option<Self>> {
        unit.map(Self::project).transpose()
    }
}

/// What a signal measures, in the core document's own wire spelling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalKindView {
    Voltage,
    Current,
    DeviceObservable,
    Scalar,
    Digital,
}

impl SignalKindView {
    fn project(kind: SignalKind) -> DetailedWasmResult<Self> {
        Ok(match kind {
            SignalKind::Voltage => Self::Voltage,
            SignalKind::Current => Self::Current,
            SignalKind::DeviceObservable => Self::DeviceObservable,
            SignalKind::Scalar => Self::Scalar,
            SignalKind::Digital => Self::Digital,
            other => return Err(unknown_variant("signal kind", format!("{other:?}"))),
        })
    }
}

/// How one sample is represented.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalValueTypeView {
    Real,
    Complex,
    Logic,
}

impl SignalValueTypeView {
    fn project(value_type: SignalValueType) -> DetailedWasmResult<Self> {
        Ok(match value_type {
            SignalValueType::Real => Self::Real,
            SignalValueType::Complex => Self::Complex,
            SignalValueType::Logic => Self::Logic,
            other => return Err(unknown_variant("signal value type", format!("{other:?}"))),
        })
    }
}

/// Rank of one sample.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalShapeView {
    Scalar,
    Vector,
    Matrix,
}

impl SignalShapeView {
    fn project(shape: SignalShape) -> DetailedWasmResult<Self> {
        Ok(match shape {
            SignalShape::Scalar => Self::Scalar,
            SignalShape::Vector => Self::Vector,
            SignalShape::Matrix => Self::Matrix,
            other => return Err(unknown_variant("signal shape", format!("{other:?}"))),
        })
    }
}

/// Which circuit object a signal belongs to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SignalOwnerView {
    Node { name: String },
    Branch { name: String },
    Device { name: String },
    Analysis,
}

impl SignalOwnerView {
    fn project(owner: &SignalOwner) -> DetailedWasmResult<Self> {
        Ok(match owner {
            SignalOwner::Node(name) => Self::Node { name: name.clone() },
            SignalOwner::Branch(name) => Self::Branch { name: name.clone() },
            SignalOwner::Device(name) => Self::Device { name: name.clone() },
            SignalOwner::Analysis => Self::Analysis,
            other => return Err(unknown_variant("signal owner", format!("{other:?}"))),
        })
    }
}

/// Coordinates of one axis: real or integer, never missing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AxisValueTypeView {
    Real,
    Integer,
}

/// One coordinate axis, without its coordinates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisDescriptor {
    pub name: String,
    pub display_name: String,
    pub kind: ResultAxisKind,
    pub unit: SignalUnitView,
    pub value_type: AxisValueTypeView,
}

impl AxisDescriptor {
    fn project(axis: &ResultAxis) -> DetailedWasmResult<Self> {
        Ok(Self {
            name: axis.name().to_owned(),
            display_name: axis.display_name().to_owned(),
            kind: axis.kind(),
            unit: SignalUnitView::project(axis.unit())?,
            value_type: match axis.values() {
                AxisValues::Real { .. } => AxisValueTypeView::Real,
                AxisValues::Integer { .. } => AxisValueTypeView::Integer,
            },
        })
    }
}

/// One signal series, without its samples.
///
/// `availability` distinguishes a series that was computed from one that
/// output projection deliberately did not retain and one that does not exist
/// at this coordinate. `hasAnySample` is the observed fact that goes with it.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalDescriptorView {
    pub canonical_name: String,
    pub display_name: String,
    pub kind: SignalKindView,
    pub unit: SignalUnitView,
    pub value_type: SignalValueTypeView,
    pub shape: SignalShapeView,
    pub owner: SignalOwnerView,
    pub qualifier: Option<SeriesQualifier>,
    pub availability: SeriesAvailability,
    pub has_any_sample: bool,
}

impl SignalDescriptorView {
    fn project(signal: &ResultSignal) -> DetailedWasmResult<Self> {
        let descriptor: &SignalDescriptor = signal.descriptor();
        Ok(Self {
            canonical_name: descriptor.canonical_name().to_owned(),
            display_name: descriptor.display_name().to_owned(),
            kind: SignalKindView::project(descriptor.kind())?,
            unit: SignalUnitView::project(descriptor.unit())?,
            value_type: SignalValueTypeView::project(descriptor.value_type())?,
            shape: SignalShapeView::project(descriptor.shape())?,
            owner: SignalOwnerView::project(descriptor.owner())?,
            qualifier: signal.qualifier().cloned(),
            availability: signal.availability(),
            has_any_sample: signal.has_any_sample(),
        })
    }
}

/// One device's operating-state history, without its values.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceStateDescriptor {
    pub device_name: String,
    pub device_kind: Option<String>,
    pub has_regions: bool,
    pub parameters: Vec<DeviceParameterDescriptor>,
}

/// One named device operating-point parameter, without its values.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceParameterDescriptor {
    pub name: String,
    pub unit: Option<SignalUnitView>,
}

impl DeviceStateDescriptor {
    fn project(state: &DeviceStateSeries) -> DetailedWasmResult<Self> {
        let mut parameters = Vec::new();
        parameters
            .try_reserve_exact(state.parameters().len())
            .map_err(|_| allocation_error("device parameter descriptors"))?;
        for parameter in state.parameters() {
            parameters.push(DeviceParameterDescriptor {
                name: parameter.name.clone(),
                unit: SignalUnitView::project_optional(parameter.unit.as_ref())?,
            });
        }
        Ok(Self {
            device_name: state.device_name().to_owned(),
            device_kind: state.device_kind().map(str::to_owned),
            has_regions: !state.regions().is_empty(),
            parameters,
        })
    }
}

/// Descriptor-level provenance of the family-specific payload.
///
/// The payload's bulk (per-point step sizes, event traces, per-trial samples,
/// spectra) is reachable losslessly through the handle's bounded JSON export;
/// what belongs in a metadata object is the family tag, the compression
/// certificate, and the identity of the post-processes this result produced.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PayloadDescriptor<'a> {
    /// Core result-family tag, identical to `resultKind`.
    pub family: &'static str,
    /// Present only when the published waveforms were decimated.
    pub compression: Option<&'a CompressionReportDocument>,
    /// Identity of each `.FFT` spectrum this transient produced. The spectra
    /// are separate results in the same handle.
    pub fft_children: &'a [FftChildReference],
}

impl<'a> PayloadDescriptor<'a> {
    fn project(payload: &'a ResultPayload) -> Self {
        let (compression, fft_children) = match payload {
            ResultPayload::Tran(transient) => (
                transient.compression.as_ref(),
                transient.fft_children.as_slice(),
            ),
            ResultPayload::Envelope(envelope) => (
                envelope.transient.compression.as_ref(),
                envelope.transient.fft_children.as_slice(),
            ),
            _ => (None, [].as_slice()),
        };
        Self {
            family: payload.result_kind().tag(),
            compression,
            fft_children,
        }
    }
}

/// The complete descriptor-only view of one result.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultMetadata<'a> {
    /// Core document schema identifier.
    pub schema: &'a str,
    /// Core document schema version.
    pub schema_version: u32,
    /// Core result-family tag.
    pub result_kind: &'static str,
    pub analysis: AnalysisIdentity,
    /// The analysis this result was post-processed from, for `fft`,
    /// `fourier`, `pac`, `pnoise` and `envelope`.
    pub parent_analysis: Option<AnalysisIdentity>,
    /// Stable run-coordinate identity, or `null` for a result that belongs to
    /// no shared-deck coordinate.
    pub coordinate_id: Option<String>,
    /// The complete coordinate, with its typed axis assignments.
    pub coordinate: Option<&'a ResultCoordinate>,
    /// Structural identity of the elaborated topology that was solved.
    pub topology_fingerprint: Option<String>,
    /// Output and checkpoint namespaces this result was written under.
    pub namespaces: Option<&'a ResultNamespaces>,
    pub point_count: usize,
    pub axes: Vec<AxisDescriptor>,
    pub signals: Vec<SignalDescriptorView>,
    /// Per-analysis scalars, published in full: they are one value each.
    pub scalars: &'a [ResultScalar],
    pub device_states: Vec<DeviceStateDescriptor>,
    pub payload: PayloadDescriptor<'a>,
    /// Numeric columns one point occupies, for sizing a window.
    pub values_per_point: usize,
    /// Every numeric value this result retains, for resource accounting.
    pub total_value_count: usize,
    /// Largest window this handle will transfer, in numeric plus validity
    /// values.
    pub maximum_window_values: usize,
}

/// Project one core document into its descriptor-only metadata.
pub(crate) fn result_metadata(
    document: &AnalysisResultDocument,
    maximum_window_values: usize,
) -> DetailedWasmResult<ResultMetadata<'_>> {
    let mut axes = Vec::new();
    axes.try_reserve_exact(document.axes().len())
        .map_err(|_| allocation_error("axis descriptors"))?;
    for axis in document.axes() {
        axes.push(AxisDescriptor::project(axis)?);
    }
    let mut signals = Vec::new();
    signals
        .try_reserve_exact(document.signals().len())
        .map_err(|_| allocation_error("signal descriptors"))?;
    for signal in document.signals() {
        signals.push(SignalDescriptorView::project(signal)?);
    }
    let mut device_states = Vec::new();
    device_states
        .try_reserve_exact(document.device_states().len())
        .map_err(|_| allocation_error("device state descriptors"))?;
    for state in document.device_states() {
        device_states.push(DeviceStateDescriptor::project(state)?);
    }

    Ok(ResultMetadata {
        schema: document.schema(),
        schema_version: document.schema_version(),
        result_kind: document.result_kind().tag(),
        analysis: AnalysisIdentity::new(document.analysis()),
        parent_analysis: document.parent_analysis().map(AnalysisIdentity::new),
        coordinate_id: document
            .coordinate()
            .map(|coordinate| coordinate.id().to_string()),
        coordinate: document.coordinate(),
        topology_fingerprint: document
            .topology_fingerprint()
            .map(|fingerprint| fingerprint.to_string()),
        namespaces: document.namespaces(),
        point_count: document.point_count(),
        axes,
        signals,
        scalars: document.scalars(),
        device_states,
        payload: PayloadDescriptor::project(document.payload()),
        values_per_point: document.values_per_point(),
        total_value_count: document.total_value_count(),
        maximum_window_values,
    })
}

/// Numeric plus validity values one window of `count` points transfers.
///
/// Every signal contributes its numeric columns and one validity byte per
/// point, because a transfer that dropped the mask would make a placeholder
/// zero indistinguishable from a measured zero.
pub(crate) fn window_transfer_values(
    document: &AnalysisResultDocument,
    count: usize,
) -> DetailedWasmResult<usize> {
    let per_point = document
        .values_per_point()
        .checked_add(document.signals().len())
        .ok_or_else(|| allocation_error("window transfer accounting"))?;
    per_point
        .checked_mul(count)
        .ok_or_else(|| allocation_error("window transfer accounting"))
}

fn unknown_variant(location: &str, found: String) -> Box<WasmError> {
    Box::new(WasmError::new(
        format!("{location} {found} has no browser representation in this build"),
        "unsupported_result_schema",
        "result_projection",
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
    use rspice_core::execution::result_document::SeriesAvailability;

    use super::*;
    use crate::runners::deck::run_authored_deck_document_detailed;

    const DECK: &str = "browser document projection deck\n\
V1 in 0 1 AC 1\n\
R1 in out 1k\n\
C1 out 0 1p\n\
.AC DEC 2 1k 10k\n\
.END\n";

    fn ac_document() -> AnalysisResultDocument {
        run_authored_deck_document_detailed(DECK)
            .expect("the AC deck runs")
            .results
            .remove(0)
    }

    /// The projection publishes descriptors and provenance, and nothing that
    /// would make it a second copy of the samples.
    #[test]
    fn metadata_publishes_descriptors_units_and_provenance() {
        let document = ac_document();
        let metadata = result_metadata(&document, 1024).expect("metadata projects");

        assert_eq!(metadata.schema, document.schema());
        assert_eq!(metadata.schema_version, document.schema_version());
        assert_eq!(metadata.result_kind, "ac");
        assert_eq!(metadata.analysis.id, "ac-001");
        assert_eq!(metadata.analysis.kind, "ac");
        assert_eq!(metadata.analysis.ordinal, 1);
        assert_eq!(metadata.parent_analysis, None);
        assert_eq!(metadata.point_count, document.point_count());
        assert_eq!(metadata.values_per_point, document.values_per_point());
        assert_eq!(metadata.total_value_count, document.total_value_count());
        assert_eq!(metadata.maximum_window_values, 1024);
        assert_eq!(metadata.payload.family, "ac");
        assert!(metadata.payload.compression.is_none());
        assert!(metadata.payload.fft_children.is_empty());

        assert_eq!(metadata.axes.len(), 1);
        assert_eq!(metadata.axes[0].name, "frequency");
        assert_eq!(metadata.axes[0].unit, SignalUnitView::Hertz);
        assert_eq!(metadata.axes[0].value_type, AxisValueTypeView::Real);

        let voltage = metadata
            .signals
            .iter()
            .find(|signal| signal.owner == SignalOwnerView::Node { name: "out".into() })
            .expect("the AC document keeps a node-voltage series for out");
        assert_eq!(voltage.kind, SignalKindView::Voltage);
        assert_eq!(voltage.unit, SignalUnitView::Volt);
        assert_eq!(voltage.value_type, SignalValueTypeView::Complex);
        assert_eq!(voltage.availability, SeriesAvailability::Available);
        assert!(voltage.has_any_sample);
    }

    /// A window charges for its numeric columns and for one validity byte per
    /// signal per point, because a transfer without the mask cannot tell a
    /// placeholder from a measurement.
    #[test]
    fn window_accounting_charges_for_the_validity_mask() {
        let document = ac_document();
        let signals = document.signals().len();
        let expected = (document.values_per_point() + signals) * 4;
        assert_eq!(
            window_transfer_values(&document, 4).expect("window accounting"),
            expected
        );
    }

    /// The projected metadata is a JavaScript object of plain fields; it
    /// serializes with the browser's camelCase spelling.
    #[test]
    fn metadata_serializes_with_the_browser_field_spelling() {
        let document = ac_document();
        let metadata = result_metadata(&document, 1024).expect("metadata projects");
        let value = serde_json::to_value(&metadata).expect("metadata serializes");
        for field in [
            "schema",
            "schemaVersion",
            "resultKind",
            "analysis",
            "parentAnalysis",
            "coordinateId",
            "coordinate",
            "topologyFingerprint",
            "namespaces",
            "pointCount",
            "axes",
            "signals",
            "scalars",
            "deviceStates",
            "payload",
            "valuesPerPoint",
            "totalValueCount",
            "maximumWindowValues",
        ] {
            assert!(
                value.get(field).is_some(),
                "browser metadata must publish `{field}`"
            );
        }
        assert!(
            value["signals"][0].get("canonicalName").is_some(),
            "signal descriptors keep their camelCase spelling"
        );
    }
}
