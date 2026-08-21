//! What the engine actually does with an independent source's waveform fields.
//!
//! A source property sheet can be filled in perfectly legally and still not
//! produce the waveform its labels describe, because the engine substitutes a
//! value for a field left at zero, clamps one to a range derived from another,
//! or reads a field the sheet never mentions. Stating that at edit time is the
//! whole point of this module.
//!
//! The rules are split by how firmly the engine speaks, because conflating the
//! two strengths is how a sheet ends up refusing a card the engine would have
//! run. Two rules that shipped as refusals are advisories here, and both were
//! verified against the evaluator rather than against SPICE folklore:
//!
//! - `TR + PW + TF > PER` was refused as "must fit within one period". The
//!   evaluator wraps `t_rel` modulo the authored period and returns V1 past
//!   `TR + PW + TF` (`circuit/storage/sources.rs` `evaluate_source_at_time_*`,
//!   the non-Xyce arm); an over-long pulse truncates, it is not rejected.
//! - `TD2 < TD1` was refused as "cannot precede the first". `resolve_exp_timing`
//!   keeps both as authored and the EXP arm subtracts the second term
//!   regardless, so the card runs.
//!
//! Only the ngspice arm of each resolver is described. Every netlist flavour the
//! project offers maps to `SpiceDialect::BestAvailable` or `SpiceDialect::Ngspice`
//! (`state/workspace.rs`, `spice_dialect`); nothing selects `SpiceDialect::Xyce`,
//! so the Xyce timing branches cannot be reached from a project and stating
//! their rules here would describe a run that cannot happen.
//!
//! One thing the rules deliberately do not do is quote a resolved number. An
//! omitted field resolves against the *active transient's* TSTEP/TSTOP, which
//! is a property of the analysis rather than of the source, and a source can be
//! read by several analyses with different stop times. The messages name the
//! substitution instead of predicting its value, so this module needs no plan
//! state and cannot disagree with the plan.

use crate::state::ComponentType;
use crate::state::property_types::{PropertySheet, PropertyValue};
use std::collections::HashMap;

/// How firmly the engine speaks about a field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContractStrength {
    /// The card is refused, or the run aborts on it. Editing cannot commit.
    Refusal,
    /// The card runs, and does something the field's label does not say.
    Advisory,
}

/// One thing the engine will do with the fields as they currently stand.
#[derive(Debug, Clone, PartialEq)]
pub struct SourceContractFinding {
    pub strength: ContractStrength,
    /// The property this is about, so the editor can put it beside the field.
    pub field: &'static str,
    pub message: String,
}

impl SourceContractFinding {
    fn refusal(field: &'static str, message: impl Into<String>) -> Self {
        Self {
            strength: ContractStrength::Refusal,
            field,
            message: message.into(),
        }
    }

    fn advisory(field: &'static str, message: impl Into<String>) -> Self {
        Self {
            strength: ContractStrength::Advisory,
            field,
            message: message.into(),
        }
    }
}

/// The waveform fields of one source, resolved the way the editor resolves them.
///
/// Resolution lives here rather than in the caller so that the rules and the
/// editor can never read a different number for the same field: an edited value
/// wins, then the sheet's default, then whatever the component already carries
/// in its parameter string.
pub struct SourceFields<'a> {
    values: &'a HashMap<String, PropertyValue>,
    sheet: &'a PropertySheet,
    params: &'a HashMap<String, String>,
    /// The component's primary value, which is where the first positional
    /// argument of a waveform is stored rather than in the parameter string.
    primary: Option<(&'a str, &'a str)>,
}

impl<'a> SourceFields<'a> {
    pub fn new(
        values: &'a HashMap<String, PropertyValue>,
        sheet: &'a PropertySheet,
        params: &'a HashMap<String, String>,
    ) -> Self {
        Self {
            values,
            sheet,
            params,
            primary: None,
        }
    }

    /// Name the field whose value is stored in `Component::value`.
    pub fn with_primary(mut self, field: &'a str, value: &'a str) -> Self {
        self.primary = Some((field, value));
        self
    }

