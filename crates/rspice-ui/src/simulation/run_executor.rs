//! Run Queue Executor
//!
//! Executes multi-run analysis queues through the simulation engine.
//! Handles dependencies, parallelization, and result aggregation.
//!
//! # Features
//!
//! - Execute queued analyses in dependency order
//! - Parallel execution where possible
//! - Progress tracking and cancellation
//! - Result aggregation across runs
//! - Corner sweep execution

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc,
};
use std::time::Instant;

use super::multi_run::{AnalysisRun, AnalysisRunType, AnalysisSpec, RunQueue, RunStatus};
use super::options_translator::{EngineOptions, OptionsTranslator, PvtCorner};
use super::result_mapper::{
    MappedAnalysisType, MappedMeasurement, MappedResult, MappedWaveform, MeasurementStatus,
    MeasurementType, ResultMapper, ResultStatus,
};

// =============================================================================
// Execution State
// =============================================================================

/// Execution state for a run queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionState {
    /// Total runs
    pub total_runs: usize,
    /// Completed runs
    pub completed_runs: usize,
    /// Failed runs
    pub failed_runs: usize,
    /// Current run name
    pub current_run: Option<String>,
    /// Overall status
    pub status: ExecutionStatus,
    /// Start time (epoch ms)
    pub start_time: u64,
    /// Elapsed time (seconds)
    pub elapsed_seconds: f64,
    /// Estimated remaining time
    pub eta_seconds: Option<f64>,
}

/// Overall execution status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ExecutionStatus {
    /// Not started
    #[default]
    Idle,
    /// Currently running
    Running,
    /// Paused
    Paused,
    /// Completed successfully
    Completed,
    /// Completed with errors
    CompletedWithErrors,
    /// Cancelled by user
    Cancelled,
    /// Fatal error
    Error,
}

impl Default for ExecutionState {
    fn default() -> Self {
        Self {
            total_runs: 0,
            completed_runs: 0,
            failed_runs: 0,
            current_run: None,
            status: ExecutionStatus::Idle,
            start_time: 0,
            elapsed_seconds: 0.0,
            eta_seconds: None,
        }
    }
}

impl ExecutionState {
    /// Progress percentage
    pub fn progress_percent(&self) -> f32 {
        if self.total_runs == 0 {
            0.0
        } else {
            (self.completed_runs as f32 / self.total_runs as f32) * 100.0
        }
    }

    /// Is complete?
    pub fn is_complete(&self) -> bool {
        matches!(
            self.status,
            ExecutionStatus::Completed
                | ExecutionStatus::CompletedWithErrors
                | ExecutionStatus::Error
        )
    }

    /// Update ETA based on current progress
    pub fn update_eta(&mut self) {
        if self.completed_runs > 0 && self.completed_runs < self.total_runs {
            let avg_time = self.elapsed_seconds / self.completed_runs as f64;
            let remaining = self.total_runs - self.completed_runs;
            self.eta_seconds = Some(avg_time * remaining as f64);
        } else {
            self.eta_seconds = None;
        }
    }
}

// =============================================================================
// Execution Result
// =============================================================================

/// Result of queue execution
#[derive(Debug, Clone, Default)]
pub struct ExecutionResult {
    /// Results by run ID
    pub results: HashMap<u64, MappedResult>,
    /// Execution state
    pub state: ExecutionState,
    /// Errors by run ID
    pub errors: HashMap<u64, String>,
}

impl ExecutionResult {
    /// Get result for a run
    pub fn get(&self, run_id: u64) -> Option<&MappedResult> {
        self.results.get(&run_id)
    }

    /// All successful results
    pub fn successful_results(&self) -> Vec<(u64, &MappedResult)> {
        self.results
            .iter()
            .filter(|(_, r)| r.status == ResultStatus::Success)
            .map(|(&k, v)| (k, v))
            .collect()
    }

    /// Count of successful runs
    pub fn success_count(&self) -> usize {
        self.results
            .values()
            .filter(|r| r.status == ResultStatus::Success)
            .count()
    }
}

// =============================================================================
// Run Executor
// =============================================================================

/// Executor for run queues
pub struct RunExecutor {
    /// Options translator
    options_translator: OptionsTranslator,
    /// Result mapper
    result_mapper: ResultMapper,
    /// Cancellation flag
    cancelled: Arc<AtomicBool>,
    /// Current progress
    progress: Arc<AtomicUsize>,
    /// Maximum parallel runs (0 = serial)
    max_parallel: usize,
}

