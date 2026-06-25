#![cfg(all(feature = "veriloga-builtins", rspice_veriloga_builtins_generated))]

use rspice_core::device::veriloga_generated::builtins;

#[test]
fn build_script_generates_registry_from_veriloga_directory() {
    assert!(
        builtins::builtin_names()
            .iter()
            .any(|name| name.eq_ignore_ascii_case("simple_res")),
        "expected simple_res in generated built-in registry, got {:?}",
        builtins::builtin_names()
    );
    assert!(
        builtins::builtin_names()
            .iter()
            .any(|name| name.eq_ignore_ascii_case("assigned_res")),
        "expected assigned_res in generated built-in registry, got {:?}",
        builtins::builtin_names()
    );
}
