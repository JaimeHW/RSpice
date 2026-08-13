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
    abort: &dyn AbortSignal,
) -> ServiceRunResult<()> {
    rspice_core::engine::apply_supply_voltage_scale_with_abort(
        netlist,
        corner_voltage,
        nominal_voltage,
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
        (Some(supply), Some(nominal)) => apply_voltage_corner(netlist, supply, nominal, abort)?,
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
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Option<Value>> {
    ensure_not_aborted(abort)?;
    let mut ground_referenced = Vec::new();
    let mut all_sources = Vec::new();

    for (index, element) in netlist.elements.iter().enumerate() {
        poll_periodically(abort, index)?;
        if let ElementKind::VoltageSource(spec) = &element.kind
            && let Some(dc) = dc_value_from_source(spec)
        {
            let abs_dc = dc.abs();
            if abs_dc <= 1e-15 {
                continue;
            }
            all_sources.push(abs_dc);
            if element
                .nodes
                .get(1)
                .map(|name| is_ground_node(name))
                .unwrap_or(false)
            {
                ground_referenced.push(abs_dc);
            }
        }
    }

    if !ground_referenced.is_empty() {
        ensure_not_aborted(abort)?;
        return Ok(ground_referenced.into_iter().max_by(|a, b| a.total_cmp(b)));
    }
    ensure_not_aborted(abort)?;
    Ok(all_sources.into_iter().max_by(|a, b| a.total_cmp(b)))
}

fn is_ground_node(node: &str) -> bool {
    let n = node.trim();
    n == "0" || n.eq_ignore_ascii_case("gnd") || n.eq_ignore_ascii_case("ground")
}

fn dc_value_from_source(spec: &SourceSpec) -> Option<Value> {
    match spec {
        SourceSpec::Dc(v) => Some(*v),
        SourceSpec::DcAc { dc_value, .. } => Some(*dc_value),
        _ => None,
    }
}
