//! Find and replace in a netlist document.

use std::ops::Range;

use regex::RegexBuilder;
use serde::{Deserialize, Serialize};

/// Search direction relative to the supplied caret byte offset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FindDirection {
    #[default]
    Forward,
    Backward,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct FindOptions {
    pub direction: FindDirection,
    pub match_case: bool,
    pub whole_word: bool,
    pub regular_expression: bool,
}

impl Default for FindOptions {
    fn default() -> Self {
        Self {
            direction: FindDirection::Forward,
            match_case: true,
            whole_word: false,
            regular_expression: false,
        }
    }
}

/// Exact byte range plus one-based Unicode scalar source coordinates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FindMatch {
    byte_range: Range<usize>,
    line: usize,
    column: usize,
    wrapped: bool,
}

/// A deliberately bounded search result. `truncated` is true when at least
/// one additional match exists beyond the caller's limit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedFindMatches {
    matches: Vec<FindMatch>,
    truncated: bool,
}

impl BoundedFindMatches {
    #[must_use]
    pub fn matches(&self) -> &[FindMatch] {
        &self.matches
    }

    #[must_use]
    pub const fn truncated(&self) -> bool {
        self.truncated
    }
}

impl FindMatch {
    #[must_use]
    pub fn byte_range(&self) -> Range<usize> {
        self.byte_range.clone()
    }

    #[must_use]
    pub const fn line(&self) -> usize {
        self.line
    }

    #[must_use]
    pub const fn column(&self) -> usize {
        self.column
    }

    #[must_use]
    #[cfg(test)]
    pub const fn wrapped(&self) -> bool {
        self.wrapped
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum FindError {
    #[error("find text must not be empty")]
    EmptyQuery,
    #[error("caret byte offset {offset} is outside the {source_len}-byte source")]
    OffsetOutsideSource { offset: usize, source_len: usize },
    #[error("caret byte offset {0} is not on a UTF-8 character boundary")]
    OffsetInsideCharacter(usize),
    #[error("invalid regular expression: {0}")]
    InvalidRegularExpression(String),
    #[error("the regular-expression match could not be reconstructed safely")]
    MatchReconstructionFailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "scope")]
pub enum ReplaceScope {
    Next { caret_byte: usize },
    All,
}

/// Exact replacement result. Ranges identify inserted replacement text in the
/// returned source, not stale ranges in the predecessor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplaceOutcome {
    source: String,
    replaced_ranges: Vec<Range<usize>>,
    wrapped: bool,
}

impl ReplaceOutcome {
    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }

    #[must_use]
    pub fn into_source(self) -> String {
        self.source
    }

    #[must_use]
    pub fn replaced_ranges(&self) -> &[Range<usize>] {
        &self.replaced_ranges
    }

    #[must_use]
    pub fn replacement_count(&self) -> usize {
        self.replaced_ranges.len()
    }

    #[must_use]
    pub const fn wrapped(&self) -> bool {
        self.wrapped
    }
}

/// Find the next exact source range and wrap at the document boundary.
/// Case-insensitive matching applies Unicode lowercase mappings while still
/// returning byte offsets into the untouched original text.
#[cfg(test)]
pub fn find_in_source(
    source: &str,
    query: &str,
    caret_byte: usize,
    options: FindOptions,
) -> Result<Option<FindMatch>, FindError> {
    validate_request(source, query, caret_byte)?;
    let candidates = candidate_ranges(source, query, options)?;
    if candidates.is_empty() {
        return Ok(None);
    }

    let Some((range, wrapped)) = select_range(&candidates, caret_byte, options.direction) else {
        return Ok(None);
    };
    let (line, column) = line_column(source, range.start);
    Ok(Some(FindMatch {
        byte_range: range,
        line,
        column,
        wrapped,
    }))
}

