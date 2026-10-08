//! Elaboration of a digital module hierarchy into one flat scope.
//!
//! A Verilog design is a tree of module instances and a simulation is a flat
//! set of processes over a flat set of nets. This pass is the step between the
//! two, for the discrete half of the language: it walks the instance tree of
//! the module being compiled and emits one
//! [`ElaboratedDigitalInstance`](super::ElaboratedDigitalInstance) per
//! instance, so that the canonical-IR lowering sees a list of frames rather
//! than a tree and produces one plan.
//!
//! The analog and digital lowerings share concrete specialized occurrences.
//! This pass binds digital nets and retains each occurrence's source and analysis;
//! analog elaboration consumes that same specialization and supplies its relocated
//! state and probe bindings. A mixed child therefore retains both domains.
//!
//! # What a port connection means
//!
//! This is the design decision of the pass, so it is stated in full. IEEE
//! 1364-2005 section 12.3.9 gives a port connection two readings, and the
//! standard uses both: a connection can be a *net collapse*, joining the two
//! nets into one, or an *implicit continuous assignment* from one side to the
//! other. Which reading applies here is decided by the port's own declared
//! class, not by its direction:
//!
//! * **A port declared as a net collapses.** The port and the net it is
//!   connected to become one elaborated signal, named by the connecting scope.
//!   Section 12.3.9.3 makes an inout connection exactly this — a bidirectional
//!   join, which no assignment in either direction describes — and section
//!   12.3.10, which asks what net type results when two dissimilar nets are
//!   connected, only has a question to answer because the two nets *become
//!   one*. For an input or an output the collapse and the assignment readings
//!   are observationally identical whenever both sides are plain nets and no
//!   delay is written, because a continuous assignment with one driver and no
//!   delay reproduces its source exactly; the collapse is chosen because it
//!   costs no process, no driver, and no scheduling delta. The cases where the
//!   two readings *do* differ are refused below rather than silently resolved
//!   in favour of the cheaper one.
//!
//! * **A bit- or part-select connection becomes an implicit continuous
//!   assignment**, in the direction the port's own direction gives it. Some of
//!   a net's bits are not a net, so there is nothing to join: an input port is
//!   *driven from* the selected bits and an output port *drives* them, and each
//!   is a real driver with a real identity on a real net. Driving four bits of
//!   an eight-bit net is what a driver's write *select* exists for, so the
//!   other four keep whatever else drives them.
//!
//! * **A variable output port becomes an implicit continuous assignment.**
//!   Section 12.3.9.2 permits an output port to be a variable, and a variable
//!   cannot be collapsed with a net: one holds a value written procedurally,
//!   the other is the resolution of its drivers. So `output q; reg q;` keeps
//!   its own elaborated signal and the connection becomes a driver on the
//!   connected net — a real
//!   [`DigitalDriver`](crate::canonical_ir::DigitalDriver) with a real
//!   identity, indistinguishable from an `assign` the parent could have
//!   written itself.
//!
//! Collapsing an output port does not make its driver invisible. A driver is
//! identified by net and by index among that net's drivers, so two instances
//! driving one net through collapsed output ports produce two drivers of that
//! net with indices 0 and 1, and a resolver sees both. That property is the
//! reason a driver identity exists at all, and it is what makes collapsing
//! safe here.
//!
//! Numeric child parameter overrides are closed in the parent's effective scope,
//! with the child's declared assignment type. Equal specializations share an
//! analyzed template, while each instance retains independent storage and process
//! identities. Widths, initializers and generated structure are rebuilt together.
//! Conditional generate can therefore terminate recursive module instantiation.
//! The explicit traversal stack detects repeating specializations on an ancestor
//! path and bounds depth and total instances without using the host call stack.
//!
//! # What is refused
//!
//! Every refusal names the construct and the clause. Nothing is dropped.
//!
//! * a port connection that is neither a declared net nor a bit- or
//!   part-select of one — an arbitrary expression, a constant, a concatenation
//!   (section 12.3.9);
//! * a bit- or part-select connected to an `inout` port, because section
//!   12.3.9.3 makes that connection a bidirectional join of two nets and no
//!   assignment in either direction describes one;
//! * a connection naming something that is not a declared discrete-domain
//!   signal, because this compiler does not create implicit nets (section
//!   4.5);
//! * a net port whose width differs from the net it is connected to, because two
//!   collapsed nets are one net and one net has one width (section 12.3.9);
//! * an `input` or `inout` port declared as a variable (section 12.3.3);
//! * an output or inout port connected to a variable, or to anything the
//!   connecting scope sees as an input port, because either would let the
//!   instance drive what it must not (section 12.3.9.1);
//! * a repeating specialization on one ancestor path, or a hierarchy exceeding
//!   the documented resource limits, as a source-located error.

use super::{
    AnalyzedContinuousAssign, AnalyzedFile, AnalyzedModule, ElaboratedDigitalInstance,
    ElaboratedDigitalSignal,
};
use crate::ast::{
    ArrayAccessExpr, ContinuousAssign, DigitalExpr, DigitalLValue, Expression,
    Identifier, Module, ModuleInstance, PartSelectExpr, PortDirection,
};
use crate::error::{CompileError, CompileResult, SemanticError, SemanticErrorKind};
use crate::source::Span;
use smol_str::SmolStr;
use std::collections::{HashMap, HashSet};

