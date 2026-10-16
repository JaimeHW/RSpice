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
//!
//! A deck is read by builds older and newer than the one that wrote it, so
//! nothing in it is dropped for being unfamiliar. A row naming a rule this
//! build does not have, or a severity it cannot read, is kept as written and
//! reported; a deck written under a schema this build does not read is not
//! applied at all, is kept byte for byte, and is reported. Either way a save
//! from this build writes back what it was given.

use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

use super::types::{DrcSeverity, DrcViolationType};

/// The deck schema this build writes.
///
/// Reading is decided per version in [`DesignRuleDeck::from_stored`], never by
/// comparing against this: a gate written as "not the current version" changes
/// meaning the day the version moves, and would start refusing decks this
/// build wrote.
pub const DESIGN_RULE_DECK_SCHEMA_VERSION: u32 = 1;

/// The version a deck that does not state one was written under.
///
/// The first schema already wrote the field, so a deck without it was written
/// by hand. It is read as the first version rather than as whichever version
/// is current, which is what it would silently become after a bump.
const UNVERSIONED_DECK_SCHEMA: u64 = 1;

/// The per-rule severities one project has decided for itself.
///
/// Only a deviation is stored. A rule the deck says nothing about is reported
/// at its own default, and setting a rule back to that default removes the
/// entry rather than recording the default a second time — so a deck's size is
/// the number of decisions a team actually made, and a default that moves
/// moves for every project that had not overridden it.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DesignRuleDeck {
    severities: BTreeMap<DrcViolationType, DrcSeverity>,
    /// Rows this build cannot apply, by the code they were stored under,
    /// exactly as they were stored: a rule it does not have, or a severity it
    /// cannot read.
    retained: BTreeMap<String, serde_json::Value>,
    /// The whole stored deck, where this build cannot read it.
    unread: Option<UnreadDeck>,
}

#[derive(Debug, Clone, PartialEq)]
struct UnreadDeck {
    stored: serde_json::Value,
    issue: DeckIssue,
}

/// Something in a stored deck that this build did not apply as written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeckIssue {
    /// A row names a rule code this build does not have. The row is kept and
    /// written back on save.
    UnknownRule { code: String },
    /// A row names a rule this build has, at a severity it cannot read. The
    /// rule is reported at its default; the row is kept and written back.
    UnreadableSeverity { code: String, stored: String },
    /// The deck was written under a schema this build does not read. None of
    /// it is applied, and it is written back exactly as it was read.
    UnsupportedSchema { version: u64 },
    /// The deck is not one this build can read at all. None of it is applied,
    /// and it is written back exactly as it was read.
    Unreadable { reason: String },
}

impl std::fmt::Display for DeckIssue {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownRule { code } => write!(
                formatter,
                "the rule deck sets a severity for {code}, a rule this build does not have; \
                 the row is kept and not applied"
            ),
            Self::UnreadableSeverity { code, stored } => write!(
                formatter,
                "the rule deck sets {code} to {stored}, a severity this build cannot read; \
                 the rule is checked at its default and the row is kept"
            ),
            Self::UnsupportedSchema { version } => write!(
                formatter,
                "the rule deck was written under deck schema {version}, which this build does \
                 not read; no rule in it is applied, and it is saved back unchanged"
            ),
            Self::Unreadable { reason } => write!(
                formatter,
                "the rule deck cannot be read ({reason}); no rule in it is applied, and it is \
                 saved back unchanged"
            ),
        }
    }
}

/// Why the deck refused an edit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeckEditRefusal {
    /// The stored deck is one this build cannot read, so an edit here would
    /// either be lost on save or overwrite a deck this build never understood.
    Unread(DeckIssue),
}

impl std::fmt::Display for DeckEditRefusal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unread(issue) => write!(formatter, "{issue}"),
        }
    }
}

impl std::error::Error for DeckEditRefusal {}

impl DesignRuleDeck {
    /// Whether this deck stores nothing at all, which is what a new project
    /// has. A deck holding only rows this build cannot apply is not empty:
    /// those rows are written back.
    pub fn is_empty(&self) -> bool {
        self.severities.is_empty() && self.retained.is_empty() && self.unread.is_none()
    }

    /// How many rules this deck decides for itself, among the rules this
    /// build has.
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
    /// Note that a finding may still be raised at a severity the instance
    /// gives it — see [`DrcViolationType::severity_for_instance`] — and that
    /// an override from this deck is applied after that and wins. This is
    /// therefore what a reader is promised for every finding the deck
    /// overrides, and what they are promised for an object that emits no
    /// instance otherwise.
    pub fn effective_severity(&self, rule: DrcViolationType) -> DrcSeverity {
        self.severity(rule)
            .unwrap_or_else(|| rule.default_severity())
    }

