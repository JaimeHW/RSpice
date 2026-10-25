//! Struct-of-Arrays storage for bias-dependent semiconductor devices.
//!
//! One container per device family — diodes, BJTs, the B3SOI DD/FD/PD
//! variants, BSIM3v3, BSIM4 v4.8, EKV and EKV3, and VDMOS — each holding its
//! family's per-instance parameters, node bindings, and Newton state in
//! parallel arrays.
//!
//! Splitting by family rather than storing one polymorphic device list keeps
//! evaluation monomorphic and lets the engine skip a family entirely when a
//! circuit contains none of it.

use super::*;

#[derive(Debug, Clone, Default)]
pub struct Diodes {
    pub devices: Vec<Diode>,
}

impl Diodes {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn stamp_charge_companions(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
        coeff: &CompanionCoefficients,
        dt: Value,
        history: &crate::numerics::integration::TwoTerminalChargeHistory,
        physical_probe: bool,
    ) {
        for (index, diode) in self.devices.iter().enumerate() {
            let voltage = diode.terminal_voltage(voltages);
            let voltage = if physical_probe {
                voltage
            } else {
                diode.transient_charge_voltage(voltage)
            };
            diode.stamp_charge_companion(matrix, rhs, voltage, coeff, dt, history.branch(index));
        }
    }

    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// The index of the split RSW sidewall that belongs to device `index`,
    /// if any. The builder stores a split sidewall directly after its owner.
    /// A split sidewall is never presented as a device of its own: its
    /// terminal current, power and operating point are its owner's.
    pub(crate) fn split_sidewall_of(&self, index: usize) -> Option<usize> {
        let owner = self.devices.get(index)?;
        if owner.split_sidewall_owner().is_some() {
            return None;
        }
        let next = index + 1;
        self.devices
            .get(next)
            .filter(|sidewall| sidewall.split_sidewall_owner() == Some(owner.name.as_str()))
            .map(|_| next)
    }

    /// [`Self::split_sidewall_of`] for every device, in storage order.
    pub(crate) fn split_sidewall_indices(&self) -> Vec<Option<usize>> {
        (0..self.devices.len())
            .map(|index| self.split_sidewall_of(index))
            .collect()
    }

    pub(crate) fn add(&mut self, diode: Diode) {
        self.devices.push(diode);
    }

    pub(crate) fn len(&self) -> usize {
        self.devices.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    /// Select Xyce's native transient convergence status policy for every
    /// diode in this circuit.  The engine calls this only after the DC/startup
    /// solve, so operating-point convergence retains the strict device check.
    pub(crate) fn set_native_xyce_transient_convergence(&mut self, enabled: bool) {
        for diode in &mut self.devices {
            diode.set_native_xyce_transient_convergence(enabled);
        }
    }

    pub(crate) fn nonlinear_state_snapshot(
        &self,
    ) -> Vec<crate::device::semiconductor::DiodeNonlinearState> {
        self.devices
            .iter()
            .map(Diode::nonlinear_state_snapshot)
            .collect()
    }

    pub(crate) fn nonlinear_state_snapshot_into(
        &self,
        states: &mut Vec<crate::device::semiconductor::DiodeNonlinearState>,
    ) {
        states.clear();
        states.extend(self.devices.iter().map(Diode::nonlinear_state_snapshot));
    }

    pub(crate) fn restore_nonlinear_state(
        &mut self,
        states: Vec<crate::device::semiconductor::DiodeNonlinearState>,
    ) {
        assert_eq!(
            self.devices.len(),
            states.len(),
            "diode rollback state must match circuit topology"
        );
        for (device, state) in self.devices.iter_mut().zip(states) {
            device.restore_nonlinear_state(state);
        }
    }

    /// Update all diodes with current solution
    pub(crate) fn update_all(&mut self, voltages: &[Value]) {
        use crate::device::NonlinearDevice;
        for d in &mut self.devices {
            d.update(voltages);
        }
    }

    /// Stamp all diodes into matrix for Newton iteration
    pub(crate) fn stamp_all(
        &self,
        matrix: &mut impl MatrixStamper,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        use crate::device::NonlinearDevice;
        for d in &self.devices {
            d.stamp_nonlinear(voltages, matrix, rhs);
        }
    }

    /// Check if all diodes have converged
    pub(crate) fn all_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        use crate::device::NonlinearDevice;
        self.devices.iter().all(|d| d.is_converged(criteria))
    }

    /// Link all diodes to matrix for O(1) stamping
    pub(crate) fn link_all(&mut self, matrix: &StaticMatrix) {
        for d in &mut self.devices {
            d.link(matrix);
        }
    }

    /// Stamp all diodes using O(1) direct indexing
    pub(crate) fn stamp_all_direct(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        for d in &self.devices {
            d.stamp_direct(matrix, rhs, voltages);
        }
    }

