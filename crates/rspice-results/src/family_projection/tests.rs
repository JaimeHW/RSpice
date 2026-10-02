//! Family coordinate, filter and projection contract cases.

use super::*;
use crate::family_metadata::MonteCarloVariableMetadata;
use crate::visualization_document::{
    AccessibleColorPalette, FamilyAggregationMethod, FamilyAggregationPolicy,
    FamilyDimension as DocumentFamilyDimension, FamilyEncodingMap, FamilyPresentationPolicy,
    FamilyXDimension, FamilyXOrdering, MissingPointPolicy,
};
use rspice_app_types::product::DatasetId;

fn corner_metadata() -> AnalysisResultFamilyMetadata {
    AnalysisResultFamilyMetadata::Corner {
        member_measurements: Vec::new(),
        x_values: vec![1.0, 2.0, 3.0],
        x_label: "RGAIN".to_owned(),
        x_unit: "kΩ".to_owned(),
        temperatures_c: vec![-40.0, 27.0, 125.0],
        corner_labels: vec!["SS".to_owned(), "TT".to_owned(), "FF".to_owned()],
        failed_corners: 2,
    }
}

fn process_policy() -> FamilyPresentationPolicy {
    let process = DocumentFamilyDimension::new("process", ValueType::Text).unwrap();
    FamilyPresentationPolicy {
        x_dimension: FamilyXDimension {
            dimension: DocumentFamilyDimension::new("RGAIN", ValueType::Real).unwrap(),
            ordering: FamilyXOrdering::Source,
        },
        family_dimensions: vec![process.clone()],
        facet_layout: None,
        aggregation: FamilyAggregationPolicy {
            method: FamilyAggregationMethod::None,
            over_dimensions: Vec::new(),
        },
        filter: None,
        missing_points: MissingPointPolicy::ExcludeWithOmissionRecord,
        encodings: vec![
            FamilyEncodingMap::Color {
                dimension: process.clone(),
                palette: AccessibleColorPalette::OkabeItoCategorical,
            },
            FamilyEncodingMap::Dash { dimension: process },
        ],
    }
}

#[test]
fn corner_projection_is_exact_and_records_omissions() {
    let manifest = FamilyManifest::from_metadata(AnalysisType::Corner, Some(&corner_metadata()))
        .expect("valid metadata")
        .expect("family");
    assert_eq!(manifest.points.len(), 3);
    assert_eq!(manifest.omitted_points, 2);
    assert_eq!(
        manifest.points[0].values.get("process"),
        Some(&FamilyValue::Text("SS".to_owned()))
    );
    assert_eq!(
        manifest.points[1].values.get("temperature"),
        Some(&FamilyValue::Number(27.0))
    );
    assert_eq!(
        manifest.points[1].values.get("sample"),
        Some(&FamilyValue::Integer(2))
    );
}

#[test]
fn exact_mockup_query_filters_typed_dimensions() {
    let manifest = FamilyManifest::from_metadata(AnalysisType::Corner, Some(&corner_metadata()))
        .unwrap()
        .unwrap();
    assert_eq!(
        manifest
            .matching_source_indices(
                "process in {TT,SS} and temperature >= 27°C and status != not-run"
            )
            .unwrap(),
        [1]
    );
}

#[test]
fn exact_mockup_slice_separator_filters_typed_dimensions() {
    let manifest = FamilyManifest::from_metadata(AnalysisType::Corner, Some(&corner_metadata()))
        .unwrap()
        .unwrap();
    assert_eq!(
        manifest
            .matching_source_indices("temperature in {27,125} · status != not-run")
            .unwrap(),
        [1, 2]
    );
}

#[test]
fn invalid_dimension_and_unit_fail_closed() {
    let manifest = FamilyManifest::from_metadata(AnalysisType::Corner, Some(&corner_metadata()))
        .unwrap()
        .unwrap();
    assert!(
        manifest
            .matching_source_indices("voltage >= 1")
            .unwrap_err()
            .contains("unknown family dimension")
    );
    assert!(
        manifest
            .matching_source_indices("temperature >= 27K")
            .unwrap_err()
            .contains("expected unit °C")
    );
}

#[test]
fn monte_carlo_requires_one_sample_per_completed_run() {
    let metadata = AnalysisResultFamilyMetadata::MonteCarlo {
        member_measurements: Vec::new(),
        seed: 2,
        runs_requested: 3,
        runs_completed: 2,
        failures: 1,
        all_converged: false,
        variables: vec![MonteCarloVariableMetadata {
            mean_confidence: None,
            name: "gain".to_owned(),
            samples: vec![1.0],
            mean: 1.0,
            std_dev: 0.0,
            min: 1.0,
            max: 1.0,
        }],
    };
    assert!(
        FamilyManifest::from_metadata(AnalysisType::MonteCarlo, Some(&metadata))
            .unwrap_err()
            .contains("1 retained samples for 2 completed runs")
    );
}

#[test]
fn waveform_compatibility_prevents_index_invention() {
    let manifest = FamilyManifest::from_metadata(AnalysisType::Corner, Some(&corner_metadata()))
        .unwrap()
        .unwrap();
    assert!(manifest.compatible_waveform_len(3).is_ok());
    assert!(manifest.compatible_waveform_len(12).is_err());
}

