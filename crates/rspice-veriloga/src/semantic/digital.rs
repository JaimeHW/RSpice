//! Semantic analysis of the discrete (IEEE 1364-2005) half of a module.
//!
//! This pass resolves and validates what the digital grammar parsed; it does
//! not lower anything. Nothing here produces executable form — a module that
//! carries digital content is refused at every backend boundary, by name — so
//! the pass's whole job is to make sure that what a later wave lowers has
//! already been proven well-formed:
//!
//!   * every declared signal has a resolvable, bounded shape;
//!   * every identifier a process reads or writes is declared;
//!   * every assignment writes something it is allowed to write;
//!   * every constant bit- and part-select is inside its signal;
//!   * every sensitivity list names signals and can actually trigger.
//!
//! Findings are recorded on the analyzer's accumulation channel rather than
//! returned, so a module with several digital defects reports all of them
//! alongside its analog ones.

use super::{AnalyzedModule, MAX_DIGITAL_VECTOR_WIDTH, SemanticAnalyzer, Symbol, SymbolKind};
use crate::ast::*;
use crate::error::SemanticErrorKind;
use crate::source::Span;
use smol_str::SmolStr;
use std::collections::HashMap;

// ============================================================================
// Analyzed output
// ============================================================================

/// Discrete-domain content of an analyzed module.
///
/// Empty for every continuous-domain model, which is every model this compiler
/// can currently execute. When it is not empty, the module is refused by each
/// executable backend, naming the construct — see
/// [`AnalyzedDigital::first_construct`].
#[derive(Debug, Clone, Default)]
pub struct AnalyzedDigital {
    /// Declared nets and variables, in declaration order.
    pub signals: Vec<AnalyzedDigitalSignal>,
    /// Processes with their static sensitivity resolved, in declaration order.
    pub processes: Vec<AnalyzedDigitalProcess>,
    /// Continuous assignments, in declaration order.
    pub continuous_assigns: Vec<AnalyzedContinuousAssign>,
    /// Instantiated child modules, elaborated flat into this module's scope.
    ///
    /// Empty as the analyzer leaves it — a module is analyzed on its own and
    /// knows nothing about what it instantiates. Hierarchy elaboration fills
    /// this in on the module it selects, depth-first in instance-declaration
    /// order, and every entry is already resolved against the scope it was
    /// instantiated in; see [`crate::semantic::digital_elaborate`].
    pub instances: Vec<ElaboratedDigitalInstance>,
    /// Integer-valued parameters and localparams of *this* module, by name.
    ///
    /// IEEE 1364-2005 section 12.2 fixes a parameter at elaboration, so the
    /// places a discrete-domain construct needs a constant — a replication
    /// count, a part-select bound, a delay — may name one. The analog half
    /// deliberately treats a parameter as a per-instance runtime value and
    /// folds nothing, which is why this is a separate table rather than a
    /// widening of the invariant environment.
    ///
    /// Only the compiled module's own. An elaborated instance's body is
    /// lowered against an empty table, so a child's `{WIDTH{1'b0}}` is refused
    /// rather than folded with the *parent's* `WIDTH` — which would be a wrong
    /// answer wearing a right one's clothes.
    pub constants: HashMap<SmolStr, i64>,
}

impl AnalyzedDigital {
    pub fn is_empty(&self) -> bool {
        self.signals.is_empty()
            && self.processes.is_empty()
            && self.continuous_assigns.is_empty()
            && self.instances.is_empty()
    }

    /// The first discrete-domain construct a backend would have to execute.
    ///
    /// Declaration order, so the diagnostic points at the top of the digital
    /// section rather than at whichever construct a hash map happened to
    /// yield first.
    pub fn first_construct(&self) -> Option<DigitalConstruct> {
        let candidates = [
            self.signals.first().map(|signal| DigitalConstruct {
                keyword: signal.class.keyword(),
                detail: format!("declaration of `{}`", signal.name),
                span: signal.span,
            }),
            self.continuous_assigns
                .first()
                .map(|assignment| DigitalConstruct {
                    keyword: "assign",
                    detail: format!("continuous assignment to `{}`", assignment.target),
                    span: assignment.span,
                }),
            self.processes.first().map(|process| DigitalConstruct {
                keyword: process.kind.keyword(),
                detail: format!("process {}", process.id),
                span: process.span,
            }),
            // An elaborated child instance is discrete-domain content of this
            // module as surely as a `reg` is. Without this arm a module whose
            // only digital content came from a child would report nothing here
            // and compile into a device silently missing the whole hierarchy.
            self.instances.first().map(|instance| DigitalConstruct {
                keyword: "instance",
                detail: format!(
                    "digital instance `{}` of module `{}`",
                    instance.path, instance.module
                ),
                span: instance.span,
            }),
        ];
        candidates
            .into_iter()
            .flatten()
            .min_by_key(|construct| construct.span.start)
    }
}

/// One discrete-domain construct, named for a diagnostic.
#[derive(Debug, Clone)]
pub struct DigitalConstruct {
    /// Source keyword that opens the construct.
    pub keyword: &'static str,
    /// What it is, in the compiler's voice.
    pub detail: String,
    pub span: Span,
}

/// Refuse a module that carries discrete-domain content.
///
/// This is the one place the wording of the digital backend refusal lives.
/// Every entry point that turns an [`AnalyzedModule`] into something
/// executable calls it: the bytecode IR builder, hierarchy elaboration, and
/// canonical-IR artifact construction. A module reaches those only after parse
/// and semantic analysis have accepted it, which is deliberate — the author
/// gets real diagnostics for the digital source, and then one clear statement
/// that this compiler cannot run it.
pub(crate) fn reject_digital_content(module: &AnalyzedModule) -> crate::error::CompileResult<()> {
    let Some(construct) = module.digital.first_construct() else {
        return Ok(());
    };
    Err(crate::error::CompileError::CodeGen(
        crate::error::CodeGenError::with_span(
            crate::error::CodeGenErrorKind::UnsupportedDigitalExecution {
                construct: construct.keyword.to_string(),
                detail: format!("module `{}` contains a {}", module.name, construct.detail),
            },
            construct.span,
        ),
    ))
}

