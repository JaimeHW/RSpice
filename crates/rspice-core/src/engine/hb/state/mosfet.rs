//! Path-dependent Meyer gate charge over retained carrier history intervals.
use super::*;
use crate::engine::transient::MosfetPeriodicChargeIncrements;

impl Engine {
    pub(super) fn hb_periodic_meyer_increments(
        &self,
        circuit: &CircuitData,
        result: &HbResult,
        phase_step: Value,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<MosfetPeriodicChargeIncrements>, SimulationError> {
        let mut result_increments = Vec::with_capacity(circuit.mosfets.len());
        let mut evaluations = 0usize;
        for device in &circuit.mosfets.devices {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            if device.uses_legacy_bsim() {
                result_increments.push(MosfetPeriodicChargeIncrements::default());
                continue;
            }
            let spectra = [device.node_gate, device.node_drain, device.node_bulk]
                .map(|node| Self::hb_terminal_voltage_spectrum(result, node, device.node_source));
            let amplitude = spectra
                .iter()
                .map(|row| row.iter().skip(1).map(|v| v.norm()).sum::<Value>())
                .fold(0.0, Value::max);
            let (gs, gd, gb) = device.overlap_capacitances();
            let charge_scale =
                (device.oxide_capacitance_total().abs() + gs.abs() + gd.abs() + gb.abs())
                    * amplitude;
            let absolute = self.charge_abstol().min(charge_scale * 1e-10);
            let relative = self.voltage_reltol().min(1e-8);
            let mut evaluate = |phase| {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                evaluations = evaluations.saturating_add(1);
                crate::ResourceLimitError::ensure(
                    crate::ResourceKind::AnalysisPoints,
                    evaluations,
                    self.config.resource_limits.max_analysis_points,
                )?;
                let rate = gate_phase_rates(device, &spectra, phase);
                if rate.iter().any(|value| !value.is_finite()) {
                    return Err(SimulationError::Circuit(format!(
                        "MOSFET '{}' produced non-finite periodic gate current",
                        device.name
                    )));
                }
                Ok(rate)
            };
            let mut integrate = |start, stop| {
                let whole = gauss8(&mut evaluate, start, stop)?;
                refine(&mut evaluate, start, stop, whole, [absolute, relative], 14)
            };
            result_increments.push(MosfetPeriodicChargeIncrements {
                older_to_previous: integrate(-2.0 * phase_step, -phase_step)?,
                previous_to_current: integrate(-phase_step, 0.0)?,
            });
        }
        Ok(result_increments)
    }
}

/// Integrate in phase: C(v)*dV/dtheta has charge units and includes any
/// nonzero mean displacement current. No periodic charge potential is assumed.
fn gate_phase_rates(
    device: &crate::device::Mosfet,
    spectra: &[Vec<Complex64>; 3],
    phase: Value,
) -> [Value; 3] {
    let samples = spectra.each_ref().map(|row| {
        row.iter()
            .enumerate()
            .fold((0.0, 0.0), |(value, rate), (k, c)| {
                let phasor = c * Complex64::from_polar(1.0, k as Value * phase);
                (value + phasor.re, rate - k as Value * phasor.im)
            })
    });
    let (gs, gd, gb) = device.ac_capacitances_at(samples[0].0, samples[1].0, samples[2].0);
    [
        gs * samples[0].1,
        gd * (samples[0].1 - samples[1].1),
        gb * (samples[0].1 - samples[2].1),
    ]
}

fn gauss8(
    evaluate: &mut impl FnMut(Value) -> Result<[Value; 3], SimulationError>,
    start: Value,
    stop: Value,
) -> Result<[Value; 3], SimulationError> {
    let mid = (start + stop) * 0.5;
    let half = (stop - start) * 0.5;
    let mut sum = [0.0; 3];
    for (node, weight) in [
        (0.183_434_642_495_649_8, 0.362_683_783_378_362),
        (0.525_532_409_916_329, 0.313_706_645_877_887_3),
        (0.796_666_477_413_626_7, 0.222_381_034_453_374_5),
        (0.960_289_856_497_536_3, 0.101_228_536_290_376_3),
    ] {
        let left = evaluate(mid - half * node)?;
        let right = evaluate(mid + half * node)?;
        for i in 0..3 {
            sum[i] += weight * (left[i] + right[i]) * half;
        }
    }
    Ok(sum)
}

fn refine(
    evaluate: &mut impl FnMut(Value) -> Result<[Value; 3], SimulationError>,
    start: Value,
    stop: Value,
    whole: [Value; 3],
    [absolute, relative]: [Value; 2],
    remaining: usize,
) -> Result<[Value; 3], SimulationError> {
    let mid = (start + stop) * 0.5;
    let left = gauss8(evaluate, start, mid)?;
    let right = gauss8(evaluate, mid, stop)?;
    let fine = std::array::from_fn(|i| left[i] + right[i]);
    if (0..3).all(|i| {
        fine[i].is_finite()
            && (fine[i] - whole[i]).abs() <= absolute + relative * (left[i].abs() + right[i].abs())
    }) {
        return Ok(fine);
    }
    if remaining == 0 {
        return Err(SimulationError::Circuit(
            "periodic Meyer charge quadrature did not meet its error tolerance".into(),
        ));
    }
    let left = refine(
        evaluate,
        start,
        mid,
        left,
        [absolute * 0.5, relative],
        remaining - 1,
    )?;
    let right = refine(
        evaluate,
        mid,
        stop,
        right,
        [absolute * 0.5, relative],
        remaining - 1,
    )?;
    Ok(std::array::from_fn(|i| left[i] + right[i]))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn periodic_meyer_charge_keeps_closed_orbit_path_integral() {
        let params = [
            ("LEVEL", 1.0),
            ("TOX", 20e-9),
            ("VTO", 1.0),
            ("PHI", 1.0),
            ("GAMMA", 0.0),
        ]
        .into_iter()
        .map(|(name, value)| (name.to_string(), value))
        .collect();
        let device = crate::device::Mosfet::new_nmos("m".into(), 1, 2, 0, 3)
            .with_params(&params)
            .with_geometry(10e-6, 1e-6);
        // Gate/body trace an ellipse entirely in the linear Cgb depletion
        // wedge. Integral Cgb*dVgb = pi*Cox*A*B despite periodic voltages.
        let spectra = [
            vec![Complex64::new(0.25, 0.0), Complex64::new(0.1, 0.0)],
            vec![Complex64::new(0.2, 0.0)],
            vec![Complex64::new(-0.5, 0.0), Complex64::new(0.0, -0.1)],
        ];
        let mut evaluate = |phase| Ok(gate_phase_rates(&device, &spectra, phase));
        let whole = gauss8(&mut evaluate, -TAU, 0.0).unwrap();
        let charge = refine(&mut evaluate, -TAU, 0.0, whole, [1e-28, 1e-10], 14).unwrap();
        let expected = std::f64::consts::PI * device.oxide_capacitance_total() * 0.01;
        assert!((charge[2] - expected).abs() < 1e-10 * expected);
        assert_eq!(charge[0], 0.0);
        assert_eq!(charge[1], 0.0);
    }
}
