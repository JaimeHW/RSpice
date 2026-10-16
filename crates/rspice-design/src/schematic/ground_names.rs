//! Which names denote the simulation reference node.
//!
//! Two questions with two answers. [`is_ground_reference`] says what reads as
//! ground on the canvas: the union over every `rspice_core::netlist::GroundPolicy`
//! variant, because a deck may be executed under any of them, so a name that
//! some dialect folds onto node `0` has to read as ground here too. The drift
//! between those two authorities is what `ground_aliases_cover_every_engine_policy`
//! forbids. [`writes_node_zero`] says which net the deck the netlister writes
//! actually puts on node `0`, and that is what every electrical judgement of
//! the drawing asks.

/// Every spelling some engine dialect folds onto node `0`.
pub const GROUND_ALIASES: [&str; 4] = ["0", "GND", "GND!", "GROUND"];

/// The net names the netlister writes as node `0`.
///
/// Narrower than [`GROUND_ALIASES`]: `GND!` folds onto node `0` only under
/// Xyce's `.PREPROCESS REPLACEGROUND`, which no deck the netlister writes asks
/// for, so a net named `GND!` is written as a global node of that name.
const DECK_NODE_ZERO_NAMES: [&str; 3] = ["0", "GND", "GROUND"];

/// `true` when the deck writes a net carrying this name as node `0`.
///
/// `name` is the net's name after the project's connectivity contract has
/// promoted it — the name every extracted net and terminal carries. That
/// promotion is the one alias setting the netlister honours: a technology
/// global-net catalog that lists `GND` makes an authored `GND!` arrive here as
/// `GND`, and node `0`, exactly as it reaches the deck. A drawn ground symbol
/// names its net `0`.
///
/// The netlister names every node through this, and every rule that reasons
/// about the reference node asks it, so a check cannot ground a net the deck
/// writes as an ordinary node, or the reverse.
pub fn writes_node_zero(name: &str) -> bool {
    DECK_NODE_ZERO_NAMES
        .iter()
        .any(|zero| name.eq_ignore_ascii_case(zero))
}

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
