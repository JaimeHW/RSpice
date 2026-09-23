//! Lead currents evaluated with the same F/Q laws and grid as the HB residual.

use super::*;

pub(crate) struct HbDeviceLeadSpectra {
    pub(crate) name: String,
    pub(crate) terminals: &'static [&'static str],
    pub(crate) currents: Vec<Vec<Complex64>>,
}

impl HbSolver {
    fn lead_phasors(
        &mut self,
        current: &[Value],
        charge: &[Value],
    ) -> Result<Vec<Complex64>, HbError> {
        let f = self.checked_periodic_spectrum(current, self.num_harmonics, "HB lead current")?;
        let q = self.checked_periodic_spectrum(charge, self.num_harmonics, "HB lead charge")?;
        (0..=self.num_harmonics)
            .map(|harmonic| {
                let omega =
                    std::f64::consts::TAU * harmonic as Value * self.config.fundamental_freq;
                let value = f.get(harmonic).copied().unwrap_or_default()
                    + Complex64::new(0.0, omega) * q.get(harmonic).copied().unwrap_or_default();
                let value = if harmonic == 0 {
                    Complex64::new(value.re, 0.0)
                } else {
                    value * 2.0
                };
                if !value.re.is_finite() || !value.im.is_finite() {
                    Err(HbError::InvalidCircuit(
                        "HB lead-current phasor is non-finite".into(),
                    ))
                } else {
                    Ok(value)
                }
            })
            .collect()
    }

