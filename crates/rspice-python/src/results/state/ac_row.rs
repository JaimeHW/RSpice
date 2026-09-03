//! Pickle state of one AC frequency row, and of the distortion products that
//! are made of them.
//!
//! A `.DISTO` product is a labelled sequence of AC rows, so it shares this
//! codec rather than duplicating the row encoding.

use super::*;

/// Complete Python-visible state of one core `AcResult` row.
pub(crate) type AcRowState = (
    f64,
    Vec<String>,
    Vec<String>,
    Vec<(f64, f64)>,
    Vec<(f64, f64)>,
);

pub(crate) fn ac_row_state(row: &AcResult) -> AcRowState {
    (
        row.frequency,
        row.node_names.clone(),
        row.branch_names.clone(),
        complex_state(&row.voltages),
        complex_state(&row.currents),
    )
}

pub(crate) fn rebuild_ac_row(state: AcRowState) -> AcResult {
    let (frequency, node_names, branch_names, voltages, currents) = state;
    AcResult {
        frequency,
        node_names,
        branch_names,
        voltages: complex_from_state(voltages),
        currents: complex_from_state(currents),
    }
}

/// One distortion product and its per-F1-point rows, keyed by stable label.
pub(crate) type DistortionProductState = (String, Vec<AcRowState>);

/// Distortion products travel as the same stable labels the accessors accept.
pub(crate) fn distortion_product_from_label(label: &str) -> PyResult<DistortionProduct> {
    match label {
        "2f1" => Ok(DistortionProduct::SecondHarmonic),
        "3f1" => Ok(DistortionProduct::ThirdHarmonic),
        "f1+f2" => Ok(DistortionProduct::Sum),
        "f1-f2" => Ok(DistortionProduct::Difference),
        "2f1-f2" => Ok(DistortionProduct::ThirdOrderDifference),
        other => Err(crate::errors::value_error(format!(
            "unknown distortion product '{other}' in pickled state"
        ))),
    }
}
