//! Device behavior traits

use crate::{NodeId, Value};

/// SPICE-style tolerances used to accept or reject a nonlinear Newton step.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NonlinearConvergenceCriteria {
    pub voltage_abs: Value,
    pub current_abs: Value,
    pub rel: Value,
}

impl Default for NonlinearConvergenceCriteria {
    fn default() -> Self {
        Self {
            voltage_abs: 1e-6,
            current_abs: 1e-12,
            rel: 1e-3,
        }
    }
}

impl NonlinearConvergenceCriteria {
    #[inline]
    pub fn new(voltage_abs: Value, current_abs: Value, rel: Value) -> Self {
        Self {
            voltage_abs,
            current_abs,
            rel,
        }
    }

    #[inline]
    pub fn voltage_tolerance(self) -> Value {
        if self.voltage_abs.is_finite() && self.voltage_abs > 0.0 {
            self.voltage_abs
        } else {
            Self::default().voltage_abs
        }
    }

    #[inline]
    pub fn current_tolerance(self) -> Value {
        if self.current_abs.is_finite() && self.current_abs > 0.0 {
            self.current_abs
        } else {
            Self::default().current_abs
        }
    }

    #[inline]
    pub fn relative_tolerance(self) -> Value {
        if self.rel.is_finite() && self.rel > 0.0 {
            self.rel
        } else {
            Self::default().rel
        }
    }
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
    fn is_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool;

    /// Forget the operating point and the limiter history, so that the next
    /// load is the instance's MODEINITJCT load again: its junction seed, or
    /// zero bias for an `OFF` instance, with no previous iterate to limit
    /// against.
    ///
    /// ngspice opens every operating-point aid on that load (cktop.c):
    /// `dynamic_gmin`, `new_gmin` and `gillespie_src` set `CKTmode` to
    /// `firstmode` and zero `CKTstate0`, and `spice3_gmin` and `spice3_src`
    /// set the mode. A device with neither a limiter nor an operating-point
    /// history keeps this default.
    fn reopen_operating_point(&mut self) {}
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
