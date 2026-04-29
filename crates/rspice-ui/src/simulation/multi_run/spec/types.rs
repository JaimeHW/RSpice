use crate::simulation::multi_run::FrequencySweep;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OptimizationGoal {
    /// Minimize objective value.
    Minimize,
    /// Maximize objective value.
    Maximize,
    /// Reach target objective value.
    Target,
}

/// Optimization algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OptimizationAlgorithm {
    /// Gradient descent with line search.
    GradientDescent,
    /// Pattern search.
    PatternSearch,
    /// Simulated annealing.
    SimulatedAnnealing,
}

/// Optimization variable bounds and initial value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptimizationVariable {
    /// Parameter name from netlist `.param`.
    pub name: String,
    /// Lower bound.
    pub min: f64,
    /// Upper bound.
    pub max: f64,
    /// Initial value.
    pub initial: f64,
}

/// S-parameter port definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpPort {
    /// Positive node.
    pub node_pos: String,
    /// Negative/reference node.
    pub node_neg: String,
    /// Optional per-port reference impedance override.
    pub z0: Option<f64>,
}

/// Harmonic balance tone request used by pipeline execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HbToneSpec {
    /// Tone frequency in Hz.
    pub frequency: f64,
    /// Number of harmonics requested for this tone.
    pub harmonics: usize,
    /// Optional independent source name this tone should drive.
    pub source: Option<String>,
    /// Optional label for display/debug.
    pub name: Option<String>,
}

impl HbToneSpec {
    pub fn new(frequency: f64, harmonics: usize) -> Self {
        Self {
            frequency,
            harmonics,
            source: None,
            name: None,
        }
    }

    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        let source = source.into();
        self.source = if source.trim().is_empty() {
            None
        } else {
            Some(source)
        };
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        let name = name.into();
        self.name = if name.trim().is_empty() {
            None
        } else {
            Some(name)
        };
        self
    }
}

/// Strongly-typed analysis request used by queue execution.
///
/// This removes hidden/default execution parameters from the run executor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnalysisSpec {
    /// DC operating point
    DcOp,
    /// DC sweep
    DcSweep {
        source_name: String,
        start: f64,
        stop: f64,
        step: f64,
        source2: Option<String>,
        start2: Option<f64>,
        stop2: Option<f64>,
        step2: Option<f64>,
    },
    /// AC analysis
    Ac {
        start_freq: f64,
        stop_freq: f64,
        points_per_unit: usize,
        sweep: FrequencySweep,
    },
    /// Distortion analysis
    Disto {
        start_freq: f64,
        stop_freq: f64,
        points_per_unit: usize,
        sweep: FrequencySweep,
        f2_over_f1: Option<f64>,
    },
    /// Transient analysis
    Transient {
        stop_time: f64,
        step_time: f64,
        start_time: f64,
        max_timestep: Option<f64>,
        uic: bool,
    },
    /// Noise analysis
    Noise {
        output_node: String,
        start_freq: f64,
        stop_freq: f64,
        points_per_decade: usize,
        temperature: f64,
    },
    /// Periodic steady-state
    Pss {
        fundamental_freq: f64,
        num_harmonics: usize,
        tolerance: f64,
    },
    /// Harmonic balance
    HarmonicBalance {
        tones: Vec<HbToneSpec>,
        reltol: f64,
        abstol: f64,
        max_iterations: usize,
        damping: f64,
        oversample: usize,
        max_mixing_order: usize,
        use_krylov: bool,
        gmres_restart: usize,
        source_stepping: bool,
        verbose: bool,
    },
    /// Transfer function
    Tf,
    /// Sensitivity
    Sensitivity {
        output_var: String,
        ac_mode: bool,
        frequency: Option<f64>,
    },
    /// Pole-zero
    PoleZero {
        input_node: String,
        input_ref: String,
        output_node: String,
        output_ref: String,
        transfer_type: String,
        analysis_type: String,
    },
    /// Periodic AC
    Pac,
    /// Periodic noise
    Pnoise,
    /// Periodic transfer function
    Pxf,
    /// Periodic stability
    Pstb,
    /// Stability analysis
    Stb {
        probe_node: String,
        start_freq: f64,
        stop_freq: f64,
        points_per_decade: usize,
    },
    /// Monte Carlo
    MonteCarlo,
    /// Parametric sweep
    Parametric,
    /// Corner analysis
    Corner,
    /// Reliability aging analysis
    Reliability {
        target_years: Vec<f64>,
        enable_hci: bool,
        enable_nbti: bool,
        enable_em: bool,
        min_stress_voltage: f64,
    },
    /// Optimization analysis.
    Optimization {
        variables: Vec<OptimizationVariable>,
        objective_node: String,
        objective_ref: String,
        goal: OptimizationGoal,
        target: Option<f64>,
        algorithm: OptimizationAlgorithm,
        max_iterations: usize,
        cost_tolerance: f64,
        fd_step: f64,
        initial_step: f64,
        min_step: f64,
    },
    /// Safety / SOA analysis.
    Soa {
        stop_time: f64,
        step_time: f64,
        check_vgs_max: bool,
        max_vgs: f64,
        check_vds_max: bool,
        max_vds: f64,
        check_vbe_max: bool,
        max_vbe: f64,
        check_vce_max: bool,
        max_vce: f64,
    },
    /// S-parameter analysis
    SParameter {
        start_freq: f64,
        stop_freq: f64,
        points_per_unit: usize,
        sweep: FrequencySweep,
        z0: f64,
        ports: Vec<SpPort>,
    },
    /// Envelope transient analysis
    Envelope {
        fundamental_freq: f64,
        stop_time: f64,
        num_harmonics: usize,
        max_step: Option<f64>,
    },
    /// Fourier analysis
    Fourier {
        fundamental_freq: f64,
        num_harmonics: usize,
        output_node: String,
        output_ref: String,
        start_time: f64,
        stop_time: f64,
    },
}
