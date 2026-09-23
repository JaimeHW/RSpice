//! Native BSIM3 periodic wiring: ordinary AC limit and explicit NQS dynamics.

use num_complex::Complex64;
use rspice_core::analysis::harmonic_balance::HbConfig;
use rspice_core::analysis::pac::{PacConfig, PacSweepType};
use rspice_core::device::mosfet::bsim3v3::{Bsim3v3, Bsim3v3Bias, Bsim3v3Geometry, Bsim3v3Model};
use rspice_core::engine::{Engine, HbAnalysisResult};
use rspice_core::netlist::Netlist;
use std::f64::consts::TAU;

const MODELS: &str = include_str!("../src/device/mosfet/bsim3v3/testdata/models018.lib");

fn pac_config(f: f64) -> PacConfig {
    PacConfig::new()
        .with_fundamental(f)
        .with_sweep(f, f, 1)
        .with_sweep_type(PacSweepType::Linear)
        .with_sidebands(0, 0)
        .with_input_source("vin")
        .with_output_node("out")
}

fn run_hb(netlist: &Netlist, f: f64) -> HbAnalysisResult {
    Engine::default()
        .run_hb(
            netlist,
            HbConfig::new(f).with_harmonics(3).with_tolerance(1e-9),
        )
        .expect("native BSIM3 HB converges")
}

fn current<'a>(result: &'a HbAnalysisResult, probe: &str) -> &'a [Complex64] {
    &result
        .device_currents
        .iter()
        .find(|value| value.probe.eq_ignore_ascii_case(probe))
        .unwrap_or_else(|| panic!("missing {probe}"))
        .coefficients
}

fn close(actual: Complex64, expected: Complex64, tolerance: f64) {
    assert!(
        (actual - expected).norm() <= 1e-12 + tolerance * expected.norm(),
        "actual={actual}, expected={expected}"
    );
}

#[test]
fn bsim3_periodic_qs_hb_and_pac_match_native_ac_with_series_resistors() {
    let f = 1e8;
    let amplitude = 1e-4;
    let models = MODELS.replace("rsh=0", "rsh=20");
    let deck = format!(
        "Native BSIM3 periodic AC limit\n\
        vdd supply 0 dc 1.8\nrd supply out 3k\n\
        vin in 0 dc 0.95 ac 1 sin(0.95 {amplitude} {f})\n\
        m1 out in 0 0 n018 w=1u l=0.18u ad=0.42p as=0.42p pd=2.84u ps=2.84u nrd=3 nrs=4 m=2\n\
        {models}\n.end\n"
    );
    let netlist = Netlist::parse(&deck).unwrap();
    let engine = Engine::default();
    let ac = engine.run_ac(&netlist, &[f]).unwrap();
    let out = ac[0]
        .node_names
        .iter()
        .position(|n| n.eq_ignore_ascii_case("out"))
        .unwrap();
    let expected = ac[0].voltages[out];
    let hb = run_hb(&netlist, f);
    let out = hb
        .result
        .spectral_voltages
        .iter()
        .find(|n| n.node_name.eq_ignore_ascii_case("out"))
        .unwrap();
    close(
        out.coefficients[1] / Complex64::new(0.0, -amplitude),
        expected,
        2e-4,
    );
    // PAC is linearized about the DC carrier, with the same resolved model.
    let dc_carrier = Netlist::parse(
        &deck
            .replace(&format!("sin(0.95 {amplitude} {f})"), "")
            .replace(" ac 1", ""),
    )
    .unwrap();
    let pac = engine.run_pac(&dc_carrier, pac_config(f)).unwrap();
    close(
        pac.result.conversion_matrix.get(0, 0, 0).unwrap(),
        expected,
        2e-6,
    );
    for harmonic in 0..=3 {
        let leads: Complex64 = ["@m1[id]", "@m1[ig]", "@m1[is]", "@m1[ib]"]
            .iter()
            .map(|probe| current(&hb, probe)[harmonic])
            .sum();
        assert!(leads.norm() < 1e-12);
    }
    assert!(current(&hb, "@m1[ig]")[1].norm() > 1e-10);
    assert!(
        !hb.result.continuation_limitations.is_empty(),
        "HB alone must not claim the not-yet-projected BSIM3 transient history"
    );
}

