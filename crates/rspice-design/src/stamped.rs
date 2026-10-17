//! A value that says when it last changed.
//!
//! Derived evidence — a check's input digest, a projection's key — is
//! expensive to recompute and must never be reused for content that moved
//! under it. Hashing the content again on every read is exact and costs the
//! whole document every frame; a revision counter is cheap and is only as
//! exact as the discipline of every caller that edits the value. [`Stamped`]
//! puts the counter where no caller can forget it: the only way to reach the
//! value mutably is [`DerefMut`], and that issues a new stamp.
//!
//! A stamp is unique for the life of the process, not per value, so a stamp
//! names one content and nothing else: two values carrying the same stamp are
//! a value and its unedited clone. A value loaded from disk, defaulted or
//! replaced wholesale is issued a stamp no earlier value ever carried, so a
//! memo keyed by stamps cannot mistake a reloaded document for the one it
//! replaced. The converse does not hold — an edit that is then undone leaves
//! equal content under a new stamp — which costs a memo one recomputation and
//! never a wrong answer.

use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_STAMP: AtomicU64 = AtomicU64::new(1);

fn issue() -> u64 {
    NEXT_STAMP.fetch_add(1, Ordering::Relaxed)
}

/// `T`, with the stamp of its current content.
pub struct Stamped<T> {
    value: T,
    stamp: u64,
}

impl<T> Stamped<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            stamp: issue(),
        }
    }

    /// The stamp of the content as it stands: equal stamps, equal content.
    pub const fn stamp(&self) -> u64 {
        self.stamp
    }

    /// Issue a new stamp without touching the value, for a change the value
    /// does not hold itself — a library edit that moves what the content
    /// resolves to.
    pub fn touch(&mut self) {
        self.stamp = issue();
    }

    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T> Deref for Stamped<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T> DerefMut for Stamped<T> {
    /// Every mutable access is taken to change the value. One that changes
    /// nothing costs a memo a recomputation; one that went unstamped would
    /// let it answer for content it never saw.
    fn deref_mut(&mut self) -> &mut T {
        self.stamp = issue();
        &mut self.value
    }
}

impl<T> From<T> for Stamped<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

/// A clone is the same content, so it keeps the stamp.
impl<T: Clone> Clone for Stamped<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            stamp: self.stamp,
        }
    }
}

impl<T: Default> Default for Stamped<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Stamped<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(formatter)
    }
}

/// Equality is the content's: two equal values issued apart are still equal.
impl<T: PartialEq> PartialEq for Stamped<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Eq> Eq for Stamped<T> {}

impl<T: serde::Serialize> serde::Serialize for Stamped<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.value.serialize(serializer)
    }
}

impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for Stamped<T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        T::deserialize(deserializer).map(Self::new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_mutable_access_issues_a_stamp_no_value_carried_before() {
        let mut value = Stamped::new(vec![1]);
        let issued = value.stamp();
        value.push(2);
        assert_ne!(value.stamp(), issued, "a push through the value");
        let pushed = value.stamp();
        let _ = &mut *value;
        assert_ne!(value.stamp(), pushed, "a bare mutable borrow");
        let borrowed = value.stamp();
        value.touch();
        assert_ne!(value.stamp(), borrowed, "a touch");
        let reread = value.stamp();
        assert_eq!(value.len(), 2);
        assert_eq!(value.stamp(), reread, "reading issues nothing");
    }

    #[test]
    fn a_clone_keeps_the_stamp_and_a_replacement_never_reuses_one() {
        let value = Stamped::new(String::from("a"));
        let clone = value.clone();
        assert_eq!(clone.stamp(), value.stamp());
        let replaced = Stamped::new(String::from("a"));
        assert_ne!(replaced.stamp(), value.stamp());
        assert_eq!(replaced, value, "equality is the content's");
        let reloaded: Stamped<String> =
            serde_json::from_str(&serde_json::to_string(&value).unwrap()).unwrap();
        assert_ne!(reloaded.stamp(), value.stamp());
        assert_ne!(Stamped::<String>::default().stamp(), value.stamp());
    }
}
