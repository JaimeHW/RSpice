//! Parser policy for Xyce-style Y-device keywords.
//!
//! RSpice uses `Y` for legacy lossy transmission lines. Xyce also has
//! keyword-style Y devices such as `YDELAY` and `YLIN`; unsupported families
//! must fail explicitly instead of parsing as shifted-node transmission lines.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::{ElementKind, Netlist, flatten_netlist_with_models};
use rspice_core::testing::{XyceRunnerConfig, XyceTestRunner};
use std::path::PathBuf;

#[test]
fn xyce_ydevice_keywords_fail_closed_instead_of_yline_parse() {
    let decks = [
        (
            "YDELAY",
            "* Xyce delay Y-device\nYDELAY delay1 2 0 1 0 TD=10N\n.op\n.end\n",
        ),
        (
            "YLIN",
            "* Xyce linear Y-device\nYLIN YLIN1 1 0 2 0 YLIN_MOD1\n.op\n.end\n",
        ),
    ];

    for (keyword, deck) in decks {
        let message = Netlist::parse(deck)
            .expect_err("Xyce Y-device keyword must fail before Y-line parsing")
            .to_string();
        assert!(
            message.contains(keyword)
                && message.contains("unsupported")
                && message.contains("native"),
            "{keyword} error should identify unsupported native Y-device, got: {message}"
        );
    }
}

#[test]
fn xyce_team_memristor_parses_with_canonical_device_namespace() {
    let deck = "Xyce TEAM memristor\n\
                .model mrm1 memristor level=2 ron=50 roff=1k\n\
                ymemristor mr1 in 0 mrm1 ivrelation=1\n\
                .tran 1n 10n\n\
                .print tran I(YMEMRISTOR!MR1) N(YMEMRISTOR!MR1_X) N(YMEMRISTOR!MR1:R)\n\
                .end\n";

    let netlist = Netlist::parse_validated(deck)
        .expect("TEAM memristor and its Xyce output namespace parse and validate");
    assert_eq!(netlist.elements.len(), 1);
    let element = &netlist.elements[0];
    assert_eq!(element.name, "YMEMRISTOR!MR1");
    assert_eq!(element.nodes, ["IN", "0"]);
    match &element.kind {
        ElementKind::XyceMemristor {
            model,
            instance_params,
            deferred_params,
        } => {
            assert_eq!(model, "MRM1");
            assert_eq!(instance_params, &[("IVRELATION".to_string(), 1.0)]);
            assert!(deferred_params.is_empty());
        }
        other => panic!("expected TEAM memristor, got {other:?}"),
    }
}

#[test]
fn xyce_memristor_family_selection_keeps_pem_fail_closed() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tests")
        .join("xyce")
        .canonicalize()
        .expect("vendored Xyce regression root exists");
    let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());

    for relative in [
        "Netlists/MEMRISTOR/memristorPEM1.cir",
        "Netlists/MEMRISTOR/memristorPEM1wIC.cir",
        "Netlists/MEMRISTOR/memristorPEM2.cir",
    ] {
        let result = runner.run_test(root.join(relative));
        assert!(
            result.passed && result.expected_unsupported,
            "{relative} must remain a named unsupported PEM-family contract, got {result:?}"
        );
        assert_eq!(result.contract, "unsupported_xyce_contract");
        assert!(result.mismatches.is_empty());
    }
}

#[test]
fn xyce_team_model_parameters_require_scalar_numeric_values() {
    let deck = "TEAM scalar model-parameter policy\n\
                V1 in 0 0\n\
                .model mrm1 memristor level=2 ron=[50 60]\n\
                YMEMRISTOR mr1 in 0 mrm1\n\
                .op\n\
                .end\n";
    let netlist = Netlist::parse_validated(deck).expect("TEAM vector-value deck validates");
    let message = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect_err("a scalar TEAM parameter must reject a vector value")
        .to_string();
    assert!(
        message.contains("RON")
            && message.contains("vector-valued")
            && message.contains("scalar numeric"),
        "error must identify the scalar TEAM parameter contract: {message}"
    );
}

