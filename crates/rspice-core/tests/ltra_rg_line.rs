//! Finite-length LTRA RG lines (`R > 0`, `G > 0`, `L = C = 0`).
//!
//! An RG line has no reactance, so its propagation constant `gamma = sqrt(R*G)`
//! and characteristic impedance `Z0 = sqrt(R/G)` are real and frequency
//! independent. That makes the line a memoryless two-port whose exact ABCD
//! parameters `A = D = cosh(theta)`, `B = Z0*sinh(theta)` and
//! `C = sinh(theta)/Z0` (with `theta = len*sqrt(R*G)`) describe it identically
//! in DC, AC, transient and every periodic analysis.
//!
//! These are the only nonzero-`G` lines ngspice-46 (`ltraset.c`
//! `LTRA_MOD_RG`, loaded by `ltraload.c`) and Xyce 7.10 (`N_DEV_LTRA.C`)
//! implement; both refuse RLGC with `G != 0`, and so does RSpice.
//!
//! The oracle here is the closed-form two-port, which is what both reference
//! simulators solve: their matrix loads are literally the ABCD coefficients
//! above, so a stamp that matches the closed form matches them. The lumped
//! ladder is the independent check that the closed form is the right physics.

use rspice_core::Netlist;
use rspice_core::engine::{Engine, SimulationConfig};

/// Terminated-line transfer `V2/V1` for an ideal source at port 1 and a
/// resistive load `RL` at port 2.
///
/// From `V1 = A*V2 + B*I2` and `I2 = V2/RL` (the current leaving port 2 into
/// the load), `V2 = V1 / (A + B/RL)`.
fn oracle_transfer(r: f64, g: f64, len: f64, load: f64) -> f64 {
    let theta = len * (r * g).sqrt();
    let a = theta.cosh();
    let b = (r / g).sqrt() * theta.sinh();
    1.0 / (a + b / load)
}

/// Input current drawn from an ideal 1 V source at port 1 with load `RL`.
///
/// `I1 = C*V2 + D*I2` with `I2 = V2/RL`, so `I1 = V2*(C + A/RL)`.
fn oracle_input_current(r: f64, g: f64, len: f64, load: f64) -> f64 {
    let theta = len * (r * g).sqrt();
    let a = theta.cosh();
    let c = theta.sinh() / (r / g).sqrt();
    oracle_transfer(r, g, len, load) * (c + a / load)
}

fn rg_deck(r: f64, g: f64, len: f64, load: f64, analysis: &str) -> String {
    format!(
        "* finite-length LTRA RG line\n\
         VIN in 0 DC 1 AC 1\n\
         O1 in 0 out 0 rgline\n\
         RL out 0 {load}\n\
         .model rgline ltra r={r} g={g} len={len}\n\
         {analysis}\n\
         .end\n"
    )
}

fn engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

fn dc_node(deck: &str, node: &str) -> f64 {
    let netlist = Netlist::parse(deck).expect("RG deck parses");
    let result = engine()
        .run_dc_op(&netlist)
        .expect("RG operating point solves");
    let index = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("missing node {node}"));
    result.node_voltages[index]
}

/// The parameter grid: three decades of `R*G` product and four lengths, so
/// `theta` spans the numerically easy small-argument regime and the regime
/// where `cosh(theta)` and `sinh(theta)` are large and nearly equal.
const CASES: &[(f64, f64, f64)] = &[
    (1.0, 1.0e-9, 1.0),
    (12.45, 1.0e-6, 16.0),
    (100.0, 1.0e-4, 5.0),
    (3.0, 1.0e-6, 10.0),
    (0.05, 20.0, 0.5),
    (1000.0, 1.0e-3, 2.0),
];

#[test]
fn dc_operating_point_matches_the_two_port_oracle() {
    for &(r, g, len) in CASES {
        let load = 75.0;
        let deck = rg_deck(r, g, len, load, ".op");
        let actual = dc_node(&deck, "out");
        let expected = oracle_transfer(r, g, len, load);
        assert!(
            (actual - expected).abs() <= 1.0e-10 * expected.abs().max(1.0e-12),
            "RG DC transfer for R={r} G={g} LEN={len}: got {actual:.17e}, oracle {expected:.17e}"
        );
    }
}