/// Elaborate the digital instance tree of `root` into a flat frame list.
///
/// Returns an empty list when nothing in the tree is digital, which is every
/// continuous-domain hierarchy this compiler has ever compiled: the pass is
/// reached only through [`super::elaborate_executable_module`], and a module
/// with no digital child leaves it exactly as it found it.
pub(crate) fn elaborate_digital_hierarchy(
    analyzed: &AnalyzedFile,
    source_modules: &HashMap<SmolStr, &Module>,
    root_source: &Module,
    root: &AnalyzedModule,
) -> CompileResult<ElaboratedHierarchy> {
    let mut elaborator = DigitalElaborator {
        analyzed,
        source_modules,
        instances: Vec::new(),
        occurrences: HashMap::new(),
        required: digital_subtrees(analyzed, source_modules),
        connections: Default::default(),
    };
    let prepared_root = super::hierarchy_connections::prepare(
        analyzed,
        source_modules,
        root_source,
        root,
        &mut elaborator.connections,
    )?;
    let (root_source, root) = prepared_root
        .as_deref()
        .map(|root| (&root.source, &root.analyzed))
        .unwrap_or((root_source, root));
    elaborator.append_instances(root_source, Scope::for_root(root, root_source))?;
    Ok(ElaboratedHierarchy {
        root: prepared_root,
        instances: elaborator.instances,
        occurrences: elaborator.occurrences,
    })
}

/// One specialization per occurrence, shared by both domain lowerings.
pub(super) struct ElaboratedHierarchy {
    pub root: Option<std::sync::Arc<SpecializedModule>>,
    pub instances: Vec<ElaboratedDigitalInstance>,
    pub occurrences: HashMap<SmolStr, std::sync::Arc<SpecializedModule>>,
}

/// Include analog containers of digital descendants, including currently inactive
/// generate arms. Pure analog subtrees keep their symbolic parameter-array path.
fn digital_subtrees(
    analyzed: &AnalyzedFile,
    sources: &HashMap<SmolStr, &Module>,
) -> HashSet<SmolStr> {
    use crate::ast::GenerateConstruct;
    let mut required = HashSet::new();
    let mut parents: HashMap<SmolStr, Vec<SmolStr>> = HashMap::new();
    for (name, source) in sources {
        let mut modules = vec![*source];
        while let Some(module) = modules.pop() {
            if module.has_digital_content()
                || module
                    .nets
                    .iter()
                    .filter_map(|net| net.discipline.as_ref())
                    .chain(
                        module
                            .port_declarations
                            .iter()
                            .filter_map(|port| port.discipline.as_ref()),
                    )
                    .any(|name| {
                        analyzed
                            .disciplines
                            .get_discipline(name)
                            .is_some_and(|discipline| {
                                discipline.domain == crate::disciplines::Domain::Discrete
                            })
                    })
            {
                required.insert(name.clone());
            }
            for instance in &module.instances {
                parents
                    .entry(instance.module.clone())
                    .or_default()
                    .push(name.clone());
            }
            if let Some(template) = &module.generate_template {
                modules.push(&template.module);
            }
            let mut constructs: Vec<_> = module.generates.iter().collect();
            while let Some(construct) = constructs.pop() {
                let blocks: Vec<_> = match construct {
                    GenerateConstruct::Loop(value) => vec![&value.body],
                    GenerateConstruct::Conditional(value) => std::iter::once(&value.then_block)
                        .chain(value.else_block.iter())
                        .collect(),
                    GenerateConstruct::Case(value) => value
                        .items
                        .iter()
                        .map(|item| &item.block)
                        .chain(value.default.iter())
                        .collect(),
                    GenerateConstruct::Block(block) => vec![block],
                };
                for block in blocks {
                    modules.push(&block.items);
                    constructs.extend(&block.nested);
                }
            }
        }
        if analyzed
            .modules
            .get(name)
            .is_some_and(|module| module.digital.has_executable_content())
        {
            required.insert(name.clone());
        }
    }
    let mut pending: Vec<_> = required.iter().cloned().collect();
    while let Some(name) = pending.pop() {
        for parent in parents.get(&name).into_iter().flatten() {
            if required.insert(parent.clone()) {
                pending.push(parent.clone());
            }
        }
    }
    required
}

/// How the elaborated scope sees one name.
#[derive(Debug, Clone)]
struct Binding {
    /// The name this signal has in the flat scope.
    elaborated: SmolStr,
    width: u32,
    /// Whether the elaborated signal is a variable (`reg`) rather than a net.
    is_variable: bool,
    /// Whether *this view* of the signal is an input port.
    ///
    /// Accumulated down the hierarchy rather than read off one declaration: a
    /// net that the parent may drive is still one the child may not, once the
    /// child receives it through an `input` port. Nothing below the port may
    /// drive it, however many levels down the driver is written.
    is_input_port: bool,
}

