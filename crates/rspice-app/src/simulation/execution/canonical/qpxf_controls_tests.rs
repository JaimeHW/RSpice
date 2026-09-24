//! QPXF identities preserve legacy bytes and bind every active selection/control.
use super::*;
use crate::simulation::multi_run::QpxfControls;
use crate::simulation::plan::QuasiPeriodicTransferDraft;
use rspice_core::engine::{QpxfFrequencyAxis, QpxfInputLattices, QpxfSources};
fn digest(spec: &AnalysisSpec) -> ContentDigest {
    let mut writer = CanonicalWriter::new("test");
    analysis_spec::encode_analysis_spec(&mut writer, spec);
    writer.finish()
}
#[test]
fn qpxf_controls_preserve_legacy_identity_and_cover_active_settings() {
    let mut draft = QuasiPeriodicTransferDraft::default();
    draft.sweep.start = "1k".into();
    draft.sweep.stop = "2k".into();
    draft.sweep.points = "2".into();
    draft.sweep.sweep = 2;
    let spec = draft.to_spec().unwrap();
    let mut legacy = CanonicalWriter::new("test");
    legacy.domain("analysis-spec");
    legacy.u8(analysis_spec::analysis_kind_tag(&spec));
    legacy.f64(1e3);
    legacy.f64(2e3);
    legacy.usize(2);
    legacy.u8(2);
    legacy.string("V1");
    legacy.string("out");
    legacy.string("0");
    for _ in 0..4 {
        legacy.i32(0);
    }
    legacy.bool(false);
    let base = digest(&spec);
    assert_eq!(base, legacy.finish());
    let changes: &[fn(&mut QpxfControls)] = &[
        |c| c.frequency_axis = QpxfFrequencyAxis::Offset,
        |c| c.explicit_frequencies = Some(vec![-1.0, 0.0, 1.0]),
        |c| c.input_sources = Some(QpxfSources::AllIndependent),
        |c| c.input_sources = Some(QpxfSources::Named(vec!["V2".into(), "I1".into()])),
        |c| c.input_lattices = Some(QpxfInputLattices::AllRetained),
        |c| c.input_lattices = Some(QpxfInputLattices::MaxOrders(vec![1, 2])),
        |c| c.input_lattices = Some(QpxfInputLattices::Explicit(vec![vec![1, -1], vec![0, 0]])),
        |c| c.branch_current = Some("L1".into()),
        |c| {
            c.solver.method =
                rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod::Krylov
        },
        |c| c.solver.restart += 1,
        |c| c.solver.max_cycles += 1,
        |c| c.solver.relative_tolerance *= 2.0,
        |c| c.group_delay_magnitude_floor = 1e-9,
    ];
    for change in changes {
        let mut changed = spec.clone();
        let AnalysisSpec::Qpxf { controls, .. } = &mut changed else {
            unreachable!()
        };
        change(controls);
        changed.validate().unwrap();
        assert_ne!(digest(&changed), base);
    }
    let mut overridden = spec;
    let AnalysisSpec::Qpxf { controls, .. } = &mut overridden else {
        unreachable!()
    };
    controls.explicit_frequencies = Some(vec![-1.0, 0.0, 1.0]);
    controls.input_sources = Some(QpxfSources::AllIndependent);
    controls.input_lattices = Some(QpxfInputLattices::AllRetained);
    controls.branch_current = Some("L1".into());
    let active = digest(&overridden);
    let AnalysisSpec::Qpxf {
        start_freq,
        stop_freq,
        points_per_unit,
        input_source,
        output_node,
        output_ref,
        input_lattice,
        ..
    } = &mut overridden
    else {
        unreachable!()
    };
    *start_freq = -100.0;
    *stop_freq = 100.0;
    *points_per_unit = 321;
    *input_source = "unused".into();
    *output_node = "unused".into();
    *output_ref = "unused".into();
    *input_lattice = vec![9, 8, 7];
    overridden.validate().unwrap();
    assert_eq!(
        digest(&overridden),
        active,
        "inactive legacy fields cannot alter the effective request"
    );
    let AnalysisSpec::Qpxf {
        output_lattice,
        group_delay,
        ..
    } = &mut overridden
    else {
        unreachable!()
    };
    *output_lattice = vec![1, -1];
    *group_delay = true;
    assert_ne!(digest(&overridden), active);
}
