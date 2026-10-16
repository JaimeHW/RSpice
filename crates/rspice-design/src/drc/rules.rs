//! Every design rule, once: its published code, what it is called, what it
//! checks, the severity it is reported at, how far a project may move it, and
//! whether one finding of it may be waived.
//!
//! A rule's code is what names it outside the program: on a finding the
//! Problems page lists, in a filter a reader types, as the key a stored rule
//! deck sets a severity against, and inside a waiver saved with a cellview.
//! Every one of those is written to disk or read by a person, so a code is a
//! contract, not a rendering of the variant's spelling. It is written out here
//! by hand, once per rule, and renaming a variant changes nothing a file holds.
//!
//! A rule whose finding means the deck cannot be built, or that the engine
//! will refuse it, has a floor of Error and cannot be waived: demoting or
//! waiving it would not let the run through, it would only move the failure
//! from the Problems page to the run. Every other rule may be set as low as a
//! note and waived one finding at a time.
//!
//! Two rules refuse the run on some findings and not others: an empty or
//! refused reference costs the deck its instance only where the netlister
//! cannot write the instance under any name. Their floor says so, rather than
//! holding every finding of the rule at the floor of the worst one: a deck
//! may set them as low as a note, and a finding on an instance the deck cannot
//! write is still reported as an error.

use super::types::{DrcSeverity, DrcViolationType};

/// One design rule, as the program publishes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DrcRule {
    /// The stable code the rule is published and stored under.
    pub code: &'static str,
    /// What the rule is called where a reader picks it out of a list.
    pub title: &'static str,
    /// What the rule checks, why it matters, and how a finding is repaired.
    pub explanation: &'static str,
    /// What a finding is reported at when the project's deck says nothing.
    ///
    /// A finding on an emitted instance the deck cannot write is reported no
    /// lower than [`DrcFloor::unwritten_instance`]; see
    /// [`DrcViolationType::severity_for_instance`].
    pub default_severity: DrcSeverity,
    /// How low the rule may be set, and how low its findings are reported.
    pub floor: DrcFloor,
    /// Whether one finding of the rule may be waived.
    pub waivable: bool,
}

/// How low a rule's findings may go.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DrcFloor {
    /// The lowest severity a project's deck may set the rule to.
    pub deck: DrcSeverity,
    /// The lowest severity a finding on an emitted instance the deck cannot
    /// write is reported at, whatever the deck sets. Never below
    /// [`Self::deck`]; above it only for a rule that refuses the run on such
    /// an instance and not on one the netlister still writes.
    pub unwritten_instance: DrcSeverity,
}

impl DrcFloor {
    /// One floor for every finding of the rule.
    const fn every(severity: DrcSeverity) -> Self {
        Self {
            deck: severity,
            unwritten_instance: severity,
        }
    }

    /// Whether some finding of the rule refuses the run whatever the deck
    /// sets.
    pub fn refuses_the_run(self) -> bool {
        self.unwritten_instance >= DrcSeverity::Error
    }
}

const ENGINE_REFUSES: DrcFloor = DrcFloor::every(DrcSeverity::Error);
const ANY: DrcFloor = DrcFloor::every(DrcSeverity::Info);
/// The deck is built and runs, with a value nobody entered in place of the
/// one the finding is about: never lower than a warning.
const INVENTED_VALUE: DrcFloor = DrcFloor::every(DrcSeverity::Warning);
/// A note on an instance the netlister still writes; the run is refused on
/// one it cannot write under any name.
const UNWRITTEN_REFUSES: DrcFloor = DrcFloor {
    deck: DrcSeverity::Info,
    unwritten_instance: DrcSeverity::Error,
};

