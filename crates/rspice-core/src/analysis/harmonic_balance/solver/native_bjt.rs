//! Shared native BJT F/Q sampling over the complete periodic MNA state.

use super::*;
use crate::device::MatrixStamper;
use std::collections::BTreeMap;

struct NativeStamp {
    residual: Vec<Value>,
    jacobian: Vec<(usize, usize, Value)>,
    invalid: bool,
}

impl NativeStamp {
    fn new(unknowns: usize) -> Self {
        Self {
            residual: vec![0.0; unknowns],
            jacobian: Vec::new(),
            invalid: false,
        }
    }
    fn clear(&mut self) {
        self.residual.fill(0.0);
        self.jacobian.clear();
        self.invalid = false;
    }
}

impl MatrixStamper for NativeStamp {
    fn stamp(&mut self, row: usize, col: usize, value: Value) {
        if row == 0 || col == 0 || value == 0.0 {
            return;
        }
        if row > self.residual.len() || col > self.residual.len() || !value.is_finite() {
            self.invalid = true;
        } else {
            self.jacobian.push((row - 1, col - 1, value));
        }
    }
    fn stamp_rhs(&mut self, row: usize, value: Value) {
        if row == 0 {
            return;
        }
        if let Some(entry) = self.residual.get_mut(row - 1) {
            *entry += value;
            self.invalid |= !entry.is_finite();
        } else {
            self.invalid = true;
        }
    }
}

impl HbSolver {
    pub(super) fn electrical_node(&self, node: usize) -> bool {
        self.non_electrical_nodes.binary_search(&node).is_err()
    }

    pub(crate) fn add_native_bjt(&mut self, mut bjt: crate::device::Bjt) -> Result<(), HbError> {
        bjt.prepare_periodic_mna(self.num_nodes)
            .map_err(HbError::InvalidCircuit)?;
        if bjt
            .mna_coupling_nodes()
            .iter()
            .any(|&node| node > self.num_nodes)
            || bjt
                .mna_rbi_branch_matrix_node(self.num_nodes)
                .is_some_and(|node| {
                    !self.exact_mna_branches().iter().any(|branch| {
                        matches!(branch, ExactMnaBranch::ConstitutivePort { branch_ordinal, .. }
                        if *branch_ordinal == node - self.num_nodes)
                    })
                })
        {
            return Err(HbError::InvalidCircuit(format!(
                "BJT '{}' has unregistered periodic MNA coordinates",
                bjt.name
            )));
        }
        for node in [bjt.node_rth, bjt.node_xf1, bjt.node_xf2] {
            if node != 0
                && let Err(index) = self.non_electrical_nodes.binary_search(&(node - 1))
            {
                self.non_electrical_nodes.insert(index, node - 1);
            }
        }
        self.native_bjts.push(bjt);
        Ok(())
    }

    fn sample_native_bjts(
        &mut self,
        solution: &[Value],
        f: &mut NativeStamp,
        q: &mut NativeStamp,
    ) -> Result<(), HbError> {
        if solution.len() != self.num_nodes + self.exact_mna_branches().len() {
            return Err(HbError::InvalidCircuit(
                "native BJT sample does not match the complete MNA state".into(),
            ));
        }
        f.clear();
        q.clear();
        for bjt in &mut self.native_bjts {
            bjt.stamp_periodic_fq(solution, f, q);
            // The canonical port owns these two linear KCL incidence entries.
            // Retain the native RBI constitutive row and thermal power coupling.
            if let Some(branch) = bjt.mna_rbi_branch_matrix_node(self.num_nodes) {
                let current = solution[branch - 1];
                for (node, sign) in [(bjt.node_bx, 1.0), (bjt.node_bi, -1.0)] {
                    f.stamp(node, branch, -sign);
                    f.stamp_rhs(node, sign * current);
                }
            }
            if f.invalid || q.invalid {
                return Err(HbError::InvalidCircuit(format!(
                    "BJT '{}' produced invalid physical F/Q entries",
                    bjt.name
                )));
            }
        }
        Ok(())
    }

    fn native_dc_sample(&mut self, state: &HbSolverState) -> Result<NativeStamp, HbError> {
        let solution: Vec<_> = state
            .x
            .iter()
            .chain(&state.mna_branch_currents)
            .map(|s| s[0].re)
            .collect();
        let mut f = NativeStamp::new(solution.len());
        let mut q = NativeStamp::new(solution.len());
        self.sample_native_bjts(&solution, &mut f, &mut q)?;
        Ok(f)
    }

    pub(super) fn add_native_dc_residual(
        &mut self,
        state: &mut HbSolverState,
    ) -> Result<(), HbError> {
        if self.native_bjts.is_empty() {
            return Ok(());
        }
        let f = self.native_dc_sample(state)?;
        for ((residual, scale), value) in state
            .residual
            .iter_mut()
            .chain(&mut state.mna_branch_residual)
            .zip(
                state
                    .residual_scale
                    .iter_mut()
                    .chain(&mut state.mna_branch_residual_scale),
            )
            .zip(f.residual)
        {
            residual[0].re += value;
            scale[0] += value.abs();
        }
        Ok(())
    }

    pub(super) fn add_native_dc_jacobian(
        &mut self,
        state: &HbSolverState,
        jacobian: &mut [Vec<Value>],
    ) -> Result<(), HbError> {
        if !self.native_bjts.is_empty() {
            for (row, col, value) in self.native_dc_sample(state)?.jacobian {
                jacobian[row][col] -= value;
            }
        }
        Ok(())
    }

