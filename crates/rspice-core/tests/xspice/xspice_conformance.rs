use rspice_core::xspice::{
    CmContext, CodeModelRegistry,
    conformance::{
        IfSpecConformancePolicy, PartialVerificationOptions, audit_ngspice_ifspec_event_port_types,
        audit_ngspice_ifspec_test_coverage, audit_ngspice_ifspec_tree,
        audit_ngspice_xspice_examples, context_with_model_defaults, verify_model_partials,
    },
};
use std::path::{Path, PathBuf};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root")
        .to_path_buf()
}

fn local_ngspice_source_root() -> Option<PathBuf> {
    if let Some(root) = std::env::var_os("NGSPICE_SOURCE_ROOT").map(PathBuf::from)
        && root.join("src").join("xspice").join("icm").is_dir()
    {
        return Some(root);
    }

    let workspace = workspace_root();
    let candidates = [
        workspace
            .parent()
            .unwrap_or(&workspace)
            .join("ngspice-46-release")
            .join("ngspice-46"),
        workspace
            .join("..")
            .join("ngspice-46-release")
            .join("ngspice-46"),
    ];

    candidates
        .into_iter()
        .find(|root| root.join("src").join("xspice").join("icm").is_dir())
}

fn skip_without_local_ngspice_source() -> Option<PathBuf> {
    let root = local_ngspice_source_root();
    if root.is_none() {
        eprintln!(
            "skipping local ngspice XSPICE conformance test; set NGSPICE_SOURCE_ROOT to an ngspice checkout"
        );
    }
    root
}

#[test]
fn ngspice46_ifspec_catalog_matches_live_xspice_registry() {
    let Some(ngspice_root) = skip_without_local_ngspice_source() else {
        return;
    };
    let registry = CodeModelRegistry::with_builtins();
    let report = audit_ngspice_ifspec_tree(
        &ngspice_root.join("src").join("xspice").join("icm"),
        &registry,
        &IfSpecConformancePolicy::ngspice46(),
    )
    .expect("ngspice ifspec tree audits");

    assert!(
        report.checked_models >= 70,
        "expected the ngspice 46 XSPICE ifspec catalog, checked {} model(s)",
        report.checked_models
    );
    assert_eq!(
        report.error_count(),
        0,
        "XSPICE ifspec conformance errors: {:#?}",
        report.issues
    );
}

#[test]
fn ngspice46_xspice_example_corpus_is_adjudicated() {
    let Some(ngspice_root) = skip_without_local_ngspice_source() else {
        return;
    };
    let report =
        audit_ngspice_xspice_examples(&ngspice_root).expect("ngspice XSPICE example corpus audits");
    let needs_adjudication = report.needs_adjudication();

    assert!(
        report.decks.len() >= 100,
        "expected the ngspice 46 XSPICE example corpus, found {} deck(s)",
        report.decks.len()
    );
    assert!(
        report.runnable_count() + report.scripted_control_count() > 0,
        "upstream XSPICE corpus should contain in-scope runnable or scripted decks"
    );
    assert!(
        report.excluded_count() > 0,
        "third-party XSPICE example decks should be explicitly excluded, not silently mixed into parity coverage"
    );
    assert!(
        report.expected_invalid_count() > 0,
        "intentionally invalid upstream XSPICE example decks should be explicitly classified"
    );
    assert!(
        report.reusable_fragment_count() > 0,
        "analysis-free reusable subcircuit/include decks should be explicitly classified"
    );
    assert!(
        needs_adjudication.is_empty(),
        "unadjudicated ngspice XSPICE example decks: {needs_adjudication:#?}"
    );
}

#[test]
fn ngspice46_ifspec_catalog_has_test_coverage_markers() {
    let Some(ngspice_root) = skip_without_local_ngspice_source() else {
        return;
    };
    let workspace = workspace_root();
    let coverage_roots = vec![
        workspace.join("crates").join("rspice-core").join("tests"),
        workspace
            .join("crates")
            .join("rspice-core")
            .join("src")
            .join("xspice"),
    ];
    let report = audit_ngspice_ifspec_test_coverage(
        &ngspice_root.join("src").join("xspice").join("icm"),
        &coverage_roots,
        &IfSpecConformancePolicy::ngspice46(),
    )
    .expect("ngspice ifspec coverage audits");

    assert!(
        report.checked_models >= 70,
        "expected the ngspice 46 XSPICE ifspec catalog, checked {} model(s)",
        report.checked_models
    );
    assert_eq!(
        report.uncovered_models,
        Vec::<String>::new(),
        "XSPICE catalog models without test coverage markers: {:#?}",
        report.uncovered_models
    );
}

