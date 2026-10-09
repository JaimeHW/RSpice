//! Bind authored port groups to physical lanes in declaration order.
use super::*;

mod digital_words;
mod mixed_inputs;
mod real_buses;

#[derive(Debug, Clone)]
struct DigitalShape {
    range: Option<VectorBounds>,
    dimensions: Vec<VectorBounds>,
    width: u32,
    real: bool,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ConnectionScope {
    nodes: HashMap<SmolStr, NodeVector>,
    arrays: HashMap<SmolStr, super::arrays::NodeArray>,
    real_buses: HashMap<SmolStr, super::real_buses::RealBus>,
    physical: HashSet<SmolStr>,
    digital: HashMap<SmolStr, DigitalShape>,
    constants: DigitalConstants,
    time_scale: crate::time_scale::ModuleTimeScale,
}

impl ConnectionScope {
    pub fn new(source: &Module, analyzed: &AnalyzedModule) -> Self {
        let digital: HashMap<_, _> = analyzed
            .digital
            .signals
            .iter()
            .map(|signal| {
                (
                    signal.name.clone(),
                    DigitalShape {
                        range: signal.range,
                        dimensions: signal.dimensions.clone(),
                        width: signal.width,
                        real: signal.class.is_real(),
                    },
                )
            })
            .collect();
        let physical = analyzed
            .ports
            .iter()
            .filter(|port| !digital.contains_key(&port.name))
            .map(|port| port.name.clone())
            .chain(analyzed.internal_nodes.iter().map(|node| node.name.clone()))
            .chain(analyzed.ground_nodes.iter().cloned())
            .collect();
        Self {
            nodes: analyzed.physical_nodes.vectors.clone(),
            arrays: analyzed.physical_nodes.arrays.clone(),
            real_buses: analyzed.physical_nodes.real_buses.clone(),
            physical,
            digital,
            constants: DigitalConstants::from_module(source),
            time_scale: source.time_scale,
        }
    }

    pub fn contains_physical(&self, actual: &Expression) -> bool {
        fn elements(scope: &ConnectionScope, values: &[ArrayLiteralElement], depth: usize) -> bool {
            if depth > MAX_REPLICATION_NESTING {
                return true;
            }
            values.iter().any(|value| match value {
                ArrayLiteralElement::Value(value) => contains(scope, value, depth),
                ArrayLiteralElement::Replication(replication) => {
                    elements(scope, &replication.elements, depth + 1)
                }
            })
        }
        fn contains(scope: &ConnectionScope, actual: &Expression, depth: usize) -> bool {
            let name = match actual {
                Expression::Identifier(id) => &id.name,
                Expression::ArrayAccess(access) => &access.array,
                Expression::Digital(DigitalExpr::PartSelect(select)) => &select.name,
                Expression::Digital(DigitalExpr::ArraySelect(select)) => &select.name,
                Expression::ArrayLiteral(concat) => {
                    return elements(scope, &concat.elements, depth + 1);
                }
                _ => return false,
            };
            scope.nodes.contains_key(name)
                || scope.arrays.contains_key(name)
                || scope.physical.contains(name)
        }
        contains(self, actual, 0)
    }

    pub fn physical_selection(
        &self,
        actual: &Expression,
    ) -> CompileResult<Option<Vec<Expression>>> {
        if !self.contains_physical(actual) {
            return Ok(None);
        }
        let mut lanes = Vec::new();
        self.append(actual, &mut lanes, 0)?;
        Ok(Some(lanes))
    }

    fn coordinate(&self, expression: &Expression) -> CompileResult<i64> {
        integer(expression, &self.constants, self.time_scale)
    }

    fn selected(&self, name: &SmolStr, index: i64, span: Span) -> CompileResult<Expression> {
        if let Some(vector) = self.nodes.get(name) {
            Ok(Expression::Identifier(Identifier {
                name: vector.lane(index, span)?,
                span,
            }))
        } else {
            Ok(Expression::ArrayAccess(ArrayAccessExpr {
                normalized: false,
                packed: None,
                discrete_validity: None,
                array: name.clone(),
                index: Box::new(exact_integer_expression(index, span)),
                span,
            }))
        }
    }

