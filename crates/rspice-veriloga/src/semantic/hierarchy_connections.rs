//! Materialize planned connect instances in each concrete HDL occurrence.
//!
//! Both hierarchy lowerings consume the rewritten occurrence. The connection
//! planner owns selection, merged/split grouping and generated instance names;
//! this adapter supplies typed segments and ordinary executable module bodies.

mod actual;
mod compatibility;
pub(super) use compatibility::net_operands;
mod inputs;
mod net_arrays;
mod packed;

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
    pub warnings: Vec<super::SemanticWarning>,
    pub specializations: HashMap<SpecializationKey, Arc<SpecializedModule>>,
    pub prepared: HashMap<(SpecializationKey, SmolStr), Option<Arc<SpecializedModule>>>,
    pub resolved_types: HashMap<SmolStr, Arc<SpecializedModule>>,
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

#[derive(Clone)]
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

pub(super) fn declared_discipline(
    source: &Module,
    module: &AnalyzedModule,
    name: &str,
) -> Option<SmolStr> {
    source
        .nets
        .iter()
        .filter(|net| net.names.iter().any(|candidate| candidate == name))
        .find_map(|net| net.discipline.clone())
        .or_else(|| {
            source
                .port_declarations
                .iter()
                .find(|port| port.names.iter().any(|candidate| candidate == name))
                .and_then(|port| port.discipline.clone())
        })
        .or_else(|| {
            module
                .physical_nodes
                .real_aliases
                .get(name)
                .and_then(|alias| {
                    source
                        .nets
                        .iter()
                        .filter(|net| net.names.contains(&alias.array))
                        .find_map(|net| net.discipline.clone())
                        .or_else(|| {
                            source
                                .port_declarations
                                .iter()
                                .find(|port| port.names.contains(&alias.array))
                                .and_then(|port| port.discipline.clone())
                        })
                })
        })
}