impl DrcViolationType {
    /// This rule's entry. Exhaustive, so a rule cannot be added without one.
    pub const fn rule(self) -> &'static DrcRule {
        match self {
            Self::FloatingNode => &DrcRule {
                code: "DRC-FLOATING-NODE",
                title: "Floating node",
                explanation: "A net reaches only one terminal. Nothing else drives or loads it, \
                    so that terminal sits at whatever potential the rest of its device leaves it \
                    at; it is almost always a wire stopped short of the pin it was drawn toward. \
                    It never stops the solver — a dangling resistor leg follows its other end and \
                    a dangling capacitor plate is settled by the conditioning shunt — so it is a \
                    warning. Connect the net to the terminal it was meant for, or delete it.",
                default_severity: DrcSeverity::Warning,
                floor: ANY,
                waivable: true,
            },
            Self::UnconnectedPin => &DrcRule {
                code: "DRC-UNCONNECTED-PIN",
                title: "Unconnected pin",
                explanation: "A terminal no conductor reaches, on a pin no interface gives a \
                    direction: a passive leg, a primitive device terminal, or a pin declared \
                    inout. The deck gives it a node of its own. Connect it, or waive the finding \
                    where the terminal is meant to be left open.",
                default_severity: DrcSeverity::Warning,
                floor: ANY,
                waivable: true,
            },
            Self::UndrivenInput => &DrcRule {
                code: "DRC-UNDRIVEN-INPUT",
                title: "Undriven input",
                explanation: "A pin the bound cell's interface declares an input or a supply is \
                    reached by no conductor, so nothing drives it or feeds its rail. The interface \
                    said what the pin is for, which makes this a stronger claim than an open \
                    passive leg. Connect a driver, or declare the pin bidirectional if it may be \
                    left open.",
                default_severity: DrcSeverity::Error,
                floor: ANY,
                waivable: true,
            },
            Self::UnloadedOutput => &DrcRule {
                code: "DRC-UNLOADED-OUTPUT",
                title: "Unloaded output",
                explanation: "A pin the bound cell's interface declares an output, or an \
                    independent voltage source's positive terminal, drives nothing. The circuit \
                    still solves; the design just loses the signal it drew. Connect the output, \
                    or waive the finding where it is meant to be open.",
                default_severity: DrcSeverity::Warning,
                floor: ANY,
                waivable: true,
            },
            Self::OrphanNetLabel => &DrcRule {
                code: "DRC-ORPHAN-NET-LABEL",
                title: "Orphan net label",
                explanation: "A net label names a net no conductor carries, so the name the \
                    drawing shows reaches nothing in the deck. Attach the label to the wire it \
                    names, or remove it.",
                default_severity: DrcSeverity::Warning,
                floor: ANY,
                waivable: true,
            },
            Self::DanglingWire => &DrcRule {
                code: "DRC-DANGLING-WIRE",
                title: "Dangling wire",
                explanation: "A wire segment ends on nothing: no terminal, junction, label or \
                    other wire. It adds no connection to the deck, which usually means the one \
                    it was drawn for is missing. Complete the connection or delete the segment.",
                default_severity: DrcSeverity::Warning,
                floor: ANY,
                waivable: true,
            },
            Self::MalformedBus => &DrcRule {
                code: "DRC-MALFORMED-BUS",
                title: "Malformed bus",
                explanation: "A bus's geometry or its declaration cannot be read as a set of \
                    conductors. The netlister cannot expand it, so the deck is not built. Redraw \
                    the bus or correct its declaration.",
                default_severity: DrcSeverity::Error,
                floor: ENGINE_REFUSES,
                waivable: false,
            },
            Self::UnnamedBus => &DrcRule {
                code: "DRC-UNNAMED-BUS",
                title: "Unnamed bus",
                explanation: "A bus carries no typed declaration, so nothing says how many \
                    conductors it holds or what they are called. The deck is built from what it \
                    touches, but a reader cannot tell what the bus means. Give it a name and a \
                    range.",
                default_severity: DrcSeverity::Warning,
                floor: ANY,
                waivable: true,
            },
            Self::BusRangeConflict => &DrcRule {
                code: "DRC-BUS-RANGE-CONFLICT",
                title: "Bus range conflict",
                explanation: "Connected bus declarations, or a tap's selector, name different \
                    conductors. The netlister cannot decide which conductor is which, so the \
                    deck is not built. Make the connected declarations and the tap ranges agree.",
                default_severity: DrcSeverity::Error,
                floor: ENGINE_REFUSES,
                waivable: false,
            },
            Self::DanglingBusTap => &DrcRule {
                code: "DRC-DANGLING-BUS-TAP",
                title: "Dangling bus tap",
                explanation: "A bus tap does not end on the kind of target it requires — a \
                    scalar wire or a typed bus. The netlister refuses the tap, so the deck is not \
                    built. Connect the tap to its target.",
                default_severity: DrcSeverity::Error,
                floor: ENGINE_REFUSES,
                waivable: false,
            },
            Self::MixedBusTap => &DrcRule {
                code: "DRC-MIXED-BUS-TAP",
                title: "Mixed bus tap",
                explanation: "A bus tap joins scalar and vector connectivity in one place, so a \
                    conductor would have to be both one wire and one member of a bus. The \
                    netlister refuses it, so the deck is not built. Separate the scalar wires \
                    from the bus.",
                default_severity: DrcSeverity::Error,
                floor: ENGINE_REFUSES,
                waivable: false,
            },
            Self::DuplicateBusMemberDriver => &DrcRule {
                code: "DRC-DUPLICATE-BUS-MEMBER-DRIVER",
                title: "Duplicate bus member driver",
                explanation: "More than one declared output drives the same member of a typed \
                    bus. The deck is built and runs, but the outputs fight over the conductor and \
                    its value is whichever wins. Leave one driver on the member.",
                default_severity: DrcSeverity::Error,
                floor: ANY,
                waivable: true,
            },
            Self::VectorWidthMismatch => &DrcRule {
                code: "DRC-VECTOR-WIDTH-MISMATCH",
                title: "Vector width mismatch",
                explanation: "A vector port, pin or instance terminal meets a bus that declares a \
                    different set of conductors. Emitting the join would mean inventing or \
                    dropping conductors the drawing never named, so the netlister refuses it \
                    under either width policy. Declare the same range on both ends, or tap an \
                    explicit slice of the bus.",
                default_severity: DrcSeverity::Error,
                floor: ENGINE_REFUSES,
                waivable: false,
            },
            Self::OffSheetConnectorWithoutPartner => &DrcRule {
                code: "DRC-OFF-SHEET-CONNECTOR-WITHOUT-PARTNER",
                title: "Off-sheet connector without partner",
                explanation: "An off-sheet connector's name appears on only one sheet, so it \
                    connects to nothing a reader would follow it to. The net is still the net \
                    its name makes it. Place the matching connector on the other sheet, or make \
                    this an ordinary net label.",
                default_severity: DrcSeverity::Info,
                floor: ANY,
                waivable: true,
            },
            Self::ShortedOutputs => &DrcRule {
                code: "DRC-SHORTED-OUTPUTS",
                title: "Shorted outputs",
                explanation: "Two or more pins their interfaces declare outputs share one net. \
                    The deck is built and runs, but the outputs drive against each other. Leave \
                    one declared output on the net.",
                default_severity: DrcSeverity::Error,
                floor: ANY,
                waivable: true,
            },
            Self::MissingGround => &DrcRule {
                code: "DRC-MISSING-GROUND",
                title: "Missing ground",
                explanation: "A design simulated on its own reaches no ground symbol anywhere in \
                    its hierarchy, so no node has a DC path to the reference node and the engine \
                    refuses the run. A cell placed by another takes its reference through its \
                    ports and is not asked. Add a ground symbol.",
                default_severity: DrcSeverity::Critical,
                floor: ENGINE_REFUSES,
                waivable: false,
            },
            Self::SourceToSource => &DrcRule {
                code: "DRC-SOURCE-TO-SOURCE",
                title: "Source to source",
                explanation: "Independent voltage sources close a loop among themselves, which \
                    leaves the loop's current undetermined: the matrix is singular and the \
                    engine refuses the run. Put a series element in the loop, or remove one of \
                    its sources.",
                default_severity: DrcSeverity::Error,
                floor: ENGINE_REFUSES,
                waivable: false,
            },
            Self::RecursiveInstantiation => &DrcRule {
                code: "DRC-RECURSIVE-INSTANTIATION",
                title: "Recursive instantiation",
                explanation: "A cellview reaches itself through its own instances, directly or \
                    through other cellviews. The design has no finite expansion, so there is no \
                    deck to run and nothing below the repeated occurrence is checked. Remove the \
                    instance that reaches its own cellview, or bind it to another view.",
                default_severity: DrcSeverity::Critical,
                floor: ENGINE_REFUSES,
                waivable: false,
            },
            Self::DuplicateName => &DrcRule {
                code: "DRC-DUPLICATE-NAME",
                title: "Duplicate name",
                explanation: "Two cards the deck writes carry one name. The engine rejects a \
                    duplicate element name, so the run is refused. Rename one of the components.",
                default_severity: DrcSeverity::Critical,
                floor: ENGINE_REFUSES,
                waivable: false,
            },
            Self::EmptyName => &DrcRule {
                code: "DRC-EMPTY-NAME",
                title: "Empty name",
                explanation: "An emitted instance has no reference designator. Where the \
                    netlister writes it under a name it chose, the run starts and its results \
                    carry that name, so it is a warning a deck may lower. Where the deck cannot \
                    carry the instance at all, the run is refused, and the finding is an error \
                    whatever the deck sets. An object that writes no card is not checked. Assign \
                    a reference designator.",
                default_severity: DrcSeverity::Warning,
                floor: UNWRITTEN_REFUSES,
                waivable: false,
            },
            Self::InvalidName => &DrcRule {
                code: "DRC-INVALID-NAME",
                title: "Invalid name",
                explanation: "An emitted instance's reference designator is one the deck cannot \
                    carry as written. Where the netlister writes the instance under a name it \
                    chose, it is a warning a deck may lower; where the instance cannot be written \
                    at all, the run is refused and the finding is an error whatever the deck \
                    sets. An object that writes no card is not checked. Use letters, digits and \
                    underscores.",
                default_severity: DrcSeverity::Warning,
                floor: UNWRITTEN_REFUSES,
                waivable: false,
            },
            Self::CaseCollidingNetNames => &DrcRule {
                code: "DRC-CASE-COLLIDING-NET-NAMES",
                title: "Case-colliding net names",
                explanation: "Two authored net names differ only by case. The deck is \
                    case-insensitive and folds them into one node, so the drawing shows two nets \
                    the simulation has as one. Spell both names the same way, or rename one of \
                    the two nets.",
                default_severity: DrcSeverity::Error,
                floor: ANY,
                waivable: true,
            },
            Self::MissingParameter => &DrcRule {
                code: "DRC-MISSING-PARAMETER",
                title: "Missing parameter",
                explanation: "A component leaves a required parameter empty. The netlister still \
                    writes the card, with a value nobody entered in its place, so the run \
                    simulates a circuit that is not the drawing. A deck may lower it to a \
                    warning and no further, and a finding cannot be waived. Set the parameter.",
                default_severity: DrcSeverity::Error,
                floor: INVENTED_VALUE,
                waivable: false,
            },
            Self::ValueOutOfRange => &DrcRule {
                code: "DRC-VALUE-OUT-OF-RANGE",
                title: "Value out of range",
                explanation: "A parameter's value lies outside the range its schema declares. \
                    The deck is built and runs with it. Check the value; waive the finding where \
                    the out-of-range value is deliberate.",
                default_severity: DrcSeverity::Warning,
                floor: ANY,
                waivable: true,
            },
            Self::UnknownComponent => &DrcRule {
                code: "DRC-UNKNOWN-COMPONENT",
                title: "Unknown component",
                explanation: "An emitted cell instance's master resolves to nothing the \
                    netlister can write, so the deck cannot carry the instance and the run is \
                    refused. It is only asked where the hierarchy can answer, and an object that \
                    writes no card is not checked. Check the component type, or define its \
                    model.",
                default_severity: DrcSeverity::Error,
                floor: ENGINE_REFUSES,
                waivable: false,
            },
            Self::UnresolvedReference => &DrcRule {
                code: "DRC-UNRESOLVED-REFERENCE",
                title: "Unresolved reference",
                explanation: "An instance names another instance — a current-controlled source \
                    its controlling element — by a reference that names no single instance of \
                    its cellview. The deck writes it as an element the engine does not have, \
                    and the run fails at parse. Name exactly one instance.",
                default_severity: DrcSeverity::Error,
                floor: ENGINE_REFUSES,
                waivable: false,
            },
            Self::SymbolUnplacedPin => &DrcRule {
                code: "SYMBOL-UNPLACED-PIN",
                title: "Unplaced symbol pin",
                explanation: "A pin the cell's interface requires has no terminal on the symbol. \
                    Every instance placed from the symbol then draws fewer terminals than the \
                    interface declares, and the netlister refuses it. Place the pin on the \
                    terminal grid.",
                default_severity: DrcSeverity::Error,
                floor: ENGINE_REFUSES,
                waivable: false,
            },
            Self::SymbolOrphanedPin => &DrcRule {
                code: "SYMBOL-ORPHANED-PIN",
                title: "Orphaned symbol pin",
                explanation: "The symbol carries a pin the cell's schematic no longer declares. \
                    Every instance placed from it draws more terminals than the interface \
                    declares, and the netlister refuses it. Delete the pin, or add the port back \
                    to the schematic.",
                default_severity: DrcSeverity::Error,
                floor: ENGINE_REFUSES,
                waivable: false,
            },
            Self::SymbolPinOffGrid => &DrcRule {
                code: "SYMBOL-PIN-OFF-GRID",
                title: "Symbol pin off grid",
                explanation: "A symbol pin's terminal is off the terminal grid, so a wire drawn \
                    on the grid cannot end on it and the pin is left open wherever the symbol \
                    is placed. Move the pin onto the grid.",
                default_severity: DrcSeverity::Error,
                floor: ANY,
                waivable: true,
            },
        }
    }

    /// The stable code this rule is published under.
    ///
    /// A schematic rule is in the `DRC-` family; a symbol's pin contract is not
    /// one of a schematic's electrical rules, and its three codes say so.
    pub const fn rule_code(self) -> &'static str {
        self.rule().code
    }

    /// The rule `code` names, or `None` where this build has no such rule.
    pub fn from_rule_code(code: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|rule| rule.rule_code() == code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The code each rule is published under, written out a second time.
    ///
    /// The table above is what the program reads; this is what files already
    /// hold. Exhaustive, so a new rule does not compile until it is given a
    /// code here too, and an edit to a code in the table fails here rather
    /// than silently orphaning every deck row and waiver written under it.
    fn published_code(rule: DrcViolationType) -> &'static str {
        match rule {
            DrcViolationType::FloatingNode => "DRC-FLOATING-NODE",
            DrcViolationType::UnconnectedPin => "DRC-UNCONNECTED-PIN",
            DrcViolationType::UndrivenInput => "DRC-UNDRIVEN-INPUT",
            DrcViolationType::UnloadedOutput => "DRC-UNLOADED-OUTPUT",
            DrcViolationType::OrphanNetLabel => "DRC-ORPHAN-NET-LABEL",
            DrcViolationType::DanglingWire => "DRC-DANGLING-WIRE",
            DrcViolationType::MalformedBus => "DRC-MALFORMED-BUS",
            DrcViolationType::UnnamedBus => "DRC-UNNAMED-BUS",
            DrcViolationType::BusRangeConflict => "DRC-BUS-RANGE-CONFLICT",
            DrcViolationType::DanglingBusTap => "DRC-DANGLING-BUS-TAP",
            DrcViolationType::MixedBusTap => "DRC-MIXED-BUS-TAP",
            DrcViolationType::DuplicateBusMemberDriver => "DRC-DUPLICATE-BUS-MEMBER-DRIVER",
            DrcViolationType::VectorWidthMismatch => "DRC-VECTOR-WIDTH-MISMATCH",
            DrcViolationType::OffSheetConnectorWithoutPartner => {
                "DRC-OFF-SHEET-CONNECTOR-WITHOUT-PARTNER"
            }
            DrcViolationType::ShortedOutputs => "DRC-SHORTED-OUTPUTS",
            DrcViolationType::MissingGround => "DRC-MISSING-GROUND",
            DrcViolationType::SourceToSource => "DRC-SOURCE-TO-SOURCE",
            DrcViolationType::RecursiveInstantiation => "DRC-RECURSIVE-INSTANTIATION",
            DrcViolationType::DuplicateName => "DRC-DUPLICATE-NAME",
            DrcViolationType::EmptyName => "DRC-EMPTY-NAME",
            DrcViolationType::InvalidName => "DRC-INVALID-NAME",
            DrcViolationType::CaseCollidingNetNames => "DRC-CASE-COLLIDING-NET-NAMES",
            DrcViolationType::MissingParameter => "DRC-MISSING-PARAMETER",
            DrcViolationType::ValueOutOfRange => "DRC-VALUE-OUT-OF-RANGE",
            DrcViolationType::UnknownComponent => "DRC-UNKNOWN-COMPONENT",
            DrcViolationType::UnresolvedReference => "DRC-UNRESOLVED-REFERENCE",
            DrcViolationType::SymbolUnplacedPin => "SYMBOL-UNPLACED-PIN",
            DrcViolationType::SymbolOrphanedPin => "SYMBOL-ORPHANED-PIN",
            DrcViolationType::SymbolPinOffGrid => "SYMBOL-PIN-OFF-GRID",
        }
    }

    #[test]
    fn every_rule_keeps_the_code_files_were_written_under() {
        for rule in DrcViolationType::ALL {
            assert_eq!(rule.rule_code(), published_code(rule), "{rule:?}");
        }
    }

    #[test]
    fn every_code_is_distinct_and_names_its_rule_back() {
        let mut codes = std::collections::BTreeSet::new();
        for rule in DrcViolationType::ALL {
            let code = rule.rule_code();
            assert!(codes.insert(code), "two rules share the code {code}");
            assert_eq!(DrcViolationType::from_rule_code(code), Some(rule));
        }
        assert_eq!(DrcViolationType::from_rule_code("DRC-NOT-A-RULE"), None);
        // A code is matched exactly: a reader's spelling is not an identity.
        assert_eq!(DrcViolationType::from_rule_code("drc-floating-node"), None);
    }

    /// `ALL` lists every variant the enum has.
    ///
    /// A fixed-length array does not prove that: a variant added to the enum
    /// and not to the array compiles, and the merge that brought in
    /// `UnresolvedReference` did exactly that. Serde's derive does know every
    /// variant, and names them all when it refuses one it does not know.
    #[test]
    fn the_rule_listing_names_every_variant() {
        let error = serde_json::from_str::<DrcViolationType>("\"NotARule\"")
            .expect_err("an unknown variant is refused")
            .to_string();
        let (_, expected) = error
            .split_once("expected one of ")
            .expect("serde names the variants it expected");
        let declared = expected
            .split(", ")
            .map(|name| name.trim_matches(|c: char| c == '`' || c.is_whitespace()))
            .map(|name| name.split('`').next().unwrap_or(name))
            .collect::<Vec<_>>();
        let listed = DrcViolationType::ALL
            .iter()
            .map(|rule| format!("{rule:?}"))
            .collect::<Vec<_>>();
        assert_eq!(declared, listed);
    }

    /// Every rule says what it is, no rule's default is below what the
    /// project may set it to, and no floor for an unwritten instance is below
    /// the rule's own.
    #[test]
    fn every_entry_is_complete_and_consistent() {
        for rule in DrcViolationType::ALL {
            let entry = rule.rule();
            assert!(!entry.title.is_empty(), "{rule:?}");
            assert!(entry.explanation.len() > 80, "{rule:?}");
            assert!(!entry.explanation.contains("  "), "{rule:?}");
            assert!(entry.default_severity >= entry.floor.deck, "{rule:?}");
            assert!(
                entry.floor.unwritten_instance >= entry.floor.deck,
                "{rule:?}"
            );
        }
    }

    /// A rule some finding of which refuses the run whatever the deck sets
    /// may not be waived: a waiver would not let the run through, it would
    /// only move the failure from the Problems page to the run. Every
    /// waivable rule may also be set as low as a note. `MissingParameter` is
    /// the one rule that is neither demotable to a note nor waivable, because
    /// the deck it builds is not the drawing.
    #[test]
    fn a_rule_that_refuses_the_run_can_neither_be_demoted_nor_waived() {
        for rule in DrcViolationType::ALL {
            let entry = rule.rule();
            if entry.floor.refuses_the_run() {
                assert!(!entry.waivable, "{rule:?}");
            }
            if entry.waivable {
                assert_eq!(entry.floor, ANY, "{rule:?}");
            }
        }
        for rule in [
            DrcViolationType::DuplicateName,
            DrcViolationType::UnresolvedReference,
            DrcViolationType::UnknownComponent,
            DrcViolationType::RecursiveInstantiation,
            DrcViolationType::MissingGround,
            DrcViolationType::SourceToSource,
            DrcViolationType::VectorWidthMismatch,
        ] {
            assert_eq!(rule.rule().floor, ENGINE_REFUSES, "{rule:?}");
        }
        assert_eq!(
            DrcViolationType::MissingParameter.rule().floor.deck,
            DrcSeverity::Warning
        );
    }

    /// An empty or refused reference refuses the run only where the instance
    /// cannot be written. The deck may lower what the netlister writes under a
    /// name it chose, and nothing else.
    #[test]
    fn a_reference_finding_is_held_at_an_error_only_on_an_unwritten_instance() {
        for rule in [DrcViolationType::EmptyName, DrcViolationType::InvalidName] {
            assert_eq!(rule.rule().floor, UNWRITTEN_REFUSES, "{rule:?}");
            assert_eq!(rule.floor_for_instance(true), DrcSeverity::Info);
            assert_eq!(rule.floor_for_instance(false), DrcSeverity::Error);
            assert_eq!(rule.severity_for_instance(true), DrcSeverity::Warning);
            assert_eq!(rule.severity_for_instance(false), DrcSeverity::Error);
        }
        // A master that resolves to nothing is never written.
        let unknown = DrcViolationType::UnknownComponent;
        assert_eq!(unknown.severity_for_instance(false), DrcSeverity::Error);
        assert_eq!(unknown.floor_for_instance(false), DrcSeverity::Error);
    }
}
