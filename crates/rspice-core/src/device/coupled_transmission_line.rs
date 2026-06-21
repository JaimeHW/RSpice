//! Modal multiconductor transmission line model.
//!
//! This realizes a coupled RLGC line in modal coordinates so transient
//! propagation uses exact per-mode delays rather than a ladder approximation.

use crate::{Value, circuit::NodeId};
use faer::{Mat, Side};

use super::TransmissionLine;
use super::cpl_native::{
    NativeCplRuntime, NativeCplStampPlan, NativeCplViHistory, NativeCplViSample,
};

const MODAL_RELATIVE_EIGEN_TOL: Value = 1e-12;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CplBranchCurrents {
    pub(crate) b1: Vec<NodeId>,
    pub(crate) b2: Vec<NodeId>,
}

impl CplBranchCurrents {
    pub(crate) fn new(b1: Vec<NodeId>, b2: Vec<NodeId>, conductors: usize) -> Result<Self, String> {
        if b1.len() != conductors || b2.len() != conductors {
            return Err(format!(
                "CPL branch-current topology requires {} b1 and b2 entries, found {} and {}",
                conductors,
                b1.len(),
                b2.len()
            ));
        }
        if b1.iter().chain(b2.iter()).any(|branch| *branch == 0) {
            return Err(
                "CPL branch-current topology requires non-ground branch ordinals".to_string(),
            );
        }
        Ok(Self { b1, b2 })
    }

    #[inline]
    pub(crate) fn conductor(&self, conductor: usize) -> Option<(NodeId, NodeId)> {
        Some((*self.b1.get(conductor)?, *self.b2.get(conductor)?))
    }

    pub(crate) fn matrix_indices_from_ordinals(&self, num_nodes: usize) -> Self {
        Self {
            b1: self.b1.iter().map(|branch| num_nodes + *branch).collect(),
            b2: self.b2.iter().map(|branch| num_nodes + *branch).collect(),
        }
    }
}

/// Native ngspice-faithful convolution runtime state for a coupled line.
///
/// `runtime` holds the *accepted* convolution state advanced up to
/// `last_committed_ps` (ngspice `cp`/cplines). Each step the stamp is computed
/// against a clone of this state so the accepted state is only mutated when a
/// step is committed.
#[derive(Debug, Clone)]
struct CplNativeState {
    runtime: NativeCplRuntime,
    history: NativeCplViHistory,
    /// Latest accepted near-end port voltages (ngspice `in_node->V`).
    near_v: Vec<Value>,
    /// Latest accepted far-end port voltages (ngspice `out_node->V`).
    far_v: Vec<Value>,
    /// Integer-picosecond time of the last committed history sample.
    last_committed_ps: i64,
    /// Last accepted solver time before picosecond truncation. ngspice's CPL
    /// keeps the same mixed clock as TXL (cplload.c): convolution
    /// exponentials advance by the fractional `CKTdelta` while the history
    /// grid and slopes run on truncated integer picoseconds.
    last_real_seconds: Value,
    /// Set once the DC operating point has seeded the convolution state.
    dc_seeded: bool,
}

#[derive(Debug, Clone)]
pub struct CoupledTransmissionLine {
    pub name: String,
    pub near_nodes: Vec<NodeId>,
    pub far_nodes: Vec<NodeId>,
    pub near_ref: NodeId,
    pub far_ref: NodeId,
    modal_from_physical_voltage: Vec<Vec<Value>>,
    modal_from_physical_current: Vec<Vec<Value>>,
    modal_to_physical_current: Vec<Vec<Value>>,
    port_admittance: Vec<Vec<Value>>,
    modal_conductances: Vec<Value>,
    dc_series_resistances: Vec<Value>,
    native_branch_ordinals: Option<CplBranchCurrents>,
    native_branch_matrix_indices: Option<CplBranchCurrents>,
    /// Pristine ngspice-faithful convolution runtime built from the RLGC
    /// matrices in [`Self::new`]. `Some` only when the native setup math
    /// succeeds and the line has grounded references (the ngspice CPL topology
    /// requires a shared ground reference at both ends).
    native_runtime_template: Option<NativeCplRuntime>,
    /// Active convolution state (seeded from the DC operating point). Drives the
    /// branch-current transient stamp when present.
    native: Option<CplNativeState>,
    modes: Vec<TransmissionLine>,
}

