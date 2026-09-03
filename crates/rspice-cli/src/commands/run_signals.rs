//! Turn engine results into named export signals.
//!
//! Core builds the inventory one analysis result materialized and the one
//! [`SignalProjection`] decides which of it the deck's
//! `.SAVE`/`.PROBE`/`.PRINT`/`.PLOT` cards select. This module owns only the
//! CLI's side of that boundary: it flattens core columns into the row type the
//! output writers consume, and it proves that a swept result keeps the schema
//! its first coordinate established, so a renamed or vanished signal fails
//! before an artifact is written instead of shifting a column.

use std::collections::{BTreeMap, HashMap};

use rspice_core::execution::{
    AnalysisResultKind, ProjectedSignal, ProjectedSignals, ProjectionSource,
    ProjectionSourceSignal, ProjectionValues, SignalProjection, SignalValueType,
    probe_registry_name, signal_descriptor,
};
use rspice_core::{Value, analysis::AcResult, engine::TransientResult, solver::SimulationResult};

pub(crate) use rspice_core::execution::SignalKind;

/// The rawfile variable type a projected column serializes as.
pub(crate) fn raw_variable_type(kind: SignalKind) -> &'static str {
    rspice_core::execution::raw_variable_type(kind)
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ScalarSignal {
    pub(crate) display_name: String,
    pub(crate) raw_name: String,
    pub(crate) kind: SignalKind,
    pub(crate) values: Vec<Value>,
}

impl ScalarSignal {
    pub(crate) fn raw_variable_type(&self) -> &'static str {
        raw_variable_type(self.kind)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ComplexSignal {
    pub(crate) display_name: String,
    pub(crate) raw_name: String,
    pub(crate) kind: SignalKind,
    pub(crate) real: Vec<Value>,
    pub(crate) imag: Vec<Value>,
}

impl ComplexSignal {
    pub(crate) fn raw_variable_type(&self) -> &'static str {
        raw_variable_type(self.kind)
    }
}

/// Describe one already-projected scalar result without inspecting its
/// numeric values.  STEP preflight unions these descriptors across every
/// coordinate before any artifact is committed, so absent signals become
/// explicit validity bits instead of zeroes or first-coordinate columns.
pub(crate) fn scalar_signal_schema(
    signals: &[ScalarSignal],
) -> Result<rspice_core::execution::SignalSchema, rspice_core::execution::SignalSchemaError> {
    let descriptors = signals
        .iter()
        .map(scalar_descriptor)
        .collect::<Result<Vec<_>, _>>()?;
    rspice_core::execution::SignalSchema::new(descriptors)
}

pub(crate) fn scalar_descriptor(
    signal: &ScalarSignal,
) -> Result<rspice_core::execution::SignalDescriptor, rspice_core::execution::SignalSchemaError> {
    let value_type = if signal.kind == SignalKind::Digital {
        SignalValueType::Logic
    } else {
        SignalValueType::Real
    };
    signal_descriptor(
        &signal.display_name,
        &signal.raw_name,
        signal.kind,
        value_type,
    )
}

pub(crate) fn complex_descriptor(
    signal: &ComplexSignal,
) -> Result<rspice_core::execution::SignalDescriptor, rspice_core::execution::SignalSchemaError> {
    signal_descriptor(
        &signal.display_name,
        &signal.raw_name,
        signal.kind,
        SignalValueType::Complex,
    )
}

/// The bare circuit symbol a result's own metadata gives one column, with an
/// ordinal only as a last resort so no exported column is anonymous.
fn registry_or_ordinal(name: &str, fallback_index: usize) -> String {
    let candidate = probe_registry_name(name).trim();
    if candidate.is_empty() {
        fallback_index.to_string()
    } else {
        candidate.to_string()
    }
}

pub(crate) fn voltage_display_name(name: &str, fallback_index: usize) -> String {
    format!("V({})", registry_or_ordinal(name, fallback_index))
}

//=============================================================================
// Projection entry points
//=============================================================================

/// Build the deck's authored output contract once per export.
pub(crate) fn projection(
    netlist: &rspice_core::Netlist,
) -> Result<SignalProjection, rspice_core::SimulationError> {
    SignalProjection::from_netlist(netlist)
}

/// One real-valued analysis result offered for authored output projection.
///
/// `lookup` carries resolvable-but-not-exported spellings (operating-point
/// observables, hierarchy aliases, `device:param`) so an authored
/// `.SAVE @D1[Id]` resolves without adding a column to an unrestricted export.
/// `ordered` carries the columns the core `.PRINT` resolver already produced
/// for the families that own one.
pub(crate) struct ScalarProjectionRequest<'a> {
    pub(crate) kind: AnalysisResultKind,
    pub(crate) instance: &'a str,
    pub(crate) axis: &'a [Value],
    pub(crate) signals: &'a [ScalarSignal],
    pub(crate) lookup: HashMap<String, &'a [Value]>,
    pub(crate) ordered: Option<Vec<ProjectedSignal>>,
}

