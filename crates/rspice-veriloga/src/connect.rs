//! Discipline resolution and connect-module insertion planning.
//!
//! This is Verilog-AMS LRM 2.4 clause 7's net-boundary machinery: which
//! discipline a net segment has, which connect module a mixed-discipline
//! connection needs, and where an instance of it goes. It answers those three
//! questions and nothing else — a connect module's *behaviour* is a module's
//! and is compiled like any other.
//!
//! # What is implemented, and under which clause
//!
//! | Clause | What |
//! |---|---|
//! | 3.11.1 | discipline and nature compatibility |
//! | 7.4.4.1 / Annex F.2.1 | the basic (default) discipline resolution algorithm |
//! | 7.7.1 / Syntax 7-6 | the connect module auto-insertion statement, its `connect_mode`, its parameters, and its port overrides |
//! | 7.7.2 / Syntax 7-7 | the `resolveto` statement, including `exclude` |
//! | 7.7.2.1 | exact-then-subset rule matching, first match with a warning |
//! | 7.6 / Table 7-2 | the three admissible connect module direction combinations |
//! | 7.8.4 | connect module selection, and the four insertion rules |
//! | 7.8.3 | `merged` and `split` segmentation |
//! | 7.8.5 | generated instance names |
//!
//! # What is refused by name
//!
//! * **Section 7.4.4.2 / Annex F.2.2, the *detail* resolution mode.** Section
//!   7.4.4 makes the choice between the two modes "vendor-specific", and this
//!   compiler's choice is the default one. The alternate algorithm propagates
//!   continuous disciplines back *down* the hierarchy and would put connect
//!   modules in different places, so selecting it is a decision with visible
//!   consequences rather than a knob; it is [`ResolutionMode::Detail`] and
//!   resolving with it errors rather than silently running the basic pass.
//! * **Section 10.2's `` `default_discipline `` qualifiers.** The unqualified
//!   directive is applied; a qualified one (`` `default_discipline d reg ``)
//!   would need a per-net-type default and is refused with its clause.
//! * **Resolution among several user *continuous* disciplines.** Section 7.4.3
//!   makes connecting incompatible continuous disciplines an error, and
//!   section 7.4.1's `resolveto` form settles compatible ones; both of those
//!   are implemented. What is not is section 7.2.4's rule that a mixed signal
//!   spanning several compatible continuous disciplines takes the *smallest*
//!   `abstol` of them, because [`crate::disciplines::Discipline`] carries no
//!   per-discipline tolerance to take the smallest of.
//!
//! # Why the port directions in a `connect` statement are the connect
//! module's own
//!
//! Section 7.7.1 says the override "specified directions are used to define
//! the type of connect module", which makes them the *module's* port
//! directions. Figures 7-9 and 7-10 write example blocks that only make sense
//! if the directions were the bridged design port's instead, and they
//! contradict each other on which — Figure 7-10 gives `d2a` the same
//! `input ttl, output electrical` that Figure 7-9 does while drawing it on the
//! opposite ports. The normative sentence is followed here and the figures are
//! not.

pub mod library;

use std::collections::{BTreeMap, BTreeSet, HashMap};

use smol_str::SmolStr;

use crate::ast::{
    ConnectInsertion, ConnectMode, ConnectResolution, ConnectResolveTarget, ConnectRulesDecl,
    ConnectRulesItem, Item, Module, ParameterOverride, PortDirection, SourceFile,
};
use crate::disciplines::{DisciplineDb, Domain};
use crate::source::Span;

/// Table 7-2's three admissible combinations of a connect module's port
/// directions, named for what the module does rather than for the row it sits
/// on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectDirection {
    /// Continuous `input`, discrete `output`: the module reads the analog side
    /// and drives the discrete one.
    AnalogToDiscrete,
    /// Continuous `output`, discrete `input`.
    DiscreteToAnalog,
    /// Both `inout`. Section 7.6 Example 3: such a module "can bridge any
    /// mixed port".
    Bidirectional,
}

impl ConnectDirection {
    fn from_ports(continuous: PortDirection, discrete: PortDirection) -> Option<Self> {
        match (continuous, discrete) {
            (PortDirection::Input, PortDirection::Output) => Some(Self::AnalogToDiscrete),
            (PortDirection::Output, PortDirection::Input) => Some(Self::DiscreteToAnalog),
            (PortDirection::Inout, PortDirection::Inout) => Some(Self::Bidirectional),
            _ => None,
        }
    }

    /// Whether a module of this kind may bridge a port that needs `required`.
    ///
    /// Section 7.6's three examples: a unidirectional module bridges exactly
    /// its own direction, a bidirectional one bridges any.
    fn admits(self, required: Self) -> bool {
        self == required || self == Self::Bidirectional
    }

    /// How exactly this kind matches `required`, for the tie-break in
    /// [`ConnectRuleTable::select`].
    fn specificity(self, required: Self) -> u8 {
        if self == required { 1 } else { 0 }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::AnalogToDiscrete => "analog-to-discrete",
            Self::DiscreteToAnalog => "discrete-to-analog",
            Self::Bidirectional => "bidirectional",
        }
    }
}

/// One side of a connect module, as section 7.6 reads its declaration.
#[derive(Debug, Clone)]
pub struct ConnectModulePort {
    pub name: SmolStr,
    pub discipline: SmolStr,
    pub direction: PortDirection,
}

/// A connect module declaration reduced to what selection needs.
#[derive(Debug, Clone)]
pub struct ConnectModuleDecl {
    pub name: SmolStr,
    pub continuous: ConnectModulePort,
    pub discrete: ConnectModulePort,
    pub direction: ConnectDirection,
    pub span: Span,
}

