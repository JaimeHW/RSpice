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
vin in 0 dc 0
r1 in mid 10k
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
