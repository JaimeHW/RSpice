//! Native BSIM4 carrier, lead-current and periodic-response integration.

use num_complex::Complex64;
use rspice_core::analysis::harmonic_balance::HbConfig;
use rspice_core::analysis::pac::{PacConfig, PacSweepType};
use rspice_core::engine::{Engine, HbAnalysisResult};
use rspice_core::netlist::Netlist;
use std::f64::consts::TAU;

const MODELS: &str = include_str!("../src/device/mosfet/bsim4v8/testdata/models45.lib");

fn pac_config(f: f64) -> PacConfig {
    PacConfig::new()
        .with_fundamental(f)
        .with_sweep(f, f, 1)
        .with_sweep_type(PacSweepType::Linear)
        .with_sidebands(0, 0)
        .with_input_source("vin")
        .with_output_node("out")
        .with_tolerances(1e-9, 1e-15)
}

fn run_hb(netlist: &Netlist, f: f64) -> HbAnalysisResult {
    Engine::default()
        .run_hb(
            netlist,
            HbConfig::new(f).with_harmonics(3).with_tolerance(1e-9),
        )
        .unwrap()
}

fn current<'a>(result: &'a HbAnalysisResult, probe: &str) -> &'a [Complex64] {
    &result
        .device_currents
        .iter()
        .find(|value| value.probe.eq_ignore_ascii_case(probe))
        .unwrap_or_else(|| panic!("missing {probe}"))
        .coefficients
}

#[track_caller]
fn close(actual: Complex64, expected: Complex64, tolerance: f64) {
    assert!(
        (actual - expected).norm() <= 1e-12 + tolerance * expected.norm(),
        "actual={actual}, expected={expected}"
    );
}

fn amplifier(options: &str, pmos: bool, waveform: &str) -> String {
    let (supply, bias, model) = if pmos {
        (-1.2, -0.7, "p90")
    } else {
        (1.2, 0.7, "n45")
    };
    format!(
        "BSIM4 periodic amplifier\nvdd supply 0 dc {supply}\nrd supply out 1k\n\
        vin in 0 dc {bias} {waveform}\n\
        m1 out in 0 0 {model} w=1u l=90n nf=2 m=2 ad=0.2p as=0.3p pd=4.4u ps=5u nrd=1 nrs=1\n\
        {}\n.end\n",
        MODELS.replace(
            "level=54 version=4.8",
            &format!("level=54 version=4.8 {options}")
        )
    )
}

#[test]
fn bsim4_periodic_qs_hb_leads_and_pac_match_ac_through_internal_networks() {
    let f = 1e9;
    let amplitude = 1e-5;
    let engine = Engine::default();
    for options in [
        "",
        "rgatemod=1 rshg=1000",
        "rgatemod=2 rdsmod=1 rbodymod=1 rshg=1000",
        "rgatemod=3 rdsmod=1 rbodymod=2 rshg=1000",
    ] {
        for pmos in [false, true] {
            let bias = if pmos { -0.7 } else { 0.7 };
            let deck = amplifier(options, pmos, "ac 1");
            let ac = engine
                .run_ac(&Netlist::parse(&deck).unwrap(), &[f])
                .unwrap();
            let out = ac[0]
                .node_names
                .iter()
                .position(|n| n.eq_ignore_ascii_case("out"))
                .unwrap();
            let expected = ac[0].voltages[out];
            let netlist =
                Netlist::parse(&deck.replace("ac 1", &format!("sin({bias} {amplitude} {f})")))
                    .unwrap();
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
                3e-4,
            );
            for harmonic in 0..=3 {
                let sum: Complex64 = ["@m1[id]", "@m1[ig]", "@m1[is]", "@m1[ib]"]
                    .iter()
                    .map(|probe| current(&hb, probe)[harmonic])
                    .sum();
                assert!(sum.norm() < 1e-12);
                let input = hb
                    .result
                    .mna_branch_currents
                    .iter()
                    .find(|b| b.device_name.eq_ignore_ascii_case("vin"))
                    .unwrap();
                close(
                    current(&hb, "@m1[ig]")[harmonic],
                    -input.coefficients[harmonic],
                    2e-5,
                );
                let supply = hb
                    .result
                    .mna_branch_currents
                    .iter()
                    .find(|b| b.device_name.eq_ignore_ascii_case("vdd"))
                    .unwrap();
                close(
                    current(&hb, "@m1[id]")[harmonic],
                    -supply.coefficients[harmonic],
                    2e-5,
                );
            }
            let pac = engine
                .run_pac(
                    &Netlist::parse(&deck.replace("ac 1", "")).unwrap(),
                    pac_config(f),
                )
                .unwrap();
            close(
                pac.result.conversion_matrix.get(0, 0, 0).unwrap(),
                expected,
                2e-6,
            );
            assert!(hb.result.continuation_limitations.contains(
                &rspice_core::analysis::harmonic_balance::HbContinuationLimitation::Bsim4ChargeHistoryNotRetained));
        }
    }
}

