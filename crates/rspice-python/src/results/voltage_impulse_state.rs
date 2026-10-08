//! Versioned voltage observations, with units independent of current charge.
use pyo3::prelude::*;
use rspice_core::{VoltageImpulseDerivative, VoltageImpulsePoint, VoltageImpulseTrace};

#[pyclass(
    name = "VoltageImpulseTrace",
    module = "rspice",
    frozen,
    skip_from_py_object
)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PyVoltageImpulseTrace {
    #[pyo3(get)]
    node_name: String,
    #[pyo3(get)]
    complete: bool,
    #[pyo3(get)]
    points: Vec<(f64, f64)>,
    #[pyo3(get)]
    derivatives: Vec<(f64, u32, f64)>,
}

type VoltageImpulseRow = (String, bool, Vec<(f64, f64)>, Vec<(f64, u32, f64)>);
pub(super) type VoltageImpulsePersistenceState = (usize, Option<Vec<VoltageImpulseRow>>);

pub(super) fn voltage_impulse_rows(
    traces: Option<&[VoltageImpulseTrace]>,
) -> Option<Vec<PyVoltageImpulseTrace>> {
    voltage_impulse_persistence_state(traces).1.map(|rows| {
        rows.into_iter()
            .map(
                |(node_name, complete, points, derivatives)| PyVoltageImpulseTrace {
                    node_name,
                    complete,
                    points,
                    derivatives,
                },
            )
            .collect()
    })
}

pub(super) fn voltage_impulse_persistence_state(
    traces: Option<&[VoltageImpulseTrace]>,
) -> VoltageImpulsePersistenceState {
    (
        1,
        traces.map(|traces| {
            traces
                .iter()
                .map(|trace| {
                    (
                        trace.node_name.clone(),
                        trace.complete,
                        trace
                            .points
                            .iter()
                            .map(|p| (p.time, p.volt_seconds))
                            .collect(),
                        trace
                            .derivatives
                            .iter()
                            .map(|p| (p.time, p.order, p.coefficient))
                            .collect(),
                    )
                })
                .collect()
        }),
    )
}

/// The rebuilt result validates ownership, ordering and its retained extent.
pub(super) fn restore_voltage_impulses(
    state: Option<VoltageImpulsePersistenceState>,
) -> Result<Option<Vec<VoltageImpulseTrace>>, String> {
    let Some((version, rows)) = state else {
        return Ok(None);
    };
    if version != 1 {
        return Err(format!(
            "unsupported voltage impulse pickle version {version}; expected 1"
        ));
    }
    Ok(rows.map(|rows| {
        rows.into_iter()
            .map(
                |(node_name, complete, points, derivatives)| VoltageImpulseTrace {
                    node_name,
                    complete,
                    points: points
                        .into_iter()
                        .map(|(time, volt_seconds)| VoltageImpulsePoint { time, volt_seconds })
                        .collect(),
                    derivatives: derivatives
                        .into_iter()
                        .map(|(time, order, coefficient)| VoltageImpulseDerivative {
                            time,
                            order,
                            coefficient,
                        })
                        .collect(),
                },
            )
            .collect()
    }))
}
