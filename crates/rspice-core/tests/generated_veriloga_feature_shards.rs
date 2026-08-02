#![cfg(feature = "veriloga-builtins-base")]

use rspice_core::device::veriloga_builtins::builtins;

#[cfg(feature = "veriloga-model-diode-cmc")]
#[test]
fn a_single_model_feature_exposes_only_the_selected_registry_entry() {
    assert!(builtins::builtin_names().contains(&"DIODE_CMC"));

    #[cfg(not(feature = "veriloga-model-juncap200"))]
    assert!(!builtins::builtin_names().contains(&"JUNCAP200"));
}

#[cfg(feature = "veriloga-model-diode-cmc")]
#[test]
fn generated_noise_is_independently_selectable() {
    let nodes =
        (0..builtins::total_node_count("DIODE_CMC").expect("node count")).collect::<Vec<_>>();
    let branches =
        (0..builtins::branch_count("DIODE_CMC").expect("branch count")).collect::<Vec<_>>();
    let instance = builtins::instantiate("DIODE_CMC", &nodes, &branches, &[])
        .expect("instantiate selected generated model")
        .expect("selected model must be present");

    #[cfg(feature = "veriloga-builtins-noise")]
    assert!(
        !instance.noise_descriptors().is_empty(),
        "the selected model must expose its generated noise descriptors"
    );

    #[cfg(not(feature = "veriloga-builtins-noise"))]
    assert!(
        instance.noise_descriptors().is_empty(),
        "noise translation units must stay excluded unless explicitly selected"
    );
}
