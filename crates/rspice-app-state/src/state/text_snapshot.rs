//! A source text at one revision, shared rather than copied.
//!
//! An editor holds its text as a rope and changes it in place. What it hands
//! to everything else that reads the text (the page, the background check,
//! colouring, find) is a snapshot: a clone of the rope, which shares the
//! rope's storage, so taking or keeping one costs nothing in proportion to the
//! text. Each snapshot carries a revision no other text in the process has
//! carried, so a result worked out for a revision is a result for exactly
//! that text, and a reader that kept one notices a newer text by comparing
//! two numbers.

use std::borrow::Cow;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

use ropey::Rope;

/// The revision the next new text takes. Zero is the empty default's.
static NEXT_REVISION: AtomicU64 = AtomicU64::new(1);

/// A text at one revision. Cloning one is cheap whatever its length.
#[derive(Clone, Default)]
pub struct TextSnapshot {
    text: Rope,
    revision: u64,
}

impl TextSnapshot {
    /// A revision no text has carried yet.
    pub fn next_revision() -> u64 {
        NEXT_REVISION.fetch_add(1, Ordering::Relaxed)
    }

    /// `text` at a revision of its own.
    #[must_use]
    pub fn new(text: &str) -> Self {
        Self {
            text: Rope::from_str(text),
            revision: Self::next_revision(),
        }
    }

    /// The text an editor holds, at the revision it gave that text. The
    /// revision must have come from [`TextSnapshot::next_revision`] for this
    /// text, or from a snapshot of it.
    #[must_use]
    pub const fn from_rope(text: Rope, revision: u64) -> Self {
        Self { text, revision }
    }

    pub const fn revision(&self) -> u64 {
        self.revision
    }

    /// The text, for a reader that walks it in place.
    pub const fn rope(&self) -> &Rope {
        &self.text
    }

    /// Its length in bytes.
    pub fn len(&self) -> usize {
        self.text.len_bytes()
    }

    pub fn is_empty(&self) -> bool {
        self.text.len_bytes() == 0
    }

    /// Whether the text holds nothing but white space.
    pub fn is_blank(&self) -> bool {
        self.text.chunks().all(|chunk| chunk.trim().is_empty())
    }

    /// The text in pieces, in order, without copying it.
    pub fn chunks(&self) -> impl Iterator<Item = &str> + '_ {
        self.text.chunks()
    }

    /// The text as one string: borrowed when it is held in one piece, else a
    /// copy. A reader that can walk [`TextSnapshot::chunks`] should.
    pub fn as_cow(&self) -> Cow<'_, str> {
        self.text.slice(..).into()
    }

    /// The number of lines, numbered as the editor numbers them, except
    /// that a final line break does not begin another line.
    pub fn line_count(&self) -> usize {
        let lines = self.text.len_lines();
        lines - usize::from(self.text.line(lines - 1).len_chars() == 0)
    }

    /// Zero-based line `index`, numbered as the editor numbers them, without
    /// its line break.
    pub fn line(&self, index: usize) -> Option<Cow<'_, str>> {
        if index >= self.line_count() {
            return None;
        }
        let line: Cow<'_, str> = self.text.line(index).into();
        Some(match line {
            Cow::Borrowed(line) => Cow::Borrowed(trim_line_break(line)),
            Cow::Owned(mut line) => {
                let kept = trim_line_break(&line).len();
                line.truncate(kept);
                Cow::Owned(line)
            }
        })
    }

    /// Whether this snapshot and `other` are the same revision: a check
    /// that costs nothing, for a reader that kept a snapshot.
    pub const fn same_revision(&self, other: &Self) -> bool {
        self.revision == other.revision
    }

    /// Whether the text is `other`, compared piece by piece in place.
    pub fn equals_str(&self, other: &str) -> bool {
        if self.text.len_bytes() != other.len() {
            return false;
        }
        let mut offset = 0;
        for chunk in self.text.chunks() {
            let end = offset + chunk.len();
            if other.get(offset..end) != Some(chunk) {
                return false;
            }
            offset = end;
        }
        true
    }
}

/// `line` without the line break that ends it: any the editor breaks lines
/// at.
fn trim_line_break(line: &str) -> &str {
    if let Some(line) = line.strip_suffix("\r\n") {
        return line;
    }
    line.strip_suffix([
        '\n', '\r', '\u{000B}', '\u{000C}', '\u{0085}', '\u{2028}', '\u{2029}',
    ])
    .unwrap_or(line)
}

impl From<&str> for TextSnapshot {
    fn from(text: &str) -> Self {
        Self::new(text)
    }
}

impl From<String> for TextSnapshot {
    fn from(text: String) -> Self {
        Self::new(&text)
    }
}

impl From<&String> for TextSnapshot {
    fn from(text: &String) -> Self {
        Self::new(text)
    }
}

impl fmt::Display for TextSnapshot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.text
            .chunks()
            .try_for_each(|chunk| formatter.write_str(chunk))
    }
}

impl fmt::Debug for TextSnapshot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.as_cow(), formatter)
    }
}

/// Two snapshots are equal when they hold the same text, whatever their
/// revisions.
impl PartialEq for TextSnapshot {
    fn eq(&self, other: &Self) -> bool {
        self.revision == other.revision || self.text == other.text
    }
}

impl Eq for TextSnapshot {}

impl PartialEq<str> for TextSnapshot {
    fn eq(&self, other: &str) -> bool {
        self.equals_str(other)
    }
}

impl PartialEq<&str> for TextSnapshot {
    fn eq(&self, other: &&str) -> bool {
        self.equals_str(other)
    }
}

impl PartialEq<String> for TextSnapshot {
    fn eq(&self, other: &String) -> bool {
        self.equals_str(other)
    }
}

impl PartialEq<TextSnapshot> for String {
    fn eq(&self, other: &TextSnapshot) -> bool {
        other.equals_str(self)
    }
}

impl PartialEq<TextSnapshot> for &str {
    fn eq(&self, other: &TextSnapshot) -> bool {
        other.equals_str(self)
    }
}

impl PartialEq<TextSnapshot> for str {
    fn eq(&self, other: &TextSnapshot) -> bool {
        other.equals_str(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_new_text_takes_a_revision_of_its_own() {
        let first = TextSnapshot::new("R1 a b 1k\n");
        let second = TextSnapshot::new("R1 a b 1k\n");
        assert_ne!(first.revision(), second.revision());
        assert_eq!(first, second, "the same text compares equal");
        assert!(first.clone().same_revision(&first));
    }

    /// Broken by `\n` and `\r\n` alone, the lines are those `str::lines`
    /// reads.
    #[test]
    fn lines_read_as_str_lines_reads_them() {
        for text in ["", "\n", "a", "a\n", "a\r\nb", "a\nb\n", "a\n\nb\r\n\r\n"] {
            let snapshot = TextSnapshot::new(text);
            let lines = text.lines().collect::<Vec<_>>();
            assert_eq!(snapshot.line_count(), lines.len(), "{text:?}");
            for (index, line) in lines.iter().enumerate() {
                assert_eq!(snapshot.line(index).as_deref(), Some(*line), "{text:?}");
            }
            assert_eq!(snapshot.line(lines.len()), None, "{text:?}");
            assert_eq!(snapshot.to_string(), text);
            assert_eq!(snapshot, text);
            assert_eq!(snapshot.is_blank(), text.trim().is_empty());
        }
    }
}
