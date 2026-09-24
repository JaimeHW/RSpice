//! Exact singular-current contributions to the carrier least-squares moments.
use super::*;

pub(super) fn projection_prefixes(
    trace: &rspice_core::CurrentImpulseTrace,
    time: &[Value],
    frequencies: &[Value],
    basis: &[EnvelopeProjectionBasis],
    window: Value,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<Vec<Value>>> {
    if !trace.complete {
        return Err(ServiceRunError::Failure(format!(
            "Envelope current '{}' has incomplete impulse history",
            trace.owner
        )));
    }
    trace
        .validate(time[0], time[time.len() - 1])
        .map_err(ServiceRunError::Failure)?;
    let mut prefixes = Vec::with_capacity(basis.len());
    for &component in basis {
        ensure_not_aborted(abort)?;
        let mut prefix = Vec::with_capacity(trace.points.len() + 1);
        prefix.push(0.0);
        let mut sum = 0.0;
        let mut correction = 0.0;
        for (index, point) in trace.points.iter().enumerate() {
            poll_periodically(abort, index)?;
            // Charge/window has current units. Never convert an impulse into
            // a finite sample by dividing it by an integration timestep.
            let normalized = point.charge_coulombs / window;
            if !normalized.is_finite() || normalized == 0.0 {
                return Err(ServiceRunError::Failure(
                    "Envelope impulse charge per window is not representable".into(),
                ));
            }
            let value = normalized * projection_basis_value(component, frequencies, point.time);
            let adjusted = value - correction;
            let next = sum + adjusted;
            correction = (next - sum) - adjusted;
            sum = next;
            if !sum.is_finite() {
                return Err(ServiceRunError::Failure(
                    "Envelope current impulse projection overflowed".into(),
                ));
            }
            prefix.push(sum);
        }
        prefixes.push(prefix);
    }
    Ok(prefixes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::{CurrentImpulseOwner, CurrentImpulsePoint, CurrentImpulseTrace};

    #[test]
    fn envelope_current_projection_integrates_exact_charge_and_window_boundaries() {
        let time = [0.0, 0.25, 0.5, 0.75, 1.0];
        let values = [0.0; 5];
        let mut trace = CurrentImpulseTrace {
            owner: CurrentImpulseOwner::Branch {
                branch_name: "V1".into(),
            },
            complete: true,
            points: vec![
                CurrentImpulsePoint {
                    time: 0.0,
                    charge_coulombs: 7.0,
                },
                CurrentImpulsePoint {
                    time: 0.125,
                    charge_coulombs: 2.0,
                },
                CurrentImpulsePoint {
                    time: 1.0,
                    charge_coulombs: -1.0,
                },
            ],
        };
        let projected = compute_carrier_envelopes_with_abort(
            &time,
            &values,
            &[0.5],
            &[1.0],
            Some(&trace),
            &NoAbort,
        )
        .unwrap();
        let expected = 4.0 * Complex64::from_polar(1.0, -std::f64::consts::FRAC_PI_4) - 2.0;
        assert!((projected[0][0] - expected).norm() < 1e-12);
        trace.complete = false;
        assert!(
            compute_carrier_envelopes_with_abort(
                &time,
                &values,
                &[0.5],
                &[1.0],
                Some(&trace),
                &NoAbort
            )
            .unwrap_err()
            .to_string()
            .contains("incomplete")
        );
        trace.complete = true;
        let cancelled = rspice_core::abort_signal::CountingAbort::new(0);
        assert!(matches!(
            compute_carrier_envelopes_with_abort(
                &time,
                &values,
                &[0.5],
                &[1.0],
                Some(&trace),
                &cancelled
            ),
            Err(ServiceRunError::Aborted)
        ));
    }
}
