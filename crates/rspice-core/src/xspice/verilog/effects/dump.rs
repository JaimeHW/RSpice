//! The value change dump tasks (IEEE 1364-2005 sections 18.1 and 18.2):
//! `$dumpfile`, `$dumpvars`, `$dumpon`, `$dumpoff`, `$dumpall`,
//! `$dumplimit` and `$dumpflush`.
//!
//! The dump is a file like any other the design writes: confined to the run
//! directory, journaled, and released when the step commits, so a rejected
//! trial dumps nothing. It is written as a stream rather than as a
//! [`crate::io::vcd::VcdDocument`], because a dump grows for the whole run,
//! but it numbers its variables with the same identifier scheme as
//! [`crate::io::vcd::write_vcd`] and reads back through
//! [`crate::io::vcd::parse_vcd_bytes_with_limits`] like any other.
//!
//! # When things are written
//!
//! `$dumpvars` calls made in one time step select the variables; at the end
//! of that step the header is written and the selection's values are dumped
//! in a `$dumpvars` block (section 18.1.2), after a block of every dumped
//! parameter's value, which never changes. From then on, the end of every
//! step writes the variables whose value changed, after a `#time` line. A
//! later `$dumpvars` is ignored with a warning. `$dumpoff` writes every
//! variable as `x` and stops recording, `$dumpon` writes every current value
//! and resumes, and `$dumpall` writes every current value (sections 18.1.3
//! and 18.1.5). Memories are not dumped (section 18.1.2). The `$date` block
//! is left empty, so a dump is a function of the design and its inputs.
//!
//! # How the header is laid out
//!
//! Section 18.2 leaves the layout to the tool, and the header follows Icarus
//! Verilog's: each `$dumpvars` argument in call order, under the scopes that
//! enclose it, a scope with its variables and then its scopes as deep as the
//! call asks. A scope an earlier argument dumped adds only its scopes, and a
//! variable an earlier argument named is skipped, each with a warning. A
//! port that its connection joined to another scope's net is listed in its
//! own scope with that net's identifier code, as one value under two names.
//! A value block lists the newest variable first, and a time step the
//! variable that first changed last first, as Icarus Verilog writes them.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::sync::Arc;

use rspice_veriloga::canonical_ir::digital::{
    CanonicalDigitalPlan, DigitalParameterValue, DigitalScopeKind, DigitalSystemArgument,
    DigitalSystemCall, DigitalSystemCallKind as Kind,
};
use rspice_veriloga::canonical_ir::digital_eval::{
    DigitalClock, DigitalEnvironment, DigitalEvalError, DigitalScalar,
};
use rspice_veriloga::canonical_ir::digital_value::FourStateValue;
use rspice_veriloga::canonical_ir::ids::DigitalSignalId;
use rspice_veriloga::four_state::FourStateBit;
use serde::{Deserialize, Serialize};

use super::files::{FileOp, text_bytes};
use super::{HdlEffects, HdlStream, integer_of, string_argument};
use crate::xspice::event_trace::EventTracePoint;
use crate::xspice::verilog::store::DigitalSignalStore;
use crate::xspice::{DigitalState, DigitalStrength, DigitalValue, EventValue};

/// Section 18.1.1's default file name.
const DEFAULT_FILE: &str = "dump.vcd";

/// Where the dump is in its life.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum DumpPhase {
    /// No `$dumpvars` yet.
    Idle,
    /// `$dumpvars` ran at this tick; the header is written when it ends.
    Selecting(u64),
    /// The header is written; changes are being dumped.
    Dumping,
    /// The dump limit was reached, or the file could not be written.
    Stopped,
}

/// One dumped variable: a value the dump follows, which one `$var` line
/// names, or several when ports joined to its net name it too.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct DumpVar {
    pub signal: u32,
    pub width: u32,
    /// `[msb:lsb]`, for a vector.
    pub range: Option<(i64, i64)>,
    pub real: bool,
    /// The plan's dumped parameter this variable is, whose value the header's
    /// parameter block writes once; `signal` is then unused.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameter: Option<u32>,
    /// A named event, which carries no value: each value block and each step
    /// it fires write `1`, and `$dumpoff` writes nothing for it, as Icarus
    /// Verilog dumps one.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub event: bool,
}

/// One line of the header, laid out as the `$dumpvars` calls ask.
#[derive(Debug, Clone, PartialEq, Eq)]
enum HeaderLine {
    Scope {
        kind: &'static str,
        name: String,
    },
    Upscope,
    /// A `$var` line naming variable `var`.
    Var {
        var: usize,
        kind: &'static str,
        name: String,
    },
}

/// The value last written for a variable; a real by its bits, so that two
/// values compare equal exactly when they print the same.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum Dumped {
    Bits(String),
    Real(u64),
}

/// The dump of one run, which is also its checkpoint image.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Dump {
    /// The name `$dumpfile` gave, as written.
    pub name: Option<String>,
    pub path: Option<PathBuf>,
    pub phase: DumpPhase,
    pub vars: Vec<DumpVar>,
    /// Signal to the variable dumping it, for the change hook.
    #[serde(skip)]
    by_signal: Arc<BTreeMap<u32, usize>>,
    /// Variables changed in this step, in the order each first changed.
    #[serde(skip)]
    dirty: Vec<usize>,
    /// Which variables are in `dirty`.
    #[serde(skip)]
    scheduled: Vec<bool>,
    pub last: Vec<Option<Dumped>>,
    /// `$dumpoff` stops recording, `$dumpon` resumes it.
    pub recording: bool,
    pub limit: Option<u64>,
    pub written: u64,
    /// The last `#time` written.
    pub time: Option<u64>,
    /// The header as the step's `$dumpvars` calls laid it out, written when
    /// that step ends.
    #[serde(skip)]
    header: Vec<HeaderLine>,
    /// The scopes whose variables those calls dumped, and the variables they
    /// named one by one, by full name.
    #[serde(skip)]
    scanned: BTreeSet<String>,
    #[serde(skip)]
    named: BTreeSet<String>,
    /// The design as the dump lays it out, built at the first `$dumpvars`.
    #[serde(skip)]
    tree: Option<Arc<DumpTree>>,
}

