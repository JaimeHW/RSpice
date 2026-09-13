//! Find procedural reads whose reaching value can precede this evaluation.

use super::{AnalyzedAssignment, AnalyzedModule, AnalyzedRegion, AnalyzedStatement};
use crate::ast::Expression;
use std::collections::{HashMap, HashSet};

pub(super) fn record(module: &mut AnalyzedModule) {
    let mut flow = Flow {
        module,
        names: module
            .variables
            .iter()
            .enumerate()
            .map(|(slot, variable)| (variable.name.as_str(), slot))
            .collect(),
        reads: HashSet::new(),
        writes: HashSet::new(),
    };
    let mut defined = HashSet::new();
    for &index in &module.prologue_statements {
        if let AnalyzedStatement::Assignment(assignment) = &module.statements[index] {
            flow.assignment(assignment, &mut defined);
        }
    }
    flow.regions(&module.body, &mut defined);
    let mut retained: Vec<_> = flow.reads.intersection(&flow.writes).copied().collect();
    retained.sort_unstable();
    for slot in retained {
        // Event-controlled storage has a distinct publication contract: the
        // executable backend consumes the assignment pass's settled candidate.
        // Ordinary procedural lifetimes must not change those reads.
        if module.variables[slot].is_event_controlled {
            continue;
        }
        module.variables[slot].retains_input = true;
        module.variables[slot].is_state = true;
        module.event_state_variables.push(slot);
    }
    module.event_state_variables.sort_unstable();
    module.event_state_variables.dedup();
}

struct Flow<'a> {
    module: &'a AnalyzedModule,
    names: HashMap<&'a str, usize>,
    reads: HashSet<usize>,
    writes: HashSet<usize>,
}

impl Flow<'_> {
    fn expression(&mut self, expression: &Expression, defined: &HashSet<usize>) {
        super::flow_probes::visit_expression(expression, &mut |expression| {
            let mut read = |slot| {
                if !defined.contains(&slot) {
                    self.reads.insert(slot);
                }
            };
            match expression {
                Expression::Identifier(identifier) => {
                    if let Some(&slot) = self.names.get(identifier.name.as_str()) {
                        read(slot);
                    } else if let Some(array) = self.module.arrays.get(&identifier.name) {
                        (array.base..array.base + array.len).for_each(read);
                    }
                }
                Expression::ArrayAccess(access) => {
                    if let Some(array) = self.module.arrays.get(&access.array) {
                        if let Expression::Number(index) = &*access.index
                            && index.value.is_finite()
                            && index.value.round() >= array.lower as f64
                            && index.value.round() < (array.lower as f64 + array.len as f64)
                        {
                            read(array.base + (index.value.round() - array.lower as f64) as usize);
                        } else {
                            (array.base..array.base + array.len).for_each(read);
                        }
                    }
                }
                _ => {}
            }
        });
    }

    fn assignment(&mut self, assignment: &AnalyzedAssignment, defined: &mut HashSet<usize>) {
        self.expression(&assignment.expression, defined);
        if let Some(index) = &assignment.index {
            self.expression(index, defined);
            if let Some(array) = self.module.arrays.get(&assignment.target) {
                self.writes.extend(array.base..array.base + array.len);
                if array.len == 1 {
                    defined.insert(array.base);
                }
            }
        } else {
            self.writes.insert(assignment.var_index);
            defined.insert(assignment.var_index);
        }
    }

    fn regions(&mut self, regions: &[AnalyzedRegion], defined: &mut HashSet<usize>) {
        for region in regions {
            match region {
                AnalyzedRegion::Assignment(assignment) => self.assignment(assignment, defined),
                AnalyzedRegion::Contribution(contribution) => {
                    self.expression(&contribution.expression, defined);
                    if let Some(abstol) = &contribution.equation_abstol {
                        self.expression(abstol, defined);
                    }
                }
                AnalyzedRegion::Conditional {
                    condition,
                    then_body,
                    else_body,
                    ..
                } => {
                    self.expression(condition, defined);
                    let mut then_defined = defined.clone();
                    let mut else_defined = defined.clone();
                    self.regions(then_body, &mut then_defined);
                    self.regions(else_body, &mut else_defined);
                    then_defined.retain(|slot| else_defined.contains(slot));
                    *defined = then_defined;
                }
                AnalyzedRegion::Loop {
                    condition, body, ..
                } => {
                    self.expression(condition, defined);
                    // The first trip starts with these definitions. Back-edge
                    // writes cannot satisfy an entry read, and zero trips cannot
                    // establish a definition after the loop.
                    self.regions(body, &mut defined.clone());
                }
                AnalyzedRegion::Task(task) => {
                    for expression in task.expressions() {
                        self.expression(expression, defined);
                    }
                }
                AnalyzedRegion::Initialization { .. } => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    fn retained(body: &str) -> Vec<String> {
        let source = format!("module m(p,n); inout p,n; electrical p,n; {body} endmodule");
        let tokens = crate::lexer::Lexer::new(&source, crate::source::SourceId::new(0))
            .collect_tokens()
            .unwrap();
        let file = crate::parser::Parser::new(&tokens).parse().unwrap();
        let file = super::super::SemanticAnalyzer::new()
            .analyze(&file)
            .unwrap();
        let module = &file.modules["m"];
        module
            .variables
            .iter()
            .enumerate()
            .filter(|(_, variable)| variable.retains_input)
            .map(|(slot, variable)| {
                assert!(variable.is_state);
                assert!(module.event_state_variables.binary_search(&slot).is_ok());
                variable.name.to_string()
            })
            .collect()
    }

    #[test]
    fn retained_inputs_follow_source_order_and_branch_definitions() {
        assert_eq!(
            retained("real held, seen; analog begin seen=held; held=2*V(p,n); I(p,n)<+seen; end"),
            ["held"]
        );
        assert!(
            retained("real x; analog begin if (V(p,n)>0) x=1; else x=2; I(p,n)<+x; end").is_empty()
        );
        assert_eq!(
            retained("real x; analog begin if (V(p,n)>0) x=1; I(p,n)<+x; end"),
            ["x"]
        );
        assert!(retained("real x; analog I(p,n)<+x;").is_empty());
    }

    #[test]
    fn retained_inputs_cover_zero_trip_loops_and_fixed_array_members() {
        assert_eq!(
            retained(
                "real x; integer i; parameter integer trips=2; analog begin i=0; while (i<trips) begin x=V(p,n); i=i+1; end I(p,n)<+x; end"
            ),
            ["x"]
        );
        assert_eq!(
            retained(
                "real a[0:1], seen; analog begin a[0]=V(p,n); seen=a[1]; a[1]=2*V(p,n); I(p,n)<+a[0]+seen; end"
            ),
            ["a[1]"]
        );
    }

    #[test]
    fn retained_inputs_preserve_event_owned_publication() {
        assert!(
            retained(
                "real count=3; analog begin @(initial_step) count=count+1; I(p,n)<+count; end"
            )
            .is_empty()
        );
        assert_eq!(
            retained(
                "real held,seen; analog initial held=3; analog begin seen=held; held=2*V(p,n); I(p,n)<+seen; end"
            ),
            ["held"]
        );
    }

    #[test]
    fn retained_inputs_do_not_turn_function_initializers_into_state() {
        assert!(retained("analog function real f; input x; real x, y; begin if (x>0) y=x; f=y; end endfunction real out; analog begin out=f(V(p,n)); I(p,n)<+out; end").is_empty());
        assert_eq!(
            retained("real held=3, seen; analog begin seen=held; held=2*V(p,n); I(p,n)<+seen; end"),
            ["held"]
        );
    }
}
