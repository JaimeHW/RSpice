//! What a swept parameter reaches through the deck's parameter scopes.
//!
//! A sweep that knows exactly which values its parameter decides updates
//! only those at each point, as Spectre and Xyce re-evaluate the dependent
//! expressions and push them into the affected instances, instead of
//! re-elaborating the deck. The knowledge comes from taint propagation
//! through the one type every binding is read and written through,
//! [`ParamContext`]:
//!
//! - the swept binding carries the parameter;
//! - a binding written while an open statement has read a carrying binding
//!   carries it too, which follows `.PARAM` chains, subcircuit instance
//!   parameters and function arguments;
//! - a copied binding carries what its source carried, and every clone of a
//!   scope shares the one observer.
//!
//! Each statement the elaboration brackets (a root card, a flattened
//! element) records the carrying names it read. A read with no statement
//! open is unattributed: its result may land anywhere, so a consumer must
//! take it as reaching everything. Tracking only ever over-approximates.

use super::*;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

/// The source specification a reached root card was read from, and the
/// scope it was read in, so the card's value can be read again at another
/// value of the parameter without reading the deck again.
#[derive(Debug, Clone)]
#[doc(hidden)]
pub struct ReachedSourceSpec {
    pub element: String,
    pub text: String,
    pub scope: ParamContext,
}

/// One statement of the elaboration that read a binding carrying the
/// parameter.
#[derive(Debug, Clone)]
#[doc(hidden)]
pub enum ParameterReachSite {
    /// A card of the deck as the parser read it.
    Card {
        line: usize,
        /// The card's first word, uppercased: `.PARAM`, `VCC`, `.MODEL`.
        head: String,
        /// Whether the card sits inside a `.SUBCKT` definition.
        in_subcircuit: bool,
        /// The elements the card added to the root deck.
        elements: Vec<String>,
        /// The carrying names it read.
        names: Vec<String>,
        /// The independent-source specifications it parsed.
        sources: Vec<ReachedSourceSpec>,
    },
    /// One element as the flattener resolved it in its instance scope. A
    /// subcircuit instance stands for everything it expands to.
    Element { path: String, names: Vec<String> },
}

/// Everything one observed elaboration found the parameter reaching.
#[derive(Debug, Clone, Default)]
#[doc(hidden)]
pub struct ParameterReach {
    pub sites: Vec<ParameterReachSite>,
    /// Where carrying bindings were read with no statement open. Any entry
    /// means the reach is unknown.
    pub unattributed: Vec<String>,
    /// Why the elaboration could not be followed at all.
    pub refusals: Vec<String>,
}

impl ParameterReach {
    /// Whether every read was attributed and nothing refused the tracking.
    pub fn is_complete(&self) -> bool {
        self.unattributed.is_empty() && self.refusals.is_empty()
    }
}

/// An open statement of the elaboration, holding what its enclosing
/// statement had read when it opened. It closes on the observer it opened
/// on, whatever scope the statement leaves behind.
#[derive(Debug)]
#[must_use]
pub(crate) struct ReachStatement {
    observer: Arc<ReachObserver>,
    touched: bool,
    enclosing: bool,
    names: BTreeSet<String>,
    sources: Vec<(String, String)>,
}

impl ReachStatement {
    /// Close the statement; what it read, if it read any carrying binding.
    pub(crate) fn end(self) -> Option<ReachRead> {
        let observer = self.observer.clone();
        observer.end(self)
    }
}

/// What a closed statement read, to be recorded as its site.
#[derive(Debug)]
pub(crate) struct ReachRead {
    observer: Arc<ReachObserver>,
    pub(crate) names: Vec<String>,
    pub(crate) sources: Vec<(String, String)>,
}

impl ReachRead {
    pub(crate) fn record(self, site: ParameterReachSite) {
        let mut log = self.observer.log();
        if !log.closed {
            log.sites.push(site);
        }
    }
}

#[derive(Debug, Default)]
struct ReachLog {
    closed: bool,
    /// Open [`ReachSuspension`]s: reads copying values into metadata.
    suspended: usize,
    depth: usize,
    /// The innermost open statement (or, at depth zero, the elaboration
    /// outside every statement) read a carrying binding.
    touched: bool,
    /// Some enclosing statement had already read one when it was opened.
    enclosing: bool,
    names: BTreeSet<String>,
    sources: Vec<(String, String)>,
    unattributed: BTreeSet<String>,
    sites: Vec<ParameterReachSite>,
    refusals: Vec<String>,
}