/// Written out so that `clone_from` refills the buffers a trial image already
/// holds: the host's image is refilled from the accepted one before every
/// candidate that writes it (`host/image.rs`), and the derived form freed and
/// reallocated every list and name set here to copy them.
impl Clone for Dump {
    fn clone(&self) -> Self {
        let Self {
            name,
            path,
            phase,
            vars,
            by_signal,
            dirty,
            scheduled,
            last,
            recording,
            limit,
            written,
            time,
            header,
            scanned,
            named,
            tree,
        } = self;
        Self {
            name: name.clone(),
            path: path.clone(),
            phase: *phase,
            vars: vars.clone(),
            by_signal: Arc::clone(by_signal),
            dirty: dirty.clone(),
            scheduled: scheduled.clone(),
            last: last.clone(),
            recording: *recording,
            limit: *limit,
            written: *written,
            time: *time,
            header: header.clone(),
            scanned: scanned.clone(),
            named: named.clone(),
            tree: tree.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        let Self {
            name,
            path,
            phase,
            vars,
            by_signal,
            dirty,
            scheduled,
            last,
            recording,
            limit,
            written,
            time,
            header,
            scanned,
            named,
            tree,
        } = self;
        name.clone_from(&source.name);
        path.clone_from(&source.path);
        *phase = source.phase;
        vars.clone_from(&source.vars);
        by_signal.clone_from(&source.by_signal);
        dirty.clone_from(&source.dirty);
        scheduled.clone_from(&source.scheduled);
        last.clone_from(&source.last);
        *recording = source.recording;
        *limit = source.limit;
        *written = source.written;
        *time = source.time;
        header.clone_from(&source.header);
        scanned.clone_from(&source.scanned);
        named.clone_from(&source.named);
        tree.clone_from(&source.tree);
        debug_assert!(*self == *source, "a refilled dump differs from its source");
    }
}

impl Default for Dump {
    fn default() -> Self {
        Self {
            name: None,
            path: None,
            phase: DumpPhase::Idle,
            vars: Vec::new(),
            by_signal: Arc::default(),
            dirty: Vec::new(),
            scheduled: Vec::new(),
            last: Vec::new(),
            recording: true,
            limit: None,
            written: 0,
            time: None,
            header: Vec::new(),
            scanned: BTreeSet::new(),
            named: BTreeSet::new(),
            tree: None,
        }
    }
}

impl Dump {
    /// Rebuild the change index after a restore.
    pub(crate) fn reindex(&mut self) {
        self.by_signal = Arc::new(
            self.vars
                .iter()
                .enumerate()
                .filter(|(_, var)| var.parameter.is_none())
                .map(|(index, var)| (var.signal, index))
                .collect(),
        );
        self.dirty.clear();
        self.scheduled = vec![false; self.vars.len()];
    }

    /// Whether the change hook has anything to do.
    pub(crate) fn watching(&self) -> bool {
        self.phase == DumpPhase::Dumping && self.recording
    }

    /// A signal changed value.
    pub(crate) fn note_change(&mut self, signal: DigitalSignalId) {
        if let Some(&index) = self.by_signal.get(&signal.index())
            && let Some(scheduled) = self.scheduled.get_mut(index)
            && !std::mem::replace(scheduled, true)
        {
            self.dirty.push(index);
        }
    }

    /// The variables changed in this step, in the order each first changed,
    /// leaving none.
    fn take_dirty(&mut self) -> Vec<usize> {
        let dirty = std::mem::take(&mut self.dirty);
        for &index in &dirty {
            self.scheduled[index] = false;
        }
        dirty
    }
}

/// The instance and port a linked port net joins, read from its name
/// (`@port:2:x1:0:in` is port `in` of `x1`), or `None` for any other signal.
/// The instance length comes first because an instance path may hold any
/// character.
pub(crate) fn port_net_spelling(name: &str) -> Option<(&str, &str)> {
    let rest = name.strip_prefix("@port:")?;
    let (length, rest) = rest.split_once(':')?;
    let instance = rest.get(..length.parse().ok()?)?;
    let rest = rest.get(instance.len()..)?.strip_prefix(':')?;
    let (_, port) = rest.split_once(':')?;
    Some((instance, port))
}

/// Whether `plan` was linked from deck instances. Linking prefixes every
/// signal with its instance path, which is also its root scope's path, and
/// adds nets named `@…`; a flattened plan names its root's own signals bare.
fn is_linked(plan: &CanonicalDigitalPlan) -> bool {
    let roots: Vec<&str> = plan
        .scopes
        .iter()
        .filter(|scope| scope.parent.is_none())
        .map(|scope| scope.path.as_str())
        .collect();
    !roots.is_empty()
        && plan.signals.iter().all(|signal| {
            signal.name.starts_with('@')
                || roots.iter().any(|root| {
                    signal
                        .name
                        .strip_prefix(root)
                        .is_some_and(|rest| rest.starts_with('.'))
                })
        })
}

/// A signal's place in the hierarchy: its enclosing scope path and its own
/// name. A flattened plan names a child's signal `u1.x` under the root it was
/// compiled for; a linked plan prefixes every name with its root, which is an
/// instance path that may have dots of its own (`x1.xtb`), and names the net
/// a port joins after the port (see [`port_net_spelling`]).
fn hierarchy(plan: &CanonicalDigitalPlan, name: &str, linked: bool) -> (String, String) {
    if let Some((instance, port)) = port_net_spelling(name) {
        return (instance.to_owned(), port.to_owned());
    }
    let roots: Vec<&str> = plan
        .scopes
        .iter()
        .filter(|scope| scope.parent.is_none())
        .map(|scope| scope.path.as_str())
        .collect();
    let rooted = linked
        || (roots.len() > 1
            && roots.iter().any(|root| {
                name.strip_prefix(root)
                    .is_some_and(|rest| rest.starts_with('.'))
            }));
    let full = if rooted || roots.is_empty() {
        name.to_owned()
    } else {
        format!("{}.{name}", roots[0])
    };
    match full.rsplit_once('.') {
        Some((scope, leaf)) => (scope.to_owned(), leaf.to_owned()),
        None => (String::new(), full),
    }
}

/// The design's scopes and variables as a dump lays them out (IEEE 1364-2005
/// section 18.2.3.4). Each scope lists its variables and then its scopes,
/// each group in the order Icarus Verilog lists it: named events, nets,
/// parameters, variables, then integers and reals; functions, generate blocks, module
/// instances, named blocks, then tasks; by name within each.
#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct DumpTree {
    nodes: Vec<DumpScope>,
    by_path: BTreeMap<String, usize>,
    roots: Vec<usize>,
    linked: bool,
}