    /// The field as a number, or `None` when it is absent or not a literal.
    ///
    /// An expression that does not reduce to a literal here is not a contract
    /// question: it is either a design variable the engine resolves later, or a
    /// parse error the expression validator already reports against the field.
    pub fn number(&self, name: &str) -> Option<f64> {
        self.values
            .get(name)
            .or_else(|| self.sheet.get(name).map(|def| &def.default_value))
            .and_then(property_value_as_number)
            .or_else(|| {
                self.params
                    .get(name)
                    .and_then(|value| crate::quantity::parse_engineering_value(value).ok())
            })
            .or_else(|| {
                self.primary
                    .filter(|(field, _)| *field == name)
                    .and_then(|(_, value)| crate::quantity::parse_engineering_value(value).ok())
            })
    }

    /// The field as a boolean, for the switch-shaped properties.
    fn boolean(&self, name: &str) -> Option<bool> {
        match self
            .values
            .get(name)
            .or_else(|| self.sheet.get(name).map(|def| &def.default_value))
        {
            Some(PropertyValue::Boolean(flag)) => Some(*flag),
            _ => self.params.get(name).map(|value| {
                matches!(
                    value.trim().to_ascii_lowercase().as_str(),
                    "1" | "true" | "yes" | "on"
                )
            }),
        }
    }
}

/// Read a property as a number, seeing through a braced literal.
///
/// A field the user typed `{2u}` into is still the number 2e-6 to the engine;
/// only a brace holding a *name* is a deferred reference, and that fails to
/// parse here and is correctly left alone.
fn property_value_as_number(value: &PropertyValue) -> Option<f64> {
    value.as_number().or_else(|| {
        let displayed = value.display_string();
        let literal = displayed
            .strip_prefix('{')
            .and_then(|displayed| displayed.strip_suffix('}'))
            .unwrap_or(&displayed);
        crate::quantity::parse_engineering_value(literal).ok()
    })
}

/// Everything the engine will do with this source that its labels do not say.
///
/// Returns an empty list for any component that is not an independent source.
pub fn source_contract_findings(
    kind: ComponentType,
    fields: &SourceFields<'_>,
) -> Vec<SourceContractFinding> {
    let mut findings = Vec::new();
    match kind {
        ComponentType::VoltageSourcePulse | ComponentType::CurrentSourcePulse => {
            pulse_findings(kind, fields, &mut findings);
        }
        ComponentType::VoltageSourceSin | ComponentType::CurrentSourceSin => {
            sin_findings(kind, fields, &mut findings);
        }
        ComponentType::VoltageSourceExp | ComponentType::CurrentSourceExp => {
            exp_findings(fields, &mut findings);
        }
        ComponentType::VoltageSourcePwl | ComponentType::CurrentSourcePwl => {
            pwl_findings(fields, &mut findings);
        }
        ComponentType::VoltageSourceSffm | ComponentType::CurrentSourceSffm => {
            sffm_findings(fields, &mut findings);
        }
        ComponentType::VoltageSourceAm | ComponentType::CurrentSourceAm => {
            am_findings(fields, &mut findings);
        }
        ComponentType::VoltageSourceNoise | ComponentType::CurrentSourceNoise => {
            trnoise_findings(fields, &mut findings);
        }
        _ => {}
    }
    findings
}

/// The initial-level field, which is `V1`/`VO` on a voltage source and
/// `I1`/`IO` on a current one.
fn is_current(kind: ComponentType) -> bool {
    matches!(
        kind,
        ComponentType::CurrentSourcePulse
            | ComponentType::CurrentSourceSin
            | ComponentType::CurrentSourceExp
            | ComponentType::CurrentSourcePwl
            | ComponentType::CurrentSourceSffm
            | ComponentType::CurrentSourceAm
            | ComponentType::CurrentSourceNoise
    )
}