#[test]
fn xyce_team_memristor_hierarchy_resolves_params_and_local_model_scope() {
    let deck = "hierarchical Xyce TEAM memristor\n\
                X1 in 0 cell PARAMS: relation=1\n\
                .subckt cell p n PARAMS: relation=0\n\
                .model local_team memristor level=2 ron=50 roff=1k\n\
                ymemristor state p n local_team ivrelation={relation}\n\
                .ends\n\
                .end\n";

    let netlist = Netlist::parse(deck).expect("hierarchical TEAM memristor parses");
    let body = &netlist.subcircuits[0].elements[0];
    match &body.kind {
        ElementKind::XyceMemristor {
            model,
            instance_params,
            deferred_params,
        } => {
            assert_eq!(model, "cell::LOCAL_TEAM");
            assert!(instance_params.is_empty());
            assert_eq!(
                deferred_params,
                &[("IVRELATION".to_string(), "relation".to_string())]
            );
        }
        other => panic!("expected deferred TEAM memristor, got {other:?}"),
    }

    let flattened =
        flatten_netlist_with_models(&netlist).expect("hierarchical TEAM memristor flattens");
    let element = &flattened.elements[0];
    assert_eq!(element.name, "X1.YMEMRISTOR!STATE");
    match &element.kind {
        ElementKind::XyceMemristor {
            model,
            instance_params,
            deferred_params,
        } => {
            assert_eq!(model, "cell::LOCAL_TEAM");
            assert!(
                netlist
                    .models
                    .iter()
                    .any(|candidate| candidate.name == *model),
                "the resolved subcircuit-local model must remain available to construction"
            );
            assert_eq!(instance_params, &[("IVRELATION".to_string(), 1.0)]);
            assert!(deferred_params.is_empty());
        }
        other => panic!("expected flattened TEAM memristor, got {other:?}"),
    }
}

#[test]
fn xyce_team_generated_names_reject_authored_node_collisions_order_independently() {
    let decks = [
        (
            "state",
            "TEAM state namespace collision before device\n\
             VCOLLIDE ymemristor!mr1_x 0 0\n\
             .model mrm1 memristor level=2\n\
             YMEMRISTOR mr1 in 0 mrm1\n\
             .op\n\
             .end\n",
        ),
        (
            "store",
            "TEAM store namespace collision after device\n\
             .model mrm1 memristor level=2\n\
             YMEMRISTOR mr1 in 0 mrm1\n\
             RCOLLIDE YMEMRISTOR!MR1.R 0 1\n\
             .op\n\
             .end\n",
        ),
    ];

    for (namespace_kind, deck) in decks {
        let netlist = Netlist::parse_validated(deck).expect("collision fixture parses");
        let message = Engine::new(SimulationConfig::default())
            .build_circuit(&netlist)
            .expect_err("authored nodes must not alias generated TEAM namespaces")
            .to_string();
        assert!(
            message.contains("YMEMRISTOR!MR1")
                && message.contains(namespace_kind)
                && message.contains("collides with authored node")
                && message.contains("case-insensitively"),
            "collision error must identify the reserved TEAM namespace: {message}"
        );
    }
}

#[test]
fn xyce_non_team_memristors_do_not_reserve_team_generated_namespaces() {
    let deck = "PEM namespace boundary\n\
                VCOLLIDE ymemristor!mr1_x 0 0\n\
                .model pem memristor level=4\n\
                YMEMRISTOR mr1 in 0 pem\n\
                .op\n\
                .end\n";
    let netlist = Netlist::parse_validated(deck).expect("PEM namespace fixture parses");
    let message = Engine::new(SimulationConfig::default())
        .build_circuit(&netlist)
        .expect_err("unsupported PEM family must fail at the model-family boundary")
        .to_string();
    assert!(
        message.contains("requires MEMRISTOR LEVEL=2") && !message.contains("collides"),
        "a non-TEAM family must not reserve TEAM generated names: {message}"
    );
}