#[derive(Debug, PartialEq, Eq)]
struct DumpScope {
    path: String,
    name: String,
    kind: &'static str,
    rank: u8,
    parent: Option<usize>,
    vars: Vec<DumpMember>,
    children: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq)]
struct DumpMember {
    rank: u8,
    name: String,
    kind: &'static str,
    item: DumpItem,
}

/// What a dumped variable follows: a signal, or a parameter of
/// [`CanonicalDigitalPlan::dump_parameters`], whose value never changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DumpItem {
    Signal(u32),
    Parameter(u32),
}

impl DumpTree {
    fn new(plan: &CanonicalDigitalPlan) -> Self {
        let mut tree = Self {
            linked: is_linked(plan),
            ..Self::default()
        };
        for scope in &plan.scopes {
            let kind = match scope.kind {
                DigitalScopeKind::Function => ("function", 0),
                DigitalScopeKind::Generate => ("begin", 1),
                DigitalScopeKind::Module => ("module", 2),
                DigitalScopeKind::Block => ("begin", 3),
                DigitalScopeKind::Task => ("task", 4),
            };
            tree.scope(&scope.path, Some(kind));
        }
        let memories: BTreeSet<u32> = plan
            .arrays
            .iter()
            .filter_map(|array| array.storage.cell_range())
            .flatten()
            .collect();
        // A variable port keeps its own storage beside the net it drives, and
        // that storage is what the source declared: the net would be a second
        // `o` in the same scope.
        let own_storage: BTreeSet<&str> = plan
            .signals
            .iter()
            .map(|signal| signal.name.as_str())
            .filter(|name| !name.starts_with('@'))
            .collect();
        for (index, signal) in plan.signals.iter().enumerate() {
            let index = index as u32;
            let real = signal.kind.is_real();
            // Storage a process keeps without a name a dump can list it by,
            // and memories (section 18.1.2), are left out.
            if (signal.local.is_some() && signal.name.starts_with('$'))
                || memories.contains(&index)
                || (!real && signal.width == 0)
            {
                continue;
            }
            if let Some((instance, port)) = port_net_spelling(&signal.name)
                && own_storage.contains(format!("{instance}.{port}").as_str())
            {
                continue;
            }
            let (rank, kind) = if signal.event {
                (0, "event")
            } else if real {
                (4, "real")
            } else if signal.integer {
                (4, "integer")
            } else if signal.procedurally_assignable {
                (3, "reg")
            } else {
                (1, "wire")
            };
            tree.member(plan, &signal.name, rank, kind, index);
        }
        for alias in &plan.signal_aliases {
            tree.member(plan, &alias.name, 1, "wire", alias.signal.index());
        }
        // Parameters follow the nets, as Icarus Verilog lists them; a real
        // one is a `real`, the VCD having no real parameter type.
        for (index, parameter) in plan.dump_parameters.iter().enumerate() {
            let Some(scope) = plan.scopes.get(parameter.scope.index()) else {
                continue;
            };
            let node = tree.scope(&scope.path, None);
            let kind = match parameter.value {
                DigitalParameterValue::Real(_) => "real",
                DigitalParameterValue::FourState(_) => "parameter",
            };
            let scope = &mut tree.nodes[node];
            if !scope.vars.iter().any(|var| var.name == parameter.name) {
                scope.vars.push(DumpMember {
                    rank: 2,
                    name: parameter.name.to_string(),
                    kind,
                    item: DumpItem::Parameter(index as u32),
                });
            }
        }
        let keys: Vec<(u8, String)> = tree
            .nodes
            .iter()
            .map(|node| (node.rank, node.name.clone()))
            .collect();
        for node in &mut tree.nodes {
            node.children.sort_by(|a, b| keys[*a].cmp(&keys[*b]));
            node.vars
                .sort_by(|a, b| (a.rank, &a.name).cmp(&(b.rank, &b.name)));
        }
        tree
    }

