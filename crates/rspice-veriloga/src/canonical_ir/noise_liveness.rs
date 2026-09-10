//! Dependencies of raw noise metadata before CFG lowering.
//!
//! Gain computation belongs to the separate small-signal replay. Requiring its
//! assignments here would reject otherwise supported models before CFG dead
//! code elimination can remove them. Keep every reaching definition and its
//! control flow for metadata; the ordinary body still checks numerical faults.

use std::collections::{HashMap, HashSet};

use super::hir::{HirAssignment, HirExprKind, HirModel, HirRegion, HirStatement};
use super::noise::{contains_noise, is_noise_call, push_expression_children};
use super::{ExprId, VariableId};

#[derive(Default)]
pub(super) struct NoiseMetadataLiveness {
    variables: HashSet<VariableId>,
    conditions: HashSet<ExprId>,
}

impl NoiseMetadataLiveness {
    pub(super) fn for_model(hir: &HirModel) -> Self {
        let mut walker = MetadataDependencies {
            hir,
            live: Self::default(),
            variables_by_name: hir
                .variables
                .iter()
                .map(|variable| (variable.name.as_str(), variable.id))
                .collect(),
            visited_values: HashSet::new(),
            visited_noise: HashSet::new(),
            noisy: HashMap::new(),
        };
        loop {
            let previous = walker.live.variables.len();
            walker.regions(&hir.body);
            for index in hir.prologue_statements.iter().rev() {
                if let Some(HirStatement::Assignment(assignment)) =
                    hir.statements.get(*index as usize)
                {
                    walker.assignment(assignment);
                }
            }
            if walker.live.variables.len() == previous {
                break;
            }
        }
        walker.live
    }

    pub(super) fn assignment_is_live(&self, hir: &HirModel, assignment: &HirAssignment) -> bool {
        if assignment.index.is_some() {
            hir.arrays
                .iter()
                .find(|array| array.base == assignment.target)
                .is_some_and(|array| {
                    (0..array.len).any(|offset| {
                        self.variables
                            .contains(&VariableId::from(usize::from(array.base) + offset as usize))
                    })
                })
        } else {
            self.variables.contains(&assignment.target)
        }
    }

    pub(super) fn condition_is_live(&self, condition: ExprId) -> bool {
        self.conditions.contains(&condition)
    }
}

struct MetadataDependencies<'a> {
    hir: &'a HirModel,
    live: NoiseMetadataLiveness,
    variables_by_name: HashMap<&'a str, VariableId>,
    visited_values: HashSet<ExprId>,
    visited_noise: HashSet<ExprId>,
    noisy: HashMap<ExprId, bool>,
}

impl MetadataDependencies<'_> {
    fn regions(&mut self, regions: &[HirRegion]) -> bool {
        let mut needed = false;
        for region in regions.iter().rev() {
            needed |= match region {
                HirRegion::Assignment(assignment) => self.assignment(assignment),
                HirRegion::Contribution(contribution) => self.noise(contribution.expression.id),
                HirRegion::Conditional {
                    condition,
                    then_body,
                    else_body,
                    ..
                } => {
                    let branches = self.regions(then_body) | self.regions(else_body);
                    self.control(condition.id, branches)
                }
                HirRegion::Loop {
                    condition, body, ..
                } => {
                    let body = self.regions(body);
                    self.control(condition.id, body)
                }
                HirRegion::Task(_) | HirRegion::Initialization { .. } => false,
            };
        }
        needed
    }

    fn control(&mut self, condition: ExprId, body: bool) -> bool {
        if self.noise(condition) | body {
            self.value(condition);
            self.live.conditions.insert(condition);
            true
        } else {
            false
        }
    }

    fn assignment(&mut self, assignment: &HirAssignment) -> bool {
        let noisy = self.noise(assignment.expr.id);
        if self.live.assignment_is_live(self.hir, assignment) {
            self.value(assignment.expr.id);
            if let Some(index) = &assignment.index {
                self.value(index.id);
            }
            true
        } else {
            noisy
        }
    }

    /// Visit only noise sites and the expressions controlling their execution.
    fn noise(&mut self, id: ExprId) -> bool {
        let noisy = *self
            .noisy
            .entry(id)
            .or_insert_with(|| contains_noise(self.hir, id));
        if !noisy || !self.visited_noise.insert(id) {
            return noisy;
        }
        let Some(expression) = self.hir.expressions.get(usize::from(id)) else {
            return false;
        };
        match &expression.kind {
            HirExprKind::NoiseSource { .. } => self.value(id),
            HirExprKind::Call { name, .. } | HirExprKind::SystemFunction { name, .. }
                if is_noise_call(name) =>
            {
                self.value(id)
            }
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => {
                self.value(*condition);
                self.noise(*then_expr);
                self.noise(*else_expr);
            }
            kind => {
                let mut children = Vec::new();
                push_expression_children(kind, &mut children);
                for child in children {
                    self.noise(child);
                }
            }
        }
        true
    }

    fn value(&mut self, root: ExprId) {
        let mut stack = vec![root];
        while let Some(id) = stack.pop() {
            if !self.visited_values.insert(id) {
                continue;
            }
            let Some(expression) = self.hir.expressions.get(usize::from(id)) else {
                continue;
            };
            match &expression.kind {
                HirExprKind::Identifier { name } => {
                    if let Some(variable) = self.variables_by_name.get(name.as_str()) {
                        self.live.variables.insert(*variable);
                    }
                }
                HirExprKind::ArrayAccess { array, .. } => {
                    if let Some(array) = self
                        .hir
                        .arrays
                        .iter()
                        .find(|candidate| candidate.name == *array)
                    {
                        self.live.variables.extend((0..array.len).map(|offset| {
                            VariableId::from(usize::from(array.base) + offset as usize)
                        }));
                    }
                }
                _ => {}
            }
            push_expression_children(&expression.kind, &mut stack);
        }
    }
}
