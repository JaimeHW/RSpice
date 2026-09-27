//! Ground policy integration with the simulation engine.

#[cfg(test)]
use rspice_design::schematic::ground_names::GROUND_ALIASES;
pub use rspice_design::schematic::ground_names::{is_ground_reference, reserved_ground_name};

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::netlist::GroundPolicy;

    const POLICIES: [GroundPolicy; 3] = [
        GroundPolicy::OnlyZero,
        GroundPolicy::NgspiceGnd,
        GroundPolicy::XyceReplace,
    ];

    #[test]
    fn ground_aliases_cover_every_engine_policy() {
        // The reserved spellings, their case and whitespace variants, and the
        // near misses that must stay ordinary supply rails.
        const CORPUS: [&str; 14] = [
            "0", "GND", "gnd", " Gnd ", "GND!", "gnd!", "GROUND", "ground", "GNDA", "AGND", "VSS",
            "VEE", "vdd!", "out",
        ];

        for name in CORPUS {
            let engine_grounds = POLICIES.iter().any(|policy| policy.is_ground(name));
            assert_eq!(
                is_ground_reference(name),
                engine_grounds,
                "`{name}` must be ground here exactly when some engine policy grounds it"
            );
        }

        for alias in GROUND_ALIASES {
            assert!(
                POLICIES
                    .iter()
                    .any(|policy| policy.canonical_node(alias) == "0"),
                "`{alias}` is reserved here but no engine policy folds it onto node 0"
            );
        }
    }

    #[test]
    fn a_reserved_name_states_why_and_a_supply_rail_is_free() {
        assert!(reserved_ground_name("0").is_some_and(|reason| reason.contains("reference node")));
        assert!(reserved_ground_name("Gnd").is_some_and(|reason| reason.contains("ground alias")));
        assert!(reserved_ground_name("vdd!").is_some_and(|reason| reason.contains("global net")));
        for free in ["GNDA", "AGND", "VSS", "VEE", "OUT"] {
            assert_eq!(reserved_ground_name(free), None, "`{free}` is a supply pin");
        }
    }
}