    /// Stamp diode companions at the candidate voltage for residual probes.
    pub(crate) fn stamp_static_probe_all_direct(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        for d in &self.devices {
            d.stamp_static_probe_direct(matrix, rhs, voltages);
        }
    }
}

/// BJT storage for nonlinear Newton-Raphson iteration
#[derive(Debug, Clone, Default)]
pub(crate) struct Bjts {
    pub devices: Vec<Bjt>,
}

impl Bjts {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Add a BJT as the elaboration builds it, boxed, moving it out here
    /// rather than in the caller's frame (see the engine builder's
    /// `boxed_bjt`).
    #[inline(never)]
    pub(crate) fn add(&mut self, bjt: Box<Bjt>) {
        self.devices.push(*bjt);
    }

    pub(crate) fn len(&self) -> usize {
        self.devices.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    /// Update all BJTs with current solution
    pub(crate) fn update_all(&mut self, voltages: &[Value]) {
        use crate::device::NonlinearDevice;
        for d in &mut self.devices {
            d.update(voltages);
        }
    }

    /// Check if all BJTs have converged
    pub(crate) fn all_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        use crate::device::NonlinearDevice;
        self.devices.iter().all(|d| d.is_converged(criteria))
    }

    /// Link all BJTs to matrix for O(1) stamping
    pub(crate) fn link_all(&mut self, matrix: &StaticMatrix) {
        for d in &mut self.devices {
            d.link(matrix);
        }
    }

    /// Stamp all BJTs using O(1) direct indexing
    pub(crate) fn stamp_all_direct(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        for d in &self.devices {
            d.stamp_direct(matrix, rhs, voltages);
        }
    }
}

/// Newton-solve storage for one device family that stamps through the generic
/// [`MatrixStamper`] path rather than the O(1) `stamp_direct` linkage of
/// [`Mosfets`].
#[derive(Debug, Clone)]
pub(crate) struct NewtonFamily<D> {
    pub devices: Vec<D>,
}

impl<D> Default for NewtonFamily<D> {
    fn default() -> Self {
        Self {
            devices: Vec::new(),
        }
    }
}

impl<D: crate::device::NonlinearDevice> NewtonFamily<D> {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn add(&mut self, device: D) {
        self.devices.push(device);
    }

    pub(crate) fn len(&self) -> usize {
        self.devices.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    /// Update every device with the current solution.
    pub(crate) fn update_all(&mut self, voltages: &[Value]) {
        for d in &mut self.devices {
            d.update(voltages);
        }
    }

    /// Stamp every device into the matrix for the Newton iteration.
    pub(crate) fn stamp_all(
        &self,
        matrix: &mut impl MatrixStamper,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        for d in &self.devices {
            d.stamp_nonlinear(voltages, matrix, rhs);
        }
    }

    /// Check whether every device has converged.
    pub(crate) fn all_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        self.devices.iter().all(|d| d.is_converged(criteria))
    }
}

/// B3SOIDD (BSIMSOI level 56) storage. The SOI families integrate their coupled
/// multi-terminal charge companion in the engine's dedicated B3SOI transient
/// pass rather than the Meyer two-terminal MOSFET pathway.
pub(crate) type B3SoiDds = NewtonFamily<crate::device::B3SoiDd>;

/// B3SOIFD (BSIMSOI level 55, fully depleted) storage. FD has no body node; its
/// CAPMOD=3 charge companion goes through the same B3SOI transient pass as DD.
pub(crate) type B3SoiFds = NewtonFamily<crate::device::B3SoiFd>;

/// B3SOIPD (BSIMSOI level 57, partially depleted) storage. PD keeps a real
/// floating-body node and the full body-current set.
pub(crate) type B3SoiPds = NewtonFamily<crate::device::B3SoiPd>;

/// BSIM3v3.3 (MOS level 8/49) storage. Its coupled four-terminal charge
/// companion is integrated by the engine's dedicated BSIM3 transient pass.
pub(crate) type Bsim3v3s = NewtonFamily<crate::device::Bsim3v3Device>;

/// BSIM4 v4.8 (MOS level 14/54) storage. Its coupled four-terminal charge
/// companion is integrated by the engine's dedicated BSIM4 transient pass.
pub(crate) type Bsim4v8s = NewtonFamily<crate::device::Bsim4v8Device>;

/// Native EKV 2.6 LEVEL=260 storage.
pub(crate) type EkvMosfets = NewtonFamily<EkvMosfet>;

/// Native EKV3 LEVEL=301 NMOS150-slice storage.
pub(crate) type Ekv3Mosfets = NewtonFamily<Ekv3Device>;

/// Native VDMOS power MOSFET storage. Its internal drain/source resistance
/// topology is instance-dependent, and the VDMOS direct stamper does not take
/// part in the MOSFET fast path.
pub(crate) type Vdmoses = NewtonFamily<Vdmos>;

/// MOSFET storage for nonlinear Newton-Raphson iteration
#[derive(Debug, Clone, Default)]
pub(crate) struct Mosfets {
    pub devices: Vec<Mosfet>,
    /// True only when every device's latest collection-wide update evaluated
    /// its raw candidate without voltage limiting.
    last_update_all_is_physical: bool,
}

