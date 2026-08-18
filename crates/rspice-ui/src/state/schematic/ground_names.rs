//! Which names denote the simulation reference node.
//!
//! One authority for the whole editor. The alias set is the union over every
//! `rspice_core::netlist::GroundPolicy` variant, because the deck a schematic
//! produces may be executed under any of them: a name that some dialect folds
//! onto node `0` has to read as ground here too, or the canvas and the engine
//! disagree about how many nodes the design has. The drift between those two
//! authorities is what `ground_aliases_cover_every_engine_policy` forbids.

/// Every spelling some engine dialect folds onto node `0`.
const GROUND_ALIASES: [&str; 4] = ["0", "GND", "GND!", "GROUND"];

/// `true` when `name` denotes the reference node under any dialect the engine
/// can be asked to run.
///
/// `VSS`, `VEE`, `AGND` and friends are ordinary supply rails: no dialect ties
/// them to node `0`, so neither does this.
pub fn is_ground_reference(name: &str) -> bool {
    let name = name.trim();
    GROUND_ALIASES
        .iter()
        .any(|alias| name.eq_ignore_ascii_case(alias))
}

/// Why `name` cannot be an authored interface-port name, or `None` when it can.
///
/// A cell formal named `0` is illegal in every SPICE dialect, one named `GND`
/// silently shorts the pin to global ground under ngspice, and a name ending
/// in `!` declares a global net — which is by definition not a pin of one
/// cell. The returned reason is the second half of the sentence the caller
/// prints, so the port and the reason are named together.
pub fn reserved_ground_name(name: &str) -> Option<&'static str> {
    let name = name.trim();
    if name.eq_ignore_ascii_case("0") {
        return Some(
            "the simulation reference node; interface ports cannot be ground — connect the \
             ground symbol instead",
        );
    }
    if is_ground_reference(name) {
        return Some(
            "a ground alias; interface ports cannot be ground — connect the ground symbol \
             instead",
        );
    }
    name.ends_with('!').then_some(
        "a global net; a global net is not a cell formal — drop the `!` or wire the port to the \
         global net instead",
    )
}

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
