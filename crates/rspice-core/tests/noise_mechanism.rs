//! Every mechanism a noise run names must be one a result can be written with.
//!
//! A frontend persists the ranked contributor table verbatim: each row carries
//! its device and its mechanism as text. So the mechanisms this engine emits
//! are part of the saved-result format, and one that falls outside the shape
//! [`is_persistable_noise_mechanism`] describes produces a run that cannot be
//! saved at all.
//!
//! The decks below cover every family that names a mechanism of its own, plus
//! the families that fall back to their broad source type, and each is solved
//! for real rather than fixtured, so the check is against what the engine
//! actually emitted.

use rspice_core::analysis::{IntegratedNoise, is_persistable_noise_mechanism};
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

/// Common band: two decades, enough points for the band integration that the
/// ranked summary is built from.
const FREQUENCIES: [f64; 5] = [1.0e1, 1.0e2, 1.0e3, 1.0e4, 1.0e5];

/// Solve a deck's noise analysis and hand back the ranked summary the
/// frontends persist.
fn ranked_mechanisms(family: &str, deck: &str, output: &str, input: &str) -> Vec<(String, String)> {
    let netlist =
        Netlist::parse(deck).unwrap_or_else(|error| panic!("{family} deck parses: {error}"));
    let results = Engine::new(SimulationConfig::default())
        .run_noise_named_with_input_source(&netlist, output, None, input, &FREQUENCIES, 300.15)
        .unwrap_or_else(|error| panic!("{family} noise analysis runs: {error}"));
    IntegratedNoise::new(results)
        .contribution_summary()
        .into_iter()
        .map(|contribution| (contribution.device_name, contribution.mechanism))
        .collect()
}

fn noise_decks() -> Vec<(&'static str, &'static str, &'static str, &'static str)> {
    vec![
        (
            "MOSFET",
            "* classic level-1 MOSFET noise\n\
             vdd dd 0 dc 5\n\
             rl dd d 10k\n\
             vin g 0 dc 2 ac 1\n\
             m1 d g 0 0 nmod w=10u l=1u\n\
             .model nmod NMOS (LEVEL=1 VTO=1 KP=100u RD=10 RS=10 KF=1e-24 AF=1)\n\
             .end\n",
            "d",
            "vin",
        ),
        (
            "BSIM3",
            "* BSIM3v3 noise\n\
             vdd dd 0 dc 1.8\n\
             rl dd d 10k\n\
             vin g 0 dc 1.2 ac 1\n\
             m1 d g 0 0 n018 w=1u l=0.18u\n\
             .model n018 nmos level=49 noimod=1 kf=1e-24 af=1 rsh=5 hdif=1u\n\
             .end\n",
            "d",
            "vin",
        ),
        (
            "BSIM4",
            "* BSIM4v4.8 noise\n\
             vdd dd 0 dc 1.0\n\
             rl dd d 10k\n\
             vin g 0 dc 0.8 ac 1\n\
             m1 d g 0 0 n45 w=1u l=45n\n\
             .model n45 nmos level=54 version=4.8 fnoimod=1 tnoimod=0\n\
             .end\n",
            "d",
            "vin",
        ),
        (
            "BSIM4 tnoiMod=2",
            "* BSIM4 correlated channel/gate thermal noise\n\
             vdd dd 0 dc 1.0\n\
             rl dd d 10k\n\
             vin g 0 dc 0.8 ac 1\n\
             m1 d g 0 0 n45 w=1u l=45n\n\
             .model n45 nmos level=54 version=4.8 fnoimod=1 tnoimod=2\n\
             .end\n",
            "d",
            "vin",
        ),
        (
            "BJT",
            "* Gummel-Poon bipolar noise\n\
             vcc cc 0 dc 10\n\
             rl cc c 10k\n\
             vin bb 0 dc 0.75 ac 1\n\
             rb bb b 1k\n\
             q1 c b 0 qmod\n\
             .model qmod NPN (IS=1e-16 BF=100 RB=100 RC=10 RE=1 KF=1e-14 AF=1)\n\
             .end\n",
            "c",
            "vin",
        ),
        (
            "VBIC",
            "* VBIC bipolar noise\n\
             vcc cc 0 dc 3\n\
             rl cc c 10k\n\
             vin bb 0 dc 0.8 ac 1\n\
             rbb bb b 1k\n\
             q1 c b 0 vmod\n\
             .model vmod NPN (LEVEL=4 IS=1e-16 NF=1 RCX=10 RBX=50 RE=1 KFN=1e-14 AFN=1 BFN=1)\n\
             .end\n",
            "c",
            "vin",
        ),
        (
            "DIODE",
            "* junction diode noise\n\
             vin in 0 dc 1 ac 1\n\
             r1 in a 1k\n\
             d1 a 0 dmod\n\
             .model dmod D IS=1e-14 N=1.5 KF=1e-16 AF=1\n\
             .end\n",
            "a",
            "vin",
        ),
        (
            "JFET",
            "* junction FET noise\n\
             vdd dd 0 dc 5\n\
             rl dd d 10k\n\
             vin g 0 dc -0.5 ac 1\n\
             j1 d g 0 jmod\n\
             .model jmod NJF (VTO=-2 BETA=1m KF=1e-18 AF=1)\n\
             .end\n",
            "d",
            "vin",
        ),
        (
            "RESISTOR",
            "* resistor thermal and model-card flicker noise\n\
             vin in 0 dc 1 ac 1\n\
             r1 in out rmod 1k\n\
             r2 out 0 2k\n\
             .model rmod R (KF=1e-18 AF=1)\n\
             .end\n",
            "out",
            "vin",
        ),
    ]
}

/// The property that failed: a reader agrees with the emitter about what a
/// mechanism may contain, so nothing the engine names refuses the write.
#[test]
fn every_ranked_mechanism_is_one_a_result_can_be_written_with() {
    for (family, deck, output, input) in noise_decks() {
        let ranked = ranked_mechanisms(family, deck, output, input);
        assert!(
            !ranked.is_empty(),
            "{family} deck contributes no ranked noise; the check would be vacuous"
        );
        for (device, mechanism) in ranked {
            assert!(
                is_persistable_noise_mechanism(&mechanism),
                "{family}: {device} contributes '{mechanism}', which no result can be written with"
            );
        }
    }
}

/// The two families whose per-mechanism identities were refused outright. Their
/// mechanisms come from the device model rather than from the broad source
/// type, so they are the ones a reader restricted to the source-type labels
/// could never take back.
#[test]
fn the_mosfet_and_bipolar_mechanisms_are_the_model_s_own() {
    let decks = noise_decks();
    let expected: [(&str, &[&str]); 2] = [
        ("MOSFET", &["ID", "FN", "RD", "RS"]),
        ("BJT", &["IC", "IB", "FN", "RB", "RC", "RE"]),
    ];

    for (family, mechanisms) in expected {
        let (_, deck, output, input) = decks
            .iter()
            .find(|(name, ..)| *name == family)
            .copied()
            .unwrap_or_else(|| panic!("{family} deck present"));
        let ranked = ranked_mechanisms(family, deck, output, input);
        let emitted = ranked
            .iter()
            .map(|(_, mechanism)| mechanism.as_str())
            .collect::<Vec<_>>();
        for mechanism in mechanisms {
            assert!(
                emitted.contains(mechanism),
                "{family} reports {mechanism}; got {emitted:?}"
            );
        }
    }
}
