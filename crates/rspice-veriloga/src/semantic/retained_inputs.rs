//! Find procedural reads whose reaching value can precede this evaluation.

use super::{AnalyzedAssignment, AnalyzedModule, AnalyzedRegion, AnalyzedStatement};
use crate::ast::Expression;
use crate::source::Span;
use smol_str::SmolStr;
use std::collections::{HashMap, HashSet};

/// One source variable whose value carries from the last accepted point,
/// with the first read that can see it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct RetainedVariable {
    /// The variable as the author declared it; an array is named once.
    pub(super) name: SmolStr,
    /// The first read, in execution order, that no assignment reaches.
    pub(super) first_read: Span,
}

/// Mark every variable some path reads before it is assigned as retained
/// state, and return them in the order their first such read executes.
pub(super) fn record(module: &mut AnalyzedModule) -> Vec<RetainedVariable> {
    let mut flow = Flow {
        module,
        names: module
            .variables
            .iter()
            .enumerate()
            .map(|(slot, variable)| (variable.name.as_str(), slot))
            .collect(),
        reads: HashMap::new(),
        writes: HashSet::new(),
        site: 0,
    };
    let mut defined = HashSet::new();
    for &index in &module.prologue_statements {
        if let AnalyzedStatement::Assignment(assignment) = &module.statements[index] {
            flow.assignment(assignment, &mut defined);
        }
    }
    flow.regions(&module.body, &mut defined);
    let Flow { reads, writes, .. } = flow;
    let mut retained: Vec<_> = reads
        .into_iter()
        .filter(|(slot, _)| writes.contains(slot))
        .collect();
    retained.sort_unstable_by_key(|&(slot, _)| slot);
    let mut reported = Vec::new();
    for (slot, first_read) in retained {
        // Event-controlled storage has a distinct publication contract: the
        // executable backend consumes the assignment pass's settled candidate.
        // Ordinary procedural lifetimes must not change those reads.
        if module.variables[slot].is_event_controlled {
            continue;
        }
        module.variables[slot].retains_input = true;
        module.variables[slot].is_state = true;
        module.event_state_variables.push(slot);
        reported.push((slot, first_read));
    }
    module.event_state_variables.sort_unstable();
    module.event_state_variables.dedup();
    report(module, reported)
}

/// Name each retained variable once, an array by its own name, at the first
/// read of any of its slots.
fn report(module: &AnalyzedModule, slots: Vec<(usize, (Order, Span))>) -> Vec<RetainedVariable> {
    let mut first: HashMap<SmolStr, (Order, Span)> = HashMap::new();
    for (slot, read) in slots {
        let name = module
            .arrays
            .iter()
            .find(|(_, array)| (array.base..array.base + array.len).contains(&slot))
            .map_or_else(
                || module.variables[slot].name.clone(),
                |(name, _)| name.clone(),
            );
        first
            .entry(name)
            .and_modify(|earliest| {
                if read.0 < earliest.0 {
                    *earliest = read;
                }
            })
            .or_insert(read);
    }
    let mut variables: Vec<_> = first.into_iter().collect();
    variables.sort_unstable_by_key(|(_, (order, _))| *order);
    variables
        .into_iter()
        .map(|(name, (_, first_read))| RetainedVariable { name, first_read })
        .collect()
}

/// Where a read runs: the expression's place in the walk, then its offset
/// within that expression, whose operands the walk need not visit in order.
type Order = (usize, u32);

struct Flow<'a> {
    module: &'a AnalyzedModule,
    names: HashMap<&'a str, usize>,
    /// Each slot read with no reaching assignment, with the execution order
    /// and site of its first such read.
    reads: HashMap<usize, (Order, Span)>,
    writes: HashSet<usize>,
    /// Expressions walked so far.
    site: usize,
}

