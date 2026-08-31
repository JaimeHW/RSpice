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
//! It is the discrete counterpart of [`elaboration`](super::elaboration), and
//! deliberately not part of it. The analog flattening rewrites a child's
//! equations into the parent's *names* — a child node becomes a synthesized
//! parent node, a child variable a synthesized parent variable — because the
//! analog body is a system of equations with no notion of where a term came
//! from. Nothing here is rewritten. A frame keeps the child's own statements
//! verbatim and carries the *scope* they resolve in, because a process has an
//! identity a scheduler will name and a driver has an identity a resolver will
//! name, and both would be destroyed by folding two instances into one body.
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
//! # What is refused
//!
//! Every refusal names the construct and the clause. Nothing is dropped.
//!
//! * a parameter override (`#(...)`) on a digital instance (section 12.2).
//!   The child's *own* parameters are not refused: section 12.2 makes the
//!   declared default the value in the absence of an override, and the frame
//!   carries that table so a parameter-sized expression inside the child folds
//!   against the child's numbers. An override is what stays refused, because
//!   honouring one would give two instances of a module two different signal
//!   widths and this pass produces one signal table per module body;
//! * a port connection that is neither a declared net nor a bit- or
//!   part-select of one — an arbitrary expression, a constant, a concatenation
//!   (section 12.3.9);
//! * a bit- or part-select connected to an `inout` port, because section
//!   12.3.9.3 makes that connection a bidirectional join of two nets and no
//!   assignment in either direction describes one;
//! * a connection naming something that is not a declared discrete-domain
//!   signal, because this compiler does not create implicit nets (section
//!   4.5);
//! * a port whose width differs from the net it is connected to, because two
//!   collapsed nets are one net and one net has one width (section 12.3.9);
//! * an `input` or `inout` port declared as a variable (section 12.3.3);
//! * an output or inout port connected to a variable, or to anything the
//!   connecting scope sees as an input port, because either would let the
//!   instance drive what it must not (section 12.3.9.1);
//! * a continuous assignment that drives a port its own module receives as an
//!   input (section 12.3.9.1);
//! * a port of a digital module that reaches here with no discrete-domain
//!   declaration at all. Section 12.3.3's implicit net covers every port the
//!   author did not declare, so this is a residual guard rather than a
//!   language restriction: what can still reach it is a port the analyzer
//!   deliberately left out of the digital table, such as one declared with a
//!   discipline;
//! * an instantiation cycle, as a typed error rather than a stack overflow;
//! * a module that mixes discrete and continuous content, and an analog module
//!   instantiated inside a digital one — mixed-signal elaboration is a later
//!   wave, and both directions refuse rather than dropping one half.

use super::{
    AnalyzedContinuousAssign, AnalyzedFile, AnalyzedModule, ElaboratedDigitalInstance,
    ElaboratedDigitalSignal, reject_digital_content,
};
use crate::ast::{
    ArrayAccessExpr, Connection, ContinuousAssign, DigitalExpr, DigitalLValue, Expression,
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
) -> CompileResult<Vec<ElaboratedDigitalInstance>> {
    let mut elaborator = DigitalElaborator {
        analyzed,
        source_modules,
        instances: Vec::new(),
    };
    let scope = Scope::for_root(root);
    let mut module_stack = vec![root.name.clone()];
    elaborator.append_instances(root_source, &scope, &mut module_stack, "", true)?;
    Ok(elaborator.instances)
}

/// Whether the analog flattening should leave this child to this pass.
///
/// The one predicate both passes consult, so that no instance can be claimed
/// by both or by neither. A child with no discrete-domain content is an analog
/// child and the analog pass flattens it as it always has.
pub(crate) fn is_digital_child(child: &AnalyzedModule) -> bool {
    !child.digital.is_empty()
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
    signals: HashMap<SmolStr, Binding>,
}

