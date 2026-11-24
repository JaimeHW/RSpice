//! What a transaction changed, kept while it runs.
//!
//! A transaction edits its view in place. While it runs, each list keeps
//! every entity as it stood before the transaction first changed, made or
//! took it out, and nothing else: a list of entities knows those it made by
//! their identities, which the document makes in ascending order, and a
//! list without identities keeps the stretch from the first place an edit
//! reached. So a refused transaction is put back exactly, a committed one is
//! checked by what it changed, and the step that undoes it holds what it
//! changed: each costs in proportion to the edits, not to the view.

use std::collections::hash_map::{Entry, RandomState};
use std::hash::BuildHasher;
use std::ops::{Deref, Range};
use std::sync::atomic::{AtomicU64, Ordering};

use super::undo::{Changed, Identified, Splice};
use super::*;

/// What a document's own fields held when a transaction began.
pub(super) struct Began {
    pub(super) tracking: ResultDocumentTracking,
    pub(super) presentation: VisualizationPresentationPolicy,
    pub(super) working_view: Option<WorkingViewSource>,
    next_serial: u64,
}

/// Apply one call to every list of a document that a transaction keeps.
macro_rules! each_list {
    ($document:expr, $entities:ident ($($entity_args:expr),*), $splices:ident ()) => {{
        let document = $document;
        document.pages.$entities($($entity_args),*);
        document.panes.$entities($($entity_args),*);
        document.axes.$entities($($entity_args),*);
        document.traces.$entities($($entity_args),*);
        document.cursors.$entities($($entity_args),*);
        document.markers.$entities($($entity_args),*);
        document.measurements.$entities($($entity_args),*);
        document.annotations.$entities($($entity_args),*);
        document.link_groups.$entities($($entity_args),*);
        document.datasets.$splices();
        document.tombstones.$splices();
        document.comparisons.$splices();
        document.user_buses.$splices();
        document.event_lane_orders.$splices();
        document.expression_traces.$splices();
        document.xy_plots.$splices();
        document.readout_pins.$splices();
        document.sheet_settings.$splices();
        document.spectrum_settings.$splices();
        document.coefficient_settings.$splices();
    }};
}

impl VisualizationDocument {
    /// Start keeping what a transaction changes.
    pub(super) fn begin(&mut self) -> Began {
        let since = self.next_serial;
        each_list!(&mut *self, begin(since), begin());
        Began {
            tracking: self.tracking,
            presentation: self.presentation,
            working_view: self.working_view,
            next_serial: self.next_serial,
        }
    }

    /// Put the document back as it stood when the transaction `began`.
    pub(super) fn roll_back(&mut self, began: Began) {
        self.tracking = began.tracking;
        self.presentation = began.presentation;
        self.working_view = began.working_view;
        self.next_serial = began.next_serial;
        each_list!(&mut *self, roll_back(), roll_back());
        self.row_index.clear();
    }

    /// Stop keeping what a committed transaction changed.
    pub(super) fn end(&mut self) {
        each_list!(&mut *self, end(), end());
    }

    /// Stamp the content a committed transaction or step left, keeping the
    /// traces it made, changed or took out, while it still keeps them.
    pub(super) fn note_change(&mut self) {
        let mut present = Vec::new();
        let mut gone = Vec::new();
        for (now, before) in self.traces.touched_with_before() {
            present.push(now.id);
            if let Some(before) = before
                && (before.pane_id != now.pane_id || signal_of(before) != signal_of(now))
            {
                gone.push((before.pane_id, signal_of(before).to_owned()));
            }
        }
        for identity in self.traces.taken_out() {
            if let Some(before) = self.traces.before(identity) {
                gone.push((before.pane_id, signal_of(before).to_owned()));
            }
        }
        self.changes = Changes {
            generation: next_generation(),
            last: Some(LastChange {
                from: self.changes.generation,
                present,
                gone,
                lanes: self.user_buses.is_touched() || self.event_lane_orders.is_touched(),
            }),
        };
    }
}

/// A number no other content of a document in this process has had.
fn next_generation() -> u64 {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    NEXT.fetch_add(1, Ordering::Relaxed)
}

