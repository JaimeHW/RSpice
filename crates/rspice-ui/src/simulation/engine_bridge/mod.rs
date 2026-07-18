//! Engine Bridge - Connect UI to rspice-core Simulation Engine
//!
//! This module provides a clean interface between the UI simulation runner
//! and the rspice-core Engine. It handles:
//!
//! - Netlist parsing from UI-generated strings
//! - Mapping UI AnalysisConfig to core engine calls
//! - Converting core results to UI WaveformData format
//! - Error translation
//!
//! # Architecture
//!
//! The bridge follows the adapter pattern:
//! ```text
//! UI SimulationRunner -> EngineBridge -> rspice-core Engine
//!      (config)           (parse)         (run)
//!      (result) <-        (convert) <-    (result)
//! ```

use std::path::Path;

use rspice_core::abort_signal::{AbortSignal, NoAbort};

use super::config::AnalysisConfig;
use super::results::SimulationResult;
use super::runner::SimulationError;

mod ac_noise;
mod dc;
mod error;
mod measure;
mod parsing;
mod pole_zero;
mod sensitivity;
mod transient;

/// Bridge between UI and rspice-core engine
///
/// Handles parsing, execution, and result conversion for all analysis types.
pub struct EngineBridge {
    /// Core engine instance
    engine: rspice_core::Engine,
}

struct SimulationInput<'a> {
    config: &'a AnalysisConfig,
    netlist_str: &'a str,
    source_path: Option<&'a Path>,
}

impl Default for EngineBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl EngineBridge {
    /// Create a new engine bridge with default configuration
    pub fn new() -> Self {
        Self {
            engine: rspice_core::Engine::default(),
        }
    }

    /// Create a new engine bridge with custom configuration
    pub fn with_config(config: rspice_core::SimulationConfig) -> Self {
        Self {
            engine: rspice_core::Engine::new(config),
        }
    }

    /// Create a new engine bridge only when the complete configuration is valid.
    ///
    /// Prefer this constructor at service and job-queue boundaries so invalid
    /// configuration is rejected before parsing or simulation work begins.
    pub fn try_with_config(
        config: rspice_core::SimulationConfig,
    ) -> Result<Self, rspice_core::SimulationConfigError> {
        Ok(Self {
            engine: rspice_core::Engine::try_new(config)?,
        })
    }

    /// Run simulation without cooperative cancellation.
    ///
    /// This compatibility convenience API is intended for direct synchronous
    /// callers. Production queued execution must use [`Self::run_with_abort`]
    /// or [`Self::run_with_abort_and_source_path`].
    pub fn run(
        &self,
        config: &AnalysisConfig,
        netlist_str: &str,
    ) -> Result<SimulationResult, SimulationError> {
        self.run_request(
            SimulationInput {
                config,
                netlist_str,
                source_path: None,
            },
            &NoAbort,
        )
    }

    /// Run simulation without cooperative cancellation, using a source path
    /// to resolve relative includes and model file references.
    ///
    /// Production queued execution must use
    /// [`Self::run_with_abort_and_source_path`].
    pub fn run_with_source_path(
        &self,
        config: &AnalysisConfig,
        netlist_str: &str,
        source_path: Option<&Path>,
    ) -> Result<SimulationResult, SimulationError> {
        self.run_request(
            SimulationInput {
                config,
                netlist_str,
                source_path,
            },
            &NoAbort,
        )
    }

    /// Run simulation with abort signal for cooperative cancellation.
    pub fn run_with_abort(
        &self,
        config: &AnalysisConfig,
        netlist_str: &str,
        abort_flag: &dyn rspice_core::abort_signal::AbortSignal,
    ) -> Result<SimulationResult, SimulationError> {
        self.run_request(
            SimulationInput {
                config,
                netlist_str,
                source_path: None,
            },
            abort_flag,
        )
    }

