//! Multi-Run Orchestration
//!
//! Analysis sequence queuing and automated simulation workflow management.
//!
//! # Features
//!
//! - Queue multiple analyses for sequential execution
//! - Dependency-aware ordering (e.g., DC OP before AC)
//! - Progress tracking with cancellation support
//! - Result aggregation across runs
//! - Corner sweep automation

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

// =============================================================================
// Analysis Run Types
// =============================================================================

/// Type of analysis run
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnalysisRunType {
    /// DC operating point
    DcOp,
    /// DC sweep
    DcSweep,
    /// AC analysis
    Ac,
    /// Transient analysis
    Transient,
    /// Noise analysis
    Noise,
    /// Transfer function
    Tf,
    /// Sensitivity analysis
    Sensitivity,
    /// Pole-zero analysis
    PoleZero,
    /// Harmonic balance
    HarmonicBalance,
    /// Periodic steady-state
    Pss,
    /// Periodic AC
    Pac,
    /// Periodic noise
    Pnoise,
    /// Periodic transfer function
    Pxf,
    /// Periodic stability
    Pstb,
    /// Loop stability
    Stb,
    /// Monte Carlo
    MonteCarlo,
    /// Parametric
    Parametric,
    /// Corner analysis
    Corner,
    /// Reliability aging analysis
    Reliability,
    /// Optimization analysis
    Optimization,
    /// Safety/SOA analysis
    Soa,
    /// S-parameter analysis
    SParameter,
    /// Envelope transient analysis
    Envelope,
    /// Fourier analysis
    Fourier,
}

impl AnalysisRunType {
    /// Display name
    pub fn display_name(&self) -> &'static str {
        match self {
            AnalysisRunType::DcOp => "DC Operating Point",
            AnalysisRunType::DcSweep => "DC Sweep",
            AnalysisRunType::Ac => "AC Analysis",
            AnalysisRunType::Transient => "Transient",
            AnalysisRunType::Noise => "Noise",
            AnalysisRunType::Tf => "Transfer Function",
            AnalysisRunType::Sensitivity => "Sensitivity",
            AnalysisRunType::PoleZero => "Pole-Zero",
            AnalysisRunType::HarmonicBalance => "Harmonic Balance",
            AnalysisRunType::Pss => "PSS",
            AnalysisRunType::Pac => "PAC",
            AnalysisRunType::Pnoise => "PNoise",
            AnalysisRunType::Pxf => "PXF",
            AnalysisRunType::Pstb => "PSTB",
            AnalysisRunType::Stb => "STB",
            AnalysisRunType::MonteCarlo => "Monte Carlo",
            AnalysisRunType::Parametric => "Parametric",
            AnalysisRunType::Corner => "Corner",
            AnalysisRunType::Reliability => "Reliability",
            AnalysisRunType::Optimization => "Optimization",
            AnalysisRunType::Soa => "Safety (SOA)",
            AnalysisRunType::SParameter => "S-Parameter",
            AnalysisRunType::Envelope => "Envelope",
            AnalysisRunType::Fourier => "Fourier",
        }
    }

    /// Whether this analysis requires a prior DC OP
    pub fn requires_dc_op(&self) -> bool {
        matches!(
            self,
            AnalysisRunType::Ac
                | AnalysisRunType::Noise
                | AnalysisRunType::Tf
                | AnalysisRunType::Sensitivity
                | AnalysisRunType::PoleZero
                | AnalysisRunType::Stb
                | AnalysisRunType::Reliability
        )
    }

    /// Whether this analysis requires PSS first
    pub fn requires_pss(&self) -> bool {
        matches!(
            self,
            AnalysisRunType::Pac
                | AnalysisRunType::Pnoise
                | AnalysisRunType::Pxf
                | AnalysisRunType::Pstb
        )
    }

    /// Estimated relative complexity (for progress estimation)
    pub fn complexity(&self) -> u32 {
        match self {
            AnalysisRunType::DcOp => 1,
            AnalysisRunType::Tf => 1,
            AnalysisRunType::DcSweep => 5,
            AnalysisRunType::Ac => 3,
            AnalysisRunType::Transient => 10,
            AnalysisRunType::Noise => 5,
            AnalysisRunType::Sensitivity => 3,
            AnalysisRunType::PoleZero => 3,
            AnalysisRunType::HarmonicBalance => 15,
            AnalysisRunType::Pss => 20,
            AnalysisRunType::Pac => 5,
            AnalysisRunType::Pnoise => 10,
            AnalysisRunType::Pxf => 8,
            AnalysisRunType::Pstb => 9,
            AnalysisRunType::Stb => 4,
            AnalysisRunType::MonteCarlo => 50,
            AnalysisRunType::Parametric => 30,
            AnalysisRunType::Corner => 25,
            AnalysisRunType::Reliability => 12,
            AnalysisRunType::Optimization => 45,
            AnalysisRunType::Soa => 14,
            AnalysisRunType::SParameter => 6,
            AnalysisRunType::Envelope => 12,
            AnalysisRunType::Fourier => 7,
        }
    }
}

// =============================================================================
// Analysis Specification
// =============================================================================

/// Frequency sweep mode used by AC/noise analyses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum FrequencySweep {
    /// Decade (logarithmic)
    #[default]
    Decade,
    /// Octave (logarithmic)
    Octave,
    /// Linear
    Linear,
}

impl FrequencySweep {
    /// Keyword expected by the simulation runner.
    pub fn runner_keyword(self) -> &'static str {
        match self {
            FrequencySweep::Decade => "dec",
            FrequencySweep::Octave => "oct",
            FrequencySweep::Linear => "lin",
        }
    }
}

