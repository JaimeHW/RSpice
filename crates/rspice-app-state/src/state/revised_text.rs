//! A text in one piece that names its own revision.
//!
//! The deck the Netlist page shows is read as one string: by the parser, the
//! run, the checks and the file it is saved to. It is kept here, shared
//! rather than copied, with a revision from the same count as
//! [`TextSnapshot`]'s. A write goes through [`RevisedText::set`] or
//! [`RevisedText::edit`] and takes a new revision, and nothing else can
//! write it, so two texts with one revision hold the same text. What a frame
//! keeps for a text (an outline, the name under the caret, whether the deck
//! is the one a run read) is kept against its revision, and a later frame
//! checks it by comparing two numbers, not two decks.

use std::cell::Cell;
use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

use super::TextSnapshot;

/// A text and its revision. Cloning one shares the text, whatever its
/// length; a write to a clone copies it first.
#[derive(Clone, Default)]
pub struct RevisedText {
    text: Arc<String>,
    revision: u64,
}

impl RevisedText {
    /// `text` at a revision of its own.
    #[must_use]
    pub fn new(text: String) -> Self {
        Self {
            text: Arc::new(text),
            revision: TextSnapshot::next_revision(),
        }
    }

    /// The revision of the text held: the same for clones of one text, and
    /// zero only for the empty default.
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }

    /// The text, for a reader that outlives this borrow, such as a build on
    /// another thread: shared, not copied.
    pub fn shared(&self) -> Arc<String> {
        Arc::clone(&self.text)
    }

    /// Hold `text` instead, at a new revision.
    pub fn set(&mut self, text: String) {
        *self = Self::new(text);
    }

    /// Change the text in place, at a new revision. A text a clone still
    /// shares is copied first.
    pub fn edit<R>(&mut self, edit: impl FnOnce(&mut String) -> R) -> R {
        let result = edit(Arc::make_mut(&mut self.text));
        self.revision = TextSnapshot::next_revision();
        result
    }

    /// The text as a string of its own, copied only while a clone shares it.
    #[must_use]
    pub fn into_string(self) -> String {
        Arc::try_unwrap(self.text).unwrap_or_else(|shared| String::clone(&shared))
    }

    /// Whether `other` holds the same text: at once when they share a
    /// revision or the text itself, else compared.
    pub fn same_text(&self, other: &Self) -> bool {
        self.revision == other.revision
            || Arc::ptr_eq(&self.text, &other.text)
            || self.text == other.text
    }
}

impl Deref for RevisedText {
    type Target = str;

    fn deref(&self) -> &str {
        &self.text
    }
}

impl AsRef<str> for RevisedText {
    fn as_ref(&self) -> &str {
        &self.text
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

impl fmt::Display for RevisedText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.text)
    }
}

impl fmt::Debug for RevisedText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.text.as_str(), formatter)
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