    /// The scope at `path`, made with its parents if new; `kind` is the
    /// plan's for it, and a scope only a path names is a module.
    fn scope(&mut self, path: &str, kind: Option<(&'static str, u8)>) -> usize {
        if let Some(&node) = self.by_path.get(path) {
            if let Some((kind, rank)) = kind {
                self.nodes[node].kind = kind;
                self.nodes[node].rank = rank;
            }
            return node;
        }
        let (parent, name) = match path.rsplit_once('.') {
            Some((above, name)) => (Some(self.scope(above, None)), name),
            None => (None, path),
        };
        let (kind, rank) = kind.unwrap_or(("module", 2));
        let node = self.nodes.len();
        self.nodes.push(DumpScope {
            path: path.to_owned(),
            name: name.to_owned(),
            kind,
            rank,
            parent,
            vars: Vec::new(),
            children: Vec::new(),
        });
        self.by_path.insert(path.to_owned(), node);
        match parent {
            Some(parent) => self.nodes[parent].children.push(node),
            None => self.roots.push(node),
        }
        node
    }

    /// Add the variable a plan names `name` to its scope, once.
    fn member(
        &mut self,
        plan: &CanonicalDigitalPlan,
        name: &str,
        rank: u8,
        kind: &'static str,
        signal: u32,
    ) {
        let (scope, name) = hierarchy(plan, name, self.linked);
        // A net elaboration made for a port expression has no source name.
        if name.starts_with("__rspice_") {
            return;
        }
        let node = self.scope(&scope, None);
        let scope = &mut self.nodes[node];
        // A function's result is not one of its variables.
        if (scope.kind == "function" && name == scope.name)
            || scope.vars.iter().any(|var| var.name == name)
        {
            return;
        }
        scope.vars.push(DumpMember {
            rank,
            name,
            kind,
            item: DumpItem::Signal(signal),
        });
    }

    /// The scope and place of the variable a plan names `name`.
    fn var(&self, plan: &CanonicalDigitalPlan, name: &str, signal: u32) -> Option<(usize, usize)> {
        let (scope, name) = hierarchy(plan, name, self.linked);
        let node = *self.by_path.get(&scope)?;
        let member = self.nodes[node]
            .vars
            .iter()
            .position(|var| var.name == name && var.item == DumpItem::Signal(signal))?;
        Some((node, member))
    }
}

/// A four-state value as VCD writes it: `0`/`1`/`x`/`z` for a scalar, `b`
/// and the bits for a vector, with the leading bits a reader extends back
/// left off (section 18.2.3.8).
fn spell(value: &FourStateValue, vector: bool) -> String {
    let bits: String = value
        .bits_msb_first()
        .iter()
        .map(|bit| match bit {
            FourStateBit::Zero => '0',
            FourStateBit::One => '1',
            FourStateBit::Unknown => 'x',
            FourStateBit::HighImpedance => 'z',
        })
        .collect();
    if !vector {
        return bits;
    }
    let bytes = bits.as_bytes();
    let mut start = 0;
    while start + 1 < bytes.len() {
        let (lead, next) = (bytes[start], bytes[start + 1]);
        // A reader extends with `0` past a leading `0` or `1`, so a `1` is
        // never redundant; `x` and `z` extend themselves.
        let redundant = match lead {
            b'0' => next == b'0' || next == b'1',
            b'1' => false,
            other => next == other,
        };
        if !redundant {
            break;
        }
        start += 1;
    }
    format!("b{}", &bits[start..])
}

fn current(store: &DigitalSignalStore, var: &DumpVar) -> Option<Dumped> {
    let signal = DigitalSignalId::new(var.signal);
    if var.event {
        Some(Dumped::Bits("1".to_owned()))
    } else if var.real {
        store
            .read_real_signal(signal)
            .map(|value| Dumped::Real(value.to_bits()))
    } else {
        store
            .read_signal(signal)
            .map(|value| Dumped::Bits(spell(&value, var.width > 1 || var.range.is_some())))
    }
}

fn change_line(value: &Dumped, identifier: &str) -> String {
    match value {
        Dumped::Bits(bits) if bits.starts_with('b') => format!("{bits} {identifier}\n"),
        Dumped::Bits(bit) => format!("{bit}{identifier}\n"),
        Dumped::Real(bits) => format!(
            "r{} {identifier}\n",
            super::format::dump_real(f64::from_bits(*bits))
        ),
    }
}

/// One HDL variable a run publishes as a trace (`.ESAVE xtb.pass`): its
/// signal, and the slot its first published value goes to. A real takes one
/// slot; a four-state variable one per bit, most significant first, which is
/// the order a bus over its bits names its members in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TracedVariable {
    pub signal: DigitalSignalId,
    pub real: bool,
    pub slot: usize,
}

/// The HDL variables a run publishes as traces, observed as the dump
/// observes the variables it dumps: the change hook marks a variable whose
/// signal changed, and the end of the time step publishes the value the step
/// left, at the step's instant.
///
/// The publications are a journal inside the host's rollback image, like the
/// output journal, so a rejected trial publishes nothing. The run drains it at
/// every accepted point, together with every variable's value there, and a
/// checkpoint is taken only once it is drained; so the observer holds nothing
/// a checkpoint carries, and a resumed run publishes each value at its first
/// point and goes on from there. A publication is an
/// [`EventTracePoint`] whose `node` is the slot.
#[derive(Debug, Clone, Default)]
pub(crate) struct TracedVariables {
    vars: Arc<[TracedVariable]>,
    /// Signal to the variable publishing it, for the change hook.
    by_signal: Arc<BTreeMap<u32, usize>>,
    /// Variables changed in this step.
    dirty: BTreeSet<usize>,
    journal: Vec<EventTracePoint>,
}

impl TracedVariables {
    pub(crate) fn new(vars: Vec<TracedVariable>) -> Self {
        Self {
            by_signal: Arc::new(
                vars.iter()
                    .enumerate()
                    .map(|(index, var)| (var.signal.index(), index))
                    .collect(),
            ),
            vars: vars.into(),
            dirty: BTreeSet::new(),
            journal: Vec::new(),
        }
    }

