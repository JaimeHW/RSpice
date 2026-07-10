use super::*;

#[derive(Debug, Clone, Default)]
pub struct Diodes {
    pub devices: Vec<Diode>,
}

impl Diodes {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, diode: Diode) {
        self.devices.push(diode);
    }

    pub fn len(&self) -> usize {
        self.devices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    /// Update all diodes with current solution
    pub fn update_all(&mut self, voltages: &[Value]) {
        use crate::device::NonlinearDevice;
        for d in &mut self.devices {
            d.update(voltages);
        }
    }

    /// Stamp all diodes into matrix for Newton iteration
    pub fn stamp_all(
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
    pub fn all_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        use crate::device::NonlinearDevice;
        self.devices.iter().all(|d| d.is_converged(criteria))
    }

    /// Link all diodes to matrix for O(1) stamping
    pub fn link_all(&mut self, matrix: &StaticMatrix) {
        for d in &mut self.devices {
            d.link(matrix);
        }
    }

    /// Stamp all diodes using O(1) direct indexing
    pub fn stamp_all_direct(
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
    pub fn stamp_static_probe_all_direct(
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
pub struct Bjts {
    pub devices: Vec<Bjt>,
}

impl Bjts {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, bjt: Bjt) {
        self.devices.push(bjt);
    }

    pub fn len(&self) -> usize {
        self.devices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    /// Update all BJTs with current solution
    pub fn update_all(&mut self, voltages: &[Value]) {
        use crate::device::NonlinearDevice;
        for d in &mut self.devices {
            d.update(voltages);
        }
    }

    /// Stamp all BJTs into matrix for Newton iteration
    pub fn stamp_all(
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

    /// Check if all BJTs have converged
    pub fn all_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        use crate::device::NonlinearDevice;
        self.devices.iter().all(|d| d.is_converged(criteria))
    }

    /// Link all BJTs to matrix for O(1) stamping
    pub fn link_all(&mut self, matrix: &StaticMatrix) {
        for d in &mut self.devices {
            d.link(matrix);
        }
    }

    /// Stamp all BJTs using O(1) direct indexing
    pub fn stamp_all_direct(
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

/// B3SOIDD (BSIMSOI level 56) storage for nonlinear Newton-Raphson iteration.
///
/// Mirrors [`Mosfets`] but holds [`crate::device::B3SoiDd`] instances. These SOI
/// devices stamp through the generic [`MatrixStamper`] path (not the O(1)
/// `stamp_direct` linkage) because their coupled multi-terminal charge companion
/// is integrated by the engine's dedicated B3SOI transient pass rather than the
/// Meyer two-terminal MOSFET pathway.
#[derive(Debug, Clone, Default)]
pub struct B3SoiDds {
    pub devices: Vec<crate::device::B3SoiDd>,
}

impl B3SoiDds {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, device: crate::device::B3SoiDd) {
        self.devices.push(device);
    }

    pub fn len(&self) -> usize {
        self.devices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    /// Update all B3SOIDD devices with the current solution.
    pub fn update_all(&mut self, voltages: &[Value]) {
        use crate::device::NonlinearDevice;
        for d in &mut self.devices {
            d.update(voltages);
        }
    }

    /// Stamp all B3SOIDD devices into the matrix for the Newton iteration.
    pub fn stamp_all(
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

    /// Check whether all B3SOIDD devices have converged.
    pub fn all_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        use crate::device::NonlinearDevice;
        self.devices.iter().all(|d| d.is_converged(criteria))
    }
}

/// B3SOIFD (BSIMSOI level 55, fully depleted) storage for the Newton solve.
///
/// Mirrors [`B3SoiDds`] but holds [`crate::device::B3SoiFd`] instances. FD has no
/// body node and its CAPMOD=3 charge companion is integrated by the same engine
/// B3SOI transient pass as DD, so it shares the orchestration shape.
#[derive(Debug, Clone, Default)]
pub struct B3SoiFds {
    pub devices: Vec<crate::device::B3SoiFd>,
}

impl B3SoiFds {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, device: crate::device::B3SoiFd) {
        self.devices.push(device);
    }

    pub fn len(&self) -> usize {
        self.devices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    /// Update all B3SOIFD devices with the current solution.
    pub fn update_all(&mut self, voltages: &[Value]) {
        use crate::device::NonlinearDevice;
        for d in &mut self.devices {
            d.update(voltages);
        }
    }

    /// Stamp all B3SOIFD devices into the matrix for the Newton iteration.
    pub fn stamp_all(
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

    /// Check whether all B3SOIFD devices have converged.
    pub fn all_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        use crate::device::NonlinearDevice;
        self.devices.iter().all(|d| d.is_converged(criteria))
    }
}

/// B3SOIPD (BSIMSOI level 57, partially depleted) storage for the Newton solve.
///
/// Mirrors [`B3SoiDds`] but holds [`crate::device::B3SoiPd`] instances. PD keeps
/// a real floating-body node and the full body-current set, so its orchestration
/// matches DD's exactly.
#[derive(Debug, Clone, Default)]
pub struct B3SoiPds {
    pub devices: Vec<crate::device::B3SoiPd>,
}

impl B3SoiPds {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, device: crate::device::B3SoiPd) {
        self.devices.push(device);
    }

    pub fn len(&self) -> usize {
        self.devices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    /// Update all B3SOIPD devices with the current solution.
    pub fn update_all(&mut self, voltages: &[Value]) {
        use crate::device::NonlinearDevice;
        for d in &mut self.devices {
            d.update(voltages);
        }
    }

    /// Stamp all B3SOIPD devices into the matrix for the Newton iteration.
    pub fn stamp_all(
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

    /// Check whether all B3SOIPD devices have converged.
    pub fn all_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        use crate::device::NonlinearDevice;
        self.devices.iter().all(|d| d.is_converged(criteria))
    }
}

/// BSIM3v3.3 (MOS level 8/49) storage for the Newton solve.
///
/// Mirrors [`B3SoiPds`] but holds [`crate::device::Bsim3v3Device`] instances.
/// Like the SOI family, these stamp through the generic [`MatrixStamper`]
/// path: their coupled four-terminal charge companion is integrated by the
/// engine's dedicated BSIM3 transient pass, not the Meyer two-terminal
/// MOSFET pathway.
#[derive(Debug, Clone, Default)]
pub struct Bsim3v3s {
    pub devices: Vec<crate::device::Bsim3v3Device>,
}

impl Bsim3v3s {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, device: crate::device::Bsim3v3Device) {
        self.devices.push(device);
    }

    pub fn len(&self) -> usize {
        self.devices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    /// Update all BSIM3 devices with the current solution.
    pub fn update_all(&mut self, voltages: &[Value]) {
        use crate::device::NonlinearDevice;
        for d in &mut self.devices {
            d.update(voltages);
        }
    }

    /// Stamp all BSIM3 devices into the matrix for the Newton iteration.
    pub fn stamp_all(
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

    /// Check whether all BSIM3 devices have converged.
    pub fn all_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        use crate::device::NonlinearDevice;
        self.devices.iter().all(|d| d.is_converged(criteria))
    }
}

/// BSIM4 v4.8 (MOS level 14/54) storage for the Newton solve.
///
/// Mirrors [`Bsim3v3s`] but holds [`crate::device::Bsim4v8Device`]
/// instances: the same generic [`MatrixStamper`] path, with the coupled
/// four-terminal charge companion integrated by the engine's dedicated
/// BSIM4 transient pass.
#[derive(Debug, Clone, Default)]
pub struct Bsim4v8s {
    pub devices: Vec<crate::device::Bsim4v8Device>,
}

impl Bsim4v8s {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, device: crate::device::Bsim4v8Device) {
        self.devices.push(device);
    }

    pub fn len(&self) -> usize {
        self.devices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    /// Update all BSIM4 devices with the current solution.
    pub fn update_all(&mut self, voltages: &[Value]) {
        use crate::device::NonlinearDevice;
        for d in &mut self.devices {
            d.update(voltages);
        }
    }

    /// Stamp all BSIM4 devices into the matrix for the Newton iteration.
    pub fn stamp_all(
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

    /// Check whether all BSIM4 devices have converged.
    pub fn all_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        use crate::device::NonlinearDevice;
        self.devices.iter().all(|d| d.is_converged(criteria))
    }
}

/// Native EKV 2.6 LEVEL=260 storage for the Newton solve.
#[derive(Debug, Clone, Default)]
pub struct EkvMosfets {
    pub devices: Vec<EkvMosfet>,
}

impl EkvMosfets {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, device: EkvMosfet) {
        self.devices.push(device);
    }

    pub fn len(&self) -> usize {
        self.devices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    /// Update all EKV 2.6 devices with the current solution.
    pub fn update_all(&mut self, voltages: &[Value]) {
        use crate::device::NonlinearDevice;
        for d in &mut self.devices {
            d.update(voltages);
        }
    }

    /// Stamp all EKV 2.6 devices into the matrix for the Newton iteration.
    pub fn stamp_all(
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

    /// Check whether all EKV 2.6 devices have converged.
    pub fn all_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        use crate::device::NonlinearDevice;
        self.devices.iter().all(|d| d.is_converged(criteria))
    }
}

/// Native EKV3 LEVEL=301 NMOS150-slice storage for the Newton solve.
#[derive(Debug, Clone, Default)]
pub struct Ekv3Mosfets {
    pub devices: Vec<Ekv3Device>,
}

impl Ekv3Mosfets {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, device: Ekv3Device) {
        self.devices.push(device);
    }

    pub fn len(&self) -> usize {
        self.devices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    /// Update all EKV3 devices with the current solution.
    pub fn update_all(&mut self, voltages: &[Value]) {
        use crate::device::NonlinearDevice;
        for d in &mut self.devices {
            d.update(voltages);
        }
    }

    /// Stamp all EKV3 devices into the matrix for the Newton iteration.
    pub fn stamp_all(
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

    /// Check whether all EKV3 devices have converged.
    pub fn all_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        use crate::device::NonlinearDevice;
        self.devices.iter().all(|d| d.is_converged(criteria))
    }
}

/// Native VDMOS power MOSFET storage.
///
/// These devices use the generic [`MatrixStamper`] path: their internal
/// drain/source resistance topology is instance-dependent, and the VDMOS
/// direct stamper does not participate in the legacy MOSFET fast path.
#[derive(Debug, Clone, Default)]
pub struct Vdmoses {
    pub devices: Vec<Vdmos>,
}

impl Vdmoses {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, vdmos: Vdmos) {
        self.devices.push(vdmos);
    }

    pub fn len(&self) -> usize {
        self.devices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    /// Update all VDMOS devices with the current solution.
    pub fn update_all(&mut self, voltages: &[Value]) {
        use crate::device::NonlinearDevice;
        for d in &mut self.devices {
            d.update(voltages);
        }
    }

    /// Stamp all VDMOS devices into the matrix for the Newton iteration.
    pub fn stamp_all(
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

    /// Check whether all VDMOS devices have converged.
    pub fn all_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        use crate::device::NonlinearDevice;
        self.devices.iter().all(|d| d.is_converged(criteria))
    }
}

/// MOSFET storage for nonlinear Newton-Raphson iteration
#[derive(Debug, Clone, Default)]
pub struct Mosfets {
    pub devices: Vec<Mosfet>,
}

impl Mosfets {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, mosfet: Mosfet) {
        self.devices.push(mosfet);
    }

    pub fn len(&self) -> usize {
        self.devices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    /// Update all MOSFETs with current solution
    pub fn update_all(&mut self, voltages: &[Value]) {
        use crate::device::NonlinearDevice;
        for d in &mut self.devices {
            d.update(voltages);
        }
    }

    /// Stamp all MOSFETs into matrix for Newton iteration
    pub fn stamp_all(
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
    pub fn all_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        use crate::device::NonlinearDevice;
        self.devices.iter().all(|d| d.is_converged(criteria))
    }

    /// Link all MOSFETs to matrix for O(1) stamping
    pub fn link_all(&mut self, matrix: &StaticMatrix) {
        for d in &mut self.devices {
            d.link(matrix);
        }
    }

    /// Stamp all MOSFETs using O(1) direct indexing
    pub fn stamp_all_direct(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        for d in &self.devices {
            d.stamp_direct(matrix, rhs, voltages);
        }
    }

    /// Stamp exact candidate equations for residual and line-search probes.
    pub fn stamp_all_static_probe_direct(
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