#[test]
fn bsim4_periodic_nqs_solves_physical_deficit_and_gate_current() {
    let f = 1e10;
    let amplitude = 1e-5;
    let deck = format!(
        "BSIM4 NQS\nvd d 0 dc 0.8\nvin g 0 dc 0.7 sin(0.7 {amplitude} {f})\n\
        m1 d g 0 0 n45 w=1u l=90n nf=2 m=2 nrd=0 nrs=0\n{}\n.end\n",
        MODELS.replace("level=54 version=4.8", "level=54 version=4.8 trnqsmod=1")
    );
    let netlist = Netlist::parse(&deck).unwrap();
    use rspice_core::device::mosfet::bsim4v8::{Bsim4v8, Bsim4v8Geometry, Bsim4v8Model};
    let card = netlist
        .models
        .iter()
        .find(|m| m.name.eq_ignore_ascii_case("n45"))
        .unwrap();
    let params = card
        .params
        .iter()
        .map(|(name, value)| (name.to_ascii_uppercase(), *value))
        .collect();
    let model = std::sync::Arc::new(Bsim4v8Model::from_params(&params, false, 300.15));
    let device = Bsim4v8::new(
        "oracle".into(),
        model,
        Bsim4v8Geometry {
            w: 1e-6,
            l: 90e-9,
            nf: 2.0,
            m: 2.0,
            drain_squares: 0.0,
            source_squares: 0.0,
            drain_squares_given: true,
            source_squares_given: true,
            ..Bsim4v8Geometry::default()
        },
        300.15,
    )
    .unwrap();
    let op = device
        .eval(
            rspice_core::device::mosfet::bsim4v8::Bsim4v8Bias {
                vds: 0.8,
                vgs: 0.7,
                vbs: 0.0,
            },
            1e-12,
            true,
        )
        .unwrap();
    let charge = op.charge.as_ref().unwrap();
    let rate = 1e-9 / charge.taunet;
    let cqgb = -(charge.cggb + charge.cbgb);
    let drive = Complex64::new(0.0, -amplitude);
    let jw = Complex64::new(0.0, TAU * f);
    let expected_deficit = jw * cqgb * drive / (rate + jw * 1e-9);
    let expected_gate = device.geom.m
        * (jw * (charge.cgdo + charge.cgso + charge.cgbo) * drive - rate * expected_deficit);
    let hb = run_hb(&netlist, f);
    let deficit = hb
        .result
        .spectral_voltages
        .iter()
        .find(|n| n.node_name.eq_ignore_ascii_case("m1.__charge"))
        .unwrap();
    close(deficit.coefficients[1], expected_deficit, 3e-4);
    close(current(&hb, "@m1[ig]")[1], expected_gate, 3e-4);
    let input = hb
        .result
        .mna_branch_currents
        .iter()
        .find(|b| b.device_name.eq_ignore_ascii_case("vin"))
        .unwrap();
    close(input.coefficients[1], -expected_gate, 3e-4);
}

#[test]
fn bsim4_periodic_ac_nqs_pac_and_quasiperiodic_responses_match_ac() {
    use rspice_core::engine::{
        QpacRequest, QpssConfig, QpxfFrequencyAxis, QpxfInputLattices, QpxfOutput, QpxfRequest,
        QpxfSources,
    };
    let f = 1e10;
    let engine = Engine::default();
    for nqs in [0, 1] {
        let deck = amplifier(
            &format!("acnqsmod=1 trnqsmod={nqs} rgatemod=3 rdsmod=1 rbodymod=2 rshg=1000"),
            false,
            "ac 1",
        );
        let ac = engine
            .run_ac(&Netlist::parse(&deck).unwrap(), &[f])
            .unwrap();
        let out = ac[0]
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .unwrap();
        let expected = ac[0].voltages[out];
        let netlist = Netlist::parse(&deck.replace("ac 1", "")).unwrap();
        let pac = engine.run_pac(&netlist, pac_config(f)).unwrap();
        close(
            pac.result.conversion_matrix.get(0, 0, 0).unwrap(),
            expected,
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
        close(qpac.output_transfer[0], expected, 2e-6);
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
        close(qpxf.transfers[0].values[0], expected, 2e-6);
    }
}

#[test]
fn bsim4_periodic_driven_ac_nqs_sidebands_agree_between_hb_and_qpss() {
    use rspice_core::engine::{QpacRequest, QpssConfig};
    let fundamental = 1e9;
    let offset = 1.03e10;
    let netlist = Netlist::parse(&amplifier(
        "acnqsmod=1 trnqsmod=1 rgatemod=3 rdsmod=1 rbodymod=1 rshg=1000",
        false,
        &format!("sin(0.7 0.03 {fundamental})"),
    ))
    .unwrap();
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
    let mut config = QpssConfig::new(
        vec![fundamental, fundamental * std::f64::consts::SQRT_2],
        vec![3, 1],
    );
    config.grid.sampling =
        rspice_core::analysis::quasi_periodic::QuasiPeriodicSampling::Exact(vec![16, 8]);
    config.solver.relative_tolerance = 1e-9;
    let point = engine.run_qpss(&netlist, config).unwrap();
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
            "the driven carrier must mix sidebands"
        );
        close(response.output_transfer[0], expected, 2e-3);
    }
}