/// `PULSE(V1 V2 TD TR TF PW PER NP)`.
///
/// The engine refuses nothing here beyond a value that is not a finite number,
/// which the schema and expression validators already reject field by field.
/// Every other departure is a substitution, so every rule below is an advisory.
/// Cites are to `resolve_pulse_timing_with_defaults` and the `SourceSpec::Pulse`
/// arm of the evaluator in `circuit/storage/sources.rs`.
fn pulse_findings(
    kind: ComponentType,
    fields: &SourceFields<'_>,
    findings: &mut Vec<SourceContractFinding>,
) {
    let initial = if is_current(kind) { "I1" } else { "V1" };
    // `rise`/`fall` keep an authored value only when it is finite and > 0;
    // anything else takes the analysis step.
    for (field, label) in [("tr", "TR"), ("tf", "TF")] {
        if fields.number(field).is_some_and(|edge| edge <= 0.0) {
            findings.push(SourceContractFinding::advisory(
                field,
                format!(
                    "{label} of 0 is not an ideal edge: the engine substitutes the analysis TSTEP"
                ),
            ));
        }
    }
    // A negative PW is neither kept nor defaulted: it falls through to zero.
    if fields.number("pw").is_some_and(|width| width < 0.0) {
        findings.push(SourceContractFinding::advisory(
            "pw",
            "A negative PW is read as 0 — the pulse becomes a triangle with no flat top",
        ));
    }
    // A non-positive authored PER is replaced by the edge-to-edge time.
    if fields.number("per").is_some_and(|period| period <= 0.0) {
        findings.push(SourceContractFinding::advisory(
            "per",
            "PER of 0 does not mean one shot: the engine repeats on TR + PW + TF",
        ));
    }
    if let (Some(rise), Some(width), Some(fall), Some(period)) = (
        fields.number("tr"),
        fields.number("pw"),
        fields.number("tf"),
        fields.number("per"),
    ) && period > 0.0
        && rise + width + fall > period
    {
        findings.push(SourceContractFinding::advisory(
            "per",
            "TR + PW + TF exceeds PER — the engine runs it and the pulse truncates inside its period",
        ));
    }
    if fields.number("td").is_some_and(|delay| delay < 0.0) {
        findings.push(SourceContractFinding::advisory(
            "td",
            "A negative TD is preserved and advances the waveform rather than being clamped to 0",
        ));
    }
    if fields.number("td").is_some_and(|delay| delay > 0.0) {
        findings.push(SourceContractFinding::advisory(
            "td",
            format!("Before TD the output holds {initial}, not 0"),
        ));
    }
    if fields.number("np").is_some_and(|count| count > 0.0) {
        findings.push(SourceContractFinding::advisory(
            "np",
            format!("The train stops after NP periods and holds {initial} for the rest of the run"),
        ));
    }
}

/// `SIN(VO VA FREQ TD THETA PHASE)`.
///
/// No refusals: `resolve_sin_frequency` substitutes for a zero frequency and
/// keeps a negative one, and the damping term is evaluated as authored.
fn sin_findings(
    kind: ComponentType,
    fields: &SourceFields<'_>,
    findings: &mut Vec<SourceContractFinding>,
) {
    let offset = if is_current(kind) { "IO" } else { "VO" };
    let amplitude = if is_current(kind) { "IA" } else { "VA" };
    if fields
        .number("freq")
        .is_some_and(|frequency| frequency == 0.0)
    {
        findings.push(SourceContractFinding::advisory(
            "freq",
            "A FREQ of 0 is not DC: the engine substitutes 1 / TSTOP of the analysis that runs it",
        ));
    }
    if fields
        .number("freq")
        .is_some_and(|frequency| frequency < 0.0)
    {
        findings.push(SourceContractFinding::advisory(
            "freq",
            "A negative FREQ is kept as authored and mirrors the waveform — state the phase instead",
        ));
    }
    if fields.number("theta").is_some_and(|damping| damping < 0.0) {
        findings.push(SourceContractFinding::advisory(
            "theta",
            "A negative THETA is a growing exponential, not a decaying one — the amplitude diverges",
        ));
    }
    if fields.number("td").is_some_and(|delay| delay > 0.0) {
        findings.push(SourceContractFinding::advisory(
            "td",
            format!("Before TD the output holds {offset} + {amplitude}·sin(PHASE), not 0 and not {offset}"),
        ));
    }
}

