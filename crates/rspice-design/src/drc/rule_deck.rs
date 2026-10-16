//! A project's durable design-rule severities.
//!
//! Every rule has a default severity of its own — see
//! [`DrcViolationType::default_severity`] — and the checker applies an
//! override after it. Until this deck existed, nothing could write one: the
//! override map was built empty on every run, so the shipped default was the
//! only severity any design could be checked at. A team that had reviewed a
//! finding and decided it was a note, or decided a note was a sign-off
//! blocker, had no recourse.
//!
//! The deck is the stored half of that contract. It belongs to the project,
//! because which findings block a sign-off is a decision a team shares, not a
//! personal preference, and because it is folded into the check's input digest:
//! changing the deck stales every report that was produced under the old one
//! rather than silently relabelling its findings.

use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

use super::types::{DrcSeverity, DrcViolationType};

pub const DESIGN_RULE_DECK_SCHEMA_VERSION: u32 = 1;

/// The per-rule severities one project has decided for itself.
///
/// Only a deviation is stored. A rule the deck says nothing about is reported
/// at its own default, and setting a rule back to that default removes the
/// entry rather than recording the default a second time — so a deck's size is
/// the number of decisions a team actually made, and a default that moves
/// moves for every project that had not overridden it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DesignRuleDeck {
    severities: BTreeMap<DrcViolationType, DrcSeverity>,
}

impl DesignRuleDeck {
    /// Whether this deck overrides nothing, which is what a new project has.
    pub fn is_empty(&self) -> bool {
        self.severities.is_empty()
    }

    /// How many rules this deck decides for itself.
    pub fn len(&self) -> usize {
        self.severities.len()
    }

    /// The severity this deck sets for `rule`, or `None` where it defers to
    /// the rule's own default.
    pub fn severity(&self, rule: DrcViolationType) -> Option<DrcSeverity> {
        self.severities.get(&rule).copied()
    }

    /// What `rule` will be reported as: its default unless this deck says
    /// otherwise.
    ///
    /// Note that a finding may still be raised at a severity the component
    /// gives it — see [`DrcViolationType::severity_for_component`] — and that
    /// an override from this deck is applied after that and wins. This is
    /// therefore what a reader is promised for every finding the deck
    /// overrides, and what they are promised for an object carrying no
    /// reference designator otherwise.
    pub fn effective_severity(&self, rule: DrcViolationType) -> DrcSeverity {
        self.severity(rule)
            .unwrap_or_else(|| rule.default_severity())
    }

    /// Report `rule` at `severity`. Returns whether the stored deck changed.
    ///
    /// `severity` equal to the rule's own default clears the entry instead of
    /// writing it.
    pub fn set_severity(&mut self, rule: DrcViolationType, severity: DrcSeverity) -> bool {
        if severity == rule.default_severity() {
            return self.clear(rule);
        }
        self.severities.insert(rule, severity) != Some(severity)
    }

    /// Put `rule` back on its own default. Returns whether the deck changed.
    pub fn clear(&mut self, rule: DrcViolationType) -> bool {
        self.severities.remove(&rule).is_some()
    }

    /// Put every rule back on its own default.
    pub fn clear_all(&mut self) {
        self.severities.clear();
    }

    /// Each rule this deck decides, in rule-code order.
    pub fn entries(&self) -> impl Iterator<Item = (DrcViolationType, DrcSeverity)> + '_ {
        self.severities
            .iter()
            .map(|(rule, severity)| (*rule, *severity))
    }

    /// The deck as [`super::DrcConfig::severity_overrides`] takes it.
    pub fn severity_overrides(&self) -> HashMap<DrcViolationType, DrcSeverity> {
        self.severities
            .iter()
            .map(|(rule, severity)| (*rule, *severity))
            .collect()
    }
}

/// The deck as it is stored: rules named by their published code rather than by
/// a Rust variant spelling, so a renamed variant does not silently drop a
/// team's decision, and a retired rule's row is dropped on load instead of
/// failing the whole project open.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DesignRuleDeckWire {
    #[serde(default = "deck_schema_version")]
    schema_version: u32,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    severities: BTreeMap<String, DrcSeverity>,
}

const fn deck_schema_version() -> u32 {
    DESIGN_RULE_DECK_SCHEMA_VERSION
}