/// Optimization goal strategy.
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpPort {
    /// Positive node.
    pub node_pos: String,
    /// Negative/reference node.
    pub node_neg: String,
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
    /// Transient analysis
    Transient { stop_time: f64, step_time: f64 },
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
        tone1_freq: f64,
        tone1_harmonics: usize,
        tone2_freq: Option<f64>,
        tone2_harmonics: usize,
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

impl AnalysisSpec {
    /// Get the corresponding run type.
    pub fn run_type(&self) -> AnalysisRunType {
        match self {
            AnalysisSpec::DcOp => AnalysisRunType::DcOp,
            AnalysisSpec::DcSweep { .. } => AnalysisRunType::DcSweep,
            AnalysisSpec::Ac { .. } => AnalysisRunType::Ac,
            AnalysisSpec::Transient { .. } => AnalysisRunType::Transient,
            AnalysisSpec::Noise { .. } => AnalysisRunType::Noise,
            AnalysisSpec::Pss { .. } => AnalysisRunType::Pss,
            AnalysisSpec::HarmonicBalance { .. } => AnalysisRunType::HarmonicBalance,
            AnalysisSpec::Tf => AnalysisRunType::Tf,
            AnalysisSpec::Sensitivity { .. } => AnalysisRunType::Sensitivity,
            AnalysisSpec::PoleZero { .. } => AnalysisRunType::PoleZero,
            AnalysisSpec::Pac => AnalysisRunType::Pac,
            AnalysisSpec::Pnoise => AnalysisRunType::Pnoise,
            AnalysisSpec::Pxf => AnalysisRunType::Pxf,
            AnalysisSpec::Pstb => AnalysisRunType::Pstb,
            AnalysisSpec::Stb { .. } => AnalysisRunType::Stb,
            AnalysisSpec::MonteCarlo => AnalysisRunType::MonteCarlo,
            AnalysisSpec::Parametric => AnalysisRunType::Parametric,
            AnalysisSpec::Corner => AnalysisRunType::Corner,
            AnalysisSpec::Reliability { .. } => AnalysisRunType::Reliability,
            AnalysisSpec::Optimization { .. } => AnalysisRunType::Optimization,
            AnalysisSpec::Soa { .. } => AnalysisRunType::Soa,
            AnalysisSpec::SParameter { .. } => AnalysisRunType::SParameter,
            AnalysisSpec::Envelope { .. } => AnalysisRunType::Envelope,
            AnalysisSpec::Fourier { .. } => AnalysisRunType::Fourier,
        }
    }

    /// Validate analysis parameters.
    pub fn validate(&self) -> Result<(), String> {
        match self {
            AnalysisSpec::DcOp => Ok(()),
            AnalysisSpec::DcSweep {
                source_name,
                start,
                stop,
                step,
                source2,
                start2,
                stop2,
                step2,
            } => {
                if source_name.trim().is_empty() {
                    return Err("DC sweep source_name is required".to_string());
                }
                if *step == 0.0 {
                    return Err("DC sweep step cannot be zero".to_string());
                }
                if (stop - start).signum() != step.signum() {
                    return Err("DC sweep step direction must match start/stop".to_string());
                }

                match (source2, start2, stop2, step2) {
                    (None, None, None, None) => {}
                    (Some(source2), Some(start2), Some(stop2), Some(step2)) => {
                        if source2.trim().is_empty() {
                            return Err("DC sweep secondary source2 is required".to_string());
                        }
                        if source2.eq_ignore_ascii_case(source_name) {
                            return Err("DC sweep secondary source2 must differ from source_name"
                                .to_string());
                        }
                        if *step2 == 0.0 {
                            return Err("DC sweep secondary step2 cannot be zero".to_string());
                        }
                        if (stop2 - start2).signum() != step2.signum() {
                            return Err(
                                "DC sweep secondary step direction must match start2/stop2"
                                    .to_string(),
                            );
                        }
                    }
                    _ => {
                        return Err(
                            "DC sweep secondary sweep requires source2/start2/stop2/step2"
                                .to_string(),
                        );
                    }
                }
                Ok(())
            }
            AnalysisSpec::Ac {
                start_freq,
                stop_freq,
                points_per_unit,
                ..
            } => {
                if *start_freq <= 0.0 {
                    return Err("AC start_freq must be > 0".to_string());
                }
                if *stop_freq <= 0.0 {
                    return Err("AC stop_freq must be > 0".to_string());
                }
                if *stop_freq <= *start_freq {
                    return Err("AC stop_freq must be > start_freq".to_string());
                }
                if *points_per_unit == 0 {
                    return Err("AC points_per_unit must be > 0".to_string());
                }
                Ok(())
            }
            AnalysisSpec::Transient {
                stop_time,
                step_time,
            } => {
                if *stop_time <= 0.0 {
                    return Err("Transient stop_time must be > 0".to_string());
                }
                if *step_time <= 0.0 {
                    return Err("Transient step_time must be > 0".to_string());
                }
                if *step_time > *stop_time {
                    return Err("Transient step_time must be <= stop_time".to_string());
                }
                Ok(())
            }
            AnalysisSpec::Noise {
                output_node,
                start_freq,
                stop_freq,
                points_per_decade,
                temperature,
            } => {
                if output_node.trim().is_empty() {
                    return Err("Noise output_node is required".to_string());
                }
                if *start_freq <= 0.0 {
                    return Err("Noise start_freq must be > 0".to_string());
                }
                if *stop_freq <= 0.0 {
                    return Err("Noise stop_freq must be > 0".to_string());
                }
                if *stop_freq <= *start_freq {
                    return Err("Noise stop_freq must be > start_freq".to_string());
                }
                if *points_per_decade == 0 {
                    return Err("Noise points_per_decade must be > 0".to_string());
                }
                if *temperature <= 0.0 {
                    return Err("Noise temperature must be > 0 K".to_string());
                }
                Ok(())
            }
            AnalysisSpec::Pss {
                fundamental_freq,
                num_harmonics,
                tolerance,
            } => {
                if *fundamental_freq <= 0.0 {
                    return Err("PSS fundamental_freq must be > 0".to_string());
                }
                if *num_harmonics == 0 {
                    return Err("PSS num_harmonics must be > 0".to_string());
                }
                if *tolerance <= 0.0 {
                    return Err("PSS tolerance must be > 0".to_string());
                }
                Ok(())
            }
            AnalysisSpec::HarmonicBalance {
                tone1_freq,
                tone1_harmonics,
                tone2_freq,
                tone2_harmonics,
            } => {
                if *tone1_freq <= 0.0 {
                    return Err("HB tone1_freq must be > 0".to_string());
                }
                if *tone1_harmonics == 0 {
                    return Err("HB tone1_harmonics must be > 0".to_string());
                }
                if tone2_freq.is_some() && *tone2_harmonics == 0 {
                    return Err("HB tone2_harmonics must be > 0 when tone2_freq is set".to_string());
                }
                Ok(())
            }
            AnalysisSpec::Sensitivity {
                output_var,
                ac_mode,
                frequency,
            } => {
                if output_var.trim().is_empty() {
                    return Err("Sensitivity output_var is required".to_string());
                }
                if *ac_mode {
                    if let Some(freq) = frequency {
                        if *freq <= 0.0 {
                            return Err("Sensitivity frequency must be > 0 for AC mode".to_string());
                        }
                    }
                } else if frequency.is_some() {
                    return Err("Sensitivity frequency is only valid in AC mode".to_string());
                }
                Ok(())
            }
            AnalysisSpec::PoleZero {
                input_node,
                input_ref,
                output_node,
                output_ref,
                transfer_type,
                analysis_type,
            } => {
                if input_node.trim().is_empty() {
                    return Err("Pole-zero input_node is required".to_string());
                }
                if input_ref.trim().is_empty() {
                    return Err("Pole-zero input_ref is required".to_string());
                }
                if output_node.trim().is_empty() {
                    return Err("Pole-zero output_node is required".to_string());
                }
                if output_ref.trim().is_empty() {
                    return Err("Pole-zero output_ref is required".to_string());
                }
                let transfer = transfer_type.trim().to_ascii_uppercase();
                if transfer != "VOL" && transfer != "CUR" {
                    return Err("Pole-zero transfer_type must be VOL or CUR".to_string());
                }
                let analysis = analysis_type.trim().to_ascii_uppercase();
                if analysis != "PZ" && analysis != "POL" && analysis != "ZER" {
                    return Err("Pole-zero analysis_type must be PZ, POL, or ZER".to_string());
                }
                Ok(())
            }
            AnalysisSpec::Stb {
                probe_node,
                start_freq,
                stop_freq,
                points_per_decade,
            } => {
                if probe_node.trim().is_empty() {
                    return Err("STB probe_node is required".to_string());
                }
                if *start_freq <= 0.0 {
                    return Err("STB start_freq must be > 0".to_string());
                }
                if *stop_freq <= 0.0 {
                    return Err("STB stop_freq must be > 0".to_string());
                }
                if *stop_freq <= *start_freq {
                    return Err("STB stop_freq must be > start_freq".to_string());
                }
                if *points_per_decade == 0 {
                    return Err("STB points_per_decade must be > 0".to_string());
                }
                Ok(())
            }
            AnalysisSpec::SParameter {
                start_freq,
                stop_freq,
                points_per_unit,
                z0,
                ports,
                ..
            } => {
                if *start_freq <= 0.0 {
                    return Err("S-parameter start_freq must be > 0".to_string());
                }
                if *stop_freq <= 0.0 {
                    return Err("S-parameter stop_freq must be > 0".to_string());
                }
                if *stop_freq <= *start_freq {
                    return Err("S-parameter stop_freq must be > start_freq".to_string());
                }
                if *points_per_unit == 0 {
                    return Err("S-parameter points_per_unit must be > 0".to_string());
                }
                if *z0 <= 0.0 {
                    return Err("S-parameter z0 must be > 0".to_string());
                }
                if ports.len() < 2 {
                    return Err("S-parameter requires at least two ports".to_string());
                }
                for (idx, port) in ports.iter().enumerate() {
                    if port.node_pos.trim().is_empty() {
                        return Err(format!(
                            "S-parameter port{} positive node is required",
                            idx + 1
                        ));
                    }
                    if port.node_neg.trim().is_empty() {
                        return Err(format!(
                            "S-parameter port{} negative node is required",
                            idx + 1
                        ));
                    }
                }
                Ok(())
            }
            AnalysisSpec::Envelope {
                fundamental_freq,
                stop_time,
                num_harmonics,
                max_step,
            } => {
                if *fundamental_freq <= 0.0 {
                    return Err("Envelope fundamental_freq must be > 0".to_string());
                }
                if *stop_time <= 0.0 {
                    return Err("Envelope stop_time must be > 0".to_string());
                }
                if *num_harmonics == 0 {
                    return Err("Envelope num_harmonics must be > 0".to_string());
                }
                if let Some(step) = max_step {
                    if *step <= 0.0 {
                        return Err("Envelope max_step must be > 0 when set".to_string());
                    }
                }
                Ok(())
            }
            AnalysisSpec::Fourier {
                fundamental_freq,
                num_harmonics,
                output_node,
                output_ref: _,
                start_time,
                stop_time,
            } => {
                if *fundamental_freq <= 0.0 {
                    return Err("Fourier fundamental_freq must be > 0".to_string());
                }
                if *num_harmonics == 0 {
                    return Err("Fourier num_harmonics must be > 0".to_string());
                }
                if output_node.trim().is_empty() {
                    return Err("Fourier output_node is required".to_string());
                }
                if *stop_time <= *start_time {
                    return Err("Fourier stop_time must be greater than start_time".to_string());
                }
                Ok(())
            }
            AnalysisSpec::Reliability {
                target_years,
                enable_hci,
                enable_nbti,
                enable_em,
                min_stress_voltage,
            } => {
                if target_years.is_empty() {
                    return Err("Reliability target_years must not be empty".to_string());
                }
                if target_years
                    .iter()
                    .any(|years| !years.is_finite() || *years <= 0.0)
                {
                    return Err("Reliability target_years must be finite and > 0".to_string());
                }
                if !enable_hci && !enable_nbti && !enable_em {
                    return Err("Reliability requires at least one enabled mechanism".to_string());
                }
                if !min_stress_voltage.is_finite() || *min_stress_voltage < 0.0 {
                    return Err(
                        "Reliability min_stress_voltage must be finite and >= 0".to_string()
                    );
                }
                Ok(())
            }
            AnalysisSpec::Optimization {
                variables,
                objective_node,
                objective_ref,
                goal,
                target,
                max_iterations,
                cost_tolerance,
                fd_step,
                initial_step,
                min_step,
                ..
            } => {
                if variables.is_empty() {
                    return Err("Optimization variables must not be empty".to_string());
                }
                if objective_node.trim().is_empty() {
                    return Err("Optimization objective_node is required".to_string());
                }
                if objective_ref.trim().is_empty() {
                    return Err("Optimization objective_ref is required".to_string());
                }
                if objective_node.eq_ignore_ascii_case(objective_ref) {
                    return Err(
                        "Optimization objective_node and objective_ref must differ".to_string()
                    );
                }
                if *max_iterations == 0 {
                    return Err("Optimization max_iterations must be > 0".to_string());
                }
                if !cost_tolerance.is_finite() || *cost_tolerance <= 0.0 {
                    return Err("Optimization cost_tolerance must be finite and > 0".to_string());
                }
                if !fd_step.is_finite() || *fd_step <= 0.0 {
                    return Err("Optimization fd_step must be finite and > 0".to_string());
                }
                if !initial_step.is_finite() || *initial_step <= 0.0 {
                    return Err("Optimization initial_step must be finite and > 0".to_string());
                }
                if !min_step.is_finite() || *min_step <= 0.0 {
                    return Err("Optimization min_step must be finite and > 0".to_string());
                }
                if min_step > initial_step {
                    return Err("Optimization min_step must be <= initial_step".to_string());
                }
                if *goal == OptimizationGoal::Target {
                    if target.is_none() || target.is_some_and(|v| !v.is_finite()) {
                        return Err(
                            "Optimization target goal requires a finite target value".to_string()
                        );
                    }
                } else if target.is_some_and(|v| !v.is_finite()) {
                    return Err("Optimization target must be finite when provided".to_string());
                }

                let mut seen = std::collections::HashSet::new();
                for var in variables {
                    if var.name.trim().is_empty() {
                        return Err("Optimization variable name must not be empty".to_string());
                    }
                    if !var.min.is_finite() || !var.max.is_finite() || !var.initial.is_finite() {
                        return Err(format!(
                            "Optimization variable '{}' bounds/initial must be finite",
                            var.name
                        ));
                    }
                    if var.max <= var.min {
                        return Err(format!(
                            "Optimization variable '{}' requires max > min",
                            var.name
                        ));
                    }
                    if var.initial < var.min || var.initial > var.max {
                        return Err(format!(
                            "Optimization variable '{}' initial must be within [{}, {}]",
                            var.name, var.min, var.max
                        ));
                    }
                    if !seen.insert(var.name.to_ascii_uppercase()) {
                        return Err(format!(
                            "Optimization variable '{}' is defined more than once",
                            var.name
                        ));
                    }
                }
                Ok(())
            }
            AnalysisSpec::Soa {
                stop_time,
                step_time,
                check_vgs_max,
                max_vgs,
                check_vds_max,
                max_vds,
                check_vbe_max,
                max_vbe,
                check_vce_max,
                max_vce,
            } => {
                if !stop_time.is_finite() || *stop_time <= 0.0 {
                    return Err("SOA stop_time must be finite and > 0".to_string());
                }
                if !step_time.is_finite() || *step_time <= 0.0 {
                    return Err("SOA step_time must be finite and > 0".to_string());
                }
                if step_time > stop_time {
                    return Err("SOA step_time must be <= stop_time".to_string());
                }
                if !check_vgs_max && !check_vds_max && !check_vbe_max && !check_vce_max {
                    return Err("SOA requires at least one enabled check".to_string());
                }
                if *check_vgs_max && (!max_vgs.is_finite() || *max_vgs <= 0.0) {
                    return Err("SOA max_vgs must be finite and > 0 when enabled".to_string());
                }
                if *check_vds_max && (!max_vds.is_finite() || *max_vds <= 0.0) {
                    return Err("SOA max_vds must be finite and > 0 when enabled".to_string());
                }
                if *check_vbe_max && (!max_vbe.is_finite() || *max_vbe <= 0.0) {
                    return Err("SOA max_vbe must be finite and > 0 when enabled".to_string());
                }
                if *check_vce_max && (!max_vce.is_finite() || *max_vce <= 0.0) {
                    return Err("SOA max_vce must be finite and > 0 when enabled".to_string());
                }
                Ok(())
            }
            AnalysisSpec::Tf
            | AnalysisSpec::Pac
            | AnalysisSpec::Pnoise
            | AnalysisSpec::Pxf
            | AnalysisSpec::Pstb
            | AnalysisSpec::MonteCarlo
            | AnalysisSpec::Parametric
            | AnalysisSpec::Corner => Ok(()),
        }
    }
}

/// Ordered analysis execution plan.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnalysisPlan {
    /// Ordered analyses.
    pub analyses: Vec<AnalysisSpec>,
    /// Stop queue execution on first failed run.
    pub stop_on_error: bool,
}