fn endpoint(source: &Module, module: &AnalyzedModule, name: &str) -> Option<Endpoint> {
    if let Some(signal) = module
        .digital
        .signals
        .iter()
        .find(|signal| signal.name == name)
    {
        let discipline = declared_discipline(source, module, name)
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
    sites: HashMap<usize, ConnectionSite>,
}

struct ConnectionSite {
    kind: Option<DigitalNetKind>,
    target: ConnectionTarget,
}

enum ConnectionTarget {
    Scalar { instance: usize, port: usize },
    Packed { net: SmolStr, bit: u32 },
}

/// Insert at typed module boundaries after generate specialization. Concrete
/// physical vector lanes, packed bits and unpacked elements retain their own
/// signal identities and connecting-scope assignments.
pub(super) fn prepare(
    analyzed: &AnalyzedFile,
    sources: &HashMap<SmolStr, &Module>,
    source: &Module,
    module: &AnalyzedModule,
    bodies: &mut ConnectionModules,
    path: &str,
) -> CompileResult<Option<Arc<SpecializedModule>>> {
    let array_views = net_arrays::prepare(analyzed, source, module)?;
    let (source, module) = array_views
        .as_deref()
        .map(|prepared| (&prepared.source, &prepared.analyzed))
        .unwrap_or((source, module));
    Ok(prepare_boundaries(analyzed, sources, source, module, bodies, path)?.or(array_views))
}

fn prepare_boundaries(
    analyzed: &AnalyzedFile,
    sources: &HashMap<SmolStr, &Module>,
    source: &Module,
    module: &AnalyzedModule,
    bodies: &mut ConnectionModules,
    path: &str,
) -> CompileResult<Option<Arc<SpecializedModule>>> {
    if source.instances.is_empty() {
        return Ok(None);
    }
    let has_rules = !analyzed.connect_rules.insertions().is_empty();
    let mut prepared_inputs = false;
    let mut input_types = None;
    let mut signals: BTreeMap<SignalIdentity, BoundarySignal> = BTreeMap::new();
    let scope = super::node_vectors::ConnectionScope::new(source, module);
    let mut prepared = source.clone();
    let mut used = declared_names(source);
    let mut aliases = module.digital.bit_aliases.clone();
    let constants = super::instance_parameters::constants(source);
    for (instance_index, instance) in source.instances.iter().enumerate() {
        let Some(child) = analyzed.modules.get(&instance.module) else {
            continue;
        };
        if !has_rules && child.digital.signals.is_empty() {
            continue;
        }
        let Some(child_source) = sources.get(&instance.module) else {
            continue;
        };
        let child_path = if path.is_empty() {
            instance.name.to_string()
        } else {
            format!("{path}.{}", instance.name)
        };
        let (_, specialized) = specialize_module(
            analyzed,
            &mut bodies.specializations,
            instance,
            child_source,
            child,
            &constants,
            source.time_scale,
            &child_path,
        )?;
        let (child_source, child) = specialized
            .as_deref()
            .map(|module| (&module.source, &module.analyzed))
            .unwrap_or((child_source, child));
        let resolved = bodies.resolved_types.get(child_path.as_str()).cloned();
        let (child_source, child) = resolved
            .as_deref()
            .map(|module| (&module.source, &module.analyzed))
            .unwrap_or((child_source, child));
        let connections =
            super::node_vectors::bind_connections(instance, child, &scope, &instance.name)?;
        prepared.instances[instance_index].connections = child
            .ports
            .iter()
            .zip(&connections)
            .map(|(port, signal)| Connection::Named {
                port: port.name.clone(),
                signal: signal.clone(),
                span: instance.span,
            })
            .collect();
        for (port_index, actual) in connections.iter().enumerate() {
            let Some(actual) = actual else { continue };
            let port = &child.ports[port_index];
            let Some(lower) = endpoint(child_source, child, &port.name) else {
                continue;
            };
            compatibility::check_actual(
                analyzed, source, module, actual, &lower, &child_path, &port.name,
            )?;
            if port.direction == PortDirection::Input
                && lower.net_kind.is_some()
                && !scope.contains_physical(actual)
                && let Some(declared) = child
                    .digital
                    .signals
                    .iter()
                    .find(|signal| signal.name == port.name)
                && inputs::requires_assignment(actual, module, declared)
            {
                inputs::insert(
                    &mut prepared,
                    &mut used,
                    child,
                    &lower,
                    (instance_index, port_index),
                    actual,
                );
                prepared_inputs = true;
                continue;
            }
            if !has_rules {
                continue;
            }
            if port.direction == PortDirection::Input
                && lower.net_kind.is_some()
                && lower.width != 0
                && (lower.width > 1 || matches!(actual, Expression::ArrayLiteral(_)))
                && scope.contains_physical(actual)
            {
                if input_types.is_none() {
                    input_types = Some(
                        crate::canonical_ir::digital_lower::ConnectionShapes::new(module, source)
                            .map_err(|diagnostics| {
                            error(
                                diagnostics
                                    .iter()
                                    .map(|entry| entry.diagnostic.to_string())
                                    .collect::<Vec<_>>()
                                    .join("; "),
                                actual.span(),
                            )
                        })?,
                    );
                }
                packed::Connections {
                    source,
                    module,
                    constants: &constants,
                    prepared: &mut prepared,
                    used: &mut used,
                    signals: &mut signals,
                    aliases: &mut aliases,
                }
                .connect_input(
                    child,
                    instance,
                    (instance_index, port_index),
                    &lower,
                    actual,
                    &scope,
                    input_types
                        .as_mut()
                        .expect("prepared input expression types"),
                )?;
                continue;
            }
            if lower.net_kind.is_some()
                && lower.width > 1
                && let Some(lanes) = scope.physical_selection(actual)?
            {
                packed::Connections {
                    source,
                    module,
                    constants: &constants,
                    prepared: &mut prepared,
                    used: &mut used,
                    signals: &mut signals,
                    aliases: &mut aliases,
                }
                .connect(
                    child,
                    instance,
                    (instance_index, port_index),
                    &lower,
                    &lanes,
                )?;
                continue;
            }
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
            let connection_index = port_index;
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
                ConnectionSite {
                    kind: lower.net_kind,
                    target: ConnectionTarget::Scalar {
                        instance: instance_index,
                        port: connection_index,
                    },
                },
            );
        }
    }
    let has_boundaries = !signals.is_empty();
    if !has_boundaries && !prepared_inputs {
        return Ok(None);
    }
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
            let lower_kind = boundary.sites[&insertion.bindings[0].lower].kind;
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
                dimensions: Vec::new(),
                range: None,
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
                name: private.clone(),
                span,
            });
            for binding in &insertion.bindings {
                let site = &boundary.sites[&binding.lower];
                if site.kind != lower_kind {
                    return Err(error(
                        "merged connection has incompatible discrete net resolution types",
                        span,
                    ));
                }
                match &site.target {
                    ConnectionTarget::Scalar { instance, port } => {
                        match &mut prepared.instances[*instance].connections[*port] {
                            Connection::Named { signal, .. }
                            | Connection::Ordered { signal, .. } => {
                                *signal = Some(private_expression.clone());
                            }
                        }
                    }
                    ConnectionTarget::Packed { net, bit } => {
                        aliases.push(super::digital::ElaboratedDigitalBitAlias {
                            left: private.clone(),
                            left_bit: 0,
                            right: net.clone(),
                            right_element: None,
                            right_bit: *bit,
                            span,
                        });
                    }
                }
            }
            let upper = if lower_kind.is_none()
                && rule.discrete.direction != PortDirection::Inout
                && !matches!(
                    boundary.actual,
                    Expression::Identifier(_) | Expression::Number(_)
                ) {
                let mut tap: SmolStr = format!("{}__actual", insertion.instance).into();
                while !used.insert(tap.clone()) {
                    tap = format!("{tap}_").into();
                }
                prepared.nets.push(NetDecl {
                    dimensions: Vec::new(),
                    range: None,
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
    // Converter insertion replaces authored selectors with concrete lanes. Keep
    // the dependencies already recorded before that rewrite, or a scalar
    // parameter update could leave the selected topology unchanged.
    retain_parameter_guards(module, &mut analyzed);
    analyzed.hierarchical_connections = module.hierarchical_connections || has_boundaries;
    analyzed.digital.bit_aliases = aliases;
    Ok(Some(Arc::new(SpecializedModule {
        source: prepared,
        analyzed,
    })))
}

fn retain_parameter_guards(module: &AnalyzedModule, analyzed: &mut AnalyzedModule) {
    // Boundary insertion reanalyzes source, which cannot encode storage aliases.
    // Keep those identities alongside the parameter specialization guards.
    let element_aliases: HashMap<_, _> = module
        .digital
        .signals
        .iter()
        .filter_map(|signal| {
            signal
                .element_alias
                .as_ref()
                .map(|alias| (&signal.name, alias))
        })
        .collect();
    for signal in &mut analyzed.digital.signals {
        if let Some(alias) = element_aliases.get(&signal.name) {
            signal.element_alias = Some((*alias).clone());
        }
    }
    let original_parameters: HashMap<_, _> = module
        .parameters
        .iter()
        .map(|parameter| (&parameter.name, parameter))
        .collect();
    for parameter in &mut analyzed.parameters {
        if let Some(original) = original_parameters.get(&parameter.name) {
            parameter.elaboration_value =
                parameter.elaboration_value.or(original.elaboration_value);
            parameter.elaboration_given =
                parameter.elaboration_given.or(original.elaboration_given);
        }
    }
}

pub(super) fn declared_names(source: &Module) -> HashSet<SmolStr> {
    source.declared_names()
}

pub(super) fn analyze_occurrence(
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