/// Which content a document holds, and what its last change changed, so a
/// reader that drew it before can draw only what changed. A document made,
/// read or copied whole has content no reader drew.
#[derive(Clone)]
pub(super) struct Changes {
    pub(super) generation: u64,
    pub(super) last: Option<LastChange>,
}

/// The traces one transaction or step made, changed or took out.
#[derive(Clone)]
pub(super) struct LastChange {
    /// The generation of the content it changed.
    pub(super) from: u64,
    /// The traces it made or changed that the document holds.
    pub(super) present: Vec<TraceId>,
    /// The pane and signal of each trace it took out, and of each trace it
    /// moved to another pane or signal as it stood before.
    pub(super) gone: Vec<(PaneId, String)>,
    /// Whether it changed the event lanes' order or buses of any pane.
    pub(super) lanes: bool,
}

impl Changes {
    /// Content no reader has drawn.
    pub(super) fn fresh() -> Self {
        Self {
            generation: next_generation(),
            last: None,
        }
    }

    /// A change of what no reader draws.
    pub(super) fn bump(&mut self) {
        *self = Self {
            generation: next_generation(),
            last: Some(LastChange {
                from: self.generation,
                present: Vec::new(),
                gone: Vec::new(),
                lanes: false,
            }),
        };
    }
}

impl PartialEq for Changes {
    /// Which content a document holds is not part of it.
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl fmt::Debug for Changes {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Changes")
            .field("generation", &self.generation)
            .finish_non_exhaustive()
    }
}

/// What a list keeps of its entities by some key besides their identity,
/// kept whole through every change the list makes.
pub(super) trait ListIndex<T>: Default + Clone {
    type Key: PartialEq;

    fn key(&self, item: &T) -> Self::Key;
    fn insert(&mut self, key: Self::Key, identity: u64);
    fn remove(&mut self, key: Self::Key, identity: u64);

    /// Whether `one` and `other` are kept under the same key.
    fn same_key(&self, one: &T, other: &T) -> bool {
        self.key(one) == self.key(other)
    }
}

/// A list kept by identity alone.
#[derive(Debug, Default, Clone)]
pub(super) struct Unindexed;

impl<T> ListIndex<T> for Unindexed {
    type Key = ();

    fn key(&self, _: &T) {}

    fn insert(&mut self, (): (), _: u64) {}

    fn remove(&mut self, (): (), _: u64) {}
}

/// The traces of each pane by the signal each reads, so a tick finds a
/// signal's traces by a look-up instead of a scan of the view. A signal is
/// held by its hash: a look-up reads each trace it finds again, so two
/// signals of one hash are told apart.
#[derive(Debug, Default, Clone)]
pub(super) struct SignalIndex {
    hashing: RandomState,
    traces: HashMap<(PaneId, u64), Vec<u64>>,
}

/// The signal a trace reads: the vector it was provisioned from, or what
/// it is called when it names none.
pub(super) fn signal_of(trace: &Trace) -> &str {
    trace.source_signal().unwrap_or(&trace.label)
}

impl ListIndex<Trace> for SignalIndex {
    type Key = (PaneId, u64);

    fn key(&self, trace: &Trace) -> (PaneId, u64) {
        (trace.pane_id, self.hashing.hash_one(signal_of(trace)))
    }

    fn same_key(&self, one: &Trace, other: &Trace) -> bool {
        one.pane_id == other.pane_id && signal_of(one) == signal_of(other)
    }

    fn insert(&mut self, key: (PaneId, u64), identity: u64) {
        let traces = self.traces.entry(key).or_default();
        if !traces.contains(&identity) {
            traces.push(identity);
        }
    }

    fn remove(&mut self, key: (PaneId, u64), identity: u64) {
        if let Entry::Occupied(mut traces) = self.traces.entry(key) {
            traces.get_mut().retain(|held| *held != identity);
            if traces.get().is_empty() {
                traces.remove();
            }
        }
    }
}

/// The entities of one kind a document holds, in the order they were made.
pub(super) struct EntityList<T, I = Unindexed> {
    items: Vec<T>,
    index: I,
    journal: Option<EntityJournal<T>>,
}