impl CoupledTransmissionLine {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        near_nodes: Vec<NodeId>,
        near_ref: NodeId,
        far_nodes: Vec<NodeId>,
        far_ref: NodeId,
        r: &[Vec<Value>],
        l: &[Vec<Value>],
        c: &[Vec<Value>],
        g: &[Vec<Value>],
        length: Value,
    ) -> Result<Self, String> {
        let conductors = near_nodes.len();
        if conductors < 2 || far_nodes.len() != conductors {
            return Err(format!(
                "Coupled transmission line '{}' requires matching near/far conductor counts >= 2",
                name
            ));
        }
        validate_square_matrix("R", r, conductors)?;
        validate_square_matrix("L", l, conductors)?;
        validate_square_matrix("C", c, conductors)?;
        validate_square_matrix("G", g, conductors)?;
        if !length.is_finite() || length <= 0.0 {
            return Err(format!(
                "Coupled transmission line '{}' has invalid length {}",
                name, length
            ));
        }

        let (c_eigs, c_vecs) = symmetric_eigendecompose(c)?;
        let c_tol = positive_eigen_tolerance(&c_eigs);
        if c_eigs.iter().any(|&x| !x.is_finite() || x <= c_tol) {
            return Err(format!(
                "Coupled transmission line '{}' requires a positive-definite capacitance matrix",
                name
            ));
        }
        let c_sqrt = reconstruct_from_eigensystem(&c_eigs, &c_vecs, Value::sqrt);
        let c_inv_sqrt = reconstruct_from_eigensystem(&c_eigs, &c_vecs, |x| 1.0 / x.sqrt());

        let modal_metric = mat_mul(&mat_mul(&c_sqrt, l), &c_sqrt);
        let (mode_metrics, mode_vecs) = symmetric_eigendecompose(&modal_metric)?;
        let mode_tol = positive_eigen_tolerance(&mode_metrics);
        if mode_metrics
            .iter()
            .any(|&x| !x.is_finite() || x <= mode_tol)
        {
            return Err(format!(
                "Coupled transmission line '{}' requires a positive-definite inductance matrix",
                name
            ));
        }

        let u_t = transpose(&mode_vecs);
        let modal_from_physical_voltage = mat_mul(&u_t, &c_sqrt);
        let modal_from_physical_current = mat_mul(&u_t, &c_inv_sqrt);
        let modal_to_physical_current = transpose(&modal_from_physical_voltage);

        let r_modal = mat_mul(
            &mat_mul(&modal_from_physical_voltage, r),
            &transpose(&modal_from_physical_voltage),
        );
        let g_modal = mat_mul(
            &mat_mul(&modal_from_physical_current, g),
            &transpose(&modal_from_physical_current),
        );

        let mut modal_conductances = Vec::with_capacity(conductors);
        let mut modes = Vec::with_capacity(conductors);
        for mode in 0..conductors {
            let z_mode = mode_metrics[mode].max(mode_tol).sqrt();
            let td_mode = length * z_mode;
            let g_mode = 1.0 / z_mode.max(mode_tol.sqrt());
            let r_mode = r_modal[mode][mode].max(0.0);
            let g_shunt_mode = g_modal[mode][mode].max(0.0);
            let alpha = 0.5 * (r_mode / z_mode.max(mode_tol.sqrt()) + g_shunt_mode * z_mode);
            let attenuation = (-alpha * length).exp().clamp(1e-6, 1.0);
            let loss_time_constant =
                0.5 * (r_mode + mode_metrics[mode].max(0.0) * g_shunt_mode) * length * length;

            let mut mode_line = TransmissionLine::new(
                format!("{}#mode{}", name, mode + 1),
                0,
                0,
                0,
                0,
                z_mode,
                td_mode,
            );
            mode_line.set_distributed_rlgc(
                r_mode,
                mode_metrics[mode].max(mode_tol),
                g_shunt_mode,
                1.0,
                length,
            );
            mode_line.set_attenuation(attenuation);
            if loss_time_constant.is_finite() && loss_time_constant > 0.0 {
                mode_line.set_loss_time_constant(loss_time_constant);
            }
            modal_conductances.push(g_mode);
            modes.push(mode_line);
        }

        let port_admittance = assemble_port_admittance(
            &modal_from_physical_voltage,
            &modal_to_physical_current,
            &modal_conductances,
        );
        let dc_series_resistances = (0..conductors)
            .map(|idx| {
                let resistance = r[idx][idx].max(0.0) * length;
                if resistance.is_finite() && resistance > 0.0 {
                    resistance
                } else {
                    1e-3
                }
            })
            .collect();

        // Build the ngspice-faithful convolution runtime up front. The native
        // path is only valid for grounded references (the ngspice CPL element
        // shares a single ground reference at each end); otherwise the line
        // falls back to the modal Norton transient model.
        let native_runtime_template = if near_ref == 0 && far_ref == 0 {
            NativeCplRuntime::setup(r, l, c, g, length).ok()
        } else {
            None
        };

        Ok(Self {
            name,
            near_nodes,
            far_nodes,
            near_ref,
            far_ref,
            modal_from_physical_voltage,
            modal_from_physical_current,
            modal_to_physical_current,
            port_admittance,
            modal_conductances,
            dc_series_resistances,
            native_branch_ordinals: None,
            native_branch_matrix_indices: None,
            native_runtime_template,
            native: None,
            modes,
        })
    }

    #[inline]
    pub fn conductors(&self) -> usize {
        self.near_nodes.len()
    }

    #[inline]
    pub fn min_mode_delay(&self) -> Value {
        self.modes
            .iter()
            .map(TransmissionLine::delay)
            .fold(Value::INFINITY, Value::min)
    }

    #[inline]
    pub fn propagation_delays(&self) -> impl Iterator<Item = Value> + '_ {
        self.modes.iter().map(TransmissionLine::delay)
    }

    #[inline]
    pub fn launched_modal_waves(&self) -> impl Iterator<Item = (Value, Value, Value)> + '_ {
        self.modes.iter().map(|mode| {
            (
                mode.delay(),
                mode.launched_forward_wave(),
                mode.launched_backward_wave(),
            )
        })
    }

    #[inline]
    pub fn port_admittance(&self) -> &[Vec<Value>] {
        &self.port_admittance
    }

    #[inline]
    pub fn dc_series_resistance(&self, conductor: usize) -> Value {
        self.dc_series_resistances[conductor]
    }

    #[inline]
    pub(crate) fn has_grounded_references(&self) -> bool {
        self.near_ref == 0 && self.far_ref == 0
    }

    pub(crate) fn set_native_branch_ordinals(
        &mut self,
        b1: Vec<NodeId>,
        b2: Vec<NodeId>,
    ) -> Result<(), String> {
        if !self.has_grounded_references() {
            return Err(format!(
                "Coupled transmission line '{}' native branch-current topology is only supported for grounded references",
                self.name
            ));
        }
        if self.native_runtime_template.is_none() {
            return Err(format!(
                "Coupled transmission line '{}' native convolution runtime is unavailable; refusing to allocate branch currents",
                self.name
            ));
        }
        self.native_branch_ordinals = Some(CplBranchCurrents::new(b1, b2, self.conductors())?);
        self.native_branch_matrix_indices = None;
        Ok(())
    }

    /// Whether the ngspice-faithful native convolution runtime is available for
    /// this line (grounded references and a successful setup).
    #[inline]
    pub(crate) fn native_runtime_available(&self) -> bool {
        self.native_runtime_template.is_some()
    }

    /// Smallest native mode delay (`taul`) in seconds, or `None` when the native
    /// runtime is unavailable. ngspice clamps the transient max step to
    /// `0.9 * min(taul)` so each mode's propagation is resolved by the
    /// convolution; mirroring that cap keeps the trapezoidal convolution faithful
    /// to the reference.
    #[inline]
    pub(crate) fn native_min_taul_seconds(&self) -> Option<Value> {
        let runtime = self.native_runtime_template.as_ref()?;
        runtime
            .taul_ps
            .iter()
            .copied()
            .filter(|t| t.is_finite() && *t > 0.0)
            .fold(None, |acc: Option<f64>, t| {
                Some(acc.map_or(t, |m| m.min(t)))
            })
            .map(|ps| ps * 1e-12)
    }

    /// Whether the active native convolution runtime drives this line's
    /// transient stamp (branch currents allocated and runtime present).
    #[inline]
    pub(crate) fn uses_native_runtime(&self) -> bool {
        self.native_branch_matrix_indices.is_some() && self.native_runtime_template.is_some()
    }

    #[inline]
    pub(crate) fn native_branch_ordinals(&self) -> Option<&CplBranchCurrents> {
        self.native_branch_ordinals.as_ref()
    }

    pub(crate) fn set_native_branch_matrix_indices(
        &mut self,
        b1: Vec<NodeId>,
        b2: Vec<NodeId>,
    ) -> Result<(), String> {
        self.native_branch_matrix_indices =
            Some(CplBranchCurrents::new(b1, b2, self.conductors())?);
        Ok(())
    }

    #[inline]
    pub(crate) fn native_branch_matrix_indices(&self) -> Option<&CplBranchCurrents> {
        self.native_branch_matrix_indices.as_ref()
    }

    /// Seed the native convolution state from the DC operating-point port
    /// voltages, mirroring ngspice's `CPLdcGiven` initialization in `CPLload`.
    ///
    /// Builds the VI history (with the DC sample at t=0), zeroes/initializes the
    /// per-pole convolution accumulators, and records the DC port voltages as
    /// the latest accepted state. No-op when the native runtime is inactive.
    pub(crate) fn native_seed_dc(
        &mut self,
        near_dc: &[Value],
        far_dc: &[Value],
    ) -> Result<(), String> {
        if !self.uses_native_runtime() {
            return Ok(());
        }
        let template = self
            .native_runtime_template
            .as_ref()
            .expect("native runtime template present when uses_native_runtime");
        let mut runtime = template.clone();
        runtime
            .initialize_dc_convolutions(near_dc, far_dc)
            .map_err(|err| format!("CPL '{}' DC convolution seed failed: {err}", self.name))?;

        let conductors = self.conductors();
        let mut history = NativeCplViHistory::new(conductors, near_dc.to_vec(), far_dc.to_vec())
            .map_err(|err| format!("CPL '{}' history init failed: {err}", self.name))?;
        // ngspice seeds the first VI sample at t=0 with the DC port voltages and
        // zero branch currents.
        history
            .push_sample(NativeCplViSample::new(
                0,
                near_dc.to_vec(),
                far_dc.to_vec(),
                vec![0.0; conductors],
                vec![0.0; conductors],
            ))
            .map_err(|err| format!("CPL '{}' history seed failed: {err}", self.name))?;

        self.native = Some(CplNativeState {
            runtime,
            history,
            near_v: near_dc.to_vec(),
            far_v: far_dc.to_vec(),
            last_committed_ps: 0,
            last_real_seconds: 0.0,
            dc_seeded: true,
        });
        Ok(())
    }

    /// Compute the per-step branch-current stamp for the requested step.
    ///
    /// `t2_seconds` is the step end-time (`t + dt`), `dt_seconds` the step size.
    /// Returns `None` when the native runtime is inactive or not yet seeded, or
    /// when the step is degenerate (non-positive dt). Errors from the underlying
    /// convolution math are surfaced so the caller can fall back gracefully.
    pub(crate) fn native_step_plan(
        &self,
        t2_seconds: Value,
        dt_seconds: Value,
    ) -> Option<NativeCplStampPlan> {
        let native = self.native.as_ref()?;
        if !native.dc_seeded || !dt_seconds.is_finite() || dt_seconds <= 0.0 {
            return None;
        }
        let t1_ps = native.last_committed_ps;
        // ngspice keeps time bookkeeping in truncated integer picoseconds. A
        // step shorter than 1 ps (e.g. a breakpoint nudge) would otherwise
        // collapse to a zero interval; clamp to at least 1 ps past the last
        // committed sample so the delayed-sample interpolation stays
        // well-defined (the convolution math still uses the true
        // `dt_seconds`).
        let mut t2_ps = (t2_seconds * 1e12).trunc() as i64;
        if t2_ps <= t1_ps {
            t2_ps = t1_ps + 1;
        }
        // The history pruning inside the plan needs a mutable view; clone the
        // history for this evaluation so the accepted history is untouched until
        // commit (ngspice prunes only via the accepted `cp->vi_head`).
        let mut history = native.history.clone();
        match native.runtime.step_stamp_plan(
            t1_ps,
            t2_ps,
            dt_seconds,
            &native.near_v,
            &native.far_v,
            &mut history,
        ) {
            Ok(plan) => Some(plan),
            Err(err) => {
                log::warn!("CPL '{}' native step stamp failed: {err}", self.name);
                None
            }
        }
    }

    /// Commit an accepted transient step into the native convolution state,
    /// mirroring the first loop of ngspice `CPLload` (add_new_vi + update_cnv +
    /// update_delayed_cnv) which advances `cp` by the accepted delta.
    ///
    /// `near_v`/`far_v`/`near_i`/`far_i` are the accepted near/far port voltages
    /// and branch currents at `accepted_time_seconds`.
    pub(crate) fn native_commit_accepted(
        &mut self,
        accepted_time_seconds: Value,
        near_v: &[Value],
        far_v: &[Value],
        near_i: &[Value],
        far_i: &[Value],
    ) {
        let name = self.name.clone();
        let Some(native) = self.native.as_mut() else {
            return;
        };
        if !native.dc_seeded {
            return;
        }
        let h_exp_seconds = accepted_time_seconds - native.last_real_seconds;
        if !(h_exp_seconds.is_finite() && h_exp_seconds > 0.0) {
            return;
        }
        let time_ps = (accepted_time_seconds * 1e12).trunc() as i64;
        if time_ps <= native.last_committed_ps {
            // ngspice merges accepted points whose truncated-picosecond label
            // does not advance: no history commit and no convolution update,
            // but the fractional clock still moves so the next step's
            // exponentials only span the remaining sub-interval.
            native.last_real_seconds = accepted_time_seconds;
            return;
        }
        let t1_ps = native.last_committed_ps;
        let h_grid_seconds = (time_ps - t1_ps) as f64 * 1e-12;

        let start_near = native.near_v.clone();
        let start_far = native.far_v.clone();

        // Advance the persistent convolution state for the just-completed step,
        // mirroring ngspice's per-load right_consts/update_cnv/update_delayed_cnv
        // ordering. The history must NOT yet contain the new (t2) sample when the
        // delayed samples for [t1, t2] are evaluated (ngspice adds the t2 sample
        // only at the start of the *next* load). The exponentials advance by the
        // solver's fractional step; the slope spans run on the integer grid
        // (ngspice's mixed clock).
        if let Err(err) = native.runtime.commit_step(
            t1_ps,
            time_ps,
            h_exp_seconds,
            h_grid_seconds,
            &start_near,
            &start_far,
            near_v,
            far_v,
            near_i,
            far_i,
            &mut native.history,
        ) {
            log::warn!("CPL '{name}' accepted convolution commit failed: {err}");
            return;
        }

        // Record the accepted sample and port state for the next step.
        if let Err(err) = native.history.push_sample(NativeCplViSample::new(
            time_ps,
            near_v.to_vec(),
            far_v.to_vec(),
            near_i.to_vec(),
            far_i.to_vec(),
        )) {
            log::warn!("CPL '{name}' history push failed: {err}");
            return;
        }
        native.near_v = near_v.to_vec();
        native.far_v = far_v.to_vec();
        native.last_committed_ps = time_ps;
        native.last_real_seconds = accepted_time_seconds;
    }

    pub fn reset(&mut self) {
        for mode in &mut self.modes {
            mode.reset();
        }
        self.native = None;
    }

    pub fn modalize_port_voltage(&self, physical: &[Value]) -> Vec<Value> {
        mat_vec_mul(&self.modal_from_physical_voltage, physical)
    }

    pub fn modalize_port_current(&self, physical: &[Value]) -> Vec<Value> {
        mat_vec_mul(&self.modal_from_physical_current, physical)
    }

    pub fn port_equivalent_current(&self, incoming_modal: &[Value]) -> Vec<Value> {
        let weighted: Vec<Value> = incoming_modal
            .iter()
            .zip(self.modal_conductances.iter())
            .map(|(incoming, g)| incoming * g)
            .collect();
        mat_vec_mul(&self.modal_to_physical_current, &weighted)
    }

    pub fn port_currents(
        &self,
        physical_voltage: &[Value],
        incoming_modal: &[Value],
    ) -> Vec<Value> {
        let mut currents = mat_vec_mul(&self.port_admittance, physical_voltage);
        let eq = self.port_equivalent_current(incoming_modal);
        for (current, eq_value) in currents.iter_mut().zip(eq.iter()) {
            *current -= eq_value;
        }
        currents
    }

    pub fn incoming_near_modal(&self, time: Value, far_refs_modal: &[Value]) -> Vec<Value> {
        self.modes
            .iter()
            .zip(far_refs_modal.iter())
            .map(|(mode, far_ref)| {
                let attenuation = mode.attenuation();
                mode.delayed_backward_at(time) + (1.0 - attenuation) * far_ref
            })
            .collect()
    }

    pub fn incoming_far_modal(&self, time: Value, near_refs_modal: &[Value]) -> Vec<Value> {
        self.modes
            .iter()
            .zip(near_refs_modal.iter())
            .map(|(mode, near_ref)| {
                let attenuation = mode.attenuation();
                mode.delayed_forward_at(time) + (1.0 - attenuation) * near_ref
            })
            .collect()
    }

    pub fn update_modal_history(
        &mut self,
        time: Value,
        near_modal_voltages: &[Value],
        near_modal_currents: &[Value],
        far_modal_voltages: &[Value],
        far_modal_currents: &[Value],
    ) {
        for mode in 0..self.modes.len() {
            self.modes[mode].update_history(
                time,
                near_modal_voltages[mode],
                near_modal_currents[mode],
                far_modal_voltages[mode],
                far_modal_currents[mode],
            );
        }
    }
}

