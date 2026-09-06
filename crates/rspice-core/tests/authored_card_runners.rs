//! Authored `.SENS`, `.PZ`, `.SP`, `.PXF` and `.PSTB` cards run through core
//! entry points that take the card, and their results project into the shared
//! result document.

use rspice_core::abort_signal::NoAbort;
use rspice_core::engine::SensitivityCardResult;
use rspice_core::execution::{
    AnalysisResultDocument, DeckPlan, ResultPayload, result_document::ScalarValue,
};
use rspice_core::netlist::AnalysisCommand;
use rspice_core::resource::ResourceLimits;
use rspice_core::{Engine, Netlist, SimulationConfig};

fn card(netlist: &Netlist, select: fn(&AnalysisCommand) -> bool) -> AnalysisCommand {
    netlist
        .analyses
        .iter()
        .find(|command| select(command))
        .expect("the deck authors the card under test")
        .clone()
}

fn instance(netlist: &Netlist, tag: &str) -> rspice_core::execution::AnalysisInstanceId {
    let plan = DeckPlan::from_netlist(netlist, &ResourceLimits::default()).expect("the deck plans");
    plan.analyses()
        .iter()
        .find(|analysis| analysis.id().tag() == tag)
        .unwrap_or_else(|| panic!("the plan has no {tag}"))
        .id()
}

const DIVIDER: &str = "Resistive divider\n\
     V1 in 0 DC 1 AC 1\n\
     R1 in out 1k\n\
     R2 out 0 1k\n\
     C1 out 0 1n\n\
     .sens V(out)\n\
     .end\n";

#[test]
fn an_authored_sens_card_resolves_its_own_output_node() {
    let netlist = Netlist::parse(DIVIDER).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let result = engine
        .run_sensitivity_from_card_with_abort(
            &netlist,
            &card(&netlist, |command| {
                matches!(command, AnalysisCommand::Sensitivity { .. })
            }),
            &NoAbort,
        )
        .expect("the authored .SENS card runs from its own node names");
    let SensitivityCardResult::Dc(dc) = result else {
        panic!("a .SENS card with no AC clause is a DC study");
    };
    assert!(
        !dc.sensitivities.is_empty(),
        "a divider has sensitive parameters"
    );
    AnalysisResultDocument::from_sensitivity(instance(&netlist, "sens-001"), &dc)
        .expect("the shared document accepts the runner's result")
        .build()
        .expect("document builds");
}

#[test]
fn an_authored_sens_ac_card_selects_the_ac_driver_and_publishes_a_document() {
    let source = DIVIDER.replace(".sens V(out)", ".sens V(out) AC DEC 2 1k 10k");
    let netlist = Netlist::parse(&source).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let result = engine
        .run_sensitivity_from_card_with_abort(
            &netlist,
            &card(&netlist, |command| {
                matches!(command, AnalysisCommand::Sensitivity { .. })
            }),
            &NoAbort,
        )
        .expect("the authored .SENS AC card runs");
    let SensitivityCardResult::Ac(ac) = result else {
        panic!("a .SENS card with an AC clause is a frequency-domain study");
    };
    assert!(!ac.frequencies.is_empty());
    let document = AnalysisResultDocument::from_ac_sensitivity(instance(&netlist, "sens-001"), &ac)
        .expect("AC sensitivity now has a document builder")
        .build()
        .expect("document builds");
    assert_eq!(document.point_count(), ac.frequencies.len());
    let ResultPayload::Sensitivity(payload) = document.payload() else {
        panic!("a .SENS card projects a sensitivity payload");
    };
    assert!(
        payload.entries.is_empty() && !payload.ac_entries.is_empty(),
        "an AC study populates the AC traces and nothing else"
    );
    for entry in &payload.ac_entries {
        assert_eq!(entry.absolute.len(), ac.frequencies.len());
        assert_eq!(entry.phase.len(), ac.frequencies.len());
    }
}

#[test]
fn an_authored_pz_card_resolves_all_four_of_its_ports() {
    let source = DIVIDER.replace(".sens V(out)", ".pz in 0 out 0 vol pz");
    let netlist = Netlist::parse(&source).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let result = engine
        .run_pz_from_card_with_abort(
            &netlist,
            &card(&netlist, |command| {
                matches!(command, AnalysisCommand::PoleZero { .. })
            }),
            &NoAbort,
        )
        .expect("the authored .PZ card runs from its own node names");
    assert!(
        !result.poles.is_empty(),
        "an RC divider has at least one pole"
    );
    AnalysisResultDocument::from_pole_zero(instance(&netlist, "pz-001"), &result)
        .expect("the shared document accepts the runner's result")
        .build()
        .expect("document builds");
}