/// One `connect <cm> …;` statement, validated against the declarations.
#[derive(Debug, Clone)]
pub struct InsertionRule {
    pub connect_module: SmolStr,
    /// Section 7.8.3: `merged` is the default.
    pub mode: ConnectMode,
    /// Section 7.7.3's parameter values, retained as written. Folding them is
    /// the instantiating pass's job, not selection's.
    pub parameters: Vec<ParameterOverride>,
    /// The connect module's continuous-side port: the discipline it bridges
    /// after any override, and the port's own name, which is what an
    /// instantiating pass binds the analog side to.
    pub continuous: ConnectModulePort,
    /// The discrete-side port, the same way.
    pub discrete: ConnectModulePort,
    pub direction: ConnectDirection,
    pub span: Span,
}

impl InsertionRule {
    /// Section 7.7.3's parameter values, folded to numbers and named.
    ///
    /// The rule keeps them as written because folding is the instantiating
    /// pass's job, and this is that pass's half of it — here rather than in
    /// the engine because [`crate::ast::Expression`] is this crate's and a
    /// second walker over it in another crate is a second answer waiting to
    /// happen.
    ///
    /// A literal, optionally signed, is the whole of what folds. Anything else
    /// — an identifier, a call, an arithmetic expression — is refused by name
    /// rather than guessed at, because a connect statement's parameter is
    /// written at the top of a design where nothing is in scope to evaluate it
    /// against.
    ///
    /// Section 7.7.3's positional form has no name to bind to, and a connect
    /// module's parameter order is not part of what selection reads, so a
    /// positional override is refused too.
    pub fn numeric_parameters(&self) -> Result<Vec<(SmolStr, f64)>, ConnectError> {
        self.parameters
            .iter()
            .map(|parameter| {
                let name = parameter.name.clone().ok_or_else(|| {
                    ConnectError::PositionalConnectParameter {
                        module: self.connect_module.clone(),
                        span: parameter.span,
                    }
                })?;
                let value = fold_literal(&parameter.value).ok_or_else(|| {
                    ConnectError::NonLiteralConnectParameter {
                        module: self.connect_module.clone(),
                        parameter: name.clone(),
                        span: parameter.span,
                    }
                })?;
                Ok((name, value))
            })
            .collect()
    }
}

/// A signed numeric literal, and nothing else.
fn fold_literal(expression: &crate::ast::Expression) -> Option<f64> {
    use crate::ast::{Expression, UnaryOp};
    match expression {
        Expression::Number(number) => Some(number.value),
        Expression::Unary(unary) => {
            let operand = fold_literal(&unary.operand)?;
            match unary.op {
                UnaryOp::Neg => Some(-operand),
                UnaryOp::Pos => Some(operand),
                UnaryOp::Not | UnaryOp::BitNot => None,
            }
        }
        _ => None,
    }
}

/// One `connect … resolveto …;` statement.
#[derive(Debug, Clone)]
pub struct ResolutionRule {
    /// The discipline list as written, deduplicated. Section 7.7.2.1 matches
    /// on the *set*, so the order is kept only for diagnostics.
    pub disciplines: BTreeSet<SmolStr>,
    pub target: ConnectResolveTarget,
    pub span: Span,
}

/// The typed table a file's `connectrules` blocks become.
///
/// Every block in the file contributes to one table. Clause 7 names a
/// `connectrules` block but gives no way to select among several, so merging
/// them is the only reading available; the consequence is that two blocks each
/// declaring a rule for one discipline pair make that pair ambiguous, which
/// [`Self::select`] refuses rather than resolving by block order.
#[derive(Debug, Clone, Default)]
pub struct ConnectRuleTable {
    modules: BTreeMap<SmolStr, ConnectModuleDecl>,
    insertions: Vec<InsertionRule>,
    resolutions: Vec<ResolutionRule>,
}

impl ConnectRuleTable {
    pub fn insertions(&self) -> &[InsertionRule] {
        &self.insertions
    }

    pub fn resolutions(&self) -> &[ResolutionRule] {
        &self.resolutions
    }

    /// Section 7.8.4 rule 3: select the one connect statement a mixed port
    /// matches.
    ///
    /// Rule 3 says the port "shall match one (and only one) connect
    /// statement". Taken alone that makes a design carrying both a
    /// bidirectional rule and a unidirectional one for the same discipline
    /// pair an error, which is not what section 7.8.3.1's Figure 7-9
    /// describes: there a `bidir` rule and a `d2a` rule coexist and each takes
    /// the ports it fits. The two are reconciled by ranking an exact direction
    /// match above a bidirectional module's catch-all, and applying rule 3
    /// *within* a rank — so a tie is still an error, and it names both rules.
    pub fn select(
        &self,
        continuous: &str,
        discrete: &str,
        required: ConnectDirection,
        db: &DisciplineDb,
    ) -> Result<&InsertionRule, ConnectError> {
        let mut best: Option<&InsertionRule> = None;
        let mut best_rank = 0u8;
        let mut tied: Option<&InsertionRule> = None;

        for rule in &self.insertions {
            if !rule.direction.admits(required)
                || !disciplines_compatible(db, &rule.continuous.discipline, continuous)
                || !disciplines_compatible(db, &rule.discrete.discipline, discrete)
            {
                continue;
            }
            let rank = rule.direction.specificity(required) + 1;
            match best {
                Some(_) if rank < best_rank => {}
                Some(previous) if rank == best_rank => {
                    tied = Some(previous);
                    best = Some(rule);
                }
                _ => {
                    best = Some(rule);
                    best_rank = rank;
                    tied = None;
                }
            }
        }

        match (best, tied) {
            (Some(rule), None) => Ok(rule),
            (Some(rule), Some(other)) => Err(ConnectError::AmbiguousConnectRule {
                continuous: continuous.into(),
                discrete: discrete.into(),
                direction: required,
                first: other.connect_module.clone(),
                second: rule.connect_module.clone(),
            }),
            (None, _) => Err(ConnectError::NoConnectRule {
                net: SmolStr::default(),
                continuous: continuous.into(),
                discrete: discrete.into(),
                direction: required,
            }),
        }
    }

