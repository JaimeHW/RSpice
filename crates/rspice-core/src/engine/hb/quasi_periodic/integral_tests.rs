//! Retained integral coordinates through the public QPSS and dependent APIs.
use super::*;
use crate::analysis::quasi_periodic::{QuasiPeriodicAcConfig, QuasiPeriodicLinearMethod};

#[test]
fn qpss_integrals_retain_typed_states_and_feed_ac_transfer_and_noise() {
    let deck = "coupled integral torus
Vprobe src 0 SIN(.7 .8 1k)
Rnoise src in 1k
Rbias in 0 1k
BV out 0 V=.2+1k*sdt(v(in)-v(out)+.1*sin(2*pi*1k*time)*cos(2*pi*1414.2135623730951*time))
Rout out 0 1k
BI 0 sink I=1k*sdt(-i(BV)-v(sink)/1k)
Rsink sink 0 1k
.end
";
    let netlist = Netlist::parse(deck).unwrap();
    let engine = Engine::default();
    let mut config = QpssConfig::new(vec![1e3, 1e3 * std::f64::consts::SQRT_2], vec![1, 1]);
    config.solver.linear.method = QuasiPeriodicLinearMethod::Krylov;
    config.solver.relative_tolerance = 1e-9;
    config.solver.current_absolute_tolerance = 1e-14;
    config.initial_state = QpssInitialState::DcOperatingPoint;
    let point = engine.run_qpss(&netlist, config.clone()).unwrap();
    let grid = engine
        .validate_qpss_operating_point_with_abort(&netlist, &point, &NoAbort)
        .unwrap();
    let physical = point.node_names().len() + point.branch_names().len();
    assert_eq!(point.integral_names(), ["B:BV:sdt:0", "B:BI:sdt:0"]);
    assert_eq!(point.spectra().len(), physical);
    assert_eq!(point.complete_spectra().len(), physical + 2);
    assert_eq!(
        point.integral_spectra(),
        &point.complete_spectra()[physical..]
    );
    let close = |a: Complex64, b: Complex64, scale: Value| {
        assert!((a - b).norm() < scale * 1e-8, "{a} != {b}")
    };
    close(
        point.integral_spectra()[0][grid.dc_index()],
        Complex64::new(0.00015, 0.0),
        1e-3,
    );
    close(
        point.integral_spectra()[1][grid.dc_index()],
        Complex64::new(0.00000035, 0.0),
        1e-6,
    );
    let h = |f| 1e3 / Complex64::new(1e3, std::f64::consts::TAU * f);
    let out = point
        .node_names()
        .iter()
        .position(|n| n.eq_ignore_ascii_case("out"))
        .unwrap();
    let sink = point
        .node_names()
        .iter()
        .position(|n| n.eq_ignore_ascii_case("sink"))
        .unwrap();
    for (k, tuple) in grid.indices().iter().enumerate() {
        let drive = match tuple.as_slice() {
            [0, 0] => Complex64::new(0.35, 0.0),
            [n, 0] => Complex64::new(0.0, -0.2 * *n as Value),
            [a, b] if a.abs() == 1 && b.abs() == 1 => Complex64::new(0.0, -0.025 * *a as Value),
            _ => Complex64::ZERO,
        };
        let expected = h(grid.frequencies_hz()[k]) * drive;
        close(point.spectra()[out][k], expected, 1.0);
        close(
            point.spectra()[sink][k],
            h(grid.frequencies_hz()[k]) * expected,
            1.0,
        );
    }
    let limits = crate::ResourceLimits::default();
    let (metadata, rows) = point.clone().into_transfer_parts();
    metadata
        .validate_transfer_layout_with_abort(
            &rows.iter().map(Vec::len).collect::<Vec<_>>(),
            &limits,
            &NoAbort,
        )
        .unwrap();
    let restored = QpssOperatingPoint::from_transfer_parts_with_abort(
        metadata.clone(),
        rows.clone(),
        &limits,
        &NoAbort,
    )
    .unwrap();
    assert_eq!(point, restored);
    let mut changed = rows.clone();
    changed[physical][grid.dc_index()].re += 1e-6;
    assert!(
        QpssOperatingPoint::from_transfer_parts_with_abort(
            metadata.clone(),
            changed,
            &limits,
            &NoAbort
        )
        .is_err()
    );
    assert!(
        QpssOperatingPoint::from_transfer_parts_with_abort(
            metadata,
            rows[..physical].to_vec(),
            &limits,
            &NoAbort
        )
        .is_err()
    );
    let encoded = serde_json::to_value(&point).unwrap();
    assert_eq!(encoded["version"], 2);
    for change in ["rename", "remove", "reorder", "version"] {
        let mut altered = encoded.clone();
        match change {
            "rename" => altered["integral_names"][0] = serde_json::json!("B:OTHER:sdt:0"),
            "remove" => {
                altered.as_object_mut().unwrap().remove("integral_names");
            }
            "reorder" => altered["integral_names"].as_array_mut().unwrap().swap(0, 1),
            _ => altered["version"] = serde_json::json!(1),
        }
        let altered: QpssOperatingPoint = serde_json::from_value(altered).unwrap();
        assert!(
            engine
                .validate_qpss_operating_point_with_abort(&netlist, &altered, &NoAbort)
                .is_err()
        );
    }
    let offsets = vec![37.0, 127.0];
    let ac = engine
        .run_qpac_from_qpss(
            &netlist,
            QpacRequest {
                offsets_hz: offsets.clone(),
                input_source: "Vprobe".into(),
                input_lattice: vec![0, 0],
                output_node: "sink".into(),
                output_ref: "0".into(),
                output_lattice: vec![0, 0],
                magnitude: 1.0,
                phase_degrees: 0.0,
                solver: QuasiPeriodicAcConfig {
                    linear: config.solver.linear.clone(),
                    ..Default::default()
                },
            },
            &restored,
        )
        .unwrap();
    let xf = engine
        .run_qpxf_from_qpss(
            &netlist,
            QpxfRequest {
                frequencies_hz: offsets.clone(),
                frequency_axis: QpxfFrequencyAxis::Output,
                input_sources: QpxfSources::Named(vec!["Vprobe".into()]),
                input_lattices: QpxfInputLattices::Explicit(vec![vec![0, 0]]),
                output: QpxfOutput::Voltage {
                    positive: "sink".into(),
                    negative: "0".into(),
                },
                output_lattice: vec![0, 0],
                linear: config.solver.linear.clone(),
                group_delay: false,
                group_delay_magnitude_floor: 0.0,
            },
            &restored,
        )
        .unwrap();
    let noise = engine
        .run_qpnoise_from_qpss(
            &netlist,
            QpnoiseRequest {
                frequencies_hz: offsets.clone(),
                frequency_axis: QpnoiseFrequencyAxis::Output,
                outputs: vec![QpnoiseOutput {
                    observation: QpnoiseObservation::Voltage {
                        positive: "sink".into(),
                        negative: "0".into(),
                    },
                    lattice: vec![0, 0],
                }],
                input: Some(QpnoiseInput {
                    source: "Vprobe".into(),
                    lattice: vec![0, 0],
                }),
                input_lattices: QpnoiseLattices::AllRetained,
                sources: QpnoiseSources::Only(vec!["RNOISE thermal".into()]),
                integration: None,
                contributor_ranking: false,
                noise_figure: None,
                linear: config.solver.linear,
            },
            &restored,
        )
        .unwrap();
    let kb = super::super::pnoise::pnoise_physical_constants(engine.config.spice_dialect).boltzmann;
    for (i, f) in offsets.into_iter().enumerate() {
        let expected = 0.5 * h(f) * h(f);
        close(
            ac.unit_solutions[i].spectra[sink][grid.dc_index()],
            expected,
            1.0,
        );
        close(xf.transfers[0].values[i], expected, 1.0);
        close(
            noise.outputs[0].input_transfer.as_ref().unwrap()[i],
            expected,
            1.0,
        );
        let psd = kb * 300.15 * 1e3 * h(f).norm_sqr().powi(2);
        close(
            noise.total_covariances[i].values[0],
            Complex64::new(psd, 0.0),
            psd,
        );
        assert_eq!(ac.unit_solutions[i].spectra.len(), physical);
        assert_eq!(xf.solutions[i].sensitivities.len(), physical);
        assert_eq!(noise.points[i].adjoints[0].sensitivities.len(), physical);
    }
    ac.validate_retained_payload_with_abort(&limits, &NoAbort)
        .unwrap();
    xf.validate_retained_payload_with_abort(&limits, &NoAbort)
        .unwrap();
    noise
        .validate_retained_payload_with_abort(&limits, &NoAbort)
        .unwrap();
    assert_eq!(point, restored);
}
