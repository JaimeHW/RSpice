//! Supplied-state queries used by elaboration, separately from value reads.

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

use smol_str::SmolStr;

use crate::ast::{Expression, Module, NumberLit, ParameterDecl};

#[derive(Debug, Clone, Default)]
pub(crate) struct GivenParameters {
    names: HashMap<SmolStr, (SmolStr, bool)>,
}

impl GivenParameters {
    pub(crate) fn new(module: &Module) -> Self {
        let mut result = Self::default();
        for parameter in &module.parameters {
            result.names.insert(
                parameter.name.to_ascii_lowercase().into(),
                (parameter.name.clone(), parameter.is_given),
            );
        }
        for alias in &module.aliasparams {
            if let Some(parameter) = module
                .parameters
                .iter()
                .find(|value| value.name == alias.target)
            {
                result.names.insert(
                    alias.alias.to_ascii_lowercase().into(),
                    (parameter.name.clone(), parameter.is_given),
                );
            }
        }
        result
    }

    /// Fold only when the caller has selected a legal elaboration context.
    /// Procedural digital expressions must never be routed through this helper.
    /// Dependencies name the queried declaration, not its current numeric value.
    pub(crate) fn fold<'a>(
        &self,
        expression: &'a Expression,
    ) -> Result<(Cow<'a, Expression>, HashSet<SmolStr>), String> {
        let mut contains_query = false;
        super::flow_probes::visit_expression(expression, &mut |expression| {
            contains_query |= matches!(expression, Expression::SystemFunction(function)
                if function.name.eq_ignore_ascii_case("$param_given")
                    || function.name.eq_ignore_ascii_case("param_given"));
        });
        if !contains_query {
            return Ok((Cow::Borrowed(expression), HashSet::new()));
        }
        let mut expression = expression.clone();
        let mut dependencies = HashSet::new();
        let mut pending = vec![&mut expression];
        while let Some(expression) = pending.pop() {
            if let Expression::SystemFunction(function) = expression
                && (function.name.eq_ignore_ascii_case("$param_given")
                    || function.name.eq_ignore_ascii_case("param_given"))
            {
                let [Expression::Identifier(identifier)] = function.args.as_slice() else {
                    return Err("$param_given requires one parameter name".into());
                };
                let Some((canonical, given)) = self
                    .names
                    .get(identifier.name.to_ascii_lowercase().as_str())
                else {
                    return Err(format!(
                        "$param_given names unknown or non-public parameter '{}'",
                        identifier.name
                    ));
                };
                dependencies.insert(canonical.clone());
                *expression = Expression::Number(NumberLit {
                    value: if *given { 1.0 } else { 0.0 },
                    raw: if *given { "1" } else { "0" }.into(),
                    span: function.span,
                });
            } else {
                super::flow_probes::for_child_mut(expression, &mut |child| pending.push(child));
            }
        }
        Ok((Cow::Owned(expression), dependencies))
    }

    pub(crate) fn declaration<'a>(
        &self,
        declaration: &'a ParameterDecl,
    ) -> Result<Cow<'a, ParameterDecl>, String> {
        let mut result = Cow::Borrowed(declaration);
        if let Some(default) = &declaration.default
            && let Cow::Owned(value) = self.fold(default)?.0
        {
            result.to_mut().default = Some(value);
        }
        if let Some(range) = &declaration.packed_range {
            if let Cow::Owned(value) = self.fold(&range.msb)?.0 {
                result
                    .to_mut()
                    .packed_range
                    .as_mut()
                    .expect("source range")
                    .msb = value;
            }
            if let Cow::Owned(value) = self.fold(&range.lsb)?.0 {
                result
                    .to_mut()
                    .packed_range
                    .as_mut()
                    .expect("source range")
                    .lsb = value;
            }
        }
        Ok(result)
    }
}
