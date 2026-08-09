#![cfg(feature = "veriloga-model-bsimsoi-va")]

use rspice_core::SpiceDialect;
use rspice_core::device::veriloga_builtins::builtins;
use rspice_core::engine::{ConvergenceConfig, Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

fn xyce_engine() -> Engine {
    let defaults = SimulationConfig::default();
    let mut convergence_config = ConvergenceConfig::robust();
    convergence_config.voltage_reltol = 1.0e-4;
    Engine::new(SimulationConfig {
        max_iterations: defaults.max_iterations.max(1200),
        convergence_config,
        spice_dialect: SpiceDialect::Xyce,
        temperature: 300.15,
        ..defaults
    })
}

#[test]
fn xyce_level70_routes_exclusively_to_generated_bsimsoi461() {
    assert!(
        builtins::builtin_names()
            .iter()
            .any(|name| name.eq_ignore_ascii_case("bsimsoi_va")),
        "the exact BSIM-SOI 4.6.1 generated artifact must be compiled"
    );

    let netlist = Netlist::parse(
        r#"
* BSIM-SOI 4.6.1 generated routing
VDD d 0 1.5
VG g 0 0
M1 d g s e NSOI W=10u L=.18u
.MODEL NSOI NMOS LEVEL=70 VERSION=4.6 TYPE=1 SOIMOD=0 SHMOD=0
.OP
.END
"#,
    )
    .expect("LEVEL=70 routing fixture parses");
    let circuit = Engine::new(SimulationConfig::default())
        .build_circuit(&netlist)
        .expect("LEVEL=70 builds through the generated BSIM-SOI model");

    assert!(circuit.has_generated_veriloga_devices());
    assert_eq!(
        circuit.device_count(),
        3,
        "the two sources and one generated MOSFET must be the only devices"
    );
    let report = circuit.device_op_report();
    let m1_entries = report
        .entries
        .iter()
        .filter(|entry| entry.name.eq_ignore_ascii_case("M1"))
        .collect::<Vec<_>>();
    assert_eq!(
        m1_entries.len(),
        1,
        "LEVEL=70 must produce exactly one generated device report"
    );
    assert_eq!(
        m1_entries[0].device_kind.to_ascii_uppercase(),
        "BSIMSOI_VA",
        "LEVEL=70 must not also instantiate a native or simplified MOSFET"
    );
}

#[test]
fn generated_level70_reports_physical_drain_current() {
    let deck = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tests/xyce/Netlists/B4SOI_461/test1.cir")
        .canonicalize()
        .expect("BSIM-SOI 4.6.1 regression deck exists");
    let netlist = Netlist::parse_file(&deck).expect("LEVEL=70 current-report fixture parses");
    let engine = xyce_engine();
    let (result, report) = engine
        .run_dc_op_with_report(&netlist)
        .expect("generated BSIM-SOI operating point solves");
    let device = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("M1"))
        .expect("generated M1 has an operating-point report");
    let id = device
        .params
        .iter()
        .find_map(|(name, value)| (*name == "id").then_some(*value))
        .expect("generated MOS report exposes drain lead current");
    let vd_current = result
        .branch_current_named("VD")
        .expect("drain voltage-source branch current exists");
    let thermal_node = result
        .try_dc_observable_named("N(M1_t)")
        .expect("generated internal thermal node is a device-owned DC observable");

    assert!(id.is_finite(), "reported drain current must be finite");
    assert!(
        thermal_node.is_finite(),
        "generated internal-node observable must be finite"
    );
    assert!(
        (id + vd_current).abs() <= 1.0e-18_f64.max(id.abs() * 1.0e-4),
        "generated drain current must satisfy external-node KCL: ID={id}, I(VD)={vd_current}"
    );
}

