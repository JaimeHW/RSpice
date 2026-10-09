//! Resolve net types and disciplines over concrete port-connected occurrences.
//! Templates stay immutable: only changed occurrences are rebuilt before lowering.
mod behavior;
mod connections;
mod disciplines;

use super::digital_elaborate::{
    SpecializationKey, SpecializedModule, check_hierarchy_capacity, digital_subtrees,
    specialize_module,
};
use super::{AnalyzedFile, AnalyzedModule, DigitalSignalClass};
use crate::ast::*;
use crate::error::CompileResult;
use crate::source::Span;
use smol_str::SmolStr;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

struct Occurrence {
    path: SmolStr,
    parent: Option<usize>,
    key: SpecializationKey,
    module: Arc<SpecializedModule>,
    nets: HashMap<SmolStr, usize>,
}

struct Net {
    name: SmolStr,
    occurrence: usize,
    width: u32,
    kind: DigitalNetKind,
    span: Span,
}

fn representative(parents: &mut [usize], mut node: usize) -> usize {
    while parents[node] != node {
        parents[node] = parents[parents[node]];
        node = parents[node];
    }
    node
}

fn join(parents: &mut [usize], a: usize, b: usize) {
    let a = representative(parents, a);
    let b = representative(parents, b);
    parents[a.max(b)] = a.min(b);
}

