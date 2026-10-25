//! Project and signed-PDK compilation preserve digital control through sealed
//! source closures, build profiles, backend selection and worker transport.

use super::test_support::assert_controlled_conductance;
use super::*;

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
        &sealed.as_parts().binding,
        sealed.as_parts().archive_digest,
        &sealed.as_parts().veriloga_artifacts,
        &sealed.as_parts().veriloga_bindings[0],
    )
    .unwrap();
    let runtime: PreparedVerilogARuntime =
        serde_json::from_slice(&serde_json::to_vec(&runtime).unwrap()).unwrap();
    runtime.validate().unwrap();
    assert_eq!(runtime.source_digest(), package.archive_digest());
    assert_eq!(runtime.terminal_names().unwrap(), ["p", "n"]);
    let canonical: rspice_veriloga::canonical_ir::CanonicalIrArtifact = serde_json::from_str(
        serde_json::to_value(&runtime).unwrap()["canonical_ir_json"]
            .as_str()
            .unwrap(),
    )
    .unwrap();
    assert!(!canonical.digital.processes.is_empty());
    assert_controlled_conductance(&runtime, 250.0);

    let mut tampered = sealed.as_parts().veriloga_artifacts.clone();
    tampered[0].source.push('\n');
    let error = compile_signed_pdk_source_runtime(
        &sealed.as_parts().binding,
        sealed.as_parts().archive_digest,
        &tampered,
        &sealed.as_parts().veriloga_bindings[0],
    )
    .unwrap_err()
    .to_string();
    assert!(error.contains("no longer matches digest"), "{error}");
}
