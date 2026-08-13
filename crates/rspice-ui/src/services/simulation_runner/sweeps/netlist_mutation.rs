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

/// Apply the temperature and supply values of one authenticated Run Set point
/// to a parsed deck.
///
/// Keeping this mutation in the service layer gives config-backed and
/// spec-driven analyses one validation contract. In particular, Monte Carlo
/// must apply the point before every trial is solved rather than merely label
/// a nominal distribution with PVT metadata.
pub(crate) fn apply_run_environment(
    netlist: &mut rspice_core::Netlist,
    temperature_celsius: Value,
    supply_voltage: Option<Value>,
    nominal_supply_voltage: Option<Value>,
    supply_source_names: &[String],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<()> {
    ensure_not_aborted(abort)?;
    if !temperature_celsius.is_finite()
        || rspice_core::constants::celsius_to_kelvin(temperature_celsius) <= 0.0
    {
        return Err(ServiceRunError::Failure(
            "Run Set temperature must be finite and above absolute zero".to_owned(),
        ));
    }
    netlist.options.temp = Some(temperature_celsius);
    match (supply_voltage, nominal_supply_voltage) {
        (Some(supply), Some(nominal)) => {
            apply_voltage_corner(netlist, supply, nominal, supply_source_names, abort)?
        }
        (None, None) => {}
        _ => {
            return Err(ServiceRunError::Failure(
                "Run Set supply and nominal voltage must be provided together".to_owned(),
            ));
        }
    }
    ensure_not_aborted(abort)
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
