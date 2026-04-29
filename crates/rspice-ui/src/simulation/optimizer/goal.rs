use serde::{Deserialize, Serialize};

// =============================================================================
// Optimization Goal
// =============================================================================

/// Strategy for meeting an optimization goal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GoalStrategy {
    /// Minimize value (e.g., power, area)
    Minimize,
    /// Maximize value (e.g., gain, bandwidth)
    Maximize,
    /// Hit target value precisely
    Target,
    /// Stay within range
    Range,
}

/// A target goal for optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationGoal {
    /// Name of the measurement/signal
    pub target: String,
    /// Strategy to use
    pub strategy: GoalStrategy,
    /// Target value (for Target strategy)
    pub target_val: Option<f64>,
    /// Acceptable range (min, max)
    pub range: Option<(f64, f64)>,
    /// Weight/Priority (0.0 - 1.0)
    pub weight: f32,
}

impl OptimizationGoal {
    /// Create a maximization goal
    pub fn maximize(target: impl Into<String>) -> Self {
        Self {
            target: target.into(),
            strategy: GoalStrategy::Maximize,
            target_val: None,
            range: None,
            weight: 1.0,
        }
    }

    /// Create a minimization goal
    pub fn minimize(target: impl Into<String>) -> Self {
        Self {
            target: target.into(),
            strategy: GoalStrategy::Minimize,
            target_val: None,
            range: None,
            weight: 1.0,
        }
    }

    /// Create a target goal
    pub fn hit_target(target: impl Into<String>, val: f64) -> Self {
        Self {
            target: target.into(),
            strategy: GoalStrategy::Target,
            target_val: Some(val),
            range: None,
            weight: 1.0,
        }
    }

    /// Calculate cost (error) for a given value
    /// 0.0 means perfect match, higher means worse
    pub fn calculate_cost(&self, current_val: f64) -> f64 {
        match self.strategy {
            GoalStrategy::Maximize => {
                // Return 1/val or 1-val depending on normalization
                if current_val > 0.0 {
                    1.0 / current_val
                } else {
                    f64::MAX
                }
            }
            GoalStrategy::Minimize => current_val.abs(),
            GoalStrategy::Target => {
                let target = self.target_val.unwrap_or(0.0);
                (current_val - target).powi(2)
            }
            GoalStrategy::Range => {
                if let Some((min, max)) = self.range {
                    if current_val < min {
                        (min - current_val).powi(2)
                    } else if current_val > max {
                        (current_val - max).powi(2)
                    } else {
                        0.0
                    }
                } else {
                    0.0
                }
            }
        }
    }
}