#[test]
fn bsim3_periodic_nqs_solves_hidden_charge_and_terminal_displacement_currents() {
    let f = 1e9;
    let amplitude = 1e-4;
    let models = MODELS.replace("capmod=3", "capmod=2 nqsmod=1 acnqsmod=0");
    let netlist = Netlist::parse(&format!(
        "Native BSIM3 NQS periodic state\n\
        vd d 0 dc 0.8\nvin g 0 dc 0.9 sin(0.9 {amplitude} {f})\n\
        m1 d g 0 0 n018 w=10u l=0.18u ad=4.2p as=4.2p pd=20.84u ps=20.84u m=2\n\
        {models}\n.end\n"
    ))
    .unwrap();
    let card = netlist
        .models
        .iter()
        .find(|model| model.name.eq_ignore_ascii_case("n018"))
        .unwrap();
    let params = card
        .params
        .iter()
        .map(|(name, value)| (name.to_ascii_uppercase(), *value))
        .collect();
    let model = std::sync::Arc::new(Bsim3v3Model::from_params(&params, false, 300.15));
    let device = Bsim3v3::new(
        "oracle".into(),
        model,
        Bsim3v3Geometry {
            l: 0.18e-6,
            w: 10e-6,
            drain_area: 4.2e-12,
            source_area: 4.2e-12,
            drain_perimeter: 20.84e-6,
            source_perimeter: 20.84e-6,
            ..Bsim3v3Geometry::default()
        },
        300.15,
    )
    .unwrap();
    let op = device
        .eval(
            Bsim3v3Bias {
                vds: 0.8,
                vgs: 0.9,
                vbs: 0.0,
            },
            1e-12,
            true,
        )
        .unwrap();
    let charge = op.charge.as_ref().unwrap();
    let drive = Complex64::new(0.0, -amplitude);
    let jw = Complex64::new(0.0, TAU * f);
    // Linearized physical deficit equation: (gtau+jw*1n)*vq = jw*cqgb*vg.
    let expected_deficit = jw * charge.cqgb * drive / (charge.gtau + jw * 1e-9);
    let expected_gate = 2.0
        * (jw * (charge.cgdo + charge.cgso + charge.cgbo) * drive - charge.gtau * expected_deficit);
    let hb = run_hb(&netlist, f);
    let deficit = hb
        .result
        .spectral_voltages
        .iter()
        .find(|n| n.node_name.eq_ignore_ascii_case("m1.__charge"))
        .unwrap();
    close(deficit.coefficients[1], expected_deficit, 2e-4);
    close(current(&hb, "@m1[ig]")[1], expected_gate, 2e-4);
    let branch = hb
        .result
        .mna_branch_currents
        .iter()
        .find(|b| b.device_name.eq_ignore_ascii_case("vin"))
        .unwrap();
    close(branch.coefficients[1], -expected_gate, 2e-4);
    for harmonic in 0..=3 {
        let leads: Complex64 = ["@m1[id]", "@m1[ig]", "@m1[is]", "@m1[ib]"]
            .iter()
            .map(|probe| current(&hb, probe)[harmonic])
            .sum();
        assert!(leads.norm() < 1e-12);
    }
}

