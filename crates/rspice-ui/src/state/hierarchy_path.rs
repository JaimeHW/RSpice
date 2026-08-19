//! The one hierarchical instance-path grammar.
//!
//! This module is the only owner of hierarchical path grammar. Every consumer
//! — design hierarchy, configuration sets, netlist emission, saved outputs,
//! probes, and persistence — imports [`InstancePath`], [`InstancePathPattern`],
//! or [`ProbeTarget`] from here. Splitting a path on `/`, `.`, or `:`
//! elsewhere, or keying a map on anything but [`InstancePath::fold_key`], is a
//! ratchet violation checked by `tests/hierarchy_guards.rs`.
//!
//! The design root is implicit, as it is in Cadence tools: it renders as `/`
//! and never occupies a segment. A stored path therefore names instances only,
//! so renaming or re-rooting the top cell cannot invalidate one.
//!
//! Three surface grammars meet here and exactly one of them is canonical.
//!
//! - Canonical — `/` for the root, `/X1/X2` otherwise. The only form stored,
//!   displayed, or serialized.
//! - Engine — `x1.x2.net`, with `:` accepted as an equal separator. Parsed
//!   here and emitted only by [`InstancePath::to_engine_name`] and
//!   [`InstancePath::engine_name_for`], because the engine names flattened
//!   nodes this way and cannot name a non-ASCII instance at all.
//! - Legacy — the spelling from before the root was implicit, in which the
//!   root occupied a literal `top` segment. [`InstancePath::parse_legacy`]
//!   accepts it and drops that one segment; deserialization uses it so
//!   projects saved with `/top/...` load unchanged.
//!
//! Case is not identity. Two paths that differ only in case name the same
//! instance, so comparison and map keys fold case, while the segments keep the
//! author's spelling for display. The fold is per character, which is what
//! keeps [`InstancePath::fold_key`] equality and segment comparison from ever
//! disagreeing — `str::to_lowercase` applies Unicode's final-sigma rule and a
//! per-character fold does not.

use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

/// Maximum rendered length of a path or pattern, in bytes.
///
/// This module owns the limit; a consumer that restates it can drift from it.
pub const MAX_INSTANCE_PATH_BYTES: usize = 1_024;

/// Maximum number of instance segments in a path or pattern.
pub const MAX_INSTANCE_PATH_DEPTH: usize = 128;

/// The segment the pre-implicit-root grammar spent on the design root.
const LEGACY_ROOT_SEGMENT: &str = "top";

/// Separator the engine uses between hierarchy levels.
const ENGINE_SEPARATOR: char = '.';

/// Separators accepted in an engine path. The engine emits only `.`; `:` is
/// the alias the netlist front end has always taken.
const ENGINE_SEPARATORS: [char; 2] = ['.', ':'];

/// The wildcard that stands for exactly one instance in a pattern.
const PATTERN_WILDCARD: &str = "*";

/// An absolute path to a design instance, rooted at the implicit design root.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct InstancePath {
    segments: Vec<String>,
}

impl InstancePath {
    /// The design root, which owns no segment and renders as `/`.
    pub fn root() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    /// Parses a canonical path (`/X1/X2`) or an engine path (`x1.x2`).
    pub fn parse(text: &str) -> Result<Self, HierarchyPathError> {
        Self::sealed(parse_path_segments(text)?)
    }

    /// Parses everything [`InstancePath::parse`] accepts, then drops one
    /// leading `top` segment so paths written against the old grammar name the
    /// same instance: `/top` is the root, `/top/XAFE` is `/XAFE`, and `/XAFE`
    /// is unchanged.
    pub fn parse_legacy(text: &str) -> Result<Self, HierarchyPathError> {
        let mut segments = parse_path_segments(text)?;
        if segments
            .first()
            .is_some_and(|first| first.eq_ignore_ascii_case(LEGACY_ROOT_SEGMENT))
        {
            segments.remove(0);
        }
        Self::sealed(segments)
    }

    pub fn is_root(&self) -> bool {
        self.segments.is_empty()
    }

    pub fn depth(&self) -> usize {
        self.segments.len()
    }

    /// The instance names, outermost first, in the author's spelling.
    pub fn segments(&self) -> &[String] {
        &self.segments
    }

    /// The enclosing instance, or `None` at the root.
    pub fn parent(&self) -> Option<Self> {
        let (_, parent) = self.segments.split_last()?;
        Some(Self {
            segments: parent.to_vec(),
        })
    }

    /// Descends into a named child instance.
    pub fn child(&self, name: &str) -> Result<Self, HierarchyPathError> {
        let mut segments = self.segments.clone();
        segments.push(name.to_owned());
        validate_segment(&render(&segments), name)?;
        Self::sealed(segments)
    }

    /// Appends another path's segments below this one.
    ///
    /// Both operands are already valid, but their concatenation can exceed the
    /// depth or byte limit, so the limits are re-checked rather than assumed.
    pub fn join(&self, tail: &Self) -> Result<Self, HierarchyPathError> {
        let mut segments = self.segments.clone();
        segments.extend(tail.segments.iter().cloned());
        Self::sealed(segments)
    }

