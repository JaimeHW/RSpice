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
    /// Parameters and localparams of *this* module a discrete-domain body may
    /// fold, by name.
    ///
    /// Only the compiled module's own. An elaborated instance's body is
    /// lowered against [`ElaboratedDigitalInstance::constants`] — the *child's*
    /// table — so a child's `{WIDTH{1'b0}}` folds with the child's `WIDTH` and
    /// never with the parent's, which would be a wrong answer wearing a right
    /// one's clothes.
    pub constants: DigitalConstants,
}

/// The elaboration-time constants a discrete-domain body may fold.
///
/// IEEE 1364-2005 section 12.2 fixes a parameter at elaboration, so a
/// discrete-domain construct may name one wherever it needs a constant. The
/// analog half deliberately treats a parameter as a per-instance runtime value
/// and folds nothing, which is why this is a separate table rather than a
/// widening of the invariant environment.
///
/// Two tables and not one, because the two are asked different questions. A
/// replication count, a part-select bound and a delay want an integer and have
/// no reading for `2.5`; a real expression wants the value the author wrote and
/// has no reading for its integer part. A name can be in both — `parameter real
/// N = 2.0;` is a legitimate bit position and a legitimate real — and answers
/// each question in the domain that asked it.
#[derive(Debug, Clone, Default)]
pub struct DigitalConstants {
    /// Parameters whose default is a whole finite number, as that number.
    ///
    /// A non-integer default is absent rather than rounded: every place this is
    /// consulted wants a bit position or a repetition count, and there is no
    /// defensible integer for `parameter GAIN = 2.5`.
    pub integers: HashMap<SmolStr, i64>,
    /// Parameters the author declared `parameter real`, as their default.
    ///
    /// Explicitly typed only. An untyped `parameter WIDTH = 8;` takes
    /// Verilog-AMS's real default type, so admitting untyped parameters here
    /// would make every existing bit-width parameter a real operand and change
    /// what `q = WIDTH;` means. `parameter real K = 0.25;` says which domain it
    /// belongs in, and this table holds exactly the parameters that said so.
    pub reals: HashMap<SmolStr, f64>,
    /// The same declarations whose default folds to an infinity or a NaN, as
    /// that value.
    ///
    /// Kept apart from [`Self::reals`] rather than dropped, and never handed
    /// to a lowering: the declaration is legal Verilog-AMS and the continuous
    /// domain accepts it, so a process that names one has to be refused for
    /// what is actually wrong with it — a real with no discrete-domain form —
    /// and not told the name was never declared.
    pub non_finite_reals: HashMap<SmolStr, f64>,
}

impl DigitalConstants {
    /// The integer a name denotes, if it denotes one.
    pub fn integer(&self, name: &str) -> Option<i64> {
        self.integers.get(name).copied()
    }

    /// The real a name denotes, if it was declared `parameter real`.
    pub fn real(&self, name: &str) -> Option<f64> {
        self.reals.get(name).copied()
    }

    /// The non-finite value a name folds to, if that is why it denotes no
    /// discrete-domain real. For diagnostics only.
    pub fn non_finite_real(&self, name: &str) -> Option<f64> {
        self.non_finite_reals.get(name).copied()
    }
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

    /// Whether the name carries a real value rather than four-state bits.
    ///
    /// Two spellings reach it, and they differ by IEEE 1364-2005 section 6.2's
    /// rule rather than by what they carry: a `wreal` is Verilog-AMS LRM 2.4
    /// section 3.7's real *net*, driven by continuous assignments, and a `real`
    /// is section 3.9's real *variable*, written procedurally. A process-local
    /// `real` is neither — it belongs to the process and is not a signal at all.
    pub const fn is_real(self) -> bool {
        match self {
            Self::Net(kind) => kind.is_real(),
            Self::Variable(kind) => kind.is_real(),
        }
    }

