//! Waivers: a recorded decision that one finding is accepted as it stands.
//!
//! A waiver names the finding it covers by what the finding is about — the
//! rule's code and the object inside the cellview it is located at — never by
//! the pass that reported it, its wording, or its place in a list. That is what
//! lets a waiver survive a re-check after an unrelated edit, and what keeps it
//! from following its rule to a different object.
//!
//! A waiver is stored with the cellview it waives, in that cellview's own
//! document, so it travels with a library. A finding a check states inside an
//! instance is about an object drawn in the instantiated cellview, so it is
//! that cellview's waivers that apply to it, at every occurrence.
//!
//! A waived finding is still a finding: it is listed with the reason, author
//! and time it was waived, and it stops counting against the design — the run
//! gate does not refuse on it. Only a rule the rule table marks waivable can be
//! waived. A waiver on any other rule, or on a rule this build does not have,
//! is kept as it was stored and is not applied.
//!
//! The list is read by builds older and newer than the one that wrote it, the
//! way a project's rule deck is: a list written under a schema this build does
//! not read is not applied, is kept byte for byte, and refuses edits, so a save
//! from this build writes back exactly what it was given.

use std::collections::BTreeMap;

use rspice_design_model::cell_view::CellViewRef;
use serde::{Deserialize, Serialize};

use super::types::{DrcLocation, DrcResult, DrcViolation, DrcViolationType};
use crate::project_sources::{CanonicalCellViewOwnerKey, canonical_cell_view_owner_key};

/// The waiver schema this build writes.
///
/// Reading is decided per version in [`DrcWaivers::from_stored`], never by
/// comparing against this, for the reason the rule deck gives.
pub const DRC_WAIVER_SCHEMA_VERSION: u32 = 1;

/// One recorded decision that one finding is accepted as it stands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DrcWaiver {
    /// The code of the rule the waived finding is an instance of.
    pub rule: String,
    /// The object the finding is located at, inside the cellview the waiver
    /// is stored with, as [`location_key`] states it.
    pub object: String,
    /// Why the finding is accepted. Never empty.
    pub reason: String,
    /// Who recorded the decision.
    pub author: String,
    /// When it was recorded, in milliseconds since the Unix epoch.
    pub waived_at_ms: u64,
}

impl DrcWaiver {
    /// The rule this waiver names, where this build has it.
    pub fn violation_type(&self) -> Option<DrcViolationType> {
        DrcViolationType::from_rule_code(&self.rule)
    }

    /// Whether this waiver is applied: it names a rule this build has, and
    /// that rule may be waived.
    pub fn applies(&self) -> bool {
        self.violation_type()
            .is_some_and(|rule| rule.rule().waivable)
    }
}

/// The waivers one cellview's document stores.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DrcWaivers {
    entries: Vec<DrcWaiver>,
    /// The whole stored list, where this build cannot read it.
    unread: Option<UnreadWaivers>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct UnreadWaivers {
    stored: serde_json::Value,
    reason: String,
}

/// Why a waiver edit was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WaiverEditRefusal {
    /// The stored waivers are ones this build cannot read, so an edit here
    /// would either be lost on save or overwrite a list it never understood.
    Unread(String),
    /// The rule may not be waived, or this build does not have it.
    NotWaivable(String),
    /// A waiver has to say why the finding is accepted.
    NoReason,
}

impl std::fmt::Display for WaiverEditRefusal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unread(reason) => write!(
                formatter,
                "the waivers stored with this cellview cannot be read ({reason}); they are kept \
                 as they are and cannot be edited here"
            ),
            Self::NotWaivable(code) => match DrcViolationType::from_rule_code(code) {
                Some(rule) => write!(
                    formatter,
                    "{} may not be waived: a finding of it is repaired, not accepted",
                    rule.rule().title
                ),
                None => write!(formatter, "{code} is not a rule this build has"),
            },
            Self::NoReason => {
                formatter.write_str("a waiver has to say why the finding is accepted")
            }
        }
    }
}

impl std::error::Error for WaiverEditRefusal {}

