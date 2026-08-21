//! What one release changes about another, as the signed catalog states it.
//!
//! A reader deciding whether to move a project onto a newer release is asking
//! one question — *what is different* — and there is exactly one trustworthy
//! answer to it: the two release entries in the snapshot this client verified.
//! Everything here is a comparison of those two records and nothing else. No
//! heuristic, no inference from a version number, no guess about what a
//! publisher probably did.
//!
//! # A fact the catalog cannot state is absent, not invented
//!
//! Snapshot schema 2 publishes, per release, a set of parts each carrying an
//! identifier, a kind, a device family, aliases, terminals, an optional symbol
//! contract, an optional one-line description and up to six authored
//! specifications — and, per release, one archive digest. That is the whole
//! comparable surface. In particular there is still **no per-part digest**, so
//! two releases that list a part identically may ship different model source
//! for it, and this module refuses to call such a part unchanged. It counts
//! them as *re-listed* and reports [`ReleaseDiff::archive_differs`] beside the
//! count, which together say exactly what is known: the documents differ, and
//! the catalog does not say where.
//!
//! # The seam that caught schema 2
//!
//! [`part_facts`] destructures both [`SnapshotPart`] values by naming every
//! field. That is deliberate, and it worked: the schema-2 vendor sync stopped
//! this file compiling until `description` and `specs` were each compared here.
//! The same holds for whatever comes next — a per-part digest, a parameter
//! table — which must be either compared or explicitly declined at this one
//! function. A diff that silently ignored a new fact would be the one failure
//! mode a "what changed" projection cannot have.

use std::collections::BTreeMap;

use rspice_pack::{PartKind, SnapshotPart, SnapshotRelease, Symbol};

/// The exact question one diff answers: read from which catalog, about which
/// pack, between which two releases.
///
/// The catalog half is the digest of the snapshot bytes this client proved —
/// a content key, never a generation counter. Two different catalogs that
/// happen to publish the same two version strings are two different questions,
/// and a cache keyed on the versions alone would answer the second with the
/// first one's result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseDiffKey {
    pub catalog_digest: String,
    pub pack_id: String,
    /// The release the reader holds or pinned.
    pub from: String,
    /// The release on offer.
    pub to: String,
}

/// What the catalog states one release changes about another.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseDiff {
    /// The question this answers. A consumer caching a diff compares this
    /// rather than re-deriving what it was computed for.
    pub key: ReleaseDiffKey,
    /// Parts the newer release publishes and the older one did not, by id.
    pub added: Vec<String>,
    /// Parts the older release published and the newer one does not, by id.
    ///
    /// A project holding one of these keeps its retained bytes and keeps
    /// solving; what it cannot do is adopt the part from a release that no
    /// longer publishes it.
    pub removed: Vec<String>,
    /// Parts both releases publish, whose catalog listing differs.
    pub changed: Vec<ChangedPart>,
    /// Parts both releases list identically, field for field.
    ///
    /// Not "unchanged": see the module header. The catalog publishes no
    /// per-part digest, so this is a count of parts the catalog says nothing
    /// different about, which is a weaker claim and the only true one.
    pub relisted: usize,
    /// Capabilities the newer release requires and the older one did not.
    pub capabilities_added: Vec<String>,
    pub capabilities_removed: Vec<String>,
    /// `(older, newer)` when the two releases publish different licences.
    pub licence: Option<(String, String)>,
    /// Whether the two releases are different archives at all.
    ///
    /// Taken from the signed `archive_sha256` of each. It is what makes the
    /// re-listed count honest: different documents, and no published fact
    /// saying which parts inside them moved.
    pub archive_differs: bool,
}

impl ReleaseDiff {
    /// Whether the catalog states any difference at all between the two
    /// releases beyond their archives being distinct documents.
    pub fn is_empty(&self) -> bool {
        self.added.is_empty()
            && self.removed.is_empty()
            && self.changed.is_empty()
            && self.capabilities_added.is_empty()
            && self.capabilities_removed.is_empty()
            && self.licence.is_none()
    }

    /// What the newer release states about one part the project pinned.
    pub fn part_standing(&self, part_id: &str) -> PartStanding<'_> {
        if self.removed.iter().any(|id| id == part_id) {
            return PartStanding::Withdrawn;
        }
        match self.changed.iter().find(|part| part.part_id == part_id) {
            Some(part) => PartStanding::Changed(part),
            None => PartStanding::Relisted,
        }
    }
}