    /// Whether `prefix` names this instance or one of its ancestors. The root
    /// is a prefix of everything.
    pub fn starts_with(&self, prefix: &Self) -> bool {
        prefix.segments.len() <= self.segments.len()
            && prefix
                .segments
                .iter()
                .zip(&self.segments)
                .all(|(expected, actual)| fold_eq(expected, actual))
    }

    /// The remainder below `prefix`, re-rooted, or `None` when `prefix` does
    /// not enclose this path.
    pub fn strip_prefix(&self, prefix: &Self) -> Option<Self> {
        self.starts_with(prefix).then(|| Self {
            segments: self.segments[prefix.segments.len()..].to_vec(),
        })
    }

    /// This path with the `from` prefix replaced by `to` — the operation a
    /// re-parented or renamed instance forces on every path beneath it.
    ///
    /// `None` when `from` does not enclose this path, or when the result would
    /// exceed the depth or byte limit.
    pub fn remap_prefix(&self, from: &Self, to: &Self) -> Option<Self> {
        let tail = self.strip_prefix(from)?;
        to.join(&tail).ok()
    }

    /// This path with one segment renamed, which is what an instance rename
    /// does to the paths that pass through it.
    pub fn rename_segment(&self, index: usize, name: &str) -> Result<Self, HierarchyPathError> {
        if index >= self.segments.len() {
            return Err(HierarchyPathError::NoSuchSegment {
                path: self.to_string(),
                index,
                depth: self.segments.len(),
            });
        }
        let mut segments = self.segments.clone();
        segments[index] = name.to_owned();
        validate_segment(&render(&segments), name)?;
        Self::sealed(segments)
    }

    /// The case-folded rendering, and the only key a map or dedupe may use.
    pub fn fold_key(&self) -> String {
        if self.segments.is_empty() {
            return "/".to_owned();
        }
        let mut key = String::with_capacity(rendered_len(&self.segments));
        for segment in &self.segments {
            key.push('/');
            fold_into(&mut key, segment);
        }
        key
    }

    /// The flattened node scope the engine uses, `x1.x2`.
    ///
    /// The root has no engine name: a flattened node at the root is named by
    /// its leaf alone, which is what [`InstancePath::engine_name_for`] gives.
    pub fn to_engine_name(&self) -> Result<String, HierarchyPathError> {
        if self.segments.is_empty() {
            return Err(HierarchyPathError::RootHasNoEngineName);
        }
        let mut name = String::with_capacity(rendered_len(&self.segments));
        for segment in &self.segments {
            require_ascii(segment)?;
            if !name.is_empty() {
                name.push(ENGINE_SEPARATOR);
            }
            name.push_str(&segment.to_ascii_lowercase());
        }
        Ok(name)
    }

    /// The engine's name for a signal or device `leaf` inside this instance:
    /// `x1.x2.net` below the hierarchy, and the bare leaf at the root.
    pub fn engine_name_for(&self, leaf: &str) -> Result<String, HierarchyPathError> {
        validate_leaf(leaf)?;
        require_ascii(leaf)?;
        let leaf = leaf.to_ascii_lowercase();
        if self.segments.is_empty() {
            return Ok(leaf);
        }
        let scope = self.to_engine_name()?;
        Ok(format!("{scope}{ENGINE_SEPARATOR}{leaf}"))
    }

    /// Applies the limits that every constructor must hold. Only a rejected
    /// path is rendered, so the accepted case costs no allocation.
    fn sealed(segments: Vec<String>) -> Result<Self, HierarchyPathError> {
        if segments.len() > MAX_INSTANCE_PATH_DEPTH {
            return Err(HierarchyPathError::TooDeep {
                path: render(&segments),
                actual: segments.len(),
                maximum: MAX_INSTANCE_PATH_DEPTH,
            });
        }
        let bytes = rendered_len(&segments);
        if bytes > MAX_INSTANCE_PATH_BYTES {
            return Err(HierarchyPathError::TooLong {
                path: render(&segments),
                actual: bytes,
                maximum: MAX_INSTANCE_PATH_BYTES,
            });
        }
        Ok(Self { segments })
    }
}

impl fmt::Display for InstancePath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.segments.is_empty() {
            return formatter.write_str("/");
        }
        for segment in &self.segments {
            write!(formatter, "/{segment}")?;
        }
        Ok(())
    }
}

impl Serialize for InstancePath {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for InstancePath {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        Self::parse_legacy(&text).map_err(serde::de::Error::custom)
    }
}

/// One position in a pattern: a named instance, or the wildcard that stands
/// for exactly one instance at that level.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PatternSegment {
    Name(String),
    Star,
}

impl fmt::Display for PatternSegment {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Name(name) => formatter.write_str(name),
            Self::Star => formatter.write_str(PATTERN_WILDCARD),
        }
    }
}