#[test]
fn dc_source_current_matches_the_two_port_oracle() {
    for &(r, g, len) in CASES {
        let load = 75.0;
        let deck = rg_deck(r, g, len, load, ".op");
        let netlist = Netlist::parse(&deck).expect("RG deck parses");
        let result = engine()
            .run_dc_op(&netlist)
            .expect("RG operating point solves");
        // The source branch current is the negative of the current the source
        // pushes into the line, in SPICE's source-current convention.
        let index = result
            .branch_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("VIN"))
            .expect("source branch current retained");
        let source = result.branch_currents[index];
        let expected = -oracle_input_current(r, g, len, load);
        assert!(
            (source - expected).abs() <= 1.0e-9 * expected.abs().max(1.0e-15),
            "RG source current for R={r} G={g} LEN={len}: got {source:.17e}, oracle {expected:.17e}"
        );
    }
}

/// A distributed RG line is the limit of a ladder of series `R*dx` resistors
/// and shunt `G*dx` conductances. This is the independent physical check on
/// the closed form: nothing in the ladder deck mentions `cosh` or `sinh`.
#[test]
fn fine_lumped_sectioning_converges_to_the_distributed_answer() {
    for &(r, g, len) in &[
        (12.45, 1.0e-6, 16.0),
        (100.0, 1.0e-4, 5.0),
        (3.0, 1.0e-6, 10.0),
    ] {
        let load = 75.0;
        let sections = 4000usize;
        let dx = len / sections as f64;
        let mut deck = String::from("* lumped RG ladder\nVIN in 0 DC 1\n");
        for section in 0..sections {
            let from = if section == 0 {
                "in".to_string()
            } else {
                format!("n{section}")
            };
            let to = if section + 1 == sections {
                "out".to_string()
            } else {
                format!("n{}", section + 1)
            };
            deck.push_str(&format!("RS{section} {from} {to} {}\n", r * dx));
            // Trapezoidal shunt placement: half the section's conductance at
            // each end, so the ladder is second-order accurate in dx.
            let shunt = 2.0 / (g * dx);
            deck.push_str(&format!("RA{section} {from} 0 {shunt}\n"));
            deck.push_str(&format!("RB{section} {to} 0 {shunt}\n"));
        }
        deck.push_str(&format!("RL out 0 {load}\n.op\n.end\n"));
        // The two half-shunts at the outer ends double-count nothing: the
        // source node is held by an ideal source and the load node's extra
        // half-shunt is what the trapezoidal rule prescribes.
        let ladder = dc_node(&deck, "out");
        let expected = oracle_transfer(r, g, len, load);
        assert!(
            (ladder - expected).abs() <= 5.0e-5 * expected.abs(),
            "ladder for R={r} G={g} LEN={len}: got {ladder:.12e}, distributed {expected:.12e}"
        );
    }
}

#[test]
fn ac_response_is_frequency_independent_and_matches_dc() {
    for &(r, g, len) in CASES {
        let load = 75.0;
        let deck = rg_deck(r, g, len, load, "");
        let netlist = Netlist::parse(&deck).expect("RG deck parses");
        let points = engine()
            .run_ac(&netlist, &[1.0, 1.0e3, 1.0e6, 1.0e9, 1.0e12])
            .expect("RG AC sweep solves");
        let expected = oracle_transfer(r, g, len, load);
        for point in &points {
            let index = point
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("out"))
                .expect("output node retained");
            let value = point.voltages[index];
            assert!(
                (value.re - expected).abs() <= 1.0e-10 * expected.abs()
                    && value.im.abs() <= 1.0e-12 * expected.abs().max(1.0e-12),
                "RG AC at {} Hz for R={r} G={g} LEN={len}: got {value:?}, oracle {expected:.17e}",
                point.frequency
            );
        }
    }
}