impl AnalysisPlan {
    /// Create empty plan.
    pub fn new() -> Self {
        Self {
            analyses: Vec::new(),
            stop_on_error: true,
        }
    }

    /// Append an analysis.
    pub fn add(mut self, analysis: AnalysisSpec) -> Self {
        self.analyses.push(analysis);
        self
    }

    /// Validate all analyses in order.
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if self.analyses.is_empty() {
            errors.push("Analysis plan is empty".to_string());
        }
        for (idx, analysis) in self.analyses.iter().enumerate() {
            if let Err(e) = analysis.validate() {
                errors.push(format!("Analysis #{}: {}", idx + 1, e));
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Build a run queue from this plan.
    pub fn into_queue(self) -> Result<RunQueue, Vec<String>> {
        self.validate()?;
        let mut queue = RunQueue::new();
        queue.stop_on_error = self.stop_on_error;
        for analysis in self.analyses {
            queue.add_analysis(analysis);
        }
        Ok(queue)
    }
}

/// Status of an analysis run
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum RunStatus {
    /// Waiting in queue
    #[default]
    Pending,
    /// Currently executing
    Running,
    /// Completed successfully
    Completed,
    /// Failed with error
    Failed,
    /// Cancelled by user
    Cancelled,
    /// Skipped due to dependency failure
    Skipped,
}

impl RunStatus {
    /// Whether this status indicates completion (success or failure)
    pub fn is_done(&self) -> bool {
        matches!(
            self,
            RunStatus::Completed | RunStatus::Failed | RunStatus::Cancelled | RunStatus::Skipped
        )
    }

    /// Whether this status indicates success
    pub fn is_success(&self) -> bool {
        *self == RunStatus::Completed
    }
}

// =============================================================================
// Analysis Run
// =============================================================================

/// A single analysis run in the queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisRun {
    /// Unique run ID
    pub id: u64,
    /// Human-readable name
    pub name: String,
    /// Analysis type
    pub run_type: AnalysisRunType,
    /// Typed analysis specification (required for parameterized analyses)
    pub spec: Option<AnalysisSpec>,
    /// Run status
    pub status: RunStatus,
    /// Progress percentage (0-100)
    pub progress: u8,
    /// Error message if failed
    pub error: Option<String>,
    /// Dependencies (run IDs that must complete first)
    pub dependencies: Vec<u64>,
    /// Corner name (if part of corner sweep)
    pub corner: Option<String>,
    /// Iteration number (if part of parametric/MC)
    pub iteration: Option<usize>,
    /// Start timestamp
    pub start_time: Option<u64>,
    /// End timestamp
    pub end_time: Option<u64>,
}

impl Default for AnalysisRun {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            run_type: AnalysisRunType::DcOp,
            spec: None,
            status: RunStatus::Pending,
            progress: 0,
            error: None,
            dependencies: Vec::new(),
            corner: None,
            iteration: None,
            start_time: None,
            end_time: None,
        }
    }
}

