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
//! [`crate::io::vcd::parse_vcd_reader`] like any other.
//!
//! # When things are written
//!
//! `$dumpvars` calls made in one time step select the variables; at the end
//! of that step the header is written and the selection's values are dumped
//! in a `$dumpvars` block (section 18.1.2). From then on, the end of every
//! step writes the variables whose value changed, after a `#time` line. A
//! later `$dumpvars` is ignored with a warning. `$dumpoff` writes every
//! variable as `x` and stops recording, `$dumpon` writes every current value
//! and resumes, and `$dumpall` writes every current value (sections 18.1.3
//! and 18.1.5). Memories are not dumped (section 18.1.2). The `$date` block
//! is left empty, so a dump is a function of the design and its inputs.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::sync::Arc;

use rspice_veriloga::canonical_ir::digital::{
    CanonicalDigitalPlan, DigitalScopeKind, DigitalSystemArgument, DigitalSystemCall,
    DigitalSystemCallKind as Kind,
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
use crate::xspice::verilog::store::DigitalSignalStore;

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

/// One dumped variable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct DumpVar {
    pub signal: u32,
    /// The scopes enclosing it, outermost first, with their VCD scope types.
    pub scope: Vec<(String, String)>,
    pub name: String,
    pub kind: String,
    pub width: u32,
    /// `[msb:lsb]`, for a vector.
    pub range: Option<(i64, i64)>,
    pub real: bool,
}

/// The value last written for a variable; a real by its bits, so that two
/// values compare equal exactly when they print the same.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum Dumped {
    Bits(String),
    Real(u64),
}

/// The dump of one run, which is also its checkpoint image.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    /// Variables changed in this step.
    #[serde(skip)]
    dirty: BTreeSet<usize>,
    pub last: Vec<Option<Dumped>>,
    /// `$dumpoff` stops recording, `$dumpon` resumes it.
    pub recording: bool,
    pub limit: Option<u64>,
    pub written: u64,
    /// The last `#time` written.
    pub time: Option<u64>,
}

impl Default for Dump {
    fn default() -> Self {
        Self {
            name: None,
            path: None,
            phase: DumpPhase::Idle,
            vars: Vec::new(),
            by_signal: Arc::default(),
            dirty: BTreeSet::new(),
            last: Vec::new(),
            recording: true,
            limit: None,
            written: 0,
            time: None,
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
                .map(|(index, var)| (var.signal, index))
                .collect(),
        );
    }

    /// Whether the change hook has anything to do.
    pub(crate) fn watching(&self) -> bool {
        self.phase == DumpPhase::Dumping && self.recording
    }

    /// A signal changed value.
    pub(crate) fn note_change(&mut self, signal: DigitalSignalId) {
        if let Some(index) = self.by_signal.get(&signal.index()) {
            self.dirty.insert(*index);
        }
    }
}

/// A signal's place in the hierarchy: its enclosing scope path and its own
/// name. A flattened plan names a child's signal `u1.x` under the root it was
/// compiled for; a linked plan prefixes every name with its root, which is an
/// instance path that may have dots of its own (`x1.xtb`).
fn hierarchy(plan: &CanonicalDigitalPlan, name: &str) -> (String, String) {
    let roots: Vec<&str> = plan
        .scopes
        .iter()
        .filter(|scope| scope.parent.is_none())
        .map(|scope| scope.path.as_str())
        .collect();
    let rooted = roots.len() > 1
        && roots.iter().any(|root| {
            name.strip_prefix(root)
                .is_some_and(|rest| rest.starts_with('.'))
        });
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

/// The VCD scope type of each scope path (section 18.2.3.4).
fn scope_types(plan: &CanonicalDigitalPlan) -> BTreeMap<String, &'static str> {
    plan.scopes
        .iter()
        .map(|scope| {
            (
                scope.path.to_string(),
                match scope.kind {
                    DigitalScopeKind::Module => "module",
                    DigitalScopeKind::Block => "begin",
                    DigitalScopeKind::Task => "task",
                    DigitalScopeKind::Function => "function",
                },
            )
        })
        .collect()
}