#[test]
fn transient_follows_the_source_with_no_delay_or_stored_energy() {
    let (r, g, len, load) = (12.45, 1.0e-6, 16.0, 75.0);
    let deck = format!(
        "* RG line is memoryless: the output steps with the source\n\
         VIN in 0 PULSE(0 1 1n 1p 1p 1u 2u)\n\
         O1 in 0 out 0 rgline\n\
         RL out 0 {load}\n\
         .model rgline ltra r={r} g={g} len={len}\n\
         .tran 1n 100n\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("RG transient deck parses");
    let result = engine()
        .run_tran(&netlist, 100.0e-9, 1.0e-9)
        .expect("RG transient solves");
    let input = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("in"))
        .expect("input node retained");
    let output = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("output node retained");
    let gain = oracle_transfer(r, g, len, load);
    let mut checked = 0usize;
    for step in 0..result.time.len() {
        let vin = result.voltages[input][step];
        let vout = result.voltages[output][step];
        assert!(
            (vout - gain * vin).abs() <= 1.0e-9 * gain.max(1.0e-12),
            "RG transient at t={:.3e}: V(out)={vout:.12e}, expected {:.12e}",
            result.time[step],
            gain * vin
        );
        checked += 1;
    }
    assert!(checked > 10, "the transient must produce a real trajectory");
}

/// A passive line can only dissipate: the power the source delivers must cover
/// the load power plus the distributed shunt loss, and neither can be negative.
#[test]
fn the_line_is_passive_at_every_operating_point() {
    for &(r, g, len) in CASES {
        let load = 75.0;
        let transfer = oracle_transfer(r, g, len, load);
        let input_current = oracle_input_current(r, g, len, load);
        let source_power = input_current;
        let load_power = transfer * transfer / load;
        assert!(
            source_power > 0.0,
            "an RG line driven by 1 V must draw positive power: R={r} G={g} LEN={len}"
        );
        assert!(
            load_power >= 0.0 && source_power + 1.0e-15 >= load_power,
            "RG line for R={r} G={g} LEN={len} delivered {load_power:e} from {source_power:e}"
        );
        assert!(
            transfer > 0.0 && transfer < 1.0,
            "a lossy RG line must attenuate: transfer {transfer} for R={r} G={g} LEN={len}"
        );
    }
}

/// LTRA carries no temperature coefficients in either reference simulator, so
/// an authored analysis temperature cannot move an RG line's answer.
#[test]
fn analysis_temperature_does_not_change_an_rg_line() {
    let (r, g, len, load) = (12.45, 1.0e-6, 16.0, 75.0);
    let nominal = dc_node(&rg_deck(r, g, len, load, ".op"), "out");
    let hot = dc_node(&rg_deck(r, g, len, load, ".options temp=125\n.op"), "out");
    assert_eq!(
        nominal.to_bits(),
        hot.to_bits(),
        "an LTRA card has no TC parameters, so TEMP must not perturb it"
    );
}

/// Neither ngspice-46 nor Xyce 7.10 registers a noise model for LTRA: the
/// device has no `DEVnoise`/`noise` entry point at all, so a line contributes
/// no source of its own however lossy it is. RSpice states that explicitly
/// rather than leaving it to the absence of code.
#[test]
fn an_rg_line_contributes_no_noise_source_of_its_own() {
    let (r, g, len) = (12.45, 1.0e-6, 16.0);
    let quiet = format!(
        "* every noise source in the deck is silenced\n\
         VIN in 0 DC 0 AC 1\n\
         RS in a 50 noisy=0\n\
         O1 a 0 out 0 rgline\n\
         RL out 0 75 noisy=0\n\
         .model rgline ltra r={r} g={g} len={len}\n\
         .end\n"
    );
    let netlist = Netlist::parse(&quiet).expect("quiet RG noise deck parses");
    let circuit = engine().build_circuit(&netlist).expect("circuit builds");
    let output = circuit.get_node_by_name("out").expect("output node");
    let results = engine()
        .run_noise_with_input_source(&netlist, output, None, "VIN", &[1.0e3, 1.0e6], 300.15)
        .expect("noise analysis runs with an RG line present");
    for result in &results {
        assert_eq!(
            result.output_noise_density, 0.0,
            "an RG line must contribute exactly zero output noise, got {:e}",
            result.output_noise_density
        );
    }
}