impl AnalysisRun {
    /// Create a new analysis run
    pub fn new(id: u64, run_type: AnalysisRunType) -> Self {
        Self {
            id,
            name: run_type.display_name().to_string(),
            run_type,
            ..Default::default()
        }
    }

    /// Attach typed analysis spec and keep run_type consistent.
    pub fn with_spec(mut self, spec: AnalysisSpec) -> Self {
        self.run_type = spec.run_type();
        self.spec = Some(spec);
        self
    }

    /// Set name
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Add dependency
    pub fn with_dependency(mut self, dep_id: u64) -> Self {
        self.dependencies.push(dep_id);
        self
    }

    /// Set corner
    pub fn with_corner(mut self, corner: impl Into<String>) -> Self {
        self.corner = Some(corner.into());
        self
    }

    /// Set iteration
    pub fn with_iteration(mut self, iter: usize) -> Self {
        self.iteration = Some(iter);
        self
    }

    /// Validate run metadata and analysis spec.
    pub fn validate(&self) -> Result<(), String> {
        match &self.spec {
            Some(spec) => {
                if spec.run_type() != self.run_type {
                    return Err(format!(
                        "Run '{}' has mismatched run_type ({:?}) vs spec ({:?})",
                        self.name,
                        self.run_type,
                        spec.run_type()
                    ));
                }
                spec.validate()
            }
            None => {
                if self.run_type == AnalysisRunType::DcOp {
                    Ok(())
                } else {
                    Err(format!(
                        "Run '{}' ({:?}) is missing AnalysisSpec",
                        self.name, self.run_type
                    ))
                }
            }
        }
    }

