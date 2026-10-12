//! Copy-on-write storage for the parts of an XSPICE instance that rarely move.
//!
//! An instance is copied whole in two places on the transient's per-step
//! path. [`super::SharedXspiceInstance::make_mut`] copies it the first time
//! something writes after a rollback snapshot captured it, and
//! [`super::XspiceInstance::evaluate`] copies the model context before every
//! model call so a failing model leaves the context as it found it. Both used
//! to clone every string-keyed map an instance holds: the port contract and
//! connections fixed at elaboration, the port bindings and parameters fixed at
//! setup, and the handful of per-call port values a model actually rewrites.
//! On a gate-level transient those two copies were the largest single cost of
//! a step, nearly all of it the allocator re-creating keys nobody changed.
//!
//! [`CopyOnWrite`] holds one such part behind an [`Arc`]. Cloning the owner
//! shares it; the first write through either handle copies that part alone.
//! The copy an owner observes is the copy a deep clone produced, so a rollback
//! image or an evaluation backup restores exactly what it did before: the only
//! mutable view is [`DerefMut`], which runs [`Arc::make_mut`], so a write never
//! lands on storage another handle can still read.
//!
//! The fields wrapped this way are private to the type that owns them, so
//! every write to them is visible in that type's own file.

use std::ops::{Deref, DerefMut};
use std::sync::Arc;

/// A value shared with its clones until one of them writes.
pub(crate) struct CopyOnWrite<T>(Arc<T>);

impl<T> CopyOnWrite<T> {
    pub(crate) fn new(value: T) -> Self {
        Self(Arc::new(value))
    }

    /// Whether both handles still share one value: neither has been written
    /// since one was cloned from the other.
    #[cfg(debug_assertions)]
    pub(crate) fn ptr_eq(a: &Self, b: &Self) -> bool {
        Arc::ptr_eq(&a.0, &b.0)
    }
}

impl<T> Clone for CopyOnWrite<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<T: Default> Default for CopyOnWrite<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> From<T> for CopyOnWrite<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> Deref for CopyOnWrite<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: Clone> DerefMut for CopyOnWrite<T> {
    /// Copy the value first if another handle still shares it.
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        Arc::make_mut(&mut self.0)
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for CopyOnWrite<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&*self.0, f)
    }
}

impl<T: PartialEq> PartialEq for CopyOnWrite<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0) || *self.0 == *other.0
    }
}

impl<'a, T> IntoIterator for &'a CopyOnWrite<T>
where
    &'a T: IntoIterator,
{
    type Item = <&'a T as IntoIterator>::Item;
    type IntoIter = <&'a T as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&*self.0).into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::CopyOnWrite;
    use std::collections::HashMap;

    #[test]
    fn a_write_through_one_handle_is_invisible_to_its_clones() {
        let mut original: CopyOnWrite<HashMap<String, f64>> = CopyOnWrite::default();
        original.insert("gain".into(), 1.0);
        let snapshot = original.clone();
        original.insert("gain".into(), 2.0);
        original.insert("offset".into(), 3.0);
        assert_eq!(snapshot.get("gain"), Some(&1.0));
        assert_eq!(snapshot.len(), 1);
        assert_eq!(original.get("gain"), Some(&2.0));
        assert_eq!(original.len(), 2);
    }

    #[test]
    fn restoring_a_clone_restores_the_value_it_shared() {
        let mut live: CopyOnWrite<Vec<u32>> = vec![1, 2, 3].into();
        let backup = live.clone();
        live.push(4);
        live = backup;
        assert_eq!(*live, vec![1, 2, 3]);
    }
}
