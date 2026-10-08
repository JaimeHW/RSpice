//! Ordered parameter defaults and bounded expansion of readonly local dependencies.
use super::*;

pub(super) fn declaration_order(module: &Module) -> Vec<(bool, usize)> {
    let mut order: Vec<_> = (0..module.parameters.len())
        .map(|index| (false, index))
        .chain((0..module.localparams.len()).map(|index| (true, index)))
        .collect();
    // The parser uses offsets in one preprocessed source. Stable sorting also
    // preserves authored order within each list for synthesized equal spans.
    order.sort_by_key(|&(local, index)| {
        let declaration = if local {
            &module.localparams[index]
        } else {
            &module.parameters[index]
        };
        (declaration.span.source.raw(), declaration.span.start)
    });
    order
}

impl SemanticAnalyzer {
    pub(super) fn prepare_localparam_default(
        &mut self,
        localparam: &ParameterDecl,
        module: &Module,
    ) -> CompileResult<Option<Expression>> {
        if !localparam.dimensions.is_empty() {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "localparam array '{}' is retained with its declared dimensions, but array-valued localparam storage and indexing are not implemented",
                    localparam.name
                )),
                localparam.span,
            )));
        }
        let closed = localparam
            .default
            .as_ref()
            .is_some_and(|value| self.parameter_operand_is_closed(value));
        let default = localparam
            .default
            .as_ref()
            .map(|expression| {
                if (localparam.packed_range.is_some() || localparam.signedness.is_some())
                    && matches!(expression, Expression::Digital(DigitalExpr::FourState(_)))
                {
                    Ok(expression.clone())
                } else {
                    self.normalize_scalar_parameter_default(localparam, expression, module)
                }
            })
            .transpose()?;
        if closed && let Some(value) = &default {
            self.exact_parameter_constants
                .retain_local_literal(localparam, value);
        } else {
            self.exact_parameter_constants
                .retain(localparam, default.as_ref());
        }
        if let Some(default) = &default {
            if let Some(value) = self
                .eval_const_value(default)
                .and_then(|value| Self::constant_for_declared_type(value, localparam.param_type))
            {
                self.param_consts.insert(localparam.name.clone(), value);
            }
            if let Some(value) = self
                .eval_const_invariant_value(default)
                .and_then(|value| Self::constant_for_declared_type(value, localparam.param_type))
            {
                self.invariant_consts.insert(localparam.name.clone(), value);
            }
        }
        self.define_symbol(Symbol {
            name: localparam.name.clone(),
            kind: SymbolKind::Parameter,
            value_type: match localparam.param_type {
                ParamType::Real => ValueType::Real,
                ParamType::Integer => ValueType::Integer,
                ParamType::String => ValueType::String,
            },
            span: localparam.span,
            attrs: Default::default(),
        })?;
        Ok(default)
    }
}

const MAX_EXPANDED_DEFAULT_NODES: usize = 1_048_576;
const MAX_LOCAL_DEPENDENCY_DEPTH: usize = 128;

#[derive(Debug)]
struct LocalDefault {
    expression: Expression,
    nodes: usize,
    depth: usize,
}

/// Keep local expressions shared until a public default needs them. Analog
/// localparams continue to use their existing prologue variables.
#[derive(Debug, Default)]
pub(crate) struct LocalDefaults {
    values: HashMap<SmolStr, LocalDefault>,
    strings: HashSet<SmolStr>,
}

impl LocalDefaults {
    fn measure(&self, expression: &Expression) -> (usize, usize) {
        let mut nodes = 0usize;
        let mut depth = 0;
        let mut external_names = HashSet::new();
        flow_probes::visit_expression(expression, &mut |value| {
            if is_parameter_query(value)
                && let Expression::SystemFunction(function) = value
            {
                external_names.extend(function.args.iter().map(|value| value as *const Expression));
            }
            // Match expansion: an external-name query does not read a same-
            // spelled local value (its lookup is also case-insensitive).
            if external_names.contains(&(value as *const Expression)) {
                nodes = nodes.saturating_add(1);
                return;
            }
            if let Expression::Identifier(identifier) = value
                && let Some(local) = self.values.get(&identifier.name)
            {
                nodes = nodes.saturating_add(local.nodes);
                depth = depth.max(local.depth.saturating_add(1));
            } else {
                nodes = nodes.saturating_add(1);
            }
        });
        (nodes, depth)
    }

    pub(super) fn insert_string(&mut self, name: SmolStr) {
        self.strings.insert(name);
    }

    pub(super) fn insert(&mut self, name: SmolStr, expression: Expression) {
        let (nodes, depth) = self.measure(&expression);
        self.values.insert(
            name,
            LocalDefault {
                expression,
                nodes,
                depth,
            },
        );
    }

    pub(super) fn expand_range(&self, range: &mut ParameterRange) -> CompileResult<()> {
        for bound in &mut range.bounds {
            bound.lower = bound
                .lower
                .as_ref()
                .map(|value| self.expand(value))
                .transpose()?;
            bound.upper = bound
                .upper
                .as_ref()
                .map(|value| self.expand(value))
                .transpose()?;
        }
        for value in &mut range.exclude {
            *value = self.expand(value)?;
        }
        Ok(())
    }

    pub(super) fn expand(&self, expression: &Expression) -> CompileResult<Expression> {
        if self.values.is_empty() && self.strings.is_empty() {
            return Ok(expression.clone());
        }
        let (nodes, depth) = self.measure(expression);
        if nodes > MAX_EXPANDED_DEFAULT_NODES || depth > MAX_LOCAL_DEPENDENCY_DEPTH {
            return Err(SemanticError::new(SemanticErrorKind::UnsupportedFeature(format!(
                "localparam dependency expansion requires {nodes} expression nodes and {depth} dependency levels; limits are {MAX_EXPANDED_DEFAULT_NODES} nodes and {MAX_LOCAL_DEPENDENCY_DEPTH} levels"
            )), expression.span()).into());
        }
        let mut expanded = expression.clone();
        let mut pending = vec![&mut expanded];
        while let Some(value) = pending.pop() {
            loop {
                let name = match &*value {
                    Expression::Identifier(identifier) => Some(&identifier.name),
                    Expression::ArrayAccess(access) => Some(&access.array),
                    Expression::Digital(expression) => expression.base_name(),
                    _ => None,
                };
                if let Some(name) = name
                    && self.strings.contains(name)
                {
                    return Err(SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "default depends on string localparam '{name}'; typed string parameter evaluation is not implemented"
                        )),
                        value.span(),
                    ).into());
                }
                let Expression::Identifier(identifier) = value else {
                    break;
                };
                let Some(local) = self.values.get(&identifier.name) else {
                    break;
                };
                *value = local.expression.clone();
            }
            if !is_parameter_query(value) {
                flow_probes::for_child_mut(value, &mut |child| pending.push(child));
            }
        }
        Ok(expanded)
    }
}

fn is_parameter_query(expression: &Expression) -> bool {
    matches!(expression, Expression::SystemFunction(function)
        if function.name.eq_ignore_ascii_case("$param_given")
            || function.name.eq_ignore_ascii_case("param_given"))
}