/// Where one pinned part stands in the newer release.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartStanding<'a> {
    /// The newer release does not publish it.
    Withdrawn,
    /// It is published, and the catalog states these differences.
    Changed(&'a ChangedPart),
    /// It is published, and the catalog states no difference — which is not
    /// the same as the model source being the same bytes.
    Relisted,
}

/// One part both releases publish, and every difference the catalog states.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangedPart {
    pub part_id: String,
    pub facts: Vec<PartFact>,
}

/// One published fact about a part that the two releases state differently.
///
/// Every variant names both sides, because "the terminals changed" is not a
/// diff — it is a rumour about one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PartFact {
    /// A model card became a subcircuit, or the reverse.
    Kind { from: PartKind, to: PartKind },
    /// The device family the part is filed under.
    Device { from: String, to: String },
    /// Alternative names the part answers to.
    Aliases {
        added: Vec<String>,
        removed: Vec<String>,
    },
    /// The terminal order a caller connects to. A change here is a change to
    /// every instance that already binds the part.
    Terminals { from: Vec<String>, to: Vec<String> },
    /// The symbol the part is drawn as, or its pin map.
    Symbol {
        from: Option<SymbolFacts>,
        to: Option<SymbolFacts>,
    },
    /// The publisher's one-line summary. Either side may be absent, which is
    /// the publisher offering no summary rather than withdrawing the part.
    Description {
        from: Option<String>,
        to: Option<String>,
    },
    /// One authored specification, named with the key it is filed under.
    ///
    /// One fact per key rather than one fact for the whole map: a reader
    /// deciding whether to move a project asks about a number — the breakdown
    /// voltage, the gain-bandwidth — and a single "specs changed" line would
    /// make them open the two releases to find out which. A part publishes at
    /// most six, so the expansion is bounded by the format.
    Spec {
        key: String,
        from: Option<String>,
        to: Option<String>,
    },
}

impl PartFact {
    /// The fact as a reader reads it, with both sides named.
    pub fn describe(&self) -> String {
        match self {
            Self::Kind { from, to } => {
                format!("kind {} → {}", kind_word(*from), kind_word(*to))
            }
            Self::Device { from, to } => format!("device {from} → {to}"),
            // ASCII `+`/`-`, the vocabulary every reader already knows a diff
            // by, and the pair certain to paint at any size on any face.
            Self::Aliases { added, removed } => {
                let mut phrase = String::from("aliases");
                if !added.is_empty() {
                    phrase.push_str(&format!(" +{}", added.join(" +")));
                }
                if !removed.is_empty() {
                    phrase.push_str(&format!(" -{}", removed.join(" -")));
                }
                phrase
            }
            Self::Terminals { from, to } => {
                format!("terminals ({}) → ({})", from.join(" "), to.join(" "))
            }
            Self::Symbol { from, to } => match (from, to) {
                (None, Some(to)) => format!("symbol added: {}", to.reference),
                (Some(from), None) => format!("symbol {} withdrawn", from.reference),
                (Some(from), Some(to)) if from.reference != to.reference => {
                    format!("symbol {} → {}", from.reference, to.reference)
                }
                (Some(from), Some(_)) => format!("symbol {} re-pinned", from.reference),
                // `part_facts` never emits an unchanged symbol.
                (None, None) => "symbol".to_owned(),
            },
            Self::Description { from, to } => sided("description", None, from, to),
            Self::Spec { key, from, to } => sided("spec", Some(key), from, to),
        }
    }
}

/// One optional value's change, with whichever sides exist named.
///
/// Both facts this serves are `Option<String>` pairs whose absent side is a
/// publisher declining to say something, not a withdrawal of the part — so
/// "added" and "withdrawn" are the honest words for the two one-sided cases,
/// and the two-sided case names both values rather than only the new one.
fn sided(noun: &str, key: Option<&str>, from: &Option<String>, to: &Option<String>) -> String {
    let named = match key {
        Some(key) => format!("{noun} {key}"),
        None => noun.to_owned(),
    };
    match (from.as_deref(), to.as_deref()) {
        (None, Some(to)) => format!("{named} added: {to}"),
        (Some(from), None) => format!("{named} withdrawn: {from}"),
        (Some(from), Some(to)) => format!("{named} {from} → {to}"),
        // `part_facts` never emits a fact both releases are silent about.
        (None, None) => named,
    }
}

/// A symbol contract, flattened to the two things a diff compares.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolFacts {
    pub reference: String,
    /// Terminal to symbol pin, as the manifest binds them.
    pub pins: BTreeMap<String, String>,
}

impl SymbolFacts {
    fn of(symbol: &Symbol) -> Self {
        let Symbol { reference, pins } = symbol;
        Self {
            reference: reference.clone(),
            pins: pins.clone(),
        }
    }
}

