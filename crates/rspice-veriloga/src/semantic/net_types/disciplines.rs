//! Apply bottom-up domain and discipline resolution to structural interconnects.
//! Physical endpoints constrain mixed boundaries; their declarations are retained.
use super::*;
use crate::connect::{
    ConnectError, ConnectValueKind, NetSegment, PortLink, ResolutionMode, Signal,
    resolve_disciplines,
};
use crate::semantic::hierarchy_connections::{declared_discipline, net_operands};

type Assignments = BTreeMap<usize, BTreeMap<SmolStr, (SmolStr, Span)>>;

struct Graph {
    signal: Signal,
    spans: Vec<Span>,
    editable: Vec<(usize, SmolStr, usize, SmolStr)>,
}

impl Graph {
    fn push(&mut self, segment: NetSegment, span: Span) -> usize {
        self.spans.push(span);
        self.signal.push(segment)
    }

    fn scope(
        &mut self,
        module: &SpecializedModule,
        path: &str,
        occurrence: Option<usize>,
    ) -> HashMap<SmolStr, usize> {
        let source = &module.source;
        let analyzed = &module.analyzed;
        let qualify = |name: &str| -> SmolStr {
            if path.is_empty() {
                name.into()
            } else {
                format!("{path}.{name}").into()
            }
        };
        let mut names = HashMap::new();
        let behavioral = super::behavior::used(module);
        for signal in &analyzed.digital.signals {
            if signal.element_alias.is_some() {
                continue;
            }
            let declared = declared_discipline(source, analyzed, &signal.name);
            // Retain RSpice's existing implicit logic default when no directive
            // applies. A connected child's discipline takes precedence over it.
            let default = analyzed
                .default_discipline
                .clone()
                .unwrap_or_else(|| "logic".into());
            let mut segment = NetSegment::new(qualify(&signal.name))
                .with_default_discipline(default.clone())
                .with_value_kind(if signal.class.is_real() {
                    ConnectValueKind::Real
                } else {
                    ConnectValueKind::FourState
                });
            // A wire used only as interconnect follows its child domains.
            // Real nets, variables and behavioral values remain discrete.
            segment.digital_behavioral = signal.class
                != DigitalSignalClass::Net(DigitalNetKind::Wire)
                || behavioral.contains(&signal.name);
            segment.declared = declared.clone();
            let index = self.push(segment, signal.span);
            if let Some(occurrence) = occurrence.filter(|_| declared.is_none()) {
                self.editable
                    .push((occurrence, signal.name.clone(), index, default));
            }
            names.insert(signal.name.clone(), index);
        }
        for (name, discipline) in analyzed
            .ports
            .iter()
            .map(|port| (&port.name, &port.discipline))
            .chain(
                analyzed
                    .internal_nodes
                    .iter()
                    .map(|node| (&node.name, &node.discipline)),
            )
        {
            if names.contains_key(name) || analyzed.physical_nodes.real_aliases.contains_key(name) {
                continue;
            }
            let index = self.push(
                NetSegment::new(qualify(name)).declared(discipline.clone()),
                source.span,
            );
            names.insert(name.clone(), index);
        }
        for (alias, target) in &analyzed.physical_nodes.real_aliases {
            if let Some(&index) = names.get(&target.array) {
                names.insert(alias.clone(), index);
            }
        }
        for (name, lanes) in analyzed
            .physical_nodes
            .vectors
            .iter()
            .map(|(name, vector)| (name, &vector.lanes))
            .chain(
                analyzed
                    .physical_nodes
                    .arrays
                    .iter()
                    .map(|(name, array)| (name, &array.lanes)),
            )
            .chain(
                analyzed
                    .physical_nodes
                    .ports
                    .iter()
                    .map(|(name, lanes)| (name, lanes)),
            )
        {
            if let Some(&index) = lanes.first().and_then(|lane| names.get(lane)) {
                names.entry(name.clone()).or_insert(index);
            }
        }
        names
    }
}

