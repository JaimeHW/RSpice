//! A text that names its own revision, held as an editor holds it.
//!
//! The deck the Netlist page shows is edited as a rope and read, by the
//! parser, the run, the checks and the file it is saved to, as one string.
//! It is kept here, shared rather than copied, with a revision from the same
//! count as [`TextSnapshot`]'s. A text the editor gives keeps the editor's
//! rope and its revision, so a keystroke hands it over without copying it;
//! the text in one piece is made the first time a reader asks for it, once
//! for every clone. A write goes through [`RevisedText::set`],
//! [`RevisedText::set_snapshot`], [`RevisedText::replace_range`] or
//! [`RevisedText::edit`] and takes a new revision, and nothing else can
//! write it, so two texts with one revision hold the same text. What a frame
//! keeps for a text (an outline, the name under the caret, whether the deck
//! is the one a run read) is kept against its revision, and a later frame
//! checks it by comparing two numbers, not two decks.

use std::borrow::Cow;
use std::cell::Cell;
use std::fmt;
use std::ops::{Deref, Range};
use std::sync::{Arc, OnceLock};

use ropey::Rope;

use super::TextSnapshot;

/// A text and its revision. Cloning one shares the text, whatever its
/// length; a write to a clone leaves the text it shared as it was.
#[derive(Clone, Default)]
pub struct RevisedText {
    body: Arc<Body>,
    revision: u64,
}

/// The text itself: as a rope, in one piece, or both, each made from the
/// other the first time it is asked for. At least one is always held.
#[derive(Clone)]
struct Body {
    rope: OnceLock<Rope>,
    string: OnceLock<String>,
    /// The line break an edit inserts in the text, once it was made ready
    /// for an editor ([`RevisedText::ready_for_editor`]).
    newline: OnceLock<&'static str>,
}

impl Body {
    fn of_string(text: String) -> Self {
        Self {
            rope: OnceLock::new(),
            string: OnceLock::from(text),
            newline: OnceLock::new(),
        }
    }

    fn of_rope(rope: Rope) -> Self {
        Self {
            rope: OnceLock::from(rope),
            string: OnceLock::new(),
            newline: OnceLock::new(),
        }
    }

    fn as_str(&self) -> &str {
        self.string.get_or_init(|| String::from(self.held_rope()))
    }

    fn rope(&self) -> &Rope {
        self.rope.get_or_init(|| Rope::from_str(self.held_string()))
    }

    fn held_rope(&self) -> &Rope {
        self.rope
            .get()
            .expect("a text holds its rope when it holds no string")
    }

    fn held_string(&self) -> &str {
        self.string
            .get()
            .expect("a text holds its string when it holds no rope")
    }

    fn len(&self) -> usize {
        self.string
            .get()
            .map_or_else(|| self.held_rope().len_bytes(), String::len)
    }

    /// The text as a string of its own.
    fn into_string(self) -> String {
        match self.string.into_inner() {
            Some(text) => text,
            None => String::from(
                &self
                    .rope
                    .into_inner()
                    .expect("a text holds one or the other"),
            ),
        }
    }
}

impl Default for Body {
    fn default() -> Self {
        Self::of_string(String::new())
    }
}

/// Whether `rope` holds `text`, compared piece by piece in place.
fn rope_is(rope: &Rope, text: &str) -> bool {
    if rope.len_bytes() != text.len() {
        return false;
    }
    let mut offset = 0;
    rope.chunks().all(|chunk| {
        let end = offset + chunk.len();
        let same = text.get(offset..end) == Some(chunk);
        offset = end;
        same
    })
}

impl RevisedText {
    /// `text` at a revision of its own.
    #[must_use]
    pub fn new(text: String) -> Self {
        Self {
            body: Arc::new(Body::of_string(text)),
            revision: TextSnapshot::next_revision(),
        }
    }

    /// The text an editor holds, at the editor's revision: its rope is
    /// shared, not copied.
    #[must_use]
    pub fn from_snapshot(snapshot: &TextSnapshot) -> Self {
        Self {
            body: Arc::new(Body::of_rope(snapshot.rope().clone())),
            revision: snapshot.revision(),
        }
    }

