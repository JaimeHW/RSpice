//! Numeric parameter-array storage and cold declaration initialization.
use super::*;

impl SemanticAnalyzer {
    pub(super) fn prepare_localparam_array(
        &mut self,
        declaration: &ParameterDecl,
        locals: &parameter_defaults::LocalDefaults,
        declarations: &[ParameterDecl],
        indices: &HashMap<SmolStr, usize>,
    ) -> CompileResult<(VariableItem, VarType)> {
        let mut parameter = declaration.clone();
        parameter.default = parameter
            .default
            .as_ref()
            .map(|value| locals.expand(value))
            .transpose()?;
        for dimension in &mut parameter.dimensions {
            dimension.start = locals.expand(&dimension.start)?;
            dimension.end = locals.expand(&dimension.end)?;
        }
        let default = parameter
            .default
            .as_ref()
            .map(|default| {
                let materialized = self.materialize_replication_expression(
                    default,
                    MAX_PARAMETER_ARRAY_ELEMENTS as usize,
                    MAX_REPLICATION_MATERIALIZATION_WORK,
                    &format!("default of localparam array '{}'", parameter.name),
                    false,
                )?;
                self.normalize_integer_expression(&materialized)
            })
            .transpose()?;
        let errors = self.errors.len();
        self.validate_parameter_array_declaration(
            &parameter,
            default.as_ref(),
            indices[&parameter.name],
            declarations,
            indices,
        );
        if self.errors.len() > errors {
            return Err(self.errors.remove(errors).into());
        }
        let (var_type, value_type) = match parameter.param_type {
            ParamType::Real => (VarType::Real, ValueType::Real),
            ParamType::Integer => (VarType::Integer, ValueType::Integer),
            ParamType::String => unreachable!("numeric parameter array validation"),
        };
        let default = if parameter.param_type == ParamType::Integer {
            default
                .map(|value| self.coerce_integer_parameter_array_default(value))
                .transpose()?
        } else {
            default
        };
        self.define_symbol(Symbol {
            name: parameter.name.clone(),
            kind: SymbolKind::Parameter,
            value_type,
            span: parameter.span,
            attrs: Default::default(),
        })?;
        Ok((
            VariableItem {
                name: parameter.name,
                dimensions: parameter.dimensions,
                init: default,
                span: parameter.span,
            },
            var_type,
        ))
    }

    pub(super) fn parameter_array_items(module: &AnalyzedModule) -> Vec<(VariableItem, VarType)> {
        module
            .parameters
            .iter()
            .filter(|parameter| !parameter.dimensions.is_empty())
            .map(|parameter| {
                (
                    VariableItem {
                        name: parameter.name.clone(),
                        dimensions: parameter
                            .dimensions
                            .iter()
                            .map(|dimension| ArrayDimension {
                                start: dimension.left.clone(),
                                end: dimension.right.clone(),
                                span: dimension.span,
                            })
                            .collect(),
                        init: parameter.default_expr.clone(),
                        span: parameter.dimensions[0].span,
                    },
                    match parameter.param_type {
                        ParamType::Real => VarType::Real,
                        ParamType::Integer => VarType::Integer,
                        ParamType::String => VarType::String,
                    },
                )
            })
            .collect()
    }

    pub(super) fn initialize_array_storage(
        &mut self,
        item: &VariableItem,
        module: &mut AnalyzedModule,
        statements: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        if item.init.is_none() {
            // Invalid parameter declarations are reported by the declaration
            // validator; never turn a missing default into an analyzer panic.
            return Ok(());
        }
        let Some(layout) = self.arrays.get(&item.name).cloned() else {
            // A bound/layout diagnostic has already been recorded.
            return Ok(());
        };
        for (offset, element) in self.array_initializer_values(item, module)? {
            let var_index = layout.base + offset;
            let expression =
                self.lower_expression_with_side_effects(element, module, statements)?;
            let (expression, expr_type) = self
                .coerce_assignment_expression(expression, module.variables[var_index].value_type)?;
            let site = self.next_analog_site();
            let assignment = AnalyzedAssignment {
                occurrence_source: None,
                target: module.variables[var_index].name.clone(),
                var_index,
                index: None,
                expression,
                site,
                expression_guard: AnalogSiteGuard::None,
                expr_type,
                span: item.span,
                unfiltered_initial_step_guard: None,
            };
            self.record_region(AnalyzedRegion::Assignment(assignment.clone()));
            statements.push(AnalyzedStatement::Assignment(assignment));
        }
        Ok(())
    }
}
