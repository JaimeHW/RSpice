//! Periodic-noise validation: stationary parity and analytic folding.
//!
//! 1. With no large-signal drive the periodic operating point is the DC
//!    point and every modulated intensity is constant, so pnoise must
//!    reproduce the ordinary .noise analysis at the same frequencies.
//! 2. A resistor chopped by an ideal switch is the classic LTV noise
//!    problem: for memoryless modulation the output PSD of each stationary
//!    source is its intensity times the time-average squared transfer,
//!    computable in closed form from the two switch states.

use rspice_core::abort_signal::NoAbort;
use rspice_core::analysis::harmonic_balance::HbConfig;
use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect};
use rspice_core::netlist::Netlist;

const K_B: f64 = 1.380649e-23;
const T_REF: f64 = 300.15;

#[test]
fn direct_pnoise_resolves_deck_temperature() {
    let resistance = 2.0e3;
    let temperature = 400.0;
    let netlist = Netlist::parse(
        "deck-temperature pnoise\n\
         r1 out 0 2k\n\
         .options temp=126.85\n\
         .end\n",
    )
    .expect("temperature deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_pnoise(&netlist, 1.0e6, &[1.0e4], "out", None, None, 0)
        .expect("direct pnoise resolves deck temperature");
    let expected = 4.0 * K_B * temperature * resistance;
    assert!(
        (result.output_noise[0] - expected).abs() <= 1.0e-12 * expected,
        "deck TEMP must set periodic thermal noise: got {:.6e}, want {expected:.6e}",
        result.output_noise[0]
    );
}

#[test]
fn direct_pnoise_applies_hb_local_options_and_accepts_typed_initializers() {
    let base = "HB-local pnoise gate\nr1 out 0 1k\n.end\n";
    let mut zero_budget = Netlist::parse(base).expect("base deck parses");
    zero_budget.options.nonlin_hb_maxstep = Some(0);
    let error = Engine::new(SimulationConfig::default())
        .run_pnoise(&zero_budget, 1.0e6, &[1.0e4], "out", None, None, 0)
        .expect_err("a zero NONLIN-HB MAXSTEP must fail at the pnoise boundary");
    assert!(error.to_string().contains("MAXSTEP must be at least 1"));

    let dc_tahb = Netlist::parse(&base.replace(".end", ".options hbint tahb=2\n.end"))
        .expect("typed TAHB deck parses");
    Engine::new(SimulationConfig::default())
        .run_pnoise(&dc_tahb, 1.0e6, &[1.0e4], "out", None, None, 0)
        .expect("a supported HB initializer remains analysis-local for direct pnoise");
}

#[test]
fn pnoise_preserves_dc_and_rejects_negative_or_nonfinite_offsets() {
    let netlist = Netlist::parse("* offset validation\nr1 out 0 1k\n.end\n").expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());

    let dc = engine
        .run_pnoise(&netlist, 1.0e6, &[0.0], "out", None, None, 0)
        .expect("driven pnoise supports the DC offset used by linear sweeps");
    assert_eq!(dc.frequencies, vec![0.0]);
    assert!(dc.output_noise[0].is_finite() && dc.output_noise[0] >= 0.0);

    for offset in [-1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let error = engine
            .run_pnoise(&netlist, 1.0e6, &[offset], "out", None, None, 0)
            .expect_err("an invalid offset must fail before a periodic solve");
        let message = error.to_string();
        assert!(
            message.contains("offsets[0]") && message.contains("finite and non-negative"),
            "invalid-offset failure must identify the value and contract: {message}"
        );
    }
}

#[test]
fn high_resistance_pnoise_is_exactly_four_k_t_r() {
    let resistance = 1.0e12;
    let deck = "\
* An implicit 1 pS shunt would suppress this result by four.
r1 out 0 1e12
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_pnoise(&netlist, 1.0e6, &[1.0e4], "out", None, None, 0)
        .expect("pnoise completes");
    let expected = 4.0 * K_B * T_REF * resistance;

    assert!(
        (result.output_noise[0] - expected).abs() <= 1.0e-12 * expected,
        "resistor output noise must be 4kTR: got {:.6e}, want {expected:.6e}",
        result.output_noise[0]
    );
}

#[test]
fn pnoise_rshunt_is_one_physical_source_per_electrical_node_and_uses_dialect_constants() {
    let resistance = 1.0e3;
    let netlist = Netlist::parse(
        "physical RSHUNT pnoise\n\
         i1 out 0 dc 0\n\
         .options rshunt=1k\n\
         .end\n",
    )
    .expect("RSHUNT deck parses");
    for (dialect, boltzmann) in [
        (SpiceDialect::Ngspice, rspice_core::constants::K_BOLTZMANN),
        (SpiceDialect::Xyce, rspice_core::constants::XYCE_K_BOLTZMANN),
    ] {
        let result = Engine::new(SimulationConfig::default().with_spice_dialect(dialect))
            .run_pnoise(&netlist, 1.0e6, &[0.0], "out", None, None, 0)
            .expect("RSHUNT pnoise completes");
        let expected = 4.0 * boltzmann * T_REF * resistance;
        assert_eq!(
            result.contributors.len(),
            1,
            "one electrical node has one shunt"
        );
        assert_eq!(
            result.contributors[0].0.to_ascii_lowercase(),
            "rshunt:out thermal"
        );
        assert!(
            (result.output_noise[0] - expected).abs() <= 2.0e-12 * expected,
            "{dialect:?} RSHUNT output noise: got {:.6e}, want {expected:.6e}",
            result.output_noise[0]
        );
        assert!(
            result
                .contributors
                .iter()
                .all(|(name, _)| !name.to_ascii_uppercase().contains("GMIN")),
            "numerical GMIN must never enter the physical source catalog"
        );
    }
}