    fn append_array(
        &self,
        actual: &Expression,
        output: &mut Vec<Expression>,
    ) -> CompileResult<bool> {
        let (name, prefix, select) = match actual {
            Expression::Identifier(id) => (&id.name, Vec::new(), None),
            Expression::ArrayAccess(access) => (
                &access.array,
                Vec::new(),
                Some(PackedSelect::Bit(access.index.clone())),
            ),
            Expression::Digital(DigitalExpr::PartSelect(select)) => (
                &select.name,
                Vec::new(),
                Some(PackedSelect::Part {
                    msb: select.msb.clone(),
                    lsb: select.lsb.clone(),
                }),
            ),
            Expression::Digital(DigitalExpr::ArraySelect(access)) => {
                let mut prefix = vec![*access.index.clone()];
                prefix.extend(access.additional_indices.clone());
                (&access.name, prefix, Some(access.select.clone()))
            }
            _ => return Ok(false),
        };
        let Some(array) = self.arrays.get(name) else {
            return Ok(false);
        };
        let (lanes, _) = array.select(
            &prefix,
            select.as_ref(),
            &self.constants,
            self.time_scale,
            actual.span(),
        )?;
        output.extend(lanes.into_iter().map(|name| {
            Expression::Identifier(Identifier {
                name,
                span: actual.span(),
            })
        }));
        Ok(true)
    }

    fn append(
        &self,
        actual: &Expression,
        output: &mut Vec<Expression>,
        depth: usize,
    ) -> CompileResult<()> {
        if depth > MAX_REPLICATION_NESTING {
            return Err(error(
                "port concatenation nesting exceeds the supported limit",
                actual.span(),
            ));
        }
        let span = actual.span();
        if !self.append_array(actual, output)? && !self.append_digital(actual, output)? {
            match actual {
                Expression::Identifier(id) => {
                    if let Some(vector) = self.nodes.get(&id.name) {
                        output.extend(vector.lanes.iter().map(|name| {
                            Expression::Identifier(Identifier {
                                name: name.clone(),
                                span,
                            })
                        }));
                    } else {
                        output.push(actual.clone());
                    }
                }
                Expression::ArrayAccess(access) if self.nodes.contains_key(&access.array) => {
                    output.push(self.selected(
                        &access.array,
                        self.coordinate(&access.index)?,
                        span,
                    )?);
                }
                Expression::Digital(DigitalExpr::PartSelect(select)) => {
                    let msb = self.coordinate(&select.msb)?;
                    let lsb = self.coordinate(&select.lsb)?;
                    let range = VectorBounds { msb, lsb };
                    if range.width() > MAX_DIGITAL_VECTOR_WIDTH {
                        return Err(error(
                            "physical port connection exceeds the lane limit",
                            span,
                        ));
                    }
                    if let Some(vector) = self.nodes.get(&select.name) {
                        if msb != lsb && (msb > lsb) != (vector.bounds.msb > vector.bounds.lsb) {
                            return Err(error(
                                "physical vector part-select direction disagrees with its declaration",
                                span,
                            ));
                        }
                    }
                    for index in range.indices_msb_first() {
                        output.push(self.selected(&select.name, index, span)?);
                    }
                }
                Expression::ArrayLiteral(concat) => {
                    if concat.assignment_pattern {
                        return Err(error(
                            "a physical port connection requires a concatenation, not an assignment pattern",
                            span,
                        ));
                    }
                    self.elements(&concat.elements, output, depth + 1)?;
                }
                _ => output.push(actual.clone()),
            }
        }
        if output.len() > MAX_DIGITAL_VECTOR_WIDTH as usize {
            return Err(error(
                "physical port connection exceeds the lane limit",
                span,
            ));
        }
        Ok(())
    }