    /// Section 7.7.2.1's connect rule resolution mechanism.
    ///
    /// "When there is an exact match for the set of disciplines specified as
    /// part of the discipline_list, the resolved discipline would be as per
    /// the rule specified in the exact match. […] When there is no exact fit,
    /// then the resolved discipline would be based on the subset of the rules
    /// specified." Both cases warn and take the first rule when more than one
    /// applies.
    fn resolve_list(
        &self,
        found: &BTreeSet<SmolStr>,
        net: &str,
        warnings: &mut Vec<String>,
    ) -> Result<Option<SmolStr>, ConnectError> {
        let exact: Vec<&ResolutionRule> = self
            .resolutions
            .iter()
            .filter(|rule| rule.disciplines == *found)
            .collect();
        let candidates: Vec<&ResolutionRule> = if exact.is_empty() {
            // "When there is no exact fit, then the resolved discipline would
            // be based on the subset of the rules specified." The subset is
            // the *net's* list inside the rule's, which is the reading the
            // standard's Example 1 forces: a net carrying only `y` and `a`
            // resolves to `a` through the rule written `connect x,y,a
            // resolveto a`, and `{x,y,a}` is not contained in `{y,a}`.
            //
            // Nothing ranks the survivors by how much of the rule they used.
            // Example 2's net carrying `x` and `y` matches all three rules and
            // is stated to take the first, which a size preference would break
            // — `{x,y,b}` and `{x,y,a}` are the same size.
            self.resolutions
                .iter()
                .filter(|rule| found.is_subset(&rule.disciplines))
                .collect()
        } else {
            exact
        };

        let Some(first) = candidates.first() else {
            return Ok(None);
        };
        if candidates.len() > 1 {
            warnings.push(format!(
                "net '{net}' matches {} connect resolution rules; \
                 Verilog-AMS LRM 2.4 section 7.7.2.1 takes the first",
                candidates.len()
            ));
        }
        match &first.target {
            ConnectResolveTarget::Exclude => Err(ConnectError::ExcludedDisciplines {
                net: net.into(),
                disciplines: first.disciplines.iter().cloned().collect(),
            }),
            ConnectResolveTarget::Discipline(target) => Ok(Some(target.clone())),
        }
    }
}

/// Section 3.11.1's discipline compatibility rules.
///
/// * Self Rule: a discipline is compatible with itself.
/// * Natureless Discipline Rule: a natureless discipline is compatible with
///   all other disciplines of the same domain.
/// * Domain Incompatibility Rule: disciplines with different domain attributes
///   are incompatible.
/// * Potential / Flow Incompatibility Rules: incompatible natures make the
///   disciplines incompatible.
///
/// Nature compatibility is the Self Rule plus the Non-Existent Binding Rule —
/// "a nature is compatible with a non-existent discipline binding" — which is
/// what makes a discipline that declares only a potential compatible with one
/// that declares both. The Base and Derived Nature Rules need a nature's base,
/// which [`crate::disciplines::Nature`] does not retain after
/// [`crate::semantic`] folds a derived nature into a standalone one, so two
/// natures derived from one base are compared by the Units Value Rule instead:
/// "two natures are compatible if they have the same value for the units
/// attribute".
pub fn disciplines_compatible(db: &DisciplineDb, left: &str, right: &str) -> bool {
    if left == right {
        return true;
    }
    let (Some(left), Some(right)) = (db.get_discipline(left), db.get_discipline(right)) else {
        return false;
    };
    if left.domain != right.domain {
        return false;
    }
    let natureless = |d: &crate::disciplines::Discipline| d.potential.is_none() && d.flow.is_none();
    if natureless(left) || natureless(right) {
        return true;
    }
    natures_compatible(db, left.potential.as_deref(), right.potential.as_deref())
        && natures_compatible(db, left.flow.as_deref(), right.flow.as_deref())
}

fn natures_compatible(db: &DisciplineDb, left: Option<&str>, right: Option<&str>) -> bool {
    let (Some(left), Some(right)) = (left, right) else {
        // Non-Existent Binding Rule.
        return true;
    };
    if left == right {
        return true;
    }
    match (db.get_nature(left), db.get_nature(right)) {
        (Some(left), Some(right)) => left.units == right.units,
        _ => false,
    }
}

fn discipline_domain(db: &DisciplineDb, name: &str) -> Option<Domain> {
    db.get_discipline(name).map(|discipline| discipline.domain)
}

/// Section 7.4.4's two modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ResolutionMode {
    /// Section 7.4.4.1 and Annex F.2.1.
    #[default]
    Basic,
    /// Section 7.4.4.2 and Annex F.2.2. Refused; see the module documentation.
    Detail,
}

