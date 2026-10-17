//! Bind hierarchical references to discrete-domain signals.
//!
//! IEEE 1364-2005 sections 12.5 and 12.6 let a scope name a net or variable of
//! another scope, below it or above it. Source binding gave the referring scope
//! a borrowed copy of the target's declaration under the reference's symbol;
//! here the symbol is bound to the target's own signal, so a read observes the
//! target, a write drives it, and the copy never takes storage.
use super::{AnalyzedDigital, AnalyzedDigitalSignal, DigitalLoweringDiagnostic, DigitalSignalId};
use std::collections::HashMap;

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
) -> Result<(), Vec<DigitalLoweringDiagnostic>> {
    if digital.signal_references.is_empty()
        && digital
            .instances
            .iter()
            .all(|instance| instance.signal_references.is_empty())
    {
        return Ok(());
    }
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
    Ok(())
}
