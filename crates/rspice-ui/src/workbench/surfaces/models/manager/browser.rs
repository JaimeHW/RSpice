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

use std::path::Path;

/// Which of the two hosts this build was compiled for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Host {
    /// A machine with a filesystem. An installed release is under local
    /// application data until something removes it.
    Desktop,
    /// A browser tab. An installed release lives in the tab's memory and lasts
    /// as long as the session that fetched it.
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

/// The one sentence a browser session owes the reader about its pack store.
///
/// Stated once, in the status line, in the same faint grey as every other fact
/// about the held catalog. Both halves are load-bearing: the first is the only
/// thing about a browser session that differs from the desktop, and the second
/// is what stops the first from being read as "nothing here is saved".
pub(super) const SESSION_SCOPE_NOTE: &str = concat!(
    "Installed packs last as long as this browser session; ",
    "parts retained into a project are saved in the project file."
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
    pub(super) const fn scope_note(self) -> Option<&'static str> {
        match self {
            Self::Desktop => None,
            Self::Browser => Some(SESSION_SCOPE_NOTE),
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
                 else — and this session's store is browser memory, which ends with the tab."
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

    /// The exact sentence, asserted as a whole.
    ///
    /// A test that checked for "browser session" would pass on a line that had
    /// lost the half about the project, which is the half that decides whether
    /// a reader believes their work is safe.
    #[test]
    fn a_browser_session_states_both_halves_of_what_it_keeps() {
        assert_eq!(
            Host::Browser.scope_note(),
            Some(
                "Installed packs last as long as this browser session; parts retained into a \
                 project are saved in the project file."
            )
        );
    }

    /// The desktop says nothing, which is what keeps the browser's line worth
    /// reading.
    #[test]
    fn the_desktop_states_nothing() {
        assert_eq!(Host::Desktop.scope_note(), None);
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

    /// A desktop build selects the desktop host, which is what makes "native
    /// renders are unchanged" a checked claim rather than an intention.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn a_desktop_build_selects_the_desktop_host() {
        assert_eq!(current_host(), Host::Desktop);
        assert_eq!(Host::default(), Host::Desktop);
        assert_eq!(Host::default().scope_note(), None);
    }

    /// And a browser build selects the other one, from the same four lines.
    #[cfg(target_arch = "wasm32")]
    #[test]
    fn a_browser_build_selects_the_browser_host() {
        assert_eq!(current_host(), Host::Browser);
        assert_eq!(Host::default().scope_note(), Some(SESSION_SCOPE_NOTE));
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
        assert!(
            Host::Browser
                .install_failure_detail()
                .contains("browser memory")
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