/// What one list kept while a transaction runs.
struct EntityJournal<T> {
    /// The identity the document made first during the transaction: every
    /// entity at or above it was made by the transaction.
    since: u64,
    /// Each entity made before the transaction that it changed, took out
    /// or put back, as it stood before, in the order first touched: `None`
    /// for one it put back.
    kept: Vec<Option<T>>,
    /// Where `kept` holds each identity first touched above every identity
    /// touched before it, in ascending identity. A transaction mostly
    /// touches a list in its order, so these are found without hashing.
    ascending: Vec<(u64, usize)>,
    /// Where `kept` holds each identity first touched out of that order.
    rest: HashMap<u64, usize>,
    /// The identities of the entities the transaction made already checked,
    /// one after another: the commit need not check them again.
    checked: Range<u64>,
}

impl<T> EntityJournal<T> {
    fn new(since: u64) -> Self {
        Self {
            since,
            kept: Vec::new(),
            ascending: Vec::new(),
            rest: HashMap::new(),
            checked: 0..0,
        }
    }

    /// Keep what `was` gives for `identity`, the first time the transaction
    /// touches it. An identity above every one kept so far is new to the
    /// journal and found again by a binary search; any other is looked up.
    fn keep(&mut self, identity: u64, was: impl FnOnce() -> Option<T>) {
        if identity >= self.since {
            return;
        }
        if self
            .ascending
            .last()
            .is_none_or(|(last, _)| *last < identity)
        {
            self.ascending.push((identity, self.kept.len()));
        } else if self.slot(identity).is_some() {
            return;
        } else {
            self.rest.insert(identity, self.kept.len());
        }
        self.kept.push(was());
    }

    fn slot(&self, identity: u64) -> Option<usize> {
        match self
            .ascending
            .binary_search_by_key(&identity, |(held, _)| *held)
        {
            Ok(at) => Some(self.ascending[at].1),
            Err(_) => self.rest.get(&identity).copied(),
        }
    }

    /// Each identity kept, with where `kept` holds it, in ascending identity.
    fn in_order(&self) -> Vec<(u64, usize)> {
        let mut order = self.ascending.clone();
        if !self.rest.is_empty() {
            order.extend(self.rest.iter().map(|(identity, slot)| (*identity, *slot)));
            order.sort_unstable_by_key(|(identity, _)| *identity);
        }
        order
    }

    /// Each identity kept, with what it stood as, in ascending identity.
    fn into_order(self) -> impl Iterator<Item = (u64, Option<T>)> {
        let order = self.in_order();
        let mut kept = self.kept;
        order
            .into_iter()
            .map(move |(identity, slot)| (identity, kept[slot].take()))
    }
}

/// Keep `item` as it stands, the first time the transaction touches it.
fn keep_present<T: Identified + Clone>(journal: &mut Option<EntityJournal<T>>, item: &T) {
    if let Some(journal) = journal {
        journal.keep(item.identity(), || Some(item.clone()));
    }
}

/// Keep that `identity` was absent, the first time the transaction puts it
/// back.
fn keep_absent<T>(journal: &mut Option<EntityJournal<T>>, identity: u64) {
    if let Some(journal) = journal {
        journal.keep(identity, || None);
    }
}

/// Where `items`, in ascending identity, hold `identity`, searching from
/// `from` on; `from` moves past what the search passed, so a run of
/// ascending look-ups reads each part of the list once. The search widens
/// from `from` before it halves, so an identity near the last one found is
/// found in a step or two.
fn position_from<T: Identified>(items: &[T], from: &mut usize, identity: u64) -> Option<usize> {
    let rest = items.get(*from..).unwrap_or_default();
    let mut reach = 1;
    while reach < rest.len() && rest[reach - 1].identity() < identity {
        reach *= 2;
    }
    let rest = &rest[..reach.min(rest.len())];
    match rest.binary_search_by_key(&identity, Identified::identity) {
        Ok(at) => {
            let index = *from + at;
            *from = index + 1;
            Some(index)
        }
        Err(at) => {
            *from += at;
            None
        }
    }
}

impl<T: Identified + Clone + PartialEq, I: ListIndex<T>> EntityList<T, I> {
    pub(super) fn new(items: Vec<T>) -> Self {
        let mut index = I::default();
        for item in &items {
            let key = index.key(item);
            index.insert(key, item.identity());
        }
        Self {
            items,
            index,
            journal: None,
        }
    }