/// The discrete-domain names one module body resolves against.
#[derive(Debug, Default)]
struct Scope {
    connections: super::node_vectors::ConnectionScope,
    signals: HashMap<SmolStr, Binding>,
    constants: super::DigitalConstants,
    time_scale: crate::time_scale::ModuleTimeScale,
}

impl Scope {
    fn for_root(root: &AnalyzedModule, source: &Module) -> Self {
        let mut scope = Self {
            connections: super::node_vectors::ConnectionScope::new(source, root),
            constants: super::DigitalConstants::from_module(source),
            time_scale: source.time_scale,
            ..Self::default()
        };
        for signal in &root.digital.signals {
            scope.signals.insert(
                signal.name.clone(),
                Binding {
                    elaborated: signal.name.clone(),
                    width: signal.width,
                    is_variable: signal.class.is_variable(),
                    // The compiled module's own ports are its boundary with
                    // the rest of the circuit, not something an enclosing
                    // module drives, so nothing here is an input port yet.
                    is_input_port: false,
                },
            );
        }
        scope
    }
}

struct DigitalElaborator<'a> {
    connections: super::hierarchy_connections::ConnectionModules,
    analyzed: &'a AnalyzedFile,
    source_modules: &'a HashMap<SmolStr, &'a Module>,
    instances: Vec<ElaboratedDigitalInstance>,
    occurrences: HashMap<SmolStr, std::sync::Arc<SpecializedModule>>,
    required: HashSet<SmolStr>,
}

fn check_hierarchy_capacity(
    depth: usize,
    instances: usize,
    path: &str,
    span: Span,
) -> CompileResult<()> {
    let detail = if depth > MAX_DIGITAL_HIERARCHY_DEPTH {
        format!("exceeds the digital hierarchy depth limit of {MAX_DIGITAL_HIERARCHY_DEPTH}")
    } else if instances >= MAX_DIGITAL_HIERARCHY_INSTANCES {
        format!("exceeds the digital hierarchy instance limit of {MAX_DIGITAL_HIERARCHY_INSTANCES}")
    } else {
        return Ok(());
    };
    Err(semantic_error(
        SemanticErrorKind::UnsupportedFeature(format!("instance `{path}` {detail}")),
        span,
    ))
}

#[derive(Debug, Clone)]
pub(super) struct SpecializedModule {
    pub source: Module,
    pub analyzed: AnalyzedModule,
}

/// These bounds apply to the discrete hierarchy below the compiled root.
/// They limit non-repeating expansion as well as finite but oversized designs.
const MAX_DIGITAL_HIERARCHY_DEPTH: usize = 256;
const MAX_DIGITAL_HIERARCHY_INSTANCES: usize = 65_536;

/// Closed explicit overrides identify a deterministic source specialization.
/// An omitted override and an explicit value equal to its default may have
/// different keys; this can delay cycle detection, but cannot
/// reject a finite hierarchy or permit unbounded expansion.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(super) struct SpecializationKey {
    module: SmolStr,
    overrides: Vec<(usize, String)>,
}

struct HierarchyFrame {
    key: SpecializationKey,
    path: SmolStr,
    scope: Scope,
    pending: std::vec::IntoIter<ModuleInstance>,
    seen: HashSet<SmolStr>,
}

