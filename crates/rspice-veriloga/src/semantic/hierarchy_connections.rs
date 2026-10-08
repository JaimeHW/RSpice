//! Materialize planned connect instances in each concrete HDL occurrence.
//!
//! Both hierarchy lowerings consume the rewritten occurrence. The connection
//! planner owns selection, merged/split grouping and generated instance names;
//! this adapter supplies typed segments and ordinary executable module bodies.

mod actual;

use super::digital_elaborate::{SpecializationKey, SpecializedModule, specialize_module};
use super::{AnalyzedFile, AnalyzedModule, DigitalSignalClass, SemanticAnalyzer};
use crate::ast::{
    Connection, ContinuousAssign, DigitalDeclItem, DigitalLValue, DigitalNetDecl, DigitalNetKind,
    Expression, Identifier, Item, Module, ModuleInstance, NetDecl, PortDirection, Signedness,
    WrealResolution,
};
use crate::connect::{
    ConnectModuleInsertion, ConnectValueKind, InsertionRule, NetSegment, PortLink, ResolutionMode,
    Signal, plan_connect_modules, resolve_disciplines,
};
use crate::disciplines::Domain;
use crate::error::{CompileError, CompileResult, SemanticError, SemanticErrorKind};
use crate::source::Span;
use smol_str::SmolStr;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;