/// Project a real-valued analysis result onto the deck's output contract.
pub(crate) fn project_scalar(
    netlist: &rspice_core::Netlist,
    projection: &SignalProjection,
    request: ScalarProjectionRequest<'_>,
    abort: &dyn rspice_core::AbortSignal,
) -> Result<Vec<ScalarSignal>, rspice_core::SimulationError> {
    let ScalarProjectionRequest {
        kind,
        instance,
        axis,
        signals,
        lookup,
        ordered,
    } = request;
    let source_signals = signals
        .iter()
        .map(|signal| {
            ProjectionSourceSignal::new(
                &signal.display_name,
                &signal.raw_name,
                signal.kind,
                ProjectionValues::Real(std::borrow::Cow::Borrowed(signal.values.as_slice())),
            )
            .map_err(|error| schema_error(instance, error))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let source = ProjectionSource::new(kind, instance)
        .with_axis(axis)
        .with_signals(source_signals)
        .with_lookup(lookup)
        .with_ordered_print_columns(ordered);
    let projected = projection.project(&netlist.params, &source, abort)?;
    scalar_signals(instance, projected)
}

/// Project a complex-valued analysis result onto the deck's output contract.
pub(crate) fn project_complex(
    netlist: &rspice_core::Netlist,
    projection: &SignalProjection,
    kind: AnalysisResultKind,
    instance: &str,
    axis: &[Value],
    signals: &[ComplexSignal],
    abort: &dyn rspice_core::AbortSignal,
) -> Result<Vec<ComplexSignal>, rspice_core::SimulationError> {
    let source_signals = signals
        .iter()
        .map(|signal| {
            ProjectionSourceSignal::new(
                &signal.display_name,
                &signal.raw_name,
                signal.kind,
                ProjectionValues::Complex {
                    real: std::borrow::Cow::Borrowed(signal.real.as_slice()),
                    imag: std::borrow::Cow::Borrowed(signal.imag.as_slice()),
                },
            )
            .map_err(|error| schema_error(instance, error))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let source = ProjectionSource::new(kind, instance)
        .with_axis(axis)
        .with_signals(source_signals);
    let projected = projection.project(&netlist.params, &source, abort)?;
    complex_signals(instance, projected)
}

fn schema_error(
    instance: &str,
    error: rspice_core::execution::SignalSchemaError,
) -> rspice_core::SimulationError {
    rspice_core::SimulationError::Circuit(format!(
        "{instance} result schema cannot be described: {error}"
    ))
}

/// A projected column whose validity mask has a gap cannot be flattened into
/// a dense table column, so it fails before an artifact is written rather
/// than being padded with a plausible number.
fn ensure_dense(
    instance: &str,
    signal: &ProjectedSignal,
) -> Result<(), rspice_core::SimulationError> {
    if signal.validity().iter().all(|valid| *valid) {
        return Ok(());
    }
    Err(rspice_core::SimulationError::requested_signal_unavailable(
        signal.descriptor().display_name(),
        instance,
        Some("one or more samples are absent".to_string()),
    ))
}

fn scalar_signals(
    instance: &str,
    projected: ProjectedSignals,
) -> Result<Vec<ScalarSignal>, rspice_core::SimulationError> {
    projected
        .into_signals()
        .into_iter()
        .map(|signal| {
            ensure_dense(instance, &signal)?;
            let display_name = signal.descriptor().display_name().to_string();
            let kind = signal.descriptor().kind();
            let values = signal.real().map(<[Value]>::to_vec).ok_or_else(|| {
                rspice_core::SimulationError::Circuit(format!(
                    "{instance} projected signal '{display_name}' is complex in a real result"
                ))
            })?;
            Ok(ScalarSignal {
                raw_name: probe_registry_name(&display_name).to_string(),
                display_name,
                kind,
                values,
            })
        })
        .collect()
}

fn complex_signals(
    instance: &str,
    projected: ProjectedSignals,
) -> Result<Vec<ComplexSignal>, rspice_core::SimulationError> {
    projected
        .into_signals()
        .into_iter()
        .map(|signal| {
            ensure_dense(instance, &signal)?;
            let display_name = signal.descriptor().display_name().to_string();
            let kind = signal.descriptor().kind();
            let (real, imag) = signal.complex().ok_or_else(|| {
                rspice_core::SimulationError::Circuit(format!(
                    "{instance} projected signal '{display_name}' is real in a complex result"
                ))
            })?;
            Ok(ComplexSignal {
                raw_name: probe_registry_name(&display_name).to_string(),
                display_name,
                kind,
                real: real.to_vec(),
                imag: imag.to_vec(),
            })
        })
        .collect()
}

//=============================================================================
// Result inventories
//=============================================================================

/// Flatten one core inventory entry into the CLI's export row.
fn inventory_row(
    signal: &ProjectionSourceSignal<'_>,
) -> Result<ScalarSignal, rspice_core::SimulationError> {
    let display_name = signal.descriptor().display_name().to_string();
    let ProjectionValues::Real(values) = signal.values() else {
        return Err(rspice_core::SimulationError::Circuit(format!(
            "result signal '{display_name}' is complex in a real inventory"
        )));
    };
    Ok(ScalarSignal {
        raw_name: probe_registry_name(&display_name).to_string(),
        display_name,
        kind: signal.descriptor().kind(),
        values: values.as_ref().to_vec(),
    })
}

fn inventory_rows(
    instance: &str,
    signals: Vec<ProjectionSourceSignal<'_>>,
) -> Result<Vec<ScalarSignal>, rspice_core::SimulationError> {
    signals
        .iter()
        .map(inventory_row)
        .collect::<Result<_, _>>()
        .map_err(|error| match error {
            rspice_core::SimulationError::Circuit(message) => {
                rspice_core::SimulationError::Circuit(format!("{instance}: {message}"))
            }
            other => other,
        })
}

pub(crate) fn transient_signals(
    result: &TransientResult,
) -> Result<Vec<ScalarSignal>, rspice_core::SimulationError> {
    let signals = rspice_core::execution::transient_projection_signals(result)
        .map_err(|error| schema_error("TRAN", error))?;
    inventory_rows("TRAN", signals)
}

pub(crate) fn dc_operating_point_signals(
    result: &SimulationResult,
) -> Result<Vec<ScalarSignal>, rspice_core::SimulationError> {
    let signals = rspice_core::execution::operating_point_projection_signals(result)
        .map_err(|error| schema_error("DC OP", error))?;
    inventory_rows("DC OP", signals)
}

//=============================================================================
// Cross-coordinate schema validation
//=============================================================================

/// The typed identity of one inventory column, used to prove that later
/// coordinates carry exactly the schema the first coordinate established.
fn scalar_identity(signal: &ScalarSignal) -> Result<String, rspice_core::SimulationError> {
    Ok(scalar_descriptor(signal)
        .map_err(|error| schema_error("DC", error))?
        .canonical_name()
        .to_string())
}

fn complex_identity(signal: &ComplexSignal) -> Result<String, rspice_core::SimulationError> {
    Ok(complex_descriptor(signal)
        .map_err(|error| schema_error("AC", error))?
        .canonical_name()
        .to_string())
}

fn validate_dc_point_shape(
    result: &SimulationResult,
    point: &str,
) -> Result<(), rspice_core::SimulationError> {
    if result.node_names.len() != result.node_voltages.len() {
        return Err(rspice_core::SimulationError::Circuit(format!(
            "DC result schema is malformed at {point}: {} node names for {} voltages",
            result.node_names.len(),
            result.node_voltages.len()
        )));
    }
    if result.branch_names.len() != result.branch_currents.len() {
        return Err(rspice_core::SimulationError::Circuit(format!(
            "DC result schema is malformed at {point}: {} branch names for {} currents",
            result.branch_names.len(),
            result.branch_currents.len()
        )));
    }
    for (index, name) in result.node_names.iter().enumerate() {
        if canonical_name_is_empty(name) {
            return Err(rspice_core::SimulationError::Circuit(format!(
                "DC result schema has an empty node name at index {index} at {point}"
            )));
        }
    }
    for (index, name) in result.branch_names.iter().enumerate() {
        if canonical_name_is_empty(name) {
            return Err(rspice_core::SimulationError::Circuit(format!(
                "DC result schema has an empty branch name at index {index} at {point}"
            )));
        }
    }
    Ok(())
}

fn canonical_name_is_empty(name: &str) -> bool {
    rspice_core::execution::probe_names_nothing(name)
}

pub(crate) fn checked_dc_operating_point_signals(
    result: &SimulationResult,
) -> Result<Vec<ScalarSignal>, rspice_core::SimulationError> {
    dc_point_signals(result, "operating point")
}

fn dc_point_signals(
    result: &SimulationResult,
    point: &str,
) -> Result<Vec<ScalarSignal>, rspice_core::SimulationError> {
    validate_dc_point_shape(result, point)?;
    dc_operating_point_signals(result)
}

fn scalar_point_index(
    signals: Vec<ScalarSignal>,
    point: &str,
) -> Result<BTreeMap<String, ScalarSignal>, rspice_core::SimulationError> {
    let mut indexed = BTreeMap::new();
    for signal in signals {
        let identity = scalar_identity(&signal)?;
        let display = signal.display_name.clone();
        if indexed.insert(identity, signal).is_some() {
            return Err(rspice_core::SimulationError::Circuit(format!(
                "DC result schema contains duplicate signal '{display}' at {point}"
            )));
        }
    }
    Ok(indexed)
}

fn schema_difference(
    expected: &BTreeMap<String, String>,
    actual: &BTreeMap<String, String>,
) -> (Vec<String>, Vec<String>) {
    let missing = expected
        .iter()
        .filter(|(identity, _)| !actual.contains_key(*identity))
        .map(|(_, display)| display.clone())
        .collect();
    let unexpected = actual
        .iter()
        .filter(|(identity, _)| !expected.contains_key(*identity))
        .map(|(_, display)| display.clone())
        .collect();
    (missing, unexpected)
}

pub(crate) fn dc_sweep_signals(
    results: &[(Value, SimulationResult)],
) -> Result<Vec<ScalarSignal>, rspice_core::SimulationError> {
    let Some((first_scale, first_result)) = results.first() else {
        return Ok(Vec::new());
    };

    let first_point = format!("sweep point 1 ({first_scale:.16e})");
    let first_signals = dc_point_signals(first_result, &first_point)?;
    let expected_identities = first_signals
        .iter()
        .map(scalar_identity)
        .collect::<Result<Vec<_>, _>>()?;
    let expected_display = expected_identities
        .iter()
        .cloned()
        .zip(
            first_signals
                .iter()
                .map(|signal| signal.display_name.clone()),
        )
        .collect::<BTreeMap<_, _>>();
    let mut aggregated = first_signals
        .into_iter()
        .map(|mut signal| {
            signal.values.clear();
            signal
        })
        .collect::<Vec<_>>();

    for (point_index, (scale, result)) in results.iter().enumerate() {
        let point = format!("sweep point {} ({scale:.16e})", point_index + 1);
        let mut actual = scalar_point_index(dc_point_signals(result, &point)?, &point)?;
        let actual_display = actual
            .iter()
            .map(|(identity, signal)| (identity.clone(), signal.display_name.clone()))
            .collect::<BTreeMap<_, _>>();
        let (missing, unexpected) = schema_difference(&expected_display, &actual_display);
        if !missing.is_empty() || !unexpected.is_empty() {
            return Err(rspice_core::SimulationError::Circuit(format!(
                "DC result schema changes at {point}: missing [{}]; unexpected [{}]",
                missing.join(", "),
                unexpected.join(", ")
            )));
        }

        for (signal, identity) in aggregated.iter_mut().zip(&expected_identities) {
            let point_signal = actual.remove(identity).ok_or_else(|| {
                rspice_core::SimulationError::Circuit(format!(
                    "DC result schema lost signal '{identity}' while aggregating {point}"
                ))
            })?;
            let value = point_signal.values.first().copied().ok_or_else(|| {
                rspice_core::SimulationError::Circuit(format!(
                    "DC result signal '{identity}' has no scalar value at {point}"
                ))
            })?;
            if point_signal.values.len() != 1 {
                return Err(rspice_core::SimulationError::Circuit(format!(
                    "DC result signal '{identity}' has {} values at {point}; expected one",
                    point_signal.values.len()
                )));
            }
            signal.values.push(value);
        }
    }

    Ok(aggregated)
}

pub(crate) fn dc_sweep_voltage_signals(
    results: &[(Value, SimulationResult)],
) -> Result<Vec<ScalarSignal>, rspice_core::SimulationError> {
    Ok(dc_sweep_signals(results)?
        .into_iter()
        .filter(|signal| signal.kind == SignalKind::Voltage)
        .collect())
}

fn ac_point_signals(
    result: &AcResult,
    point: &str,
) -> Result<Vec<ComplexSignal>, rspice_core::SimulationError> {
    if result.node_names.len() != result.voltages.len() {
        return Err(rspice_core::SimulationError::Circuit(format!(
            "AC result schema is malformed at {point}: {} node names for {} voltages",
            result.node_names.len(),
            result.voltages.len()
        )));
    }
    if result.branch_names.len() != result.currents.len() {
        return Err(rspice_core::SimulationError::Circuit(format!(
            "AC result schema is malformed at {point}: {} branch names for {} currents",
            result.branch_names.len(),
            result.currents.len()
        )));
    }
    for (index, name) in result.node_names.iter().enumerate() {
        if canonical_name_is_empty(name) {
            return Err(rspice_core::SimulationError::Circuit(format!(
                "AC result schema has an empty node name at index {index} at {point}"
            )));
        }
    }
    for (index, name) in result.branch_names.iter().enumerate() {
        if canonical_name_is_empty(name) {
            return Err(rspice_core::SimulationError::Circuit(format!(
                "AC result schema has an empty branch name at index {index} at {point}"
            )));
        }
    }

    let mut signals = Vec::with_capacity(result.voltages.len() + result.currents.len());
    for (node_idx, value) in result.voltages.iter().copied().enumerate() {
        let raw_name = registry_or_ordinal(&result.node_names[node_idx], node_idx + 1);
        signals.push(ComplexSignal {
            display_name: format!("V({raw_name})"),
            raw_name,
            kind: SignalKind::Voltage,
            real: vec![value.re],
            imag: vec![value.im],
        });
    }
    for (branch_idx, value) in result.currents.iter().copied().enumerate() {
        let raw_name = registry_or_ordinal(&result.branch_names[branch_idx], branch_idx + 1);
        signals.push(ComplexSignal {
            display_name: format!("I({raw_name})"),
            raw_name,
            kind: SignalKind::Current,
            real: vec![value.re],
            imag: vec![value.im],
        });
    }
    Ok(signals)
}

fn complex_point_index(
    signals: Vec<ComplexSignal>,
    point: &str,
) -> Result<BTreeMap<String, ComplexSignal>, rspice_core::SimulationError> {
    let mut indexed = BTreeMap::new();
    for signal in signals {
        let identity = complex_identity(&signal)?;
        let display = signal.display_name.clone();
        if indexed.insert(identity, signal).is_some() {
            return Err(rspice_core::SimulationError::Circuit(format!(
                "AC result schema contains duplicate signal '{display}' at {point}"
            )));
        }
    }
    Ok(indexed)
}

pub(crate) fn ac_signals(
    results: &[AcResult],
) -> Result<Vec<ComplexSignal>, rspice_core::SimulationError> {
    let Some(first_result) = results.first() else {
        return Ok(Vec::new());
    };
    let first_point = format!("frequency point 1 ({:.16e} Hz)", first_result.frequency);
    let first_signals = ac_point_signals(first_result, &first_point)?;
    let expected_identities = first_signals
        .iter()
        .map(complex_identity)
        .collect::<Result<Vec<_>, _>>()?;
    let expected_display = expected_identities
        .iter()
        .cloned()
        .zip(
            first_signals
                .iter()
                .map(|signal| signal.display_name.clone()),
        )
        .collect::<BTreeMap<_, _>>();
    let mut aggregated = first_signals
        .into_iter()
        .map(|mut signal| {
            signal.real.clear();
            signal.imag.clear();
            signal
        })
        .collect::<Vec<_>>();

    for (point_index, result) in results.iter().enumerate() {
        let point = format!(
            "frequency point {} ({:.16e} Hz)",
            point_index + 1,
            result.frequency
        );
        let mut actual = complex_point_index(ac_point_signals(result, &point)?, &point)?;
        let actual_display = actual
            .iter()
            .map(|(identity, signal)| (identity.clone(), signal.display_name.clone()))
            .collect::<BTreeMap<_, _>>();
        let (missing, unexpected) = schema_difference(&expected_display, &actual_display);
        if !missing.is_empty() || !unexpected.is_empty() {
            return Err(rspice_core::SimulationError::Circuit(format!(
                "AC result schema changes at {point}: missing [{}]; unexpected [{}]",
                missing.join(", "),
                unexpected.join(", ")
            )));
        }

        for (signal, identity) in aggregated.iter_mut().zip(&expected_identities) {
            let point_signal = actual.remove(identity).ok_or_else(|| {
                rspice_core::SimulationError::Circuit(format!(
                    "AC result schema lost signal '{identity}' while aggregating {point}"
                ))
            })?;
            if point_signal.real.len() != 1 || point_signal.imag.len() != 1 {
                return Err(rspice_core::SimulationError::Circuit(format!(
                    "AC result signal '{identity}' is not scalar at {point}"
                )));
            }
            signal.real.push(point_signal.real[0]);
            signal.imag.push(point_signal.imag[0]);
        }
    }

    Ok(aggregated)
}

//=============================================================================
// Per-family export projection
//=============================================================================

pub(crate) fn dc_operating_point_export_signals(
    netlist: &rspice_core::Netlist,
    result: &SimulationResult,
    abort: &dyn rspice_core::AbortSignal,
) -> Result<Vec<ScalarSignal>, rspice_core::SimulationError> {
    let projection = projection(netlist)?;
    let signals = checked_dc_operating_point_signals(result)?;
    let observables = rspice_core::execution::operating_point_observable_series(result);
    let lookup = rspice_core::execution::observable_lookup(&observables);
    project_scalar(
        netlist,
        &projection,
        ScalarProjectionRequest {
            kind: AnalysisResultKind::OperatingPoint,
            instance: "DC OP",
            axis: &[0.0],
            signals: &signals,
            lookup,
            ordered: None,
        },
        abort,
    )
}

pub(crate) fn dc_export_signals(
    netlist: &rspice_core::Netlist,
    results: &[(Value, SimulationResult)],
    limits: rspice_core::ResourceLimits,
    abort: &dyn rspice_core::AbortSignal,
) -> Result<Vec<ScalarSignal>, rspice_core::SimulationError> {
    let projection = projection(netlist)?;
    let ordered = projection.ordered_dc_columns(netlist, results, limits, abort)?;
    let signals = dc_sweep_signals(results)?;
    let observables = rspice_core::execution::dc_sweep_observable_series(results);
    let lookup = rspice_core::execution::observable_lookup(&observables);
    let axis = results.iter().map(|(scale, _)| *scale).collect::<Vec<_>>();
    project_scalar(
        netlist,
        &projection,
        ScalarProjectionRequest {
            kind: AnalysisResultKind::DcSweep,
            instance: "DC",
            axis: &axis,
            signals: &signals,
            lookup,
            ordered,
        },
        abort,
    )
}

pub(crate) fn transient_export_signals(
    netlist: &rspice_core::Netlist,
    result: &TransientResult,
    limits: rspice_core::ResourceLimits,
    abort: &dyn rspice_core::AbortSignal,
) -> Result<Vec<ScalarSignal>, rspice_core::SimulationError> {
    let projection = projection(netlist)?;
    let ordered = projection.ordered_transient_columns(netlist, result, limits, abort)?;
    let signals = transient_signals(result)?;
    let lookup = rspice_core::analysis::measure_signals::transient_signal_map(result);
    project_scalar(
        netlist,
        &projection,
        ScalarProjectionRequest {
            kind: AnalysisResultKind::Transient,
            instance: "TRAN",
            axis: &result.time,
            signals: &signals,
            lookup,
            ordered,
        },
        abort,
    )
}

/// Project one already-materialized scalar inventory (PSS waveforms, a `.STEP`
/// table) onto the authored output contract.
pub(crate) fn scalar_export_signals(
    netlist: &rspice_core::Netlist,
    kind: AnalysisResultKind,
    instance: &str,
    axis: &[Value],
    signals: &[ScalarSignal],
    abort: &dyn rspice_core::AbortSignal,
) -> Result<Vec<ScalarSignal>, rspice_core::SimulationError> {
    let projection = projection(netlist)?;
    project_scalar(
        netlist,
        &projection,
        ScalarProjectionRequest {
            kind,
            instance,
            axis,
            signals,
            lookup: HashMap::new(),
            ordered: None,
        },
        abort,
    )
}

/// Project one already-materialized complex inventory (AC, distortion, HB)
/// onto the authored output contract.
pub(crate) fn complex_export_signals(
    netlist: &rspice_core::Netlist,
    kind: AnalysisResultKind,
    instance: &str,
    axis: &[Value],
    signals: &[ComplexSignal],
    abort: &dyn rspice_core::AbortSignal,
) -> Result<Vec<ComplexSignal>, rspice_core::SimulationError> {
    let projection = projection(netlist)?;
    project_complex(netlist, &projection, kind, instance, axis, signals, abort)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::Complex64;
    use rspice_core::NoAbort;

    fn dc_result(nodes: &[(&str, Value)], branches: &[(&str, Value)]) -> SimulationResult {
        let mut result = SimulationResult::new(nodes.len(), branches.len());
        result.node_names = std::iter::once("0".to_string())
            .chain(nodes.iter().map(|(name, _)| (*name).to_string()))
            .collect();
        result.node_voltages = std::iter::once(0.0)
            .chain(nodes.iter().map(|(_, value)| *value))
            .collect();
        result.branch_names = branches
            .iter()
            .map(|(name, _)| (*name).to_string())
            .collect();
        result.branch_currents = branches.iter().map(|(_, value)| *value).collect();
        result
    }

    fn ac_result(
        frequency: Value,
        nodes: &[(&str, Complex64)],
        branches: &[(&str, Complex64)],
    ) -> AcResult {
        AcResult {
            frequency,
            node_names: nodes.iter().map(|(name, _)| (*name).to_string()).collect(),
            branch_names: branches
                .iter()
                .map(|(name, _)| (*name).to_string())
                .collect(),
            voltages: nodes.iter().map(|(_, value)| *value).collect(),
            currents: branches.iter().map(|(_, value)| *value).collect(),
        }
    }

    fn scalar_values<'a>(signals: &'a [ScalarSignal], name: &str) -> &'a [Value] {
        signals
            .iter()
            .find(|signal| signal.display_name.eq_ignore_ascii_case(name))
            .unwrap_or_else(|| panic!("missing scalar signal {name}"))
            .values
            .as_slice()
    }

    fn complex_values<'a>(signals: &'a [ComplexSignal], name: &str) -> (&'a [Value], &'a [Value]) {
        let signal = signals
            .iter()
            .find(|signal| signal.display_name.eq_ignore_ascii_case(name))
            .unwrap_or_else(|| panic!("missing complex signal {name}"));
        (&signal.real, &signal.imag)
    }

    fn netlist_with_saves(saves: &str) -> rspice_core::Netlist {
        rspice_core::Netlist::parse(&format!(
            "device projection test\nV1 out 0 1\nR1 out 0 1k\n.DC V1 0 1 1\n{saves}.END\n"
        ))
        .expect("test netlist parses")
    }

    #[test]
    fn dc_device_save_materializes_authored_qualified_name() {
        let mut result = dc_result(&[("out", 1.0)], &[]);
        result.dc_observables.push(("D1:ID".to_string(), 2.5e-3));
        let netlist = netlist_with_saves(".SAVE @D1[Id]\n");

        let op = dc_operating_point_export_signals(&netlist, &result, &NoAbort)
            .expect("DC observable is available");
        assert_eq!(op.len(), 1);
        assert_eq!(op[0].display_name, "@D1[Id]");
        assert_eq!(op[0].values, [2.5e-3]);
        assert_eq!(op[0].kind, SignalKind::DeviceObservable);

        let sweep = dc_export_signals(
            &netlist,
            &[(0.0, result.clone()), (1.0, result)],
            rspice_core::ResourceLimits::default(),
            &NoAbort,
        )
        .expect("DC sweep observable is available");
        assert_eq!(sweep.len(), 1);
        assert_eq!(sweep[0].display_name, "@D1[Id]");
        assert_eq!(sweep[0].values, [2.5e-3, 2.5e-3]);
    }

    #[test]
    fn a_missing_complex_save_is_a_typed_authored_symbol_error() {
        let netlist = netlist_with_saves(".SAVE @Mdriver[Id]\n");
        let error = complex_export_signals(
            &netlist,
            AnalysisResultKind::Ac,
            "AC",
            &[1.0],
            &[],
            &NoAbort,
        )
        .expect_err("AC result has no device-observable registry");
        assert_eq!(
            error.descriptor().code,
            rspice_core::SimulationErrorCode::RequestedSignalUnavailable
        );
        let rspice_core::SimulationError::RequestedSignalUnavailable(detail) = error else {
            panic!("missing typed unavailable-signal error");
        };
        assert_eq!(detail.signal, "@Mdriver[Id]");
        assert_eq!(detail.analysis_label, "AC");

        let netlist = netlist_with_saves(".SAVE V(MissingNode)\n");
        let error = complex_export_signals(
            &netlist,
            AnalysisResultKind::Ac,
            "AC",
            &[1.0],
            &[],
            &NoAbort,
        )
        .expect_err("missing AC voltage cannot become a frequency-only result");
        let rspice_core::SimulationError::RequestedSignalUnavailable(detail) = error else {
            panic!("missing typed unavailable-voltage error");
        };
        assert_eq!(detail.signal, "V(MissingNode)");
    }

    #[test]
    fn a_scalar_save_validates_each_request_and_supports_wildcards() {
        let signals = vec![ScalarSignal {
            display_name: "V(out)".to_string(),
            raw_name: "out".to_string(),
            kind: SignalKind::Voltage,
            values: vec![1.0],
        }];
        let wildcard = netlist_with_saves(".SAVE V(o*)\n");
        let projected = scalar_export_signals(
            &wildcard,
            AnalysisResultKind::OperatingPoint,
            "DC OP",
            &[0.0],
            &signals,
            &NoAbort,
        )
        .expect("wildcard resolves one voltage");
        assert_eq!(projected.len(), 1);
        assert_eq!(projected[0].display_name, "V(out)");

        let missing = netlist_with_saves(".SAVE I(AuthoredCase)\n");
        let error = scalar_export_signals(
            &missing,
            AnalysisResultKind::OperatingPoint,
            "DC OP",
            &[0.0],
            &signals,
            &NoAbort,
        )
        .expect_err("missing current must not produce an empty result");
        let rspice_core::SimulationError::RequestedSignalUnavailable(detail) = error else {
            panic!("missing typed unavailable-current error");
        };
        assert_eq!(detail.signal, "I(AuthoredCase)");
    }

    #[test]
    fn dc_sweep_aggregation_tracks_signal_identity_across_storage_reordering() {
        let results = vec![
            (
                1.0,
                dc_result(&[("a", 1.0), ("b", 2.0)], &[("V1", 3.0), ("L1", 4.0)]),
            ),
            (
                2.0,
                dc_result(&[("b", 20.0), ("a", 10.0)], &[("L1", 40.0), ("V1", 30.0)]),
            ),
        ];

        let signals = dc_sweep_signals(&results).expect("same named schema aggregates");
        assert_eq!(scalar_values(&signals, "V(a)"), [1.0, 10.0]);
        assert_eq!(scalar_values(&signals, "V(b)"), [2.0, 20.0]);
        assert_eq!(scalar_values(&signals, "I(V1)"), [3.0, 30.0]);
        assert_eq!(scalar_values(&signals, "I(L1)"), [4.0, 40.0]);
    }

    #[test]
    fn dc_sweep_topology_change_fails_instead_of_swapping_or_zero_filling() {
        let changed_node = vec![
            (1.0, dc_result(&[("a", 1.0)], &[("V1", 2.0)])),
            (2.0, dc_result(&[("b", 3.0)], &[("V1", 4.0)])),
        ];
        let error = dc_sweep_signals(&changed_node).expect_err("renamed topology must fail closed");
        let message = error.to_string();
        assert!(message.contains("missing [V(a)]"), "{message}");
        assert!(message.contains("unexpected [V(b)]"), "{message}");

        let missing_branch = vec![
            (1.0, dc_result(&[("a", 1.0)], &[("V1", 2.0)])),
            (2.0, dc_result(&[("a", 3.0)], &[])),
        ];
        let error = dc_sweep_signals(&missing_branch)
            .expect_err("missing branch must not become a zero current");
        let message = error.to_string();
        assert!(message.contains("missing [I(V1)]"), "{message}");
    }

    #[test]
    fn malformed_dc_result_shape_is_reported_before_indexing() {
        let mut malformed = dc_result(&[("a", 1.0)], &[]);
        malformed.node_names.clear();
        let error = dc_sweep_signals(&[(1.0, malformed)]).expect_err("shape mismatch must fail");
        assert!(error.to_string().contains("0 node names for 2 voltages"));

        let mut unnamed = dc_result(&[("a", 1.0)], &[("V1", 2.0)]);
        unnamed.branch_names[0] = " I( ) ".to_string();
        let error = dc_sweep_signals(&[(1.0, unnamed)])
            .expect_err("empty canonical names must not fall back to ordinals");
        assert!(error.to_string().contains("empty branch name"));
    }

    #[test]
    fn ac_aggregation_tracks_names_and_refuses_schema_changes() {
        let first = ac_result(
            1.0,
            &[
                ("a", Complex64::new(1.0, 2.0)),
                ("b", Complex64::new(3.0, 4.0)),
            ],
            &[("V1", Complex64::new(5.0, 6.0))],
        );
        let reordered = ac_result(
            2.0,
            &[
                ("b", Complex64::new(30.0, 40.0)),
                ("a", Complex64::new(10.0, 20.0)),
            ],
            &[("V1", Complex64::new(50.0, 60.0))],
        );
        let signals = ac_signals(&[first.clone(), reordered]).expect("same AC schema aggregates");
        assert_eq!(
            complex_values(&signals, "V(a)"),
            (&[1.0, 10.0][..], &[2.0, 20.0][..])
        );
        assert_eq!(
            complex_values(&signals, "V(b)"),
            (&[3.0, 30.0][..], &[4.0, 40.0][..])
        );
        assert_eq!(
            complex_values(&signals, "I(V1)"),
            (&[5.0, 50.0][..], &[6.0, 60.0][..])
        );

        let changed = ac_result(
            2.0,
            &[
                ("c", Complex64::new(7.0, 8.0)),
                ("b", Complex64::new(9.0, 10.0)),
            ],
            &[("V1", Complex64::new(11.0, 12.0))],
        );
        let error =
            ac_signals(&[first.clone(), changed]).expect_err("changed AC schema must fail closed");
        let message = error.to_string();
        assert!(message.contains("missing [V(a)]"), "{message}");
        assert!(message.contains("unexpected [V(c)]"), "{message}");

        let reordered_branches = ac_result(
            2.0,
            &[
                ("a", Complex64::new(7.0, 8.0)),
                ("b", Complex64::new(9.0, 10.0)),
            ],
            &[
                ("L1", Complex64::new(30.0, 40.0)),
                ("V1", Complex64::new(50.0, 60.0)),
            ],
        );
        let first_with_branches = ac_result(
            1.0,
            &[
                ("a", Complex64::new(1.0, 2.0)),
                ("b", Complex64::new(3.0, 4.0)),
            ],
            &[
                ("V1", Complex64::new(5.0, 6.0)),
                ("L1", Complex64::new(7.0, 8.0)),
            ],
        );
        let signals = ac_signals(&[first_with_branches, reordered_branches])
            .expect("named AC branch currents aggregate after reordering");
        assert_eq!(
            complex_values(&signals, "I(V1)"),
            (&[5.0, 50.0][..], &[6.0, 60.0][..])
        );
        assert_eq!(
            complex_values(&signals, "I(L1)"),
            (&[7.0, 30.0][..], &[8.0, 40.0][..])
        );

        let mut missing_branch = first.clone();
        missing_branch.branch_names.clear();
        missing_branch.currents.clear();
        let error = ac_signals(&[first.clone(), missing_branch])
            .expect_err("missing AC current must not become a zero");
        assert!(error.to_string().contains("missing [I(V1)]"));

        let mut unnamed = first;
        unnamed.node_names[0] = "V( )".to_string();
        let error = ac_signals(&[unnamed])
            .expect_err("empty canonical AC names must not fall back to ordinals");
        assert!(error.to_string().contains("empty node name"));
    }
}
