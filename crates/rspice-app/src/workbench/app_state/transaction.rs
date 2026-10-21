//! Installing a project transaction's candidate as the live state.
//!
//! A project transaction builds its result in a clone of [`AppState`] and
//! installs it only once the whole result validates. Cloning deliberately
//! leaves out the runtime sessions the live state owns, because they answer to
//! the frame loop rather than to the project: a copied session would duplicate
//! a print, export or publish, or a file picker a draft is waiting on. So the
//! candidate is never assigned over the live state directly. That would drop
//! those sessions with the state they lived in, and the next frame would find
//! them gone and cancel the operation they own.

use super::AppState;

impl AppState {
    /// Install `candidate`, a validated project transaction built from a clone
    /// of this state.
    ///
    /// A project transaction replaces project data, never the runtime sessions
    /// a clone deliberately leaves out. Those move from the live state into the
    /// candidate first: the hardcopy session and its in-flight print, export or
    /// publish, and every reference picker a specification draft is waiting
    /// on. Everything else is the candidate's.
    pub(crate) fn commit_candidate(&mut self, mut candidate: AppState) {
        std::mem::swap(&mut candidate.dialogs.hardcopy, &mut self.dialogs.hardcopy);
        if let (Some(committed), Some(live)) = (
            candidate.ui.results.session.spec_drafts.as_mut(),
            self.ui.results.session.spec_drafts.as_mut(),
        ) && committed.len() == live.len()
        {
            for (committed, live) in committed.iter_mut().zip(live) {
                committed.adopt_reference_exchange(live);
            }
        }
        *self = candidate;
    }
}