    /// Run simulation with cooperative cancellation and a source path for
    /// relative include/model resolution.
    pub fn run_with_abort_and_source_path(
        &self,
        config: &AnalysisConfig,
        netlist_str: &str,
        source_path: Option<&Path>,
        abort_flag: &dyn rspice_core::abort_signal::AbortSignal,
    ) -> Result<SimulationResult, SimulationError> {
        self.run_request(
            SimulationInput {
                config,
                netlist_str,
                source_path,
            },
            abort_flag,
        )
    }

    pub(crate) fn run_ac_frequencies_with_source_path(
        &self,
        netlist_str: &str,
        source_path: Option<&Path>,
        frequencies: Vec<f64>,
        abort_flag: &dyn rspice_core::abort_signal::AbortSignal,
    ) -> Result<SimulationResult, SimulationError> {
        let netlist =
            self.parse_netlist_with_abort_and_source_path(netlist_str, source_path, abort_flag)?;
        self.run_ac_frequencies(&netlist, frequencies, abort_flag)
    }

    fn run_request(
        &self,
        input: SimulationInput<'_>,
        abort_flag: &dyn AbortSignal,
    ) -> Result<SimulationResult, SimulationError> {
        let netlist = self.parse_netlist_with_abort_and_source_path(
            input.netlist_str,
            input.source_path,
            abort_flag,
        )?;
        self.dispatch_analysis(input.config, &netlist, abort_flag)
    }

    fn dispatch_analysis(
        &self,
        config: &AnalysisConfig,
        netlist: &rspice_core::Netlist,
        abort_flag: &dyn AbortSignal,
    ) -> Result<SimulationResult, SimulationError> {
        ensure_not_aborted(abort_flag)?;
        match config {
            AnalysisConfig::DcOp => self.run_dc_op(netlist, abort_flag),
            AnalysisConfig::DcSweep(dc_config) => self.run_dc_sweep(netlist, dc_config, abort_flag),
            AnalysisConfig::Transient(tran_config) => {
                self.run_transient(netlist, tran_config, abort_flag)
            }
            AnalysisConfig::Ac(ac_config) => self.run_ac(netlist, ac_config, abort_flag),
            AnalysisConfig::Noise(noise_config) => {
                self.run_noise(netlist, noise_config, abort_flag)
            }
            AnalysisConfig::PoleZero(pz_config) => self.run_pz(netlist, pz_config, abort_flag),
            AnalysisConfig::Sensitivity(sens_config) => {
                self.run_sensitivity(netlist, sens_config, abort_flag)
            }
        }
    }
}

