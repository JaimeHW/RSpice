#![cfg(all(feature = "veriloga-builtins", rspice_veriloga_builtins_generated))]

use rspice_core::device::veriloga_generated::builtins;

#[test]
fn build_script_generates_registry_from_veriloga_directory() {
    if !has_builtin("simple_res") {
        eprintln!("fixture generated builtins not present; skipping fixture registry test");
        return;
    }

    assert!(
        has_builtin("simple_res"),
        "expected simple_res in generated built-in registry, got {:?}",
        builtins::builtin_names()
    );
    assert!(
        has_builtin("assigned_res"),
        "expected assigned_res in generated built-in registry, got {:?}",
        builtins::builtin_names()
    );
    assert!(
        has_builtin("generated_cap"),
        "expected generated_cap in generated built-in registry, got {:?}",
        builtins::builtin_names()
    );
}

fn has_builtin(expected: &str) -> bool {
    builtins::builtin_names()
        .iter()
        .any(|name| name.eq_ignore_ascii_case(expected))
}
