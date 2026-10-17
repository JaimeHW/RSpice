//! The names the Label tool places one per click, as Virtuoso's Create Wire
//! Name places them.
//!
//! The entry is typed into once. A name ending in a number counts on from
//! it (`D0`, then `D1`, `D2`, ...), a list (`a b c`) or a bus range
//! (`D<0:7>`) is placed one name per click, and the canvas ghost shows the
//! name the next click will place. Runtime only: what reaches the document is
//! one ordinary label per click.

use rspice_design::schematic::bus::BusDeclaration;
use rspice_design::schematic::net_label::NetLabelKind;
use std::collections::VecDeque;

/// What the Label tool places on the clicks after the entry named the first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetLabelSequence {
    /// Every label in the sequence is this kind: the connector direction the
    /// entry chose stays with it.
    pub kind: NetLabelKind,
    order: SequenceOrder,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SequenceOrder {
    /// Names typed as a list or a bus range, still to place, in order.
    Listed {
        names: VecDeque<String>,
        total: usize,
    },
    /// A numbered name counting on: `stem` then `next`, padded to `width`
    /// digits as the typed name was.
    Counting {
        stem: String,
        next: u64,
        width: usize,
    },
}

/// The names an entry asks for, in the order the clicks place them.
///
/// Whitespace separates names, which no net name may contain. A bus range is
/// read as one name per member unless `keep_bus_ranges` (the anchor is on a
/// bus, which the range names whole).
pub fn entry_names(entry: &str, keep_bus_ranges: bool) -> Vec<String> {
    let mut names = Vec::new();
    for token in entry.split_whitespace() {
        match BusDeclaration::parse(token) {
            Ok(range) if !keep_bus_ranges => {
                names.extend(range.members().iter().map(ToString::to_string));
            }
            _ => names.push(token.to_owned()),
        }
    }
    names
}

impl NetLabelSequence {
    /// The sequence that follows the first of `names` (the one the entry
    /// places itself), if any: the rest of a list, or the count after a
    /// single numbered name. A single name with no number places once.
    pub fn following(names: &[String], kind: NetLabelKind) -> Option<Self> {
        let order = match names {
            [] => return None,
            [only] => {
                let (stem, number, width) = trailing_number(only)?;
                SequenceOrder::Counting {
                    stem: stem.to_owned(),
                    next: number.checked_add(1)?,
                    width,
                }
            }
            [_, rest @ ..] => SequenceOrder::Listed {
                names: rest.iter().cloned().collect(),
                total: names.len(),
            },
        };
        Some(Self { kind, order })
    }

    /// The name the next click places.
    pub fn next_name(&self) -> Option<String> {
        match &self.order {
            SequenceOrder::Listed { names, .. } => names.front().cloned(),
            SequenceOrder::Counting { stem, next, width } => Some(format!("{stem}{next:0width$}")),
        }
    }

    /// `k/n` for a listed sequence, where `k` is the next name's place.
    pub fn position(&self) -> Option<(usize, usize)> {
        match &self.order {
            SequenceOrder::Listed { names, total } => Some((total - names.len() + 1, *total)),
            SequenceOrder::Counting { .. } => None,
        }
    }

    /// Step past the name just placed. Returns `false` when nothing is left.
    pub fn advance(&mut self) -> bool {
        match &mut self.order {
            SequenceOrder::Listed { names, .. } => {
                names.pop_front();
                !names.is_empty()
            }
            SequenceOrder::Counting { next, .. } => match next.checked_add(1) {
                Some(following) => {
                    *next = following;
                    true
                }
                None => false,
            },
        }
    }
}

/// `D07` as `("D", 7, 2)`: the stem, the number it ends in, and how many
/// digits that number was written with.
fn trailing_number(name: &str) -> Option<(&str, u64, usize)> {
    let stem = name.trim_end_matches(|character: char| character.is_ascii_digit());
    let digits = &name[stem.len()..];
    if digits.is_empty() || stem.is_empty() {
        return None;
    }
    Some((stem, digits.parse().ok()?, digits.len()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(entry: &str) -> Vec<String> {
        entry_names(entry, false)
    }

    #[test]
    fn a_numbered_name_counts_on_keeping_its_padding() {
        let mut sequence =
            NetLabelSequence::following(&names("D0"), NetLabelKind::Local).expect("counts");
        assert_eq!(sequence.next_name().as_deref(), Some("D1"));
        assert!(sequence.advance());
        assert_eq!(sequence.next_name().as_deref(), Some("D2"));
        assert_eq!(sequence.position(), None);

        let padded =
            NetLabelSequence::following(&names("A009"), NetLabelKind::Local).expect("counts");
        assert_eq!(padded.next_name().as_deref(), Some("A010"));
        let carry = NetLabelSequence::following(&names("n9"), NetLabelKind::Local).expect("counts");
        assert_eq!(carry.next_name().as_deref(), Some("n10"));
    }

    #[test]
    fn a_plain_name_or_a_bare_number_places_once() {
        assert_eq!(
            NetLabelSequence::following(&names("vout"), NetLabelKind::Local),
            None
        );
        assert_eq!(
            NetLabelSequence::following(&names("0"), NetLabelKind::Local),
            None
        );
        assert_eq!(NetLabelSequence::following(&[], NetLabelKind::Local), None);
    }

    #[test]
    fn a_list_places_one_name_per_click_then_ends() {
        let typed = names("a b  c");
        assert_eq!(typed, ["a", "b", "c"]);
        let mut sequence =
            NetLabelSequence::following(&typed, NetLabelKind::Local).expect("listed");
        assert_eq!(sequence.next_name().as_deref(), Some("b"));
        assert_eq!(sequence.position(), Some((2, 3)));
        assert!(sequence.advance());
        assert_eq!(sequence.next_name().as_deref(), Some("c"));
        assert_eq!(sequence.position(), Some((3, 3)));
        assert!(!sequence.advance());
        assert_eq!(sequence.next_name(), None);
    }

    #[test]
    fn a_bus_range_is_one_bit_per_click_unless_it_names_a_bus() {
        assert_eq!(names("D<0:3>"), ["D<0>", "D<1>", "D<2>", "D<3>"]);
        assert_eq!(names("Q[2:1] en"), ["Q[2]", "Q[1]", "en"]);
        assert_eq!(entry_names("D<0:3>", true), ["D<0:3>"]);
    }
}