/// What kind of thing a discrete-domain name is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DigitalSignalClass {
    Net(DigitalNetKind),
    Variable(DigitalVariableKind),
}

impl DigitalSignalClass {
    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Net(kind) => kind.keyword(),
            Self::Variable(kind) => kind.keyword(),
        }
    }

    /// Whether a procedural assignment may write this name.
    ///
    /// IEEE 1364-2005 section 6.2: a procedural assignment drives a variable;
    /// a net is driven only by a continuous assignment or a primitive.
    pub const fn is_variable(self) -> bool {
        matches!(self, Self::Variable(_))
    }
}

/// Resolved packed bounds of a vector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VectorBounds {
    /// Left bound, exactly as written.
    pub msb: i64,
    /// Right bound, exactly as written.
    pub lsb: i64,
}

impl VectorBounds {
    pub const fn width(self) -> u32 {
        (self.msb.abs_diff(self.lsb) + 1) as u32
    }

    pub const fn contains(self, index: i64) -> bool {
        let (low, high) = if self.msb <= self.lsb {
            (self.msb, self.lsb)
        } else {
            (self.lsb, self.msb)
        };
        index >= low && index <= high
    }

    /// Bounds as `[msb:lsb]`, for diagnostics.
    pub fn spelling(self) -> String {
        format!("[{}:{}]", self.msb, self.lsb)
    }
}

/// A declared discrete-domain net or variable.
#[derive(Debug, Clone)]
pub struct AnalyzedDigitalSignal {
    pub name: SmolStr,
    pub class: DigitalSignalClass,
    pub signedness: Signedness,
    /// Packed range. `None` is a one-bit scalar.
    pub range: Option<VectorBounds>,
    pub width: u32,
    /// Whether the name is also a module port.
    ///
    /// `output q; reg q;` is the standard way to make a port procedurally
    /// assignable (IEEE 1364-2005 section 12.3.4), so the second declaration
    /// re-types the port instead of colliding with it.
    pub redeclares_port: bool,
    pub span: Span,
}

/// A process with its static sensitivity resolved.
#[derive(Debug, Clone)]
pub struct AnalyzedDigitalProcess {
    pub id: DigitalProcessId,
    pub kind: DigitalProcessKind,
    /// Signals the process's opening event control waits on.
    ///
    /// `None` means the process has no single static sensitivity list: it
    /// opens with `@*`, with a delay, or with something other than a timing
    /// control.
    pub sensitivity: Option<Vec<AnalyzedSensitivity>>,
    /// Whether the opening event control was written `@*` / `@(*)`.
    pub implicit_sensitivity: bool,
    /// The body, unmodified. A later wave lowers this into process IR; keeping
    /// it verbatim is what lets that wave see the suspension points the author
    /// wrote.
    pub body: DigitalStatement,
    pub span: Span,
}

/// One resolved term of a static sensitivity list.
#[derive(Debug, Clone)]
pub struct AnalyzedSensitivity {
    /// `None` is a level-sensitive term.
    pub edge: Option<EdgeKind>,
    pub signal: SmolStr,
    pub span: Span,
}

/// A resolved continuous assignment.
#[derive(Debug, Clone)]
pub struct AnalyzedContinuousAssign {
    /// Name of the driven net. A concatenation target reports its first name.
    pub target: SmolStr,
    pub assignment: ContinuousAssign,
    pub span: Span,
}

/// One instantiated digital module, elaborated into the compiled module.
///
/// A frame, not a tree: hierarchy elaboration walks the instance tree once and
/// emits one of these per instance, depth-first in declaration order, each
/// already resolved against the scope it was instantiated in. Two instances of
/// one child module are two frames that share nothing, which is what makes
/// their processes and drivers separately addressable.
#[derive(Debug, Clone)]
pub struct ElaboratedDigitalInstance {
    /// Instance path below the compiled module: `g1`, or `u1.g2` when nested.
    ///
    /// The IEEE 1364-2005 section 12.4 hierarchical name, minus the top
    /// module. A `.` cannot occur in an identifier, so a path can never
    /// collide with a name the author wrote.
    pub path: SmolStr,
    /// The instantiated module's name, for diagnostics.
    pub module: SmolStr,
    /// The instance's own nets and variables, in declaration order.
    pub signals: Vec<ElaboratedDigitalSignal>,
    /// The instance's processes, in declaration order.
    ///
    /// Each carries the process id its *source module* was given, which two
    /// instances of one module therefore share. Lowering allocates a fresh id
    /// per frame; the one here is never used as an identity.
    pub processes: Vec<AnalyzedDigitalProcess>,
    /// The instance's continuous assignments, in declaration order.
    pub continuous_assigns: Vec<AnalyzedContinuousAssign>,
    /// The implicit continuous assignments of this instance's variable output
    /// ports (IEEE 1364-2005 section 12.3.9.2), in port-declaration order.
    ///
    /// Written in *elaborated* names, not the instance's own: the target is
    /// the connected net as the connecting scope knows it, and the source is
    /// this instance's variable. They are the one construct here that is not
    /// something the author wrote.
    pub port_drivers: Vec<AnalyzedContinuousAssign>,
    /// The instance statement, for diagnostics.
    pub span: Span,
}

/// One signal of an elaborated instance, with the name it takes in the flat
/// scope.
#[derive(Debug, Clone)]
pub struct ElaboratedDigitalSignal {
    /// The declaration as the instantiated module wrote it.
    pub declared: AnalyzedDigitalSignal,
    /// The name this signal has in the elaborated scope.
    ///
    /// Ordinarily `path.name`. A net port that collapsed onto the net it is
    /// connected to carries *that net's* elaborated name instead — which is
    /// what collapsing is, stated as a name rather than as a flag: the two
    /// declarations name one signal, so the elaborated plan has one.
    pub name: SmolStr,
}

