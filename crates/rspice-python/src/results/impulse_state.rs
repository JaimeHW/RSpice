//! Lossless, versioned persistence of physical current ownership and charge.
use pyo3::prelude::*;
use rspice_core::{CurrentImpulseOwner, CurrentImpulsePoint, CurrentImpulseTrace};

/// Readable, immutable snapshot independent of finite waveform retention.
#[pyclass(
    name = "CurrentImpulseTrace",
    module = "rspice",
    frozen,
    skip_from_py_object
)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PyCurrentImpulseTrace {
    #[pyo3(get)]
    owner_kind: String,
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    parameter: Option<String>,
    #[pyo3(get)]
    complete: bool,
    #[pyo3(get)]
    points: Vec<(f64, f64)>,
}

#[pymethods]
impl PyCurrentImpulseTrace {
    fn __repr__(&self) -> String {
        format!(
            "CurrentImpulseTrace(owner_kind={:?}, name={:?}, parameter={:?}, complete={}, points={})",
            self.owner_kind,
            self.name,
            self.parameter,
            self.complete,
            self.points.len()
        )
    }
}

type ImpulseRow = (String, String, Option<String>, bool, Vec<(f64, f64)>);
type ImpulseRows = Option<Vec<ImpulseRow>>;
type LegacyImpulseRows = Option<Vec<(String, Vec<(f64, f64)>)>>;
pub(super) type ImpulsePersistenceState = (usize, ImpulseRows);

#[derive(Debug, Clone, pyo3::FromPyObject)]
pub(super) enum VersionedImpulseState {
    #[pyo3(transparent)]
    Current(ImpulsePersistenceState),
    #[pyo3(transparent)]
    Legacy((usize, LegacyImpulseRows)),
}

fn owner_fields(owner: &CurrentImpulseOwner) -> (String, String, Option<String>) {
    match owner {
        CurrentImpulseOwner::Branch { branch_name } => ("branch".into(), branch_name.clone(), None),
        CurrentImpulseOwner::DeviceLead {
            device_name,
            parameter,
        } => (
            "device_lead".into(),
            device_name.clone(),
            Some(parameter.clone()),
        ),
    }
}

pub(super) fn impulse_rows(
    traces: Option<&[CurrentImpulseTrace]>,
) -> Option<Vec<PyCurrentImpulseTrace>> {
    traces.map(|traces| {
        traces
            .iter()
            .map(|trace| {
                let (owner_kind, name, parameter) = owner_fields(&trace.owner);
                PyCurrentImpulseTrace {
                    owner_kind,
                    name,
                    parameter,
                    complete: trace.complete,
                    points: trace
                        .points
                        .iter()
                        .map(|point| (point.time, point.charge_coulombs))
                        .collect(),
                }
            })
            .collect()
    })
}

pub(super) fn impulse_persistence_state(
    traces: Option<&[CurrentImpulseTrace]>,
) -> ImpulsePersistenceState {
    (
        2,
        traces.map(|traces| {
            traces
                .iter()
                .map(|trace| {
                    let (kind, name, parameter) = owner_fields(&trace.owner);
                    (
                        kind,
                        name,
                        parameter,
                        trace.complete,
                        trace
                            .points
                            .iter()
                            .map(|point| (point.time, point.charge_coulombs))
                            .collect(),
                    )
                })
                .collect()
        }),
    )
}

/// Old rows retain their observations without claiming complete ownership.
/// The caller validates times, charges and ownership against the rebuilt result.
pub(super) fn restore_impulses(
    state: Option<VersionedImpulseState>,
) -> Result<Option<Vec<CurrentImpulseTrace>>, String> {
    let rows = match state {
        None => return Ok(None),
        Some(VersionedImpulseState::Current((2, rows))) => rows,
        // Python's empty/None containers fit both shapes. Preserve v1's lack
        // of owner coverage, regardless of which extractor matched first.
        Some(VersionedImpulseState::Current((1, rows)))
            if rows.as_ref().is_none_or(Vec::is_empty) =>
        {
            rows
        }
        Some(VersionedImpulseState::Legacy((1, rows))) => rows.map(|rows| {
            rows.into_iter()
                .map(|(name, points)| ("branch".into(), name, None, false, points))
                .collect()
        }),
        Some(
            VersionedImpulseState::Current((version, _))
            | VersionedImpulseState::Legacy((version, _)),
        ) => {
            return Err(format!(
                "unsupported current impulse pickle version or row shape {version}; expected version 1 legacy rows or version 2 owned rows"
            ));
        }
    };
    rows.map(|rows| {
        rows.into_iter()
            .map(|(kind, name, parameter, complete, points)| {
                let owner = match (kind.as_str(), parameter) {
                    ("branch", None) => CurrentImpulseOwner::Branch { branch_name: name },
                    ("device_lead", Some(parameter)) => CurrentImpulseOwner::DeviceLead {
                        device_name: name,
                        parameter,
                    },
                    _ => return Err("invalid current impulse owner kind or parameter".into()),
                };
                Ok(CurrentImpulseTrace {
                    owner,
                    complete,
                    points: points
                        .into_iter()
                        .map(|(time, charge_coulombs)| CurrentImpulsePoint {
                            time,
                            charge_coulombs,
                        })
                        .collect(),
                })
            })
            .collect()
    })
    .transpose()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn impulse_pickle_preserves_availability_sign_float_bits_and_coverage() {
        let traces = vec![
            CurrentImpulseTrace {
                owner: CurrentImpulseOwner::Branch {
                    branch_name: "Vdrive".into(),
                },
                complete: false,
                points: vec![
                    CurrentImpulsePoint {
                        time: f64::from_bits(1),
                        charge_coulombs: -f64::from_bits(1),
                    },
                    CurrentImpulsePoint {
                        time: 1.0,
                        charge_coulombs: 1.25e-18,
                    },
                ],
            },
            CurrentImpulseTrace {
                owner: CurrentImpulseOwner::DeviceLead {
                    device_name: "Q1".into(),
                    parameter: "ic".into(),
                },
                complete: true,
                points: vec![],
            },
        ];
        for input in [None, Some(Vec::new()), Some(traces)] {
            let restored = restore_impulses(Some(VersionedImpulseState::Current(
                impulse_persistence_state(input.as_deref()),
            )))
            .unwrap();
            assert_eq!(restored, input);
        }
        assert_eq!(restore_impulses(None).unwrap(), None);
        for rows in [None, Some(vec![])] {
            assert_eq!(
                restore_impulses(Some(VersionedImpulseState::Current((1, rows.clone())))).unwrap(),
                rows.map(|_| vec![])
            );
        }
        let restored = restore_impulses(Some(VersionedImpulseState::Legacy((
            1,
            Some(vec![("Vdrive".into(), vec![(0.0, -1e-12)])]),
        ))))
        .unwrap()
        .unwrap();
        assert!(!restored[0].complete);
        for version in [0, 3, usize::MAX] {
            assert!(
                restore_impulses(Some(VersionedImpulseState::Current((version, None)))).is_err()
            );
        }
    }
}