/// One net segment of a signal — section 7.2.3's "the appearance of a signal
/// in a particular context".
#[derive(Debug, Clone)]
pub struct NetSegment {
    /// Section 7.8.5's `SigName`.
    pub name: SmolStr,
    /// The discipline declared on the net in this context, in or out of it.
    /// Section 3.10 gives out-of-context declarations precedence over local
    /// ones; both arrive here already reduced to one name, because that is a
    /// conflict the declaring pass can see and this one cannot.
    pub declared: Option<SmolStr>,
    /// Annex F.2.1 step 4a: "Any net which is used in digital behavioral code
    /// shall be considered digital."
    pub digital_behavioral: bool,
    /// Ports for which this segment is the upper (actual) connection.
    pub children: Vec<PortLink>,
}

impl NetSegment {
    pub fn new(name: impl Into<SmolStr>) -> Self {
        Self {
            name: name.into(),
            declared: None,
            digital_behavioral: false,
            children: Vec::new(),
        }
    }

    pub fn declared(mut self, discipline: impl Into<SmolStr>) -> Self {
        self.declared = Some(discipline.into());
        self
    }

    pub fn digital_behavioral(mut self) -> Self {
        self.digital_behavioral = true;
        self
    }

    pub fn with_child(mut self, link: PortLink) -> Self {
        self.children.push(link);
        self
    }
}

/// One port, which section 7.8.4 defines as "a connection between two net
/// segments of a signal".
#[derive(Debug, Clone)]
pub struct PortLink {
    /// The lower (formal) connection: the segment inside the instantiated
    /// module.
    pub lower: usize,
    /// The direction the instantiated module declares for the port.
    pub direction: PortDirection,
    /// Section 7.8.5's `InstName`.
    pub instance: SmolStr,
    /// Section 7.8.5's `PortName`.
    pub port: SmolStr,
}

impl PortLink {
    pub fn new(
        lower: usize,
        direction: PortDirection,
        instance: impl Into<SmolStr>,
        port: impl Into<SmolStr>,
    ) -> Self {
        Self {
            lower,
            direction,
            instance: instance.into(),
            port: port.into(),
        }
    }
}

/// One signal: a contiguous collection of net segments joined by ports.
///
/// Segments are addressed by their index in [`Signal::segments`], and a
/// segment's [`PortLink`]s name its children by the same index. A segment with
/// no parent is a root.
#[derive(Debug, Clone, Default)]
pub struct Signal {
    pub segments: Vec<NetSegment>,
}

impl Signal {
    pub fn push(&mut self, segment: NetSegment) -> usize {
        self.segments.push(segment);
        self.segments.len() - 1
    }

    fn roots(&self) -> Vec<usize> {
        let mut has_parent = vec![false; self.segments.len()];
        for segment in &self.segments {
            for link in &segment.children {
                if let Some(slot) = has_parent.get_mut(link.lower) {
                    *slot = true;
                }
            }
        }
        (0..self.segments.len())
            .filter(|index| !has_parent[*index])
            .collect()
    }

    /// Segment indices in post-order depth-first order, which is the traversal
    /// Annex F.2 defines: "all the children net segments of a parent net
    /// segment shall be traversed before that parent net segment is traversed".
    fn post_order(&self) -> Result<Vec<usize>, ConnectError> {
        let mut order = Vec::with_capacity(self.segments.len());
        let mut state = vec![0u8; self.segments.len()];
        for root in self.roots() {
            self.visit(root, &mut state, &mut order)?;
        }
        if order.len() != self.segments.len() {
            // Every segment not reached from a root is inside a cycle, which a
            // port hierarchy cannot have.
            return Err(ConnectError::CyclicSignal);
        }
        Ok(order)
    }

    fn visit(
        &self,
        index: usize,
        state: &mut [u8],
        order: &mut Vec<usize>,
    ) -> Result<(), ConnectError> {
        match state.get(index) {
            None => return Err(ConnectError::UnknownSegment { index }),
            Some(2) => return Ok(()),
            Some(1) => return Err(ConnectError::CyclicSignal),
            _ => {}
        }
        state[index] = 1;
        for link in &self.segments[index].children {
            self.visit(link.lower, state, order)?;
        }
        state[index] = 2;
        order.push(index);
        Ok(())
    }
}

/// What resolution produced.
#[derive(Debug, Clone)]
pub struct ResolvedSignal {
    /// One entry per segment, in segment order. `None` is section 7.4.4's
    /// legal unknown: a net with no mixed-port connection needs no discipline.
    pub disciplines: Vec<Option<SmolStr>>,
    pub domains: Vec<Option<Domain>>,
    pub warnings: Vec<String>,
}

impl ResolvedSignal {
    pub fn discipline(&self, segment: usize) -> Option<&str> {
        self.disciplines.get(segment)?.as_deref()
    }

    pub fn domain(&self, segment: usize) -> Option<Domain> {
        *self.domains.get(segment)?
    }
}

