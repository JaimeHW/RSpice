//! Installing a project transaction's candidate as the live state.
//!
//! A project transaction builds its result in a clone of [`AppState`] and
//! installs it only once the whole result validates. Cloning deliberately
//! leaves out the runtime sessions the live state owns, because they answer to
//! the frame loop rather than to the project: a copied session would duplicate
//! a print, export or publish, a file picker a draft is waiting on, or a
//! notice not yet toasted. So the candidate is never assigned over the live
//! state directly. That would drop those sessions with the state they lived
//! in, and the next frame would find them gone and cancel the operation they
//! own, or never raise the notice.

use super::AppState;

impl AppState {
    /// Install `candidate`, a validated project transaction built from a clone
    /// of this state.
    ///
    /// A project transaction replaces project data, never the runtime sessions
    /// a clone deliberately leaves out. Those move from the live state into the
    /// candidate first: the hardcopy session and its in-flight print, export or
    /// publish, every reference picker a specification draft is waiting on,
    /// and the notices reported and not yet toasted, ahead of any the
    /// transaction reported itself. Everything else is the candidate's.
    pub(crate) fn commit_candidate(&mut self, mut candidate: AppState) {
        std::mem::swap(&mut candidate.dialogs.hardcopy, &mut self.dialogs.hardcopy);
        candidate
            .model
            .notices
            .prepend(std::mem::take(&mut self.model.notices));
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

#[cfg(test)]
mod tests {
    use super::AppState;

    /// A notice pending when the transaction began is raised once, ahead of
    /// one the transaction reported, whichever state reported it.
    #[test]
    fn a_committed_transaction_raises_every_pending_notice_once_in_order() {
        let mut live = AppState::default();
        live.report_failure("Autosave failed", "The disk is full.");
        let mut candidate = live.clone();
        assert!(
            candidate.model.notices.is_empty(),
            "a clone duplicates nothing"
        );
        candidate.report_warning("Netlist updated", "One device was renamed.");

        live.commit_candidate(candidate);

        let titles: Vec<_> = live
            .model
            .notices
            .take()
            .into_iter()
            .map(|notice| notice.title)
            .collect();
        assert_eq!(titles, ["Autosave failed", "Netlist updated"]);
    }
}