#[test]
fn an_authored_pz_card_naming_an_absent_node_fails_before_any_solve() {
    let source = DIVIDER.replace(".sens V(out)", ".pz in 0 nowhere 0 vol pz");
    let netlist = Netlist::parse(&source).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let error = engine
        .run_pz_from_card_with_abort(
            &netlist,
            &card(&netlist, |command| {
                matches!(command, AnalysisCommand::PoleZero { .. })
            }),
            &NoAbort,
        )
        .expect_err("an absent port node must fail closed");
    let message = error.to_string();
    assert!(
        message.to_ascii_lowercase().contains("nowhere") && message.contains(".PZ output"),
        "the failure names the port and the node it could not resolve: {message}"
    );
}

const TWO_PORT: &str = "Two-port pad\n\
     V1 p1 0 AC 1 portnum=1 z0=50\n\
     V2 p2 0 AC 0 portnum=2 z0=50\n\
     R1 p1 mid 25\n\
     R2 mid 0 50\n\
     R3 mid p2 25\n\
     .sp lin 3 1meg 3meg 1\n\
     .end\n";

#[test]
fn an_authored_sp_card_publishes_the_shared_sp_and_port_noise_documents() {
    let netlist = Netlist::parse(TWO_PORT).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let sp = card(&netlist, |command| {
        matches!(command, AnalysisCommand::Sp { .. })
    });
    let run = engine
        .run_sp_with_abort(&netlist, &sp, &NoAbort)
        .expect(".SP runs");
    let id = instance(&netlist, "sp-001");

    let scattering = AnalysisResultDocument::from_s_parameters(id, &run.scattering)
        .expect("the shared S-parameter document accepts exactly what the runner returns")
        .build()
        .expect("document builds");
    assert_eq!(scattering.point_count(), run.scattering.data.len());
    let ResultPayload::Sp(payload) = scattering.payload() else {
        panic!("a .SP card projects an S-parameter payload");
    };
    assert_eq!(payload.ports.len(), 2);

    let noise = run.port_noise.expect("the card requested port noise");
    let port_noise = AnalysisResultDocument::from_port_noise(id, &noise)
        .expect("the shared port-noise document accepts exactly what the runner returns")
        .build()
        .expect("document builds");
    assert_eq!(port_noise.point_count(), noise.points.len());
}

#[test]
fn a_stability_document_from_an_uncrossed_loop_records_the_absent_crossover() {
    // A single-pole loop below unity gain never crosses 0 dB, so its Tian
    // margins are unbounded. The projection must publish that determination
    // rather than fail the whole run closed.
    let netlist = Netlist::parse(
        "Uncrossed loop\n\
         V1 in 0 AC 1\n\
         Vprobe mid fb DC 0\n\
         R1 in mid 1k\n\
         R2 mid 0 1k\n\
         E1 fb 0 mid 0 0.1\n\
         .stb dec 4 1 1k probe=Vprobe\n\
         .end\n",
    )
    .expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let AnalysisCommand::Stb {
        variation,
        points,
        start_freq,
        stop_freq,
        probe,
    } = card(&netlist, |command| {
        matches!(command, AnalysisCommand::Stb { .. })
    })
    else {
        panic!("the deck authors a .STB card");
    };
    let config = rspice_core::analysis::StbConfig::new()
        .with_sweep(start_freq, stop_freq, points)
        .with_sweep_type(match variation {
            rspice_core::netlist::FreqVariation::Lin => rspice_core::analysis::StbSweepType::Linear,
            rspice_core::netlist::FreqVariation::Dec => rspice_core::analysis::StbSweepType::Decade,
            rspice_core::netlist::FreqVariation::Oct => rspice_core::analysis::StbSweepType::Octave,
        })
        .with_probe(&probe);
    let result = engine
        .run_stb_with_abort(&netlist, config, &NoAbort)
        .expect(".STB runs");
    let document =
        AnalysisResultDocument::from_stability(instance(&netlist, "stb-001"), &result.result)
            .expect("an unconditionally stable loop must publish")
            .build()
            .expect("document builds");
    let margin = document
        .scalars()
        .iter()
        .find(|scalar| scalar.name() == "gain_margin_db")
        .expect("the document reports a gain margin");
    assert!(
        matches!(
            margin.value(),
            ScalarValue::Real { .. } | ScalarValue::Unavailable { .. }
        ),
        "a margin is either a number or a typed determination"
    );
}