/// Section 7.4 discipline resolution, in the basic mode of section 7.4.4.1.
///
/// `default_discipline` is section 3.8's directive, applied by Annex F.2.1
/// step 4b's first bullet: "If there are no disciplines in the list apply any
/// `default_discipline` directives to the net, provided their domain is the
/// same as the domain of the net."
pub fn resolve_disciplines(
    signal: &Signal,
    table: &ConnectRuleTable,
    db: &DisciplineDb,
    default_discipline: Option<&str>,
    mode: ResolutionMode,
) -> Result<ResolvedSignal, ConnectError> {
    if mode == ResolutionMode::Detail {
        return Err(ConnectError::DetailResolutionMode);
    }

    let mut disciplines: Vec<Option<SmolStr>> = vec![None; signal.segments.len()];
    let mut domains: Vec<Option<Domain>> = vec![None; signal.segments.len()];
    let mut warnings = Vec::new();

    // Annex F.2.1 steps 2 and 3: apply every declaration first, so a declared
    // segment is never overwritten by what its children carry. Section
    // 7.4.4.3 calls this coercion.
    for (index, segment) in signal.segments.iter().enumerate() {
        let Some(declared) = segment.declared.as_deref() else {
            continue;
        };
        let Some(domain) = discipline_domain(db, declared) else {
            return Err(ConnectError::UnknownDiscipline {
                name: declared.into(),
                context: segment.name.clone(),
            });
        };
        disciplines[index] = Some(declared.into());
        domains[index] = Some(domain);
    }

    // Annex F.2.1 step 4, post-order so a parent always sees resolved children.
    for index in signal.post_order()? {
        if disciplines[index].is_some() {
            continue;
        }
        let segment = &signal.segments[index];

        // 4a. "Any net which is used in digital behavioral code shall be
        // considered digital. Any net whose child nets are all digital shall
        // be considered digital (discrete domain), any others shall be
        // considered analog (continuous domain)." A segment with no children
        // satisfies "all children digital" vacuously, which is the reading
        // that keeps a leaf out of the continuous domain it has no claim on.
        let domain = if segment.digital_behavioral {
            Domain::Discrete
        } else if segment
            .children
            .iter()
            .all(|link| domains[link.lower] != Some(Domain::Continuous))
        {
            Domain::Discrete
        } else {
            Domain::Continuous
        };
        domains[index] = Some(domain);

        // 4b. "examine all the child nets of that net and construct a list of
        // all disciplines of the child nets whose domains match the domain of
        // the segment".
        let found: BTreeSet<SmolStr> = segment
            .children
            .iter()
            .filter(|link| domains[link.lower] == Some(domain))
            .filter_map(|link| disciplines[link.lower].clone())
            .collect();

        let resolved = match found.len() {
            0 => match default_discipline {
                // "provided their domain is the same as the domain of the net"
                Some(default) if discipline_domain(db, default) == Some(domain) => {
                    Some(SmolStr::from(default))
                }
                Some(_) | None => None,
            },
            1 => found.iter().next().cloned(),
            _ => table.resolve_list(&found, &segment.name, &mut warnings)?,
        };
        disciplines[index] = resolved;
    }

    // 4b's last bullet: "Otherwise the discipline is unknown. This is legal
    // provided the net has no mixed-port connections (i.e., it does not
    // connect through a port to a segment of a different domain). Otherwise
    // this is an error." A port is a connection in both directions, so a
    // segment's parent counts as much as its children do.
    let mut parent_domain: Vec<Option<Domain>> = vec![None; signal.segments.len()];
    for (index, segment) in signal.segments.iter().enumerate() {
        for link in &segment.children {
            parent_domain[link.lower] = domains[index];
        }
    }
    for (index, segment) in signal.segments.iter().enumerate() {
        if disciplines[index].is_some() {
            continue;
        }
        let own_domain = domains[index];
        let crosses_domain = segment
            .children
            .iter()
            .any(|link| domains[link.lower] != own_domain)
            || parent_domain[index].is_some_and(|domain| Some(domain) != own_domain);
        if crosses_domain {
            return Err(ConnectError::UnresolvedDiscipline {
                net: segment.name.clone(),
            });
        }
    }

    Ok(ResolvedSignal {
        disciplines,
        domains,
        warnings,
    })
}

/// One port a connect module instance bridges.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectModuleBinding {
    /// The port's upper (actual) net segment.
    pub upper: usize,
    /// The port's lower (formal) net segment.
    pub lower: usize,
    pub instance: SmolStr,
    pub port: SmolStr,
    pub direction: PortDirection,
}

/// One planned connect module instance.
#[derive(Debug, Clone)]
pub struct ConnectModuleInsertion {
    /// Section 7.8.5's generated instance name.
    pub instance: String,
    pub connect_module: SmolStr,
    pub mode: ConnectMode,
    /// Section 7.8.4 insertion rule 2: "The connect module for a port shall be
    /// instantiated in the context of the port's upper connection."
    pub context: usize,
    /// The resolved discipline on the continuous side of the boundary, and the
    /// name of the connect module port that binds to it.
    pub continuous: SmolStr,
    pub continuous_port: SmolStr,
    /// The same for the discrete side.
    pub discrete: SmolStr,
    pub discrete_port: SmolStr,
    pub direction: ConnectDirection,
    /// Every design port this one instance bridges, in traversal order. More
    /// than one is section 7.8.3.1's `merged`.
    pub bindings: Vec<ConnectModuleBinding>,
    pub parameters: Vec<ParameterOverride>,
}

/// What insertion planning produced.
#[derive(Debug, Clone, Default)]
pub struct ConnectModulePlan {
    pub insertions: Vec<ConnectModuleInsertion>,
    pub warnings: Vec<String>,
}

