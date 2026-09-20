//! QPAC request identity covers authored controls without invalidating legacy defaults.
use super::*;
use crate::simulation::multi_run::QpacControls;
use crate::simulation::plan::QuasiPeriodicAcDraft;

fn digest(spec: &AnalysisSpec) -> ContentDigest {
    let mut writer = CanonicalWriter::new("test");
    analysis_spec::encode_analysis_spec(&mut writer, spec);
    writer.finish()
}

#[test]
fn qpac_controls_identity_preserves_legacy_bytes_and_covers_every_active_option() {
    let mut draft = QuasiPeriodicAcDraft::default();
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
    let base = digest(&spec);
    assert_eq!(base, legacy.finish());
    let changes: &[fn(&mut QpacControls)] = &[
        |c| c.magnitude = 2.0,
        |c| c.phase_degrees = 17.0,
        |c| c.explicit_offsets = Some(vec![1.0, 2.0]),
        |c| {
            c.solver.linear.method =
                rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod::Krylov
        },
        |c| c.solver.linear.restart += 1,
        |c| c.solver.linear.max_cycles += 1,
        |c| c.solver.linear.relative_tolerance *= 2.0,
        |c| c.solver.current_absolute_tolerance *= 2.0,
        |c| c.solver.voltage_absolute_tolerance *= 2.0,
    ];
    for change in changes {
        let mut next = spec.clone();
        let AnalysisSpec::Qpac { controls, .. } = &mut next else {
            unreachable!()
        };
        change(controls);
        next.validate().unwrap();
        assert_ne!(digest(&next), base);
    }
    let mut expanded = spec.clone();
    if let AnalysisSpec::Qpac {
        input_lattice,
        output_lattice,
        ..
    } = &mut expanded
    {
        input_lattice.push(0);
        output_lattice.push(0);
    }
    expanded.validate().unwrap();
    assert_ne!(digest(&expanded), base);
    let AnalysisSpec::Qpac { controls, .. } = &mut expanded else {
        unreachable!()
    };
    controls.explicit_offsets = Some(vec![1.0, 37.0]);
    let explicit = digest(&expanded);
    if let AnalysisSpec::Qpac {
        start_freq,
        stop_freq,
        points_per_unit,
        ..
    } = &mut expanded
    {
        *start_freq = 17.0;
        *stop_freq = 91.0;
        *points_per_unit = 100;
    }
    assert_eq!(
        digest(&expanded),
        explicit,
        "inactive generated sweep does not change an explicit-list request"
    );
}