// ============================================================================
// Analysis
// ============================================================================

/// A variable declared inside a process, in the block that declares it.
///
/// IEEE 1364-2005 section 9.8.1 gives a `begin`/`end` block its own
/// declarative region, so a name declared there is not a module signal and
/// does not collide with one — it shadows it for the extent of the block. The
/// analyzer therefore resolves names against a stack of these before it
/// consults the module's signals.
#[derive(Debug, Clone)]
pub struct AnalyzedProcessLocal {
    pub name: SmolStr,
    pub kind: ProcessLocalKind,
    /// Packed range of a `reg`, `None` for a scalar or a non-vector type.
    pub range: Option<VectorBounds>,
    pub width: u32,
    pub span: Span,
}

/// What a process-local declaration declared.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessLocalKind {
    /// `reg [msb:lsb] name;`
    Reg,
    /// `integer name;` — IEEE 1364-2005 section 3.9 makes it 32 bits.
    Integer,
    /// `real name;`
    Real,
    /// `string name;`
    String,
}

impl ProcessLocalKind {
    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Reg => "reg",
            Self::Integer => "integer",
            Self::Real => "real",
            Self::String => "string",
        }
    }

    /// Whether a bit or part select may name a bit of one.
    const fn is_selectable(self) -> bool {
        matches!(self, Self::Reg | Self::Integer)
    }
}

/// Where a name resolves.
enum Resolution {
    /// A variable declared in an enclosing `begin`/`end` block.
    ProcessLocal(AnalyzedProcessLocal),
    Digital(usize),
    Analog(SymbolKind),
    Undeclared,
}

