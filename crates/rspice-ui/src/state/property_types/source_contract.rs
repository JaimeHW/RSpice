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
//!
//! A refusal here mirrors something the deck would refuse, and nothing else.
//! Some rule sets have to be read with the netlist generator open beside them,
//! because the generator normalizes fields before the parser ever sees them: an
//! omitted `PAT` edge time takes the sheet's default rather than reaching the
//! parser as a zero, a bit pattern typed without its leading `B` is given one,
//! and a `TRRANDOM` card is written with all five fields whether or not anyone
//! set them. A rule for any of those would refuse a card the generator was
//! about to fix.
//!
//! These rules audit a stimulus *definition* as well as a placed instance. A
//! definition is the same `(component type, value, params)` triple, so it is
//! audited through the transient component it realizes to and there is only one
//! rule set for both.

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

    /// The field as text, for the two waveform fields that are not numbers:
    /// a bit pattern and a file reference.
    ///
    /// Resolved in the same order as [`Self::number`], so a rule and the editor
    /// beside it cannot read a different string for the same field.
    pub fn text(&self, name: &str) -> Option<String> {
        self.values
            .get(name)
            .or_else(|| self.sheet.get(name).map(|def| &def.default_value))
            .map(PropertyValue::display_string)
            .filter(|value| !value.trim().is_empty())
            .or_else(|| self.params.get(name).cloned())
            .or_else(|| {
                self.primary
                    .filter(|(field, _)| *field == name)
                    .map(|(_, value)| value.to_owned())
            })
            .filter(|value| !value.trim().is_empty())
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
        ComponentType::VoltageSourcePwlFile | ComponentType::CurrentSourcePwlFile => {
            pwl_file_findings(fields, &mut findings);
        }
        ComponentType::VoltageSourcePat | ComponentType::CurrentSourcePat => {
            pat_findings(fields, &mut findings);
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
        ComponentType::VoltageSourceRandom | ComponentType::CurrentSourceRandom => {
            trrandom_findings(fields, &mut findings);
        }
        _ => {}
    }
    findings
}

/// The four TRRANDOM distributions, in the order the engine numbers them.
///
/// The card carries `TYPE` as an integer from 1 through 4
/// (`netlist/parser/source_specs.rs` `parse_trrandom_spec`); this is the
/// spelling the sheet offers and the generator translates. One table, so the
/// editor's chips, the rules and the emitted card cannot disagree about which
/// distribution number 3 is.
pub const TRRANDOM_DISTRIBUTIONS: [&str; 4] = ["uniform", "gaussian", "exponential", "poisson"];

