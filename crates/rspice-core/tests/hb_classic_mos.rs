//! Native classic MOS models, Meyer rate states, response and periodic noise.
use num_complex::Complex64;
use rspice_core::analysis::harmonic_balance::HbConfig;
use rspice_core::analysis::pac::{PacConfig, PacSweepType};
use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect};
use rspice_core::{Netlist, NoAbort};
use std::f64::consts::TAU;

fn engine(dialect: SpiceDialect) -> Engine {
    Engine::new(SimulationConfig {
        spice_dialect: dialect,
        ..Default::default()
    })
}

fn amplifier(level: usize, amplitude: f64, noise: bool) -> Netlist {
    let p = if level % 2 == 0 { -1.0 } else { 1.0 };
    let kind = if p < 0.0 { "PMOS" } else { "NMOS" };
    let model = if matches!(level, 4 | 5) {
        "VFB=-0.8 PHI=0.7 K1=0.5 TOX=0.02".to_string()
    } else {
        format!("VTO={} KP=50u GAMMA=0.5 PHI=0.7 TOX=20n", p * 0.6)
    };
    let terminals = if level == 3 {
        "source gate out 0"
    } else {
        "out gate source 0"
    };
    let flicker = if noise {
        "KF=2e-24 AF=1.2 EF=0.9 NLEV=3 GDSNOI=0.8 RSH=10"
    } else {
        ""
    };
    Netlist::parse(&format!(
        "Native classic MOS periodic\nVDD supply 0 {}\nVIN in 0 DC {} AC 1 SIN({} {} 100meg)\nRD supply out 500\nRG in gate 10000\nRS source 0 10\nM1 {terminals} mm L=1u W=10u AD=4p AS=5p PD=20u PS=22u M=2 OFF\n.model mm {kind} LEVEL={level} {model} CGSO=1e-9 CGDO=2e-9 CGBO=1e-9 CJ=0.001 CJSW=1e-10 MJ=0.4 MJSW=0.23 {flicker}\n.options TEMP=60\n.end\n",
        p*1.8, p*0.9, p*0.9, p*amplitude,
    )).unwrap()
}

fn pac(offset: f64) -> PacConfig {
    PacConfig::new()
        .with_fundamental(1e8)
        .with_sweep(offset, offset, 1)
        .with_sweep_type(PacSweepType::Linear)
        .with_sidebands(0, 0)
        .with_input_source("vin")
        .with_output_node("out")
        .with_tolerances(1e-9, 1e-15)
}

#[test]
fn classic_mos_periodic_response_matches_native_stationary_ac() {
    for (level, dialect) in [
        (1, SpiceDialect::Ngspice),
        (2, SpiceDialect::Ngspice),
        (3, SpiceDialect::Ngspice),
        (4, SpiceDialect::Ngspice),
        (5, SpiceDialect::Ngspice),
        (6, SpiceDialect::Ngspice),
        (9, SpiceDialect::Ngspice),
        (2, SpiceDialect::Xyce),
        (3, SpiceDialect::Xyce),
    ] {
        let engine = engine(dialect);
        let deck = amplifier(level, 0.0, false);
        let ac = engine.run_ac(&deck, &[3.7e8]).unwrap();
        let out = ac[0]
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap();
        let expected = ac[0].voltages[out];
        let response = engine
            .run_pac(&deck, pac(3.7e8))
            .unwrap_or_else(|e| panic!("level {level} {dialect:?}: {e}"));
        let actual = response.result.conversion_matrix.get(0, 0, 0).unwrap();
        assert!(
            (actual - expected).norm() < 1e-10 + expected.norm() * 3e-6,
            "level {level} {dialect:?}: {actual} vs {expected}"
        );
    }
}

fn interpolate(time: &[f64], values: &[f64], at: f64) -> f64 {
    let hi = time.partition_point(|t| *t < at).clamp(1, time.len() - 1);
    let lo = hi - 1;
    values[lo] + (values[hi] - values[lo]) * (at - time[lo]) / (time[hi] - time[lo])
}

