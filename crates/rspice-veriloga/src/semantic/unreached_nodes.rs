//! Tie to ground an internal node that no contribution reaches.
//!
//! A module may declare an internal node that only some of its configurations
//! connect. HiSIM-SOI's body-network nodes `db` and `sb` take contributions
//! only under `COBCNODE == 1`, and EPFL-HEMT declares four nodes it never
//! uses. In every other configuration nothing stamps the node's row or column:
//! the node floats, and the system is singular however a solver approaches it.
//!
//! When the instance decides every condition around the node's contributions
//! before it solves (literals, parameters, `$param_given`, `$port_connected`),
//! the node gets `I(node) <+ V(node)` under the negation of those conditions.
//! An instance that leaves the node unconnected then ties it to ground through
//! one siemens, and an instance that connects it is unchanged. Where another
//! configuration already holds the node to ground with a potential source, as
//! ASM-HEMT's trap nodes do, the tie is `V(node) <+ 0` on that same branch
//! instead, so it adds no unknown. The tie is added
//! only when every read of the node sits in a block that also connects it, so
//! no other equation sees the tied value. A node under a condition the
//! instance cannot decide, or one a loop or an initial block touches, is left
//! as written.

use super::*;
use std::borrow::Cow;
use std::collections::HashSet;

/// When the node's row receives a contribution.
enum Reach {
    Never,
    Always,
    /// Exactly when this condition, decided before the solve, holds.
    When(Expression),
    /// Under a condition the instance cannot decide before it solves.
    Unknown,
}

struct Node<'a> {
    module: &'a AnalyzedModule,
    name: &'a str,
    parameters: HashSet<&'a str>,
}

impl Node<'_> {
    fn ends(&self, branch: &str) -> Option<(&str, &str)> {
        self.module
            .branches
            .iter()
            .find(|declared| declared.name == branch)
            .map(|declared| (declared.pos_node.as_str(), declared.neg_node.as_str()))
    }

    fn contribution_ends<'c>(
        &'c self,
        contribution: &'c AnalyzedContribution,
    ) -> (&'c str, &'c str) {
        let (pos, neg) = contribution
            .declared_branch
            .as_deref()
            .and_then(|branch| self.ends(branch))
            .unwrap_or_else(|| {
                contribution
                    .branch
                    .split_once(',')
                    .unwrap_or((contribution.branch.as_str(), ""))
            });
        (pos.trim(), neg.trim())
    }

    fn contributes(&self, contribution: &AnalyzedContribution) -> bool {
        let (pos, neg) = self.contribution_ends(contribution);
        pos == self.name || neg == self.name
    }

    fn is_ground(&self, node: &str) -> bool {
        node.is_empty()
            || is_global_ground_name(node)
            || self.module.ground_nodes.iter().any(|ground| ground == node)
    }

    /// The branch of a potential source that already holds the node to ground
    /// in some configuration, as its contribution names it.
    fn grounding_branch(&self) -> Option<(SmolStr, Option<SmolStr>)> {
        self.module
            .contributions
            .iter()
            .find(|contribution| {
                let (pos, neg) = self.contribution_ends(contribution);
                !contribution.is_current
                    && !contribution.indirect
                    && ((pos == self.name && self.is_ground(neg))
                        || (neg == self.name && self.is_ground(pos)))
            })
            .map(|contribution| {
                (
                    contribution.branch.clone(),
                    contribution.declared_branch.clone(),
                )
            })
    }

    fn reads(&self, expression: &Expression) -> bool {
        let mut reads = false;
        visit_expression(expression, &mut |expression| {
            if let Expression::BranchAccess(access) = expression {
                reads |= match access {
                    BranchAccess::Nodes { pos, neg, .. } => {
                        pos == self.name || neg.as_deref() == Some(self.name)
                    }
                    BranchAccess::Branch { name, .. } => self
                        .ends(name)
                        .is_none_or(|(pos, neg)| pos == self.name || neg == self.name),
                };
            }
        });
        reads
    }

    /// Whether the instance knows `condition` before it solves.
    fn decided(&self, condition: &Expression) -> bool {
        let mut decided = true;
        visit_expression(condition, &mut |expression| {
            decided &= match expression {
                Expression::Number(_) | Expression::Unary(_) | Expression::Conditional(_) => true,
                Expression::Binary(binary) => {
                    !matches!(binary.op, BinaryOp::CheckedValue | BinaryOp::DiscreteValue)
                }
                Expression::Identifier(identifier) => {
                    self.parameters.contains(identifier.name.as_str())
                }
                Expression::SystemFunction(function) => {
                    matches!(function.name.as_str(), "$param_given" | "$port_connected")
                }
                _ => false,
            };
        });
        decided
    }

    fn reach(&self, regions: &[AnalyzedRegion]) -> Reach {
        regions.iter().fold(Reach::Never, |reach, region| {
            or(reach, self.region_reach(region))
        })
    }

    fn region_reach(&self, region: &AnalyzedRegion) -> Reach {
        match region {
            AnalyzedRegion::Contribution(contribution) if self.contributes(contribution) => {
                Reach::Always
            }
            AnalyzedRegion::Conditional {
                condition,
                then_body,
                else_body,
                ..
            } => match (self.reach(then_body), self.reach(else_body)) {
                (Reach::Always, Reach::Always) => Reach::Always,
                (Reach::Never, Reach::Never) => Reach::Never,
                (Reach::Unknown, _) | (_, Reach::Unknown) => Reach::Unknown,
                _ if !self.decided(condition) => Reach::Unknown,
                (then_reach, else_reach) => or(
                    and(condition.clone(), then_reach),
                    and(SemanticAnalyzer::not_expr(condition.clone()), else_reach),
                ),
            },
            AnalyzedRegion::Loop { body, .. } | AnalyzedRegion::Initialization { body, .. } => {
                match self.reach(body) {
                    Reach::Never => Reach::Never,
                    _ => Reach::Unknown,
                }
            }
            _ => Reach::Never,
        }
    }

    /// Whether every execution of `regions` connects the node.
    fn always_reached(&self, regions: &[AnalyzedRegion]) -> bool {
        regions.iter().any(|region| match region {
            AnalyzedRegion::Contribution(contribution) => self.contributes(contribution),
            AnalyzedRegion::Conditional {
                then_body,
                else_body,
                ..
            } => self.always_reached(then_body) && self.always_reached(else_body),
            _ => false,
        })
    }

    /// Whether every read of the node sits in a block that connects it.
    fn reads_covered(&self, regions: &[AnalyzedRegion], outer: bool) -> bool {
        let covered = outer || self.always_reached(regions);
        let unread = |expression: &Expression| covered || !self.reads(expression);
        regions.iter().all(|region| match region {
            AnalyzedRegion::Assignment(assignment) => {
                unread(&assignment.expression) && assignment.index.as_ref().is_none_or(unread)
            }
            AnalyzedRegion::Contribution(contribution) => {
                unread(&contribution.expression)
                    && contribution.equation_abstol.as_ref().is_none_or(unread)
            }
            AnalyzedRegion::Conditional {
                condition,
                then_body,
                else_body,
                ..
            } => {
                unread(condition)
                    && self.reads_covered(then_body, covered)
                    && self.reads_covered(else_body, covered)
            }
            AnalyzedRegion::Loop {
                condition, body, ..
            } => unread(condition) && self.reads_covered(body, covered),
            AnalyzedRegion::Task(task) => task.expressions().all(unread),
            AnalyzedRegion::Initialization { body, .. } => self.reads_covered(body, covered),
        })
    }
}