    pub(crate) fn nonlinear_lead_spectra(
        &mut self,
        state: &HbSolverState,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<HbDeviceLeadSpectra>, HbError> {
        let mut result = Vec::new();
        if !self.nonlinear_devices.is_empty() {
            let waves = self.periodic_state_waveforms(state, "HB lead-current observation")?;
            let count = self.fft.size();
            for index in 0..self.nonlinear_devices.len() {
                if abort.is_aborted() {
                    return Err(HbError::Aborted);
                }
                let mut device = self.nonlinear_devices[index].clone();
                let terminals: &'static [&'static str] = match device.device_type {
                    NonlinearDeviceType::Diode => &["d", "k"],
                    NonlinearDeviceType::Nmos | NonlinearDeviceType::Pmos => &["d", "g", "s", "b"],
                    NonlinearDeviceType::Njfet | NonlinearDeviceType::Pjfet => &["d", "g", "s"],
                    NonlinearDeviceType::VoltageSwitch => &["p", "n", "cp", "cn"],
                };
                let nodes = device.terminals.clone();
                // Preserve separate leads when terminals share a circuit node.
                // These are observation coordinates only; the device's resolved
                // physical parameters and constitutive equations stay identical.
                device.device.terminals = (0..nodes.len()).collect();
                let mut f = vec![vec![0.0; count]; nodes.len()];
                let mut q = f.clone();
                let mut voltages = vec![0.0; nodes.len()];
                for time in 0..count {
                    if abort.is_aborted() {
                        return Err(HbError::Aborted);
                    }
                    for (value, &node) in voltages.iter_mut().zip(&nodes) {
                        *value = waves.get(node).map_or(0.0, |wave| wave[time]);
                    }
                    for (terminal, injection) in device.evaluate(&voltages) {
                        f[terminal][time] -= injection;
                    }
                    for (terminal, injection) in device.charge(&voltages) {
                        q[terminal][time] -= injection;
                    }
                }
                let currents = f
                    .iter()
                    .zip(&q)
                    .map(|(f, q)| self.lead_phasors(f, q))
                    .collect::<Result<_, _>>()?;
                result.push(HbDeviceLeadSpectra {
                    name: self.nonlinear_device_names[index].clone(),
                    terminals,
                    currents,
                });
            }
        }
        if !self.native_bjts.is_empty() {
            let count = self.fft.size();
            let waves = self.native_state_waveforms(state)?;
            let mut solution = vec![0.0; waves.len()];
            for index in 0..self.native_bjts.len() {
                let mut device = self.native_bjts[index].clone();
                let name = device.name.to_string();
                let mut f = vec![vec![0.0; count]; 4];
                let mut q = f.clone();
                for time in 0..count {
                    if abort.is_aborted() {
                        return Err(HbError::Aborted);
                    }
                    for (value, wave) in solution.iter_mut().zip(&waves) {
                        *value = wave[time];
                    }
                    device.update_mna_static_probe(&solution);
                    let (currents, charges) = device
                        .periodic_lead_fq(&solution)
                        .map_err(HbError::InvalidCircuit)?;
                    for terminal in 0..4 {
                        f[terminal][time] = currents[terminal];
                        q[terminal][time] = charges[terminal];
                    }
                }
                let currents = f
                    .iter()
                    .zip(&q)
                    .map(|(f, q)| self.lead_phasors(f, q))
                    .collect::<Result<_, _>>()?;
                result.push(HbDeviceLeadSpectra {
                    name,
                    terminals: &["c", "b", "e", "s"],
                    currents,
                });
            }
        }
        if !self.native_bsim3.is_empty() {
            let count = self.fft.size();
            let waves = self.native_state_waveforms(state)?;
            let mut solution = vec![0.0; waves.len()];
            for index in 0..self.native_bsim3.len() {
                let device = self.native_bsim3[index].clone();
                let mut f = vec![vec![0.0; count]; 4];
                let mut q = f.clone();
                for time in 0..count {
                    if abort.is_aborted() {
                        return Err(HbError::Aborted);
                    }
                    for (value, wave) in solution.iter_mut().zip(&waves) {
                        *value = wave[time];
                    }
                    let (currents, charges) = device
                        .periodic_lead_fq(&solution)
                        .map_err(HbError::InvalidCircuit)?;
                    for terminal in 0..4 {
                        f[terminal][time] = currents[terminal];
                        q[terminal][time] = charges[terminal];
                    }
                }
                let currents = f
                    .iter()
                    .zip(&q)
                    .map(|(f, q)| self.lead_phasors(f, q))
                    .collect::<Result<_, _>>()?;
                result.push(HbDeviceLeadSpectra {
                    name: device.name.clone(),
                    terminals: &["d", "g", "s", "b"],
                    currents,
                });
            }
        }
        if !self.native_bsim4.is_empty() {
            let count = self.fft.size();
            let waves = self.native_state_waveforms(state)?;
            let mut solution = vec![0.0; waves.len()];
            for index in 0..self.native_bsim4.len() {
                let device = self.native_bsim4[index].clone();
                let mut f = vec![vec![0.0; count]; 4];
                let mut q = f.clone();
                for time in 0..count {
                    if abort.is_aborted() {
                        return Err(HbError::Aborted);
                    }
                    for (value, wave) in solution.iter_mut().zip(&waves) {
                        *value = wave[time];
                    }
                    let (currents, charges) = device
                        .periodic_lead_fq(&solution)
                        .map_err(HbError::InvalidCircuit)?;
                    for terminal in 0..4 {
                        f[terminal][time] = currents[terminal];
                        q[terminal][time] = charges[terminal];
                    }
                }
                let currents = f
                    .iter()
                    .zip(&q)
                    .map(|(f, q)| self.lead_phasors(f, q))
                    .collect::<Result<_, _>>()?;
                result.push(HbDeviceLeadSpectra {
                    name: device.name.clone(),
                    terminals: &["d", "g", "s", "b"],
                    currents,
                });
            }
        }
        if !self.native_mos.is_empty() {
            let count = self.fft.size();
            let waves = self.native_state_waveforms(state)?;
            let mut solution = vec![0.0; waves.len()];
            let mut rate_start = self.num_nodes + self.mos_rate_branch_start();
            for index in 0..self.native_mos.len() {
                let device = self.native_mos[index].clone();
                let rates = (!device.uses_legacy_bsim()).then(|| {
                    let nodes = [rate_start + 1, rate_start + 2, rate_start + 3];
                    rate_start += 3;
                    nodes
                });
                let mut f = vec![vec![0.0; count]; 4];
                let mut q = f.clone();
                for time in 0..count {
                    if abort.is_aborted() {
                        return Err(HbError::Aborted);
                    }
                    for (value, wave) in solution.iter_mut().zip(&waves) {
                        *value = wave[time];
                    }
                    let (currents, charges) = device
                        .periodic_terminal_fq(&solution, rates)
                        .map_err(HbError::InvalidCircuit)?;
                    for terminal in 0..4 {
                        f[terminal][time] = currents[terminal];
                        q[terminal][time] = charges[terminal];
                    }
                }
                let currents = f
                    .iter()
                    .zip(&q)
                    .map(|(f, q)| self.lead_phasors(f, q))
                    .collect::<Result<_, _>>()?;
                result.push(HbDeviceLeadSpectra {
                    name: device.name.clone(),
                    terminals: &["d", "g", "s", "b"],
                    currents,
                });
            }
        }
        if !self.periodic_capacitors.is_empty() {
            let waves = self.native_state_waveforms(state)?;
            let count = self.fft.size();
            let mut solution = vec![0.0; waves.len()];
            let zero_charge = vec![0.0; count];
            let rate_start = self.num_nodes + self.capacitor_rate_start();
            let integral_start = self.num_nodes + self.physical_branch_count();
            let mut state_start = integral_start + self.behavioral_sources.integral_count();
            for index in 0..self.periodic_capacitors.len() {
                let mut capacitor = self.periodic_capacitors[index].clone();
                let mut current = vec![0.0; count];
                for (sample, current) in current.iter_mut().enumerate() {
                    if abort.is_aborted() {
                        return Err(HbError::Aborted);
                    }
                    for (value, wave) in solution.iter_mut().zip(&waves) {
                        *value = wave[sample];
                    }
                    let time = sample as Value / count as Value / self.config.fundamental_freq;
                    *current = capacitor.multiplier
                        * capacitor
                            .expression
                            .sample_periodic_capacitance(
                                crate::device::behavioral::BehavioralFqPoint {
                                    inputs: &solution,
                                    time,
                                    num_nodes: self.num_nodes,
                                    unknowns: solution.len(),
                                    integral_start,
                                    prescribed_integrals: &[],
                                },
                                state_start,
                            )
                            .map_err(HbError::InvalidCircuit)?
                            .value
                        * self.config.fundamental_freq
                        * solution[rate_start + index];
                }
                let positive = self.lead_phasors(&current, &zero_charge)?;
                let negative = positive.iter().map(|value| -*value).collect();
                state_start += capacitor.expression.program.sdt_count;
                result.push(HbDeviceLeadSpectra {
                    name: capacitor.expression.name,
                    terminals: &["p", "n"],
                    currents: vec![positive, negative],
                });
            }
        }
        if !self.behavioral_sources.current_sources.is_empty() {
            let waves = self.native_state_waveforms(state)?;
            let count = self.fft.size();
            let mut solution = vec![0.0; waves.len()];
            let zero_charge = vec![0.0; count];
            let integral_start = self.num_nodes + self.physical_branch_count();
            let mut state_start = integral_start
                + self
                    .behavioral_sources
                    .voltage_sources
                    .iter()
                    .map(|source| source.program.sdt_count)
                    .sum::<usize>();
            for index in 0..self.behavioral_sources.current_sources.len() {
                let mut source = self.behavioral_sources.current_sources[index].clone();
                let mut current = vec![0.0; count];
                for (time, sample) in current.iter_mut().enumerate() {
                    if abort.is_aborted() {
                        return Err(HbError::Aborted);
                    }
                    for (value, wave) in solution.iter_mut().zip(&waves) {
                        *value = wave[time];
                    }
                    *sample = source
                        .evaluate_periodic_output(
                            crate::device::behavioral::BehavioralFqPoint {
                                inputs: &solution,
                                time: time as Value / count as Value / self.config.fundamental_freq,
                                num_nodes: self.num_nodes,
                                unknowns: solution.len(),
                                integral_start,
                                prescribed_integrals: &[],
                            },
                            state_start,
                        )
                        .map_err(|error| HbError::InvalidCircuit(error.to_string()))?;
                }
                state_start += source.program.sdt_count;
                let positive = self.lead_phasors(&current, &zero_charge)?;
                let negative = positive.iter().map(|value| -*value).collect();
                result.push(HbDeviceLeadSpectra {
                    name: source.name,
                    terminals: &["p", "n"],
                    currents: vec![positive, negative],
                });
            }
        }
        Ok(result)
    }
}