#[test]
fn nonfinite_negative_and_reactive_g_cards_still_fail_closed() {
    for (model, evidence) in [
        (".model line ltra r=3 g=-1e-6 len=10", "invalid G="),
        (".model line ltra r=3 g=1e400 len=10", "non-finite G"),
        (
            ".model line ltra r=1 l=1n g=1e-6 c=1p len=1",
            "finite nonzero G",
        ),
        (
            ".model line ltra r=0 g=1e-6 len=10",
            "nonzero G is valid only for a pure RG line with R>0 and L=C=0",
        ),
    ] {
        let deck = format!(
            "LTRA RG fail-closed\nVIN in 0 AC 1\nO1 in 0 out 0 line\nRLOAD out 0 50\n{model}\n.end\n"
        );
        let netlist = Netlist::parse(&deck).expect("fail-closed deck parses");
        let error = engine()
            .run_ac(&netlist, &[1.0e3])
            .expect_err("a nonrepresentable RG/RLGC card must fail before stamping");
        let text = error.to_string();
        assert!(
            text.contains(evidence),
            "rejection for {model} must name '{evidence}': {text}"
        );
        assert!(
            !text.contains("singular") && !text.contains("solve failed"),
            "semantic rejection must precede solver diagnostics for {model}: {text}"
        );
    }
}

/// Literal transcription of ngspice-46 `ltraload.c`'s `LTRA_MOD_RG` setup,
/// including its `1e-10` substitution cutoffs and the `(1 + GMIN)` scaling it
/// applies to the two transfer coefficients.
///
/// Neither the ngspice source tree nor `Xyce_Regression-master` ships a
/// finite-length RG fixture — both carry only `LEN=0` RG cards — so this is
/// the reference algorithm rather than a captured waveform. It is the exact
/// arithmetic the reference simulator performs, so a match pins RSpice to the
/// oracle's matrix load and not merely to a re-derivation of the same physics.
fn ngspice_ltraload_rg_coefficients(r: f64, g: f64, len: f64, gmin: f64) -> (f64, f64, f64) {
    let mut dummy1 = len * (r * g).sqrt();
    let dummy2 = (-dummy1).exp();
    dummy1 = dummy1.exp();
    let coshlroot_gr = 0.5 * (dummy1 + dummy2);
    let r_rs_lr_gror_g = if g <= 1.0e-10 {
        len * r
    } else {
        0.5 * (dummy1 - dummy2) * (r / g).sqrt()
    };
    let r_gs_lr_gror_r = if r <= 1.0e-10 {
        len * g
    } else {
        0.5 * (dummy1 - dummy2) * (g / r).sqrt()
    };
    (
        coshlroot_gr,
        (1.0 + gmin) * r_rs_lr_gror_g,
        (1.0 + gmin) * r_gs_lr_gror_r,
    )
}

/// RSpice does not apply ngspice's `(1 + GMIN)` scaling: that factor is a
/// matrix-conditioning hack on an already nonsingular two-port, and applying
/// it would make the physical answer depend on a solver option. With ngspice's
/// default `GMIN = 1e-12` the two loads differ by at most one part in `1e12`,
/// which is the tolerance declared here.
const NGSPICE_GMIN: f64 = 1.0e-12;
const NGSPICE_ORACLE_TOLERANCE: f64 = 1.0e-9;

