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
    /// The owning plot's name, a dataset's or one `setplot new` made; `None`
    /// is ngspice's constants plot, which holds the vectors bound before any
    /// analysis ran.
    plot: Option<String>,
    value: VectorValue,
    /// A scalar `let` keeps its session parameter, which answers every read
    /// that does not name a plot; this copy answers `plot.name`.
    scalar: bool,
}

/// A plot `setplot new` made (frontend/vectors.c:1164). It holds only the
/// vectors bound in it, and the first of them is its scale (vec_new).
#[derive(Debug)]
struct UserPlot {
    name: String,
    scale: Option<String>,
}

#[derive(Debug, Default)]
pub(in crate::engine::control) struct VectorStore {
    vectors: Vec<UserVector>,
    plots: Vec<UserPlot>,
    /// The user plot that is current, if `setplot` chose one last.
    current: Option<usize>,
}

impl VectorStore {
    /// The current plot's name when it is a user plot.
    pub(in crate::engine::control) fn current_plot(&self) -> Option<&str> {
        self.current.map(|index| self.plots[index].name.as_str())
    }

    /// A dataset became the current plot.
    pub(in crate::engine::control) fn leave_plot(&mut self) {
        self.current = None;
    }

    /// `setplot new`: an empty plot of this name becomes current.
    pub(in crate::engine::control) fn new_plot(&mut self, name: String) {
        self.plots.push(UserPlot { name, scale: None });
        self.current = Some(self.plots.len() - 1);
    }

    /// `setplot name` of a user plot; false when there is none of that name.
    pub(in crate::engine::control) fn select_plot(&mut self, name: &str) -> bool {
        let index = self
            .plots
            .iter()
            .position(|plot| plot.name.eq_ignore_ascii_case(name));
        if index.is_some() {
            self.current = index;
        }
        index.is_some()
    }

    /// The plots `setplot new` made, oldest first.
    pub(in crate::engine::control) fn plot_names(&self) -> Vec<String> {
        self.plots.iter().map(|plot| plot.name.clone()).collect()
    }

    /// Drop the vectors bound in a plot.
    pub(in crate::engine::control) fn remove_vectors_of(&mut self, plot: &str) {
        self.vectors.retain(|vector| {
            !vector
                .plot
                .as_deref()
                .is_some_and(|owner| owner.eq_ignore_ascii_case(plot))
        });
    }

    /// `destroy` of a plot `setplot new` made; false when there is none.
    pub(in crate::engine::control) fn remove_plot(&mut self, name: &str) -> bool {
        let Some(index) = self
            .plots
            .iter()
            .position(|plot| plot.name.eq_ignore_ascii_case(name))
        else {
            return false;
        };
        let plot = self.plots.remove(index);
        self.remove_vectors_of(&plot.name);
        self.current = match self.current {
            Some(current) if current == index => None,
            Some(current) if current > index => Some(current - 1),
            current => current,
        };
        true
    }

    pub(in crate::engine::control) fn has_plot(&self, name: &str) -> bool {
        self.plot(name).is_some()
    }

    fn plot(&self, name: &str) -> Option<&str> {
        self.plots
            .iter()
            .find(|plot| plot.name.eq_ignore_ascii_case(name))
            .map(|plot| plot.name.as_str())
    }

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
    /// ngspice `$curplot` (frontend/options.c:72): the current plot's name,
    /// `const` before any plot exists.
    pub(in crate::engine::control) fn plot_name(&self) -> String {
        self.current_plot().unwrap_or("const").to_owned()
    }

    /// The current plot's name; `None` before any plot exists, when ngspice
    /// binds vectors in its constants plot.
    fn current_plot(&self) -> Option<&str> {
        self.vectors
            .current_plot()
            .or_else(|| self.current_dataset().map(|dataset| dataset.name.as_str()))
    }

    /// ngspice `vec_get`: a plot prefix names the plot, otherwise the current
    /// plot; the constants plot answers for either when it has the name. A
    /// scalar `let`'s copy answers only a read that names its plot.
    fn user_vector(&self, name: &str) -> Option<usize> {
        self.find_vector(name, false)
    }