    /// The revision of the text held: the same for clones of one text, and
    /// zero only for the empty default.
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    /// The text in one piece, made the first time any clone of it asks.
    pub fn as_str(&self) -> &str {
        self.body.as_str()
    }

    /// The text for an editor, at this revision: the rope it was given
    /// shared, or made once from the text in one piece.
    pub fn snapshot(&self) -> TextSnapshot {
        TextSnapshot::prepared(
            self.body.rope().clone(),
            self.revision,
            self.body.newline.get().copied(),
        )
    }

    /// Make, on this thread, what an editor reads of the text when it first
    /// shows it: its rope, and the line break an edit inserts in it. A text
    /// read away from the frame is made ready there, so the frame that first
    /// shows it reads nothing in proportion to it.
    pub fn ready_for_editor(&self) {
        let rope = self.body.rope();
        self.body
            .newline
            .get_or_init(|| super::preferred_newline(rope));
    }

    /// Its length in bytes, read without putting the text in one piece.
    pub fn len(&self) -> usize {
        self.body.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// How many lines the editor shows for the text: any line break starts
    /// one, so a text that ends in a break ends in an empty line. It is read
    /// from the rope, which a text held only in one piece makes once.
    pub fn line_count(&self) -> usize {
        self.body.rope().len_lines()
    }

    /// The byte the line holding byte `at` starts at, a line ending at `\n`,
    /// found without putting the text in one piece.
    pub fn line_start_at(&self, at: usize) -> usize {
        if let Some(text) = self.body.string.get() {
            return text.as_bytes()[..at]
                .iter()
                .rposition(|&byte| byte == b'\n')
                .map_or(0, |found| found + 1);
        }
        // The rope breaks lines at more than `\n`: its line holding `at`
        // starts at or after the one sought.
        let rope = self.body.held_rope();
        let mut line = rope.byte_to_line(at);
        loop {
            let start = rope.line_to_byte(line);
            if start == 0 || rope.byte(start - 1) == b'\n' {
                return start;
            }
            line -= 1;
        }
    }

    /// The byte of the `\n` that ends the line holding byte `at`, or the
    /// text's length on its last line, found without putting the text in one
    /// piece.
    pub fn line_end_at(&self, at: usize) -> usize {
        if let Some(text) = self.body.string.get() {
            return text.as_bytes()[at..]
                .iter()
                .position(|&byte| byte == b'\n')
                .map_or(text.len(), |found| at + found);
        }
        let rope = self.body.held_rope();
        let mut line = rope.byte_to_line(at);
        loop {
            let end = rope.line_to_byte(line + 1);
            if end > at && rope.byte(end - 1) == b'\n' {
                return end - 1;
            }
            if end == rope.len_bytes() {
                return end;
            }
            line += 1;
        }
    }

    /// The bytes `range` of the text, borrowed where they are held in one
    /// piece, and copied out of the rope where they are not.
    pub fn slice(&self, range: Range<usize>) -> Cow<'_, str> {
        if let Some(text) = self.body.string.get() {
            return Cow::Borrowed(&text[range]);
        }
        let slice = self.body.held_rope().byte_slice(range);
        slice
            .as_str()
            .map_or_else(|| Cow::Owned(String::from(slice)), Cow::Borrowed)
    }

    /// Whether the text is held in one piece yet.
    #[cfg(any(test, feature = "test-support"))]
    pub fn is_in_one_piece(&self) -> bool {
        self.body.string.get().is_some()
    }

    /// Hold `text` instead, at a new revision.
    pub fn set(&mut self, text: String) {
        *self = Self::new(text);
    }

    /// Hold the text an editor gives instead, at the editor's revision.
    pub fn set_snapshot(&mut self, snapshot: &TextSnapshot) {
        *self = Self::from_snapshot(snapshot);
    }

    /// Put `text` in place of the bytes `range`, at a new revision. A text
    /// held as a rope is changed as one, without putting it in one piece.
    pub fn replace_range(&mut self, range: Range<usize>, text: &str) {
        if let Some(rope) = self.body.rope.get() {
            let mut rope = rope.clone();
            let start = rope.byte_to_char(range.start);
            let end = rope.byte_to_char(range.end);
            rope.remove(start..end);
            rope.insert(start, text);
            self.body = Arc::new(Body::of_rope(rope));
            self.revision = TextSnapshot::next_revision();
        } else {
            self.edit(|held| held.replace_range(range, text));
        }
    }

