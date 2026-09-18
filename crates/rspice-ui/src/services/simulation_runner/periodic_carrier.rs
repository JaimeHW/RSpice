//! Which large-signal periodic solve a small-signal periodic run linearizes
//! around.
//!
//! The engine's `.PAC`, `.PXF` and `.PNOISE` cards carry one keyword for this
//! and it names a *family*, not an instance: `card_source_selector` in
//! `rspice-core/src/netlist/parser/periodic_cards.rs` reads `FROM=PSS|HB` and
//! refuses everything else with `expected: "PSS or HB"`. An absent key is a
//! third position rather than a synonym for either, because
//! `resolve_periodic_source` in `rspice-core/src/execution/plan.rs` binds a
//! card without `FROM=` to the nearest preceding `.PSS` *or* `.HB`, whichever
//! the deck wrote last.
//!
//! So a per-instance spelling does not exist to be offered: a card naming one
//! plan instance out of several is a card the engine refuses by name. The
//! family is the whole of what the deck can say, and the instance follows from
//! authored order — on both sides.
//!
//! `.PSTB` is deliberately absent from this type's users. Its card has no
//! `FROM=` arm at all, and the plan binds it to the preceding `.PSS`
//! unconditionally, because [`rspice_core::engine::PssAnalysisResult`] is the
//! only thing in the engine carrying a monodromy matrix.

/// The periodic carrier a small-signal periodic run reads.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PeriodicCarrier {
    /// The nearest preceding periodic solve of either family. Writes no
    /// `FROM=` keyword, which is exactly what the card means without one.
    #[default]
    Preceding,
    /// The nearest preceding shooting `.PSS`.
    Pss,
    /// The nearest preceding `.HB`.
    ///
    /// Accepted by all three engine cards and by both readers, and executable
    /// on the command line through `Engine::run_pac_from_hb_with_abort` and its
    /// two siblings. The Studio has no runner that takes a harmonic-balance
    /// operating point for these three kinds, so a request naming it is
    /// refused by name rather than silently bound to a `.PSS` that is not the
    /// carrier that was asked for.
    Hb,
}

impl PeriodicCarrier {
    /// Every position, in the order the chooser paints them.
    pub const ALL: &'static [Self] = &[Self::Preceding, Self::Pss, Self::Hb];

    /// The `FROM=` value a card states for this carrier, or `None` where the
    /// absent keyword *is* the selection.
    pub fn spice_name(self) -> Option<&'static str> {
        match self {
            Self::Preceding => None,
            Self::Pss => Some("pss"),
            Self::Hb => Some("hb"),
        }
    }

    /// This carrier's chooser label.
    pub fn display_name(self) -> &'static str {
        match self {
            Self::Preceding => "preceding solve",
            Self::Pss => "periodic steady state",
            Self::Hb => "harmonic balance",
        }
    }

    /// The carrier a deck's `FROM=` spelling names.
    ///
    /// The two spellings are the engine's, read off `card_source_selector`;
    /// anything else is the engine's `InvalidChoice` and is `None` here.
    pub fn from_spice_name(spelling: &str) -> Option<Self> {
        match spelling.trim().to_ascii_lowercase().as_str() {
            "pss" => Some(Self::Pss),
            "hb" => Some(Self::Hb),
            _ => None,
        }
    }

    /// The position this carrier holds in [`Self::ALL`].
    pub fn index(self) -> usize {
        Self::ALL
            .iter()
            .position(|candidate| *candidate == self)
            .unwrap_or_default()
    }

    /// The carrier a chooser position names, or the default for a position a
    /// project saved that this build no longer offers.
    pub fn at(index: usize) -> Self {
        Self::ALL.get(index).copied().unwrap_or_default()
    }

    /// The few words a chooser paints beside a position it cannot select.
    ///
    /// The same fact as [`Self::unroutable_reason`], cut to what fits in a
    /// select row. The sentence stays there, where a project or a deck that
    /// names this carrier is answered; a row has no space for it.
    pub fn chooser_restriction(self) -> Option<&'static str> {
        match self {
            Self::Preceding | Self::Pss => None,
            Self::Hb => Some("no Studio route; runs on the command line"),
        }
    }

    /// Why this carrier cannot be run here, in the engine's own terms, or
    /// `None` where the Studio has the route.
    ///
    /// One sentence for all three kinds because the limitation is one thing:
    /// no service runner in this crate takes a harmonic-balance operating
    /// point for a `.PAC`-family card. It names where the card does run, so an
    /// operator holding a deck the engine accepts is told what to do with it
    /// instead of being told the card is wrong.
    pub fn unroutable_reason(self, directive: &str) -> Option<String> {
        match self {
            Self::Preceding | Self::Pss => None,
            Self::Hb => Some(format!(
                "{directive} from=hb has no route in the Studio: no runner here linearizes a \
                 harmonic-balance carrier for this card. The engine does, so a deck carrying it \
                 runs on the command line; author from=pss to run it here"
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The chooser positions are stable, so a saved position means the same
    /// carrier in every build that reads it.
    #[test]
    fn every_carrier_round_trips_through_its_chooser_position() {
        for carrier in PeriodicCarrier::ALL {
            assert_eq!(PeriodicCarrier::at(carrier.index()), *carrier);
        }
        assert_eq!(PeriodicCarrier::at(0), PeriodicCarrier::Preceding);
        assert_eq!(
            PeriodicCarrier::at(PeriodicCarrier::ALL.len()),
            PeriodicCarrier::Preceding
        );
    }

    /// The deck spelling and the card keyword are one vocabulary, so a card
    /// this writes is a card the reader gives back unchanged.
    #[test]
    fn the_card_keyword_and_the_reader_share_one_vocabulary() {
        for carrier in PeriodicCarrier::ALL {
            match carrier.spice_name() {
                Some(spelling) => assert_eq!(
                    PeriodicCarrier::from_spice_name(spelling),
                    Some(*carrier),
                    "{spelling} must read back as the carrier that wrote it"
                ),
                None => assert_eq!(*carrier, PeriodicCarrier::Preceding),
            }
        }
        assert_eq!(
            PeriodicCarrier::from_spice_name("  HB "),
            Some(PeriodicCarrier::Hb)
        );
        assert_eq!(PeriodicCarrier::from_spice_name("tran"), None);
    }
}