/// The engine's `TYPE` for a distribution as the sheet spells it, or for a
/// bare integer someone authored directly.
///
/// `None` is a value the netlist parser will refuse, and this module reports it
/// as such rather than substituting one — a card that named `normal` and ran as
/// `uniform` would be worse than a card that did not run.
#[must_use]
pub fn trrandom_distribution_number(authored: &str) -> Option<u8> {
    let authored = authored.trim();
    if let Some(index) = TRRANDOM_DISTRIBUTIONS
        .iter()
        .position(|name| authored.eq_ignore_ascii_case(name))
    {
        return u8::try_from(index + 1).ok();
    }
    match authored.parse::<u8>() {
        Ok(number @ 1..=4) => Some(number),
        _ => None,
    }
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

/// `PWL FILE="path" [TD=] [R=] [TSCALE=] [VSCALE=] [TOFFSET=] [VOFFSET=]`.
///
/// The table itself is the file's business and cannot be judged from a property
/// sheet — whether it parses, whether its times increase — so the refusals here
/// are only the ones a card can carry on its own face. Two of them are the
/// parser's (`validate_pwl_file_scaling`, `validate_pwl_delay` and
/// `validate_pwl_repeat_from` in `netlist/parser/source_specs.rs`) and one is
/// the netlist generator's own, which refuses a file-backed source with nothing
/// selected rather than dispatching a run that fails inside the engine.
fn pwl_file_findings(fields: &SourceFields<'_>, findings: &mut Vec<SourceContractFinding>) {
    if fields.text("file").is_none() {
        findings.push(SourceContractFinding::refusal(
            "file",
            "A file-backed PWL source has no waveform until a data file is selected",
        ));
    }
    match fields.number("tscale") {
        Some(scale) if scale <= 0.0 => findings.push(SourceContractFinding::refusal(
            "tscale",
            "PWL FILE TSCALE must be finite and positive — the netlist parser refuses zero or a negative time scale",
        )),
        Some(scale) if scale != 1.0 => findings.push(SourceContractFinding::advisory(
            "tscale",
            "Times in the file are divided by TSCALE, so a scale above 1 stretches the waveform and one below it compresses",
        )),
        _ => {}
    }
    match fields.number("td") {
        Some(delay) if delay < 0.0 => findings.push(SourceContractFinding::refusal(
            "td",
            "PWL FILE TD must be finite and non-negative — the netlist parser refuses a negative delay",
        )),
        Some(delay) if delay > 0.0 => findings.push(SourceContractFinding::advisory(
            "td",
            "Output is exactly 0 before TD, then holds the file's first value until its time",
        )),
        _ => {}
    }
    if let Some(repeat) = fields
        .text("r")
        .and_then(|value| crate::quantity::parse_engineering_value(&value).ok())
    {
        if repeat < 0.0 {
            findings.push(SourceContractFinding::refusal(
                "r",
                "PWL FILE R must be finite and non-negative — the netlist parser refuses a negative repeat time",
            ));
        } else {
            findings.push(SourceContractFinding::advisory(
                "r",
                "Past the file's last point the waveform folds back to R and repeats from there, indefinitely",
            ));
        }
    }
    if fields
        .number("vscale")
        .is_some_and(|scale| scale != 1.0 && scale.is_finite())
        || fields
            .number("voffset")
            .is_some_and(|offset| offset != 0.0 && offset.is_finite())
    {
        findings.push(SourceContractFinding::advisory(
            "vscale",
            "Each value in the file is read as VALUE * VSCALE + VOFFSET — the scale is applied first, and the offset is not a DC level added to a scaled waveform",
        ));
    }
}

/// `PAT(VHI VLO TD TR TF TSAMPLE DATA [R=n])`.
///
/// The netlist generator normalizes two things before emission, so the rules
/// here deliberately do not repeat them: an omitted TR/TF/TSAMPLE takes the
/// sheet's default rather than reaching the parser as a zero, and a bit string
/// typed without its leading `B` is given one. What is left is what the parser
/// really refuses (`validate_pat_spec`, `parse_pat_repeat_count`).
fn pat_findings(fields: &SourceFields<'_>, findings: &mut Vec<SourceContractFinding>) {
    for (field, label) in [("tr", "TR"), ("tf", "TF"), ("tsample", "TSAMPLE")] {
        if fields.number(field).is_some_and(|value| value <= 0.0) {
            findings.push(SourceContractFinding::refusal(
                field,
                format!(
                    "PAT {label} must be positive — the netlist parser refuses a zero or negative \
                     edge or bit interval"
                ),
            ));
        }
    }
    // A blank pattern is not a refusal: the generator emits the sheet's own
    // default for it, exactly as it does for an omitted TR. Only a pattern
    // someone actually wrote, and wrote something other than bits into, reaches
    // the parser as an error.
    if let Some(data) = fields.text("data") {
        let bits = data.trim();
        let bits = bits.strip_prefix(['b', 'B']).unwrap_or(bits);
        if bits.is_empty() || !bits.chars().all(|bit| matches!(bit, '0' | '1')) {
            findings.push(SourceContractFinding::refusal(
                "data",
                "PAT DATA must be a bit string of 0s and 1s — the netlist parser refuses any \
                 other character",
            ));
        }
    }
    match fields.number("repeat_count") {
        Some(repeat) if repeat.fract() != 0.0 => findings.push(SourceContractFinding::refusal(
            "repeat_count",
            "PAT R must be a whole repeat count — the netlist parser refuses a fractional one",
        )),
        Some(repeat) if repeat < -1.0 => findings.push(SourceContractFinding::advisory(
            "repeat_count",
            "A repeat count below -1 is read as 0: the pattern plays once and then holds its last bit",
        )),
        _ => {}
    }
    if fields.number("td").is_some_and(|delay| delay > 0.0) {
        findings.push(SourceContractFinding::advisory(
            "td",
            "Output holds VLO before TD, then the first bit's level",
        ));
    }
}

/// `SFFM(VO VA FC MDI FM TD PHASEM PHASEC)`.
///
/// No refusals: the evaluator limits MDI with ngspice's if/else-if chain, so a
/// negative FM turns `FC/FM` negative and MDI lands on that ratio (or on 0 when
/// below it) — the card runs, it just never uses the authored index.
fn sffm_findings(fields: &SourceFields<'_>, findings: &mut Vec<SourceContractFinding>) {
    if fields.number("fm").is_some_and(|signal| signal < 0.0) {
        findings.push(SourceContractFinding::advisory(
            "fm",
            "A negative FM inverts the MDI limiter: MDI lands on FC/FM (negative, mirroring the modulation) or on 0, never on the authored value",
        ));
    }
    if fields.number("fc").is_some_and(|carrier| carrier <= 0.0) {
        findings.push(SourceContractFinding::advisory(
            "fc",
            "An FC of 0 is read as omitted: the engine substitutes 5 / TSTOP",
        ));
    }
    if fields.number("fm").is_some_and(|signal| signal == 0.0) {
        findings.push(SourceContractFinding::advisory(
            "fm",
            "An FM of 0 is read as omitted: the engine substitutes 500 / TSTOP",
        ));
    }
    if let (Some(index), Some(carrier), Some(signal)) = (
        fields.number("mdi"),
        fields.number("fc"),
        fields.number("fm"),
    ) && carrier > 0.0
        && signal > 0.0
        && (index < 0.0 || index > carrier / signal)
    {
        findings.push(SourceContractFinding::advisory(
            "mdi",
            "MDI is clamped to 0 … FC/FM at evaluation — the authored index is outside that range",
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

/// `TRNOISE(NA NT NALPHA NAMP RTSAM RTSCAPT RTSEMT)`.
///
/// Every refusal here is the netlist parser's own, and each is conditional on
/// an amplitude actually being set — a fully zeroed noise source parses.
///
/// The random-telegraph tail is a group, and the engine treats it as one
/// (`netlist/parser/source_specs.rs:606-615`, mirrored by the evaluator at
/// `engine/transient/noise.rs:221-226`):
///
/// - `RTSAM` at zero disables the telegraph outright; the two mean times are
///   then read and ignored.
/// - Both mean times at zero *also* disable it, silently. That is the parser's
///   own "incomplete group" allowance, so it is advised, never refused.
/// - Exactly one mean time at zero, or either of them negative, with an
///   amplitude set, is a hard parse error. The deck does not run.
fn trnoise_findings(fields: &SourceFields<'_>, findings: &mut Vec<SourceContractFinding>) {
    let white = fields.number("na").unwrap_or(0.0);
    let flicker = fields.number("namp").unwrap_or(0.0);
    let telegraph = fields.number("rtsam").unwrap_or(0.0);
    let capture = fields.number("rtscapt").unwrap_or(0.0);
    let emission = fields.number("rtsemt").unwrap_or(0.0);
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
    // The engine's own all-quiet shortcut is the three amplitudes together
    // (`engine/transient/noise.rs:161-163`), so the RTS step height counts as
    // an amplitude here exactly as it does there.
    if white == 0.0 && flicker == 0.0 && telegraph == 0.0 {
        findings.push(SourceContractFinding::advisory(
            "na",
            "Every noise amplitude is 0 — the source contributes exactly its DC level",
        ));
    }
    let telegraph_is_disabled_by_its_times = capture == 0.0 && emission == 0.0;
    if telegraph != 0.0 {
        if telegraph_is_disabled_by_its_times {
            findings.push(SourceContractFinding::advisory(
                "rtscapt",
                "Both RTS mean times are 0, so the engine drops the telegraph entirely and this \
                 amplitude does nothing",
            ));
        } else if !(capture > 0.0 && emission > 0.0) {
            findings.push(SourceContractFinding::refusal(
                if capture > 0.0 { "rtsemt" } else { "rtscapt" },
                "TRNOISE RTS requires positive capture and emission mean times once RTSAM is set; \
                 leave both at 0 to disable the telegraph instead",
            ));
        }
    } else if capture != 0.0 || emission != 0.0 {
        findings.push(SourceContractFinding::advisory(
            "rtsam",
            "RTS amplitude is 0, so the capture and emission mean times are read and ignored",
        ));
    }
    findings.push(SourceContractFinding::advisory(
        "nt",
        "The operating point sees exactly 0 from this source: the noise train is zero-mean and transient-only",
    ));
}

/// `TRRANDOM(TYPE TS TD PARAM1 PARAM2)`.
///
/// The refusals are the parser's own, in the order it makes them
/// (`netlist/parser/source_specs.rs` `parse_trrandom_spec`, lines 663-728): a
/// TYPE that is not an integer from 1 through 4, a TS that is not positive, and
/// a negative TD or PARAM1. Omission is not among them — the parser refuses a
/// card with no TYPE or TS, but the generator substitutes the sheet's default
/// for every omitted field before the parser sees one, so a rule for an unset
/// field would refuse a card that was about to be written correctly. That is
/// the same reason `pat_findings` does not refuse a blank pattern.
///
/// The advisories are what TYPE does to the other two fields. PARAM1 and PARAM2
/// keep one label across four distributions that read them differently
/// (`engine/transient/noise.rs` `generate_trrandom_points`), and that is exactly
/// the class of thing this module exists to state.
fn trrandom_findings(fields: &SourceFields<'_>, findings: &mut Vec<SourceContractFinding>) {
    let authored_type = fields.text("type");
    let distribution = authored_type
        .as_deref()
        .and_then(trrandom_distribution_number);
    if authored_type.is_some() && distribution.is_none() {
        findings.push(SourceContractFinding::refusal(
            "type",
            "TRRANDOM TYPE must be one of uniform, gaussian, exponential or poisson — the netlist \
             parser refuses anything but the integers 1 through 4",
        ));
    }
    if fields.number("ts").is_some_and(|interval| interval <= 0.0) {
        findings.push(SourceContractFinding::refusal(
            "ts",
            "TRRANDOM requires a positive sample interval TS",
        ));
    }
    if fields.number("td").is_some_and(|delay| delay < 0.0) {
        findings.push(SourceContractFinding::refusal(
            "td",
            "TRRANDOM refuses a negative TD: the draw starts at or after time zero",
        ));
    }
    if fields.number("param1").is_some_and(|spread| spread < 0.0) {
        findings.push(SourceContractFinding::refusal(
            "param1",
            "TRRANDOM refuses a negative PARAM1 — use PARAM2 to move the level, not the spread",
        ));
    }
    match distribution {
        Some(1) => findings.push(SourceContractFinding::advisory(
            "param1",
            "PARAM1 is a half-range here: every sample lands between PARAM2 - PARAM1 and \
             PARAM2 + PARAM1",
        )),
        Some(3) => findings.push(SourceContractFinding::advisory(
            "param1",
            "The exponential draw is one-sided: PARAM1 is its mean and no sample ever falls below \
             PARAM2",
        )),
        Some(4) => findings.push(SourceContractFinding::advisory(
            "param1",
            "Poisson reads PARAM1 as a mean count rather than an amplitude, and adds the whole \
             number it draws to PARAM2",
        )),
        _ => {}
    }
    if fields.number("param1") == Some(0.0) {
        findings.push(SourceContractFinding::advisory(
            "param1",
            "A PARAM1 of 0 leaves nothing to draw: every sample is exactly PARAM2",
        ));
    }
    if fields.number("td").is_some_and(|delay| delay > 0.0) {
        findings.push(SourceContractFinding::advisory(
            "td",
            "Before TD the output holds PARAM2, not 0",
        ));
    }
    findings.push(SourceContractFinding::advisory(
        "ts",
        "The operating point sees exactly PARAM2 from this source: the draw is expanded into a \
         seeded sample train, one value every TS, only when a transient builds its circuit",
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
    /// chain, so a negative FM lands MDI on the negative `FC/FM` ratio (or on
    /// 0 when below it) and the run keeps going — advise, never refuse.
    #[test]
    fn a_negative_sffm_signal_frequency_is_advised_not_refused() {
        let findings = findings_for(
            ComponentType::VoltageSourceSffm,
            &[("fc", "1Meg"), ("fm", "-1k"), ("mdi", "1")],
        );
        assert!(refusals(&findings).is_empty(), "{findings:?}");
        assert!(
            advisories(&findings)
                .iter()
                .any(|finding| finding.field == "fm" && finding.message.contains("FC/FM")),
            "{findings:?}"
        );
    }

    #[test]
    fn an_out_of_range_sffm_index_is_advised_as_clamped() {
        let findings = findings_for(
            ComponentType::VoltageSourceSffm,
            &[("fc", "1k"), ("fm", "1k"), ("mdi", "50")],
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

    /// An RTS step height is an amplitude: a source carrying one is not
    /// silent, and the engine's own all-quiet shortcut agrees
    /// (`engine/transient/noise.rs:161-163`).
    #[test]
    fn a_telegraph_only_noise_source_is_not_called_silent() {
        let findings = findings_for(
            ComponentType::VoltageSourceNoise,
            &[
                ("na", "0"),
                ("nt", "1u"),
                ("namp", "0"),
                ("rtsam", "5m"),
                ("rtscapt", "2u"),
                ("rtsemt", "3u"),
            ],
        );
        assert!(refusals(&findings).is_empty(), "{findings:?}");
        assert!(
            !advisories(&findings)
                .iter()
                .any(|finding| finding.message.contains("exactly its DC level")),
            "{findings:?}"
        );
    }

    /// The parser refuses exactly one half of the dwell pair
    /// (`netlist/parser/source_specs.rs:606-615`); the evaluator refuses the
    /// same shape (`engine/transient/noise.rs:224-226`). Refusal, therefore,
    /// not advice.
    #[test]
    fn half_an_rts_dwell_pair_is_refused_and_neither_half_is_guessed() {
        for (capture, emission, field) in [("2u", "0", "rtsemt"), ("0", "3u", "rtscapt")] {
            let findings = findings_for(
                ComponentType::VoltageSourceNoise,
                &[
                    ("na", "1n"),
                    ("nt", "1u"),
                    ("rtsam", "5m"),
                    ("rtscapt", capture),
                    ("rtsemt", emission),
                ],
            );
            assert_eq!(refusals(&findings).len(), 1, "{findings:?}");
            assert_eq!(refusals(&findings)[0].field, field, "{findings:?}");
        }
        // A negative mean time fails the same `> 0` test the parser applies.
        let findings = findings_for(
            ComponentType::CurrentSourceNoise,
            &[
                ("na", "1n"),
                ("nt", "1u"),
                ("rtsam", "5m"),
                ("rtscapt", "-2u"),
                ("rtsemt", "3u"),
            ],
        );
        assert_eq!(refusals(&findings).len(), 1, "{findings:?}");
        assert_eq!(refusals(&findings)[0].field, "rtscapt");
    }

    /// Both mean times at zero is the parser's own "incomplete group is
    /// disabled" allowance (`source_specs.rs:606-607`). The card runs, so this
    /// is advice; what it has to say is that the amplitude does nothing.
    #[test]
    fn an_rts_amplitude_with_no_dwell_times_is_advised_never_refused() {
        let findings = findings_for(
            ComponentType::VoltageSourceNoise,
            &[("na", "1n"), ("nt", "1u"), ("rtsam", "5m")],
        );
        assert!(refusals(&findings).is_empty(), "{findings:?}");
        assert!(
            advisories(&findings)
                .iter()
                .any(|finding| finding.field == "rtscapt"
                    && finding.message.contains("does nothing")),
            "{findings:?}"
        );
    }

    /// Dwell times with no amplitude are read and thrown away
    /// (`source_specs.rs:607`, `noise.rs:221`), which the sheet has no other
    /// way of saying.
    #[test]
    fn rts_dwell_times_without_an_amplitude_are_advised_as_ignored() {
        let findings = findings_for(
            ComponentType::VoltageSourceNoise,
            &[
                ("na", "1n"),
                ("nt", "1u"),
                ("rtsam", "0"),
                ("rtscapt", "2u"),
                ("rtsemt", "3u"),
            ],
        );
        assert!(refusals(&findings).is_empty(), "{findings:?}");
        assert!(
            advisories(&findings)
                .iter()
                .any(|finding| finding.field == "rtsam" && finding.message.contains("ignored")),
            "{findings:?}"
        );
    }

    #[test]
    fn a_pat_edge_or_bit_interval_of_zero_is_refused_as_the_parser_refuses_it() {
        for field in ["tr", "tf", "tsample"] {
            let findings = findings_for(ComponentType::VoltageSourcePat, &[(field, "0")]);
            assert!(
                refusals(&findings)
                    .iter()
                    .any(|finding| finding.field == field),
                "{field}: {findings:?}"
            );
        }
    }

    /// The generator supplies its own default for an omitted edge time and
    /// prefixes a bare bit string, so neither may be refused here.
    #[test]
    fn an_omitted_pat_field_and_an_unprefixed_pattern_are_left_alone() {
        assert!(findings_for(ComponentType::CurrentSourcePat, &[]).is_empty());
        assert!(
            findings_for(ComponentType::VoltageSourcePat, &[("data", "0101")])
                .iter()
                .all(|finding| finding.field != "data")
        );
    }

    #[test]
    fn a_pat_pattern_that_is_not_bits_is_refused() {
        let findings = findings_for(ComponentType::VoltageSourcePat, &[("data", "b01x1")]);
        assert!(
            refusals(&findings)
                .iter()
                .any(|finding| finding.field == "data"),
            "{findings:?}"
        );
    }

    #[test]
    fn a_fractional_pat_repeat_count_is_refused_and_a_deep_negative_one_is_advised() {
        assert!(
            refusals(&findings_for(
                ComponentType::VoltageSourcePat,
                &[("repeat_count", "2.5")]
            ))
            .iter()
            .any(|finding| finding.field == "repeat_count")
        );
        let findings = findings_for(ComponentType::VoltageSourcePat, &[("repeat_count", "-4")]);
        assert!(refusals(&findings).is_empty(), "{findings:?}");
        assert!(
            advisories(&findings)
                .iter()
                .any(|finding| finding.field == "repeat_count"),
            "{findings:?}"
        );
    }

    #[test]
    fn a_file_backed_pwl_source_with_no_file_is_refused() {
        let findings = findings_for(ComponentType::VoltageSourcePwlFile, &[]);
        assert!(
            refusals(&findings)
                .iter()
                .any(|finding| finding.field == "file"),
            "{findings:?}"
        );
        assert!(
            findings_for(
                ComponentType::CurrentSourcePwlFile,
                &[("file", "bridge_step.csv")]
            )
            .iter()
            .all(|finding| finding.field != "file")
        );
    }

    #[test]
    fn a_non_positive_pwl_file_time_scale_is_refused_and_any_other_scale_is_advised() {
        for scale in ["0", "-1"] {
            let findings = findings_for(
                ComponentType::VoltageSourcePwlFile,
                &[("file", "step.csv"), ("tscale", scale)],
            );
            assert!(
                refusals(&findings)
                    .iter()
                    .any(|finding| finding.field == "tscale"),
                "{scale}: {findings:?}"
            );
        }
        let findings = findings_for(
            ComponentType::VoltageSourcePwlFile,
            &[("file", "step.csv"), ("tscale", "1m")],
        );
        assert!(refusals(&findings).is_empty(), "{findings:?}");
        assert!(
            advisories(&findings)
                .iter()
                .any(|finding| finding.field == "tscale"),
            "{findings:?}"
        );
    }

    #[test]
    fn a_negative_pwl_file_delay_or_repeat_time_is_refused() {
        for (field, value) in [("td", "-1n"), ("r", "-1")] {
            let findings = findings_for(
                ComponentType::VoltageSourcePwlFile,
                &[("file", "step.csv"), (field, value)],
            );
            assert!(
                refusals(&findings)
                    .iter()
                    .any(|finding| finding.field == field),
                "{field}: {findings:?}"
            );
        }
    }

    #[test]
    fn pwl_file_value_scaling_states_the_order_the_engine_applies_it_in() {
        let findings = findings_for(
            ComponentType::VoltageSourcePwlFile,
            &[("file", "step.csv"), ("vscale", "2"), ("voffset", "1")],
        );
        assert!(refusals(&findings).is_empty(), "{findings:?}");
        assert!(
            advisories(&findings)
                .iter()
                .any(|finding| finding.field == "vscale"
                    && finding.message.contains("VALUE * VSCALE + VOFFSET")),
            "{findings:?}"
        );
    }

    /// Every TRRANDOM refusal is one the netlist parser makes, and nothing else
    /// is a refusal (`netlist/parser/source_specs.rs:693-719`).
    #[test]
    fn the_trrandom_refusals_are_exactly_the_parsers_own() {
        for (pairs, field) in [
            (vec![("type", "normal"), ("ts", "1u")], "type"),
            (vec![("type", "5"), ("ts", "1u")], "type"),
            (vec![("type", "uniform"), ("ts", "0")], "ts"),
            (vec![("type", "uniform"), ("ts", "-1u")], "ts"),
            (vec![("type", "uniform"), ("ts", "1u"), ("td", "-1u")], "td"),
            (
                vec![("type", "uniform"), ("ts", "1u"), ("param1", "-1")],
                "param1",
            ),
        ] {
            let findings = findings_for(ComponentType::VoltageSourceRandom, &pairs);
            assert_eq!(refusals(&findings).len(), 1, "{pairs:?} {findings:?}");
            assert_eq!(refusals(&findings)[0].field, field, "{findings:?}");
        }

        // The sheet's own defaults, and the four spellings it offers, are clean.
        for distribution in TRRANDOM_DISTRIBUTIONS {
            let findings = findings_for(
                ComponentType::CurrentSourceRandom,
                &[("type", distribution), ("ts", "1u"), ("param1", "1")],
            );
            assert!(refusals(&findings).is_empty(), "{findings:?}");
        }
    }

    /// A field nobody has set is not a refusal even though the parser would
    /// refuse the card without it: the generator writes the sheet's default
    /// first. The same doctrine as `pat_findings`.
    #[test]
    fn an_unset_trrandom_type_is_left_to_the_generator() {
        let findings = findings_for(ComponentType::VoltageSourceRandom, &[("ts", "1u")]);
        assert!(refusals(&findings).is_empty(), "{findings:?}");
    }

    /// PARAM1 keeps one label across four distributions that read it
    /// differently (`engine/transient/noise.rs` `generate_trrandom_points`), so
    /// each one says what it will do with the number.
    #[test]
    fn each_trrandom_distribution_states_what_it_makes_of_param1() {
        for (distribution, fragment) in [
            ("uniform", "half-range"),
            ("exponential", "one-sided"),
            ("poisson", "mean count"),
        ] {
            let findings = findings_for(
                ComponentType::VoltageSourceRandom,
                &[("type", distribution), ("ts", "1u"), ("param1", "1")],
            );
            assert!(
                advisories(&findings)
                    .iter()
                    .any(|finding| finding.field == "param1"
                        && finding.message.to_lowercase().contains(fragment)),
                "{distribution}: {findings:?}"
            );
        }
        // Gaussian reads PARAM1 as its label already says, so it says nothing.
        let gaussian = findings_for(
            ComponentType::VoltageSourceRandom,
            &[("type", "gaussian"), ("ts", "1u"), ("param1", "1")],
        );
        assert!(
            !advisories(&gaussian)
                .iter()
                .any(|finding| finding.field == "param1"),
            "{gaussian:?}"
        );
    }

    /// The operating point sees PARAM2, not zero and not a draw — the same
    /// class of statement TRNOISE's closing advisory makes, with the offset the
    /// engine actually returns (`circuit/storage/sources.rs:1385`).
    #[test]
    fn a_random_source_always_states_that_the_draw_waits_for_a_transient() {
        let findings = findings_for(
            ComponentType::CurrentSourceRandom,
            &[("type", "gaussian"), ("ts", "1u"), ("param1", "1")],
        );
        assert!(
            advisories(&findings)
                .iter()
                .any(|finding| finding.field == "ts" && finding.message.contains("exactly PARAM2")),
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
        let findings = findings_for(ComponentType::VoltageSourceSffm, &[("fm", "{fsig}")]);
        assert!(findings.is_empty(), "{findings:?}");
    }
}