/// Section 7.8: plan every connect module instance a resolved signal needs.
///
/// Section 7.8.4 narrows the work before it starts: the rules "apply only to
/// mixed signals", and a connection element "is selected for each port where
/// one connection is analog and the other digital". Every other port is
/// skipped, which is why a wholly digital or wholly analog signal produces an
/// empty plan rather than an error.
pub fn plan_connect_modules(
    signal: &Signal,
    resolved: &ResolvedSignal,
    table: &ConnectRuleTable,
    db: &DisciplineDb,
) -> Result<ConnectModulePlan, ConnectError> {
    let mut plan = ConnectModulePlan::default();
    // Section 7.8.4 insertion rule 3 groups ports by their upper connection,
    // the module selected, and the bottom discipline; rule 4 gives every other
    // port its own instance. The key carries exactly rule 3's three parts.
    let mut merged: HashMap<(usize, SmolStr, SmolStr), usize> = HashMap::new();

    for (upper, segment) in signal.segments.iter().enumerate() {
        let Some(upper_domain) = resolved.domain(upper) else {
            continue;
        };
        for link in &segment.children {
            let Some(lower_domain) = resolved.domain(link.lower) else {
                continue;
            };
            if upper_domain == lower_domain {
                continue;
            }

            let upper_discipline = resolved.discipline(upper).unwrap_or_default();
            let lower_discipline = resolved.discipline(link.lower).unwrap_or_default();
            let (continuous, discrete) = if upper_domain == Domain::Continuous {
                (upper_discipline, lower_discipline)
            } else {
                (lower_discipline, upper_discipline)
            };
            let required = required_direction(link.direction, upper_domain);

            let rule =
                table
                    .select(continuous, discrete, required, db)
                    .map_err(|error| match error {
                        // The selector knows the disciplines but not the net; the
                        // diagnostic clause 7 asks for names both.
                        ConnectError::NoConnectRule {
                            continuous,
                            discrete,
                            direction,
                            ..
                        } => ConnectError::NoConnectRule {
                            net: segment.name.clone(),
                            continuous,
                            discrete,
                            direction,
                        },
                        other => other,
                    })?;

            let binding = ConnectModuleBinding {
                upper,
                lower: link.lower,
                instance: link.instance.clone(),
                port: link.port.clone(),
                direction: link.direction,
            };
            let bottom_discipline = SmolStr::from(lower_discipline);

            if rule.mode == ConnectMode::Merged {
                let key = (
                    upper,
                    rule.connect_module.clone(),
                    bottom_discipline.clone(),
                );
                if let Some(existing) = merged.get(&key) {
                    plan.insertions[*existing].bindings.push(binding);
                    continue;
                }
                merged.insert(key, plan.insertions.len());
            }

            plan.insertions.push(ConnectModuleInsertion {
                instance: insertion_instance_name(rule, segment, &binding, &bottom_discipline),
                connect_module: rule.connect_module.clone(),
                mode: rule.mode,
                context: upper,
                continuous: continuous.into(),
                continuous_port: rule.continuous.name.clone(),
                discrete: discrete.into(),
                discrete_port: rule.discrete.name.clone(),
                direction: required,
                bindings: vec![binding],
                parameters: rule.parameters.clone(),
            });
        }
    }

    plan.warnings = resolved.warnings.clone();
    Ok(plan)
}

/// Section 7.8.5's naming scheme. The separator is a double underscore.
fn insertion_instance_name(
    rule: &InsertionRule,
    segment: &NetSegment,
    binding: &ConnectModuleBinding,
    bottom_discipline: &str,
) -> String {
    match rule.mode {
        ConnectMode::Merged => format!(
            "{}__{}__{}",
            segment.name, rule.connect_module, bottom_discipline
        ),
        ConnectMode::Split => {
            format!("{}__{}__{}", segment.name, binding.instance, binding.port)
        }
    }
}

/// Which of Table 7-2's kinds a mixed port needs.
///
/// Section 7.6's two unidirectional examples turn on which side *drives*: an
/// `input` port is driven by its upper connection, an `output` port by its
/// lower one. The connect module reads the driving side, so its `input` port
/// carries that side's discipline.
fn required_direction(direction: PortDirection, upper_domain: Domain) -> ConnectDirection {
    let driver_is_upper = match direction {
        PortDirection::Input => true,
        PortDirection::Output => false,
        PortDirection::Inout => return ConnectDirection::Bidirectional,
    };
    let driver_domain = if driver_is_upper {
        upper_domain
    } else {
        match upper_domain {
            Domain::Continuous => Domain::Discrete,
            Domain::Discrete => Domain::Continuous,
        }
    };
    match driver_domain {
        Domain::Continuous => ConnectDirection::AnalogToDiscrete,
        Domain::Discrete => ConnectDirection::DiscreteToAnalog,
    }
}

/// Read every `connectmodule` and `connectrules` block in a file into one
/// validated table.
pub fn build_connect_rule_table(
    source: &SourceFile,
    db: &DisciplineDb,
) -> Result<ConnectRuleTable, ConnectError> {
    let mut modules = BTreeMap::new();
    for item in &source.items {
        if let Item::ConnectModule(module) = item {
            let decl = connect_module_decl(module, db)?;
            modules.insert(decl.name.clone(), decl);
        }
    }

    let mut table = ConnectRuleTable {
        modules,
        ..Default::default()
    };
    for item in &source.items {
        let Item::ConnectRules(block) = item else {
            continue;
        };
        extend_table(&mut table, block, db)?;
    }
    Ok(table)
}

fn extend_table(
    table: &mut ConnectRuleTable,
    block: &ConnectRulesDecl,
    db: &DisciplineDb,
) -> Result<(), ConnectError> {
    for item in &block.items {
        match item {
            ConnectRulesItem::Insertion(insertion) => {
                let rule = insertion_rule(table, insertion, db)?;
                table.insertions.push(rule);
            }
            ConnectRulesItem::Resolution(resolution) => {
                table.resolutions.push(resolution_rule(resolution, db)?);
            }
        }
    }
    Ok(())
}

