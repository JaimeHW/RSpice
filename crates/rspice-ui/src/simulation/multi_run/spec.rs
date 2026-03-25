use super::{AnalysisRunType, FrequencySweep};
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

impl AnalysisSpec {
    /// Get the corresponding run type.
    pub fn run_type(&self) -> AnalysisRunType {
        match self {
            AnalysisSpec::DcOp => AnalysisRunType::DcOp,
            AnalysisSpec::DcSweep { .. } => AnalysisRunType::DcSweep,
            AnalysisSpec::Ac { .. } => AnalysisRunType::Ac,
            AnalysisSpec::Disto { .. } => AnalysisRunType::Disto,
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
            AnalysisSpec::Disto {
                start_freq,
                stop_freq,
                points_per_unit,
                f2_over_f1,
                ..
            } => {
                if *start_freq <= 0.0 {
                    return Err("DISTO start_freq must be > 0".to_string());
                }
                if *stop_freq <= 0.0 {
                    return Err("DISTO stop_freq must be > 0".to_string());
                }
                if *stop_freq <= *start_freq {
                    return Err("DISTO stop_freq must be > start_freq".to_string());
                }
                if *points_per_unit == 0 {
                    return Err("DISTO points_per_unit must be > 0".to_string());
                }
                if let Some(ratio) = f2_over_f1
                    && (!ratio.is_finite() || *ratio <= 1.0) {
                        return Err("DISTO f2_over_f1 must be finite and > 1".to_string());
                    }
                Ok(())
            }
            AnalysisSpec::Transient {
                stop_time,
                step_time,
                start_time,
                max_timestep,
                ..
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
                if !start_time.is_finite() || *start_time < 0.0 {
                    return Err("Transient start_time must be finite and >= 0".to_string());
                }
                if *start_time >= *stop_time {
                    return Err("Transient start_time must be < stop_time".to_string());
                }
                if let Some(max_step) = max_timestep
                    && (!max_step.is_finite() || *max_step <= 0.0) {
                        return Err(
                            "Transient max_timestep must be finite and > 0 when set".to_string()
                        );
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
                tones,
                reltol,
                abstol,
                max_iterations,
                damping,
                oversample,
                max_mixing_order,
                gmres_restart,
                ..
            } => {
                if tones.is_empty() {
                    return Err("HB must define at least one tone".to_string());
                }
                for (idx, tone) in tones.iter().enumerate() {
                    if !tone.frequency.is_finite() || tone.frequency <= 0.0 {
                        return Err(format!("HB tone {} frequency must be > 0", idx + 1));
                    }
                    if tone.harmonics == 0 {
                        return Err(format!("HB tone {} harmonics must be > 0", idx + 1));
                    }
                }
                if *reltol <= 0.0 {
                    return Err("HB reltol must be > 0".to_string());
                }
                if *abstol <= 0.0 {
                    return Err("HB abstol must be > 0".to_string());
                }
                if *max_iterations == 0 {
                    return Err("HB max_iterations must be > 0".to_string());
                }
                if !damping.is_finite() || *damping <= 0.0 || *damping > 1.0 {
                    return Err("HB damping must be in (0, 1]".to_string());
                }
                if *oversample == 0 {
                    return Err("HB oversample must be > 0".to_string());
                }
                if *max_mixing_order == 0 {
                    return Err("HB max_mixing_order must be > 0".to_string());
                }
                if *gmres_restart == 0 {
                    return Err("HB gmres_restart must be > 0".to_string());
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
                    if let Some(freq) = frequency
                        && *freq <= 0.0 {
                            return Err("Sensitivity frequency must be > 0 for AC mode".to_string());
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
                    if let Some(port_z0) = port.z0
                        && (!port_z0.is_finite() || port_z0 <= 0.0) {
                            return Err(format!("S-parameter port{} z0 must be > 0", idx + 1));
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
                if let Some(step) = max_step
                    && *step <= 0.0 {
                        return Err("Envelope max_step must be > 0 when set".to_string());
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