    /// Change the text in place, at a new revision. A text a clone still
    /// shares is copied first.
    pub fn edit<R>(&mut self, edit: impl FnOnce(&mut String) -> R) -> R {
        let mut text = match Arc::get_mut(&mut self.body) {
            Some(body) => std::mem::take(body).into_string(),
            None => match self.body.string.get() {
                Some(text) => text.clone(),
                None => String::from(self.body.held_rope()),
            },
        };
        let result = edit(&mut text);
        self.body = Arc::new(Body::of_string(text));
        self.revision = TextSnapshot::next_revision();
        result
    }

    /// The text as a string of its own, copied only while a clone shares it.
    #[must_use]
    pub fn into_string(self) -> String {
        Arc::try_unwrap(self.body)
            .map_or_else(|shared| shared.as_str().to_owned(), Body::into_string)
    }

    /// Whether `other` holds the same text: at once when they share a
    /// revision or the text itself, or differ in length, else compared in
    /// whatever form the two already hold.
    pub fn same_text(&self, other: &Self) -> bool {
        if self.revision == other.revision || Arc::ptr_eq(&self.body, &other.body) {
            return true;
        }
        if self.len() != other.len() {
            return false;
        }
        let (one, two) = (&self.body, &other.body);
        match (one.string.get(), two.string.get()) {
            (Some(one), Some(two)) => one == two,
            (Some(text), None) => rope_is(two.held_rope(), text),
            (None, Some(text)) => rope_is(one.held_rope(), text),
            (None, None) => one.held_rope() == two.held_rope(),
        }
    }
}

impl Deref for RevisedText {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<str> for RevisedText {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<String> for RevisedText {
    fn from(text: String) -> Self {
        Self::new(text)
    }
}

impl From<&str> for RevisedText {
    fn from(text: &str) -> Self {
        Self::new(text.to_owned())
    }
}

impl From<&String> for RevisedText {
    fn from(text: &String) -> Self {
        Self::new(text.clone())
    }
}

/// Written as it is held: a rope piece by piece, without putting it in one.
impl fmt::Display for RevisedText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.body.string.get() {
            Some(text) => formatter.write_str(text),
            None => self
                .body
                .held_rope()
                .chunks()
                .try_for_each(|chunk| formatter.write_str(chunk)),
        }
    }
}

impl fmt::Debug for RevisedText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), formatter)
    }
}

/// Two texts are equal when they hold the same text, whatever their
/// revisions.
impl PartialEq for RevisedText {
    fn eq(&self, other: &Self) -> bool {
        self.same_text(other)
    }
}

impl Eq for RevisedText {}

