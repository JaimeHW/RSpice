//! Smoke coverage for the small RSpice-authored foundation library.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::library::{FoundationDeviceFamily, LibraryManager, foundation_card_source};
use rspice_core::netlist::{ModelDef, Netlist};

/// The `LEVEL=` a card declares, if any. Two foundation families share a SPICE
/// type token with a simpler family and are told apart only by level:
/// `NPN`/`PNP` LEVEL=4 is VBIC, `NMOS`/`PMOS` LEVEL=57 is BSIMSOI.
fn declared_level(model: &ModelDef) -> Option<i32> {
    model
        .params
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("LEVEL"))
        .map(|(_, value)| value.round() as i32)
}

/// A deck that biases one foundation card into conduction.
///
/// Every card gets one: an `.op` that merely converges proves nothing about a
/// generic starter model, because a device biased into cut-off converges
/// trivially — which is how a p-channel JFET card whose VTO had the wrong sign
/// passed this file for as long as it only asked whether the deck solved.
///
/// The card is written into the deck rather than left to the engine's embedded
/// fallback, because that is the shape a generated deck has: only the diode,
/// bipolar, MOSFET and JFET bind sites consult the embedded library, so a
/// probe leaning on it would test nothing for the MESFET, VDMOS, SOI and
/// thermal-bipolar cards.
fn probe_deck(model: &ModelDef) -> Option<String> {
    let name = &model.name;
    let card = foundation_card_source(name).expect("every library card has verbatim source");
    let head = format!("* foundation probe {name}\n{card}\n");
    let level = declared_level(model);
    let body = match model.model_type.to_ascii_uppercase().as_str() {
        "D" => format!("V1 a 0 0.6\nD1 a 0 {name}\n"),
        // VBIC's thermal node is optional on a three-terminal placement, so
        // one collector/base bias exercises both bipolar levels.
        "NPN" => format!("V1 c 0 5\nV2 b 0 0.65\nQ1 c b 0 {name}\n"),
        "PNP" => format!("V1 c 0 -5\nV2 b 0 -0.65\nQ1 c b 0 {name}\n"),
        "NJF" => format!("V1 d 0 5\nV2 g 0 -0.5\nJ1 d g 0 {name}\n"),
        "PJF" => format!("V1 d 0 -5\nV2 g 0 0.5\nJ1 d g 0 {name}\n"),
        "NMF" => format!("V1 d 0 5\nV2 g 0 -0.5\nZ1 d g 0 {name}\n"),
        "PMF" => format!("V1 d 0 -5\nV2 g 0 0.5\nZ1 d g 0 {name}\n"),
        // The SOI cards take the five-terminal placement the schematic
        // generator emits, with the geometry BSIMSOI needs to converge.
        "NMOS" if level == Some(57) => {
            format!("V1 d 0 3\nV2 g 0 3\nM1 d g 0 0 0 {name} W=1u L=180n\n")
        }
        "PMOS" if level == Some(57) => {
            format!("V1 d 0 -3\nV2 g 0 -3\nM1 d g 0 0 0 {name} W=1u L=180n\n")
        }
        "NMOS" => format!("V1 d 0 5\nV2 g 0 3\nM1 d g 0 0 {name}\n"),
        "PMOS" => format!("V1 d 0 -5\nV2 g 0 -3\nM1 d g 0 0 {name}\n"),
        "NVDMOS" => format!("V1 d 0 10\nV2 g 0 6\nM1 d g 0 0 {name}\n"),
        "PVDMOS" => format!("V1 d 0 -10\nV2 g 0 -6\nM1 d g 0 0 {name}\n"),
        _ => return None,
    };
    Some(format!("{head}{body}.op\n.end\n"))
}

/// Current below which the probed device is not conducting in any useful
/// sense: an off device leaks at saturation-current scale, several decades
/// under this, while every card here is biased hard on.
const CONDUCTION_FLOOR_AMPS: f64 = 1.0e-9;

/// Solve one probe deck and report the magnitude of the current it draws.
fn probed_current(deck: &str) -> Result<f64, String> {
    let parsed = Netlist::parse(deck).map_err(|error| format!("does not parse: {error}"))?;
    let result = Engine::new(SimulationConfig::default())
        .run_dc_op(&parsed)
        .map_err(|error| format!("does not solve: {error}"))?;
    Ok(result
        .branch_current_named("v1")
        .map(f64::abs)
        .unwrap_or(0.0))
}

#[test]
fn every_foundation_model_card_solves_and_conducts() {
    let manager = LibraryManager::new();
    let content = manager
        .get_library_content("foundation.lib")
        .expect("foundation library is embedded");
    let library = Netlist::parse(content).expect("foundation library parses");
    let mut probed = 0usize;
    let mut failures = Vec::new();

    for model in &library.models {
        let Some(deck) = probe_deck(model) else {
            failures.push(format!(
                "{} ({}) has no bias probe; every foundation card must be exercised",
                model.name, model.model_type
            ));
            continue;
        };
        probed += 1;
        match probed_current(&deck) {
            Ok(current) if current > CONDUCTION_FLOOR_AMPS => {}
            Ok(current) => failures.push(format!(
                "{} solves but does not conduct: |I(V1)| = {current:e} A",
                model.name
            )),
            Err(error) => failures.push(format!("{} {error}", model.name)),
        }
    }

    assert!(
        failures.is_empty(),
        "{} of {probed} foundation cards failed:\n  {}",
        failures.len(),
        failures.join("\n  ")
    );
    // The fifteen unbound-device family defaults plus the Zener, which is a
    // catalog card rather than any family's fallback.
    assert_eq!(
        probed,
        FoundationDeviceFamily::ALL.len() + 1,
        "expected one probe per foundation model card"
    );
}

/// The card an unbound placement of a family falls back to must be one of the
/// cards the probe above biases into conduction. Without this the family table
/// could name a card that parses, sits in the catalog and never conducts —
/// which is exactly how the p-channel JFET card shipped broken.
#[test]
fn every_family_default_is_a_conducting_library_card() {
    let manager = LibraryManager::new();
    let content = manager
        .get_library_content("foundation.lib")
        .expect("foundation library is embedded");
    let library = Netlist::parse(content).expect("foundation library parses");

    for family in FoundationDeviceFamily::ALL {
        let name = family.default_model_name();
        let model = library
            .models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case(name))
            .unwrap_or_else(|| panic!("{family:?} names absent foundation card '{name}'"));
        let deck = probe_deck(model)
            .unwrap_or_else(|| panic!("{family:?} card '{name}' has no bias probe"));
        let current = probed_current(&deck)
            .unwrap_or_else(|error| panic!("{family:?} default '{name}' {error}"));
        assert!(
            current > CONDUCTION_FLOOR_AMPS,
            "{family:?} default '{name}' does not conduct: |I(V1)| = {current:e} A"
        );
    }
}

#[test]
fn foundation_library_is_small_and_indexed() {
    let manager = LibraryManager::new();
    let content = manager
        .get_library_content("foundation.lib")
        .expect("foundation library is embedded");
    Netlist::parse(content).expect("foundation library parses");
    assert_eq!(manager.model_count(), 16);
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