    /// Mark as started
    pub fn start(&mut self, timestamp: u64) {
        self.status = RunStatus::Running;
        self.start_time = Some(timestamp);
        self.progress = 0;
    }

    /// Update progress
    pub fn update_progress(&mut self, progress: u8) {
        self.progress = progress.min(100);
    }

    /// Mark as completed
    pub fn complete(&mut self, timestamp: u64) {
        self.status = RunStatus::Completed;
        self.end_time = Some(timestamp);
        self.progress = 100;
    }

    /// Mark as failed
    pub fn fail(&mut self, error: impl Into<String>, timestamp: u64) {
        self.status = RunStatus::Failed;
        self.error = Some(error.into());
        self.end_time = Some(timestamp);
    }

    /// Mark as cancelled
    pub fn cancel(&mut self, timestamp: u64) {
        self.status = RunStatus::Cancelled;
        self.end_time = Some(timestamp);
    }

    /// Mark as skipped
    pub fn skip(&mut self) {
        self.status = RunStatus::Skipped;
    }

    /// Get elapsed time in seconds
    pub fn elapsed(&self) -> Option<u64> {
        match (self.start_time, self.end_time) {
            (Some(start), Some(end)) => Some(end - start),
            _ => None,
        }
    }

    /// Check if all dependencies are completed
    pub fn dependencies_met(&self, completed_ids: &[u64]) -> bool {
        self.dependencies.iter().all(|d| completed_ids.contains(d))
    }
}

// =============================================================================
// Run Queue
// =============================================================================

/// Queue of analysis runs with orchestration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RunQueue {
    /// All runs in the queue
    runs: Vec<AnalysisRun>,
    /// Next run ID
    next_id: u64,
    /// Currently running run ID
    pub current_run: Option<u64>,
    /// Whether queue is paused
    pub paused: bool,
    /// Whether to stop on first error
    pub stop_on_error: bool,
    /// Total estimated complexity
    total_complexity: u32,
    /// Completed complexity
    completed_complexity: u32,
    /// Netlist source for simulation (required for execution)
    netlist: Option<String>,
}

impl RunQueue {
    /// Create a new queue
    pub fn new() -> Self {
        Self {
            stop_on_error: true,
            ..Default::default()
        }
    }

    /// Set netlist source for simulation
    pub fn with_netlist(mut self, netlist: impl Into<String>) -> Self {
        self.netlist = Some(netlist.into());
        self
    }

    /// Set netlist source (mutable)
    pub fn set_netlist(&mut self, netlist: impl Into<String>) {
        self.netlist = Some(netlist.into());
    }

    /// Get netlist source
    pub fn netlist(&self) -> Option<&str> {
        self.netlist.as_deref()
    }