impl Scope {
    fn for_root(root: &AnalyzedModule) -> Self {
        let mut scope = Self::default();
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
    analyzed: &'a AnalyzedFile,
    source_modules: &'a HashMap<SmolStr, &'a Module>,
    instances: Vec<ElaboratedDigitalInstance>,
}

impl DigitalElaborator<'_> {
    fn append_instances(
        &mut self,
        source_module: &Module,
        scope: &Scope,
        module_stack: &mut Vec<SmolStr>,
        path_prefix: &str,
        analog_children_allowed: bool,
    ) -> CompileResult<()> {
        let mut seen: HashSet<SmolStr> = HashSet::new();
        for instance in &source_module.instances {
            if !seen.insert(instance.name.clone()) {
                return Err(semantic_error(
                    SemanticErrorKind::DuplicateSymbol {
                        name: instance.name.clone(),
                        first_defined: instance.span,
                    },
                    instance.span,
                ));
            }
            let child = self.analyzed.modules.get(&instance.module).ok_or_else(|| {
                semantic_error(
                    SemanticErrorKind::UndefinedModule(instance.module.to_string()),
                    instance.span,
                )
            })?;
            let path = qualify(path_prefix, &instance.name);
            if !is_digital_child(child) {
                if analog_children_allowed {
                    // An analog instance of the compiled module. The analog
                    // flattening owns it, and owns everything under it.
                    continue;
                }
                return Err(semantic_error(
                    SemanticErrorKind::UnsupportedFeature(format!(
                        "instance `{path}` puts the continuous-domain module `{}` inside a \
                         discrete-domain module; a mixed-signal hierarchy has no elaborated \
                         form yet",
                        instance.module
                    )),
                    instance.span,
                ));
            }
            self.append_instance(instance, child, scope, module_stack, &path)?;
        }
        Ok(())
    }

