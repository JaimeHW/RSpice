//! Device behavior traits

use crate::{Value, circuit::NodeId};

/// Trait for devices that contribute to the conductance matrix (linear elements)
pub trait LinearDevice {
    /// Stamp the device into the conductance matrix
    fn stamp_linear(&self, matrix: &mut impl MatrixStamper, rhs: &mut [Value]);
}

/// Trait for devices that require Newton-Raphson iteration (nonlinear elements)
pub trait NonlinearDevice {
    /// Update the device state based on current node voltages
    fn update(&mut self, voltages: &[Value]);

    /// Stamp the device's linearized contribution
    fn stamp_nonlinear(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
        rhs: &mut [Value],
    );

    /// Check if the device has converged
    fn is_converged(&self, tolerance: Value) -> bool;
}

/// Trait for devices with dynamic behavior (capacitors, inductors)
pub trait DynamicDevice {
    /// Stamp the device for transient analysis using companion model
    fn stamp_transient(
        &self,
        voltages: &[Value],
        dt: Value,
        matrix: &mut impl MatrixStamper,
        rhs: &mut [Value],
    );

    /// Update internal state after a time step
    fn step(&mut self, voltages: &[Value], dt: Value);
}

/// Trait for stamping values into the circuit matrix
pub trait MatrixStamper {
    /// Add a value to the conductance matrix at (row, col)
    fn stamp(&mut self, row: NodeId, col: NodeId, value: Value);

    /// Add a value to the RHS vector at index
    fn stamp_rhs(&mut self, index: NodeId, value: Value);
}
