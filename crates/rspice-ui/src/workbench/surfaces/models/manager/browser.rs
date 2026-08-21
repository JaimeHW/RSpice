//! Which host this build runs on, and every sentence that depends on it.
//!
//! Two hosts, and they differ in exactly two ways the Models workspace can
//! see: where the Model Hub keeps the releases it installs — a directory under
//! local application data on the desktop, the tab's own memory in a browser —
//! and whether there is a filesystem to read a source file out of at all.
//! Everything else the workspace says about packs is true on both.
//!
//! That is why this is one small value rather than a second Models workspace.
//! The projection picks the host once, the render paints whatever it hands
//! back, and no `cfg` reaches the painting code — which is also what lets a
//! desktop test, and the raster harness, compose the browser projection and
//! look at it.
//!
//! # What is kept, and by whom
//!
//! Installed packs are written to this browser's own storage, so they are
//! there next time — but "there next time" is the browser's promise to make
//! and it makes three different ones. It may agree not to reclaim the space;
//! it may keep the bytes while reserving the right to evict them for quota, a
//! private window, or the reader clearing site data; or it may refuse storage
//! outright, in which case packs really do end with the tab. The note is
//! therefore *derived* from what the browser answered rather than asserted,
//! because a workspace that promised durability it had not been granted would
//! be lying on exactly the sessions where it matters.
//!
//! A part retained into a project is a separate question and a stronger
//! answer: it is copied into the project's own closure and saved with the
//! project file, so it opens wherever the project opens whatever the browser
//! does about storage. Every note here states both halves, because the first
//! one alone is true about the store and wrong about the reader's work.
//!
//! # It is a fact, not a warning
//!
//! Nothing here is a banner, a colour, or an adjective. A browser session that
//! behaves exactly as documented has nothing to apologise for, and a workspace
//! that apologises anyway teaches its reader to read past the lines that
//! matter.

use std::path::Path;

use crate::state::model_hub::durable::PackStorageStanding;

/// Which of the two hosts this build was compiled for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Host {
    /// A machine with a filesystem. An installed release is under local
    /// application data until something removes it.
    Desktop,
    /// A browser tab. An installed release is written to this browser's
    /// storage and re-proved on the next session that reads it back.
    Browser,
}

/// The host this build compiles against.
///
/// `cfg!` rather than two `#[cfg]` constants: both arms are compiled on both
/// targets, which keeps each variant *constructed* somewhere on every target
/// and out of the dead-code gate that `-D warnings` runs on the desktop lib and
/// on both wasm checks. The condition still folds to a constant, and the whole
/// of this build's platform knowledge is these four lines.
pub(super) const fn current_host() -> Host {
    if cfg!(target_arch = "wasm32") {
        Host::Browser
    } else {
        Host::Desktop
    }
}

impl Default for Host {
    fn default() -> Self {
        current_host()
    }
}

/// What a browser owes the reader when it has refused to keep anything.
///
/// The sentence this workspace used to print unconditionally, now printed only
/// on the sessions it is true of. Both halves are load-bearing: the first is
/// the only thing about such a session that differs from the desktop, and the
/// second is what stops the first from being read as "nothing here is saved".
pub(super) const SESSION_SCOPE_NOTE: &str = concat!(
    "Installed packs last as long as this browser session; ",
    "parts retained into a project are saved in the project file."
);

/// What a browser owes the reader when it has agreed not to evict.
///
/// It still does not say "permanently". The reader can clear site data, and a
/// promise this workspace cannot keep is worse than the smaller one it can.
pub(super) const PERSISTENT_SCOPE_NOTE: &str = concat!(
    "Installed packs are kept in this browser's storage and re-proved when they are read back; ",
    "parts retained into a project are saved in the project file."
);

/// What a browser owes the reader when it has kept the bytes and promised
/// nothing.
///
/// This is the ordinary case, and it is the one the wording exists for. The
/// packs *are* there next session, almost always — and the browser is entitled
/// to take them back for space, in a private window, or when the reader clears
/// site data, so the note names all three rather than implying none.
pub(super) const BEST_EFFORT_SCOPE_NOTE: &str = concat!(
    "Installed packs are kept in this browser's storage and re-proved when they are read back, ",
    "though the browser may still reclaim them for space, in a private window, or when site ",
    "data is cleared; parts retained into a project are saved in the project file."
);