//=============================================================================
// .PXF
//=============================================================================

const PXF_FUNDAMENTAL: f64 = 1.0e6;

/// An RC low-pass whose corner sits exactly on the carrier fundamental, so
/// `H(f) = 1 / (1 + j f / 1 MHz)` is a closed-form answer at every sideband.
///
/// The cards are authored so the deck plans a `pxf-001` bound to a carrier;
/// the runs below drive the engine entries directly with cards built by hand,
/// which is what this file exists to gate.
const PXF_LOW_PASS: &str = "PXF card runner\n\
     vin in 0 dc 0 ac 1\n\
     r1 in out 1k\n\
     c1 out 0 159.154943091895p\n\
     .pss fund=1meg harms=8 points=128 tstabperiods=1\n\
     .pxf lin 3 100k 500k input=vin out=v(out) maxsideband=1\n\
     .end\n";

fn pxf_card(input_sideband: i32, output_sideband: i32) -> rspice_core::netlist::PxfCard {
    rspice_core::netlist::PxfCard {
        sweep: rspice_core::netlist::PeriodicSweep {
            variation: rspice_core::netlist::FreqVariation::Lin,
            points: 3,
            start_freq: 1.0e5,
            stop_freq: 5.0e5,
        },
        input_source: "VIN".to_owned(),
        input_sideband,
        output_node: "OUT".to_owned(),
        output_ref: None,
        output_sideband,
        max_sideband: 1,
        reltol: 1.0e-3,
        abstol: 1.0e-12,
        source: rspice_core::netlist::PeriodicSourceSelector::Preceding,
    }
}

fn pxf_carrier(engine: &Engine, netlist: &Netlist) -> rspice_core::engine::PssOperatingPoint {
    engine
        .run_pss_operating_point_with_abort(
            netlist,
            rspice_core::analysis::PssConfig::new(PXF_FUNDAMENTAL)
                .with_harmonics(8)
                .with_points_per_period(128)
                .with_tstab_periods(0),
            &NoAbort,
        )
        .expect("the linear carrier converges")
}

#[test]
fn an_authored_pxf_card_reads_the_conversion_element_its_sideband_pair_names() {
    // The engine entry is a PAC solve plus one conversion read. This pins that
    // it reads that element and no other, bit for bit, which is what the
    // Studio's own route produces for the same deck by way of a dense cube.
    let netlist = Netlist::parse(PXF_LOW_PASS).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let carrier = pxf_carrier(&engine, &netlist);
    let card = pxf_card(1, 1);

    let pxf = engine
        .run_pxf_card_from_pss_with_abort(&netlist, &card, &carrier, &NoAbort)
        .expect("the authored .PXF card runs against its carrier");
    let pac = engine
        .run_pac_from_pss_with_abort(
            &netlist,
            rspice_core::analysis::pac::PacConfig::from(&card),
            &carrier,
            &NoAbort,
        )
        .expect("the same card describes a runnable PAC solve");

    assert_eq!(pxf.points.len(), pac.result.frequencies.len());
    assert_eq!(pxf.fundamental_freq, PXF_FUNDAMENTAL);
    for (index, point) in pxf.points.iter().enumerate() {
        let element = pac
            .result
            .conversion_matrix
            .get(index, card.output_sideband, card.input_sideband)
            .expect("the conversion element exists");
        assert_eq!(
            point.transfer, element,
            "point {index} must be the (out {}, in {}) conversion element itself",
            card.output_sideband, card.input_sideband
        );
        assert_eq!(point.freq_in, pac.result.frequencies[index]);
        // The output frequency is absolute: offset + output_sideband * f0,
        // the same rule `PacSidebandDescriptor::absolute_frequencies` states.
        assert!(
            (point.freq_out - (point.freq_in + f64::from(card.output_sideband) * PXF_FUNDAMENTAL))
                .abs()
                <= 1.0e-9,
            "point {index} published {} Hz as its converted output frequency",
            point.freq_out
        );
        assert_eq!(point.sideband_in, card.input_sideband);
        assert_eq!(point.sideband_out, card.output_sideband);
    }
}