    /// How a real net combines its drivers, if it is one.
    pub const fn wreal_resolution(self) -> Option<WrealResolution> {
        match self {
            Self::Net(kind) => kind.resolution(),
            Self::Variable(_) => None,
        }
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
    /// Packed range. `None` is a one-bit scalar, and is the only shape a real
    /// net has — see [`Self::width`].
    pub range: Option<VectorBounds>,
    /// Declared width in bits.
    ///
    /// Zero for a `wreal`, which has no bit width at all: Verilog-AMS LRM 2.4
    /// section 3.7 makes it a real-valued connection, not a vector of bits.
    /// The same spelling [`ProcessLocalKind::Real`] already uses, so that "how
    /// many bits does this carry" has one answer everywhere and `0` means the
    /// same thing in both places.
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
    /// The instantiated module's *own* integer parameters and localparams.
    ///
    /// Not the parent's, and not the parent's merged with the child's. A
    /// child's `{WIDTH{1'b0}}` means the child's `WIDTH`, and folding it with a
    /// parent's `WIDTH` would be a wrong answer wearing a right one's clothes —
    /// so the two tables never meet. With a parameter override on a digital
    /// instance refused (section 12.2), the child's declared defaults *are* its
    /// elaborated values, and every instance of one module sees the same table.
    pub constants: DigitalConstants,
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
        self.reject_drivers_on_input_ports(module, &continuous_assigns);

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
            let keyword = declaration.kind.keyword();
            // Verilog-AMS LRM 2.4 Syntax 3-8 permits a range on a `wreal`,
            // which declares a *bus of real nets* — an unpacked array of
            // reals, not a packed vector of bits. Nothing downstream has an
            // array of signals, so it is refused by name rather than read as
            // one net of some width, which is what a range on a `wire` means
            // and is the one wrong answer available here.
            let bounds = if declaration.kind.is_real() {
                if let Some(range) = &declaration.range {
                    self.record_error_at(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "a range on a `{keyword}` declares a bus of real nets, which is not \
                             supported yet; Verilog-AMS LRM 2.4 section 3.7 makes each element a \
                             real-valued net of its own, so declare them separately"
                        )),
                        range.span,
                    );
                }
                None
            } else {
                self.resolve_vector_range(declaration.range.as_ref(), keyword)
            };
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
            // A `real` variable has neither a packed range nor a sign: IEEE
            // 1364-2005 section 3.9 gives it no bits to range over and no sign
            // bit to interpret. Each is refused where the source wrote it
            // rather than dropped, because `output real signed [3:0] v;` says
            // three things and only one of them is a real.
            let bounds = if declaration.kind.is_real() {
                if let Some(range) = &declaration.range {
                    self.record_error_at(
                        SemanticErrorKind::UnsupportedFeature(
                            "a packed range on a `real` has no meaning; IEEE 1364-2005 section \
                             3.9 makes a `real` a variable with no bit width"
                                .to_string(),
                        ),
                        range.span,
                    );
                }
                None
            } else {
                self.resolve_vector_range(declaration.range.as_ref(), "reg")
            };
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
        self.promote_module_level_reals(module, &mut signals, &mut seen);
        self.push_implicit_port_nets(module, &mut signals, &mut seen);
        signals
    }

    /// Move a module-level `real` into the discrete domain when a process owns
    /// it, and refuse the case the standard forbids.
    ///
    /// # The ownership rule is the standard's, not this compiler's
    ///
    /// Verilog-AMS LRM 2.4 section 7.3: "Read operations of nets and variables
    /// in both domains are allowed from both contexts. **Write operations of
    /// nets and variables are only allowed from the context of their domain.**"
    /// So a variable belongs to whichever domain writes it, that domain is the
    /// only one that may, and either domain may read it. That is a rule, not
    /// an implementation-defined area, and it decides all three cases here.
    ///
    /// `real state;` at module level is the *continuous* domain's declaration —
    /// it is the same production a Verilog-A model writes, and every one of the
    /// shipped analog models is full of them. What makes it a discrete-domain
    /// variable is a process writing it, so it becomes one exactly when:
    ///
    /// 1. some `always`, `initial` or continuous assignment **writes** it, and
    /// 2. the analog body does **not** write it.
    ///
    /// Condition 1 is what makes the promotion necessary: a variable no process
    /// writes has nothing to gain from moving, and leaving it where it was
    /// keeps it the continuous body's own state.
    ///
    /// Condition 2 is section 7.3's sentence. Both halves writing one variable
    /// is not a synchronization problem to be solved with a rule about clocks —
    /// it is a program the standard does not admit, so it is refused by name
    /// and cited, permanently rather than "not yet".
    ///
    /// This used to be a question about the *module* — any analog block at all
    /// disqualified every module-level `real` — which refused a module whose
    /// analog body never mentions the variable, and refused it with a message
    /// about clocks that did not apply. The question is about the name.
    ///
    /// A pure-analog module is still untouched by construction: it has no
    /// processes, so condition 1 fails for every one of its variables and
    /// neither question can change how it compiles.
    ///
    /// # What is still refused, and what it is waiting for
    ///
    /// A variable a process writes and the **analog body reads**. Section 7.3
    /// allows that read and section 7.3.6.5 fixes its value — "the digital
    /// value calculated for the greatest digital time tick which is less than
    /// or equal to the analog time when the expression is evaluated", which is
    /// the same zero-order hold the D/A bridge already implements. What is
    /// missing is the seam, not the semantics: the analog body is evaluated by
    /// compiled code that has no route to the digital signal store, so the
    /// refusal names the clause it is short of rather than the boundary in
    /// general.
    ///
    /// An `output real` port does not come through here at all. It is an
    /// explicit discrete-domain declaration — IEEE 1364-2005 section 12.3.4's
    /// variable port form, with section 3.9's `real` as the type — so the
    /// parser already put it in `digital_variables`, exactly as it does for
    /// `output reg`.
    fn promote_module_level_reals(
        &mut self,
        module: &Module,
        signals: &mut Vec<AnalyzedDigitalSignal>,
        seen: &mut HashMap<SmolStr, Span>,
    ) {
        let mut written: std::collections::HashSet<SmolStr> = std::collections::HashSet::new();
        for process in &module.digital_processes {
            collect_written_names(&process.body, &mut written);
        }
        for assignment in &module.continuous_assigns {
            for (name, _) in assignment.target.written_names() {
                written.insert(name.clone());
            }
        }
        if written.is_empty() {
            return;
        }
        let analog = module
            .analog_block
            .as_ref()
            .or(module.analog_initial.as_ref())
            .or(module.analog_final.as_ref());
        // What the continuous body does with each name, over every analog
        // block the module declares rather than only the first: `analog
        // initial x = 0;` beside `analog V(a) <+ x;` is two blocks and one
        // variable, and asking only one of them would answer about half a
        // module.
        let mut analog_writes = std::collections::HashSet::new();
        let mut analog_reads = std::collections::HashSet::new();
        for block in [
            module.analog_block.as_ref(),
            module.analog_initial.as_ref(),
            module.analog_final.as_ref(),
        ]
        .into_iter()
        .flatten()
        {
            for statement in &block.statements {
                collect_analog_names(statement, &mut analog_writes, &mut analog_reads);
            }
        }
        // One expression form the walk could not enumerate makes every name a
        // possible read. Erring that way promotes nothing the analog body
        // might still be using, which is the direction that cannot produce a
        // wrong answer — only a refusal.
        let opaque_read = analog_reads.contains(OPAQUE_ANALOG_READ);

        for declaration in &module.variables {
            if declaration.var_type != VarType::Real {
                continue;
            }
            for item in &declaration.items {
                if !written.contains(&item.name) || seen.contains_key(&item.name) {
                    continue;
                }
                // Verilog-AMS LRM 2.4 section 7.3: a variable's own domain is
                // the only one that may write it. Two writers is a program the
                // standard does not admit, and no scheduling rule would make
                // it one.
                if analog_writes.contains(&item.name) {
                    self.record_error_at(
                        SemanticErrorKind::InvalidExpression(format!(
                            "`{}` is written by both the analog body and a discrete process; \
                             Verilog-AMS LRM 2.4 section 7.3 allows a write only from the \
                             context of the variable's own domain, so one of the two writes \
                             has to go",
                            item.name
                        )),
                        item.span,
                    );
                    continue;
                }
                // Section 7.3 allows this read and section 7.3.6.5 fixes its
                // value, so the refusal is about the seam rather than about
                // the program.
                if (opaque_read || analog_reads.contains(&item.name))
                    && let Some(block) = analog
                {
                    self.record_error_at(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "`{}` is written by a discrete process and read by the analog body; \
                             Verilog-AMS LRM 2.4 section 7.3.6.5 makes that read the digital \
                             value at the greatest tick at or before the analog time, and the \
                             compiled analog body has no route to the digital signal store yet",
                            item.name
                        )),
                        block.span,
                    );
                    continue;
                }
                if !item.dimensions.is_empty() {
                    self.record_error_at(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "`{}` is an array of `real`, and a process writes it; an array has \
                             no discrete-domain signal form yet",
                            item.name
                        )),
                        item.span,
                    );
                    continue;
                }
                if item.init.is_some() {
                    self.record_error_at(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "a declaration initializer on the module-level `real` `{}` that a \
                             process writes is not supported yet; IEEE 1364-2005 section 6.2.1 \
                             makes it equivalent to an `initial` assignment, so write one",
                            item.name
                        )),
                        item.span,
                    );
                    continue;
                }
                seen.insert(item.name.clone(), item.span);
                signals.push(AnalyzedDigitalSignal {
                    name: item.name.clone(),
                    class: DigitalSignalClass::Variable(DigitalVariableKind::Real),
                    signedness: Signedness::Unsigned,
                    range: None,
                    // No bits, the way every real quantity says it here.
                    width: 0,
                    redeclares_port: false,
                    span: item.span,
                });
            }
        }
    }

    /// Refuse a continuous assignment that drives one of this module's own
    /// `input` ports.
    ///
    /// IEEE 1364-2005 section 12.3.9.1: an input port is driven from outside
    /// the instance, so nothing inside the module may assign it. Hierarchy
    /// elaboration already refuses this — but only for a module somebody
    /// *instantiates*, and only when the connecting scope's own view says the
    /// net is an input port. A module compiled as the top of the design is
    /// never instantiated, so without this the same source is accepted or
    /// refused depending on where it sits in a hierarchy, and a top-level
    /// design with the defect gets a net with a driver the standard says it
    /// cannot have.
    ///
    /// `inout` is deliberately not here. Section 12.3.9.3 makes a bidirectional
    /// port drivable from both sides, which is the whole reason the direction
    /// exists.
    fn reject_drivers_on_input_ports(
        &mut self,
        module: &Module,
        assignments: &[AnalyzedContinuousAssign],
    ) {
        let inputs: std::collections::HashSet<&SmolStr> = module
            .port_declarations
            .iter()
            .filter(|declaration| declaration.direction == PortDirection::Input)
            .flat_map(|declaration| declaration.names.iter())
            .collect();
        if inputs.is_empty() {
            return;
        }
        for assignment in assignments {
            for (name, span) in assignment.assignment.target.written_names() {
                if inputs.contains(name) {
                    self.record_error_at(
                        SemanticErrorKind::InvalidContribution(format!(
                            "`{name}`, which module `{}` declares an `input` port; IEEE \
                             1364-2005 section 12.3.9.1 drives an input port from outside the \
                             instance, so nothing inside the module may assign one",
                            module.name
                        )),
                        span,
                    );
                }
            }
        }
    }

    /// The module's foldable parameters and localparams, by name.
    ///
    /// Read out of the declarations rather than out of the analyzer's constant
    /// environments so the table names exactly what this module declares: the
    /// environments accumulate, and a table that inherited a neighbour's
    /// parameter would fold a name this module never wrote.
    fn digital_constants(&self, module: &Module) -> DigitalConstants {
        let mut constants = DigitalConstants::default();
        let declarations = module.parameters.iter().chain(&module.localparams);
        for parameter in declarations {
            let Some(default) = &parameter.default else {
                continue;
            };
            let Some(value) = self.eval_const_parameter_default(default) else {
                continue;
            };
            // A parameter array is a name with no scalar value at all; folding
            // its first element under the array's own name would answer a
            // question nobody asked.
            if !parameter.dimensions.is_empty() {
                continue;
            }
            let is_real = parameter.type_is_explicit && parameter.param_type == ParamType::Real;
            if !value.is_finite() {
                // Neither table takes it: the discrete domain has no spelling
                // for an infinity or a NaN, and folding one into a process
                // would put a value there that no discrete operation defines.
                // It is remembered under its own name so a process that reads
                // it is refused for the value rather than for the name.
                if is_real {
                    constants
                        .non_finite_reals
                        .insert(parameter.name.clone(), value);
                }
                continue;
            }
            if is_real {
                constants.reals.insert(parameter.name.clone(), value);
            }
            if value.fract() == 0.0 {
                constants
                    .integers
                    .insert(parameter.name.clone(), value as i64);
            }
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
            // A real net has no bits, and says so with zero.
            width: if class.is_real() {
                0
            } else {
                range.map_or(1, VectorBounds::width)
            },
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
            // IEEE 1364-2005 section 9.7.2 classifies an edge from a scalar
            // transition, and table 5-2 does so over the four values a bit can
            // take; an edge on a vector is an edge on its least significant
            // bit. A `wreal` has no bits, so there is no transition to
            // classify and no defensible reading of `posedge` on one — a
            // threshold crossing would be an invented rule, and the value
            // change the standard *does* define is what a bare term already
            // asks for.
            if term.edge.is_some()
                && index
                    .get(&name)
                    .is_some_and(|position| signals[*position].class.is_real())
            {
                let keyword = signals[index[&name]].class.keyword();
                self.record_error_at(
                    SemanticErrorKind::InvalidExpression(format!(
                        "`{}` on `{name}`, which is a `{keyword}`; IEEE 1364-2005 section 9.7.2 \
                         classifies an edge from a bit transition and a real net has no bits, so \
                         write `@({name})` for the value-change event Verilog-AMS LRM 2.4 \
                         section 3.7 gives one",
                        match term.edge {
                            Some(EdgeKind::Posedge) => "posedge",
                            _ => "negedge",
                        }
                    )),
                    term.span,
                );
                continue;
            }
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
            Resolution::Digital(position) if signals[position].class.is_real() => {
                // Verilog-AMS LRM 2.4 section 3.7 makes a `wreal` a real-valued
                // connection. It has no bit representation to index into, and
                // the standard's own answer to "the bits of a real" is the
                // explicit `$realtobits` of that same clause.
                self.record_error_at(
                    SemanticErrorKind::InvalidExpression(format!(
                        "`{name}` is a `{}`, which carries a real value and has no bits to \
                         select; Verilog-AMS LRM 2.4 section 3.7 converts one to bits with \
                         `$realtobits`",
                        signals[position].class.keyword()
                    )),
                    expression.span(),
                );
                return;
            }
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
            // Verilog-AMS LRM 2.4 section 7.3.3: "All continuous nets can be
            // probed from a discrete context using access functions." This is
            // the one continuous-domain form that belongs in a process, and
            // `check_analog_probe` decides which of its spellings this
            // compiler can serve.
            Expression::BranchAccess(access) => self.check_analog_probe(access, index),
            // Continuous-domain-only forms. The parser accepts them because
            // one expression grammar serves both halves of the language; they
            // are meaningless in a process, so they stop here by name.
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
                // The two the standard leaves open. Verilog-AMS LRM 2.4 section
                // 3.7 names them as the *only* bridge between a real net and
                // bits — "connection to explicitly declared 64-bit wires can be
                // done via system tasks `$realtobits` and `$bitstoreal`" — and
                // every refusal of an implicit conversion in this compiler
                // points the author at them, so accepting them is what makes
                // that advice followable.
                if matches!(function.name.as_str(), "$realtobits" | "$bitstoreal") {
                    if function.args.len() != 1 {
                        self.record_error_at(
                            SemanticErrorKind::InvalidExpression(format!(
                                "`{}` takes exactly one argument, and was given {}",
                                function.name,
                                function.args.len()
                            )),
                            function.span,
                        );
                    }
                } else {
                    self.record_error_at(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "system function `{}` inside a discrete-domain expression is not \
                             supported yet",
                            function.name
                        )),
                        function.span,
                    );
                }
                for argument in &function.args {
                    self.check_digital_expression(argument, signals, index);
                }
            }
        }
    }

    /// Decide whether a probe of a continuous net can be served from a
    /// discrete-domain expression.
    ///
    /// # What the standard allows
    ///
    /// Verilog-AMS LRM 2.4 section 7.3 opens the whole cross-domain question
    /// with one sentence: "Read operations of nets and variables in both
    /// domains are allowed from both contexts. Write operations of nets and
    /// variables are only allowed from the context of their domain." Section
    /// 7.3.3 spends the read half of that on probes — "All continuous nets can
    /// be probed from a discrete context using access functions. All probes
    /// which are legal in a continuous context of a module are also legal in
    /// the discrete context of a module" — and its own example is the sampler
    /// this exists for:
    ///
    /// ```verilog
    /// always @(posedge clk)
    ///     out = V(in);
    /// ```
    ///
    /// So a probe in a process is not an error to be reported; the read half
    /// of section 7.3 is the whole point of a mixed module. What is decided
    /// here is which *spellings* of it this compiler can answer, and each
    /// refusal names the reason rather than the clause, because the clause
    /// permits all of them.
    ///
    /// # What is refused, and why each one is
    ///
    /// * **A flow probe** (`I(a, b)`, or any access function its discipline
    ///   declares a flow). Section 7.3.3 makes it legal. A potential is an
    ///   entry of the solution vector and can be read from wherever the analog
    ///   solver last left one; a flow is not — it is the analog body's own
    ///   accumulated contribution to a branch, produced by evaluating that
    ///   body, and there is nothing to sample between evaluations. Reading a
    ///   stale one would be a plausible number for a quantity nobody computed.
    /// * **The named-branch form** (`V(<b>)`). It names an entry of the analog
    ///   branch table, which the discrete plan does not carry — a probe names
    ///   its nets, so the two halves need no shared numbering. The equivalent
    ///   node form is what to write.
    /// * **A net that is not continuous.** A discrete net has no potential to
    ///   probe; it is read by naming it, which is what a process already does.
    fn check_analog_probe(&mut self, access: &BranchAccess, index: &HashMap<SmolStr, usize>) {
        let (function, positive, negative) = match access {
            BranchAccess::Nodes {
                access: function,
                pos,
                neg,
                ..
            } => (function, pos, neg.as_ref()),
            BranchAccess::Branch {
                access: function,
                name,
                span,
            } => {
                self.record_error_at(
                    SemanticErrorKind::UnsupportedFeature(format!(
                        "`{function}(<{name}>)` probes a declared branch from a discrete-domain \
                         expression; Verilog-AMS LRM 2.4 section 7.3.3 allows it, but a \
                         discrete-domain probe names its nets rather than the analog branch \
                         table — write the equivalent node form"
                    )),
                    *span,
                );
                return;
            }
        };
        if self.disciplines.resolve_access(function).is_none() && function != "I" {
            self.record_error_at(
                SemanticErrorKind::InvalidExpression(format!(
                    "`{function}` is not an access function of any declared discipline"
                )),
                access.span(),
            );
            return;
        }
        if self.is_flow_access(function) {
            self.record_error_at(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "`{function}` is a flow access, and a flow has no value between analog \
                     evaluations to sample from a discrete-domain expression; Verilog-AMS LRM \
                     2.4 section 7.3.3 allows it, but only a potential probe is served here"
                )),
                access.span(),
            );
            return;
        }
        for net in std::iter::once(positive).chain(negative) {
            self.check_probe_net(function, net, access.span(), index);
        }
    }

    /// Refuse one operand of a discrete-domain probe that is not a continuous
    /// net of this module.
    fn check_probe_net(
        &mut self,
        function: &SmolStr,
        net: &SmolStr,
        span: Span,
        index: &HashMap<SmolStr, usize>,
    ) {
        // A name the module declared in its discrete half is refused first and
        // by that fact alone, without asking the discipline database. A `wire`
        // written in a process's own half of the module is a discrete net
        // whether or not discipline resolution ever gave it a discipline, and
        // the author who wrote `V(clk)` needs to be told that `clk` is not a
        // thing with a potential — not that it is undeclared.
        if index.contains_key(net) {
            self.record_error_at(
                SemanticErrorKind::InvalidExpression(format!(
                    "`{function}({net})` probes `{net}`, which is a discrete-domain net and has \
                     no potential; a process reads one by naming it"
                )),
                span,
            );
            return;
        }
        // A net with no discipline of its own has not been resolved to a
        // discrete one either, so it is read as continuous — the same default
        // the analog half applies to an undeclared net.
        let resolved = self.symbols.lookup(net).map(|symbol| {
            let discrete = symbol.attrs.discipline.as_ref().is_some_and(|discipline| {
                self.disciplines
                    .get_discipline(discipline)
                    .is_some_and(|discipline| {
                        discipline.domain == crate::disciplines::Domain::Discrete
                    })
            });
            (symbol.kind, discrete)
        });
        let Some((kind, discrete)) = resolved else {
            self.record_error_at(
                SemanticErrorKind::UndeclaredSymbol { name: net.clone() },
                span,
            );
            return;
        };
        if !matches!(kind, SymbolKind::Port | SymbolKind::Node) {
            self.record_error_at(
                SemanticErrorKind::InvalidExpression(format!(
                    "`{function}({net})` probes `{net}`, which is not a net"
                )),
                span,
            );
            return;
        }
        if discrete {
            self.record_error_at(
                SemanticErrorKind::InvalidExpression(format!(
                    "`{function}({net})` probes `{net}`, which is a discrete-domain net and has \
                     no potential; a process reads one by naming it"
                )),
                span,
            );
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

/// The name recorded for an expression form [`collect_analog_names`] cannot
/// look inside.
///
/// Not a legal Verilog identifier, so it can never collide with one the author
/// wrote. It exists so that "the analog body reads something this walk could
/// not enumerate" is a fact the read set carries rather than a silence.
const OPAQUE_ANALOG_READ: &str = "$opaque";

/// What the continuous body does with each name it mentions.
///
/// Two sets and one walk, because the ownership rule asks two questions about
/// the same name and walking twice would be two chances for the two answers to
/// come from different traversals. `written` is assignment *targets* only;
/// `read` is every identifier reached from an expression, including the
/// right-hand side of an assignment whose target is also written — `x = x + 1`
/// both writes and reads `x`, and saying so is what makes the write rule fire
/// on it rather than the read refusal.
///
/// Over-approximating in one direction on purpose, the same way
/// [`collect_written_names`] does: a name inside a branch that never runs still
/// counts, because whether it runs is a question about a simulation and this is
/// a question about a declaration.
///
/// A `read` entry that is not a variable at all — a parameter, a net inside a
/// branch access, a function name — is harmless: the caller only ever asks
/// about names it has already established are module-level `real` variables a
/// process writes.
fn collect_analog_names(
    statement: &AnalogStatement,
    written: &mut std::collections::HashSet<SmolStr>,
    read: &mut std::collections::HashSet<SmolStr>,
) {
    match statement {
        AnalogStatement::Null(_) | AnalogStatement::Disable(_) => {}
        AnalogStatement::Contribution(contribution) => {
            collect_expression_names(&contribution.value, read);
        }
        AnalogStatement::IndirectContribution(contribution) => {
            collect_expression_names(&contribution.lhs, read);
            collect_expression_names(&contribution.rhs, read);
        }
        AnalogStatement::Assignment(assignment) => {
            written.insert(assignment.target_name().clone());
            if let LValue::ArrayAccess { index, .. } = &assignment.target {
                collect_expression_names(index, read);
            }
            collect_expression_names(&assignment.value, read);
        }
        AnalogStatement::Conditional(conditional) => {
            collect_expression_names(&conditional.condition, read);
            collect_analog_names(&conditional.then_branch, written, read);
            if let Some(branch) = &conditional.else_branch {
                collect_analog_names(branch, written, read);
            }
        }
        AnalogStatement::Case(case) => {
            collect_expression_names(&case.expr, read);
            for item in &case.items {
                for value in &item.matches {
                    collect_expression_names(value, read);
                }
                collect_analog_names(&item.statement, written, read);
            }
            if let Some(default) = &case.default {
                collect_analog_names(default, written, read);
            }
        }
        AnalogStatement::For(statement) => {
            // The loop variable is written by the header, and its initializer
            // and update are ordinary expressions of the same body.
            written.insert(statement.var.clone());
            collect_expression_names(&statement.init, read);
            collect_expression_names(&statement.condition, read);
            collect_analog_names(
                &AnalogStatement::Assignment((*statement.update).clone()),
                written,
                read,
            );
            collect_analog_names(&statement.body, written, read);
        }
        AnalogStatement::While(statement) => {
            collect_expression_names(&statement.condition, read);
            collect_analog_names(&statement.body, written, read);
        }
        AnalogStatement::Repeat(statement) => {
            collect_expression_names(&statement.count, read);
            collect_analog_names(&statement.body, written, read);
        }
        AnalogStatement::Block(block) => {
            for statement in &block.statements {
                collect_analog_names(statement, written, read);
            }
        }
        AnalogStatement::EventControl(event) => {
            collect_analog_names(&event.statement, written, read);
        }
        AnalogStatement::Call(call) => {
            for argument in &call.args {
                collect_expression_names(argument, read);
            }
        }
    }
}

/// Every identifier an expression reads.
///
/// A branch access contributes its *net* names, which is deliberate: they can
/// never collide with a module-level `real`, so including them costs nothing
/// and leaving them out would mean one more shape to keep in step.
fn collect_expression_names(
    expression: &Expression,
    read: &mut std::collections::HashSet<SmolStr>,
) {
    match expression {
        Expression::Number(_) | Expression::StringLit(_) | Expression::NullArgument(_) => {}
        Expression::Identifier(identifier) => {
            read.insert(identifier.name.clone());
        }
        Expression::ArrayAccess(access) => {
            read.insert(access.array.clone());
            collect_expression_names(&access.index, read);
        }
        Expression::BranchAccess(access) => match access {
            BranchAccess::Nodes { pos, neg, .. } => {
                read.insert(pos.clone());
                if let Some(neg) = neg {
                    read.insert(neg.clone());
                }
            }
            BranchAccess::Branch { name, .. } => {
                read.insert(name.clone());
            }
        },
        Expression::Binary(binary) => {
            collect_expression_names(&binary.left, read);
            collect_expression_names(&binary.right, read);
        }
        Expression::Unary(unary) => collect_expression_names(&unary.operand, read),
        Expression::Conditional(conditional) => {
            collect_expression_names(&conditional.condition, read);
            collect_expression_names(&conditional.then_expr, read);
            collect_expression_names(&conditional.else_expr, read);
        }
        Expression::Call(call) => {
            for argument in &call.args {
                collect_expression_names(argument, read);
            }
        }
        Expression::SystemFunction(function) => {
            for argument in &function.args {
                collect_expression_names(argument, read);
            }
        }
        // Never present in a parsed tree, which is the only kind this walks.
        // `ddt(x)` and its siblings are `Call`s until `expr_converter` rewrites
        // them, and that runs after this. Recording the sentinel means a name
        // reachable only through one is treated as read rather than silently
        // missed, which is the safe direction: the write rule's failure mode is
        // promoting a variable the analog body still uses.
        Expression::AnalogOperator(_) => {
            read.insert(OPAQUE_ANALOG_READ.into());
        }
        Expression::NoiseSource(source) => match source {
            NoiseSource::White { power, .. } => collect_expression_names(power, read),
            NoiseSource::Flicker {
                power, exponent, ..
            } => {
                collect_expression_names(power, read);
                collect_expression_names(exponent, read);
            }
            NoiseSource::Table { data, .. } => {
                for value in data {
                    collect_expression_names(value, read);
                }
            }
        },
        Expression::Digital(digital) => {
            if let Some(name) = digital.base_name() {
                read.insert(name.clone());
            }
            for child in digital.children() {
                collect_expression_names(child, read);
            }
        }
        Expression::ArrayLiteral(literal) => {
            for element in &literal.elements {
                match element {
                    ArrayLiteralElement::Value(value) => collect_expression_names(value, read),
                    ArrayLiteralElement::Replication(replication) => {
                        collect_expression_names(&replication.count, read);
                        for inner in &replication.elements {
                            if let ArrayLiteralElement::Value(value) = inner {
                                collect_expression_names(value, read);
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Every name a process assigns to, anywhere in its body.
///
/// Over-approximating on purpose in one direction only: a name inside a branch
/// that never runs still counts, because whether it runs is a question about a
/// simulation and this is a question about a declaration. It does not
/// over-approximate the other way — a name is here only if it is an assignment
/// *target*, so reading one does not claim it.
///
/// A process-local declaration is not filtered out. A local shadows the module
/// name for the statements under it, so a body that declares `real acc;` and
/// writes `acc` never means the module's — but the module's `acc` is then not
/// promoted, and stays exactly where it was. Erring toward "the process writes
/// it" would promote a variable the process cannot reach; erring the other way,
/// which is what would happen if this filtered, would leave a written variable
/// behind. The match is exhaustive so a new statement form cannot slip past it.
fn collect_written_names(
    statement: &DigitalStatement,
    written: &mut std::collections::HashSet<SmolStr>,
) {
    match statement {
        DigitalStatement::Null(_) => {}
        DigitalStatement::Block(block) => {
            for statement in &block.statements {
                collect_written_names(statement, written);
            }
        }
        DigitalStatement::BlockingAssign(assign) | DigitalStatement::NonblockingAssign(assign) => {
            for (name, _) in assign.target.written_names() {
                written.insert(name.clone());
            }
        }
        DigitalStatement::Conditional(conditional) => {
            collect_written_names(&conditional.then_branch, written);
            if let Some(branch) = &conditional.else_branch {
                collect_written_names(branch, written);
            }
        }
        DigitalStatement::Case(case) => {
            for item in &case.items {
                collect_written_names(&item.statement, written);
            }
            if let Some(default) = &case.default {
                collect_written_names(default, written);
            }
        }
        DigitalStatement::For(statement) => {
            collect_written_names(
                &DigitalStatement::BlockingAssign((*statement.init).clone()),
                written,
            );
            collect_written_names(
                &DigitalStatement::BlockingAssign((*statement.update).clone()),
                written,
            );
            collect_written_names(&statement.body, written);
        }
        DigitalStatement::While(statement) => collect_written_names(&statement.body, written),
        DigitalStatement::Repeat(statement) => collect_written_names(&statement.body, written),
        DigitalStatement::Forever(statement) => collect_written_names(&statement.body, written),
        DigitalStatement::Timing(timing) => {
            if let Some(statement) = &timing.statement {
                collect_written_names(statement, written);
            }
        }
    }
}