    /// Where the entity `identity` stands.
    pub(super) fn position(&self, identity: u64) -> Option<usize> {
        lookup::position(&self.items, identity)
    }

    /// Add `item`, made after every entity the list holds.
    pub(super) fn push(&mut self, item: T) {
        debug_assert!(
            self.items
                .last()
                .is_none_or(|last| last.identity() < item.identity()),
            "a list holds its entities in the order they were made"
        );
        keep_absent(&mut self.journal, item.identity());
        let key = self.index.key(&item);
        self.index.insert(key, item.identity());
        self.items.push(item);
    }

    /// [`Self::push`] for an entity made already checked. Entities so made
    /// one after another in a transaction are not checked again when it
    /// commits.
    pub(super) fn push_checked(&mut self, item: T) {
        let identity = item.identity();
        if let Some(journal) = &mut self.journal {
            if journal.checked.is_empty() {
                journal.checked = identity..identity + 1;
            } else if journal.checked.end == identity {
                journal.checked.end += 1;
            }
        }
        self.push(item);
    }

    /// Whether the running transaction made `identity` already checked.
    pub(super) fn made_checked(&self, identity: u64) -> bool {
        self.journal
            .as_ref()
            .is_some_and(|journal| journal.checked.contains(&identity))
    }

    /// Change the entity at `index` with `change`, which keeps its identity.
    pub(super) fn update_at<R>(&mut self, index: usize, change: impl FnOnce(&mut T) -> R) -> R {
        let item = &mut self.items[index];
        keep_present(&mut self.journal, item);
        let identity = item.identity();
        let before = self.index.key(item);
        let result = change(item);
        debug_assert_eq!(item.identity(), identity, "a change keeps the identity");
        let after = self.index.key(item);
        if before != after {
            self.index.remove(before, identity);
            self.index.insert(after, identity);
        }
        result
    }

    /// Change the entity `identity` with `change`; `None` when the list
    /// holds no such entity.
    pub(super) fn update<R>(
        &mut self,
        identity: u64,
        change: impl FnOnce(&mut T) -> R,
    ) -> Option<R> {
        let index = self.position(identity)?;
        Some(self.update_at(index, change))
    }

    /// Take out every entity `keep` refuses, in one pass.
    pub(super) fn retain(&mut self, mut keep: impl FnMut(&T) -> bool) {
        let Self {
            items,
            index,
            journal,
        } = self;
        items.retain(|item| {
            if keep(item) {
                return true;
            }
            keep_present(journal, item);
            let key = index.key(item);
            index.remove(key, item.identity());
            false
        });
    }

    /// Put `incoming`, entities of ascending identity that the list does
    /// not hold, each in its place by identity, in one pass.
    fn merge_in(&mut self, incoming: Vec<T>) {
        if incoming.is_empty() {
            return;
        }
        for item in &incoming {
            keep_absent(&mut self.journal, item.identity());
            let key = self.index.key(item);
            self.index.insert(key, item.identity());
        }
        let after_all = match (self.items.last(), incoming.first()) {
            (Some(last), Some(first)) => last.identity() < first.identity(),
            _ => true,
        };
        if after_all {
            self.items.extend(incoming);
            return;
        }
        let held = std::mem::take(&mut self.items);
        let mut merged = Vec::with_capacity(held.len() + incoming.len());
        let mut incoming = incoming.into_iter().peekable();
        for item in held {
            while let Some(earlier) = incoming.next_if(|new| new.identity() < item.identity()) {
                merged.push(earlier);
            }
            merged.push(item);
        }
        merged.extend(incoming);
        self.items = merged;
    }

    /// Start keeping what a transaction changes; `since` is the identity
    /// the document makes next.
    pub(super) fn begin(&mut self, since: u64) {
        self.journal = Some(EntityJournal::new(since));
    }

    /// Stop keeping what the transaction changed.
    pub(super) fn end(&mut self) {
        self.journal = None;
    }