#[test]
fn xyce_team_generated_names_reject_cross_instance_aliases() {
    let deck = "TEAM generated namespace alias\n\
                V1 in 0 0\n\
                V2 in2 0 0\n\
                .model team memristor level=2\n\
                YMEMRISTOR a:b in 0 team\n\
                YMEMRISTOR a.b in2 0 team\n\
                .op\n\
                .end\n";
    let netlist = Netlist::parse_validated(deck).expect("TEAM alias fixture parses");
    let message = Engine::new(SimulationConfig::default())
        .build_circuit(&netlist)
        .expect_err("two TEAM devices must not share a generated namespace alias")
        .to_string();
    assert!(
        message.contains("TEAM memristors")
            && message.contains("globally unique")
            && message.contains("':' and '.'"),
        "error must identify the cross-instance generated-name alias: {message}"
    );
}

#[test]
fn xyce_team_private_capacitor_has_internal_provenance_and_is_not_an_authored_device() {
    let deck = "TEAM private capacitor provenance\n\
                V1 in 0 0.1\n\
                C1 in 0 1p\n\
                .model mrm1 memristor level=2\n\
                YMEMRISTOR mr1 in 0 mrm1\n\
                .op\n\
                .end\n";
    let netlist = Netlist::parse_validated(deck).expect("TEAM provenance fixture validates");
    let circuit = Engine::new(SimulationConfig::default())
        .build_circuit(&netlist)
        .expect("TEAM provenance fixture builds");
    let capacitors = circuit.capacitor_storage();

    assert_eq!(
        capacitors.len(),
        2,
        "authored and private capacitors are stored"
    );
    assert_eq!(capacitors.authored_len(), 1);
    assert!(!capacitors.is_internal(0));
    assert!(capacitors.is_internal(1));
    assert_eq!(capacitors.names[0], "C1");
    assert_eq!(capacitors.names[1], "__RSPICE_TEAM_Q!YMEMRISTOR!MR1");
    assert_eq!(
        circuit.device_count(),
        3,
        "V1, C1, and the TEAM instance are authored devices; its private Q(x) companion is not"
    );

    let cloned = circuit.clone();
    assert_eq!(
        cloned.capacitor_storage().internal,
        capacitors.internal,
        "private-capacitor provenance must survive circuit snapshots/clones"
    );
}

#[test]
fn ordinary_yline_transmission_line_still_parses() {
    let deck = "* ordinary Y-line transmission line\n\
                Y1 in 0 out 0 Z0=50 TD=1n\n\
                .op\n\
                .end\n";

    let netlist = Netlist::parse(deck).expect("ordinary Y-line transmission line parses");
    assert_eq!(netlist.elements.len(), 1);
    assert_eq!(netlist.elements[0].name, "Y1");
}

#[test]
fn xyce_team_memristor_transient_exposes_only_the_physical_branch_current() {
    let deck = "TEAM transient output contract\n\
                V1 in 0 0.1\n\
                .model mrm1 memristor level=2 ron=50 roff=1k xon=0 xoff=1\n\
                YMEMRISTOR mr1 in 0 mrm1 ivrelation=0\n\
                .tran 1n 4n\n\
                .end\n";
    let netlist = Netlist::parse_validated(deck).expect("TEAM transient deck validates");
    let result = Engine::new(SimulationConfig::default())
        .run_tran(&netlist, 4.0e-9, 1.0e-9)
        .expect("TEAM transient runs");

    let branch_index = result
        .branch_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("YMEMRISTOR!mr1"))
        .expect("the physical TEAM branch current is public");
    assert!(
        result
            .branch_names
            .iter()
            .all(|name| !name.starts_with("__RSPICE_TEAM_Q!")),
        "the private integration companion must not leak into output"
    );
    assert_eq!(
        result.branch_currents[branch_index].len(),
        result.time.len()
    );
    assert!(
        result.branch_currents[branch_index]
            .iter()
            .all(|current| current.is_finite()),
        "accepted TEAM branch-current samples must be finite"
    );
    let resistance = result
        .try_store_waveform_named("YMEMRISTOR!MR1:R")
        .expect("TEAM resistance store waveform is public");
    assert_eq!(resistance.len(), result.time.len());
    assert!(resistance.iter().all(|value| value.is_finite()));
}

