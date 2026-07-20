#![cfg(feature = "veriloga")]

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use rspice_core::{
    Engine, Netlist, ProjectVerilogARuntimeRegistration, ResourceLimits,
    register_project_veriloga_runtimes_for_session,
    register_project_veriloga_runtimes_for_session_with_limits,
};

static PROJECT_NONCE: AtomicU64 = AtomicU64::new(0);

fn project_scope(label: &str) -> String {
    let nonce = PROJECT_NONCE.fetch_add(1, Ordering::Relaxed);
    format!("test-{label}-{}-{nonce}", std::process::id())
}

fn key(project: &str, digest: &str, file_name: &str) -> PathBuf {
    PathBuf::from(format!("__rspice_project__/{project}/{digest}/{file_name}"))
}

fn registration(
    source_key: PathBuf,
    module_name: &str,
    aliases: &[&str],
) -> ProjectVerilogARuntimeRegistration {
    let compiled = rspice_veriloga::VerilogACompiler::default()
        .compile_runtime(
            &format!(
                "module {module_name}(p, n); inout p, n; electrical p, n; analog I(p,n) <+ V(p,n); endmodule\n"
            ),
            None,
        )
        .expect("compile project runtime");
    ProjectVerilogARuntimeRegistration {
        source_key,
        aliases: aliases.iter().map(|alias| (*alias).to_owned()).collect(),
        model: compiled.model,
        canonical_ir: compiled.canonical_ir,
    }
}

#[test]
fn plural_registration_accepts_a_complete_noncolliding_set() {
    let project = project_scope("complete");
    let first_key = key(&project, "first", "one.va");
    let second_key = key(&project, "second", "two.va");
    register_project_veriloga_runtimes_for_session([
        registration(first_key.clone(), "batch_one", &["ONE"]),
        registration(second_key.clone(), "batch_two", &["TWO"]),
    ])
    .expect("register complete runtime set atomically");

    let deck = format!(
        "atomic project runtime batch\nV1 in 0 1\n.veriloga \"{}\" ONE\n.veriloga \"{}\" TWO\nX1 in 0 ONE\nX2 in 0 TWO\n.op\n.end\n",
        first_key.display(),
        second_key.display()
    );
    let netlist = Netlist::parse(&deck).expect("parse deck using both registered aliases");
    let result = Engine::default()
        .run_dc_op(&netlist)
        .expect("both registered runtimes execute in one circuit");
    let input_index = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("in"))
        .expect("input node exists");
    assert!((result.node_voltages[input_index] - 1.0).abs() < 1e-9);
}

#[test]
fn differing_installed_key_collision_rolls_back_every_candidate() {
    let project = project_scope("key-rollback");
    let stable_key = key(&project, "stable", "model.va");
    let candidate_key = key(&project, "candidate", "candidate.va");
    register_project_veriloga_runtimes_for_session([registration(
        stable_key.clone(),
        "stable_runtime",
        &["STABLE"],
    )])
    .expect("install stable runtime");

    let error = register_project_veriloga_runtimes_for_session([
        registration(candidate_key.clone(), "candidate_runtime", &["CANDIDATE"]),
        registration(stable_key.clone(), "conflicting_runtime", &["CONFLICT"]),
    ])
    .expect_err("installed-key collision must reject the whole batch");
    assert!(error.contains("differing installed artifact"), "{error}");

    assert!(
        register_project_veriloga_runtimes_for_session([registration(
            stable_key,
            "stable_replacement_probe",
            &["STABLE_PROBE"],
        )])
        .is_err(),
        "the stable entry must remain installed after rollback"
    );
    register_project_veriloga_runtimes_for_session([registration(
        candidate_key,
        "candidate_absence_probe",
        &["CANDIDATE_PROBE"],
    )])
    .expect("the rejected candidate must not have been partially installed");
}

#[test]
fn aggregate_resource_failure_rolls_back_without_eviction() {
    let project = project_scope("resource-rollback");
    let stable_key = key(&project, "stable", "model.va");
    let candidate_key = key(&project, "candidate", "candidate.va");
    register_project_veriloga_runtimes_for_session([registration(
        stable_key.clone(),
        "budget_stable",
        &[],
    )])
    .expect("install stable runtime");

    let mut limits = ResourceLimits::default();
    limits.max_shared_cache_bytes = 1;
    let error = register_project_veriloga_runtimes_for_session_with_limits(
        [registration(
            candidate_key.clone(),
            "oversized_candidate",
            &[],
        )],
        limits,
    )
    .expect_err("aggregate runtime set must respect its cache budget");
    assert!(
        error.contains("shared_cache_bytes limit exceeded"),
        "{error}"
    );

    assert!(
        register_project_veriloga_runtimes_for_session([registration(
            stable_key,
            "budget_stable_replacement_probe",
            &[],
        )])
        .is_err(),
        "resource rejection must not evict the stable entry"
    );
    register_project_veriloga_runtimes_for_session([registration(
        candidate_key,
        "budget_candidate_absence_probe",
        &[],
    )])
    .expect("resource rejection must not publish the candidate");
}

