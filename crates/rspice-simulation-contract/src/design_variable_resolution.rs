//! A plan's design variables resolved by the engine's own parameter evaluator.
//!
//! A variable's value is an expression that may name other variables of the
//! same plan. The deck writes each one as `.param NAME={expression}` and the
//! engine resolves them; this module resolves them the same way, with the same
//! expression parser and evaluator, so the number a page shows is the number a
//! run reads. A variable that names one the plan does not define, or that
//! names itself through others, has no value and says why.

use std::collections::{BTreeSet, HashMap, HashSet};

use rspice_core::netlist::expr::{Expr, ParamContext, evaluate, parse_expression};

use crate::design_variable::DesignVariable;

/// Every variable of a plan, resolved.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedDesignVariables {
    values: Vec<Result<f64, String>>,
    deck_order: Vec<usize>,
}

impl ResolvedDesignVariables {
    /// Each variable's value, or why it has none, in the plan's order.
    pub fn values(&self) -> &[Result<f64, String>] {
        &self.values
    }

    /// The value of the variable at `index` in the plan, or why it has none.
    pub fn value(&self, index: usize) -> Option<&Result<f64, String>> {
        self.values.get(index)
    }

    /// The plan's variables in the order the deck writes them: each after
    /// every variable it names, and otherwise by name, so reordering the
    /// plan's rows never changes the deck. A variable caught in a loop has no
    /// place in it.
    pub fn deck_order(&self) -> &[usize] {
        &self.deck_order
    }

    /// Every variable that has no value, with the reason, in the plan's order.
    pub fn errors(&self) -> impl Iterator<Item = (usize, &str)> {
        self.values
            .iter()
            .enumerate()
            .filter_map(|(index, value)| value.as_ref().err().map(|error| (index, error.as_str())))
    }
}

/// Resolve every variable of a plan with the engine's parameter evaluator.
///
/// Names are matched without regard to case, as the deck matches them. A
/// variable is evaluated only after every variable it names, so the order the
/// plan lists them in does not matter. Bounds are expressions too and are
/// resolved against the same values; a value outside its bounds is an error on
/// its row, never clamped.
pub fn resolve_design_variables(variables: &[DesignVariable]) -> ResolvedDesignVariables {
    let count = variables.len();
    let mut index_of = HashMap::<String, usize>::with_capacity(count);
    for (index, variable) in variables.iter().enumerate() {
        index_of
            .entry(variable.name.to_ascii_uppercase())
            .or_insert(index);
    }

    let parsed = variables
        .iter()
        .map(|variable| {
            parse_expression(&variable.deck_expression()).map_err(|error| error.to_string())
        })
        .collect::<Vec<_>>();
    let mut values: Vec<Option<Result<f64, String>>> = vec![None; count];
    let mut needs = vec![BTreeSet::<usize>::new(); count];
    for (index, expression) in parsed.iter().enumerate() {
        let expression = match expression {
            Ok(expression) => expression,
            Err(error) => {
                values[index] = Some(Err(error.clone()));
                continue;
            }
        };
        let mut unknown = Vec::new();
        for name in named_parameters(expression) {
            match index_of.get(&name) {
                Some(&other) => {
                    needs[index].insert(other);
                }
                None => unknown.push(typed_spelling(&variables[index].expression, &name)),
            }
        }
        if !unknown.is_empty() {
            values[index] = Some(Err(not_defined(&unknown)));
        }
    }

    for (index, members) in loops(&needs) {
        if values[index].is_none() {
            let names = members
                .iter()
                .map(|&member| variables[member].name.as_str())
                .collect::<Vec<_>>();
            values[index] = Some(Err(if names.len() == 1 {
                format!("{} names itself", names[0])
            } else {
                format!("{} name each other", listed(&names))
            }));
        }
    }

    let deck_order = deck_order(variables, &needs);
    let mut context = ParamContext::new();
    for &index in &deck_order {
        if values[index].is_some() {
            continue;
        }
        let failed = needs[index]
            .iter()
            .find(|&&other| !matches!(values[other], Some(Ok(_))));
        let value = if let Some(&other) = failed {
            Err(format!(
                "uses {}, which has no value",
                variables[other].name
            ))
        } else {
            let expression = parsed[index]
                .as_ref()
                .expect("a variable with no parse error was parsed");
            match evaluate(expression, &context) {
                Ok(value) if value.is_finite() => {
                    within_bounds(&variables[index], value, &context).map(|()| value)
                }
                Ok(_) => Err("does not resolve to a finite number".to_owned()),
                Err(error) => Err(error.to_string()),
            }
        };
        if let Ok(value) = value {
            context.set(&variables[index].name, value);
        }
        values[index] = Some(value);
    }

    // What is left reads a variable caught in a loop, directly or through
    // another: it never became ready, and has no value for the same reason.
    let unresolved = values
        .iter()
        .map(|value| !matches!(value, Some(Ok(_))))
        .collect::<Vec<_>>();
    for index in 0..count {
        if values[index].is_none() {
            let other = needs[index]
                .iter()
                .copied()
                .find(|&other| unresolved[other])
                .expect("a variable left out of the deck order reads one with no value");
            values[index] = Some(Err(format!(
                "uses {}, which has no value",
                variables[other].name
            )));
        }
    }

    ResolvedDesignVariables {
        values: values
            .into_iter()
            .map(|value| value.expect("every variable is resolved or refused"))
            .collect(),
        deck_order,
    }
}