#[test]
fn xyce_team_memristor_dc_uses_the_physical_state_equation_and_store_outputs() {
    let deck = "TEAM DC equation and output contract\n\
                V1 in 0 0.1\n\
                .model mrm1 memristor level=2 ron=50 roff=150 xon=0 xoff=1\n\
                + ion=-1m ioff=1m kon=-1 koff=1 alphaon=1 alphaoff=1 wt=0\n\
                YMEMRISTOR mr1 in 0 mrm1\n\
                .op\n\
                .end\n";
    let netlist = Netlist::parse_validated(deck).expect("TEAM DC deck validates");
    let result = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("biased TEAM DC operating point converges");

    let x = result
        .try_voltage_named("YMEMRISTOR!MR1_X")
        .expect("TEAM state is an MNA unknown");
    assert!((x - 0.5).abs() <= 1.0e-8, "physical DC state is {x}");
    assert_eq!(
        result.node_index_named("YMEMRISTOR!MR1:R"),
        None,
        "resistance is a store output, not an MNA unknown"
    );
    let resistance = result
        .try_dc_observable_named("N(YMEMRISTOR!MR1:R)")
        .expect("TEAM DC resistance store is available");
    let current = result
        .try_dc_observable_named("I(YMEMRISTOR!MR1)")
        .expect("TEAM DC lead current is available");
    assert!((resistance - 100.0).abs() <= 1.0e-6);
    assert!((current - 1.0e-3).abs() <= 1.0e-11);
}

#[test]
fn xyce_team_deadband_gauge_and_active_dc_root_are_independent_of_nodal_gmin() {
    let deadband = Netlist::parse_validated(
        "TEAM deadband gauge\n\
         V1 in 0 0\n\
         .model mrm1 memristor level=2 ion=-8.9u\n\
         YMEMRISTOR mr1 in 0 mrm1\n\
         .op\n\
         .end\n",
    )
    .expect("TEAM deadband fixture validates");
    let mut zero_gmin = SimulationConfig::default();
    zero_gmin.convergence_config.gmin_target = 0.0;
    let deadband_result = Engine::new(zero_gmin.clone())
        .run_dc_op(&deadband)
        .expect("rank-deficient deadband receives its explicit DC gauge");
    assert_eq!(
        deadband_result
            .try_voltage_named("YMEMRISTOR!MR1_X")
            .expect("TEAM state exists")
            .to_bits(),
        0.0f64.to_bits()
    );

    let active = Netlist::parse_validated(
        "TEAM GMIN-invariant active root\n\
         V1 in 0 0.1\n\
         .model mrm1 memristor level=2 ron=50 roff=150 xon=0 xoff=1\n\
         + ion=-1m ioff=1m kon=-1 koff=1 alphaon=1 alphaoff=1 wt=0\n\
         YMEMRISTOR mr1 in 0 mrm1\n\
         .op\n\
         .end\n",
    )
    .expect("TEAM active-root fixture validates");
    let zero_root = Engine::new(zero_gmin)
        .run_dc_op(&active)
        .expect("zero-GMIN active root converges")
        .try_voltage_named("YMEMRISTOR!MR1_X")
        .expect("TEAM state exists");

    let mut large_gmin = SimulationConfig::default();
    large_gmin.convergence_config.gmin_target = 1.0e-2;
    large_gmin.convergence_config.gmin_initial = 1.0e-2;
    let large_root = Engine::new(large_gmin)
        .run_dc_op(&active)
        .expect("large electrical GMIN cannot perturb the TEAM state row")
        .try_voltage_named("YMEMRISTOR!MR1_X")
        .expect("TEAM state exists");
    assert!((zero_root - 0.5).abs() <= 1.0e-8);
    assert!((large_root - zero_root).abs() <= 1.0e-10);
}