/// The graph uses authored net groups: a wire bus promotes as a complete bus,
/// never as a mixture of real-valued and four-state bits.
pub(super) fn resolve(
    file: &AnalyzedFile,
    sources: &HashMap<SmolStr, &Module>,
    source: &Module,
    analyzed: &AnalyzedModule,
    specializations: &mut HashMap<SpecializationKey, Arc<SpecializedModule>>,
    warnings: &mut Vec<super::SemanticWarning>,
) -> CompileResult<HashMap<SmolStr, Arc<SpecializedModule>>> {
    let mut required = digital_subtrees(file, sources);
    if !analyzed.digital.signals.is_empty() {
        required.insert(source.name.clone());
    }
    if !required.contains(&source.name) {
        return Ok(HashMap::new());
    }
    let mut templates = HashMap::new();
    let root = Arc::new(SpecializedModule {
        source: source.clone(),
        analyzed: analyzed.clone(),
    });
    let mut occurrences = vec![Occurrence {
        path: "".into(),
        parent: None,
        key: SpecializationKey::root(source.name.clone()),
        module: root,
        nets: HashMap::new(),
    }];
    let mut links = Vec::new();
    let mut cursor = 0;
    while cursor < occurrences.len() {
        let parent = occurrences[cursor].module.clone();
        let constants = super::instance_parameters::constants(&parent.source);
        for instance in &parent.source.instances {
            if !required.contains(&instance.module) {
                continue;
            }
            let (Some(child_source), Some(child)) = (
                sources.get(&instance.module),
                file.modules.get(&instance.module),
            ) else {
                continue;
            };
            let path: SmolStr = if occurrences[cursor].path.is_empty() {
                instance.name.clone()
            } else {
                format!("{}.{}", occurrences[cursor].path, instance.name).into()
            };
            let (key, specialized) = specialize_module(
                file,
                specializations,
                instance,
                child_source,
                child,
                &constants,
                parent.source.time_scale,
                &path,
            )?;
            let mut depth = 0;
            let mut ancestor = Some(cursor);
            while let Some(index) = ancestor {
                if occurrences[index].key == key {
                    return Err(crate::error::SemanticError::new(
                        crate::error::SemanticErrorKind::CircularDependency(format!("digital module hierarchy at instance '{path}': specialization repeats ancestor '{}'", occurrences[index].path)),
                        instance.span).into());
                }
                depth += 1;
                ancestor = occurrences[index].parent;
            }
            check_hierarchy_capacity(depth, occurrences.len() - 1, &path, instance.span)?;
            let module = specialized.unwrap_or_else(|| {
                templates
                    .entry(instance.module.clone())
                    .or_insert_with(|| {
                        Arc::new(SpecializedModule {
                            source: (*child_source).clone(),
                            analyzed: child.clone(),
                        })
                    })
                    .clone()
            });
            let child_index = occurrences.len();
            occurrences.push(Occurrence {
                path,
                parent: Some(cursor),
                key,
                module,
                nets: HashMap::new(),
            });
            links.push((cursor, child_index, instance.connections.clone()));
        }
        cursor += 1;
    }
    let inherited = disciplines::resolve(file, sources, &occurrences, specializations, warnings)?;
    let mut nets = Vec::new();
    for (index, occurrence) in occurrences.iter_mut().enumerate() {
        for signal in &occurrence.module.analyzed.digital.signals {
            let DigitalSignalClass::Net(kind) = signal.class else {
                continue;
            };
            if signal.element_alias.is_some()
                || inherited
                    .get(&index)
                    .and_then(|values| values.get(&signal.name))
                    .is_some_and(|(discipline, _)| disciplines::continuous(file, discipline))
            {
                continue;
            }
            let bus = occurrence
                .module
                .analyzed
                .physical_nodes
                .real_buses
                .get(&signal.name);
            if signal.unpacked.is_some() && bus.is_none_or(|bus| bus.dimensions.len() != 1) {
                continue;
            }
            let width = bus.map_or(signal.width.max(1), |bus| bus.bounds.width());
            occurrence.nets.insert(signal.name.clone(), nets.len());
            nets.push(Net {
                name: signal.name.clone(),
                occurrence: index,
                width,
                kind,
                span: signal.span,
            });
        }
    }
    let mut parents: Vec<_> = (0..nets.len()).collect();
    let mut selected_types = Vec::new();
    let mut selected_links = Vec::new();
    for (parent, child, connections) in links {
        let upper = &occurrences[parent];
        let lower = &occurrences[child];
        for (ordinal, connection) in connections.iter().enumerate() {
            let (formal, actual) = match connection {
                Connection::Named { port, signal, .. } => (port, signal.as_ref()),
                Connection::Ordered { signal, .. } => {
                    let Some(port) = lower.module.source.ports.get(ordinal) else {
                        continue;
                    };
                    (&port.name, signal.as_ref())
                }
            };
            let (Some(&lower_net), Some(actual)) = (lower.nets.get(formal), actual) else {
                continue;
            };
            let Some(group) = connections::collect(upper, &nets, actual) else {
                continue;
            };
            if group.width != nets[lower_net].width {
                continue;
            }
            for segment in group.segments {
                match segment {
                    connections::Segment::Net {
                        node,
                        complete: true,
                        ..
                    } => join(&mut parents, node, lower_net),
                    connections::Segment::Net {
                        node,
                        complete: false,
                        span,
                    } => selected_links.push((node, lower_net, span)),
                    connections::Segment::Real(kind) => selected_types.push((lower_net, kind)),
                }
            }
        }
    }
    let mut real_types = BTreeMap::new();
    for (index, kind) in nets
        .iter()
        .enumerate()
        .map(|(index, net)| (index, net.kind))
        .chain(selected_types)
    {
        let DigitalNetKind::Wreal(resolution) = kind else {
            continue;
        };
        let root = representative(&mut parents, index);
        if let Some(previous) = real_types.insert(root, resolution) {
            if previous != resolution {
                return Err(super::node_vectors::error(
                    "connected real nets must have the same real resolution policy",
                    nets[index].span,
                ));
            }
        }
    }
    // A selected part may acquire a real type after its complete parent bus
    // resolves elsewhere. Propagate that fact without promoting isolated bits.
    let mut pending: std::collections::VecDeque<_> = real_types.keys().copied().collect();
    let mut selected_children: HashMap<usize, Vec<(usize, Span)>> = HashMap::new();
    for &(upper, lower, span) in &selected_links {
        selected_children
            .entry(representative(&mut parents, upper))
            .or_default()
            .push((representative(&mut parents, lower), span));
    }
    while let Some(upper) = pending.pop_front() {
        for &(lower, span) in selected_children.get(&upper).into_iter().flatten() {
            let kind = real_types[&upper];
            if let Some(previous) = real_types.insert(lower, kind) {
                if previous != kind {
                    return Err(super::node_vectors::error(
                        "connected real nets must have the same real resolution policy",
                        span,
                    ));
                }
            } else {
                pending.push_back(lower);
            }
        }
    }
    for (upper, lower, span) in selected_links {
        if real_types.contains_key(&representative(&mut parents, lower))
            && !real_types.contains_key(&representative(&mut parents, upper))
        {
            return Err(super::node_vectors::error(
                "a wire bus must resolve as a complete real bus before a selected part can connect to a real net",
                span,
            ));
        }
    }
    let mut promotions: BTreeMap<usize, BTreeMap<SmolStr, WrealResolution>> = BTreeMap::new();
    for (index, net) in nets.iter().enumerate() {
        if net.kind != DigitalNetKind::Wire {
            continue;
        }
        if let Some(&kind) = real_types.get(&representative(&mut parents, index)) {
            promotions
                .entry(net.occurrence)
                .or_default()
                .insert(net.name.clone(), kind);
        }
    }
    let mut resolved = HashMap::new();
    let changed: std::collections::BTreeSet<_> =
        promotions.keys().chain(inherited.keys()).copied().collect();
    for index in changed {
        let occurrence = &occurrences[index];
        let mut source = occurrence.module.source.clone();
        if let Some(types) = promotions.get(&index) {
            promote(&mut source, types);
        }
        if let Some(disciplines) = inherited.get(&index) {
            disciplines::apply(file, &mut source, disciplines);
        }
        let previous = &occurrence.module.analyzed;
        let analyzed = super::hierarchy_connections::analyze_occurrence(
            file,
            &source,
            previous.default_transition,
            previous.default_discipline.clone(),
        )?;
        resolved.insert(
            occurrence.path.clone(),
            Arc::new(SpecializedModule { source, analyzed }),
        );
    }
    Ok(resolved)
}

