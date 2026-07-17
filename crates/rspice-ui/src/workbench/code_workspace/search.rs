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

/// Return every non-overlapping match in source order for match-count and
/// exact-line result lists. The regex engine advances zero-width matches on
/// UTF-8 boundaries, so this operation cannot stall on empty expressions.
pub fn find_all_in_source(
    source: &str,
    query: &str,
    options: FindOptions,
) -> Result<Vec<FindMatch>, FindError> {
    validate_request(source, query, 0)?;
    Ok(candidate_ranges(source, query, options)?
        .into_iter()
        .map(|byte_range| {
            let (line, column) = line_column(source, byte_range.start);
            FindMatch {
                byte_range,
                line,
                column,
                wrapped: false,
            }
        })
        .collect())
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
    for range in selected {
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
    let (folded_source, boundaries) = lowercase_with_boundaries(source);
    let folded_query = query.to_lowercase();
    if folded_query.is_empty() {
        return Vec::new();
    }

    folded_source
        .match_indices(&folded_query)
        .filter_map(|(start, value)| {
            let end = start + value.len();
            let original_start = boundaries
                .binary_search_by_key(&start, |(folded, _)| *folded)
                .ok()
                .map(|index| boundaries[index].1)?;
            let original_end = boundaries
                .binary_search_by_key(&end, |(folded, _)| *folded)
                .ok()
                .map(|index| boundaries[index].1)?;
            Some(original_start..original_end)
        })
        .collect()
}

/// Return folded text plus every legal `(folded byte, original byte)` source
/// character boundary, including the terminal boundary.
fn lowercase_with_boundaries(source: &str) -> (String, Vec<(usize, usize)>) {
    let mut folded = String::new();
    let mut boundaries = Vec::with_capacity(source.chars().count() + 1);
    for (original_offset, ch) in source.char_indices() {
        boundaries.push((folded.len(), original_offset));
        folded.extend(ch.to_lowercase());
    }
    boundaries.push((folded.len(), source.len()));
    (folded, boundaries)
}

fn is_whole_word(source: &str, range: &Range<usize>) -> bool {
    let left = source[..range.start].chars().next_back();
    let right = source[range.end..].chars().next();
    !left.is_some_and(is_word_character) && !right.is_some_and(is_word_character)
}

fn is_word_character(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

fn line_column(source: &str, offset: usize) -> (usize, usize) {
    let prefix = &source[..offset];
    let line = prefix.bytes().filter(|byte| *byte == b'\n').count() + 1;
    let line_start = prefix.rfind('\n').map_or(0, |index| index + 1);
    let column = source[line_start..offset].chars().count() + 1;
    (line, column)
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
        let matches =
            find_all_in_source(source, "μ", FindOptions::default()).expect("literal matches");
        assert_eq!(
            matches
                .iter()
                .map(|found| (found.line(), found.column()))
                .collect::<Vec<_>>(),
            vec![(1, 1), (2, 4), (3, 4)]
        );
        assert!(matches.iter().all(|found| !found.wrapped()));

        let zero_width = find_all_in_source(
            "μΔ",
            r"(?:)",
            FindOptions {
                regular_expression: true,
                ..FindOptions::default()
            },
        )
        .expect("zero-width matches");
        assert_eq!(
            zero_width
                .iter()
                .map(FindMatch::byte_range)
                .collect::<Vec<_>>(),
            vec![0..0, 2..2, 4..4]
        );
    }
}