fn resolution_rule(
    resolution: &ConnectResolution,
    db: &DisciplineDb,
) -> Result<ResolutionRule, ConnectError> {
    for name in &resolution.disciplines {
        if db.get_discipline(name).is_none() {
            return Err(ConnectError::UnknownDiscipline {
                name: name.clone(),
                context: "connect … resolveto".into(),
            });
        }
    }
    if let ConnectResolveTarget::Discipline(target) = &resolution.target
        && db.get_discipline(target).is_none()
    {
        return Err(ConnectError::UnknownDiscipline {
            name: target.clone(),
            context: "connect … resolveto".into(),
        });
    }
    Ok(ResolutionRule {
        disciplines: resolution.disciplines.iter().cloned().collect(),
        target: resolution.target.clone(),
        span: resolution.span,
    })
}

fn insertion_rule(
    table: &ConnectRuleTable,
    insertion: &ConnectInsertion,
    db: &DisciplineDb,
) -> Result<InsertionRule, ConnectError> {
    let decl = table
        .modules
        .get(&insertion.connect_module)
        .ok_or_else(|| ConnectError::UnknownConnectModule {
            name: insertion.connect_module.clone(),
            span: insertion.span,
        })?;

    let mut continuous = decl.continuous.clone();
    let mut discrete = decl.discrete.clone();

    if let Some(overrides) = &insertion.port_overrides {
        // Section 7.7.1: "the specified disciplines shall be compatible for
        // both the continuous and discrete disciplines of the given connect
        // module". Which override is which side is settled by the override's
        // own domain, not by its position, because Syntax 7-6 fixes no order.
        let mut sides = [&overrides.first, &overrides.second];
        let domains = sides.map(|side| discipline_domain(db, &side.discipline));
        for (side, domain) in sides.iter().zip(domains) {
            if domain.is_none() {
                return Err(ConnectError::UnknownDiscipline {
                    name: side.discipline.clone(),
                    context: insertion.connect_module.clone(),
                });
            }
        }
        if domains[0] == domains[1] {
            // Section 7.8: "When two disciplines are specified in a connect
            // statement, one shall be discrete and the other continuous."
            return Err(ConnectError::RuleDomains {
                module: insertion.connect_module.clone(),
                span: insertion.span,
            });
        }
        if domains[0] == Some(Domain::Discrete) {
            sides.swap(0, 1);
        }
        for (side, declared) in sides.iter().zip([&decl.continuous, &decl.discrete]) {
            if !disciplines_compatible(db, &side.discipline, &declared.discipline) {
                return Err(ConnectError::OverrideIncompatible {
                    module: insertion.connect_module.clone(),
                    declared: declared.discipline.clone(),
                    specified: side.discipline.clone(),
                    span: side.span,
                });
            }
        }
        continuous.discipline = sides[0].discipline.clone();
        discrete.discipline = sides[1].discipline.clone();
        if let (Some(first), Some(second)) = (sides[0].direction, sides[1].direction) {
            continuous.direction = first;
            discrete.direction = second;
        }
    }

    // With no directed override the declaration's own kind stands, already
    // validated against Table 7-2 when the module was read. An override that
    // named directions has to be checked again, because section 7.7.1 lets it
    // pick any pair and only Table 7-2's three are admissible.
    let direction = if insertion
        .port_overrides
        .as_ref()
        .is_some_and(|overrides| overrides.first.direction.is_some())
    {
        ConnectDirection::from_ports(continuous.direction, discrete.direction).ok_or_else(|| {
            ConnectError::ConnectModuleDirections {
                module: insertion.connect_module.clone(),
                span: insertion.span,
            }
        })?
    } else {
        decl.direction
    };

    Ok(InsertionRule {
        connect_module: insertion.connect_module.clone(),
        // Section 7.8.3: "The default is merged."
        mode: insertion.mode.unwrap_or(ConnectMode::Merged),
        parameters: insertion.parameters.clone(),
        continuous,
        discrete,
        direction,
        span: insertion.span,
    })
}

/// Reduce a `connectmodule` declaration to the two ports section 7.6 reads.
fn connect_module_decl(
    module: &Module,
    db: &DisciplineDb,
) -> Result<ConnectModuleDecl, ConnectError> {
    let mut ports: Vec<ConnectModulePort> = Vec::new();
    // A net declaration overrides the discipline written on the port itself,
    // which is what `crate::semantic` does for an ordinary module.
    let net_disciplines: HashMap<&str, &str> = module
        .nets
        .iter()
        .flat_map(|net| {
            net.names
                .iter()
                .map(move |name| (name.as_str(), net.discipline.as_str()))
        })
        .collect();

    for declaration in &module.port_declarations {
        for name in &declaration.names {
            let discipline = net_disciplines
                .get(name.as_str())
                .copied()
                .or(declaration.discipline.as_deref())
                .ok_or_else(|| ConnectError::ConnectModulePortDiscipline {
                    module: module.name.clone(),
                    port: name.clone(),
                    span: declaration.span,
                })?;
            ports.push(ConnectModulePort {
                name: name.clone(),
                discipline: discipline.into(),
                direction: declaration.direction,
            });
        }
    }

    if ports.len() != 2 {
        return Err(ConnectError::ConnectModulePortCount {
            module: module.name.clone(),
            found: ports.len(),
            span: module.span,
        });
    }

    let mut domains = Vec::with_capacity(2);
    for port in &ports {
        domains.push(discipline_domain(db, &port.discipline).ok_or_else(|| {
            ConnectError::UnknownDiscipline {
                name: port.discipline.clone(),
                context: module.name.clone(),
            }
        })?);
    }
    let (continuous, discrete) = match (domains[0], domains[1]) {
        (Domain::Continuous, Domain::Discrete) => (ports[0].clone(), ports[1].clone()),
        (Domain::Discrete, Domain::Continuous) => (ports[1].clone(), ports[0].clone()),
        _ => {
            return Err(ConnectError::ConnectModuleDomains {
                module: module.name.clone(),
                span: module.span,
            });
        }
    };

    let direction = ConnectDirection::from_ports(continuous.direction, discrete.direction)
        .ok_or_else(|| ConnectError::ConnectModuleDirections {
            module: module.name.clone(),
            span: module.span,
        })?;

    Ok(ConnectModuleDecl {
        name: module.name.clone(),
        continuous,
        discrete,
        direction,
        span: module.span,
    })
}