#[test]
fn case_colliding_keys_and_aliases_reject_differing_artifacts() {
    let key_project = project_scope("case-key");
    let upper_key = key(&key_project, "digest", "Model.va");
    let lower_key = key(&key_project, "digest", "model.va");
    let key_error = register_project_veriloga_runtimes_for_session([
        registration(upper_key.clone(), "case_upper", &[]),
        registration(lower_key, "case_lower", &[]),
    ])
    .expect_err("case-colliding keys with differing artifacts must fail");
    assert!(key_error.contains("case-colliding"), "{key_error}");
    register_project_veriloga_runtimes_for_session([registration(
        upper_key,
        "case_key_absence_probe",
        &[],
    )])
    .expect("case-key collision must not partially install either artifact");

    let alias_project = project_scope("case-alias");
    let first_key = key(&alias_project, "first", "first.va");
    let second_key = key(&alias_project, "second", "second.va");
    let alias_error = register_project_veriloga_runtimes_for_session([
        registration(first_key.clone(), "alias_first", &["SharedAlias"]),
        registration(second_key, "alias_second", &["sharedalias"]),
    ])
    .expect_err("case-colliding aliases with differing artifacts must fail");
    assert!(alias_error.contains("alias 'SHAREDALIAS'"), "{alias_error}");
    register_project_veriloga_runtimes_for_session([registration(
        first_key,
        "case_alias_absence_probe",
        &[],
    )])
    .expect("case-alias collision must not partially install either artifact");
}

#[test]
fn exact_duplicate_key_with_differing_artifacts_is_rejected_atomically() {
    let project = project_scope("duplicate-key");
    let duplicate_key = key(&project, "digest", "model.va");
    let error = register_project_veriloga_runtimes_for_session([
        registration(duplicate_key.clone(), "duplicate_first", &[]),
        registration(duplicate_key.clone(), "duplicate_second", &[]),
    ])
    .expect_err("an exact duplicate key cannot identify differing artifacts");
    assert!(error.contains("claimed by differing artifacts"), "{error}");

    register_project_veriloga_runtimes_for_session([registration(
        duplicate_key,
        "duplicate_absence_probe",
        &[],
    )])
    .expect("duplicate-key rejection must not publish either artifact");
}

#[test]
fn identical_reinstall_is_idempotent() {
    let project = project_scope("idempotent");
    let source_key = key(&project, "digest", "model.va");
    let runtime = registration(source_key, "idempotent_runtime", &["SAME"]);
    let mut duplicate = runtime.clone();
    duplicate.aliases = vec!["same".to_owned(), "SECOND_ALIAS".to_owned()];
    register_project_veriloga_runtimes_for_session([runtime.clone(), duplicate])
        .expect("identical duplicate artifacts merge their aliases atomically");
    register_project_veriloga_runtimes_for_session([runtime])
        .expect("an exact reinstall must be idempotent");
}

#[test]
fn aliases_are_scoped_to_one_runtime_set_not_all_project_history() {
    let project = project_scope("configuration-history");
    register_project_veriloga_runtimes_for_session([registration(
        key(&project, "configuration-a", "device.va"),
        "configuration_a",
        &["ACTIVE_DEVICE"],
    )])
    .expect("install first configuration runtime");

    register_project_veriloga_runtimes_for_session([registration(
        key(&project, "configuration-b", "device.va"),
        "configuration_b",
        &["active_device"],
    )])
    .expect("a later execution may bind the same alias to another exact source key");
}

#[test]
fn identical_filenames_and_aliases_are_isolated_by_project() {
    let first_project = project_scope("isolation-one");
    let second_project = project_scope("isolation-two");
    let first_key = key(&first_project, "digest", "device.va");
    let second_key = key(&second_project, "digest", "device.va");

    register_project_veriloga_runtimes_for_session([
        registration(first_key.clone(), "isolated_first", &["SHARED_DEVICE"]),
        registration(second_key.clone(), "isolated_second", &["shared_device"]),
    ])
    .expect("aliases and filenames must be isolated between projects");

    assert!(
        register_project_veriloga_runtimes_for_session([registration(
            first_key,
            "first_overwrite_probe",
            &[],
        )])
        .is_err(),
        "first project key must retain its own artifact"
    );
    assert!(
        register_project_veriloga_runtimes_for_session([registration(
            second_key,
            "second_overwrite_probe",
            &[],
        )])
        .is_err(),
        "second project key must retain its own artifact"
    );
}

#[test]
fn identical_filenames_with_distinct_content_keys_are_isolated_within_a_project() {
    let project = project_scope("same-project-file-isolation");
    let first_key = key(&project, "digest-one", "device.va");
    let second_key = key(&project, "digest-two", "device.va");

    register_project_veriloga_runtimes_for_session([
        registration(first_key.clone(), "revision_one", &["REVISION_ONE"]),
        registration(second_key.clone(), "revision_two", &["REVISION_TWO"]),
    ])
    .expect("content-addressed keys must isolate same-named files within a project");

    assert!(
        register_project_veriloga_runtimes_for_session([registration(
            first_key,
            "revision_one_overwrite_probe",
            &[],
        )])
        .is_err()
    );
    assert!(
        register_project_veriloga_runtimes_for_session([registration(
            second_key,
            "revision_two_overwrite_probe",
            &[],
        )])
        .is_err()
    );
}
