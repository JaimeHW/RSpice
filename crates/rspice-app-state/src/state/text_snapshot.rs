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
use rspice_core::netlist::DeckTextChange;

/// The revision the next new text takes. Zero is the empty default's.
static NEXT_REVISION: AtomicU64 = AtomicU64::new(1);

/// A text at one revision. Cloning one is cheap whatever its length.
#[derive(Clone, Default)]
pub struct TextSnapshot {
    text: Rope,
    revision: u64,
    /// The line break an edit inserts in the text, when it was worked out
    /// before the text was handed over.
    newline: Option<&'static str>,
    /// The revision this text was edited from, and where: the bytes
    /// `start..old_end` of that revision are the bytes `start..new_end` of
    /// this one. `None` when the editor cannot say.
    edit: Option<(u64, DeckTextChange)>,
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
            newline: None,
            edit: None,
        }
    }

    /// The text an editor holds, at the revision it gave that text. The
    /// revision must have come from [`TextSnapshot::next_revision`] for this
    /// text, or from a snapshot of it.
    #[must_use]
    pub const fn from_rope(text: Rope, revision: u64) -> Self {
        Self {
            text,
            revision,
            newline: None,
            edit: None,
        }
    }

    /// This text, which is the text of revision `from` after `change`.
    #[must_use]
    pub fn edited_from(mut self, from: u64, change: DeckTextChange) -> Self {
        self.edit = Some((from, change));
        self
    }

    /// The revision this text was edited from, and where, when the editor
    /// said.
    pub const fn edit(&self) -> Option<(u64, DeckTextChange)> {
        self.edit
    }

    /// [`TextSnapshot::from_rope`], with the line break an edit inserts in
    /// the text when it was worked out already.
    pub(crate) const fn prepared(text: Rope, revision: u64, newline: Option<&'static str>) -> Self {
        Self {
            text,
            revision,
            newline,
            edit: None,
        }
    }

    pub const fn revision(&self) -> u64 {
        self.revision
    }

    /// The text, for a reader that walks it in place.
    pub const fn rope(&self) -> &Rope {
        &self.text
    }

    /// The line break an edit inserts in the text (see
    /// [`preferred_newline`]): worked out before the text was handed over,
    /// or else read from it now.
    pub fn newline(&self) -> &'static str {
        self.newline
            .unwrap_or_else(|| preferred_newline(&self.text))
    }

    /// The text as one string: borrowed when it is held in one piece, else a
    /// copy. A reader that can walk the rope's chunks should.
    pub fn as_cow(&self) -> Cow<'_, str> {
        self.text.slice(..).into()
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

/// The line break an edit inserts in `source`: its most common one,
/// preferring CRLF then LF on a tie, and LF when it has none. A text with no
/// carriage return ends its lines with a line feed, told from one count of
/// its bytes; only a text holding one has its pairs counted.
pub fn preferred_newline(source: &Rope) -> &'static str {
    let count = |wanted: u8| -> usize {
        source
            .chunks()
            .map(|chunk| {
                chunk
                    .as_bytes()
                    .iter()
                    .filter(|&&byte| byte == wanted)
                    .count()
            })
            .sum()
    };
    let returns = count(b'\r');
    if returns == 0 {
        return "\n";
    }
    let mut crlf = 0_usize;
    let mut return_ends_chunk = false;
    for chunk in source.chunks() {
        let bytes = chunk.as_bytes();
        if return_ends_chunk && bytes.first() == Some(&b'\n') {
            crlf += 1;
        }
        crlf += bytes.windows(2).filter(|pair| *pair == b"\r\n").count();
        if let Some(&last) = bytes.last() {
            return_ends_chunk = last == b'\r';
        }
    }
    let lf = count(b'\n') - crlf;
    let cr = returns - crlf;
    if crlf >= lf && crlf >= cr && crlf > 0 {
        "\r\n"
    } else if cr > lf && cr > 0 {
        "\r"
    } else {
        "\n"
    }
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

/// A copy of the whole text, made in one allocation of its length. Prefer
/// it to `to_string`, which grows the copy as it writes it.
impl From<&TextSnapshot> for String {
    fn from(snapshot: &TextSnapshot) -> Self {
        Self::from(&snapshot.text)
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
        assert_eq!(
            first.clone().revision(),
            first.revision(),
            "a clone is the same text"
        );
    }

    /// A snapshot reads back, and compares equal to, the text it was made
    /// from, whatever its line breaks.
    #[test]
    fn a_snapshot_reads_back_the_text_it_was_made_from() {
        for text in ["", "\n", "a", "a\n", "a\r\nb", "a\nb\n", "a\n\nb\r\n\r\n"] {
            let snapshot = TextSnapshot::new(text);
            assert_eq!(String::from(&snapshot), text);
            assert_eq!(snapshot.to_string(), text);
            assert_eq!(snapshot.as_cow(), text);
            assert_eq!(snapshot, text);
        }
    }
}
