//! A family read as rows of swept quantities and measured values, on
//! families whose every value is known exactly.

use super::*;
use crate::analysis_type::AnalysisType;
use crate::provenance::{
    AnalysisResultProvenance, AnalysisResultSourceDomain, AnalysisResultStepCoordinate,
};
use rspice_vocabulary::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};

/// The gain of a divider whose lower leg is 1 kΩ and upper leg `r`.
fn divider_gain(r: f64) -> f64 {
    1e3 / (r + 1e3)
}

fn measured(name: &str, value: f64) -> rspice_core::MeasureResult {
    rspice_core::MeasureResult {
        name: name.to_owned(),
        value: Some(value),
        raw_value: Some(value),
        error: None,
        passed: true,
        expected: None,
        tolerance: None,
        failure_limit: None,
        failure_limit_exceeded: false,
        event_axis: None,
        units: None,
    }
}

/// One `.STEP` point of a transient: `V(out)` ramps from 0 to the divider's
/// gain over one second, and `.MEAS gain` states that gain.
fn step_point(index: usize, coordinate: &[(&str, f64, &str)], gain: f64) -> AnalysisResult {
    let member = AnalysisResultFamilyMember::StepPoint {
        index,
        coordinate: coordinate
            .iter()
            .map(|(name, value, unit)| AnalysisResultStepCoordinate {
                name: (*name).to_owned(),
                value: *value,
                unit: (*unit).to_owned(),
            })
            .collect(),
    };
    let provenance = AnalysisResultProvenance::new(
        AnalysisInstanceId::new(),
        ObjectRevision::INITIAL,
        ContentDigest::from_bytes([7; 32]),
        Vec::new(),
    )
    .unwrap()
    .with_family_member(Some(member.clone()))
    .unwrap();
    let time: Vec<f64> = (0..=100).map(|step| f64::from(step) / 100.0).collect();
    let out: Vec<f64> = time.iter().map(|t| gain * t).collect();
    AnalysisResult::new(
        index as u64,
        AnalysisType::Transient,
        member.titled("Transient"),
        0.0,
    )
    .with_waveforms(vec![RetainedWaveform::new("V(out)", time, out)])
    .with_measurements(vec![measured("gain", gain)])
    .with_provenance(provenance)
}

fn close(actual: Option<f64>, expected: f64) {
    let actual = actual.expect("a value");
    assert!(
        (actual - expected).abs() <= 1e-12 * expected.abs().max(1.0),
        "{actual} != {expected}"
    );
}

#[test]
fn a_stepped_transient_reads_each_points_measurement_against_its_stepped_value() {
    // Declared out of order: the table reads in sweep order all the same.
    let family = vec![
        step_point(2, &[("r", 4e3, "")], divider_gain(4e3)),
        step_point(0, &[("r", 1e3, "")], divider_gain(1e3)),
        step_point(1, &[("r", 2e3, "")], divider_gain(2e3)),
    ];
    let table = sweep_table(&family, 0).expect("a stepped family is a sweep");
    assert_eq!(table.rows, ["r=1k", "r=2k", "r=4k"]);
    assert_eq!(table.analyses, [1, 2, 0]);
    assert_eq!(table.axes.len(), 1);
    assert_eq!(table.axes[0].name, "r");
    let gain = table.output("gain").expect("the deck's .MEAS");
    assert_eq!(gain.kind, OutputKind::Measurement);
    for (value, r) in gain.values.iter().zip([1e3, 2e3, 4e3]) {
        close(*value, divider_gain(r));
    }
    // A retained signal reads at the end of each record, or at a time the
    // reader names, on each member's own samples.
    let end = signal_values(&family, &table, "V(out)", None);
    let half = signal_values(&family, &table, "V(out)", Some(0.5));
    for ((end, half), r) in end.iter().zip(&half).zip([1e3, 2e3, 4e3]) {
        close(*end, divider_gain(r));
        close(*half, divider_gain(r) / 2.0);
    }
    assert_eq!(table.signal_axis, Some(("time".to_owned(), "s".to_owned())));
    let curves = table.curves("r");
    assert_eq!(curves.len(), 1);
    assert_eq!(curves[0].label, "");
    assert_eq!(curves[0].rows, [0, 1, 2]);
}

/// Two swept quantities draw one curve per value of the one not on X, and
/// swapping which stands on X regroups the same rows.
#[test]
fn a_two_quantity_sweep_draws_a_curve_per_value_of_the_other_quantity() {
    let mut family = Vec::new();
    for (index, (r, c)) in [(1e3, 1e-12), (2e3, 1e-12), (1e3, 1e-11), (2e3, 1e-11)]
        .into_iter()
        .enumerate()
    {
        family.push(step_point(
            index,
            &[("r", r, ""), ("C1", c, "F")],
            divider_gain(r),
        ));
    }
    let table = sweep_table(&family, 3).expect("a two-quantity sweep");
    assert_eq!(
        table.rows,
        [
            "r=1k, C1=1pF",
            "r=2k, C1=1pF",
            "r=1k, C1=10pF",
            "r=2k, C1=10pF"
        ]
    );
    let by_r = table.curves("r");
    assert_eq!(
        by_r.iter()
            .map(|curve| (
                curve.label.as_str(),
                curve.rows.clone(),
                curve.levels.clone()
            ))
            .collect::<Vec<_>>(),
        [
            ("C1=1pF", vec![0, 1], vec![0]),
            ("C1=10pF", vec![2, 3], vec![1])
        ]
    );
    let by_c = table.curves("C1");
    assert_eq!(
        by_c.iter()
            .map(|curve| (curve.label.as_str(), curve.rows.clone()))
            .collect::<Vec<_>>(),
        [("r=1k", vec![0, 2]), ("r=2k", vec![1, 3])]
    );
}

