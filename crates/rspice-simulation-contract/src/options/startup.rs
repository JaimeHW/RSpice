//! Node voltages a run starts from.
//!
//! An initial condition (`.IC`, Spectre's `ic`) holds its net at a voltage
//! while the operating point a transient starts from is solved, then lets it
//! go. A nodeset (`.NODESET`, Spectre's `nodeset`) only starts the operating
//! point's search there, so a circuit with more than one solution, a latch
//! or a bistable, settles in the state the nodesets point it to. Virtuoso
//! ADE sets both under Simulation › Convergence Aids, and ADS places an
//! InitCond or NodeSet on the net; here they are the plan's own, written
//! into the deck as one card per net.

/// One net's voltage: the net as the design names it, and its volts.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NodeVoltage {
    pub net: String,
    pub volts: f64,
}

/// Which of the two a row is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StartupKind {
    InitialCondition,
    NodeSet,
}

impl StartupKind {
    pub const ALL: [Self; 2] = [Self::InitialCondition, Self::NodeSet];

    /// The deck card it is written as.
    #[must_use]
    pub const fn card(self) -> &'static str {
        match self {
            Self::InitialCondition => ".IC",
            Self::NodeSet => ".NODESET",
        }
    }

    /// What the Solver page calls the grid.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::InitialCondition => "Initial conditions",
            Self::NodeSet => "Node sets",
        }
    }
}

/// The deck's cards for `rows`, one `.IC V(net)=volts` or
/// `.NODESET V(net)=volts` each, in the order they were set.
#[must_use]
pub fn startup_cards(kind: StartupKind, rows: &[NodeVoltage]) -> Vec<String> {
    rows.iter()
        .map(|row| format!("{} V({})={}", kind.card(), row.net, row.volts))
        .collect()
}

/// Why `rows` cannot be written into the deck as stated, each reason naming
/// the row it is about. The design's own nets are checked where the design
/// is known; this checks what any deck needs.
#[must_use]
pub fn startup_problems(kind: StartupKind, rows: &[NodeVoltage]) -> Vec<String> {
    let mut problems = Vec::new();
    for (index, row) in rows.iter().enumerate() {
        let name = row.net.as_str();
        if let Some(problem) = net_name_problem(name) {
            problems.push(format!("{} {}: {problem}", kind.label(), index + 1));
        } else if rows[..index]
            .iter()
            .any(|earlier| earlier.net.eq_ignore_ascii_case(name))
        {
            problems.push(format!("{}: {name} is set twice", kind.label()));
        }
        if !row.volts.is_finite() {
            problems.push(format!(
                "{} {name}: {} is not a voltage",
                kind.label(),
                row.volts
            ));
        }
    }
    problems
}

/// Why `name` cannot name a node on a startup card, if it cannot.
#[must_use]
pub fn net_name_problem(name: &str) -> Option<String> {
    if name.is_empty() {
        return Some("names no net".to_owned());
    }
    if is_ground(name) {
        return Some(format!(
            "{name} is ground, the reference every voltage is measured from"
        ));
    }
    if name
        .chars()
        .any(|character| character.is_whitespace() || "()=,'\"{}".contains(character))
    {
        return Some(format!("'{name}' is not a net name"));
    }
    None
}

/// `true` for the names SPICE reads as the ground node.
#[must_use]
pub fn is_ground(name: &str) -> bool {
    name == "0" || name.eq_ignore_ascii_case("gnd") || name.eq_ignore_ascii_case("gnd!")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(net: &str, volts: f64) -> NodeVoltage {
        NodeVoltage {
            net: net.to_owned(),
            volts,
        }
    }

    #[test]
    fn each_row_is_one_card_naming_its_net_and_volts() {
        assert_eq!(
            startup_cards(StartupKind::InitialCondition, &[row("out", 1.0)]),
            [".IC V(out)=1"]
        );
        assert_eq!(
            startup_cards(StartupKind::NodeSet, &[row("Q", 3.3), row("QB", 0.0)]),
            [".NODESET V(Q)=3.3", ".NODESET V(QB)=0"]
        );
    }

    #[test]
    fn a_row_a_deck_cannot_carry_is_named_with_its_reason() {
        let rows = [
            row("out", 1.0),
            row("OUT", 2.0),
            row("0", 1.0),
            row("a b", 1.0),
            row("x", f64::NAN),
        ];
        assert_eq!(
            startup_problems(StartupKind::InitialCondition, &rows),
            [
                "Initial conditions: OUT is set twice",
                "Initial conditions 3: 0 is ground, the reference every voltage is measured from",
                "Initial conditions 4: 'a b' is not a net name",
                "Initial conditions x: NaN is not a voltage",
            ]
        );
        assert!(startup_problems(StartupKind::NodeSet, &rows[..1]).is_empty());
    }
}
