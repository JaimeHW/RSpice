//! Which store this build's Model Hub keeps packs in, and what that means.
//!
//! The desktop keeps installed releases in a directory under local application
//! data; a browser session keeps them in the tab's own memory. Every other
//! sentence the workspace says about packs is true on both, which is why this
//! is one small value rather than a second Models workspace: the projection
//! picks the store once, the render paints whatever it hands back, and no
//! `cfg` reaches the painting code.
//!
//! # What is session-scoped, and what is not
//!
//! Only the *pack store* is. A part retained into a project is copied into the
//! project's own closure and saved with the project file, so it opens wherever
//! the project opens and is unaffected by the tab closing. Saying "your packs
//! are temporary" without that second half would be true about the store and
//! wrong about the reader's work, which is why the note states both or neither.
//!
//! # It is a fact, not a warning
//!
//! Nothing here is a banner, a colour, or an adjective. A browser session that
//! behaves exactly as documented has nothing to apologise for, and a workspace
//! that apologises anyway teaches its reader to read past the lines that
//! matter.

/// Where this build's Model Hub keeps the releases it installs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PackStore {
    /// A directory under local application data. An installed release is on
    /// the machine until something removes it.
    Machine,
    /// This browser tab's memory. An installed release lasts as long as the
    /// session that fetched it.
    Session,
}

/// The store the desktop build compiles against.
#[cfg(not(target_arch = "wasm32"))]
pub(super) const CURRENT_STORE: PackStore = PackStore::Machine;

/// The store the browser build compiles against.
#[cfg(target_arch = "wasm32")]
pub(super) const CURRENT_STORE: PackStore = PackStore::Session;

impl Default for PackStore {
    fn default() -> Self {
        CURRENT_STORE
    }
}

/// The one sentence a session-scoped store owes the reader.
///
/// Stated once, in the status line, in the same faint grey as every other fact
/// about the held catalog. Both halves are load-bearing: the first is the only
/// thing about a browser session that differs from the desktop, and the second
/// is what stops the first from being read as "nothing here is saved".
pub(super) const SESSION_SCOPE_NOTE: &str = concat!(
    "Installed packs last as long as this browser session; ",
    "parts retained into a project are saved in the project file."
);

impl PackStore {
    /// What this store is, in one sentence, or nothing when there is nothing
    /// to say.
    ///
    /// The machine store says nothing at all. "Installed packs stay installed"
    /// is what every desktop application already promises by existing, and a
    /// line restating it on every paint is a line a reader learns to skip —
    /// taking the browser's line with it on the day they open the same
    /// workspace in a tab.
    pub(super) const fn scope_note(self) -> Option<&'static str> {
        match self {
            Self::Machine => None,
            Self::Session => Some(SESSION_SCOPE_NOTE),
        }
    }

    /// What a failed install leaves behind in this store.
    ///
    /// The shape is identical — expand into staging, publish under the
    /// release's name only when the whole archive proved — and only the
    /// wreckage differs, so only the wreckage is worded twice. The desktop
    /// sentence used to be printed in a browser session too, where it promised
    /// a directory that does not exist and a sweep at a start that never comes.
    pub(super) const fn install_failure_detail(self) -> &'static str {
        match self {
            Self::Machine => {
                "An install is a staged expansion followed by a rename, so a failure — including \
                 a killed process — leaves a staging directory and nothing else, which the next \
                 start sweeps."
            }
            Self::Session => {
                "An install is staged and published under the release's name only once the whole \
                 archive has proved, so a failure leaves an unreachable staged copy and nothing \
                 else — and this session's store is browser memory, which ends with the tab."
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The exact sentence, asserted as a whole.
    ///
    /// A test that checked for "browser session" would pass on a line that had
    /// lost the half about the project, which is the half that decides whether
    /// a reader believes their work is safe.
    #[test]
    fn the_session_store_states_both_halves_of_what_a_browser_session_keeps() {
        assert_eq!(
            PackStore::Session.scope_note(),
            Some(
                "Installed packs last as long as this browser session; parts retained into a \
                 project are saved in the project file."
            )
        );
    }

    /// The desktop says nothing, which is what keeps the browser's line worth
    /// reading.
    #[test]
    fn the_machine_store_states_nothing() {
        assert_eq!(PackStore::Machine.scope_note(), None);
    }

    /// The note never says a retained part is at risk, and never says an
    /// install survives anything.
    #[test]
    fn the_session_note_claims_no_durability_it_does_not_have() {
        let note = SESSION_SCOPE_NOTE.to_ascii_lowercase();
        for forbidden in ["lost", "will be lost", "temporar", "permanent", "forever"] {
            assert!(
                !note.contains(forbidden),
                "the session note says '{forbidden}': {SESSION_SCOPE_NOTE}"
            );
        }
        assert!(note.contains("this browser session"));
        assert!(note.contains("saved in the project file"));
    }

    /// A desktop build selects the machine store, which is what makes "native
    /// renders are unchanged" a checked claim rather than an intention.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn a_desktop_build_selects_the_machine_store() {
        assert_eq!(CURRENT_STORE, PackStore::Machine);
        assert_eq!(PackStore::default(), PackStore::Machine);
        assert_eq!(PackStore::default().scope_note(), None);
    }

    /// The two stores fail an install differently, and say so differently.
    #[test]
    fn each_store_describes_the_wreckage_it_actually_leaves() {
        assert!(
            PackStore::Machine
                .install_failure_detail()
                .contains("staging directory")
        );
        assert!(
            !PackStore::Session
                .install_failure_detail()
                .contains("directory"),
            "the session store has no directory to leave behind"
        );
        assert!(
            PackStore::Session
                .install_failure_detail()
                .contains("browser memory")
        );
    }
}