    /// The vector bound by `let` that `settype` names, with the id it
    /// reports; com_stype (frontend/typesdef.c) types the vector itself.
    pub(in crate::engine::control) fn user_vector_id(
        &self,
        name: &str,
    ) -> Option<(usize, ControlVectorId)> {
        let index = self.find_vector(name, true)?;
        let vector = &self.vectors.vectors[index];
        let dataset = vector.plot.clone().unwrap_or_else(|| "const".into());
        let signal = vector.name.clone();
        Some((index, ControlVectorId { dataset, signal }))
    }

    pub(in crate::engine::control) fn set_user_vector_unit(
        &mut self,
        index: usize,
        unit: &SignalUnit,
    ) {
        self.vectors.vectors[index].value.unit = unit.clone();
    }

    /// `vec_get` over every vector, a scalar `let`'s copy included.
    fn find_vector(&self, name: &str, scalars: bool) -> Option<usize> {
        let (plot, name) = match name.split_once('.') {
            Some((prefix, rest)) if prefix.eq_ignore_ascii_case("const") => (Some(None), rest),
            Some((prefix, rest)) => match self
                .dataset(prefix)
                .map(|dataset| dataset.name.as_str())
                .or_else(|| self.vectors.plot(prefix))
            {
                Some(plot) => (Some(Some(plot)), rest),
                None => (None, name),
            },
            None => (None, name),
        };
        let scalars = scalars || plot.is_some();
        let plot = plot.unwrap_or_else(|| self.current_plot());
        let find = |plot: Option<&str>| {
            self.vectors.vectors.iter().position(|vector| {
                vector.plot.as_deref() == plot
                    && vector.name.eq_ignore_ascii_case(name)
                    && (scalars || !vector.scalar)
            })
        };
        find(plot).or_else(|| find(None))
    }

    /// Whether only the vector evaluator can evaluate the expression: it
    /// reads a `let` vector or a constant, or calls a function the dataset
    /// resolver lacks, or a plot `setplot new` made is current, where the
    /// dataset resolver has no dataset to read.
    pub(in crate::engine::control) fn needs_vector_path(
        &self,
        expression: &Expr,
        variables: &ParamContext,
    ) -> bool {
        self.vectors.current_plot().is_some()
            || self.names_user_vector(expression)
            || self.names_constant(expression, variables)
            || needs_vector_functions(expression, variables)
    }