impl DigitalElaborator<'_> {
    fn append_instances(&mut self, source: &Module, scope: Scope) -> CompileResult<()> {
        let mut stack = vec![HierarchyFrame {
            key: SpecializationKey {
                module: source.name.clone(),
                overrides: Vec::new(),
            },
            path: "".into(),
            scope,
            pending: source.instances.clone().into_iter(),
            seen: HashSet::new(),
        }];
        // Depth-first source order is retained, including signal/process IDs.
        // Finished siblings leave the stack, so sharing their specialization
        // is not mistaken for an ancestor cycle.
        while let Some(mut frame) = stack.pop() {
            let Some(instance) = frame.pending.next() else {
                continue;
            };
            if !frame.seen.insert(instance.name.clone()) {
                return Err(semantic_error(
                    SemanticErrorKind::DuplicateSymbol {
                        name: instance.name.clone(),
                        first_defined: instance.span,
                    },
                    instance.span,
                ));
            }
            let connection = self.connections.modules.get(&instance.module).cloned();
            let child = connection
                .as_deref()
                .map(|module| &module.analyzed)
                .or_else(|| self.analyzed.modules.get(&instance.module))
                .ok_or_else(|| {
                    semantic_error(
                        SemanticErrorKind::UndefinedModule(instance.module.to_string()),
                        instance.span,
                    )
                })?;
            let path = qualify(&frame.path, &instance.name);
            stack.push(frame);
            if connection.is_none() && !self.required.contains(&instance.module) {
                continue;
            }
            check_hierarchy_capacity(stack.len(), self.instances.len(), &path, instance.span)?;
            let next = self.append_instance(&instance, child, &stack, &path)?;
            stack.push(next);
        }
        Ok(())
    }

    fn append_instance(
        &mut self,
        instance: &ModuleInstance,
        child: &AnalyzedModule,
        ancestors: &[HierarchyFrame],
        path: &str,
    ) -> CompileResult<HierarchyFrame> {
        let parent_scope = &ancestors.last().expect("instance parent").scope;
        let connection = self.connections.modules.get(&instance.module).cloned();
        let child_source = connection
            .as_deref()
            .map(|module| &module.source)
            .or_else(|| self.source_modules.get(&instance.module).copied())
            .ok_or_else(|| {
                internal_error(format!(
                    "digital module '{}' was analyzed but not retained",
                    instance.module
                ))
            })?;

        let (key, specialized) = specialize_module(
            self.analyzed,
            &mut self.connections.specializations,
            instance,
            child_source,
            child,
            &parent_scope.constants,
            parent_scope.time_scale,
            path,
        )?;
        if let Some(first) = ancestors.iter().position(|frame| frame.key == key) {
            let mut cycle: Vec<_> = ancestors[first..]
                .iter()
                .map(|frame| frame.key.module.as_str())
                .collect();
            cycle.push(instance.module.as_str());
            let origin = &ancestors[first].path;
            let origin = if origin.is_empty() {
                "<root>"
            } else {
                origin.as_str()
            };
            return Err(semantic_error(
                SemanticErrorKind::CircularDependency(format!(
                    "digital module hierarchy {} at instance '{path}': specialization repeats ancestor '{origin}'",
                    cycle.join(" -> ")
                )),
                instance.span,
            ));
        }
        let (child_source, child) = specialized
            .as_deref()
            .map(|specialized| (&specialized.source, &specialized.analyzed))
            .unwrap_or((child_source, child));

        let connected = if let Some(prepared) = self.connections.prepared.get(&key) {
            prepared.clone()
        } else {
            let prepared = super::hierarchy_connections::prepare(
                self.analyzed,
                self.source_modules,
                child_source,
                child,
                &mut self.connections,
            )?;
            self.connections
                .prepared
                .insert(key.clone(), prepared.clone());
            prepared
        };
        let (child_source, child) = connected
            .as_deref()
            .map(|module| (&module.source, &module.analyzed))
            .unwrap_or((child_source, child));

        let connections = super::node_vectors::bind_connections(
            instance,
            child,
            &parent_scope.connections,
            path,
        )?;
        let borrowed: Vec<_> = connections.iter().map(Option::as_ref).collect();
        let (signals, mut scope, port_drivers) =
            self.bind_ports(instance, child, parent_scope, path, &borrowed)?;
        scope.connections = super::node_vectors::ConnectionScope::new(child_source, child);

        // A continuous assignment driving one of the child's own `input` ports
        // is *not* checked here. Semantic analysis refuses it on the module
        // itself, before the module is instantiated at all — which is where the
        // check belongs, because the prohibition is a property of the module's
        // own text and holds whether or not anybody instantiates it. What is
        // left for the connection to police is the other half: an output or
        // inout port connected to something the connecting scope receives
        // through an input port, which `bind_ports` refuses above.

        let retained = connected
            .clone()
            .or_else(|| specialized.clone())
            .unwrap_or_else(|| {
                self.connections
                    .specializations
                    .entry(key.clone())
                    .or_insert_with(|| {
                        std::sync::Arc::new(SpecializedModule {
                            source: child_source.clone(),
                            analyzed: child.clone(),
                        })
                    })
                    .clone()
            });
        self.occurrences.insert(path.into(), retained);
        self.instances.push(ElaboratedDigitalInstance {
            analog_events: child.digital.analog_events.clone(),
            event_assigned_variables: child.digital.event_assigned_variables.clone(),
            immutable_analog_variables: child.digital.immutable_analog_variables.clone(),
            analog_variables: HashMap::new(),
            time_scale: child.digital.time_scale,
            elaboration_parameters: child.digital.elaboration_parameters.clone(),
            path: path.into(),
            module: instance.module.clone(),
            signals,
            processes: child.digital.processes.clone(),
            continuous_assigns: child.digital.continuous_assigns.clone(),
            port_drivers,
            constants: child.digital.constants.clone(),
            span: instance.span,
        });

        Ok(HierarchyFrame {
            key,
            path: path.into(),
            scope,
            pending: child_source.instances.clone().into_iter(),
            seen: HashSet::new(),
        })
    }

    /// Resolve one instance's ports into elaborated names.
    ///
    /// Returns the instance's signals with the name each takes in the flat
    /// scope, the scope its body resolves against, and the implicit continuous
    /// assignments its variable output ports produce.
    fn bind_ports(
        &self,
        instance: &ModuleInstance,
        child: &AnalyzedModule,
        parent_scope: &Scope,
        path: &str,
        connections: &[Option<&Expression>],
    ) -> CompileResult<(
        Vec<ElaboratedDigitalSignal>,
        Scope,
        Vec<AnalyzedContinuousAssign>,
    )> {
        let mut bindings: HashMap<SmolStr, Binding> = HashMap::new();
        let mut port_drivers = Vec::new();

        for (index, port) in child.ports.iter().enumerate() {
            let Some(declared) = child
                .digital
                .signals
                .iter()
                .find(|signal| signal.name == port.name)
            else {
                // Continuous ports are bound by the analog lowering of this
                // same occurrence. They never become discrete signal storage.
                continue;
            };
            let is_variable = declared.class.is_variable();
            if is_variable && port.direction != PortDirection::Output {
                return Err(semantic_error(
                    SemanticErrorKind::InvalidContribution(format!(
                        "`{}`, which module `{}` declares both a `{}` port and a `{}`; IEEE \
                         1364-2005 section 12.3.3 lets only an output port be a variable",
                        port.name,
                        instance.module,
                        direction_keyword(port.direction),
                        declared.class.keyword()
                    )),
                    declared.span,
                ));
            }

            let resizes_variable = is_variable && !declared.class.is_real();
            let own = qualify(path, &port.name);
            let form = connections[index]
                .map(|expression| connection_form(expression, path, &port.name))
                .transpose()?;
            let binding = match form {
                // An unconnected port is a net of its own. IEEE 1364-2005
                // section 12.3.9 leaves an unconnected input at high
                // impedance, which is what a net nothing drives already is.
                None => Binding {
                    elaborated: own,
                    width: declared.width,
                    is_variable,
                    is_input_port: port.direction == PortDirection::Input,
                },
                // Some of a net's bits, not a net. The two cannot be joined —
                // one net has one width — so section 12.3.9's *other* reading
                // applies: the port keeps its own signal and the connection is
                // an implicit continuous assignment, in whichever direction the
                // port's own direction makes it.
                Some(ConnectionForm::Select { name, select, span }) => {
                    let outer = lookup_connection(name, parent_scope, path, &port.name, span)?;
                    let width = select.width(span)?;
                    if width != declared.width && !resizes_variable {
                        return Err(semantic_error(
                            SemanticErrorKind::TypeMismatch {
                                expected: format!("{}-bit connection", declared.width),
                                found: format!("a {width}-bit select of `{}`", outer.elaborated),
                                context: format!(
                                    "port `{}` of instance `{path}`; IEEE 1364-2005 section \
                                     12.3.9 connects a port to a differently sized expression \
                                     by truncating or extending it, and this compiler requires \
                                     the two to agree",
                                    port.name
                                ),
                            },
                            span,
                        ));
                    }
                    match port.direction {
                        PortDirection::Input => {
                            // The parent drives the port: `assign u1.a = bus[3];`
                            port_drivers.push(assignment(
                                DigitalLValue::Identifier {
                                    name: own.clone(),
                                    span,
                                },
                                select.expression(&outer.elaborated, span),
                                span,
                            ));
                        }
                        PortDirection::Output => {
                            if outer.is_variable {
                                return Err(variable_connection_error(&outer, port, path, span));
                            }
                            if outer.is_input_port {
                                return Err(input_port_connection_error(&outer, port, path, span));
                            }
                            // The port drives some of the parent's bits:
                            // `assign bus[3] = u1.y;`
                            port_drivers.push(assignment(
                                select.lvalue(&outer.elaborated, span),
                                Expression::Identifier(Identifier {
                                    name: own.clone(),
                                    span,
                                }),
                                span,
                            ));
                        }
                        // Section 12.3.9.3 makes an inout connection a
                        // bidirectional join of two nets. A select is not a
                        // net, and neither direction of assignment describes
                        // the join, so there is nothing honest to build.
                        PortDirection::Inout => {
                            return Err(semantic_error(
                                SemanticErrorKind::UnsupportedFeature(format!(
                                    "the `inout` port `{}` of instance `{path}` is connected to \
                                     a select of `{}`; IEEE 1364-2005 section 12.3.9.3 makes an \
                                     inout connection a bidirectional join of two nets, which no \
                                     assignment in either direction describes — connect a \
                                     declared net by name",
                                    port.name, outer.elaborated
                                )),
                                span,
                            ));
                        }
                    }
                    Binding {
                        elaborated: own,
                        width: declared.width,
                        is_variable,
                        is_input_port: port.direction == PortDirection::Input,
                    }
                }
                Some(ConnectionForm::Net(identifier)) => {
                    let span = identifier.span;
                    let outer =
                        lookup_connection(&identifier.name, parent_scope, path, &port.name, span)?;
                    if outer.width != declared.width && !(resizes_variable && outer.width != 0) {
                        return Err(semantic_error(
                            SemanticErrorKind::TypeMismatch {
                                expected: format!("{}-bit connection", declared.width),
                                found: format!(
                                    "`{}`, which is {} bits",
                                    outer.elaborated, outer.width
                                ),
                                context: format!(
                                    "port `{}` of instance `{path}`; IEEE 1364-2005 section \
                                     12.3.9 connects a port to a differently sized net by \
                                     truncating or extending it, and this compiler joins the \
                                     two into one net instead, which requires equal widths",
                                    port.name
                                ),
                            },
                            span,
                        ));
                    }
                    if port.direction != PortDirection::Input {
                        if outer.is_variable {
                            return Err(variable_connection_error(&outer, port, path, span));
                        }
                        if outer.is_input_port {
                            return Err(input_port_connection_error(&outer, port, path, span));
                        }
                    }

                    if is_variable {
                        // Section 12.3.9.2: the port keeps its own signal and
                        // the connection becomes a driver on the outer net.
                        port_drivers.push(implicit_port_assignment(&outer.elaborated, &own, span));
                        Binding {
                            elaborated: own,
                            width: declared.width,
                            is_variable: true,
                            is_input_port: false,
                        }
                    } else {
                        // Section 12.3.9.3 / 12.3.10: the two nets are one.
                        Binding {
                            elaborated: outer.elaborated,
                            width: outer.width,
                            is_variable: outer.is_variable,
                            is_input_port: outer.is_input_port
                                || port.direction == PortDirection::Input,
                        }
                    }
                }
            };
            bindings.insert(port.name.clone(), binding);
        }

        let mut signals = Vec::with_capacity(child.digital.signals.len());
        let mut scope = Scope {
            constants: child.digital.constants.clone(),
            time_scale: child.digital.time_scale,
            ..Scope::default()
        };
        for declared in &child.digital.signals {
            let binding = bindings.get(&declared.name).cloned().unwrap_or(Binding {
                elaborated: qualify(path, &declared.name),
                width: declared.width,
                is_variable: declared.class.is_variable(),
                is_input_port: false,
            });
            signals.push(ElaboratedDigitalSignal {
                declared: declared.clone(),
                name: binding.elaborated.clone(),
            });
            scope.signals.insert(declared.name.clone(), binding);
        }
        Ok((signals, scope, port_drivers))
    }
}