#[test]
fn an_authored_pxf_card_on_a_linear_circuit_reproduces_the_closed_form_transfer() {
    // A linear network cannot convert between sidebands, so the k -> k
    // transfer is exactly the ordinary AC response at offset + k * f0.
    let netlist = Netlist::parse(PXF_LOW_PASS).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let carrier = pxf_carrier(&engine, &netlist);

    for sideband in [0, 1] {
        let card = pxf_card(sideband, sideband);
        let pxf = engine
            .run_pxf_card_from_pss_with_abort(&netlist, &card, &carrier, &NoAbort)
            .expect(".PXF runs at every analyzed sideband");
        for point in &pxf.points {
            let absolute = point.freq_in + f64::from(sideband) * PXF_FUNDAMENTAL;
            let expected = num_complex::Complex64::new(1.0, 0.0)
                / num_complex::Complex64::new(1.0, absolute / PXF_FUNDAMENTAL);
            assert!(
                (point.transfer - expected).norm() <= 1.0e-6,
                "sideband {sideband} at {absolute:.3e} Hz: got {}, expected {expected}",
                point.transfer
            );
        }
    }
}

#[test]
fn an_authored_pxf_card_publishes_the_curve_metrics_core_computes() {
    let netlist = Netlist::parse(PXF_LOW_PASS).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let carrier = pxf_carrier(&engine, &netlist);
    let card = pxf_card(1, 1);

    let pxf = engine
        .run_pxf_card_from_pss_with_abort(&netlist, &card, &carrier, &NoAbort)
        .expect(".PXF runs");

    let mut oracle = rspice_core::analysis::pxf::PxfResult::new(pxf.fundamental_freq, 1, 1);
    for point in &pxf.points {
        oracle.add_point(point.clone());
    }
    oracle.compute_metrics();
    assert_eq!(pxf.peak_gain, oracle.peak_gain);
    assert_eq!(pxf.bandwidth_3db, oracle.bandwidth_3db);
    assert_eq!(pxf.unity_gain_freq, oracle.unity_gain_freq);
    assert_eq!(pxf.dc_gain, oracle.dc_gain);
    assert_eq!(
        pxf.group_delay_curve().len(),
        pxf.points.len() - 1,
        "group delay lives on the midpoint grid"
    );

    // And what the document publishes is those same numbers, not a second
    // derivation of them. The Studio's own route computes the four metrics and
    // drops them; the engine route is where they become evidence.
    let document = AnalysisResultDocument::from_pxf(instance(&netlist, "pxf-001"), &card, &pxf)
        .expect("the shared document accepts exactly what the runner returns")
        .build()
        .expect("document builds");
    assert_eq!(document.point_count(), pxf.points.len());
    let scalar = |name: &str| {
        document
            .scalars()
            .iter()
            .find(|scalar| scalar.name() == name)
            .unwrap_or_else(|| panic!("the document publishes {name}"))
            .value()
            .clone()
    };
    let published = |name: &str| match scalar(name) {
        ScalarValue::Real { value } => value,
        // The four metrics record "the curve has no such point" as a typed
        // determination rather than as a number.
        ScalarValue::Unavailable { .. } => None,
        other => panic!("{name} is neither a real nor a determination: {other:?}"),
    };
    assert_eq!(
        published("peak_gain_db"),
        oracle.peak_gain.map(|(_, db)| db)
    );
    assert_eq!(
        published("peak_gain_frequency"),
        oracle.peak_gain.map(|(frequency, _)| frequency)
    );
    assert_eq!(published("bandwidth_3db"), oracle.bandwidth_3db);
    assert_eq!(published("unity_gain_frequency"), oracle.unity_gain_freq);
    let ResultPayload::Pxf(payload) = document.payload() else {
        panic!("a .PXF card projects a PXF payload");
    };
    assert_eq!(payload.group_delay.len(), pxf.group_delay_curve().len());
    for (sample, (frequency, delay)) in payload.group_delay.iter().zip(pxf.group_delay_curve()) {
        assert_eq!(sample.frequency, frequency);
        assert_eq!(sample.delay, delay);
    }
}

