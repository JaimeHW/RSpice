//! One shared native noise input across physical ports and independent phases.
use super::*;
use crate::analysis::noise::{
    Bsim4CorrelatedNoiseWaveform,
    correlated::{coherent_sum, induced_gate_factor},
};
use std::collections::{BTreeMap, BTreeSet};

impl QuasiPeriodicNoiseProjector {
    pub(super) fn correlated_workspace(
        &self,
        outputs: usize,
        waveform: &Bsim4CorrelatedNoiseWaveform,
        abort: &dyn AbortSignal,
    ) -> Result<usize, Error> {
        let mut ports = BTreeSet::new();
        for sample in &waveform.samples {
            check_abort(abort)?;
            ports.extend(
                [sample.drain_port, sample.gate_port]
                    .into_iter()
                    .filter(|p| p[0] != p[1]),
            );
        }
        let modes = self
            .grid
            .dimensions()
            .iter()
            .fold(1usize, |n, d| n.saturating_mul(2 * (d / 2) + 1));
        let pairs = modes.saturating_mul(self.selected.len());
        ResourceLimitError::ensure(
            ResourceKind::AnalysisPoints,
            pairs,
            self.limits.max_analysis_points,
        )?;
        Ok(pairs
            .saturating_mul(self.grid.dimensions().len() + 16 + 16 * outputs)
            .saturating_add(
                modes.saturating_mul(4 * ports.len() + 2 * self.grid.dimensions().len() + 2),
            )
            .saturating_add(waveform.samples.len().saturating_mul(8)))
    }

    pub(super) fn correlated(
        &mut self,
        adjoints: &[QuasiPeriodicAdjointSolution],
        waveform: &Bsim4CorrelatedNoiseWaveform,
        abort: &dyn AbortSignal,
    ) -> Result<QuasiPeriodicNoiseCovariance, Error> {
        let mut ports = BTreeSet::new();
        for sample in &waveform.samples {
            check_abort(abort)?;
            for port in [sample.drain_port, sample.gate_port] {
                if port[0] != port[1] {
                    ports.insert(port);
                }
            }
        }
        if ports.is_empty() {
            return covariance(adjoints.len(), |_, _| Ok((Complex64::ZERO, 0.0)), abort);
        }
        self.check_values(
            adjoints.len(),
            self.correlated_workspace(adjoints.len(), waveform, abort)?,
        )?;
        let mut drain_spectra = Vec::new();
        let mut tuples = Vec::new();
        for &port in &ports {
            check_abort(abort)?;
            let samples = waveform
                .samples
                .iter()
                .map(|s| {
                    if s.drain_port == port {
                        s.drain_amplitude
                    } else {
                        0.0
                    }
                })
                .collect::<Vec<_>>();
            let spectrum = self.transform.to_complete_real_spectrum_with_abort(
                &samples,
                &self.limits,
                abort,
            )?;
            tuples = spectrum.tuples;
            drain_spectra.push(spectrum.coefficients);
        }
        // Differences of signed tuples, never modulo a collocation dimension.
        // Include the full source interpolant, even above the circuit basis.
        let mut support: BTreeMap<Vec<i32>, Vec<(usize, usize)>> = BTreeMap::new();
        for &k in &self.selected {
            for (index, modulation) in tuples.iter().enumerate() {
                if index.is_multiple_of(256) {
                    check_abort(abort)?;
                }
                let input = self.grid.indices()[k]
                    .iter()
                    .zip(modulation)
                    .map(|(k, m)| {
                        k.checked_sub(*m)
                            .ok_or_else(|| invalid("correlated stationary tuple overflows"))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                support.entry(input).or_default().push((k, index));
            }
        }
        let mut folded = Vec::with_capacity(support.len());
        for (input, paths) in support {
            check_abort(abort)?;
            let frequency = self.grid.frequency_relative_to(
                adjoints[0].frequency_hz,
                &adjoints[0].frequency_lattice,
                &input,
            )?;
            let mut gate_spectra = Vec::new();
            for &port in &ports {
                let samples = waveform
                    .samples
                    .iter()
                    .map(|s| {
                        if s.gate_port == port {
                            s.gate_amplitude * induced_gate_factor(frequency, s.gate_time_constant)
                        } else {
                            0.0
                        }
                    })
                    .collect::<Vec<_>>();
                gate_spectra.push(
                    self.transform
                        .to_complete_real_spectrum_with_abort(&samples, &self.limits, abort)?
                        .coefficients,
                );
            }
            let mut channels = Vec::with_capacity(adjoints.len());
            for adjoint in adjoints {
                check_abort(abort)?;
                let transfer = coherent_sum(|visit| {
                    for (p, port) in ports.iter().enumerate() {
                        for (i, &(k, m)) in paths.iter().enumerate() {
                            if i.is_multiple_of(256) && abort.is_aborted() {
                                return Err("aborted");
                            }
                            for (terminal, sign) in [(port[0], 1.0), (port[1], -1.0)] {
                                if terminal == usize::MAX {
                                    continue;
                                }
                                // QP solves A^H lambda=c, unlike HB's plain
                                // transpose. The direct transfer uses conj(lambda).
                                let gain = adjoint.sensitivities[terminal][k].conj();
                                visit(scaled_complex_product3(
                                    gain,
                                    drain_spectra[p][m],
                                    Complex64::new(sign, 0.0),
                                    0,
                                )?)?;
                                visit(scaled_complex_product3(
                                    gain,
                                    gate_spectra[p][m],
                                    Complex64::new(0.0, sign),
                                    0,
                                )?)?;
                            }
                        }
                    }
                    Ok(())
                })
                .map_err(|reason| {
                    if abort.is_aborted() {
                        Error::Aborted
                    } else {
                        numerical(reason)
                    }
                })?;
                channels.push(transfer);
            }
            folded.push(channels);
        }
        covariance(
            adjoints.len(),
            |r, c| {
                sum_terms(
                    |visit| {
                        for values in &folded {
                            check_abort(abort)?;
                            visit(
                                scaled_flicker_cross_term(
                                    values[r],
                                    values[c],
                                    1.0,
                                    waveform.binary_scale_exponent,
                                    1.0,
                                    0.0,
                                )
                                .map_err(numerical)?,
                            )?;
                        }
                        Ok(())
                    },
                    r == c,
                )
            },
            abort,
        )
    }
}
