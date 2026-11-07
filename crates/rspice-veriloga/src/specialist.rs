//! Source-authentic specialist checks retained with runtime artifacts.
//!
//! These checks are advisory engineering evidence, not substitutes for the
//! parser, semantic analyzer, or backend qualification gates. Findings name
//! the exact conservative rule that fired; an evaluated check with no finding
//! means only that the implemented rules found none.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::ast::{BinaryOp, Expression, Item, Module, SourceFile, UnaryOp};
use crate::prepared_diagnostics::PreparedSourceMap;
use crate::runtime_report::{
    RuntimeQualificationOptions, RuntimeTarget, RuntimeTargetQualifications, RuntimeTargetReadiness,
};
use crate::semantic::{AnalyzedFile, AnalyzedModule, AnalyzedRegion};
use crate::source::Span;

pub const SPECIALIST_REPORT_SCHEMA: &str = "rspice.veriloga-specialist/v2";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecialistReport {
    pub schema: String,
    pub module_name: String,
    pub checks: Vec<SpecialistCheckSummary>,
    pub findings: Vec<SpecialistFinding>,
    pub evidence: SpecialistEvidence,
}

impl Default for SpecialistReport {
    fn default() -> Self {
        Self {
            schema: SPECIALIST_REPORT_SCHEMA.to_owned(),
            module_name: String::new(),
            checks: SpecialistCheckKind::ALL
                .into_iter()
                .map(|kind| SpecialistCheckSummary {
                    kind,
                    disposition: SpecialistCheckDisposition::NotEvaluated,
                    findings: 0,
                    detail:
                        "Report was decoded from an artifact that predates specialist evidence."
                            .to_owned(),
                })
                .collect(),
            findings: Vec::new(),
            evidence: SpecialistEvidence::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpecialistCheckKind {
    HiddenState,
    Discontinuity,
    UnitsAndRanges,
    Convergence,
    Portability,
}

impl SpecialistCheckKind {
    pub const ALL: [Self; 5] = [
        Self::HiddenState,
        Self::Discontinuity,
        Self::UnitsAndRanges,
        Self::Convergence,
        Self::Portability,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpecialistCheckDisposition {
    Evaluated,
    Findings,
    EvidenceOnly,
    NotEvaluated,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecialistCheckSummary {
    pub kind: SpecialistCheckKind,
    pub disposition: SpecialistCheckDisposition,
    pub findings: usize,
    pub detail: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SpecialistFindingSeverity {
    Information,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecialistFinding {
    pub check: SpecialistCheckKind,
    pub code: String,
    pub severity: SpecialistFindingSeverity,
    pub summary: String,
    pub detail: String,
    pub span: SpecialistSpan,
    /// Where `span` starts in the author's own files, when the compile kept
    /// a map back to them. A finding about the module as a whole has none.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<SpecialistLocation>,
    pub action: Option<SpecialistCodeAction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecialistSpan {
    pub source_id: u32,
    pub byte_start: u32,
    pub byte_end: u32,
}

impl From<Span> for SpecialistSpan {
    fn from(span: Span) -> Self {
        Self {
            source_id: span.source.raw(),
            byte_start: span.start,
            byte_end: span.end,
        }
    }
}

/// A finding's place in the document it was written in.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecialistLocation {
    /// The document's logical path; `None` for text compiled without a
    /// document name.
    pub path: Option<String>,
    /// One-based line.
    pub line: usize,
    /// One-based column, counted in characters.
    pub column: usize,
    /// The finding's bytes in that document.
    pub byte_start: usize,
    pub byte_end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecialistCodeAction {
    pub title: String,
    pub replacement_hint: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecialistEvidence {
    pub state_variables: usize,
    pub conditional_regions: usize,
    pub runtime_loops: usize,
    pub contributions: usize,
    pub parameters: usize,
    pub parameters_with_units: usize,
    pub parameters_with_ranges: usize,
    pub module_instances: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instance_bindings: Vec<SpecialistModuleInstance>,
    pub requested_generated_rust: bool,
    pub requested_native_x64_jit: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecialistModuleInstance {
    pub instance_name: String,
    /// Stable path relative to the selected root module.
    pub instance_path: String,
    pub module_name: String,
    pub parent_module: String,
}

/// `source_map`, when the compile kept one, places each finding in the file
/// and on the line and column it was written.
pub(crate) fn analyze(
    analyzed: &AnalyzedFile,
    module_name: &str,
    source: &str,
    source_map: Option<&PreparedSourceMap>,
    targets: &RuntimeTargetQualifications,
    requested: RuntimeQualificationOptions,
) -> SpecialistReport {
    let analyzed_module = analyzed.modules.get(module_name);
    let ast_module = analyzed.source.items.iter().find_map(|item| match item {
        Item::Module(module) | Item::ConnectModule(module) if module.name == module_name => {
            Some(module)
        }
        _ => None,
    });
    let mut findings = Vec::new();
    let mut evidence = SpecialistEvidence {
        requested_generated_rust: requested.generated_rust,
        requested_native_x64_jit: requested.native_x64_jit,
        ..SpecialistEvidence::default()
    };

    if let Some(module) = analyzed_module {
        analyze_regions(module, source, &mut evidence, &mut findings);
    }
    if let Some(module) = ast_module {
        analyze_parameter_evidence(module, &mut evidence);
    }
    analyze_instance_evidence(&analyzed.source, module_name, &mut evidence);
    analyze_portability(targets, requested, &mut findings);

    // Elaboration can copy one source site into several executable regions.
    // Report a source diagnostic once while retaining the full region census.
    // Match the identity that validation enforces; distinct spans and details
    // remain independent findings, including different variables at one site.
    let mut identities = BTreeSet::new();
    findings.retain(|finding| {
        identities.insert((
            finding.check,
            finding.code.clone(),
            finding.span.source_id,
            finding.span.byte_start,
            finding.span.byte_end,
            finding.detail.clone(),
        ))
    });
    if let Some(source_map) = source_map {
        for finding in &mut findings {
            finding.location = locate(source_map, source, finding.span);
        }
    }

    let mut counts = BTreeMap::new();
    for finding in &findings {
        *counts.entry(finding.check).or_insert(0_usize) += 1;
    }
    let checks = SpecialistCheckKind::ALL
        .into_iter()
        .map(|kind| {
            let count = counts.get(&kind).copied().unwrap_or_default();
            let (disposition, detail) = match kind {
                SpecialistCheckKind::UnitsAndRanges => (
                    SpecialistCheckDisposition::EvidenceOnly,
                    format!(
                        "Recorded {} parameters, {} unit declarations, and {} range constraints; dimensional intent is not inferred for undeclared parameters.",
                        evidence.parameters,
                        evidence.parameters_with_units,
                        evidence.parameters_with_ranges
                    ),
                ),
                _ if count == 0 => (
                    SpecialistCheckDisposition::Evaluated,
                    check_rule_detail(kind).to_owned(),
                ),
                _ => (
                    SpecialistCheckDisposition::Findings,
                    check_rule_detail(kind).to_owned(),
                ),
            };
            SpecialistCheckSummary {
                kind,
                disposition,
                findings: count,
                detail,
            }
        })
        .collect();

    SpecialistReport {
        schema: SPECIALIST_REPORT_SCHEMA.to_owned(),
        module_name: module_name.to_owned(),
        checks,
        findings,
        evidence,
    }
}

impl SpecialistReport {
    pub(crate) fn validate(&self, module_name: &str) -> Result<(), String> {
        if self.schema != SPECIALIST_REPORT_SCHEMA {
            return Err(format!("unsupported specialist schema '{}'", self.schema));
        }
        if self.module_name != module_name {
            return Err(format!(
                "specialist module '{}' does not match artifact module '{module_name}'",
                self.module_name
            ));
        }
        if self.checks.len() != SpecialistCheckKind::ALL.len() {
            return Err("specialist check matrix is not exhaustive".to_owned());
        }
        let kinds = self
            .checks
            .iter()
            .map(|check| check.kind)
            .collect::<BTreeSet<_>>();
        if kinds.len() != SpecialistCheckKind::ALL.len() {
            return Err("specialist check matrix contains duplicate kinds".to_owned());
        }
        for check in &self.checks {
            let actual = self
                .findings
                .iter()
                .filter(|finding| finding.check == check.kind)
                .count();
            if actual != check.findings {
                return Err(format!(
                    "specialist finding count mismatch for {:?}: summary={}, actual={actual}",
                    check.kind, check.findings
                ));
            }
        }
        let mut identities = BTreeSet::new();
        for finding in &self.findings {
            if finding.code.trim().is_empty()
                || finding.summary.trim().is_empty()
                || finding.detail.trim().is_empty()
                || finding.span.byte_start > finding.span.byte_end
            {
                return Err("specialist finding contains invalid required fields".to_owned());
            }
            let identity = (
                finding.check,
                finding.code.as_str(),
                finding.span.source_id,
                finding.span.byte_start,
                finding.span.byte_end,
                finding.detail.as_str(),
            );
            if !identities.insert(identity) {
                return Err("specialist report contains a duplicate finding".to_owned());
            }
        }
        if self.evidence.module_instances != self.evidence.instance_bindings.len() {
            return Err(format!(
                "specialist module-instance count mismatch: summary={}, actual={}",
                self.evidence.module_instances,
                self.evidence.instance_bindings.len()
            ));
        }
        let mut instance_paths = BTreeSet::new();
        for binding in &self.evidence.instance_bindings {
            let leaf = binding.instance_path.rsplit('/').next().unwrap_or_default();
            if binding.instance_name.is_empty()
                || binding.instance_path.is_empty()
                || binding.module_name.is_empty()
                || binding.parent_module.is_empty()
                || leaf != binding.instance_name
            {
                return Err("specialist module-instance evidence is malformed".to_owned());
            }
            if !instance_paths.insert(binding.instance_path.as_str()) {
                return Err(format!(
                    "specialist module-instance path '{}' is duplicated",
                    binding.instance_path
                ));
            }
        }
        Ok(())
    }
}

fn analyze_regions(
    module: &AnalyzedModule,
    source: &str,
    evidence: &mut SpecialistEvidence,
    findings: &mut Vec<SpecialistFinding>,
) {
    let state_variables = module
        .variables
        .iter()
        .map(|variable| variable.name.to_string())
        .collect::<BTreeSet<_>>();
    evidence.state_variables = state_variables.len();
    let parameters = module
        .parameters
        .iter()
        .map(|parameter| {
            (
                parameter.name.to_string(),
                parameter
                    .range
                    .as_ref()
                    .map_or(Signs::ANY, Signs::of_range),
            )
        })
        .collect();
    let context = RegionContext {
        source,
        state_variables: &state_variables,
        parameters,
    };
    walk_regions(
        &module.body,
        &context,
        0,
        &mut PathFacts::default(),
        evidence,
        findings,
    );
}

/// What the region walk reads but never changes.
struct RegionContext<'a> {
    source: &'a str,
    state_variables: &'a BTreeSet<String>,
    /// The signs each parameter's declared range allows. A parameter with no
    /// range can be set to anything, zero included.
    parameters: BTreeMap<String, Signs>,
}

/// What every evaluation path reaching a point has established: the
/// variables assigned on all of them, each with the signs its value can
/// have there. A variable missing here may still hold whatever the previous
/// evaluation left in it.
#[derive(Debug, Clone, Default)]
struct PathFacts {
    assigned: BTreeMap<String, Signs>,
}

impl PathFacts {
    /// The facts that hold after either of two paths: a variable is assigned
    /// only when both assigned it, to a value either could have left.
    fn join(&self, other: &Self) -> Self {
        Self {
            assigned: self
                .assigned
                .iter()
                .filter_map(|(name, signs)| {
                    other
                        .assigned
                        .get(name)
                        .map(|other| (name.clone(), signs.union(*other)))
                })
                .collect(),
        }
    }

    /// Forget the values of `targets`, which a region that may or may not
    /// run assigns, keeping whether each was already assigned.
    fn widen(&mut self, targets: &BTreeSet<String>) {
        for target in targets {
            if let Some(signs) = self.assigned.get_mut(target) {
                *signs = Signs::ANY;
            }
        }
    }
}

fn walk_regions(
    regions: &[AnalyzedRegion],
    context: &RegionContext<'_>,
    conditional_depth: usize,
    facts: &mut PathFacts,
    evidence: &mut SpecialistEvidence,
    findings: &mut Vec<SpecialistFinding>,
) {
    for region in regions {
        match region {
            AnalyzedRegion::Task(_) => {}
            // An initialization runs in its own phase only, so a variable it
            // assigns may hold either value afterwards.
            AnalyzedRegion::Initialization { body, .. } => {
                facts.widen(&assigned_state_targets(body, context.state_variables));
            }
            AnalyzedRegion::Assignment(assignment) => {
                if assignment.index.is_none() {
                    let signs = context.signs(&assignment.expression, facts);
                    facts.assigned.insert(assignment.target.to_string(), signs);
                }
            }
            AnalyzedRegion::Contribution(contribution) => {
                evidence.contributions += 1;
                let text = source_slice(context.source, contribution.span).to_ascii_lowercase();
                let discontinuous = conditional_depth > 0 || uses_discontinuous_operator(&text);
                let smoothed = text.contains("transition(") || text.contains("slew(");
                if discontinuous && !smoothed {
                    findings.push(finding(
                        SpecialistCheckKind::Discontinuity,
                        "VA-DISCONTINUITY-UNSMOOTHED",
                        SpecialistFindingSeverity::Warning,
                        "Potential unsmoothed discontinuity",
                        "A contribution is conditional or uses a discontinuous operator without transition() or slew() in the same source region.",
                        contribution.span,
                        Some((
                            "Review transition smoothing",
                            "Use transition() or slew() only when the physical model permits a continuous approximation.",
                        )),
                    ));
                }
                if contains_raw_exp(&text) {
                    findings.push(finding(
                        SpecialistCheckKind::Convergence,
                        "VA-CONVERGENCE-RAW-EXP",
                        SpecialistFindingSeverity::Warning,
                        "Unbounded exponential in contribution",
                        "The contribution calls exp() directly; limexp() generally provides a Newton-safe exponential for compact models.",
                        contribution.span,
                        Some(("Consider limexp()", "Replace exp() with limexp() only after confirming equivalent model intent.")),
                    ));
                }
                for denominator in context.unbounded_denominators(&contribution.expression, facts) {
                    let text = source_slice(context.source, denominator)
                        .split_whitespace()
                        .collect::<Vec<_>>()
                        .join(" ");
                    findings.push(finding(
                        SpecialistCheckKind::Convergence,
                        "VA-CONVERGENCE-DIVISION",
                        SpecialistFindingSeverity::Information,
                        "Denominator can reach zero",
                        &format!(
                            "Nothing keeps the denominator `{text}` away from zero: no declared parameter range, positive constant or positive function such as exp() or max(x, eps) excludes it."
                        ),
                        denominator,
                        Some((
                            "Bound the denominator",
                            "Declare a parameter range that excludes zero, or divide by max(x, eps) where the model allows it.",
                        )),
                    ));
                }
            }
            AnalyzedRegion::Conditional {
                then_body,
                else_body,
                span,
                ..
            } => {
                evidence.conditional_regions += 1;
                let then_assigned = assigned_state_targets(then_body, context.state_variables);
                let else_assigned = assigned_state_targets(else_body, context.state_variables);
                // A variable assigned on every path before the conditional
                // holds this evaluation's value on the branch that leaves it
                // alone: there is no state to hide.
                for target in then_assigned
                    .symmetric_difference(&else_assigned)
                    .filter(|target| !facts.assigned.contains_key(*target))
                {
                    findings.push(finding(
                        SpecialistCheckKind::HiddenState,
                        "VA-HIDDEN-STATE-PARTIAL-ASSIGNMENT",
                        SpecialistFindingSeverity::Information,
                        &format!("'{target}' is not assigned on every branch"),
                        &format!(
                            "State variable '{target}' is assigned in only one branch of this conditional and not before it; its previous value may become hidden state."
                        ),
                        *span,
                        Some((
                            "Assign an explicit branch value",
                            "Provide physically correct assignments on every branch or document intentional retained state.",
                        )),
                    ));
                }
                let mut then_facts = facts.clone();
                walk_regions(
                    then_body,
                    context,
                    conditional_depth + 1,
                    &mut then_facts,
                    evidence,
                    findings,
                );
                let mut else_facts = facts.clone();
                walk_regions(
                    else_body,
                    context,
                    conditional_depth + 1,
                    &mut else_facts,
                    evidence,
                    findings,
                );
                *facts = then_facts.join(&else_facts);
            }
            AnalyzedRegion::Loop { body, span, .. } => {
                evidence.runtime_loops += 1;
                let body_assigned = assigned_state_targets(body, context.state_variables);
                if !body_assigned.is_empty() {
                    findings.push(finding(
                        SpecialistCheckKind::HiddenState,
                        "VA-HIDDEN-STATE-RUNTIME-LOOP",
                        SpecialistFindingSeverity::Warning,
                        "State assignment occurs in a runtime loop",
                        "A runtime loop may execute zero times, so state assignments inside it require an explicit value on the zero-iteration path.",
                        *span,
                        None,
                    ));
                }
                findings.push(finding(
                    SpecialistCheckKind::Convergence,
                    "VA-CONVERGENCE-RUNTIME-LOOP",
                    SpecialistFindingSeverity::Warning,
                    "Runtime loop requires a bounded termination review",
                    "The loop bound is evaluated at runtime. Verify monotonic progress and a finite iteration bound for every qualified parameter set.",
                    *span,
                    None,
                ));
                // A later iteration sees what an earlier one assigned, and the
                // loop may run no iteration at all.
                let mut body_facts = facts.clone();
                body_facts.widen(&body_assigned);
                walk_regions(
                    body,
                    context,
                    conditional_depth,
                    &mut body_facts,
                    evidence,
                    findings,
                );
                *facts = facts.join(&body_facts);
            }
        }
    }
}

impl RegionContext<'_> {
    /// The denominators in `expression` that nothing established on this
    /// path keeps away from zero.
    fn unbounded_denominators(&self, expression: &Expression, facts: &PathFacts) -> Vec<Span> {
        let mut denominators = Vec::new();
        crate::ast::visit_expression(expression, &mut |node| {
            if let Expression::Binary(binary) = node
                && matches!(binary.op, BinaryOp::Div | BinaryOp::IntDiv)
                && self.signs(&binary.right, facts).may_be_zero()
            {
                denominators.push(binary.right.span());
            }
        });
        denominators
    }

    /// The signs `expression` can take on this path.
    fn signs(&self, expression: &Expression, facts: &PathFacts) -> Signs {
        match expression {
            Expression::Number(number) => Signs::of_value(number.value),
            Expression::Identifier(identifier) => {
                let name = identifier.name.as_str();
                facts
                    .assigned
                    .get(name)
                    .or_else(|| self.parameters.get(name))
                    .copied()
                    .unwrap_or(Signs::ANY)
            }
            Expression::SystemFunction(function) => {
                match (function.name.as_str(), function.args.as_slice()) {
                    ("$temperature" | "$vt", []) => Signs::POSITIVE,
                    ("$vt", [temperature]) => self.signs(temperature, facts),
                    ("$abstime", []) => Signs::NON_NEGATIVE,
                    _ => Signs::ANY,
                }
            }
            Expression::Unary(unary) => {
                let operand = self.signs(&unary.operand, facts);
                match unary.op {
                    UnaryOp::Pos | UnaryOp::ToReal => operand,
                    UnaryOp::Neg => operand.negated(),
                    UnaryOp::ToInteger => operand.truncated(),
                    _ => Signs::ANY,
                }
            }
            Expression::Binary(binary) => {
                let left = self.signs(&binary.left, facts);
                let right = self.signs(&binary.right, facts);
                match binary.op {
                    BinaryOp::Add | BinaryOp::IntAdd => left.plus(right),
                    BinaryOp::Sub | BinaryOp::IntSub => left.plus(right.negated()),
                    BinaryOp::Mul | BinaryOp::IntMul => left.times(right),
                    BinaryOp::Div => left.divided_by(right),
                    BinaryOp::IntDiv => left.divided_by(right).truncated(),
                    BinaryOp::Pow | BinaryOp::IntPow if left == Signs::POSITIVE => {
                        Signs::POSITIVE
                    }
                    _ => Signs::ANY,
                }
            }
            Expression::Conditional(conditional) => self
                .signs(&conditional.then_expr, facts)
                .union(self.signs(&conditional.else_expr, facts)),
            Expression::Call(call) => match (call.name.as_str(), call.args.as_slice()) {
                ("exp" | "limexp" | "cosh", [_]) => Signs::POSITIVE,
                ("abs", [value]) => self.signs(value, facts).absolute(),
                ("sqrt", [value]) => self.signs(value, facts).square_root(),
                ("pow", [base, _]) if self.signs(base, facts) == Signs::POSITIVE => {
                    Signs::POSITIVE
                }
                ("max", [left, right]) => self
                    .signs(left, facts)
                    .pairwise(self.signs(right, facts), Sign::larger),
                ("min", [left, right]) => self
                    .signs(left, facts)
                    .pairwise(self.signs(right, facts), Sign::smaller),
                _ => Signs::ANY,
            },
            _ => Signs::ANY,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Sign {
    Negative,
    Zero,
    Positive,
}

impl Sign {
    const ALL: [Self; 3] = [Self::Negative, Self::Zero, Self::Positive];

    const fn signs(self) -> Signs {
        match self {
            Self::Negative => Signs::NEGATIVE,
            Self::Zero => Signs::ZERO,
            Self::Positive => Signs::POSITIVE,
        }
    }

    fn larger(self, other: Self) -> Signs {
        Ord::max(self, other).signs()
    }

    fn smaller(self, other: Self) -> Signs {
        Ord::min(self, other).signs()
    }
}

/// The signs a real value can have: a set of negative, zero and positive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Signs(u8);

impl Signs {
    const NEGATIVE: Self = Self(0b001);
    const ZERO: Self = Self(0b010);
    const POSITIVE: Self = Self(0b100);
    const NON_NEGATIVE: Self = Self(0b110);
    const ANY: Self = Self(0b111);

    fn of_value(value: f64) -> Self {
        if value > 0.0 {
            Self::POSITIVE
        } else if value < 0.0 {
            Self::NEGATIVE
        } else if value == 0.0 {
            Self::ZERO
        } else {
            Self::ANY
        }
    }

    /// The signs a declared range admits. Only its numeric bounds and
    /// exclusions count; a bound another parameter supplies admits anything.
    fn of_range(range: &crate::types::ParameterRange) -> Self {
        let mut signs = Self::ANY;
        if let Some(min) = range.min {
            if min > 0.0 || (min == 0.0 && range.min_exclusive) {
                signs = signs.without(Self::NEGATIVE.union(Self::ZERO));
            } else if min == 0.0 {
                signs = signs.without(Self::NEGATIVE);
            }
        }
        if let Some(max) = range.max {
            if max < 0.0 || (max == 0.0 && range.max_exclusive) {
                signs = signs.without(Self::NON_NEGATIVE);
            } else if max == 0.0 {
                signs = signs.without(Self::POSITIVE);
            }
        }
        if range.exclude.contains(&0.0) {
            signs = signs.without(Self::ZERO);
        }
        signs.or_any()
    }

    const fn contains(self, sign: Sign) -> bool {
        self.0 & sign.signs().0 != 0
    }

    const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    const fn without(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    /// An empty set names no reachable value; read it as unknown.
    const fn or_any(self) -> Self {
        if self.0 == 0 { Self::ANY } else { self }
    }

    const fn may_be_zero(self) -> bool {
        self.contains(Sign::Zero)
    }

    fn pairwise(self, other: Self, combine: impl Fn(Sign, Sign) -> Self) -> Self {
        let mut signs = Self(0);
        for left in Sign::ALL.into_iter().filter(|sign| self.contains(*sign)) {
            for right in Sign::ALL.into_iter().filter(|sign| other.contains(*sign)) {
                signs = signs.union(combine(left, right));
            }
        }
        signs.or_any()
    }

    fn negated(self) -> Self {
        self.pairwise(Self::ZERO, |sign, _| match sign {
            Sign::Negative => Self::POSITIVE,
            Sign::Zero => Self::ZERO,
            Sign::Positive => Self::NEGATIVE,
        })
    }

    fn plus(self, other: Self) -> Self {
        self.pairwise(other, |left, right| match (left, right) {
            (Sign::Zero, sign) | (sign, Sign::Zero) => sign.signs(),
            (Sign::Positive, Sign::Positive) => Self::POSITIVE,
            (Sign::Negative, Sign::Negative) => Self::NEGATIVE,
            _ => Self::ANY,
        })
    }

    fn times(self, other: Self) -> Self {
        self.pairwise(other, |left, right| match (left, right) {
            (Sign::Zero, _) | (_, Sign::Zero) => Self::ZERO,
            (left, right) if left == right => Self::POSITIVE,
            _ => Self::NEGATIVE,
        })
    }

    /// The quotient's signs; a denominator that can only be zero gives no
    /// finite value, and so any.
    fn divided_by(self, other: Self) -> Self {
        let nonzero = other.without(Self::ZERO);
        if nonzero.0 == 0 {
            return Self::ANY;
        }
        self.times(nonzero)
    }

    /// Truncation toward zero can turn any nonzero value into zero.
    const fn truncated(self) -> Self {
        self.union(Self::ZERO)
    }

    fn absolute(self) -> Self {
        self.pairwise(Self::ZERO, |sign, _| match sign {
            Sign::Zero => Self::ZERO,
            _ => Self::POSITIVE,
        })
    }

    fn square_root(self) -> Self {
        if self == Self::POSITIVE {
            Self::POSITIVE
        } else if self.contains(Sign::Negative) {
            Self::ANY
        } else {
            Self::NON_NEGATIVE
        }
    }
}

fn assigned_state_targets(
    regions: &[AnalyzedRegion],
    state_variables: &BTreeSet<String>,
) -> BTreeSet<String> {
    let mut assigned = BTreeSet::new();
    for region in regions {
        match region {
            AnalyzedRegion::Task(_) | AnalyzedRegion::Initialization { .. } => {}
            AnalyzedRegion::Assignment(assignment) => {
                let target = assignment.target.to_string();
                if state_variables.contains(&target) {
                    assigned.insert(target);
                }
            }
            AnalyzedRegion::Conditional {
                then_body,
                else_body,
                ..
            } => {
                assigned.extend(assigned_state_targets(then_body, state_variables));
                assigned.extend(assigned_state_targets(else_body, state_variables));
            }
            AnalyzedRegion::Loop { body, .. } => {
                assigned.extend(assigned_state_targets(body, state_variables));
            }
            AnalyzedRegion::Contribution(_) => {}
        }
    }
    assigned
}

fn analyze_parameter_evidence(module: &Module, evidence: &mut SpecialistEvidence) {
    evidence.parameters = module.parameters.len();
    evidence.parameters_with_units = module
        .parameters
        .iter()
        .filter(|parameter| parameter.units.is_some())
        .count();
    evidence.parameters_with_ranges = module
        .parameters
        .iter()
        .filter(|parameter| parameter.range.is_some())
        .count();
}

fn analyze_instance_evidence(
    source: &SourceFile,
    selected_module: &str,
    evidence: &mut SpecialistEvidence,
) {
    let modules = source
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Module(module) | Item::ConnectModule(module) => {
                Some((module.name.as_str(), module))
            }
            _ => None,
        })
        .collect::<BTreeMap<_, _>>();
    let mut ancestry = BTreeSet::new();
    let mut bindings = Vec::new();
    walk_module_instances(selected_module, "", &modules, &mut ancestry, &mut bindings);
    evidence.module_instances = bindings.len();
    evidence.instance_bindings = bindings;
}

fn walk_module_instances(
    module_name: &str,
    prefix: &str,
    modules: &BTreeMap<&str, &Module>,
    ancestry: &mut BTreeSet<String>,
    bindings: &mut Vec<SpecialistModuleInstance>,
) {
    if !ancestry.insert(module_name.to_owned()) {
        return;
    }
    let Some(module) = modules.get(module_name).copied() else {
        ancestry.remove(module_name);
        return;
    };
    for instance in &module.instances {
        let instance_path = if prefix.is_empty() {
            instance.name.to_string()
        } else {
            format!("{prefix}/{}", instance.name)
        };
        bindings.push(SpecialistModuleInstance {
            instance_name: instance.name.to_string(),
            instance_path: instance_path.clone(),
            module_name: instance.module.to_string(),
            parent_module: module_name.to_owned(),
        });
        walk_module_instances(
            instance.module.as_str(),
            &instance_path,
            modules,
            ancestry,
            bindings,
        );
    }
    ancestry.remove(module_name);
}

fn analyze_portability(
    targets: &RuntimeTargetQualifications,
    requested: RuntimeQualificationOptions,
    findings: &mut Vec<SpecialistFinding>,
) {
    for (required, target) in [
        (true, RuntimeTarget::BytecodeVm),
        (true, RuntimeTarget::WasmInterpreter),
        (requested.generated_rust, RuntimeTarget::GeneratedRust),
        (requested.native_x64_jit, RuntimeTarget::NativeX64Jit),
    ] {
        let qualification = targets.get(target);
        if required && qualification.readiness != RuntimeTargetReadiness::Available {
            findings.push(SpecialistFinding {
                check: SpecialistCheckKind::Portability,
                code: format!("VA-PORTABILITY-{:?}", target).to_ascii_uppercase(),
                severity: SpecialistFindingSeverity::Warning,
                summary: format!("{} target did not qualify", target.label()),
                detail: qualification.detail.clone(),
                span: SpecialistSpan {
                    source_id: 0,
                    byte_start: 0,
                    byte_end: 0,
                },
                location: None,
                action: None,
            });
        }
    }
}

fn finding(
    check: SpecialistCheckKind,
    code: &str,
    severity: SpecialistFindingSeverity,
    summary: &str,
    detail: &str,
    span: Span,
    action: Option<(&str, &str)>,
) -> SpecialistFinding {
    SpecialistFinding {
        check,
        code: code.to_owned(),
        severity,
        summary: summary.to_owned(),
        detail: detail.to_owned(),
        span: span.into(),
        location: None,
        action: action.map(|(title, replacement_hint)| SpecialistCodeAction {
            title: title.to_owned(),
            replacement_hint: replacement_hint.to_owned(),
        }),
    }
}

/// Where a finding's span starts in its own document. A finding that covers
/// no source text is about the module as a whole and has no place.
fn locate(
    source_map: &PreparedSourceMap,
    source: &str,
    span: SpecialistSpan,
) -> Option<SpecialistLocation> {
    if span.byte_start >= span.byte_end {
        return None;
    }
    let found = source_map.map_span(
        source,
        &crate::CompileDiagnosticSpan {
            source_id: span.source_id,
            byte_start: span.byte_start,
            byte_end: span.byte_end,
            start: None,
            end: None,
        },
    )?;
    // Text compiled without a document is named "<input>" by the
    // preprocessor; its findings belong to whichever document holds it.
    let path = (found.path != std::path::Path::new("<input>"))
        .then(|| crate::virtual_source::path_to_logical(found.path));
    Some(SpecialistLocation {
        path,
        line: found.line,
        column: found.column,
        byte_start: found.range.start,
        byte_end: found.range.end,
    })
}

fn source_slice(source: &str, span: Span) -> &str {
    let start = usize::try_from(span.start).unwrap_or(usize::MAX);
    let end = usize::try_from(span.end).unwrap_or(usize::MAX);
    if start <= end
        && end <= source.len()
        && source.is_char_boundary(start)
        && source.is_char_boundary(end)
    {
        &source[start..end]
    } else {
        ""
    }
}

/// Whether a contribution's source region selects or compares rather than
/// only computing.
///
/// Each operator is read at its full width instead of being searched for as a
/// substring. Three operators contain a relational operator's characters
/// without being one: the contribution operator `<+` — which every
/// contribution has, so a substring test for `<` reports every model in the
/// language — and the shifts `<<`, `>>` (and their arithmetic forms). `<=`
/// and `>=` stay comparisons: analog assignment is `=` and contribution is
/// `<+`, so neither spelling is anything else here.
fn uses_discontinuous_operator(source: &str) -> bool {
    let bytes = source.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        let (is_discontinuous, width) = match &bytes[index..] {
            [b'<', b'<', b'<', ..] | [b'>', b'>', b'>', ..] => (false, 3),
            [b'<', b'+', ..] | [b'<', b'<', ..] | [b'>', b'>', ..] => (false, 2),
            [b'=', b'=', ..] | [b'!', b'=', ..] | [b'<', b'=', ..] | [b'>', b'=', ..] => (true, 2),
            [b'<', ..] | [b'>', ..] | [b'?', ..] | [b'%', ..] => (true, 1),
            _ => (false, 1),
        };
        if is_discontinuous {
            return true;
        }
        index += width;
    }
    false
}

fn contains_raw_exp(source: &str) -> bool {
    source.match_indices("exp(").any(|(index, _)| {
        let prefix = &source[..index];
        !prefix.ends_with("lim")
            && prefix
                .chars()
                .next_back()
                .is_none_or(|character| !character.is_ascii_alphanumeric() && character != '_')
    })
}

const fn check_rule_detail(kind: SpecialistCheckKind) -> &'static str {
    match kind {
        SpecialistCheckKind::HiddenState => {
            "Evaluated state assignments across conditional and zero-iteration runtime-loop paths."
        }
        SpecialistCheckKind::Discontinuity => {
            "Evaluated conditional/operator discontinuities in contribution regions and local smoothing evidence."
        }
        SpecialistCheckKind::UnitsAndRanges => {
            "Recorded declared parameter units and range constraints without inferring undeclared dimensional intent."
        }
        SpecialistCheckKind::Convergence => {
            "Evaluated raw exponentials, contribution divisions, and runtime-loop termination review points."
        }
        SpecialistCheckKind::Portability => {
            "Evaluated mandatory portable targets and every explicitly requested optimized backend."
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CompilerOptions, VerilogACompiler};

    #[test]
    fn report_finds_partial_state_and_raw_exponential() {
        let source = r#"
module specialist(p, n);
  inout p, n; electrical p, n;
  real state;
  analog begin
    if (V(p,n) > 0) state = 1;
    I(p,n) <+ exp(V(p,n)) / state;
  end
endmodule
"#;
        let report = VerilogACompiler::new(CompilerOptions::default())
            .compile_runtime(source, Some("specialist"))
            .unwrap();
        assert!(
            report
                .specialist
                .findings
                .iter()
                .any(|finding| { finding.code == "VA-HIDDEN-STATE-PARTIAL-ASSIGNMENT" })
        );
        assert!(
            report
                .specialist
                .findings
                .iter()
                .any(|finding| finding.code == "VA-CONVERGENCE-RAW-EXP")
        );
        report.validate_integrity().unwrap();
    }

    /// A behavioural op-amp as a reviewer writes one: both divisions are by
    /// quantities the parameter ranges keep positive, and the clip reassigns a
    /// variable assigned unconditionally just before it.
    const OP_AMP: &str = r#"`include "constants.vams"
`include "disciplines.vams"

module opamp_bhv(inp, inn, out, vdd, vss);
  input inp, inn;
  output out;
  inout vdd, vss;
  electrical inp, inn, out, vdd, vss, mid;
  parameter real a0 = 1e5 from (0:inf);
  parameter real gbw = 10M from (0:inf);
  parameter real sr = 5M from (0:inf);
  parameter real rout = 50 from (0:inf);
  parameter integer clip = 1 from [0:1];
  real vin, vslew, vtarget, wp;
  analog begin
    wp = `M_TWO_PI * gbw / a0;
    vin = V(inp, inn);
    I(mid) <+ -a0 * vin;
    I(mid) <+ V(mid) + ddt(V(mid)) / wp;
    vslew = slew(V(mid), sr, -sr);
    vtarget = vslew;
    if (clip)
      vtarget = min(max(vslew, V(vss)), V(vdd));
    I(out) <+ (V(out) - vtarget) / rout;
  end
endmodule
"#;

    fn findings_with_code(source: &str, module: &str, code: &str) -> Vec<SpecialistFinding> {
        let report = VerilogACompiler::default()
            .compile_runtime(source, Some(module))
            .expect("the model compiles");
        report.validate_integrity().unwrap();
        report
            .specialist
            .findings
            .into_iter()
            .filter(|finding| finding.code == code)
            .collect()
    }

    /// A module dividing `denominator`, with `r` ranged (0:inf), `g` ranged
    /// [1:10], `z` ranged [0:inf), `u` unranged, and `x` assigned `setup`
    /// before the contribution.
    fn division_findings(setup: &str, denominator: &str) -> Vec<SpecialistFinding> {
        let source = format!(
            "module divide(p, n, c);
  inout p, n, c; electrical p, n, c;
  parameter real r = 1 from (0:inf);
  parameter real g = 2 from [1:10];
  parameter real z = 1 from [0:inf);
  parameter real u = 1;
  real x;
  analog begin
    {setup}
    I(p, n) <+ V(p, n) / ({denominator});
  end
endmodule
"
        );
        findings_with_code(&source, "divide", "VA-CONVERGENCE-DIVISION")
    }

    #[test]
    fn ranged_denominators_and_a_clip_after_an_assignment_raise_no_lint() {
        for code in [
            "VA-CONVERGENCE-DIVISION",
            "VA-HIDDEN-STATE-PARTIAL-ASSIGNMENT",
        ] {
            assert_eq!(
                findings_with_code(OP_AMP, "opamp_bhv", code),
                [],
                "{code} on a module whose denominators are positive"
            );
        }
    }

    #[test]
    fn denominators_kept_from_zero_raise_nothing() {
        for (setup, denominator) in [
            ("", "r"),
            ("", "g"),
            ("", "2.5"),
            ("", "-r"),
            ("", "r * g"),
            ("", "r / g"),
            ("", "r + z"),
            ("", "r - -g"),
            ("", "max(V(c), 1e-12)"),
            ("", "max(1f, V(c))"),
            ("", "abs(V(c)) + 1"),
            ("", "exp(V(c))"),
            ("", "limexp(V(c))"),
            ("", "pow(r, V(c))"),
            ("", "sqrt(r)"),
            ("", "$vt"),
            ("", "$temperature"),
            ("", "V(c) > 0 ? r : g"),
            ("x = r * r;", "x"),
            ("x = 1; if (V(c) > 0) x = g; else x = r;", "x"),
            ("x = r; if (V(c) > 0) x = 2 * x;", "x"),
        ] {
            assert_eq!(
                division_findings(setup, denominator),
                [],
                "`{setup}` then a division by `{denominator}`"
            );
        }
    }

    #[test]
    fn denominators_that_can_reach_zero_are_reported() {
        for (setup, denominator) in [
            ("", "z"),
            ("", "u"),
            ("", "r - g"),
            ("", "V(c)"),
            ("", "sqrt(z)"),
            ("", "abs(V(c))"),
            ("", "max(V(c), z)"),
            ("", "pow(V(c), 2)"),
            ("x = r;", "x - 1"),
            ("if (V(c) > 0) x = r;", "x"),
            ("x = r; if (V(c) > 0) x = V(c);", "x"),
        ] {
            assert_eq!(
                division_findings(setup, denominator).len(),
                1,
                "`{setup}` then a division by `{denominator}`"
            );
        }
    }

    #[test]
    fn a_branch_denominator_is_one_information_finding_where_it_was_written() {
        let source = "`include \"disciplines.vams\"\n\nmodule probe(p, n, c);\n  inout p, n, c;\n  electrical p, n, c;\n  analog I(p, n) <+ V(p, n) / V(c);\nendmodule\n";
        let findings = findings_with_code(source, "probe", "VA-CONVERGENCE-DIVISION");
        let [finding] = findings.as_slice() else {
            panic!("one finding for one division: {findings:#?}");
        };
        assert_eq!(finding.severity, SpecialistFindingSeverity::Information);
        let location = finding.location.as_ref().expect("the finding is placed");
        assert_eq!(location.path, None, "the input was compiled without a name");
        assert_eq!((location.line, location.column), (6, 31));
        assert_eq!(&source[location.byte_start..location.byte_end], "V(c)");
    }

    #[test]
    fn a_partial_assignment_reports_only_a_variable_left_unassigned_before_it() {
        let source = r#"
module latch(p, n);
  inout p, n; electrical p, n;
  real held, fresh;
  analog begin
    fresh = 0;
    if (V(p, n) > 0) begin
      held = V(p, n);
      fresh = V(p, n);
    end
    I(p, n) <+ 1m * (held + fresh);
  end
endmodule
"#;
        let findings = findings_with_code(source, "latch", "VA-HIDDEN-STATE-PARTIAL-ASSIGNMENT");
        let [finding] = findings.as_slice() else {
            panic!("one variable is left unassigned: {findings:#?}");
        };
        assert!(finding.summary.contains("'held'"), "{}", finding.summary);
        assert_eq!(finding.severity, SpecialistFindingSeverity::Information);
        assert_eq!(finding.location.as_ref().map(|location| location.line), Some(7));
    }

    fn discontinuity_findings(source: &str, module: &str) -> Vec<String> {
        let report = VerilogACompiler::new(CompilerOptions::default())
            .compile_runtime(source, Some(module))
            .expect("the model compiles");
        report
            .specialist
            .findings
            .iter()
            .filter(|finding| finding.code == "VA-DISCONTINUITY-UNSMOOTHED")
            .map(|finding| finding.detail.clone())
            .collect()
    }

    #[test]
    fn repeated_source_sites_report_each_finding_once() {
        let source = "module repeated(p,n); inout p,n; electrical p,n; integer k;
            analog begin for(k=0;k<3;k=k+1) if(V(p,n)>0) I(p,n)<+exp(V(p,n));
            I(p,n)<+exp(2*V(p,n)); end endmodule";
        let report = VerilogACompiler::default()
            .compile_runtime(source, Some("repeated"))
            .expect("unrolling must not invalidate specialist diagnostics");
        report.validate_integrity().unwrap();
        assert_eq!(report.specialist.evidence.contributions, 4);
        assert_eq!(
            report
                .specialist
                .findings
                .iter()
                .filter(|finding| finding.code == "VA-CONVERGENCE-RAW-EXP")
                .count(),
            2
        );
        assert_eq!(
            report
                .specialist
                .findings
                .iter()
                .filter(|finding| finding.code == "VA-DISCONTINUITY-UNSMOOTHED")
                .count(),
            1
        );
        let mut duplicated = report.specialist.clone();
        let finding = duplicated.findings[0].clone();
        duplicated
            .checks
            .iter_mut()
            .find(|check| check.kind == finding.check)
            .unwrap()
            .findings += 1;
        duplicated.findings.push(finding);
        assert!(
            duplicated
                .validate("repeated")
                .unwrap_err()
                .contains("duplicate finding")
        );
    }

    #[test]
    fn the_contribution_operator_is_not_read_as_a_comparison() {
        // `<+` is the only `<` in this model. Reading it as a relational
        // operator reported every model in the language as discontinuous.
        let source = r#"
module smooth(p, n);
  inout p, n; electrical p, n;
  parameter real gain = 2.0;
  analog begin
    I(p, n) <+ gain * V(p, n);
    V(p, n) <+ 0.0;
  end
endmodule
"#;
        assert!(discontinuity_findings(source, "smooth").is_empty());
    }

    #[test]
    fn shifts_are_not_read_as_comparisons() {
        assert!(!uses_discontinuous_operator("i(p, n) <+ v(p, n)"));
        assert!(!uses_discontinuous_operator("x <+ (a << 2) + (b >> 1)"));
        assert!(!uses_discontinuous_operator("x <+ (a <<< 2) + (b >>> 1)"));
        for comparison in [
            "x <+ a < b",
            "x <+ a > b",
            "x <+ a <= b",
            "x <+ a >= b",
            "x <+ a == b",
            "x <+ a != b",
            "x <+ a ? b : c",
            "x <+ a % b",
        ] {
            assert!(
                uses_discontinuous_operator(comparison),
                "'{comparison}' is a comparison or a selection"
            );
        }
    }

    #[test]
    fn a_comparison_inside_the_contributed_value_is_still_reported() {
        let source = r#"
module stepped(p, n);
  inout p, n; electrical p, n;
  parameter real vth = 0.5;
  analog I(p, n) <+ (V(p, n) > vth) * V(p, n);
endmodule
"#;
        assert_eq!(discontinuity_findings(source, "stepped").len(), 1);
    }

    #[test]
    fn a_contribution_under_a_conditional_is_still_reported() {
        let source = r#"
module gated(p, n);
  inout p, n; electrical p, n;
  parameter real vth = 0.5;
  analog begin
    if (V(p, n) > vth) I(p, n) <+ V(p, n);
    else I(p, n) <+ 0.0;
  end
endmodule
"#;
        assert_eq!(discontinuity_findings(source, "gated").len(), 2);
    }

    #[test]
    fn units_check_reports_evidence_without_inventing_dimensional_intent() {
        let source = r#"
module evidence(p, n);
  inout p, n; electrical p, n;
  (* units="Ohm" *) parameter real resistance = 1k from (0:inf);
  parameter real multiplier = 1;
  analog I(p,n) <+ V(p,n) / resistance * multiplier;
endmodule
"#;
        let report = VerilogACompiler::default()
            .compile_runtime(source, Some("evidence"))
            .unwrap();
        assert_eq!(report.specialist.evidence.parameters, 2);
        assert_eq!(report.specialist.evidence.parameters_with_units, 1);
        assert_eq!(report.specialist.evidence.parameters_with_ranges, 1);
        let check = report
            .specialist
            .checks
            .iter()
            .find(|check| check.kind == SpecialistCheckKind::UnitsAndRanges)
            .unwrap();
        assert_eq!(check.disposition, SpecialistCheckDisposition::EvidenceOnly);
    }

    #[test]
    fn elaboration_evidence_uses_exact_hierarchical_instance_paths() {
        let source = r#"
module leaf(p, n);
  inout p, n; electrical p, n;
endmodule
module child(p, n);
  inout p, n; electrical p, n;
  leaf inner (.p(p), .n(n));
endmodule
module top(p, n);
  inout p, n; electrical p, n;
  child outer (.p(p), .n(n));
endmodule
"#;
        let mut source_map = crate::SourceMap::new();
        let source_id = source_map.add_source("<hierarchy-evidence>", source);
        let tokens = crate::Lexer::new(source, source_id)
            .collect_tokens()
            .unwrap();
        let parsed = crate::Parser::new(&tokens).parse().unwrap();
        let mut evidence = SpecialistEvidence::default();
        analyze_instance_evidence(&parsed, "top", &mut evidence);

        assert_eq!(evidence.module_instances, 2);
        assert_eq!(
            evidence
                .instance_bindings
                .iter()
                .map(|binding| {
                    (
                        binding.instance_path.as_str(),
                        binding.module_name.as_str(),
                        binding.parent_module.as_str(),
                    )
                })
                .collect::<Vec<_>>(),
            [("outer", "child", "top"), ("outer/inner", "leaf", "child"),]
        );
    }
}