/// The log every clone of an observed scope shares.
#[derive(Debug, Default)]
pub(crate) struct ReachObserver {
    /// Set once with the log's own flag, so a scope still holding a closed
    /// observer reads and writes without taking the lock.
    closed: AtomicBool,
    log: Mutex<ReachLog>,
}

impl ReachObserver {
    fn is_closed(&self) -> bool {
        self.closed.load(Ordering::Relaxed)
    }

    fn log(&self) -> std::sync::MutexGuard<'_, ReachLog> {
        self.log
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn read(&self, name: &str, location: &std::panic::Location<'_>) {
        if self.is_closed() {
            return;
        }
        let mut log = self.log();
        if log.closed || log.suspended > 0 {
            return;
        }
        log.touched = true;
        if log.depth == 0 {
            log.unattributed
                .insert(format!("{name} at {}:{}", location.file(), location.line()));
        } else {
            log.names.insert(name.to_owned());
        }
    }

    /// Whether a value written now may carry the parameter.
    fn carrying(&self) -> bool {
        if self.is_closed() {
            return false;
        }
        let log = self.log();
        !log.closed && (log.touched || log.enclosing)
    }

    fn begin(self: &Arc<Self>) -> Option<ReachStatement> {
        if self.is_closed() {
            return None;
        }
        let mut log = self.log();
        if log.closed {
            return None;
        }
        log.depth += 1;
        let enclosing = log.touched || log.enclosing;
        let saved = ReachStatement {
            observer: self.clone(),
            touched: std::mem::take(&mut log.touched),
            enclosing: std::mem::replace(&mut log.enclosing, enclosing),
            names: std::mem::take(&mut log.names),
            sources: std::mem::take(&mut log.sources),
        };
        Some(saved)
    }

    fn end(&self, saved: ReachStatement) -> Option<ReachRead> {
        let mut log = self.log();
        log.depth = log.depth.saturating_sub(1);
        let touched = std::mem::replace(&mut log.touched, saved.touched);
        log.enclosing = saved.enclosing;
        let names = std::mem::replace(&mut log.names, saved.names);
        let sources = std::mem::replace(&mut log.sources, saved.sources);
        if !touched || log.closed {
            return None;
        }
        // An enclosing statement read whatever this one read: an instance
        // stands for every element it expands to. Nothing encloses a root
        // statement, whose reads are its own.
        if log.depth > 0 {
            log.touched = true;
            log.names.extend(names.iter().cloned());
        }
        Some(ReachRead {
            observer: saved.observer,
            names: names.into_iter().collect(),
            sources,
        })
    }
}

/// Reads, while held, that copy values into metadata the elaboration never
/// reads back, as instance paths record their resolved scopes: they decide
/// nothing an elaboration builds, so they are not its reads.
#[must_use]
pub(crate) struct ReachSuspension(Option<Arc<ReachObserver>>);

impl Drop for ReachSuspension {
    fn drop(&mut self) {
        if let Some(observer) = &self.0 {
            let mut log = observer.log();
            log.suspended = log.suspended.saturating_sub(1);
        }
    }
}

/// A scope's reach tracking. A copy taken once the observation has ended
/// has none, so a deck cloned from an observed one evaluates as one that
/// was never observed.
#[derive(Debug, Default)]
pub(crate) struct ReachSlot(Option<Box<ParameterReachScope>>);

impl Clone for ReachSlot {
    fn clone(&self) -> Self {
        Self(
            self.0
                .as_ref()
                .filter(|reach| !reach.observer.is_closed())
                .cloned(),
        )
    }
}

impl std::ops::Deref for ReachSlot {
    type Target = Option<Box<ParameterReachScope>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ReachSlot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// The carrying bindings of one scope, and the observer it reports to.
#[derive(Debug, Clone)]
pub(crate) struct ParameterReachScope {
    observer: Arc<ReachObserver>,
    /// The swept parameter: it carries itself in every scope that has it.
    swept: String,
    ordinary: HashSet<String>,
    global: HashSet<String>,
}

impl ParameterReachScope {
    fn carries(&self, key: &str, global: bool) -> bool {
        if global {
            self.global.contains(key)
        } else {
            self.ordinary.contains(key)
        }
    }

