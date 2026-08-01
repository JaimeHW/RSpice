//! Optimization goals.
//!
//! What the search is trying to achieve — minimize, maximize, or meet a
//! target — and how multiple goals combine into one score.

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
}