/// A scoped pattern over instance paths, as configuration overrides and
/// required-instance policies write them.
///
/// A pattern matches at one depth only: `*` stands for exactly one instance,
/// never for a subtree, so `/*/X1` matches `/I0/X1` and never `/X1`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InstancePathPattern {
    segments: Vec<PatternSegment>,
}

impl InstancePathPattern {
    pub fn parse(text: &str) -> Result<Self, HierarchyPathError> {
        Self::sealed(parse_pattern_segments(text)?)
    }

    /// Parses a pattern written against the old grammar, dropping one leading
    /// `top` segment the way [`InstancePath::parse_legacy`] does. A leading
    /// wildcard is never dropped.
    pub fn parse_legacy(text: &str) -> Result<Self, HierarchyPathError> {
        let mut segments = parse_pattern_segments(text)?;
        if segments.first().is_some_and(|first| {
            matches!(first, PatternSegment::Name(name) if name.eq_ignore_ascii_case(LEGACY_ROOT_SEGMENT))
        }) {
            segments.remove(0);
        }
        Self::sealed(segments)
    }

    pub fn is_root(&self) -> bool {
        self.segments.is_empty()
    }

    pub fn depth(&self) -> usize {
        self.segments.len()
    }

    pub fn segments(&self) -> &[PatternSegment] {
        &self.segments
    }

    /// Whether this pattern names `path`.
    pub fn matches(&self, path: &InstancePath) -> bool {
        self.segments.len() == path.segments.len()
            && self
                .segments
                .iter()
                .zip(&path.segments)
                .all(|(expected, actual)| match expected {
                    PatternSegment::Star => true,
                    PatternSegment::Name(name) => fold_eq(name, actual),
                })
    }

    /// How many positions the pattern pins by name. Precedence between two
    /// matching patterns is decided by this and nothing else.
    pub fn specificity(&self) -> usize {
        self.segments
            .iter()
            .filter(|segment| !matches!(segment, PatternSegment::Star))
            .count()
    }

    /// Whether some path could match both patterns. Two overlapping patterns
    /// of equal specificity are ambiguous, which is the condition an override
    /// catalog must reject.
    pub fn overlaps(&self, other: &Self) -> bool {
        self.segments.len() == other.segments.len()
            && self
                .segments
                .iter()
                .zip(&other.segments)
                .all(|(left, right)| match (left, right) {
                    (PatternSegment::Star, _) | (_, PatternSegment::Star) => true,
                    (PatternSegment::Name(left), PatternSegment::Name(right)) => {
                        fold_eq(left, right)
                    }
                })
    }

    fn sealed(segments: Vec<PatternSegment>) -> Result<Self, HierarchyPathError> {
        let pattern = Self { segments };
        let rendered = pattern.to_string();
        if pattern.segments.len() > MAX_INSTANCE_PATH_DEPTH {
            return Err(HierarchyPathError::TooDeep {
                path: rendered,
                actual: pattern.segments.len(),
                maximum: MAX_INSTANCE_PATH_DEPTH,
            });
        }
        if rendered.len() > MAX_INSTANCE_PATH_BYTES {
            return Err(HierarchyPathError::TooLong {
                actual: rendered.len(),
                path: rendered,
                maximum: MAX_INSTANCE_PATH_BYTES,
            });
        }
        Ok(pattern)
    }
}

impl fmt::Display for InstancePathPattern {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.segments.is_empty() {
            return formatter.write_str("/");
        }
        for segment in &self.segments {
            write!(formatter, "/{segment}")?;
        }
        Ok(())
    }
}

impl Serialize for InstancePathPattern {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for InstancePathPattern {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        Self::parse_legacy(&text).map_err(serde::de::Error::custom)
    }
}

/// A signal or device probe: the instance whose scope it is read in, and the
/// leaf name inside that instance.
///
/// The leaf is not an instance name. Node and device names carry `$`, `+`, and
/// `-`, none of which may appear in a path segment, so the two grammars stay
/// separate and only meet in [`InstancePath::engine_name_for`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProbeTarget {
    pub scope: InstancePath,
    pub leaf: String,
}

impl ProbeTarget {
    /// Parses `leaf`, `/X1/leaf`, or `x1.x2.leaf`. The scope prefix is written
    /// in one grammar or the other, never mixed.
    pub fn parse(text: &str) -> Result<Self, HierarchyPathError> {
        Self::parse_scoped(text, InstancePath::parse)
    }

    /// Parses everything [`ProbeTarget::parse`] accepts, resolving the scope
    /// through [`InstancePath::parse_legacy`] so a probe saved as
    /// `/top/X1/net` names `/X1/net`.
    pub fn parse_legacy(text: &str) -> Result<Self, HierarchyPathError> {
        Self::parse_scoped(text, InstancePath::parse_legacy)
    }

    /// The engine's flattened name for this probe, `x1.x2.net`.
    pub fn engine_name(&self) -> Result<String, HierarchyPathError> {
        self.scope.engine_name_for(&self.leaf)
    }

