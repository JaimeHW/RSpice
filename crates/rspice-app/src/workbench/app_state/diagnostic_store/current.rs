//! The shared current finding, kept on its finding when a source editor
//! revalidates.
//!
//! A store record keeps its identity across a re-check, because the identity
//! is structural, and the store re-anchors the rare one that is re-minted as
//! it republishes. The netlist and code editors are different on both
//! counts: they mint an identity from the buffer revision and the byte range,
//! so every revalidation re-mints every finding, and they replace their
//! collections without the store seeing it happen. So the store remembers
//! what the current finding is about while it still resolves, and when it
//! stops resolving, stands on the finding that is about the same thing.

use std::sync::Arc;

use super::FindingKey;
use crate::workbench::app_state::AppState;
use crate::workbench::documents::code_workspace::CodeDiagnosticCollection;
use crate::workbench::documents::netlist_document::NetlistDiagnosticCollection;

/// One source editor's collection of findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SourceSegment {
    Netlist,
    VerilogA,
    VerilogAFailure,
    Automation,
}

impl SourceSegment {
    const ALL: [Self; 4] = [
        Self::Netlist,
        Self::VerilogA,
        Self::VerilogAFailure,
        Self::Automation,
    ];
}

/// Where the current finding was last seen among the source editors'
/// findings, and what it is about.
///
/// The position makes "is it still there" one lookup a frame rather than a
/// walk of a collection that can hold a million findings; the key is what
/// finds it again once it is not.
#[derive(Debug, Clone)]
pub(super) struct SourceAnchor {
    segment: SourceSegment,
    index: usize,
    key: FindingKey,
}

/// Enough about the source collections to tell that one has changed: which
/// allocation each is, and the netlist buffer's revision, which moves on
/// every edit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct SourceFingerprint {
    collections: [usize; 4],
    netlist_revision: u64,
}

/// One source editor's findings, as the reconciliation reads them.
enum Records<'a> {
    Netlist(&'a NetlistDiagnosticCollection),
    Code(&'a CodeDiagnosticCollection),
}

impl Records<'_> {
    fn len(&self) -> usize {
        match self {
            Self::Netlist(collection) => collection.len(),
            Self::Code(collection) => collection.len(),
        }
    }

    fn id(&self, index: usize) -> Option<uuid::Uuid> {
        match self {
            Self::Netlist(collection) => collection
                .get(index)
                .map(|diagnostic| diagnostic.canonical.diagnostic_id),
            Self::Code(collection) => collection
                .as_slice()
                .get(index)
                .map(|diagnostic| diagnostic.canonical.diagnostic_id),
        }
    }

    /// What the finding at `index` is about: who reported it, the rule, the
    /// document, and what it says. A source finding names no object, and its
    /// position is exactly what an edit moves, so its wording is the nearest
    /// thing it has to one.
    fn key(&self, index: usize) -> Option<FindingKey> {
        match self {
            Self::Netlist(collection) => collection.get(index).map(|diagnostic| {
                FindingKey::new(
                    Arc::clone(&diagnostic.canonical.source),
                    Arc::clone(&diagnostic.canonical.code),
                    Arc::clone(&diagnostic.canonical.document_id),
                    diagnostic.message.as_str(),
                )
            }),
            Self::Code(collection) => collection.as_slice().get(index).map(|diagnostic| {
                FindingKey::new(
                    Arc::clone(&diagnostic.canonical.source),
                    Arc::clone(&diagnostic.canonical.code),
                    Arc::clone(&diagnostic.canonical.document_id),
                    Arc::clone(&diagnostic.message),
                )
            }),
        }
    }

    /// Whether the finding at `index` is about what `key` names, without
    /// building a key to compare.
    fn matches(&self, index: usize, key: &FindingKey) -> bool {
        match self {
            Self::Netlist(collection) => collection.get(index).is_some_and(|diagnostic| {
                diagnostic.canonical.source == key.producer
                    && diagnostic.canonical.code == key.code
                    && diagnostic.canonical.document_id == key.document
                    && *diagnostic.message == *key.object
            }),
            Self::Code(collection) => {
                collection.as_slice().get(index).is_some_and(|diagnostic| {
                    diagnostic.canonical.source == key.producer
                        && diagnostic.canonical.code == key.code
                        && diagnostic.canonical.document_id == key.document
                        && diagnostic.message == key.object
                })
            }
        }
    }
}

fn records(state: &AppState, segment: SourceSegment) -> Option<Records<'_>> {
    let code = &state.ui.code_workspace;
    match segment {
        SourceSegment::Netlist => Some(Records::Netlist(state.ui.netlist.diagnostics.as_ref())),
        SourceSegment::VerilogA => code
            .veriloga
            .receipt
            .as_ref()
            .map(|receipt| Records::Code(receipt.diagnostics.as_ref())),
        SourceSegment::VerilogAFailure => Some(Records::Code(code.veriloga.last_failure.as_ref())),
        SourceSegment::Automation => Some(Records::Code(code.automation.diagnostics.as_ref())),
    }
}