pub(super) fn specialize_module(
    analyzed: &AnalyzedFile,
    cache: &mut HashMap<SpecializationKey, std::sync::Arc<SpecializedModule>>,
    instance: &ModuleInstance,
    source: &Module,
    child: &AnalyzedModule,
    constants: &super::DigitalConstants,
    time_scale: crate::time_scale::ModuleTimeScale,
    path: &str,
) -> CompileResult<(SpecializationKey, Option<std::sync::Arc<SpecializedModule>>)> {
    if instance.parameters.is_empty() {
        validate_parameter_ranges(source, path)?;
        return Ok((
            SpecializationKey {
                module: instance.module.clone(),
                overrides: Vec::new(),
            },
            None,
        ));
    }
    let mut overrides: Vec<_> =
        super::elaboration::bind_parameter_overrides(instance, child, path)?
            .into_iter()
            .collect();
    overrides.sort_by_key(|(index, _)| *index);
    let mut values = Vec::new();
    let mut key = Vec::new();
    for (index, expression) in overrides {
        let declaration = &source.parameters[index];
        let span = expression.span();
        let value = super::instance_parameters::close_override(
            declaration,
            expression,
            constants,
            time_scale,
        )
        .map_err(|message| {
            semantic_error(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "parameter `{}` of instance `{path}`: {message}",
                    declaration.name
                )),
                span,
            )
        })?;
        let identity =
            super::instance_parameters::override_identity(&value).map_err(internal_error)?;
        key.push((index, identity));
        values.push((index, value));
    }
    let key = SpecializationKey {
        module: instance.module.clone(),
        overrides: key,
    };
    if let Some(specialized) = cache.get(&key) {
        return Ok((key, Some(specialized.clone())));
    }
    let mut source = source.clone();
    for (index, value) in values {
        source.parameters[index].default = Some(value);
        source.parameters[index].is_given = true;
    }
    validate_parameter_ranges(&source, path)?;
    crate::parser::expand_specialized_generates(&mut source)?;
    let mut analyzer = super::SemanticAnalyzer::new();
    analyzer.disciplines = analyzed.disciplines.clone();
    analyzer.current_default_transition = child.default_transition;
    let mut analyzed = analyzer.analyze_module(&source, child.default_transition)?;
    analyzed.default_discipline = child.default_discipline.clone();
    let specialized = std::sync::Arc::new(SpecializedModule { source, analyzed });
    cache.insert(key.clone(), specialized.clone());
    Ok((key, Some(specialized)))
}