#[test]
fn a_pxf_document_refuses_a_card_that_did_not_produce_its_result() {
    // The card states the measurement and the result carries it out. Pairing
    // one card's statement with another's numbers would publish a path nothing
    // computed, so the projection refuses rather than relabelling.
    let netlist = Netlist::parse(PXF_LOW_PASS).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let carrier = pxf_carrier(&engine, &netlist);
    let pxf = engine
        .run_pxf_card_from_pss_with_abort(&netlist, &pxf_card(1, 1), &carrier, &NoAbort)
        .expect(".PXF runs");

    let error =
        AnalysisResultDocument::from_pxf(instance(&netlist, "pxf-001"), &pxf_card(0, 1), &pxf)
            .expect_err("a card naming a different sideband pair must be refused");
    let message = error.to_string();
    assert!(
        message.contains("sideband"),
        "the refusal names the disagreement: {message}"
    );
}

#[test]
fn an_authored_pxf_card_runs_against_a_harmonic_balance_carrier() {
    // `.PAC` accepts either periodic carrier and `.PXF` is a reading of the
    // conversion matrix that solve fills, so the harmonic-balance sibling must
    // produce the same closed-form answer the shooting one does.
    let netlist = Netlist::parse(PXF_LOW_PASS).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let hb = engine
        .run_hb_with_abort(
            &netlist,
            rspice_core::analysis::HbConfig::new(PXF_FUNDAMENTAL).with_harmonics(8),
            &NoAbort,
        )
        .expect("the harmonic-balance carrier converges");
    let card = pxf_card(1, 1);

    let pxf = engine
        .run_pxf_card_from_hb_with_abort(&netlist, &card, &hb.operating_point, &NoAbort)
        .expect(".PXF runs against a harmonic-balance carrier");
    assert_eq!(pxf.points.len(), 3);
    for point in &pxf.points {
        let absolute = point.freq_in + PXF_FUNDAMENTAL;
        let expected = num_complex::Complex64::new(1.0, 0.0)
            / num_complex::Complex64::new(1.0, absolute / PXF_FUNDAMENTAL);
        assert!(
            (point.transfer - expected).norm() <= 1.0e-6,
            "HB-carried .PXF at {absolute:.3e} Hz: got {}",
            point.transfer
        );
    }
}

#[test]
fn an_authored_pxf_card_honours_cancellation() {
    let netlist = Netlist::parse(PXF_LOW_PASS).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let carrier = pxf_carrier(&engine, &netlist);
    let error = engine
        .run_pxf_card_from_pss_with_abort(
            &netlist,
            &pxf_card(1, 1),
            &carrier,
            &rspice_core::abort_signal::ImmediateAbort,
        )
        .expect_err("an aborted .PXF run must not publish a partial transfer");
    assert!(matches!(
        error,
        rspice_core::engine::SimulationError::Aborted
    ));
}

//=============================================================================
// .PSTB
//=============================================================================

const PSTB_FUNDAMENTAL: f64 = 1.0e6;

/// A driven series-RLC with one capacitor and one inductor, so the shooting
/// state carries both kinds of coordinate and the loop probe's index is a
/// non-trivial offset into the basis rather than zero by luck.
const PSTB_RESONATOR: &str = "PSTB card runner\n\
     vin in 0 SIN(0 1 1meg)\n\
     r1 in a 50\n\
     l1 a out 10u\n\
     c1 out 0 1n\n\
     .end\n";

fn pstb_card(probe: &str) -> rspice_core::netlist::PstbCard {
    rspice_core::netlist::PstbCard {
        probe_instance: probe.to_owned(),
        max_harmonics: 4,
        num_multipliers: 10,
        stability_threshold: 1.0 + 1.0e-6,
        detect_subharmonics: true,
        eigenvalue_tolerance: 1.0e-10,
    }
}

fn pstb_carrier(engine: &Engine, netlist: &Netlist) -> rspice_core::engine::PssOperatingPoint {
    engine
        .run_pss_operating_point_with_abort(
            netlist,
            rspice_core::analysis::PssConfig::new(PSTB_FUNDAMENTAL)
                .with_harmonics(8)
                .with_points_per_period(64)
                .with_tstab_periods(2),
            &NoAbort,
        )
        .expect("the driven resonator converges")
}

