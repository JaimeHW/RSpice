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

/// The property a probe rests on: `DNO(M1)` resolves its argument against the
/// deck's element names, so a ranked row that names anything else is a
/// contribution no probe can reach -- and, because the whole-device query sums
/// the rows that carry the device, a contribution the whole-device answer
/// silently leaves out. Mechanisms belong in the mechanism column; the device
/// column is the instance the netlist spells.
#[test]
fn every_ranked_device_is_one_the_deck_names() {
    for (family, deck, output, input) in noise_decks() {
        let netlist =
            Netlist::parse(deck).unwrap_or_else(|error| panic!("{family} deck parses: {error}"));
        let instances = netlist
            .elements
            .iter()
            .map(|element| element.name.as_str())
            .collect::<Vec<_>>();
        for (device, mechanism) in ranked_mechanisms(family, deck, output, input) {
            assert!(
                instances
                    .iter()
                    .any(|instance| instance.eq_ignore_ascii_case(&device)),
                "{family}: '{device}' contributes '{mechanism}' but is not an element of the                  deck, so no DNO({device}) can reach it; the deck names {instances:?}"
            );
        }
    }
}

/// The families whose per-mechanism identities were refused outright. Their
/// mechanisms come from the device model rather than from the broad source
/// type, so they are the ones a reader restricted to the source-type labels
/// could never take back.
#[test]
fn the_mosfet_and_bipolar_mechanisms_are_the_model_s_own() {
    let decks = noise_decks();
    let expected: [(&str, &[&str]); 6] = [
        ("MOSFET", &["ID", "FN", "RD", "RS"]),
        ("BSIM4", &["ID", "FN"]),
        ("BSIM4 tnoiMod=2", &["ID", "FN", "CORL"]),
        ("BJT", &["IC", "IB", "FN", "RB", "RC", "RE"]),
        (
            "VBIC",
            &[
                "IC", "IBE", "IBEX", "IBEP", "RCX", "RBX", "RE", "RBP", "FN", "FN_BEP",
            ],
        ),
        ("JFET", &["ID", "IGS", "IGD", "FN"]),
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

#[test]
fn classic_mos_signed_noise_reaches_output_and_port_analyses() {
    use rspice_core::analysis::{NoiseContributionProbe, noise::NoisePhysicalConstants};
    use rspice_core::engine::SpiceDialect;
    for dialect in [
        SpiceDialect::BestAvailable,
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
    ] {
        let constants = if dialect == SpiceDialect::Xyce {
            NoisePhysicalConstants::XYCE_7_10
        } else {
            NoisePhysicalConstants::MODERN
        };
        let engine = Engine::new(SimulationConfig::default().with_spice_dialect(dialect));
        for (kind, p) in [("NMOS", 1.0), ("PMOS", -1.0)] {
            for nlev in 0..=3 {
                for (af, ef) in [(-0.5, -0.25), (0.0, 0.0), (1.3, 1.2)] {
                    let make = |kf, port| {
                        let load = if port {
                            format!("VD d 0 {}", p * 2.0)
                        } else {
                            format!("VDD supply 0 {}\nRL supply d 1k", p * 3.0)
                        };
                        Netlist::parse(&format!(
                            "Classic MOS noise transport\n{load}\nVIN g 0 DC {} AC 1\nM1 d g 0 0 mm W=2u L=1u M=5\n.model mm {kind}(VTO={p} KP=100u TOX=20n IS=0 KF={kf} AF={af} EF={ef} NLEV={nlev})\n.options GMIN=0 RELTOL=1e-9 ABSTOL=1e-14 VNTOL=1e-11\n.end\n", p*1.4)).unwrap()
                    };
                    let frequencies = [1000.0, 10_000.0];
                    let output = engine
                        .run_noise_named_with_input_source(
                            &make(1e-24, false),
                            "d",
                            None,
                            "VIN",
                            &frequencies,
                            300.15,
                        )
                        .unwrap();
                    let port = engine
                        .run_port_noise_correlation(
                            &make(1e-24, true),
                            &["VD".into()],
                            &frequencies,
                            300.15,
                        )
                        .unwrap();
                    let quiet = engine
                        .run_port_noise_correlation(
                            &make(0.0, true),
                            &["VD".into()],
                            &frequencies,
                            300.15,
                        )
                        .unwrap();
                    let cox = 3.9 * 8.854_214_871e-12 / 20e-9;
                    for (i, &f) in frequencies.iter().enumerate() {
                        // Independent square-law saturation: Id=16 uA,
                        // gm=80 uS per instance; five independent instances.
                        let expected = if dialect == SpiceDialect::Xyce {
                            5e-24 * (16e-6_f64).powf(af) / (f * 2e-12 * cox * cox)
                        } else {
                            match nlev {
                                0 => 5e-24 * (16e-6_f64).powf(af) / (f.powf(ef) * 1e-12 * cox),
                                1 => 5e-24 * (16e-6_f64).powf(af) / (f.powf(ef) * 2e-12 * cox),
                                _ => 5e-24 * (80e-6_f64).powi(2) / (f.powf(af) * 2e-12 * cox),
                            }
                        };
                        let contribution = |name| {
                            output[i]
                                .contribution(&NoiseContributionProbe::parse(name).unwrap())
                                .unwrap()
                        };
                        let measured = contribution("DNO(M1,FN)") / contribution("DNO(RL)")
                            * (4.0 * constants.boltzmann * 300.15 / 1000.0);
                        let port_measured = port[i].current_correlation[0][0].re
                            - quiet[i].current_correlation[0][0].re;
                        for actual in [measured, port_measured] {
                            assert!(
                                (actual - expected).abs() < expected * 2e-7,
                                "{dialect:?} {kind} NLEV={nlev} AF={af} EF={ef} f={f}: {actual:e} vs {expected:e}"
                            );
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn extreme_mos_flicker_power_reaches_output_and_port_analyses() {
    use rspice_core::analysis::NoiseContributionProbe;
    let mut config = SimulationConfig::default();
    config.convergence_config.gmin_target = 0.0;
    config.convergence_config.junction_gmin_target = 0.0;
    let engine = Engine::new(config);
    for (kind, p) in [("NMOS", 1.0), ("PMOS", -1.0)] {
        // Independent 100-digit evaluation of the square-law Id=16 uA
        // and Cox=3.9*epsilon0/20 nm, with five parallel instances at 1 kHz.
        for (kf, af, expected) in [
            (1e-260, -80.0, 1.355_772_196_661_055_7e136),
            (1e280, 80.0, 6.185_630_138_289_223e-92),
        ] {
            let deck = |port| {
                let load = if port {
                    format!("VD d 0 {}", p * 2.0)
                } else {
                    format!("VDD supply 0 {}\nRL supply d 1k", p * 3.0)
                };
                Netlist::parse(&format!(
                    "Extreme MOS noise\n{load}\nVIN g 0 DC {} AC 1\nM1 d g 0 0 mm W=2u L=1u M=5\n.model mm {kind}(VTO={p} KP=100u TOX=20n IS=0 KF={kf} AF={af} NLEV=0)\n.options GMIN=0 RELTOL=1e-9 ABSTOL=1e-14 VNTOL=1e-11\n.end\n", p*1.4)).unwrap()
            };
            let result = engine
                .run_noise_named_with_input_source(
                    &deck(false),
                    "d",
                    None,
                    "VIN",
                    &[1000.0],
                    300.15,
                )
                .unwrap();
            let density = result[0]
                .contribution(&NoiseContributionProbe::parse("DNO(M1,FN)").unwrap())
                .unwrap();
            // Lambda=0: the 1 kohm load supplies an exact 1e6 V^2/A^2 gain.
            assert!(
                (density / 1e6 - expected).abs() < expected * 5e-11,
                "{kind} AF={af}: output PSD {density:e}, expected source {expected:e}"
            );
            if af < 0.0 {
                let port = engine
                    .run_port_noise_correlation(&deck(true), &["VD".into()], &[1000.0], 300.15)
                    .unwrap();
                // Here flicker is more than 150 decades above thermal noise.
                let density = port[0].current_correlation[0][0].re;
                assert!((density - expected).abs() < expected * 5e-11);
            }
        }
    }
}

#[test]
fn mos1_nlev3_channel_noise_matches_charge_in_triode_and_saturation() {
    use rspice_core::analysis::noise::NoisePhysicalConstants;
    use rspice_core::engine::SpiceDialect;
    for dialect in [
        SpiceDialect::BestAvailable,
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
    ] {
        let mut config = SimulationConfig::default().with_spice_dialect(dialect);
        config.convergence_config.gmin_target = 0.0;
        config.convergence_config.junction_gmin_target = 0.0;
        let engine = Engine::new(config);
        let kb = if dialect == SpiceDialect::Xyce {
            NoisePhysicalConstants::XYCE_7_10.boltzmann
        } else {
            NoisePhysicalConstants::MODERN.boltzmann
        };
        for (kind, p) in [("NMOS", 1.0), ("PMOS", -1.0)] {
            for inverse in [false, true] {
                for body in [-0.5_f64, 0.0, 0.2] {
                    for vds in [0.0_f64, 0.1, 0.4, 2.0] {
                        for nlev in [2, 3] {
                            for gdsnoi in [0.0, 1.0, 10.0] {
                                let netlist = Netlist::parse(&format!(
                                    "MOS channel charge noise\nVD d 0 {}\nVS s 0 {}\nVG g 0 {}\nVB b 0 {}\nM1 d g s b mm W=2u L=1u M=2.5 NF=2\n.model mm {kind}(LEVEL=1 VTO={p} KP=100u LD=0.1u GAMMA=0.4 PHI=0.6 LAMBDA=0.1 IS=0 NLEV={nlev} GDSNOI={gdsnoi})\n.options GMIN=0\n.end\n",
                                    if inverse {0.0} else {p*vds},if inverse {p*vds} else {0.0},p*1.4,p*body)).unwrap();
                                let value = engine
                                    .run_port_noise_correlation(
                                        &netlist,
                                        &["VD".into()],
                                        &[1000.0],
                                        300.15,
                                    )
                                    .unwrap()[0]
                                    .current_correlation[0][0]
                                    .re;
                                let sqrt_phi = 0.6_f64.sqrt();
                                let vth = if body > 0.0 {
                                    1.0 - 0.4 * body / (2.0 * sqrt_phi)
                                } else {
                                    1.0 + 0.4 * ((0.6 - body).sqrt() - sqrt_phi)
                                };
                                let overdrive = 1.4 - vth;
                                let beta = 100e-6 * 2e-6 / 0.8e-6 * 5.0;
                                let conductance = if nlev == 3 && dialect != SpiceDialect::Xyce {
                                    let alpha = if vds >= overdrive {
                                        0.0
                                    } else {
                                        1.0 - vds / overdrive
                                    };
                                    (2.0 / 3.0)
                                        * gdsnoi
                                        * beta
                                        * overdrive
                                        * (1.0 + alpha + alpha * alpha)
                                        / (1.0 + alpha)
                                } else {
                                    (2.0 / 3.0) * beta * vds.min(overdrive) * (1.0 + 0.1 * vds)
                                };
                                let expected = 4.0 * kb * 300.15 * conductance;
                                assert!(
                                    (value - expected).abs() <= expected * 2e-10,
                                    "{dialect:?} {kind} inverse={inverse} body={body} Vds={vds} NLEV={nlev} GDSNOI={gdsnoi}: {value:e} vs {expected:e}"
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn invalid_mos_channel_noise_controls_fail_in_stationary_and_periodic_analyses() {
    for control in [
        "NLEV=3 GDSNOI=-1",
        "NLEV=3 TNOIA=-1",
        "NLEV=3 GAMMA_NOISE=-1",
        "NLEV=0.5",
    ] {
        let netlist=Netlist::parse(&format!(
            "Invalid MOS noise control\nVG g 0 1.4\nVD d 0 2\nM1 d g 0 0 mm\n.model mm NMOS(VTO=1 KP=100u IS=0 {control})\n.end\n")).unwrap();
        let engine = Engine::default();
        let errors = [
            engine
                .run_port_noise_correlation(&netlist, &["VD".into()], &[1000.0], 300.15)
                .unwrap_err(),
            engine
                .run_pnoise(&netlist, 1e6, &[1e4], "d", None, None, 0)
                .unwrap_err(),
        ];
        for error in errors {
            let message = error.to_string();
            assert!(
                message.contains("M1:ID") && message.contains("MOS"),
                "{control}: {message}"
            );
        }
    }
}

#[test]
fn mos_flicker_coefficients_outside_f64_range_preserve_in_band_noise() {
    use rspice_core::analysis::NoiseContributionProbe;
    use rspice_core::engine::SpiceDialect;
    let cox = 3.9 * 8.854_214_871e-12 / 20e-9;
    for dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
        let mut config = SimulationConfig::default().with_spice_dialect(dialect);
        config.convergence_config.gmin_target = 0.0;
        config.convergence_config.junction_gmin_target = 0.0;
        let engine = Engine::new(config);
        for nlev in 0..=3 {
            if dialect == SpiceDialect::Xyce && nlev != 0 {
                continue;
            }
            for (kind, p) in [("NMOS", 1.0), ("PMOS", -1.0)] {
                for (kf, m, exponent) in [(1e308, 5.0, 10), (f64::from_bits(1), 1e-20, -10)] {
                    let current_law = nlev < 2 || dialect == SpiceDialect::Xyce;
                    let af = if !current_law || dialect == SpiceDialect::Xyce {
                        exponent
                    } else {
                        0
                    };
                    let make = |port| {
                        let supply = if port {
                            format!("VD d 0 {}", p * 2.0)
                        } else {
                            format!("VDD supply 0 {}\nRL supply d 1k", p * 3.0)
                        };
                        Netlist::parse(&format!(
                            "MOS coefficient range\n{supply}\nVIN g 0 DC {} AC 1\nM1 d g 0 0 mm W=2u L=1u M={m}\n.model mm {kind}(VTO={p} KP=100u TOX=20n IS=0 KF={kf} AF={af} EF={exponent} NLEV={nlev} GAMMA_NOISE=0)\n.options GMIN=0\n.end\n",p*1.4)).unwrap()
                    };
                    let frequencies = [1e4_f64, 2e4];
                    let scalar = engine
                        .run_noise_named_with_input_source(
                            &make(false),
                            "d",
                            None,
                            "VIN",
                            &frequencies,
                            300.15,
                        )
                        .unwrap();
                    let port = engine
                        .run_port_noise_correlation(
                            &make(true),
                            &["VD".into()],
                            &frequencies,
                            300.15,
                        )
                        .unwrap();
                    for (i, &frequency) in frequencies.iter().enumerate() {
                        // Reorder the independent square-law formula so the
                        // reference never materializes the unrepresentable 1-Hz coefficient.
                        let expected = if dialect == SpiceDialect::Xyce {
                            (kf * (16e-6_f64).powi(af)) / frequency * m / (2e-12 * cox * cox)
                        } else {
                            let frequency_scaled = kf / frequency.powi(exponent);
                            if current_law {
                                frequency_scaled / (if nlev == 0 { 1e-12 } else { 2e-12 } * cox) * m
                            } else {
                                frequency_scaled * (80e-6_f64).powi(2) / (2e-12 * cox) * m
                            }
                        };
                        let contribution = scalar[i]
                            .contribution(&NoiseContributionProbe::parse("DNO(M1,FN)").unwrap())
                            .unwrap();
                        for actual in [port[i].current_correlation[0][0].re, contribution / 1e6] {
                            assert!(expected.is_normal());
                            assert!(
                                (actual - expected).abs() < expected * 3e-11,
                                "{dialect:?} {kind} NLEV={nlev} KF={kf:e} M={m} f={frequency}: {actual:e} vs {expected:e}"
                            );
                        }
                    }
                }
            }
        }
    }
}