    /// The same variables, with nothing yet published: what a host that
    /// starts an analysis afresh traces.
    pub(crate) fn fresh(&self) -> Self {
        Self {
            vars: Arc::clone(&self.vars),
            by_signal: Arc::clone(&self.by_signal),
            dirty: BTreeSet::new(),
            journal: Vec::new(),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.vars.is_empty()
    }

    /// Whether every publication has been drained, which a checkpoint needs.
    pub(crate) fn is_drained(&self) -> bool {
        self.journal.is_empty() && self.dirty.is_empty()
    }

    /// Forget what a restored image never saw.
    pub(crate) fn clear_pending(&mut self) {
        self.dirty.clear();
        self.journal.clear();
    }

    /// A signal changed value.
    pub(crate) fn note_change(&mut self, signal: DigitalSignalId) {
        if let Some(index) = self.by_signal.get(&signal.index()) {
            self.dirty.insert(*index);
        }
    }

    /// The end of a time step: publish what the step left of each variable
    /// that changed in it.
    pub(crate) fn end_of_step(&mut self, store: &DigitalSignalStore, clock: Option<DigitalClock>) {
        if self.dirty.is_empty() {
            return;
        }
        let dirty = std::mem::take(&mut self.dirty);
        let Some(clock) = clock else {
            return;
        };
        for index in dirty {
            publish(
                store,
                &self.vars[index],
                clock.absolute_seconds,
                &mut self.journal,
            );
        }
    }

    /// Move the journal into `points`. The value [`Self::snapshot`] publishes
    /// next answers any change still marked.
    pub(crate) fn drain_into(&mut self, points: &mut Vec<EventTracePoint>) {
        self.dirty.clear();
        points.append(&mut self.journal);
    }

    /// Every variable's value at the accepted `time`, which the run settles
    /// after the journal's.
    pub(crate) fn snapshot(
        &self,
        time: f64,
        store: &DigitalSignalStore,
        points: &mut Vec<EventTracePoint>,
    ) {
        for var in self.vars.iter() {
            publish(store, var, time, points);
        }
    }
}

fn publish(
    store: &DigitalSignalStore,
    var: &TracedVariable,
    time: f64,
    points: &mut Vec<EventTracePoint>,
) {
    if var.real {
        if let Some(value) = store.read_real_signal(var.signal) {
            points.push(EventTracePoint {
                time,
                node: var.slot,
                value: EventValue::Real(value),
            });
        }
        return;
    }
    let Some(value) = store.read_signal(var.signal) else {
        return;
    };
    for (offset, bit) in value.bits_msb_first().iter().enumerate() {
        let value = match bit {
            FourStateBit::Zero => DigitalValue::zero(),
            FourStateBit::One => DigitalValue::one(),
            FourStateBit::Unknown => DigitalValue::unknown(),
            FourStateBit::HighImpedance => {
                DigitalValue::new(DigitalState::HighZ, DigitalStrength::Strong)
            }
        };
        points.push(EventTracePoint {
            time,
            node: var.slot + offset,
            value: EventValue::Digital(value),
        });
    }
}

impl HdlEffects {
    /// One of the dump tasks.
    pub(super) fn dump_task(
        &mut self,
        plan: &CanonicalDigitalPlan,
        site: &DigitalSystemCall,
        args: &[DigitalScalar],
        clock: Option<DigitalClock>,
        store: &mut DigitalSignalStore,
    ) -> Result<(), DigitalEvalError> {
        let tick = clock.map_or(0, |clock| clock.tick);
        match site.kind {
            Kind::Dumpfile => {
                let Some(name) = string_argument(site, args, 0) else {
                    self.warn(plan, site, clock, "the file name is not a string");
                    return Ok(());
                };
                if self.dump.phase == DumpPhase::Idle {
                    self.dump.name = Some(name);
                } else {
                    self.warn(
                        plan,
                        site,
                        clock,
                        "ignored: the dump has already started (IEEE 1364-2005 section 18.1.1)",
                    );
                }
            }
            Kind::Dumpvars => match self.dump.phase {
                DumpPhase::Idle | DumpPhase::Selecting(_) => {
                    self.select_dump_vars(plan, site, args, clock);
                    self.dump.phase = DumpPhase::Selecting(tick);
                }
                DumpPhase::Dumping | DumpPhase::Stopped => self.warn(
                    plan,
                    site,
                    clock,
                    "ignored: every `$dumpvars` must run in the step the dump starts \
                     (IEEE 1364-2005 section 18.1.2)",
                ),
            },
            Kind::Dumpoff | Kind::Dumpon | Kind::Dumpall => {
                if self.dump.phase != DumpPhase::Dumping {
                    if site.kind != Kind::Dumpall {
                        self.dump.recording = site.kind == Kind::Dumpon;
                    }
                    return Ok(());
                }
                let block = match site.kind {
                    Kind::Dumpoff if self.dump.recording => "dumpoff",
                    Kind::Dumpon if !self.dump.recording => "dumpon",
                    Kind::Dumpall if self.dump.recording => "dumpall",
                    _ => return Ok(()),
                };
                // Changes already made in this step come first.
                self.dump_changes(plan, Some(site), clock, store, tick);
                self.dump_block(plan, Some(site), clock, store, tick, block);
            }
            Kind::Dumplimit => match integer_of(
                match site.arguments.first() {
                    Some(DigitalSystemArgument::Value { operand, .. }) => {
                        args.get(*operand as usize)
                    }
                    _ => None,
                },
                true,
            ) {
                Some(limit) if limit >= 0 => self.dump.limit = Some(limit as u64),
                _ => self.warn(plan, site, clock, "the limit is not a non-negative integer"),
            },
            // Every write reaches the file when the step commits.
            Kind::Dumpflush => {}
            _ => {}
        }
        Ok(())
    }