/// An in-analysis `.STEP` family names its swept quantity from its target,
/// with the unit the swept thing names, and reads one sample per point.
#[test]
fn an_in_analysis_step_names_its_quantity_from_its_target() {
    assert_eq!(step_target_axis("PARAM rload"), ("rload".into(), "".into()));
    assert_eq!(step_target_axis("DEVICE R1"), ("R1".into(), "Ω".into()));
    assert_eq!(step_target_axis("DEVICE M1.w"), ("M1.w".into(), "".into()));
    assert_eq!(step_target_axis("TEMP"), ("temp".into(), "°C".into()));
    let values = vec![1e3, 2e3, 4e3];
    let gains: Vec<f64> = values.iter().map(|r| divider_gain(*r)).collect();
    let family = vec![
        AnalysisResult::new(1, AnalysisType::Parametric, "Sweep", 0.0)
            .with_waveforms(vec![RetainedWaveform::new(
                "V(out)",
                values.clone(),
                gains.clone(),
            )])
            .with_family_metadata(AnalysisResultFamilyMetadata::Parametric {
                target: "DEVICE R1".to_owned(),
                sweep_values: values,
                failed_points: 0,
                member_measurements: Vec::new(),
            }),
    ];
    let table = sweep_table(&family, 0).expect("an in-analysis sweep");
    assert_eq!(table.rows, ["R1=1kΩ", "R1=2kΩ", "R1=4kΩ"]);
    assert_eq!(table.analyses, [0, 0, 0]);
    assert_eq!(table.signal_axis, None);
    let out = table.output("V(out)").expect("the point's sample");
    assert_eq!(out.kind, OutputKind::Sample);
    for (value, gain) in out.values.iter().zip(&gains) {
        close(*value, *gain);
    }
}

/// A corner run names each row by what differs between corners: a run at
/// one temperature does not sweep the temperature.
#[test]
fn a_corner_run_keeps_only_the_quantities_that_differ() {
    let family = vec![
        AnalysisResult::new(1, AnalysisType::Corner, "Corners", 0.0)
            .with_waveforms(vec![RetainedWaveform::new(
                "V(out)",
                vec![27.0, 27.0, 27.0],
                vec![1.0, 1.1, 0.9],
            )])
            .with_family_metadata(AnalysisResultFamilyMetadata::Corner {
                x_values: vec![27.0, 27.0, 27.0],
                x_label: "Temperature".to_owned(),
                x_unit: "C".to_owned(),
                temperatures_c: vec![27.0, 27.0, 27.0],
                corner_labels: vec!["TT".into(), "FF".into(), "SS".into()],
                failed_corners: 0,
                member_measurements: Vec::new(),
            }),
    ];
    let table = sweep_table(&family, 0).expect("a corner run");
    assert_eq!(table.rows, ["TT", "FF", "SS"]);
    assert_eq!(
        table
            .axes
            .iter()
            .map(|axis| axis.name.as_str())
            .collect::<Vec<_>>(),
        ["corner"]
    );
    assert_eq!(table.numeric_axes().count(), 0);
}

/// Points of one authored analysis solved at PVT corners read as one table:
/// the process and the temperature, each where it differs.
#[test]
fn a_pvt_family_reads_its_corners_and_temperatures() {
    let authored = AnalysisInstanceId::new();
    let points = [
        ("tt", 27.0),
        ("ff", -40.0),
        ("ss", 125.0),
        ("tt", -40.0),
        ("tt", 125.0),
    ];
    let family: Vec<AnalysisResult> = points
        .iter()
        .enumerate()
        .map(|(index, (process, temperature))| {
            let provenance = AnalysisResultProvenance::new_with_authored_source_domain(
                AnalysisResultSourceDomain::SimulationPlan,
                AnalysisInstanceId::new(),
                authored,
                ObjectRevision::INITIAL,
                ContentDigest::from_bytes([7; 32]),
                Vec::new(),
            )
            .unwrap()
            .with_pvt_point(Some(
                AnalysisResultPvtPoint::new(*process, None, *temperature, None, index == 0)
                    .unwrap(),
            ));
            AnalysisResult::new(index as u64, AnalysisType::Ac, "AC", 0.0)
                .with_waveforms(vec![RetainedWaveform::new(
                    "V(out)",
                    vec![1.0, 10.0],
                    vec![1.0, 0.5],
                )])
                .with_provenance(provenance)
        })
        .collect();
    let table = sweep_table(&family, 2).expect("a PVT family");
    assert_eq!(
        table.rows,
        [
            "tt, temp=27 °C",
            "ff, temp=\u{2212}40 °C",
            "ss, temp=125 °C",
            "tt, temp=\u{2212}40 °C",
            "tt, temp=125 °C"
        ]
    );
    assert_eq!(
        table
            .numeric_axes()
            .map(|axis| axis.name.as_str())
            .collect::<Vec<_>>(),
        ["temp"]
    );
    // Drawn against the temperature, the typical corner is one curve of
    // three points; the fast and slow corners are a point each.
    let curves = table.curves("temp");
    assert_eq!(
        curves
            .iter()
            .map(|curve| (curve.label.as_str(), curve.rows.clone()))
            .collect::<Vec<_>>(),
        [("tt", vec![3, 0, 4]), ("ff", vec![1]), ("ss", vec![2])]
    );
    assert_eq!(
        table.signal_axis,
        Some(("frequency".to_owned(), "Hz".to_owned()))
    );
}