    /// Add a run to the queue
    pub fn add(&mut self, run_type: AnalysisRunType) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let run = AnalysisRun::new(id, run_type);
        self.total_complexity += run_type.complexity();
        self.runs.push(run);
        id
    }

    /// Add a run using an explicit typed analysis spec.
    pub fn add_analysis(&mut self, spec: AnalysisSpec) -> u64 {
        let run_type = spec.run_type();
        let id = self.next_id;
        self.next_id += 1;

        let run = AnalysisRun::new(id, run_type).with_spec(spec);
        self.total_complexity += run_type.complexity();
        self.runs.push(run);
        id
    }

    /// Set or replace the spec for an existing run.
    pub fn set_spec(&mut self, run_id: u64, spec: AnalysisSpec) -> Result<(), String> {
        let run = self
            .get_mut(run_id)
            .ok_or_else(|| format!("Run {} not found", run_id))?;
        run.run_type = spec.run_type();
        run.spec = Some(spec);
        Ok(())
    }

    /// Add a run with automatic dependency resolution
    pub fn add_with_deps(&mut self, run_type: AnalysisRunType) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let mut run = AnalysisRun::new(id, run_type);

        // Auto-add DC OP dependency if needed
        if run_type.requires_dc_op() {
            if let Some(dc_op) = self.find_run_by_type(AnalysisRunType::DcOp) {
                run.dependencies.push(dc_op);
            } else {
                // Auto-insert DC OP
                let dc_id = self.add(AnalysisRunType::DcOp);
                run.dependencies.push(dc_id);
            }
        }

        // Auto-add PSS dependency if needed
        if run_type.requires_pss() {
            if let Some(pss) = self.find_run_by_type(AnalysisRunType::Pss) {
                run.dependencies.push(pss);
            } else {
                // Auto-insert PSS with DC OP
                let pss_id = self.add_with_deps(AnalysisRunType::Pss);
                run.dependencies.push(pss_id);
            }
        }

        self.total_complexity += run_type.complexity();
        self.runs.push(run);
        id
    }

    /// Find a run by type
    fn find_run_by_type(&self, run_type: AnalysisRunType) -> Option<u64> {
        self.runs
            .iter()
            .find(|r| r.run_type == run_type)
            .map(|r| r.id)
    }

    /// Get a run by ID
    pub fn get(&self, id: u64) -> Option<&AnalysisRun> {
        self.runs.iter().find(|r| r.id == id)
    }

    /// Get mutable run by ID
    pub fn get_mut(&mut self, id: u64) -> Option<&mut AnalysisRun> {
        self.runs.iter_mut().find(|r| r.id == id)
    }

    /// Get the next runnable analysis
    pub fn next_runnable(&self) -> Option<u64> {
        if self.paused {
            return None;
        }

        let completed: Vec<u64> = self
            .runs
            .iter()
            .filter(|r| r.status.is_success())
            .map(|r| r.id)
            .collect();

        self.runs
            .iter()
            .find(|r| r.status == RunStatus::Pending && r.dependencies_met(&completed))
            .map(|r| r.id)
    }

    /// Start the next runnable analysis
    pub fn start_next(&mut self, timestamp: u64) -> Option<u64> {
        if let Some(id) = self.next_runnable() {
            if let Some(run) = self.get_mut(id) {
                run.start(timestamp);
                self.current_run = Some(id);
                return Some(id);
            }
        }
        None
    }

    /// Complete the current run
    pub fn complete_current(&mut self, timestamp: u64) {
        if let Some(id) = self.current_run {
            if let Some(run) = self.get_mut(id) {
                let complexity = run.run_type.complexity();
                run.complete(timestamp);
                self.completed_complexity += complexity;
            }
            self.current_run = None;
        }
    }

    /// Fail the current run
    pub fn fail_current(&mut self, error: impl Into<String>, timestamp: u64) {
        if let Some(id) = self.current_run {
            if let Some(run) = self.get_mut(id) {
                run.fail(error, timestamp);
            }
            self.current_run = None;

            // Skip dependent runs if stop_on_error
            if self.stop_on_error {
                self.skip_dependents(id);
            }
        }
    }

    /// Skip all runs that depend on the given ID
    fn skip_dependents(&mut self, failed_id: u64) {
        let mut to_skip: VecDeque<u64> = VecDeque::new();
        to_skip.push_back(failed_id);

        while let Some(id) = to_skip.pop_front() {
            for run in &mut self.runs {
                if run.dependencies.contains(&id) && run.status == RunStatus::Pending {
                    run.skip();
                    to_skip.push_back(run.id);
                }
            }
        }
    }

    /// Cancel all pending runs
    pub fn cancel_all(&mut self, timestamp: u64) {
        if let Some(id) = self.current_run {
            if let Some(run) = self.get_mut(id) {
                run.cancel(timestamp);
            }
        }
        self.current_run = None;

        for run in &mut self.runs {
            if run.status == RunStatus::Pending {
                run.status = RunStatus::Cancelled;
            }
        }
    }

    /// Pause the queue
    pub fn pause(&mut self) {
        self.paused = true;
    }

    /// Resume the queue
    pub fn resume(&mut self) {
        self.paused = false;
    }

    /// Get overall progress (0-100)
    pub fn overall_progress(&self) -> u8 {
        if self.total_complexity == 0 {
            return 0;
        }

        let current_progress = self
            .current_run
            .and_then(|id| self.get(id))
            .map(|r| (r.progress as u32 * r.run_type.complexity()) / 100)
            .unwrap_or(0);

        let total = self.completed_complexity + current_progress;
        ((total * 100) / self.total_complexity) as u8
    }

    /// Count runs by status
    pub fn count_by_status(&self, status: RunStatus) -> usize {
        self.runs.iter().filter(|r| r.status == status).count()
    }

    /// Get all runs
    pub fn runs(&self) -> &[AnalysisRun] {
        &self.runs
    }

    /// Get pending runs
    pub fn pending(&self) -> Vec<&AnalysisRun> {
        self.runs
            .iter()
            .filter(|r| r.status == RunStatus::Pending)
            .collect()
    }

    /// Get completed runs
    pub fn completed(&self) -> Vec<&AnalysisRun> {
        self.runs
            .iter()
            .filter(|r| r.status == RunStatus::Completed)
            .collect()
    }

    /// Get failed runs
    pub fn failed(&self) -> Vec<&AnalysisRun> {
        self.runs
            .iter()
            .filter(|r| r.status == RunStatus::Failed)
            .collect()
    }

    /// Is queue empty?
    pub fn is_empty(&self) -> bool {
        self.runs.is_empty()
    }

    /// Total run count
    pub fn len(&self) -> usize {
        self.runs.len()
    }

    /// Is queue done (all runs finished)?
    pub fn is_done(&self) -> bool {
        self.runs.iter().all(|r| r.status.is_done())
    }

    /// Clear the queue
    pub fn clear(&mut self) {
        self.runs.clear();
        self.current_run = None;
        self.total_complexity = 0;
        self.completed_complexity = 0;
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // AnalysisRunType Tests
    // =========================================================================

    #[test]
    fn test_run_type_display() {
        assert_eq!(AnalysisRunType::DcOp.display_name(), "DC Operating Point");
        assert_eq!(AnalysisRunType::Transient.display_name(), "Transient");
        assert_eq!(AnalysisRunType::Stb.display_name(), "STB");
        assert_eq!(AnalysisRunType::Pxf.display_name(), "PXF");
        assert_eq!(AnalysisRunType::Pstb.display_name(), "PSTB");
        assert_eq!(AnalysisRunType::Reliability.display_name(), "Reliability");
        assert_eq!(AnalysisRunType::Optimization.display_name(), "Optimization");
        assert_eq!(AnalysisRunType::Soa.display_name(), "Safety (SOA)");
    }

    #[test]
    fn test_run_type_requires_dc_op() {
        assert!(AnalysisRunType::Ac.requires_dc_op());
        assert!(AnalysisRunType::Noise.requires_dc_op());
        assert!(AnalysisRunType::Stb.requires_dc_op());
        assert!(AnalysisRunType::Reliability.requires_dc_op());
        assert!(!AnalysisRunType::Optimization.requires_dc_op());
        assert!(!AnalysisRunType::Soa.requires_dc_op());
        assert!(!AnalysisRunType::Transient.requires_dc_op());
        assert!(!AnalysisRunType::DcOp.requires_dc_op());
    }

    #[test]
    fn test_analysis_spec_stb_validation() {
        let valid = AnalysisSpec::Stb {
            probe_node: "LSTB".to_string(),
            start_freq: 1.0,
            stop_freq: 1e9,
            points_per_decade: 10,
        };
        assert!(valid.validate().is_ok());

        let invalid = AnalysisSpec::Stb {
            probe_node: "".to_string(),
            start_freq: 0.0,
            stop_freq: 1.0,
            points_per_decade: 0,
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_analysis_spec_pstb_validation() {
        let spec = AnalysisSpec::Pstb;
        assert!(spec.validate().is_ok());
        assert_eq!(spec.run_type(), AnalysisRunType::Pstb);
    }

    #[test]
    fn test_analysis_spec_reliability_validation() {
        let valid = AnalysisSpec::Reliability {
            target_years: vec![1.0, 5.0, 10.0],
            enable_hci: true,
            enable_nbti: false,
            enable_em: true,
            min_stress_voltage: 0.05,
        };
        assert!(valid.validate().is_ok());
        assert_eq!(valid.run_type(), AnalysisRunType::Reliability);

        let invalid = AnalysisSpec::Reliability {
            target_years: vec![0.0],
            enable_hci: false,
            enable_nbti: false,
            enable_em: false,
            min_stress_voltage: -1.0,
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_analysis_spec_optimization_validation() {
        let valid = AnalysisSpec::Optimization {
            variables: vec![OptimizationVariable {
                name: "RLOAD".to_string(),
                min: 1e3,
                max: 10e3,
                initial: 2e3,
            }],
            objective_node: "out".to_string(),
            objective_ref: "0".to_string(),
            goal: OptimizationGoal::Target,
            target: Some(1.2),
            algorithm: OptimizationAlgorithm::PatternSearch,
            max_iterations: 64,
            cost_tolerance: 1e-8,
            fd_step: 1e-4,
            initial_step: 0.1,
            min_step: 1e-8,
        };
        assert!(valid.validate().is_ok());
        assert_eq!(valid.run_type(), AnalysisRunType::Optimization);

        let invalid = AnalysisSpec::Optimization {
            variables: vec![OptimizationVariable {
                name: "RLOAD".to_string(),
                min: 10e3,
                max: 1e3,
                initial: 2e3,
            }],
            objective_node: "out".to_string(),
            objective_ref: "out".to_string(),
            goal: OptimizationGoal::Target,
            target: None,
            algorithm: OptimizationAlgorithm::PatternSearch,
            max_iterations: 0,
            cost_tolerance: -1.0,
            fd_step: 0.0,
            initial_step: 0.0,
            min_step: 1.0,
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_analysis_spec_soa_validation() {
        let valid = AnalysisSpec::Soa {
            stop_time: 1e-6,
            step_time: 1e-9,
            check_vgs_max: true,
            max_vgs: 1.8,
            check_vds_max: true,
            max_vds: 3.3,
            check_vbe_max: false,
            max_vbe: 0.9,
            check_vce_max: false,
            max_vce: 5.0,
        };
        assert!(valid.validate().is_ok());
        assert_eq!(valid.run_type(), AnalysisRunType::Soa);

        let invalid = AnalysisSpec::Soa {
            stop_time: 1e-9,
            step_time: 1e-6,
            check_vgs_max: false,
            max_vgs: 0.0,
            check_vds_max: false,
            max_vds: 0.0,
            check_vbe_max: false,
            max_vbe: 0.0,
            check_vce_max: false,
            max_vce: 0.0,
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_analysis_spec_sparameter_validation_supports_multiport() {
        let valid = AnalysisSpec::SParameter {
            start_freq: 1e3,
            stop_freq: 1e9,
            points_per_unit: 10,
            sweep: FrequencySweep::Decade,
            z0: 50.0,
            ports: vec![
                SpPort {
                    node_pos: "in".to_string(),
                    node_neg: "0".to_string(),
                },
                SpPort {
                    node_pos: "out".to_string(),
                    node_neg: "0".to_string(),
                },
                SpPort {
                    node_pos: "aux".to_string(),
                    node_neg: "0".to_string(),
                },
            ],
        };
        assert!(valid.validate().is_ok());
        assert_eq!(valid.run_type(), AnalysisRunType::SParameter);
    }

    #[test]
    fn test_analysis_spec_sparameter_validation_rejects_missing_ports() {
        let invalid = AnalysisSpec::SParameter {
            start_freq: 1e3,
            stop_freq: 1e9,
            points_per_unit: 10,
            sweep: FrequencySweep::Decade,
            z0: 50.0,
            ports: vec![SpPort {
                node_pos: "in".to_string(),
                node_neg: "0".to_string(),
            }],
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_run_type_requires_pss() {
        assert!(AnalysisRunType::Pac.requires_pss());
        assert!(AnalysisRunType::Pnoise.requires_pss());
        assert!(AnalysisRunType::Pxf.requires_pss());
        assert!(AnalysisRunType::Pstb.requires_pss());
        assert!(!AnalysisRunType::Ac.requires_pss());
    }

    #[test]
    fn test_analysis_spec_sensitivity_validation() {
        let valid = AnalysisSpec::Sensitivity {
            output_var: "V(out)".to_string(),
            ac_mode: true,
            frequency: Some(1e6),
        };
        assert!(valid.validate().is_ok());

        let invalid = AnalysisSpec::Sensitivity {
            output_var: "V(out)".to_string(),
            ac_mode: false,
            frequency: Some(1e6),
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_analysis_spec_pole_zero_validation() {
        let valid = AnalysisSpec::PoleZero {
            input_node: "in".to_string(),
            input_ref: "0".to_string(),
            output_node: "out".to_string(),
            output_ref: "0".to_string(),
            transfer_type: "VOL".to_string(),
            analysis_type: "PZ".to_string(),
        };
        assert!(valid.validate().is_ok());

        let invalid_transfer = AnalysisSpec::PoleZero {
            input_node: "in".to_string(),
            input_ref: "0".to_string(),
            output_node: "out".to_string(),
            output_ref: "0".to_string(),
            transfer_type: "BAD".to_string(),
            analysis_type: "PZ".to_string(),
        };
        assert!(invalid_transfer.validate().is_err());

        let invalid_type = AnalysisSpec::PoleZero {
            input_node: "in".to_string(),
            input_ref: "0".to_string(),
            output_node: "out".to_string(),
            output_ref: "0".to_string(),
            transfer_type: "VOL".to_string(),
            analysis_type: "UNKNOWN".to_string(),
        };
        assert!(invalid_type.validate().is_err());
    }

    #[test]
    fn test_analysis_spec_dc_sweep_nested_validation() {
        let valid = AnalysisSpec::DcSweep {
            source_name: "V1".to_string(),
            start: 0.0,
            stop: 1.0,
            step: 0.1,
            source2: Some("V2".to_string()),
            start2: Some(0.0),
            stop2: Some(2.0),
            step2: Some(0.5),
        };
        assert!(valid.validate().is_ok());
    }

    #[test]
    fn test_analysis_spec_dc_sweep_nested_requires_complete_secondary_fields() {
        let invalid = AnalysisSpec::DcSweep {
            source_name: "V1".to_string(),
            start: 0.0,
            stop: 1.0,
            step: 0.1,
            source2: Some("V2".to_string()),
            start2: Some(0.0),
            stop2: Some(2.0),
            step2: None,
        };
        assert!(invalid.validate().is_err());
    }

    // =========================================================================
    // AnalysisRun Tests
    // =========================================================================

    #[test]
    fn test_run_creation() {
        let run = AnalysisRun::new(1, AnalysisRunType::Ac);
        assert_eq!(run.id, 1);
        assert_eq!(run.run_type, AnalysisRunType::Ac);
        assert_eq!(run.status, RunStatus::Pending);
    }

    #[test]
    fn test_run_with_spec_sets_type() {
        let run = AnalysisRun::new(1, AnalysisRunType::DcOp).with_spec(AnalysisSpec::Ac {
            start_freq: 1.0,
            stop_freq: 1e6,
            points_per_unit: 20,
            sweep: FrequencySweep::Decade,
        });
        assert_eq!(run.run_type, AnalysisRunType::Ac);
        assert!(run.spec.is_some());
    }

    #[test]
    fn test_run_validate_requires_spec_for_parameterized_run() {
        let run = AnalysisRun::new(1, AnalysisRunType::Ac);
        assert!(run.validate().is_err());
    }

    #[test]
    fn test_run_lifecycle() {
        let mut run = AnalysisRun::new(1, AnalysisRunType::DcOp);

        run.start(1000);
        assert_eq!(run.status, RunStatus::Running);
        assert_eq!(run.start_time, Some(1000));

        run.update_progress(50);
        assert_eq!(run.progress, 50);

        run.complete(1010);
        assert_eq!(run.status, RunStatus::Completed);
        assert_eq!(run.elapsed(), Some(10));
    }

    #[test]
    fn test_run_failure() {
        let mut run = AnalysisRun::new(1, AnalysisRunType::DcOp);
        run.start(1000);
        run.fail("Convergence failure", 1005);

        assert_eq!(run.status, RunStatus::Failed);
        assert_eq!(run.error, Some("Convergence failure".to_string()));
    }

    #[test]
    fn test_run_dependencies() {
        let run = AnalysisRun::new(3, AnalysisRunType::Ac)
            .with_dependency(1)
            .with_dependency(2);

        assert!(!run.dependencies_met(&[1]));
        assert!(run.dependencies_met(&[1, 2]));
        assert!(run.dependencies_met(&[1, 2, 3]));
    }

    // =========================================================================
    // RunQueue Tests
    // =========================================================================

    #[test]
    fn test_queue_creation() {
        let queue = RunQueue::new();
        assert!(queue.is_empty());
        assert!(queue.stop_on_error);
    }

    #[test]
    fn test_queue_add() {
        let mut queue = RunQueue::new();
        let id = queue.add(AnalysisRunType::DcOp);

        assert_eq!(queue.len(), 1);
        assert!(queue.get(id).is_some());
    }

    #[test]
    fn test_queue_add_analysis_spec() {
        let mut queue = RunQueue::new();
        let id = queue.add_analysis(AnalysisSpec::DcSweep {
            source_name: "V1".to_string(),
            start: 0.0,
            stop: 1.0,
            step: 0.1,
            source2: None,
            start2: None,
            stop2: None,
            step2: None,
        });

        let run = queue.get(id).expect("run should exist");
        assert_eq!(run.run_type, AnalysisRunType::DcSweep);
        assert!(run.validate().is_ok());
    }

    #[test]
    fn test_analysis_plan_into_queue() {
        let plan = AnalysisPlan::new()
            .add(AnalysisSpec::DcOp)
            .add(AnalysisSpec::Ac {
                start_freq: 1.0,
                stop_freq: 1e6,
                points_per_unit: 10,
                sweep: FrequencySweep::Decade,
            });

        let queue = plan.into_queue().expect("valid plan should produce queue");
        assert_eq!(queue.len(), 2);
        assert_eq!(queue.runs()[0].run_type, AnalysisRunType::DcOp);
        assert_eq!(queue.runs()[1].run_type, AnalysisRunType::Ac);
    }

    #[test]
    fn test_queue_auto_deps() {
        let mut queue = RunQueue::new();
        let ac_id = queue.add_with_deps(AnalysisRunType::Ac);

        // Should auto-add DC OP
        assert_eq!(queue.len(), 2);

        let ac_run = queue.get(ac_id).unwrap();
        assert!(!ac_run.dependencies.is_empty());
    }

    #[test]
    fn test_queue_execution() {
        let mut queue = RunQueue::new();
        queue.add(AnalysisRunType::DcOp);
        queue.add(AnalysisRunType::Transient);

        // Start first
        let id = queue.start_next(1000).unwrap();
        assert_eq!(queue.current_run, Some(id));

        // Complete it
        queue.complete_current(1010);
        assert!(queue.current_run.is_none());
        assert_eq!(queue.count_by_status(RunStatus::Completed), 1);

        // Start second
        let id2 = queue.start_next(1010).unwrap();
        assert_ne!(id, id2);
    }

    #[test]
    fn test_queue_dependency_ordering() {
        let mut queue = RunQueue::new();
        let dc_id = queue.add(AnalysisRunType::DcOp);
        let ac_id = queue.add(AnalysisRunType::Ac);

        // Add dependency manually
        if let Some(ac) = queue.get_mut(ac_id) {
            ac.dependencies.push(dc_id);
        }

        // Only DC should be runnable
        assert_eq!(queue.next_runnable(), Some(dc_id));

        // Complete DC
        queue.start_next(1000);
        queue.complete_current(1010);

        // Now AC is runnable
        assert_eq!(queue.next_runnable(), Some(ac_id));
    }

    #[test]
    fn test_queue_failure_skip() {
        let mut queue = RunQueue::new();
        let id1 = queue.add(AnalysisRunType::DcOp);
        let id2 = queue.add(AnalysisRunType::Ac);

        // Make AC depend on DC
        if let Some(ac) = queue.get_mut(id2) {
            ac.dependencies.push(id1);
        }

        // Start and fail DC
        queue.start_next(1000);
        queue.fail_current("Error", 1005);

        // AC should be skipped
        assert_eq!(queue.get(id2).unwrap().status, RunStatus::Skipped);
    }

    #[test]
    fn test_queue_progress() {
        let mut queue = RunQueue::new();
        queue.add(AnalysisRunType::DcOp);
        queue.add(AnalysisRunType::DcOp);

        assert_eq!(queue.overall_progress(), 0);

        queue.start_next(1000);
        queue.complete_current(1010);

        assert_eq!(queue.overall_progress(), 50);

        queue.start_next(1010);
        queue.complete_current(1020);

        assert_eq!(queue.overall_progress(), 100);
    }

    #[test]
    fn test_queue_pause_resume() {
        let mut queue = RunQueue::new();
        queue.add(AnalysisRunType::DcOp);

        queue.pause();
        assert!(queue.paused);
        assert!(queue.next_runnable().is_none());

        queue.resume();
        assert!(!queue.paused);
        assert!(queue.next_runnable().is_some());
    }

    #[test]
    fn test_queue_cancel_all() {
        let mut queue = RunQueue::new();
        queue.add(AnalysisRunType::DcOp);
        queue.add(AnalysisRunType::Ac);
        queue.start_next(1000);

        queue.cancel_all(1005);

        assert_eq!(queue.count_by_status(RunStatus::Cancelled), 2);
    }

    #[test]
    fn test_queue_is_done() {
        let mut queue = RunQueue::new();
        queue.add(AnalysisRunType::DcOp);

        assert!(!queue.is_done());

        queue.start_next(1000);
        queue.complete_current(1010);

        assert!(queue.is_done());
    }
}