    /// Whether the expression reads one of ngspice's constants, which a
    /// dataset vector or a parameter of the same name would hide.
    fn names_constant(&self, expression: &Expr, variables: &ParamContext) -> bool {
        match expression {
            Expr::Param(name) => {
                constant(name).is_some()
                    && variables.get_complex(name).is_none()
                    && !self
                        .qualified(name, 0)
                        .is_ok_and(|(dataset, raw)| self.select(dataset, raw, None, 0).is_ok())
            }
            Expr::FnCall { name, args } => {
                !is_probe(name) && args.iter().any(|arg| self.names_constant(arg, variables))
            }
            Expr::BinOp { left, right, .. } => {
                self.names_constant(left, variables) || self.names_constant(right, variables)
            }
            Expr::UnaryOp { operand, .. } => self.names_constant(operand, variables),
            Expr::Number(_) | Expr::ComplexNumber(_) | Expr::StringLiteral(_) => false,
        }
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
    /// it names a `let` vector, a scalar one too, before a node of the same
    /// name.
    fn voltage_alias(&self, name: &str, args: &[Expr]) -> Option<usize> {
        let [Expr::Param(vector)] = args else {
            return None;
        };
        let (prefix, probe) = name.rsplit_once('.').unwrap_or(("", name));
        if probe != "V" {
            return None;
        }
        if prefix.is_empty() {
            self.find_vector(vector, true)
        } else {
            self.find_vector(&format!("{prefix}.{vector}"), true)
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
        self.bind(name, value, false, line)
    }

    /// A scalar `let` stays a session parameter; ngspice also binds it as a
    /// one-element vector of the current plot, which `plot.name` reads.
    pub(in crate::engine::control) fn bind_scalar(
        &mut self,
        name: &str,
        value: ComplexValue,
        line: usize,
    ) -> Result<(), ControlError> {
        if !value.re.is_finite() || !value.im.is_finite() {
            // The session refuses the value; nothing is bound.
            return Ok(());
        }
        let value = VectorValue {
            samples: vec![value],
            complex: value.im != 0.0,
            scale: None,
            unit: SignalUnit::Unspecified,
        };
        self.bind(name, value, true, line)
    }

    fn bind(
        &mut self,
        name: &str,
        value: VectorValue,
        scalar: bool,
        line: usize,
    ) -> Result<(), ControlError> {
        let index = self.find_vector(name, true);
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
            Some(index) => {
                let vector = &mut self.vectors.vectors[index];
                vector.value = value;
                vector.scalar = scalar;
            }
            None => self.push_vector(name, value, scalar),
        }
        Ok(())
    }

    /// `let name[index] = expression` (frontend/com_let.c:115): the elements
    /// the index selects of the vector `vec_get` finds take the expression's
    /// values, which must fill them exactly; a real vector cannot take
    /// complex values. Returns the new element of a scalar `let`.
    pub(in crate::engine::control) fn assign_elements(
        &mut self,
        name: &str,
        index: &str,
        expression: &str,
        variables: &ParamContext,
        line: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Option<ComplexValue>, ControlError> {
        let target = self.find_vector(name, true).ok_or_else(|| {
            command_error(
                line,
                format!("'{name}' is no vector; when creating a new vector, it cannot be indexed"),
            )
        })?;
        let length = self.vectors.vectors[target].value.samples.len();
        let (low, high) = self.element_range(index, length, variables, line, abort)?;
        let value = match self.vector_or_scalar(expression, variables, line, abort)? {
            Evaluated::Scalar(value) => VectorValue {
                samples: vec![value],
                complex: value.im != 0.0,
                scale: None,
                unit: SignalUnit::Unspecified,
            },
            Evaluated::Vector { value, .. } => value,
        };
        let required = high - low + 1;
        if value.samples.len() != required {
            return Err(command_error(
                line,
                format!(
                    "data for an index vector must fit exactly: the indexed range required {required} \
                     element{}, but {} {} supplied",
                    if required == 1 { "" } else { "s" },
                    value.samples.len(),
                    if value.samples.len() == 1 {
                        "was"
                    } else {
                        "were"
                    },
                ),
            ));
        }
        let vector = &mut self.vectors.vectors[target].value;
        if !vector.complex && value.complex {
            return Err(command_error(
                line,
                "complex data cannot be used to fill an array of real data",
            ));
        }
        vector.samples[low..=high].copy_from_slice(&value.samples);
        Ok(self.vectors.vectors[target]
            .scalar
            .then(|| self.vectors.vectors[target].value.samples[0]))
    }

    /// ngspice `find_indices` (com_let.c:302) for a vector of one dimension:
    /// `[i]` or `[low:high]`, where a blank low is 0 and a blank high the
    /// last element.
    fn element_range(
        &self,
        index: &str,
        length: usize,
        variables: &ParamContext,
        line: usize,
        abort: &dyn AbortSignal,
    ) -> Result<(usize, usize), ControlError> {
        let (text, rest) = index
            .split_once(']')
            .ok_or_else(|| command_error(line, "the ']' for dimension 1 could not be found"))?;
        if text.contains(',') || rest.trim_start().starts_with('[') {
            return Err(command_error(
                line,
                "too many dimensions given; only 1 is present",
            ));
        }
        if !rest.trim().is_empty() {
            return Err(command_error(
                line,
                format!("invalid text after the index: '{}'", rest.trim()),
            ));
        }
        let element = |text: &str| self.element_index(text, variables, line, abort);
        let (low, high) = match text.split_once(':') {
            None => {
                let index =
                    element(text)?.ok_or_else(|| command_error(line, "the index is empty"))?;
                (index, index)
            }
            Some((low, high)) => (
                element(low)?.unwrap_or(0),
                element(high)?.unwrap_or(length.saturating_sub(1)),
            ),
        };
        if low > high {
            return Err(command_error(
                line,
                format!("low range ({low}) is greater than high range ({high})"),
            ));
        }
        // ngspice checks only a range; past the end it writes out of bounds.
        if high >= length {
            return Err(command_error(
                line,
                format!(
                    "high range ({high}) exceeds the maximum value ({})",
                    length as isize - 1
                ),
            ));
        }
        Ok((low, high))
    }

    /// ngspice `get_one_index_value` (com_let.c:497): `None` for blank text,
    /// else a real scalar rounded to the nearest integer, never negative.
    fn element_index(
        &self,
        text: &str,
        variables: &ParamContext,
        line: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Option<usize>, ControlError> {
        if text.trim().is_empty() {
            return Ok(None);
        }
        let value = match self.vector_or_scalar(text, variables, line, abort)? {
            Evaluated::Scalar(value) => Some(value),
            Evaluated::Vector { value, .. } => match value.samples.as_slice() {
                [sample] if !value.complex => Some(*sample),
                _ => None,
            },
        };
        let Some(value) = value.filter(|value| value.im == 0.0 && value.re.is_finite()) else {
            return Err(command_error(
                line,
                format!("index expression '{}' is not a real scalar", text.trim()),
            ));
        };
        let index = (value.re + 0.5).floor();
        if index < 0.0 {
            return Err(command_error(
                line,
                format!("negative index ({index}) is not allowed"),
            ));
        }
        // A huge index saturates and fails the range check.
        Ok(Some(index as usize))
    }

    /// `compose`: replace the vector of this name in the current plot only
    /// (com_compose.c:83 vec_remove), or add one there.
    pub(in crate::engine::control) fn compose_vector(
        &mut self,
        name: &str,
        value: VectorValue,
        line: usize,
    ) -> Result<(), ControlError> {
        let plot = self.current_plot().map(str::to_owned);
        if let Some(index) = self
            .vectors
            .vectors
            .iter()
            .position(|vector| vector.plot == plot && vector.name.eq_ignore_ascii_case(name))
        {
            self.vectors.vectors.remove(index);
        }
        let requested = self
            .retained_values
            .saturating_add(self.vectors.values())
            .saturating_add(value.samples.len().saturating_mul(2));
        if requested > self.value_limit {
            return Err(ControlError::new(
                line,
                ControlErrorKind::ResourceLimit,
                format!("result values limit {} exceeded", self.value_limit),
            ));
        }
        self.push_vector(name, value, false);
        Ok(())
    }

    /// Add a vector to the current plot. A user plot's first vector is its
    /// scale (frontend/vectors.c:921 vec_new).
    fn push_vector(&mut self, name: &str, value: VectorValue, scalar: bool) {
        let plot = self.current_plot().map(str::to_owned);
        if let Some(index) = self.vectors.current
            && self.vectors.plots[index].scale.is_none()
        {
            self.vectors.plots[index].scale = Some(name.to_owned());
        }
        self.vectors.vectors.push(UserVector {
            name: name.to_owned(),
            plot,
            value,
            scalar,
        });
    }

    /// The scale of the current user plot, when it has `length` elements.
    fn user_plot_scale(&self, length: usize) -> Option<&UserVector> {
        let plot = &self.vectors.plots[self.vectors.current?];
        let scale = plot.scale.as_deref()?;
        self.vectors
            .vectors
            .iter()
            .find(|vector| {
                vector.plot.as_deref() == Some(plot.name.as_str())
                    && vector.name.eq_ignore_ascii_case(scale)
            })
            .filter(|vector| vector.value.samples.len() == length)
    }

    /// `$&name`: the vector a `let` or dataset vector expression evaluates
    /// to, as ngspice's words for it (frontend/options.c:132
    /// `cp_enqvec_as_var`): each element's real part, printed `%G`.
    pub(in crate::engine::control) fn expand_vector(
        &self,
        name: &str,
        variables: &ParamContext,
        line: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Option<Vec<String>>, ControlError> {
        // ngspice looks the name up (vectors.c `vec_get`); it does not
        // evaluate an index, so `$&w[2]` names no vector.
        if name.contains('[') && !name.starts_with('@') {
            return Ok(None);
        }
        let Ok(parsed) = parse_control_expression_with_abort(name, abort) else {
            aborted(abort, line)?;
            return Ok(None);
        };
        match self.evaluate_vector(&parsed, variables, line, abort) {
            Ok(value) => Ok(Some(
                value
                    .samples
                    .iter()
                    .map(|sample| crate::engine::control::shell::percent_g(sample.re))
                    .collect(),
            )),
            Err(error)
                if matches!(
                    error.kind,
                    ControlErrorKind::Expression | ControlErrorKind::Host
                ) =>
            {
                Ok(None)
            }
            Err(error) => Err(error),
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
        let mut inlined = expression.clone();
        let mut nodes = MAX_INLINED_NODES;
        inline_functions(&mut inlined, variables, 0, &mut nodes, line, abort)?;
        self.vector_value(&inlined, &mut budget, variables, line, abort)
    }

    /// The vector an expression whose user functions are expanded evaluates
    /// to, charging its operands and result against `budget`.
    fn vector_value(
        &self,
        expression: &Expr,
        budget: &mut usize,
        variables: &ParamContext,
        line: usize,
        abort: &dyn AbortSignal,
    ) -> Result<VectorValue, ControlError> {
        let mut bound = expression.clone();
        let mut leaves = Vec::new();
        let shape = self.bind_leaves(&mut bound, &mut leaves, budget, variables, line, abort)?;
        bound
            .ensure_stack_safe_depth()
            .map_err(|error| command_error(line, error.to_string()))?;
        if leaves.len() == 1 && matches!(&bound, Expr::Param(name) if name == "\0V0") {
            return Ok(leaves.remove(0));
        }
        let length = leaves
            .iter()
            .map(|leaf: &VectorValue| leaf.samples.len())
            .max()
            .unwrap_or(1);
        charge(budget, length, line)?;
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
                let value = variables
                    .get_complex(name)
                    .or_else(|| constant(name))
                    .ok_or_else(|| {
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
                if name == "[]" {
                    let value = self.index_vector(args, budget, variables, line, abort)?;
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
                if functions::is_vector_function(name) {
                    let [argument] = args.as_slice() else {
                        return Err(command_error(
                            line,
                            format!("{name} requires one vector expression"),
                        ));
                    };
                    let argument = self.vector_value(argument, budget, variables, line, abort)?;
                    let value = functions::apply(self, name, argument, line, abort)?;
                    charge(budget, value.samples.len(), line)?;
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

    /// `x[i]` and `x[lo,hi]` (frontend/evaluate.c:697 `op_ind`): a bound
    /// rounds to the nearest integer and is clamped into the vector (ngspice
    /// warns and clamps), a range with `hi < lo` runs backwards, and the
    /// result has no scale. A complex index `x[lo+hi*j]` is a range too.
    fn index_vector(
        &self,
        args: &[Expr],
        budget: &mut usize,
        variables: &ParamContext,
        line: usize,
        abort: &dyn AbortSignal,
    ) -> Result<VectorValue, ControlError> {
        let [operand, bounds @ ..] = args else {
            return Err(command_error(line, "an index needs a vector"));
        };
        let vector = self.vector_value(operand, budget, variables, line, abort)?;
        if vector.samples.len() <= 1 {
            return Err(command_error(line, "indexing a scalar"));
        }
        let mut bound = |expression: &Expr| -> Result<(ComplexValue, bool), ControlError> {
            let value = self.vector_value(expression, budget, variables, line, abort)?;
            match value.samples.as_slice() {
                [sample] => Ok((*sample, value.complex)),
                _ => Err(command_error(line, "an index must be of length 1")),
            }
        };
        let (low, high) = match bounds {
            [index] => match bound(index)? {
                (value, true) => (value.re, value.im),
                (value, false) => (value.re, value.re),
            },
            [low, high] => (bound(low)?.0.re, bound(high)?.0.re),
            _ => return Err(command_error(line, "an index takes one or two bounds")),
        };
        let last = (vector.samples.len() - 1) as Value;
        let clamp = |value: Value| (value + 0.5).floor().clamp(0.0, last) as usize;
        let (low, high) = (clamp(low), clamp(high));
        let samples = if low <= high {
            vector.samples[low..=high].to_vec()
        } else {
            vector.samples[high..=low].iter().rev().copied().collect()
        };
        Ok(VectorValue {
            samples,
            complex: vector.complex,
            scale: None,
            unit: vector.unit,
        })
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
        let dataset = self.vector_dataset(&value);
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
            None => self.vector_scale(&value, &dataset, line, abort)?,
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

    /// The plot a vector is drawn from: its scale's, else the current one.
    pub(in crate::engine::control) fn vector_dataset(&self, value: &VectorValue) -> String {
        value
            .scale
            .clone()
            .or_else(|| self.current_plot().map(str::to_owned))
            .unwrap_or_else(|| "const".into())
    }

    /// The scale a vector follows (`v_scale`): its plot's, when the lengths
    /// agree, else a user plot's of its length, else its indices.
    pub(in crate::engine::control) fn vector_scale(
        &self,
        value: &VectorValue,
        dataset: &str,
        line: usize,
        abort: &dyn AbortSignal,
    ) -> Result<ControlVector, ControlError> {
        Ok(
            match value
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
                None => match self.user_plot_scale(value.samples.len()) {
                    Some(scale) => ControlVector {
                        expression: scale.name.clone(),
                        dataset: dataset.to_owned(),
                        unit: scale.value.unit.clone(),
                        samples: scale.value.samples.clone(),
                        current_sources: Vec::new(),
                        voltage_sources: Vec::new(),
                    },
                    None => ControlVector {
                        expression: "index".into(),
                        dataset: dataset.to_owned(),
                        unit: SignalUnit::Dimensionless,
                        samples: (0..value.samples.len())
                            .map(|index| ComplexValue::from(index as Value))
                            .collect(),
                        current_sources: Vec::new(),
                        voltage_sources: Vec::new(),
                    },
                },
            },
        )
    }
}

/// Calls of user functions nest at most this deep, and their expansion holds
/// at most this many expression nodes.
const MAX_INLINE_DEPTH: usize = 64;
const MAX_INLINED_NODES: usize = 1 << 16;

/// Whether the expression calls a user function, one of ngspice's predefined
/// ones or a whole-vector function, which only the vector evaluator knows.
fn needs_vector_functions(expression: &Expr, variables: &ParamContext) -> bool {
    match expression {
        Expr::FnCall { name, args } => {
            name == "[]"
                || variables.has_function(name)
                || functions::is_vector_function(name)
                || builtin_function(name, args.len()).is_some()
                || args
                    .iter()
                    .any(|arg| needs_vector_functions(arg, variables))
        }
        Expr::BinOp { left, right, .. } => {
            needs_vector_functions(left, variables) || needs_vector_functions(right, variables)
        }
        Expr::UnaryOp { operand, .. } => needs_vector_functions(operand, variables),
        Expr::Number(_) | Expr::ComplexNumber(_) | Expr::Param(_) | Expr::StringLiteral(_) => false,
    }
}

/// The functions ngspice defines at startup (frontend/cpitf.c:52).
fn builtin_function(name: &str, arity: usize) -> Option<(&'static [&'static str], &'static str)> {
    const ONE: &[&str] = &["X"];
    const TWO: &[&str] = &["X", "Y"];
    Some(match (name, arity) {
        ("MAX", 2) => (TWO, "(x > y) * x + (x <= y) * y"),
        ("MIN", 2) => (TWO, "(x < y) * x + (x >= y) * y"),
        ("VDB", 1) => (ONE, "db(v(x))"),
        ("VDB", 2) => (TWO, "db(v(x) - v(y))"),
        ("VI", 1) => (ONE, "im(v(x))"),
        ("VI", 2) => (TWO, "im(v(x) - v(y))"),
        ("VM", 1) => (ONE, "mag(v(x))"),
        ("VM", 2) => (TWO, "mag(v(x) - v(y))"),
        ("VP", 1) => (ONE, "ph(v(x))"),
        ("VP", 2) => (TWO, "ph(v(x) - v(y))"),
        ("VR", 1) => (ONE, "re(v(x))"),
        ("VR", 2) => (TWO, "re(v(x) - v(y))"),
        _ => return None,
    })
}

/// ngspice's constants plot (frontend/cpitf.c:40). The ngspice-46 release
/// prints boltz = 1.380649e-23, so its charge, Boltzmann and Planck
/// constants are the exact SI 2019 values.
fn constant(name: &str) -> Option<ComplexValue> {
    Some(match name.to_ascii_lowercase().as_str() {
        "yes" | "true" => 1.0.into(),
        "no" | "false" => 0.0.into(),
        "pi" => std::f64::consts::PI.into(),
        "e" => std::f64::consts::E.into(),
        "c" => 299_792_458.0.into(),
        "i" => ComplexValue::new(0.0, 1.0),
        "kelvin" => (-273.15).into(),
        "echarge" => 1.602_176_634e-19.into(),
        "boltz" => 1.380_649e-23.into(),
        "planck" => 6.626_070_15e-34.into(),
        _ => return None,
    })
}

/// Expand each call of a user function in place. ngspice substitutes the
/// argument trees for the formals (frontend/define.c:299 `trcopy`), so an
/// argument stays the whole vector it is.
fn inline_functions(
    expression: &mut Expr,
    variables: &ParamContext,
    depth: usize,
    nodes: &mut usize,
    line: usize,
    abort: &dyn AbortSignal,
) -> Result<(), ControlError> {
    aborted(abort, line)?;
    match expression {
        Expr::FnCall { name, args } => {
            for arg in args.iter_mut() {
                inline_functions(arg, variables, depth, nodes, line, abort)?;
            }
            let (formals, text) = match variables.get_function(name) {
                Some(function) => (function.args.clone(), function.body.as_str()),
                None => match builtin_function(name, args.len()) {
                    Some((formals, body)) => (
                        formals.iter().map(|formal| (*formal).to_owned()).collect(),
                        body,
                    ),
                    None => return Ok(()),
                },
            };
            if depth >= MAX_INLINE_DEPTH {
                return Err(command_error(
                    line,
                    format!("user function '{name}' nests deeper than {MAX_INLINE_DEPTH} calls"),
                ));
            }
            if formals.len() != args.len() {
                return Err(command_error(
                    line,
                    format!(
                        "user function '{name}' takes {} arguments, not {}",
                        formals.len(),
                        args.len()
                    ),
                ));
            }
            let text = text.trim();
            let text = text
                .strip_prefix('{')
                .and_then(|text| text.strip_suffix('}'))
                .unwrap_or(text);
            let mut body = parse_control_expression_with_abort(text, abort)
                .map_err(|error| ControlError::evaluation(line, error.into()))?;
            substitute_formals(&mut body, &formals, args, nodes, line)?;
            inline_functions(&mut body, variables, depth + 1, nodes, line, abort)?;
            body.ensure_stack_safe_depth()
                .map_err(|error| command_error(line, error.to_string()))?;
            *expression = body;
        }
        Expr::BinOp { left, right, .. } => {
            inline_functions(left, variables, depth, nodes, line, abort)?;
            inline_functions(right, variables, depth, nodes, line, abort)?;
        }
        Expr::UnaryOp { operand, .. } => {
            inline_functions(operand, variables, depth, nodes, line, abort)?
        }
        Expr::Number(_) | Expr::ComplexNumber(_) | Expr::Param(_) | Expr::StringLiteral(_) => {}
    }
    Ok(())
}

fn substitute_formals(
    body: &mut Expr,
    formals: &[String],
    actuals: &[Expr],
    nodes: &mut usize,
    line: usize,
) -> Result<(), ControlError> {
    *nodes = nodes
        .checked_sub(1)
        .ok_or_else(|| command_error(line, "user function expansion exceeds its size limit"))?;
    match body {
        Expr::Param(name) => {
            if let Some(index) = formals
                .iter()
                .position(|formal| formal.eq_ignore_ascii_case(name))
            {
                let actual = actuals[index].clone();
                *nodes = nodes.checked_sub(node_count(&actual)).ok_or_else(|| {
                    command_error(line, "user function expansion exceeds its size limit")
                })?;
                *body = actual;
            }
        }
        Expr::FnCall { args, .. } => {
            for arg in args {
                substitute_formals(arg, formals, actuals, nodes, line)?;
            }
        }
        Expr::BinOp { left, right, .. } => {
            substitute_formals(left, formals, actuals, nodes, line)?;
            substitute_formals(right, formals, actuals, nodes, line)?;
        }
        Expr::UnaryOp { operand, .. } => {
            substitute_formals(operand, formals, actuals, nodes, line)?
        }
        Expr::Number(_) | Expr::ComplexNumber(_) | Expr::StringLiteral(_) => {}
    }
    Ok(())
}

fn node_count(expression: &Expr) -> usize {
    match expression {
        Expr::FnCall { args, .. } => args
            .iter()
            .fold(1usize, |count, arg| count.saturating_add(node_count(arg))),
        Expr::BinOp { left, right, .. } => node_count(left)
            .saturating_add(node_count(right))
            .saturating_add(1),
        Expr::UnaryOp { operand, .. } => node_count(operand).saturating_add(1),
        Expr::Number(_) | Expr::ComplexNumber(_) | Expr::Param(_) | Expr::StringLiteral(_) => 1,
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