#[test]
fn dc_and_ac_match_the_ngspice_ltraload_rg_matrix_load() {
    for &(r, g, len) in CASES {
        let load = 75.0;
        let (a, b, c) = ngspice_ltraload_rg_coefficients(r, g, len, NGSPICE_GMIN);
        // Solve ngspice's own two rows for the terminated line:
        //   V1 - A*V2 + B*I2 = 0,  I1 + A*I2 - C*V2 = 0,  I2 = -V2/RL
        // with V1 = 1 V held by the ideal source.
        let expected_v2 = 1.0 / (a + b / load);
        let expected_i1 = expected_v2 * (c + a / load);

        let deck = rg_deck(r, g, len, load, ".op");
        let actual_v2 = dc_node(&deck, "out");
        assert!(
            (actual_v2 - expected_v2).abs() <= NGSPICE_ORACLE_TOLERANCE * expected_v2,
            "ngspice RG load V(out) for R={r} G={g} LEN={len}: got {actual_v2:.17e}, oracle {expected_v2:.17e}"
        );

        let netlist = Netlist::parse(&deck).expect("RG deck parses");
        let result = engine()
            .run_dc_op(&netlist)
            .expect("RG operating point solves");
        let index = result
            .branch_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("VIN"))
            .expect("source branch current retained");
        let actual_i1 = -result.branch_currents[index];
        assert!(
            (actual_i1 - expected_i1).abs() <= NGSPICE_ORACLE_TOLERANCE * expected_i1,
            "ngspice RG load I(VIN) for R={r} G={g} LEN={len}: got {actual_i1:.17e}, oracle {expected_i1:.17e}"
        );
    }
}

#[test]
fn an_unrepresentable_propagation_constant_fails_closed_before_any_instance_exists() {
    // cosh(len*sqrt(R*G)) overflows to infinity here, which is exactly the
    // overflow ngspice's own comment warns about and silently propagates.
    let deck = rg_deck(1.0e6, 1.0e6, 1.0e3, 75.0, ".op");
    let netlist = Netlist::parse(&deck).expect("overflowing RG deck still parses");
    let error = engine()
        .run_dc_op(&netlist)
        .expect_err("a nonrepresentable RG two-port must fail before it is stamped");
    let text = error.to_string();
    assert!(
        text.contains("not representable"),
        "the rejection must say the two-port is not representable: {text}"
    );
}

/// The periodic-analysis capability descriptors admit an RG line wherever a
/// linear resistor is admitted, because it is linear and memoryless.
#[test]
fn periodic_analyses_admit_an_rg_line() {
    use rspice_core::analysis::PssConfig;
    use rspice_core::analysis::harmonic_balance::HbConfig;

    let (r, g, len, load) = (12.45, 1.0e-6, 16.0, 75.0);
    let gain = oracle_transfer(r, g, len, load);
    let f0 = 1.0e6;

    let hb_deck = format!(
        "* RG line under harmonic balance\n\
         VIN in 0 SIN(0 1 {f0})\n\
         O1 in 0 out 0 rgline\n\
         RL out 0 {load}\n\
         .model rgline ltra r={r} g={g} len={len}\n\
         .end\n"
    );
    let hb = engine()
        .run_hb(
            &Netlist::parse(&hb_deck).expect("HB RG deck parses"),
            HbConfig::new(f0).with_harmonics(3),
        )
        .expect("an RG line is admitted by the exact periodic MNA descriptor");
    let spectrum_at = |node: &str| {
        hb.result
            .spectral_voltages
            .iter()
            .find(|spectrum| spectrum.node_name.eq_ignore_ascii_case(node))
            .unwrap_or_else(|| panic!("missing HB node '{node}'"))
    };
    let input = spectrum_at("in").coefficients[1];
    let output = spectrum_at("out").coefficients[1];
    // A memoryless line scales every harmonic by the same real gain, so the
    // output fundamental is the input's times that gain with no added phase.
    assert!(
        (output - input * gain).norm() <= 1.0e-9 * (input.norm() * gain).max(1.0e-12),
        "a memoryless RG line must scale the fundamental by its DC transfer with no added phase: \
         got {output}, expected {}",
        input * gain
    );
    for harmonic in [2, 3] {
        assert!(
            spectrum_at("out").coefficients[harmonic].norm() <= 1.0e-9 * gain,
            "a linear line generates no harmonic {harmonic}: {}",
            spectrum_at("out").coefficients[harmonic]
        );
    }

    let pss_deck = format!(
        "* RG line under a shooting PSS continuation\n\
         VIN in 0 SIN(0 1 {f0})\n\
         RS in a 50\n\
         O1 a 0 out 0 rgline\n\
         CL out 0 100p\n\
         RL out 0 {load}\n\
         .model rgline ltra r={r} g={g} len={len}\n\
         .end\n"
    );
    engine()
        .run_pss_with_continuation_state(
            &Netlist::parse(&pss_deck).expect("PSS RG deck parses"),
            PssConfig::new(f0)
                .with_harmonics(2)
                .with_points_per_period(64)
                .with_tstab_periods(0)
                .with_tolerance(1.0e-6),
        )
        .expect("an RG line carries no period-map state and must be admitted");

    let pz_deck = format!(
        "* RG line under pole-zero descriptor extraction\n\
         VIN in 0 AC 1\n\
         RS in a 50\n\
         O1 a 0 out 0 rgline\n\
         CL out 0 100p\n\
         RL out 0 {load}\n\
         .model rgline ltra r={r} g={g} len={len}\n\
         .end\n"
    );
    let netlist = Netlist::parse(&pz_deck).expect("PZ RG deck parses");
    let circuit = engine().build_circuit(&netlist).expect("circuit builds");
    let input = circuit.get_node_by_name("a").expect("input node");
    let output = circuit.get_node_by_name("out").expect("output node");
    let pz = engine()
        .run_pz(&netlist, input, output)
        .expect("an RG line contributes no dynamic state and must be admitted");
    assert!(
        !pz.poles.is_empty(),
        "the RC pole behind the RG line must still be extracted"
    );
}

