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

struct PackedSite<'a> {
    net: &'a SmolStr,
    port: SmolStr,
    bit: u32,
    direction: PortDirection,
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
        let port = &child.ports[site.1];
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
        let (proxy, bounds) = self.proxy(child, instance, site, lower, lanes.len())?;
        let span = instance.span;
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
            self.connect_physical(
                upper,
                actual,
                instance,
                lower,
                PackedSite {
                    net: &proxy,
                    port: format!("{}[{coordinate}]", port.name).into(),
                    bit: lower.width - 1 - ordinal as u32,
                    direction: port.direction,
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

    pub fn connect_input(
        &mut self,
        child: &AnalyzedModule,
        instance: &ModuleInstance,
        site: (usize, usize),
        lower: &Endpoint,
        actual: &Expression,
        scope: &super::super::node_vectors::ConnectionScope,
        types: &mut crate::canonical_ir::digital_lower::ConnectionShapes<'_>,
    ) -> CompileResult<()> {
        let (value, lanes) = scope.mixed_input(actual, types)?;
        let (proxy, bounds) = self.proxy(child, instance, site, lower, lanes.len())?;
        let port = &child.ports[site.1];
        if lanes.iter().any(Option::is_none) {
            // One source expression retains self-determined concatenation widths
            // and evaluates each replication operand once. Z positions leave the
            // physical bits exclusively owned by the selected converters.
            self.prepared.continuous_assigns.push(ContinuousAssign {
                target: DigitalLValue::Identifier {
                    name: proxy.clone(),
                    span: actual.span(),
                },
                value,
                delay: None,
                span: actual.span(),
            });
        }
        for (ordinal, (lane, coordinate)) in lanes
            .into_iter()
            .zip(bounds.indices_msb_first())
            .enumerate()
        {
            let Some(lane) = lane else { continue };
            let (upper, actual) =
                actual::endpoint(self.source, self.module, &lane, self.constants)?
                    .filter(|(endpoint, _)| endpoint.net_kind.is_none() && endpoint.width == 1)
                    .ok_or_else(|| {
                        error(
                            "a mixed input physical lane requires a scalar physical net",
                            lane.span(),
                        )
                    })?;
            self.connect_physical(
                upper,
                actual,
                instance,
                lower,
                PackedSite {
                    net: &proxy,
                    port: format!("{}[{coordinate}]", port.name).into(),
                    bit: lower.width - 1 - ordinal as u32,
                    direction: PortDirection::Input,
                },
            );
        }
        Ok(())
    }

    fn proxy(
        &mut self,
        child: &AnalyzedModule,
        instance: &ModuleInstance,
        site: (usize, usize),
        lower: &Endpoint,
        width: usize,
    ) -> CompileResult<(SmolStr, super::super::VectorBounds)> {
        let (instance_index, port_index) = site;
        let port = &child.ports[port_index];
        if width != lower.width as usize {
            return Err(error(
                format!(
                    "packed port '{}.{}' requires {} lanes, but its connection supplies {}",
                    instance.name, port.name, lower.width, width,
                ),
                instance.span,
            ));
        }
        let declared = child
            .digital
            .signals
            .iter()
            .find(|signal| signal.name == port.name)
            .expect("a packed discrete formal has a signal declaration");
        let bounds = declared.range.unwrap_or(super::super::VectorBounds::SCALAR);
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
        Ok((proxy, bounds))
    }

    fn connect_physical(
        &mut self,
        upper: Endpoint,
        actual: Expression,
        instance: &ModuleInstance,
        lower: &Endpoint,
        site: PackedSite<'_>,
    ) {
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
        let mut segment = lower.segment.clone();
        segment.name = site.port.clone();
        let index = boundary.signal.push(segment);
        boundary.signal.segments[0].children.push(PortLink::new(
            index,
            site.direction,
            instance.name.clone(),
            site.port,
        ));
        boundary.sites.insert(
            index,
            ConnectionSite {
                kind: lower.net_kind,
                target: ConnectionTarget::Packed {
                    net: site.net.clone(),
                    bit: site.bit,
                },
            },
        );
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
                        right_element: None,
                        right_bit: bounds.position_of(selected) as u32,
                        span,
                    });
            }
        }
        Ok(None)
    }
}
