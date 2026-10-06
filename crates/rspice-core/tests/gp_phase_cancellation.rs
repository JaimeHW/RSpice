//! Cancellation requested after accepted GP work, rather than during setup.
//! Work-count assertions are portable; wall-time measurements live in rspice-bench.

use rspice_core::abort_signal::{AbortSignal, TransientSample};
use rspice_core::engine::TransientStartupMode;
use rspice_core::{
    Engine, GpTransientPhaseModel, Netlist, SimulationConfig, SimulationError, SpiceDialect,
};
use std::sync::atomic::{AtomicUsize, Ordering};

struct CancelAfterSamples {
    request_at: usize,
    samples: AtomicUsize,
    observed_polls: AtomicUsize,
}

impl CancelAfterSamples {
    fn new(request_at: usize) -> Self {
        Self {
            request_at,
            samples: AtomicUsize::new(0),
            observed_polls: AtomicUsize::new(0),
        }
    }

    fn assert_stopped(&self) {
        assert_eq!(
            self.samples.load(Ordering::Relaxed),
            self.request_at,
            "no further sample may be accepted after cancellation on these nonlinear decks"
        );
        assert!(self.observed_polls.load(Ordering::Relaxed) > 0);
    }
}

impl AbortSignal for CancelAfterSamples {
    fn is_aborted(&self) -> bool {
        if self.samples.load(Ordering::Relaxed) >= self.request_at {
            self.observed_polls.fetch_add(1, Ordering::Relaxed);
            true
        } else {
            false
        }
    }

    fn observe_transient_sample(&self, _sample: TransientSample<'_>) {
        self.samples.fetch_add(1, Ordering::Relaxed);
    }
}

fn netlist() -> Netlist {
    Netlist::parse("GP accepted-work cancellation\nVC c 0 2\nVB b 0 DC .6 SIN(.6 .005 1G)\nQ1 c b 0 qm\n.model qm NPN IS=1e-16 BF=100 BR=1 TF=1n PTF=21 RB=100 RBM=20 IRB=1e-5 RE=1 RC=2\n.options RELTOL=1e-7 ABSTOL=1e-16 VNTOL=1e-10 GMIN=0\n.end\n").unwrap()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_cancellation_after_accepted_work_bounds_progress_and_preserves_reuse() {
    let source = netlist();
    for dialect in [
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
        SpiceDialect::BestAvailable,
    ] {
        for model in [
            GpTransientPhaseModel::ExactDelay,
            GpTransientPhaseModel::NgspiceWeil,
        ] {
            let engine = Engine::new(SimulationConfig {
                spice_dialect: dialect,
                gp_transient_phase_model: model,
                ..SimulationConfig::default()
            });
            for startup in [
                TransientStartupMode::OperatingPoint,
                TransientStartupMode::Uic,
            ] {
                let baseline = engine
                    .run_tran_with_startup_mode(&source, 2e-9, 4e-12, startup)
                    .unwrap();
                for request_at in [1, 32, 128] {
                    let abort = CancelAfterSamples::new(request_at);
                    let outcome = engine.run_tran_with_startup_mode_and_abort(
                        &source, 2e-9, 4e-12, startup, &abort,
                    );
                    assert!(
                        matches!(outcome, Err(SimulationError::Aborted)),
                        "{dialect:?}/{model:?}/{startup:?}: {outcome:?}"
                    );
                    abort.assert_stopped();
                }
                let reused = engine
                    .run_tran_with_startup_mode(&source, 2e-9, 4e-12, startup)
                    .unwrap();
                assert_eq!(baseline.time, reused.time);
                assert_eq!(baseline.step_sizes, reused.step_sizes);
                assert_eq!(baseline.voltages, reused.voltages);
                assert_eq!(baseline.branch_currents, reused.branch_currents);
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_cancelled_continuation_preserves_checkpoint_and_future_continuation() {
    let source = netlist();
    for model in [
        GpTransientPhaseModel::ExactDelay,
        GpTransientPhaseModel::NgspiceWeil,
    ] {
        let engine = Engine::new(SimulationConfig {
            gp_transient_phase_model: model,
            ..SimulationConfig::default()
        });
        let (_, checkpoint) = engine.run_tran_checkpointed(&source, 1e-9, 4e-12).unwrap();
        let encoded = checkpoint.to_text();
        let (baseline, final_checkpoint) = engine
            .run_tran_resume(&source, &checkpoint, 2e-9, 4e-12)
            .unwrap();
        let abort = CancelAfterSamples::new(32);
        assert!(matches!(
            engine.run_tran_resume_with_abort(&source, &checkpoint, 2e-9, 4e-12, &abort),
            Err(SimulationError::Aborted)
        ));
        abort.assert_stopped();
        assert_eq!(checkpoint.to_text(), encoded);
        let (reused, reused_checkpoint) = engine
            .run_tran_resume(&source, &checkpoint, 2e-9, 4e-12)
            .unwrap();
        assert_eq!(baseline.time, reused.time);
        assert_eq!(baseline.step_sizes, reused.step_sizes);
        assert_eq!(baseline.voltages, reused.voltages);
        assert_eq!(baseline.branch_currents, reused.branch_currents);
        assert_eq!(final_checkpoint.to_text(), reused_checkpoint.to_text());
    }
}
