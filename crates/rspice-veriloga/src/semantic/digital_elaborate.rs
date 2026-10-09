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
//! other. The port class, direction and connection form determine the binding:
//!
//! * **Computed inputs become parent-side continuous assignments.** Input
//!   expressions, dynamic selections and width/type conversions are prepared
//!   before binding, with the formal's type and the parent's parameters, arrays
//!   and time scale. The resulting temporary net binds through the ordinary
//!   net path below. Existing constant selections keep their direct path.
//!
//! * **A port declared as a net collapses.** Equal-width nets with matching
//!   packed bounds and signedness share one elaborated signal. Differing views
//!   retain their own declarations and join the same positional wire bits, so
//!   each body uses its local coordinates and signed expression semantics.
//!   Section 12.3.9.3 makes an inout connection exactly this — a bidirectional
//!   join, which no assignment in either direction describes — and section
//!   12.3.10, which asks what net type results when two dissimilar nets are
//!   connected, only has a question to answer because the two nets *become
//!   one*. Collapsing nets retains every original driver and lets either
//!   body's reads observe the shared resolution. It adds no assignment process,
//!   copied driver value or scheduling delta. A generated input binding whose
//!   formal view differs from its actual instead uses an assignment, including
//!   when the actual is a variable with no net resolution to share.
//!
//! * **A bit- or part-select input/output connection becomes an implicit
//!   continuous assignment.** An inout connection instead aliases the selected
//!   wire bits. Each original driver contributes to the shared resolution, and
//!   releasing one side cannot leave a copied value driving the other side.
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
//! * output/inout connection forms not normalized by mixed-boundary preparation
//!   that are neither declared nets nor bit/part selections (section 12.3.9);
//! * a connection naming something that is not a declared discrete-domain
//!   signal after structural implicit-net creation (section 4.5);
//! * an output/inout net collapse whose widths differ; input conversions and
//!   variable outputs instead use assignment sizing (section 12.3.9);
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
    ArrayAccessExpr, ContinuousAssign, DigitalExpr, DigitalLValue, Expression, Identifier, Module,
    ModuleInstance, PartSelectExpr, PortDirection,
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
    elaborator.connections.resolved_types = super::net_types::resolve(
        analyzed,
        source_modules,
        root_source,
        root,
        &mut elaborator.connections.specializations,
        &mut elaborator.connections.warnings,
    )?;
    let resolved_root = elaborator.connections.resolved_types.get("").cloned();
    let (root_source, root) = resolved_root
        .as_deref()
        .map(|root| (&root.source, &root.analyzed))
        .unwrap_or((root_source, root));
    let prepared_root = super::hierarchy_connections::prepare(
        analyzed,
        source_modules,
        root_source,
        root,
        &mut elaborator.connections,
        "",
    )?
    .or(resolved_root.clone());
    let (root_source, root) = prepared_root
        .as_deref()
        .map(|root| (&root.source, &root.analyzed))
        .unwrap_or((root_source, root));
    elaborator.append_instances(root_source, Scope::for_root(root, root_source)?)?;
    // Domain resolution may turn an entire structural subtree into physical
    // connectivity. Empty frames must not force a digital runtime for that tree.
    elaborator.instances.retain(|frame| {
        !frame.signals.is_empty()
            || !frame.processes.is_empty()
            || !frame.continuous_assigns.is_empty()
            || !frame.port_drivers.is_empty()
            || !frame.bit_aliases.is_empty()
            || !frame.analog_events.is_empty()
            || !frame.event_assigned_variables.is_empty()
            || !frame.immutable_analog_variables.is_empty()
    });
    Ok(ElaboratedHierarchy {
        warnings: elaborator.connections.warnings,
        root: prepared_root,
        instances: elaborator.instances,
        occurrences: elaborator.occurrences,
    })
}

/// One specialization per occurrence, shared by both domain lowerings.
pub(super) struct ElaboratedHierarchy {
    pub warnings: Vec<super::SemanticWarning>,
    pub root: Option<std::sync::Arc<SpecializedModule>>,
    pub instances: Vec<ElaboratedDigitalInstance>,
    pub occurrences: HashMap<SmolStr, std::sync::Arc<SpecializedModule>>,
}