impl Mosfets {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn add(&mut self, mosfet: Mosfet) {
        self.devices.push(mosfet);
        self.last_update_all_is_physical = false;
    }

    pub(crate) fn len(&self) -> usize {
        self.devices.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    pub(crate) fn nonlinear_state_snapshot(
        &self,
    ) -> Vec<crate::device::mosfet::MosfetNonlinearState> {
        self.devices
            .iter()
            .map(Mosfet::nonlinear_state_snapshot)
            .collect()
    }

    pub(crate) fn nonlinear_state_snapshot_into(
        &self,
        states: &mut Vec<crate::device::mosfet::MosfetNonlinearState>,
    ) {
        states.clear();
        states.extend(self.devices.iter().map(Mosfet::nonlinear_state_snapshot));
    }

    pub(crate) fn restore_nonlinear_state(
        &mut self,
        states: Vec<crate::device::mosfet::MosfetNonlinearState>,
    ) {
        assert_eq!(
            self.devices.len(),
            states.len(),
            "classic MOS rollback state must match circuit topology"
        );
        for (device, state) in self.devices.iter_mut().zip(states) {
            device.restore_nonlinear_state(state);
        }
        self.last_update_all_is_physical = self
            .devices
            .iter()
            .all(Mosfet::cached_linearization_is_physical);
    }

    /// Update all MOSFETs with current solution
    pub(crate) fn update_all(&mut self, voltages: &[Value]) {
        use crate::device::NonlinearDevice;
        let mut all_physical = true;
        for d in &mut self.devices {
            d.update(voltages);
            all_physical &= d.cached_linearization_is_physical();
        }
        self.last_update_all_is_physical = all_physical;
    }

    /// Evaluate independent classic MOS instances on the active bounded
    /// Rayon pool. The caller owns the threshold and worker policy so small
    /// circuits never pay parallel dispatch overhead.
    #[cfg(feature = "parallel")]
    pub(crate) fn update_all_parallel(&mut self, voltages: &[Value], worker_count: usize) {
        use crate::device::NonlinearDevice;
        use rayon::prelude::*;

        // One Rayon item per very small classic-MOS evaluation lets every
        // thread in a large host pool contend for this memory-bound array.
        // Submit only the independently useful number of coarse tasks. This
        // caps active workers without constructing another nested pool and
        // retains contiguous device traversal inside each task.
        let chunk_size = self.devices.len().div_ceil(worker_count.max(1)).max(1);
        self.last_update_all_is_physical = self
            .devices
            .par_chunks_mut(chunk_size)
            .map(|chunk| {
                let mut all_physical = true;
                for device in chunk {
                    device.update(voltages);
                    all_physical &= device.cached_linearization_is_physical();
                }
                all_physical
            })
            .reduce(|| true, |left, right| left && right);
    }

    /// Whether the latest collection-wide update is also an exact physical
    /// static-probe evaluation for every classic MOS instance.
    #[inline]
    pub(crate) fn last_update_all_is_physical(&self) -> bool {
        self.last_update_all_is_physical
    }

    /// Record the aggregate result of a collection-wide update performed by
    /// a fused transient kernel.
    #[inline]
    pub(crate) fn record_last_update_all_is_physical(&mut self, physical: bool) {
        self.last_update_all_is_physical = physical;
    }

    /// Stamp all MOSFETs into matrix for Newton iteration
    pub(crate) fn stamp_all(
        &self,
        matrix: &mut impl MatrixStamper,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        use crate::device::NonlinearDevice;
        for d in &self.devices {
            d.stamp_nonlinear(voltages, matrix, rhs);
        }
    }

    /// Check if all MOSFETs have converged
    pub(crate) fn all_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        use crate::device::NonlinearDevice;
        self.devices.iter().all(|d| d.is_converged(criteria))
    }

    /// Link all MOSFETs to matrix for O(1) stamping
    pub(crate) fn link_all(&mut self, matrix: &StaticMatrix) {
        for d in &mut self.devices {
            d.link(matrix);
        }
    }

    /// Stamp all MOSFETs using O(1) direct indexing
    pub(crate) fn stamp_all_direct(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        for d in &self.devices {
            d.stamp_direct(matrix, rhs, voltages);
        }
    }

    /// Stamp device linearizations whose cached biases are already known to
    /// match the active Newton iterate.
    pub(crate) fn stamp_all_cached_direct(&self, matrix: &mut StaticMatrix, rhs: &mut [Value]) {
        for device in &self.devices {
            device.stamp_cached_direct(matrix, rhs);
        }
    }

    /// Stamp exact candidate equations for residual and line-search probes.
    pub(crate) fn stamp_all_static_probe_direct(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        for d in &self.devices {
            d.stamp_static_probe_direct(matrix, rhs, voltages);
        }
    }
}