#[test]
fn generated_internal_node_observable_does_not_alias_authored_node() {
    let deck = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tests/xyce/Netlists/B4SOI_461/test1.cir")
        .canonicalize()
        .expect("BSIM-SOI 4.6.1 regression deck exists");
    let source = std::fs::read_to_string(&deck)
        .expect("BSIM-SOI deck is readable")
        .replacen("ve substrate 0 0", "ve substrate 0 0\nVALIAS M1_t 0 7", 1);
    let netlist = Netlist::parse_with_path(&source, &deck)
        .expect("authored-node collision fixture parses with the canonical model card");
    let (result, _) = xyce_engine()
        .run_dc_op_with_report(&netlist)
        .expect("authored-node collision operating point solves");

    let authored = result
        .try_voltage_named("M1_t")
        .expect("authored M1_t node remains addressable as a circuit voltage");
    let generated = result
        .try_dc_observable_named("N(M1_t)")
        .expect("generated M1 t node remains addressable as a device-owned vector");
    assert!((authored - 7.0).abs() < 1.0e-12);
    assert!(generated.is_finite());
    assert!(
        (generated - authored).abs() > 1.0,
        "N(M1_t) must retain device ownership instead of aliasing authored V(M1_t)"
    );
}

#[test]
fn native_b3soi_levels_are_not_shadowed_by_generated_bsimsoi461() {
    for (model_selector, expected_kind) in
        [("LEVEL=10 SOIMOD=0", "B3SOIPD"), ("LEVEL=57", "B3SOIPD")]
    {
        let source = format!(
            r#"
* Native BSIM3-SOI routing must survive the generated 4.6.1 feature
VD d 0 1
VG g 0 .8
M1 d g 0 0 NSOI W=4u L=1u
.MODEL NSOI NMOS {model_selector} CAPMOD=2
.OP
.END
"#
        );
        let netlist = Netlist::parse(&source).expect("native B3SOI routing fixture parses");
        let engine = Engine::new(SimulationConfig::default());
        let circuit = engine
            .build_circuit(&netlist)
            .unwrap_or_else(|error| panic!("{model_selector} must build natively: {error}"));
        assert!(
            !circuit.has_generated_veriloga_devices(),
            "{model_selector} must not instantiate the LEVEL=70 generated artifact"
        );

        let (_, report) = engine
            .run_dc_op_with_report(&netlist)
            .unwrap_or_else(|error| panic!("{model_selector} must run natively: {error}"));
        let device = report
            .entries
            .iter()
            .find(|entry| entry.name.eq_ignore_ascii_case("M1"))
            .unwrap_or_else(|| panic!("{model_selector} must produce a native M1 OP report"));
        assert_eq!(
            device.device_kind, expected_kind,
            "{model_selector} must retain its native BSIM3-SOI family"
        );
    }
}

#[test]
fn generated_level70_routes_strict_dotted_version_metadata() {
    let netlist = Netlist::parse(
        r#"
* Generated model dotted VERSION metadata
VDD d 0 1.5
VG g 0 0
M1 d g s e NSOI W=10u L=.18u
.MODEL NSOI NMOS LEVEL=70 VERSION=4.6.1 TYPE=1 SOIMOD=0 SHMOD=0
.OP
.END
"#,
    )
    .expect("dotted VERSION routing fixture parses");
    assert_eq!(
        netlist.models[0].string_params,
        [("VERSION".to_string(), "4.6.1".to_string())],
        "the parser must preserve multi-component VERSION metadata losslessly"
    );

    let circuit = Engine::new(SimulationConfig::default())
        .build_circuit(&netlist)
        .expect("strict dotted VERSION metadata routes to the generated model");
    assert!(circuit.has_generated_veriloga_devices());
}

#[test]
fn generated_level70_rejects_nonnumeric_version_metadata() {
    let netlist = Netlist::parse(
        r#"
* Generated model invalid VERSION metadata
VDD d 0 1.5
VG g 0 0
M1 d g s e NSOI W=10u L=.18u
.MODEL NSOI NMOS LEVEL=70 VERSION="release" TYPE=1 SOIMOD=0 SHMOD=0
.OP
.END
"#,
    )
    .expect("invalid VERSION routing fixture remains syntactically valid");
    let error = Engine::new(SimulationConfig::default())
        .build_circuit(&netlist)
        .expect_err("nonnumeric VERSION metadata must fail closed")
        .to_string();
    assert!(
        error.contains("invalid VERSION=\"release\"")
            && error.contains("finite dotted numeric version"),
        "diagnostic must identify the rejected generated-model VERSION metadata: {error}"
    );
}
