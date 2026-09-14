use super::*;
use std::collections::{BTreeMap, BTreeSet};

/// Prove a one-to-one correspondence with retained K cards. Only the runtime
/// overlays are stamped; the authored records never add a second mutual term.
pub(super) fn validate(
    circuit: &crate::CircuitData,
    options: &EventOptions,
    abort: &dyn AbortSignal,
) -> Result<()> {
    let l = &circuit.inductors;
    let pairs = &circuit.coupled_inductor_pairs;
    let entries = l
        .len()
        .saturating_add(pairs.len())
        .saturating_add(circuit.couplings.len());
    ResourceLimitError::ensure(
        ResourceKind::ResultValues,
        circuit
            .matrix_size()
            .saturating_mul(64)
            .saturating_add(entries.saturating_mul(64)),
        options.limits.max_result_values,
    )?;
    let mut names = BTreeMap::new();
    let mut branches = BTreeMap::new();
    for (index, name) in l.names.iter().enumerate() {
        check_abort(abort)?;
        if names.insert(name.to_ascii_lowercase(), index).is_some()
            || branches.insert(l.branch_indices[index], index).is_some()
        {
            return Err(error(format!(
                "inductor '{name}' has ambiguous event identity"
            )));
        }
    }
    let mut overlays = BTreeMap::new();
    for (index, pair) in pairs.iter().enumerate() {
        check_abort(abort)?;
        let d = &pair.device;
        let a = pair.branch1_ordinal;
        let b = pair.branch2_ordinal;
        let key = (d.name.to_ascii_lowercase(), a.min(b), a.max(b));
        if a == b || overlays.insert(key, index).is_some() {
            return Err(error(format!(
                "mutual coupling '{}' has duplicate or aliased event overlays",
                d.name
            )));
        }
        for (ordinal, p, n, value) in [
            (a, d.node1_pos, d.node1_neg, d.l1),
            (b, d.node2_pos, d.node2_neg, d.l2),
        ] {
            let i = *branches.get(&ordinal).ok_or_else(|| {
                error(format!(
                    "mutual coupling '{}' references a missing flux branch",
                    d.name
                ))
            })?;
            if value <= 0.0 || value != l.inductances[i] || p != l.node_pos[i] || n != l.node_neg[i]
            {
                return Err(error(format!(
                    "mutual coupling '{}' disagrees with its canonical inductor",
                    d.name
                )));
            }
        }
    }
    let mut coupling_names = BTreeSet::new();
    for coupling in &circuit.couplings {
        check_abort(abort)?;
        let name = coupling.name.to_ascii_lowercase();
        if name.trim().is_empty()
            || !coupling_names.insert(name.clone())
            || coupling.inductor_names.len() < 2
            || !coupling.coefficient.is_finite()
            || coupling.coefficient.abs() > 1.0
        {
            return Err(error(format!(
                "mutual coupling '{}' has invalid event metadata",
                coupling.name
            )));
        }
        let mut indices = BTreeSet::new();
        for winding in &coupling.inductor_names {
            check_abort(abort)?;
            let i = *names.get(&winding.to_ascii_lowercase()).ok_or_else(|| {
                error(format!(
                    "mutual coupling '{}' references missing inductor '{winding}'",
                    coupling.name
                ))
            })?;
            if !indices.insert(i) {
                return Err(error(format!(
                    "mutual coupling '{}' repeats inductor '{winding}'",
                    coupling.name
                )));
            }
        }
        for &i in &indices {
            for &j in indices.range((std::ops::Bound::Excluded(i), std::ops::Bound::Unbounded)) {
                check_abort(abort)?;
                let a = l.branch_indices[i];
                let b = l.branch_indices[j];
                let key = (name.clone(), a.min(b), a.max(b));
                let index = overlays.remove(&key).ok_or_else(|| {
                    error(format!(
                        "mutual coupling '{}' has no unique event overlay for '{}' and '{}'",
                        coupling.name, l.names[i], l.names[j]
                    ))
                })?;
                let d = &pairs[index].device;
                let expected = coupling.mutual_inductance(l.inductances[i], l.inductances[j]);
                // Both are built by the same scaled geometric-mean function.
                // No absolute floor may hide a corrupted tiny inductance.
                if d.k != coupling.coefficient || !d.m.is_finite() || d.m != expected {
                    return Err(error(format!(
                        "mutual coupling '{}' has inconsistent event coefficients",
                        coupling.name
                    )));
                }
            }
        }
    }
    if let Some(((name, _, _), _)) = overlays.first_key_value() {
        return Err(error(format!(
            "mutual coupling '{name}' has no authored event owner"
        )));
    }
    Ok(())
}