    fn mark(&mut self, key: &str, global: bool, carries: bool) {
        let set = if global {
            &mut self.global
        } else {
            &mut self.ordinary
        };
        if carries || key == self.swept {
            set.insert(key.to_owned());
        } else {
            set.remove(key);
        }
    }
}

impl ParamContext {
    /// Start tracking what the binding `name` reaches in every scope derived
    /// from this one, with a fresh observer.
    pub(crate) fn observe_parameter_reach(&mut self, name: &str, global: bool) {
        let key = name.to_uppercase();
        let mut scope = ParameterReachScope {
            observer: Arc::default(),
            swept: key.clone(),
            ordinary: HashSet::new(),
            global: HashSet::new(),
        };
        scope.mark(&key, global, true);
        *self.reach = Some(Box::new(scope));
    }

    /// Whether this scope's effective binding of `key` carries the parameter.
    fn reach_carries(&self, key: &str) -> bool {
        self.reach.as_ref().is_some_and(|reach| {
            if self.has_parameter_binding(key) {
                reach.carries(key, false)
            } else {
                reach.carries(key, true)
            }
        })
    }

    /// Report a read of `key`'s effective binding.
    #[track_caller]
    pub(super) fn reach_read(&self, key: &str) {
        if let Some(reach) = &*self.reach
            && self.reach_carries(key)
        {
            reach.observer.read(key, std::panic::Location::caller());
        }
    }

    /// Report a read of `key`'s binding in one namespace.
    #[track_caller]
    pub(super) fn reach_read_binding(&self, key: &str, global: bool) {
        if let Some(reach) = &*self.reach
            && reach.carries(key, global)
        {
            reach.observer.read(key, std::panic::Location::caller());
        }
    }

    /// Report a read of every binding at once, as an export does.
    #[track_caller]
    pub(super) fn reach_read_all(&self) {
        if let Some(reach) = &*self.reach
            && let Some(key) = reach.ordinary.iter().chain(&reach.global).min()
        {
            reach.observer.read(key, std::panic::Location::caller());
        }
    }

    /// Report a read of the bindings `keys` name in one namespace, as an
    /// export of one kind of binding does.
    #[track_caller]
    pub(super) fn reach_read_keys<'a>(&self, keys: impl Iterator<Item = &'a String>, global: bool) {
        if let Some(reach) = &*self.reach
            && let Some(key) = keys.filter(|key| reach.carries(key, global)).min()
        {
            reach.observer.read(key, std::panic::Location::caller());
        }
    }

    /// This scope's tracking while its observation is open; a closed one
    /// is dropped here.
    fn open_reach(&mut self) -> Option<&mut ParameterReachScope> {
        if self
            .reach
            .as_ref()
            .is_some_and(|reach| reach.observer.is_closed())
        {
            *self.reach = None;
        }
        self.reach.as_deref_mut()
    }

    /// A binding of `key` was just written in this scope.
    pub(super) fn reach_write(&mut self, key: &str, global: bool) {
        if let Some(reach) = self.open_reach() {
            let carries = reach.observer.carrying();
            reach.mark(key, global, carries);
        }
    }

    /// A binding of `key` was rewritten from what it already held: it keeps
    /// carrying what it carried.
    pub(super) fn reach_keep(&mut self, key: &str, global: bool) {
        if let Some(reach) = self.open_reach() {
            let carries = reach.carries(key, global) || reach.observer.carrying();
            reach.mark(key, global, carries);
        }
    }

    /// A binding of `key` was just copied from `source`'s binding: it
    /// carries what that one carried.
    pub(super) fn reach_copy(&mut self, key: &str, global: bool, source: &ParamContext) {
        let source_carries = source
            .reach
            .as_ref()
            .is_some_and(|reach| reach.carries(key, global));
        if self.reach.is_none()
            && let Some(reach) = &*source.reach
            && !reach.observer.is_closed()
        {
            *self.reach = Some(Box::new(ParameterReachScope {
                observer: reach.observer.clone(),
                swept: reach.swept.clone(),
                ordinary: HashSet::new(),
                global: HashSet::new(),
            }));
        }
        if let Some(reach) = self.open_reach() {
            let carries = source_carries || reach.observer.carrying();
            reach.mark(key, global, carries);
        }
    }

    /// Open a statement of the elaboration.
    pub(crate) fn reach_statement_begin(&self) -> Option<ReachStatement> {
        self.reach.as_ref()?.observer.begin()
    }

    /// Note, in the open statement, the specification text an independent
    /// source `element` was parsed from.
    pub(crate) fn reach_note_source_spec(&self, element: &str, text: &str) {
        if let Some(reach) = &*self.reach {
            let mut log = reach.observer.log();
            if !log.closed && log.depth > 0 {
                log.sources.push((element.to_owned(), text.to_owned()));
            }
        }
    }