/// Return at most `limit` non-overlapping matches in source order and report
/// whether more exist. The extra sentinel match is discarded, bounding result
/// storage to `limit` even for dense literals and zero-width regular
/// expressions. The regex engine advances zero-width matches on UTF-8
/// boundaries, so this operation cannot stall on empty expressions.
///
/// There is deliberately no unbounded twin. A flat deck answers a one-letter
/// query a hundred and fifty thousand times, and a surface that asked for all
/// of them got a result set it could neither hold nor show.
pub fn find_all_in_source_bounded(
    source: &str,
    query: &str,
    options: FindOptions,
    limit: usize,
) -> Result<BoundedFindMatches, FindError> {
    validate_request(source, query, 0)?;
    let mut candidates = candidate_ranges_bounded(source, query, options, limit.saturating_add(1))?;
    let truncated = candidates.len() > limit;
    candidates.truncate(limit);
    let mut cursor = SourceCursor::new(source);
    Ok(BoundedFindMatches {
        matches: candidates
            .into_iter()
            .map(|byte_range| {
                let (line, column) = cursor.line_column(byte_range.start);
                FindMatch {
                    byte_range,
                    line,
                    column,
                    wrapped: false,
                }
            })
            .collect(),
        truncated,
    })
}

/// Replace the next directional match or every source-order match. Literal
/// replacements are inserted exactly. Regular-expression replacements use
/// the regex crate's `$1`/`$name` capture expansion semantics.
pub fn replace_in_source(
    source: &str,
    query: &str,
    replacement: &str,
    options: FindOptions,
    scope: ReplaceScope,
) -> Result<ReplaceOutcome, FindError> {
    let caret = match scope {
        ReplaceScope::Next { caret_byte } => caret_byte,
        ReplaceScope::All => 0,
    };
    validate_request(source, query, caret)?;
    let candidates = candidate_ranges(source, query, options)?;
    let (selected, wrapped) = match scope {
        ReplaceScope::All => (candidates, false),
        ReplaceScope::Next { caret_byte } => {
            let Some(found) = select_range(&candidates, caret_byte, options.direction) else {
                return Ok(ReplaceOutcome {
                    source: source.to_owned(),
                    replaced_ranges: Vec::new(),
                    wrapped: false,
                });
            };
            (vec![found.0], found.1)
        }
    };
    replace_source_ranges(source, query, replacement, options, &selected, wrapped)
}

/// Replace an already selected, source-ordered set of exact matches. This is
/// used by syntax-aware project search after comment-only candidates have
/// been excluded. Every supplied range is reconstructed against the original
/// source before any output is returned, so a stale or masked range fails
/// atomically.
pub fn replace_source_ranges(
    source: &str,
    query: &str,
    replacement: &str,
    options: FindOptions,
    selected: &[Range<usize>],
    wrapped: bool,
) -> Result<ReplaceOutcome, FindError> {
    validate_request(source, query, 0)?;
    if selected.is_empty() {
        return Ok(ReplaceOutcome {
            source: source.to_owned(),
            replaced_ranges: Vec::new(),
            wrapped,
        });
    }

    let expression = options
        .regular_expression
        .then(|| build_regex(query, options.match_case))
        .transpose()?;
    let mut output = String::with_capacity(source.len());
    let mut replaced_ranges = Vec::with_capacity(selected.len());
    let mut predecessor_end = 0;
    for range in selected.iter().cloned() {
        if range.start < predecessor_end
            || range.end > source.len()
            || range.start > range.end
            || !source.is_char_boundary(range.start)
            || !source.is_char_boundary(range.end)
        {
            return Err(FindError::MatchReconstructionFailed);
        }
        output.push_str(&source[predecessor_end..range.start]);
        let replacement_start = output.len();
        if let Some(expression) = &expression {
            let Some(captures) = expression
                .captures_at(source, range.start)
                .filter(|captures| captures.get(0).is_some_and(|found| found.range() == range))
            else {
                return Err(FindError::MatchReconstructionFailed);
            };
            captures.expand(replacement, &mut output);
        } else {
            let valid_literal = if options.match_case {
                source.get(range.clone()) == Some(query)
            } else {
                source
                    .get(range.clone())
                    .is_some_and(|candidate| candidate.to_lowercase() == query.to_lowercase())
            };
            if !valid_literal {
                return Err(FindError::MatchReconstructionFailed);
            }
            output.push_str(replacement);
        }
        replaced_ranges.push(replacement_start..output.len());
        predecessor_end = range.end;
    }
    output.push_str(&source[predecessor_end..]);
    Ok(ReplaceOutcome {
        source: output,
        replaced_ranges,
        wrapped,
    })
}