    fn parse_scoped(
        text: &str,
        parse_scope: fn(&str) -> Result<InstancePath, HierarchyPathError>,
    ) -> Result<Self, HierarchyPathError> {
        if text.is_empty() {
            return Err(HierarchyPathError::Empty);
        }
        if text.len() > MAX_INSTANCE_PATH_BYTES {
            return Err(HierarchyPathError::TooLong {
                path: text.to_owned(),
                actual: text.len(),
                maximum: MAX_INSTANCE_PATH_BYTES,
            });
        }
        let (scope, leaf) = if let Some(index) = text.rfind('/') {
            if !text.starts_with('/') {
                return Err(HierarchyPathError::MalformedProbe(text.to_owned()));
            }
            (&text[..index], &text[index + 1..])
        } else if let Some(index) = text.rfind(ENGINE_SEPARATORS) {
            (&text[..index], &text[index + 1..])
        } else {
            ("", text)
        };
        validate_leaf(leaf)?;
        let scope = if scope.is_empty() {
            InstancePath::root()
        } else {
            parse_scope(scope)?
        };
        Ok(Self {
            scope,
            leaf: leaf.to_owned(),
        })
    }
}

impl fmt::Display for ProbeTarget {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.scope.is_root() {
            return formatter.write_str(&self.leaf);
        }
        write!(formatter, "{}/{}", self.scope, self.leaf)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum HierarchyPathError {
    #[error("a hierarchical path is required")]
    Empty,
    #[error("hierarchy pattern {0:?} must begin with '/'")]
    NotRooted(String),
    #[error("hierarchical path {0:?} has an empty segment; the design root is written '/'")]
    EmptySegment(String),
    #[error(
        "instance name {segment:?} in {path:?} contains unsupported character {character:?}; \
         instance names use Unicode letters, digits, and '_'"
    )]
    InvalidSegment {
        path: String,
        segment: String,
        character: char,
    },
    #[error("'*' matches instances only in a hierarchy pattern, and {0:?} is an instance path")]
    WildcardInPath(String),
    #[error("hierarchical path {path:?} is {actual} segments deep; the maximum is {maximum}")]
    TooDeep {
        path: String,
        actual: usize,
        maximum: usize,
    },
    #[error("hierarchical path {path:?} is {actual} bytes long; the maximum is {maximum}")]
    TooLong {
        path: String,
        actual: usize,
        maximum: usize,
    },
    #[error("segment {index} does not exist in {path:?}, which is {depth} deep")]
    NoSuchSegment {
        path: String,
        index: usize,
        depth: usize,
    },
    #[error("the design root has no engine node name")]
    RootHasNoEngineName,
    #[error("the engine cannot name {0:?}; engine node names are ASCII")]
    NonAsciiEngineName(String),
    #[error("probe target {0:?} must be written /X1/X2/leaf or x1.x2.leaf")]
    MalformedProbe(String),
    #[error("a signal name is required")]
    EmptyLeaf,
    #[error(
        "signal name {leaf:?} contains unsupported character {character:?}; signal names use \
         Unicode letters, digits, '_', '$', '+', and '-'"
    )]
    InvalidLeaf { leaf: String, character: char },
}

/// Splits a canonical or engine path into validated segments.
fn parse_path_segments(text: &str) -> Result<Vec<String>, HierarchyPathError> {
    split_hierarchy(text)?
        .into_iter()
        .map(|raw| {
            validate_segment(text, raw)?;
            Ok(raw.to_owned())
        })
        .collect()
}

/// Splits a pattern into validated segments. Patterns are authored in the
/// canonical grammar only, so an engine path is not a pattern.
fn parse_pattern_segments(text: &str) -> Result<Vec<PatternSegment>, HierarchyPathError> {
    if text.is_empty() {
        return Err(HierarchyPathError::Empty);
    }
    if !text.starts_with('/') {
        return Err(HierarchyPathError::NotRooted(text.to_owned()));
    }
    split_hierarchy(text)?
        .into_iter()
        .map(|raw| {
            if raw == PATTERN_WILDCARD {
                return Ok(PatternSegment::Star);
            }
            validate_segment(text, raw)?;
            Ok(PatternSegment::Name(raw.to_owned()))
        })
        .collect()
}

/// Splits either accepted grammar into raw, still unvalidated segments.
fn split_hierarchy(text: &str) -> Result<Vec<&str>, HierarchyPathError> {
    if text.is_empty() {
        return Err(HierarchyPathError::Empty);
    }
    if text.len() > MAX_INSTANCE_PATH_BYTES {
        return Err(HierarchyPathError::TooLong {
            path: text.to_owned(),
            actual: text.len(),
            maximum: MAX_INSTANCE_PATH_BYTES,
        });
    }
    let segments = match text.strip_prefix('/') {
        Some("") => Vec::new(),
        Some(rest) => rest.split('/').collect(),
        None => text.split(ENGINE_SEPARATORS).collect(),
    };
    if segments.len() > MAX_INSTANCE_PATH_DEPTH {
        return Err(HierarchyPathError::TooDeep {
            path: text.to_owned(),
            actual: segments.len(),
            maximum: MAX_INSTANCE_PATH_DEPTH,
        });
    }
    Ok(segments)
}