impl DrcWaivers {
    /// Whether this document stores nothing at all. A list this build could
    /// not read is not empty: it is written back.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty() && self.unread.is_none()
    }

    /// Every waiver stored, in the order they were recorded. Empty where the
    /// stored list could not be read.
    pub fn entries(&self) -> &[DrcWaiver] {
        &self.entries
    }

    /// Why the stored list could not be read, where it could not.
    pub fn unread(&self) -> Option<&str> {
        self.unread.as_ref().map(|unread| unread.reason.as_str())
    }

    /// The waiver stored for `rule` on `object`.
    pub fn get(&self, rule: &str, object: &str) -> Option<&DrcWaiver> {
        self.entries
            .iter()
            .find(|waiver| waiver.rule == rule && waiver.object == object)
    }

    /// Record `waiver`, replacing one already stored for the same finding.
    ///
    /// Refused for a rule that may not be waived, for a waiver that does not
    /// say why, and where the stored list could not be read.
    pub fn waive(&mut self, mut waiver: DrcWaiver) -> Result<(), WaiverEditRefusal> {
        self.editable()?;
        if !waiver.applies() {
            return Err(WaiverEditRefusal::NotWaivable(waiver.rule));
        }
        let reason = waiver.reason.trim();
        if reason.is_empty() {
            return Err(WaiverEditRefusal::NoReason);
        }
        waiver.reason = reason.to_owned();
        match self
            .entries
            .iter_mut()
            .find(|stored| stored.rule == waiver.rule && stored.object == waiver.object)
        {
            Some(stored) => *stored = waiver,
            None => self.entries.push(waiver),
        }
        Ok(())
    }

    /// Remove the waiver stored for `rule` on `object`, returning it.
    ///
    /// Any stored waiver may be removed, including one this build does not
    /// apply: that is how a reader acts on one reported as ignored.
    pub fn remove(
        &mut self,
        rule: &str,
        object: &str,
    ) -> Result<Option<DrcWaiver>, WaiverEditRefusal> {
        self.editable()?;
        Ok(self
            .entries
            .iter()
            .position(|waiver| waiver.rule == rule && waiver.object == object)
            .map(|index| self.entries.remove(index)))
    }

    fn editable(&self) -> Result<(), WaiverEditRefusal> {
        match &self.unread {
            Some(unread) => Err(WaiverEditRefusal::Unread(unread.reason.clone())),
            None => Ok(()),
        }
    }

    /// Read a stored list, keeping whatever this build cannot read.
    ///
    /// Never fails: a cellview whose waivers this build cannot read still
    /// opens, with the list kept as it was, applied nowhere and reported.
    fn from_stored(stored: serde_json::Value) -> Self {
        let read = match stored.get("schema_version").map(serde_json::Value::as_u64) {
            Some(Some(1)) => Self::read_v1(&stored),
            Some(Some(version)) => Err(format!(
                "waiver schema {version} is not one this build reads"
            )),
            Some(None) => Err("its schema_version is not a version".to_owned()),
            None => Err("it states no schema_version".to_owned()),
        };
        read.unwrap_or_else(|reason| Self {
            entries: Vec::new(),
            unread: Some(UnreadWaivers { stored, reason }),
        })
    }

    /// The first waiver schema: `schema_version`, and `waivers`, each one a
    /// [`DrcWaiver`] with nothing else in it.
    fn read_v1(stored: &serde_json::Value) -> Result<Self, String> {
        let serde_json::Value::Object(fields) = stored else {
            return Err("it is not an object".to_owned());
        };
        if let Some(field) = fields
            .keys()
            .find(|field| !matches!(field.as_str(), "schema_version" | "waivers"))
        {
            return Err(format!("unknown field `{field}`"));
        }
        let entries = match fields.get("waivers") {
            None => Vec::new(),
            Some(waivers) => serde_json::from_value::<Vec<DrcWaiver>>(waivers.clone())
                .map_err(|error| error.to_string())?,
        };
        Ok(Self {
            entries,
            unread: None,
        })
    }
}

/// The waivers as this build writes them.
#[derive(Serialize)]
struct DrcWaiversWire<'a> {
    schema_version: u32,
    waivers: &'a [DrcWaiver],
}

impl Serialize for DrcWaivers {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self.unread {
            Some(unread) => unread.stored.serialize(serializer),
            None => DrcWaiversWire {
                schema_version: DRC_WAIVER_SCHEMA_VERSION,
                waivers: &self.entries,
            }
            .serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for DrcWaivers {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer).map(Self::from_stored)
    }
}