    fn elements(
        &self,
        elements: &[ArrayLiteralElement],
        output: &mut Vec<Expression>,
        depth: usize,
    ) -> CompileResult<()> {
        for element in elements {
            match element {
                ArrayLiteralElement::Value(value) => self.append(value, output, depth)?,
                ArrayLiteralElement::Replication(replication) => {
                    if depth >= MAX_REPLICATION_NESTING {
                        return Err(error(
                            "port concatenation nesting exceeds the supported limit",
                            replication.count.span(),
                        ));
                    }
                    let count = self.coordinate(&replication.count)?;
                    if count <= 0 || count > i64::from(MAX_DIGITAL_VECTOR_WIDTH) {
                        return Err(error(
                            "port concatenation repetition must be positive and bounded",
                            replication.count.span(),
                        ));
                    }
                    let mut repeated = Vec::new();
                    self.elements(&replication.elements, &mut repeated, depth + 1)?;
                    let length = repeated
                        .len()
                        .checked_mul(count as usize)
                        .and_then(|length| length.checked_add(output.len()));
                    if length.is_none_or(|length| length > MAX_DIGITAL_VECTOR_WIDTH as usize) {
                        return Err(error(
                            "physical port concatenation exceeds the lane limit",
                            replication.count.span(),
                        ));
                    }
                    for _ in 0..count {
                        output.extend(repeated.iter().cloned());
                    }
                }
            }
        }
        Ok(())
    }
}

/// The returned list follows the analyzed scalar port order. Pure discrete
/// formals preserve their existing expression/assignment semantics.
pub(in crate::semantic) fn bind(
    instance: &ModuleInstance,
    child: &AnalyzedModule,
    parent: &ConnectionScope,
    path: &str,
) -> CompileResult<Vec<Option<Expression>>> {
    let named = instance
        .connections
        .iter()
        .any(|connection| matches!(connection, Connection::Named { .. }));
    let ordered = instance
        .connections
        .iter()
        .any(|connection| matches!(connection, Connection::Ordered { .. }));
    if named && ordered {
        return Err(error(
            format!("instance '{path}' mixes named and ordered port connections"),
            instance.span,
        ));
    }
    let fallback;
    let groups = if child.physical_nodes.ports.is_empty() {
        fallback = child
            .ports
            .iter()
            .map(|port| (port.name.clone(), vec![port.name.clone()]))
            .collect::<Vec<_>>();
        &fallback
    } else {
        &child.physical_nodes.ports
    };
    if ordered && instance.connections.len() > groups.len() {
        return Err(SemanticError::new(
            SemanticErrorKind::ArgumentCountMismatch {
                name: path.into(),
                expected: format!("at most {} port connections", groups.len()),
                got: instance.connections.len(),
            },
            instance.span,
        )
        .into());
    }
    let positions: HashMap<_, _> = child
        .ports
        .iter()
        .enumerate()
        .map(|(index, port)| (&port.name, index))
        .collect();
    let discrete: HashSet<_> = child
        .digital
        .signals
        .iter()
        .map(|signal| &signal.name)
        .collect();
    let mut bound = vec![None; child.ports.len()];
    let mut seen = HashSet::new();
    for (ordinal, connection) in instance.connections.iter().enumerate() {
        let (formal, lanes, actual, span) = match connection {
            Connection::Named { port, signal, span } => {
                if let Some((formal, lanes)) = groups.iter().find(|(name, _)| name == port) {
                    (formal, lanes.as_slice(), signal.as_ref(), *span)
                } else if positions.contains_key(port) {
                    // Internal prepared occurrences can name one expanded lane.
                    (port, std::slice::from_ref(port), signal.as_ref(), *span)
                } else {
                    return Err(SemanticError::new(
                        SemanticErrorKind::UndeclaredSymbol { name: port.clone() },
                        *span,
                    )
                    .into());
                }
            }
            Connection::Ordered { signal, span } => {
                let (formal, lanes) = &groups[ordinal];
                (formal, lanes.as_slice(), signal.as_ref(), *span)
            }
        };
        for lane in lanes {
            if !seen.insert(lane.clone()) {
                return Err(SemanticError::new(
                    SemanticErrorKind::DuplicateSymbol {
                        name: formal.clone(),
                        first_defined: span,
                    },
                    span,
                )
                .into());
            }
        }
        let Some(actual) = actual else { continue };
        if child.ports[positions[&lanes[0]]].direction != PortDirection::Input
            && matches!(actual, Expression::ArrayLiteral(concat) if concat.first_replication().is_some())
        {
            return Err(error(
                "replicated concatenations cannot connect to output or inout ports",
                actual.span(),
            ));
        }
        if lanes.len() == 1
            && discrete.contains(&lanes[0])
            && !child.physical_nodes.real_buses.contains_key(formal)
        {
            // Mixed input expressions retain their digital operands until the
            // connection planner has the parent's complete type environment.
            let mut real_lanes = Vec::new();
            if parent.append_real_bus(actual, &mut real_lanes)? {
                if real_lanes.len() != 1 {
                    return Err(error(
                        format!("scalar port '{path}.{formal}' requires one real bus lane"),
                        actual.span(),
                    ));
                }
                bound[positions[&lanes[0]]] = real_lanes.pop();
                continue;
            }
            bound[positions[&lanes[0]]] = Some(
                if child.ports[positions[&lanes[0]]].direction == PortDirection::Input
                    && matches!(actual, Expression::ArrayLiteral(_))
                {
                    actual.clone()
                } else {
                    match parent.physical_selection(actual)? {
                        Some(mut selected) if selected.len() == 1 => selected.remove(0),
                        _ => actual.clone(),
                    }
                },
            );
            continue;
        }
        let mut actuals = Vec::new();
        parent.append(actual, &mut actuals, 0)?;
        if actuals.len() != lanes.len() {
            return Err(error(
                format!(
                    "physical port '{path}.{formal}' requires {} lanes, but its connection supplies {}",
                    lanes.len(),
                    actuals.len()
                ),
                actual.span(),
            ));
        }
        for (lane, actual) in lanes.iter().zip(actuals) {
            bound[positions[lane]] = Some(actual);
        }
    }
    Ok(bound)
}