#[test]
fn classic_mos_hb_native_storage_matches_settled_transient_and_lead_kcl() {
    for (level, dialect) in [
        (1, SpiceDialect::Ngspice),
        (3, SpiceDialect::Xyce),
        (5, SpiceDialect::Ngspice),
    ] {
        let engine = engine(dialect);
        let deck = amplifier(level, 0.1, false);
        let hb = engine
            .run_hb(
                &deck,
                HbConfig::new(1e8).with_harmonics(7).with_tolerance(1e-9),
            )
            .unwrap();
        let tran = engine.run_tran(&deck, 200e-9, 10e-9 / 512.0).unwrap();
        for name in ["gate", "out"] {
            let reference = tran.try_voltage_waveform_named(name).unwrap();
            let spectrum = hb
                .result
                .spectral_voltages
                .iter()
                .find(|row| row.node_name.eq_ignore_ascii_case(name))
                .unwrap();
            for i in 0..64 {
                let time = i as f64 / 64.0 / 1e8;
                let actual: f64 = spectrum
                    .coefficients
                    .iter()
                    .enumerate()
                    .map(|(k, c)| (c * Complex64::from_polar(1.0, TAU * 1e8 * k as f64 * time)).re)
                    .sum();
                let expected = interpolate(&tran.time, &reference, 190e-9 + time);
                assert!(
                    (actual - expected).abs() < 2e-4,
                    "level {level} {dialect:?} {name} t={time}: {actual} vs {expected}"
                );
            }
        }
        let current = |name: &str, k: usize| {
            hb.device_currents
                .iter()
                .find(|row| row.probe.eq_ignore_ascii_case(name))
                .unwrap()
                .coefficients[k]
        };
        for k in 0..=7 {
            let sum: Complex64 = ["@m1[id]", "@m1[ig]", "@m1[is]", "@m1[ib]"]
                .iter()
                .map(|name| current(name, k))
                .sum();
            assert!(sum.norm() < 1e-12);
            let vin = hb
                .result
                .mna_branch_currents
                .iter()
                .find(|row| row.device_name.eq_ignore_ascii_case("vin"))
                .unwrap();
            assert!((current("@m1[ig]", k) + vin.coefficients[k]).norm() < 1e-11);
        }
        assert!(hb.result.continuation_limitations.contains(&rspice_core::analysis::harmonic_balance::HbContinuationLimitation::ClassicMosChargeHistoryNotRetained));
    }
}

#[test]
fn classic_mos_periodic_noise_uses_native_channel_and_flicker_laws() {
    let offsets = [1e3, 1e7, 3.7e8];
    for (level, dialect) in [
        (1, SpiceDialect::Ngspice),
        (3, SpiceDialect::Xyce),
        (5, SpiceDialect::Ngspice),
    ] {
        let engine = engine(dialect);
        let deck = amplifier(level, 0.0, true);
        let reference = engine
            .run_noise_named_with_input_source_and_abort(
                &deck, "out", None, "vin", &offsets, 333.15, &NoAbort,
            )
            .unwrap();
        let hb = engine
            .run_hb(
                &deck,
                HbConfig::new(1e8).with_harmonics(3).with_tolerance(1e-9),
            )
            .unwrap();
        let periodic = engine
            .run_pnoise_from_hb_with_abort(
                &deck,
                &offsets,
                "out",
                None,
                Some("vin"),
                0,
                &hb.operating_point,
                &NoAbort,
            )
            .unwrap();
        for (i, stationary) in reference.iter().enumerate() {
            assert!(
                (periodic.output_noise[i] / stationary.output_noise_density - 1.0).abs() < 3e-5,
                "level {level} {dialect:?} f={}: {} vs {}",
                offsets[i],
                periodic.output_noise[i],
                stationary.output_noise_density
            );
            for contribution in &stationary.contributions {
                if !contribution.identity.device.eq_ignore_ascii_case("m1") {
                    continue;
                }
                let label = format!("m1:{}", contribution.identity.mechanism.as_ref().unwrap());
                let actual = periodic
                    .contributors
                    .iter()
                    .find(|(name, _)| name.eq_ignore_ascii_case(&label))
                    .unwrap()
                    .1[i];
                assert!(
                    (actual - contribution.output_contribution).abs()
                        < 1e-34 + 3e-5 * contribution.output_contribution,
                    "level {level} {label} f={}: {actual} vs {}",
                    offsets[i],
                    contribution.output_contribution
                );
            }
        }
    }
}