/// A stable key of the object a finding is located at.
///
/// What survives an unrelated edit is the object's own identity, so that is
/// what the key is: a component or wire by its id, a terminal by its
/// component and pin, a named net or a label by its name, and a net the
/// drawing does not name by the terminals on it. Where the object is drawn
/// and what the message says about it are not part of it. A finding inside an
/// instantiated cellview is keyed by the occurrence path down to that view and
/// its own key inside it, which is what tells `X1/M3` from `X2/M3`. A point is
/// the last resort, for a finding that names no object at all.
///
/// An unnamed net's name is the extractor's number for it, which placing
/// anything to its left gives another net, so it is never the key: a waiver
/// keyed by it would move to whichever net took the number. One stored that
/// way by an earlier build covers nothing, and is reported for it.
pub fn location_key(location: &DrcLocation) -> String {
    match location {
        DrcLocation::Terminal {
            component_id,
            pin_name,
            ..
        } => format!("terminal:{component_id}:{pin_name}"),
        DrcLocation::Component { id, .. } => format!("component:{id}"),
        DrcLocation::Node {
            net_name,
            terminals,
        } if terminals.is_empty() => format!("net:{net_name}"),
        DrcLocation::Node { terminals, .. } => {
            // Length-prefixed, because a pin name may contain any separator
            // chosen here.
            let terminals = terminals
                .iter()
                .map(|(component, pin)| format!("{component}:{}:{pin}", pin.len()))
                .collect::<String>();
            format!("net-of:{terminals}")
        }
        DrcLocation::Wire { id } => format!("wire:{id}"),
        DrcLocation::NetLabel { name } => format!("label:{name}"),
        DrcLocation::Bus { id } => format!("bus:{id}"),
        DrcLocation::BusTap { id } => format!("bus-tap:{id}"),
        DrcLocation::InsideInstance { path, view, inner } => {
            // Length-prefixed, because an instance name or a net name inside
            // may contain any separator chosen here.
            let path = path.to_string();
            let view = view.key();
            let inner = location_key(inner);
            format!(
                "instance:{}:{path}{}:{view}{}:{inner}",
                path.len(),
                view.len(),
                inner.len()
            )
        }
        DrcLocation::Global => "global".to_owned(),
        DrcLocation::CellView { reference } => format!("cellview:{}", reference.key()),
        DrcLocation::SymbolPin {
            reference,
            pin_name,
            ..
        } => format!("symbol-pin:{}:{pin_name}", reference.key()),
        DrcLocation::Point { x, y } => format!("point:{x}:{y}"),
    }
}

/// Where the waiver of a finding located at `location` is stored, and the
/// object it names there.
///
/// A finding stated inside an instance is about an object drawn in the
/// instantiated cellview, so its waiver is that cellview's, keyed by the
/// object's place inside it and not by the occurrence: one waiver covers the
/// object at every occurrence. `None` is the checked cellview itself.
pub fn waiver_site(location: &DrcLocation) -> (Option<&CellViewRef>, String) {
    match location {
        DrcLocation::InsideInstance { view, inner, .. } => match waiver_site(inner) {
            (Some(deeper), object) => (Some(deeper), object),
            (None, object) => (Some(view), object),
        },
        // About the cellview itself, so it would be stored there.
        DrcLocation::CellView { reference } => (Some(reference), "cellview".to_owned()),
        other => (None, location_key(other)),
    }
}

/// The waivers one check applies: those stored with the checked cellview,
/// and those stored with each cellview it instantiates.
///
/// Ordered by cellview, so two equal sets print and compare alike however
/// each was gathered: it rides in the configuration the run gate digests.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CheckWaivers {
    subject: Vec<DrcWaiver>,
    instantiated: BTreeMap<CanonicalCellViewOwnerKey, Vec<DrcWaiver>>,
}

impl CheckWaivers {
    /// The waivers stored with the checked cellview. Only those this build
    /// applies are kept.
    pub fn for_subject(waivers: &DrcWaivers) -> Self {
        Self {
            subject: applied(waivers),
            instantiated: BTreeMap::new(),
        }
    }

    /// Add the waivers stored with `cellview`, which apply to findings stated
    /// inside an instance of it.
    #[must_use]
    pub fn instantiating(mut self, cellview: &CellViewRef, waivers: &DrcWaivers) -> Self {
        let applied = applied(waivers);
        if !applied.is_empty() {
            self.instantiated.insert(owner_key(cellview), applied);
        }
        self
    }

    /// Whether no waiver is applied at all.
    pub fn is_empty(&self) -> bool {
        self.subject.is_empty() && self.instantiated.is_empty()
    }