impl Serialize for DesignRuleDeck {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        DesignRuleDeckWire {
            schema_version: DESIGN_RULE_DECK_SCHEMA_VERSION,
            severities: self
                .severities
                .iter()
                .map(|(rule, severity)| (rule.rule_code(), *severity))
                .collect(),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DesignRuleDeck {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let wire = DesignRuleDeckWire::deserialize(deserializer)?;
        Ok(Self {
            severities: wire
                .severities
                .into_iter()
                .filter_map(|(code, severity)| {
                    DrcViolationType::from_rule_code(&code).map(|rule| (rule, severity))
                })
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_rule_at_its_own_default_stores_nothing() {
        let mut deck = DesignRuleDeck::default();
        assert!(deck.is_empty());
        assert!(!deck.set_severity(
            DrcViolationType::DanglingWire,
            DrcViolationType::DanglingWire.default_severity()
        ));
        assert!(deck.is_empty());
        assert_eq!(
            deck.effective_severity(DrcViolationType::DanglingWire),
            DrcSeverity::Warning
        );
    }

    #[test]
    fn an_override_is_stored_until_it_is_set_back_to_the_default() {
        let mut deck = DesignRuleDeck::default();
        assert!(deck.set_severity(DrcViolationType::DanglingWire, DrcSeverity::Critical));
        assert_eq!(deck.len(), 1);
        assert_eq!(
            deck.severity(DrcViolationType::DanglingWire),
            Some(DrcSeverity::Critical)
        );
        assert_eq!(
            deck.effective_severity(DrcViolationType::DanglingWire),
            DrcSeverity::Critical
        );
        // The rule's own default is untouched by the deck that overrode it.
        assert_eq!(
            DrcViolationType::DanglingWire.default_severity(),
            DrcSeverity::Warning
        );

        assert!(!deck.set_severity(DrcViolationType::DanglingWire, DrcSeverity::Critical));
        assert!(deck.set_severity(DrcViolationType::DanglingWire, DrcSeverity::Warning));
        assert!(deck.is_empty());
    }

    #[test]
    fn the_deck_reaches_the_checker_as_its_override_map() {
        let mut deck = DesignRuleDeck::default();
        deck.set_severity(DrcViolationType::UnloadedOutput, DrcSeverity::Error);
        deck.set_severity(DrcViolationType::MissingGround, DrcSeverity::Info);
        let overrides = deck.severity_overrides();
        assert_eq!(overrides.len(), 2);
        assert_eq!(
            overrides.get(&DrcViolationType::UnloadedOutput),
            Some(&DrcSeverity::Error)
        );
        assert_eq!(
            overrides.get(&DrcViolationType::MissingGround),
            Some(&DrcSeverity::Info)
        );
    }

    #[test]
    fn a_stored_deck_round_trips_through_published_rule_codes() {
        let mut deck = DesignRuleDeck::default();
        deck.set_severity(DrcViolationType::UnconnectedPin, DrcSeverity::Critical);
        deck.set_severity(DrcViolationType::MissingGround, DrcSeverity::Warning);
        let json = serde_json::to_string(&deck).expect("the deck serializes");
        assert!(
            json.contains("\"DRC-UNCONNECTED-PIN\":\"Critical\""),
            "a stored row is keyed by the rule's published code: {json}"
        );
        assert!(json.contains("\"schema_version\":1"));
        assert_eq!(
            serde_json::from_str::<DesignRuleDeck>(&json).expect("the deck restores"),
            deck
        );
    }

    #[test]
    fn an_absent_deck_restores_as_one_that_overrides_nothing() {
        assert!(
            serde_json::from_str::<DesignRuleDeck>("{}")
                .expect("an empty object restores")
                .is_empty()
        );
    }

    /// A rule this build does not have is dropped, not fatal.
    ///
    /// The alternative is a project that cannot be opened at all because a
    /// rule it once set a severity for has since been retired.
    #[test]
    fn a_row_naming_a_retired_rule_is_dropped_on_load() {
        let deck: DesignRuleDeck = serde_json::from_str(
            r#"{"schema_version":1,"severities":{"DRC-RETIRED-RULE":"Info","DRC-DANGLING-WIRE":"Error"}}"#,
        )
        .expect("an unknown rule code does not fail the load");
        assert_eq!(deck.len(), 1);
        assert_eq!(
            deck.severity(DrcViolationType::DanglingWire),
            Some(DrcSeverity::Error)
        );
    }

    #[test]
    fn every_rule_has_a_distinct_code_that_resolves_back_to_it() {
        let mut codes = std::collections::BTreeSet::new();
        for rule in DrcViolationType::ALL {
            let code = rule.rule_code();
            assert!(code.starts_with("DRC-"), "{code} is not a rule code");
            assert!(
                codes.insert(code.clone()),
                "two rules share the code {code}"
            );
            assert_eq!(DrcViolationType::from_rule_code(&code), Some(rule));
        }
        assert_eq!(codes.len(), DrcViolationType::ALL.len());
        assert_eq!(DrcViolationType::from_rule_code("DRC-NOT-A-RULE"), None);
    }

    /// The listing a surface walks is the listing the checker can report, and
    /// it is banded in the order it is declared.
    #[test]
    fn the_rule_listing_is_grouped_by_category() {
        let mut seen = Vec::new();
        for rule in DrcViolationType::ALL {
            let category = rule.category();
            if seen.last() != Some(&category) {
                assert!(
                    !seen.contains(&category),
                    "{} is split into two runs",
                    category.label()
                );
                seen.push(category);
            }
        }
        assert_eq!(seen.len(), 6, "every category heads exactly one run");
    }
}