#[test]
fn retained_pnoise_preserves_modulation_above_the_conversion_window() {
    for branch_form in [false, true] {
        let option = if branch_form {
            ".options device zeroresistancetol=2\n"
        } else {
            ""
        };
        let deck = Netlist::parse(&format!(
            "High-harmonic flicker\nI1 0 out SIN(0 1 10)\nR1 out 0 RM 1\n.model RM R(KF=1 AF=2 EF=1)\n{option}.end\n"
        )).unwrap();
        let engine = Engine::default();
        let hb = engine
            .run_hb(&deck, HbConfig::new(1.0).with_harmonics(16))
            .unwrap();
        for sidebands in [0, 2] {
            let result = engine
                .run_pnoise_from_hb_with_abort(
                    &deck,
                    &[0.25],
                    "out",
                    None,
                    None,
                    sidebands,
                    &hb.operating_point,
                    &NoAbort,
                )
                .unwrap();
            let actual = result
                .contributors
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("r1 flicker"))
                .unwrap()
                .1[0];
            let expected = 0.25 * (1.0 / 9.75 + 1.0 / 10.25);
            assert!(
                (actual / expected - 1.0).abs() < 2e-12,
                "branch={branch_form}, K={sidebands}: {actual} vs {expected}"
            );
        }
    }
}

#[test]
fn pnoise_flicker_sums_sidebands_before_rounding() {
    let small_amplitude = 2.0_f64.powi(-26);
    for (bias, first, second, kf, expected) in [
        (0.0, 1.0, 0.0, f64::from_bits(2), f64::from_bits(1)),
        (
            1.0,
            small_amplitude,
            small_amplitude,
            1.0,
            1.0 + f64::EPSILON,
        ),
    ] {
        let deck = Netlist::parse(&format!(
            "Flicker sideband rounding\nI1 0 out SIN({bias} {first} 1)\nI2 0 out SIN(0 {second} 2)\nR1 out 0 RM 1\n.model RM R(KF={kf} AF=2 EF=0)\n.end\n"
        )).unwrap();
        let result = Engine::default()
            .run_pnoise(&deck, 1.0, &[0.25], "out", None, None, 0)
            .unwrap();
        let actual = result
            .contributors
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("r1 flicker"))
            .unwrap()
            .1[0];
        // With EF=0, Parseval gives KF*(bias^2 + first^2/2 + second^2/2).
        // The first case sums two half-subnormal powers; the second recovers
        // four quarter-ulp powers beside the DC term.
        assert_eq!(
            actual.to_bits(),
            expected.to_bits(),
            "KF={kf}, bias={bias}: {actual:e} vs {expected:e}"
        );
    }
}

#[test]
fn pnoise_resistor_flicker_retains_coefficient_scale() {
    for (m, kf, frequency, tolerance) in
        [(1e300, 1e-300, 1e-300, 1.0), (1e-200, 1e200, 1e200, 1e201)]
    {
        for branch_form in [false, true] {
            let tolerance = if branch_form { tolerance } else { 0.0 };
            let deck = Netlist::parse(&format!("Scaled periodic flicker\nI1 0 out {m}\nR1 out 0 RM 1 M={m}\n.model RM R(KF={kf} AF=2 EF=2)\n.options device zeroresistancetol={tolerance}\n.end\n")).unwrap();
            let result = Engine::default()
                .run_pnoise(&deck, 1000.0, &[frequency], "out", None, None, 0)
                .unwrap();
            let flicker = result
                .contributors
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("r1 flicker"))
                .unwrap()
                .1[0];
            assert!(
                (flicker - 1.0).abs() < 2e-12,
                "M={m}, branch={branch_form}: {flicker}"
            );
        }
    }
}

#[test]
fn pnoise_resistor_flicker_retains_signed_current_modulation() {
    let offsets = [250.0_f64, 1250.0];
    for branch_form in [false, true] {
        for (dc_current, multiplicity) in
            [(0.0_f64, 1.0_f64), (0.0002, 1.0), (0.0, 5.0), (0.0002, 5.0)]
        {
            let option = if branch_form {
                ".options device zeroresistancetol=2000\n"
            } else {
                ""
            };
            let netlist = Netlist::parse(&format!(
                "signed resistor flicker\ni1 0 out SIN({dc_current} 1m 1k)\nr1 out 0 rm 1k AC=2k M={multiplicity}\n.model rm R (KF=1e-12 AF=2 EF=1)\n{option}.end\n"
            )).unwrap();
            for sidebands in [0, 2] {
                let result = Engine::new(SimulationConfig::default())
                    .run_pnoise(&netlist, 1000.0, &offsets, "out", None, None, sidebands)
                    .expect("AF=2 resistor flicker has exact signed periodic modulation");
                let flicker = &result
                    .contributors
                    .iter()
                    .find(|(name, _)| name.eq_ignore_ascii_case("r1 flicker"))
                    .expect("authored flicker contributor")
                    .1;
                for (index, &frequency) in offsets.iter().enumerate() {
                    // Fluctuating resistance times the signed bias current:
                    // the sinusoid translates noise to f +/- f0; rectifying
                    // it would invent a low-frequency 1/f contribution.
                    let expected = 1e-12 * 4e6 / multiplicity.powi(3)
                        * (dc_current * dc_current / frequency
                            + 0.25e-6
                                * (1.0 / (frequency - 1000.0).abs() + 1.0 / (frequency + 1000.0)));
                    assert!(
                        (flicker[index] / expected - 1.0).abs() < 2e-12,
                        "branch={branch_form}, DC={dc_current}, K={sidebands}, f={frequency}: {} vs {expected}",
                        flicker[index]
                    );
                    let total = expected + 4.0 * K_B * T_REF * 2000.0 / multiplicity;
                    assert!((result.output_noise[index] / total - 1.0).abs() < 2e-12);
                }
            }
        }
    }
}