    /// Put the list back as it stood when the transaction began.
    pub(super) fn roll_back(&mut self) {
        let Some(journal) = self.journal.take() else {
            return;
        };
        let made = self
            .items
            .partition_point(|item| item.identity() < journal.since);
        for item in self.items.drain(made..) {
            let key = self.index.key(&item);
            self.index.remove(key, item.identity());
        }
        let mut put_back = Vec::new();
        let mut taken_out = Vec::new();
        let mut from = 0;
        for (identity, was) in journal.into_order() {
            match (position_from(&self.items, &mut from, identity), was) {
                (Some(index), Some(was)) => {
                    let now = std::mem::replace(&mut self.items[index], was);
                    let was = &self.items[index];
                    if !self.index.same_key(&now, was) {
                        let (now, was) = (self.index.key(&now), self.index.key(was));
                        self.index.remove(now, identity);
                        self.index.insert(was, identity);
                    }
                }
                (Some(_), None) => taken_out.push(identity),
                (None, Some(was)) => put_back.push(was),
                (None, None) => {}
            }
        }
        if !taken_out.is_empty() {
            self.retain(|item| taken_out.binary_search(&item.identity()).is_err());
        }
        self.merge_in(put_back);
    }

    /// Stop keeping what the transaction changed, and return what puts the
    /// list back as it stood: the identities to take out (those it made or
    /// changed) and the entities to put back (those it changed or took out).
    pub(super) fn finish(&mut self) -> Changed<T> {
        let Some(journal) = self.journal.take() else {
            return Changed::default();
        };
        let made = self
            .items
            .partition_point(|item| item.identity() < journal.since);
        let mut changed = Changed::default();
        let mut from = 0;
        for (identity, was) in journal.into_order() {
            let now =
                position_from(&self.items, &mut from, identity).map(|index| &self.items[index]);
            match (was, now) {
                (Some(was), Some(now)) if was == *now => {}
                (Some(was), Some(_)) => {
                    changed.out.push(identity);
                    changed.back.push(was);
                }
                (Some(was), None) => changed.back.push(was),
                (None, Some(_)) => changed.out.push(identity),
                (None, None) => {}
            }
        }
        changed
            .out
            .extend(self.items[made..].iter().map(Identified::identity));
        changed
    }

    /// Put back what `changed` holds: take out what it takes out and put
    /// back what it holds, each in its place by identity. An entity another
    /// edit took out since is not missed; one another edit made since stays.
    pub(super) fn put_back(&mut self, changed: Changed<T>) {
        let Changed { out, back } = changed;
        let held: HashSet<u64> = back.iter().map(Identified::identity).collect();
        let taken_out: HashSet<u64> = out
            .into_iter()
            .filter(|identity| !held.contains(identity))
            .collect();
        let mut absent = Vec::new();
        for item in back {
            match self.position(item.identity()) {
                Some(index) => self.update_at(index, |now| *now = item),
                None => absent.push(item),
            }
        }
        if !taken_out.is_empty() {
            self.retain(|item| !taken_out.contains(&item.identity()));
        }
        self.merge_in(absent);
    }

    /// Each entity the transaction made or changed, in the list's order.
    pub(super) fn touched(&self) -> Vec<&T> {
        self.touched_with_before()
            .into_iter()
            .map(|(now, _)| now)
            .collect()
    }

    /// Each entity the transaction made or changed, in the list's order,
    /// with what it stood as before: `None` for one it made or put back.
    pub(super) fn touched_with_before(&self) -> Vec<(&T, Option<&T>)> {
        let Some(journal) = &self.journal else {
            return Vec::new();
        };
        let made = self
            .items
            .partition_point(|item| item.identity() < journal.since);
        let mut from = 0;
        let mut touched = Vec::new();
        for (identity, slot) in journal.in_order() {
            if let Some(index) = position_from(&self.items[..made], &mut from, identity) {
                touched.push((&self.items[index], journal.kept[slot].as_ref()));
            }
        }
        touched.extend(self.items[made..].iter().map(|item| (item, None)));
        touched
    }

    /// The identities of the entities the transaction took out, ascending.
    pub(super) fn taken_out(&self) -> Vec<u64> {
        let Some(journal) = &self.journal else {
            return Vec::new();
        };
        let mut from = 0;
        journal
            .in_order()
            .into_iter()
            .filter(|(identity, slot)| {
                journal.kept[*slot].is_some()
                    && position_from(&self.items, &mut from, *identity).is_none()
            })
            .map(|(identity, _)| identity)
            .collect()
    }