const fn kind_word(kind: PartKind) -> &'static str {
    match kind {
        PartKind::Model => "model card",
        PartKind::Subckt => "subcircuit",
    }
}

/// Compares the two release records the catalog publishes.
///
/// Both sides come from one verified snapshot, so this is a comparison of
/// signed statements rather than of anything a client derived.
pub fn release_diff(
    key: ReleaseDiffKey,
    from: &SnapshotRelease,
    to: &SnapshotRelease,
) -> ReleaseDiff {
    let older: BTreeMap<&str, &SnapshotPart> = from
        .parts
        .iter()
        .map(|part| (part.id.as_str(), part))
        .collect();
    let newer: BTreeMap<&str, &SnapshotPart> = to
        .parts
        .iter()
        .map(|part| (part.id.as_str(), part))
        .collect();

    let added = newer
        .keys()
        .filter(|id| !older.contains_key(*id))
        .map(|id| (*id).to_owned())
        .collect();
    let removed = older
        .keys()
        .filter(|id| !newer.contains_key(*id))
        .map(|id| (*id).to_owned())
        .collect();

    let mut changed = Vec::new();
    let mut relisted = 0;
    for (id, old_part) in &older {
        let Some(new_part) = newer.get(id) else {
            continue;
        };
        let facts = part_facts(old_part, new_part);
        if facts.is_empty() {
            relisted += 1;
        } else {
            changed.push(ChangedPart {
                part_id: (*id).to_owned(),
                facts,
            });
        }
    }

    ReleaseDiff {
        key,
        added,
        removed,
        changed,
        relisted,
        capabilities_added: missing_from(&to.capabilities, &from.capabilities),
        capabilities_removed: missing_from(&from.capabilities, &to.capabilities),
        licence: (from.spdx != to.spdx).then(|| (from.spdx.clone(), to.spdx.clone())),
        archive_differs: from.archive_sha256 != to.archive_sha256,
    }
}

/// Values in `values` that `other` does not carry, in the catalog's order.
fn missing_from(values: &[String], other: &[String]) -> Vec<String> {
    values
        .iter()
        .filter(|value| !other.contains(value))
        .cloned()
        .collect()
}

/// Every published fact the two listings of one part state differently.
///
/// Both parts are destructured by naming every field. A schema that grows one
/// stops this function compiling, which is the point: a new published fact is
/// either compared here or explicitly declined here, never dropped silently.
fn part_facts(from: &SnapshotPart, to: &SnapshotPart) -> Vec<PartFact> {
    let SnapshotPart {
        // Equal by construction: the caller matched these parts by identifier.
        id: _,
        kind: from_kind,
        device: from_device,
        aliases: from_aliases,
        terminals: from_terminals,
        symbol: from_symbol,
        description: from_description,
        specs: from_specs,
    } = from;
    let SnapshotPart {
        id: _,
        kind: to_kind,
        device: to_device,
        aliases: to_aliases,
        terminals: to_terminals,
        symbol: to_symbol,
        description: to_description,
        specs: to_specs,
    } = to;

    let mut facts = Vec::new();
    if from_kind != to_kind {
        facts.push(PartFact::Kind {
            from: *from_kind,
            to: *to_kind,
        });
    }
    if from_device != to_device {
        facts.push(PartFact::Device {
            from: from_device.clone(),
            to: to_device.clone(),
        });
    }
    let aliases_added = missing_from(to_aliases, from_aliases);
    let aliases_removed = missing_from(from_aliases, to_aliases);
    if !aliases_added.is_empty() || !aliases_removed.is_empty() {
        facts.push(PartFact::Aliases {
            added: aliases_added,
            removed: aliases_removed,
        });
    }
    if from_terminals != to_terminals {
        facts.push(PartFact::Terminals {
            from: from_terminals.clone(),
            to: to_terminals.clone(),
        });
    }
    if from_symbol != to_symbol {
        facts.push(PartFact::Symbol {
            from: from_symbol.as_ref().map(SymbolFacts::of),
            to: to_symbol.as_ref().map(SymbolFacts::of),
        });
    }
    if from_description != to_description {
        facts.push(PartFact::Description {
            from: from_description.clone(),
            to: to_description.clone(),
        });
    }
    // Both maps are walked, not just the newer one: a specification the newer
    // release stopped publishing is a change a reader has to be told about, and
    // iterating only `to_specs` would report exactly the additions and silently
    // drop every withdrawal. `BTreeMap` order makes the union deterministic, so
    // two clients reading one catalog produce the same list of facts.
    for key in from_specs.keys().chain(to_specs.keys()) {
        let (before, after) = (from_specs.get(key), to_specs.get(key));
        if before == after || facts.iter().any(|fact| names_spec(fact, key)) {
            continue;
        }
        facts.push(PartFact::Spec {
            key: key.clone(),
            from: before.cloned(),
            to: after.cloned(),
        });
    }
    facts
}