/// Check `value` against the variable's bounds, each resolved against the
/// values already known.
fn within_bounds(
    variable: &DesignVariable,
    value: f64,
    context: &ParamContext,
) -> Result<(), String> {
    let Some(range) = &variable.allowed_range else {
        return Ok(());
    };
    let bound = |label: &str, text: &str| {
        parse_expression(&crate::design_variable::deck_text(text, variable.quantity))
            .and_then(|expression| evaluate(&expression, context))
            .map_err(|error| format!("its {label} {text} does not resolve: {error}"))
    };
    let minimum = bound("minimum", &range.minimum)?;
    let maximum = bound("maximum", &range.maximum)?;
    if minimum > maximum {
        return Err(format!(
            "its minimum {} is above its maximum {}",
            range.minimum, range.maximum
        ));
    }
    if value < minimum || value > maximum {
        return Err(format!(
            "is outside its range {} \u{2026} {}",
            range.minimum, range.maximum
        ));
    }
    Ok(())
}

/// Every parameter `expression` names, folded to upper case as the parser
/// folds them, each once.
fn named_parameters(expression: &Expr) -> Vec<String> {
    let mut names = Vec::new();
    let mut seen = HashSet::new();
    let mut pending = vec![expression];
    while let Some(node) = pending.pop() {
        match node {
            Expr::Param(name) => {
                if seen.insert(name.clone()) {
                    names.push(name.clone());
                }
            }
            Expr::BinOp { left, right, .. } => {
                pending.push(right);
                pending.push(left);
            }
            Expr::UnaryOp { operand, .. } => pending.push(operand),
            Expr::FnCall { args, .. } => pending.extend(args.iter().rev()),
            Expr::Number(_) | Expr::ComplexNumber(_) | Expr::StringLiteral(_) => {}
        }
    }
    names
}