impl SemanticAnalyzer {
    /// Resolve and validate the module's discrete-domain content.
    ///
    /// Runs after the module's analog declarations are in the symbol table, so
    /// a digital name that collides with a parameter, variable, node, or
    /// branch is caught, and so a digital expression can read an analog
    /// `integer` or `real`.
    pub(super) fn analyze_digital(&mut self, module: &Module, analyzed: &mut AnalyzedModule) {
        if !module.has_digital_content() {
            return;
        }

        let signals = self.collect_digital_signals(module);
        let index: HashMap<SmolStr, usize> = signals
            .iter()
            .enumerate()
            .map(|(position, signal)| (signal.name.clone(), position))
            .collect();

        let mut processes = Vec::new();
        for process in &module.digital_processes {
            processes.push(self.analyze_digital_process(process, &signals, &index));
        }

        let mut continuous_assigns = Vec::new();
        // IEEE 1364-2005 section 6.1.2: a net declaration assignment *is* a
        // continuous assignment, so it becomes one here rather than being
        // dropped on the floor of the declaration — a net whose driver went
        // missing describes a different circuit and says nothing about it.
        for declaration in &module.digital_nets {
            for item in &declaration.items {
                let Some(init) = &item.init else { continue };
                let assignment = ContinuousAssign {
                    target: DigitalLValue::Identifier {
                        name: item.name.clone(),
                        span: item.span,
                    },
                    value: init.clone(),
                    delay: None,
                    span: item.span,
                };
                if let Some(analyzed) =
                    self.analyze_continuous_assign(&assignment, &signals, &index)
                {
                    continuous_assigns.push(analyzed);
                }
            }
        }
        // A *variable* declaration assignment is not one. Section 6.2.1 makes
        // it an initial-block assignment, which is a process rather than a
        // driver, and synthesizing that process is not this wave's — so it
        // refuses instead of being dropped the way the net form was.
        for declaration in &module.digital_variables {
            for item in &declaration.items {
                if item.init.is_some() {
                    self.record_error_at(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "a declaration initializer on the `{}` `{}` is not supported yet; \
                             IEEE 1364-2005 section 6.2.1 makes it equivalent to an `initial` \
                             assignment, so write one",
                            declaration.kind.keyword(),
                            item.name
                        )),
                        item.span,
                    );
                }
            }
        }
        for assignment in &module.continuous_assigns {
            if let Some(analyzed_assignment) =
                self.analyze_continuous_assign(assignment, &signals, &index)
            {
                continuous_assigns.push(analyzed_assignment);
            }
        }

        analyzed.digital = AnalyzedDigital {
            signals,
            processes,
            continuous_assigns,
            // A module is analyzed on its own, so it knows nothing yet about
            // what it instantiates; hierarchy elaboration fills this in on the
            // module it selects.
            instances: Vec::new(),
            constants: self.digital_constants(module),
        };
    }

    // ------------------------------------------------------------------
    // Declarations
    // ------------------------------------------------------------------

    fn collect_digital_signals(&mut self, module: &Module) -> Vec<AnalyzedDigitalSignal> {
        let mut signals: Vec<AnalyzedDigitalSignal> = Vec::new();
        let mut seen: HashMap<SmolStr, Span> = HashMap::new();

        for declaration in &module.digital_nets {
            let bounds = self.resolve_vector_range(declaration.range.as_ref(), "wire");
            for item in &declaration.items {
                self.push_digital_signal(
                    &mut signals,
                    &mut seen,
                    item,
                    DigitalSignalClass::Net(declaration.kind),
                    declaration.signedness,
                    bounds,
                );
            }
        }
        for declaration in &module.digital_variables {
            let bounds = self.resolve_vector_range(declaration.range.as_ref(), "reg");
            for item in &declaration.items {
                self.push_digital_signal(
                    &mut signals,
                    &mut seen,
                    item,
                    DigitalSignalClass::Variable(declaration.kind),
                    declaration.signedness,
                    bounds,
                );
            }
        }
        self.push_implicit_port_nets(module, &mut signals, &mut seen);
        signals
    }

    /// The module's integer parameters and localparams, by name.
    ///
    /// Read out of the declarations rather than out of the analyzer's constant
    /// environments so the table names exactly what this module declares: the
    /// environments accumulate, and a table that inherited a neighbour's
    /// parameter would fold a name this module never wrote.
    ///
    /// A non-integer default is skipped rather than rounded. Every place this
    /// table is consulted wants a bit position or a repetition count, and there
    /// is no defensible integer for `parameter GAIN = 2.5`.
    fn digital_constants(&self, module: &Module) -> HashMap<SmolStr, i64> {
        let mut constants = HashMap::new();
        let declarations = module.parameters.iter().chain(&module.localparams);
        for parameter in declarations {
            let Some(default) = &parameter.default else {
                continue;
            };
            let Some(value) = self.eval_const_parameter_default(default) else {
                continue;
            };
            if value.fract() != 0.0 || !value.is_finite() {
                continue;
            }
            constants.insert(parameter.name.clone(), value as i64);
        }
        constants
    }

    /// Declare the ports that nothing else declared.
    ///
    /// IEEE 1364-2005 section 12.3.3: a port with no net or variable
    /// declaration of its own is implicitly a net of the port's declared range.
    /// A structural design — `module c17 (N1, N2, N22); input N1, N2; output
    /// N22; ... endmodule` — declares nothing else at all, so without this its
    /// ports would be absent from the plan and every reference to one would be
    /// an undeclared name.
    ///
    /// Runs after the explicit declarations so that the two-declaration form of
    /// section 12.3.4 still wins: `output q; reg q;` has already put `q` in as
    /// a variable, and this adds nothing.
    ///
    /// A port carrying a *discipline* is a continuous-domain port and is not a
    /// digital net, so it is skipped — as is a port that appears in an analog
    /// net declaration, which is the other spelling of the same thing. Without
    /// both exclusions a mixed module's `electrical p, n;` would gain a
    /// four-state wire apiece.
    fn push_implicit_port_nets(
        &mut self,
        module: &Module,
        signals: &mut Vec<AnalyzedDigitalSignal>,
        seen: &mut HashMap<SmolStr, Span>,
    ) {
        let analog: std::collections::HashSet<&SmolStr> = module
            .nets
            .iter()
            .flat_map(|net| net.names.iter())
            .collect();
        for declaration in &module.port_declarations {
            if declaration.discipline.is_some() {
                continue;
            }
            let bounds = self.resolve_vector_range(declaration.range.as_ref(), "wire");
            for name in &declaration.names {
                if seen.contains_key(name) || analog.contains(name) {
                    continue;
                }
                seen.insert(name.clone(), declaration.span);
                signals.push(AnalyzedDigitalSignal {
                    name: name.clone(),
                    class: DigitalSignalClass::Net(DigitalNetKind::Wire),
                    signedness: declaration.signedness,
                    range: bounds,
                    width: bounds.map_or(1, VectorBounds::width),
                    redeclares_port: true,
                    span: declaration.span,
                });
            }
        }
    }

    fn push_digital_signal(
        &mut self,
        signals: &mut Vec<AnalyzedDigitalSignal>,
        seen: &mut HashMap<SmolStr, Span>,
        item: &DigitalDeclItem,
        class: DigitalSignalClass,
        signedness: Signedness,
        range: Option<VectorBounds>,
    ) {
        if let Some(first_defined) = seen.get(&item.name) {
            self.record_error_at(
                SemanticErrorKind::DuplicateSymbol {
                    name: item.name.clone(),
                    first_defined: *first_defined,
                },
                item.span,
            );
            return;
        }

        // `output q; reg q;` re-types a port; anything else that already owns
        // the name is a collision.
        let mut redeclares_port = false;
        if let Some(existing) = self.symbols.lookup(&item.name) {
            if existing.kind == SymbolKind::Port {
                redeclares_port = true;
            } else {
                self.record_error_at(
                    SemanticErrorKind::DuplicateSymbol {
                        name: item.name.clone(),
                        first_defined: existing.span,
                    },
                    item.span,
                );
                return;
            }
        }

        // Unpacked (memory) dimensions parse but have no wave-1 shape
        // resolution, so they are refused rather than silently flattened.
        if !item.dimensions.is_empty() {
            self.record_error_at(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "unpacked array dimensions on `{}` are not supported yet; declare a \
                     packed vector instead",
                    item.name
                )),
                item.span,
            );
            return;
        }

        seen.insert(item.name.clone(), item.span);
        signals.push(AnalyzedDigitalSignal {
            name: item.name.clone(),
            class,
            signedness,
            range,
            width: range.map_or(1, VectorBounds::width),
            redeclares_port,
            span: item.span,
        });
    }

    /// Resolve a packed range to constant bounds.
    ///
    /// IEEE 1364-2005 section 4.2.1 requires both bounds to be constant
    /// expressions, and permits either direction.
    ///
    /// # Why a `parameter` counts as one here and not in the analog half
    ///
    /// The two halves disagree about what a parameter is, and both are right.
    /// A Verilog-A model parameter is a per-instance runtime value — the same
    /// compiled device serves a hundred instances with a hundred values — so
    /// nothing continuous-domain may fold it, and `eval_const_invariant`
    /// deliberately does not see one.
    ///
    /// A packed range is not a continuous-domain quantity. IEEE 1364-2005
    /// section 12.2 fixes a parameter's value at *elaboration*, and the width
    /// of `reg [WIDTH-1:0] q;` is decided then and cannot vary afterwards: a
    /// four-bit signal and a five-bit signal are different entries in the plan,
    /// not one entry with a runtime width. So a digital range is resolved
    /// against the declared defaults, which is what elaborating the top module
    /// with no overrides means.
    ///
    /// What that does not yet cover is an instance *override* reaching a
    /// digital range — `child #(.WIDTH(8)) u1 (...)` — which would elaborate
    /// the child's signals at a different width than its own default says.
    /// Hierarchy elaboration refuses a parameter override on a digital instance
    /// rather than applying the default silently.
    fn resolve_vector_range(
        &mut self,
        range: Option<&VectorRange>,
        keyword: &str,
    ) -> Option<VectorBounds> {
        let range = range?;
        let (Some(msb), Some(lsb)) = (
            self.eval_const_invariant(&range.msb)
                .or_else(|| self.eval_const_parameter_default(&range.msb)),
            self.eval_const_invariant(&range.lsb)
                .or_else(|| self.eval_const_parameter_default(&range.lsb)),
        ) else {
            self.record_error_at(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "`{keyword}` vector bounds must be compile-time constants"
                )),
                range.span,
            );
            return None;
        };
        if msb.fract() != 0.0 || lsb.fract() != 0.0 {
            self.record_error_at(
                SemanticErrorKind::TypeMismatch {
                    expected: "integer bounds".to_string(),
                    found: format!("[{msb}:{lsb}]"),
                    context: format!("`{keyword}` vector range"),
                },
                range.span,
            );
            return None;
        }
        let bounds = VectorBounds {
            msb: msb as i64,
            lsb: lsb as i64,
        };
        if bounds.width() > MAX_DIGITAL_VECTOR_WIDTH {
            self.record_error_at(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "`{keyword}` vector {} is {} bits; this compiler supports at most {} \
                     bits per signal",
                    bounds.spelling(),
                    bounds.width(),
                    MAX_DIGITAL_VECTOR_WIDTH
                )),
                range.span,
            );
            return None;
        }
        Some(bounds)
    }

    // ------------------------------------------------------------------
    // Processes
    // ------------------------------------------------------------------

    fn analyze_digital_process(
        &mut self,
        process: &DigitalProcess,
        signals: &[AnalyzedDigitalSignal],
        index: &HashMap<SmolStr, usize>,
    ) -> AnalyzedDigitalProcess {
        // IEEE 1364-2005 section 9.9.2: an `always` process restarts the
        // instant it finishes. One with no timing control anywhere in its body
        // never yields, so simulation time cannot advance past it.
        if process.kind == DigitalProcessKind::Always && !process.has_timing_control() {
            self.record_error_at(
                SemanticErrorKind::UnsupportedFeature(
                    "`always` process has no timing control, so it never suspends and \
                     simulation time cannot advance; add an event control such as \
                     `@(posedge clk)` or a delay"
                        .to_string(),
                ),
                process.span,
            );
        }

        let (sensitivity, implicit_sensitivity) = match process.event_control() {
            Some(control) => match &control.sensitivity {
                Sensitivity::Implicit => (None, true),
                Sensitivity::Explicit(terms) => {
                    (Some(self.resolve_sensitivity(terms, signals, index)), false)
                }
            },
            None => (None, false),
        };

        self.digital_scopes.clear();
        self.check_digital_statement(&process.body, signals, index);
        self.digital_scopes.clear();

        AnalyzedDigitalProcess {
            id: process.id,
            kind: process.kind,
            sensitivity,
            implicit_sensitivity,
            body: process.body.clone(),
            span: process.span,
        }
    }

    fn resolve_sensitivity(
        &mut self,
        terms: &[EventTerm],
        signals: &[AnalyzedDigitalSignal],
        index: &HashMap<SmolStr, usize>,
    ) -> Vec<AnalyzedSensitivity> {
        let mut resolved = Vec::new();
        for term in terms {
            self.check_digital_expression(&term.signal, signals, index);
            let Some(name) = Self::sensitivity_signal_name(&term.signal) else {
                // A term that names no signal can never be triggered by one.
                self.record_error_at(
                    SemanticErrorKind::InvalidExpression(
                        "sensitivity-list term names no signal, so nothing can trigger it"
                            .to_string(),
                    ),
                    term.span,
                );
                continue;
            };
            if let Some(previous) = resolved
                .iter()
                .find(|entry: &&AnalyzedSensitivity| entry.signal == name)
            {
                self.record_error_at(
                    SemanticErrorKind::DuplicateSymbol {
                        name: name.clone(),
                        first_defined: previous.span,
                    },
                    term.span,
                );
                continue;
            }
            resolved.push(AnalyzedSensitivity {
                edge: term.edge,
                signal: name,
                span: term.span,
            });
        }
        resolved
    }

    /// The single signal a sensitivity term watches.
    ///
    /// IEEE 1364-2005 section 9.7.4 allows an arbitrary expression, but a term
    /// that watches more than one signal, or none, has no static list entry.
    fn sensitivity_signal_name(expression: &Expression) -> Option<SmolStr> {
        match expression {
            Expression::Identifier(identifier) => Some(identifier.name.clone()),
            Expression::ArrayAccess(access) => Some(access.array.clone()),
            Expression::Digital(digital) => digital.base_name().cloned(),
            _ => None,
        }
    }

    // ------------------------------------------------------------------
    // Statements
    // ------------------------------------------------------------------

    fn check_digital_statement(
        &mut self,
        statement: &DigitalStatement,
        signals: &[AnalyzedDigitalSignal],
        index: &HashMap<SmolStr, usize>,
    ) {
        match statement {
            DigitalStatement::Block(block) => {
                let scope = self.collect_process_locals(block);
                self.digital_scopes.push(scope);
                // Initializers are checked inside the scope they belong to, so
                // `integer i = 0, j = i;` resolves the way it reads.
                for declaration in &block.variables {
                    for item in &declaration.items {
                        if let Some(init) = &item.init {
                            self.check_digital_expression(init, signals, index);
                        }
                    }
                }
                for declaration in &block.digital_variables {
                    for item in &declaration.items {
                        if let Some(init) = &item.init {
                            self.check_digital_expression(init, signals, index);
                        }
                    }
                }
                for inner in &block.statements {
                    self.check_digital_statement(inner, signals, index);
                }
                self.digital_scopes.pop();
            }
            DigitalStatement::BlockingAssign(assign)
            | DigitalStatement::NonblockingAssign(assign) => {
                self.check_digital_assignment(assign, signals, index);
            }
            DigitalStatement::Conditional(conditional) => {
                self.check_digital_expression(&conditional.condition, signals, index);
                self.check_digital_statement(&conditional.then_branch, signals, index);
                if let Some(branch) = &conditional.else_branch {
                    self.check_digital_statement(branch, signals, index);
                }
            }
            DigitalStatement::Case(case) => {
                self.check_digital_expression(&case.selector, signals, index);
                for item in &case.items {
                    for label in &item.labels {
                        self.check_digital_expression(label, signals, index);
                    }
                    self.check_digital_statement(&item.statement, signals, index);
                }
                if let Some(default) = &case.default {
                    self.check_digital_statement(default, signals, index);
                }
            }
            DigitalStatement::For(statement) => {
                self.check_digital_assignment(&statement.init, signals, index);
                self.check_digital_expression(&statement.condition, signals, index);
                self.check_digital_assignment(&statement.update, signals, index);
                self.check_digital_statement(&statement.body, signals, index);
            }
            DigitalStatement::While(statement) => {
                self.check_digital_expression(&statement.condition, signals, index);
                self.check_digital_statement(&statement.body, signals, index);
            }
            DigitalStatement::Repeat(statement) => {
                self.check_digital_expression(&statement.count, signals, index);
                self.check_digital_statement(&statement.body, signals, index);
            }
            DigitalStatement::Forever(statement) => {
                self.check_digital_statement(&statement.body, signals, index);
            }
            DigitalStatement::Timing(timing) => {
                self.check_timing_control(&timing.control, signals, index);
                if let Some(inner) = &timing.statement {
                    self.check_digital_statement(inner, signals, index);
                }
            }
            DigitalStatement::Null(_) => {}
        }
    }

    /// Resolve the declarations of one `begin`/`end` block into a scope.
    ///
    /// IEEE 1364-2005 section 9.8.1 gives the block its own declarative
    /// region. A name declared here shadows a module signal of the same name
    /// rather than colliding with it; a name declared twice in one region, or
    /// in a region already inside another that declares it, is the collision.
    fn collect_process_locals(&mut self, block: &DigitalBlock) -> Vec<AnalyzedProcessLocal> {
        let mut scope: Vec<AnalyzedProcessLocal> = Vec::new();

        for declaration in &block.variables {
            let kind = match declaration.var_type {
                VarType::Real => ProcessLocalKind::Real,
                VarType::Integer => ProcessLocalKind::Integer,
                VarType::String => ProcessLocalKind::String,
            };
            for item in &declaration.items {
                if !item.dimensions.is_empty() {
                    self.record_error_at(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "array dimensions on the process-local `{}` are not supported yet",
                            item.name
                        )),
                        item.span,
                    );
                    continue;
                }
                self.push_process_local(
                    &mut scope,
                    AnalyzedProcessLocal {
                        name: item.name.clone(),
                        kind,
                        range: None,
                        // IEEE 1364-2005 section 3.9: an `integer` is 32 bits.
                        // A `real` has no bit width, and says so with zero.
                        width: u32::from(kind == ProcessLocalKind::Integer) * 32,
                        span: item.span,
                    },
                );
            }
        }

        for declaration in &block.digital_variables {
            let range = self.resolve_vector_range(declaration.range.as_ref(), "reg");
            for item in &declaration.items {
                if !item.dimensions.is_empty() {
                    self.record_error_at(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "unpacked array dimensions on the process-local `{}` are not \
                             supported yet; declare a packed vector instead",
                            item.name
                        )),
                        item.span,
                    );
                    continue;
                }
                self.push_process_local(
                    &mut scope,
                    AnalyzedProcessLocal {
                        name: item.name.clone(),
                        kind: ProcessLocalKind::Reg,
                        range,
                        width: range.map_or(1, VectorBounds::width),
                        span: item.span,
                    },
                );
            }
        }

        scope
    }

    fn push_process_local(
        &mut self,
        scope: &mut Vec<AnalyzedProcessLocal>,
        local: AnalyzedProcessLocal,
    ) {
        let existing = scope
            .iter()
            .chain(self.digital_scopes.iter().flatten())
            .find(|entry| entry.name == local.name);
        if let Some(existing) = existing {
            self.record_error_at(
                SemanticErrorKind::DuplicateSymbol {
                    name: local.name.clone(),
                    first_defined: existing.span,
                },
                local.span,
            );
            return;
        }
        scope.push(local);
    }

    fn check_timing_control(
        &mut self,
        control: &TimingControl,
        signals: &[AnalyzedDigitalSignal],
        index: &HashMap<SmolStr, usize>,
    ) {
        match control {
            TimingControl::Delay(delay) => {
                self.check_digital_expression(&delay.value, signals, index);
            }
            TimingControl::Event(event) => match &event.sensitivity {
                Sensitivity::Implicit => {}
                Sensitivity::Explicit(terms) => {
                    self.resolve_sensitivity(terms, signals, index);
                }
            },
        }
    }

    fn check_digital_assignment(
        &mut self,
        assign: &DigitalAssign,
        signals: &[AnalyzedDigitalSignal],
        index: &HashMap<SmolStr, usize>,
    ) {
        self.check_digital_lvalue(&assign.target, signals, index, true);
        if let Some(timing) = &assign.timing {
            self.check_timing_control(timing, signals, index);
        }
        self.check_digital_expression(&assign.value, signals, index);
    }

    /// Validate an assignment target.
    ///
    /// `procedural` selects the IEEE 1364-2005 section 6.2 rule that applies:
    /// a procedural assignment drives a variable, a continuous assignment
    /// drives a net.
    fn check_digital_lvalue(
        &mut self,
        target: &DigitalLValue,
        signals: &[AnalyzedDigitalSignal],
        index: &HashMap<SmolStr, usize>,
        procedural: bool,
    ) {
        match target {
            DigitalLValue::Concat { elements, .. } => {
                for element in elements {
                    self.check_digital_lvalue(element, signals, index, procedural);
                }
            }
            DigitalLValue::Identifier { name, span } => {
                self.check_assignable(name, *span, signals, index, procedural);
            }
            DigitalLValue::BitSelect {
                name,
                index: bit,
                span,
            } => {
                if self.check_assignable(name, *span, signals, index, procedural) {
                    self.check_bit_index(name, bit, signals, index);
                }
                self.check_digital_expression(bit, signals, index);
            }
            DigitalLValue::PartSelect {
                name,
                msb,
                lsb,
                span,
            } => {
                if self.check_assignable(name, *span, signals, index, procedural) {
                    self.check_bit_index(name, msb, signals, index);
                    self.check_bit_index(name, lsb, signals, index);
                }
                self.check_digital_expression(msb, signals, index);
                self.check_digital_expression(lsb, signals, index);
            }
        }
    }

    /// Whether `name` may be written here, recording the reason if not.
    fn check_assignable(
        &mut self,
        name: &SmolStr,
        span: Span,
        signals: &[AnalyzedDigitalSignal],
        index: &HashMap<SmolStr, usize>,
        procedural: bool,
    ) -> bool {
        match self.resolve_digital_name(name, index) {
            // A process-local is a variable of the process, so a procedural
            // assignment writes it and a continuous one cannot reach it at all.
            Resolution::ProcessLocal(local) => {
                if !procedural {
                    self.record_error_at(
                        SemanticErrorKind::InvalidContribution(format!(
                            "`{name}`, which is a process-local `{}`; a continuous `assign` \
                             drives a net declared in the module",
                            local.kind.keyword()
                        )),
                        span,
                    );
                    return false;
                }
                true
            }
            Resolution::Digital(position) => {
                let signal = &signals[position];
                if procedural && !signal.class.is_variable() {
                    self.record_error_at(
                        SemanticErrorKind::InvalidContribution(format!(
                            "`{name}`, which is a `{}`; IEEE 1364-2005 section 6.2 lets a \
                             procedural assignment write only a variable, so declare it \
                             `reg` or drive it with a continuous `assign`",
                            signal.class.keyword()
                        )),
                        span,
                    );
                    return false;
                }
                if !procedural && signal.class.is_variable() {
                    self.record_error_at(
                        SemanticErrorKind::InvalidContribution(format!(
                            "`{name}`, which is a `{}`; a continuous `assign` drives a net, \
                             so declare it `wire` or write it from an `always` process",
                            signal.class.keyword()
                        )),
                        span,
                    );
                    return false;
                }
                true
            }
            Resolution::Analog(kind) => {
                // An analog `integer`/`real` is a variable and may be written
                // procedurally; anything else is not an assignment target.
                let assignable = procedural && kind == SymbolKind::Variable;
                if !assignable {
                    self.record_error_at(
                        SemanticErrorKind::InvalidContribution(format!(
                            "`{name}`, which is a {kind:?} of the continuous-domain module \
                             and is not a discrete-domain assignment target"
                        )),
                        span,
                    );
                }
                assignable
            }
            Resolution::Undeclared => {
                self.record_error_at(
                    SemanticErrorKind::UndeclaredSymbol { name: name.clone() },
                    span,
                );
                false
            }
        }
    }

    /// Check a constant bit index against the signal's declared bounds.
    fn check_bit_index(
        &mut self,
        name: &SmolStr,
        expression: &Expression,
        signals: &[AnalyzedDigitalSignal],
        index: &HashMap<SmolStr, usize>,
    ) {
        let (range, kind) = match self.resolve_digital_name(name, index) {
            Resolution::Digital(position) => (signals[position].range, None),
            Resolution::ProcessLocal(local) => {
                if !local.kind.is_selectable() {
                    self.record_error_at(
                        SemanticErrorKind::InvalidExpression(format!(
                            "`{name}` is a process-local `{}`, which has no bits to select",
                            local.kind.keyword()
                        )),
                        expression.span(),
                    );
                    return;
                }
                // IEEE 1364-2005 section 3.9 numbers an `integer`'s bits
                // [31:0]; a `reg` uses the range it was declared with.
                let range = local.range.or(match local.kind {
                    ProcessLocalKind::Integer => Some(VectorBounds { msb: 31, lsb: 0 }),
                    _ => None,
                });
                (range, Some(local.kind))
            }
            Resolution::Analog(_) | Resolution::Undeclared => return,
        };
        // A non-constant index is checked at run time by a later wave; only a
        // constant one can be refused here.
        let Some(value) = self.eval_const_invariant(expression) else {
            return;
        };
        if value.fract() != 0.0 {
            self.record_error_at(
                SemanticErrorKind::TypeMismatch {
                    expected: "integer bit index".to_string(),
                    found: value.to_string(),
                    context: format!("select on `{name}`"),
                },
                expression.span(),
            );
            return;
        }
        let selected = value as i64;
        let inside = match range {
            Some(bounds) => bounds.contains(selected),
            // A scalar signal has exactly one bit, numbered zero.
            None => selected == 0,
        };
        if !inside {
            let declared = range.map_or_else(
                || {
                    kind.map_or_else(
                        || "a scalar (1 bit)".to_string(),
                        |kind| format!("a scalar `{}` (1 bit)", kind.keyword()),
                    )
                },
                VectorBounds::spelling,
            );
            self.record_error_at(
                SemanticErrorKind::IndexOutOfBounds(format!(
                    "bit {selected} of `{name}`, which is declared {declared}"
                )),
                expression.span(),
            );
        }
    }

    // ------------------------------------------------------------------
    // Continuous assignments
    // ------------------------------------------------------------------

    fn analyze_continuous_assign(
        &mut self,
        assignment: &ContinuousAssign,
        signals: &[AnalyzedDigitalSignal],
        index: &HashMap<SmolStr, usize>,
    ) -> Option<AnalyzedContinuousAssign> {
        self.check_digital_lvalue(&assignment.target, signals, index, false);
        if let Some(delay) = &assignment.delay {
            self.check_digital_expression(delay, signals, index);
        }
        self.check_digital_expression(&assignment.value, signals, index);

        let (target, _) = *assignment.target.written_names().first()?;
        Some(AnalyzedContinuousAssign {
            target: target.clone(),
            assignment: assignment.clone(),
            span: assignment.span,
        })
    }

    // ------------------------------------------------------------------
    // Expressions
    // ------------------------------------------------------------------

    /// Resolve every identifier a discrete-domain expression reads, and refuse
    /// the continuous-domain forms that cannot appear in one.
    fn check_digital_expression(
        &mut self,
        expression: &Expression,
        signals: &[AnalyzedDigitalSignal],
        index: &HashMap<SmolStr, usize>,
    ) {
        match expression {
            Expression::Number(_) | Expression::StringLit(_) | Expression::NullArgument(_) => {}
            Expression::Identifier(identifier) => {
                if matches!(
                    self.resolve_digital_name(&identifier.name, index),
                    Resolution::Undeclared
                ) {
                    self.record_error_at(
                        SemanticErrorKind::UndeclaredSymbol {
                            name: identifier.name.clone(),
                        },
                        identifier.span,
                    );
                }
            }
            Expression::Digital(digital) => {
                if let Some(name) = digital.base_name() {
                    if matches!(
                        self.resolve_digital_name(name, index),
                        Resolution::Undeclared
                    ) {
                        self.record_error_at(
                            SemanticErrorKind::UndeclaredSymbol { name: name.clone() },
                            digital.span(),
                        );
                    } else if let DigitalExpr::PartSelect(select) = digital {
                        self.check_bit_index(name, &select.msb, signals, index);
                        self.check_bit_index(name, &select.lsb, signals, index);
                    }
                }
                for child in digital.children() {
                    self.check_digital_expression(child, signals, index);
                }
            }
            Expression::ArrayAccess(access) => {
                if matches!(
                    self.resolve_digital_name(&access.array, index),
                    Resolution::Undeclared
                ) {
                    self.record_error_at(
                        SemanticErrorKind::UndeclaredSymbol {
                            name: access.array.clone(),
                        },
                        access.span,
                    );
                } else {
                    self.check_bit_index(&access.array, &access.index, signals, index);
                }
                self.check_digital_expression(&access.index, signals, index);
            }
            Expression::Binary(binary) => {
                self.check_digital_expression(&binary.left, signals, index);
                self.check_digital_expression(&binary.right, signals, index);
            }
            Expression::Unary(unary) => {
                self.check_digital_expression(&unary.operand, signals, index);
            }
            Expression::Conditional(conditional) => {
                self.check_digital_expression(&conditional.condition, signals, index);
                self.check_digital_expression(&conditional.then_expr, signals, index);
                self.check_digital_expression(&conditional.else_expr, signals, index);
            }
            Expression::ArrayLiteral(literal) => {
                for element in &literal.elements {
                    match element {
                        ArrayLiteralElement::Value(value) => {
                            self.check_digital_expression(value, signals, index);
                        }
                        ArrayLiteralElement::Replication(replication) => {
                            self.check_digital_expression(&replication.count, signals, index);
                            for inner in &replication.elements {
                                if let ArrayLiteralElement::Value(value) = inner {
                                    self.check_digital_expression(value, signals, index);
                                }
                            }
                        }
                    }
                }
            }
            // Continuous-domain-only forms. The parser accepts them because
            // one expression grammar serves both halves of the language; they
            // are meaningless in a process, so they stop here by name.
            Expression::BranchAccess(access) => {
                self.record_error_at(
                    SemanticErrorKind::InvalidExpression(
                        "a branch access reads a continuous-domain signal and has no meaning \
                         in a discrete-domain expression"
                            .to_string(),
                    ),
                    access.span(),
                );
            }
            Expression::AnalogOperator(operator) => {
                self.record_error_at(
                    SemanticErrorKind::InvalidAnalogOperator(
                        "an analog operator is evaluated on every Newton iteration and has no \
                         meaning in a discrete-domain expression"
                            .to_string(),
                    ),
                    operator.span(),
                );
            }
            Expression::NoiseSource(source) => {
                self.record_error_at(
                    SemanticErrorKind::InvalidExpression(
                        "a noise source contributes to a small-signal analysis and has no \
                         meaning in a discrete-domain expression"
                            .to_string(),
                    ),
                    source.span(),
                );
            }
            // Function and system-task calls in a process are outside this
            // wave; their arguments are still resolved so an undeclared name
            // inside one is not hidden by the refusal.
            Expression::Call(call) => {
                self.record_error_at(
                    SemanticErrorKind::UnsupportedFeature(format!(
                        "call to `{}` inside a discrete-domain expression is not supported yet",
                        call.name
                    )),
                    call.span,
                );
                for argument in &call.args {
                    self.check_digital_expression(argument, signals, index);
                }
            }
            Expression::SystemFunction(function) => {
                self.record_error_at(
                    SemanticErrorKind::UnsupportedFeature(format!(
                        "system function `{}` inside a discrete-domain expression is not \
                         supported yet",
                        function.name
                    )),
                    function.span,
                );
                for argument in &function.args {
                    self.check_digital_expression(argument, signals, index);
                }
            }
        }
    }

    /// Where a name written inside a process resolves.
    ///
    /// Innermost declarative region first, per IEEE 1364-2005 section 9.8.1,
    /// then the module's discrete-domain signals, then its continuous-domain
    /// symbols. The order is the shadowing rule, and the lowering resolves in
    /// the same order — a name that meant the local here and the signal there
    /// would be two compilers.
    fn resolve_digital_name(&self, name: &SmolStr, index: &HashMap<SmolStr, usize>) -> Resolution {
        for scope in self.digital_scopes.iter().rev() {
            if let Some(local) = scope.iter().find(|local| local.name == *name) {
                return Resolution::ProcessLocal(local.clone());
            }
        }
        if let Some(position) = index.get(name) {
            return Resolution::Digital(*position);
        }
        match self.symbols.lookup(name) {
            Some(Symbol { kind, .. }) => Resolution::Analog(*kind),
            None => Resolution::Undeclared,
        }
    }
}
