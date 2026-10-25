//! A project's Verilog-A compiled through the code slot's receipt and through
//! configured execution, which must agree.

use rspice_simulation::project_veriloga::{
    build_profile, compile_project_source_bundle_runtime, prepare_project_runtime,
};

use crate::simulation::veriloga_tests::test_support::{
    assert_controlled_conductance, project_bundle,
};

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
    assert!(
        !receipt
            .compilation
            .report()
            .canonical_ir
            .digital
            .processes
            .is_empty()
    );
    let configured = compile_project_source_bundle_runtime(
        project_id,
        &bundle,
        "project_controlled",
        None,
        Default::default(),
    )
    .unwrap();
    let editor = prepare_project_runtime(
        project_id,
        &bundle,
        &receipt.compilation.token(),
        "project_controlled",
        receipt.compilation.report(),
        configured.netlist_alias(),
        None,
        Default::default(),
    )
    .unwrap();
    assert_eq!(
        editor, configured,
        "editor and configured execution retain the same artifact"
    );
    assert_eq!(configured.terminal_names().unwrap(), ["p", "n"]);
    assert_controlled_conductance(&configured, 500.0);

    let error = compile_project_source_bundle_runtime(
        project_id,
        &bundle,
        "PROJECT_CONTROLLED",
        None,
        Default::default(),
    )
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
    let error = compile_project_source_bundle_runtime(
        project_id,
        &required,
        "project_controlled",
        None,
        Default::default(),
    )
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
    let error = compile_project_source_bundle_runtime(
        project_id,
        &invalid_binding,
        "project_controlled",
        None,
        Default::default(),
    )
    .unwrap_err()
    .to_string();
    assert!(
        error.contains("not present in the elaboration graph"),
        "{error}"
    );
}
