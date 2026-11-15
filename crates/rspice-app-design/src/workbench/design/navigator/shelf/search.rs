//! How the Component shelf reads a query.
//!
//! A ranked keyword match, not a substring test. Text is folded to lowercase
//! alphanumerics, so `op amp`, `opamp` and `Op-Amp` are one word, and a label
//! or keyword matches only from the start of one of its words: `led` finds
//! `LED` and never `Coupled` or `Controlled`. Part numbers are the exception,
//! because `1N4148` is one token whose middle a reader may well type.
//!
//! The navigator tree beside the shelf filters the design in front of the
//! reader and keeps its own plain substring filter.

/// One folded shelf query.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ShelfQuery {
    folded: String,
}

/// How well one row answered a query; lower is better.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum MatchTier {
    /// The whole label or one whole keyword.
    Exact,
    /// The start of a word in the label or a keyword.
    WordPrefix,
    /// The start of a part number.
    PartNumberPrefix,
    /// Anywhere inside a part number.
    PartNumberSubstring,
}

/// A tier; within it, a row whose own label answered before one that only
/// lists the query as a synonym, and a label that opens with the query before
/// one that holds it further in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct MatchRank {
    pub(in super::super) tier: MatchTier,
    by_keyword: bool,
    label_trails: bool,
}

/// The text one shelf row offers a query.
pub(crate) struct ShelfTerms<'a> {
    pub(crate) label: &'a str,
    pub(crate) keywords: &'a [&'a str],
    pub(crate) part_numbers: &'a [&'a str],
}

impl ShelfQuery {
    pub(crate) fn new(raw: &str) -> Self {
        Self {
            folded: fold(raw).0,
        }
    }

    /// A query with nothing searchable in it, which shows the whole catalog.
    pub(in super::super) fn is_empty(&self) -> bool {
        self.folded.is_empty()
    }

    /// How `terms` answers this query, or `None` when it does not. An empty
    /// query answers every row at the bottom tier.
    pub(crate) fn rank(&self, terms: &ShelfTerms<'_>) -> Option<MatchRank> {
        if self.is_empty() {
            return Some(MatchRank {
                tier: MatchTier::PartNumberSubstring,
                by_keyword: true,
                label_trails: true,
            });
        }
        let label_trails = !fold(terms.label).0.starts_with(&self.folded);
        let worded = std::iter::once((terms.label, false))
            .chain(terms.keywords.iter().map(|keyword| (*keyword, true)))
            .filter_map(|(text, by_keyword)| {
                self.worded_tier(text).map(|tier| MatchRank {
                    tier,
                    by_keyword,
                    label_trails,
                })
            });
        let numbered = terms.part_numbers.iter().filter_map(|number| {
            let (folded, _) = fold(number);
            let tier = if folded.starts_with(&self.folded) {
                MatchTier::PartNumberPrefix
            } else if folded.contains(&self.folded) {
                MatchTier::PartNumberSubstring
            } else {
                return None;
            };
            Some(MatchRank {
                tier,
                by_keyword: true,
                label_trails,
            })
        });
        worded.chain(numbered).min()
    }

    fn worded_tier(&self, text: &str) -> Option<MatchTier> {
        let (folded, starts) = fold(text);
        if folded == self.folded {
            return Some(MatchTier::Exact);
        }
        starts
            .iter()
            .any(|start| folded[*start..].starts_with(&self.folded))
            .then_some(MatchTier::WordPrefix)
    }
}

/// `text` lowercased with everything but letters and digits removed, and the
/// byte offset in the result at which each of its words starts.
fn fold(text: &str) -> (String, Vec<usize>) {
    let mut folded = String::with_capacity(text.len());
    let mut starts = Vec::new();
    let mut at_word_start = true;
    for character in text.chars() {
        if character.is_alphanumeric() {
            if at_word_start {
                starts.push(folded.len());
                at_word_start = false;
            }
            folded.extend(character.to_lowercase());
        } else {
            at_word_start = true;
        }
    }
    (folded, starts)
}
