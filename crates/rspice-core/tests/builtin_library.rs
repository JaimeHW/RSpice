//! Smoke coverage for the small RSpice-authored foundation library.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::library::LibraryManager;
use rspice_core::netlist::Netlist;

fn probe_deck(name: &str, model_type: &str) -> Option<String> {
    let head = format!("* foundation probe {name}\n");
    let body = match model_type.to_ascii_uppercase().as_str() {
        "D" => format!("V1 a 0 0.6\nD1 a 0 {name}\n"),
        "NPN" => format!("V1 c 0 5\nV2 b 0 0.65\nQ1 c b 0 {name}\n"),
        "PNP" => format!("V1 c 0 -5\nV2 b 0 -0.65\nQ1 c b 0 {name}\n"),
        "NJF" => format!("V1 d 0 5\nV2 g 0 -0.5\nJ1 d g 0 {name}\n"),
        "PJF" => format!("V1 d 0 -5\nV2 g 0 0.5\nJ1 d g 0 {name}\n"),
        "NMOS" => format!("V1 d 0 5\nV2 g 0 3\nM1 d g 0 0 {name}\n"),
        "PMOS" => format!("V1 d 0 -5\nV2 g 0 -3\nM1 d g 0 0 {name}\n"),
        _ => return None,
    };
    Some(format!("{head}{body}.op\n.end\n"))
}

#[test]
fn every_foundation_model_card_solves() {
    let manager = LibraryManager::new();
    let content = manager
        .get_library_content("foundation.lib")
        .expect("foundation library is embedded");
    let library = Netlist::parse(content).expect("foundation library parses");
    let mut probed = 0usize;
    let mut failures = Vec::new();

    for model in &library.models {
        let Some(deck) = probe_deck(&model.name, &model.model_type) else {
            continue;
        };
        probed += 1;
        match Netlist::parse(&deck) {
            Ok(parsed) => {
                if let Err(error) = Engine::new(SimulationConfig::default()).run_dc_op(&parsed) {
                    failures.push(format!("{} does not solve: {error}", model.name));
                }
            }
            Err(error) => failures.push(format!("{} does not parse: {error}", model.name)),
        }
    }

    assert!(
        failures.is_empty(),
        "{} of {probed} foundation cards failed:\n  {}",
        failures.len(),
        failures.join("\n  ")
    );
    assert_eq!(probed, 8, "expected eight foundation model cards");
}

#[test]
fn foundation_library_is_small_and_indexed() {
    let manager = LibraryManager::new();
    let content = manager
        .get_library_content("foundation.lib")
        .expect("foundation library is embedded");
    Netlist::parse(content).expect("foundation library parses");
    assert_eq!(manager.model_count(), 8);
    assert_eq!(manager.subcircuit_count(), 1);
    assert!(
        content.len() < 4096,
        "foundation library unexpectedly grew to {} bytes",
        content.len()
    );
}

#[test]
fn foundation_mosfet_conducts() {
    let netlist = Netlist::parse(
        "* foundation mosfet\nV1 d 0 5\nV2 g 0 3\nM1 d g 0 0 RSPICE_NMOS\n.op\n.end\n",
    )
    .expect("deck parses");
    assert_eq!(netlist.lint_unknown_references(), Vec::new());
    let result = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("operating point converges");
    let drain_current = -result
        .branch_current_named("v1")
        .expect("v1 branch current present");
    assert!(
        drain_current > 0.0,
        "expected conduction, got {drain_current:e} A"
    );
}

#[test]
fn foundation_opamp_subcircuit_is_an_implicit_fallback() {
    let netlist = Netlist::parse(
        "* foundation opamp follower\nV1 in 0 0.1\nX1 in out out RSPICE_OPAMP\nR1 out 0 10k\n.op\n.end\n",
    )
    .expect("deck parses");
    assert_eq!(netlist.lint_unknown_references(), Vec::new());
    let result = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("operating point converges");
    let output = result
        .try_voltage_named("out")
        .expect("output voltage present");
    assert!((output - 0.1).abs() < 1.0e-4, "unexpected output {output}");
}
