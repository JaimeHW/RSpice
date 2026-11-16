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

/// One net's voltage: the net as the design names it, the drawn address
/// such as `/X1/mid` for a net inside an instance, and its value as written: volts
/// in SPICE notation, or an expression of the plan's design variables, which
/// a corner or a sweep then moves with them.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NodeVoltage {
    pub net: String,
    pub value: String,
}

impl NodeVoltage {
    /// The net as the deck names it: a net of the root as drawn, a net
    /// inside an instance by the flattened name the engine solves, `/amp/mid`
    /// as `xamp.mid`.
    #[must_use]
    pub fn deck_net(&self) -> String {
        if self.net.starts_with('/')
            && let Ok(name) = rspice_vocabulary::hierarchy_path::ProbeTarget::parse(&self.net)
                .and_then(|target| target.engine_name())
        {
            return name;
        }
        self.net.clone()
    }

    /// The value as a number, when it is one rather than an expression.
    #[must_use]
    pub fn volts(&self) -> Option<f64> {
        super::parse_si_value(self.value.trim()).ok()
    }

    /// The value as the deck reads it: a number exactly, an expression in
    /// braces so the deck evaluates it against its parameters.
    #[must_use]
    pub fn deck_value(&self) -> String {
        match self.volts() {
            Some(volts) => volts.to_string(),
            None => format!("{{{}}}", self.value.trim()),
        }
    }
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

/// The deck's cards for `rows`, one `.IC V(net)=value` or
/// `.NODESET V(net)=value` each, in the order they were set.
#[must_use]
pub fn startup_cards(kind: StartupKind, rows: &[NodeVoltage]) -> Vec<String> {
    rows.iter()
        .map(|row| format!("{} V({})={}", kind.card(), row.deck_net(), row.deck_value()))
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
        if let Some(problem) = value_problem(&row.value) {
            problems.push(format!("{} {name}: {problem}", kind.label()));
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

/// Why `value` cannot stand on a startup card, if it cannot: nothing
/// written, a number that is no voltage, or text that would break the card.
/// Whether an expression names the plan's design variables is checked where
/// the plan is known.
#[must_use]
pub fn value_problem(value: &str) -> Option<String> {
    let value = value.trim();
    if value.is_empty() {
        return Some("has no value".to_owned());
    }
    match super::parse_si_value(value) {
        Ok(volts) if !volts.is_finite() => Some(format!("{value} is not a voltage")),
        Ok(_) => None,
        Err(_)
            if value
                .chars()
                .any(|character| "{}'\"\n\r".contains(character)) =>
        {
            Some(format!("'{value}' is not a value"))
        }
        Err(_) => None,
    }
}

/// `true` for the names SPICE reads as the ground node.
#[must_use]
pub fn is_ground(name: &str) -> bool {
    name == "0" || name.eq_ignore_ascii_case("gnd") || name.eq_ignore_ascii_case("gnd!")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(net: &str, value: &str) -> NodeVoltage {
        NodeVoltage {
            net: net.to_owned(),
            value: value.to_owned(),
        }
    }

    #[test]
    fn each_row_is_one_card_naming_its_net_and_volts() {
        assert_eq!(
            startup_cards(
                StartupKind::InitialCondition,
                &[row("out", "250m"), row("/amp/mid", "vdd/2")]
            ),
            [".IC V(out)=0.25", ".IC V(xamp.mid)={vdd/2}"]
        );
        assert_eq!(
            startup_cards(StartupKind::NodeSet, &[row("Q", "3.3"), row("QB", "0")]),
            [".NODESET V(Q)=3.3", ".NODESET V(QB)=0"]
        );
    }

    #[test]
    fn a_row_a_deck_cannot_carry_is_named_with_its_reason() {
        let rows = [
            row("out", "1"),
            row("OUT", "2"),
            row("0", "1"),
            row("a b", "1"),
            row("x", ""),
            row("y", "{vdd}"),
        ];
        assert_eq!(
            startup_problems(StartupKind::InitialCondition, &rows),
            [
                "Initial conditions: OUT is set twice",
                "Initial conditions 3: 0 is ground, the reference every voltage is measured from",
                "Initial conditions 4: 'a b' is not a net name",
                "Initial conditions x: has no value",
                "Initial conditions y: '{vdd}' is not a value",
            ]
        );
        assert!(startup_problems(StartupKind::NodeSet, &rows[..1]).is_empty());
    }
}
