//! Which executed deck the run-snapshot document is showing, and what it can
//! be checked to be.
//!
//! The selection is a run and a point; the deck text itself stays in the
//! session archive and is resolved from there every time it is needed, so a
//! copy can never outlive the archive holding the original and become a deck
//! nobody can say is still the one that ran.
//!
//! The verdict beside it is the answer to the only question a durable deck
//! raises: is this still what the run was authorized over. It is a real
//! recomputation over the exact retained bytes, and it is made once — when
//! the selection is made — because both of its inputs are sealed.

// Through the parent rather than `crate::workbench`: the module's edge onto
// `app_state` is being retired, and a submodule of it does not need its own.
use super::AppState;

/// The text the run-snapshot document is showing right now.
///
/// One resolution, because the document is projected from three places — when
/// it is opened, when a tab returns to it, and when the surface re-reads it —
/// and a second spelling would let one of them show a different deck than the
/// header above it names.
///
/// An executed deck this session no longer holds resolves to nothing rather
/// than to the manual baseline. Silently substituting a different document
/// under the same header is the one outcome worse than an empty viewer.
pub(crate) fn run_snapshot_source(state: &AppState) -> String {
    let Some(selection) = state.ui.netlist.executed_deck_view else {
        return state.ui.netlist.last_run_buffer.clone().unwrap_or_default();
    };
    state
        .simulation
        .executed_decks
        .get(selection.run_id)
        .and_then(|deck| deck.point(selection.point))
        .map(|point| point.deck.to_string())
        .unwrap_or_default()
}

/// What a retained executed deck can honestly claim about itself.
///
/// Only one of these is a claim of verification, and it is only ever reached
/// by recomputing a digest over the exact bytes on screen. The rest name, as
/// precisely as the receipt allows, why no such claim can be made — because
/// naming the reason is the difference between a deck a reader can rely on and
/// one they merely have.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutedDeckVerification {
    /// Recomputing the receipt's own sealed-source digest over these exact
    /// bytes reproduced the digest the receipt carries. This is the deck the
    /// run was authorized over.
    Verified,
    /// The receipt seals one run-level source and another retained point of
    /// this run reproduces it, so the run's sealed source is intact and this
    /// point is a per-point variant of it — a corner's materialized process
    /// models, an expanded parameter, an overridden supply. No receipt field
    /// digests a per-point source on its own, so nothing here can be checked.
    PointVariant,
    /// A receipt seals a source and nothing retained for this run reproduces
    /// it. Whatever this deck is, the run was not authorized over it.
    Unmatched,
    /// The run carries no receipt digest that covers deck text at all: history
    /// recorded before prepared receipts, or before a run stated which kind of
    /// source it executed.
    NotRecorded,
}

impl ExecutedDeckVerification {
    /// Recompute the run receipt's sealed source digest over one point's exact
    /// retained bytes.
    ///
    /// `source_content_digest` is the one receipt field that authenticates
    /// deck text — see [`crate::simulation::execution::sealed_executable_source_digest`]
    /// — so it is the one thing a retained deck can be checked against.
    #[must_use]
    pub fn of(state: &AppState, run_id: u64, point: usize) -> Option<Self> {
        let record = state.simulation.executed_decks.get(run_id)?;
        let selected = record.point(point)?;
        let Some(receipt) = state
            .simulation
            .run_by_sequence(run_id)
            .and_then(crate::state::SimulationRun::prepared_receipt)
        else {
            return Some(Self::NotRecorded);
        };
        let domain = receipt.source_domain();
        let sealed = receipt.source_content_digest();
        let recomputed =
            crate::simulation::execution::sealed_executable_source_digest(domain, &selected.deck);
        let Some(recomputed) = recomputed else {
            return Some(Self::NotRecorded);
        };
        if recomputed == sealed {
            return Some(Self::Verified);
        }
        // Points that solved one source share one allocation, so a sweep whose
        // points all ran the run-level deck verbatim is one hash, not one per
        // point. The scan also stops at the first match.
        let mut hashed: Vec<*const u8> = Vec::new();
        let run_source_retained = record.points.iter().any(|other| {
            let identity = std::ptr::from_ref::<str>(other.deck.as_ref()).cast::<u8>();
            if hashed.contains(&identity) {
                return false;
            }
            hashed.push(identity);
            crate::simulation::execution::sealed_executable_source_digest(domain, &other.deck)
                == Some(sealed)
        });
        Some(if run_source_retained {
            Self::PointVariant
        } else {
            Self::Unmatched
        })
    }
}

/// One point of one completed run, as the executed-deck viewer shows it.
///
/// A run and a point, and nothing else. The deck text itself is looked up in
/// the session archive every time it is needed rather than copied here: a
/// second copy could outlive the archive that holds the first, and would then
/// be a deck nobody could say was still the one that ran.
///
/// The verdict is the one thing carried rather than re-derived, and it is
/// carried because it was *earned*: it is the result of a real recomputation
/// performed when this selection was made. Redoing it every frame would hash
/// a megabyte of deck sixty times a second to answer a question whose inputs
/// — immutable bytes, an immutable receipt — cannot have changed since.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExecutedDeckSelection {
    pub run_id: u64,
    pub point: usize,
    pub verification: ExecutedDeckVerification,
}