/// Include analog containers of digital descendants and all concrete source
/// occurrences needed by hierarchical references, including inactive templates.
/// Other pure analog subtrees retain their symbolic parameter-array path.
pub(super) fn digital_subtrees(
    analyzed: &AnalyzedFile,
    sources: &HashMap<SmolStr, &Module>,
) -> HashSet<SmolStr> {
    use crate::ast::GenerateConstruct;
    let mut required = HashSet::new();
    let mut parents: HashMap<SmolStr, Vec<SmolStr>> = HashMap::new();
    for (name, source) in sources {
        let mut modules = vec![*source];
        while let Some(module) = modules.pop() {
            if module.reference_context.is_some()
                || module.has_digital_content()
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
    /// Coordinates in this occurrence, independent of a collapsed outer name.
    range: super::VectorBounds,
    signed: bool,
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
    fn for_root(root: &AnalyzedModule, source: &Module) -> CompileResult<Self> {
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
                    elaborated: element_binding_name(signal, &root.digital.signals, "")?,
                    width: signal.width,
                    range: signal.range.unwrap_or(super::VectorBounds::SCALAR),
                    signed: signal.signedness.is_signed(),
                    is_variable: signal.class.is_variable(),
                    // The compiled module's own ports are its boundary with
                    // the rest of the circuit, not something an enclosing
                    // module drives, so nothing here is an input port yet.
                    is_input_port: false,
                },
            );
        }
        Ok(scope)
    }
}

/// Keep the canonical element spelling through analog as well as digital
/// hierarchy. Scalar child ports then resolve the same storage and drivers.
fn element_binding_name(
    signal: &super::AnalyzedDigitalSignal,
    declarations: &[super::AnalyzedDigitalSignal],
    path: &str,
) -> CompileResult<SmolStr> {
    let Some(alias) = &signal.element_alias else {
        return Ok(if path.is_empty() {
            signal.name.clone()
        } else {
            qualify(path, &signal.name)
        });
    };
    let array = declarations
        .iter()
        .find(|item| item.name == alias.array)
        .ok_or_else(|| {
            internal_error(format!("array view '{}' has no source array", signal.name))
        })?;
    let bounds: Vec<_> = array
        .dimensions
        .iter()
        .map(|axis| (axis.msb, axis.lsb))
        .collect();
    let layout = crate::array_index::UnpackedArrayLayout::new(
        &bounds,
        super::SemanticAnalyzer::MAX_ARRAY_ELEMENTS,
    )
    .map_err(|_| {
        internal_error(format!(
            "array view '{}' has an invalid source shape",
            signal.name
        ))
    })?;
    if alias.offset as usize >= layout.len() {
        return Err(internal_error(format!(
            "array view '{}' is outside its source array",
            signal.name
        )));
    }
    let name = if path.is_empty() {
        alias.array.clone()
    } else {
        qualify(path, &alias.array)
    };
    Ok(crate::array_index::element_name(&name, &layout, alias.offset as usize).into())
}

struct DigitalElaborator<'a> {
    connections: super::hierarchy_connections::ConnectionModules,
    analyzed: &'a AnalyzedFile,
    source_modules: &'a HashMap<SmolStr, &'a Module>,
    instances: Vec<ElaboratedDigitalInstance>,
    occurrences: HashMap<SmolStr, std::sync::Arc<SpecializedModule>>,
    required: HashSet<SmolStr>,
}

pub(super) fn check_hierarchy_capacity(
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
    context: Option<SmolStr>,
}

