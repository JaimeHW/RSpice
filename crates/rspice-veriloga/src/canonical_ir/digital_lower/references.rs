//! Bind hierarchical references to discrete-domain signals.
//!
//! IEEE 1364-2005 sections 12.5 and 12.6 let a scope name a net or variable of
//! another scope, below it or above it. Source binding gave the referring scope
//! a borrowed copy of the target's declaration under the reference's symbol;
//! here the symbol is bound to the target's own signal, so a read observes the
//! target, a write drives it, and the copy never takes storage.
//!
//! A reference into a task, function or named block names an item that has
//! no signal until a reference needs one: here it gets its storage, shaped as
//! the borrowed declaration, and the scope's routine or process lowering
//! keeps the item there.
use super::{AnalyzedDigital, AnalyzedDigitalSignal, DigitalLoweringDiagnostic, DigitalSignalId};
use crate::semantic::DigitalLocalReference;
use smol_str::SmolStr;
use std::borrow::Cow;
use std::collections::HashMap;

/// A named block's variable a reference gave storage to.
pub(super) struct BlockTarget {
    /// The generate scope of the process that declares it.
    pub generate_scope: Option<SmolStr>,
    pub declaration: crate::source::Span,
    /// The storage's name, which the scope's signal index resolves.
    pub name: SmolStr,
    pub id: DigitalSignalId,
}

/// The storage references gave to the items of tasks, functions and named
/// blocks, by scope: the root at 0 and instance frame `i` at `i + 1`.
#[derive(Default)]
pub(super) struct LocalTargets {
    routines: HashMap<usize, HashMap<(SmolStr, SmolStr), DigitalSignalId>>,
    blocks: HashMap<usize, Vec<BlockTarget>>,
}

static NO_ROUTINE_ITEMS: std::sync::LazyLock<HashMap<(SmolStr, SmolStr), DigitalSignalId>> =
    std::sync::LazyLock::new(HashMap::new);

impl LocalTargets {
    /// Scope `scope`'s routine items, by `(routine, item)`.
    pub fn routine_items(&self, scope: usize) -> &HashMap<(SmolStr, SmolStr), DigitalSignalId> {
        self.routines.get(&scope).unwrap_or(&NO_ROUTINE_ITEMS)
    }

    /// Scope `scope`'s named-block storage, for its signal index.
    pub fn block_names(&self, scope: usize) -> impl Iterator<Item = (&str, DigitalSignalId)> {
        self.blocks
            .get(&scope)
            .into_iter()
            .flatten()
            .map(|target| (target.name.as_str(), target.id))
    }

    /// Scope `scope`'s processes, each told which of its named-block
    /// variables live in a reference's storage: the same hook keeps a local
    /// in a shared signal as an analog input does.
    pub fn processes<'a>(
        &self,
        scope: usize,
        processes: &'a [crate::semantic::AnalyzedDigitalProcess],
    ) -> Cow<'a, [crate::semantic::AnalyzedDigitalProcess]> {
        let Some(targets) = self.blocks.get(&scope) else {
            return Cow::Borrowed(processes);
        };
        let mut processes = processes.to_vec();
        for process in &mut processes {
            for target in targets
                .iter()
                .filter(|target| target.generate_scope == process.generate_scope)
            {
                process
                    .analog_local_inputs
                    .insert(target.declaration, target.name.clone());
            }
        }
        Cow::Owned(processes)
    }
}

/// Whether `name` is a symbol a scope borrowed for a hierarchical reference.
pub(super) fn is_reference(
    references: &std::collections::BTreeMap<
        smol_str::SmolStr,
        crate::semantic::DigitalSignalReference,
    >,
    name: &str,
) -> bool {
    references.contains_key(name)
}

