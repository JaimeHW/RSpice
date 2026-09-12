use super::*;
use crate::state::{
    ProjectSourceBundle, ProjectSourceDependency, ProjectSourceFile, ProjectSourceLanguage,
    ProjectSourceOwner, ProjectSourceRole, ProjectSourceRoleBinding,
};

const PROJECT_SOURCE: &str = r#"`include "gain.vams"
module project_controlled(p, n);
    inout p, n; electrical p, n;
    reg selected;
    initial begin selected = 1; #1 selected = 0; end
    analog I(p,n) <+ `PROFILE_SCALE * `BASE_GAIN * (1 + selected) * V(p,n);
endmodule
"#;

fn project_bundle(profile: &build_profile::VerilogABuildProfile) -> ProjectSourceBundle {
    ProjectSourceBundle::try_new_with_roles(
        ProjectSourceOwner::code_workspace(ProjectSourceLanguage::VerilogA),
        ProjectSourceLanguage::VerilogA,
        "model.va",
        PROJECT_SOURCE,
        [
            ProjectSourceFile::try_new("models/gain.vams", "`define BASE_GAIN 0.001\n").unwrap(),
            ProjectSourceFile::try_new("config/build.data", profile.to_toml().unwrap()).unwrap(),
        ],
        [
            ProjectSourceDependency::try_new("model.va", "models/gain.vams").unwrap(),
            ProjectSourceDependency::try_new("model.va", "config/build.data").unwrap(),
        ],
        [ProjectSourceRoleBinding::try_new(
            "config/build.data",
            ProjectSourceRole::VerilogABuildProfile,
        )
        .unwrap()],
    )
    .unwrap()
}

fn assert_controlled_conductance(runtime: &PreparedVerilogARuntime, series_resistance: f64) {
    runtime.install().unwrap();
    let deck = rspice_core::Netlist::parse(&format!(
        "profiled mixed device\nV1 input 0 1\nR1 input out {series_resistance}\nX1 out 0 {}\n{}\n.end\n",
        runtime.netlist_alias(), project_veriloga_directive(runtime.source_key(), runtime.netlist_alias()),
    )).unwrap();
    let result = rspice_core::Engine::default()
        .run_tran(&deck, 2e-9, 0.2e-9)
        .unwrap();
    let out = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .unwrap();
    let mut checked = [0usize; 2];
    for (&time, &voltage) in result.time.iter().zip(&result.voltages[out]) {
        let (phase, expected) = if time < 0.8e-9 {
            (0, 1.0 / 3.0)
        } else if time > 1.2e-9 {
            (1, 0.5)
        } else {
            continue;
        };
        checked[phase] += 1;
        assert!(
            (voltage - expected).abs() < 1e-9,
            "{} at {time:e}: {voltage}, expected {expected}",
            runtime.source_key()
        );
    }
    assert!(checked.into_iter().all(|count| count > 0));
}

#[test]
fn unified_mixed_project_compilation_preserves_profile_and_backend_requirements() {
    use crate::workbench::documents::code_workspace::compile_project_bundle_receipt;
    let mut profile = build_profile::VerilogABuildProfile::starter("project_controlled");
    profile.include_paths.push("models".to_owned());
    profile
        .preprocessor
        .defines
        .insert("PROFILE_SCALE".to_owned(), "2".to_owned());
    let bundle = project_bundle(&profile);
    let project_id = crate::product::ProjectId::new();
    let receipt =
        compile_project_bundle_receipt(project_id, &bundle, Some("project_controlled")).unwrap();
    assert!(!receipt.report.canonical_ir.digital.processes.is_empty());
    let configured =
        compile_project_source_bundle_runtime(project_id, &bundle, "project_controlled").unwrap();
    let editor = PreparedVerilogARuntime::try_new(
        project_id,
        &bundle,
        &receipt.token,
        "project_controlled",
        &receipt.report,
        configured.netlist_alias(),
    )
    .unwrap();
    assert_eq!(
        editor, configured,
        "editor and configured execution retain the same artifact"
    );
    assert_eq!(configured.terminal_names().unwrap(), ["p", "n"]);
    assert_controlled_conductance(&configured, 500.0);

    let error = compile_project_source_bundle_runtime(project_id, &bundle, "PROJECT_CONTROLLED")
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("not declared by the Verilog-A build profile"),
        "{error}"
    );
    let error = compile_project_bundle_receipt(project_id, &bundle, Some("PROJECT_CONTROLLED"))
        .unwrap_err();
    assert!(format!("{error:?}").contains("not declared by the Verilog-A build profile"));

    profile.targets.generated_rust = true;
    profile.targets.fallback = build_profile::VerilogAFallbackPolicy::Reject;
    let required = project_bundle(&profile);
    let error = compile_project_source_bundle_runtime(project_id, &required, "project_controlled")
        .unwrap_err()
        .to_string();
    assert!(error.contains("digital process execution"), "{error}");
    let error = compile_project_bundle_receipt(project_id, &required, Some("project_controlled"))
        .unwrap_err();
    assert!(format!("{error:?}").contains("digital process execution"));

    profile.targets.generated_rust = false;
    profile
        .cell_bindings
        .insert("missing".to_owned(), "project_controlled".to_owned());
    let invalid_binding = project_bundle(&profile);
    let error =
        compile_project_source_bundle_runtime(project_id, &invalid_binding, "project_controlled")
            .unwrap_err()
            .to_string();
    assert!(
        error.contains("not present in the elaboration graph"),
        "{error}"
    );
}

#[test]
fn unified_mixed_signed_pdk_compilation_retains_discrete_control_and_authentication() {
    let source = br#"`include "parts/resistance.vams"
module pdk_resistor(p, n);
    inout p, n; electrical p, n;
    parameter real r = `PDK_RESISTANCE;
    reg selected;
    initial begin selected = 1; #1 selected = 0; end
    analog I(p,n) <+ (1 + selected) * V(p,n) / r;
endmodule
"#;
    let (archive, trust, authority) =
        crate::state::pdk_config::signed_veriloga_source_test_fixture(source);
    let mut registry = crate::state::pdk_config::PdkTechnologyRegistry::default();
    registry
        .install_archive_bytes(&archive, &trust, &authority, "Qualify signed mixed runtime")
        .unwrap();
    let package = registry.validated_packages()[0].clone();
    let sealed = registry
        .seal_model_sources_for_binding(&package.binding(), package.archive_digest())
        .unwrap();
    let runtime = compile_signed_pdk_source_runtime(
        &sealed.binding,
        sealed.archive_digest,
        &sealed.veriloga_artifacts,
        &sealed.veriloga_bindings[0],
    )
    .unwrap();
    let runtime: PreparedVerilogARuntime =
        serde_json::from_slice(&serde_json::to_vec(&runtime).unwrap()).unwrap();
    runtime.validate().unwrap();
    assert_eq!(runtime.source_digest(), package.archive_digest());
    assert_eq!(runtime.terminal_names().unwrap(), ["p", "n"]);
    let canonical: rspice_veriloga::canonical_ir::CanonicalIrArtifact =
        serde_json::from_str(&runtime.canonical_ir_json).unwrap();
    assert!(!canonical.digital.processes.is_empty());
    assert_controlled_conductance(&runtime, 250.0);

    let mut tampered = sealed.veriloga_artifacts.clone();
    tampered[0].source.push('\n');
    let error = compile_signed_pdk_source_runtime(
        &sealed.binding,
        sealed.archive_digest,
        &tampered,
        &sealed.veriloga_bindings[0],
    )
    .unwrap_err()
    .to_string();
    assert!(error.contains("no longer matches digest"), "{error}");
}
