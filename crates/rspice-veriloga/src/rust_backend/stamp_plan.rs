//! Which matrix entries a device actually writes.
//!
//! A stamp is a residual and a row of the Jacobian, written into the matrix at
//! a branch's two nodes. The interesting part is what *not* to write: in the
//! output this replaces, 202 of 931 stamp arguments are literal zeros —
//! `multiplicity * 0.0` — and even the two-node CMC resistor spends two of its
//! three stamper calls writing nothing.
//!
//! Zeros arrive here two ways and both are dropped.
//!
//! **Structurally**, when a residual cannot reach an unknown at all. This is
//! the one that does the work: the derivative pass already knows it — a packed
//! derivative carries only its own live lanes — so the entry never exists
//! rather than existing and being zero.
//!
//! **Arithmetically**, when an entry survives differentiation and the
//! simplifier folds it to the literal constant zero. This is rarer than it
//! sounds, and the reason is worth stating: `0.0 * V(a,b)` does *not* fold
//! here. `x * 0` is false for NaN, this backend deliberately does not apply
//! that identity, and an entry written that way stays an expression and gets
//! written. What is dropped is only an entry that *is* the constant — a fact
//! about the graph rather than a claim about the arithmetic, so not writing it
//! is exactly as correct as writing it and strictly cheaper.

use crate::canonical_ir::cfg::{CfgFunction, CfgValueKind};
use crate::canonical_ir::mir::{MirEquationKind, MirModel};
use crate::canonical_ir::{NodeId, ValueId};

/// One stamper call: a branch, its residual, and the Jacobian entries that are
/// not known to be zero.
#[derive(Debug, Clone, PartialEq)]
pub struct StampRow {
    /// `None` is ground, which the stamper drops.
    pub pos: Option<NodeId>,
    pub neg: Option<NodeId>,
    /// Whether this equation contributes a current or a potential.
    pub kind: MirEquationKind,
    pub residual: ValueId,
    /// `(unknown, entry)`, ordered by unknown. Unknowns are numbered as the
    /// derivative seeds were: node potentials first, then branch unknowns.
    pub derivatives: Vec<(usize, ValueId)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StampPlan {
    pub rows: Vec<StampRow>,
    /// Entries the derivative pass never produced, because the residual cannot
    /// reach that unknown.
    pub structurally_absent: usize,
    /// Entries that were produced and then folded to a constant zero.
    pub folded_to_zero: usize,
}

impl StampPlan {
    /// Build the plan from each equation's residual and its Jacobian row.
    ///
    /// `rows` are the derivative rows as [`AdFunction::derivative_row`] returns
    /// them — one per equation, parallel to `mir.equations`, each as wide as the
    /// seed list with `None` where the residual cannot reach that unknown.
    pub fn new(mir: &MirModel, residuals: &[ValueId], rows: &[Vec<Option<ValueId>>]) -> Self {
        let mut plan = Self {
            rows: Vec::with_capacity(mir.equations.len()),
            structurally_absent: 0,
            folded_to_zero: 0,
        };
        for (index, equation) in mir.equations.iter().enumerate() {
            let residual = residuals[usize::from(equation.contribution)];
            let row = rows.get(index).cloned().unwrap_or_default();
            plan.structurally_absent += row.iter().filter(|entry| entry.is_none()).count();
            plan.rows.push(StampRow {
                pos: equation.branch.pos_node,
                neg: equation.branch.neg_node,
                kind: equation.kind,
                residual,
                derivatives: row
                    .into_iter()
                    .enumerate()
                    .filter_map(|(unknown, entry)| Some((unknown, entry?)))
                    .collect(),
            });
        }
        plan
    }

    /// Every value the plan names, in a stable order.
    ///
    /// This is what to hand the simplifier: it keeps exactly the values a stamp
    /// reads alive and lets it delete the rest.
    pub fn wanted(&self) -> Vec<ValueId> {
        let mut wanted = Vec::new();
        for row in &self.rows {
            wanted.push(row.residual);
            wanted.extend(row.derivatives.iter().map(|(_, value)| *value));
        }
        wanted
    }

    /// Re-point the plan at the values the simplifier returned.
    ///
    /// Positional, because `optimize_cfg` returns the requested values in the
    /// order they were requested — the same order [`Self::wanted`] produced.
    pub fn remap(&mut self, optimized: &[ValueId]) {
        let mut next = optimized.iter().copied();
        for row in &mut self.rows {
            if let Some(value) = next.next() {
                row.residual = value;
            }
            for (_, value) in &mut row.derivatives {
                if let Some(remapped) = next.next() {
                    *value = remapped;
                }
            }
        }
    }

    /// Drop the entries that simplified to a literal zero.
    ///
    /// Only after [`Self::remap`], and only against the simplified function:
    /// before it, an entry that will fold is still an arbitrary expression.
    pub fn drop_zeros(&mut self, function: &CfgFunction) {
        let is_zero = |value: ValueId| {
            matches!(
                function.value(value).kind,
                CfgValueKind::RealConstant(constant) if constant == 0.0
            )
        };
        for row in &mut self.rows {
            let before = row.derivatives.len();
            row.derivatives.retain(|(_, value)| !is_zero(*value));
            self.folded_to_zero += before - row.derivatives.len();
        }
    }

    /// Jacobian entries the plan will write.
    pub fn entries(&self) -> usize {
        self.rows.iter().map(|row| row.derivatives.len()).sum()
    }
}
