//! Xyce TEAM random-telegraph resistance noise must be reproducible across
//! ordinary runs and checkpoint/restart boundaries.

use rspice_core::engine::{
    Engine, SimulationConfig, SpiceDialect, TransientCheckpoint, TransientStartupMode,
};
use rspice_core::netlist::Netlist;

fn deck(seed: i32, enabled: bool) -> Netlist {
    Netlist::parse_validated(&format!(
        "TEAM resistance RTN\n\
         V1 in 0 1\n\
         .model mrm1 memristor level=2 ron=100 roff=200 xon=0 xoff=1 \
         ion=-1 ioff=1 kon=-1 koff=1 alphaon=1 alphaoff=1 wt=0 \
         resnoise={} resseed={seed} reslambda=1 restd=0.7n reseptd=1p \
         resdelta=2 resdeltagrad=0.2\n\
         YMEMRISTOR mr1 in 0 mrm1\n\
         .tran 0.25n 8n\n\
         .end\n",
        u8::from(enabled)
    ))
    .expect("TEAM resistance-noise fixture validates")
}

fn engine() -> Engine {
    Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce))
}

fn resistance_bits(result: &rspice_core::engine::TransientResult) -> Vec<u64> {
    result
        .try_store_waveform_named("YMEMRISTOR!MR1:R")
        .expect("TEAM resistance store trace")
        .iter()
        .map(|value| value.to_bits())
        .collect()
}

#[test]
fn seeded_noise_replays_and_different_seeds_diverge() {
    let engine = engine();
    let first = engine
        .run_tran(&deck(41, true), 8.0e-9, 0.25e-9)
        .expect("first TEAM RTN transient solves");
    let repeat = engine
        .run_tran(&deck(41, true), 8.0e-9, 0.25e-9)
        .expect("repeated TEAM RTN transient solves");
    assert_eq!(first.time, repeat.time);
    assert_eq!(resistance_bits(&first), resistance_bits(&repeat));

    let different = engine
        .run_tran(&deck(42, true), 8.0e-9, 0.25e-9)
        .expect("different-seed TEAM RTN transient solves");
    assert_ne!(
        resistance_bits(&first),
        resistance_bits(&different),
        "different RESSEED values must select different dwell trajectories"
    );
}

#[test]
fn disabled_noise_is_bit_exact_with_the_deterministic_team_model() {
    let baseline = Netlist::parse_validated(
        "TEAM deterministic baseline\n\
         V1 in 0 1\n\
         .model mrm1 memristor level=2 ron=100 roff=200 xon=0 xoff=1 \
         ion=-1 ioff=1 kon=-1 koff=1 alphaon=1 alphaoff=1 wt=0\n\
         YMEMRISTOR mr1 in 0 mrm1\n\
         .tran 0.25n 8n\n\
         .end\n",
    )
    .expect("deterministic TEAM baseline validates");
    let engine = engine();
    let expected = engine
        .run_tran(&baseline, 8.0e-9, 0.25e-9)
        .expect("deterministic TEAM baseline solves");
    let actual = engine
        .run_tran(&deck(99, false), 8.0e-9, 0.25e-9)
        .expect("disabled TEAM RTN transient solves");
    assert_eq!(expected.time, actual.time);
    assert_eq!(resistance_bits(&expected), resistance_bits(&actual));
}

#[test]
fn checkpoint_text_round_trip_resumes_the_exact_noise_suffix() {
    let engine = engine();
    let deck = deck(71, true);
    let (uninterrupted, scheduled) = engine
        .run_tran_checkpoint_schedule_with_startup_mode(
            &deck,
            8.0e-9,
            0.25e-9,
            TransientStartupMode::OperatingPoint,
            &[3.0e-9],
        )
        .expect("TEAM RTN checkpoint trajectory solves");
    let checkpoint = TransientCheckpoint::from_text(&scheduled[0].checkpoint.to_text())
        .expect("TEAM RTN checkpoint round-trips");
    let (resumed, _) = engine
        .run_tran_resume(&deck, &checkpoint, 8.0e-9, 0.25e-9)
        .expect("TEAM RTN checkpoint resumes");
    let seam = uninterrupted
        .time
        .iter()
        .position(|time| time.to_bits() == checkpoint.time.to_bits())
        .expect("checkpoint accepted time is present in uninterrupted result");
    assert_eq!(&uninterrupted.time[seam..], resumed.time.as_slice());
    assert_eq!(
        &resistance_bits(&uninterrupted)[seam..],
        resistance_bits(&resumed).as_slice()
    );
}

#[test]
fn checkpoint_state_version_and_provenance_fail_closed() {
    let engine = engine();
    let deck = deck(11, true);
    let (_, checkpoint) = engine
        .run_tran_checkpointed(&deck, 1.0e-9, 0.25e-9)
        .expect("TEAM RTN checkpoint captures");
    let text = checkpoint.to_text();

    let bad_version = text.replacen(
        "xyce_team_resistance_noise_state 1 ",
        "xyce_team_resistance_noise_state 2 ",
        1,
    );
    let error = TransientCheckpoint::from_text(&bad_version)
        .expect_err("unknown TEAM RTN state versions must fail closed");
    assert!(error.contains("unsupported state version"), "{error}");

    let row = text
        .lines()
        .find(|line| line.starts_with("xyce_team_resistance_noise_state "))
        .expect("TEAM RTN state row");
    let provenance = row
        .split_whitespace()
        .nth(3)
        .expect("TEAM RTN provenance field");
    let wrong_provenance = format!("{:016x}", u64::from_str_radix(provenance, 16).unwrap() ^ 1);
    let tampered = text.replacen(provenance, &wrong_provenance, 1);
    let checkpoint = TransientCheckpoint::from_text(&tampered)
        .expect("a syntactically valid provenance mutation parses");
    let error = engine
        .run_tran_resume(&deck, &checkpoint, 2.0e-9, 0.25e-9)
        .expect_err("TEAM RTN provenance mismatch must reject resume")
        .to_string();
    assert!(error.contains("provenance"), "{error}");
}
