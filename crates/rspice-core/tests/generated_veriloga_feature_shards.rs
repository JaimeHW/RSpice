#![cfg(feature = "veriloga-builtins-base")]

#[cfg(feature = "veriloga-model-diode-cmc")]
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

/// A build that carries the generated catalog without its noise schedules
/// contributes no noise from a generated card at all, where the native port
/// for the same level contributes several. Nothing used to say so: the
/// descriptor table was simply empty, so a `DNO` against the instance reported
/// a device the deck plainly contains as unknown, which reads as a broken deck
/// rather than as the build decision it is.
#[cfg(feature = "veriloga-model-diode-cmc")]
#[test]
fn a_probe_on_a_generated_instance_names_the_missing_noise_feature() {
    use rspice_core::analysis::NoiseContributionProbe;
    use rspice_core::engine::{Engine, SimulationConfig};
    use rspice_core::netlist::Netlist;

    let deck = "* DIODE_CMC noise probe
                vin in 0 dc 0.7 ac 1
                r1 in a 1k
                d1 a 0 dmod
                .model dmod D level=2002
                .end
";
    let netlist = Netlist::parse(deck).expect("DIODE_CMC deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let output = engine
        .build_circuit(&netlist)
        .expect("circuit builds")
        .get_node_by_name("a")
        .expect("output node");
    let results = engine
        .run_noise_with_input_source(&netlist, output, None, "vin", &[1.0e3], 300.15)
        .expect("noise analysis runs");
    let result = &results[0];
    let probe = NoiseContributionProbe::parse("DNO(D1)").expect("probe parses");

    #[cfg(feature = "veriloga-builtins-noise")]
    {
        assert!(
            result.mechanisms_unavailable.is_empty(),
            "a build carrying the noise schedules withholds nothing: {:?}",
            result.mechanisms_unavailable
        );
        assert!(
            result.contribution(&probe).is_ok(),
            "DNO(D1) must resolve against a generated instance the deck contains"
        );
    }

    #[cfg(not(feature = "veriloga-builtins-noise"))]
    {
        assert_eq!(
            result
                .mechanisms_unavailable
                .iter()
                .map(|instance| instance.to_ascii_lowercase())
                .collect::<Vec<_>>(),
            vec!["d1".to_string()],
            "the instance whose schedules this build omitted has to be named"
        );
        let error = result
            .contribution(&probe)
            .expect_err("a generated instance without its schedules answers no probe");
        assert!(
            matches!(
                error,
                rspice_core::analysis::NoiseContributionProbeError::MechanismsUnavailable {
                    ref device,
                    feature,
                    // The Cargo feature that compiles the generated noise
                    // schedules, spelled the way a rebuild has to spell it.
                } if device.eq_ignore_ascii_case("d1") && feature == "veriloga-builtins-noise"
            ),
            "the probe must name the missing feature rather than the device: {error}"
        );
    }
}
