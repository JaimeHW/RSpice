use super::*;

/// Units and storage ownership of a non-nodal MNA equation. A flux row
/// contains Q=-flux linkage and F=terminal voltage (including any physical
/// series voltage terms). Its current remains finite through this operator;
/// topologies requiring a voltage impulse need a descriptor transition.
#[derive(Clone, Copy)]
pub(in crate::engine::transient) enum EventBranchEquation {
    /// The absolute tolerance has the units of F, usually volts for RBI.
    Algebraic(Value),
    Flux {
        /// Weber-turns, independently of the nodal charge tolerance.
        flux_tolerance: Value,
        voltage_tolerance: Value,
    },
}

impl EventBranchEquation {
    pub(super) fn valid(self) -> bool {
        match self {
            Self::Algebraic(absolute) => absolute.is_finite() && absolute > 0.0,
            Self::Flux {
                flux_tolerance,
                voltage_tolerance,
            } => [flux_tolerance, voltage_tolerance]
                .into_iter()
                .all(|value| value.is_finite() && value > 0.0),
        }
    }

    pub(super) fn flux_tolerance(self) -> Option<Value> {
        match self {
            Self::Algebraic(_) => None,
            Self::Flux { flux_tolerance, .. } => Some(flux_tolerance),
        }
    }

    pub(super) fn jump_tolerance(self) -> Value {
        match self {
            Self::Algebraic(absolute) => absolute,
            Self::Flux { flux_tolerance, .. } => flux_tolerance,
        }
    }

    pub(super) fn rate_tolerance(self) -> Value {
        match self {
            // This equation was differentiated: its audit uses relative
            // backward error in rate units, with no invented time interval.
            Self::Algebraic(_) => 0.0,
            Self::Flux {
                voltage_tolerance, ..
            } => voltage_tolerance,
        }
    }
}