#[test]
fn pnoise_rejects_active_device_colored_controls_but_accepts_exact_zero() {
    let cases = [
        (
            "resistor",
            "r1 out 0 rm 1k\n.model rm R (KF=1e-18 AF=1 EF=1)",
            "r1 out 0 rm 1k\n.model rm R (KF=0 AF=1 EF=1)",
            "out",
            "r1",
        ),
        (
            "diode",
            "v1 in 0 1\nr1 in out 1k\nd1 out 0 dm\n.model dm D (IS=1e-12 KF=1e-18 AF=1)",
            "v1 in 0 1\nr1 in out 1k\nd1 out 0 dm\n.model dm D (IS=1e-12 KF=0 AF=1)",
            "out",
            "d1",
        ),
        (
            "MOSFET",
            "vdd vdd 0 5\nvg g 0 1.5\nrd vdd d 10k\nm1 d g 0 0 mm w=20u l=2u\n.model mm NMOS (LEVEL=1 VTO=1 KP=60u KF=1e-24 AF=1)",
            "vdd vdd 0 5\nvg g 0 1.5\nrd vdd d 10k\nm1 d g 0 0 mm w=20u l=2u\n.model mm NMOS (LEVEL=1 VTO=1 KP=60u KF=0 AF=1)",
            "d",
            "m1",
        ),
        (
            "JFET",
            "vdd vdd 0 5\nvg g 0 -0.5\nrd vdd d 10k\nj1 d g 0 jm\n.model jm NJF (VTO=-2 BETA=1m KF=1e-18 AF=1)",
            "vdd vdd 0 5\nvg g 0 -0.5\nrd vdd d 10k\nj1 d g 0 jm\n.model jm NJF (VTO=-2 BETA=1m KF=0 AF=1)",
            "d",
            "j1",
        ),
    ];
    let engine = Engine::new(SimulationConfig::default());
    for (mechanism, active_body, zero_body, output, instance) in cases {
        let active = Netlist::parse(&format!(
            "active colored {mechanism}\n{active_body}\n.end\n"
        ))
        .expect("active colored deck parses");
        let error = engine
            .run_pnoise(&active, 1.0e6, &[1.0e4], output, None, None, 0)
            .expect_err("periodically bias-dependent colored noise must fail closed");
        let message = error.to_string();
        assert!(
            message.contains("cyclostationary colored-noise")
                && message.to_ascii_lowercase().contains(instance),
            "{mechanism} rejection must identify the exact instance and mechanism: {message}"
        );

        let zero = Netlist::parse(&format!("zero colored {mechanism}\n{zero_body}\n.end\n"))
            .expect("exact-zero colored deck parses");
        engine
            .run_pnoise(&zero, 1.0e6, &[1.0e4], output, None, None, 0)
            .unwrap_or_else(|error| {
                panic!("exact-zero {mechanism} control must remain accepted: {error}")
            });
    }
}