#[test]
fn ngspice46_ifspec_catalog_uses_supported_event_port_types() {
    let Some(ngspice_root) = skip_without_local_ngspice_source() else {
        return;
    };
    let report = audit_ngspice_ifspec_event_port_types(
        &ngspice_root.join("src").join("xspice").join("icm"),
        &IfSpecConformancePolicy::ngspice46(),
    )
    .expect("ngspice ifspec event port type audit runs");

    assert!(
        report.checked_models >= 70,
        "expected the ngspice 46 XSPICE ifspec catalog, checked {} model(s)",
        report.checked_models
    );
    assert!(
        !report.has_unsupported_event_ports(),
        "ngspice 46 model(s) require unsupported XSPICE event port types: {:#?}",
        report.unsupported_event_ports
    );
}

#[test]
fn scalar_analog_partials_match_finite_difference() {
    let registry = CodeModelRegistry::with_builtins();
    verify_scalar_model(&registry, "gain", |ctx| {
        ctx.set_input_analog("in", 0.375);
        ctx.set_param("gain", 2.75);
        ctx.set_param("in_offset", -0.125);
        ctx.set_param("out_offset", 0.5);
    });
    verify_scalar_model(&registry, "limit", |ctx| {
        ctx.set_input_analog("in", 0.25);
        ctx.set_param("gain", 1.5);
        ctx.set_param("in_offset", 0.0);
        ctx.set_param("out_lower_limit", -10.0);
        ctx.set_param("out_upper_limit", 10.0);
    });
    verify_scalar_model(&registry, "pwl", |ctx| {
        ctx.set_input_analog("in", 0.25);
        ctx.set_real_vector_param("x_array", vec![-1.0, 0.0, 1.0]);
        ctx.set_real_vector_param("y_array", vec![-2.0, 0.0, 4.0]);
        ctx.set_param("input_domain", 0.0);
        ctx.set_param("fraction", 1.0);
    });
}

#[test]
fn vector_analog_partials_match_finite_difference() {
    let registry = CodeModelRegistry::with_builtins();
    let model = registry.get("summer").expect("summer registered");
    let mut ctx = context_with_model_defaults(model.as_ref());
    ctx.set_port_width("in", 3);
    ctx.set_input_analog_vector("in", &[0.25, -0.5, 0.75])
        .expect("seed summer vector input");
    ctx.set_real_vector_param("in_offset", vec![0.0, 0.1, -0.2]);
    ctx.set_real_vector_param("in_gain", vec![1.0, -2.0, 0.5]);
    ctx.set_param("out_gain", 1.25);
    ctx.set_param("out_offset", -0.375);

    let mismatches = verify_model_partials(
        model.as_ref(),
        &ctx,
        PartialVerificationOptions {
            absolute_tolerance: 1.0e-8,
            relative_tolerance: 1.0e-6,
            ..PartialVerificationOptions::default()
        },
    )
    .expect("summer partials verify");
    assert!(mismatches.is_empty(), "{mismatches:#?}");
}

fn verify_scalar_model(
    registry: &CodeModelRegistry,
    name: &str,
    configure: impl FnOnce(&mut CmContext),
) {
    let model = registry.get(name).expect("model registered");
    let mut ctx = context_with_model_defaults(model.as_ref());
    configure(&mut ctx);
    let mismatches = verify_model_partials(
        model.as_ref(),
        &ctx,
        PartialVerificationOptions {
            absolute_tolerance: 1.0e-8,
            relative_tolerance: 1.0e-6,
            ..PartialVerificationOptions::default()
        },
    )
    .unwrap_or_else(|err| panic!("{name} partial verifier failed: {err}"));
    assert!(
        mismatches.is_empty(),
        "{name} partial mismatch(es): {mismatches:#?}"
    );
}