    /// The waiver that covers `violation`, where one does.
    pub fn covering(&self, violation: &DrcViolation) -> Option<&DrcWaiver> {
        if !violation.violation_type.rule().waivable {
            return None;
        }
        let code = violation.violation_type.rule_code();
        let (site, object) = waiver_site(&violation.location);
        let stored = match site {
            None => self.subject.as_slice(),
            Some(cellview) => self
                .instantiated
                .get(&owner_key(cellview))
                .map_or(&[][..], Vec::as_slice),
        };
        stored
            .iter()
            .find(|waiver| waiver.rule == code && waiver.object == object)
    }

    /// State on every finding in `result` the waiver that covers it, and on
    /// every other finding that none does.
    pub fn apply(&self, result: &mut DrcResult) {
        for violation in result.violations_mut() {
            violation.waiver = self.covering(violation).cloned();
        }
    }

    /// The waivers stored with the checked cellview that cover none of its
    /// own findings in `result`: the finding they were recorded against is
    /// gone.
    pub fn unmatched_in<'a>(&'a self, result: &DrcResult) -> Vec<&'a DrcWaiver> {
        self.subject
            .iter()
            .filter(|waiver| {
                !result.violations().iter().any(|violation| {
                    matches!(waiver_site(&violation.location), (None, ref object) if *object == waiver.object)
                        && violation.violation_type.rule_code() == waiver.rule
                })
            })
            .collect()
    }

    /// Every waiver this check applies that covers none of the findings in
    /// `result`, with the cellview it is stored with: `None` for the checked
    /// cellview, whose own come first, then each instantiated cellview's in
    /// the order of its key. A waiver stored with an instantiated cellview is
    /// matched by a finding at any occurrence of it.
    pub fn unmatched_anywhere<'a>(
        &'a self,
        result: &DrcResult,
    ) -> Vec<(Option<&'a CanonicalCellViewOwnerKey>, &'a DrcWaiver)> {
        let covers = |waiver: &DrcWaiver| {
            result.violations().iter().any(|violation| {
                self.covering(violation)
                    .is_some_and(|covering| std::ptr::eq(covering, waiver))
            })
        };
        self.subject
            .iter()
            .map(|waiver| (None, waiver))
            .chain(self.instantiated.iter().flat_map(|(cellview, waivers)| {
                waivers.iter().map(move |waiver| (Some(cellview), waiver))
            }))
            .filter(|(_, waiver)| !covers(waiver))
            .collect()
    }
}

fn applied(waivers: &DrcWaivers) -> Vec<DrcWaiver> {
    waivers
        .entries()
        .iter()
        .filter(|waiver| waiver.applies())
        .cloned()
        .collect()
}

