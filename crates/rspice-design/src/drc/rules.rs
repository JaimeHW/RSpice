//! The published identity of every design rule.
//!
//! A rule's code is what names it outside the program: on a finding the
//! Problems page lists, in a filter a reader types, as the key a stored rule
//! deck sets a severity against, and inside a waiver saved with a cellview.
//! Every one of those is written to disk or read by a person, so a code is a
//! contract, not a rendering of the variant's spelling. It is written out here
//! by hand, once per rule, and renaming a variant changes nothing a file holds.

use super::types::DrcViolationType;

impl DrcViolationType {
    /// The stable code this rule is published under.
    ///
    /// Exhaustive, so a rule cannot be added without one. A schematic rule is
    /// in the `DRC-` family; a symbol's pin contract is not one of a
    /// schematic's electrical rules, and its three codes say so.
    pub const fn rule_code(self) -> &'static str {
        match self {
            Self::FloatingNode => "DRC-FLOATING-NODE",
            Self::UnconnectedPin => "DRC-UNCONNECTED-PIN",
            Self::UndrivenInput => "DRC-UNDRIVEN-INPUT",
            Self::UnloadedOutput => "DRC-UNLOADED-OUTPUT",
            Self::OrphanNetLabel => "DRC-ORPHAN-NET-LABEL",
            Self::DanglingWire => "DRC-DANGLING-WIRE",
            Self::MalformedBus => "DRC-MALFORMED-BUS",
            Self::UnnamedBus => "DRC-UNNAMED-BUS",
            Self::BusRangeConflict => "DRC-BUS-RANGE-CONFLICT",
            Self::DanglingBusTap => "DRC-DANGLING-BUS-TAP",
            Self::MixedBusTap => "DRC-MIXED-BUS-TAP",
            Self::DuplicateBusMemberDriver => "DRC-DUPLICATE-BUS-MEMBER-DRIVER",
            Self::VectorWidthMismatch => "DRC-VECTOR-WIDTH-MISMATCH",
            Self::OffSheetConnectorWithoutPartner => "DRC-OFF-SHEET-CONNECTOR-WITHOUT-PARTNER",
            Self::ShortedOutputs => "DRC-SHORTED-OUTPUTS",
            Self::MissingGround => "DRC-MISSING-GROUND",
            Self::SourceToSource => "DRC-SOURCE-TO-SOURCE",
            Self::RecursiveInstantiation => "DRC-RECURSIVE-INSTANTIATION",
            Self::DuplicateName => "DRC-DUPLICATE-NAME",
            Self::EmptyName => "DRC-EMPTY-NAME",
            Self::InvalidName => "DRC-INVALID-NAME",
            Self::CaseCollidingNetNames => "DRC-CASE-COLLIDING-NET-NAMES",
            Self::MissingParameter => "DRC-MISSING-PARAMETER",
            Self::ValueOutOfRange => "DRC-VALUE-OUT-OF-RANGE",
            Self::UnknownComponent => "DRC-UNKNOWN-COMPONENT",
            Self::UnresolvedReference => "DRC-UNRESOLVED-REFERENCE",
            Self::SymbolUnplacedPin => "SYMBOL-UNPLACED-PIN",
            Self::SymbolOrphanedPin => "SYMBOL-ORPHANED-PIN",
            Self::SymbolPinOffGrid => "SYMBOL-PIN-OFF-GRID",
        }
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
}
