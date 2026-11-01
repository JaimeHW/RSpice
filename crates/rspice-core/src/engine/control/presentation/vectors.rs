//! Vectors a script binds with `let`, kept per plot as ngspice keeps them
//! (frontend/com_let.c, vectors.c `vec_get`), and whole-vector evaluation of
//! the expressions that name them.

use super::*;

/// The value of a vector expression (ngspice `struct dvec`).
#[derive(Debug, Clone)]
pub(in crate::engine::control) struct VectorValue {
    pub(in crate::engine::control) samples: Vec<ComplexValue>,
    /// ngspice VF_COMPLEX: an AC result stays complex even where every
    /// imaginary part is zero.
    pub(in crate::engine::control) complex: bool,
    /// The dataset whose scale the samples follow (`v_scale`), if any.
    pub(in crate::engine::control) scale: Option<String>,
    pub(in crate::engine::control) unit: SignalUnit,
}

#[derive(Debug, Clone)]
struct UserVector {
    name: String,
    /// The owning plot's dataset name; `None` is ngspice's constants plot,
    /// which holds the vectors bound before any analysis ran.
    plot: Option<String>,
    value: VectorValue,
}

#[derive(Debug, Default)]
pub(in crate::engine::control) struct VectorStore {
    vectors: Vec<UserVector>,
}

impl VectorStore {
    /// Retained values, counting each sample as a complex pair.
    fn values(&self) -> usize {
        self.vectors.iter().fold(0usize, |count, vector| {
            count.saturating_add(vector.value.samples.len().saturating_mul(2))
        })
    }
}

/// How a bound subexpression combines: its unit and whether it is complex.
#[derive(Clone)]
struct Shape {
    unit: SignalUnit,
    complex: bool,
}

impl Shape {
    fn real(unit: SignalUnit) -> Self {
        Self {
            unit,
            complex: false,
        }
    }
}

impl ControlCircuit {
    /// ngspice `vec_get`: a plot prefix names the plot, otherwise the current
    /// plot; the constants plot answers for either when it has the name.
    fn user_vector(&self, name: &str) -> Option<usize> {
        let (plot, name) = match name.split_once('.') {
            Some((prefix, rest)) if prefix.eq_ignore_ascii_case("const") => (Some(None), rest),
            Some((prefix, rest)) => match self.dataset(prefix) {
                Some(dataset) => (Some(Some(dataset.name.as_str())), rest),
                None => (None, name),
            },
            None => (None, name),
        };
        let plot = plot.unwrap_or_else(|| self.current_dataset().map(|d| d.name.as_str()));
        let find = |plot: Option<&str>| {
            self.vectors.vectors.iter().position(|vector| {
                vector.plot.as_deref() == plot && vector.name.eq_ignore_ascii_case(name)
            })
        };
        find(plot).or_else(|| find(None))
    }

    /// Whether the expression reads a vector bound by `let`.
    pub(in crate::engine::control) fn names_user_vector(&self, expression: &Expr) -> bool {
        match expression {
            Expr::Param(name) => self.user_vector(name).is_some(),
            Expr::FnCall { name, args } => {
                self.voltage_alias(name, args).is_some()
                    || (!is_probe(name) && args.iter().any(|arg| self.names_user_vector(arg)))
            }
            Expr::BinOp { left, right, .. } => {
                self.names_user_vector(left) || self.names_user_vector(right)
            }
            Expr::UnaryOp { operand, .. } => self.names_user_vector(operand),
            Expr::Number(_) | Expr::ComplexNumber(_) | Expr::StringLiteral(_) => false,
        }
    }

    /// ngspice reads `v(name)` as the vector `name` (frontend/parse.c), so
    /// it names a `let` vector before a node of the same name.
    fn voltage_alias(&self, name: &str, args: &[Expr]) -> Option<usize> {
        let [Expr::Param(vector)] = args else {
            return None;
        };
        let (prefix, probe) = name.rsplit_once('.').unwrap_or(("", name));
        if probe != "V" {
            return None;
        }
        if prefix.is_empty() {
            self.user_vector(vector)
        } else {
            self.user_vector(&format!("{prefix}.{vector}"))
        }
    }