    /// Lay out `$dumpvars(levels, scope_or_variable, ...)`'s selection as
    /// Icarus Verilog does: each argument under the scopes enclosing it, a
    /// scope with its variables and, `levels` deep (all when 0, section
    /// 18.1.2), its scopes. A scope already dumped dumps only its scopes, and
    /// a variable already named is skipped.
    fn select_dump_vars(
        &mut self,
        plan: &CanonicalDigitalPlan,
        site: &DigitalSystemCall,
        args: &[DigitalScalar],
        clock: Option<DigitalClock>,
    ) {
        let levels = match site.arguments.first() {
            Some(DigitalSystemArgument::Value { operand, class }) => integer_of(
                args.get(*operand as usize),
                matches!(
                    class,
                    rspice_veriloga::canonical_ir::digital::DigitalValueClass::FourState {
                        signed: true,
                        ..
                    }
                ),
            )
            .unwrap_or(0)
            .max(0) as usize,
            _ => 0,
        };
        let levels = if levels == 0 { usize::MAX } else { levels };
        let tree = Arc::clone(
            self.dump
                .tree
                .get_or_insert_with(|| Arc::new(DumpTree::new(plan))),
        );
        // A scope, or a variable as its scope and place.
        let mut items: Vec<(usize, Option<usize>)> = Vec::new();
        for argument in site.arguments.iter().skip(1) {
            let item = match argument {
                DigitalSystemArgument::Signal(signal) => plan
                    .signals
                    .get(signal.index() as usize)
                    .and_then(|declared| tree.var(plan, &declared.name, signal.index())),
                DigitalSystemArgument::Alias(alias) => plan
                    .signal_aliases
                    .get(*alias as usize)
                    .and_then(|alias| tree.var(plan, &alias.name, alias.signal.index())),
                DigitalSystemArgument::Scope(scope) => {
                    let found = plan
                        .scopes
                        .get(scope.index())
                        .and_then(|scope| tree.by_path.get(scope.path.as_str()));
                    items.extend(found.map(|&node| (node, None)));
                    None
                }
                _ => None,
            };
            items.extend(item.map(|(node, member)| (node, Some(member))));
        }
        if site.arguments.len() <= 1 {
            items = tree.roots.iter().map(|&root| (root, None)).collect();
        }
        let mut vars: BTreeMap<u32, usize> = self
            .dump
            .vars
            .iter()
            .enumerate()
            .filter(|(_, var)| var.parameter.is_none())
            .map(|(index, var)| (var.signal, index))
            .collect();
        for (node, member) in items {
            let scope = &tree.nodes[node];
            match member {
                Some(member) => {
                    let var = &scope.vars[member];
                    let full = format!("{}.{}", scope.path, var.name);
                    if self.dump.scanned.contains(&scope.path) || self.dump.named.contains(&full) {
                        self.warn(
                            plan,
                            site,
                            clock,
                            &format!("skipping `{full}`, which is already dumped"),
                        );
                        continue;
                    }
                    let depth = self.open_dump_scopes(&tree, Some(node));
                    self.dump_member(plan, var, &mut vars);
                    self.close_dump_scopes(depth);
                    self.dump.named.insert(full);
                }
                None => {
                    let depth = self.open_dump_scopes(&tree, scope.parent);
                    self.scan_dump_scope(plan, site, clock, (&tree, node), levels, &mut vars);
                    self.close_dump_scopes(depth);
                }
            }
        }
    }

    /// Open `node` and the scopes enclosing it, outermost first; how many.
    fn open_dump_scopes(&mut self, tree: &DumpTree, node: Option<usize>) -> usize {
        let mut chain = Vec::new();
        let mut at = node;
        while let Some(node) = at {
            chain.push(node);
            at = tree.nodes[node].parent;
        }
        for &node in chain.iter().rev() {
            self.dump.header.push(HeaderLine::Scope {
                kind: tree.nodes[node].kind,
                name: tree.nodes[node].name.clone(),
            });
        }
        chain.len()
    }

    fn close_dump_scopes(&mut self, depth: usize) {
        self.dump
            .header
            .extend(std::iter::repeat_n(HeaderLine::Upscope, depth));
    }

    /// A scope with its variables, unless an earlier call dumped them, and
    /// its scopes `levels` deep.
    fn scan_dump_scope(
        &mut self,
        plan: &CanonicalDigitalPlan,
        site: &DigitalSystemCall,
        clock: Option<DigitalClock>,
        (tree, node): (&DumpTree, usize),
        levels: usize,
        vars: &mut BTreeMap<u32, usize>,
    ) {
        let scope = &tree.nodes[node];
        let again = !self.dump.scanned.insert(scope.path.clone());
        if again {
            self.warn(
                plan,
                site,
                clock,
                &format!(
                    "the variables of `{}` are already dumped; only its scopes are added",
                    scope.path
                ),
            );
        }
        self.dump.header.push(HeaderLine::Scope {
            kind: scope.kind,
            name: scope.name.clone(),
        });
        if !again {
            for member in &scope.vars {
                if !self
                    .dump
                    .named
                    .contains(&format!("{}.{}", scope.path, member.name))
                {
                    self.dump_member(plan, member, vars);
                }
            }
        }
        if levels > 1 {
            for &child in &scope.children {
                self.scan_dump_scope(plan, site, clock, (tree, child), levels - 1, vars);
            }
        }
        self.dump.header.push(HeaderLine::Upscope);
    }

    /// A `$var` line for `member`, whose signal is a new variable unless a
    /// line already names it; a parameter is a new variable each time.
    fn dump_member(
        &mut self,
        plan: &CanonicalDigitalPlan,
        member: &DumpMember,
        vars: &mut BTreeMap<u32, usize>,
    ) {
        let var = match member.item {
            DumpItem::Signal(signal) => {
                let Some(declared) = plan.signals.get(signal as usize) else {
                    return;
                };
                *vars.entry(signal).or_insert_with(|| {
                    let real = declared.kind.is_real();
                    self.dump.vars.push(DumpVar {
                        signal,
                        width: if real { 64 } else { declared.width },
                        range: declared.bounds.filter(|_| !real),
                        real,
                        parameter: None,
                        event: declared.event,
                    });
                    self.dump.vars.len() - 1
                })
            }
            DumpItem::Parameter(index) => {
                let Some(parameter) = plan.dump_parameters.get(index as usize) else {
                    return;
                };
                let (width, real) = match &parameter.value {
                    DigitalParameterValue::FourState(value) => (value.width(), false),
                    DigitalParameterValue::Real(_) => (64, true),
                };
                self.dump.vars.push(DumpVar {
                    signal: u32::MAX,
                    width,
                    range: None,
                    real,
                    parameter: Some(index),
                    event: false,
                });
                self.dump.vars.len() - 1
            }
        };
        self.dump.header.push(HeaderLine::Var {
            var,
            kind: member.kind,
            name: member.name.clone(),
        });
    }