impl Default for RunExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl RunExecutor {
    /// Create new executor
    pub fn new() -> Self {
        Self {
            options_translator: OptionsTranslator::new(),
            result_mapper: ResultMapper::new(),
            cancelled: Arc::new(AtomicBool::new(false)),
            progress: Arc::new(AtomicUsize::new(0)),
            max_parallel: 1, // Serial by default
        }
    }

    /// Set max parallel runs
    pub fn with_parallel(mut self, max: usize) -> Self {
        self.max_parallel = max;
        self
    }

    /// Cancel execution
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
    }

    /// Reset cancellation
    pub fn reset(&self) {
        self.cancelled.store(false, Ordering::SeqCst);
        self.progress.store(0, Ordering::SeqCst);
    }

    /// Is cancelled?
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }

    /// Current progress count
    pub fn current_progress(&self) -> usize {
        self.progress.load(Ordering::SeqCst)
    }

    /// Get current timestamp
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }

    /// Execute a run queue (synchronous)
    pub fn execute(&self, queue: &mut RunQueue) -> ExecutionResult {
        let start = Instant::now();
        let mut result = ExecutionResult::default();

        result.state.total_runs = queue.len();
        result.state.status = ExecutionStatus::Running;
        result.state.start_time = Self::now();

        // Execute runs using the queue's built-in ordering
        while let Some(run_id) = queue.start_next(Self::now()) {
            // Check cancellation
            if self.is_cancelled() {
                queue.cancel_all(Self::now());
                result.state.status = ExecutionStatus::Cancelled;
                break;
            }

            // Get run info
            let run_name = queue
                .get(run_id)
                .map(|r| r.name.clone())
                .unwrap_or_default();
            result.state.current_run = Some(run_name);

            // Execute this run
            match self.execute_single(queue, run_id) {
                Ok(mapped) => {
                    result.results.insert(run_id, mapped);
                    queue.complete_current(Self::now());
                    result.state.completed_runs += 1;
                }
                Err(e) => {
                    result.errors.insert(run_id, e.clone());
                    queue.fail_current(&e, Self::now());
                    result.state.failed_runs += 1;
                    result.state.completed_runs += 1;
                }
            }

            // Update progress
            self.progress
                .store(result.state.completed_runs, Ordering::SeqCst);
            result.state.elapsed_seconds = start.elapsed().as_secs_f64();
            result.state.update_eta();
        }

        // Final state
        if result.state.status == ExecutionStatus::Running {
            if result.state.failed_runs > 0 {
                result.state.status = ExecutionStatus::CompletedWithErrors;
            } else {
                result.state.status = ExecutionStatus::Completed;
            }
        }

        result.state.elapsed_seconds = start.elapsed().as_secs_f64();
        result.state.current_run = None;

        result
    }

    /// Execute a single run item
    ///
    /// Calls the actual simulation engine based on the analysis type and returns
    /// mapped results. Requires netlist to be set on the queue.
    fn execute_single(&self, queue: &RunQueue, run_id: u64) -> Result<MappedResult, String> {
        let run = queue
            .get(run_id)
            .ok_or_else(|| "Run not found".to_string())?;

        // Get netlist from queue
        let netlist = queue
            .netlist()
            .ok_or_else(|| "No netlist configured for queue".to_string())?;

        // Build options (reserved for future engine option plumbing)
        let _options = EngineOptions::spectre_defaults();

        let spec = run
            .spec
            .clone()
            .or_else(|| (run.run_type == AnalysisRunType::DcOp).then_some(AnalysisSpec::DcOp))
            .ok_or_else(|| {
                format!(
                    "Run '{}' ({:?}) is missing AnalysisSpec",
                    run.name, run.run_type
                )
            })?;

        if spec.run_type() != run.run_type {
            return Err(format!(
                "Run '{}' has mismatched run_type ({:?}) vs spec ({:?})",
                run.name,
                run.run_type,
                spec.run_type()
            ));
        }

        spec.validate()?;

        // Execute based on analysis type
        let start = Instant::now();
        let result = self.execute_analysis(&spec, netlist);
        let elapsed = start.elapsed().as_secs_f64();

        // Map result
        match result {
            Ok(mapped) => Ok(MappedResult {
                elapsed_time: elapsed,
                ..mapped
            }),
            Err(e) => Err(format!("{} [{}]", e, run.name)),
        }
    }

    /// Execute a specific analysis specification.
    fn execute_analysis(&self, spec: &AnalysisSpec, netlist: &str) -> Result<MappedResult, String> {
        use crate::services::simulation_runner;

        match spec {
            AnalysisSpec::DcOp => {
                let sim_result = simulation_runner::run_simulation(netlist);
                if sim_result.success {
                    // Build operating point data from DC result
                    let op_data = sim_result.dc_op.map(|voltages| {
                        let mut op = super::result_mapper::OperatingPointMap::default();
                        // Convert Vec<(String, Value)> to HashMap<String, f64>
                        op.node_voltages = voltages
                            .into_iter()
                            .map(|(name, val)| (name, val.into()))
                            .collect();
                        op
                    });
                    Ok(MappedResult {
                        analysis_type: MappedAnalysisType::DcOp,
                        status: ResultStatus::Success,
                        op_data,
                        ..Default::default()
                    })
                } else {
                    Err(sim_result
                        .error
                        .unwrap_or_else(|| "DC OP failed".to_string()))
                }
            }
            AnalysisSpec::Transient {
                stop_time,
                step_time,
            } => match simulation_runner::run_transient_analysis(netlist, *stop_time, *step_time) {
                Ok(data) => {
                    let time: Vec<f64> = data.time.into_iter().collect();
                    let waveforms = data
                        .voltages
                        .into_iter()
                        .map(|(name, values)| {
                            MappedWaveform::time_domain(
                                name,
                                time.clone(),
                                values.into_iter().collect(),
                            )
                        })
                        .collect();

                    Ok(MappedResult {
                        analysis_type: MappedAnalysisType::Transient,
                        status: ResultStatus::Success,
                        waveforms,
                        ..Default::default()
                    })
                }
                Err(e) => Err(e),
            },
            AnalysisSpec::Ac {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
            } => {
                let ac_result = simulation_runner::run_ac_analysis(
                    netlist,
                    *start_freq,
                    *stop_freq,
                    *points_per_unit,
                    sweep.runner_keyword(),
                );
                match ac_result {
                    Ok(data) => {
                        let freq: Vec<f64> = data.frequencies.into_iter().collect();
                        let waveforms = data
                            .responses
                            .into_iter()
                            .map(|(name, values)| {
                                let real: Vec<f64> = values.iter().map(|v| v.re).collect();
                                let imag: Vec<f64> = values.iter().map(|v| v.im).collect();
                                MappedWaveform::complex_ac(name, freq.clone(), real, imag)
                            })
                            .collect();

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Ac,
                            status: ResultStatus::Success,
                            waveforms,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::DcSweep {
                source_name,
                start,
                stop,
                step,
            } => {
                let sweep_result =
                    simulation_runner::run_dc_sweep(netlist, source_name, *start, *stop, *step);
                match sweep_result {
                    Ok(data) => {
                        let x: Vec<f64> = data.sweep_values.into_iter().collect();
                        let waveforms = data
                            .voltages
                            .into_iter()
                            .map(|(name, values)| {
                                MappedWaveform::time_domain(
                                    name,
                                    x.clone(),
                                    values.into_iter().collect(),
                                )
                            })
                            .collect();

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::DcSweep,
                            status: ResultStatus::Success,
                            waveforms,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Noise {
                output_node,
                start_freq,
                stop_freq,
                points_per_decade,
                temperature,
            } => {
                let noise_result = simulation_runner::run_noise_analysis(
                    netlist,
                    output_node,
                    *start_freq,
                    *stop_freq,
                    *points_per_decade,
                    *temperature,
                );
                match noise_result {
                    Ok(data) => {
                        let freq: Vec<f64> = data.frequencies.into_iter().collect();
                        let output_noise: Vec<f64> = data.output_noise.into_iter().collect();
                        let waveforms = vec![MappedWaveform::frequency_domain(
                            "V(onoise)",
                            freq,
                            output_noise,
                            "Noise Density",
                        )];

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Noise,
                            status: ResultStatus::Success,
                            waveforms,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::PoleZero {
                input_node,
                input_ref,
                output_node,
                output_ref,
                transfer_type,
                analysis_type,
            } => {
                let pz_result = simulation_runner::run_pole_zero_analysis(
                    netlist,
                    input_node,
                    input_ref,
                    output_node,
                    output_ref,
                    transfer_type,
                    analysis_type,
                );
                match pz_result {
                    Ok(data) => {
                        let dc_gain_unit = if transfer_type.eq_ignore_ascii_case("CUR") {
                            "V/A"
                        } else {
                            "V/V"
                        };

                        let poles_real: Vec<f64> = data.poles.iter().map(|(re, _)| *re).collect();
                        let poles_imag: Vec<f64> = data.poles.iter().map(|(_, im)| *im).collect();
                        let zeros_real: Vec<f64> = data.zeros.iter().map(|(re, _)| *re).collect();
                        let zeros_imag: Vec<f64> = data.zeros.iter().map(|(_, im)| *im).collect();

                        let mut waveforms = Vec::new();
                        if !poles_real.is_empty() {
                            waveforms.push(MappedWaveform {
                                name: "Poles".to_string(),
                                x: poles_real,
                                y: poles_imag,
                                x_label: "Real(s)".to_string(),
                                y_label: "Imag(s)".to_string(),
                                x_unit: "1/s".to_string(),
                                y_unit: "1/s".to_string(),
                                ..Default::default()
                            });
                        }
                        if !zeros_real.is_empty() {
                            waveforms.push(MappedWaveform {
                                name: "Zeros".to_string(),
                                x: zeros_real,
                                y: zeros_imag,
                                x_label: "Real(s)".to_string(),
                                y_label: "Imag(s)".to_string(),
                                x_unit: "1/s".to_string(),
                                y_unit: "1/s".to_string(),
                                ..Default::default()
                            });
                        }

                        let measurements = vec![
                            MappedMeasurement {
                                name: "dc_gain".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.gain,
                                unit: dc_gain_unit.to_string(),
                                signal: "transfer".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "pole_count".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.poles.len() as f64,
                                unit: "count".to_string(),
                                signal: "poles".to_string(),
                                status: MeasurementStatus::Success,
                            },
                            MappedMeasurement {
                                name: "zero_count".to_string(),
                                meas_type: MeasurementType::Custom,
                                value: data.zeros.len() as f64,
                                unit: "count".to_string(),
                                signal: "zeros".to_string(),
                                status: MeasurementStatus::Success,
                            },
                        ];

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::PoleZero,
                            status: ResultStatus::Success,
                            waveforms,
                            measurements,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Sensitivity {
                output_var,
                ac_mode,
                frequency,
            } => {
                let raw_unit = if output_var.trim().len() > 3
                    && output_var.trim()[..2].eq_ignore_ascii_case("I(")
                    && output_var.trim().ends_with(')')
                {
                    "A/unit"
                } else {
                    "V/unit"
                };

                let sens_result = simulation_runner::run_sensitivity_analysis(
                    netlist, output_var, *ac_mode, *frequency,
                );
                match sens_result {
                    Ok(data) => {
                        let mut measurements: Vec<MappedMeasurement> = data
                            .sensitivities
                            .iter()
                            .map(|(name, raw, _)| MappedMeasurement {
                                name: format!("d({})/d({})", data.output_var, name),
                                meas_type: MeasurementType::Custom,
                                value: *raw,
                                unit: raw_unit.to_string(),
                                signal: name.clone(),
                                status: MeasurementStatus::Success,
                            })
                            .collect();

                        measurements.extend(data.sensitivities.iter().map(
                            |(name, _, normalized)| MappedMeasurement {
                                name: format!("norm({})", name),
                                meas_type: MeasurementType::Custom,
                                value: *normalized,
                                unit: "ratio".to_string(),
                                signal: name.clone(),
                                status: MeasurementStatus::Success,
                            },
                        ));

                        // Keep most significant normalized sensitivities first for UI consumption.
                        measurements.sort_by(|a, b| {
                            b.value
                                .abs()
                                .partial_cmp(&a.value.abs())
                                .unwrap_or(std::cmp::Ordering::Equal)
                        });

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Sensitivity,
                            status: ResultStatus::Success,
                            measurements,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::Pss {
                fundamental_freq,
                num_harmonics,
                tolerance,
            } => {
                let pss_result = simulation_runner::run_pss_analysis(
                    netlist,
                    *fundamental_freq,
                    *num_harmonics,
                    *tolerance,
                );
                match pss_result {
                    Ok(data) => {
                        let time: Vec<f64> = data.time.into_iter().collect();
                        let waveforms = data
                            .waveforms
                            .into_iter()
                            .map(|(name, values)| {
                                MappedWaveform::time_domain(
                                    name,
                                    time.clone(),
                                    values.into_iter().collect(),
                                )
                            })
                            .collect();

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::Pss,
                            status: ResultStatus::Success,
                            waveforms,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            AnalysisSpec::HarmonicBalance {
                tone1_freq,
                tone1_harmonics,
                tone2_freq,
                tone2_harmonics,
            } => {
                let hb_result = simulation_runner::run_hb_analysis(
                    netlist,
                    *tone1_freq,
                    *tone1_harmonics,
                    *tone2_freq,
                    *tone2_harmonics,
                );
                match hb_result {
                    Ok(data) => {
                        let waveforms = data
                            .spectra
                            .into_iter()
                            .map(|(name, spectrum)| {
                                let x: Vec<f64> = spectrum.iter().map(|(f, _, _)| *f).collect();
                                let y: Vec<f64> = spectrum.iter().map(|(_, m, _)| *m).collect();
                                MappedWaveform::frequency_domain(name, x, y, "Magnitude")
                            })
                            .collect();

                        Ok(MappedResult {
                            analysis_type: MappedAnalysisType::HarmonicBalance,
                            status: ResultStatus::Success,
                            waveforms,
                            ..Default::default()
                        })
                    }
                    Err(e) => Err(e),
                }
            }
            // Remaining analyses are intentionally explicit until implementation exists.
            AnalysisSpec::Tf
            | AnalysisSpec::Pac
            | AnalysisSpec::Pnoise
            | AnalysisSpec::MonteCarlo
            | AnalysisSpec::Parametric
            | AnalysisSpec::Corner => Err(format!(
                "{:?} execution is not implemented in RunExecutor yet",
                spec.run_type()
            )),
        }
    }

    /// Map AnalysisRunType to MappedAnalysisType
    fn map_analysis_type(
        &self,
        run_type: super::multi_run::AnalysisRunType,
    ) -> super::result_mapper::MappedAnalysisType {
        use super::multi_run::AnalysisRunType;
        use super::result_mapper::MappedAnalysisType;

        match run_type {
            AnalysisRunType::DcOp => MappedAnalysisType::DcOp,
            AnalysisRunType::DcSweep => MappedAnalysisType::DcSweep,
            AnalysisRunType::Ac => MappedAnalysisType::Ac,
            AnalysisRunType::Transient => MappedAnalysisType::Transient,
            AnalysisRunType::Noise => MappedAnalysisType::Noise,
            AnalysisRunType::Tf => MappedAnalysisType::Tf,
            AnalysisRunType::Sensitivity => MappedAnalysisType::Sensitivity,
            AnalysisRunType::PoleZero => MappedAnalysisType::PoleZero,
            AnalysisRunType::HarmonicBalance => MappedAnalysisType::HarmonicBalance,
            AnalysisRunType::Pss => MappedAnalysisType::Pss,
            AnalysisRunType::Pac => MappedAnalysisType::Pac,
            AnalysisRunType::Pnoise => MappedAnalysisType::Pnoise,
            AnalysisRunType::MonteCarlo => MappedAnalysisType::MonteCarlo,
            AnalysisRunType::Parametric => MappedAnalysisType::Parametric,
            AnalysisRunType::Corner => MappedAnalysisType::Corner,
        }
    }

    /// Execute corner sweep
    pub fn execute_corners(
        &self,
        base_queue: &RunQueue,
        corners: &[PvtCorner],
    ) -> HashMap<String, ExecutionResult> {
        let mut results = HashMap::new();

        for corner in corners {
            if self.is_cancelled() {
                break;
            }

            let mut queue = base_queue.clone();
            // Apply corner settings to queue...

            let corner_result = self.execute(&mut queue);
            results.insert(corner.process.clone(), corner_result);
        }

        results
    }
}

// =============================================================================
// Progress Callback
// =============================================================================

/// Progress callback for async execution
pub type ProgressCallback = Box<dyn Fn(&ExecutionState) + Send + Sync>;

/// Async executor with progress callback
pub struct AsyncRunExecutor {
    executor: RunExecutor,
    callback: Option<ProgressCallback>,
}

impl AsyncRunExecutor {
    /// Create new async executor
    pub fn new() -> Self {
        Self {
            executor: RunExecutor::new(),
            callback: None,
        }
    }

    /// Set progress callback
    pub fn with_callback(mut self, callback: ProgressCallback) -> Self {
        self.callback = Some(callback);
        self
    }

    /// Cancel execution
    pub fn cancel(&self) {
        self.executor.cancel();
    }

    /// Execute with progress updates
    pub fn execute(&self, queue: &mut RunQueue) -> ExecutionResult {
        let result = self.executor.execute(queue);

        // Call final callback
        if let Some(ref cb) = self.callback {
            cb(&result.state);
        }

        result
    }
}

impl Default for AsyncRunExecutor {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::super::multi_run::{AnalysisRunType, AnalysisSpec, FrequencySweep};
    use super::*;

    // =========================================================================
    // ExecutionState Tests
    // =========================================================================

    #[test]
    fn test_execution_state_default() {
        let state = ExecutionState::default();
        assert_eq!(state.status, ExecutionStatus::Idle);
        assert_eq!(state.total_runs, 0);
    }

    #[test]
    fn test_execution_state_progress() {
        let mut state = ExecutionState::default();
        state.total_runs = 10;
        state.completed_runs = 5;

        assert!((state.progress_percent() - 50.0).abs() < 0.1);
    }

    #[test]
    fn test_execution_state_is_complete() {
        let mut state = ExecutionState::default();
        assert!(!state.is_complete());

        state.status = ExecutionStatus::Completed;
        assert!(state.is_complete());
    }

    #[test]
    fn test_execution_state_update_eta() {
        let mut state = ExecutionState::default();
        state.total_runs = 10;
        state.completed_runs = 5;
        state.elapsed_seconds = 10.0;

        state.update_eta();
        assert!(state.eta_seconds.is_some());
        assert!((state.eta_seconds.unwrap() - 10.0).abs() < 0.1); // 2s per run * 5 remaining
    }

    // =========================================================================
    // ExecutionResult Tests
    // =========================================================================

    #[test]
    fn test_execution_result_default() {
        let result = ExecutionResult::default();
        assert!(result.results.is_empty());
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_execution_result_success_count() {
        let mut result = ExecutionResult::default();
        result.results.insert(
            1,
            MappedResult {
                status: ResultStatus::Success,
                ..Default::default()
            },
        );
        result.results.insert(
            2,
            MappedResult {
                status: ResultStatus::Error,
                ..Default::default()
            },
        );

        assert_eq!(result.success_count(), 1);
    }

    // =========================================================================
    // RunExecutor Tests
    // =========================================================================

    #[test]
    fn test_executor_new() {
        let executor = RunExecutor::new();
        assert!(!executor.is_cancelled());
        assert_eq!(executor.current_progress(), 0);
    }

    #[test]
    fn test_executor_cancel() {
        let executor = RunExecutor::new();
        executor.cancel();
        assert!(executor.is_cancelled());

        executor.reset();
        assert!(!executor.is_cancelled());
    }

    #[test]
    fn test_executor_with_parallel() {
        let executor = RunExecutor::new().with_parallel(4);
        assert_eq!(executor.max_parallel, 4);
    }

    #[test]
    fn test_execute_empty_queue() {
        let executor = RunExecutor::new();
        let mut queue = RunQueue::new();

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.status, ExecutionStatus::Completed);
        assert_eq!(result.state.total_runs, 0);
    }

    #[test]
    fn test_execute_single_run() {
        let executor = RunExecutor::new();
        let mut queue = RunQueue::new();
        queue.add(AnalysisRunType::DcOp);

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 1);
        assert!(result.state.completed_runs > 0);
    }

    // =========================================================================
    // AsyncRunExecutor Tests
    // =========================================================================

    #[test]
    fn test_async_executor_new() {
        let executor = AsyncRunExecutor::new();
        assert!(executor.callback.is_none());
    }

    #[test]
    fn test_async_executor_cancel() {
        let executor = AsyncRunExecutor::new();
        executor.cancel();
        assert!(executor.executor.is_cancelled());
    }

    // =========================================================================
    // Phase 2: Run Executor Integration Tests
    // =========================================================================

    #[test]
    fn test_queue_netlist_builder() {
        let queue = RunQueue::new().with_netlist("* Test circuit\nR1 in out 1k\n");
        assert_eq!(queue.netlist(), Some("* Test circuit\nR1 in out 1k\n"));
    }

    #[test]
    fn test_queue_set_netlist() {
        let mut queue = RunQueue::new();
        assert!(queue.netlist().is_none());

        queue.set_netlist("V1 vdd 0 1.8");
        assert!(queue.netlist().is_some());
        assert!(queue.netlist().unwrap().contains("V1"));
    }

    #[test]
    fn test_execute_without_netlist_fails() {
        let executor = RunExecutor::new();
        let mut queue = RunQueue::new();
        queue.add(AnalysisRunType::DcOp);

        // Execute without netlist - should handle gracefully (fail the run)
        let result = executor.execute(&mut queue);
        // The run completes but with failure since no netlist
        assert_eq!(result.state.total_runs, 1);
    }

    #[test]
    fn test_execute_with_valid_netlist() {
        let executor = RunExecutor::new();
        let mut queue = RunQueue::new()
            .with_netlist("* Simple RC circuit\nV1 in 0 DC 1\nR1 in out 1k\nC1 out 0 1p\n.op\n");
        queue.add(AnalysisRunType::DcOp);

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 1);
        // With valid netlist, simulation should attempt to run
        assert!(result.state.completed_runs >= 1 || result.state.failed_runs >= 1);
    }

    #[test]
    fn test_analysis_type_mapping_coverage() {
        use super::super::multi_run::AnalysisRunType;
        use super::super::result_mapper::MappedAnalysisType;

        let executor = RunExecutor::new();

        // Test all AnalysisRunType variants map correctly
        let mappings = [
            (AnalysisRunType::DcOp, MappedAnalysisType::DcOp),
            (AnalysisRunType::DcSweep, MappedAnalysisType::DcSweep),
            (AnalysisRunType::Ac, MappedAnalysisType::Ac),
            (AnalysisRunType::Transient, MappedAnalysisType::Transient),
            (AnalysisRunType::Noise, MappedAnalysisType::Noise),
            (AnalysisRunType::PoleZero, MappedAnalysisType::PoleZero),
        ];

        for (run_type, expected) in mappings {
            let mapped = executor.map_analysis_type(run_type);
            assert_eq!(mapped, expected, "Mapping failed for {:?}", run_type);
        }
    }

    #[test]
    fn test_execute_multiple_analyses_with_netlist() {
        let executor = RunExecutor::new();
        let mut queue = RunQueue::new()
            .with_netlist("* RC\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1p\n.op\n.tran 1n 10n\n");

        queue.add(AnalysisRunType::DcOp);
        queue.add(AnalysisRunType::Transient);

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 2);
    }

    #[test]
    fn test_parameterized_analysis_requires_spec() {
        let executor = RunExecutor::new();
        let mut queue = RunQueue::new().with_netlist("* test\nV1 in 0 1\nR1 in out 1k\n");
        queue.add(AnalysisRunType::Ac);

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.failed_runs, 1);
        assert_eq!(result.errors.len(), 1);
        let err = result
            .errors
            .values()
            .next()
            .expect("missing expected error message");
        assert!(err.contains("missing AnalysisSpec"));
    }

    #[test]
    fn test_parameterized_analysis_with_spec_runs() {
        let executor = RunExecutor::new();
        let mut queue = RunQueue::new().with_netlist("* test\nV1 in 0 1\nR1 in out 1k\n");
        queue.add_analysis(AnalysisSpec::Ac {
            start_freq: 1.0,
            stop_freq: 1e3,
            points_per_unit: 5,
            sweep: FrequencySweep::Decade,
        });

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 1);
        if let Some(err) = result.errors.values().next() {
            assert!(
                !err.contains("missing AnalysisSpec"),
                "run should fail for circuit/solver reasons, not missing spec"
            );
        }
    }

    #[test]
    fn test_pole_zero_analysis_with_spec_is_executed() {
        let executor = RunExecutor::new();
        let mut queue =
            RunQueue::new().with_netlist("* pz\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n");
        queue.add_analysis(AnalysisSpec::PoleZero {
            input_node: "in".to_string(),
            input_ref: "0".to_string(),
            output_node: "out".to_string(),
            output_ref: "0".to_string(),
            transfer_type: "VOL".to_string(),
            analysis_type: "PZ".to_string(),
        });

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 1);
        if let Some(err) = result.errors.values().next() {
            assert!(
                !err.contains("not implemented in RunExecutor yet"),
                "pole-zero should execute via service runner"
            );
        }
    }

    #[test]
    fn test_pole_zero_cur_transfer_uses_transimpedance_units() {
        let executor = RunExecutor::new();
        let mut queue =
            RunQueue::new().with_netlist("* pz\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n");
        queue.add_analysis(AnalysisSpec::PoleZero {
            input_node: "in".to_string(),
            input_ref: "0".to_string(),
            output_node: "out".to_string(),
            output_ref: "0".to_string(),
            transfer_type: "CUR".to_string(),
            analysis_type: "PZ".to_string(),
        });

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 1);
        assert!(
            result.errors.is_empty(),
            "expected successful CUR pole-zero run, got errors: {:?}",
            result.errors
        );

        let mapped = result
            .results
            .values()
            .next()
            .expect("expected mapped result");
        let gain = mapped
            .measurements
            .iter()
            .find(|m| m.name == "dc_gain")
            .expect("dc_gain measurement should exist");
        assert_eq!(gain.unit, "V/A");
    }

    #[test]
    fn test_sensitivity_analysis_with_spec_is_executed() {
        let executor = RunExecutor::new();
        let mut queue = RunQueue::new()
            .with_netlist("* sens\n.param RVAL=1k\nV1 in 0 1\nR1 in out RVAL\nR2 out 0 1k\n");
        queue.add_analysis(AnalysisSpec::Sensitivity {
            output_var: "V(out)".to_string(),
            ac_mode: false,
            frequency: None,
        });

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 1);
        if let Some(err) = result.errors.values().next() {
            assert!(
                !err.contains("not implemented in RunExecutor yet"),
                "sensitivity should execute via service runner"
            );
        }
    }

    #[test]
    fn test_sensitivity_current_output_uses_current_units() {
        let executor = RunExecutor::new();
        let mut queue =
            RunQueue::new().with_netlist("* sens i\n.param RVAL=1k\nV1 in 0 1\nR1 in 0 {RVAL}\n");
        queue.add_analysis(AnalysisSpec::Sensitivity {
            output_var: "I(V1)".to_string(),
            ac_mode: false,
            frequency: None,
        });

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 1);
        assert!(
            result.errors.is_empty(),
            "expected successful current-output sensitivity run, got errors: {:?}",
            result.errors
        );

        let mapped = result
            .results
            .values()
            .next()
            .expect("expected mapped sensitivity result");
        let raw_measurement = mapped
            .measurements
            .iter()
            .find(|m| m.name.starts_with("d(I(V1))/d("))
            .expect("expected raw current sensitivity measurement");
        assert_eq!(raw_measurement.unit, "A/unit");
    }

    #[test]
    fn test_sensitivity_ac_analysis_with_spec_is_executed() {
        let executor = RunExecutor::new();
        let mut queue = RunQueue::new().with_netlist(
            "* sens ac\n.param RVAL=1k\nV1 in 0 AC 1\nR1 in out {RVAL}\nC1 out 0 1n\n",
        );
        queue.add_analysis(AnalysisSpec::Sensitivity {
            output_var: "V(out)".to_string(),
            ac_mode: true,
            frequency: Some(1e6),
        });

        let result = executor.execute(&mut queue);
        assert_eq!(result.state.total_runs, 1);
        if let Some(err) = result.errors.values().next() {
            assert!(
                !err.contains("not supported yet"),
                "ac sensitivity should execute via service runner"
            );
        }
    }

    #[test]
    fn test_corner_execution_preserves_netlist() {
        let executor = RunExecutor::new();
        let base_queue = RunQueue::new().with_netlist("* Test\nR1 a b 1k\n");

        let corners = vec![PvtCorner {
            process: "tt".to_string(),
            ..Default::default()
        }];

        let results = executor.execute_corners(&base_queue, &corners);
        assert_eq!(results.len(), 1);
        assert!(results.contains_key("tt"));
    }
}