fn validate_segment(path: &str, segment: &str) -> Result<(), HierarchyPathError> {
    if segment.is_empty() {
        return Err(HierarchyPathError::EmptySegment(path.to_owned()));
    }
    if segment == PATTERN_WILDCARD {
        return Err(HierarchyPathError::WildcardInPath(path.to_owned()));
    }
    match segment
        .chars()
        .find(|character| !is_segment_char(*character))
    {
        Some(character) => Err(HierarchyPathError::InvalidSegment {
            path: path.to_owned(),
            segment: segment.to_owned(),
            character,
        }),
        None => Ok(()),
    }
}

fn validate_leaf(leaf: &str) -> Result<(), HierarchyPathError> {
    if leaf.is_empty() {
        return Err(HierarchyPathError::EmptyLeaf);
    }
    match leaf.chars().find(|character| !is_leaf_char(*character)) {
        Some(character) => Err(HierarchyPathError::InvalidLeaf {
            leaf: leaf.to_owned(),
            character,
        }),
        None => Ok(()),
    }
}

fn require_ascii(name: &str) -> Result<(), HierarchyPathError> {
    if name.is_ascii() {
        Ok(())
    } else {
        Err(HierarchyPathError::NonAsciiEngineName(name.to_owned()))
    }
}

fn is_segment_char(character: char) -> bool {
    character.is_alphanumeric() || character == '_'
}

fn is_leaf_char(character: char) -> bool {
    // `#` is the deck's own spelling for bus bits (`data#3`) and branch
    // currents (`v1#branch`); a leaf grammar that refuses it cannot name the
    // very signals the engine solves for.
    is_segment_char(character) || matches!(character, '$' | '+' | '-' | '#')
}

/// Length of the canonical rendering, without building it.
fn rendered_len(segments: &[String]) -> usize {
    segments
        .iter()
        .map(|segment| segment.len() + 1)
        .sum::<usize>()
        .max(1)
}

fn render(segments: &[String]) -> String {
    if segments.is_empty() {
        return "/".to_owned();
    }
    let mut rendered = String::with_capacity(rendered_len(segments));
    for segment in segments {
        rendered.push('/');
        rendered.push_str(segment);
    }
    rendered
}

/// Per-character case fold. Used by every comparison and by `fold_key`, so the
/// two can never disagree.
fn fold_into(target: &mut String, text: &str) {
    target.extend(text.chars().flat_map(char::to_lowercase));
}