    /// The run ends at `clock`'s time: a dump still recording ends with that
    /// time, as Icarus Verilog closes its dump, unless it is already the last.
    pub(super) fn dump_end(
        &mut self,
        plan: &CanonicalDigitalPlan,
        site: Option<&DigitalSystemCall>,
        clock: Option<DigitalClock>,
    ) {
        if !self.dump.watching() {
            return;
        }
        if let Some(text) = self.timestamp(clock.map_or(0, |clock| clock.tick)) {
            self.dump_write(plan, site, clock, text);
        }
    }

    /// The run ended at `clock`'s time without `$finish`, `$stop` or
    /// `$fatal`: its last event's time for a digital run, its stop time for a
    /// mixed one. The dump closes as `$finish` closes it, as Icarus Verilog
    /// closes a dump at the end of simulation however it ends.
    pub(crate) fn dump_run_end(
        &mut self,
        plan: &CanonicalDigitalPlan,
        clock: Option<DigitalClock>,
    ) {
        let tick = clock.map_or(0, |clock| clock.tick);
        if self.dump_open_at_run_end() && self.dump.time.is_none_or(|last| last < tick) {
            self.dump_end(plan, None, clock);
        }
    }

    /// Whether the run's end has a dump to close: one still recording that
    /// no `$finish`, `$stop` or `$fatal` closed.
    pub(crate) fn dump_open_at_run_end(&self) -> bool {
        self.finish.is_none() && self.dump.watching()
    }

    /// Append dump text to the dump file.
    fn dump_write(
        &mut self,
        plan: &CanonicalDigitalPlan,
        site: Option<&DigitalSystemCall>,
        clock: Option<DigitalClock>,
        text: String,
    ) {
        let Some(path) = self.dump.path.clone() else {
            return;
        };
        let bytes = text_bytes(&text);
        if let Some(limit) = self.dump.limit
            && self.dump.written + bytes.len() as u64 > limit
        {
            // Section 18.1.4: the dump stops at the limit, saying so.
            self.dump.phase = DumpPhase::Stopped;
            let note = "$comment\n\tdump limit reached\n$end\n".to_owned();
            self.dump.written += note.len() as u64;
            self.files.journal(FileOp::Append {
                path,
                bytes: text_bytes(&note),
            });
            self.dump_record(plan, site, clock, note);
            return;
        }
        self.dump.written += bytes.len() as u64;
        self.files.journal(FileOp::Append { path, bytes });
        self.dump_record(plan, site, clock, text);
    }

    fn dump_record(
        &mut self,
        plan: &CanonicalDigitalPlan,
        site: Option<&DigitalSystemCall>,
        clock: Option<DigitalClock>,
        text: String,
    ) {
        let stream = HdlStream::File {
            handle: 0,
            path: self.dump.name.clone().unwrap_or_default(),
        };
        match site {
            Some(site) => self.record(plan, site, clock, stream, text),
            None => self.record_unsited(clock, stream, text),
        }
    }

    fn timestamp(&mut self, tick: u64) -> Option<String> {
        (self.dump.time != Some(tick)).then(|| {
            self.dump.time = Some(tick);
            format!("#{tick}\n")
        })
    }

    /// Write the variables that changed in this step.
    fn dump_changes(
        &mut self,
        plan: &CanonicalDigitalPlan,
        site: Option<&DigitalSystemCall>,
        clock: Option<DigitalClock>,
        store: &DigitalSignalStore,
        tick: u64,
    ) {
        if self.dump.dirty.is_empty() {
            return;
        }
        let dirty = self.dump.take_dirty();
        if !self.dump.watching() {
            return;
        }
        // The last variable to change first comes first, as Icarus Verilog
        // writes a time step.
        let mut text = String::new();
        for &index in dirty.iter().rev() {
            let var = &self.dump.vars[index];
            let Some(value) = current(store, var) else {
                continue;
            };
            // An event fired in this step however many times its storage
            // toggled, back to its old level or not.
            if !var.event && self.dump.last[index].as_ref() == Some(&value) {
                continue;
            }
            text.push_str(&change_line(
                &value,
                &crate::io::vcd::canonical_identifier(index),
            ));
            self.dump.last[index] = Some(value);
        }
        if !text.is_empty() {
            let mut out = self.timestamp(tick).unwrap_or_default();
            out.push_str(&text);
            self.dump_write(plan, site, clock, out);
        }
    }

    /// A `$dumpvars`, `$dumpall`, `$dumpon` or `$dumpoff` block of every
    /// variable.
    fn dump_block(
        &mut self,
        plan: &CanonicalDigitalPlan,
        site: Option<&DigitalSystemCall>,
        clock: Option<DigitalClock>,
        store: &DigitalSignalStore,
        tick: u64,
        block: &str,
    ) {
        let mut text = self.timestamp(tick).unwrap_or_default();
        text.push_str(&format!("${block}\n"));
        // The newest variable first, as Icarus Verilog writes a block; a
        // parameter's value is in the header's parameter block.
        for (index, var) in self.dump.vars.iter().enumerate().rev() {
            if var.parameter.is_some() {
                continue;
            }
            let identifier = crate::io::vcd::canonical_identifier(index);
            if block == "dumpoff" {
                // Section 18.1.3: every variable is dumped as `x`; a real has
                // no unknown, and keeps its value, and an event has no value.
                if !var.real && !var.event {
                    let vector = var.width > 1 || var.range.is_some();
                    text.push_str(&if vector {
                        format!("bx {identifier}\n")
                    } else {
                        format!("x{identifier}\n")
                    });
                }
                continue;
            }
            if let Some(value) = current(store, var) {
                text.push_str(&change_line(&value, &identifier));
                self.dump.last[index] = Some(value);
            }
        }
        text.push_str("$end\n");
        match block {
            "dumpoff" => self.dump.recording = false,
            "dumpon" => self.dump.recording = true,
            _ => {}
        }
        self.dump.take_dirty();
        self.dump_write(plan, site, clock, text);
    }

