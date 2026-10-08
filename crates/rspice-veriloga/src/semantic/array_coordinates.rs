//! Checked source coordinates for continuous array reads and writes.
use super::*;

impl SemanticAnalyzer {
    pub(super) fn analog_array_value_type(
        &self,
        name: &str,
        module: &AnalyzedModule,
    ) -> Option<crate::array_values::ArrayType> {
        use crate::array_values::{ArrayType, ElementType};
        let name = self.resolve_substituted_name(&name.into());
        let array = self.arrays.get(&name)?;
        let dimensions = if array.declared_dimensions.is_empty() {
            vec![(array.lower, array.lower.checked_add(array.len as i64 - 1)?)]
        } else {
            array.declared_dimensions.clone()
        };
        let mut element = match module.variables.get(array.base)?.var_type {
            VarType::Real => ElementType::Real,
            VarType::Integer => ElementType::Integral {
                width: 32,
                signed: true,
            },
            VarType::String => ElementType::String,
        };
        if let Some(signal) = module
            .digital
            .signals
            .iter()
            .find(|signal| signal.name == name)
        {
            element = if signal.class.is_real() {
                ElementType::Real
            } else {
                ElementType::Integral {
                    width: signal.width,
                    signed: signal.signedness.is_signed(),
                }
            };
        }
        Some(ArrayType {
            layout: crate::array_index::UnpackedArrayLayout::new(
                &dimensions,
                Self::MAX_ARRAY_ELEMENTS,
            )
            .ok()?,
            element,
        })
    }

    pub(super) fn analyze_whole_array_assignment(
        &mut self,
        assign: &AssignmentStmt,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<bool> {
        let LValue::Variable { name, span } = &assign.target else {
            return Ok(false);
        };
        let name = self.resolve_substituted_name(name);
        let Some(array) = self.arrays.get(&name).cloned() else {
            return Ok(false);
        };
        let target = self
            .analog_array_value_type(&name, module)
            .ok_or_else(|| self.array_coordinate_error(&name, "has no array-value type", *span))?;
        let elements = crate::array_values::assignment_elements(&assign.value, &target, |name| {
            self.analog_array_value_type(name, module)
        })
        .map_err(|detail| self.array_coordinate_error(&name, &detail, assign.value.span()))?;
        let var_type = module.variables[array.base].var_type;
        let mut captured = Vec::with_capacity(elements.len());
        // Capture every RHS before modifying any destination, including self
        // permutations and expressions with function output/inout effects.
        for expression in elements {
            self.local_counter += 1;
            let temp: SmolStr = format!("$rspice$array_value${}", self.local_counter).into();
            self.register_function_temp(module, temp.clone(), var_type, *span)?;
            self.analyze_assignment(
                &AssignmentStmt {
                    target: LValue::Variable {
                        name: temp.clone(),
                        span: *span,
                    },
                    value: expression,
                    span: assign.span,
                },
                module,
                sink,
            )?;
            captured.push(temp);
        }
        for (ordinal, temp) in captured.into_iter().enumerate() {
            let slot = target
                .layout
                .declaration_slot(ordinal)
                .expect("array assignment element");
            let index = array.lower + slot as i64;
            self.analyze_assignment(
                &AssignmentStmt {
                    target: LValue::ArrayAccess {
                        name: name.clone(),
                        normalized: true,
                        index: Box::new(Expression::Number(NumberLit {
                            value: index as f64,
                            raw: index.to_string().into(),
                            span: *span,
                        })),
                        additional_indices: Vec::new(),
                        span: *span,
                    },
                    value: Expression::Identifier(Identifier {
                        name: temp,
                        span: *span,
                    }),
                    span: assign.span,
                },
                module,
                sink,
            )?;
        }
        Ok(true)
    }

    pub(super) fn array_coordinate_error(
        &self,
        name: &str,
        detail: &str,
        span: Span,
    ) -> CompileError {
        CompileError::Semantic(SemanticError::new(
            SemanticErrorKind::InvalidExpression(format!("array '{name}' {detail}")),
            span,
        ))
    }

    pub(super) fn array_element_name(name: &str, array: &AnalyzedArray, index: i64) -> SmolStr {
        if array.dimensions.is_empty() {
            return format!("{name}[{index}]").into();
        }
        let shape = crate::array_index::UnpackedArrayLayout::new(
            &array.dimensions,
            Self::MAX_ARRAY_ELEMENTS,
        )
        .expect("validated array shape");
        crate::array_index::element_name(name, &shape, index as usize).into()
    }

    pub(super) fn array_initializer_values<'a>(
        &mut self,
        item: &'a VariableItem,
    ) -> CompileResult<Vec<(usize, &'a Expression)>> {
        let mut bounds = Vec::with_capacity(item.dimensions.len());
        for dimension in &item.dimensions {
            let mut axis = Vec::with_capacity(2);
            for expression in [&dimension.start, &dimension.end] {
                let expression = constant_dependencies::bound_expression(self, expression)
                    .ok_or_else(|| {
                        self.array_coordinate_error(
                            &item.name,
                            "requires constant bounds",
                            item.span,
                        )
                    })?;
                let value = crate::canonical_ir::digital_lower::elaboration_constant(
                    &expression,
                    &self.digital_selector_constants,
                    self.current_time_scale,
                )
                .and_then(|value| match value {
                    crate::numeric_literal::NumericLiteralValue::Integer(value) => Some(value),
                    crate::numeric_literal::NumericLiteralValue::Real(value) => {
                        ConstantValue::Real(value).as_exact_i64()
                    }
                })
                .ok_or_else(|| {
                    self.array_coordinate_error(&item.name, "requires integer bounds", item.span)
                })?;
                axis.push(value);
            }
            bounds.push((axis[0], axis[1]));
        }
        let shape = crate::array_index::UnpackedArrayLayout::new(&bounds, Self::MAX_ARRAY_ELEMENTS)
            .map_err(|_| {
                self.array_coordinate_error(
                    &item.name,
                    "has an invalid initializer shape",
                    item.span,
                )
            })?;
        let initializer = item.init.as_ref().expect("array initializer");
        let elements =
            crate::canonical_ir::digital_lower::array_initializer_elements(initializer, &shape)
                .map_err(|reason| {
                    self.array_coordinate_error(&item.name, &reason, initializer.span())
                })?;
        Ok(elements
            .into_iter()
            .enumerate()
            .map(|(ordinal, expression)| {
                (
                    shape.declaration_slot(ordinal).expect("array element"),
                    expression,
                )
            })
            .collect())
    }