fn fold_eq(left: &str, right: &str) -> bool {
    left.chars()
        .flat_map(char::to_lowercase)
        .eq(right.chars().flat_map(char::to_lowercase))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_paths_round_trip_and_malformed_text_is_rejected() {
        for text in ["/", "/X1", "/X1/X2", "/x1/X2/x3", "/X_1/net_2"] {
            let path = InstancePath::parse(text).expect("canonical path parses");
            assert_eq!(path.to_string(), text);
        }
        assert_eq!(InstancePath::parse("/"), Ok(InstancePath::root()));
        assert!(InstancePath::root().is_root());
        assert_eq!(InstancePath::default(), InstancePath::root());
        assert_eq!(InstancePath::parse("/X1/X2").expect("depth").depth(), 2);

        assert_eq!(InstancePath::parse(""), Err(HierarchyPathError::Empty));
        for malformed in ["//", "/X1/", "/X1//X2", "/X1/ /X2"] {
            assert!(
                matches!(
                    InstancePath::parse(malformed),
                    Err(HierarchyPathError::EmptySegment(_)
                        | HierarchyPathError::InvalidSegment { .. })
                ),
                "{malformed} must be rejected"
            );
        }
        assert!(matches!(
            InstancePath::parse("/X1/*"),
            Err(HierarchyPathError::WildcardInPath(_))
        ));
        assert!(matches!(
            InstancePath::parse("/X1 2"),
            Err(HierarchyPathError::InvalidSegment { character: ' ', .. })
        ));
        assert!(matches!(
            InstancePath::parse("/X1\u{7}"),
            Err(HierarchyPathError::InvalidSegment {
                character: '\u{7}',
                ..
            })
        ));

        let at_depth = "/X".repeat(MAX_INSTANCE_PATH_DEPTH);
        assert!(InstancePath::parse(&at_depth).is_ok());
        let too_deep = "/X".repeat(MAX_INSTANCE_PATH_DEPTH + 1);
        assert!(matches!(
            InstancePath::parse(&too_deep),
            Err(HierarchyPathError::TooDeep { actual, maximum, .. })
                if actual == MAX_INSTANCE_PATH_DEPTH + 1 && maximum == MAX_INSTANCE_PATH_DEPTH
        ));
        let at_limit = format!("/{}", "X".repeat(MAX_INSTANCE_PATH_BYTES - 1));
        assert!(InstancePath::parse(&at_limit).is_ok());
        let too_long = format!("/{}", "X".repeat(MAX_INSTANCE_PATH_BYTES));
        assert!(matches!(
            InstancePath::parse(&too_long),
            Err(HierarchyPathError::TooLong { actual, .. })
                if actual == MAX_INSTANCE_PATH_BYTES + 1
        ));
    }

    #[test]
    fn legacy_paths_lose_exactly_one_root_segment() {
        for (input, expected) in [
            ("/", "/"),
            ("/top", "/"),
            ("/TOP/xafe", "/xafe"),
            ("/top/XAFE", "/XAFE"),
            ("/XAFE", "/XAFE"),
            ("top.x1.net", "/x1/net"),
        ] {
            let path = InstancePath::parse_legacy(input).expect("legacy path parses");
            assert_eq!(path.to_string(), expected, "{input}");
            assert_eq!(
                InstancePath::parse_legacy(&path.to_string()).expect("migrated path re-parses"),
                path,
                "{input} must migrate idempotently"
            );
        }
        assert_eq!(
            InstancePath::parse_legacy("/top/top")
                .expect("nested instance named top")
                .to_string(),
            "/top",
            "only the root segment is dropped"
        );
        assert_eq!(
            InstancePath::parse("/top/XAFE")
                .expect("canonical parse keeps every segment")
                .to_string(),
            "/top/XAFE"
        );
    }

    #[test]
    fn engine_forms_parse_alike_and_only_engine_names_use_separators() {
        let dotted = InstancePath::parse("x1.x2.net").expect("dotted engine path");
        let coloned = InstancePath::parse("x1:x2:net").expect("coloned engine path");
        assert_eq!(dotted, coloned);
        assert_eq!(dotted.to_string(), "/x1/x2/net");
        assert_eq!(dotted.to_engine_name().expect("engine name"), "x1.x2.net");

        let scope = InstancePath::parse("/X1/X2").expect("scope");
        assert_eq!(scope.to_engine_name().expect("engine name"), "x1.x2");
        assert_eq!(
            scope.engine_name_for("Net").expect("engine node name"),
            "x1.x2.net"
        );
        assert_eq!(
            InstancePath::root()
                .engine_name_for("Vdd$")
                .expect("root node name"),
            "vdd$"
        );
        assert_eq!(
            InstancePath::root().to_engine_name(),
            Err(HierarchyPathError::RootHasNoEngineName)
        );
        assert!(matches!(
            scope.engine_name_for("net.other"),
            Err(HierarchyPathError::InvalidLeaf { character: '.', .. })
        ));
        assert_eq!(
            scope.engine_name_for(""),
            Err(HierarchyPathError::EmptyLeaf)
        );
        assert!(
            InstancePath::parse("/x1.x2").is_err(),
            "a canonical path never carries an engine separator"
        );
    }

    #[test]
    fn non_ascii_instances_are_nameable_in_the_design_and_not_in_the_engine() {
        let path = InstancePath::parse("/Xβ/X2").expect("unicode segment parses");
        assert_eq!(path.depth(), 2);
        assert_eq!(path.to_string(), "/Xβ/X2");
        assert_eq!(
            path.to_engine_name(),
            Err(HierarchyPathError::NonAsciiEngineName("Xβ".to_owned()))
        );
        assert!(matches!(
            path.engine_name_for("net"),
            Err(HierarchyPathError::NonAsciiEngineName(_))
        ));
        assert_eq!(
            InstancePath::root().engine_name_for("nétz"),
            Err(HierarchyPathError::NonAsciiEngineName("nétz".to_owned()))
        );
    }

    #[test]
    fn fold_key_is_the_only_identity() {
        let upper = InstancePath::parse_legacy("/TOP/xafe").expect("upper");
        let lower = InstancePath::parse_legacy("/top/XAFE").expect("lower");
        assert_ne!(upper, lower, "the author's spelling is preserved");
        assert_eq!(upper.fold_key(), lower.fold_key());
        assert_eq!(upper.fold_key(), "/xafe");
        assert_eq!(InstancePath::root().fold_key(), "/");
        assert_eq!(
            InstancePath::parse("/X1/X2")
                .expect("mixed case")
                .fold_key(),
            "/x1/x2"
        );

        let capital_sigma = InstancePath::parse("/XΣ").expect("capital sigma");
        let small_sigma = InstancePath::parse("/Xσ").expect("small sigma");
        assert_eq!(
            capital_sigma.fold_key(),
            small_sigma.fold_key(),
            "the fold is per character, so no final-sigma rule splits the key"
        );
        assert!(capital_sigma.starts_with(&small_sigma));
    }

    #[test]
    fn navigation_keeps_every_result_inside_the_grammar() {
        let root = InstancePath::root();
        assert_eq!(root.parent(), None);
        let x1 = root.child("X1").expect("child of root");
        assert_eq!(x1.to_string(), "/X1");
        let x1x2 = x1.child("X2").expect("grandchild");
        assert_eq!(x1x2.parent(), Some(x1.clone()));
        assert_eq!(x1.parent(), Some(root.clone()));
        assert_eq!(
            x1x2.segments()
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            ["X1", "X2"]
        );

        assert!(x1x2.starts_with(&root));
        assert!(x1x2.starts_with(&InstancePath::parse("/x1").expect("folded prefix")));
        assert!(!x1.starts_with(&x1x2));
        assert_eq!(
            x1x2.strip_prefix(&x1).expect("tail").to_string(),
            "/X2",
            "the tail is re-rooted"
        );
        assert_eq!(x1x2.strip_prefix(&root), Some(x1x2.clone()));
        assert_eq!(
            x1x2.strip_prefix(&InstancePath::parse("/X9").expect("other")),
            None
        );

        assert_eq!(
            x1.join(&InstancePath::parse("/X2/X3").expect("tail"))
                .expect("join")
                .to_string(),
            "/X1/X2/X3"
        );
        assert_eq!(x1.join(&root).expect("join root"), x1);
        assert_eq!(
            x1x2.rename_segment(0, "XA")
                .expect("rename outer")
                .to_string(),
            "/XA/X2"
        );
        assert!(matches!(
            x1x2.rename_segment(2, "XA"),
            Err(HierarchyPathError::NoSuchSegment {
                index: 2,
                depth: 2,
                ..
            })
        ));
        assert!(matches!(
            x1x2.rename_segment(0, "X A"),
            Err(HierarchyPathError::InvalidSegment { .. })
        ));
        assert!(matches!(
            x1.child("X 2"),
            Err(HierarchyPathError::InvalidSegment { .. })
        ));
        assert!(matches!(
            x1.child("*"),
            Err(HierarchyPathError::WildcardInPath(_))
        ));

        let renamed = InstancePath::parse("/Y1/Y2").expect("new prefix");
        assert_eq!(
            x1x2.remap_prefix(&x1, &renamed).expect("remap").to_string(),
            "/Y1/Y2/X2"
        );
        assert_eq!(x1x2.remap_prefix(&x1x2, &root), Some(root.clone()));
        assert_eq!(
            x1x2.remap_prefix(&InstancePath::parse("/X9").expect("other"), &renamed),
            None
        );
    }

    #[test]
    fn every_constructor_holds_the_depth_and_byte_limits() {
        let deepest =
            InstancePath::parse(&"/X".repeat(MAX_INSTANCE_PATH_DEPTH)).expect("at the depth limit");
        assert!(matches!(
            deepest.child("X"),
            Err(HierarchyPathError::TooDeep { .. })
        ));
        assert!(matches!(
            deepest.join(&deepest),
            Err(HierarchyPathError::TooDeep { .. })
        ));
        assert_eq!(
            deepest.remap_prefix(
                &InstancePath::parse("/X").expect("prefix"),
                &InstancePath::parse("/X/Y").expect("longer prefix")
            ),
            None,
            "a remap that would exceed the depth limit does not produce a path"
        );

        let widest = InstancePath::parse(&format!("/{}", "X".repeat(MAX_INSTANCE_PATH_BYTES - 1)))
            .expect("at the byte limit");
        assert!(matches!(
            widest.child("Y"),
            Err(HierarchyPathError::TooLong { .. })
        ));
        assert!(matches!(
            widest.rename_segment(0, &"X".repeat(MAX_INSTANCE_PATH_BYTES)),
            Err(HierarchyPathError::TooLong { .. })
        ));
    }

    #[test]
    fn probe_targets_split_a_scope_from_a_signal() {
        for (text, scope, leaf) in [
            ("net1", "/", "net1"),
            ("/net1", "/", "net1"),
            ("/vdd$", "/", "vdd$"),
            ("/X1/out+", "/X1", "out+"),
            ("/X1/X2/out-", "/X1/X2", "out-"),
            ("x1.x2.net", "/x1/x2", "net"),
            ("x1:x2:net", "/x1/x2", "net"),
            ("x1.data#3", "/x1", "data#3"),
            ("v1#branch", "/", "v1#branch"),
        ] {
            let probe = ProbeTarget::parse(text).expect("probe parses");
            assert_eq!(probe.scope.to_string(), scope, "{text}");
            assert_eq!(probe.leaf, leaf, "{text}");
        }
        assert_eq!(
            ProbeTarget::parse("/X1/Net")
                .expect("probe")
                .engine_name()
                .expect("engine name"),
            "x1.net"
        );
        assert_eq!(
            ProbeTarget::parse("/X1/DATA#3")
                .expect("probe")
                .engine_name()
                .expect("engine name"),
            "x1.data#3",
            "the deck's bus-bit spelling is a nameable leaf"
        );
        assert_eq!(
            ProbeTarget::parse_legacy("/top/X1/net")
                .expect("legacy probe")
                .to_string(),
            "/X1/net"
        );
        assert_eq!(
            ProbeTarget::parse("net").expect("bare leaf").to_string(),
            "net"
        );

        assert_eq!(ProbeTarget::parse(""), Err(HierarchyPathError::Empty));
        assert_eq!(
            ProbeTarget::parse("/X1/"),
            Err(HierarchyPathError::EmptyLeaf)
        );
        assert!(matches!(
            ProbeTarget::parse("X1/net"),
            Err(HierarchyPathError::MalformedProbe(_))
        ));
        assert!(
            matches!(
                ProbeTarget::parse("/X1/x2.net"),
                Err(HierarchyPathError::InvalidLeaf { character: '.', .. })
            ),
            "the two scope grammars are never mixed"
        );
    }

    #[test]
    fn patterns_match_and_rank_exactly_as_scoped_overrides_did() {
        let wildcard = InstancePathPattern::parse_legacy("/top/*").expect("wildcard");
        let specific = InstancePathPattern::parse_legacy("/top/Xcritical").expect("specific");
        let critical = InstancePath::parse_legacy("/top/xCRITICAL").expect("path");
        let other = InstancePath::parse_legacy("/top/Xother").expect("path");
        assert!(wildcard.matches(&critical));
        assert!(specific.matches(&critical));
        assert!(wildcard.matches(&other));
        assert!(!specific.matches(&other));
        assert!(specific.specificity() > wildcard.specificity());
        assert_eq!(wildcard.specificity(), 0);
        assert_eq!(wildcard.depth(), 1);
        assert!(
            !wildcard.matches(&InstancePath::parse_legacy("/top/X1/X2").expect("deeper path")),
            "a wildcard stands for one instance, not a subtree"
        );

        let left = InstancePathPattern::parse_legacy("/top/*/X1").expect("left");
        let right = InstancePathPattern::parse_legacy("/top/I0/*").expect("right");
        assert!(left.overlaps(&right));
        assert_eq!(left.specificity(), right.specificity());
        assert!(
            !wildcard.overlaps(&left),
            "patterns of unequal depth cannot overlap"
        );
        assert!(
            !InstancePathPattern::parse_legacy("/top/I0/X1")
                .expect("left")
                .overlaps(&InstancePathPattern::parse_legacy("/top/I1/X1").expect("right"))
        );
        assert!(
            InstancePathPattern::parse_legacy("/top/I0/X1")
                .expect("left")
                .overlaps(&InstancePathPattern::parse_legacy("/top/i0/x1").expect("right")),
            "overlap folds case"
        );

        let root = InstancePathPattern::parse("/").expect("root pattern");
        assert!(root.is_root());
        assert_eq!(root.to_string(), "/");
        assert!(root.matches(&InstancePath::root()));
        assert!(!root.matches(&InstancePath::parse("/X1").expect("path")));
        assert_eq!(
            InstancePathPattern::parse_legacy("/top/*/X1")
                .expect("legacy pattern")
                .to_string(),
            "/*/X1"
        );
        assert_eq!(
            InstancePathPattern::parse("/*/X1")
                .expect("pattern")
                .segments(),
            [PatternSegment::Star, PatternSegment::Name("X1".to_owned())]
        );

        assert!(
            matches!(
                InstancePathPattern::parse("/**"),
                Err(HierarchyPathError::InvalidSegment { character: '*', .. })
            ),
            "'**' was never a wildcard"
        );
        assert!(matches!(
            InstancePathPattern::parse("X1/*"),
            Err(HierarchyPathError::NotRooted(_))
        ));
        assert!(matches!(
            InstancePathPattern::parse("x1.x2"),
            Err(HierarchyPathError::NotRooted(_))
        ));
        assert_eq!(
            InstancePathPattern::parse(""),
            Err(HierarchyPathError::Empty)
        );
    }

    #[test]
    fn serde_renders_canonically_and_loads_legacy_projects() {
        let path = InstancePath::parse("/X1/X2").expect("path");
        assert_eq!(
            serde_json::to_string(&path).expect("serialize"),
            "\"/X1/X2\""
        );
        assert_eq!(
            serde_json::from_str::<InstancePath>("\"/top/X1/X2\"").expect("legacy path loads"),
            path
        );
        assert_eq!(
            serde_json::from_str::<InstancePath>("\"/top\"").expect("legacy root loads"),
            InstancePath::root()
        );
        assert_eq!(
            serde_json::from_str::<InstancePath>("\"/\"").expect("root loads"),
            InstancePath::root()
        );
        assert!(serde_json::from_str::<InstancePath>("\"/X1/\"").is_err());
        assert!(serde_json::from_str::<InstancePath>("\"/X1/*\"").is_err());

        let pattern = InstancePathPattern::parse("/*/X1").expect("pattern");
        assert_eq!(
            serde_json::to_string(&pattern).expect("serialize"),
            "\"/*/X1\""
        );
        assert_eq!(
            serde_json::from_str::<InstancePathPattern>("\"/top/*/X1\"")
                .expect("legacy pattern loads"),
            pattern
        );
        assert!(serde_json::from_str::<InstancePathPattern>("\"/top/**\"").is_err());
    }
}