#[test]
fn bsim3_periodic_ac_only_nqs_response_matches_native_ac() {
    use rspice_core::engine::{
        QpacRequest, QpssConfig, QpxfFrequencyAxis, QpxfInputLattices, QpxfOutput, QpxfRequest,
        QpxfSources,
    };
    let f = 1e10;
    for nqs in [0, 1] {
        let deck = format!(
            "BSIM3 AC-only NQS contract\n\
        vdd supply 0 dc 1.8\nrd supply out 3k\nvin in 0 dc 0.95 ac 1\n\
        m1 out in 0 0 n018 w=1u l=0.18u\n\
        {}\n.end\n",
            MODELS.replace("capmod=3", &format!("capmod=3 acnqsmod=1 nqsmod={nqs}"))
        );
        let netlist = Netlist::parse(&deck.replace(" ac 1", "")).unwrap();
        let ac_netlist = Netlist::parse(&deck).unwrap();
        assert!(run_hb(&netlist, f).converged);
        let engine = Engine::default();
        let ac = engine.run_ac(&ac_netlist, &[f]).unwrap();
        let out = ac[0]
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap();
        let pac = engine.run_pac(&netlist, pac_config(f)).unwrap();
        close(
            pac.result.conversion_matrix.get(0, 0, 0).unwrap(),
            ac[0].voltages[out],
            2e-6,
        );
        let pss = engine
            .run_pss_operating_point_with_abort(
                &netlist,
                rspice_core::analysis::PssConfig::new(f)
                    .with_points_per_period(64)
                    .with_tstab_periods(0),
                &rspice_core::NoAbort,
            )
            .unwrap();
        let from_pss = engine
            .run_pac_from_pss_with_abort(&netlist, pac_config(f), &pss, &rspice_core::NoAbort)
            .unwrap();
        close(
            from_pss.result.conversion_matrix.get(0, 0, 0).unwrap(),
            ac[0].voltages[out],
            2e-6,
        );
        let point = engine
            .run_qpss(
                &netlist,
                QpssConfig::new(vec![1e8, 1e8 * std::f64::consts::SQRT_2], vec![1, 1]),
            )
            .unwrap();
        let qpac = engine
            .run_qpac_from_qpss(
                &netlist,
                QpacRequest {
                    offsets_hz: vec![f],
                    input_source: "vin".into(),
                    input_lattice: vec![0, 0],
                    output_node: "out".into(),
                    output_ref: "0".into(),
                    output_lattice: vec![0, 0],
                    magnitude: 1.0,
                    phase_degrees: 0.0,
                    solver: Default::default(),
                },
                &point,
            )
            .unwrap();
        close(qpac.output_transfer[0], ac[0].voltages[out], 2e-6);
        let qpxf = engine
            .run_qpxf_from_qpss(
                &netlist,
                QpxfRequest {
                    frequencies_hz: vec![f],
                    frequency_axis: QpxfFrequencyAxis::Offset,
                    input_sources: QpxfSources::Named(vec!["vin".into()]),
                    input_lattices: QpxfInputLattices::Explicit(vec![vec![0, 0]]),
                    output: QpxfOutput::Voltage {
                        positive: "out".into(),
                        negative: "0".into(),
                    },
                    output_lattice: vec![0, 0],
                    linear: Default::default(),
                    group_delay: false,
                    group_delay_magnitude_floor: 0.0,
                },
                &point,
            )
            .unwrap();
        close(qpxf.transfers[0].values[0], ac[0].voltages[out], 2e-6);
        let noise = engine
            .run_noise_named_with_input_source_and_abort(
                &ac_netlist,
                "out",
                None,
                "vin",
                &[f],
                300.15,
                &rspice_core::NoAbort,
            )
            .unwrap();
        let periodic_noise = engine
            .run_pnoise(&netlist, 1e8, &[f], "out", None, Some("vin"), 0)
            .unwrap();
        assert!(
            (periodic_noise.output_noise[0] / noise[0].output_noise_density - 1.0).abs() < 2e-6
        );
        if nqs == 1 {
            use rspice_core::engine::{
                QpnoiseFrequencyAxis, QpnoiseInput, QpnoiseLattices, QpnoiseObservation,
                QpnoiseOutput, QpnoiseRequest, QpnoiseSources,
            };
            let noise = engine
                .run_qpnoise_from_qpss(
                    &netlist,
                    QpnoiseRequest {
                        frequencies_hz: vec![f],
                        frequency_axis: QpnoiseFrequencyAxis::Offset,
                        outputs: vec![QpnoiseOutput {
                            observation: QpnoiseObservation::Voltage {
                                positive: "out".into(),
                                negative: "0".into(),
                            },
                            lattice: vec![0, 0],
                        }],
                        input: Some(QpnoiseInput {
                            source: "vin".into(),
                            lattice: vec![0, 0],
                        }),
                        input_lattices: QpnoiseLattices::AllRetained,
                        sources: QpnoiseSources::All,
                        integration: None,
                        contributor_ranking: false,
                        noise_figure: None,
                        linear: Default::default(),
                    },
                    &point,
                )
                .unwrap();
            assert!(
                (noise.total_covariances[0].values[0].re / periodic_noise.output_noise[0] - 1.0)
                    .abs()
                    < 2e-6
            );
        }
    }
}

#[test]
fn bsim3_ac_only_nqs_driven_sidebands_match_quasiperiodic_response() {
    use rspice_core::engine::{QpacRequest, QpssConfig};
    let fundamental = 1e9;
    let offset = 1.03e10;
    let netlist = Netlist::parse(&format!(
        "Driven AC-only NQS\nVDD supply 0 1.8\nRD supply out 3k\nVIN in 0 SIN(.95 .08 {fundamental})\nM1 out in 0 0 n018 W=1u L=.18u\n{}\n.options hbint tahb=0\n.end\n",
        MODELS.replace("capmod=3", "capmod=3 acnqsmod=1")
    )).unwrap();
    let engine = Engine::default();
    let hb = engine
        .run_hb(
            &netlist,
            HbConfig::new(fundamental)
                .with_harmonics(6)
                .with_tolerance(1e-9),
        )
        .unwrap();
    let pac = engine
        .run_pac_from_hb_with_abort(
            &netlist,
            pac_config(offset)
                .with_fundamental(fundamental)
                .with_sidebands(-3, 3),
            &hb.operating_point,
            &rspice_core::NoAbort,
        )
        .unwrap();
    let mut qpss = QpssConfig::new(
        vec![fundamental, fundamental * std::f64::consts::SQRT_2],
        vec![3, 1],
    );
    qpss.grid.sampling =
        rspice_core::analysis::quasi_periodic::QuasiPeriodicSampling::Exact(vec![16, 8]);
    let point = engine.run_qpss(&netlist, qpss).unwrap();
    for sideband in [-1, 0, 1] {
        let response = engine
            .run_qpac_from_qpss(
                &netlist,
                QpacRequest {
                    offsets_hz: vec![offset],
                    input_source: "vin".into(),
                    input_lattice: vec![0, 0],
                    output_node: "out".into(),
                    output_ref: "0".into(),
                    output_lattice: vec![sideband, 0],
                    magnitude: 1.0,
                    phase_degrees: 0.0,
                    solver: Default::default(),
                },
                &point,
            )
            .unwrap();
        let expected = pac.result.conversion_matrix.get(0, sideband, 0).unwrap();
        assert!(
            expected.norm() > 1e-4,
            "the pumped response must mix sidebands"
        );
        close(response.output_transfer[0], expected, 2e-3);
    }
}

