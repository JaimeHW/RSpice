//! Transient noise (`.TRAN … NOISEFMAX=`) beyond the native device catalog.
//!
//! The native mechanisms have their own analytic oracles beside the injection
//! code. This suite owns what a transient-noise run promises about everything
//! else that can carry a noise source, and about the run's own contract:
//!
//! * the run seed is one seed, whatever authored the noise;
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

/// One run, one seed: an authored `TRNOISE` source draws from `NOISESEED=`
/// when the transient card names one, exactly as device noise does, and
/// from `.OPTIONS SEED` otherwise.
#[test]
fn trnoise_draws_from_the_transient_cards_noise_seed() {
    // An ideal source pins `in` to the TRNOISE value, so the resistor's own
    // injected thermal current cannot move the observed waveform.
    let waveform = |options: &str, card_seed: &str| {
        let deck = format!(
            "trnoise seed\n\
             {options}\n\
             V1 in 0 TRNOISE(1m 1n 0 0)\n\
             R1 in 0 1k\n\
             .TRAN 1n 200n NOISEFMAX=500meg {card_seed}\n\
             .END\n"
        );
        let netlist = Netlist::parse(&deck).expect("the deck parses");
        let result = engine()
            .run_tran(&netlist, 200.0e-9, 1.0e-9)
            .expect("the run converges");
        let values = result
            .try_voltage_waveform_named("in")
            .expect("the source node is retained")
            .to_vec();
        assert!(
            values.iter().any(|value| *value != 0.0),
            "the TRNOISE source must draw"
        );
        (result.time, values)
    };

    let card = waveform("", "NOISESEED=42");
    let options = waveform(".OPTIONS SEED=42", "");
    let both = waveform(".OPTIONS SEED=7", "NOISESEED=42");
    let other = waveform(".OPTIONS SEED=7", "");

    assert_eq!(
        card, options,
        "NOISESEED=42 and .OPTIONS SEED=42 name the same run seed"
    );
    assert_eq!(
        both, card,
        "the transient card's NOISESEED wins over .OPTIONS SEED"
    );
    assert_ne!(
        other.1, card.1,
        "a different run seed must draw a different TRNOISE realization"
    );
}