impl Host {
    /// What this host's pack store is, in one sentence, or nothing when there
    /// is nothing to say.
    ///
    /// The desktop says nothing at all. "Installed packs stay installed" is
    /// what every desktop application already promises by existing, and a line
    /// restating it on every paint is a line a reader learns to skip — taking
    /// the browser's line with it on the day they open the same workspace in a
    /// tab.
    ///
    /// A browser says whichever of three sentences its *storage* earned. It
    /// takes the standing rather than reading it, because the projection this
    /// belongs to is composed by a desktop test and by the raster harness, and
    /// a function that reached for the real browser would make both of them
    /// paint a sentence about a browser that is not there.
    pub(super) const fn scope_note(self, standing: &PackStorageStanding) -> Option<&'static str> {
        match (self, standing) {
            (Self::Desktop, _) => None,
            (Self::Browser, PackStorageStanding::Persistent) => Some(PERSISTENT_SCOPE_NOTE),
            (Self::Browser, PackStorageStanding::BestEffort) => Some(BEST_EFFORT_SCOPE_NOTE),
            // `NotApplicable` is a browser that has not finished asking yet,
            // and it reads as the modest claim rather than the confident one:
            // a first frame that promised durability and a second frame that
            // withdrew it is worse than a first frame that promised nothing.
            (
                Self::Browser,
                PackStorageStanding::NotApplicable | PackStorageStanding::Unavailable(_),
            ) => Some(SESSION_SCOPE_NOTE),
        }
    }

    /// What a failed install leaves behind on this host.
    ///
    /// The shape is identical — expand into staging, publish under the
    /// release's name only when the whole archive proved — and only the
    /// wreckage differs, so only the wreckage is worded twice. The desktop
    /// sentence used to be printed in a browser session too, where it promised
    /// a directory that does not exist and a sweep at a start that never comes.
    pub(super) const fn install_failure_detail(self) -> &'static str {
        match self {
            Self::Desktop => {
                "An install is a staged expansion followed by a rename, so a failure — including \
                 a killed process — leaves a staging directory and nothing else, which the next \
                 start sweeps."
            }
            Self::Browser => {
                "An install is staged and published under the release's name only once the whole \
                 archive has proved, so a failure leaves an unreachable staged copy and nothing \
                 else — and nothing at all is written to this browser's storage until the \
                 release is published."
            }
        }
    }

    /// Why the source file behind one part cannot be opened here, if it cannot.
    ///
    /// Opening a card reads a file, and a browser session has no filesystem to
    /// read one out of. That used to render as a permanently greyed control
    /// with no tooltip and no node description, so a reader could see the
    /// button and never learn why it never worked. Nothing gains a capability
    /// here; the browser is told what it already was.
    pub(super) fn card_refusal(self, source: Option<&Path>) -> Option<&'static str> {
        match self {
            Self::Browser => Some(
                "Cards are opened from the source file on disk, and a browser session has no \
                 filesystem to read one out of.",
            ),
            Self::Desktop => match source {
                Some(path) if path.is_file() => None,
                Some(_) => Some("The source this card names is not on this machine."),
                None => Some("This definition records no source file to open."),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each standing gets its own sentence, and every sentence states both
    /// halves.
    ///
    /// A test that checked for "browser" would pass on a line that had lost
    /// the half about the project, which is the half that decides whether a
    /// reader believes their work is safe.
    #[test]
    fn a_browser_session_states_both_halves_of_what_it_keeps() {
        for standing in [
            PackStorageStanding::Persistent,
            PackStorageStanding::BestEffort,
            PackStorageStanding::NotApplicable,
            PackStorageStanding::Unavailable("storage is denied".to_owned()),
        ] {
            let note = Host::Browser
                .scope_note(&standing)
                .expect("a browser states what its store keeps");
            assert!(
                note.contains("parts retained into a project are saved in the project file"),
                "{standing:?} lost the half about the project: {note}"
            );
        }
    }

    /// The three sentences are three different sentences.
    ///
    /// The whole point of deriving the note from the standing is that a
    /// browser which promised nothing does not read like one that promised not
    /// to evict. Collapsing any two of them would put the derivation back to
    /// being decoration.
    #[test]
    fn a_browser_says_something_different_for_each_promise_it_was_given() {
        let granted = Host::Browser.scope_note(&PackStorageStanding::Persistent);
        let best_effort = Host::Browser.scope_note(&PackStorageStanding::BestEffort);
        let none_at_all = Host::Browser.scope_note(&PackStorageStanding::Unavailable(
            "storage is denied".to_owned(),
        ));
        assert_ne!(granted, best_effort);
        assert_ne!(best_effort, none_at_all);
        assert_ne!(granted, none_at_all);
        // And only the one that was actually granted a promise mentions
        // keeping anything without a caveat beside it.
        assert!(
            best_effort
                .expect("best effort states its caveats")
                .contains("may still reclaim")
        );
        assert!(
            none_at_all
                .expect("a denied store states the truth")
                .contains("as long as this browser session")
        );
    }

    /// A browser that has not finished asking claims the smaller thing.
    ///
    /// The first frames of a session paint before storage has answered. A note
    /// that promised durability and then withdrew it on frame three would
    /// teach the reader that the line is noise.
    #[test]
    fn a_browser_that_has_not_asked_yet_promises_nothing() {
        assert_eq!(
            Host::Browser.scope_note(&PackStorageStanding::NotApplicable),
            Some(SESSION_SCOPE_NOTE)
        );
    }

    /// The desktop says nothing, which is what keeps the browser's line worth
    /// reading.
    #[test]
    fn the_desktop_states_nothing() {
        for standing in [
            PackStorageStanding::Persistent,
            PackStorageStanding::BestEffort,
            PackStorageStanding::NotApplicable,
        ] {
            assert_eq!(Host::Desktop.scope_note(&standing), None);
        }
    }

    /// No note claims a durability it has not been granted, and none says a
    /// retained part is at risk.
    #[test]
    fn no_note_claims_a_durability_it_does_not_have() {
        for note in [
            SESSION_SCOPE_NOTE,
            PERSISTENT_SCOPE_NOTE,
            BEST_EFFORT_SCOPE_NOTE,
        ] {
            let lowered = note.to_ascii_lowercase();
            for forbidden in [
                "lost",
                "temporar",
                "permanent",
                "forever",
                "always",
                "guarantee",
                "never be removed",
            ] {
                assert!(
                    !lowered.contains(forbidden),
                    "a scope note says '{forbidden}': {note}"
                );
            }
            assert!(lowered.contains("saved in the project file"));
        }
        // The two that say packs are kept also say they are re-proved, because
        // "kept" without "re-proved" invites the reader to believe stored
        // bytes are trusted bytes.
        for kept in [PERSISTENT_SCOPE_NOTE, BEST_EFFORT_SCOPE_NOTE] {
            assert!(kept.contains("re-proved when they are read back"), "{kept}");
        }
    }

    /// A desktop build selects the desktop host, which is what makes "native
    /// renders are unchanged" a checked claim rather than an intention.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn a_desktop_build_selects_the_desktop_host() {
        assert_eq!(current_host(), Host::Desktop);
        assert_eq!(Host::default(), Host::Desktop);
        assert_eq!(
            Host::default().scope_note(&PackStorageStanding::NotApplicable),
            None
        );
    }

    /// And a browser build selects the other one, from the same four lines.
    #[cfg(target_arch = "wasm32")]
    #[test]
    fn a_browser_build_selects_the_browser_host() {
        assert_eq!(current_host(), Host::Browser);
        assert_eq!(
            Host::default().scope_note(&PackStorageStanding::BestEffort),
            Some(BEST_EFFORT_SCOPE_NOTE)
        );
    }

    /// Every way durable pack storage can disappoint a reader lands on a rung
    /// that names the right recovery.
    ///
    /// The sentences are composed down in `state`, which may not name the
    /// shell, so this is where the two halves are put together: that a
    /// discarded archive reads as `Corrupted` — the bytes claimed to be a pack
    /// and were not — and that storage refusing to open reads as `Offline`,
    /// whose consequence is the true one, that nothing on this machine changed.
    /// Neither may fall through to the generic execution error, which tells a
    /// reader to go and read a diagnostic.
    #[test]
    fn each_way_storage_can_disappoint_reaches_a_rung_that_names_the_recovery() {
        use crate::state::model_hub::durable::{HydrationReport, RejectedArchive};
        use crate::workbench::state::ModelsOperationalState as State;

        let discarded = HydrationReport {
            restored: 0,
            rejected: vec![RejectedArchive {
                digest: "0".repeat(64),
                error: crate::state::model_hub::ModelHubError::Format(
                    "the archive signature does not verify".to_owned(),
                ),
            }],
        };
        let refusal = discarded
            .refusal()
            .expect("a discarded archive is always stated");
        assert_eq!(State::from_failure(&refusal), State::Corrupted);

        let PackStorageStanding::Unavailable(reason) = PackStorageStanding::Unavailable(
            "This browser could not open storage for model packs (IndexedDB is unavailable), so \
             installed packs last only as long as this tab."
                .to_owned(),
        ) else {
            unreachable!("constructed as unavailable")
        };
        assert_eq!(State::from_failure(&reason), State::Offline);
        assert!(
            State::Offline
                .consequence()
                .contains("Nothing changed on this machine"),
            "storage refusing to open must not be read as work being lost"
        );

        // And a restore that discarded nothing states nothing, so a healthy
        // session never reaches a rung at all.
        assert!(HydrationReport::default().refusal().is_none());
    }

    /// The two hosts fail an install differently, and say so differently.
    #[test]
    fn each_host_describes_the_wreckage_it_actually_leaves() {
        assert!(
            Host::Desktop
                .install_failure_detail()
                .contains("staging directory")
        );
        assert!(
            !Host::Browser.install_failure_detail().contains("directory"),
            "a browser session has no directory to leave behind"
        );
        // The claim the mirrored store's own test proves: the durable copy is
        // written on commit and never on a stage. A failed install therefore
        // leaves this browser's storage exactly as it was, and the sentence
        // says so rather than describing memory that ends with the tab.
        assert!(
            Host::Browser
                .install_failure_detail()
                .contains("nothing at all is written to this browser's storage until the release")
        );
    }

    /// Opening a card is refused in a browser whatever the row records, and
    /// refused on the desktop for a reason that names which of the two it is.
    #[test]
    fn every_refusal_to_open_a_card_states_which_one_it_is() {
        let present = Path::new(file!());
        assert!(
            Host::Browser.card_refusal(Some(present)).is_some(),
            "there is no filesystem to open it out of, whatever the row says"
        );
        assert_eq!(
            Host::Desktop.card_refusal(None),
            Some("This definition records no source file to open.")
        );
        assert_eq!(
            Host::Desktop.card_refusal(Some(Path::new("/no/such/model.lib"))),
            Some("The source this card names is not on this machine.")
        );
    }
}