fn and(condition: Expression, reach: Reach) -> Reach {
    match reach {
        Reach::Never => Reach::Never,
        Reach::Always => Reach::When(condition),
        Reach::When(inner) => Reach::When(SemanticAnalyzer::binary_expr(
            BinaryOp::And,
            condition,
            inner,
        )),
        Reach::Unknown => Reach::Unknown,
    }
}

fn or(left: Reach, right: Reach) -> Reach {
    match (left, right) {
        (Reach::Always, _) | (_, Reach::Always) => Reach::Always,
        (Reach::Unknown, _) | (_, Reach::Unknown) => Reach::Unknown,
        (Reach::Never, reach) | (reach, Reach::Never) => reach,
        (Reach::When(left), Reach::When(right)) => {
            Reach::When(SemanticAnalyzer::binary_expr(BinaryOp::Or, left, right))
        }
    }
}

/// Add `I(node) <+ V(node)` for every internal node no contribution reaches,
/// under the conditions in which none does.
pub(crate) fn lower(mut module: Cow<'_, AnalyzedModule>) -> CompileResult<Cow<'_, AnalyzedModule>> {
    // A flattened hierarchy or an aliased node can be connected from outside
    // the body this pass reads.
    if module.hierarchical_connections
        || !module.node_aliases.is_empty()
        || !module.hoisted_ports.is_empty()
        || !module.external_leaves.is_empty()
    {
        return Ok(module);
    }
    let Some(span) = module
        .contributions
        .first()
        .map(|contribution| contribution.span)
    else {
        return Ok(module);
    };
    let mut ties = Vec::new();
    {
        let parameters: HashSet<&str> = module
            .parameters
            .iter()
            .map(|parameter| parameter.name.as_str())
            .chain(
                module
                    .param_aliases
                    .iter()
                    .map(|alias| alias.alias.as_str()),
            )
            .collect();
        for internal in &module.internal_nodes {
            if internal.is_state
                || internal.discipline != "electrical"
                || module.ground_nodes.contains(&internal.name)
                || module.ports.iter().any(|port| port.name == internal.name)
            {
                continue;
            }
            let node = Node {
                module: &module,
                name: internal.name.as_str(),
                parameters: parameters.clone(),
            };
            let condition = match node.reach(&module.body) {
                Reach::Never => None,
                Reach::When(reached) => Some(SemanticAnalyzer::not_expr(reached)),
                Reach::Always | Reach::Unknown => continue,
            };
            if node.reads_covered(&module.body, false) {
                ties.push((internal.name.clone(), condition, node.grounding_branch()));
            }
        }
    }
    if ties.is_empty() {
        return Ok(module);
    }
    let target = module.to_mut();
    for (node, condition, grounding) in ties {
        let site = AnalogSiteId(target.analog_site_count);
        target.analog_site_count = target.analog_site_count.checked_add(1).ok_or_else(|| {
            CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UnsupportedFeature("too many analog equation sites".into()),
                span,
            ))
        })?;
        // A node that another configuration holds to ground with a potential
        // source is held at zero on that same branch, which shares that
        // source's unknown; any other node takes one siemens.
        let (branch, declared_branch, is_current, value) = match grounding {
            Some((branch, declared_branch)) => (
                branch,
                declared_branch,
                false,
                SemanticAnalyzer::number_expr(0.0, span),
            ),
            None => (
                node.clone(),
                None,
                true,
                Expression::BranchAccess(BranchAccess::Nodes {
                    access: "V".into(),
                    kind: Some(AccessKind::Potential),
                    pos: node.clone(),
                    pos_indices: Vec::new(),
                    neg_indices: Vec::new(),
                    neg: None,
                    span,
                }),
            ),
        };
        let contribution =
            |expression: Expression, expression_guard: AnalogSiteGuard| AnalyzedContribution {
                branch: branch.clone(),
                declared_branch: declared_branch.clone(),
                is_current,
                indirect: false,
                equation_abstol: None,
                expression,
                site,
                expression_guard,
                expr_type: ValueType::Real,
                span,
            };
        match condition {
            None => {
                let tie = contribution(value, AnalogSiteGuard::None);
                target.body.push(AnalyzedRegion::Contribution(tie.clone()));
                target.contributions.push(tie);
            }
            Some(condition) => {
                target.body.push(AnalyzedRegion::Conditional {
                    condition: condition.clone(),
                    condition_site: None,
                    then_body: vec![AnalyzedRegion::Contribution(contribution(
                        value.clone(),
                        AnalogSiteGuard::Select,
                    ))],
                    else_body: Vec::new(),
                    span,
                });
                target.contributions.push(contribution(
                    Expression::Conditional(ConditionalExpr {
                        condition: Box::new(condition),
                        then_expr: Box::new(value),
                        else_expr: Box::new(SemanticAnalyzer::number_expr(0.0, span)),
                        span,
                    }),
                    AnalogSiteGuard::Select,
                ));
            }
        }
    }
    Ok(module)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The ties the pass adds to a module with ports `p, n`, as
    /// `(branch, is_current, conditional)`.
    fn ties(declarations: &str, body: &str) -> Vec<(String, bool, bool)> {
        let source = format!(
            "module m(p,n); inout p,n; electrical p,n; {declarations} analog begin {body} end endmodule"
        );
        let tokens = crate::lexer::Lexer::new(&source, crate::source::SourceId::new(0))
            .collect_tokens()
            .unwrap();
        let file = crate::parser::Parser::new(&tokens).parse().unwrap();
        let file = SemanticAnalyzer::new().analyze(&file).unwrap();
        let module = &file.modules["m"];
        let authored = module.contributions.len();
        let lowered = lower(Cow::Borrowed(module)).unwrap();
        lowered.contributions[authored..]
            .iter()
            .map(|tie| {
                (
                    tie.branch.to_string(),
                    tie.is_current,
                    matches!(tie.expression, Expression::Conditional(_)),
                )
            })
            .collect()
    }

    #[test]
    fn a_node_left_unconnected_takes_one_siemens_to_ground() {
        assert_eq!(
            ties(
                "electrical x, unused; parameter integer body=0;",
                "I(p,n)<+V(p,n); if (body==1) begin I(p,x)<+V(p,x); I(x,n)<+V(x,n); end",
            ),
            [
                ("x".to_string(), true, true),
                ("unused".to_string(), true, false)
            ]
        );
    }

    #[test]
    fn a_node_another_configuration_holds_at_zero_is_tied_on_that_branch() {
        assert_eq!(
            ties(
                "electrical x; parameter integer mode=0;",
                "I(p,n)<+V(p,n); if (mode==0) V(x)<+0; \
                 else if (mode==1) begin I(p,x)<+V(p,x); I(x)<+V(x); end",
            ),
            [("x".to_string(), false, true)]
        );
    }

    #[test]
    fn a_node_under_a_solved_condition_or_read_unconnected_is_left_alone() {
        assert!(
            ties(
                "electrical x;",
                "I(p,n)<+V(p,n); if (V(p,n)>0) I(p,x)<+V(p,x);"
            )
            .is_empty()
        );
        assert!(
            ties(
                "electrical x; parameter integer body=0;",
                "if (body==1) I(p,x)<+V(p,x); I(p,n)<+V(x,n);",
            )
            .is_empty()
        );
    }
}