fn fingerprint(state: &AppState) -> SourceFingerprint {
    let code = &state.ui.code_workspace;
    let address = |collection: Option<&Arc<CodeDiagnosticCollection>>| {
        collection.map_or(0, |collection| Arc::as_ptr(collection) as usize)
    };
    SourceFingerprint {
        collections: [
            Arc::as_ptr(&state.ui.netlist.diagnostics) as usize,
            address(
                code.veriloga
                    .receipt
                    .as_ref()
                    .map(|receipt| &receipt.diagnostics),
            ),
            address(Some(&code.veriloga.last_failure)),
            address(Some(&code.automation.diagnostics)),
        ],
        netlist_revision: state.ui.netlist.revision,
    }
}

impl AppState {
    /// Keep the shared current finding on its finding after a source editor
    /// revalidated.
    ///
    /// Asked once a frame. While the current finding still resolves this is
    /// one lookup; once it does not, the source collections are searched for
    /// it, then for the finding about the same thing, and a search that finds
    /// neither is not repeated until a collection changes.
    pub(in crate::workbench) fn reconcile_current_finding(&mut self) {
        let Some(current) = self.diagnostics.current else {
            return;
        };
        // A store record is re-anchored by the store as it republishes.
        if self.diagnostics.owners.contains_key(&current) {
            return;
        }
        if let Some(anchor) = self.diagnostics.source_anchor.as_ref()
            && records(self, anchor.segment).and_then(|records| records.id(anchor.index))
                == Some(current)
        {
            return;
        }
        let fingerprint = fingerprint(self);
        if self.diagnostics.unresolved == Some((current, fingerprint)) {
            return;
        }

        let found = find(self, |records, index| records.id(index) == Some(current));
        if let Some((segment, index)) = found {
            self.diagnostics.source_anchor = records(self, segment)
                .and_then(|records| records.key(index))
                .map(|key| SourceAnchor {
                    segment,
                    index,
                    key,
                });
            self.diagnostics.unresolved = None;
            return;
        }

        let reanchored = self.diagnostics.source_anchor.as_ref().and_then(|anchor| {
            find(self, |records, index| records.matches(index, &anchor.key))
                .and_then(|(segment, index)| {
                    let id = records(self, segment)?.id(index)?;
                    Some((segment, index, id, anchor.key.clone()))
                })
        });
        match reanchored {
            Some((segment, index, id, key)) => {
                self.diagnostics.current = Some(id);
                self.diagnostics.source_anchor = Some(SourceAnchor {
                    segment,
                    index,
                    key,
                });
                self.diagnostics.unresolved = None;
            }
            None => self.diagnostics.unresolved = Some((current, fingerprint)),
        }
    }
}

/// The first source finding `wanted` accepts, by collection and position.
fn find(
    state: &AppState,
    wanted: impl Fn(&Records<'_>, usize) -> bool,
) -> Option<(SourceSegment, usize)> {
    SourceSegment::ALL.into_iter().find_map(|segment| {
        let records = records(state, segment)?;
        (0..records.len())
            .find(|&index| wanted(&records, index))
            .map(|index| (segment, index))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::documents::netlist_document::Diagnostic;

    fn collection(diagnostics: Vec<Diagnostic>) -> Arc<NetlistDiagnosticCollection> {
        Arc::new(
            NetlistDiagnosticCollection::try_new(diagnostics, "nchh\nr1 a b 1k\n")
                .expect("the fixture collection is well formed"),
        )
    }

    /// A netlist finding's identity names the buffer revision and its byte
    /// range, so an edit above it re-mints it. The current finding follows
    /// it to its new identity rather than being lost.
    #[test]
    fn the_current_netlist_finding_survives_a_revalidation_that_re_mints_it() {
        let mut state = AppState::default();
        let first = Diagnostic::error("unknown model nchh").with_line(Some(0));
        let other = Diagnostic::error("missing node").with_line(Some(1));
        let selected = first.canonical.diagnostic_id;
        state.ui.netlist.diagnostics = collection(vec![first, other]);
        state.diagnostics.set_current_finding(Some(selected));
        state.reconcile_current_finding();
        assert_eq!(state.diagnostics.current_finding(), Some(selected));

        // An edit above it moves it down a line: same finding, new identity.
        let moved = Diagnostic::error("unknown model nchh").with_line(Some(1));
        let renamed = moved.canonical.diagnostic_id;
        assert_ne!(renamed, selected, "the fixture re-mints the identity");
        state.ui.netlist.diagnostics = collection(vec![
            Diagnostic::error("missing node").with_line(Some(2)),
            moved,
        ]);
        state.reconcile_current_finding();

        assert_eq!(state.diagnostics.current_finding(), Some(renamed));
    }

    /// A finding that was fixed has nothing to follow, and the current
    /// selection is not moved onto another finding.
    #[test]
    fn a_fixed_netlist_finding_hands_the_selection_to_nothing() {
        let mut state = AppState::default();
        let fixed = Diagnostic::error("unknown model nchh").with_line(Some(0));
        let selected = fixed.canonical.diagnostic_id;
        state.ui.netlist.diagnostics = collection(vec![
            fixed,
            Diagnostic::error("missing node").with_line(Some(1)),
        ]);
        state.diagnostics.set_current_finding(Some(selected));
        state.reconcile_current_finding();

        state.ui.netlist.diagnostics =
            collection(vec![Diagnostic::error("missing node").with_line(Some(0))]);
        state.reconcile_current_finding();

        assert_eq!(state.diagnostics.current_finding(), Some(selected));
    }
}