    /// The entity `identity` as it stood before the transaction, when the
    /// transaction changed or took it out.
    pub(super) fn before(&self, identity: u64) -> Option<&T> {
        let journal = self.journal.as_ref()?;
        journal.kept[journal.slot(identity)?].as_ref()
    }
}

impl<T: Identified + Clone + PartialEq> EntityList<T, Unindexed> {
    /// The entity `identity`, to change; `None` when the list holds none.
    pub(super) fn get_mut(&mut self, identity: u64) -> Option<&mut T> {
        let index = self.position(identity)?;
        Some(self.at_mut(index))
    }

    /// The entity at `index`, to change.
    pub(super) fn at_mut(&mut self, index: usize) -> &mut T {
        keep_present(&mut self.journal, &self.items[index]);
        &mut self.items[index]
    }

    /// The entities `pick` picks, to change.
    pub(super) fn iter_mut_where(
        &mut self,
        mut pick: impl FnMut(&T) -> bool,
    ) -> impl Iterator<Item = &mut T> {
        let journal = &mut self.journal;
        self.items
            .iter_mut()
            .filter(move |item| pick(item))
            .inspect(move |item| keep_present(journal, item))
    }
}

impl EntityList<Trace, SignalIndex> {
    /// Show or hide each trace of pane `pane_id` reading `signal`, and
    /// whether the pane has one.
    pub(super) fn show_signal(&mut self, pane_id: PaneId, signal: &str, visible: bool) -> bool {
        let key = (pane_id, self.index.hashing.hash_one(signal));
        let (first, rest) = match self.index.traces.get(&key).map(Vec::as_slice) {
            None | Some([]) => return false,
            Some([one]) => (*one, Vec::new()),
            Some([one, more @ ..]) => (*one, more.to_vec()),
        };
        let mut held = false;
        for identity in std::iter::once(first).chain(rest) {
            let Some(index) = self.position(identity) else {
                continue;
            };
            let trace = &self.items[index];
            if trace.pane_id != pane_id || signal_of(trace) != signal {
                continue;
            }
            held = true;
            if trace.visible != visible {
                self.update_at(index, |trace| trace.visible = visible);
            }
        }
        held
    }

    /// Whether pane `pane_id` has a trace reading `signal`, and if so
    /// whether every such trace is shown.
    pub(super) fn drawn(&self, pane_id: PaneId, signal: &str) -> Option<bool> {
        let key = (pane_id, self.index.hashing.hash_one(signal));
        let mut drawn = None;
        for identity in self.index.traces.get(&key)? {
            let Some(index) = self.position(*identity) else {
                continue;
            };
            let trace = &self.items[index];
            if trace.pane_id == pane_id && signal_of(trace) == signal {
                drawn = Some(drawn.unwrap_or(true) && trace.visible);
            }
        }
        drawn
    }

    /// Where each trace of `pane_id` reading `signal` stands, in order.
    pub(super) fn reading(&self, pane_id: PaneId, signal: &str) -> Vec<usize> {
        let key = (pane_id, self.index.hashing.hash_one(signal));
        let mut found: Vec<usize> = self
            .index
            .traces
            .get(&key)
            .into_iter()
            .flatten()
            .filter_map(|identity| self.position(*identity))
            .filter(|index| {
                let trace = &self.items[*index];
                trace.pane_id == pane_id && signal_of(trace) == signal
            })
            .collect();
        found.sort_unstable();
        found
    }
}

impl<T, I> Deref for EntityList<T, I> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.items
    }
}

impl<'a, T, I> IntoIterator for &'a EntityList<T, I> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

impl<T: Clone, I: Clone> Clone for EntityList<T, I> {
    /// A copy holds the entities, not what a running transaction kept.
    fn clone(&self) -> Self {
        Self {
            items: self.items.clone(),
            index: self.index.clone(),
            journal: None,
        }
    }
}

impl<T: PartialEq, I> PartialEq for EntityList<T, I> {
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items
    }
}

impl<T: fmt::Debug, I> fmt::Debug for EntityList<T, I> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.items.fmt(formatter)
    }
}

impl<T: Serialize, I> Serialize for EntityList<T, I> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.items.serialize(serializer)
    }
}