/// Point every borrowed symbol at its target's signal id.
///
/// `root_ids` and `frame_ids` are the positional ids of the root's and each
/// instance's signals, with a borrowed symbol's entry still unset.
pub(super) fn bind(
    digital: &AnalyzedDigital,
    root_ids: &mut [DigitalSignalId],
    frame_ids: &mut [Vec<DigitalSignalId>],
    signals: &mut Vec<super::DigitalSignal>,
    arrays: &mut Vec<super::super::digital::DigitalArray>,
) -> Result<LocalTargets, Vec<DigitalLoweringDiagnostic>> {
    let mut locals = LocalTargets::default();
    if digital.signal_references.is_empty()
        && digital
            .instances
            .iter()
            .all(|instance| instance.signal_references.is_empty())
    {
        return Ok(locals);
    }
    let scope_index: HashMap<&str, usize> = std::iter::once(("", 0))
        .chain(
            digital
                .instances
                .iter()
                .enumerate()
                .map(|(index, instance)| (instance.path.as_str(), index + 1)),
        )
        .collect();
    let mut created: HashMap<(usize, DigitalLocalReference), DigitalSignalId> = HashMap::new();
    // Each scope's own signals by local name, with their declarations.
    let mut scopes: HashMap<&str, HashMap<&str, (DigitalSignalId, &AnalyzedDigitalSignal)>> =
        HashMap::new();
    scopes.insert(
        "",
        digital
            .signals
            .iter()
            .zip(root_ids.iter())
            .filter(|(signal, _)| !is_reference(&digital.signal_references, &signal.name))
            .map(|(signal, id)| (signal.name.as_str(), (*id, signal)))
            .collect(),
    );
    for (instance, ids) in digital.instances.iter().zip(frame_ids.iter()) {
        scopes.insert(
            instance.path.as_str(),
            instance
                .signals
                .iter()
                .zip(ids)
                .filter(|(signal, _)| {
                    !is_reference(&instance.signal_references, &signal.declared.name)
                })
                .map(|(signal, id)| (signal.declared.name.as_str(), (*id, &signal.declared)))
                .collect(),
        );
    }
    let mut diagnostics = Vec::new();
    let mut resolve = |borrowed: &AnalyzedDigitalSignal,
                       reference: &crate::semantic::DigitalSignalReference|
     -> Option<DigitalSignalId> {
        if let Some(local) = &reference.local {
            let Some(&scope) = scope_index.get(reference.path.as_str()) else {
                diagnostics.push(DigitalLoweringDiagnostic::invariant(
                    format!(
                        "hierarchical reference `{}` names a scope this design does not have",
                        reference.written
                    ),
                    reference.span.into(),
                ));
                return None;
            };
            if let Some(id) = created.get(&(scope, local.clone())) {
                return Some(*id);
            }
            // Named as the routine lowering names a static routine's storage:
            // `u1.ok.error`.
            let mut name = if reference.path.is_empty() {
                reference.name.to_string()
            } else {
                format!("{}.{}", reference.path, reference.name)
            };
            while signals.iter().any(|signal| signal.name == name) {
                name.insert(0, '$');
            }
            let name = SmolStr::from(name);
            let id = super::append_signal(borrowed, name.clone(), signals, arrays);
            created.insert((scope, local.clone()), id);
            match local {
                DigitalLocalReference::Routine { routine, item } => {
                    locals
                        .routines
                        .entry(scope)
                        .or_default()
                        .insert((routine.clone(), item.clone()), id);
                }
                DigitalLocalReference::Block {
                    generate_scope,
                    declaration,
                } => locals.blocks.entry(scope).or_default().push(BlockTarget {
                    generate_scope: generate_scope.clone(),
                    declaration: *declaration,
                    name,
                    id,
                }),
            }
            return Some(id);
        }
        let Some((id, target)) = scopes
            .get(reference.path.as_str())
            .and_then(|scope| scope.get(reference.name.as_str()))
        else {
            diagnostics.push(DigitalLoweringDiagnostic::refusal(
                format!(
                    "hierarchical reference `{}` names no discrete-domain signal of this \
                     design; a continuous-domain or analog-owned target has no digital view",
                    reference.written
                ),
                reference.span.into(),
            ));
            return None;
        };
        let same_shape = borrowed.width == target.width
            && borrowed.signedness == target.signedness
            && borrowed.unpacked == target.unpacked
            && borrowed.dimensions == target.dimensions
            && borrowed.class == target.class;
        if !same_shape {
            diagnostics.push(DigitalLoweringDiagnostic::invariant(
                format!(
                    "hierarchical reference `{}` was declared as {} [{}] but its target `{}` \
                     is {} [{}]",
                    reference.written,
                    borrowed.class.keyword(),
                    borrowed.width,
                    reference.name,
                    target.class.keyword(),
                    target.width,
                ),
                reference.span.into(),
            ));
            return None;
        }
        Some(*id)
    };
    let mut root_bound = Vec::new();
    for (index, signal) in digital.signals.iter().enumerate() {
        if let Some(reference) = digital.signal_references.get(&signal.name)
            && let Some(id) = resolve(signal, reference)
        {
            root_bound.push((index, id));
        }
    }
    let mut frame_bound = Vec::new();
    for (frame, instance) in digital.instances.iter().enumerate() {
        for (index, signal) in instance.signals.iter().enumerate() {
            if let Some(reference) = instance.signal_references.get(&signal.declared.name)
                && let Some(id) = resolve(&signal.declared, reference)
            {
                frame_bound.push((frame, index, id));
            }
        }
    }
    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }
    for (index, id) in root_bound {
        root_ids[index] = id;
    }
    for (frame, index, id) in frame_bound {
        frame_ids[frame][index] = id;
    }
    Ok(locals)
}