fn positive_eigen_tolerance(eigenvalues: &[Value]) -> Value {
    eigenvalues
        .iter()
        .copied()
        .fold(0.0_f64, |scale, value| scale.max(value.abs()))
        .max(Value::EPSILON)
        * MODAL_RELATIVE_EIGEN_TOL
}

fn validate_square_matrix(
    name: &str,
    matrix: &[Vec<Value>],
    expected: usize,
) -> Result<(), String> {
    if matrix.len() != expected || matrix.iter().any(|row| row.len() != expected) {
        return Err(format!(
            "Coupled transmission line matrix '{}' must be {}x{}",
            name, expected, expected
        ));
    }
    Ok(())
}

fn symmetric_eigendecompose(
    matrix: &[Vec<Value>],
) -> Result<(Vec<Value>, Vec<Vec<Value>>), String> {
    let n = matrix.len();
    let mut faer_mat = Mat::<Value>::zeros(n, n);
    for row in 0..n {
        for col in 0..n {
            faer_mat[(row, col)] = matrix[row][col];
        }
    }

    let evd = faer_mat
        .self_adjoint_eigen(Side::Lower)
        .map_err(|err| format!("self-adjoint eigendecomposition failed: {err:?}"))?;
    let eigenvalues = (0..n)
        .map(|idx| evd.S().column_vector()[idx])
        .collect::<Vec<_>>();
    let eigenvectors = (0..n)
        .map(|row| (0..n).map(|col| evd.U()[(row, col)]).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    Ok((eigenvalues, eigenvectors))
}

fn reconstruct_from_eigensystem(
    eigenvalues: &[Value],
    eigenvectors: &[Vec<Value>],
    map: impl Fn(Value) -> Value,
) -> Vec<Vec<Value>> {
    let n = eigenvalues.len();
    let mut result = vec![vec![0.0; n]; n];
    for mode in 0..n {
        let weight = map(eigenvalues[mode]);
        for row in 0..n {
            for col in 0..n {
                result[row][col] += eigenvectors[row][mode] * weight * eigenvectors[col][mode];
            }
        }
    }
    result
}

fn assemble_port_admittance(
    modal_from_physical_voltage: &[Vec<Value>],
    modal_to_physical_current: &[Vec<Value>],
    modal_conductances: &[Value],
) -> Vec<Vec<Value>> {
    let n = modal_conductances.len();
    let mut result = vec![vec![0.0; n]; n];
    for row in 0..n {
        for col in 0..n {
            let mut value = 0.0;
            for mode in 0..n {
                value += modal_to_physical_current[row][mode]
                    * modal_conductances[mode]
                    * modal_from_physical_voltage[mode][col];
            }
            result[row][col] = value;
        }
    }
    result
}

fn mat_mul(a: &[Vec<Value>], b: &[Vec<Value>]) -> Vec<Vec<Value>> {
    let rows = a.len();
    let cols = b[0].len();
    let inner = b.len();
    let mut result = vec![vec![0.0; cols]; rows];
    for row in 0..rows {
        for col in 0..cols {
            let mut sum = 0.0;
            for idx in 0..inner {
                sum += a[row][idx] * b[idx][col];
            }
            result[row][col] = sum;
        }
    }
    result
}

fn mat_vec_mul(a: &[Vec<Value>], x: &[Value]) -> Vec<Value> {
    a.iter()
        .map(|row| row.iter().zip(x.iter()).map(|(aij, xj)| aij * xj).sum())
        .collect()
}

fn transpose(matrix: &[Vec<Value>]) -> Vec<Vec<Value>> {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut result = vec![vec![0.0; rows]; cols];
    for row in 0..rows {
        for col in 0..cols {
            result[col][row] = matrix[row][col];
        }
    }
    result
}