/// The engine entry is the carrier's monodromy put through `PstbAnalyzer` with
/// the configuration the card states, plus the probe's participation in each
/// mode shape. This pins that it is exactly that and nothing else, which is
/// what the Studio's own route produces for the same deck by way of a circuit
/// it builds itself and 260 lines of re-derivation it no longer needs.
#[test]
fn an_authored_pstb_card_reproduces_the_spectrum_its_configuration_asks_for() {
    let netlist = Netlist::parse(PSTB_RESONATOR).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let carrier = pstb_carrier(&engine, &netlist);
    let card = pstb_card("L1");

    let stability = engine
        .run_pstb_card_from_pss_with_abort(&netlist, &card, &carrier, &NoAbort)
        .expect("the authored .PSTB card runs against its carrier");

    let oracle = rspice_core::analysis::pstb::PstbAnalyzer::new(
        rspice_core::analysis::pstb::PstbConfig::new()
            .with_num_eigenvalues(card.num_multipliers)
            .with_orbit_kind(carrier.analysis().result.floquet_orbit_kind)
            .with_eigenvectors(true)
            .with_tolerance(card.eigenvalue_tolerance)
            .with_stability_threshold(card.stability_threshold)
            .with_subharmonic_detection(card.detect_subharmonics),
    )
    .analyze_monodromy_with_abort(
        &carrier.analysis().monodromy,
        carrier.analysis().period,
        &NoAbort,
    )
    .expect("the same configuration describes a runnable Floquet study");

    assert_eq!(stability.result.period, oracle.period);
    assert_eq!(stability.result.stability_verdict, oracle.stability_verdict);
    assert_eq!(stability.result.stability, oracle.stability);
    assert_eq!(stability.result.num_unstable, oracle.num_unstable);
    assert_eq!(
        stability.result.max_multiplier_magnitude,
        oracle.max_multiplier_magnitude
    );
    assert_eq!(
        stability.result.min_stability_margin_db,
        oracle.min_stability_margin_db
    );
    assert_eq!(stability.result.subharmonics, oracle.subharmonics);
    assert_eq!(stability.result.multipliers.len(), oracle.multipliers.len());
    for (mine, theirs) in stability.result.multipliers.iter().zip(&oracle.multipliers) {
        assert_eq!(mine.value, theirs.value, "the multiplier itself");
        assert_eq!(mine.exponent, theirs.exponent);
        assert_eq!(mine.is_unstable, theirs.is_unstable);
        assert_eq!(mine.is_trivial, theirs.is_trivial);
        assert_eq!(mine.subharmonic_order, theirs.subharmonic_order);
    }

    // The participation is the probe coordinate's share of each mode shape,
    // stated for every retained mode and never only the displayed ones.
    assert_eq!(
        stability.probe_participation.len(),
        stability.result.multipliers.len()
    );
    for (index, multiplier) in oracle.multipliers.iter().enumerate() {
        let vector = multiplier.eigenvector.as_ref().expect("eigenvector");
        let mut norm = 0.0_f64;
        for value in vector {
            norm = norm.hypot(value.norm());
        }
        let expected = (vector[stability.probe_state_index].norm() / norm).clamp(0.0, 1.0);
        assert_eq!(
            stability.probe_participation[index],
            expected,
            "mode {} participation",
            index + 1
        );
    }
}

/// The probe index the engine derives from the carrier's shooting-state basis
/// and the one `CircuitData` derives from a built circuit are the same number.
///
/// They are two derivations of one fact, `capacitors.len() + inductor_index`,
/// reached by different routes, and nothing but this gate stops them drifting
/// apart. The deck deliberately carries a capacitor as well as an inductor, so
/// a wrong offset cannot hide behind a zero-length capacitor block.
#[test]
fn the_basis_derived_probe_index_is_the_circuits_own_inductor_state_index() {
    let netlist = Netlist::parse(PSTB_RESONATOR).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let carrier = pstb_carrier(&engine, &netlist);

    let basis = carrier.shooting_state_basis();
    assert!(
        basis.iter().any(|name| name.starts_with("C:"))
            && basis.iter().any(|name| name.starts_with("L:")),
        "the fixture must exercise a non-zero capacitor offset: {basis:?}"
    );

    let circuit = engine.build_circuit(&netlist).expect("the circuit builds");
    let branch = circuit
        .get_branch_by_name("L1")
        .expect("the probe is a branch-capable element");
    let from_circuit = circuit
        .inductor_probe_for_branch(branch)
        .expect("the probe is an inductor");

    let stability = engine
        .run_pstb_card_from_pss_with_abort(&netlist, &pstb_card("l1"), &carrier, &NoAbort)
        .expect(".PSTB runs");

    assert_eq!(stability.probe_state_index, from_circuit.state_index);
    assert_eq!(stability.probe_instance, from_circuit.canonical_name);
    assert_eq!(basis[stability.probe_state_index], "L:L1");
    assert!(
        stability.probe_state_index < carrier.analysis().monodromy.len(),
        "the resolved coordinate must index the retained monodromy"
    );
}