    pub(super) fn lower_array_coordinates(
        &mut self,
        name: &str,
        dimensions: &[(i64, i64)],
        indices: &[&Expression],
        span: Span,
    ) -> CompileResult<Expression> {
        let shape =
            crate::array_index::UnpackedArrayLayout::new(dimensions, Self::MAX_ARRAY_ELEMENTS)
                .map_err(|_| {
                    self.array_coordinate_error(name, "has an invalid coordinate layout", span)
                })?;
        if indices.len() != dimensions.len() {
            return Err(self.array_coordinate_error(
                name,
                &format!("requires {} unpacked indices", dimensions.len()),
                span,
            ));
        }
        let mut offset = None;
        for (axis, index) in shape.axes().iter().zip(indices) {
            let index = self.lower_packed_selector(index)?;
            let lower = axis.left.min(axis.right);
            let coordinate = if let Some(index) = self.constant_array_index(&index, name)? {
                let value =
                    crate::array_index::checked_integer_array_slot(index, 0, axis.len(), lower)
                        .map_err(|_| {
                            self.array_coordinate_error(
                                name,
                                &format!(
                                    "index {index} is outside [{lower}:{}]",
                                    axis.left.max(axis.right)
                                ),
                                span,
                            )
                        })?;
                Self::number_expr(value as f64, span)
            } else {
                Expression::Unary(UnaryExpr {
                    op: UnaryOp::ArrayIndex {
                        lower,
                        len: axis.len() as u32,
                    },
                    operand: Box::new(index),
                    span,
                })
            };
            let term = if axis.stride() == 1 {
                coordinate
            } else {
                Self::binary_expr(
                    BinaryOp::Mul,
                    coordinate,
                    Self::number_expr(axis.stride() as f64, span),
                )
            };
            offset = Some(match offset {
                Some(left) => Self::binary_expr(BinaryOp::Add, left, term),
                None => term,
            });
        }
        Ok(offset.expect("nonempty array rank"))
    }

    pub(super) fn lower_shaped_array_read(
        &mut self,
        select: &ArraySelectExpr,
    ) -> CompileResult<Option<Expression>> {
        let name = self.resolve_substituted_name(&select.name);
        let dimensions = self
            .arrays
            .get(&name)
            .map(|array| array.dimensions.clone())
            .filter(|dimensions| !dimensions.is_empty())
            .or_else(|| {
                self.discrete_projection
                    .dimensions(&name)
                    .map(<[_]>::to_vec)
            });
        let Some(dimensions) = dimensions else {
            if !select.additional_indices.is_empty() {
                return Err(self.array_coordinate_error(
                    &name,
                    "has too many subscripts",
                    select.span,
                ));
            }
            return Ok(None);
        };
        let Some((indices, packed)) = select.split(dimensions.len()) else {
            return Err(self.array_coordinate_error(
                &name,
                &format!(
                    "requires {} unpacked indices before an optional packed selection",
                    dimensions.len()
                ),
                select.span,
            ));
        };
        let offset = self.lower_array_coordinates(&name, &dimensions, &indices, select.span)?;
        if let Some(packed) = packed {
            return self
                .lower_normalized_packed_analog_read(&name, Some(&offset), packed, select.span)
                .map(Some);
        }
        self.lower_expression(&Expression::ArrayAccess(ArrayAccessExpr {
            normalized: true,
            array: name,
            index: Box::new(offset),
            packed: None,
            discrete_validity: None,
            span: select.span,
        }))
        .map(Some)
    }
}