/// Whether the module has continuous-domain content to flatten.
/// Match an instance's connections to the child's ports.
///
/// IEEE 1364-2005 sections 12.3.5 and 12.3.6 give the two forms, and section
/// 12.3.6 forbids mixing them in one instance.
/// The shapes a port connection may take.
///
/// IEEE 1364-2005 section 12.3.9 permits an arbitrary expression, and reads a
/// connection two ways depending on what it is. These are the two this compiler
/// can build something honest out of, and they take the two different readings:
/// a whole net collapses, a select becomes an implicit continuous assignment.
/// Everything else — a concatenation, a constant, an arithmetic expression —
/// is refused where it is written.
enum ConnectionForm<'a> {
    /// `.a(bus)` — the whole of a declared net.
    Net(&'a Identifier),
    /// `.a(bus[3])` or `.a(bus[7:4])` — some of a declared net's bits.
    Select {
        name: &'a SmolStr,
        select: SelectBounds<'a>,
        span: Span,
    },
}

/// Which bits of a net a connection selects.
enum SelectBounds<'a> {
    Bit(&'a Expression),
    Part {
        msb: &'a Expression,
        lsb: &'a Expression,
    },
}

impl SelectBounds<'_> {
    /// How many bits the select carries.
    ///
    /// A bit select is one, with no evaluation: the index does not have to be
    /// known here, and a generate-unrolled `carry[i+1]` is folded where it is
    /// lowered. A part select's *bounds* are what give its width, so those must
    /// fold, and a pair that does not is refused rather than guessed at.
    fn width(&self, span: Span) -> CompileResult<u32> {
        match self {
            Self::Bit(_) => Ok(1),
            Self::Part { msb, lsb } => {
                let (Some(msb), Some(lsb)) = (constant_bound(msb), constant_bound(lsb)) else {
                    return Err(semantic_error(
                        SemanticErrorKind::UnsupportedFeature(
                            "a part-select port connection whose bounds this compiler cannot \
                             fold to constants; IEEE 1364-2005 section 5.2.1 requires constant \
                             bounds, and the width of the connection is what the port must \
                             agree with"
                                .to_string(),
                        ),
                        span,
                    ));
                };
                Ok(msb.abs_diff(lsb) as u32 + 1)
            }
        }
    }

    /// The select, rebuilt against the elaborated name of the net it reads.
    fn expression(&self, elaborated: &str, span: Span) -> Expression {
        match self {
            Self::Bit(index) => Expression::ArrayAccess(ArrayAccessExpr {
                normalized: false,
                packed: None,
                discrete_validity: None,
                array: SmolStr::from(elaborated),
                index: Box::new((*index).clone()),
                span,
            }),
            Self::Part { msb, lsb } => {
                Expression::Digital(DigitalExpr::PartSelect(PartSelectExpr {
                    name: SmolStr::from(elaborated),
                    msb: Box::new((*msb).clone()),
                    lsb: Box::new((*lsb).clone()),
                    span,
                }))
            }
        }
    }

    /// The same select as an assignment target.
    fn lvalue(&self, elaborated: &str, span: Span) -> DigitalLValue {
        match self {
            Self::Bit(index) => DigitalLValue::BitSelect {
                name: SmolStr::from(elaborated),
                index: Box::new((*index).clone()),
                span,
            },
            Self::Part { msb, lsb } => DigitalLValue::PartSelect {
                name: SmolStr::from(elaborated),
                msb: Box::new((*msb).clone()),
                lsb: Box::new((*lsb).clone()),
                span,
            },
        }
    }
}

