//! Which foundation card an unbound device family falls back to.
//!
//! A schematic device the user never bound a model to still has to simulate.
//! Deciding *what it is* when nobody said is one decision, and this is the one
//! place it is made: every consumer — the netlist generator that writes the
//! deck, the flattener that instantiates it, the engine that binds it — reads
//! the same family-to-card table and the same card text, so a bare placement
//! cannot mean one thing in the editor and another in the solver.
//!
//! The cards themselves live in `foundation.lib` and are handed out verbatim.
//! Nothing here restates a parameter value: a second spelling of `BF=100` is
//! exactly the drift this module exists to prevent.

use crate::builtin_lib::FOUNDATION_LIB;

/// A device family a schematic can place without binding a model.
///
/// Polarity is part of the family because it is part of the card: an
/// n-channel and a p-channel placement are different devices, not one device
/// with a sign.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FoundationDeviceFamily {
    /// Two-terminal junction diode.
    Diode,
    /// Three- and four-terminal NPN bipolar (Gummel-Poon).
    NpnBjt,
    /// Three- and four-terminal PNP bipolar (Gummel-Poon).
    PnpBjt,
    /// Five-terminal self-heating NPN bipolar (VBIC), whose dT node is solved.
    NpnBjtThermal,
    /// Five-terminal self-heating PNP bipolar (VBIC), whose dT node is solved.
    PnpBjtThermal,
    /// Four-terminal n-channel MOSFET.
    Nmos,
    /// Four-terminal p-channel MOSFET.
    Pmos,
    /// Five-terminal n-channel silicon-on-insulator MOSFET.
    NmosSoi,
    /// Five-terminal p-channel silicon-on-insulator MOSFET.
    PmosSoi,
    /// n-channel vertical power MOSFET.
    NVdmos,
    /// p-channel vertical power MOSFET.
    PVdmos,
    /// n-channel junction FET.
    Njfet,
    /// p-channel junction FET.
    Pjfet,
    /// n-channel MESFET.
    Nmesfet,
    /// p-channel MESFET.
    Pmesfet,
}

impl FoundationDeviceFamily {
    /// Every family with a foundation default, for exhaustive coverage.
    pub const ALL: &'static [Self] = &[
        Self::Diode,
        Self::NpnBjt,
        Self::PnpBjt,
        Self::NpnBjtThermal,
        Self::PnpBjtThermal,
        Self::Nmos,
        Self::Pmos,
        Self::NmosSoi,
        Self::PmosSoi,
        Self::NVdmos,
        Self::PVdmos,
        Self::Njfet,
        Self::Pjfet,
        Self::Nmesfet,
        Self::Pmesfet,
    ];

    /// The foundation card an unbound placement of this family resolves to.
    #[must_use]
    pub const fn default_model_name(self) -> &'static str {
        match self {
            Self::Diode => "RSPICE_DIODE",
            Self::NpnBjt => "RSPICE_NPN",
            Self::PnpBjt => "RSPICE_PNP",
            Self::NpnBjtThermal => "RSPICE_NPN_THERMAL",
            Self::PnpBjtThermal => "RSPICE_PNP_THERMAL",
            Self::Nmos => "RSPICE_NMOS",
            Self::Pmos => "RSPICE_PMOS",
            Self::NmosSoi => "RSPICE_NMOS_SOI",
            Self::PmosSoi => "RSPICE_PMOS_SOI",
            Self::NVdmos => "RSPICE_NVDMOS",
            Self::PVdmos => "RSPICE_PVDMOS",
            Self::Njfet => "RSPICE_NJFET",
            Self::Pjfet => "RSPICE_PJFET",
            Self::Nmesfet => "RSPICE_NMESFET",
            Self::Pmesfet => "RSPICE_PMESFET",
        }
    }
}