fn var_for(
    plan: &CanonicalDigitalPlan,
    signal: u32,
    types: &BTreeMap<String, &'static str>,
) -> Option<DumpVar> {
    let declared = plan.signals.get(signal as usize)?;
    let real = declared.kind.is_real();
    if !real && declared.width == 0 {
        // A named event has no value to dump.
        return None;
    }
    let (scope, name) = hierarchy(plan, &declared.name);
    let mut path = String::new();
    let mut levels = Vec::new();
    for segment in scope.split('.').filter(|segment| !segment.is_empty()) {
        if !path.is_empty() {
            path.push('.');
        }
        path.push_str(segment);
        let kind = types.get(&path).copied().unwrap_or("module");
        levels.push((segment.to_owned(), kind.to_owned()));
    }
    let kind = if real {
        "real"
    } else if declared.integer {
        "integer"
    } else if declared.procedurally_assignable {
        "reg"
    } else {
        "wire"
    };
    Some(DumpVar {
        signal,
        scope: levels,
        name,
        kind: kind.to_owned(),
        width: if real { 64 } else { declared.width },
        range: declared.bounds.filter(|_| !real),
        real,
    })
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
    if var.real {
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
        Dumped::Real(bits) => format!("r{:?} {identifier}\n", f64::from_bits(*bits)),
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
                    self.select_dump_vars(plan, site, args);
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

    /// Add `$dumpvars(levels, scope_or_variable, ...)`'s selection.
    fn select_dump_vars(
        &mut self,
        plan: &CanonicalDigitalPlan,
        site: &DigitalSystemCall,
        args: &[DigitalScalar],
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
        let memories: BTreeSet<u32> = plan
            .arrays
            .iter()
            .filter_map(|array| array.storage.cell_range())
            .flatten()
            .collect();
        let mut chosen: Vec<u32> = Vec::new();
        let roots: Vec<String> = plan
            .scopes
            .iter()
            .filter(|scope| scope.parent.is_none())
            .map(|scope| scope.path.to_string())
            .collect();
        let mut scopes: Vec<String> = Vec::new();
        for argument in site.arguments.iter().skip(1) {
            match argument {
                DigitalSystemArgument::Signal(signal) => chosen.push(signal.index()),
                DigitalSystemArgument::Scope(scope) => {
                    if let Some(scope) = plan.scopes.get(scope.index()) {
                        scopes.push(scope.path.to_string());
                    }
                }
                _ => {}
            }
        }
        if site.arguments.len() <= 1 {
            scopes = roots;
        }
        for (index, signal) in plan.signals.iter().enumerate() {
            let index = index as u32;
            if signal.local.is_some() || memories.contains(&index) || scopes.is_empty() {
                continue;
            }
            let (scope, _) = hierarchy(plan, &signal.name);
            let inside = scopes.iter().any(|root| {
                let depth = if scope == *root {
                    1
                } else if let Some(rest) = scope.strip_prefix(root.as_str())
                    && rest.starts_with('.')
                {
                    rest.split('.').count()
                } else {
                    return false;
                };
                levels == 0 || depth <= levels
            });
            if inside {
                chosen.push(index);
            }
        }
        let types = scope_types(plan);
        let mut known: BTreeSet<u32> = self.dump.vars.iter().map(|var| var.signal).collect();
        for signal in chosen {
            if known.insert(signal)
                && let Some(var) = var_for(plan, signal, &types)
            {
                self.dump.vars.push(var);
            }
        }
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
        let dirty = std::mem::take(&mut self.dump.dirty);
        if !self.dump.watching() {
            return;
        }
        let mut text = String::new();
        for index in dirty {
            let var = &self.dump.vars[index];
            let Some(value) = current(store, var) else {
                continue;
            };
            if self.dump.last[index].as_ref() == Some(&value) {
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
        for (index, var) in self.dump.vars.iter().enumerate() {
            let identifier = crate::io::vcd::canonical_identifier(index);
            if block == "dumpoff" {
                // Section 18.1.3: every variable is dumped as `x`; a real has
                // no unknown, and keeps its value.
                if !var.real {
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
        self.dump.dirty.clear();
        self.dump_write(plan, site, clock, text);
    }

    /// The end of a time step: write the header once the selection's step
    /// ends, and every change after it.
    pub(crate) fn end_of_step(
        &mut self,
        plan: &CanonicalDigitalPlan,
        store: &DigitalSignalStore,
        clock: Option<DigitalClock>,
    ) {
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
        let mut open: Vec<(String, String)> = Vec::new();
        for (index, var) in self.dump.vars.iter().enumerate() {
            let shared = open
                .iter()
                .zip(&var.scope)
                .take_while(|(open, wanted)| open == wanted)
                .count();
            while open.len() > shared {
                open.pop();
                header.push_str("$upscope $end\n");
            }
            for (name, kind) in var.scope.iter().skip(shared) {
                header.push_str(&format!("$scope {kind} {name} $end\n"));
                open.push((name.clone(), kind.clone()));
            }
            let range = var
                .range
                .map(|(msb, lsb)| format!(" [{msb}:{lsb}]"))
                .unwrap_or_default();
            header.push_str(&format!(
                "$var {} {} {} {}{range} $end\n",
                var.kind,
                var.width,
                crate::io::vcd::canonical_identifier(index),
                var.name
            ));
        }
        for _ in open {
            header.push_str("$upscope $end\n");
        }
        header.push_str("$enddefinitions $end\n");
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
}
