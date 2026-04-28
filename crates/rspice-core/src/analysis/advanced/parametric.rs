//! Parametric Analysis (.STEP directive)
//!
//! Implements parameter sweeping for design exploration and optimization.
//!
//! # SPICE Syntax
//! ```text
//! .STEP PARAM <name> <start> <stop> <step>
//! .STEP PARAM <name> LIST <v1> <v2> <v3> ...
//! .STEP <element> <param> <start> <stop> <step>
//! .STEP LIN <element> <param> <start> <stop> <points>
//! .STEP DEC <element> <param> <start> <stop> <points>
//! .STEP OCT <element> <param> <start> <stop> <points>
//! ```
//!
//! # Examples
//! ```text
//! .STEP PARAM R1 1k 10k 1k           ; Linear step
//! .STEP PARAM Cload LIST 10p 22p 47p ; Discrete list
//! .STEP R1 1k 10k 1k                 ; Resistor value sweep
//! .STEP DEC Cfilter 1p 1u 10         ; Decade sweep
//! ```

use crate::Value;

//=============================================================================
// Step Type
//=============================================================================

/// Type of parametric step
#[derive(Debug, Clone, PartialEq)]
pub enum StepType {
    /// Linear step: start to stop with fixed increment
    Linear {
        start: Value,
        stop: Value,
        step: Value,
    },
    /// Linear with specified number of points
    LinearPoints {
        start: Value,
        stop: Value,
        points: usize,
    },
    /// Logarithmic decade step
    Decade {
        start: Value,
        stop: Value,
        points_per_decade: usize,
    },
    /// Logarithmic octave step
    Octave {
        start: Value,
        stop: Value,
        points_per_octave: usize,
    },
    /// Explicit list of values
    List(Vec<Value>),
}

impl StepType {
    /// Generate all step values
    pub fn values(&self) -> Vec<Value> {
        match self {
            StepType::Linear { start, stop, step } => {
                let mut values = Vec::new();
                let mut v = *start;
                let direction = if stop >= start { 1.0 } else { -1.0 };
                let step_abs = step.abs() * direction;

                while (direction > 0.0 && v <= *stop + step_abs * 0.5)
                    || (direction < 0.0 && v >= *stop + step_abs * 0.5)
                {
                    values.push(v);
                    v += step_abs;
                }
                values
            }
            StepType::LinearPoints {
                start,
                stop,
                points,
            } => {
                if *points <= 1 {
                    return vec![*start];
                }
                (0..*points)
                    .map(|i| start + (stop - start) * (i as f64) / ((*points - 1) as f64))
                    .collect()
            }
            StepType::Decade {
                start,
                stop,
                points_per_decade,
            } => {
                let start_log = start.log10();
                let stop_log = stop.log10();
                let num_decades = (stop_log - start_log).abs();
                let total_points = (num_decades * (*points_per_decade as f64)).ceil() as usize + 1;

                (0..total_points)
                    .map(|i| {
                        let log_v = start_log
                            + (stop_log - start_log) * (i as f64)
                                / ((total_points - 1).max(1) as f64);
                        10.0_f64.powf(log_v)
                    })
                    .collect()
            }
            StepType::Octave {
                start,
                stop,
                points_per_octave,
            } => {
                let start_log2 = start.log2();
                let stop_log2 = stop.log2();
                let num_octaves = (stop_log2 - start_log2).abs();
                let total_points = (num_octaves * (*points_per_octave as f64)).ceil() as usize + 1;

                (0..total_points)
                    .map(|i| {
                        let log_v = start_log2
                            + (stop_log2 - start_log2) * (i as f64)
                                / ((total_points - 1).max(1) as f64);
                        2.0_f64.powf(log_v)
                    })
                    .collect()
            }
            StepType::List(values) => values.clone(),
        }
    }

    /// Get number of step points
    pub fn num_points(&self) -> usize {
        self.values().len()
    }
}

//=============================================================================
// Step Specification
//=============================================================================

/// Target of a parametric step
#[derive(Debug, Clone, PartialEq)]
pub enum StepTarget {
    /// Step a named parameter (.PARAM)
    Parameter(String),
    /// Step a device property
    Device {
        device_name: String,
        property: Option<String>,
    },
    /// Step a model parameter
    Model {
        model_name: String,
        param_name: String,
    },
    /// Temperature sweep
    Temperature,
}

/// Complete step specification
#[derive(Debug, Clone)]
pub struct StepSpec {
    /// What to step
    pub target: StepTarget,
    /// How to step
    pub step_type: StepType,
}

impl StepSpec {
    /// Create a new parameter step
    pub fn param(name: &str, start: Value, stop: Value, step: Value) -> Self {
        Self {
            target: StepTarget::Parameter(name.to_string()),
            step_type: StepType::Linear { start, stop, step },
        }
    }

    /// Create a parameter step with list of values
    pub fn param_list(name: &str, values: Vec<Value>) -> Self {
        Self {
            target: StepTarget::Parameter(name.to_string()),
            step_type: StepType::List(values),
        }
    }

    /// Create a device value step
    pub fn device(device_name: &str, start: Value, stop: Value, step: Value) -> Self {
        Self {
            target: StepTarget::Device {
                device_name: device_name.to_string(),
                property: None,
            },
            step_type: StepType::Linear { start, stop, step },
        }
    }