    /// Report `rule` at `severity`. Returns whether the stored deck changed.
    ///
    /// `severity` equal to the rule's own default clears the entry instead of
    /// writing it. A row kept for the same code because its severity could
    /// not be read is replaced: the reader has now decided it.
    pub fn set_severity(
        &mut self,
        rule: DrcViolationType,
        severity: DrcSeverity,
    ) -> Result<bool, DeckEditRefusal> {
        self.editable()?;
        if severity == rule.default_severity() {
            return self.clear(rule);
        }
        let replaced = self.retained.remove(rule.rule_code()).is_some();
        Ok(self.severities.insert(rule, severity) != Some(severity) || replaced)
    }

    /// Put `rule` back on its own default. Returns whether the deck changed.
    pub fn clear(&mut self, rule: DrcViolationType) -> Result<bool, DeckEditRefusal> {
        self.editable()?;
        let replaced = self.retained.remove(rule.rule_code()).is_some();
        Ok(self.severities.remove(&rule).is_some() || replaced)
    }

    /// Put every rule this build has back on its own default.
    ///
    /// Rows naming a rule this build does not have are kept: it cannot say
    /// what that rule's default is, and a newer build reading the project
    /// still can.
    pub fn clear_all(&mut self) -> Result<(), DeckEditRefusal> {
        self.editable()?;
        self.severities.clear();
        self.retained
            .retain(|code, _| DrcViolationType::from_rule_code(code).is_none());
        Ok(())
    }

    /// Each rule this deck decides, in declaration order.
    pub fn entries(&self) -> impl Iterator<Item = (DrcViolationType, DrcSeverity)> + '_ {
        self.severities
            .iter()
            .map(|(rule, severity)| (*rule, *severity))
    }

    /// The deck as [`super::DrcConfig::severity_overrides`] takes it.
    ///
    /// Empty for a deck this build could not read: a check run under part of
    /// a deck it does not understand would be run under a policy nobody wrote.
    pub fn severity_overrides(&self) -> HashMap<DrcViolationType, DrcSeverity> {
        self.severities
            .iter()
            .map(|(rule, severity)| (*rule, *severity))
            .collect()
    }

    /// Everything in the stored deck this build did not apply as written, in
    /// a stable order: the whole deck first where it was not read, then each
    /// kept row by its code.
    pub fn issues(&self) -> Vec<DeckIssue> {
        let mut issues = Vec::new();
        if let Some(unread) = &self.unread {
            issues.push(unread.issue.clone());
        }
        for (code, stored) in &self.retained {
            issues.push(if DrcViolationType::from_rule_code(code).is_some() {
                DeckIssue::UnreadableSeverity {
                    code: code.clone(),
                    stored: stored.to_string(),
                }
            } else {
                DeckIssue::UnknownRule { code: code.clone() }
            });
        }
        issues
    }

    fn editable(&self) -> Result<(), DeckEditRefusal> {
        match &self.unread {
            Some(unread) => Err(DeckEditRefusal::Unread(unread.issue.clone())),
            None => Ok(()),
        }
    }

    /// Read a stored deck, keeping whatever this build cannot apply.
    ///
    /// Never fails: a project whose deck this build cannot read still opens,
    /// with the deck kept as it was and reported, rather than refusing the
    /// whole project over its severities.
    fn from_stored(stored: serde_json::Value) -> Self {
        let version = match stored.get("schema_version") {
            None => Some(UNVERSIONED_DECK_SCHEMA),
            Some(version) => version.as_u64(),
        };
        let read = match version {
            Some(1) => Self::read_v1(&stored),
            Some(version) => Err(DeckIssue::UnsupportedSchema { version }),
            None => Err(DeckIssue::Unreadable {
                reason: format!(
                    "schema_version {} is not a version",
                    stored["schema_version"]
                ),
            }),
        };
        read.unwrap_or_else(|issue| Self {
            unread: Some(UnreadDeck { stored, issue }),
            ..Self::default()
        })
    }

    /// The first deck schema: `schema_version`, and `severities` keyed by
    /// rule code. Anything else in it is not a first-schema deck.
    fn read_v1(stored: &serde_json::Value) -> Result<Self, DeckIssue> {
        let unreadable = |reason: String| DeckIssue::Unreadable { reason };
        let serde_json::Value::Object(fields) = stored else {
            return Err(unreadable("it is not an object".to_owned()));
        };
        if let Some(field) = fields
            .keys()
            .find(|field| !matches!(field.as_str(), "schema_version" | "severities"))
        {
            return Err(unreadable(format!("unknown field `{field}`")));
        }
        let mut deck = Self::default();
        let rows = match fields.get("severities") {
            None => return Ok(deck),
            Some(serde_json::Value::Object(rows)) => rows,
            Some(_) => return Err(unreadable("severities is not a table".to_owned())),
        };
        for (code, stored) in rows {
            let severity = DrcViolationType::from_rule_code(code).and_then(|rule| {
                serde_json::from_value::<DrcSeverity>(stored.clone())
                    .ok()
                    .map(|severity| (rule, severity))
            });
            match severity {
                Some((rule, severity)) => {
                    deck.severities.insert(rule, severity);
                }
                None => {
                    deck.retained.insert(code.clone(), stored.clone());
                }
            }
        }
        Ok(deck)
    }
}

