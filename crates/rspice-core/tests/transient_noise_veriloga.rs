//! Transient noise (`.TRAN … NOISEFMAX=`) beyond the native device catalog.
//!
//! The native mechanisms have their own analytic oracles beside the injection
//! code. This suite owns what a transient-noise run promises about everything
//! else that can carry a noise source, and about the run's own contract:
//!
//! * a transient-noise run that would need its noise streams persisted is a
//!   capability refusal, typed, at both places that can be asked for one.
//!
//! Every statistical assertion states its interval and where the interval
//! comes from before it runs, and every run is seeded, so a failure is a
//! defect rather than a draw.

use rspice_core::abort_signal::NoAbort;
use rspice_core::engine::{TransientCheckpointStart, TransientStartupMode};
use rspice_core::identity::AnalysisKind;
use rspice_core::{Engine, Netlist, SimulationConfig, SimulationError};

/// The one token both checkpoint refusals carry.
const CHECKPOINT_TOKEN: &str = "analysis.tran.checkpoint.transient_noise";

fn engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

/// Assert a refusal is the typed transient-noise checkpoint refusal.
fn assert_checkpoint_refusal(error: &SimulationError, site: &str) {
    let SimulationError::UnsupportedCapability(refusal) = error else {
        panic!("{site}: expected a capability refusal, got {error}");
    };
    assert_eq!(refusal.capability, CHECKPOINT_TOKEN, "{site}: {error}");
    assert_eq!(
        error.descriptor().analysis_kind,
        Some(AnalysisKind::Tran),
        "{site}: the refusal must name the transient as its analysis kind"
    );
    assert!(
        error.to_string().contains("NOISEFMAX"),
        "{site}: the detail must still name the keyword for a person: {error}"
    );
}

/// F8: a checkpoint or resume of a transient-noise run is refused as a
/// capability, at the streamed-request validator and at the run itself.
#[test]
fn checkpointing_a_transient_noise_run_is_a_typed_capability_refusal() {
    let netlist = Netlist::parse(
        "checkpointed transient noise\n\
         R1 out 0 1k\n\
         C1 out 0 1p\n\
         .TRAN 1n 100n NOISEFMAX=500meg NOISESEED=7\n\
         .END\n",
    )
    .expect("the deck parses");

    let validator = engine()
        .validate_transient_checkpoint_request(
            &netlist,
            100.0e-9,
            1.0e-9,
            TransientCheckpointStart::Fresh(TransientStartupMode::OperatingPoint),
            &[50.0e-9],
            &NoAbort,
        )
        .expect_err("a scheduled checkpoint of a transient-noise run must be refused");
    assert_checkpoint_refusal(&validator, "streamed checkpoint validator");

    let run = engine()
        .run_tran_checkpointed(&netlist, 100.0e-9, 1.0e-9)
        .expect_err("a retained final checkpoint of a transient-noise run must be refused");
    assert_checkpoint_refusal(&run, "checkpointed transient run");
}
