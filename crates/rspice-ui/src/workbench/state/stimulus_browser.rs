//! How the Stimulus Library's browser is looking at the library.
//!
//! Two runtime facts, and neither of them is the project: which slice of the
//! library the scope strip is showing, and the waveform each row draws. Both
//! are readings rather than data — a reader who narrowed the list to the
//! definitions nothing has adopted has not changed the library — so they live
//! here beside the editor's drafts and are cleared with them.
//!
//! The minis are here rather than in the painter because a painter has no
//! frames. Evaluating a fourteen row library is fourteen engine parses and
//! nearly a thousand sample steps; doing it per frame would cost that on every
//! pointer move, and nothing on screen would show it. The cache is the link
//! dialog's own ([`crate::properties::source_preview::MiniCache`]), keyed by
//! name and revision, so the browser's row and the dialog's row are one
//! evaluation of one picture at two sizes.

use crate::properties::source_preview::{self, MiniCache};
use crate::simulation::stimulus_realize::{PreviewTiming, WaveformTrace};
use crate::state::StimulusLibrary;

/// Which slice of the library the browser is showing.
///
/// Adoption is the one axis worth a strip: it is the fact a definition cannot
/// state about itself, it is what a reader tidying a library is looking for,
/// and it is the fact the footer already counts. A second strip over family
/// would return the list the tree's own groups already separate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StimulusScope {
    /// Every definition the project owns.
    #[default]
    All,
    /// The definitions something in the design has adopted.
    Adopted,
    /// The definitions nothing has adopted.
    Unadopted,
}

impl StimulusScope {
    /// The three scopes, in the order the strip offers them.
    pub const ALL: [Self; 3] = [Self::All, Self::Adopted, Self::Unadopted];

    /// The word on the switch.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Adopted => "Adopted",
            Self::Unadopted => "Unadopted",
        }
    }

    /// Whether a definition with this many adopters belongs to this scope.
    #[must_use]
    pub const fn admits(self, adopters: usize) -> bool {
        match self {
            Self::All => true,
            Self::Adopted => adopters > 0,
            Self::Unadopted => adopters == 0,
        }
    }
}

/// What the library browser is showing, and the waveforms it has evaluated.
#[derive(Debug, Clone, Default)]
pub struct StimulusBrowserState {
    /// Which slice of the library the scope strip is on.
    pub scope: StimulusScope,
    minis: MiniCache,
    /// How many minis have been evaluated since this browser was opened. The
    /// cache is a claim about per-frame cost, and this is what lets a test hold
    /// the browser to it.
    evaluations: u64,
}

impl StimulusBrowserState {
    /// Evaluate whatever the library holds that has not been drawn yet, and
    /// hand back every mini the rows will paint.
    ///
    /// A definition whose saved card the engine refuses is cached as that
    /// refusal, so the row shows its family mark rather than asking again on
    /// the next frame.
    pub fn minis(&mut self, library: &StimulusLibrary, timing: PreviewTiming) -> &MiniCache {
        let evaluated = source_preview::ensure_minis(&mut self.minis, library, timing);
        self.evaluations = self.evaluations.saturating_add(evaluated as u64);
        &self.minis
    }

    /// The mini for one saved revision, if it has been evaluated.
    #[must_use]
    pub fn mini(&self, name: &str, revision: u32) -> Option<&Result<WaveformTrace, String>> {
        self.minis.get(&(name.to_owned(), revision))
    }

    /// How many waveforms this browser has asked the engine for.
    #[cfg(test)]
    #[must_use]
    pub const fn evaluations(&self) -> u64 {
        self.evaluations
    }

    /// Forget everything. The project closed, or its library was reverted.
    pub fn clear(&mut self) {
        self.scope = StimulusScope::default();
        self.minis.clear();
    }
}
