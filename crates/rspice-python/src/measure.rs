//! .MEAS evaluation against simulation results.
//!
//! Thin adapter over the shared harness in
//! `rspice_core::analysis::measure_signals`, which builds the signal table
//! (voltages, branch currents, the TIME axis) and evaluates the netlist's
//! measurement statements. Only the mapping to Python result objects lives
//! here.

use rspice_core::analysis::advanced::measure_signals;
use rspice_core::engine::TransientResult;
use rspice_core::solver::SimulationResult;
use rspice_core::Netlist;

use crate::results::PyMeasurement;

/// Evaluate the netlist's transient .MEAS statements against a result.
pub(crate) fn evaluate_tran_measurements(
    netlist: &Netlist,
    result: &TransientResult,
) -> Vec<PyMeasurement> {
    measure_signals::evaluate_tran_measurements(netlist, result)
        .iter()
        .map(|r| PyMeasurement::from_core(r, "TRAN"))
        .collect()
}

/// Evaluate the netlist's DC .MEAS statements against a sweep.
pub(crate) fn evaluate_dc_measurements(
    netlist: &Netlist,
    sweep: &[(f64, SimulationResult)],
) -> Vec<PyMeasurement> {
    measure_signals::evaluate_dc_measurements(netlist, sweep)
        .iter()
        .map(|r| PyMeasurement::from_core(r, "DC"))
        .collect()
}

/// Evaluate the netlist's AC .MEAS statements against a sweep.
///
/// Complex results are addressed through the derived real series
/// (`V(x)` magnitude, `VDB`, `VP` in degrees, `VR`/`VI`); the frequency
/// axis is reachable as `TIME`, `FREQUENCY`, or `FREQ`.
pub(crate) fn evaluate_ac_measurements(
    netlist: &Netlist,
    sweep: &[rspice_core::analysis::AcResult],
) -> Vec<PyMeasurement> {
    measure_signals::evaluate_ac_measurements(netlist, sweep)
        .iter()
        .map(|r| PyMeasurement::from_core(r, "AC"))
        .collect()
}

/// Produce explicit not-evaluated entries for measurements whose analysis
/// did not run, so CI fails loudly instead of silently skipping checks.
pub(crate) fn unevaluated_measurements(
    netlist: &Netlist,
    analysis: &str,
    reason: &str,
) -> Vec<PyMeasurement> {
    measure_signals::measurements_for_analysis(netlist, analysis)
        .iter()
        .map(|m| PyMeasurement::unevaluated(&m.name, analysis, reason))
        .collect()
}