    /// Bind `name` as ngspice `com_let` does: replace the vector `vec_get`
    /// finds, or add one to the current plot.
    pub(in crate::engine::control) fn bind_vector(
        &mut self,
        name: &str,
        value: VectorValue,
        line: usize,
    ) -> Result<(), ControlError> {
        let index = self.user_vector(name);
        let replaced = index.map_or(0, |index| {
            self.vectors.vectors[index]
                .value
                .samples
                .len()
                .saturating_mul(2)
        });
        let requested = self
            .retained_values
            .saturating_add(self.vectors.values() - replaced)
            .saturating_add(value.samples.len().saturating_mul(2));
        if requested > self.value_limit {
            return Err(ControlError::new(
                line,
                ControlErrorKind::ResourceLimit,
                format!("result values limit {} exceeded", self.value_limit),
            ));
        }
        match index {
            Some(index) => self.vectors.vectors[index].value = value,
            None => {
                let plot = self.current_dataset().map(|dataset| dataset.name.clone());
                self.vectors.vectors.push(UserVector {
                    name: name.to_owned(),
                    plot,
                    value,
                });
            }
        }
        Ok(())
    }

    /// A scalar `let` of a name drops the vector it would otherwise leave
    /// shadowing the new value.
    pub(in crate::engine::control) fn unbind_vector(&mut self, name: &str) {
        if let Some(index) = self.user_vector(name) {
            self.vectors.vectors.remove(index);
        }
    }

    /// Evaluate a control expression to a whole vector. Operands of different
    /// lengths are padded with their last element (frontend/evaluate.c
    /// `doop`), and the result follows the scale of its first full-length
    /// operand that has one.
    pub(in crate::engine::control) fn evaluate_vector(
        &self,
        expression: &Expr,
        variables: &ParamContext,
        line: usize,
        abort: &dyn AbortSignal,
    ) -> Result<VectorValue, ControlError> {
        let mut budget = self
            .value_limit
            .saturating_sub(self.retained_values)
            .saturating_sub(self.vectors.values());
        let mut bound = expression.clone();
        let mut leaves = Vec::new();
        let shape =
            self.bind_leaves(&mut bound, &mut leaves, &mut budget, variables, line, abort)?;
        bound
            .ensure_stack_safe_depth()
            .map_err(|error| command_error(line, error.to_string()))?;
        let length = leaves
            .iter()
            .map(|leaf: &VectorValue| leaf.samples.len())
            .max()
            .unwrap_or(1);
        charge(&mut budget, length, line)?;
        let keys: Vec<_> = (0..leaves.len())
            .map(|index| format!("\0V{index}"))
            .collect();
        let mut context = ParamContext::new();
        let mut samples = Vec::with_capacity(length);
        for row in 0..length {
            aborted(abort, line)?;
            for (leaf, key) in leaves.iter().zip(&keys) {
                let last = leaf.samples.len() - 1;
                context.set_complex(key, leaf.samples[row.min(last)]);
            }
            let value = evaluate_complex_with_functions_and_abort(
                &bound,
                &context,
                &mut |_| Ok(None),
                &mut |_, _| Ok(None),
                abort,
            )
            .map_err(|error| ControlError::evaluation(line, error))?;
            if !value.re.is_finite() || !value.im.is_finite() {
                return Err(ControlError::new(
                    line,
                    ControlErrorKind::Expression,
                    format!("vector expression is nonfinite at index {row}"),
                ));
            }
            samples.push(value);
        }
        // An operand padded to the result's length cannot lend it a scale.
        let scale = leaves
            .iter()
            .filter(|leaf| leaf.samples.len() == length)
            .chain(&leaves)
            .find_map(|leaf| leaf.scale.clone());
        Ok(VectorValue {
            samples,
            complex: shape.complex,
            scale,
            unit: shape.unit,
        })
    }