/// `EXP(V1 V2 TD1 TAU1 TD2 TAU2)`.
///
/// `resolve_exp_timing` treats an authored zero exactly like an omitted field,
/// so the sheet's own zero defaults are substituted rather than honoured. No
/// refusals: an out-of-order pair runs.
fn exp_findings(fields: &SourceFields<'_>, findings: &mut Vec<SourceContractFinding>) {
    for (field, label) in [("td1", "TD1"), ("tau1", "TAU1"), ("tau2", "TAU2")] {
        if fields.number(field).is_some_and(|value| value == 0.0) {
            findings.push(SourceContractFinding::advisory(
                field,
                format!(
                    "A {label} of 0 is read as omitted: the engine substitutes the analysis TSTEP"
                ),
            ));
        }
    }
    if fields.number("td2").is_some_and(|delay| delay == 0.0) {
        findings.push(SourceContractFinding::advisory(
            "td2",
            "A TD2 of 0 is read as omitted: the engine substitutes TD1 + the analysis TSTEP",
        ));
    }
    for (field, label) in [("tau1", "TAU1"), ("tau2", "TAU2")] {
        if fields.number(field).is_some_and(|tau| tau < 0.0) {
            findings.push(SourceContractFinding::advisory(
                field,
                format!("A negative {label} is kept as authored and the exponential diverges"),
            ));
        }
    }
    if let (Some(first), Some(second)) = (fields.number("td1"), fields.number("td2"))
        && second != 0.0
        && second < first
    {
        findings.push(SourceContractFinding::advisory(
            "td2",
            "TD2 precedes TD1 — the engine runs it, with the decay term starting before the rise",
        ));
    }
}

/// `PWL(t1 v1 t2 v2 ...) [TD=] [R=]`.
///
/// The point list itself is the PWL editor's contract and stays there; this
/// covers the two fields beside it. `validate_pwl_delay` in the netlist parser
/// is the one genuine refusal in the family.
fn pwl_findings(fields: &SourceFields<'_>, findings: &mut Vec<SourceContractFinding>) {
    match fields.number("td") {
        Some(delay) if delay < 0.0 => findings.push(SourceContractFinding::refusal(
            "td",
            "PWL TD must be finite and non-negative — the netlist parser refuses a negative delay",
        )),
        Some(delay) if delay > 0.0 => findings.push(SourceContractFinding::advisory(
            "td",
            "Output is exactly 0 before TD, then holds the first point's level until its time",
        )),
        _ => {}
    }
    if fields.boolean("repeat").unwrap_or(false) {
        findings.push(SourceContractFinding::advisory(
            "repeat",
            "Repeat emits R=0, which folds the whole list back to its first point",
        ));
    }
}

/// `SFFM(VO VA FC MDI FS TD PHASEM PHASEC)`.
///
/// No refusals: the evaluator limits MDI with ngspice's if/else-if chain, so a
/// negative FS turns `FC/FS` negative and MDI lands on that ratio (or on 0 when
/// below it) — the card runs, it just never uses the authored index.
fn sffm_findings(fields: &SourceFields<'_>, findings: &mut Vec<SourceContractFinding>) {
    if fields.number("fs").is_some_and(|signal| signal < 0.0) {
        findings.push(SourceContractFinding::advisory(
            "fs",
            "A negative FS inverts the MDI limiter: MDI lands on FC/FS (negative, mirroring the modulation) or on 0, never on the authored value",
        ));
    }
    if fields.number("fc").is_some_and(|carrier| carrier <= 0.0) {
        findings.push(SourceContractFinding::advisory(
            "fc",
            "An FC of 0 is read as omitted: the engine substitutes 5 / TSTOP",
        ));
    }
    if fields.number("fs").is_some_and(|signal| signal == 0.0) {
        findings.push(SourceContractFinding::advisory(
            "fs",
            "An FS of 0 is read as omitted: the engine substitutes 500 / TSTOP",
        ));
    }
    if let (Some(index), Some(carrier), Some(signal)) = (
        fields.number("mdi"),
        fields.number("fc"),
        fields.number("fs"),
    ) && carrier > 0.0
        && signal > 0.0
        && (index < 0.0 || index > carrier / signal)
    {
        findings.push(SourceContractFinding::advisory(
            "mdi",
            "MDI is clamped to 0 … FC/FS at evaluation — the authored index is outside that range",
        ));
    }
    if fields.number("td").is_some_and(|delay| delay > 0.0) {
        findings.push(SourceContractFinding::advisory(
            "td",
            "Output is exactly 0 before TD, not the offset",
        ));
    }
}