fn owner_key(cellview: &CellViewRef) -> CanonicalCellViewOwnerKey {
    canonical_cell_view_owner_key(&cellview.library, &cellview.cell, &cellview.view)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::drc::DrcSeverity;
    use rspice_app_types::hierarchy_path::InstancePath;

    fn waiver(rule: &str, object: &str) -> DrcWaiver {
        DrcWaiver {
            rule: rule.to_owned(),
            object: object.to_owned(),
            reason: "the node is a test point".to_owned(),
            author: "ana".to_owned(),
            waived_at_ms: 1_760_000_000_000,
        }
    }

    fn component(id: u64) -> DrcLocation {
        DrcLocation::Component {
            id,
            name: format!("R{id}"),
        }
    }

    fn finding(rule: DrcViolationType, location: DrcLocation) -> DrcViolation {
        DrcViolation::new(0, rule, "found", location)
    }

    fn stored(waivers: &[DrcWaiver]) -> DrcWaivers {
        let mut stored = DrcWaivers::default();
        for waiver in waivers {
            stored
                .waive(waiver.clone())
                .expect("the rule may be waived");
        }
        stored
    }

    /// What a document stores is what it reads back, under the schema it
    /// was written in.
    #[test]
    fn waivers_are_written_and_read_back_as_they_were() {
        let waivers = stored(&[waiver("DRC-FLOATING-NODE", "component:5")]);
        let text = serde_json::to_string(&waivers).expect("waivers always write");
        let value = serde_json::from_str::<serde_json::Value>(&text).expect("they write JSON");
        assert_eq!(value["schema_version"], 1);
        assert_eq!(
            serde_json::from_str::<DrcWaivers>(&text).expect("they read back"),
            waivers
        );
    }

    /// A list written under a schema this build does not read, or holding a
    /// waiver it cannot read, is kept byte for byte, applied nowhere and
    /// refuses edits.
    #[test]
    fn a_list_this_build_cannot_read_is_kept_applied_nowhere_and_not_edited() {
        for stored in [
            serde_json::json!({
                "schema_version": 2,
                "waivers": [{"rule": "DRC-FLOATING-NODE", "object": "component:5"}],
            }),
            serde_json::json!({
                "schema_version": 1,
                "waivers": [{
                    "rule": "DRC-FLOATING-NODE",
                    "object": "component:5",
                    "reason": "test point",
                    "author": "ana",
                    "waived_at_ms": 1,
                    "expires_at_ms": 2,
                }],
            }),
        ] {
            let mut read = serde_json::from_value::<DrcWaivers>(stored.clone())
                .expect("a stored list always reads");
            assert!(read.unread().is_some(), "{stored}");
            assert!(read.entries().is_empty(), "{stored}");
            assert!(!read.is_empty(), "{stored}");
            assert!(CheckWaivers::for_subject(&read).is_empty(), "{stored}");
            assert!(matches!(
                read.waive(waiver("DRC-FLOATING-NODE", "component:6")),
                Err(WaiverEditRefusal::Unread(_))
            ));
            assert!(matches!(
                read.remove("DRC-FLOATING-NODE", "component:5"),
                Err(WaiverEditRefusal::Unread(_))
            ));
            assert_eq!(serde_json::to_value(&read).expect("it writes back"), stored);
        }
    }

    /// A waiver covers the one finding it was recorded against: its rule on
    /// its object. It follows nothing to another object of the same rule, or
    /// to another rule on the same object.
    #[test]
    fn a_waiver_covers_its_finding_and_follows_nothing_to_another() {
        let applied =
            CheckWaivers::for_subject(&stored(&[waiver("DRC-FLOATING-NODE", "component:5")]));
        let mut result = DrcResult::new();
        result.add_violation(finding(DrcViolationType::FloatingNode, component(5)));
        result.add_violation(finding(DrcViolationType::FloatingNode, component(6)));
        result.add_violation(finding(DrcViolationType::UnconnectedPin, component(5)));
        applied.apply(&mut result);
        let waived = result
            .violations()
            .iter()
            .map(|violation| violation.waiver.is_some())
            .collect::<Vec<_>>();
        assert_eq!(waived, [true, false, false]);
    }

    /// A waived error stops counting against the design: the run gate asks
    /// whether there are errors, and there are none.
    #[test]
    fn a_waived_error_does_not_count() {
        let applied =
            CheckWaivers::for_subject(&stored(&[waiver("DRC-SHORTED-OUTPUTS", "net:out")]));
        let mut result = DrcResult::new();
        result.add_violation(finding(
            DrcViolationType::ShortedOutputs,
            DrcLocation::Node {
                net_name: "out".to_owned(),
                terminals: Vec::new(),
            },
        ));
        assert!(result.has_errors());
        applied.apply(&mut result);
        assert!(!result.has_errors());
        assert!(result.passed());
        assert_eq!(result.summary().errors, 0);
        assert_eq!(
            result.total_count(),
            1,
            "a waived finding is still a finding"
        );
    }

    /// A rule that may not be waived refuses a waiver, and one stored anyway —
    /// by hand, or by a build whose table differed — is kept and applied to
    /// nothing.
    #[test]
    fn a_waiver_on_a_rule_that_may_not_be_waived_is_refused_and_ignored() {
        let duplicate = waiver("DRC-DUPLICATE-NAME", "component:5");
        assert_eq!(
            DrcWaivers::default().waive(duplicate.clone()),
            Err(WaiverEditRefusal::NotWaivable(
                "DRC-DUPLICATE-NAME".to_owned()
            ))
        );
        let stored = serde_json::from_value::<DrcWaivers>(serde_json::json!({
            "schema_version": 1,
            "waivers": [duplicate],
        }))
        .expect("a stored list always reads");
        assert_eq!(stored.entries().len(), 1, "it is kept");
        let mut result = DrcResult::new();
        result.add_violation(DrcViolation {
            severity: DrcSeverity::Critical,
            ..finding(DrcViolationType::DuplicateName, component(5))
        });
        CheckWaivers::for_subject(&stored).apply(&mut result);
        assert!(result.violations()[0].waiver.is_none());
        assert!(result.has_errors());
    }

    /// A finding stated inside an instance is about an object drawn in the
    /// instantiated cellview, so that cellview's waiver covers it at every
    /// occurrence, and the checked cellview's own waiver of the same key does
    /// not.
    #[test]
    fn a_finding_inside_an_instance_is_waived_by_the_cellview_it_is_drawn_in() {
        let amp = CellViewRef::new("work", "amp", "schematic");
        let amp_waivers = stored(&[waiver("DRC-FLOATING-NODE", "component:5")]);
        let inside = |instance: &str| DrcLocation::InsideInstance {
            path: InstancePath::parse(instance).expect("an instance path"),
            view: amp.clone(),
            inner: Box::new(component(5)),
        };
        let applied = CheckWaivers::for_subject(&DrcWaivers::default())
            .instantiating(&CellViewRef::new("WORK", "Amp", "schematic"), &amp_waivers);
        let mut result = DrcResult::new();
        result.add_violation(finding(DrcViolationType::FloatingNode, inside("/X1")));
        result.add_violation(finding(DrcViolationType::FloatingNode, inside("/X2")));
        result.add_violation(finding(DrcViolationType::FloatingNode, component(5)));
        applied.apply(&mut result);
        let waived = result
            .violations()
            .iter()
            .map(|violation| violation.waiver.is_some())
            .collect::<Vec<_>>();
        assert_eq!(waived, [true, true, false]);

        let own = CheckWaivers::for_subject(&amp_waivers);
        let mut result = DrcResult::new();
        result.add_violation(finding(DrcViolationType::FloatingNode, inside("/X1")));
        own.apply(&mut result);
        assert!(result.violations()[0].waiver.is_none());
    }

    /// A waiver whose finding is gone is found, so it can be reported.
    #[test]
    fn a_waiver_whose_finding_is_gone_is_unmatched() {
        let applied = CheckWaivers::for_subject(&stored(&[
            waiver("DRC-FLOATING-NODE", "component:5"),
            waiver("DRC-FLOATING-NODE", "component:6"),
        ]));
        let mut result = DrcResult::new();
        result.add_violation(finding(DrcViolationType::FloatingNode, component(5)));
        assert_eq!(
            applied
                .unmatched_in(&result)
                .into_iter()
                .map(|waiver| waiver.object.as_str())
                .collect::<Vec<_>>(),
            ["component:6"]
        );
    }

    /// Across the design, a waiver stored with an instantiated cellview is
    /// matched by its finding at any occurrence, and one matched nowhere is
    /// reported with the cellview it is stored with.
    #[test]
    fn a_waiver_matched_at_no_occurrence_is_unmatched_where_it_is_stored() {
        let amp = CellViewRef::new("work", "amp", "schematic");
        let applied =
            CheckWaivers::for_subject(&stored(&[waiver("DRC-FLOATING-NODE", "component:1")]))
                .instantiating(
                    &amp,
                    &stored(&[
                        waiver("DRC-FLOATING-NODE", "component:5"),
                        waiver("DRC-FLOATING-NODE", "component:6"),
                    ]),
                );
        let mut result = DrcResult::new();
        result.add_violation(finding(
            DrcViolationType::FloatingNode,
            DrcLocation::InsideInstance {
                path: InstancePath::parse("/X2").expect("an instance path"),
                view: amp.clone(),
                inner: Box::new(component(5)),
            },
        ));
        let unmatched = applied
            .unmatched_anywhere(&result)
            .into_iter()
            .map(|(stored_with, waiver)| (stored_with.is_some(), waiver.object.as_str()))
            .collect::<Vec<_>>();
        assert_eq!(unmatched, [(false, "component:1"), (true, "component:6")]);
    }

    /// A waiver has to say why, and the reason is stored as written, less
    /// the space around it.
    #[test]
    fn a_waiver_without_a_reason_is_refused() {
        let mut waivers = DrcWaivers::default();
        let mut blank = waiver("DRC-FLOATING-NODE", "component:5");
        blank.reason = "  ".to_owned();
        assert_eq!(waivers.waive(blank), Err(WaiverEditRefusal::NoReason));
        let mut padded = waiver("DRC-FLOATING-NODE", "component:5");
        padded.reason = " test point ".to_owned();
        waivers.waive(padded).expect("a reason is given");
        assert_eq!(waivers.entries()[0].reason, "test point");
    }
}
