//! Physical and discrete lanes sharing one packed port connection.
use super::*;

pub(super) struct Connections<'a> {
    pub source: &'a Module,
    pub module: &'a AnalyzedModule,
    pub constants: &'a super::super::DigitalConstants,
    pub prepared: &'a mut Module,
    pub used: &'a mut HashSet<SmolStr>,
    pub signals: &'a mut BTreeMap<SignalIdentity, BoundarySignal>,
    pub aliases: &'a mut Vec<super::super::digital::ElaboratedDigitalBitAlias>,
}

impl Connections<'_> {
    pub fn connect(
        &mut self,
        child: &AnalyzedModule,
        instance: &ModuleInstance,
        site: (usize, usize),
        lower: &Endpoint,
        lanes: &[Expression],
    ) -> CompileResult<()> {
        let (instance_index, port_index) = site;
        let port = &child.ports[port_index];
        if lanes.len() != lower.width as usize {
            return Err(error(
                format!(
                    "packed port '{}.{}' requires {} lanes, but its connection supplies {}",
                    instance.name,
                    port.name,
                    lower.width,
                    lanes.len(),
                ),
                instance.span,
            ));
        }
        let endpoints = lanes
            .iter()
            .map(|lane| {
                actual::endpoint(self.source, self.module, lane, self.constants)?
                    .filter(|(endpoint, _)| {
                        endpoint.width == 1 && !endpoint.unpacked
                    })
                    .ok_or_else(|| {
                        error(
                            "a mixed bus connection requires scalar physical or four-state digital lanes",
                            lane.span(),
                        )
                    })
            })
            .collect::<CompileResult<Vec<_>>>()?;
        let declared = child
            .digital
            .signals
            .iter()
            .find(|signal| signal.name == port.name)
            .expect("a packed discrete formal has a signal declaration");
        let bounds = declared.range.expect("a packed formal has bounds");
        let span = instance.span;
        let proxy = inputs::temporary_net(
            self.prepared,
            self.used,
            format!("__rspice_bus_{}_{}", instance_index, port_index).into(),
            lower,
            declared,
            span,
        );
        match &mut self.prepared.instances[instance_index].connections[port_index] {
            Connection::Named { signal, .. } | Connection::Ordered { signal, .. } => {
                *signal = Some(Expression::Identifier(Identifier {
                    name: proxy.clone(),
                    span,
                }));
            }
        }
        let mut digital_targets = Vec::new();
        let mut digital_values = Vec::new();
        for (ordinal, ((upper, actual), coordinate)) in endpoints
            .into_iter()
            .zip(bounds.indices_msb_first())
            .enumerate()
        {
            if upper.net_kind.is_some() {
                if let Some((target, value)) = self.connect_digital(
                    &upper,
                    actual,
                    (&proxy, coordinate, lower.width - 1 - ordinal as u32),
                    port.direction,
                )? {
                    digital_targets.push(target);
                    digital_values.push(crate::ast::ArrayLiteralElement::Value(value));
                }
                continue;
            }
            let boundary = self.signals.entry(upper.identity).or_insert_with(|| {
                let mut signal = Signal::default();
                signal.push(upper.segment);
                BoundarySignal {
                    signal,
                    actual,
                    upper_kind: None,
                    sites: HashMap::new(),
                }
            });
            let port_name: SmolStr = format!("{}[{coordinate}]", port.name).into();
            let mut segment = lower.segment.clone();
            segment.name = port_name.clone();
            let index = boundary.signal.push(segment);
            boundary.signal.segments[0].children.push(PortLink::new(
                index,
                port.direction,
                instance.name.clone(),
                port_name,
            ));
            boundary.sites.insert(
                index,
                ConnectionSite {
                    kind: lower.net_kind,
                    target: ConnectionTarget::Packed {
                        net: proxy.clone(),
                        bit: lower.width - 1 - ordinal as u32,
                    },
                },
            );
        }
        if !digital_targets.is_empty() {
            // One assignment captures the concatenated value and publishes its
            // affected vector lanes through the existing grouped-write path.
            self.prepared.continuous_assigns.push(ContinuousAssign {
                target: DigitalLValue::Concat {
                    elements: digital_targets,
                    span,
                },
                value: Expression::ArrayLiteral(crate::ast::ArrayLiteralExpr {
                    elements: digital_values,
                    assignment_pattern: false,
                    span,
                }),
                delay: None,
                span,
            });
        }
        Ok(())
    }

    fn connect_digital(
        &mut self,
        upper: &Endpoint,
        actual: Expression,
        proxy: (&SmolStr, i64, u32),
        direction: PortDirection,
    ) -> CompileResult<Option<(DigitalLValue, Expression)>> {
        let (proxy, coordinate, position) = proxy;
        let span = actual.span();
        let declaration = self
            .module
            .digital
            .signals
            .iter()
            .find(|signal| signal.name == upper.identity.name)
            .expect("a digital endpoint has a declaration");
        if direction != PortDirection::Input {
            if declaration.class.is_variable() {
                return Err(error(
                    "an output or inout mixed bus lane requires a net, not a variable",
                    span,
                ));
            }
            if self
                .module
                .ports
                .iter()
                .any(|port| port.name == declaration.name && port.direction == PortDirection::Input)
            {
                return Err(error(
                    "an output or inout mixed bus lane cannot drive a parent input port",
                    span,
                ));
            }
        }
        let proxy_bit = Expression::ArrayAccess(crate::ast::ArrayAccessExpr {
            normalized: false,
            packed: None,
            discrete_validity: None,
            array: proxy.clone(),
            index: Box::new(super::super::exact_integer_expression(coordinate, span)),
            span,
        });
        match direction {
            PortDirection::Input | PortDirection::Output => {
                let (target, value) = if direction == PortDirection::Input {
                    (actual::lvalue(&proxy_bit), actual)
                } else {
                    (actual::lvalue(&actual), proxy_bit)
                };
                return Ok(Some((target, value)));
            }
            PortDirection::Inout => {
                let bounds = declaration
                    .range
                    .unwrap_or(super::super::VectorBounds::SCALAR);
                let selected = upper.identity.bits.map_or(bounds.lsb, |(msb, _)| msb);
                if !upper.identity.elements.is_empty() || !bounds.contains(selected) {
                    return Err(error(
                        "an inout mixed bus lane requires an in-range wire bit",
                        span,
                    ));
                }
                // Aliases use normalized positions, independent of each
                // side's authored ascending or descending bit coordinates.
                self.aliases
                    .push(super::super::digital::ElaboratedDigitalBitAlias {
                        left: proxy.clone(),
                        left_bit: position,
                        right: declaration.name.clone(),
                        right_bit: bounds.position_of(selected) as u32,
                        span,
                    });
            }
        }
        Ok(None)
    }
}
