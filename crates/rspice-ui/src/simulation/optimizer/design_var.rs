use serde::{Deserialize, Serialize};

// =============================================================================
// Design Variables
// =============================================================================

/// A design variable that can be tuned by the optimizer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignVar {
    /// Variable name (must match schematic VAR)
    pub name: String,
    /// Current value
    pub value: f64,
    /// Nominal/Center value
    pub nominal: f64,
    /// Lower bound
    pub min: f64,
    /// Upper bound
    pub max: f64,
    /// Hard constraint? (cannot exceed)
    pub hard: bool,
}

impl DesignVar {
    pub fn new(name: impl Into<String>, value: f64, min: f64, max: f64) -> Self {
        Self {
            name: name.into(),
            value,
            nominal: value,
            min,
            max,
            hard: true,
        }
    }

    /// Update value with clamping
    pub fn update(&mut self, new_val: f64) {
        self.value = new_val.clamp(self.min, self.max);
    }
}
