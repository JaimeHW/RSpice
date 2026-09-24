//! QPNOISE identity compatibility and effective measurement configuration.
use super::*;
use crate::simulation::multi_run::{FrequencySweep, QpnoiseControls};
use rspice_core::engine::{
    QpnoiseFrequencyAxis, QpnoiseIntegrationMethod, QpnoiseLattices, QpnoiseNoiseFigure,
    QpnoiseObservation, QpnoiseOutput, QpnoiseSources,
};
fn digest(spec: &AnalysisSpec) -> ContentDigest {
    let mut writer = CanonicalWriter::new("test");
    analysis_spec::encode_analysis_spec(&mut writer, spec);
    writer.finish()
}
#[test]
fn qpnoise_studio_identity_preserves_legacy_and_authenticates_active_options() {
    let spec = AnalysisSpec::Qpnoise {
        start_freq: 100.0,
        stop_freq: 1000.0,
        points_per_unit: 3,
        sweep: FrequencySweep::Linear,
        output_node: "out".into(),
        output_ref: "0".into(),
        input_source: "V1".into(),
        lattice_min: vec![-1, -2],
        lattice_max: vec![1, 2],
        integrated_noise: true,
        contributor_ranking: true,
        controls: QpnoiseControls::default(),
    };
    let mut legacy = CanonicalWriter::new("test");
    legacy.domain("analysis-spec");
    legacy.u8(analysis_spec::analysis_kind_tag(&spec));
    legacy.f64(100.0);
    legacy.f64(1000.0);
    legacy.usize(3);
    legacy.u8(2);
    legacy.string("out");
    legacy.string("0");
    legacy.string("V1");
    for k in [-1, -2, 1, 2] {
        legacy.i32(k);
    }
    legacy.bool(true);
    legacy.bool(true);
    let base = digest(&spec);
    assert_eq!(base, legacy.finish());
    let changes: &[fn(&mut QpnoiseControls)] = &[
        |c| c.frequency_axis = QpnoiseFrequencyAxis::Offset,
        |c| c.explicit_frequencies = Some(vec![-100.0, 0.0, 100.0]),
        |c| c.input_referral = false,
        |c| c.input_lattice = vec![1, -1],
        |c| c.output_lattice = vec![-1, 1],
        |c| c.branch_current = Some("L1".into()),
        |c| {
            c.additional_outputs.push(QpnoiseOutput {
                observation: QpnoiseObservation::Voltage {
                    positive: "in".into(),
                    negative: "out".into(),
                },
                lattice: vec![0, 0],
            })
        },
        |c| c.noise_lattices = Some(QpnoiseLattices::AllRetained),
        |c| c.noise_lattices = Some(QpnoiseLattices::MaxOrders { orders: vec![1, 2] }),
        |c| {
            c.noise_lattices = Some(QpnoiseLattices::Explicit {
                tuples: vec![vec![0, 0]],
            })
        },
        |c| c.sources = QpnoiseSources::Only(vec!["Rs thermal".into()]),
        |c| c.sources = QpnoiseSources::Except(vec!["Rs thermal".into()]),
        |c| c.integration_band = Some([200.0, 800.0]),
        |c| c.integration_method = QpnoiseIntegrationMethod::LogLog,
        |c| {
            c.noise_figure = Some(QpnoiseNoiseFigure {
                source_resistor: "Rs".into(),
                reference_temperature: 290.0,
                reference_lattices: Some(vec![vec![0, 0], vec![1, 1]]),
            })
        },
        |c| {
            c.solver.method =
                rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod::Krylov
        },
        |c| c.solver.restart += 1,
        |c| c.solver.max_cycles += 1,
        |c| c.solver.relative_tolerance *= 2.0,
    ];
    for change in changes {
        let mut changed = spec.clone();
        let AnalysisSpec::Qpnoise { controls, .. } = &mut changed else {
            unreachable!()
        };
        change(controls);
        changed.validate().unwrap();
        assert_ne!(digest(&changed), base);
    }
    let mut active = spec;
    let AnalysisSpec::Qpnoise {
        controls,
        integrated_noise,
        ..
    } = &mut active
    else {
        unreachable!()
    };
    controls.explicit_frequencies = Some(vec![100.0, 200.0]);
    controls.branch_current = Some("L1".into());
    controls.input_referral = false;
    controls.noise_lattices = Some(QpnoiseLattices::AllRetained);
    *integrated_noise = false;
    let effective = digest(&active);
    let AnalysisSpec::Qpnoise {
        start_freq,
        stop_freq,
        points_per_unit,
        sweep,
        output_node,
        output_ref,
        input_source,
        lattice_min,
        lattice_max,
        controls,
        ..
    } = &mut active
    else {
        unreachable!()
    };
    *start_freq = -9.0;
    *stop_freq = 30.0;
    *points_per_unit = 90;
    *sweep = FrequencySweep::Octave;
    *output_node = "unused".into();
    *output_ref = "unused".into();
    *input_source = "unused".into();
    *lattice_min = vec![9, 8, 7];
    *lattice_max = vec![];
    controls.input_lattice = vec![9, 8, 7];
    controls.integration_band = Some([2.0, 3.0]);
    controls.integration_method = QpnoiseIntegrationMethod::LogLog;
    active.validate().unwrap();
    assert_eq!(
        digest(&active),
        effective,
        "inactive fields must not alter the measurement identity"
    );
}
