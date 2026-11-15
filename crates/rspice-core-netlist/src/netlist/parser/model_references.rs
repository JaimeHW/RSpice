//! Which `.model` cards a deck can reach, for a parse that reads only those.
//!
//! A deck that reads a foundry library selects one section of it and then
//! names a handful of its models. Parsing every card of the section costs the
//! same on every replay of the deck, so a simulation parse may skip the cards
//! nothing names (`NetlistParseOptions::skip_unreferenced_models`).
//!
//! The scan is deliberately coarse and errs toward reading a card: a model is
//! reachable when any word of any other card is its name, or the part of its
//! name before a `.` (the family a binned `nch.1` belongs to), and whatever a
//! reachable model's own card names (a PSpice `AKO:` base) is reachable too.
//! Conditional branches, comments and parameter values all count as words, so
//! an extra card is read rather than a needed one skipped. A deck that draws
//! statistical values reads every card, because skipping one would move every
//! later draw of the deck's random stream.

use rustc_hash::{FxHashMap, FxHashSet};

use super::{ParseWithAbortError, poll_parse_abort};
use crate::abort_signal::AbortSignal;

/// The functions that draw from the deck's random stream when evaluated.
const DRAWING_FUNCTIONS: [&str; 8] = [
    "gauss", "agauss", "unif", "aunif", "mc", "rand", "random", "limit",
];

/// Lower-case names of the model cards no other card of a deck reaches.
#[derive(Debug, Default)]
pub(super) struct UnreferencedModels {
    names: FxHashSet<String>,
}

/// One logical card of the scan: its physical lines and whether it is a
/// `.model` card the parse may skip.
struct ScannedCard {
    first_line: usize,
    last_line: usize,
    model: Option<String>,
}

impl UnreferencedModels {
    /// Scan the deck's physical lines from `body_start`. `None` when every
    /// model card is reachable, or the deck draws statistical values.
    pub(super) fn scan(
        lines: &[&str],
        body_start: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Option<Self>, ParseWithAbortError> {
        let cards = logical_cards(lines, body_start, abort)?;
        // Every key a reference may spell: the model's name and each family
        // prefix of it.
        let mut pending: FxHashMap<String, Vec<usize>> = FxHashMap::default();
        for (index, card) in cards.iter().enumerate() {
            let Some(name) = &card.model else {
                continue;
            };
            pending.entry(name.clone()).or_default().push(index);
            for (at, _) in name.match_indices('.') {
                if at > 0 {
                    pending
                        .entry(name[..at].to_owned())
                        .or_default()
                        .push(index);
                }
            }
        }
        if pending.is_empty() {
            return Ok(None);
        }

        let mut reached = vec![false; cards.len()];
        let mut worklist = Vec::new();
        let mut word = String::new();
        let mut reach =
            |text: &str, pending: &mut FxHashMap<String, Vec<usize>>, worklist: &mut Vec<usize>| {
                for_each_word(text, &mut word, |word| {
                    if let Some(indices) = pending.remove(word) {
                        worklist.extend(indices);
                    }
                });
            };
        for (index, card) in cards.iter().enumerate() {
            poll_parse_abort(abort, index)?;
            if card.model.is_some() {
                continue;
            }
            for line in &lines[card.first_line..=card.last_line] {
                reach(line, &mut pending, &mut worklist);
            }
            if pending.is_empty() {
                // Every model card is named somewhere; there is nothing to skip.
                return Ok(None);
            }
        }
        while let Some(index) = worklist.pop() {
            if std::mem::replace(&mut reached[index], true) {
                continue;
            }
            let card = &cards[index];
            for line in &lines[card.first_line..=card.last_line] {
                reach(line, &mut pending, &mut worklist);
            }
        }

        let names: FxHashSet<String> = cards
            .iter()
            .zip(&reached)
            .filter(|(_, reached)| !**reached)
            .filter_map(|(card, _)| card.model.clone())
            .collect();
        if names.is_empty() || draws_statistical_values(lines, body_start, abort)? {
            return Ok(None);
        }
        Ok(Some(Self { names }))
    }

    /// Whether the logical card `card` is a `.model` card nothing reaches.
    pub(super) fn skips(&self, card: &str) -> bool {
        model_card_name(card).is_some_and(|name| self.names.contains(&name))
    }
}

/// Group the physical lines into logical cards as the body parser does: a
/// `+` line continues the card before it, across blank and comment lines.
fn logical_cards(
    lines: &[&str],
    body_start: usize,
    abort: &dyn AbortSignal,
) -> Result<Vec<ScannedCard>, ParseWithAbortError> {
    let mut cards: Vec<ScannedCard> = Vec::new();
    for (index, line) in lines.iter().enumerate().skip(body_start) {
        poll_parse_abort(abort, index)?;
        let trimmed = line.trim_start();
        if trimmed.is_empty() || trimmed.starts_with('*') {
            continue;
        }
        if trimmed.starts_with('+')
            && let Some(card) = cards.last_mut()
        {
            card.last_line = index;
            continue;
        }
        cards.push(ScannedCard {
            first_line: index,
            last_line: index,
            model: model_card_name(trimmed),
        });
    }
    Ok(cards)
}

/// The lower-case name a `.model` card defines, when the card is one and its
/// name is a single word of the scan.
fn model_card_name(card: &str) -> Option<String> {
    let mut fields = card.split_whitespace();
    if !fields.next()?.eq_ignore_ascii_case(".model") {
        return None;
    }
    let field = fields.next()?;
    let name = field.split('(').next().unwrap_or_default();
    (!name.is_empty() && name.bytes().all(is_word_byte)).then(|| name.to_ascii_lowercase())
}

/// Whether `byte` may be part of a model name as the scan reads words.
const fn is_word_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric()
        || matches!(
            byte,
            b'_' | b'.' | b'-' | b'$' | b'#' | b'!' | b'@' | b'%' | b'&' | b'+'
        )
        || !byte.is_ascii()
}

