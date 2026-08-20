//! Source-authentic specialist checks retained with runtime artifacts.
//!
//! These checks are advisory engineering evidence, not substitutes for the
//! parser, semantic analyzer, or backend qualification gates. Findings name
//! the exact conservative rule that fired; an evaluated check with no finding
//! means only that the implemented rules found none.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::ast::{Item, Module, SourceFile};
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

pub(crate) fn analyze(
    analyzed: &AnalyzedFile,
    module_name: &str,
    source: &str,
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
    walk_regions(
        &module.body,
        source,
        &state_variables,
        0,
        evidence,
        findings,
    );
}

fn walk_regions(
    regions: &[AnalyzedRegion],
    source: &str,
    state_variables: &BTreeSet<String>,
    conditional_depth: usize,
    evidence: &mut SpecialistEvidence,
    findings: &mut Vec<SpecialistFinding>,
) {
    for region in regions {
        match region {
            AnalyzedRegion::Assignment(_) => {}
            AnalyzedRegion::Contribution(contribution) => {
                evidence.contributions += 1;
                let text = source_slice(source, contribution.span).to_ascii_lowercase();
                let discontinuous = conditional_depth > 0 || uses_discontinuous_operator(&text);
                let smoothed = text.contains("transition(") || text.contains("slew(");
                if discontinuous && !smoothed {
                    findings.push(finding(
                        SpecialistCheckKind::Discontinuity,
                        "VA-DISCONTINUITY-UNSMOOTHED",
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
                        "Unbounded exponential in contribution",
                        "The contribution calls exp() directly; limexp() generally provides a Newton-safe exponential for compact models.",
                        contribution.span,
                        Some(("Consider limexp()", "Replace exp() with limexp() only after confirming equivalent model intent.")),
                    ));
                }
                if text.contains('/') {
                    findings.push(finding(
                        SpecialistCheckKind::Convergence,
                        "VA-CONVERGENCE-DIVISION",
                        "Review contribution denominator",
                        "The contribution contains division. Verify that every reachable denominator is bounded away from zero across the qualified parameter domain.",
                        contribution.span,
                        None,
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
                let then_assigned = assigned_state_targets(then_body, state_variables);
                let else_assigned = assigned_state_targets(else_body, state_variables);
                for target in then_assigned.symmetric_difference(&else_assigned) {
                    findings.push(finding(
                        SpecialistCheckKind::HiddenState,
                        "VA-HIDDEN-STATE-PARTIAL-ASSIGNMENT",
                        "State variable is not assigned on every branch",
                        &format!(
                            "State variable '{target}' is assigned in only one branch of this conditional; its previous value may become hidden state."
                        ),
                        *span,
                        Some((
                            "Assign an explicit branch value",
                            "Provide physically correct assignments on every branch or document intentional retained state.",
                        )),
                    ));
                }
                walk_regions(
                    then_body,
                    source,
                    state_variables,
                    conditional_depth + 1,
                    evidence,
                    findings,
                );
                walk_regions(
                    else_body,
                    source,
                    state_variables,
                    conditional_depth + 1,
                    evidence,
                    findings,
                );
            }
            AnalyzedRegion::Loop { body, span, .. } => {
                evidence.runtime_loops += 1;
                if !assigned_state_targets(body, state_variables).is_empty() {
                    findings.push(finding(
                        SpecialistCheckKind::HiddenState,
                        "VA-HIDDEN-STATE-RUNTIME-LOOP",
                        "State assignment occurs in a runtime loop",
                        "A runtime loop may execute zero times, so state assignments inside it require an explicit value on the zero-iteration path.",
                        *span,
                        None,
                    ));
                }
                findings.push(finding(
                    SpecialistCheckKind::Convergence,
                    "VA-CONVERGENCE-RUNTIME-LOOP",
                    "Runtime loop requires a bounded termination review",
                    "The loop bound is evaluated at runtime. Verify monotonic progress and a finite iteration bound for every qualified parameter set.",
                    *span,
                    None,
                ));
                walk_regions(
                    body,
                    source,
                    state_variables,
                    conditional_depth,
                    evidence,
                    findings,
                );
            }
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
                action: None,
            });
        }
    }
}

fn finding(
    check: SpecialistCheckKind,
    code: &str,
    summary: &str,
    detail: &str,
    span: Span,
    action: Option<(&str, &str)>,
) -> SpecialistFinding {
    SpecialistFinding {
        check,
        code: code.to_owned(),
        severity: SpecialistFindingSeverity::Warning,
        summary: summary.to_owned(),
        detail: detail.to_owned(),
        span: span.into(),
        action: action.map(|(title, replacement_hint)| SpecialistCodeAction {
            title: title.to_owned(),
            replacement_hint: replacement_hint.to_owned(),
        }),
    }
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
        let source_map = crate::SourceMap::new();
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