/// A list without identities: per-pane settings, tombstones, comparison
/// receipts and the datasets a document reads.
pub(super) struct SpliceList<T> {
    items: Vec<T>,
    journal: Option<SpliceJournal<T>>,
}

/// What one list without identities kept while a transaction runs: the
/// list stands as it stood before up to `at`, and `tail` is what stood from
/// there.
enum SpliceJournal<T> {
    Untouched,
    From { at: usize, tail: Vec<T> },
}

impl<T: Clone + PartialEq> SpliceList<T> {
    pub(super) fn new(items: Vec<T>) -> Self {
        Self {
            items,
            journal: None,
        }
    }

    pub(super) fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Keep what stands from `at` on, before an edit changes it.
    fn keep_from(&mut self, at: usize) {
        let at = at.min(self.items.len());
        match &mut self.journal {
            None => {}
            Some(journal @ SpliceJournal::Untouched) => {
                *journal = SpliceJournal::From {
                    at,
                    tail: self.items[at..].to_vec(),
                };
            }
            Some(SpliceJournal::From { at: kept, tail }) => {
                if at < *kept {
                    let mut earlier = self.items[at..*kept].to_vec();
                    earlier.append(tail);
                    *tail = earlier;
                    *kept = at;
                }
            }
        }
    }

    /// Add `item` at the end.
    pub(super) fn push(&mut self, item: T) {
        self.keep_from(self.items.len());
        self.items.push(item);
    }

    /// Put `replacement` in place of the items at `range`.
    pub(super) fn splice(&mut self, range: Range<usize>, replacement: Vec<T>) {
        self.keep_from(range.start);
        let end = range.end.min(self.items.len());
        let start = range.start.min(end);
        self.items.splice(start..end, replacement);
    }

    /// The list, to change as a whole.
    pub(super) fn edit(&mut self) -> &mut Vec<T> {
        self.keep_from(0);
        &mut self.items
    }

    /// Take out every item `keep` refuses; a list that loses none is not
    /// changed.
    pub(super) fn retain(&mut self, mut keep: impl FnMut(&T) -> bool) {
        let Some(first) = self.items.iter().position(|item| !keep(item)) else {
            return;
        };
        self.keep_from(first);
        let mut index = 0;
        self.items.retain(|item| {
            let kept = index < first || (index > first && keep(item));
            index += 1;
            kept
        });
    }

    pub(super) fn begin(&mut self) {
        self.journal = Some(SpliceJournal::Untouched);
    }

    pub(super) fn end(&mut self) {
        self.journal = None;
    }

    pub(super) fn roll_back(&mut self) {
        if let Some(SpliceJournal::From { at, tail }) = self.journal.take() {
            self.items.truncate(at);
            self.items.extend(tail);
        }
    }

    /// Stop keeping what the transaction changed, and return the one
    /// stretch that puts the list back as it stood.
    pub(super) fn finish(&mut self) -> Splice<T> {
        match self.journal.take() {
            Some(SpliceJournal::From { at, tail }) => {
                let mut splice = Splice::back_to(&tail, &self.items[at..]);
                splice.at += at;
                splice
            }
            _ => Splice::default(),
        }
    }

    /// Whether the transaction changed the list.
    pub(super) fn is_touched(&self) -> bool {
        matches!(self.journal, Some(SpliceJournal::From { .. }))
    }

    /// The items the transaction added at the end, when it only added.
    pub(super) fn added(&self) -> &[T] {
        match &self.journal {
            Some(SpliceJournal::From { at, tail }) if tail.is_empty() => &self.items[*at..],
            _ => &[],
        }
    }
}

impl<T> Deref for SpliceList<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.items
    }
}

impl<'a, T> IntoIterator for &'a SpliceList<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

impl<T: Clone> Clone for SpliceList<T> {
    fn clone(&self) -> Self {
        Self {
            items: self.items.clone(),
            journal: None,
        }
    }
}

impl<T: PartialEq> PartialEq for SpliceList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items
    }
}

impl<T: fmt::Debug> fmt::Debug for SpliceList<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.items.fmt(formatter)
    }
}

impl<T: Serialize> Serialize for SpliceList<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.items.serialize(serializer)
    }
}