/// Whether a fact already reported this specification key.
///
/// The union above visits a key present in both maps twice, and a part that
/// changed one value would otherwise carry the same fact twice.
fn names_spec(fact: &PartFact, key: &str) -> bool {
    matches!(fact, PartFact::Spec { key: named, .. } if named == key)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key() -> ReleaseDiffKey {
        ReleaseDiffKey {
            catalog_digest: "ab".repeat(32),
            pack_id: "rspice-proving".to_owned(),
            from: "1.0.0".to_owned(),
            to: "1.1.0".to_owned(),
        }
    }

    fn part(id: &str) -> SnapshotPart {
        SnapshotPart {
            id: id.to_owned(),
            kind: PartKind::Subckt,
            device: "network".to_owned(),
            aliases: Vec::new(),
            terminals: vec!["IN".to_owned(), "OUT".to_owned()],
            symbol: None,
            description: None,
            specs: BTreeMap::new(),
        }
    }

    fn release(version: &str, digest: &str, parts: Vec<SnapshotPart>) -> SnapshotRelease {
        SnapshotRelease {
            version: version.to_owned(),
            archive_sha256: digest.repeat(32),
            archive_length: 4_096,
            capabilities: vec!["subckt".to_owned()],
            spdx: "LicenseRef-RSpice-Models".to_owned(),
            parts,
        }
    }

    #[test]
    fn a_part_list_diff_names_what_arrived_what_left_and_what_stayed() {
        let from = release("1.0.0", "aa", vec![part("KEPT"), part("GONE")]);
        let to = release("1.1.0", "bb", vec![part("KEPT"), part("NEW")]);
        let diff = release_diff(key(), &from, &to);
        assert_eq!(diff.added, vec!["NEW".to_owned()]);
        assert_eq!(diff.removed, vec!["GONE".to_owned()]);
        assert!(diff.changed.is_empty());
        assert_eq!(diff.relisted, 1);
        assert!(diff.archive_differs);
        assert!(!diff.is_empty());
    }

    #[test]
    fn every_published_part_field_is_compared_and_names_both_sides() {
        let mut before = part("P");
        before.aliases = vec!["OLD".to_owned()];
        before.symbol = Some(Symbol {
            reference: "sym/old".to_owned(),
            pins: BTreeMap::from([("IN".to_owned(), "1".to_owned())]),
        });
        let mut after = part("P");
        after.kind = PartKind::Model;
        after.device = "diode".to_owned();
        after.aliases = vec!["NEW".to_owned()];
        after.terminals = vec!["A".to_owned(), "K".to_owned()];
        after.symbol = Some(Symbol {
            reference: "sym/new".to_owned(),
            pins: BTreeMap::from([("A".to_owned(), "1".to_owned())]),
        });

        let diff = release_diff(
            key(),
            &release("1.0.0", "aa", vec![before]),
            &release("1.1.0", "bb", vec![after]),
        );
        assert_eq!(diff.relisted, 0);
        let changed = diff
            .changed
            .first()
            .expect("the part is published by both releases");
        assert_eq!(changed.part_id, "P");
        let phrases = changed
            .facts
            .iter()
            .map(PartFact::describe)
            .collect::<Vec<_>>();
        assert_eq!(
            phrases,
            vec![
                "kind subcircuit → model card".to_owned(),
                "device network → diode".to_owned(),
                "aliases +NEW -OLD".to_owned(),
                "terminals (IN OUT) → (A K)".to_owned(),
                "symbol sym/old → sym/new".to_owned(),
            ]
        );
    }

    /// The two facts snapshot schema 2 added, each naming both sides.
    ///
    /// A withdrawn specification is the case a one-sided walk would drop: the
    /// newer release simply stops publishing the key, and reporting only what
    /// arrived would tell a reader nothing had gone.
    #[test]
    fn a_description_or_specification_change_names_both_sides() {
        let mut before = part("P");
        before.description = Some("Low-noise dual op-amp".to_owned());
        before.specs = BTreeMap::from([
            ("gbw".to_owned(), "3 MHz".to_owned()),
            ("iq".to_owned(), "1.4 mA".to_owned()),
            ("supply".to_owned(), "+/-15 V".to_owned()),
        ]);
        let mut after = part("P");
        after.description = Some("Low-noise JFET-input dual op-amp".to_owned());
        after.specs = BTreeMap::from([
            ("gbw".to_owned(), "4 MHz".to_owned()),
            ("supply".to_owned(), "+/-15 V".to_owned()),
            ("vos".to_owned(), "3 mV".to_owned()),
        ]);

        let diff = release_diff(
            key(),
            &release("1.0.0", "aa", vec![before]),
            &release("1.1.0", "bb", vec![after]),
        );
        assert_eq!(diff.relisted, 0);
        let changed = diff.changed.first().expect("both releases publish it");
        assert_eq!(
            changed
                .facts
                .iter()
                .map(PartFact::describe)
                .collect::<Vec<_>>(),
            vec![
                "description Low-noise dual op-amp → Low-noise JFET-input dual op-amp".to_owned(),
                "spec gbw 3 MHz → 4 MHz".to_owned(),
                "spec iq withdrawn: 1.4 mA".to_owned(),
                "spec vos added: 3 mV".to_owned(),
            ],
            "a key present in both is reported once, and neither side is dropped"
        );
    }

    /// Publishing a summary or a specification for the first time is a change,
    /// and so is stopping.
    #[test]
    fn newly_published_and_newly_silent_presentation_are_both_reported() {
        let bare = part("P");
        let mut described = part("P");
        described.description = Some("Zener reference".to_owned());
        described.specs = BTreeMap::from([("vz".to_owned(), "5.1 V".to_owned())]);

        let arriving = release_diff(
            key(),
            &release("1.0.0", "aa", vec![bare.clone()]),
            &release("1.1.0", "bb", vec![described.clone()]),
        );
        assert_eq!(
            arriving.changed[0]
                .facts
                .iter()
                .map(PartFact::describe)
                .collect::<Vec<_>>(),
            vec![
                "description added: Zener reference".to_owned(),
                "spec vz added: 5.1 V".to_owned(),
            ]
        );

        let leaving = release_diff(
            key(),
            &release("1.0.0", "aa", vec![described]),
            &release("1.1.0", "bb", vec![bare]),
        );
        assert_eq!(
            leaving.changed[0]
                .facts
                .iter()
                .map(PartFact::describe)
                .collect::<Vec<_>>(),
            vec![
                "description withdrawn: Zener reference".to_owned(),
                "spec vz withdrawn: 5.1 V".to_owned(),
            ]
        );
    }

    /// Identical listings and different archives is the case the projection
    /// must not overstate: the catalog says nothing changed about this part,
    /// and the catalog does not publish enough to say that it did not.
    #[test]
    fn a_relisted_part_is_never_reported_as_unchanged_bytes() {
        let from = release("1.0.0", "aa", vec![part("P")]);
        let to = release("1.1.0", "bb", vec![part("P")]);
        let diff = release_diff(key(), &from, &to);
        assert_eq!(diff.relisted, 1);
        assert!(diff.changed.is_empty());
        assert!(diff.is_empty(), "the catalog states no difference");
        assert!(
            diff.archive_differs,
            "and the archives are still different documents"
        );
        assert_eq!(diff.part_standing("P"), PartStanding::Relisted);
    }

    #[test]
    fn a_capability_or_licence_change_is_reported_with_both_sides() {
        let mut from = release("1.0.0", "aa", vec![part("P")]);
        from.capabilities = vec!["subckt".to_owned(), "resistor".to_owned()];
        let mut to = release("1.1.0", "bb", vec![part("P")]);
        to.capabilities = vec!["subckt".to_owned(), "veriloga".to_owned()];
        to.spdx = "LicenseRef-RSpice-Models-Restricted".to_owned();

        let diff = release_diff(key(), &from, &to);
        assert_eq!(diff.capabilities_added, vec!["veriloga".to_owned()]);
        assert_eq!(diff.capabilities_removed, vec!["resistor".to_owned()]);
        assert_eq!(
            diff.licence,
            Some((
                "LicenseRef-RSpice-Models".to_owned(),
                "LicenseRef-RSpice-Models-Restricted".to_owned()
            ))
        );
    }

    #[test]
    fn a_withdrawn_part_is_told_apart_from_one_that_is_merely_re_listed() {
        let from = release("1.0.0", "aa", vec![part("KEPT"), part("GONE")]);
        let to = release("1.1.0", "bb", vec![part("KEPT")]);
        let diff = release_diff(key(), &from, &to);
        assert_eq!(diff.part_standing("GONE"), PartStanding::Withdrawn);
        assert_eq!(diff.part_standing("KEPT"), PartStanding::Relisted);
    }
}