/// Read one port connection's shape, refusing the forms with no elaborated
/// meaning.
fn connection_form<'a>(
    expression: &'a Expression,
    path: &str,
    port: &str,
) -> CompileResult<ConnectionForm<'a>> {
    match expression {
        Expression::Identifier(identifier) => Ok(ConnectionForm::Net(identifier)),
        Expression::ArrayAccess(access) => Ok(ConnectionForm::Select {
            name: &access.array,
            select: SelectBounds::Bit(&access.index),
            span: access.span,
        }),
        Expression::Digital(DigitalExpr::PartSelect(select)) => Ok(ConnectionForm::Select {
            name: &select.name,
            select: SelectBounds::Part {
                msb: &select.msb,
                lsb: &select.lsb,
            },
            span: select.span,
        }),
        other => Err(semantic_error(
            SemanticErrorKind::UnsupportedFeature(format!(
                "port `{port}` of instance `{path}` is connected to an expression; IEEE \
                 1364-2005 section 12.3.9 permits one, and this compiler connects a port to a \
                 declared net or to a bit- or part-select of one, so name one of those"
            )),
            other.span(),
        )),
    }
}

/// The connecting scope's binding for a name a connection reads.
fn lookup_connection(
    name: &SmolStr,
    scope: &Scope,
    path: &str,
    port: &str,
    span: Span,
) -> CompileResult<Binding> {
    scope.signals.get(name).cloned().ok_or_else(|| {
        semantic_error(
            SemanticErrorKind::UnsupportedFeature(format!(
                "port `{port}` of instance `{path}` is connected to `{name}`, which is not a \
                 declared discrete-domain signal; this compiler does not create the implicit \
                 net of IEEE 1364-2005 section 4.5, so declare `{name}` with a `wire` \
                 declaration"
            )),
            span,
        )
    })
}

/// Fold a select bound to a constant.
///
/// Deliberately minimal, and deliberately here rather than shared with the
/// analyzer's evaluator: what reaches this is a bound written on a port
/// connection, after any generate genvar has been substituted, so what has to
/// fold is a literal or a small arithmetic expression over literals. A bound
/// that needs more is refused with the clause.
fn constant_bound(expression: &Expression) -> Option<i64> {
    match expression {
        Expression::Number(number) if number.value.fract() == 0.0 && number.value.is_finite() => {
            Some(number.value as i64)
        }
        Expression::Unary(unary) => {
            let operand = constant_bound(&unary.operand)?;
            match unary.op {
                crate::ast::UnaryOp::Neg => operand.checked_neg(),
                crate::ast::UnaryOp::Pos => Some(operand),
                _ => None,
            }
        }
        Expression::Binary(binary) => {
            let left = constant_bound(&binary.left)?;
            let right = constant_bound(&binary.right)?;
            match binary.op {
                crate::ast::BinaryOp::Add => left.checked_add(right),
                crate::ast::BinaryOp::Sub => left.checked_sub(right),
                crate::ast::BinaryOp::Mul => left.checked_mul(right),
                crate::ast::BinaryOp::Div => left.checked_div(right),
                _ => None,
            }
        }
        _ => None,
    }
}