    /// Replace each vector operand with a placeholder bound per element, and
    /// snapshot scalar parameters into literals. NUL cannot occur in an
    /// authored identifier, so no user binding can capture a placeholder.
    fn bind_leaves(
        &self,
        expression: &mut Expr,
        leaves: &mut Vec<VectorValue>,
        budget: &mut usize,
        variables: &ParamContext,
        line: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Shape, ControlError> {
        aborted(abort, line)?;
        let shape = match expression {
            Expr::Number(_) => Shape::real(SignalUnit::Dimensionless),
            Expr::ComplexNumber(value) => Shape {
                unit: SignalUnit::Dimensionless,
                complex: value.im != 0.0,
            },
            Expr::Param(name) => {
                if let Some(index) = self.user_vector(name) {
                    let value = self.vectors.vectors[index].value.clone();
                    charge(budget, value.samples.len(), line)?;
                    return Ok(leaf(expression, value, leaves));
                }
                if let Ok((dataset, raw)) = self.qualified(name, line)
                    && let Ok(selected) = self.select(dataset, raw, None, line)
                {
                    let value = sampled(&selected, budget, line, abort)?;
                    return Ok(leaf(expression, value, leaves));
                }
                let value = variables.get_complex(name).ok_or_else(|| {
                    command_error(line, format!("vector or scalar '{name}' is unavailable"))
                })?;
                *expression = Expr::ComplexNumber(value);
                Shape {
                    unit: SignalUnit::Unspecified,
                    complex: value.im != 0.0,
                }
            }
            Expr::FnCall { name, args } => {
                if let Some(index) = self.voltage_alias(name, args) {
                    let value = self.vectors.vectors[index].value.clone();
                    charge(budget, value.samples.len(), line)?;
                    return Ok(leaf(expression, value, leaves));
                }
                if is_probe(name) {
                    let probe = name.rsplit('.').next().unwrap_or(name);
                    if matches!(probe, "V" | "N") && args.len() == 2 {
                        let mut first = Expr::FnCall {
                            name: name.clone(),
                            args: vec![args[0].clone()],
                        };
                        let mut second = Expr::FnCall {
                            name: name.clone(),
                            args: vec![args[1].clone()],
                        };
                        let a =
                            self.bind_leaves(&mut first, leaves, budget, variables, line, abort)?;
                        let b =
                            self.bind_leaves(&mut second, leaves, budget, variables, line, abort)?;
                        *expression = Expr::BinOp {
                            op: BinOpKind::Sub,
                            left: Box::new(first),
                            right: Box::new(second),
                        };
                        return Ok(Shape {
                            unit: binary_unit(BinOpKind::Sub, a.unit, b.unit),
                            complex: a.complex || b.complex,
                        });
                    }
                    let selected = self.direct(expression, variables, line, abort)?;
                    let value = sampled(&selected, budget, line, abort)?;
                    return Ok(leaf(expression, value, leaves));
                }
                // The functions whose control-vector semantics the print
                // path implements; each returns a real vector.
                let canonical = match name.as_str() {
                    "ABS" | "MAG" | "MAGNITUDE" => "ABS",
                    "REAL" | "RE" => "REAL",
                    "IMAG" | "IMG" | "IM" => "IMAG",
                    _ => {
                        return Err(command_error(
                            line,
                            format!("control vector function '{name}' is not supported"),
                        ));
                    }
                };
                if args.len() != 1 {
                    return Err(command_error(
                        line,
                        format!("{name} requires one vector expression"),
                    ));
                }
                let shape =
                    self.bind_leaves(&mut args[0], leaves, budget, variables, line, abort)?;
                *name = canonical.into();
                Shape::real(shape.unit)
            }
            Expr::BinOp { op, left, right } => {
                let a = self.bind_leaves(left, leaves, budget, variables, line, abort)?;
                let b = self.bind_leaves(right, leaves, budget, variables, line, abort)?;
                use BinOpKind::{And, Eq, Ge, Gt, Le, Lt, Ne, Or};
                // Relational results are real (evaluate.c doop `relflag`).
                let relational = matches!(op, Gt | Lt | Ge | Le | Eq | Ne | And | Or);
                Shape {
                    unit: binary_unit(*op, a.unit, b.unit),
                    complex: !relational && (a.complex || b.complex),
                }
            }
            Expr::UnaryOp { op, operand } => {
                let shape = self.bind_leaves(operand, leaves, budget, variables, line, abort)?;
                if *op == UnaryOpKind::Not {
                    Shape::real(SignalUnit::Dimensionless)
                } else {
                    shape
                }
            }
            Expr::StringLiteral(_) => {
                return Err(command_error(line, "a string is not a numeric vector"));
            }
        };
        Ok(shape)
    }

    /// One print or plot trace of an expression that names a `let` vector.
    /// Without `vs`, the trace runs against its scale when it has one of its
    /// length, and against the element index otherwise.
    pub(in crate::engine::control) fn vector_trace(
        &self,
        y: (&Expr, &str),
        x: Option<(&Expr, &str)>,
        variables: &ParamContext,
        line: usize,
        abort: &dyn AbortSignal,
    ) -> Result<ControlTrace, ControlError> {
        let value = self.evaluate_vector(y.0, variables, line, abort)?;
        let dataset = value
            .scale
            .clone()
            .or_else(|| self.current_dataset().map(|dataset| dataset.name.clone()))
            .unwrap_or_else(|| "const".into());
        let x = match x {
            Some((expression, label)) => {
                let x = self.evaluate_vector(expression, variables, line, abort)?;
                if x.samples.len() != value.samples.len() {
                    return Err(command_error(
                        line,
                        format!(
                            "'{label}' has {} values and '{}' has {}",
                            x.samples.len(),
                            y.1,
                            value.samples.len()
                        ),
                    ));
                }
                ControlVector {
                    expression: label.to_owned(),
                    dataset: x.scale.clone().unwrap_or_else(|| dataset.clone()),
                    unit: x.unit,
                    samples: x.samples,
                    current_sources: Vec::new(),
                    voltage_sources: Vec::new(),
                }
            }
            None => match value
                .scale
                .as_deref()
                .and_then(|name| self.dataset(name))
                .filter(|scale| scale.length() == value.samples.len())
            {
                Some(scale) => {
                    let mut samples = Vec::with_capacity(scale.length());
                    for row in 0..scale.length() {
                        aborted(abort, line)?;
                        let value = scale
                            .scale_value(row)
                            .filter(|value| value.is_finite())
                            .ok_or_else(|| {
                                command_error(line, "dataset scale is unavailable or nonfinite")
                            })?;
                        samples.push(value.into());
                    }
                    ControlVector {
                        expression: scale.scale_name().into(),
                        dataset: scale.name.clone(),
                        unit: scale.scale_unit(),
                        samples,
                        current_sources: Vec::new(),
                        voltage_sources: Vec::new(),
                    }
                }
                None => ControlVector {
                    expression: "index".into(),
                    dataset: dataset.clone(),
                    unit: SignalUnit::Dimensionless,
                    samples: (0..value.samples.len())
                        .map(|index| ComplexValue::from(index as Value))
                        .collect(),
                    current_sources: Vec::new(),
                    voltage_sources: Vec::new(),
                },
            },
        };
        Ok(ControlTrace {
            x,
            y: ControlVector {
                expression: y.1.to_owned(),
                dataset,
                unit: value.unit,
                samples: value.samples,
                current_sources: Vec::new(),
                voltage_sources: Vec::new(),
            },
        })
    }
}

/// Names the dataset resolver reads directly: probes and root references.
fn is_probe(name: &str) -> bool {
    matches!(
        name.rsplit('.').next().unwrap_or(name),
        "V" | "N" | "I" | "DNO" | "DNI" | "OUTPUT_IMPEDANCE_AT_V" | "POLE" | "ZERO" | "DISTO"
    )
}

fn leaf(expression: &mut Expr, value: VectorValue, leaves: &mut Vec<VectorValue>) -> Shape {
    *expression = Expr::Param(format!("\0V{}", leaves.len()));
    let shape = Shape {
        unit: value.unit.clone(),
        complex: value.complex,
    };
    leaves.push(value);
    shape
}

/// Every retained sample of a dataset vector, on that dataset's scale.
fn sampled(
    selected: &Selected<'_>,
    budget: &mut usize,
    line: usize,
    abort: &dyn AbortSignal,
) -> Result<VectorValue, ControlError> {
    let finite = |row| {
        selected
            .sample(row)
            .filter(|value| value.re.is_finite() && value.im.is_finite())
            .ok_or_else(|| unavailable(line, selected.dataset, &selected.id.signal))
    };
    if selected.is_scalar() {
        let value = finite(0)?;
        charge(budget, 1, line)?;
        return Ok(VectorValue {
            samples: vec![value],
            complex: value.im != 0.0,
            scale: None,
            unit: selected.unit.clone(),
        });
    }
    let length = selected.dataset.length();
    if length == 0 {
        return Err(unavailable(line, selected.dataset, &selected.id.signal));
    }
    charge(budget, length, line)?;
    let mut samples = Vec::with_capacity(length);
    for row in 0..length {
        aborted(abort, line)?;
        samples.push(finite(row)?);
    }
    let complex = !matches!(selected.column, Column::Scale)
        && (selected.dataset.is_complex() || samples.iter().any(|value| value.im != 0.0));
    Ok(VectorValue {
        samples,
        complex,
        scale: Some(selected.dataset.name.clone()),
        unit: selected.unit.clone(),
    })
}

fn charge(budget: &mut usize, samples: usize, line: usize) -> Result<(), ControlError> {
    let values = samples.saturating_mul(2);
    if values > *budget {
        return Err(ControlError::new(
            line,
            ControlErrorKind::ResourceLimit,
            "result values limit exceeded by a vector expression",
        ));
    }
    *budget -= values;
    Ok(())
}

fn aborted(abort: &dyn AbortSignal, line: usize) -> Result<(), ControlError> {
    if abort.is_aborted() {
        Err(ControlError::new(
            line,
            ControlErrorKind::Aborted,
            "control execution aborted",
        ))
    } else {
        Ok(())
    }
}