fn selected_real_type(
    module: &SpecializedModule,
    expression: &Expression,
) -> Option<(DigitalNetKind, u32)> {
    let (name, indices, part) = match expression {
        Expression::ArrayAccess(access) => (&access.array, vec![access.index.as_ref()], None),
        Expression::Digital(DigitalExpr::PartSelect(select)) => (
            &select.name,
            Vec::new(),
            Some((select.msb.as_ref(), select.lsb.as_ref())),
        ),
        Expression::Digital(DigitalExpr::ArraySelect(access)) => {
            let mut indices = vec![access.index.as_ref()];
            indices.extend(&access.additional_indices);
            let part = match &access.select {
                PackedSelect::Bit(index) => {
                    indices.push(index.as_ref());
                    None
                }
                PackedSelect::Part { msb, lsb } => Some((msb.as_ref(), lsb.as_ref())),
            };
            (&access.name, indices, part)
        }
        _ => return None,
    };
    let signal = module
        .analyzed
        .digital
        .signals
        .iter()
        .find(|signal| signal.name == *name)?;
    let DigitalSignalClass::Net(kind @ DigitalNetKind::Wreal(_)) = signal.class else {
        return None;
    };
    let constants = super::instance_parameters::constants(&module.source);
    let integer = |expression| match crate::canonical_ir::digital_lower::elaboration_constant(
        expression,
        &constants,
        module.source.time_scale,
    ) {
        Some(crate::numeric_literal::NumericLiteralValue::Integer(value)) => Some(value),
        _ => None,
    };
    let coordinates: Option<Vec<_>> = indices.iter().map(|index| integer(*index)).collect();
    let coordinates = coordinates?;
    if coordinates
        .iter()
        .zip(&signal.dimensions)
        .any(|(coordinate, axis)| !axis.contains(*coordinate))
    {
        return None;
    }
    let bus = module.analyzed.physical_nodes.real_buses.get(name);
    let width = if let Some((msb, lsb)) = part {
        let bus = bus?;
        if coordinates.len() + 1 != signal.dimensions.len() {
            return None;
        }
        let (msb, lsb) = (integer(msb)?, integer(lsb)?);
        if !bus.bounds.contains(msb) || !bus.bounds.contains(lsb) {
            return None;
        }
        u32::try_from(msb.abs_diff(lsb) + 1).ok()?
    } else if coordinates.len() == signal.dimensions.len() {
        1
    } else if coordinates.len() + 1 == signal.dimensions.len() {
        bus?.bounds.width()
    } else {
        return None;
    };
    Some((kind, width))
}