fn validate_request(source: &str, query: &str, caret_byte: usize) -> Result<(), FindError> {
    if query.is_empty() {
        return Err(FindError::EmptyQuery);
    }
    if caret_byte > source.len() {
        return Err(FindError::OffsetOutsideSource {
            offset: caret_byte,
            source_len: source.len(),
        });
    }
    if !source.is_char_boundary(caret_byte) {
        return Err(FindError::OffsetInsideCharacter(caret_byte));
    }
    Ok(())
}

fn candidate_ranges(
    source: &str,
    query: &str,
    options: FindOptions,
) -> Result<Vec<Range<usize>>, FindError> {
    let mut candidates = if options.regular_expression {
        build_regex(query, options.match_case)?
            .find_iter(source)
            .map(|found| found.range())
            .collect::<Vec<_>>()
    } else if options.match_case {
        source
            .match_indices(query)
            .map(|(start, value)| start..start + value.len())
            .collect::<Vec<_>>()
    } else {
        case_insensitive_ranges(source, query)
    };
    if options.whole_word {
        candidates.retain(|range| is_whole_word(source, range));
    }
    Ok(candidates)
}

fn candidate_ranges_bounded(
    source: &str,
    query: &str,
    options: FindOptions,
    limit: usize,
) -> Result<Vec<Range<usize>>, FindError> {
    if limit == 0 {
        return Ok(Vec::new());
    }
    let whole_word = |range: &Range<usize>| !options.whole_word || is_whole_word(source, range);
    if options.regular_expression {
        return Ok(build_regex(query, options.match_case)?
            .find_iter(source)
            .map(|found| found.range())
            .filter(whole_word)
            .take(limit)
            .collect());
    }
    if options.match_case {
        return Ok(source
            .match_indices(query)
            .map(|(start, value)| start..start + value.len())
            .filter(whole_word)
            .take(limit)
            .collect());
    }
    Ok(case_insensitive_ranges_bounded(
        source,
        query,
        limit,
        options.whole_word,
    ))
}

fn build_regex(query: &str, match_case: bool) -> Result<regex::Regex, FindError> {
    RegexBuilder::new(query)
        .case_insensitive(!match_case)
        .unicode(true)
        .build()
        .map_err(|error| FindError::InvalidRegularExpression(error.to_string()))
}

fn select_range(
    candidates: &[Range<usize>],
    caret_byte: usize,
    direction: FindDirection,
) -> Option<(Range<usize>, bool)> {
    match direction {
        FindDirection::Forward => candidates
            .iter()
            .find(|range| range.start >= caret_byte)
            .map(|range| (range.clone(), false))
            .or_else(|| candidates.first().cloned().map(|range| (range, true))),
        FindDirection::Backward => candidates
            .iter()
            .rev()
            .find(|range| range.start < caret_byte)
            .map(|range| (range.clone(), false))
            .or_else(|| candidates.last().cloned().map(|range| (range, true))),
    }
}

fn case_insensitive_ranges(source: &str, query: &str) -> Vec<Range<usize>> {
    case_insensitive_ranges_bounded(source, query, usize::MAX, false)
}

/// Non-overlapping case-insensitive matches in source order.
///
/// The source is folded a character at a time as the scan reaches it. Folding
/// it up front costs a second copy of the deck plus a boundary table sixteen
/// bytes wide per character — around fifty megabytes for a three-megabyte
/// netlist, allocated on every frame the find surface is open, which is not a
/// bound at all.
fn case_insensitive_ranges_bounded(
    source: &str,
    query: &str,
    limit: usize,
    whole_word: bool,
) -> Vec<Range<usize>> {
    let folded_query = query.to_lowercase();
    if folded_query.is_empty() || limit == 0 {
        return Vec::new();
    }

    let next_character =
        |start: usize| start + source[start..].chars().next().map_or(1, char::len_utf8);
    let mut found = Vec::new();
    let mut start = 0;
    while start < source.len() {
        let Some(end) = folded_match_end(source, start, &folded_query) else {
            start = next_character(start);
            continue;
        };
        let range = start..end;
        // Resuming at the end of a match is what keeps the results
        // non-overlapping, including when the whole-word filter rejects it.
        start = if end > start {
            end
        } else {
            next_character(start)
        };
        if whole_word && !is_whole_word(source, &range) {
            continue;
        }
        found.push(range);
        if found.len() == limit {
            break;
        }
    }
    found
}