impl PartialEq<str> for RevisedText {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for RevisedText {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<String> for RevisedText {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<RevisedText> for str {
    fn eq(&self, other: &RevisedText) -> bool {
        self == other.as_str()
    }
}

impl PartialEq<RevisedText> for &str {
    fn eq(&self, other: &RevisedText) -> bool {
        *self == other.as_str()
    }
}

impl PartialEq<RevisedText> for String {
    fn eq(&self, other: &RevisedText) -> bool {
        self == other.as_str()
    }
}

/// Whether two texts are the same, compared once for each pair of their
/// revisions: a frame that asks again of the same two texts reads the
/// answer, however long they are.
#[derive(Debug, Clone, Default)]
pub struct SameTextMemo(Cell<Option<(u64, u64, bool)>>);

impl SameTextMemo {
    pub fn same(&self, one: &RevisedText, other: &RevisedText) -> bool {
        if one.revision == other.revision {
            return true;
        }
        let pair = (one.revision, other.revision);
        if let Some((first, second, same)) = self.0.get()
            && (first, second) == pair
        {
            return same;
        }
        let same = one.same_text(other);
        self.0.set(Some((pair.0, pair.1, same)));
        same
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every write takes a new revision; a clone keeps its text's, and
    /// writing to it leaves the text it was cloned from as it was.
    #[test]
    fn every_write_takes_a_revision_and_a_clone_keeps_its_own() {
        let mut deck = RevisedText::from("R1 a b 1k\n");
        let first = deck.revision();
        let kept = deck.clone();
        assert_eq!(kept.revision(), first);

        deck.edit(|text| text.push_str(".end\n"));
        assert_ne!(deck.revision(), first);
        assert_eq!(deck, "R1 a b 1k\n.end\n");
        assert_eq!(kept, "R1 a b 1k\n", "the clone still holds its text");
        assert_eq!(kept.revision(), first);

        let edited = deck.revision();
        deck.set("R1 a b 1k\n".to_owned());
        assert_ne!(deck.revision(), edited);
        assert_ne!(deck.revision(), first);
        assert_eq!(deck, kept, "the same text at another revision is equal");
        assert_eq!(RevisedText::default().revision(), 0);
    }

    /// A text an editor gives keeps the editor's rope and revision, and is
    /// put in one piece only when a reader asks, once for every clone; an
    /// edit to it as a rope leaves it out of one piece.
    #[test]
    fn a_text_from_an_editor_is_put_in_one_piece_only_when_read() {
        let snapshot = TextSnapshot::new("R1 a b 1k\n.end\n");
        let mut deck = RevisedText::from_snapshot(&snapshot);
        assert_eq!(deck.revision(), snapshot.revision());
        assert_eq!(deck.len(), 15);
        assert!(!deck.is_in_one_piece());
        assert_eq!(deck.to_string(), "R1 a b 1k\n.end\n");
        assert!(!deck.is_in_one_piece(), "writing it out reads the rope");
        assert_eq!(deck.snapshot().revision(), snapshot.revision());

        let kept = deck.clone();
        deck.replace_range(3..4, "x");
        assert!(!deck.is_in_one_piece());
        assert_ne!(deck.revision(), snapshot.revision());
        assert_eq!(deck.as_str(), "R1 x b 1k\n.end\n");
        assert_eq!(kept.as_str(), "R1 a b 1k\n.end\n");
        assert!(kept.is_in_one_piece());
        assert!(kept.clone().is_in_one_piece(), "a clone shares the piece");
        assert!(kept.same_text(&RevisedText::from("R1 a b 1k\n.end\n")));
        assert!(!deck.same_text(&kept));
        assert_eq!(kept.into_string(), "R1 a b 1k\n.end\n");
    }

    /// Where a line starts and ends, a stretch of the text and how many
    /// lines it has read the same from a rope as from the text in one
    /// piece: a line ends at `\n`, whatever other breaks the rope counts.
    #[test]
    fn a_line_is_found_the_same_in_a_rope_as_in_one_piece() {
        let text = "title\r\nR1 a b 1k\rC1 b 0 1p\n+ ic=0\u{2028}more\n\nlast";
        let whole = RevisedText::from(text);
        let roped = RevisedText::from_snapshot(&TextSnapshot::new(text));
        for at in (0..=text.len()).filter(|&at| text.is_char_boundary(at)) {
            let start = text[..at].rfind('\n').map_or(0, |found| found + 1);
            let end = text[at..].find('\n').map_or(text.len(), |found| at + found);
            for held in [&whole, &roped] {
                assert_eq!(
                    (held.line_start_at(at), held.line_end_at(at)),
                    (start, end),
                    "byte {at}"
                );
                assert_eq!(held.slice(start..end), &text[start..end]);
            }
        }
        assert!(!roped.is_in_one_piece(), "the rope is read in place");
        assert_eq!(roped.line_count(), whole.line_count());
        assert_eq!(
            RevisedText::from("a\nb\n").line_count(),
            3,
            "a text ending in a break ends in an empty line"
        );
    }

    /// The memo compares two texts once per pair of revisions, and again
    /// once either text is written.
    #[test]
    fn the_memo_answers_again_once_either_text_is_written() {
        let memo = SameTextMemo::default();
        let ran = RevisedText::from("V1 a 0 1\n");
        let mut shown = RevisedText::from("V1 a 0 1\n");
        assert!(memo.same(&shown, &ran));
        shown.edit(|text| text.push('x'));
        assert!(!memo.same(&shown, &ran));
        shown.edit(|text| {
            text.pop();
        });
        assert!(memo.same(&shown, &ran));
        assert!(memo.same(&ran.clone(), &ran));
    }
}