/// A legacy identityless operating point carries no shooting-state basis, so
/// there is nothing to resolve a probe against. Resolving anyway would name
/// coordinate zero, which is some other circuit's capacitor, so it is a typed
/// refusal instead.
#[test]
fn a_carrier_with_no_shooting_state_basis_refuses_the_probe() {
    let netlist = Netlist::parse(PSTB_RESONATOR).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let carrier = pstb_carrier(&engine, &netlist);

    let legacy = rspice_core::engine::PssOperatingPoint::try_from_parts(
        carrier.config().clone(),
        carrier.analysis().clone(),
        carrier.shooting_state().to_vec(),
    )
    .expect("a legacy artifact is still structurally valid");
    assert!(legacy.shooting_state_basis().is_empty());

    let message = engine
        .run_pstb_card_from_pss_with_abort(&netlist, &pstb_card("L1"), &legacy, &NoAbort)
        .expect_err("an identityless carrier cannot name its own coordinates")
        .to_string();
    assert!(
        message.contains("shooting-state basis"),
        "the refusal must name what is missing: {message}"
    );
}

#[test]
fn a_pstb_probe_that_is_not_an_inductor_current_is_refused_by_name() {
    let netlist = Netlist::parse(PSTB_RESONATOR).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let carrier = pstb_carrier(&engine, &netlist);

    let missing = engine
        .run_pstb_card_from_pss_with_abort(&netlist, &pstb_card("LNOPE"), &carrier, &NoAbort)
        .expect_err("a probe the deck does not author must be refused")
        .to_string();
    assert!(
        missing.contains("branch") && missing.contains("L1"),
        "the refusal must list what the deck does offer: {missing}"
    );

    let not_inductive = engine
        .run_pstb_card_from_pss_with_abort(&netlist, &pstb_card("VIN"), &carrier, &NoAbort)
        .expect_err("a voltage source is branch-capable but carries no reactive state")
        .to_string();
    assert!(
        not_inductive.contains("inductor") && not_inductive.contains("L1"),
        "the refusal must name the inductor probes: {not_inductive}"
    );
}

#[test]
fn an_authored_pstb_card_refuses_a_carrier_it_cannot_be_read_from() {
    let netlist = Netlist::parse(PSTB_RESONATOR).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let carrier = pstb_carrier(&engine, &netlist);

    let mut coarse = pstb_card("L1");
    coarse.max_harmonics = 1_000;
    let error = engine
        .run_pstb_card_from_pss_with_abort(&netlist, &coarse, &carrier, &NoAbort)
        .expect_err("a carrier that cannot represent the requested harmonics is refused")
        .to_string();
    assert!(
        error.contains("MAXHARM") && error.contains("capacity"),
        "the refusal must name the precondition: {error}"
    );

    let mut impossible = pstb_card("L1");
    impossible.stability_threshold = 0.5;
    assert!(
        engine
            .run_pstb_card_from_pss_with_abort(&netlist, &impossible, &carrier, &NoAbort)
            .is_err(),
        "a boundary inside the unit circle would call a stable mode unstable"
    );
}

#[test]
fn an_authored_pstb_card_honours_cancellation() {
    let netlist = Netlist::parse(PSTB_RESONATOR).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let carrier = pstb_carrier(&engine, &netlist);
    let error = engine
        .run_pstb_card_from_pss_with_abort(
            &netlist,
            &pstb_card("L1"),
            &carrier,
            &rspice_core::abort_signal::ImmediateAbort,
        )
        .expect_err("an aborted .PSTB run must not publish a partial spectrum");
    assert!(matches!(
        error,
        rspice_core::engine::SimulationError::Aborted
    ));
}