/// Where the folded query ends inside `source` when matched at `start`, or
/// `None` when it does not match there.
///
/// Both ends must fall on source character boundaries: a query that would
/// match only part of what one character folds to has not matched a character
/// the source contains.
fn folded_match_end(source: &str, start: usize, folded_query: &str) -> Option<usize> {
    let mut wanted = folded_query.chars();
    let mut consumed = start;
    for character in source[start..].chars() {
        if wanted.as_str().is_empty() {
            break;
        }
        for folded in character.to_lowercase() {
            if wanted.next() != Some(folded) {
                return None;
            }
        }
        consumed += character.len_utf8();
    }
    wanted.as_str().is_empty().then_some(consumed)
}

fn is_whole_word(source: &str, range: &Range<usize>) -> bool {
    let left = source[..range.start].chars().next_back();
    let right = source[range.end..].chars().next();
    !left.is_some_and(is_word_character) && !right.is_some_and(is_word_character)
}

fn is_word_character(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

/// Maps byte offsets to one-based line and column in a single forward pass.
///
/// Deriving a coordinate from its own prefix costs the whole prefix, so a
/// result set costs the source once per match: a dense query over a large deck
/// stopped being a search and became a hang. Matches arrive in source order,
/// so the walk resumes where the previous one stopped.
struct SourceCursor<'a> {
    source: &'a str,
    /// Byte offset the walk has reached.
    offset: usize,
    /// One-based line containing `offset`.
    line: usize,
    /// Byte offset of the first character of `line`.
    line_start: usize,
    /// Characters between `line_start` and `offset`.
    characters: usize,
}

impl<'a> SourceCursor<'a> {
    const fn new(source: &'a str) -> Self {
        Self {
            source,
            offset: 0,
            line: 1,
            line_start: 0,
            characters: 0,
        }
    }

    /// One-based line and column of `offset`, which must be a character
    /// boundary. An offset behind the walk is still answered exactly; it just
    /// costs a rewind, so an out-of-order producer stays correct.
    fn line_column(&mut self, offset: usize) -> (usize, usize) {
        if offset < self.offset {
            *self = Self::new(self.source);
        }
        for (index, byte) in self.source.as_bytes()[self.offset..offset]
            .iter()
            .enumerate()
        {
            if *byte == b'\n' {
                self.line += 1;
                self.line_start = self.offset + index + 1;
                self.characters = 0;
            } else if byte & 0xC0 != 0x80 {
                // Continuation bytes belong to a character already counted.
                self.characters += 1;
            }
        }
        self.offset = offset;
        (self.line, self.characters + 1)
    }
}