/// `name` as the author typed it in `text`: the parser folds names to one
/// case, and the reader looks for the spelling they wrote.
fn typed_spelling(text: &str, name: &str) -> String {
    text.split(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
        .find(|word| word.eq_ignore_ascii_case(name))
        .unwrap_or(name)
        .to_owned()
}

fn not_defined(names: &[String]) -> String {
    let names = names.iter().map(String::as_str).collect::<Vec<_>>();
    if names.len() == 1 {
        format!("{} is not defined", names[0])
    } else {
        format!("{} are not defined", listed(&names))
    }
}

/// `A`, `A and B`, `A, B and C`.
fn listed(names: &[&str]) -> String {
    match names {
        [] => String::new(),
        [only] => (*only).to_owned(),
        [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
    }
}

/// Every variable that names itself, directly or through others, with the
/// variables of its loop in the plan's order.
fn loops(needs: &[BTreeSet<usize>]) -> Vec<(usize, Vec<usize>)> {
    let reach = |start: usize| {
        let mut reached = BTreeSet::new();
        let mut pending = needs[start].iter().copied().collect::<Vec<_>>();
        while let Some(node) = pending.pop() {
            if reached.insert(node) {
                pending.extend(needs[node].iter().copied());
            }
        }
        reached
    };
    let reached = (0..needs.len()).map(reach).collect::<Vec<_>>();
    (0..needs.len())
        .filter(|&index| reached[index].contains(&index))
        .map(|index| {
            let members = reached[index]
                .iter()
                .copied()
                .filter(|&other| reached[other].contains(&index))
                .collect();
            (index, members)
        })
        .collect()
}

/// The variables in an order where each follows every variable it names,
/// ties broken by name; a variable in a loop, or after one, is left out.
fn deck_order(variables: &[DesignVariable], needs: &[BTreeSet<usize>]) -> Vec<usize> {
    let key = |index: usize| (variables[index].name.to_ascii_lowercase(), index);
    let mut waiting = needs.iter().map(BTreeSet::len).collect::<Vec<_>>();
    let mut readers = vec![Vec::new(); needs.len()];
    for (index, named) in needs.iter().enumerate() {
        for &other in named {
            readers[other].push(index);
        }
    }
    let mut ready = (0..needs.len())
        .filter(|&index| waiting[index] == 0)
        .map(key)
        .collect::<BTreeSet<_>>();
    let mut order = Vec::with_capacity(needs.len());
    while let Some((_, index)) = ready.pop_first() {
        order.push(index);
        for &reader in &readers[index] {
            waiting[reader] -= 1;
            if waiting[reader] == 0 {
                ready.insert(key(reader));
            }
        }
    }
    order
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::design_variable::{
        DesignVariableOverridePolicy, DesignVariableRange, DesignVariableSweepEligibility,
    };
    use crate::design_variable_quantity::DesignVariableQuantity;

    fn variable(name: &str, expression: &str) -> DesignVariable {
        DesignVariable::new(
            name,
            expression,
            DesignVariableQuantity::Dimensionless,
            "",
            None,
            DesignVariableSweepEligibility::NestedSweepAndOptimization,
            DesignVariableOverridePolicy::ExplicitTestLocalOverride,
        )
        .expect("a well-formed variable")
    }

    fn close(actual: f64, expected: f64) -> bool {
        (actual - expected).abs() <= expected.abs() * 1e-12
    }

    /// `W2 = 2*W1`, `VCM = VDD/2` and `fc = 1/(2*pi*R*C)` resolve, each after
    /// what it names, whatever order the plan lists them in; and the deck
    /// written in that order gives the engine the same numbers.
    #[test]
    fn a_variable_may_name_other_variables_and_functions() {
        let variables = [
            variable("fc", "1/(2*pi*R*C)"),
            variable("W2", "2*W1"),
            variable("VCM", "VDD/2"),
            variable("W1", "4u"),
            variable("VDD", "1.0"),
            variable("R", "1k"),
            variable("C", "1n"),
        ];
        let resolved = resolve_design_variables(&variables);
        let value = |index: usize| *resolved.value(index).unwrap().as_ref().unwrap();
        assert!(close(
            value(0),
            1.0 / (2.0 * std::f64::consts::PI * 1e3 * 1e-9)
        ));
        assert!(close(value(1), 8e-6));
        assert_eq!(value(2), 0.5);

        let position = |index: usize| {
            resolved
                .deck_order()
                .iter()
                .position(|&at| at == index)
                .unwrap()
        };
        assert!(position(3) < position(1), "W1 is written before W2");
        assert!(position(4) < position(2), "VDD is written before VCM");
        assert!(position(5) < position(0) && position(6) < position(0));

        let cards = resolved
            .deck_order()
            .iter()
            .map(|&index| variables[index].netlist_statement())
            .collect::<Vec<_>>();
        let deck = format!("variables\n{}\n.end\n", cards.join("\n"));
        let parsed = rspice_core::Netlist::parse(&deck).expect("the cards parse");
        assert!(close(parsed.params.get("W2").unwrap(), 8e-6));
        assert_eq!(parsed.params.get("VCM"), Some(0.5));
        assert!(close(parsed.params.get("fc").unwrap(), value(0)));
    }

    /// A loop names every variable in it; a variable that reads a loop says
    /// which variable it is waiting on.
    #[test]
    fn a_loop_names_both_variables() {
        let variables = [
            variable("A", "B"),
            variable("B", "A"),
            variable("C", "A+1"),
            variable("D", "D+1"),
        ];
        let resolved = resolve_design_variables(&variables);
        assert_eq!(
            resolved.values(),
            [
                Err("A and B name each other".to_owned()),
                Err("A and B name each other".to_owned()),
                Err("uses A, which has no value".to_owned()),
                Err("D names itself".to_owned()),
            ]
        );
        assert!(resolved.deck_order().is_empty());
    }

    /// A name no variable defines is named as it was typed.
    #[test]
    fn an_unknown_name_is_named() {
        let variables = [variable("VCM", "vdd/2"), variable("W2", "2*W1+Wx")];
        let resolved = resolve_design_variables(&variables);
        assert_eq!(
            resolved.values(),
            [
                Err("vdd is not defined".to_owned()),
                Err("W1 and Wx are not defined".to_owned()),
            ]
        );
    }

    /// Bounds resolve against the same values, and a value they exclude is
    /// an error on its row.
    #[test]
    fn a_resolved_value_outside_its_bounds_is_an_error() {
        let mut wide = variable("W2", "2*W1");
        wide.allowed_range = Some(DesignVariableRange {
            minimum: "W1".to_owned(),
            maximum: "6u".to_owned(),
        });
        let resolved = resolve_design_variables(&[variable("W1", "4u"), wide]);
        assert_eq!(
            resolved.value(1),
            Some(&Err("is outside its range W1 \u{2026} 6u".to_owned()))
        );
    }
}
