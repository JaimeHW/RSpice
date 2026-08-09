#[path = "../src/engine/builder/version_metadata.rs"]
mod version_metadata;

use version_metadata::parse_dotted_version_metadata;

fn assert_version(input: &str, expected: f64) {
    let actual = parse_dotted_version_metadata(input)
        .unwrap_or_else(|| panic!("VERSION metadata {input:?} must parse"));
    assert!(
        (actual - expected).abs() <= 1.0e-15,
        "VERSION metadata {input:?} mapped to {actual}, expected {expected}"
    );
}

#[test]
fn decimal_version_metadata_is_preserved() {
    assert_version("4.6", 4.6);
    assert_version("  '3.30'  ", 3.30);
    assert_version("4e0", 4.0);
}

#[test]
fn multi_component_versions_follow_xyce_recursive_weighting() {
    assert_version("4.6.1", 4.61);
    assert_version("4.10.6", 5.06);
    assert_version("4.10.6.2", 5.062);
    assert_version("4.06.1", 4.61);
}

#[test]
fn invalid_or_non_finite_version_metadata_is_rejected() {
    for value in [
        "", "   ", "release", "4..1", ".4.1", "4.1.", "4.a.1", "NaN", "inf", "1e309",
    ] {
        assert_eq!(
            parse_dotted_version_metadata(value),
            None,
            "VERSION metadata {value:?} must fail closed"
        );
    }
}