/// A delay line still has an irrational descriptor, so the pole-zero rejection
/// the RG case lifts must remain in place for every other line.
#[test]
fn a_delay_line_is_still_refused_by_pole_zero_extraction() {
    let deck = "* a delay line has no finite explicit descriptor state
VIN in 0 AC 1
RS in a 50
T1 a 0 out 0 z0=50 td=1n
RL out 0 50
.end
";
    let netlist = Netlist::parse(deck).expect("delay-line deck parses");
    let circuit = engine().build_circuit(&netlist).expect("circuit builds");
    let input = circuit.get_node_by_name("a").expect("input node");
    let output = circuit.get_node_by_name("out").expect("output node");
    let error = engine()
        .run_pz(&netlist, input, output)
        .expect_err("a delay line must stay outside pole-zero extraction")
        .to_string();
    assert!(
        error.contains("irrational"),
        "the rejection must name the missing capability: {error}"
    );
}

/// The `LEN=0` RC/RG ideal-through case is the other memoryless line, and it
/// reaches the periodic analyses through the same declaration: no propagation
/// history for the period map, no dynamic state for the descriptor.
#[test]
fn the_zero_length_through_connection_is_admitted_alongside_the_rg_line() {
    use rspice_core::analysis::PssConfig;

    let deck = "\
* LEN=0 RC through connection carries no state either
VIN in 0 SIN(0 1 1meg)
RS in a 50
O1 a 0 out 0 through
CL out 0 100p
RL out 0 75
.model through ltra r=0.05 c=20p len=0
.end
";
    let netlist = Netlist::parse(deck).expect("zero-length deck parses");
    engine()
        .run_pss_with_continuation_state(
            &netlist,
            PssConfig::new(1.0e6)
                .with_harmonics(2)
                .with_points_per_period(64)
                .with_tstab_periods(0)
                .with_tolerance(1.0e-6),
        )
        .expect("an ideal through connection carries no period-map state");

    let circuit = engine().build_circuit(&netlist).expect("circuit builds");
    let input = circuit.get_node_by_name("a").expect("input node");
    let output = circuit.get_node_by_name("out").expect("output node");
    let pz = engine()
        .run_pz(&netlist, input, output)
        .expect("an ideal through connection contributes no dynamic state");
    assert!(
        !pz.poles.is_empty(),
        "the RC pole behind the through connection must still be extracted"
    );
}