#[test]
fn family_styles_are_deterministic_across_filter_subsets() {
    let manifest = FamilyManifest::from_metadata(AnalysisType::Corner, Some(&corner_metadata()))
        .unwrap()
        .unwrap();
    let policy = process_policy();
    let dataset = DatasetId::new();
    let full = SourceSampleSelection::new(dataset, 7, vec![0, 1, 2])
        .unwrap()
        .with_family_presentation(&manifest, &policy)
        .unwrap();
    let tt_only = SourceSampleSelection::new(dataset, 7, vec![1])
        .unwrap()
        .with_family_presentation(&manifest, &policy)
        .unwrap();

    let full_tt = full
        .family_render_plan()
        .unwrap()
        .groups()
        .iter()
        .find(|group| group.label.contains("TT"))
        .unwrap();
    let filtered_tt = &tt_only.family_render_plan().unwrap().groups()[0];
    assert_eq!(full_tt.style, filtered_tt.style);
    assert_eq!(full_tt.stable_key, filtered_tt.stable_key);
    assert_eq!(full_tt.source_indices, [1]);
    assert_eq!(full.fingerprint(), full.clone().fingerprint());
}

#[test]
fn unsupported_family_flattening_fails_closed() {
    let manifest = FamilyManifest::from_metadata(AnalysisType::Corner, Some(&corner_metadata()))
        .unwrap()
        .unwrap();
    let mut policy = process_policy();
    policy.x_dimension.ordering = FamilyXOrdering::Descending;
    let error = SourceSampleSelection::new(DatasetId::new(), 7, vec![0, 1, 2])
        .unwrap()
        .with_family_presentation(&manifest, &policy)
        .unwrap_err();
    assert!(error.contains("exact source ordering"));
}

#[test]
fn typed_filter_ast_is_authoritative_and_covers_every_predicate_form() {
    let manifest = FamilyManifest::from_metadata(AnalysisType::Corner, Some(&corner_metadata()))
        .unwrap()
        .unwrap();
    let process = DocumentFamilyDimension::new("process", ValueType::Text).unwrap();
    let x = DocumentFamilyDimension::new("RGAIN", ValueType::Real).unwrap();
    let temperature = DocumentFamilyDimension::new("temperature", ValueType::Real).unwrap();
    let status = DocumentFamilyDimension::new("status", ValueType::Text).unwrap();
    let filter = FamilyFilterExpression {
        source: "this source text is intentionally not executable".to_owned(),
        predicate: FamilyPredicate::All {
            predicates: vec![
                FamilyPredicate::Constant { value: true },
                FamilyPredicate::In {
                    dimension: process,
                    values: vec![
                        TypedValue::Text("TT".to_owned()),
                        TypedValue::Text("FF".to_owned()),
                    ],
                },
                FamilyPredicate::Between {
                    dimension: x,
                    lower: TypedValue::Real(2.0),
                    upper: TypedValue::Real(3.0),
                    include_lower: true,
                    include_upper: true,
                },
                FamilyPredicate::Any {
                    predicates: vec![
                        FamilyPredicate::Compare {
                            dimension: temperature,
                            operator: FamilyComparisonOperator::GreaterThanOrEqual,
                            value: TypedValue::Real(27.0),
                        },
                        FamilyPredicate::Constant { value: false },
                    ],
                },
                FamilyPredicate::Not {
                    predicate: Box::new(FamilyPredicate::Compare {
                        dimension: status,
                        operator: FamilyComparisonOperator::Equal,
                        value: TypedValue::Text("not-run".to_owned()),
                    }),
                },
            ],
        },
    };

    assert_eq!(
        manifest
            .matching_source_indices_for_filter(Some(&filter))
            .unwrap(),
        [1, 2]
    );
}

#[test]
fn family_x_requires_finite_lossless_numeric_projection() {
    let group = DocumentFamilyDimension::new("group", ValueType::Text).unwrap();
    let policy = |x_type| FamilyPresentationPolicy {
        x_dimension: FamilyXDimension {
            dimension: DocumentFamilyDimension::new("x", x_type).unwrap(),
            ordering: FamilyXOrdering::Source,
        },
        family_dimensions: vec![group.clone()],
        facet_layout: None,
        aggregation: FamilyAggregationPolicy {
            method: FamilyAggregationMethod::None,
            over_dimensions: Vec::new(),
        },
        filter: None,
        missing_points: MissingPointPolicy::ExcludeWithOmissionRecord,
        encodings: vec![FamilyEncodingMap::Label {
            dimension: group.clone(),
            prefix: None,
        }],
    };
    let manifest = |kind, value| FamilyManifest {
        dimensions: vec![
            dimension("x", "Exact X", Some("u"), kind),
            dimension("group", "Group", None, FamilyValueKind::Text),
        ],
        points: vec![point(
            0,
            [("x", value), ("group", FamilyValue::Text("A".to_owned()))],
        )],
        omitted_points: 0,
    };

    let non_finite = manifest(FamilyValueKind::Number, FamilyValue::Number(f64::NAN));
    let error = SourceSampleSelection::new(DatasetId::new(), 1, vec![0])
        .unwrap()
        .with_family_presentation(&non_finite, &policy(ValueType::Real))
        .unwrap_err();
    assert!(error.contains("non-finite X value"));

    let lossy = manifest(
        FamilyValueKind::Integer,
        FamilyValue::Integer(9_007_199_254_740_993),
    );
    let error = SourceSampleSelection::new(DatasetId::new(), 1, vec![0])
        .unwrap()
        .with_family_presentation(&lossy, &policy(ValueType::Integer))
        .unwrap_err();
    assert!(error.contains("cannot be represented losslessly"));
}