#[inline]
pub(super) fn ensure_not_aborted(abort: &dyn AbortSignal) -> Result<(), SimulationError> {
    if abort.is_aborted() {
        Err(SimulationError::Aborted)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod cancellation_tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;
    use crate::simulation::config::{
        AcAnalysisConfig, AnalysisConfig, DcSweepConfig, NoiseAnalysisConfig, PoleZeroConfig,
        SensitivityConfig, TransientAnalysisConfig,
    };

    const TEST_NETLIST: &str = "cancellable bridge analyses\n\
         .param rload=1k\n\
         Vin in 0 DC 1 AC 1\n\
         R1 in out {rload}\n\
         C1 out 0 1n\n\
         .end\n";

    struct AbortOnPoll {
        abort_on: usize,
        polls: AtomicUsize,
    }

    impl AbortOnPoll {
        fn new(abort_on: usize) -> Self {
            Self {
                abort_on,
                polls: AtomicUsize::new(0),
            }
        }

        fn poll_count(&self) -> usize {
            self.polls.load(Ordering::Relaxed)
        }
    }

    impl AbortSignal for AbortOnPoll {
        fn is_aborted(&self) -> bool {
            self.polls.fetch_add(1, Ordering::Relaxed) + 1 >= self.abort_on
        }
    }

    fn test_netlist() -> rspice_core::Netlist {
        rspice_core::Netlist::parse(TEST_NETLIST).expect("test netlist parses")
    }

    fn assert_typed_abort<T>(result: Result<T, SimulationError>, signal: &AbortOnPoll) {
        match result {
            Err(SimulationError::Aborted) => {}
            Err(SimulationError::InvalidConfig(message)) => {
                panic!("cancellation was incorrectly reported as InvalidConfig: {message}")
            }
            Err(other) => panic!("expected typed Aborted, got {other}"),
            Ok(_) => panic!("expected cancellation, analysis completed"),
        }
        assert!(
            signal.poll_count() >= signal.abort_on,
            "analysis did not poll the supplied signal enough to reach cancellation"
        );
    }

    #[test]
    fn dc_op_observes_counter_based_cancellation() {
        let signal = AbortOnPoll::new(2);
        assert_typed_abort(
            EngineBridge::new().run_dc_op(&test_netlist(), &signal),
            &signal,
        );
    }

    #[test]
    fn dc_sweep_observes_counter_based_cancellation() {
        let signal = AbortOnPoll::new(2);
        assert_typed_abort(
            EngineBridge::new().run_dc_sweep(&test_netlist(), &DcSweepConfig::default(), &signal),
            &signal,
        );
    }

    #[test]
    fn ac_observes_counter_based_cancellation() {
        let signal = AbortOnPoll::new(2);
        assert_typed_abort(
            EngineBridge::new().run_ac(&test_netlist(), &AcAnalysisConfig::default(), &signal),
            &signal,
        );
    }

    #[test]
    fn noise_observes_counter_based_cancellation() {
        let signal = AbortOnPoll::new(2);
        assert_typed_abort(
            EngineBridge::new().run_noise(
                &test_netlist(),
                &NoiseAnalysisConfig::default(),
                &signal,
            ),
            &signal,
        );
    }

    #[test]
    fn pole_zero_observes_counter_based_cancellation() {
        let signal = AbortOnPoll::new(2);
        assert_typed_abort(
            EngineBridge::new().run_pz(&test_netlist(), &PoleZeroConfig::default(), &signal),
            &signal,
        );
    }

    #[test]
    fn sensitivity_observes_counter_based_cancellation() {
        let signal = AbortOnPoll::new(2);
        assert_typed_abort(
            EngineBridge::new().run_sensitivity(
                &test_netlist(),
                &SensitivityConfig::default(),
                &signal,
            ),
            &signal,
        );
    }

    #[test]
    fn transient_observes_counter_based_cancellation() {
        let signal = AbortOnPoll::new(2);
        assert_typed_abort(
            EngineBridge::new().run_transient(
                &test_netlist(),
                &TransientAnalysisConfig::default(),
                &signal,
            ),
            &signal,
        );
    }

    #[test]
    fn production_dispatch_threads_the_signal_to_every_config_family() {
        let cases = [
            ("dc-op", AnalysisConfig::DcOp),
            (
                "dc-sweep",
                AnalysisConfig::DcSweep(DcSweepConfig::default()),
            ),
            (
                "transient",
                AnalysisConfig::Transient(TransientAnalysisConfig::default()),
            ),
            ("ac", AnalysisConfig::Ac(AcAnalysisConfig::default())),
            (
                "noise",
                AnalysisConfig::Noise(NoiseAnalysisConfig::default()),
            ),
            (
                "pole-zero",
                AnalysisConfig::PoleZero(PoleZeroConfig::default()),
            ),
            (
                "sensitivity",
                AnalysisConfig::Sensitivity(SensitivityConfig::default()),
            ),
        ];

        for (family, config) in cases {
            let signal = AbortOnPoll::new(4);
            let result = EngineBridge::new().run_with_abort(&config, TEST_NETLIST, &signal);
            match result {
                Err(SimulationError::Aborted) => {}
                Err(SimulationError::InvalidConfig(message)) => panic!(
                    "{family} cancellation was incorrectly reported as InvalidConfig: {message}"
                ),
                Err(other) => panic!("{family} expected typed Aborted, got {other}"),
                Ok(_) => panic!("{family} should have observed cancellation"),
            }
            assert!(signal.poll_count() >= 4);
        }
    }
}