/// A synthesized continuous assignment, in elaborated names.
///
/// Nothing about the resulting driver says it was synthesized, which is the
/// point: it lowers through exactly the path a source `assign` does, and the
/// net it drives resolves it against every other driver of that net.
fn assignment(target: DigitalLValue, value: Expression, span: Span) -> AnalyzedContinuousAssign {
    let name = target
        .written_names()
        .first()
        .map(|(name, _)| (*name).clone())
        .unwrap_or_default();
    AnalyzedContinuousAssign {
        target: name,
        assignment: ContinuousAssign {
            target,
            value,
            delay: None,
            span,
        },
        span,
    }
}

fn variable_connection_error(
    outer: &Binding,
    port: &crate::semantic::AnalyzedPort,
    path: &str,
    span: Span,
) -> CompileError {
    semantic_error(
        SemanticErrorKind::InvalidContribution(format!(
            "`{}`, which is a variable connected to the `{}` port `{}` of instance `{path}`; \
             IEEE 1364-2005 section 12.3.9.2 connects an output or inout port to a net",
            outer.elaborated,
            direction_keyword(port.direction),
            port.name
        )),
        span,
    )
}

fn input_port_connection_error(
    outer: &Binding,
    port: &crate::semantic::AnalyzedPort,
    path: &str,
    span: Span,
) -> CompileError {
    semantic_error(
        SemanticErrorKind::InvalidContribution(format!(
            "`{}`, which the connecting module receives through an input port and the `{}` port \
             `{}` of instance `{path}` would drive; IEEE 1364-2005 section 12.3.9.1 drives an \
             input port from outside",
            outer.elaborated,
            direction_keyword(port.direction),
            port.name
        )),
        span,
    )
}

/// The implicit continuous assignment of a variable output port.
///
/// IEEE 1364-2005 section 12.3.9.2, written in elaborated names so that it
/// lowers through exactly the path a source `assign` does. Nothing about the
/// resulting driver says it was synthesized, which is the point: the net has a
/// driver, and it resolves with every other driver of that net.
fn implicit_port_assignment(net: &str, source: &str, span: Span) -> AnalyzedContinuousAssign {
    let target = SmolStr::from(net);
    AnalyzedContinuousAssign {
        target: target.clone(),
        assignment: ContinuousAssign {
            target: DigitalLValue::Identifier { name: target, span },
            value: Expression::Identifier(Identifier {
                name: SmolStr::from(source),
                span,
            }),
            delay: None,
            span,
        },
        span,
    }
}

/// The instance path of `leaf` below `prefix`.
fn qualify(prefix: &str, leaf: &str) -> SmolStr {
    if prefix.is_empty() {
        SmolStr::from(leaf)
    } else {
        SmolStr::from(format!("{prefix}.{leaf}"))
    }
}

const fn direction_keyword(direction: PortDirection) -> &'static str {
    match direction {
        PortDirection::Input => "input",
        PortDirection::Output => "output",
        PortDirection::Inout => "inout",
    }
}

fn semantic_error(kind: SemanticErrorKind, span: Span) -> CompileError {
    SemanticError::new(kind, span).into()
}

fn internal_error(message: String) -> CompileError {
    crate::error::CodeGenError::new(crate::error::CodeGenErrorKind::Internal(message)).into()
}

/// Instance constraints see the final typed values, including dependent defaults.
fn validate_parameter_ranges(source: &Module, path: &str) -> CompileResult<()> {
    use crate::ast::{BinaryExpr, BinaryOp};
    let constants = super::DigitalConstants::from_module(source);
    for parameter in &source.parameters {
        let Some(range) = &parameter.range else {
            continue;
        };
        let mut conditions = Vec::new();
        for bound in &range.bounds {
            if let Some(lower) = &bound.lower {
                conditions.push((
                    if bound.lower_inclusive {
                        BinaryOp::Ge
                    } else {
                        BinaryOp::Gt
                    },
                    lower,
                ));
            }
            if let Some(upper) = &bound.upper {
                conditions.push((
                    if bound.upper_inclusive {
                        BinaryOp::Le
                    } else {
                        BinaryOp::Lt
                    },
                    upper,
                ));
            }
        }
        conditions.extend(range.exclude.iter().map(|value| (BinaryOp::Ne, value)));
        for (op, right) in conditions {
            let comparison = Expression::Binary(BinaryExpr {
                op,
                left: Box::new(Expression::Identifier(Identifier {
                    name: parameter.name.clone(),
                    span: parameter.span,
                })),
                right: Box::new(right.clone()),
                span: parameter.span,
            });
            if !matches!(
                crate::canonical_ir::digital_lower::selector_constant(
                    &comparison,
                    &constants,
                    source.time_scale
                ),
                Some(crate::numeric_literal::NumericLiteralValue::Integer(1))
            ) {
                return Err(semantic_error(
                    SemanticErrorKind::InvalidExpression(format!(
                        "parameter `{}` of instance `{path}` violates its declared range or has an indeterminate range value",
                        parameter.name
                    )),
                    parameter.span,
                ));
            }
        }
    }
    Ok(())
}