/// Call `visit` with every lower-case word of `text`, then with each part of a
/// word that punctuation separates (`nch` and `1` of `nch.1`, `dmod` of a
/// `+dmod` continuation).
fn for_each_word(text: &str, buffer: &mut String, mut visit: impl FnMut(&str)) {
    let is_part = |c: char| !c.is_ascii() || c.is_ascii_alphanumeric() || c == '_';
    for word in text
        .split(|c: char| c.is_ascii() && !is_word_byte(c as u8))
        .filter(|word| !word.is_empty())
    {
        buffer.clear();
        buffer.push_str(word);
        buffer.make_ascii_lowercase();
        visit(buffer);
        if !buffer.chars().all(is_part) {
            let whole = std::mem::take(buffer);
            for part in whole
                .split('.')
                .chain(whole.split(|c: char| !is_part(c)))
                .filter(|part| !part.is_empty())
            {
                visit(part);
            }
            *buffer = whole;
        }
    }
}

/// Whether any card of the deck calls a function that draws from the random
/// stream.
fn draws_statistical_values(
    lines: &[&str],
    body_start: usize,
    abort: &dyn AbortSignal,
) -> Result<bool, ParseWithAbortError> {
    let mut buffer = String::new();
    for (index, line) in lines.iter().enumerate().skip(body_start) {
        poll_parse_abort(abort, index)?;
        let mut draws = false;
        for_each_word(line, &mut buffer, |word| {
            draws |= DRAWING_FUNCTIONS.contains(&word);
        });
        if draws {
            return Ok(true);
        }
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::NoAbort;

    fn skipped(deck: &str) -> Vec<String> {
        let lines = deck.lines().collect::<Vec<_>>();
        let mut names = UnreferencedModels::scan(&lines, 1, &NoAbort)
            .expect("scan completes")
            .map(|models| models.names.into_iter().collect::<Vec<_>>())
            .unwrap_or_default();
        names.sort();
        names
    }

    #[test]
    fn a_model_no_card_names_is_skipped() {
        assert_eq!(
            skipped("t\nD1 a 0 used\n.model used D(IS=1e-14)\n.model spare D(IS=2e-14)\n.end\n"),
            ["spare"]
        );
    }

    #[test]
    fn a_family_name_reaches_every_bin_and_a_bin_reaches_its_family() {
        let deck = "t\nM1 d g 0 0 nch W=1u L=1u\nM2 d g 0 0 pch.2\n\
                    .model nch.1 nmos level=1\n.model nch.2 nmos level=1\n\
                    .model pch pmos level=1\n.model other.1 nmos level=1\n.end\n";
        assert_eq!(skipped(deck), ["other.1"]);
    }

    #[test]
    fn a_reached_model_reaches_the_cards_it_names() {
        let deck = "t\nD1 a 0 child\n.model child AKO:base\n.model base D(IS=1e-14)\n\
                    .model lonely D\n.end\n";
        assert_eq!(skipped(deck), ["lonely"]);
    }

    #[test]
    fn continuation_lines_belong_to_their_model_card() {
        // `spare`'s continuation names `used2`, but nothing reaches `spare`.
        let deck = "t\nD1 a 0 used\n.model used D\n.model spare D(IS=1e-14\n* note\n+ BV=used2)\n\
                    .model used2 D\n.end\n";
        assert_eq!(skipped(deck), ["spare", "used2"]);
    }

    #[test]
    fn a_deck_that_draws_statistical_values_reads_every_model() {
        let deck = "t\n.param r0={agauss(1k, 10, 3)}\nR1 a 0 {r0}\nD1 a 0 used\n\
                    .model used D\n.model spare D\n.end\n";
        assert!(skipped(deck).is_empty());
    }

    #[test]
    fn names_in_parameters_strings_and_conditionals_count_as_references() {
        let deck = "t\n.param kind=\"viaparam\"\nX1 a b cell m=pick\n.if (0)\nD9 a 0 inbranch\n\
                    .endif\n.model viaparam D\n.model pick D\n.model inbranch D\n.model spare D\n.end\n";
        assert_eq!(skipped(deck), ["spare"]);
    }

    fn parse(deck: &str, skip: bool) -> crate::Netlist {
        crate::netlist::parser::parse_netlist_with_options(
            deck,
            crate::netlist::NetlistParseOptions {
                skip_unreferenced_models: skip,
                ..Default::default()
            },
        )
        .expect("the deck parses")
    }

    fn model_names(netlist: &crate::Netlist) -> Vec<&str> {
        netlist
            .models
            .iter()
            .map(|model| model.name.as_str())
            .collect()
    }

    #[test]
    fn a_skipping_parse_reads_the_named_models_as_a_full_parse_does() {
        let deck = "skip
V1 in 0 1
R1 in a 1k
D1 a 0 dused
M1 a in 0 0 nch W=1u L=1u
.model dused D(IS=1e-14 N=1.1)
.model dspare D(IS=2e-14)
.model nch.1 NMOS(LEVEL=1 VTO=0.5)
.model nch.2 NMOS(LEVEL=1 VTO=0.6)
.model qspare NPN(BF=100)
.op
.end
";
        let full = parse(deck, false);
        let skipping = parse(deck, true);
        assert_eq!(model_names(&skipping), ["DUSED", "NCH.1", "NCH.2"]);
        assert_eq!(skipping.skipped_model_cards, 2);
        assert!(skipping.skips_unreferenced_models());
        for model in &skipping.models {
            let read = full
                .models
                .iter()
                .find(|candidate| candidate.name == model.name)
                .expect("a full parse reads every model");
            assert_eq!(format!("{model:?}"), format!("{read:?}"));
        }
        assert_eq!(
            format!("{:?}", skipping.elements),
            format!("{:?}", full.elements)
        );
    }

    #[test]
    fn a_deck_with_hdl_reads_every_model_card() {
        let deck = "hdl
.hdl \"amp.va\"
X1 in out amp
D1 in 0 dused
.model dused D
.model dspare D
.end
";
        let netlist = parse(deck, true);
        assert_eq!(model_names(&netlist), ["DUSED", "DSPARE"]);
        assert_eq!(netlist.skipped_model_cards, 0);
        assert!(!netlist.skips_unreferenced_models());
    }

    #[test]
    fn a_deck_with_a_control_script_reads_every_model_card() {
        // Only the script names `dalt`.
        let deck = "control
D1 in 0 dused
V1 in 0 1
.model dused D
.model dalt D(IS=2e-14)
.control
altermod dalt is = 3e-14
op
.endc
.end
";
        let netlist = crate::Netlist::parse_with_options(
            deck,
            crate::netlist::NetlistParseOptions {
                retain_control_script: true,
                skip_unreferenced_models: true,
                ..Default::default()
            },
        )
        .expect("the deck parses");
        assert!(netlist.control_script.is_some());
        assert_eq!(model_names(&netlist), ["DUSED", "DALT"]);
        assert!(!netlist.skips_unreferenced_models());
    }

    #[test]
    fn the_skip_test_reads_the_logical_card_name() {
        let models = UnreferencedModels {
            names: ["spare".to_owned()].into_iter().collect(),
        };
        assert!(models.skips(".MODEL Spare D(IS=1e-14) + BV=5"));
        assert!(models.skips(".model spare(D IS=1e-14)"));
        assert!(!models.skips(".model used D"));
        assert!(!models.skips("Dspare a b spare"));
    }
}