fn selected_wire<'a>(
    module: &'a SpecializedModule,
    expression: &Expression,
) -> Option<(&'a SmolStr, u32, bool)> {
    let (name, msb, lsb) = match expression {
        Expression::ArrayAccess(access) => {
            (&access.array, access.index.as_ref(), access.index.as_ref())
        }
        Expression::Digital(DigitalExpr::PartSelect(select)) => {
            (&select.name, select.msb.as_ref(), select.lsb.as_ref())
        }
        _ => return None,
    };
    let signal = module
        .analyzed
        .digital
        .signals
        .iter()
        .find(|signal| signal.name == *name)?;
    if signal.class != DigitalSignalClass::Net(DigitalNetKind::Wire) || signal.unpacked.is_some() {
        return None;
    }
    let range = signal.range?;
    let constants = super::instance_parameters::constants(&module.source);
    let integer = |expression| match crate::canonical_ir::digital_lower::elaboration_constant(
        expression,
        &constants,
        module.source.time_scale,
    ) {
        Some(crate::numeric_literal::NumericLiteralValue::Integer(value)) => Some(value),
        _ => None,
    };
    let (msb, lsb) = (integer(msb)?, integer(lsb)?);
    if !range.contains(msb)
        || !range.contains(lsb)
        || (msb != lsb && (msb > lsb) != (range.msb > range.lsb))
    {
        return None;
    }
    Some((
        &signal.name,
        u32::try_from(msb.abs_diff(lsb) + 1).ok()?,
        msb == range.msb && lsb == range.lsb,
    ))
}

fn promote(source: &mut Module, types: &BTreeMap<SmolStr, WrealResolution>) {
    let mut declared = std::collections::HashSet::new();
    source.digital_nets = source
        .digital_nets
        .iter()
        .flat_map(|declaration| {
            declaration
                .items
                .iter()
                .map(|item| {
                    let mut net = declaration.clone();
                    net.items = vec![item.clone()];
                    if let Some(&resolution) = types.get(&item.name) {
                        net.kind = DigitalNetKind::Wreal(resolution);
                        net.signedness = Signedness::Unsigned;
                        declared.insert(item.name.clone());
                    }
                    net
                })
                .collect::<Vec<_>>()
        })
        .collect();
    source.port_declarations = source
        .port_declarations
        .iter()
        .flat_map(|declaration| {
            declaration.names.iter().map(|name| {
                let mut port = declaration.clone();
                port.names = vec![name.clone()];
                port.initializers.retain(|(target, _)| target == name);
                if let Some(&resolution) = types.get(name) {
                    port.net_type = Some(PortNetType::Wreal(resolution));
                    port.signedness = Signedness::Unsigned;
                }
                port
            })
        })
        .collect();
    for (name, &resolution) in types {
        if declared.contains(name) {
            continue;
        }
        let declaration = source.nets.iter().find(|net| net.names.contains(name));
        let port = source
            .port_declarations
            .iter()
            .find(|port| port.names.contains(name));
        let range = declaration
            .and_then(|net| net.range.clone())
            .or_else(|| port.and_then(|port| port.range.clone()));
        let span = declaration
            .map(|net| net.span)
            .or_else(|| port.map(|port| port.span))
            .unwrap_or(source.span);
        source.digital_nets.push(DigitalNetDecl {
            kind: DigitalNetKind::Wreal(resolution),
            signedness: Signedness::Unsigned,
            range,
            items: vec![DigitalDeclItem {
                name: name.clone(),
                dimensions: Vec::new(),
                init: None,
                span,
            }],
            span,
        });
    }
}
