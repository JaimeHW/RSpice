//! Physical vector actuals feeding packed discrete formals.
use super::*;

pub(super) struct Connections<'a> {
    pub source: &'a Module,
    pub module: &'a AnalyzedModule,
    pub constants: &'a super::super::DigitalConstants,
    pub prepared: &'a mut Module,
    pub used: &'a mut HashSet<SmolStr>,
    pub signals: &'a mut BTreeMap<SignalIdentity, BoundarySignal>,
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
                    "packed port '{}.{}' requires {} physical lanes, but its connection supplies {}",
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
                        endpoint.net_kind.is_none() && endpoint.width == 1 && !endpoint.unpacked
                    })
                    .ok_or_else(|| {
                        error(
                            "a physical bus connection requires scalar physical lanes",
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
        let mut proxy: SmolStr = format!("__rspice_bus_{}_{}", instance_index, port_index).into();
        while !self.used.insert(proxy.clone()) {
            proxy = format!("{proxy}_").into();
        }
        let range = crate::ast::VectorRange {
            msb: super::super::exact_integer_expression(bounds.msb, span),
            lsb: super::super::exact_integer_expression(bounds.lsb, span),
            span,
        };
        self.prepared.nets.push(NetDecl {
            range: Some(range.clone()),
            discipline: lower.segment.declared.clone(),
            names: vec![proxy.clone()],
            is_ground: false,
            is_internal: true,
            span,
        });
        self.prepared.digital_nets.push(DigitalNetDecl {
            kind: lower.net_kind.expect("discrete formal"),
            signedness: declared.signedness,
            range: Some(range),
            items: vec![DigitalDeclItem {
                name: proxy.clone(),
                dimensions: Vec::new(),
                init: None,
                span,
            }],
            span,
        });
        match &mut self.prepared.instances[instance_index].connections[port_index] {
            Connection::Named { signal, .. } | Connection::Ordered { signal, .. } => {
                *signal = Some(Expression::Identifier(Identifier {
                    name: proxy.clone(),
                    span,
                }));
            }
        }
        for (ordinal, ((upper, actual), coordinate)) in endpoints
            .into_iter()
            .zip(bounds.indices_msb_first())
            .enumerate()
        {
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
        Ok(())
    }
}