impl Flow<'_> {
    fn expression(&mut self, expression: &Expression, defined: &HashSet<usize>) {
        self.site += 1;
        let site = self.site;
        super::flow_probes::visit_expression(expression, &mut |expression| {
            let span = expression.span();
            let read_at = ((site, span.start), span);
            let mut read = |slot| {
                if !defined.contains(&slot) {
                    let first = self.reads.entry(slot).or_insert(read_at);
                    if read_at.0 < first.0 {
                        *first = read_at;
                    }
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
                            && access.packed.is_none()
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

    /// A case with no default leaves its variable unassigned when no arm
    /// matches; a default that assigns it closes that path. Neither direction
    /// may be approximated: the first must retain, the second must not.
    #[test]
    fn retained_inputs_are_exact_on_case_arms() {
        let case = |default: &str| {
            retained(&format!(
                "real x; integer sel; analog begin sel=V(p,n)>0; \
                 case (sel) 0: x=1; 1: x=2; {default} endcase I(p,n)<+x; end"
            ))
        };
        assert_eq!(case(""), ["x"]);
        assert!(case("default: x=3;").is_empty());
        assert_eq!(case("default: ;"), ["x"]);
    }

    /// A runtime loop's first trip reads what the previous evaluation left in
    /// a variable its body assigns later; a read after the body's own
    /// assignment on the same trip does not. A compile-time-bounded loop runs
    /// its body, so its assignments reach the reads after it.
    #[test]
    fn retained_inputs_are_exact_on_loop_carried_and_unrolled_reads() {
        assert_eq!(
            retained(
                "real x, y; integer i; parameter integer trips=2; analog begin y=0; i=0; \
                 while (i<trips) begin y=y+x; x=V(p,n); i=i+1; end I(p,n)<+y; end"
            ),
            ["x"]
        );
        assert!(
            retained(
                "real x, y; integer i; parameter integer trips=2; analog begin y=0; i=0; \
                 while (i<trips) begin x=V(p,n); y=y+x; i=i+1; end I(p,n)<+y; end"
            )
            .is_empty()
        );
        assert!(
            retained(
                "real x; integer i; analog begin for (i=0; i<2; i=i+1) x=V(p,n)*i; I(p,n)<+x; end"
            )
            .is_empty()
        );
    }

    fn warnings(source: &str) -> Vec<(String, String)> {
        let tokens = crate::lexer::Lexer::new(source, crate::source::SourceId::new(0))
            .collect_tokens()
            .unwrap();
        let file = crate::parser::Parser::new(&tokens).parse().unwrap();
        let warnings = super::super::SemanticAnalyzer::new()
            .analyze(&file)
            .unwrap()
            .warnings;
        crate::runtime_report::semantic_warning_diagnostics(
            source,
            &warnings,
            &crate::runtime_report::ExpandedLines(source),
        )
        .into_iter()
        .filter(|warning| warning.code == super::super::RETAINED_VARIABLE_CODE)
        .map(|warning| {
            let span = warning.span.unwrap();
            let text = &source[span.byte_start as usize..span.byte_end as usize];
            (
                warning.message,
                format!("{}: {text}", span.start.unwrap().line),
            )
        })
        .collect()
    }

    /// A module's retained variables are one finding, at the first read no
    /// assignment reaches, listing each variable once (an array by its own
    /// name) with the line of its own first such read, in the order those
    /// reads run.
    #[test]
    fn each_module_lists_its_retained_variables_in_one_finding() {
        let source = "module m(p,n); inout p,n; electrical p,n;\n\
                      real count, a[0:2], seen, x;\n\
                      analog begin\n\
                      x = V(p,n);\n\
                      if (x > 0.5) count = count + 1;\n\
                      seen = a[1] + count + a[2];\n\
                      a[1] = x; a[2] = seen;\n\
                      I(p,n) <+ count + seen + x;\n\
                      end endmodule\n\
                      module h(p,n); inout p,n; electrical p,n; real held;\n\
                      analog begin if (V(p,n) > 0.5) held = V(p,n);\n\
                      I(p,n) <+ held; end endmodule";
        assert_eq!(
            warnings(source),
            [
                (
                    "Module `m` has 2 variables read before they are assigned, so they keep \
                     their values from the last accepted time point: `count` (line 5), `a` \
                     (line 6)"
                        .to_string(),
                    "5: count".to_string()
                ),
                (
                    "Module `h` has 1 variable read before it is assigned, so it keeps its \
                     value from the last accepted time point: `held` (line 12)"
                        .to_string(),
                    "12: held".to_string()
                ),
            ]
        );
        assert!(
            warnings(
                "module m(p,n); inout p,n; electrical p,n; real x; analog begin \
                 x = V(p,n); I(p,n) <+ x; end endmodule"
            )
            .is_empty()
        );
    }
}