    /// The end of a time step: write the header once the selection's step
    /// ends, and every change after it; and publish the traced variables
    /// that changed.
    pub(crate) fn end_of_step(
        &mut self,
        plan: &CanonicalDigitalPlan,
        store: &DigitalSignalStore,
        clock: Option<DigitalClock>,
    ) {
        self.traced.end_of_step(store, clock);
        let tick = clock.map_or(0, |clock| clock.tick);
        match self.dump.phase {
            DumpPhase::Selecting(_) => self.start_dump(plan, store, clock, tick),
            DumpPhase::Dumping => self.dump_changes(plan, None, clock, store, tick),
            DumpPhase::Idle | DumpPhase::Stopped => {}
        }
    }

    fn start_dump(
        &mut self,
        plan: &CanonicalDigitalPlan,
        store: &DigitalSignalStore,
        clock: Option<DigitalClock>,
        tick: u64,
    ) {
        let name = self
            .dump
            .name
            .get_or_insert_with(|| DEFAULT_FILE.to_owned())
            .clone();
        let options = self.config.files.clone();
        let path = match options.resolve_write(&name) {
            Ok(path) if options.store.can_create(&path) => path,
            Ok(_) => {
                self.dump.phase = DumpPhase::Stopped;
                self.warn_unsited(clock, &format!("$dumpfile: cannot create `{name}`"));
                return;
            }
            Err(message) => {
                self.dump.phase = DumpPhase::Stopped;
                self.warn_unsited(clock, &format!("$dumpfile: {message}"));
                return;
            }
        };
        self.files.journal(FileOp::Create {
            path: path.clone(),
            truncate: true,
        });
        self.dump.path = Some(path);
        self.dump.phase = DumpPhase::Dumping;
        self.dump.last = vec![None; self.dump.vars.len()];
        self.dump.reindex();
        let mut header = format!(
            "$date\n$end\n$version\n\t{}\n$end\n$timescale\n\t{}\n$end\n",
            crate::io::vcd::VCD_WRITER_VERSION,
            super::format::time_unit_name(plan.timing.precision_exponent)
        );
        for line in std::mem::take(&mut self.dump.header) {
            match line {
                HeaderLine::Scope { kind, name } => {
                    header.push_str(&format!("$scope {kind} {name} $end\n"));
                }
                HeaderLine::Upscope => header.push_str("$upscope $end\n"),
                HeaderLine::Var { var, kind, name } => {
                    let width = self.dump.vars[var].width;
                    let range = self.dump.vars[var]
                        .range
                        .map(|(msb, lsb)| format!(" [{msb}:{lsb}]"))
                        .unwrap_or_default();
                    header.push_str(&format!(
                        "$var {kind} {width} {} {name}{range} $end\n",
                        crate::io::vcd::canonical_identifier(var),
                    ));
                }
            }
        }
        header.push_str("$enddefinitions $end\n");
        // Each parameter's value, the newest first, once: a parameter never
        // changes, so no later block lists it. Icarus Verilog writes the
        // block, empty or not, before the first time.
        header.push_str("$comment Show the parameter values. $end\n$dumpall\n");
        for (index, var) in self.dump.vars.iter().enumerate().rev() {
            let Some(value) = var
                .parameter
                .and_then(|parameter| plan.dump_parameters.get(parameter as usize))
            else {
                continue;
            };
            let value = match &value.value {
                DigitalParameterValue::FourState(bits) => Dumped::Bits(spell(bits, var.width > 1)),
                DigitalParameterValue::Real(real) => Dumped::Real(real.to_bits()),
            };
            header.push_str(&change_line(
                &value,
                &crate::io::vcd::canonical_identifier(index),
            ));
        }
        header.push_str("$end\n");
        self.dump.tree = None;
        self.dump.scanned.clear();
        self.dump.named.clear();
        self.dump_write(plan, None, clock, header);
        self.dump.recording = true;
        self.dump_block(plan, None, clock, store, tick, "dumpvars");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn value(bits: &str) -> FourStateValue {
        let bits: Vec<FourStateBit> = bits
            .chars()
            .map(|bit| match bit {
                '0' => FourStateBit::Zero,
                '1' => FourStateBit::One,
                'x' => FourStateBit::Unknown,
                _ => FourStateBit::HighImpedance,
            })
            .collect();
        FourStateValue::from_bits_msb_first(&bits)
    }

    /// Section 18.2.3.8: a reader left-extends with `0` a vector led by `0`
    /// or `1`, and with `x` or `z` one led by those; the writer leaves off
    /// exactly what extension restores.
    #[test]
    fn vectors_drop_the_bits_extension_restores() {
        assert_eq!(spell(&value("0101"), true), "b101");
        assert_eq!(spell(&value("0000"), true), "b0");
        assert_eq!(spell(&value("xxx"), true), "bx");
        assert_eq!(spell(&value("0x1"), true), "b0x1");
        assert_eq!(spell(&value("zz01"), true), "bz01");
        assert_eq!(spell(&value("1100"), true), "b1100");
        assert_eq!(spell(&value("1"), false), "1");
    }

    /// A port net names its instance by length, so an instance path with
    /// dots or colons still reads back whole.
    #[test]
    fn a_port_net_reads_back_as_its_instance_and_port() {
        assert_eq!(port_net_spelling("@port:2:x1:0:in"), Some(("x1", "in")));
        assert_eq!(
            port_net_spelling("@port:7:x1.x:t2:3:data"),
            Some(("x1.x:t2", "data"))
        );
        assert_eq!(port_net_spelling("@real:4"), None);
        assert_eq!(port_net_spelling("x1.o"), None);
    }
}