    fn append_instance(
        &mut self,
        instance: &ModuleInstance,
        child: &AnalyzedModule,
        parent_scope: &Scope,
        module_stack: &mut Vec<SmolStr>,
        path: &str,
    ) -> CompileResult<()> {
        // Before anything recurses. A module that instantiates itself, however
        // indirectly, describes an infinite design; reporting the cycle is the
        // only thing that can be done with it, and doing so here is what keeps
        // the walk from being the thing that reports it, as a stack overflow.
        if module_stack.contains(&instance.module) {
            let mut cycle = module_stack.iter().map(SmolStr::as_str).collect::<Vec<_>>();
            cycle.push(instance.module.as_str());
            return Err(semantic_error(
                SemanticErrorKind::CircularDependency(format!(
                    "digital module hierarchy {} at instance '{path}'",
                    cycle.join(" -> ")
                )),
                instance.span,
            ));
        }

        let child_source = self
            .source_modules
            .get(&instance.module)
            .copied()
            .ok_or_else(|| {
                internal_error(format!(
                    "digital module '{}' was analyzed but not retained",
                    instance.module
                ))
            })?;

        // A module with content in both domains is refused with the wording
        // every other digital-backend boundary uses, so an author meets one
        // message for "this compiler cannot execute that" rather than two.
        if has_analog_content(child) {
            reject_digital_content(child)?;
        }
        // A child that declares parameters elaborates at its own declared
        // defaults, which section 12.2 makes the value in the absence of an
        // override. What stays refused is the override itself — see below —
        // because honouring one would give two instances of a module two
        // different signal *widths*, and this pass produces one signal table
        // per module body.
        if let Some(override_) = instance.parameters.first() {
            return Err(semantic_error(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "instance `{path}` overrides a parameter of `{}`; IEEE 1364-2005 section \
                     12.2 parameter value assignment on a digital instance is not supported yet",
                    instance.module
                )),
                override_.span,
            ));
        }

        let connections = bind_connections(instance, child, path)?;
        let (signals, scope, port_drivers) =
            self.bind_ports(instance, child, parent_scope, path, &connections)?;

        // A continuous assignment inside the instance may not drive a net the
        // instance receives through an `input` port, however many levels above
        // the driven net was declared. Checked here rather than at the port,
        // because the driver is written in the child and the prohibition is a
        // property of the connection.
        for assignment in &child.digital.continuous_assigns {
            for (name, span) in assignment.assignment.target.written_names() {
                if scope
                    .signals
                    .get(name)
                    .is_some_and(|binding| binding.is_input_port)
                {
                    return Err(semantic_error(
                        SemanticErrorKind::InvalidContribution(format!(
                            "`{name}`, which instance `{path}` receives through an input port; \
                             IEEE 1364-2005 section 12.3.9.1 drives an input port from outside \
                             the instance, so nothing inside it may assign one"
                        )),
                        span,
                    ));
                }
            }
        }

        self.instances.push(ElaboratedDigitalInstance {
            path: path.into(),
            module: instance.module.clone(),
            signals,
            processes: child.digital.processes.clone(),
            continuous_assigns: child.digital.continuous_assigns.clone(),
            port_drivers,
            constants: child.digital.constants.clone(),
            span: instance.span,
        });

        module_stack.push(instance.module.clone());
        let nested = self.append_instances(child_source, &scope, module_stack, path, false);
        module_stack.pop();
        nested
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
                return Err(semantic_error(
                    SemanticErrorKind::UnsupportedFeature(format!(
                        "port `{}` of the digital module `{}` has no discrete-domain \
                         declaration; IEEE 1364-2005 section 12.3.3's implicit net covers a \
                         port the author did not declare, so a port reaching here is one the \
                         analyzer left out of the digital table — a port carrying a \
                         discipline, which is a continuous-domain port and cannot be \
                         connected to a digital instance",
                        port.name, instance.module
                    )),
                    instance.span,
                ));
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
                    if width != declared.width {
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
                    if outer.width != declared.width {
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
        let mut scope = Scope::default();
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

/// Whether the module has continuous-domain content to flatten.
fn has_analog_content(module: &AnalyzedModule) -> bool {
    !module.contributions.is_empty()
        || !module.body.is_empty()
        || !module.statements.is_empty()
        || !module.branches.is_empty()
        || !module.internal_nodes.is_empty()
        || !module.variables.is_empty()
}

/// Match an instance's connections to the child's ports.
///
/// IEEE 1364-2005 sections 12.3.5 and 12.3.6 give the two forms, and section
/// 12.3.6 forbids mixing them in one instance.
fn bind_connections<'a>(
    instance: &'a ModuleInstance,
    child: &AnalyzedModule,
    path: &str,
) -> CompileResult<Vec<Option<&'a Expression>>> {
    let has_named = instance
        .connections
        .iter()
        .any(|connection| matches!(connection, Connection::Named { .. }));
    let has_ordered = instance
        .connections
        .iter()
        .any(|connection| matches!(connection, Connection::Ordered { .. }));
    if has_named && has_ordered {
        return Err(semantic_error(
            SemanticErrorKind::UnsupportedFeature(format!(
                "instance `{path}` mixes named and ordered port connections; IEEE 1364-2005 \
                 section 12.3.6 permits one form per instance"
            )),
            instance.span,
        ));
    }

    let mut bound: Vec<Option<&Expression>> = vec![None; child.ports.len()];
    if has_named {
        let mut seen = vec![false; child.ports.len()];
        for connection in &instance.connections {
            let Connection::Named { port, signal, span } = connection else {
                unreachable!("the connection list is all named")
            };
            let Some(index) = child.ports.iter().position(|entry| entry.name == *port) else {
                return Err(semantic_error(
                    SemanticErrorKind::UndeclaredSymbol { name: port.clone() },
                    *span,
                ));
            };
            if std::mem::replace(&mut seen[index], true) {
                return Err(semantic_error(
                    SemanticErrorKind::DuplicateSymbol {
                        name: port.clone(),
                        first_defined: *span,
                    },
                    *span,
                ));
            }
            bound[index] = signal.as_ref();
        }
    } else {
        if instance.connections.len() > child.ports.len() {
            return Err(semantic_error(
                SemanticErrorKind::ArgumentCountMismatch {
                    name: path.to_string(),
                    expected: format!("at most {} port connections", child.ports.len()),
                    got: instance.connections.len(),
                },
                instance.span,
            ));
        }
        for (index, connection) in instance.connections.iter().enumerate() {
            let Connection::Ordered { signal, .. } = connection else {
                unreachable!("the connection list is all ordered")
            };
            bound[index] = signal.as_ref();
        }
    }
    Ok(bound)
}

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
                             fold to constants; IEEE 1364-2005 section 4.2.1 requires constant \
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
