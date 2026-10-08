//! Numeric parameter-array storage and cold declaration initialization.
use super::*;

impl SemanticAnalyzer {
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