#[test]
fn pnoise_names_and_models_finite_branch_form_resistor_noise() {
    let netlist = Netlist::parse(
        "near-zero branch resistor pnoise\n\
         v1 in 0 1\n\
         Rtiny in out 0.6 TEMP=50 DTEMP=10 NOISY=1\n\
         rload out 0 1k\n\
         .options device zeroresistancetol=1\n\
         .end\n",
    )
    .expect("near-zero branch-resistor deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_pnoise(&netlist, 1.0e6, &[1.0e4], "out", None, None, 0)
        .expect("finite branch-form resistor noise is represented exactly");
    assert!(
        result
            .contributors
            .iter()
            .any(|(name, values)| name.eq_ignore_ascii_case("Rtiny thermal")
                && values[0].is_finite()
                && values[0] >= 0.0),
        "branch-form thermal contributor must preserve its authored identity: {:?}",
        result.contributors
    );
}

#[test]
fn pnoise_resistor_thermal_density_preserves_extreme_scaling() {
    let resistance = 1.0e154;
    let temperature = 1.0e-150;
    let netlist =
        Netlist::parse("* scaled resistor density\nr1 out 0 1e154\n.end\n").expect("deck parses");
    let config = SimulationConfig {
        temperature,
        ..SimulationConfig::default()
    };
    let result = Engine::new(config)
        .run_pnoise(&netlist, 1.0e6, &[1.0e4], "out", None, None, 0)
        .expect("scaled thermal source and transfer remain representable");
    let expected = 4.0 * K_B * temperature * resistance;

    assert!(
        (result.output_noise[0] - expected).abs() <= 2.0e-12 * expected,
        "scaled 4kTR must survive an unrepresentable current-source PSD: got {:.6e}, want {expected:.6e}",
        result.output_noise[0]
    );
}

#[test]
fn pnoise_resistor_dtemp_matches_its_absolute_noise_temperature() {
    let resistance = 10.0e3;
    let dtemp = 150.0;
    let netlist =
        Netlist::parse("* resistor DTEMP\nr1 out 0 10k dtemp=150\n.end\n").expect("deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_pnoise(&netlist, 1.0e6, &[1.0e4], "out", None, None, 0)
        .expect("resistor DTEMP pnoise completes");
    let expected = 4.0 * K_B * (T_REF + dtemp) * resistance;

    assert!(
        (result.output_noise[0] - expected).abs() <= 1.0e-12 * expected,
        "resistor DTEMP must heat periodic thermal noise: got {:.6e}, want {expected:.6e}",
        result.output_noise[0]
    );
}

#[test]
fn pnoise_resistor_temp_survives_extreme_ambient_and_outranks_dtemp() {
    let resistance = 10.0e3;
    let deck = "\
* Resistor TEMP provenance under extreme ambient
.options tnom=27
r1 out 0 10k rm temp=27 dtemp=-1000
.model rm R (tnom=100)
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let run = |temperature| {
        Engine::new(SimulationConfig {
            temperature,
            ..SimulationConfig::default()
        })
        .run_pnoise(&netlist, 1.0e6, &[1.0e4], "out", None, None, 0)
        .expect("resistor TEMP pnoise completes")
        .output_noise[0]
    };
    let ordinary = run(T_REF);
    let extreme = run(1.0e20);
    // ngspice resnoise.c resolves authored TEMP as TEMP_K + model TNOM_C.
    let expected = 4.0 * K_B * (T_REF + 100.0) * resistance;
    assert!(
        (ordinary - expected).abs() <= 1.0e-12 * expected,
        "resistor TEMP must retain ngspice's resolved absolute source temperature: got {ordinary:.6e}, want {expected:.6e}"
    );
    assert_eq!(
        extreme.to_bits(),
        ordinary.to_bits(),
        "resistor TEMP must not be reconstructed through a lossy ambient-relative offset"
    );
}

#[test]
fn pnoise_mos_dtemp_matches_ambient_while_inexact_jfet_scaling_fails_closed() {
    let run_contributors = |deck: &str, temperature: f64| {
        let netlist = Netlist::parse(deck).expect("device deck parses");
        let config = SimulationConfig {
            temperature,
            ..SimulationConfig::default()
        };
        let result = Engine::new(config)
            .run_pnoise(&netlist, 1.0e6, &[1.0e4], "d", None, None, 0)
            .expect("stationary device pnoise completes");
        result.contributors
    };
    let contribution = |contributors: &[(String, Vec<f64>)], label: &str| {
        let value = contributors
            .iter()
            .find(|(name, _)| {
                name.to_ascii_lowercase()
                    .contains(&label.to_ascii_lowercase())
            })
            .map(|(_, values)| values[0])
            .unwrap_or_else(|| panic!("missing channel contributor '{label}': {:?}", contributors));
        assert!(
            value.is_finite() && value > 0.0,
            "channel contributor '{label}' must be finite and strictly positive, got {value:.6e}"
        );
        value
    };

    let mos_ambient = "\
* MOS periodic-noise temperature equivalence
vdd vdd 0 dc 5
vg g 0 dc 1.5
rd vdd d 10k
m1 d g 0 0 nm w=20u l=2u
.model nm nmos level=1 vto=1 kp=60u lambda=0.02 rd=75 rs=50
.end
";
    let mos_dtemp = mos_ambient.replace(
        "m1 d g 0 0 nm w=20u l=2u",
        "m1 d g 0 0 nm w=20u l=2u dtemp=150",
    );
    let mos_hot_contributors = run_contributors(mos_ambient, T_REF + 150.0);
    let mos_offset_contributors = run_contributors(&mos_dtemp, T_REF);
    let mos_hot = contribution(&mos_hot_contributors, "m1 channel thermal");
    let mos_offset = contribution(&mos_offset_contributors, "m1 channel thermal");
    assert!(
        (mos_offset - mos_hot).abs() <= 1.0e-10 * mos_hot,
        "MOS DTEMP channel noise must equal the same absolute ambient temperature: {mos_offset:.6e} vs {mos_hot:.6e}"
    );
    let mos_temp_priority = mos_ambient.replace(
        "m1 d g 0 0 nm w=20u l=2u",
        "m1 d g 0 0 nm w=20u l=2u temp=150 dtemp=-1000",
    );
    let mos_absolute_contributors = run_contributors(mos_ambient, 423.15);
    let mos_priority_contributors = run_contributors(&mos_temp_priority, T_REF);
    let mos_extreme_contributors = run_contributors(&mos_temp_priority, 1.0e20);
    let mos_absolute = contribution(&mos_absolute_contributors, "m1 channel thermal");
    let mos_priority = contribution(&mos_priority_contributors, "m1 channel thermal");
    let mos_extreme_ambient = contribution(&mos_extreme_contributors, "m1 channel thermal");
    assert!(
        (mos_priority - mos_absolute).abs() <= 1.0e-10 * mos_absolute,
        "MOS TEMP must set the absolute channel-noise temperature and outrank DTEMP: {mos_priority:.6e} vs {mos_absolute:.6e}"
    );
    assert_eq!(
        mos_extreme_ambient.to_bits(),
        mos_priority.to_bits(),
        "MOS TEMP must not be reconstructed through a lossy ambient-relative offset"
    );
    for label in ["m1.__rd thermal", "m1.__rs thermal"] {
        let ordinary = contribution(&mos_priority_contributors, label);
        let extreme = contribution(&mos_extreme_contributors, label);
        assert_eq!(
            extreme.to_bits(),
            ordinary.to_bits(),
            "MOS {label} must retain the parent device's absolute TEMP"
        );
    }

    let jfet_ambient = "\
* JFET periodic-noise temperature equivalence
vdd vdd 0 dc 12
vg g 0 dc -0.5
rd vdd d 2k
j1 d g 0 jn
.model jn njf vto=-2 beta=1m lambda=0.01 rd=75 rs=50
.end
";
    let jfet_dtemp = jfet_ambient.replace("j1 d g 0 jn", "j1 d g 0 jn dtemp=150");
    let jfet_temp = jfet_ambient.replace("j1 d g 0 jn", "j1 d g 0 jn temp=150 dtemp=-1000");
    for (deck, temperature) in [
        (jfet_ambient, T_REF + 150.0),
        (jfet_dtemp.as_str(), T_REF),
        (jfet_temp.as_str(), T_REF),
    ] {
        let netlist = Netlist::parse(deck).expect("temperature-scaled JFET deck parses");
        let error = Engine::new(SimulationConfig {
            temperature,
            ..SimulationConfig::default()
        })
        .run_pnoise(&netlist, 1.0e6, &[1.0e4], "d", None, None, 0)
        .expect_err("PNoise must not publish a JFET state with incomplete temperature scaling");
        assert!(
            error.to_string().contains("temperature-scaled"),
            "JFET temperature capability failure must be explicit: {error}"
        );
    }
}

#[test]
fn pnoise_rejects_nonphysical_mos_and_jfet_instance_temperatures() {
    let decks = [
        (
            "MOSFET",
            "M1",
            "vdd vdd 0 5\nvg g 0 1.5\nrd vdd d 10k\nm1 d g 0 0 nm temp=-273.15\n.model nm nmos level=1 vto=1 kp=60u\n.end\n",
        ),
        (
            "JFET",
            "J1",
            "vdd vdd 0 12\nvg g 0 -0.5\nrd vdd d 2k\nj1 d g 0 jn temp=-273.15\n.model jn njf vto=-2 beta=1m\n.end\n",
        ),
        (
            "MOSFET",
            "M1",
            "vdd vdd 0 5\nvg g 0 1.5\nrd vdd d 10k\nm1 d g 0 0 nm dtemp=-400\n.model nm nmos level=1 vto=1 kp=60u\n.end\n",
        ),
        (
            "JFET",
            "J1",
            "vdd vdd 0 12\nvg g 0 -0.5\nrd vdd d 2k\nj1 d g 0 jn dtemp=-400\n.model jn njf vto=-2 beta=1m\n.end\n",
        ),
        (
            "MESFET",
            "Z1",
            "vdd vdd 0 12\nvg g 0 -0.5\nrd vdd d 2k\nz1 d g 0 zm temp=-273.15\n.model zm nmf vto=-2 beta=1m\n.end\n",
        ),
    ];
    for (kind, device_name, deck) in decks {
        let netlist = Netlist::parse(deck).expect("invalid-temperature deck parses");
        let error = Engine::new(SimulationConfig::default())
            .run_pnoise(&netlist, 1.0e6, &[1.0e4], "d", None, None, 0)
            .expect_err("non-positive absolute device temperature must fail");
        let message = error.to_string();
        assert!(
            message.contains(kind)
                && message.contains(device_name)
                && message.contains("finite and positive"),
            "invalid {kind} temperature failure must be contextual: {message}"
        );
    }
}

#[test]
fn pnoise_without_large_signal_drive_matches_stationary_noise() {
    // Forward-biased diode divider: thermal (R1) plus shot (D1) noise with
    // frequency shaping from the 1 nF capacitor.
    let deck = "\
* stationary parity network (thermal + shot)
v1 in 0 dc 2
r1 in mid 10k
d1 mid 0 dmod
c1 mid 0 1n
.model dmod D IS=1e-12 N=1.0 CJ0=0 TT=0 RS=0 KF=0 AF=1
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let offsets = [1.0e2, 1.0e3, 1.0e5, 1.0e7];

    let pnoise = engine
        .run_pnoise(&netlist, 1.0e6, &offsets, "mid", None, None, 6)
        .expect("pnoise completes");
    let carrier_only = engine
        .run_pnoise(&netlist, 1.0e6, &offsets, "mid", None, None, 0)
        .expect("carrier-only pnoise completes without silently adding sidebands");
    let invalid = engine
        .run_pnoise(&netlist, 1.0e6, &offsets, "mid", None, None, -1)
        .expect_err("negative sideband bounds must fail closed");
    assert!(
        invalid.to_string().contains("non-negative"),
        "negative sideband failure should identify the invalid bound: {invalid}"
    );

    // Reference: the stationary noise analysis at the same frequencies.
    let dc = engine.run_dc_op(&netlist).expect("dc op");
    // run_dc_op node names include ground at index 0, so the position is
    // already the matrix node index run_noise_ports expects.
    let mid_idx = dc
        .node_names
        .iter()
        .position(|n| n.eq_ignore_ascii_case("mid"))
        .expect("mid node");
    let stationary = engine
        .run_noise_ports(&netlist, mid_idx, None, &offsets, T_REF)
        .expect("stationary noise completes");

    for (i, &freq) in offsets.iter().enumerate() {
        let folded = pnoise.output_noise[i];
        let reference = stationary[i].output_noise_density;
        assert!(
            (folded - reference).abs() < 0.03 * reference,
            "at {freq:.1e} Hz pnoise must match stationary noise: \
             {folded:.4e} vs {reference:.4e} V^2/Hz"
        );
        let carrier = carrier_only.output_noise[i];
        assert!(
            (carrier - reference).abs() < 0.03 * reference,
            "at {freq:.1e} Hz carrier-only pnoise must match stationary noise: \
             {carrier:.4e} vs {reference:.4e} V^2/Hz"
        );
    }
}

#[test]
fn chopped_resistor_noise_folds_to_the_time_average_transfer() {
    // 50% chopper between two 1k resistors. Closed form per source
    // (time-average squared transfer of the two switch states):
    //   R1 (source side): on Z = R1*(ron+R2)/(R1+ron+R2), off ~0
    //   R2 (output side): on Z = R2*(ron+R1)/(R1+ron+R2), off Z = R2
    // Switch ron thermal contributes ~4kT*0.25, negligible but modeled.
    let deck = "\
* chopped resistor noise
vlo ctl 0 sin(0 1 1meg)
r1 src 0 1k
s1 src out ctl 0 swmod
r2 out 0 1k
c1 out 0 1f
.model swmod sw vt=0 ron=1 roff=1e9 smooth=1m
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());

    let result = engine
        .run_pnoise(&netlist, 1.0e6, &[1.0e4], "out", None, None, 12)
        .expect("pnoise completes");
    assert!(result.converged, "operating point must converge");

    let (r1, r2, ron) = (1000.0, 1000.0, 1.0);
    let loop_r = r1 + ron + r2;
    let z_r1_on = r1 * (ron + r2) / loop_r;
    let z_r2_on = r2 * (ron + r1) / loop_r;
    let z_sw_on = ron * r2 / loop_r; // parallel current source across ron

    let s_r1 = 4.0 * K_B * T_REF / r1 * 0.5 * z_r1_on * z_r1_on;
    let s_r2 = 4.0 * K_B * T_REF / r2 * 0.5 * (z_r2_on * z_r2_on + r2 * r2);
    let s_sw = 4.0 * K_B * T_REF / ron * 0.5 * z_sw_on * z_sw_on;
    let expected = s_r1 + s_r2 + s_sw;

    let got = result.output_noise[0];
    assert!(
        (got - expected).abs() < 0.04 * expected,
        "chopped-resistor output noise must fold to the time-average \
         transfer: got {got:.4e}, want {expected:.4e} V^2/Hz; contributors: {:?}",
        result.contributors
    );
}

/// Per-source contributions must decompose the total exactly (independent
/// sources), so the contributor list is a true breakdown rather than an
/// estimate.
#[test]
fn pnoise_contributors_sum_to_the_total() {
    let deck = "\
* contributor decomposition network
v1 in 0 dc 2
r1 in mid 10k
d1 mid 0 dmod
c1 mid 0 1n
.model dmod D IS=1e-12 N=1.0 CJ0=0 TT=0 RS=0
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let offsets = [1.0e4, 1.0e6];

    let result = engine
        .run_pnoise(&netlist, 1.0e6, &offsets, "mid", None, None, 6)
        .expect("pnoise completes");

    assert!(
        !result.contributors.is_empty(),
        "thermal and shot contributors must be reported"
    );
    for (i, &total) in result.output_noise.iter().enumerate() {
        assert!(total.is_finite() && total >= 0.0);
        for (_, psds) in &result.contributors {
            assert_eq!(psds.len(), offsets.len());
            assert!(psds.iter().all(|value| value.is_finite() && *value >= 0.0));
        }
        let sum: f64 = result.contributors.iter().map(|(_, psds)| psds[i]).sum();
        assert!(
            (sum - total).abs() <= 1e-12 * total.max(1e-300),
            "contributors must sum to the total at offset {}: {sum:.6e} vs {total:.6e}",
            offsets[i]
        );
    }
}

/// Input-referred pnoise divides the output PSD by the squared conversion
/// transfer from the input source. With no LO and a linear divider both
/// pieces are closed-form: H(f) = (R2 || Zc) / (R1 + R2 || Zc).
#[test]
fn input_referred_pnoise_matches_the_closed_form_transfer() {
    let deck = "\
* linear divider for input-referred check
vin in 0 dc 0
r1 in mid 10k
r2 mid 0 10k
c1 mid 0 1n
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let offsets = [1.0e3, 1.0e5];

    let result = engine
        .run_pnoise(&netlist, 1.0e6, &offsets, "mid", None, Some("vin"), 6)
        .expect("pnoise completes");
    let input_noise = result.input_noise.expect("input-referred present");

    let (r1, r2, c) = (10.0e3, 10.0e3, 1.0e-9);
    for (i, &f) in offsets.iter().enumerate() {
        let w = 2.0 * std::f64::consts::PI * f;
        let zc = num_complex::Complex64::new(0.0, -1.0 / (w * c));
        let z2 = (num_complex::Complex64::new(r2, 0.0) * zc)
            / (num_complex::Complex64::new(r2, 0.0) + zc);
        let h = z2 / (num_complex::Complex64::new(r1, 0.0) + z2);
        let expected = result.output_noise[i] / h.norm_sqr();
        assert!(
            (input_noise[i] - expected).abs() < 0.01 * expected,
            "input-referred noise at {f:.0e} Hz must be output/|H|^2: \
             got {:.4e}, want {expected:.4e}",
            input_noise[i]
        );
    }
}

#[test]
fn pnoise_uses_the_exact_voltage_source_transfer_for_input_referral() {
    let resistance = 1.0e-6;
    let deck = "\
* exact voltage input and low-impedance noisy divider
vin in 0 dc 0 ac 7 37
r1 in out 1u
r2 out 0 1u
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_pnoise(&netlist, 1.0e6, &[1.0e4], "out", None, Some("vin"), 0)
        .expect("pnoise completes");

    let expected_output = 2.0 * K_B * T_REF * resistance;
    let expected_input = 8.0 * K_B * T_REF * resistance;
    let input_noise = result
        .input_noise
        .expect("input-referred result is present");
    assert!(
        (result.output_noise[0] - expected_output).abs() <= 1.0e-10 * expected_output,
        "exact divider output noise = {:.6e}, expected {expected_output:.6e}",
        result.output_noise[0]
    );
    assert!(
        (input_noise[0] - expected_input).abs() <= 1.0e-10 * expected_input,
        "exact divider input-referred noise = {:.6e}, expected {expected_input:.6e}",
        input_noise[0]
    );
    for (name, contribution) in &result.contributors {
        let expected_contribution = K_B * T_REF * resistance;
        assert!(
            (contribution[0] - expected_contribution).abs() <= 1.0e-10 * expected_contribution,
            "{name} output contribution = {:.6e}, expected {expected_contribution:.6e}",
            contribution[0]
        );
    }
}

#[test]
fn pnoise_exact_dc_inductor_branch_clamps_output_noise_to_zero() {
    let deck = "\
* exact DC inductor short
r1 out 0 1k
l1 out 0 1m
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_pnoise(&netlist, 1.0e6, &[0.0], "out", None, None, 0)
        .expect("DC pnoise completes");

    assert_eq!(result.output_noise, vec![0.0]);
    assert_eq!(result.contributors.len(), 1);
    assert_eq!(result.contributors[0].1, vec![0.0]);
}

#[test]
fn input_referred_pnoise_rejects_an_exact_transfer_null() {
    let deck = "\
* disconnected input and noisy output
vin in 0 dc 0
rout out 0 1k
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let error = Engine::new(SimulationConfig::default())
        .run_pnoise(&netlist, 1.0e6, &[1.0e4], "out", None, Some("vin"), 0)
        .expect_err("input referral is undefined at a zero input-to-output transfer");
    assert!(
        error.to_string().contains("zero input-transfer"),
        "transfer-null failure must identify the undefined input referral: {error}"
    );
}

#[test]
fn retained_hb_pnoise_matches_the_same_exact_periodic_noise_problem() {
    let deck = "\
* retained-HB periodic-noise parity
vin in 0 SIN(.2 1 1meg)
r1 in mid rm 10k
.model rm R(KF=1e-12 AF=2 EF=1)
r2 mid 0 10k
c1 mid 0 1n
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let f0 = 1.0e6;
    let offsets = [1.0e3, 1.0e5];
    let hb = engine
        .run_hb(&netlist, HbConfig::new(f0).with_harmonics(8))
        .expect("HB operating point completes");

    let retained = engine
        .run_pnoise_from_hb_with_abort(
            &netlist,
            &offsets,
            "mid",
            None,
            Some("vin"),
            3,
            &hb.operating_point,
            &NoAbort,
        )
        .expect("retained-HB pnoise completes");
    let reference = engine
        .run_pnoise(&netlist, f0, &offsets, "mid", None, Some("vin"), 3)
        .expect("reference pnoise completes");

    assert_eq!(retained.frequencies, reference.frequencies);
    assert_eq!(retained.output_noise.len(), offsets.len());
    assert_eq!(retained.contributors.len(), reference.contributors.len());
    assert!(
        retained
            .contributors
            .iter()
            .any(|(name, values)| name.eq_ignore_ascii_case("r1 flicker")
                && values.iter().all(|&v| v > 0.0))
    );
    for (actual, expected) in retained.output_noise.iter().zip(&reference.output_noise) {
        assert!(
            (actual - expected).abs() <= 1e-12 * expected.abs().max(1e-300),
            "retained state changed output noise: {actual:.6e} vs {expected:.6e}"
        );
    }
    for (actual, expected) in retained
        .input_noise
        .as_ref()
        .expect("retained input noise")
        .iter()
        .zip(
            reference
                .input_noise
                .as_ref()
                .expect("reference input noise"),
        )
    {
        assert!(
            (actual - expected).abs() <= 1e-12 * expected.abs().max(1e-300),
            "retained state changed input-referred noise: {actual:.6e} vs {expected:.6e}"
        );
    }
}

#[test]
fn mos_nlev3_periodic_noise_follows_modulated_inversion_charge_at_zero_vds() {
    let mut config = SimulationConfig::default();
    config.convergence_config.gmin_target = 0.0;
    config.convergence_config.junction_gmin_target = 0.0;
    let engine = Engine::new(config);
    for (kind, p) in [("NMOS", 1.0), ("PMOS", -1.0)] {
        for amplitude in [0.0, 0.1, 0.25] {
            for gdsnoi in [0.0, 1.0, 3.0] {
                // With TOX/overlap/junction charge absent, VDS stays exactly
                // zero and the output is a resistor in parallel with g(t).
                let deck=Netlist::parse(&format!(
                    "Modulated NLEV3 noise\nVG g 0 SIN({} {} 1meg)\nRL out 0 1k\nM1 out g 0 0 mm W=2u L=1u M=5\n.model mm {kind}(LEVEL=1 VTO={p} KP=100u IS=0 NLEV=3 GDSNOI={gdsnoi})\n.options GMIN=0\n.end\n",p*1.4,p*amplitude)).unwrap();
                let result = engine
                    .run_pnoise(&deck, 1e6, &[1e4], "out", None, None, 8)
                    .unwrap();
                let beta = 1e-3_f64;
                let a = 1e-3 + beta * 0.4;
                let b = beta * amplitude;
                let d = (a * a - b * b).sqrt();
                // Exact averages: <1/(a+b*sin)>=1/d and
                // <1/(a+b*sin)^2>=a/d^3. MOS Sid(t)=4kT*g(t)*GDSNOI.
                let resistor = 1e-3 * a / (d * d * d);
                let channel = 1.0 / d - resistor;
                let expected = 4.0 * K_B * T_REF * (resistor + gdsnoi * channel);
                assert!(
                    (result.output_noise[0] - expected).abs() < expected * 2e-7,
                    "{kind} amplitude={amplitude} GDSNOI={gdsnoi}: {:e} vs {expected:e}",
                    result.output_noise[0]
                );
                if amplitude == 0.0 {
                    let dc = engine.run_dc_op(&deck).unwrap();
                    let out = dc
                        .node_names
                        .iter()
                        .position(|n| n.eq_ignore_ascii_case("out"))
                        .unwrap();
                    let stationary = engine
                        .run_noise_ports(&deck, out, None, &[1e4], T_REF)
                        .unwrap();
                    assert!(
                        (stationary[0].output_noise_density - expected).abs() < expected * 2e-10
                    );
                }
            }
        }
    }
}

#[test]
fn pnoise_card_reports_integrated_input_and_output_noise() {
    use rspice_core::engine::PeriodicNoiseResult;
    use rspice_core::netlist::AnalysisCommand;

    let netlist = Netlist::parse(
        "* integrated periodic thermal noise\nvin in 0 dc 0\nr1 in out 1k\nr2 out 0 1k\n\
         .pnoise lin 3 1k 2k out=V(out) input=vin maxsideband=1 integratednoise=yes\n.end\n",
    )
    .unwrap();
    let AnalysisCommand::Pnoise(card) = &netlist.analyses[0] else {
        panic!("PNOISE card")
    };
    assert!(card.integrated_noise, "{card:?}");
    let engine = Engine::default();
    let hb = engine
        .run_hb(&netlist, HbConfig::new(1e6).with_harmonics(8))
        .unwrap();
    let PeriodicNoiseResult::Driven { result, .. } = engine
        .run_pnoise_card_from_hb_with_abort(&netlist, card, &hb.operating_point, &NoAbort)
        .unwrap()
    else {
        panic!("driven result")
    };
    assert_eq!(result.frequencies, [1000.0, 1500.0, 2000.0], "{result:?}");
    // Two 1 kohm resistors in parallel, with voltage gain 1/2 from VIN.
    let expected = (4.0 * K_B * T_REF * 500.0 * 1000.0).sqrt();
    assert!((result.integrated_output_noise.unwrap() / expected - 1.0).abs() < 1e-12);
    assert!((result.integrated_input_noise.unwrap() / (2.0 * expected) - 1.0).abs() < 1e-12);

    let mut disabled = card.clone();
    disabled.integrated_noise = false;
    let PeriodicNoiseResult::Driven { result, .. } = engine
        .run_pnoise_card_from_hb_with_abort(&netlist, &disabled, &hb.operating_point, &NoAbort)
        .unwrap()
    else {
        panic!("driven result")
    };
    assert_eq!(result.integrated_output_noise, None);
    assert_eq!(result.integrated_input_noise, None);
}

#[test]
fn native_gp_periodic_shot_and_flicker_match_independent_junction_modulation() {
    use num_complex::Complex64;
    let f0 = 1e6;
    let offsets = [2e5_f64, 1.2e6];
    let resistance = 1000.0;
    let m = 3.0;
    let vt = K_B * T_REF / rspice_core::constants::Q_ELECTRON;
    for (kind, p, af, kf, drive) in [
        ("NPN", 1.0, 1.0_f64, 1e-12, 0.01),
        ("PNP", -1.0, 2.0, 1e-4, 0.01),
        // The nonlinear flicker amplitude extends well above the eight
        // voltage harmonics, even though the terminal drive is sinusoidal.
        ("NPN", 1.0, 10.0, 1e40, 0.06),
    ] {
        // Ideal terminal biases and a current monitor give exactly R transimpedance
        // from the base noise port. Independent Shockley-current quadrature
        // determines the white intensity and the colored amplitude spectrum.
        let netlist = Netlist::parse(&format!(
            "GP modulated noise oracle\nVC c 0 {}\nVB b 0 DC {} SIN({} {} 1meg)\nQ1 c b 0 qm AREA=2 M=3\nFmonitor out 0 VB 1\nRload out 0 1k NOISY=0\n.model qm {kind}(LEVEL=1 IS=1e-15 BF=100 BR=1e6 KF={kf} AF={af} EF=0.7)\n.options GMIN=0 VNTOL=1e-12\n.end\n", p*1.2, p*0.6, p*0.6, p*drive
        )).unwrap();
        let mut simulation = SimulationConfig::default();
        simulation.convergence_config.gmin_target = 0.0;
        simulation.convergence_config.junction_gmin_target = 0.0;
        let engine = Engine::new(simulation);
        let mut config = HbConfig::new(f0)
            .with_harmonics(8)
            .with_oversample(4)
            .with_tolerance(1e-10);
        config.abstol = 1e-18;
        let hb = engine.run_hb(&netlist, config).unwrap();
        let points = 1024;
        let currents: Vec<_> = (0..points)
            .map(|i| {
                let phase = std::f64::consts::TAU * i as f64 / points as f64;
                2e-15 / 100.0 * ((0.6 + drive * phase.sin()) / vt).exp_m1()
            })
            .collect();
        let mean = currents.iter().sum::<f64>() / points as f64;
        let amplitude: Vec<_> = (0..=64)
            .map(|h| {
                currents
                    .iter()
                    .enumerate()
                    .map(|(i, current)| {
                        Complex64::from_polar(
                            current.powf(af / 2.0),
                            -std::f64::consts::TAU * h as f64 * i as f64 / points as f64,
                        )
                    })
                    .sum::<Complex64>()
                    / points as f64
            })
            .collect();
        for sidebands in [0, 2] {
            let noise = engine
                .run_pnoise_from_hb_with_abort(
                    &netlist,
                    &offsets,
                    "out",
                    None,
                    None,
                    sidebands,
                    &hb.operating_point,
                    &NoAbort,
                )
                .unwrap();
            let mechanism = |name: &str, i: usize| -> f64 {
                noise
                    .contributors
                    .iter()
                    .filter(|(label, _)| label.eq_ignore_ascii_case(name))
                    .map(|(_, values)| values[i])
                    .sum()
            };
            for (i, &frequency) in offsets.iter().enumerate() {
                let shot =
                    resistance * resistance * 2.0 * rspice_core::constants::Q_ELECTRON * m * mean;
                let flicker = resistance
                    * resistance
                    * kf
                    * m
                    * (-64_i32..=64)
                        .map(|h| {
                            amplitude[h.unsigned_abs() as usize].norm_sqr()
                                / (frequency + h as f64 * f0).abs().powf(0.7)
                        })
                        .sum::<f64>();
                for (name, expected) in [("Q1:IB", shot), ("Q1:FN", flicker)] {
                    let actual = mechanism(name, i);
                    assert!(
                        (actual / expected - 1.0).abs() < 2e-7,
                        "{kind} AF={af} K={sidebands} f={frequency} {name}: {actual:e} vs {expected:e}"
                    );
                }
                assert!((noise.output_noise[i] / (shot + flicker) - 1.0).abs() < 2e-7);
            }
        }
    }
}

#[test]
fn native_vbic_periodic_noise_matches_stationary_physical_sources() {
    let offsets = [1e3, 1e6];
    for (level, dialect, kind, p) in [
        (4, SpiceDialect::Ngspice, "NPN", 1.0),
        (11, SpiceDialect::Xyce, "PNP", -1.0),
        (12, SpiceDialect::Xyce, "NPN", 1.0),
    ] {
        let substrate = if level == 11 { "" } else { " 0" };
        let self_heat = if level == 4 { "SELFT=1" } else { "" };
        let netlist = Netlist::parse(&format!(
            "VBIC stationary periodic noise\nVCC supply 0 {}\nRL supply c 500\nVB drive 0 {}\nRIN drive b 100\nQ1 c b 0{substrate} th vm AREA=2 M=3\n.model vm {kind}(LEVEL={level} IS=1e-15 IBEI=1e-17 IBCI=1e-17 IBEIP=1e-17 ISP=1e-17 RCX=10 RCI=20 RBX=10 RBI=40 RE=1 RBP=10 RS=1 CJE=10p CJC=5p CJEP=3p CJCP=2p TF=10n TR=2n QCO=10f GAMM=1e-9 WBE=.8 {self_heat} RTH=1000 CTH=1n TD=100n KFN=1e-12 AFN=1.2 BFN=0.7)\n.options GMIN=0 VNTOL=1e-12 RELTOL=1e-9 ABSTOL=1e-16\n.end\n", p*2.0, p*0.65
        )).unwrap();
        let mut simulation = SimulationConfig::default().with_spice_dialect(dialect);
        simulation.convergence_config.gmin_target = 0.0;
        simulation.convergence_config.junction_gmin_target = 0.0;
        let engine = Engine::new(simulation);
        let noise = engine
            .run_noise_named_with_input_source(&netlist, "c", None, "VB", &offsets, T_REF)
            .unwrap();
        let mut config = HbConfig::new(1e6).with_harmonics(8).with_tolerance(1e-10);
        // This loaded, finite-resistance network cannot resolve attoamp
        // residuals from binary64 voltage differences. Keep the independent
        // mechanism and total-density comparisons below at 2 ppm.
        config.abstol = 1e-14;
        let hb = engine.run_hb(&netlist, config).unwrap();
        let periodic = engine
            .run_pnoise_from_hb_with_abort(
                &netlist,
                &offsets,
                "c",
                None,
                Some("VB"),
                0,
                &hb.operating_point,
                &NoAbort,
            )
            .unwrap();
        for (i, point) in noise.iter().enumerate() {
            for contribution in &point.contributions {
                if !contribution.identity.device.eq_ignore_ascii_case("Q1") {
                    continue;
                }
                let name = format!("Q1:{}", contribution.identity.mechanism.as_deref().unwrap());
                let expected = contribution.output_contribution;
                let actual: f64 = periodic
                    .contributors
                    .iter()
                    .filter(|(label, _)| label.eq_ignore_ascii_case(&name))
                    .map(|(_, values)| values[i])
                    .sum();
                assert!(
                    (actual - expected).abs() < 2e-6 * expected + 1e-40,
                    "LEVEL={level} {name} f={}: {actual:e} vs {expected:e}",
                    offsets[i]
                );
            }
            assert!((periodic.output_noise[i] / point.output_noise_density - 1.0).abs() < 2e-6);
        }
    }
}