#[test]
fn classic_mos_driven_rates_feed_shooting_and_quasi_periodic_responses() {
    use rspice_core::analysis::PssConfig;
    use rspice_core::engine::{
        QpacRequest, QpnoiseFrequencyAxis, QpnoiseInput, QpnoiseLattices, QpnoiseObservation,
        QpnoiseOutput, QpnoiseRequest, QpnoiseSources, QpssConfig,
    };
    let engine = engine(SpiceDialect::Ngspice);
    let deck = amplifier(1, 0.03, true);
    let offset = 1.03e7;
    let hb = engine
        .run_hb(
            &deck,
            HbConfig::new(1e8).with_harmonics(6).with_tolerance(1e-9),
        )
        .unwrap();
    let config = pac(offset).with_sidebands(-3, 3);
    let response = engine
        .run_pac_from_hb_with_abort(&deck, config.clone(), &hb.operating_point, &NoAbort)
        .unwrap();
    let pss = engine
        .run_pss_operating_point_with_abort(
            &deck,
            PssConfig::new(1e8)
                .with_points_per_period(128)
                .with_tstab_periods(0),
            &NoAbort,
        )
        .unwrap();
    let shooting = engine
        .run_pac_from_pss_with_abort(&deck, config, &pss, &NoAbort)
        .unwrap();
    let mut qconfig = QpssConfig::new(vec![1e8, 1e8 * std::f64::consts::SQRT_2], vec![3, 1]);
    qconfig.grid.sampling =
        rspice_core::analysis::quasi_periodic::QuasiPeriodicSampling::Exact(vec![16, 8]);
    qconfig.solver.relative_tolerance = 1e-9;
    let quasi = engine.run_qpss(&deck, qconfig).unwrap();
    for sideband in [-1, 0, 1] {
        let expected = response
            .result
            .conversion_matrix
            .get(0, sideband, 0)
            .unwrap();
        assert!(expected.norm() > 1e-5, "carrier must mix sidebands");
        let sampled = shooting
            .result
            .conversion_matrix
            .get(0, sideband, 0)
            .unwrap();
        assert!(
            (sampled - expected).norm() < 2e-6 + 3e-3 * expected.norm(),
            "PSS sideband {sideband}: {sampled} vs {expected}"
        );
        let qpac = engine
            .run_qpac_from_qpss(
                &deck,
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
                &quasi,
            )
            .unwrap();
        assert!((qpac.output_transfer[0] - expected).norm() < 2e-6 + 3e-3 * expected.norm());
    }
    let periodic_noise = engine
        .run_pnoise_from_hb_with_abort(
            &deck,
            &[offset],
            "out",
            None,
            Some("vin"),
            3,
            &hb.operating_point,
            &NoAbort,
        )
        .unwrap();
    let noise = engine
        .run_qpnoise_from_qpss(
            &deck,
            QpnoiseRequest {
                frequencies_hz: vec![offset],
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
            &quasi,
        )
        .unwrap();
    assert!(
        (noise.total_covariances[0].values[0].re / periodic_noise.output_noise[0] - 1.0).abs()
            < 3e-3
    );
}