    /// The elaboration did something the tracking cannot follow.
    pub(crate) fn reach_refuse(&self, reason: impl Into<String>) {
        if let Some(reach) = &*self.reach {
            let mut log = reach.observer.log();
            if !log.closed {
                log.refusals.push(reason.into());
            }
        }
    }

    /// This scope without its reach tracking, as a site retains it.
    pub(crate) fn without_parameter_reach(&self) -> Self {
        let mut scope = self.clone();
        *scope.reach = None;
        scope
    }

    /// What the observed elaboration found so far.
    #[doc(hidden)]
    pub fn parameter_reach(&self) -> Option<ParameterReach> {
        let reach = self.reach.as_ref()?;
        let log = reach.observer.log();
        Some(ParameterReach {
            sites: log.sites.clone(),
            unattributed: log.unattributed.iter().cloned().collect(),
            refusals: log.refusals.clone(),
        })
    }

    /// Stop the observation for every scope sharing it and drop this scope's
    /// tracking, returning what was found.
    #[doc(hidden)]
    pub fn end_parameter_reach(&mut self) -> Option<ParameterReach> {
        let reach = self.reach.take()?;
        let mut log = reach.observer.log();
        log.closed = true;
        reach.observer.closed.store(true, Ordering::Relaxed);
        Some(ParameterReach {
            sites: std::mem::take(&mut log.sites),
            unattributed: std::mem::take(&mut log.unattributed).into_iter().collect(),
            refusals: std::mem::take(&mut log.refusals),
        })
    }

    /// A copy of this scope that a device keeps to evaluate at run time.
    /// Those later reads happen outside the elaboration a sweep observes, so
    /// taking the copy counts as reading every binding that carries the
    /// parameter.
    #[doc(hidden)]
    #[track_caller]
    pub fn retained_runtime_scope(&self) -> Self {
        self.reach_read_all();
        self.clone()
    }

    /// Suspend the observation every scope sharing this one's reports to,
    /// for reads that only copy values into metadata.
    pub(crate) fn suspend_parameter_reach(&self) -> ReachSuspension {
        ReachSuspension(self.reach.as_ref().map(|reach| {
            reach.observer.log().suspended += 1;
            reach.observer.clone()
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn site(path: &str) -> ParameterReachSite {
        ParameterReachSite::Element {
            path: path.to_owned(),
            names: Vec::new(),
        }
    }

    #[test]
    fn a_binding_written_after_a_carrying_read_carries_the_parameter() {
        let mut scope = ParamContext::new();
        scope.set("A", 1.0);
        scope.set("C", 3.0);
        scope.observe_parameter_reach("A", false);

        let statement = scope.reach_statement_begin().expect("observed");
        let a = scope.get("A").expect("A is bound");
        scope.set("B", 2.0 * a);
        let read = statement.end().expect("the statement read A");
        assert_eq!(read.names, ["A"]);
        read.record(site("first"));

        let statement = scope.reach_statement_begin().expect("observed");
        scope.get("C");
        assert!(statement.end().is_none(), "C carries nothing");

        // A clone shares the observation and the carried bindings.
        let statement = scope.reach_statement_begin().expect("observed");
        let derived = scope.clone();
        derived.get("B");
        assert_eq!(statement.end().expect("B carries A").names, ["B"]);

        let reach = scope.end_parameter_reach().expect("observed");
        assert!(reach.is_complete());
        assert_eq!(reach.sites.len(), 1);
    }

    #[test]
    fn a_nested_statement_reads_for_the_statement_enclosing_it() {
        let mut scope = ParamContext::new();
        scope.set("A", 1.0);
        scope.observe_parameter_reach("A", false);
        let outer = scope.reach_statement_begin().expect("observed");
        let inner = scope.reach_statement_begin().expect("observed");
        scope.get("A");
        assert_eq!(inner.end().expect("inner read A").names, ["A"]);
        assert_eq!(outer.end().expect("outer stands for inner").names, ["A"]);
    }

    #[test]
    fn a_read_outside_every_statement_leaves_the_reach_unknown() {
        let mut scope = ParamContext::new();
        scope.set("A", 1.0);
        scope.observe_parameter_reach("A", false);
        scope.get("A");
        let retained = scope.retained_runtime_scope();
        let reach = scope.end_parameter_reach().expect("observed");
        assert!(!reach.is_complete());
        assert!(
            reach
                .unattributed
                .iter()
                .all(|read| read.starts_with("A at "))
        );
        // The observation is over for every scope that shared it.
        assert!(retained.reach_statement_begin().is_none());
        // A copy taken now carries no tracking at all.
        assert!(retained.clone().reach.is_none());
    }
}