/// The deck as this build writes it.
#[derive(Serialize)]
struct DesignRuleDeckWire {
    schema_version: u32,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    severities: BTreeMap<String, serde_json::Value>,
}

impl Serialize for DesignRuleDeck {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(unread) = &self.unread {
            return unread.stored.serialize(serializer);
        }
        let mut severities = self.retained.clone();
        for (rule, severity) in &self.severities {
            let severity = serde_json::to_value(severity).map_err(serde::ser::Error::custom)?;
            severities.insert(rule.rule_code().to_owned(), severity);
        }
        DesignRuleDeckWire {
            schema_version: DESIGN_RULE_DECK_SCHEMA_VERSION,
            severities,
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DesignRuleDeck {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer).map(Self::from_stored)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(json: &str) -> DesignRuleDeck {
        serde_json::from_str(json).expect("a stored deck always reads")
    }

    fn written(deck: &DesignRuleDeck) -> serde_json::Value {
        serde_json::to_value(deck).expect("the deck serializes")
    }

    #[test]
    fn a_rule_at_its_own_default_stores_nothing() {
        let mut deck = DesignRuleDeck::default();
        assert!(deck.is_empty());
        assert_eq!(
            deck.set_severity(
                DrcViolationType::DanglingWire,
                DrcViolationType::DanglingWire.default_severity()
            ),
            Ok(false)
        );
        assert!(deck.is_empty());
        assert_eq!(
            deck.effective_severity(DrcViolationType::DanglingWire),
            DrcSeverity::Warning
        );
    }

    #[test]
    fn an_override_is_stored_until_it_is_set_back_to_the_default() {
        let mut deck = DesignRuleDeck::default();
        assert_eq!(
            deck.set_severity(DrcViolationType::DanglingWire, DrcSeverity::Critical),
            Ok(true)
        );
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

        assert_eq!(
            deck.set_severity(DrcViolationType::DanglingWire, DrcSeverity::Critical),
            Ok(false)
        );
        assert_eq!(
            deck.set_severity(DrcViolationType::DanglingWire, DrcSeverity::Warning),
            Ok(true)
        );
        assert!(deck.is_empty());
    }

    #[test]
    fn the_deck_reaches_the_checker_as_its_override_map() {
        let mut deck = DesignRuleDeck::default();
        deck.set_severity(DrcViolationType::UnloadedOutput, DrcSeverity::Error)
            .unwrap();
        deck.set_severity(DrcViolationType::FloatingNode, DrcSeverity::Info)
            .unwrap();
        let overrides = deck.severity_overrides();
        assert_eq!(overrides.len(), 2);
        assert_eq!(
            overrides.get(&DrcViolationType::UnloadedOutput),
            Some(&DrcSeverity::Error)
        );
        assert_eq!(
            overrides.get(&DrcViolationType::FloatingNode),
            Some(&DrcSeverity::Info)
        );
    }

    #[test]
    fn a_stored_deck_round_trips_through_published_rule_codes() {
        let mut deck = DesignRuleDeck::default();
        deck.set_severity(DrcViolationType::UnconnectedPin, DrcSeverity::Critical)
            .unwrap();
        deck.set_severity(DrcViolationType::SymbolPinOffGrid, DrcSeverity::Critical)
            .unwrap();
        let json = serde_json::to_string(&deck).expect("the deck serializes");
        assert!(
            json.contains("\"DRC-UNCONNECTED-PIN\":\"Critical\""),
            "a stored row is keyed by the rule's published code: {json}"
        );
        assert!(
            json.contains("\"SYMBOL-PIN-OFF-GRID\":\"Critical\""),
            "the code a symbol finding is published under: {json}"
        );
        assert!(json.contains("\"schema_version\":1"));
        assert_eq!(read(&json), deck);
        assert!(read(&json).issues().is_empty());
    }

    #[test]
    fn an_absent_deck_restores_as_one_that_overrides_nothing() {
        let deck = read("{}");
        assert!(deck.is_empty());
        assert!(deck.issues().is_empty());
    }

    /// A row naming a rule this build does not have is kept, reported, and
    /// written back — so a newer build's rule survives a round trip through
    /// this one.
    #[test]
    fn a_row_naming_an_unknown_rule_is_kept_and_reported() {
        let mut deck = read(
            r#"{"schema_version":1,"severities":{"DRC-FUTURE-RULE":"Info","DRC-DANGLING-WIRE":"Error"}}"#,
        );
        assert_eq!(deck.len(), 1);
        assert_eq!(
            deck.severity(DrcViolationType::DanglingWire),
            Some(DrcSeverity::Error)
        );
        assert_eq!(
            deck.issues(),
            [DeckIssue::UnknownRule {
                code: "DRC-FUTURE-RULE".to_owned()
            }]
        );
        assert!(!deck.severity_overrides().is_empty());

        // An edit, and a reset, leave the unknown row where it was.
        deck.set_severity(DrcViolationType::FloatingNode, DrcSeverity::Error)
            .unwrap();
        deck.clear_all().unwrap();
        assert!(!deck.is_empty(), "the kept row is still stored");
        assert_eq!(
            written(&deck),
            serde_json::json!({"schema_version":1,"severities":{"DRC-FUTURE-RULE":"Info"}})
        );
    }

    #[test]
    fn a_severity_this_build_cannot_read_is_kept_until_the_rule_is_set() {
        let mut deck = read(r#"{"schema_version":1,"severities":{"DRC-DANGLING-WIRE":"Fatal"}}"#);
        assert_eq!(deck.severity(DrcViolationType::DanglingWire), None);
        assert_eq!(
            deck.issues(),
            [DeckIssue::UnreadableSeverity {
                code: "DRC-DANGLING-WIRE".to_owned(),
                stored: "\"Fatal\"".to_owned(),
            }]
        );
        assert_eq!(
            written(&deck),
            serde_json::json!({"schema_version":1,"severities":{"DRC-DANGLING-WIRE":"Fatal"}})
        );
        // Deciding the rule here replaces the row this build could not read.
        assert_eq!(
            deck.set_severity(DrcViolationType::DanglingWire, DrcSeverity::Error),
            Ok(true)
        );
        assert!(deck.issues().is_empty());
        assert_eq!(
            written(&deck),
            serde_json::json!({"schema_version":1,"severities":{"DRC-DANGLING-WIRE":"Error"}})
        );
    }

    /// A deck from a newer schema is not applied, is reported, cannot be
    /// edited here, and is written back exactly as it was read.
    #[test]
    fn a_deck_from_a_newer_schema_is_kept_verbatim_and_not_applied() {
        let stored = serde_json::json!({
            "schema_version": 2,
            "severities": {"DRC-DANGLING-WIRE": {"severity": "Error", "scope": "top"}},
            "waivable": ["DRC-FLOATING-NODE"],
        });
        let mut deck = read(&stored.to_string());
        assert!(!deck.is_empty());
        assert!(deck.severity_overrides().is_empty());
        assert_eq!(deck.len(), 0);
        assert_eq!(deck.issues(), [DeckIssue::UnsupportedSchema { version: 2 }]);
        assert!(matches!(
            deck.set_severity(DrcViolationType::DanglingWire, DrcSeverity::Info),
            Err(DeckEditRefusal::Unread(DeckIssue::UnsupportedSchema {
                version: 2
            }))
        ));
        assert!(deck.clear_all().is_err());
        assert_eq!(written(&deck), stored);
    }

    #[test]
    fn a_first_schema_deck_with_a_field_it_never_had_is_not_applied() {
        let stored = serde_json::json!({
            "schema_version": 1,
            "severities": {"DRC-DANGLING-WIRE": "Error"},
            "waivers": [],
        });
        let deck = read(&stored.to_string());
        assert!(deck.severity_overrides().is_empty());
        assert_eq!(
            deck.issues(),
            [DeckIssue::Unreadable {
                reason: "unknown field `waivers`".to_owned()
            }]
        );
        assert_eq!(written(&deck), stored);
    }

    /// A deck without a version was written by hand, before or outside any
    /// build, and is read as the first schema rather than as whichever schema
    /// is current.
    #[test]
    fn a_deck_that_states_no_version_is_read_as_the_first_schema() {
        let deck = read(r#"{"severities":{"DRC-DANGLING-WIRE":"Error"}}"#);
        assert_eq!(
            deck.severity(DrcViolationType::DanglingWire),
            Some(DrcSeverity::Error)
        );
        assert!(deck.issues().is_empty());
    }

    #[test]
    fn a_deck_that_is_not_a_table_still_lets_the_project_open() {
        for stored in ["[]", "\"strict\"", r#"{"schema_version":"one"}"#] {
            let deck = read(stored);
            assert!(deck.severity_overrides().is_empty(), "{stored}");
            assert!(
                matches!(deck.issues().as_slice(), [DeckIssue::Unreadable { .. }]),
                "{stored}: {:?}",
                deck.issues()
            );
            assert_eq!(
                written(&deck),
                serde_json::from_str::<serde_json::Value>(stored).unwrap()
            );
        }
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