#[test]
fn xyce_team_transient_state_trajectory_is_independent_of_nodal_gmin() {
    let netlist = Netlist::parse_validated(
        "TEAM transient GMIN invariance\n\
         V1 in 0 0.1\n\
         .model mrm1 memristor level=2 ron=50 roff=150 xon=0 xoff=1\n\
         + ion=-1m ioff=1m kon=-1 koff=1 alphaon=1 alphaoff=1 wt=0\n\
         YMEMRISTOR mr1 in 0 mrm1\n\
         .tran 1m 4m\n\
         .end\n",
    )
    .expect("TEAM transient GMIN fixture validates");

    let run = |gmin: f64| {
        let mut config = SimulationConfig::default();
        config.convergence_config.gmin_target = gmin;
        config.convergence_config.gmin_initial = gmin;
        Engine::new(config)
            .run_tran(&netlist, 4.0e-3, 1.0e-3)
            .expect("TEAM transient trajectory converges")
    };
    let zero = run(0.0);
    let large = run(1.0e-2);
    assert_eq!(zero.time, large.time);
    let zero_r = zero
        .try_store_waveform_named("YMEMRISTOR!MR1:R")
        .expect("zero-GMIN resistance trace exists");
    let large_r = large
        .try_store_waveform_named("YMEMRISTOR!MR1:R")
        .expect("large-GMIN resistance trace exists");
    assert_eq!(zero_r.len(), large_r.len());
    for (index, (&expected, &actual)) in zero_r.iter().zip(large_r).enumerate() {
        assert!(
            (actual - expected).abs() <= 1.0e-9,
            "electrical GMIN perturbed TEAM resistance at sample {index}: {expected} vs {actual}"
        );
    }
}

#[test]
fn xyce_team_memristor_stochastic_requests_fail_closed_exactly() {
    let deck = "TEAM stochastic policy\n\
                V1 in 0 0\n\
                .model mrm1 memristor level=2 resnoise=1e-16\n\
                YMEMRISTOR mr1 in 0 mrm1\n\
                .op\n\
                .end\n";
    let netlist = Netlist::parse_validated(deck).expect("TEAM stochastic deck validates");
    let message = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect_err("every nonzero stochastic request must fail closed")
        .to_string();
    assert!(
        message.contains("RESNOISE") && message.contains("unsupported"),
        "error must identify the unsupported stochastic contract: {message}"
    );
}

#[test]
fn xyce_team_memristor_small_signal_analysis_fails_closed() {
    let deck = "TEAM AC policy\n\
                V1 in 0 0.1 AC 1\n\
                .model mrm1 memristor level=2\n\
                YMEMRISTOR mr1 in 0 mrm1\n\
                .ac dec 1 1 10\n\
                .end\n";
    let netlist = Netlist::parse_validated(deck).expect("TEAM AC deck validates");
    let message = Engine::new(SimulationConfig::default())
        .run_ac(&netlist, &[1.0])
        .expect_err("TEAM small-signal analysis must fail until it is implemented")
        .to_string();
    assert!(
        message.contains("TEAM") && message.contains("small-signal"),
        "error must identify the unsupported TEAM small-signal contract: {message}"
    );
}

#[test]
fn xyce_team_memristor_pss_analysis_fails_closed() {
    let deck = "TEAM PSS policy\n\
                V1 in 0 SIN(0 0.1 1k)\n\
                .model mrm1 memristor level=2\n\
                YMEMRISTOR mr1 in 0 mrm1\n\
                .end\n";
    let netlist = Netlist::parse_validated(deck).expect("TEAM PSS deck validates");
    let message = Engine::new(SimulationConfig::default())
        .run_pss(&netlist, rspice_core::analysis::PssConfig::default())
        .expect_err("TEAM PSS must fail until its periodic state contract is implemented")
        .to_string();
    assert!(
        message.contains("TEAM") && message.contains("periodic dynamic-state"),
        "error must identify the unsupported TEAM PSS contract: {message}"
    );
}
