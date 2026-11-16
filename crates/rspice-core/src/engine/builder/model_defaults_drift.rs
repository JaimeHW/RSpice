//! The new-model defaults table against the engine's own defaults: for every
//! device type and every parameter the table writes, the card with that
//! parameter at its table default builds the same device as the bare card.

use crate::library::{DEVICE_MODEL_DEFAULTS, DeviceModelDefaults};

/// A deck placing one device of `defaults`' type on model `m`, with `card`.
fn deck(defaults: &DeviceModelDefaults, card: &str) -> String {
    let element = match defaults.spice_type {
        "D" => "D1 a 0 m",
        "NPN" | "PNP" => "Q1 a b 0 m",
        "NMOS" | "PMOS" => "M1 a b 0 0 m L=2u W=10u",
        "NJF" | "PJF" => "J1 a b 0 m",
        "NMF" | "PMF" => "Z1 a b 0 m",
        "VDMOS" => "M1 a b 0 m",
        "R" => "R1 a b 1k m",
        "C" => "C1 a b 1p m",
        "L" => "L1 a b 1u m",
        "SW" => "S1 a 0 b 0 m",
        "CSW" => "W1 a 0 VB m",
        other => panic!("no element places a {other} model"),
    };
    format!("defaults\nVA a 0 DC 1\nVB b 0 DC 0.5\nRB b 0 1k\n{element}\n{card}\n.end\n")
}

/// Every device the deck builds, as the engine holds it.
fn devices(deck: &str) -> String {
    let netlist = crate::Netlist::parse(deck).unwrap_or_else(|error| panic!("{error}\n{deck}"));
    // Away from the nominal temperature, so a temperature coefficient is
    // seen in the device it scales.
    let config = crate::SimulationConfig {
        temperature: 348.15,
        ..crate::SimulationConfig::default()
    };
    let circuit = crate::Engine::new(config)
        .build_circuit(&netlist)
        .unwrap_or_else(|error| panic!("{error}\n{deck}"));
    format!(
        "{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}",
        circuit.resistors,
        circuit.capacitors,
        circuit.inductors,
        circuit.diodes,
        circuit.bjts,
        circuit.mosfets,
        circuit.vdmoses,
        circuit.jfets,
        circuit.vswitches,
        circuit.iswitches,
    )
}

#[test]
fn every_new_model_default_is_the_engines_own() {
    let mut drifted = Vec::new();
    for defaults in DEVICE_MODEL_DEFAULTS {
        let bare = devices(&deck(defaults, &defaults.bare_card("m")));
        assert_eq!(
            bare,
            devices(&deck(defaults, &defaults.bare_card("m"))),
            "{}: a deck builds the same devices twice",
            defaults.spice_type
        );
        for parameter in defaults.parameters {
            let card = defaults.card_with("m", [parameter]);
            if devices(&deck(defaults, &card)) != bare {
                drifted.push(format!(
                    "{} {}={}",
                    defaults.spice_type, parameter.name, parameter.default
                ));
            }
            // A parameter the comparison cannot see would pass whatever its
            // default: another value of it must build another device.
            let mut moved = *parameter;
            moved.default = if parameter.default == 0.0 {
                // A length stays short of the channel it is cut from.
                if parameter.unit == "m" { 1e-7 } else { 0.25 }
            } else {
                parameter.default * 1.5
            };
            if devices(&deck(defaults, &defaults.card_with("m", [&moved]))) == bare {
                drifted.push(format!(
                    "{} {}: no value of it changes the device the comparison sees",
                    defaults.spice_type, parameter.name
                ));
            }
        }
        let full = defaults.card_with("m", defaults.parameters);
        if devices(&deck(defaults, &full)) != bare {
            drifted.push(format!("{}: the whole card", defaults.spice_type));
        }
    }
    assert!(
        drifted.is_empty(),
        "these defaults build a different device than the bare card:\n{}",
        drifted.join("\n")
    );
}