/// `AM(VO VMO VMA FM FC TD PHASEM PHASEC)`.
///
/// The sheet already exposes the engine's eight parameters under the engine's
/// names. No refusals: both frequencies substitute when non-positive.
fn am_findings(fields: &SourceFields<'_>, findings: &mut Vec<SourceContractFinding>) {
    if fields
        .number("fm")
        .is_some_and(|modulating| modulating <= 0.0)
    {
        findings.push(SourceContractFinding::advisory(
            "fm",
            "An FM of 0 is read as omitted: the engine substitutes 5 / TSTOP",
        ));
    }
    if fields.number("fc").is_some_and(|carrier| carrier <= 0.0) {
        findings.push(SourceContractFinding::advisory(
            "fc",
            "An FC of 0 is read as omitted: the engine substitutes 500 / TSTOP",
        ));
    }
    if let (Some(modulating), Some(carrier)) = (fields.number("fm"), fields.number("fc"))
        && modulating > 0.0
        && carrier > 0.0
        && carrier < modulating
    {
        findings.push(SourceContractFinding::advisory(
            "fc",
            "The carrier is slower than the envelope — the engine multiplies them anyway, and the result is not an AM envelope",
        ));
    }
    if fields.number("td").is_some_and(|delay| delay > 0.0) {
        findings.push(SourceContractFinding::advisory(
            "td",
            "Output is exactly 0 before TD, not the offset",
        ));
    }
}