    /// Create a decade sweep
    pub fn decade(target: StepTarget, start: Value, stop: Value, points_per_decade: usize) -> Self {
        Self {
            target,
            step_type: StepType::Decade {
                start,
                stop,
                points_per_decade,
            },
        }
    }

    /// Create a temperature sweep
    pub fn temperature(start: Value, stop: Value, step: Value) -> Self {
        Self {
            target: StepTarget::Temperature,
            step_type: StepType::Linear { start, stop, step },
        }
    }

    /// Get all values for this step
    pub fn values(&self) -> Vec<Value> {
        self.step_type.values()
    }
}

//=============================================================================
// Parametric Sweep Controller
//=============================================================================

/// Controller for multi-dimensional parametric sweeps
#[derive(Debug, Clone)]
pub struct ParametricSweep {
    /// Step specifications (can have multiple for multi-dimensional)
    steps: Vec<StepSpec>,
    /// Current indices into each step dimension
    current_indices: Vec<usize>,
    /// Total number of combinations
    total_combinations: usize,
    /// Current combination index
    current_combination: usize,
}

impl ParametricSweep {
    /// Create a new parametric sweep with given steps
    pub fn new(steps: Vec<StepSpec>) -> Self {
        let total = steps.iter().map(|s| s.step_type.num_points()).product();
        let current_indices = vec![0; steps.len()];

        Self {
            steps,
            current_indices,
            total_combinations: total,
            current_combination: 0,
        }
    }

    /// Create an empty sweep (single run with no stepping)
    pub fn empty() -> Self {
        Self {
            steps: Vec::new(),
            current_indices: Vec::new(),
            total_combinations: 1,
            current_combination: 0,
        }
    }

    /// Add a step specification
    pub fn add_step(&mut self, step: StepSpec) {
        self.steps.push(step);
        self.current_indices.push(0);
        self.total_combinations = self
            .steps
            .iter()
            .map(|s| s.step_type.num_points())
            .product();
    }

    /// Get number of step dimensions
    pub fn num_dimensions(&self) -> usize {
        self.steps.len()
    }

    /// Get total number of combinations
    pub fn total_combinations(&self) -> usize {
        self.total_combinations
    }

    /// Get current combination index (0-based)
    pub fn current_index(&self) -> usize {
        self.current_combination
    }

    /// Check if sweep is complete
    pub fn is_complete(&self) -> bool {
        self.current_combination >= self.total_combinations
    }

    /// Get current parameter values as (target, value) pairs
    pub fn current_values(&self) -> Vec<(&StepTarget, Value)> {
        self.steps
            .iter()
            .zip(self.current_indices.iter())
            .map(|(step, &idx)| {
                let values = step.values();
                let value = values.get(idx).copied().unwrap_or(values[0]);
                (&step.target, value)
            })
            .collect()
    }

    /// Advance to next combination
    /// Returns false if sweep is complete
    pub fn advance(&mut self) -> bool {
        if self.is_complete() {
            return false;
        }

        self.current_combination += 1;

        if self.current_combination >= self.total_combinations {
            return false;
        }

        // Increment indices (like counting in mixed radix)
        let mut carry = true;
        for i in (0..self.steps.len()).rev() {
            if carry {
                self.current_indices[i] += 1;
                let max = self.steps[i].step_type.num_points();
                if self.current_indices[i] >= max {
                    self.current_indices[i] = 0;
                    carry = true;
                } else {
                    carry = false;
                }
            }
        }

        true
    }

    /// Reset to beginning
    pub fn reset(&mut self) {
        self.current_indices = vec![0; self.steps.len()];
        self.current_combination = 0;
    }

    /// Get progress as fraction (0.0 to 1.0)
    pub fn progress(&self) -> Value {
        if self.total_combinations == 0 {
            return 1.0;
        }
        self.current_combination as f64 / self.total_combinations as f64
    }
}

impl Default for ParametricSweep {
    fn default() -> Self {
        Self::empty()
    }
}

//=============================================================================
// Stepped Result Storage
//=============================================================================

/// Result from a single step
#[derive(Debug, Clone)]
pub struct StepResult<T> {
    /// Parameter values for this step
    pub param_values: Vec<(String, Value)>,
    /// Result data
    pub data: T,
}

/// Collection of results from a parametric sweep
#[derive(Debug, Clone)]
pub struct ParametricResults<T> {
    /// All results
    pub results: Vec<StepResult<T>>,
    /// Step specifications used
    pub step_specs: Vec<StepSpec>,
}

impl<T> ParametricResults<T> {
    /// Create new empty results container
    pub fn new(step_specs: Vec<StepSpec>) -> Self {
        Self {
            results: Vec::new(),
            step_specs,
        }
    }

    /// Add a result
    pub fn add(&mut self, param_values: Vec<(String, Value)>, data: T) {
        self.results.push(StepResult { param_values, data });
    }

    /// Get number of results
    pub fn len(&self) -> usize {
        self.results.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }

    /// Get result by index
    pub fn get(&self, index: usize) -> Option<&StepResult<T>> {
        self.results.get(index)
    }

    /// Iterate over results
    pub fn iter(&self) -> impl Iterator<Item = &StepResult<T>> {
        self.results.iter()
    }
}

//=============================================================================
// Tests
//=============================================================================

