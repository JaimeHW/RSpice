//! Semiconductor device models and shared junction equations.
//!
//! Includes diodes and bipolar junction transistors (BJTs).

mod bjt;
mod diode;
mod limiting;

#[cfg(test)]
pub(crate) use bjt::BJT_ACCEPTED_NONLINEAR_RUNTIME_TAG;
pub(crate) use bjt::{
    AcceptedBjtChargeSnapshotCheckpoint, AcceptedBjtNonlinearCheckpoint,
    BJT_ACCEPTED_CHARGE_SNAPSHOT_STATE_VALUE_COUNT, BJT_ACCEPTED_NONLINEAR_STATE_VALUE_COUNT,
    BJT_DYNAMIC_CHARGE_COUNT, BJT_EXTERNAL_STATE_DIM, BJT_INTERNAL_STATE_DIM, BjtChargeBranch,
    BjtChargeSnapshot, VBIC_ACCEPTED_NONLINEAR_STATE_VALUE_COUNT,
    VBIC_TRANSIENT_CONVERGENCE_BRANCH_COUNT,
};
pub use bjt::{Bjt, BjtType};
pub(crate) use diode::{
    AcceptedDiodeNonlinearCheckpoint, DIODE_ACCEPTED_NONLINEAR_RUNTIME_TAG, DiodeNonlinearState,
    ResolvedDiodeJunction,
};
pub use diode::{Diode, DiodeLevel};

/// Graded-junction depletion charge and its voltage derivative. The forward
/// continuation is anchored at the knee so both quantities remain continuous.
/// Callers own model validation and temperature/geometry scaling.
pub(crate) fn depletion_charge_and_capacitance(
    voltage: crate::Value,
    capacitance: crate::Value,
    potential: crate::Value,
    grading: crate::Value,
    forward_coefficient: crate::Value,
) -> (crate::Value, crate::Value) {
    if capacitance == 0.0 {
        return (0.0, 0.0);
    }
    if grading == 0.0 {
        return (capacitance * voltage, capacitance);
    }
    let knee = forward_coefficient * potential;
    let depletion_voltage = voltage.min(knee);
    let log = (-depletion_voltage / potential).ln_1p();
    let charge = if grading == 1.0 {
        -capacitance * potential * log
    } else {
        -capacitance * potential * ((1.0 - grading) * log).exp_m1() / (1.0 - grading)
    };
    let slope = capacitance * (-grading * log).exp();
    if voltage < knee {
        (charge, slope)
    } else {
        let offset = voltage - knee;
        let slope_change = slope * (grading * (offset / potential) / (1.0 - forward_coefficient));
        (
            charge + offset * (slope + 0.5 * slope_change),
            slope + slope_change,
        )
    }
}