/// `TRNOISE(NA NT NALPHA NAMP)`.
///
/// Both refusals are the netlist parser's own, and both are conditional on an
/// amplitude actually being set — a fully zeroed noise source parses.
fn trnoise_findings(fields: &SourceFields<'_>, findings: &mut Vec<SourceContractFinding>) {
    let white = fields.number("na").unwrap_or(0.0);
    let flicker = fields.number("namp").unwrap_or(0.0);
    if (white != 0.0 || flicker != 0.0) && fields.number("nt").is_some_and(|step| step <= 0.0) {
        findings.push(SourceContractFinding::refusal(
            "nt",
            "TRNOISE requires a positive sample interval NT whenever NA or NAMP is set",
        ));
    }
    if flicker != 0.0
        && fields
            .number("nalpha")
            .is_some_and(|alpha| !(0.0..2.0).contains(&alpha))
    {
        findings.push(SourceContractFinding::refusal(
            "nalpha",
            "TRNOISE NALPHA must satisfy 0 <= NALPHA < 2 when NAMP is set",
        ));
    }
    if white == 0.0 && flicker == 0.0 {
        findings.push(SourceContractFinding::advisory(
            "na",
            "Every noise amplitude is 0 — the source contributes exactly its DC level",
        ));
    }
    findings.push(SourceContractFinding::advisory(
        "nt",
        "The operating point sees exactly 0 from this source: the noise train is zero-mean and transient-only",
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sheet() -> PropertySheet {
        PropertySheet::new()
    }

    fn fields<'a>(
        values: &'a HashMap<String, PropertyValue>,
        sheet: &'a PropertySheet,
        params: &'a HashMap<String, String>,
    ) -> SourceFields<'a> {
        SourceFields::new(values, sheet, params)
    }

    fn params(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(key, value)| ((*key).to_owned(), (*value).to_owned()))
            .collect()
    }

    fn findings_for(kind: ComponentType, pairs: &[(&str, &str)]) -> Vec<SourceContractFinding> {
        let values = HashMap::new();
        let sheet = sheet();
        let params = params(pairs);
        source_contract_findings(kind, &fields(&values, &sheet, &params))
    }

    fn refusals(findings: &[SourceContractFinding]) -> Vec<&SourceContractFinding> {
        findings
            .iter()
            .filter(|finding| finding.strength == ContractStrength::Refusal)
            .collect()
    }

    fn advisories(findings: &[SourceContractFinding]) -> Vec<&SourceContractFinding> {
        findings
            .iter()
            .filter(|finding| finding.strength == ContractStrength::Advisory)
            .collect()
    }

    /// The rule this replaced refused the card outright. The engine runs it, so
    /// a lane that "restores" the old strength has to delete this test first.
    #[test]
    fn an_over_long_pulse_is_advised_and_never_refused() {
        let findings = findings_for(
            ComponentType::VoltageSourcePulse,
            &[
                ("tr", "1u"),
                ("tf", "1u"),
                ("pw", "10u"),
                ("per", "2u"),
                ("td", "0"),
            ],
        );
        assert!(refusals(&findings).is_empty(), "{findings:?}");
        assert!(
            advisories(&findings)
                .iter()
                .any(|finding| finding.field == "per" && finding.message.contains("truncates")),
            "{findings:?}"
        );
    }

    #[test]
    fn a_zero_pulse_edge_is_advised_as_a_tstep_substitution() {
        let findings = findings_for(
            ComponentType::VoltageSourcePulse,
            &[("tr", "0"), ("tf", "0")],
        );
        assert!(refusals(&findings).is_empty(), "{findings:?}");
        let edges = advisories(&findings)
            .iter()
            .filter(|finding| finding.message.contains("TSTEP"))
            .count();
        assert_eq!(edges, 2, "{findings:?}");
    }

    #[test]
    fn a_bounded_pulse_train_states_what_it_holds_afterwards() {
        let findings = findings_for(ComponentType::CurrentSourcePulse, &[("np", "8")]);
        assert!(
            advisories(&findings)
                .iter()
                .any(|finding| finding.field == "np" && finding.message.contains("I1")),
            "{findings:?}"
        );
    }

    /// The old rule refused this pair. `resolve_exp_timing` keeps both.
    #[test]
    fn an_out_of_order_exp_pair_is_advised_and_never_refused() {
        let findings = findings_for(
            ComponentType::VoltageSourceExp,
            &[("td1", "5u"), ("tau1", "1u"), ("td2", "1u"), ("tau2", "1u")],
        );
        assert!(refusals(&findings).is_empty(), "{findings:?}");
        assert!(
            advisories(&findings)
                .iter()
                .any(|finding| finding.field == "td2" && finding.message.contains("precedes")),
            "{findings:?}"
        );
    }

    /// The sheet's own TD1 default is zero, so this advisory is what a freshly
    /// placed EXP source says about itself.
    #[test]
    fn a_zero_exp_delay_is_advised_as_omitted() {
        let findings = findings_for(
            ComponentType::VoltageSourceExp,
            &[("td1", "0"), ("tau1", "1u"), ("td2", "5u"), ("tau2", "1u")],
        );
        assert!(
            advisories(&findings)
                .iter()
                .any(|finding| finding.field == "td1" && finding.message.contains("TSTEP")),
            "{findings:?}"
        );
    }

    #[test]
    fn a_zero_sin_frequency_is_advised_as_a_tstop_substitution() {
        let findings = findings_for(ComponentType::VoltageSourceSin, &[("freq", "0")]);
        assert!(refusals(&findings).is_empty(), "{findings:?}");
        assert!(
            advisories(&findings)
                .iter()
                .any(|finding| finding.field == "freq" && finding.message.contains("1 / TSTOP")),
            "{findings:?}"
        );
    }

    #[test]
    fn a_delayed_sin_states_the_level_it_holds_before_the_delay() {
        let findings = findings_for(
            ComponentType::CurrentSourceSin,
            &[("freq", "1k"), ("td", "1m")],
        );
        assert!(
            advisories(&findings)
                .iter()
                .any(|finding| finding.field == "td" && finding.message.contains("IO")),
            "{findings:?}"
        );
    }

    /// Verified against the evaluator: the MDI limiter is ngspice's if/else-if
    /// chain, so a negative FS lands MDI on the negative `FC/FS` ratio (or on
    /// 0 when below it) and the run keeps going — advise, never refuse.
    #[test]
    fn a_negative_sffm_signal_frequency_is_advised_not_refused() {
        let findings = findings_for(
            ComponentType::VoltageSourceSffm,
            &[("fc", "1Meg"), ("fs", "-1k"), ("mdi", "1")],
        );
        assert!(refusals(&findings).is_empty(), "{findings:?}");
        assert!(
            advisories(&findings)
                .iter()
                .any(|finding| finding.field == "fs" && finding.message.contains("FC/FS")),
            "{findings:?}"
        );
    }

    #[test]
    fn an_out_of_range_sffm_index_is_advised_as_clamped() {
        let findings = findings_for(
            ComponentType::VoltageSourceSffm,
            &[("fc", "1k"), ("fs", "1k"), ("mdi", "50")],
        );
        assert!(refusals(&findings).is_empty(), "{findings:?}");
        assert!(
            advisories(&findings)
                .iter()
                .any(|finding| finding.field == "mdi" && finding.message.contains("clamped")),
            "{findings:?}"
        );
    }

    #[test]
    fn a_zero_am_frequency_is_advised_as_omitted() {
        let findings = findings_for(
            ComponentType::VoltageSourceAm,
            &[("fm", "0"), ("fc", "1Meg")],
        );
        assert!(refusals(&findings).is_empty(), "{findings:?}");
        assert!(
            advisories(&findings)
                .iter()
                .any(|finding| finding.field == "fm" && finding.message.contains("5 / TSTOP")),
            "{findings:?}"
        );
    }

    #[test]
    fn an_inverted_am_pair_is_advised_and_never_refused() {
        let findings = findings_for(
            ComponentType::CurrentSourceAm,
            &[("fm", "1Meg"), ("fc", "1k")],
        );
        assert!(refusals(&findings).is_empty(), "{findings:?}");
        assert!(
            advisories(&findings)
                .iter()
                .any(|finding| finding.field == "fc"),
            "{findings:?}"
        );
    }

    #[test]
    fn a_negative_pwl_delay_is_refused() {
        let findings = findings_for(ComponentType::VoltageSourcePwl, &[("td", "-1n")]);
        assert_eq!(refusals(&findings).len(), 1, "{findings:?}");
        assert_eq!(refusals(&findings)[0].field, "td");
    }

    #[test]
    fn a_delayed_pwl_states_that_it_is_zero_first() {
        let findings = findings_for(ComponentType::CurrentSourcePwl, &[("td", "1u")]);
        assert!(refusals(&findings).is_empty(), "{findings:?}");
        assert!(
            advisories(&findings)
                .iter()
                .any(|finding| finding.field == "td" && finding.message.contains("exactly 0")),
            "{findings:?}"
        );
    }

    #[test]
    fn trnoise_without_a_sample_interval_is_refused_on_either_quantity() {
        for kind in [
            ComponentType::VoltageSourceNoise,
            ComponentType::CurrentSourceNoise,
        ] {
            let findings = findings_for(kind, &[("na", "1n"), ("nt", "0")]);
            assert_eq!(refusals(&findings).len(), 1, "{kind:?} {findings:?}");
            assert_eq!(refusals(&findings)[0].field, "nt", "{kind:?}");
        }
    }

    /// The engine accepts NALPHA = 0 as the unshaped endpoint of the generator,
    /// so the refusal is the range check, not a "greater than zero" one.
    #[test]
    fn a_zero_flicker_exponent_is_accepted_and_two_is_refused() {
        let accepted = findings_for(
            ComponentType::VoltageSourceNoise,
            &[("na", "1n"), ("nt", "1u"), ("namp", "1n"), ("nalpha", "0")],
        );
        assert!(refusals(&accepted).is_empty(), "{accepted:?}");
        let refused = findings_for(
            ComponentType::VoltageSourceNoise,
            &[("na", "1n"), ("nt", "1u"), ("namp", "1n"), ("nalpha", "2")],
        );
        assert_eq!(refusals(&refused).len(), 1, "{refused:?}");
        assert_eq!(refusals(&refused)[0].field, "nalpha");
    }

    #[test]
    fn a_silent_noise_source_says_so() {
        let findings = findings_for(
            ComponentType::VoltageSourceNoise,
            &[("na", "0"), ("nt", "1u"), ("namp", "0")],
        );
        assert!(refusals(&findings).is_empty(), "{findings:?}");
        assert!(
            advisories(&findings)
                .iter()
                .any(|finding| finding.field == "na"),
            "{findings:?}"
        );
    }

    #[test]
    fn a_component_that_is_not_an_independent_source_has_no_contract() {
        let findings = findings_for(ComponentType::Resistor, &[("tr", "0")]);
        assert!(findings.is_empty(), "{findings:?}");
    }

    /// An expression naming a design variable is not a contract question: it
    /// reduces to nothing here and the rules stay silent rather than guessing.
    #[test]
    fn a_deferred_expression_produces_no_finding() {
        let findings = findings_for(ComponentType::VoltageSourceSffm, &[("fs", "{fsig}")]);
        assert!(findings.is_empty(), "{findings:?}");
    }
}