pub(super) fn resolve(
    file: &AnalyzedFile,
    sources: &HashMap<SmolStr, &Module>,
    occurrences: &[Occurrence],
    specializations: &mut HashMap<SpecializationKey, Arc<SpecializedModule>>,
    warnings: &mut Vec<super::super::SemanticWarning>,
) -> CompileResult<Assignments> {
    let mut graph = Graph {
        signal: Signal::default(),
        spans: Vec::new(),
        editable: Vec::new(),
    };
    let maps: Vec<_> = occurrences
        .iter()
        .enumerate()
        .map(|(index, occurrence)| graph.scope(&occurrence.module, &occurrence.path, Some(index)))
        .collect();
    let paths: HashMap<_, _> = occurrences
        .iter()
        .enumerate()
        .map(|(index, occurrence)| (occurrence.path.as_str(), index))
        .collect();
    for (index, occurrence) in occurrences.iter().enumerate() {
        let source = &occurrence.module.source;
        let constants = super::super::instance_parameters::constants(source);
        for instance in &source.instances {
            let path = if occurrence.path.is_empty() {
                instance.name.to_string()
            } else {
                format!("{}.{}", occurrence.path, instance.name)
            };
            let analog_leaf;
            let names;
            let (child, child_names) = if let Some(&child_index) = paths.get(path.as_str()) {
                (&*occurrences[child_index].module, &maps[child_index])
            } else {
                // A pure analog child is a typed endpoint here, not another
                // digital traversal. Its parameterized declarations still apply.
                let (Some(child_source), Some(child)) = (
                    sources.get(&instance.module),
                    file.modules.get(&instance.module),
                ) else {
                    continue;
                };
                let (_, specialized) = specialize_module(
                    file,
                    specializations,
                    instance,
                    child_source,
                    child,
                    &constants,
                    source.time_scale,
                    &path,
                )?;
                analog_leaf = specialized.unwrap_or_else(|| {
                    Arc::new(SpecializedModule {
                        source: (*child_source).clone(),
                        analyzed: child.clone(),
                    })
                });
                names = graph.scope(&analog_leaf, &path, None);
                (&*analog_leaf, &names)
            };
            for (ordinal, connection) in instance.connections.iter().enumerate() {
                let (formal, actual) = match connection {
                    Connection::Named { port, signal, .. } => (port, signal.as_ref()),
                    Connection::Ordered { signal, .. } => {
                        let Some(port) = child.source.ports.get(ordinal) else {
                            continue;
                        };
                        (&port.name, signal.as_ref())
                    }
                };
                let (Some(&lower), Some(actual)) = (child_names.get(formal), actual) else {
                    continue;
                };
                for (name, _) in net_operands(source, &occurrence.module.analyzed, actual) {
                    if let Some(&upper) = maps[index].get(name) {
                        graph.signal.segments[upper].children.push(PortLink::new(
                            lower,
                            PortDirection::Inout,
                            path.clone(),
                            formal.clone(),
                        ));
                    }
                }
            }
        }
    }
    let resolved = resolve_disciplines(
        &graph.signal,
        &file.connect_rules,
        &file.disciplines,
        None,
        ResolutionMode::Basic,
    )
    .map_err(|cause| {
        let net = match &cause {
            ConnectError::UnresolvedDiscipline { net }
            | ConnectError::ExcludedDisciplines { net, .. }
            | ConnectError::IncompatibleNetDisciplines { net, .. } => Some(net),
            _ => None,
        };
        let span = net
            .and_then(|net| {
                graph
                    .signal
                    .segments
                    .iter()
                    .position(|segment| segment.name == *net)
            })
            .map(|index| graph.spans[index])
            .unwrap_or(occurrences[0].module.source.span);
        super::super::node_vectors::error(cause.to_string(), span)
    })?;
    for (&index, message) in resolved.warning_segments.iter().zip(&resolved.warnings) {
        warnings.push(super::super::SemanticWarning {
            code: "VA-SEM-DISCIPLINE-RESOLUTION",
            message: message.clone(),
            span: graph.spans[index],
        });
    }
    let mut assignments = Assignments::new();
    for (occurrence, name, index, default) in graph.editable {
        if let Some(discipline) = resolved
            .discipline(index)
            .filter(|discipline| *discipline != default.as_str() || continuous(file, discipline))
        {
            assignments
                .entry(occurrence)
                .or_default()
                .insert(name, (discipline.into(), graph.spans[index]));
        }
    }
    Ok(assignments)
}

pub(super) fn continuous(file: &AnalyzedFile, discipline: &str) -> bool {
    file.disciplines
        .get_discipline(discipline)
        .is_some_and(|discipline| discipline.domain == crate::disciplines::Domain::Continuous)
}

pub(super) fn apply(
    file: &AnalyzedFile,
    source: &mut Module,
    assignments: &BTreeMap<SmolStr, (SmolStr, Span)>,
) {
    let physical: std::collections::HashSet<_> = assignments
        .iter()
        .filter(|(_, (discipline, _))| continuous(file, discipline))
        .map(|(name, _)| name.clone())
        .collect();
    let mut shapes = HashMap::new();
    for declaration in &mut source.digital_nets {
        declaration.items.retain(|item| {
            if !physical.contains(&item.name) {
                return true;
            }
            shapes.insert(
                item.name.clone(),
                (declaration.range.clone(), item.dimensions.clone()),
            );
            false
        });
    }
    source
        .digital_nets
        .retain(|declaration| !declaration.items.is_empty());
    for (name, (discipline, span)) in assignments {
        let (range, dimensions) = if physical.contains(name) {
            shapes.remove(name).unwrap_or_else(|| {
                (
                    source
                        .port_declarations
                        .iter()
                        .find(|port| port.names.contains(name))
                        .and_then(|port| port.range.clone()),
                    Vec::new(),
                )
            })
        } else {
            (None, Vec::new())
        };
        source.nets.push(NetDecl {
            discipline: Some(discipline.clone()),
            names: vec![name.clone()],
            range,
            dimensions: if dimensions.is_empty() {
                Vec::new()
            } else {
                vec![(name.clone(), dimensions)]
            },
            is_ground: false,
            is_internal: false,
            span: *span,
        });
    }
}
