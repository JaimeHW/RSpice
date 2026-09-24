//! Applying a sweep point to the deck.
//!
//! Rewrites the netlist for one corner: supply voltages, temperature, and
//! model section selection. Supply inference is explicit rather than
//! guessed from the largest source.

use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
use rspice_core::netlist::{ElementKind, SourceSpec};

use super::super::error::{
    ServiceRunError, ServiceRunResult, ensure_not_aborted, poll_periodically,
};

pub(crate) fn apply_voltage_corner(
    netlist: &mut rspice_core::Netlist,
    corner_voltage: Value,
    nominal_voltage: Value,
    supply_source_names: &[String],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<()> {
    rspice_core::engine::apply_supply_voltage_scale_with_abort(
        netlist,
        corner_voltage,
        nominal_voltage,
        supply_source_names,
        abort,
    )
    .map_err(|error| match error {
        rspice_core::SimulationError::Aborted => ServiceRunError::Aborted,
        error => ServiceRunError::Failure(error.to_string()),
    })
}

pub(crate) fn infer_nominal_supply_voltage(
    netlist: &rspice_core::Netlist,
    supply_source_names: &[String],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Option<Value>> {
    ensure_not_aborted(abort)?;
    if supply_source_names.is_empty() {
        return Err(ServiceRunError::Failure(
            "Nominal supply resolution requires at least one explicitly bound source".to_owned(),
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    let mut values = Vec::with_capacity(supply_source_names.len());
    for (index, source_name) in supply_source_names.iter().enumerate() {
        poll_periodically(abort, index)?;
        if source_name.trim().is_empty()
            || source_name != source_name.trim()
            || source_name.chars().any(char::is_control)
            || !seen.insert(source_name.to_ascii_lowercase())
        {
            return Err(ServiceRunError::Failure(format!(
                "Supply source binding {source_name:?} is malformed or duplicated"
            )));
        }
        let Some(element) = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(source_name))
        else {
            return Err(ServiceRunError::Failure(format!(
                "Bound supply source {source_name:?} is absent from the executable netlist"
            )));
        };
        let ElementKind::VoltageSource(spec) = &element.kind else {
            return Err(ServiceRunError::Failure(format!(
                "Bound supply source {source_name:?} is not an independent voltage source"
            )));
        };
        let Some(dc) = dc_value_from_source(spec) else {
            return Err(ServiceRunError::Failure(format!(
                "Bound supply source {source_name:?} has no scalable DC value"
            )));
        };
        let abs_dc = dc.abs();
        if abs_dc <= 1e-15 {
            return Err(ServiceRunError::Failure(format!(
                "Bound supply source {source_name:?} has a zero nominal magnitude"
            )));
        }
        values.push(abs_dc);
    }
    ensure_not_aborted(abort)?;
    Ok(values.into_iter().max_by(|a, b| a.total_cmp(b)))
}

fn dc_value_from_source(spec: &SourceSpec) -> Option<Value> {
    match spec {
        SourceSpec::Dc(v) => Some(*v),
        SourceSpec::DcAc { dc_value, .. } => Some(*dc_value),
        _ => None,
    }
}
