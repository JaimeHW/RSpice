//! Effective packed assignments and the numeric inputs fixed by elaboration.
use super::*;
use crate::canonical_ir::digital_lower::ParameterAssignment;

pub(super) struct ParameterAssignments {
    values: HashMap<SmolStr, ParameterAssignment>,
    fixed_inputs: HashSet<SmolStr>,
}

impl ParameterAssignments {
    pub(super) fn analyze(module: &Module) -> CompileResult<Self> {
        let order = parameter_defaults::declaration_order(module);
        let declarations: Vec<_> = order
            .iter()
            .map(|&(local, index)| {
                if local {
                    &module.localparams[index]
                } else {
                    &module.parameters[index]
                }
            })
            .collect();
        // Ordinary explicitly typed models need no inference or packed closure.
        if declarations
            .iter()
            .all(|parameter| parameter.type_is_explicit)
        {
            return Ok(Self {
                values: HashMap::new(),
                fixed_inputs: HashSet::new(),
            });
        }
        let assignments = crate::canonical_ir::digital_lower::parameter_assignments(
            &declarations,
            module.time_scale,
        )
        .map_err(|errors| {
            let diagnostic = &errors[0].diagnostic;
            let span = diagnostic.span.map_or(module.span, |span| Span {
                source: crate::source::SourceId::new(span.source_file_id),
                start: span.start,
                end: span.end,
            });
            SemanticError::new(
                SemanticErrorKind::UnsupportedFeature(diagnostic.message.clone()),
                span,
            )
        })?;
        let values: HashMap<_, _> = declarations
            .iter()
            .zip(assignments)
            .map(|(declaration, value)| (declaration.name.clone(), value))
            .collect();
        let by_name: HashMap<_, _> = declarations
            .iter()
            .map(|value| (value.name.clone(), *value))
            .collect();
        let mut pending: Vec<_> = declarations
            .iter()
            .filter(|parameter| {
                (parameter.packed_range.is_some() || parameter.signedness.is_some())
                    && values.get(&parameter.name).is_some_and(|value| {
                        matches!(
                            value.value,
                            Some(Expression::Digital(DigitalExpr::FourState(_)))
                        )
                    })
            })
            .map(|value| value.name.clone())
            .collect();
        let mut fixed_inputs = HashSet::new();
        while let Some(name) = pending.pop() {
            if !fixed_inputs.insert(name.clone()) {
                continue;
            }
            let Some(parameter) = by_name.get(&name) else {
                continue;
            };
            let mut read = |expression: &Expression| {
                flow_probes::visit_expression(expression, &mut |expression| {
                    let name = match expression {
                        Expression::Identifier(value) => Some(&value.name),
                        Expression::ArrayAccess(value) => Some(&value.array),
                        Expression::Digital(value) => value.base_name(),
                        _ => None,
                    };
                    if let Some(name) = name.filter(|name| by_name.contains_key(*name)) {
                        pending.push(name.clone());
                    }
                });
            };
            if let Some(default) = &parameter.default {
                read(default);
            }
            if let Some(range) = &parameter.packed_range {
                read(&range.msb);
                read(&range.lsb);
            }
        }
        Ok(Self {
            values,
            fixed_inputs,
        })
    }

    pub(super) fn prepare(&self, parameter: &mut ParameterDecl) {
        let Some(value) = self.values.get(&parameter.name) else {
            return;
        };
        parameter.param_type = value.numeric_type;
        if let Some(bounds) = value.bounds {
            let span = parameter
                .packed_range
                .as_ref()
                .map_or(parameter.span, |range| range.span);
            parameter.packed_range = Some(VectorRange {
                msb: exact_integer_expression(bounds.msb, span),
                lsb: exact_integer_expression(bounds.lsb, span),
                span,
            });
        }
    }

    pub(super) fn packed_default(&self, parameter: &ParameterDecl) -> Option<Expression> {
        if parameter.packed_range.is_none() && parameter.signedness.is_none() {
            return None;
        }
        self.values
            .get(&parameter.name)
            .and_then(|value| value.value.as_ref())
            .filter(|value| matches!(value, Expression::Digital(DigitalExpr::FourState(_))))
            .cloned()
    }

    pub(super) fn fixed_value(&self, parameter: &ParameterDecl) -> CompileResult<Option<f64>> {
        if !self.fixed_inputs.contains(&parameter.name) || self.packed_default(parameter).is_some()
        {
            return Ok(None);
        }
        let value = self
            .values
            .get(&parameter.name)
            .and_then(|value| value.value.as_ref())
            .and_then(|expression| match expression {
                Expression::Number(value) => Some(value.value),
                Expression::Digital(DigitalExpr::FourState(value)) => {
                    crate::numeric_literal::parse_numeric_literal(&value.value.raw)
                        .ok()
                        .and_then(|value| value.as_exact_f64("elaboration dependency").ok())
                }
                _ => None,
            });
        value.map(Some).ok_or_else(|| {
            SemanticError::new(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "parameter '{}' requires an exact numeric elaboration dependency value",
                    parameter.name
                )),
                parameter.span,
            )
            .into()
        })
    }

    pub(super) fn bounds(&self, name: &str) -> Option<VectorBounds> {
        self.values.get(name).and_then(|value| value.bounds)
    }
}