#[cfg(test)]
fn line_column(source: &str, offset: usize) -> (usize, usize) {
    SourceCursor::new(source).line_column(offset)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forward_and_backward_search_wrap_deterministically() {
        let source = "R1 in 0 1k\nR2 out 0 2k\nR1 sense 0 3k\n";
        let options = FindOptions::default();
        let first = find_in_source(source, "R1", 1, options)
            .expect("valid search")
            .expect("match");
        assert_eq!(first.byte_range(), 23..25);
        assert!(!first.wrapped());

        let wrapped = find_in_source(source, "R1", source.len(), options)
            .expect("valid search")
            .expect("match");
        assert_eq!(wrapped.byte_range(), 0..2);
        assert!(wrapped.wrapped());

        let previous = find_in_source(
            source,
            "R1",
            23,
            FindOptions {
                direction: FindDirection::Backward,
                ..options
            },
        )
        .expect("valid search")
        .expect("match");
        assert_eq!(previous.byte_range(), 0..2);
    }

    #[test]
    fn unicode_search_returns_original_byte_range_and_scalar_coordinates() {
        let source = "* μ source\n.param ΜAX=2\nVμ μ 0 1\n";
        let found = find_in_source(
            source,
            "μax",
            0,
            FindOptions {
                match_case: false,
                ..FindOptions::default()
            },
        )
        .expect("valid search")
        .expect("match");
        assert_eq!(&source[found.byte_range()], "ΜAX");
        assert_eq!((found.line(), found.column()), (2, 8));
    }

    #[test]
    fn whole_word_uses_unicode_boundaries() {
        let source = ".param gain=1 gain_stage=2 μgain=3\n";
        let found = find_in_source(
            source,
            "gain",
            0,
            FindOptions {
                whole_word: true,
                ..FindOptions::default()
            },
        )
        .expect("valid search")
        .expect("whole word");
        assert_eq!(found.byte_range(), 7..11);
        assert_eq!(
            find_in_source(
                source,
                "stage",
                0,
                FindOptions {
                    whole_word: true,
                    ..FindOptions::default()
                }
            )
            .expect("valid search"),
            None
        );
    }

    #[test]
    fn invalid_utf8_offsets_are_rejected_instead_of_rounded() {
        assert_eq!(
            find_in_source("μ", "μ", 1, FindOptions::default()),
            Err(FindError::OffsetInsideCharacter(1))
        );
        assert!(matches!(
            find_in_source("x", "x", 2, FindOptions::default()),
            Err(FindError::OffsetOutsideSource { .. })
        ));
        assert_eq!(
            find_in_source("x", "", 0, FindOptions::default()),
            Err(FindError::EmptyQuery)
        );
    }

    #[test]
    fn regular_expression_find_is_unicode_case_aware_and_rejects_invalid_patterns() {
        let source = "Rμ μ 0 1k\nRΔ Δ 0 2k\n";
        let options = FindOptions {
            match_case: false,
            regular_expression: true,
            ..FindOptions::default()
        };
        let found = find_in_source(source, r"r(μ|δ)", 0, options)
            .expect("valid expression")
            .expect("match");
        assert_eq!(&source[found.byte_range()], "Rμ");
        assert!(matches!(
            find_in_source(
                source,
                "(",
                0,
                FindOptions {
                    regular_expression: true,
                    ..FindOptions::default()
                }
            ),
            Err(FindError::InvalidRegularExpression(_))
        ));
    }

    #[test]
    fn bounded_search_reports_truncation_without_retaining_extra_matches() {
        let found = find_all_in_source_bounded("x x x x", "x", FindOptions::default(), 2).unwrap();
        assert_eq!(found.matches().len(), 2);
        assert!(found.truncated());

        let exact = find_all_in_source_bounded("x x", "x", FindOptions::default(), 2).unwrap();
        assert_eq!(exact.matches().len(), 2);
        assert!(!exact.truncated());
    }

    #[test]
    fn literal_replace_next_wraps_and_treats_dollar_text_exactly() {
        let source = ".param gain=1\n.param gain=2\n";
        let outcome = replace_in_source(
            source,
            "gain",
            "$gain",
            FindOptions::default(),
            ReplaceScope::Next {
                caret_byte: source.len(),
            },
        )
        .expect("replace next");
        assert_eq!(outcome.source(), ".param $gain=1\n.param gain=2\n");
        assert_eq!(outcome.replacement_count(), 1);
        assert_eq!(outcome.replaced_ranges().len(), 1);
        assert_eq!(outcome.replaced_ranges()[0], 7..12);
        assert!(outcome.wrapped());
    }

    #[test]
    fn regex_replace_all_expands_named_captures_and_returns_new_ranges() {
        let source = "R1 in 0 1k\nR22 out 0 2k\n";
        let outcome = replace_in_source(
            source,
            r"R(?P<number>\d+)",
            "X${number}_copy",
            FindOptions {
                regular_expression: true,
                ..FindOptions::default()
            },
            ReplaceScope::All,
        )
        .expect("replace all");
        assert_eq!(outcome.source(), "X1_copy in 0 1k\nX22_copy out 0 2k\n");
        assert_eq!(outcome.replaced_ranges(), &[0..7, 16..24]);
        assert!(!outcome.wrapped());
    }

    #[test]
    fn regex_replace_respects_whole_unicode_symbol_boundaries() {
        let source = "gain gain_stage μgain gain\n";
        let outcome = replace_in_source(
            source,
            r"g(?:ain)",
            "G",
            FindOptions {
                whole_word: true,
                regular_expression: true,
                ..FindOptions::default()
            },
            ReplaceScope::All,
        )
        .expect("replace all");
        assert_eq!(outcome.source(), "G gain_stage μgain G\n");
        assert_eq!(outcome.replacement_count(), 2);
    }

    #[test]
    fn zero_width_regex_replacement_advances_on_utf8_boundaries() {
        let outcome = replace_in_source(
            "μΔ",
            r"^|$",
            "·",
            FindOptions {
                regular_expression: true,
                ..FindOptions::default()
            },
            ReplaceScope::All,
        )
        .expect("zero-width replace");
        assert_eq!(outcome.source(), "·μΔ·");
        assert_eq!(outcome.replacement_count(), 2);
    }

    #[test]
    fn find_all_returns_source_order_lines_and_safe_zero_width_matches() {
        let source = "μ\nR1 μ 0 1k\nR2 μ 0 2k\n";
        let found = find_all_in_source_bounded(source, "μ", FindOptions::default(), 16)
            .expect("literal matches");
        assert_eq!(
            found
                .matches()
                .iter()
                .map(|found| (found.line(), found.column()))
                .collect::<Vec<_>>(),
            vec![(1, 1), (2, 4), (3, 4)]
        );
        assert!(!found.truncated());
        assert!(found.matches().iter().all(|found| !found.wrapped()));

        let zero_width = find_all_in_source_bounded(
            "μΔ",
            r"(?:)",
            FindOptions {
                regular_expression: true,
                ..FindOptions::default()
            },
            16,
        )
        .expect("zero-width matches");
        assert_eq!(
            zero_width
                .matches()
                .iter()
                .map(FindMatch::byte_range)
                .collect::<Vec<_>>(),
            vec![0..0, 2..2, 4..4]
        );
    }

    /// A result set walks the source once instead of once per match, which is
    /// the difference between a search and a hang on a large deck. The
    /// coordinate it reports has to be the one the source states at every
    /// boundary, and a producer that hands back an earlier offset has to get
    /// the same answer as one that does not.
    #[test]
    fn source_coordinates_survive_one_forward_pass_and_a_rewind() {
        fn stated(source: &str, offset: usize) -> (usize, usize) {
            let prefix = &source[..offset];
            let line_start = prefix.rfind('\n').map_or(0, |index| index + 1);
            (
                prefix.bytes().filter(|byte| *byte == b'\n').count() + 1,
                source[line_start..offset].chars().count() + 1,
            )
        }

        let source = "μαβ head\nsecond card\n\n.param Δ = 2\n";
        let boundaries = (0..=source.len())
            .filter(|offset| source.is_char_boundary(*offset))
            .collect::<Vec<_>>();

        let mut forward = SourceCursor::new(source);
        for offset in &boundaries {
            assert_eq!(forward.line_column(*offset), stated(source, *offset));
        }
        let mut backward = SourceCursor::new(source);
        for offset in boundaries.iter().rev() {
            assert_eq!(backward.line_column(*offset), stated(source, *offset));
        }
    }

    /// A one-character query against a three-megabyte deck.
    ///
    /// This matched a hundred and fifty thousand times and took the better
    /// part of a minute, on every frame the find surface was open, because
    /// each match derived its line by counting newlines from the start of the
    /// deck. Both spellings of the search are gated: case folding used to
    /// build a second copy of the source and a boundary table sixteen bytes
    /// wide per character before it looked at anything.
    #[test]
    fn a_dense_query_over_a_large_deck_is_bounded_in_both_time_and_results() {
        let mut source = String::new();
        for card in 0..100_000 {
            source.push_str(&format!("R{card} n{card} n{} 1k\n", card + 1));
        }

        for match_case in [true, false] {
            let options = FindOptions {
                match_case,
                ..FindOptions::default()
            };
            let started = crate::time_compat::Instant::now();
            let found =
                find_all_in_source_bounded(&source, "1", options, 500).expect("valid search");
            let elapsed = started.elapsed();

            assert_eq!(found.matches().len(), 500);
            assert!(found.truncated(), "the deck holds far more than 500");
            assert!(
                found
                    .matches()
                    .windows(2)
                    .all(|pair| pair[0].line() <= pair[1].line()),
                "matches must stay in source order"
            );
            assert!(
                elapsed < std::time::Duration::from_millis(100),
                "match_case {match_case} took {elapsed:?}"
            );
        }

        // A query that matches nothing still has to read the deck exactly once.
        let started = crate::time_compat::Instant::now();
        let missing = find_all_in_source_bounded(&source, "zzz", FindOptions::default(), 500)
            .expect("valid search");
        let elapsed = started.elapsed();
        assert!(missing.matches().is_empty());
        assert!(!missing.truncated());
        assert!(
            elapsed < std::time::Duration::from_millis(500),
            "a full scan took {elapsed:?}"
        );
    }
}