impl SpecializationKey {
    pub(super) fn same_specialization(&self, other: &Self) -> bool {
        self.module == other.module && self.overrides == other.overrides
    }
    pub(super) fn root(source: &Module) -> CompileResult<Self> {
        let overrides = source
            .parameters
            .iter()
            .enumerate()
            .filter(|(_, parameter)| parameter.is_given)
            .filter_map(|(index, parameter)| parameter.default.as_ref().map(|value| (index, value)))
            .map(|(index, value)| {
                super::instance_parameters::override_identity(value)
                    .map(|identity| (index, identity))
                    .map_err(internal_error)
            })
            .collect::<CompileResult<Vec<_>>>()?;
        Ok(Self {
            module: source.name.clone(),
            overrides,
            context: source.reference_context.clone(),
        })
    }
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
            key: SpecializationKey::root(source)?,
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
        if let Some(first) = ancestors
            .iter()
            .position(|frame| frame.key.same_specialization(&key))
        {
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

        if connection.is_some() {
            // Authored connect bodies become concrete modules only after boundary
            // selection. Resolve their internal hierarchy with the same contract.
            let resolved = super::net_types::resolve(
                self.analyzed,
                self.source_modules,
                child_source,
                child,
                &mut self.connections.specializations,
                &mut self.connections.warnings,
            )?;
            for (relative, module) in resolved {
                let name = if relative.is_empty() {
                    path.into()
                } else {
                    qualify(path, &relative)
                };
                self.connections.resolved_types.insert(name, module);
            }
        }
        let resolved = self.connections.resolved_types.get(path).cloned();
        let (child_source, child) = resolved
            .as_deref()
            .map(|module| (&module.source, &module.analyzed))
            .unwrap_or((child_source, child));
        let preparation_key = (
            key.clone(),
            if self.connections.resolved_types.is_empty() {
                SmolStr::default()
            } else {
                path.into()
            },
        );
        let connected = if let Some(prepared) = self.connections.prepared.get(&preparation_key) {
            prepared.clone()
        } else {
            let prepared = super::hierarchy_connections::prepare(
                self.analyzed,
                self.source_modules,
                child_source,
                child,
                &mut self.connections,
                path,
            )?
            .or(resolved.clone());
            self.connections
                .prepared
                .insert(preparation_key, prepared.clone());
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
        let (signals, mut scope, port_drivers, bit_aliases) =
            self.bind_ports(instance, child, parent_scope, path, &borrowed)?;
        scope.connections = super::node_vectors::ConnectionScope::new(child_source, child);
        scope.constants = super::DigitalConstants::from_module(child_source);
        scope.time_scale = child_source.time_scale;

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
            bit_aliases,
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
    /// assignments and wire-bit aliases its port connections produce.
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
        Vec<super::digital::ElaboratedDigitalBitAlias>,
    )> {
        let mut bindings: HashMap<SmolStr, Binding> = HashMap::new();
        let mut port_drivers = Vec::new();
        let mut bit_aliases = Vec::new();

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
            let own = element_binding_name(declared, &child.digital.signals, path)?;
            let form = connections[index]
                .map(|expression| connection_form(expression, parent_scope, path, &port.name))
                .transpose()?;
            let binding = match form {
                // An unconnected port is a net of its own. IEEE 1364-2005
                // section 12.3.9 leaves an unconnected input at high
                // impedance, which is what a net nothing drives already is.
                None => Binding {
                    elaborated: own,
                    width: declared.width,
                    range: declared.range.unwrap_or(super::VectorBounds::SCALAR),
                    signed: declared.signedness.is_signed(),
                    is_variable,
                    is_input_port: port.direction == PortDirection::Input,
                },
                // Input/output selections keep their assignment semantics;
                // an inout selection joins normalized wire bits directly.
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
                        PortDirection::Inout => {
                            if outer.is_variable {
                                return Err(variable_connection_error(&outer, port, path, span));
                            }
                            if outer.is_input_port {
                                return Err(input_port_connection_error(&outer, port, path, span));
                            }
                            let coordinates = select.coordinates(span)?;
                            if outer.width == 0
                                || !outer.range.contains(coordinates.msb)
                                || !outer.range.contains(coordinates.lsb)
                                || (coordinates.msb != coordinates.lsb
                                    && (coordinates.msb > coordinates.lsb)
                                        != (outer.range.msb > outer.range.lsb))
                            {
                                return Err(semantic_error(
                                    SemanticErrorKind::InvalidContribution(format!(
                                        "inout port '{path}.{}' requires an in-range wire selection with the declared direction",
                                        port.name
                                    )),
                                    span,
                                ));
                            }
                            for (position, coordinate) in
                                coordinates.indices_msb_first().enumerate()
                            {
                                bit_aliases.push(super::digital::ElaboratedDigitalBitAlias {
                                    left: own.clone(),
                                    left_bit: declared.width - 1 - position as u32,
                                    right: outer.elaborated.clone(),
                                    right_element: None,
                                    right_bit: outer.range.position_of(coordinate) as u32,
                                    span,
                                });
                            }
                        }
                    }
                    Binding {
                        elaborated: own,
                        width: declared.width,
                        range: declared.range.unwrap_or(super::VectorBounds::SCALAR),
                        signed: declared.signedness.is_signed(),
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
                            range: declared.range.unwrap_or(super::VectorBounds::SCALAR),
                            signed: declared.signedness.is_signed(),
                            is_variable: true,
                            is_input_port: false,
                        }
                    } else if declared.width != 0
                        && (declared.range.unwrap_or(super::VectorBounds::SCALAR) != outer.range
                            || declared.signedness.is_signed() != outer.signed)
                    {
                        // Collapse connectivity without erasing the formal's view.
                        // Reusing the outer signal ID would make body selections
                        // and signed operations use the parent's declaration.
                        if port.direction == PortDirection::Input {
                            // Generated converter instances may bind directly to
                            // a differently shaped variable. Inputs copy its
                            // value; variables cannot join a wire's resolution.
                            port_drivers.push(implicit_port_assignment(
                                &own,
                                &outer.elaborated,
                                span,
                            ));
                        } else {
                            for bit in 0..declared.width {
                                bit_aliases.push(super::digital::ElaboratedDigitalBitAlias {
                                    left: own.clone(),
                                    left_bit: bit,
                                    right: outer.elaborated.clone(),
                                    right_element: None,
                                    right_bit: bit,
                                    span,
                                });
                            }
                        }
                        Binding {
                            elaborated: own,
                            width: declared.width,
                            range: declared.range.unwrap_or(super::VectorBounds::SCALAR),
                            signed: declared.signedness.is_signed(),
                            is_variable: false,
                            is_input_port: outer.is_input_port
                                || port.direction == PortDirection::Input,
                        }
                    } else {
                        // Matching views can share storage as well as connectivity.
                        Binding {
                            elaborated: outer.elaborated,
                            width: outer.width,
                            range: declared.range.unwrap_or(super::VectorBounds::SCALAR),
                            signed: declared.signedness.is_signed(),
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
                elaborated: element_binding_name(declared, &child.digital.signals, path)?,
                width: declared.width,
                range: declared.range.unwrap_or(super::VectorBounds::SCALAR),
                signed: declared.signedness.is_signed(),
                is_variable: declared.class.is_variable(),
                is_input_port: child.physical_nodes.real_input_buses.contains(&declared.name)
                    || declared.element_alias.as_ref().is_some_and(|alias| {
                        child.physical_nodes.real_input_buses.contains(&alias.array)
                    }),
            });
            let mut elaborated_declaration = declared.clone();
            if let Some(alias) = &mut elaborated_declaration.element_alias {
                alias.array = qualify(path, &alias.array);
            }
            signals.push(ElaboratedDigitalSignal {
                declared: elaborated_declaration,
                name: binding.elaborated.clone(),
            });
            scope.signals.insert(declared.name.clone(), binding);
        }
        for alias in &child.digital.bit_aliases {
            let mut alias = alias.clone();
            let qualify = |name: &SmolStr| -> CompileResult<SmolStr> {
                scope
                    .signals
                    .get(name)
                    .map(|binding| binding.elaborated.clone())
                    .ok_or_else(|| {
                        internal_error(format!("prepared wire alias '{path}.{name}' has no signal"))
                    })
            };
            alias.left = qualify(&alias.left)?;
            alias.right = qualify(&alias.right)?;
            bit_aliases.push(alias);
        }
        Ok((signals, scope, port_drivers, bit_aliases))
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
    let occurrence = super::source_references::context::occurrence(source, path);
    if instance.parameters.is_empty() && occurrence.is_none() {
        validate_parameter_ranges(source, path)?;
        return Ok((
            SpecializationKey {
                module: instance.module.clone(),
                overrides: Vec::new(),
                context: None,
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
        context: occurrence.as_ref().map(|_| path.into()),
    };
    if let Some(specialized) = cache.get(&key) {
        return Ok((key, Some(specialized.clone())));
    }
    let has_occurrence = occurrence.is_some();
    let mut source = occurrence.unwrap_or_else(|| source.clone());
    if !has_occurrence {
        for (index, value) in values {
            source.parameters[index].default = Some(value);
            source.parameters[index].is_given = true;
        }
        crate::parser::expand_specialized_generates(&mut source)?;
    }
    validate_parameter_ranges(&source, path)?;
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
/// a whole net collapses; selected inouts alias bits and other selections assign.
/// Computed input expressions have already become parent-side assignments.
/// Other unprepared connection forms are refused where they are written.
enum ConnectionForm<'a> {
    /// `.a(bus)` — the whole of a declared net.
    Net(&'a Identifier),
    /// `.a(bus[3])` or `.a(bus[7:4])` — some of a declared net's bits.
    Select {
        name: &'a SmolStr,
        select: Box<SelectBounds>,
        span: Span,
    },
}

/// Which bits of a net a connection selects.
enum SelectBounds {
    Bit(Expression),
    Part { msb: Expression, lsb: Expression },
}

impl SelectBounds {
    fn coordinates(&self, span: Span) -> CompileResult<super::VectorBounds> {
        let bounds = match self {
            Self::Bit(index) => constant_bound(index).map(|index| (index, index)),
            Self::Part { msb, lsb } => constant_bound(msb).zip(constant_bound(lsb)),
        };
        let Some((msb, lsb)) = bounds else {
            return Err(semantic_error(SemanticErrorKind::UnsupportedFeature(
                "a part-select or bidirectional connection requires integer coordinates at elaboration".into()
            ), span));
        };
        let bounds = super::VectorBounds { msb, lsb };
        if bounds.width() > super::MAX_DIGITAL_VECTOR_WIDTH {
            return Err(semantic_error(
                SemanticErrorKind::UnsupportedFeature(
                    "port selection exceeds the supported width".into(),
                ),
                span,
            ));
        }
        Ok(bounds)
    }

    fn width(&self, span: Span) -> CompileResult<u32> {
        match self {
            Self::Bit(_) => Ok(1),
            Self::Part { .. } => Ok(self.coordinates(span)?.width()),
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
                index: Box::new(index.clone()),
                span,
            }),
            Self::Part { msb, lsb } => {
                Expression::Digital(DigitalExpr::PartSelect(PartSelectExpr {
                    name: SmolStr::from(elaborated),
                    msb: Box::new(msb.clone()),
                    lsb: Box::new(lsb.clone()),
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
                index: Box::new(index.clone()),
                span,
            },
            Self::Part { msb, lsb } => DigitalLValue::PartSelect {
                name: SmolStr::from(elaborated),
                msb: Box::new(msb.clone()),
                lsb: Box::new(lsb.clone()),
                span,
            },
        }
    }
}

/// Read one port connection's shape, refusing the forms with no elaborated
/// meaning.
fn connection_form<'a>(
    expression: &'a Expression,
    scope: &Scope,
    path: &str,
    port: &str,
) -> CompileResult<ConnectionForm<'a>> {
    let close = |value: &Expression| match crate::canonical_ir::digital_lower::elaboration_constant(
        value,
        &scope.constants,
        scope.time_scale,
    ) {
        Some(crate::numeric_literal::NumericLiteralValue::Integer(value)) => {
            super::exact_integer_expression(value, expression.span())
        }
        _ => value.clone(),
    };
    match expression {
        Expression::Identifier(identifier) => Ok(ConnectionForm::Net(identifier)),
        Expression::ArrayAccess(access) => Ok(ConnectionForm::Select {
            name: &access.array,
            select: Box::new(SelectBounds::Bit(close(&access.index))),
            span: access.span,
        }),
        Expression::Digital(DigitalExpr::PartSelect(select)) => Ok(ConnectionForm::Select {
            name: &select.name,
            select: Box::new(SelectBounds::Part {
                msb: close(&select.msb),
                lsb: close(&select.lsb),
            }),
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

/// Coordinates already closed in their parent's typed constant scope.
fn constant_bound(expression: &Expression) -> Option<i64> {
    match crate::canonical_ir::digital_lower::elaboration_constant(
        expression,
        &super::DigitalConstants::default(),
        Default::default(),
    ) {
        Some(crate::numeric_literal::NumericLiteralValue::Integer(value)) => Some(value),
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