#[test]
fn bsim3_periodic_two_tone_qpss_preserves_independent_frequency_response() {
    use rspice_core::NoAbort;
    use rspice_core::analysis::quasi_periodic::{QuasiPeriodicGrid, QuasiPeriodicSampling};
    use rspice_core::engine::{QpssConfig, QpssInitialState};
    let frequencies = [1e8, 1e8 * std::f64::consts::SQRT_2];
    let amplitude = 1e-4;
    let netlist = Netlist::parse(&format!(
        "Native BSIM3 two-tone response\n\
        vdd supply 0 dc 1.8\nrd supply out 3k\n\
        vin in mid dc 0.95 ac 1 sin(0.95 {amplitude} {})\n\
        vsecond mid 0 dc 0 sin(0 {amplitude} {})\n\
        m1 out in 0 0 n018 w=1u l=0.18u m=2\n\
        {MODELS}\n.end\n",
        frequencies[0], frequencies[1]
    ))
    .unwrap();
    let engine = Engine::default();
    let ac = engine.run_ac(&netlist, &frequencies).unwrap();
    let mut config = QpssConfig::new(frequencies.to_vec(), vec![2, 2]);
    config.initial_state = QpssInitialState::DcOperatingPoint;
    config.grid.sampling = QuasiPeriodicSampling::Exact(vec![8, 8]);
    config.solver.relative_tolerance = 1e-9;
    let grid =
        QuasiPeriodicGrid::new_with_abort(config.grid.clone(), &Default::default(), &NoAbort)
            .unwrap();
    let point = engine.run_qpss(&netlist, config).unwrap();
    let row = point
        .node_names()
        .iter()
        .position(|n| n.eq_ignore_ascii_case("out"))
        .unwrap();
    for (tone, tuple) in [[1, 0], [0, 1]].into_iter().enumerate() {
        let out = ac[tone]
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .unwrap();
        let actual = point.spectra()[row][grid.index_of(&tuple).unwrap()]
            / Complex64::new(0.0, -0.5 * amplitude);
        close(actual, ac[tone].voltages[out], 2e-4);
    }
}

#[test]
fn bsim3_periodic_noise_matches_stationary_noise_for_every_model_selector() {
    let offsets = [1e3, 1e5];
    let engine = Engine::new(rspice_core::engine::SimulationConfig {
        temperature: 348.15,
        ..Default::default()
    });
    for selector in 1..=6 {
        let models = MODELS
            .replace(
                "level=49",
                &format!("level=49 noimod={selector} kf=2e-24 af=1.2 ef=0.9"),
            )
            .replace("rsh=0", "rsh=20");
        let netlist = Netlist::parse(&format!(
            "Native BSIM3 periodic noise\n\
            vdd supply 0 dc 1.8\nrd supply out 3k\n\
            vin in 0 dc 0.95 ac 1 sin(0.95 0 1meg)\n\
            m1 out in 0 0 n018 w=1u l=0.18u nrd=3 nrs=4 m=2\n\
            {models}\n.end\n"
        ))
        .unwrap();
        let reference = engine
            .run_noise_named_with_input_source_and_abort(
                &netlist,
                "out",
                None,
                "vin",
                &offsets,
                348.15,
                &rspice_core::NoAbort,
            )
            .unwrap();
        let periodic = engine
            .run_pnoise(&netlist, 1e6, &offsets, "out", None, Some("vin"), 0)
            .unwrap();
        for (&actual, expected) in periodic.output_noise.iter().zip(&reference) {
            assert!(
                (actual / expected.output_noise_density - 1.0).abs() < 2e-6,
                "NOIMOD={selector}: periodic={actual:e}, stationary={:e}",
                expected.output_noise_density
            );
        }
    }
}