/// The verbatim `.MODEL` source of one foundation card, continuations included.
///
/// Verbatim rather than re-rendered from the parsed card: a deck that must
/// carry its own models carries the authored bytes, so what the deck says and
/// what the engine's embedded fallback would have applied are the same text,
/// not two roundings of the same intent.
///
/// Cards declared inside a `.SUBCKT` body are model-scoped to that subcircuit
/// and are not addressable by name from a deck, so they are not offered here.
#[must_use]
pub fn foundation_card_source(name: &str) -> Option<&'static str> {
    let mut subckt_depth = 0usize;
    let mut offset = 0usize;

    while offset < FOUNDATION_LIB.len() {
        let line_end = FOUNDATION_LIB[offset..]
            .find('\n')
            .map_or(FOUNDATION_LIB.len(), |index| offset + index);
        let line = FOUNDATION_LIB[offset..line_end].trim();

        if starts_with_directive(line, ".subckt") {
            subckt_depth += 1;
        } else if starts_with_directive(line, ".ends") {
            subckt_depth = subckt_depth.saturating_sub(1);
        } else if subckt_depth == 0
            && let Some(rest) = directive_rest(line, ".model")
            && rest
                .split_whitespace()
                .next()
                .is_some_and(|card| card.eq_ignore_ascii_case(name))
        {
            return Some(FOUNDATION_LIB[offset..card_end(line_end)].trim_end());
        }

        offset = (line_end + 1).min(FOUNDATION_LIB.len());
    }

    None
}

/// Extend a card that starts at `line_end`'s line through its `+` continuations.
fn card_end(mut line_end: usize) -> usize {
    while line_end < FOUNDATION_LIB.len() {
        let next_start = line_end + 1;
        if next_start > FOUNDATION_LIB.len() {
            break;
        }
        let next_end = FOUNDATION_LIB[next_start..]
            .find('\n')
            .map_or(FOUNDATION_LIB.len(), |index| next_start + index);
        if !FOUNDATION_LIB[next_start..next_end]
            .trim_start()
            .starts_with('+')
        {
            break;
        }
        line_end = next_end;
    }
    line_end
}

/// The text after `directive` when `line` opens with exactly that directive.
fn directive_rest<'a>(line: &'a str, directive: &str) -> Option<&'a str> {
    let rest = line.get(..directive.len())?;
    if !rest.eq_ignore_ascii_case(directive) {
        return None;
    }
    let rest = &line[directive.len()..];
    if rest.is_empty() || rest.starts_with(char::is_whitespace) {
        Some(rest.trim_start())
    } else {
        None
    }
}

fn starts_with_directive(line: &str, directive: &str) -> bool {
    directive_rest(line, directive).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every family names a card that is actually in the library. A family
    /// pointing at a card nobody authored is an unbound placement that cannot
    /// simulate, and it must not be discoverable only at run time.
    #[test]
    fn every_family_resolves_to_a_present_card() {
        for family in FoundationDeviceFamily::ALL {
            let name = family.default_model_name();
            let source = foundation_card_source(name)
                .unwrap_or_else(|| panic!("{family:?} names absent foundation card '{name}'"));
            assert!(
                source
                    .to_ascii_uppercase()
                    .contains(&name.to_ascii_uppercase()),
                "{family:?} card source does not name '{name}': {source}"
            );
        }
    }

    /// No two families share a card: a card serving two families would make
    /// one of them a lie about what was placed.
    #[test]
    fn families_do_not_share_a_card() {
        let mut seen = std::collections::HashSet::new();
        for family in FoundationDeviceFamily::ALL {
            let name = family.default_model_name();
            assert!(seen.insert(name), "'{name}' is claimed by two families");
        }
    }

    /// The returned text is one whole card and nothing else.
    #[test]
    fn card_source_is_the_single_authored_line() {
        let source = foundation_card_source("rspice_npn").expect("case-insensitive lookup");
        assert!(source.starts_with(".model RSPICE_NPN NPN ("), "{source}");
        assert!(source.ends_with(')'), "{source}");
        assert_eq!(source.lines().count(), 1, "{source}");
    }

    /// A card declared inside the op-amp subcircuit body would not be
    /// addressable from a deck, so the scan must not offer subcircuit content.
    #[test]
    fn subcircuit_bodies_are_not_scanned() {
        assert_eq!(foundation_card_source("RINP"), None);
        assert_eq!(foundation_card_source("RSPICE_OPAMP"), None);
    }

    #[test]
    fn unknown_cards_resolve_to_nothing() {
        assert_eq!(foundation_card_source("RSPICE_NOT_A_CARD"), None);
    }
}
