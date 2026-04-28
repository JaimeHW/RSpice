use super::*;

#[derive(Debug, Default)]
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
}

/// BJT storage for nonlinear Newton-Raphson iteration
#[derive(Debug, Default)]
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

/// MOSFET storage for nonlinear Newton-Raphson iteration
#[derive(Debug, Default)]
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
}