    fn native_state_waveforms(
        &mut self,
        state: &HbSolverState,
    ) -> Result<Vec<Vec<Value>>, HbError> {
        let mut waves = self.periodic_state_waveforms(state, "native BJT F/Q evaluation")?;
        for spectrum in &state.mna_branch_currents {
            let waveform = self.fft.to_time_domain(spectrum);
            if waveform.len() != self.fft.size() || waveform.iter().any(|v| !v.is_finite()) {
                return Err(HbError::InvalidCircuit(
                    "native BJT branch-current waveform is invalid".into(),
                ));
            }
            waves.push(waveform);
        }
        Ok(waves)
    }

    /// Visit native models at each unlimited physical bias of the retained
    /// periodic state, including branch-current and non-electrical coordinates.
    pub(crate) fn visit_native_bjt_samples(
        &mut self,
        state: &HbSolverState,
        abort: &dyn AbortSignal,
        mut visit: impl FnMut(usize, usize, &[crate::device::Bjt], &[Value]) -> Result<(), HbError>,
    ) -> Result<(), HbError> {
        if self.native_bjts.is_empty() {
            return Ok(());
        }
        let waves = self.native_state_waveforms(state)?;
        let count = self.fft.size();
        let mut solution = vec![0.0; waves.len()];
        for time in 0..count {
            if abort.is_aborted() {
                return Err(HbError::Aborted);
            }
            for (value, wave) in solution.iter_mut().zip(&waves) {
                *value = wave[time];
            }
            for bjt in &mut self.native_bjts {
                if abort.is_aborted() {
                    return Err(HbError::Aborted);
                }
                bjt.update_mna_static_probe(&solution);
            }
            visit(time, count, &self.native_bjts, &solution)?;
        }
        Ok(())
    }

    pub(super) fn add_native_periodic_residual(
        &mut self,
        state: &mut HbSolverState,
    ) -> Result<(), HbError> {
        if self.native_bjts.is_empty() {
            return Ok(());
        }
        let waves = self.native_state_waveforms(state)?;
        let size = waves.len();
        let times = self.fft.size();
        let mut solution = vec![0.0; size];
        let mut f = NativeStamp::new(size);
        let mut q = NativeStamp::new(size);
        let mut f_time = vec![vec![0.0; times]; size];
        let mut q_time = vec![vec![0.0; times]; size];
        for time in 0..times {
            for (value, wave) in solution.iter_mut().zip(&waves) {
                *value = wave[time];
            }
            self.sample_native_bjts(&solution, &mut f, &mut q)?;
            for row in 0..size {
                f_time[row][time] = f.residual[row];
                q_time[row][time] = q.residual[row];
            }
        }
        let omega = std::f64::consts::TAU * self.config.fundamental_freq;
        for (row, (f, q)) in f_time.iter().zip(&q_time).enumerate() {
            let f =
                self.checked_periodic_spectrum(f, (self.fft.size() - 1) / 2, "native BJT current")?;
            let q =
                self.checked_periodic_spectrum(q, (self.fft.size() - 1) / 2, "native BJT charge")?;
            let (residual, scale) = if row < self.num_nodes {
                (&mut state.residual[row], &mut state.residual_scale[row])
            } else {
                (
                    &mut state.mna_branch_residual[row - self.num_nodes],
                    &mut state.mna_branch_residual_scale[row - self.num_nodes],
                )
            };
            for k in 0..=self.num_harmonics {
                let current = f.get(k).copied().unwrap_or_default();
                let displacement =
                    Complex64::new(0.0, omega * k as Value) * q.get(k).copied().unwrap_or_default();
                residual[k] += current + displacement;
                scale[k] += current.norm() + displacement.norm();
            }
        }
        Ok(())
    }

    pub(super) fn native_bjt_spectra(
        &mut self,
        state: &HbSolverState,
        harmonics: usize,
        charge: bool,
    ) -> Result<Vec<(usize, usize, Vec<Complex64>)>, HbError> {
        if self.native_bjts.is_empty() {
            return Ok(Vec::new());
        }
        let waves = self.native_state_waveforms(state)?;
        let mut solution = vec![0.0; waves.len()];
        let mut f = NativeStamp::new(waves.len());
        let mut q = NativeStamp::new(waves.len());
        let times = self.fft.size();
        let mut entries: BTreeMap<(usize, usize), Vec<Value>> = BTreeMap::new();
        for time in 0..times {
            for (value, wave) in solution.iter_mut().zip(&waves) {
                *value = wave[time];
            }
            self.sample_native_bjts(&solution, &mut f, &mut q)?;
            for &(row, col, value) in if charge { &q.jacobian } else { &f.jacobian } {
                let sum = &mut entries
                    .entry((row, col))
                    .or_insert_with(|| vec![0.0; times])[time];
                *sum += value;
                if !sum.is_finite() {
                    return Err(HbError::InvalidCircuit(
                        "native BJT Jacobian accumulation is non-finite".into(),
                    ));
                }
            }
        }
        let mut spectra = Vec::with_capacity(entries.len());
        for ((row, col), wave) in entries {
            let spectrum =
                self.checked_periodic_spectrum(&wave, harmonics, "native BJT Jacobian")?;
            if !spectrum.is_empty() {
                spectra.push((row, col, spectrum));
            }
        }
        Ok(spectra)
    }
}