/// Everything clause 7's machinery refuses, each naming what it refused.
#[derive(Debug, Clone, thiserror::Error)]
pub enum ConnectError {
    #[error(
        "connect statement names '{name}', which is not a declared connectmodule \
         (Verilog-AMS LRM 2.4 section 7.7.1)"
    )]
    UnknownConnectModule { name: SmolStr, span: Span },

    #[error(
        "connectmodule '{module}' declares {found} ports; Verilog-AMS LRM 2.4 section 7.6 \
         bridges exactly one continuous and one discrete port"
    )]
    ConnectModulePortCount {
        module: SmolStr,
        found: usize,
        span: Span,
    },

    #[error(
        "connectmodule '{module}' port '{port}' has no discipline; Verilog-AMS LRM 2.4 \
         section 7.6 selects a connect module by its port disciplines"
    )]
    ConnectModulePortDiscipline {
        module: SmolStr,
        port: SmolStr,
        span: Span,
    },

    #[error(
        "connectmodule '{module}' does not bridge one continuous and one discrete discipline \
         (Verilog-AMS LRM 2.4 section 7.8)"
    )]
    ConnectModuleDomains { module: SmolStr, span: Span },

    #[error(
        "connectmodule '{module}' port directions are not one of Verilog-AMS LRM 2.4 \
         Table 7-2's three combinations: continuous input with discrete output, \
         continuous output with discrete input, or both inout"
    )]
    ConnectModuleDirections { module: SmolStr, span: Span },

    #[error("undefined discipline '{name}' in {context}")]
    UnknownDiscipline { name: SmolStr, context: SmolStr },

    #[error(
        "connect statement for '{module}' overrides discipline '{declared}' with '{specified}', \
         which is not compatible with it (Verilog-AMS LRM 2.4 section 7.7.1)"
    )]
    OverrideIncompatible {
        module: SmolStr,
        declared: SmolStr,
        specified: SmolStr,
        span: Span,
    },

    #[error(
        "connect statement for '{module}' names two disciplines of the same domain; \
         Verilog-AMS LRM 2.4 section 7.8 requires one discrete and one continuous"
    )]
    RuleDomains { module: SmolStr, span: Span },

    #[error(
        "net '{net}' connects discipline '{continuous}' to discipline '{discrete}' and no \
         connect statement applies to that {} pair (Verilog-AMS LRM 2.4 section 7.8.4)",
        direction.label()
    )]
    NoConnectRule {
        net: SmolStr,
        continuous: SmolStr,
        discrete: SmolStr,
        direction: ConnectDirection,
    },

    #[error(
        "disciplines '{continuous}' and '{discrete}' match both connect module '{first}' and \
         '{second}'; Verilog-AMS LRM 2.4 section 7.8.4 requires one and only one"
    )]
    AmbiguousConnectRule {
        continuous: SmolStr,
        discrete: SmolStr,
        direction: ConnectDirection,
        first: SmolStr,
        second: SmolStr,
    },

    #[error(
        "net '{net}' carries disciplines {disciplines:?}, which a `resolveto exclude` statement \
         declares incompatible (Verilog-AMS LRM 2.4 section 7.7.2)"
    )]
    ExcludedDisciplines {
        net: SmolStr,
        disciplines: Vec<SmolStr>,
    },

    #[error(
        "net '{net}' has no resolved discipline and connects segments of different domains \
         (Verilog-AMS LRM 2.4 Annex F.2.1 step 4b)"
    )]
    UnresolvedDiscipline { net: SmolStr },

    #[error(
        "the detail discipline resolution mode of Verilog-AMS LRM 2.4 section 7.4.4.2 and \
         Annex F.2.2 is not implemented; section 7.4.4 makes the choice of mode \
         vendor-specific and this compiler implements the basic mode of section 7.4.4.1"
    )]
    DetailResolutionMode,

    #[error(
        "the connect statement for '{module}' passes a parameter positionally; \
         Verilog-AMS LRM 2.4 section 7.7.3 parameter values are bound to a connect \
         module's parameters by name"
    )]
    PositionalConnectParameter { module: SmolStr, span: Span },

    #[error(
        "the connect statement for '{module}' gives parameter '{parameter}' a value that is \
         not a numeric literal; a Verilog-AMS LRM 2.4 section 7.7.3 parameter is written \
         where no design scope exists to evaluate an expression against"
    )]
    NonLiteralConnectParameter {
        module: SmolStr,
        parameter: SmolStr,
        span: Span,
    },

    #[error("net segment {index} is not part of the signal")]
    UnknownSegment { index: usize },

    #[error("the signal's port hierarchy contains a cycle")]
    CyclicSignal,
}

#[cfg(test)]
mod tests;