#[derive(Default)]
pub(super) struct ConnectionModules {
    pub specializations: HashMap<SpecializationKey, Arc<SpecializedModule>>,
    pub prepared: HashMap<SpecializationKey, Option<Arc<SpecializedModule>>>,
    pub modules: HashMap<SmolStr, Arc<SpecializedModule>>,
    identities: HashMap<String, SmolStr>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SignalIdentity {
    name: SmolStr,
    elements: Vec<i64>,
    bits: Option<(i64, i64)>,
}

impl SignalIdentity {
    fn whole(name: &str) -> Self {
        Self {
            name: name.into(),
            elements: Vec::new(),
            bits: None,
        }
    }
}

struct Endpoint {
    identity: SignalIdentity,
    segment: NetSegment,
    net_kind: Option<DigitalNetKind>,
    width: u32,
    unpacked: bool,
}

fn error(message: impl Into<String>, span: Span) -> CompileError {
    CompileError::Semantic(SemanticError::new(
        SemanticErrorKind::UnsupportedFeature(message.into()),
        span,
    ))
}

fn endpoint(source: &Module, module: &AnalyzedModule, name: &str) -> Option<Endpoint> {
    if let Some(signal) = module
        .digital
        .signals
        .iter()
        .find(|signal| signal.name == name)
    {
        let discipline = source
            .nets
            .iter()
            .find(|net| net.names.iter().any(|candidate| candidate == name))
            .and_then(|net| net.discipline.clone())
            .or_else(|| {
                source
                    .port_declarations
                    .iter()
                    .find(|port| port.names.iter().any(|candidate| candidate == name))
                    .and_then(|port| port.discipline.clone())
            })
            .or_else(|| module.default_discipline.clone())
            .unwrap_or_else(|| "logic".into());
        let net_kind = match signal.class {
            DigitalSignalClass::Net(kind) => kind,
            DigitalSignalClass::Variable(kind) if kind.is_real() => {
                DigitalNetKind::Wreal(WrealResolution::Single)
            }
            DigitalSignalClass::Variable(_) => DigitalNetKind::Wire,
        };
        return Some(Endpoint {
            identity: SignalIdentity::whole(name),
            segment: NetSegment::new(name)
                .declared(discipline)
                .digital_behavioral()
                .with_value_kind(if signal.class.is_real() {
                    ConnectValueKind::Real
                } else {
                    ConnectValueKind::FourState
                }),
            net_kind: Some(net_kind),
            width: signal.width,
            unpacked: signal.unpacked.is_some(),
        });
    }
    let discipline = module
        .ports
        .iter()
        .find(|port| port.name == name)
        .map(|port| port.discipline.clone())
        .or_else(|| {
            module
                .internal_nodes
                .iter()
                .find(|node| node.name == name)
                .map(|node| node.discipline.clone())
        })
        .or_else(|| {
            (name == "0" || module.ground_nodes.iter().any(|node| node == name))
                .then(|| SmolStr::new("electrical"))
        })?;
    Some(Endpoint {
        identity: SignalIdentity::whole(name),
        segment: NetSegment::new(name).declared(discipline),
        net_kind: None,
        width: 1,
        unpacked: false,
    })
}

struct BoundarySignal {
    signal: Signal,
    actual: Expression,
    upper_kind: Option<DigitalNetKind>,
    // Segment -> source instance, connection, and the lower net's type.
    sites: HashMap<usize, (usize, usize, Option<DigitalNetKind>)>,
}

/// Insert at typed module boundaries after generate specialization. Concrete
/// packed bits and unpacked elements retain their own signal identities and
/// connecting-scope assignments. Whole vector ports require per-lane topology.
pub(super) fn prepare(
    analyzed: &AnalyzedFile,
    sources: &HashMap<SmolStr, &Module>,
    source: &Module,
    module: &AnalyzedModule,
    bodies: &mut ConnectionModules,
) -> CompileResult<Option<Arc<SpecializedModule>>> {
    if analyzed.connect_rules.insertions().is_empty() || source.instances.is_empty() {
        return Ok(None);
    }
    let mut signals: BTreeMap<SignalIdentity, BoundarySignal> = BTreeMap::new();
    let constants = super::instance_parameters::constants(source);
    for (instance_index, instance) in source.instances.iter().enumerate() {
        let Some(child) = analyzed.modules.get(&instance.module) else {
            continue;
        };
        let Some(child_source) = sources.get(&instance.module) else {
            continue;
        };
        let (_, specialized) = specialize_module(
            analyzed,
            &mut bodies.specializations,
            instance,
            child_source,
            child,
            &constants,
            source.time_scale,
            &format!("{}.{}", source.name, instance.name),
        )?;
        let (child_source, child) = specialized
            .as_deref()
            .map(|module| (&module.source, &module.analyzed))
            .unwrap_or((child_source, child));
        let connections =
            super::digital_elaborate::bind_connections(instance, child, &instance.name)?;
        for (port_index, actual) in connections.into_iter().enumerate() {
            let Some(actual) = actual else { continue };
            let port = &child.ports[port_index];
            let Some(lower) = endpoint(child_source, child, &port.name) else {
                continue;
            };
            // A selected discrete actual can only create a mixed boundary at
            // a continuous formal. Leave ordinary digital selection semantics
            // to the digital hierarchy binder, including dynamic expressions.
            let selected = !matches!(actual, Expression::Identifier(_) | Expression::Number(_));
            if selected && lower.net_kind.is_some() {
                continue;
            }
            let Some((upper, actual)) = actual::endpoint(source, module, actual, &constants)?
            else {
                continue;
            };
            let name = upper.segment.name.clone();
            if upper.net_kind.is_some() == lower.net_kind.is_some() {
                continue;
            }
            if [(&upper, name.as_str()), (&lower, port.name.as_str())]
                .iter()
                .any(|(endpoint, _)| endpoint.width > 1 || endpoint.unpacked)
            {
                return Err(error(
                    format!(
                        "mixed connection '{}.{}' requires scalar net segments; vector and array conversion must retain individual element connections",
                        instance.name, port.name,
                    ),
                    actual.span(),
                ));
            }
            let connection_index = instance.connections.iter().position(|connection| {
                matches!(connection, Connection::Named { port: name, .. } if name == &port.name)
            }).unwrap_or(port_index);
            let boundary = signals.entry(upper.identity).or_insert_with(|| {
                let mut signal = Signal::default();
                signal.push(upper.segment);
                BoundarySignal {
                    signal,
                    actual: actual.clone(),
                    upper_kind: upper.net_kind,
                    sites: HashMap::new(),
                }
            });
            let lower_index = boundary.signal.push(lower.segment);
            boundary.signal.segments[0].children.push(PortLink::new(
                lower_index,
                port.direction,
                instance.name.clone(),
                port.name.clone(),
            ));
            boundary.sites.insert(
                lower_index,
                (instance_index, connection_index, lower.net_kind),
            );
        }
    }
    if signals.is_empty() {
        return Ok(None);
    }
    let mut prepared = source.clone();
    let mut used = declared_names(source);
    for boundary in signals.into_values() {
        let resolved = resolve_disciplines(
            &boundary.signal,
            &analyzed.connect_rules,
            &analyzed.disciplines,
            module.default_discipline.as_deref(),
            ResolutionMode::Basic,
        )
        .map_err(|cause| {
            error(
                format!("module '{}': {cause}", source.name),
                boundary.actual.span(),
            )
        })?;
        let plan = plan_connect_modules(
            &boundary.signal,
            &resolved,
            &analyzed.connect_rules,
            &analyzed.disciplines,
        )
        .map_err(|cause| {
            error(
                format!("module '{}': {cause}", source.name),
                boundary.actual.span(),
            )
        })?;
        for insertion in plan.insertions {
            let span = boundary.actual.span();
            if !used.insert(insertion.instance.clone().into()) {
                return Err(error(
                    format!(
                        "generated connect instance '{}' conflicts with a declaration in module '{}'",
                        insertion.instance, source.name,
                    ),
                    span,
                ));
            }
            let lower_kind = boundary.sites[&insertion.bindings[0].lower].2;
            let value_kind = boundary.signal.segments[if lower_kind.is_some() {
                insertion.bindings[0].lower
            } else {
                0
            }]
            .value_kind;
            let rule = analyzed
                .connect_rules
                .select_typed(
                    &insertion.continuous,
                    &insertion.discrete,
                    insertion.direction,
                    value_kind,
                    &analyzed.disciplines,
                )
                .map_err(|cause| error(cause.to_string(), span))?;
            // Connect rules have no parent instance parameter scope. Validate
            // their constants before ordinary hierarchy override resolution.
            rule.numeric_parameters()
                .map_err(|cause| error(cause.to_string(), span))?;
            let body_name = bodies.materialize(analyzed, sources, rule, &insertion)?;
            let mut private: SmolStr = format!("{}__net", insertion.instance).into();
            while !used.insert(private.clone()) {
                private = format!("{private}_").into();
            }
            let lower_discipline = if lower_kind.is_some() {
                insertion.discrete.clone()
            } else {
                insertion.continuous.clone()
            };
            prepared.nets.push(NetDecl {
                discipline: Some(lower_discipline),
                names: vec![private.clone()],
                is_ground: false,
                is_internal: true,
                span,
            });
            if let Some(kind) = lower_kind {
                prepared.digital_nets.push(DigitalNetDecl {
                    kind,
                    signedness: Signedness::Unsigned,
                    range: None,
                    items: vec![DigitalDeclItem {
                        name: private.clone(),
                        dimensions: Vec::new(),
                        init: None,
                        span,
                    }],
                    span,
                });
            }
            let private_expression = Expression::Identifier(Identifier {
                name: private,
                span,
            });
            for binding in &insertion.bindings {
                let (instance_index, connection_index, kind) = boundary.sites[&binding.lower];
                if kind != lower_kind {
                    return Err(error(
                        "merged connection has incompatible discrete net resolution types",
                        span,
                    ));
                }
                match &mut prepared.instances[instance_index].connections[connection_index] {
                    Connection::Named { signal, .. } | Connection::Ordered { signal, .. } => {
                        *signal = Some(private_expression.clone());
                    }
                }
            }
            let upper = if lower_kind.is_none()
                && !matches!(
                    boundary.actual,
                    Expression::Identifier(_) | Expression::Number(_)
                ) {
                if rule.discrete.direction == PortDirection::Inout {
                    return Err(error(
                        "bidirectional mixed connections to selected lanes require net alias elaboration",
                        span,
                    ));
                }
                let mut tap: SmolStr = format!("{}__actual", insertion.instance).into();
                while !used.insert(tap.clone()) {
                    tap = format!("{tap}_").into();
                }
                prepared.nets.push(NetDecl {
                    discipline: Some(insertion.discrete.clone()),
                    names: vec![tap.clone()],
                    is_ground: false,
                    is_internal: true,
                    span,
                });
                prepared.digital_nets.push(DigitalNetDecl {
                    kind: boundary.upper_kind.expect("selected discrete actual"),
                    signedness: Signedness::Unsigned,
                    range: None,
                    items: vec![DigitalDeclItem {
                        name: tap.clone(),
                        dimensions: Vec::new(),
                        init: None,
                        span,
                    }],
                    span,
                });
                let tap_expression = Expression::Identifier(Identifier {
                    name: tap.clone(),
                    span,
                });
                let (target, value) = if rule.discrete.direction == PortDirection::Input {
                    (
                        DigitalLValue::Identifier { name: tap, span },
                        boundary.actual.clone(),
                    )
                } else {
                    (actual::lvalue(&boundary.actual), tap_expression.clone())
                };
                prepared.continuous_assigns.push(ContinuousAssign {
                    target,
                    value,
                    delay: None,
                    span,
                });
                tap_expression
            } else {
                boundary.actual.clone()
            };
            let (continuous, discrete) = if lower_kind.is_some() {
                (upper, private_expression)
            } else {
                (private_expression, upper)
            };
            prepared.instances.push(ModuleInstance {
                module: body_name,
                name: insertion.instance.into(),
                parameters: insertion.parameters,
                connections: vec![
                    Connection::Named {
                        port: insertion.continuous_port,
                        signal: Some(continuous),
                        span,
                    },
                    Connection::Named {
                        port: insertion.discrete_port,
                        signal: Some(discrete),
                        span,
                    },
                ],
                span,
            });
        }
    }
    let mut analyzed = analyze_occurrence(
        analyzed,
        &prepared,
        module.default_transition,
        module.default_discipline.clone(),
    )?;
    analyzed.hierarchical_connections = true;
    Ok(Some(Arc::new(SpecializedModule {
        source: prepared,
        analyzed,
    })))
}

fn declared_names(source: &Module) -> HashSet<SmolStr> {
    source
        .ports
        .iter()
        .map(|port| port.name.clone())
        .chain(source.nets.iter().flat_map(|net| net.names.clone()))
        .chain(
            source
                .digital_nets
                .iter()
                .flat_map(|net| net.items.iter().map(|item| item.name.clone())),
        )
        .chain(
            source
                .digital_variables
                .iter()
                .flat_map(|net| net.items.iter().map(|item| item.name.clone())),
        )
        .chain(
            source
                .variables
                .iter()
                .flat_map(|net| net.items.iter().map(|item| item.name.clone())),
        )
        .chain(
            source
                .parameters
                .iter()
                .chain(&source.localparams)
                .map(|parameter| parameter.name.clone()),
        )
        .chain(source.aliasparams.iter().map(|alias| alias.alias.clone()))
        .chain(source.branches.iter().map(|branch| branch.name.clone()))
        .chain(
            source
                .functions
                .iter()
                .map(|function| function.name.clone()),
        )
        .chain(
            source
                .instances
                .iter()
                .map(|instance| instance.name.clone()),
        )
        .collect()
}

fn analyze_occurrence(
    file: &AnalyzedFile,
    source: &Module,
    transition: f64,
    default_discipline: Option<SmolStr>,
) -> CompileResult<AnalyzedModule> {
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.disciplines = file.disciplines.clone();
    analyzer.current_default_transition = transition;
    let mut module = analyzer.analyze_module(source, transition)?;
    module.default_discipline = default_discipline;
    Ok(module)
}

impl ConnectionModules {
    fn materialize(
        &mut self,
        analyzed: &AnalyzedFile,
        sources: &HashMap<SmolStr, &Module>,
        rule: &InsertionRule,
        insertion: &ConnectModuleInsertion,
    ) -> CompileResult<SmolStr> {
        let key = format!(
            "{:?}:{:?}:{:?}:{:?}:{:?}",
            rule.connect_module,
            insertion.continuous,
            insertion.discrete,
            rule.continuous.direction,
            rule.discrete.direction
        );
        if let Some(name) = self.identities.get(&key) {
            return Ok(name.clone());
        }
        let mut transition = SemanticAnalyzer::SIMULATOR_DEFAULT_TRANSITION;
        let mut default_discipline = None;
        let mut body = None;
        for item in &analyzed.source.items {
            match item {
                Item::DefaultTransition(directive) => {
                    transition =
                        SemanticAnalyzer::eval_const_with(&directive.value, &HashMap::new())
                            .ok_or_else(|| {
                                error(
                                    "invalid default transition in connect source",
                                    directive.span,
                                )
                            })?;
                }
                Item::DefaultDiscipline(directive) => {
                    default_discipline = directive.discipline.clone()
                }
                Item::ConnectModule(module) if module.name == rule.connect_module => {
                    body = Some(module.clone());
                    break;
                }
                _ => {}
            }
        }
        let mut body = body.ok_or_else(|| {
            error(
                format!(
                    "selected connect module '{}' has no retained source body",
                    rule.connect_module,
                ),
                rule.span,
            )
        })?;
        let mut id = self.modules.len();
        let name = loop {
            let name: SmolStr = format!("__rspice_connect_{id}").into();
            if !sources.contains_key(&name) && !self.modules.contains_key(&name) {
                break name;
            }
            id += 1;
        };
        fn apply_ports(
            body: &mut Module,
            name: &SmolStr,
            rule: &InsertionRule,
            insertion: &ConnectModuleInsertion,
        ) {
            body.name = name.clone();
            body.port_declarations = body
                .port_declarations
                .iter()
                .flat_map(|port| {
                    port.names.iter().map(|port_name| {
                        let mut port = port.clone();
                        port.names = vec![port_name.clone()];
                        for (selected, discipline) in [
                            (&rule.continuous, &insertion.continuous),
                            (&rule.discrete, &insertion.discrete),
                        ] {
                            if *port_name == selected.name {
                                port.direction = selected.direction;
                                port.discipline = Some(discipline.clone());
                            }
                        }
                        port
                    })
                })
                .collect();
            body.nets = body
                .nets
                .iter()
                .flat_map(|net| {
                    net.names.iter().map(|net_name| {
                        let mut net = net.clone();
                        net.names = vec![net_name.clone()];
                        if *net_name == rule.continuous.name {
                            net.discipline = Some(insertion.continuous.clone());
                        }
                        if *net_name == rule.discrete.name {
                            net.discipline = Some(insertion.discrete.clone());
                        }
                        net
                    })
                })
                .collect();
            if let Some(template) = &mut body.generate_template {
                apply_ports(&mut template.module, name, rule, insertion);
            }
        }
        apply_ports(&mut body, &name, rule, insertion);
        let module = analyze_occurrence(analyzed, &body, transition, default_discipline)?;
        // A natureless discrete discipline is still discrete storage; conversely
        // an overridden continuous port must remain a physical unknown.
        for (port, domain) in [
            (&rule.continuous.name, Domain::Continuous),
            (&rule.discrete.name, Domain::Discrete),
        ] {
            let discrete = module
                .digital
                .signals
                .iter()
                .any(|signal| &signal.name == port);
            if discrete != (domain == Domain::Discrete) {
                return Err(error(
                    format!(
                        "connect module '{}' port '{port}' has incompatible executable storage",
                        rule.connect_module
                    ),
                    rule.span,
                ));
            }
        }
        self.modules.insert(
            name.clone(),
            Arc::new(SpecializedModule {
                source: body,
                analyzed: module,
            }),
        );
        self.identities.insert(key, name.clone());
        Ok(name)
    }
}
