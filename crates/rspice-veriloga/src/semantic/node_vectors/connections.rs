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

    /// Whether `name` is a continuous node, node vector or node array here.
    pub fn is_physical(&self, name: &str) -> bool {
        self.nodes.contains_key(name)
            || self.arrays.contains_key(name)
            || self.physical.contains(name)
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

    /// One element's share of a discrete-domain connection to an array of
    /// instances (IEEE 1364-2005 section 12.1.3): the whole connection when
    /// it is as wide as the port, otherwise the element's `port_width` bits,
    /// the leftmost element taking the leftmost bits.
    fn array_slice(
        &self,
        actual: &Expression,
        port_width: u32,
        (position, count): (u32, u32),
        port: &str,
    ) -> CompileResult<Expression> {
        let vector = |name: &SmolStr| {
            self.digital
                .get(name)
                .filter(|shape| shape.dimensions.is_empty() && !shape.real)
        };
        let (name, bounds, span) = match actual {
            Expression::Identifier(identifier) if vector(&identifier.name).is_some() => (
                identifier.name.clone(),
                vector(&identifier.name)
                    .and_then(|shape| shape.range)
                    .unwrap_or(VectorBounds::SCALAR),
                identifier.span,
            ),
            Expression::ArrayAccess(access) if vector(&access.array).is_some() => {
                let index = self.coordinate(&access.index)?;
                (
                    access.array.clone(),
                    VectorBounds {
                        msb: index,
                        lsb: index,
                    },
                    access.span,
                )
            }
            Expression::Digital(crate::ast::DigitalExpr::PartSelect(select))
                if vector(&select.name).is_some() =>
            {
                (
                    select.name.clone(),
                    VectorBounds {
                        msb: self.coordinate(&select.msb)?,
                        lsb: self.coordinate(&select.lsb)?,
                    },
                    select.span,
                )
            }
            _ => {
                return Err(SemanticError::new(
                    SemanticErrorKind::UnsupportedFeature(format!(
                        "instance array connection: port '{port}' of an array of instances is \
                         connected to an expression; IEEE 1364-2005 section 12.1.3 shares it \
                         out by width, which this compiler does for a net or a constant \
                         select of one"
                    )),
                    actual.span(),
                )
                .into());
            }
        };
        let width = bounds.width();
        if width == port_width {
            return Ok(actual.clone());
        }
        if u64::from(width) != u64::from(port_width) * u64::from(count) {
            return Err(error(
                format!(
                    "port '{port}' of an array of {count} instances is {port_width} bits and its \
                     connection is {width}; IEEE 1364-2005 section 12.1.3 requires the \
                     connection to be as wide as the port or as all the ports together"
                ),
                span,
            ));
        }
        let coordinate = |bit: u32| {
            if bounds.msb >= bounds.lsb {
                bounds.lsb + i64::from(bit)
            } else {
                bounds.lsb - i64::from(bit)
            }
        };
        let low = (count - 1 - position) * port_width;
        Ok(Expression::Digital(crate::ast::DigitalExpr::PartSelect(
            crate::ast::PartSelectExpr {
                name,
                msb: Box::new(exact_integer_expression(
                    coordinate(low + port_width - 1),
                    span,
                )),
                lsb: Box::new(exact_integer_expression(coordinate(low), span)),
                span,
            },
        )))
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
    let element = match instance.array {
        None => None,
        Some(crate::ast::InstanceArray::Element { position, count }) => Some((position, count)),
        Some(crate::ast::InstanceArray::Range { .. }) => {
            return Err(error(
                format!("instance array '{path}' reached elaboration unexpanded"),
                instance.span,
            ));
        }
    };
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
        let discrete_lane = lanes.len() == 1
            && discrete.contains(&lanes[0])
            && !child.physical_nodes.real_buses.contains_key(formal);
        let sliced;
        let actual = match (element, discrete_lane) {
            (Some(element), true) => {
                let width = child
                    .digital
                    .signals
                    .iter()
                    .find(|signal| signal.name == lanes[0])
                    .map_or(0, |signal| signal.width);
                sliced = parent.array_slice(actual, width, element, formal)?;
                &sliced
            }
            _ => actual,
        };
        if child.ports[positions[&lanes[0]]].direction != PortDirection::Input
            && matches!(actual, Expression::ArrayLiteral(concat) if concat.first_replication().is_some())
        {
            return Err(error(
                "replicated concatenations cannot connect to output or inout ports",
                actual.span(),
            ));
        }
        if discrete_lane {
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
        if let Some((position, count)) = element
            && actuals.len() != lanes.len()
            && actuals.len() == lanes.len() * count as usize
        {
            // Section 12.1.3 for physical lanes: the leftmost element takes
            // the leftmost lanes.
            let start = position as usize * lanes.len();
            actuals = actuals.drain(start..start + lanes.len()).collect();
        }
        if actuals.len() != lanes.len()
            && lanes.len() == 1
            && child.digital.signals.is_empty()
            && child.contributions.is_empty()
            && child.statements.is_empty()
        {
            // A port that stands as one lane of a module with no behavior of
            // either domain: a purely structural module, which only the
            // instances inside it connect, and whose vector ports neither
            // domain expanded.
            return Err(SemanticError::new(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "structural vector port: port '{path}.{formal}' is declared with no net \
                     or variable of its own, in a module that declares no discrete-domain \
                     signal for it"
                )),
                actual.span(),
            )
            .into());
        }
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
