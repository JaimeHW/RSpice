//! Lossless, versioned persistence of sparse current impulses.

use rspice_core::{CurrentImpulsePoint, CurrentImpulseTrace};

/// Branch names and exact (seconds, coulombs) pairs. None means unavailable.
pub(super) type ImpulseRows = Option<Vec<(String, Vec<(f64, f64)>)>>;
/// Version 1 retains availability as well as every signed charge and time.
pub(super) type ImpulsePersistenceState = (usize, ImpulseRows);

pub(super) fn impulse_rows(traces: Option<&[CurrentImpulseTrace]>) -> ImpulseRows {
    traces.map(|traces| {
        traces
            .iter()
            .map(|trace| {
                (
                    trace.branch_name.clone(),
                    trace
                        .points
                        .iter()
                        .map(|point| (point.time, point.charge_coulombs))
                        .collect(),
                )
            })
            .collect()
    })
}

pub(super) fn impulse_persistence_state(
    traces: Option<&[CurrentImpulseTrace]>,
) -> ImpulsePersistenceState {
    (1, impulse_rows(traces))
}

/// Old pickles have no state and therefore no recorded impulse history.
/// The caller validates times, charges and ownership against the rebuilt result.
pub(super) fn restore_impulses(
    state: Option<ImpulsePersistenceState>,
) -> Result<Option<Vec<CurrentImpulseTrace>>, String> {
    let Some((version, rows)) = state else {
        return Ok(None);
    };
    if version != 1 {
        return Err(format!(
            "unsupported current impulse pickle version {version}; expected 1"
        ));
    }
    Ok(rows.map(|traces| {
        traces
            .into_iter()
            .map(|(branch_name, points)| CurrentImpulseTrace {
                branch_name,
                points: points
                    .into_iter()
                    .map(|(time, charge_coulombs)| CurrentImpulsePoint {
                        time,
                        charge_coulombs,
                    })
                    .collect(),
            })
            .collect()
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn impulse_pickle_preserves_availability_sign_and_float_bits() {
        let traces = vec![CurrentImpulseTrace {
            branch_name: "Vdrive".into(),
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
        }];
        for input in [None, Some(Vec::new()), Some(traces)] {
            let restored =
                restore_impulses(Some(impulse_persistence_state(input.as_deref()))).unwrap();
            assert_eq!(restored, input);
        }
        assert_eq!(restore_impulses(None).unwrap(), None);
        for version in [0, 2, usize::MAX] {
            assert!(restore_impulses(Some((version, None))).is_err());
        }
    }
}
